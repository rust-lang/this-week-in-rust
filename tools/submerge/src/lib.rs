use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand};
use gix::{ObjectId, Repository};
use log::{info, warn};
use octocrab::{
    Octocrab,
    models::{checks::ListCheckRuns, pulls::PullRequest, repos::DiffEntry},
    params::State,
    params::repos::Commitish,
};
use pulldown_cmark::{Event, HeadingLevel, Parser as MarkdownParser, Tag, TagEnd};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use url::Url;

mod marker;
mod md;

const GITHUB_OWNER: &str = "rust-lang";
const GITHUB_REPO: &str = "this-week-in-rust";
const PUBLIC_GIT_URL: &str = "https://github.com/rust-lang/this-week-in-rust.git";
const DEFAULT_BASE: &str = "main";
static DRAFT_FILENAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}-this-week-in-rust\.md$").expect("valid draft filename regex")
});

#[derive(Debug, Parser)]
#[command(
    name = "submerge",
    about = "Aggregate one-list-item TWiR submission PRs into a local multi-parent merge commit"
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

    /// Allow tracked local modifications before running.
    #[arg(long)]
    pub allow_dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Submission {
    pr: u64,
    pr_title: String,
    author: String,
    pr_url: Url,
    head_sha: ObjectId,
    section: String,
    item: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiState {
    Success,
    Failure,
    Unknown,
}

#[derive(Debug)]
pub struct SkippedPr {
    pub pr: u64,
    pub title: String,
    pub reason: anyhow::Error,
    pub url: Url,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum ClassifiedSubmission {
    Ignore(anyhow::Error),
    Success(Submission),
}

impl ClassifiedSubmission {
    fn new_ignore(reason: anyhow::Error) -> Self {
        Self::Ignore(reason)
    }
}

#[derive(Debug, Clone)]
struct PullSummary {
    number: u64,
    title: String,
    author: String,
    url: Url,
    base_sha: ObjectId,
    head_sha: ObjectId,
    draft: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditedBuffer {
    pub final_text: String,
    pub included: Vec<Submission>,
}

struct CommandContext {
    repo: Repository,
    workdir: PathBuf,
    /// Path to draft, relative to the git workdir.
    draft_rel: PathBuf,
}

pub async fn run(args: Args) -> Result<()> {
    match args.command {
        CommandArgs::Fetch(args) => run_fetch(args).await,
        CommandArgs::Merge(args) => run_merge(args),
    }
}

async fn run_fetch(args: FetchArgs) -> Result<()> {
    let context = command_context(args.draft)?;
    if !args.allow_dirty {
        info!("checking tracked worktree status");
        ensure_tracked_worktree_clean_with_allowed(&context.repo, None)?;
    }
    let draft_text = fs::read_to_string(context.workdir.join(&context.draft_rel))
        .with_context(|| format!("read {}", context.draft_rel.display()))?;

    let client = github_client()?;
    info!(
        "listing open PRs for {}/{} targeting {}",
        GITHUB_OWNER, GITHUB_REPO, DEFAULT_BASE
    );
    let pulls = fetch_open_pulls(&client, GITHUB_OWNER, GITHUB_REPO, DEFAULT_BASE).await?;
    info!("found {} open PRs", pulls.len());
    let mut submissions = Vec::new();
    let mut skipped = Vec::new();

    for pull in pulls {
        info!("checking PR #{}: {}", pull.number, pull.title);
        match fetch_submission(
            &client,
            GITHUB_OWNER,
            GITHUB_REPO,
            &context.draft_rel,
            &draft_text,
            &pull,
        )
        .await
        .with_context(|| format!("could not fetch PR #{}", pull.number))?
        {
            ClassifiedSubmission::Success(submission) => submissions.push(submission),
            ClassifiedSubmission::Ignore(reason) => skipped.push(SkippedPr {
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

    fs::write(context.workdir.join(&context.draft_rel), buffer)
        .with_context(|| format!("write editable buffer {}", context.draft_rel.display()))?;
    println!("wrote editable buffer to {}", context.draft_rel.display());
    println!(
        "edit {}, run any checks you want, then run: submerge merge",
        context.draft_rel.display()
    );
    Ok(())
}

async fn fetch_submission(
    client: &Octocrab,
    owner: &str,
    name: &str,
    draft_rel: &Path,
    current_text: &str,
    pull: &PullSummary,
) -> Result<ClassifiedSubmission> {
    if pull.draft {
        return Ok(ClassifiedSubmission::new_ignore(anyhow!("draft PR")));
    }

    let files = fetch_pr_files(client, owner, name, pull.number)
        .await
        .with_context(|| format!("could not fetch changed files for PR #{}", pull.number))?;
    // Use the PR's base to determine what it adds even if main has moved since it was opened.
    let base_text = fetch_file_text(client, owner, name, draft_rel, pull.base_sha)
        .await
        .with_context(|| format!("could not fetch base draft for PR #{}", pull.number))?;
    let head_text = fetch_file_text(client, owner, name, draft_rel, pull.head_sha)
        .await
        .with_context(|| format!("could not fetch head draft for PR #{}", pull.number))?;
    let submission = match classify_submission(
        pull,
        &files,
        draft_rel,
        current_text,
        &base_text,
        &head_text,
    )
    .context("submission failed validation")
    {
        Ok(submission) => submission,
        Err(reason) => return Ok(ClassifiedSubmission::new_ignore(reason)),
    };

    info!("checking CI state for PR #{}", submission.pr);
    match fetch_ci_state(client, owner, name, submission.head_sha)
        .await
        .with_context(|| format!("could not fetch CI status for PR #{}", submission.pr))?
    {
        CiState::Success => {}
        CiState::Failure => {
            warn!("skipping PR #{}: CI is failing", submission.pr);
            return Ok(ClassifiedSubmission::new_ignore(anyhow!("CI is failing")));
        }
        CiState::Unknown => {
            warn!(
                "skipping PR #{}: CI status is unknown or pending",
                submission.pr
            );
            return Ok(ClassifiedSubmission::new_ignore(anyhow!(
                "CI status is unknown or pending"
            )));
        }
    }
    Ok(ClassifiedSubmission::Success(submission))
}

fn run_merge(args: MergeArgs) -> Result<()> {
    let context = command_context(args.draft)?;

    if !args.allow_dirty {
        info!("checking tracked worktree status");
        ensure_tracked_worktree_clean_with_allowed(
            &context.repo,
            Some(context.draft_rel.as_path()),
        )?;
    }

    info!("reading edited buffer {}", context.draft_rel.display());
    let edited = fs::read_to_string(context.workdir.join(&context.draft_rel))
        .with_context(|| format!("read edited buffer {}", context.draft_rel.display()))?;
    info!("parsing edited buffer");
    let parsed = parse_edited_buffer(&edited)?;
    if parsed.included.is_empty() {
        bail!("edited buffer did not retain any PR markers");
    }
    info!("retained {} PRs", parsed.included.len());

    info!("fetching retained PR heads");
    let parent_oids = fetch_and_verify_pr_heads(&context.repo, &parsed.included)?;
    info!("creating local merge commit");
    let commit_oid = create_merge_commit(
        &context.repo,
        &context.workdir,
        &context.draft_rel,
        &parsed.final_text,
        &parsed.included,
        &parent_oids,
    )?;

    println!("created local merge commit {commit_oid}");
    Ok(())
}

fn command_context(draft: Option<PathBuf>) -> Result<CommandContext> {
    info!("opening git repository");
    let repo = gix::discover(".").context("open git repository")?;
    let workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("submerge must run in a non-bare repository"))?
        .to_path_buf();
    let draft = match draft {
        Some(path) => path,
        None => find_latest_draft(&workdir)?,
    };
    let draft_rel = normalize_repo_relative_path(&workdir, &draft)?;
    info!("using draft {}", draft_rel.display());
    Ok(CommandContext {
        repo,
        workdir,
        draft_rel,
    })
}

fn github_client() -> Result<Octocrab> {
    let token = env::var("GITHUB_TOKEN")
        .or_else(|_| env::var("GH_TOKEN"))
        .ok();
    let mut builder = Octocrab::builder();
    if let Some(token) = token {
        info!("using authenticated GitHub API client");
        builder = builder.personal_token(token);
    } else {
        warn!(
            "GITHUB_TOKEN/GH_TOKEN is not set; GitHub API requests are unauthenticated and may be rate-limited. \
Create a fine-grained personal access token at https://github.com/settings/personal-access-tokens/new; \
no repository permissions are needed because tokens can read public repositories."
        );
    }
    builder.build().context("build GitHub client")
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
    repo_owner: &str,
    repo_name: &str,
    pr: u64,
) -> Result<Vec<DiffEntry>> {
    let page = client
        .pulls(repo_owner, repo_name)
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
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing html_url"))?;
    let base = pull
        .base
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing base"))?;
    let head = pull
        .head
        .ok_or_else(|| anyhow!("GitHub PR #{number} response is missing head"))?;
    let base_sha = base
        .sha
        .parse()
        .with_context(|| format!("GitHub PR #{number} response has invalid base SHA"))?;
    let head_sha = head
        .sha
        .parse()
        .with_context(|| format!("GitHub PR #{number} response has invalid head SHA"))?;

    Ok(PullSummary {
        number,
        title,
        author,
        url,
        base_sha,
        head_sha,
        draft: pull.draft.unwrap_or(false),
    })
}

async fn fetch_file_text(
    client: &Octocrab,
    owner: &str,
    name: &str,
    path: &Path,
    reference: ObjectId,
) -> Result<String> {
    let mut contents = client
        .repos(owner, name)
        .get_content()
        .path(path.to_string_lossy())
        .r#ref(reference.to_string())
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

async fn fetch_ci_state(
    client: &Octocrab,
    owner: &str,
    name: &str,
    sha: ObjectId,
) -> Result<CiState> {
    let check_runs = client
        .checks(owner, name)
        .list_check_runs_for_git_ref(Commitish(sha.to_string()))
        .per_page(100)
        .send()
        .await
        .with_context(|| format!("list check runs for {sha}"))?;
    Ok(classify_check_runs(&check_runs))
}

fn classify_check_runs(check_runs: &ListCheckRuns) -> CiState {
    if check_runs.total_count == 0 || check_runs.check_runs.is_empty() {
        return CiState::Unknown;
    }

    let mut saw_pending = false;
    for check in &check_runs.check_runs {
        match check.conclusion.as_deref() {
            Some("success" | "skipped" | "neutral") => {}
            Some(_) => return CiState::Failure,
            None => {
                saw_pending = true;
            }
        }
    }

    if saw_pending {
        CiState::Unknown
    } else {
        CiState::Success
    }
}

fn classify_pr(
    pull: &PullSummary,
    files: &[DiffEntry],
    draft_rel: &Path,
    base_text: &str,
    head_text: &str,
) -> Result<Submission> {
    if files.len() != 1 {
        bail!("changes {} files", files.len());
    }

    let file = &files[0];
    if Path::new(&file.filename) != draft_rel {
        bail!("changes {}, not {}", file.filename, draft_rel.display());
    }

    let candidate = md::find_single_added_community_list_item(base_text, head_text)?;
    Ok(Submission {
        pr: pull.number,
        pr_title: pull.title.clone(),
        author: pull.author.clone(),
        pr_url: pull.url.clone(),
        head_sha: pull.head_sha,
        section: candidate
            .section
            .expect("community list items have sections"),
        item: candidate.item,
    })
}

fn classify_submission(
    pull: &PullSummary,
    files: &[DiffEntry],
    draft_rel: &Path,
    current_text: &str,
    base_text: &str,
    head_text: &str,
) -> Result<Submission> {
    let submission = classify_pr(pull, files, draft_rel, base_text, head_text)?;
    if md::contains_list_item(current_text, &submission.section, &submission.item) {
        bail!("submission is already present in current draft");
    }
    Ok(submission)
}

fn build_edit_buffer(draft_text: &str, submissions: &[Submission]) -> Result<String> {
    let mut grouped: BTreeMap<&str, Vec<&Submission>> = BTreeMap::new();
    for submission in submissions {
        grouped
            .entry(submission.section.as_str())
            .or_default()
            .push(submission);
    }

    let mut inserted = BTreeSet::new();
    let mut insertion_points = Vec::new();
    let mut sections = md::SectionTracker::default();

    for (event, range) in MarkdownParser::new(draft_text).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => sections.start_heading(level),
            Event::End(TagEnd::Heading(_)) => {
                let Some((HeadingLevel::H3, section)) = sections.end_heading() else {
                    continue;
                };
                if sections.current_section().as_deref() != Some(section.as_str()) {
                    continue;
                }
                let Some(section_submissions) = grouped.get(section.as_str()) else {
                    continue;
                };
                if !inserted.insert(section.clone()) {
                    bail!("draft contains duplicate section heading {section:?}");
                }
                insertion_points.push((range.end, section_submissions));
            }
            Event::Text(value)
            | Event::Code(value)
            | Event::InlineMath(value)
            | Event::DisplayMath(value)
            | Event::Html(value)
            | Event::InlineHtml(value)
            | Event::FootnoteReference(value) => sections.push_text(&value),
            Event::SoftBreak | Event::HardBreak => sections.push_break(),
            _ => {}
        }
    }

    for section in grouped.keys() {
        if !inserted.contains(*section) {
            bail!("draft is missing community section {section:?}");
        }
    }

    let mut out = draft_text.to_string();
    for (offset, section_submissions) in insertion_points.into_iter().rev() {
        let items = section_submissions
            .iter()
            .map(|submission| mark_submission_item(submission))
            .collect::<Vec<_>>()
            .join("\n");
        out.insert_str(offset, &format!("\n{items}\n"));
    }
    Ok(out)
}

fn mark_submission_item(submission: &Submission) -> String {
    let comment = marker::Attrs::from_submission(submission).to_comment();
    let item = normalize_initial_list_marker(&submission.item);
    match item.find('\n') {
        Some(index) => format!("{} {}{}", &item[..index], comment, &item[index..]),
        None => format!("{item} {comment}"),
    }
}

fn normalize_initial_list_marker(item: &str) -> String {
    let item = item.trim_start();
    let marker_end = item
        .find(char::is_whitespace)
        .expect("submission item has a valid Markdown list marker");
    format!("*{}", &item[marker_end..])
}

#[derive(Default)]
struct EditedItemCapture {
    start: usize,
    section: Option<String>,
    markers: Vec<marker::Capture>,
}

fn parse_edited_buffer(text: &str) -> Result<EditedBuffer> {
    let mut seen_prs = BTreeSet::new();
    let mut included = Vec::new();
    let mut marker_ranges = Vec::new();
    let mut sections = md::SectionTracker::default();
    let mut item_stack: Vec<EditedItemCapture> = Vec::new();

    for (event, range) in MarkdownParser::new(text).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                sections.start_heading(level);
            }
            Event::End(TagEnd::Heading(_)) => {
                sections.end_heading();
            }
            Event::Start(Tag::Item) => {
                item_stack.push(EditedItemCapture {
                    start: range.start,
                    section: sections.current_section(),
                    markers: Vec::new(),
                });
            }
            Event::End(TagEnd::Item) => {
                let Some(item) = item_stack.pop() else {
                    continue;
                };
                if !item_stack.is_empty() || item.markers.is_empty() {
                    continue;
                }
                if item.markers.len() != 1 {
                    bail!("list item contains {} submerge markers", item.markers.len());
                }
                let section = item.section.ok_or_else(|| {
                    anyhow!("submerge marker appears outside a community section")
                })?;
                let marker = &item.markers[0];
                let item_text = text
                    .get(item.start..range.end)
                    .ok_or_else(|| anyhow!("could not read marked list item text"))?
                    .trim_end();
                let local_marker_start = marker.start - item.start;
                let local_marker_end = marker.end - item.start;
                if local_marker_start > item_text.len() || local_marker_end > item_text.len() {
                    bail!("submerge marker has invalid offsets");
                }
                let item_without_marker = format!(
                    "{}{}",
                    item_text[..local_marker_start].trim_end(),
                    &item_text[local_marker_end..]
                )
                .trim_end()
                .to_string();

                let marker_attrs = marker::Attrs::parse(&marker.text)?;
                if !seen_prs.insert(marker_attrs.pr) {
                    bail!("duplicate marker for PR #{}", marker_attrs.pr);
                }
                if !md::is_single_list_item(&item_without_marker) {
                    bail!(
                        "PR #{} marker is attached to an invalid list item: {}",
                        marker_attrs.pr,
                        item_without_marker
                    );
                }
                included.push(Submission {
                    pr: marker_attrs.pr,
                    pr_title: marker_attrs.pr_title,
                    author: marker_attrs.author,
                    pr_url: marker_attrs.url,
                    head_sha: marker_attrs.sha,
                    section,
                    item: item_without_marker,
                });
                marker_ranges.push((item.start, marker.start, marker.end));
            }
            Event::Text(value)
            | Event::Code(value)
            | Event::InlineMath(value)
            | Event::DisplayMath(value)
            | Event::FootnoteReference(value) => {
                sections.push_text(&value);
            }
            Event::Html(value) | Event::InlineHtml(value) => {
                if let Some(item) = item_stack.last_mut() {
                    if value.contains(marker::TOKEN) {
                        item.markers.push(marker::Capture {
                            start: range.start,
                            end: range.end,
                            text: value.to_string(),
                        });
                    }
                } else if value.contains(marker::TOKEN) {
                    bail!("submerge marker appears outside a list item: {value}");
                }
                sections.push_text(&value);
            }
            Event::SoftBreak | Event::HardBreak => {
                sections.push_break();
            }
            _ => {}
        }
    }

    Ok(EditedBuffer {
        final_text: marker::remove_comments(text, &marker_ranges),
        included,
    })
}

fn fetch_and_verify_pr_heads(
    repo: &Repository,
    submissions: &[Submission],
) -> Result<Vec<ObjectId>> {
    let mut remote = repo
        .remote_at(PUBLIC_GIT_URL)
        .with_context(|| format!("open anonymous remote {PUBLIC_GIT_URL}"))?;
    let refspecs = submissions
        .iter()
        .map(|submission| {
            format!(
                "+refs/pull/{}/head:refs/submerge/pr-{}",
                submission.pr, submission.pr
            )
        })
        .collect::<Vec<_>>();
    remote
        .replace_refspecs(
            refspecs.iter().map(String::as_str),
            gix::remote::Direction::Fetch,
        )
        .context("configure PR head fetch")?;
    info!(
        "fetching {} PR heads from {}",
        submissions.len(),
        PUBLIC_GIT_URL
    );
    let mut progress = gix::progress::Discard;
    remote
        .connect(gix::remote::Direction::Fetch)
        .context("connect to fetch PR heads")?
        .prepare_fetch(&mut progress, Default::default())
        .context("prepare PR head fetch")?
        .receive(&mut progress, &std::sync::atomic::AtomicBool::new(false))
        .context("fetch PR heads")?;

    let mut oids = Vec::new();
    for submission in submissions {
        let local_ref = format!("refs/submerge/pr-{}", submission.pr);
        let oid = repo
            .find_reference(&local_ref)
            .with_context(|| format!("resolve fetched ref {local_ref}"))?
            .id()
            .detach();
        if oid != submission.head_sha {
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

fn create_merge_commit(
    repo: &Repository,
    workdir: &Path,
    draft_rel: &Path,
    final_text: &str,
    included: &[Submission],
    pr_parent_oids: &[ObjectId],
) -> Result<ObjectId> {
    let draft_abs = workdir.join(draft_rel);
    fs::write(&draft_abs, final_text).with_context(|| format!("write {}", draft_rel.display()))?;

    let head_oid = repo.head_id().context("resolve HEAD commit")?.detach();
    let head_tree_oid = repo.head_tree_id().context("resolve HEAD tree")?.detach();
    let blob_oid = repo
        .write_blob(final_text.as_bytes())
        .context("write draft blob")?
        .detach();
    let draft_rel = draft_rel
        .to_str()
        .ok_or_else(|| anyhow!("draft path is not valid UTF-8: {}", draft_rel.display()))?;
    let mut tree_editor = repo.edit_tree(head_tree_oid).context("edit HEAD tree")?;
    tree_editor
        .upsert(draft_rel, gix::object::tree::EntryKind::Blob, blob_oid)
        .with_context(|| format!("add {draft_rel} to tree"))?;
    let tree_oid = tree_editor.write().context("write tree")?.detach();

    let mut index = repo
        .index_from_tree(tree_oid.as_ref())
        .context("create index from merge tree")?;
    index
        .write(gix::index::write::Options::default())
        .context("write index")?;

    let mut parents = Vec::with_capacity(pr_parent_oids.len() + 1);
    parents.push(head_oid);
    for oid in pr_parent_oids {
        repo.find_commit(*oid)
            .with_context(|| format!("find fetched PR parent {oid}"))?;
        parents.push(*oid);
    }

    let signature = commit_signature(repo)?;
    let mut committer_time = gix::date::parse::TimeBuf::default();
    let mut author_time = gix::date::parse::TimeBuf::default();
    let message = commit_message(included);
    repo.commit_as(
        signature.to_ref(&mut committer_time),
        signature.to_ref(&mut author_time),
        "HEAD",
        &message,
        tree_oid,
        parents,
    )
    .context("create merge commit")
    .map(|id| id.detach())
}

fn commit_signature(repo: &Repository) -> Result<gix::actor::Signature> {
    match repo.committer() {
        Some(signature) => signature?
            .to_owned()
            .context("read configured commit signature"),
        None => Ok(gix::actor::Signature {
            name: "submerge".into(),
            email: "submerge@example.invalid".into(),
            time: gix::date::Time::now_local_or_utc(),
        }),
    }
}

fn commit_message(included: &[Submission]) -> String {
    let mut message = String::from("Submerge TWiR submissions\n\nIncluded PRs:\n");
    for submission in included {
        message.push_str(&format!(
            "- #{} {} ({})\n",
            submission.pr,
            submission_item_summary(submission),
            submission.author
        ));
    }
    message
}

fn submission_item_summary(submission: &Submission) -> &str {
    submission.item.lines().next().unwrap_or("").trim()
}

fn format_skipped_pr_summary(skipped: &SkippedPr) -> String {
    format!(
        "#{} {} {}/files:\n{:?}\n\n",
        skipped.pr,
        skipped.title,
        skipped.url.as_str().trim_end_matches('/'),
        skipped.reason,
    )
}

fn ensure_tracked_worktree_clean_with_allowed(
    repo: &Repository,
    allowed_path: Option<&Path>,
) -> Result<()> {
    let paths = dirty_paths(repo)?
        .filter(|path| allowed_path != Some(path.as_path()))
        .take(10)
        .collect::<Vec<_>>();
    if paths.is_empty() {
        return Ok(());
    }

    let paths = paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    bail!("tracked worktree is not clean: {paths}; use --allow-dirty to run anyway");
}

fn dirty_paths(repo: &Repository) -> Result<std::vec::IntoIter<PathBuf>> {
    let statuses = repo
        .status(gix::progress::Discard)
        .context("prepare git status")?
        .untracked_files(gix::status::UntrackedFiles::None)
        .into_iter(Vec::<gix::bstr::BString>::new())
        .context("read git status")?;
    let paths = statuses
        .map(|entry| {
            let entry = entry.context("read git status entry")?;
            Ok(gix::path::from_bstr(entry.location()).into_owned())
        })
        .collect::<Result<BTreeSet<_>>>()?;
    Ok(paths.into_iter().collect::<Vec<_>>().into_iter())
}

fn find_latest_draft(workdir: &Path) -> Result<PathBuf> {
    let draft_dir = workdir.join("draft");
    let mut files = fs::read_dir(&draft_dir)
        .with_context(|| format!("read {}", draft_dir.display()))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if DRAFT_FILENAME_RE.is_match(&name) {
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
    println!("---");
    println!("eligible submissions: {}", submissions.len());
    for submission in submissions {
        println!(
            "#{} [{}] {}",
            submission.pr, submission.section, submission.item
        );
    }
    println!("---");
    println!("non-eligible PRs: {}", skipped.len());
    for skipped in skipped {
        println!("{}", format_skipped_pr_summary(skipped));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    fn test_signature() -> gix::actor::Signature {
        gix::actor::Signature {
            name: "Test".into(),
            email: "test@example.com".into(),
            time: gix::date::Time::now_utc(),
        }
    }

    fn tree_with_file(
        repo: &Repository,
        base_tree: ObjectId,
        path: &str,
        content: &str,
    ) -> ObjectId {
        let blob = repo.write_blob(content.as_bytes()).unwrap().detach();
        let mut editor = repo.edit_tree(base_tree).unwrap();
        editor
            .upsert(path, gix::object::tree::EntryKind::Blob, blob)
            .unwrap();
        editor.write().unwrap().detach()
    }

    fn write_index(repo: &Repository, tree: ObjectId) {
        let mut index = repo.index_from_tree(tree.as_ref()).unwrap();
        index.write(gix::index::write::Options::default()).unwrap();
    }

    fn commit_to_head(
        repo: &Repository,
        message: &str,
        tree: ObjectId,
        parents: impl IntoIterator<Item = ObjectId>,
    ) -> ObjectId {
        let signature = test_signature();
        let mut committer_time = gix::date::parse::TimeBuf::default();
        let mut author_time = gix::date::parse::TimeBuf::default();
        repo.commit_as(
            signature.to_ref(&mut committer_time),
            signature.to_ref(&mut author_time),
            "HEAD",
            message,
            tree,
            parents,
        )
        .unwrap()
        .detach()
    }

    fn new_commit(
        repo: &Repository,
        message: &str,
        tree: ObjectId,
        parents: impl IntoIterator<Item = ObjectId>,
    ) -> ObjectId {
        let signature = test_signature();
        let mut committer_time = gix::date::parse::TimeBuf::default();
        let mut author_time = gix::date::parse::TimeBuf::default();
        repo.new_commit_as(
            signature.to_ref(&mut committer_time),
            signature.to_ref(&mut author_time),
            message,
            tree,
            parents,
        )
        .unwrap()
        .id()
        .detach()
    }

    fn pull(number: u64) -> PullSummary {
        PullSummary {
            number,
            title: format!("PR {number}"),
            author: "alice".to_string(),
            url: Url::parse(&format!(
                "https://github.com/rust-lang/this-week-in-rust/pull/{number}"
            ))
            .unwrap(),
            base_sha: "fedcba9876543210fedcba9876543210fedcba98".parse().unwrap(),
            head_sha: "0123456789abcdef0123456789abcdef01234567".parse().unwrap(),
            draft: false,
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

    fn check_runs(value: serde_json::Value) -> ListCheckRuns {
        serde_json::from_value(value).unwrap()
    }

    #[tokio::test]
    async fn github_client_builds() {
        github_client().unwrap();
    }

    fn check_run(conclusion: Option<&str>) -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "node_id": "CR_kwDOAZBsV88AAAA",
            "details_url": null,
            "head_sha": "0123456789abcdef0123456789abcdef01234567",
            "url": "https://api.github.com/repos/rust-lang/this-week-in-rust/check-runs/1",
            "html_url": null,
            "conclusion": conclusion,
            "output": {
                "title": null,
                "summary": null,
                "text": null,
                "annotations_count": 0,
                "annotations_url": "https://api.github.com/repos/rust-lang/this-week-in-rust/check-runs/1/annotations"
            },
            "started_at": null,
            "completed_at": null,
            "name": "Run Markdown Checks",
            "pull_requests": []
        })
    }

    #[test]
    fn ci_state_is_unknown_without_check_runs() {
        let check_runs = check_runs(serde_json::json!({
            "total_count": 0,
            "check_runs": []
        }));

        assert_eq!(classify_check_runs(&check_runs), CiState::Unknown);
    }

    #[test]
    fn ci_state_is_unknown_with_pending_check_run() {
        let check_runs = check_runs(serde_json::json!({
            "total_count": 1,
            "check_runs": [check_run(None)]
        }));

        assert_eq!(classify_check_runs(&check_runs), CiState::Unknown);
    }

    #[test]
    fn ci_state_failure_wins_over_pending_check_run() {
        let check_runs = check_runs(serde_json::json!({
            "total_count": 2,
            "check_runs": [
                check_run(None),
                check_run(Some("failure"))
            ]
        }));

        assert_eq!(classify_check_runs(&check_runs), CiState::Failure);
    }

    #[test]
    fn ci_state_success_with_successful_check_run() {
        let check_runs = check_runs(serde_json::json!({
            "total_count": 1,
            "check_runs": [check_run(Some("success"))]
        }));

        assert_eq!(classify_check_runs(&check_runs), CiState::Success);
    }

    #[test]
    fn classified_submission_ignore_keeps_reason() {
        match ClassifiedSubmission::new_ignore(anyhow!("draft PR")) {
            ClassifiedSubmission::Ignore(reason) => assert_eq!(reason.to_string(), "draft PR"),
            ClassifiedSubmission::Success(_) => panic!("expected ignored submission"),
        }
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
        assert_eq!(
            submission.item,
            "* [Ratatui 0.30.2 is released](https://ratatui.rs/highlights/v0302/)"
        );
    }

    #[test]
    fn accepts_item_added_after_existing_item() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com/existing)\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com/existing)\n* [New](https://example.com/new)\n\n### Observations/Thoughts\n";
        let submission = classify_pr(
            &pull(20),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();

        assert_eq!(submission.item, "* [New](https://example.com/new)");
    }

    #[test]
    fn accepts_item_added_before_existing_item() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com/existing)\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New](https://example.com/new)\n* [Existing](https://example.com/existing)\n\n### Observations/Thoughts\n";
        let submission = classify_pr(
            &pull(21),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();

        assert_eq!(submission.item, "* [New](https://example.com/new)");
    }

    #[test]
    fn accepts_pr_based_before_locally_merged_submissions() {
        let pr_base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let current = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Already merged](https://example.com/existing)\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New submission](https://example.com/new)\n\n### Observations/Thoughts\n";
        let submission = classify_submission(
            &pull(8380),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            current,
            pr_base,
            head,
        )
        .unwrap();

        let buffer = build_edit_buffer(current, &[submission]).unwrap();
        assert!(buffer.contains("[Already merged](https://example.com/existing)"));
        assert!(buffer.contains("[New submission](https://example.com/new)"));
    }

    #[test]
    fn skips_submission_already_merged_into_current_draft() {
        let pr_base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let current = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Already merged](https://example.com/existing)\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Already merged](https://example.com/existing)\n\n### Observations/Thoughts\n";
        let err = classify_submission(
            &pull(8372),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            current,
            pr_base,
            head,
        )
        .unwrap_err();

        assert_eq!(
            err.to_string(),
            "submission is already present in current draft"
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
    fn accepts_first_item_in_empty_section() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New item](https://example.com/new)\n";
        let submission = classify_pr(
            &pull(17),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.item, "* [New item](https://example.com/new)");
    }

    #[test]
    fn ignores_shifted_ranges_for_unchanged_following_content() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\nExisting paragraph after the insertion point.\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New item](https://example.com/new)\n\nExisting paragraph after the insertion point.\n";
        let submission = classify_pr(
            &pull(18),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.section, "Project/Tooling Updates");
        assert_eq!(submission.item, "* [New item](https://example.com/new)");
    }

    #[test]
    fn normalizes_indented_submission_item_in_edit_buffer() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n  - [New item](https://example.com/new)\n";
        let submission = classify_pr(
            &pull(6),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.item, "- [New item](https://example.com/new)");

        let buffer = build_edit_buffer(base, &[submission]).unwrap();
        assert!(buffer.contains("\n* [New item](https://example.com/new) <!--"));
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
            submission.item,
            "* Rust 1.88 has a nice writeup at [release notes](https://blog.rust-lang.org/)"
        );
    }

    #[test]
    fn accepts_list_item_without_link() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* Rustaceans announced a thing\n";
        let submission = classify_pr(
            &pull(8),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.item, "* Rustaceans announced a thing");
    }

    #[test]
    fn classifies_single_multiline_list_item() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New item](https://example.com/new)\n    * extra detail\n";
        let submission = classify_pr(
            &pull(9),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(
            submission.item,
            "* [New item](https://example.com/new)\n    * extra detail"
        );
    }

    #[test]
    fn allows_whitespace_lines_around_one_added_item() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n* [New item](https://example.com/new)\n   \n\n### Observations/Thoughts\n";
        let submission = classify_pr(
            &pull(13),
            &[file("", 3, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap();
        assert_eq!(submission.item, "* [New item](https://example.com/new)");
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
        assert!(err.to_string().contains("valid community list item"));
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
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn rejects_removed_existing_item_even_with_one_addition() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com/old)\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New](https://example.com/new)\n";
        let err = classify_pr(
            &pull(10),
            &[file("", 1, 1)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn rejects_added_section_with_one_item() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n";
        let head = "## Updates from Rust Community\n\n### New Section\n* [New](https://example.com/new)\n\n### Project/Tooling Updates\n";
        let err = classify_pr(
            &pull(11),
            &[file("", 3, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn rejects_existing_item_edit_plus_one_addition() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com/old)\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing changed](https://example.com/old)\n* [New](https://example.com/new)\n";
        let err = classify_pr(
            &pull(12),
            &[file("", 2, 1)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn rejects_nested_addition_to_existing_item() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com/old)\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Existing](https://example.com/old)\n    * extra detail\n";
        let err = classify_pr(
            &pull(14),
            &[file("", 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn rejects_split_non_whitespace_additions() {
        let base = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New](https://example.com/new)\n\n### Observations/Thoughts\n* [Other](https://example.com/other)\n";
        let err = classify_pr(
            &pull(15),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(err.to_string().contains("multiple items"));
    }

    #[test]
    fn rejects_unrelated_content_with_one_valid_item() {
        let base =
            "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n## Upcoming Events\n";
        let head = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [New](https://example.com/new)\n\n## Upcoming Events\nRandom event note\n";
        let err = classify_pr(
            &pull(16),
            &[file("", 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            base,
            head,
        )
        .unwrap_err();
        assert!(err.to_string().contains("unexpected"));
    }

    #[test]
    fn build_and_parse_edit_buffer() {
        let draft = "## Updates from Rust Community\n\n  ### Project/Tooling Updates ###\n\n### Observations/Thoughts\n";
        let submissions = vec![Submission {
            pr: 42,
            pr_title: "Add example link".to_string(),
            author: "alice".to_string(),
            pr_url: Url::parse("https://github.com/rust-lang/this-week-in-rust/pull/42").unwrap(),
            head_sha: "0123456789abcdef0123456789abcdef01234567".parse().unwrap(),
            section: "Project/Tooling Updates".to_string(),
            item: "* [Example](https://example.com/post)".to_string(),
        }];
        let buffer = build_edit_buffer(draft, &submissions).unwrap();
        assert!(buffer.contains("* [Example](https://example.com/post) <!-- url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr=42"));
        assert!(buffer.contains("title=Add example link -->"));

        let parsed = parse_edited_buffer(&buffer).unwrap();
        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].pr, 42);
        assert_eq!(parsed.included[0].pr_title, "Add example link");
        assert!(!parsed.final_text.contains("submerge-pr=42"));
        assert!(
            parsed
                .final_text
                .contains("* [Example](https://example.com/post)")
        );
    }

    #[test]
    fn parse_edit_buffer_rejects_duplicate_markers() {
        let text = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Example](https://example.com) <!-- url=https://example.com submerge-pr=42 sha=0123456789abcdef0123456789abcdef01234567 author=alice title=First -->\n* [Example](https://example.com) <!-- url=https://example.com submerge-pr=42 sha=0123456789abcdef0123456789abcdef01234567 author=alice title=Second -->\n";
        let err = parse_edited_buffer(text).unwrap_err();
        assert!(err.to_string().contains("duplicate"));
    }

    #[test]
    fn parse_edit_buffer_reads_marker_pr_title() {
        let text = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Edited Title](https://example.com/edited) <!-- url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr=42 sha=0123456789abcdef0123456789abcdef01234567 author=alice title=Add a helpful link -->\n";
        let parsed = parse_edited_buffer(text).unwrap();

        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].pr_title, "Add a helpful link");
    }

    #[test]
    fn marker_comment_value_does_not_close_comment() {
        assert_eq!(
            marker::comment_escape("Title --> with\nnewline"),
            "Title -- > with newline"
        );
    }

    #[test]
    fn parse_edit_buffer_accepts_embedded_or_missing_links() {
        let text = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* Read more in [release notes](https://example.com) today <!-- url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr=42 sha=0123456789abcdef0123456789abcdef01234567 author=alice title=Linked item -->\n* Plain community announcement <!-- url=https://github.com/rust-lang/this-week-in-rust/pull/43 submerge-pr=43 sha=0123456789abcdef0123456789abcdef01234567 author=bob title=Plain item -->\n";
        let parsed = parse_edited_buffer(text).unwrap();

        assert_eq!(parsed.included.len(), 2);
        assert!(
            parsed
                .final_text
                .contains("* Read more in [release notes](https://example.com) today")
        );
        assert!(parsed.final_text.contains("* Plain community announcement"));
    }

    #[test]
    fn build_and_parse_edit_buffer_with_multiline_item() {
        let draft = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n";
        let submissions = vec![Submission {
            pr: 42,
            pr_title: "Add example link".to_string(),
            author: "alice".to_string(),
            pr_url: Url::parse("https://github.com/rust-lang/this-week-in-rust/pull/42").unwrap(),
            head_sha: "0123456789abcdef0123456789abcdef01234567".parse().unwrap(),
            section: "Project/Tooling Updates".to_string(),
            item: "- [Example](https://example.com/post)\n    * extra detail".to_string(),
        }];
        let buffer = build_edit_buffer(draft, &submissions).unwrap();
        assert!(buffer.contains("* [Example](https://example.com/post) <!-- url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr=42"));
        assert!(buffer.contains("    * extra detail"));

        let parsed = parse_edited_buffer(&buffer).unwrap();
        assert_eq!(parsed.included.len(), 1);
        assert_eq!(
            parsed.included[0].item,
            "* [Example](https://example.com/post)\n    * extra detail"
        );
        assert!(
            parsed
                .final_text
                .contains("* [Example](https://example.com/post)\n    * extra detail")
        );
        assert!(!parsed.final_text.contains("submerge-pr=42"));
    }

    #[test]
    fn normalizes_initial_list_markers() {
        assert_eq!(normalize_initial_list_marker("- item"), "* item");
        assert_eq!(normalize_initial_list_marker("+ item"), "* item");
        assert_eq!(normalize_initial_list_marker("1. item"), "* item");
        assert_eq!(normalize_initial_list_marker("   - item"), "* item");
        assert_eq!(
            normalize_initial_list_marker("- item\n    + nested detail"),
            "* item\n    + nested detail"
        );
    }

    #[test]
    fn formats_non_eligible_pr_summary_with_changes_url() {
        let skipped = SkippedPr {
            pr: 1234,
            title: "Not a submission".to_string(),
            reason: anyhow!("changes 2 files"),
            url: Url::parse("https://github.com/rust-lang/this-week-in-rust/pull/1234").unwrap(),
        };

        assert_eq!(
            format_skipped_pr_summary(&skipped),
            "#1234 Not a submission https://github.com/rust-lang/this-week-in-rust/pull/1234/files:\nchanges 2 files\n\n"
        );
    }

    #[test]
    fn formats_skipped_pr_summary_with_error_context() {
        let skipped = SkippedPr {
            pr: 1234,
            title: "Not a submission".to_string(),
            reason: anyhow!("changes 2 files").context("submission failed validation"),
            url: Url::parse("https://github.com/rust-lang/this-week-in-rust/pull/1234").unwrap(),
        };

        assert_eq!(
            format_skipped_pr_summary(&skipped),
            "#1234 Not a submission https://github.com/rust-lang/this-week-in-rust/pull/1234/files:\nsubmission failed validation\n\nCaused by:\n    changes 2 files\n\n"
        );
    }

    #[test]
    fn dirty_worktree_error_mentions_allow_dirty() {
        let dir = tempfile::tempdir().unwrap();
        let repo = gix::init(dir.path()).unwrap();
        fs::write(dir.path().join("tracked.txt"), "clean\n").unwrap();
        let tree = tree_with_file(&repo, repo.empty_tree().id, "tracked.txt", "clean\n");
        write_index(&repo, tree);
        commit_to_head(&repo, "base", tree, []);

        fs::write(dir.path().join("tracked.txt"), "dirty\n").unwrap();
        let err = ensure_tracked_worktree_clean_with_allowed(&repo, None).unwrap_err();
        assert!(err.to_string().contains("--allow-dirty"));
    }

    #[test]
    fn create_merge_commit_updates_head_and_worktree_clean() {
        let dir = tempfile::tempdir().unwrap();
        let repo = gix::init(dir.path()).unwrap();
        fs::create_dir(dir.path().join("draft")).unwrap();
        fs::write(
            dir.path().join("draft/2026-06-24-this-week-in-rust.md"),
            "old\n",
        )
        .unwrap();
        let base_tree = tree_with_file(
            &repo,
            repo.empty_tree().id,
            "draft/2026-06-24-this-week-in-rust.md",
            "old\n",
        );
        write_index(&repo, base_tree);
        let base = commit_to_head(&repo, "base", base_tree, []);
        let pr_tree = tree_with_file(&repo, base_tree, "other.txt", "parent\n");
        let pr_parent = new_commit(&repo, "pr parent", pr_tree, [base]);

        let submission = Submission {
            pr: 7,
            pr_title: "Add example link".to_string(),
            author: "alice".to_string(),
            pr_url: Url::parse("https://github.com/rust-lang/this-week-in-rust/pull/7").unwrap(),
            head_sha: pr_parent,
            section: "Project/Tooling Updates".to_string(),
            item: "* [Example](https://example.com/post)".to_string(),
        };
        let oid = create_merge_commit(
            &repo,
            dir.path(),
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
            "new\n",
            &[submission],
            &[pr_parent],
        )
        .unwrap();
        let commit = repo.find_commit(oid).unwrap();
        assert_eq!(commit.parent_ids().count(), 2);
        assert_eq!(
            fs::read_to_string(dir.path().join("draft/2026-06-24-this-week-in-rust.md")).unwrap(),
            "new\n"
        );
        ensure_tracked_worktree_clean_with_allowed(&repo, None).unwrap();
    }
}
