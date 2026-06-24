use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use git2::{
    Commit, Cred, FetchOptions, Oid, RemoteCallbacks, Repository, Signature, StatusOptions,
};
use log::{info, warn};
use octocrab::{
    models::{
        pulls::PullRequest, repos::DiffEntry, CheckRuns, CheckStatus, CombinedStatus, StatusState,
    },
    params::State,
    Octocrab,
};
use pulldown_cmark::{Event, HeadingLevel, Parser as MarkdownParser, Tag, TagEnd};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_REMOTE: &str = "rust-lang/this-week-in-rust";
const DEFAULT_BASE: &str = "main";
const COMMUNITY_HEADING: &str = "## Updates from Rust Community";
const MARKER_TOKEN: &str = "submerge-pr:";

#[derive(Debug, Parser)]
#[command(
    name = "submerge",
    about = "Aggregate one-link TWiR submission PRs into a local multi-parent merge commit"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: CommandArgs,
}

#[derive(Debug, Subcommand)]
pub enum CommandArgs {
    /// Discover eligible PRs and write an editable merge buffer.
    Fetch(FetchArgs),
    /// Read an edited merge buffer and create the local multi-parent merge commit.
    Merge(MergeArgs),
}

#[derive(Debug, Parser)]
pub struct FetchArgs {
    /// Draft file to update. Defaults to the latest draft/YYYY-MM-DD-this-week-in-rust.md.
    #[arg(long)]
    pub draft: Option<PathBuf>,

    /// Editable Markdown buffer to write. Defaults to the selected draft file.
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Allow tracked local modifications before writing the intermediate draft.
    #[arg(long)]
    pub allow_dirty: bool,

    /// Print the editable merge buffer instead of writing files.
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Parser)]
pub struct MergeArgs {
    /// Draft file to update. Defaults to the latest draft/YYYY-MM-DD-this-week-in-rust.md.
    #[arg(long)]
    pub draft: Option<PathBuf>,

    /// Edited Markdown buffer to read. Defaults to the selected draft file.
    #[arg(long)]
    pub input: Option<PathBuf>,

    /// Allow tracked local modifications before running.
    #[arg(long)]
    pub allow_dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Submission {
    pub pr: u64,
    pub title: String,
    pub pr_title: String,
    pub author: String,
    pub pr_url: String,
    pub head_sha: String,
    pub ci_state: CiState,
    pub section: String,
    pub line: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiState {
    Success,
    Failure,
}

impl CiState {
    fn emoji(self) -> &'static str {
        match self {
            CiState::Success => "✅",
            CiState::Failure => "❌",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedPr {
    pub pr: u64,
    pub title: String,
    pub reason: String,
    pub url: String,
}

#[derive(Debug, Clone)]
struct PullSummary {
    number: u64,
    title: String,
    author: String,
    url: String,
    head_sha: String,
    base_sha: String,
    draft: bool,
    base_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MarkdownListItem {
    section: String,
    title: String,
    line: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditedBuffer {
    pub final_text: String,
    pub included: Vec<Submission>,
}

pub async fn run(args: Args) -> Result<()> {
    match args.command {
        CommandArgs::Fetch(args) => run_fetch(args).await,
        CommandArgs::Merge(args) => run_merge(args),
    }
}

async fn run_fetch(args: FetchArgs) -> Result<()> {
    info!("opening git repository");
    let repo = Repository::discover(".").context("open git repository")?;
    if !args.allow_dirty {
        info!("checking tracked worktree status");
        ensure_tracked_worktree_clean(&repo)?;
    }
    let workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("submerge must run in a non-bare repository"))?;
    let draft = match args.draft {
        Some(path) => path,
        None => find_latest_draft(workdir)?,
    };
    let draft_rel = normalize_repo_relative_path(workdir, &draft)?;
    info!("using draft {}", draft_rel.display());
    let draft_text = fs::read_to_string(workdir.join(&draft_rel))
        .with_context(|| format!("read {}", draft_rel.display()))?;

    let (owner, name) = parse_repo_name(DEFAULT_REMOTE)?;
    let (client, authenticated) = github_client()?;
    if authenticated {
        info!("using authenticated GitHub API client");
    } else {
        warn!(
            "GITHUB_TOKEN/GH_TOKEN is not set; GitHub API requests are unauthenticated and may be rate-limited. \
Create a fine-grained personal access token at https://github.com/settings/personal-access-tokens/new; \
no repository permissions are needed because tokens can read public repositories."
        );
    }
    info!(
        "listing open PRs for {}/{} targeting {}",
        owner, name, DEFAULT_BASE
    );
    let pulls = fetch_open_pulls(&client, owner, name, DEFAULT_BASE).await?;
    info!("found {} open PRs", pulls.len());
    let mut submissions = Vec::new();
    let mut skipped = Vec::new();

    for pull in pulls {
        info!("checking PR #{}: {}", pull.number, pull.title);
        if pull.draft {
            skipped.push(SkippedPr {
                pr: pull.number,
                title: pull.title,
                reason: "draft PR".to_string(),
                url: pull.url,
            });
            continue;
        }
        if pull.base_ref != DEFAULT_BASE {
            skipped.push(SkippedPr {
                pr: pull.number,
                title: pull.title,
                reason: format!("targets {}, not {}", pull.base_ref, DEFAULT_BASE),
                url: pull.url,
            });
            continue;
        }

        let files = fetch_pr_files(&client, owner, name, pull.number).await?;
        let base_text =
            match fetch_file_text(&client, owner, name, &draft_rel, &pull.base_sha).await {
                Ok(text) => text,
                Err(error) => {
                    skipped.push(SkippedPr {
                        pr: pull.number,
                        title: pull.title,
                        reason: format!("could not read base draft: {error:#}"),
                        url: pull.url,
                    });
                    continue;
                }
            };
        let head_text =
            match fetch_file_text(&client, owner, name, &draft_rel, &pull.head_sha).await {
                Ok(text) => text,
                Err(error) => {
                    skipped.push(SkippedPr {
                        pr: pull.number,
                        title: pull.title,
                        reason: format!("could not read head draft: {error:#}"),
                        url: pull.url,
                    });
                    continue;
                }
            };
        match classify_pr(&pull, &files, &draft_rel, &base_text, &head_text) {
            Ok(mut submission) => {
                info!("checking CI state for PR #{}", submission.pr);
                submission.ci_state =
                    fetch_ci_state(&client, owner, name, &submission.head_sha).await?;
                submissions.push(submission);
            }
            Err(reason) => skipped.push(SkippedPr {
                pr: pull.number,
                title: pull.title,
                reason,
                url: pull.url,
            }),
        }
    }

    submissions.sort_by_key(|submission| submission.pr);
    info!("classification complete");
    print_summary(&submissions, &skipped);

    if submissions.is_empty() {
        bail!("no eligible one-link community submissions found");
    }

    let buffer = build_edit_buffer(&draft_text, &submissions)?;
    if args.dry_run {
        println!("\n--- editable buffer preview ---\n{buffer}");
        return Ok(());
    }

    let output = match args.output {
        Some(path) => normalize_repo_relative_path(workdir, &path)?,
        None => draft_rel.clone(),
    };
    fs::write(workdir.join(&output), buffer)
        .with_context(|| format!("write editable buffer {}", output.display()))?;
    println!("wrote editable buffer to {}", output.display());
    println!(
        "edit {}, run any checks you want, then run: submerge merge",
        output.display()
    );
    Ok(())
}

fn run_merge(args: MergeArgs) -> Result<()> {
    info!("opening git repository");
    let repo = Repository::discover(".").context("open git repository")?;
    let workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("submerge must run in a non-bare repository"))?;

    let draft = match args.draft {
        Some(path) => path,
        None => find_latest_draft(workdir)?,
    };
    let draft_rel = normalize_repo_relative_path(workdir, &draft)?;
    info!("using draft {}", draft_rel.display());
    let input_rel = match args.input {
        Some(path) => normalize_repo_relative_path(workdir, &path)?,
        None => draft_rel.clone(),
    };

    if !args.allow_dirty {
        info!("checking tracked worktree status");
        if input_rel == draft_rel {
            ensure_tracked_worktree_clean_except(&repo, &draft_rel)?;
        } else {
            ensure_tracked_worktree_clean(&repo)?;
        }
    }

    info!("reading edited buffer {}", input_rel.display());
    let edited = fs::read_to_string(workdir.join(&input_rel))
        .with_context(|| format!("read edited buffer {}", input_rel.display()))?;
    info!("parsing edited buffer");
    let parsed = parse_edited_buffer(&edited)?;
    if parsed.included.is_empty() {
        bail!("edited buffer did not retain any PR markers");
    }
    info!("retained {} PRs", parsed.included.len());

    info!("fetching retained PR heads");
    let parent_oids = fetch_and_verify_pr_heads(&repo, &parsed.included)?;
    info!("creating local merge commit");
    let commit_oid = create_merge_commit(
        &repo,
        workdir,
        &draft_rel,
        &parsed.final_text,
        &parsed.included,
        &[],
        &parent_oids,
    )?;

    println!("created local merge commit {commit_oid}");
    Ok(())
}

fn parse_repo_name(repo: &str) -> Result<(&str, &str)> {
    let (owner, name) = repo
        .split_once('/')
        .ok_or_else(|| anyhow!("repo must be in owner/name form"))?;
    if owner.is_empty() || name.is_empty() {
        bail!("repo must be in owner/name form");
    }
    Ok((owner, name))
}

fn github_client() -> Result<(Octocrab, bool)> {
    let token = env::var("GITHUB_TOKEN")
        .or_else(|_| env::var("GH_TOKEN"))
        .ok();
    let mut builder = Octocrab::builder();
    let authenticated = token.is_some();
    if let Some(token) = token {
        builder = builder.personal_token(token);
    }
    Ok((
        builder.build().context("build GitHub client")?,
        authenticated,
    ))
}

async fn fetch_open_pulls(
    client: &Octocrab,
    owner: &str,
    name: &str,
    base: &str,
) -> Result<Vec<PullSummary>> {
    let page = client
        .pulls(owner, name)
        .list()
        .state(State::Open)
        .base(base)
        .per_page(100)
        .send()
        .await
        .with_context(|| format!("list open PRs for {owner}/{name}"))?;
    let pulls = client
        .all_pages(page)
        .await
        .with_context(|| format!("list all open PR pages for {owner}/{name}"))?;

    pulls.into_iter().map(pull_summary).collect()
}

async fn fetch_pr_files(
    client: &Octocrab,
    owner: &str,
    name: &str,
    pr: u64,
) -> Result<Vec<DiffEntry>> {
    let page = client
        .pulls(owner, name)
        .list_files(pr)
        .await
        .with_context(|| format!("list files for PR #{pr}"))?;
    client
        .all_pages(page)
        .await
        .with_context(|| format!("list all file pages for PR #{pr}"))
}

fn pull_summary(pull: PullRequest) -> Result<PullSummary> {
    let number = pull
        .number
        .ok_or_else(|| anyhow!("GitHub PR response is missing number"))?;
    let title = pull
        .title
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing title"))?;
    let author = pull
        .user
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing user"))?
        .login;
    let url = pull
        .html_url
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing html_url"))?
        .to_string();
    let head = pull
        .head
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing head"))?;
    let base = pull
        .base
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing base"))?;

    Ok(PullSummary {
        number,
        title,
        author,
        url,
        head_sha: head.sha,
        base_sha: base.sha,
        draft: pull.draft.unwrap_or(false),
        base_ref: base.ref_field,
    })
}

async fn fetch_file_text(
    client: &Octocrab,
    owner: &str,
    name: &str,
    path: &Path,
    reference: &str,
) -> Result<String> {
    let mut contents = client
        .repos(owner, name)
        .get_content()
        .path(path.to_string_lossy())
        .r#ref(reference)
        .send()
        .await
        .with_context(|| format!("read {} at {}", path.display(), reference))?;
    let mut items = contents.take_items();
    if items.len() != 1 {
        bail!(
            "expected one content item for {} at {}, got {}",
            path.display(),
            reference,
            items.len()
        );
    }
    items.remove(0).decoded_content().ok_or_else(|| {
        anyhow!(
            "GitHub did not return decodable content for {}",
            path.display()
        )
    })
}

async fn fetch_ci_state(client: &Octocrab, owner: &str, name: &str, sha: &str) -> Result<CiState> {
    #[derive(serde::Serialize)]
    struct CheckRunParams {
        per_page: u8,
    }

    #[derive(serde::Serialize)]
    struct EmptyParams {}

    let check_runs: CheckRuns = client
        .get(
            format!("/repos/{owner}/{name}/commits/{sha}/check-runs"),
            Some(&CheckRunParams { per_page: 100 }),
        )
        .await
        .with_context(|| format!("list check runs for {sha}"))?;
    let combined_status: CombinedStatus = client
        .get(
            format!("/repos/{owner}/{name}/commits/{sha}/status"),
            Some(&EmptyParams {}),
        )
        .await
        .with_context(|| format!("read combined status for {sha}"))?;

    let has_check_runs = check_runs.total_count > 0;
    let check_runs_ok = check_runs.check_runs.iter().all(|check| {
        check.status == Some(CheckStatus::Completed)
            && matches!(
                check.conclusion.as_deref(),
                Some("success" | "skipped" | "neutral")
            )
    });

    let has_statuses = !combined_status.statuses.is_empty();
    let combined_state_ok = combined_status.state == StatusState::Success
        && combined_status
            .statuses
            .iter()
            .all(|status| status.state == StatusState::Success);

    if (has_check_runs || has_statuses) && check_runs_ok && (!has_statuses || combined_state_ok) {
        Ok(CiState::Success)
    } else {
        Ok(CiState::Failure)
    }
}

fn classify_pr(
    pull: &PullSummary,
    files: &[DiffEntry],
    draft_rel: &Path,
    base_text: &str,
    head_text: &str,
) -> std::result::Result<Submission, String> {
    if files.len() != 1 {
        return Err(format!("changes {} files", files.len()));
    }

    let file = &files[0];
    if Path::new(&file.filename) != draft_rel {
        return Err(format!(
            "changes {}, not {}",
            file.filename,
            draft_rel.display()
        ));
    }

    let base_items = extract_markdown_list_items(base_text);
    let head_items = extract_markdown_list_items(head_text);
    let mut candidates: Vec<_> = head_items
        .into_iter()
        .filter(|head| {
            !base_items
                .iter()
                .any(|base| base.section == head.section && base.title == head.title)
        })
        .collect();

    if candidates.is_empty() {
        return Err("does not add a valid community list item".to_string());
    }
    if candidates.len() != 1 {
        return Err(format!(
            "adds {} valid community list items",
            candidates.len()
        ));
    }

    let candidate = candidates.remove(0);
    Ok(Submission {
        pr: pull.number,
        title: candidate.title,
        pr_title: pull.title.clone(),
        author: pull.author.clone(),
        pr_url: pull.url.clone(),
        head_sha: pull.head_sha.clone(),
        ci_state: CiState::Failure,
        section: candidate.section,
        line: candidate.line,
    })
}

#[derive(Default)]
struct ItemCapture {
    start: usize,
    section: Option<String>,
    title: String,
}

fn extract_markdown_list_items(text: &str) -> Vec<MarkdownListItem> {
    let mut items = Vec::new();
    let mut in_community = false;
    let mut current_section: Option<String> = None;
    let mut heading: Option<(HeadingLevel, String)> = None;
    let mut item_stack: Vec<ItemCapture> = Vec::new();

    for (event, range) in MarkdownParser::new(text).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                heading = Some((level, String::new()));
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, title)) = heading.take() {
                    let title = title.trim();
                    match level {
                        HeadingLevel::H2 => {
                            in_community = title == COMMUNITY_HEADING.trim_start_matches("## ");
                            current_section = None;
                        }
                        HeadingLevel::H3 if in_community => {
                            current_section = Some(title.to_string());
                        }
                        HeadingLevel::H3 => {
                            current_section = None;
                        }
                        _ => {}
                    }
                }
            }
            Event::Start(Tag::Item) => {
                item_stack.push(ItemCapture {
                    start: range.start,
                    section: if in_community {
                        current_section.clone()
                    } else {
                        None
                    },
                    title: String::new(),
                });
            }
            Event::End(TagEnd::Item) => {
                if let Some(item) = item_stack.pop() {
                    if item_stack.is_empty() {
                        collect_markdown_list_item(text, item, range.end, &mut items);
                    }
                }
            }
            Event::Start(Tag::Link { .. }) | Event::End(TagEnd::Link) => {}
            Event::Text(value)
            | Event::Code(value)
            | Event::InlineMath(value)
            | Event::DisplayMath(value)
            | Event::Html(value)
            | Event::InlineHtml(value)
            | Event::FootnoteReference(value) => {
                if let Some((_level, title)) = heading.as_mut() {
                    title.push_str(&value);
                }
                if let Some(item) = item_stack.last_mut() {
                    item.title.push_str(&value);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some((_level, title)) = heading.as_mut() {
                    title.push(' ');
                }
                if let Some(item) = item_stack.last_mut() {
                    item.title.push(' ');
                }
            }
            Event::Rule | Event::TaskListMarker(_) => {}
            _ => {}
        }
    }

    items
}

fn collect_markdown_list_item(
    text: &str,
    item: ItemCapture,
    end: usize,
    items: &mut Vec<MarkdownListItem>,
) {
    let Some(section) = item.section else {
        return;
    };
    let title = normalize_markdown_text(&item.title);
    if title.is_empty() {
        return;
    }
    let Some(line) = text.get(item.start..end).map(str::trim_end) else {
        return;
    };
    if line.contains('\n') {
        return;
    }
    items.push(MarkdownListItem {
        section,
        title,
        line: line.to_string(),
    });
}

fn normalize_markdown_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn parse_submission_line(line: &str) -> Option<String> {
    let wrapped = format!("{}\n\n### Edited\n{}\n", COMMUNITY_HEADING, line.trim_end());
    let mut items = extract_markdown_list_items(&wrapped);
    if items.len() != 1 {
        return None;
    }
    Some(items.remove(0).title)
}

fn build_edit_buffer(draft_text: &str, submissions: &[Submission]) -> Result<String> {
    let mut grouped: BTreeMap<&str, Vec<&Submission>> = BTreeMap::new();
    for submission in submissions {
        grouped
            .entry(submission.section.as_str())
            .or_default()
            .push(submission);
    }

    let mut out = Vec::new();
    let mut inserted = BTreeSet::new();

    for line in draft_text.lines() {
        out.push(line.to_string());
        let Some(section) = line.strip_prefix("### ").map(str::trim) else {
            continue;
        };
        let Some(section_submissions) = grouped.get(section) else {
            continue;
        };
        if !inserted.insert(section.to_string()) {
            bail!("draft contains duplicate section heading {section:?}");
        }
        for submission in section_submissions {
            out.push(format!(
                "<!-- {} submerge-pr:{} sha={} url={} author={} title={} -->",
                submission.ci_state.emoji(),
                submission.pr,
                submission.head_sha,
                submission.pr_url,
                submission.author,
                marker_comment_value(&submission.pr_title)
            ));
            out.push(submission.line.clone());
        }
        out.push(String::new());
    }

    for section in grouped.keys() {
        if !inserted.contains(*section) {
            bail!("draft is missing community section {section:?}");
        }
    }

    let trailing_newline = if draft_text.ends_with('\n') { "\n" } else { "" };
    Ok(format!("{}{}", out.join("\n"), trailing_newline))
}

fn marker_comment_value(value: &str) -> String {
    value
        .replace("-->", "-- >")
        .replace(['\r', '\n'], " ")
        .trim()
        .to_string()
}

fn parse_edited_buffer(text: &str) -> Result<EditedBuffer> {
    let marker_re = Regex::new(
        r"^<!-- (?:(?P<ci>✅|❌) )?submerge-pr:(?P<pr>\d+) sha=(?P<sha>[0-9a-fA-F]{40}) url=(?P<url>\S+) author=(?P<author>\S+)(?: title=(?P<pr_title>.*?))? -->$",
    )?;
    let mut seen_prs = BTreeSet::new();
    let mut included = Vec::new();
    let mut output = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut idx = 0;
    let mut current_section: Option<String> = None;

    while idx < lines.len() {
        let line = lines[idx];
        if line.trim_start().starts_with("<!--") && line.contains(MARKER_TOKEN) {
            let captures = marker_re
                .captures(line.trim())
                .ok_or_else(|| anyhow!("malformed submerge marker: {line}"))?;
            let pr: u64 = captures["pr"].parse()?;
            if !seen_prs.insert(pr) {
                bail!("duplicate marker for PR #{pr}");
            }

            let mut probe = idx + 1;
            while probe < lines.len() && lines[probe].trim().is_empty() {
                probe += 1;
            }
            if probe >= lines.len() {
                bail!("PR #{pr} marker is not followed by a list item");
            }
            let title = parse_submission_line(lines[probe]).ok_or_else(|| {
                anyhow!(
                    "PR #{pr} marker is followed by an invalid list item: {}",
                    lines[probe]
                )
            })?;
            let section = current_section
                .clone()
                .ok_or_else(|| anyhow!("PR #{pr} marker appears before a section heading"))?;
            included.push(Submission {
                pr,
                title,
                pr_title: captures
                    .name("pr_title")
                    .map(|m| m.as_str())
                    .unwrap_or("")
                    .to_string(),
                author: captures["author"].to_string(),
                pr_url: captures["url"].to_string(),
                head_sha: captures["sha"].to_string(),
                ci_state: match captures.name("ci").map(|m| m.as_str()) {
                    Some("✅") => CiState::Success,
                    _ => CiState::Failure,
                },
                section,
                line: lines[probe].to_string(),
            });
        } else {
            if let Some(section) = line.strip_prefix("### ").map(str::trim) {
                current_section = Some(section.to_string());
            }
            output.push(line.to_string());
        }
        idx += 1;
    }

    let trailing_newline = if text.ends_with('\n') { "\n" } else { "" };
    Ok(EditedBuffer {
        final_text: format!("{}{}", output.join("\n"), trailing_newline),
        included,
    })
}

fn fetch_and_verify_pr_heads(repo: &Repository, submissions: &[Submission]) -> Result<Vec<Oid>> {
    let remote_url = public_git_url()?;
    let mut remote = repo
        .remote_anonymous(&remote_url)
        .with_context(|| format!("open anonymous remote {remote_url}"))?;

    let mut fetch_options = FetchOptions::new();
    if let Ok(git_token) = env::var("GITHUB_TOKEN").or_else(|_| env::var("GH_TOKEN")) {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_url, _username_from_url, _allowed| {
            Cred::userpass_plaintext("x-access-token", &git_token)
        });
        fetch_options.remote_callbacks(callbacks);
    }

    let mut oids = Vec::new();
    for submission in submissions {
        info!("fetching PR #{} from {}", submission.pr, remote_url);
        let local_ref = format!("refs/submerge/pr-{}", submission.pr);
        let refspec = format!("+refs/pull/{}/head:{local_ref}", submission.pr);
        remote
            .fetch(
                &[refspec.as_str()],
                Some(&mut fetch_options),
                Some("submerge fetch"),
            )
            .with_context(|| format!("fetch PR #{}", submission.pr))?;
        let oid = repo
            .refname_to_id(&local_ref)
            .with_context(|| format!("resolve fetched ref {local_ref}"))?;
        if oid.to_string() != submission.head_sha {
            bail!(
                "PR #{} head changed: GitHub reported {}, fetched {}",
                submission.pr,
                submission.head_sha,
                oid
            );
        }
        oids.push(oid);
    }
    Ok(oids)
}

fn public_git_url() -> Result<String> {
    let normalized_repo = DEFAULT_REMOTE
        .trim()
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .trim_end_matches('/');
    let (owner, name) = parse_repo_name(normalized_repo)?;
    Ok(format!("https://github.com/{owner}/{name}.git"))
}

fn create_merge_commit(
    repo: &Repository,
    workdir: &Path,
    draft_rel: &Path,
    final_text: &str,
    included: &[Submission],
    skipped: &[SkippedPr],
    pr_parent_oids: &[Oid],
) -> Result<Oid> {
    let draft_abs = workdir.join(draft_rel);
    fs::write(&draft_abs, final_text).with_context(|| format!("write {}", draft_rel.display()))?;

    let mut index = repo.index().context("open index")?;
    index
        .add_path(draft_rel)
        .with_context(|| format!("add {} to index", draft_rel.display()))?;
    index.write().context("write index")?;
    let tree_oid = index.write_tree().context("write tree")?;
    let tree = repo.find_tree(tree_oid).context("find written tree")?;

    let head = repo
        .head()?
        .peel_to_commit()
        .context("resolve HEAD commit")?;
    let mut parent_commits = Vec::with_capacity(pr_parent_oids.len() + 1);
    parent_commits.push(head);
    for oid in pr_parent_oids {
        parent_commits.push(
            repo.find_commit(*oid)
                .with_context(|| format!("find fetched PR parent {oid}"))?,
        );
    }
    let parent_refs: Vec<&Commit<'_>> = parent_commits.iter().collect();

    let signature = repo
        .signature()
        .or_else(|_| Signature::now("submerge", "submerge@example.invalid"))
        .context("create commit signature")?;
    let message = commit_message(included, skipped);
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &parent_refs,
    )
    .context("create merge commit")
}

fn commit_message(included: &[Submission], skipped: &[SkippedPr]) -> String {
    let mut message = String::from("Submerge TWiR submissions\n\nIncluded PRs:\n");
    for submission in included {
        message.push_str(&format!(
            "- #{} {} ({})\n",
            submission.pr, submission.title, submission.author
        ));
    }
    if !skipped.is_empty() {
        message.push_str("\nSkipped PRs:\n");
        for skipped in skipped {
            message.push_str(&format!("- {}\n", format_skipped_pr_summary(skipped)));
        }
    }
    message
}

fn format_skipped_pr_summary(skipped: &SkippedPr) -> String {
    format!(
        "#{} {} ({}) {}/files",
        skipped.pr,
        skipped.title,
        skipped.reason,
        skipped.url.trim_end_matches('/')
    )
}

fn ensure_tracked_worktree_clean(repo: &Repository) -> Result<()> {
    ensure_tracked_worktree_clean_with_allowed(repo, None)
}

fn ensure_tracked_worktree_clean_except(repo: &Repository, allowed_path: &Path) -> Result<()> {
    ensure_tracked_worktree_clean_with_allowed(repo, Some(allowed_path))
}

fn ensure_tracked_worktree_clean_with_allowed(
    repo: &Repository,
    allowed_path: Option<&Path>,
) -> Result<()> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(false)
        .recurse_untracked_dirs(false)
        .renames_head_to_index(true);
    let statuses = repo.statuses(Some(&mut opts)).context("read git status")?;
    let paths = statuses
        .iter()
        .filter_map(|entry| entry.path().ok().map(str::to_string))
        .filter(|path| allowed_path != Some(Path::new(path)))
        .take(10)
        .collect::<Vec<_>>()
        .join(", ");
    if paths.is_empty() {
        return Ok(());
    }

    bail!("tracked worktree is not clean: {paths}; use --allow-dirty to run anyway");
}

fn find_latest_draft(workdir: &Path) -> Result<PathBuf> {
    let draft_dir = workdir.join("draft");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}-this-week-in-rust\.md$")?;
    let mut files = fs::read_dir(&draft_dir)
        .with_context(|| format!("read {}", draft_dir.display()))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if re.is_match(&name) {
                Some(PathBuf::from("draft").join(name.as_ref()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    files.sort();
    files
        .pop()
        .ok_or_else(|| anyhow!("no draft/YYYY-MM-DD-this-week-in-rust.md files found"))
}

fn normalize_repo_relative_path(workdir: &Path, path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path
            .strip_prefix(workdir)
            .with_context(|| format!("{} is not inside {}", path.display(), workdir.display()))?
            .to_path_buf())
    } else {
        Ok(path.to_path_buf())
    }
}

fn print_summary(submissions: &[Submission], skipped: &[SkippedPr]) {
    println!("eligible submissions: {}", submissions.len());
    for submission in submissions {
        println!(
            "  #{} [{}] {}",
            submission.pr, submission.section, submission.line
        );
    }
    println!("non-eligible PRs: {}", skipped.len());
    for skipped in skipped {
        println!("  {}", format_skipped_pr_summary(skipped));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{IndexAddOption, RepositoryInitOptions};

    fn pull(number: u64) -> PullSummary {
        PullSummary {
            number,
            title: format!("PR {number}"),
            author: "alice".to_string(),
            url: format!("https://github.com/rust-lang/this-week-in-rust/pull/{number}"),
            head_sha: "0123456789abcdef0123456789abcdef01234567".to_string(),
            base_sha: "abcdef0123456789abcdef0123456789abcdef01".to_string(),
            draft: false,
            base_ref: "main".to_string(),
        }
    }

    fn file(patch: &str, additions: u64, deletions: u64) -> DiffEntry {
        serde_json::from_value(serde_json::json!({
            "sha": "0123456789abcdef0123456789abcdef01234567",
            "filename": "draft/2026-06-24-this-week-in-rust.md",
            "status": "modified",
            "additions": additions,
            "deletions": deletions,
            "changes": additions + deletions,
            "blob_url": null,
            "raw_url": null,
            "contents_url": "https://api.github.com/repos/rust-lang/this-week-in-rust/contents/draft/2026-06-24-this-week-in-rust.md",
            "patch": patch,
            "previous_filename": null
        }))
        .unwrap()
    }

    #[test]
    fn classifies_one_link_submission() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Ratatui 0.30.2 is released](https://ratatui.rs/highlights/v0302/)\n\n### Observations/Thoughts\n";
        let submission = classify_pr(
            &pull(1),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.section, "Project/Tooling Updates");
        assert_eq!(submission.title, "Ratatui 0.30.2 is released");
        assert_eq!(
            submission.line,
            "* [Ratatui 0.30.2 is released](https://ratatui.rs/highlights/v0302/)"
        );
    }

    #[test]
    fn allows_one_link_plus_blank_line() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [m-vis 0.4.0-rc1 is released](https://github.com/SickleFire/m-vis)\n\n\n### Observations/Thoughts\n";
        let submission = classify_pr(
            &pull(2),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.pr, 2);
    }

    #[test]
    fn accepts_any_community_subsection() {
        let base = "## Updates from Rust Community\n\n### New Section\n\n";
        let head = "## Updates from Rust Community\n\n### New Section\n* [New item](https://example.com/new)\n";
        let submission = classify_pr(
            &pull(5),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.section, "New Section");
    }

    #[test]
    fn preserves_seeded_submission_line() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n- [New item](https://example.com/new)\n";
        let submission = classify_pr(
            &pull(6),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.line, "- [New item](https://example.com/new)");
    }

    #[test]
    fn accepts_list_item_with_embedded_link_text() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* Rust 1.88 has a nice writeup at [release notes](https://blog.rust-lang.org/)\n";
        let submission = classify_pr(
            &pull(7),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(
            submission.title,
            "Rust 1.88 has a nice writeup at release notes"
        );
        assert_eq!(
            submission.line,
            "* Rust 1.88 has a nice writeup at [release notes](https://blog.rust-lang.org/)"
        );
    }

    #[test]
    fn accepts_list_item_without_link() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head =
            "## Updates from Rust Community\n\n### Project/Tooling Updates\n* Rustaceans announced a thing\n";
        let submission = classify_pr(
            &pull(8),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.title, "Rustaceans announced a thing");
        assert_eq!(submission.line, "* Rustaceans announced a thing");
    }

    #[test]
    fn skips_event_blocks() {
        let base = "## Upcoming Events\n\n* 2026-06-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)\n";
        let head = "## Upcoming Events\n\n* 2026-06-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)\n* 2026-06-25 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse/)\n    * [**Rust Toulouse Meetup - Bevy & ESP32**](https://www.meetup.com/rust-community-toulouse/events/314947457/)\n";
        let err = classify_pr(
            &pull(3),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(err.contains("valid community list item"));
    }

    #[test]
    fn skips_when_no_new_list_item() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com)\n";
        let head = base;
        let err = classify_pr(
            &pull(4),
            &[file("", 0, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(err.contains("valid community list item"));
    }

    #[test]
    fn build_and_parse_edit_buffer() {
        let draft = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let submissions = vec![Submission {
            pr: 42,
            title: "Example".to_string(),
            pr_title: "Add example link".to_string(),
            author: "alice".to_string(),
            pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/42".to_string(),
            head_sha: "0123456789abcdef0123456789abcdef01234567".to_string(),
            ci_state: CiState::Success,
            section: "Project/Tooling Updates".to_string(),
            line: "* [Example](https://example.com/post)".to_string(),
        }];
        let buffer = build_edit_buffer(draft, &submissions).unwrap();
        assert!(buffer.contains("<!-- ✅ submerge-pr:42"));
        assert!(buffer.contains("title=Add example link -->"));

        let parsed = parse_edited_buffer(&buffer).unwrap();
        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].pr, 42);
        assert_eq!(parsed.included[0].title, "Example");
        assert_eq!(parsed.included[0].pr_title, "Add example link");
        assert_eq!(parsed.included[0].ci_state, CiState::Success);
        assert!(!parsed.final_text.contains("submerge-pr:42"));
        assert!(parsed
            .final_text
            .contains("* [Example](https://example.com/post)"));
    }

    #[test]
    fn parse_edit_buffer_rejects_duplicate_markers() {
        let text = "### Project/Tooling Updates\n<!-- ✅ submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 url=https://example.com author=alice -->\n* [Example](https://example.com)\n<!-- ❌ submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 url=https://example.com author=alice -->\n* [Example](https://example.com)\n";
        let err = parse_edited_buffer(text).unwrap_err();
        assert!(err.to_string().contains("duplicate"));
    }

    #[test]
    fn parse_edit_buffer_reconstructs_submission_from_comments() {
        let text = "### Project/Tooling Updates\n<!-- ❌ submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 url=https://github.com/rust-lang/this-week-in-rust/pull/42 author=alice -->\n* [Edited Title](https://example.com/edited)\n";
        let parsed = parse_edited_buffer(text).unwrap();

        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].pr, 42);
        assert_eq!(parsed.included[0].title, "Edited Title");
        assert_eq!(parsed.included[0].pr_title, "");
        assert_eq!(
            parsed.included[0].pr_url,
            "https://github.com/rust-lang/this-week-in-rust/pull/42"
        );
        assert_eq!(parsed.included[0].ci_state, CiState::Failure);
        assert_eq!(parsed.included[0].section, "Project/Tooling Updates");
        assert!(!parsed.final_text.contains("submerge-pr"));
    }

    #[test]
    fn parse_edit_buffer_reads_marker_pr_title() {
        let text = "### Project/Tooling Updates\n<!-- ❌ submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 url=https://github.com/rust-lang/this-week-in-rust/pull/42 author=alice title=Add a helpful link -->\n* [Edited Title](https://example.com/edited)\n";
        let parsed = parse_edited_buffer(text).unwrap();

        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].title, "Edited Title");
        assert_eq!(parsed.included[0].pr_title, "Add a helpful link");
    }

    #[test]
    fn marker_comment_value_does_not_close_comment() {
        assert_eq!(
            marker_comment_value("Title --> with\nnewline"),
            "Title -- > with newline"
        );
    }

    #[test]
    fn parse_edit_buffer_accepts_embedded_or_missing_links() {
        let text = "### Project/Tooling Updates\n<!-- ❌ submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 url=https://github.com/rust-lang/this-week-in-rust/pull/42 author=alice -->\n* Read more in [release notes](https://example.com) today\n<!-- ❌ submerge-pr:43 sha=0123456789abcdef0123456789abcdef01234567 url=https://github.com/rust-lang/this-week-in-rust/pull/43 author=bob -->\n* Plain community announcement\n";
        let parsed = parse_edited_buffer(text).unwrap();

        assert_eq!(parsed.included.len(), 2);
        assert_eq!(parsed.included[0].title, "Read more in release notes today");
        assert_eq!(parsed.included[1].title, "Plain community announcement");
        assert!(parsed
            .final_text
            .contains("* Read more in [release notes](https://example.com) today"));
        assert!(parsed.final_text.contains("* Plain community announcement"));
    }

    #[test]
    fn uses_in_code_public_github_endpoint() {
        let url = public_git_url().unwrap();
        assert_eq!(url, "https://github.com/rust-lang/this-week-in-rust.git");
    }

    #[test]
    fn formats_non_eligible_pr_summary_with_changes_url() {
        let skipped = SkippedPr {
            pr: 1234,
            title: "Not a submission".to_string(),
            reason: "changes 2 files".to_string(),
            url: "https://github.com/rust-lang/this-week-in-rust/pull/1234".to_string(),
        };

        assert_eq!(
            format_skipped_pr_summary(&skipped),
            "#1234 Not a submission (changes 2 files) https://github.com/rust-lang/this-week-in-rust/pull/1234/files"
        );
    }

    #[test]
    fn dirty_worktree_error_mentions_allow_dirty() {
        let dir = tempfile::tempdir().unwrap();
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main");
        let repo = Repository::init_opts(dir.path(), &opts).unwrap();
        fs::write(dir.path().join("tracked.txt"), "clean\n").unwrap();

        let mut index = repo.index().unwrap();
        index.add_path(Path::new("tracked.txt")).unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let sig = Signature::now("Test", "test@example.com").unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "base", &tree, &[])
            .unwrap();
        drop(tree);

        fs::write(dir.path().join("tracked.txt"), "dirty\n").unwrap();
        let err = ensure_tracked_worktree_clean(&repo).unwrap_err();
        assert!(err.to_string().contains("--allow-dirty"));
    }

    #[test]
    fn create_merge_commit_updates_head_and_worktree_clean() {
        let dir = tempfile::tempdir().unwrap();
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main");
        let repo = Repository::init_opts(dir.path(), &opts).unwrap();
        fs::create_dir(dir.path().join("draft")).unwrap();
        fs::write(
            dir.path().join("draft/2026-06-24-this-week-in-rust.md"),
            "old\n",
        )
        .unwrap();

        let mut index = repo.index().unwrap();
        index
            .add_all(["draft"].iter(), IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let sig = Signature::now("Test", "test@example.com").unwrap();
        let base = repo
            .commit(Some("HEAD"), &sig, &sig, "base", &tree, &[])
            .unwrap();

        fs::write(dir.path().join("other.txt"), "parent\n").unwrap();
        let mut index = repo.index().unwrap();
        index
            .add_all(["other.txt"].iter(), IndexAddOption::DEFAULT, None)
            .unwrap();
        index.write().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let base_commit = repo.find_commit(base).unwrap();
        let pr_parent = repo
            .commit(None, &sig, &sig, "pr parent", &tree, &[&base_commit])
            .unwrap();
        drop(tree);
        drop(base_commit);

        repo.set_head_detached(base).unwrap();
        repo.checkout_head(None).unwrap();

        let submission = Submission {
            pr: 7,
            title: "Example".to_string(),
            pr_title: "Add example link".to_string(),
            author: "alice".to_string(),
            pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/7".to_string(),
            head_sha: pr_parent.to_string(),
            ci_state: CiState::Success,
            section: "Project/Tooling Updates".to_string(),
            line: "* [Example](https://example.com/post)".to_string(),
        };
        let oid = create_merge_commit(
            &repo,
            dir.path(),
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            "new\n",
            &[submission],
            &[],
            &[pr_parent],
        )
        .unwrap();
        let commit = repo.find_commit(oid).unwrap();
        assert_eq!(commit.parent_count(), 2);
        assert_eq!(
            fs::read_to_string(dir.path().join("draft/2026-06-24-this-week-in-rust.md")).unwrap(),
            "new\n"
        );
        ensure_tracked_worktree_clean(&repo).unwrap();
    }
}
