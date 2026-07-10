use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand};
use git2::{
    Commit, Cred, FetchOptions, Oid, RemoteCallbacks, Repository, Signature, StatusOptions,
};
use log::{info, warn};
use octocrab::{
    Octocrab,
    models::{checks::ListCheckRuns, pulls::PullRequest, repos::DiffEntry},
    params::State,
    params::repos::Commitish,
};
use pulldown_cmark::{Event, Parser as MarkdownParser, Tag, TagEnd};
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod marker;
mod md;

const GITHUB_OWNER: &str = "rust-lang";
const GITHUB_REPO: &str = "this-week-in-rust";
const PUBLIC_GIT_URL: &str = "https://github.com/rust-lang/this-week-in-rust.git";
const DEFAULT_BASE: &str = "main";

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
    pr_url: String,
    head_sha: String,
    section: String,
    item: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiState {
    Success,
    Failure,
    Unknown,
}

impl CiState {
    fn emoji(self) -> &'static str {
        match self {
            CiState::Success => "✅",
            CiState::Failure => "❌",
            CiState::Unknown => "❓",
        }
    }
}

#[derive(Debug)]
pub struct SkippedPr {
    pub pr: u64,
    pub title: String,
    pub reason: anyhow::Error,
    pub url: String,
}

#[derive(Debug, Clone)]
struct PullSummary {
    number: u64,
    title: String,
    author: String,
    url: String,
    head_sha: String,
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
        ensure_tracked_worktree_clean(&context.repo)?;
    }
    let draft_text = fs::read_to_string(context.workdir.join(&context.draft_rel))
        .with_context(|| format!("read {}", context.draft_rel.display()))?;

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
        {
            Ok(submission) => submissions.push(submission),
            Err(reason) => skipped.push(SkippedPr {
                pr: pull.number,
                title: pull.title,
                reason,
                url: pull.url,
            }),
        }
    }

    submissions.sort_by_key(|(submission, _ci_state)| submission.pr);
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
    base_text: &str,
    pull: &PullSummary,
) -> Result<(Submission, CiState)> {
    if pull.draft {
        bail!("draft PR");
    }

    let files = fetch_pr_files(client, owner, name, pull.number).await?;
    let head_text = fetch_file_text(client, owner, name, draft_rel, &pull.head_sha)
        .await
        .context("could not read head draft")?;
    let submission = classify_pr(pull, &files, draft_rel, base_text, &head_text)?;

    info!("checking CI state for PR #{}", submission.pr);
    let ci_state = fetch_ci_state(client, owner, name, &submission.head_sha).await?;
    Ok((submission, ci_state))
}

fn run_merge(args: MergeArgs) -> Result<()> {
    let context = command_context(args.draft)?;

    if !args.allow_dirty {
        info!("checking tracked worktree status");
        ensure_tracked_worktree_clean_except(&context.repo, &context.draft_rel)?;
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
    let repo = Repository::discover(".").context("open git repository")?;
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

    Ok(PullSummary {
        number,
        title,
        author,
        url,
        head_sha: head.sha,
        draft: pull.draft.unwrap_or(false),
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
        head_sha: pull.head_sha.clone(),
        section: candidate
            .section
            .expect("community list items have sections"),
        item: candidate.item,
    })
}

fn build_edit_buffer(draft_text: &str, submissions: &[(Submission, CiState)]) -> Result<String> {
    let mut grouped: BTreeMap<&str, Vec<(&Submission, CiState)>> = BTreeMap::new();
    for (submission, ci_state) in submissions {
        grouped
            .entry(submission.section.as_str())
            .or_default()
            .push((submission, *ci_state));
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
        for (submission, ci_state) in section_submissions {
            out.push(mark_submission_item(submission, *ci_state));
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

fn mark_submission_item(submission: &Submission, ci_state: CiState) -> String {
    let comment = marker::Attrs::from_submission(submission, ci_state).to_comment();
    match submission.item.find('\n') {
        Some(index) => format!(
            "{} {}{}",
            &submission.item[..index],
            comment,
            &submission.item[index..]
        ),
        None => format!("{} {}", submission.item, comment),
    }
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
                let item_without_marker = marker::remove_comments(
                    item_text,
                    &[(0, local_marker_start, local_marker_end)],
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

fn fetch_and_verify_pr_heads(repo: &Repository, submissions: &[Submission]) -> Result<Vec<Oid>> {
    let mut remote = repo
        .remote_anonymous(PUBLIC_GIT_URL)
        .with_context(|| format!("open anonymous remote {PUBLIC_GIT_URL}"))?;

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
        info!("fetching PR #{} from {}", submission.pr, PUBLIC_GIT_URL);
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

fn create_merge_commit(
    repo: &Repository,
    workdir: &Path,
    draft_rel: &Path,
    final_text: &str,
    included: &[Submission],
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
    let message = commit_message(included);
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
    let mut opts = StatusOptions::new();
    opts.include_untracked(false)
        .recurse_untracked_dirs(false)
        .renames_head_to_index(true);
    let statuses = repo.statuses(Some(&mut opts)).context("read git status")?;
    let paths = statuses
        .iter()
        .filter_map(|entry| entry.path().ok().map(PathBuf::from))
        .collect::<Vec<_>>();
    Ok(paths.into_iter())
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

fn print_summary(submissions: &[(Submission, CiState)], skipped: &[SkippedPr]) {
    println!("eligible submissions: {}", submissions.len());
    for (submission, _ci_state) in submissions {
        println!(
            "  #{} [{}] {}",
            submission.pr, submission.section, submission.item
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
    fn preserves_seeded_submission_item() {
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
        assert_eq!(submission.item, "- [New item](https://example.com/new)");
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
        assert!(err.to_string().contains("multiple blocks"));
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
        assert!(err.to_string().contains("multiple blocks"));
    }

    #[test]
    fn build_and_parse_edit_buffer() {
        let draft = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let submissions = vec![(
            Submission {
                pr: 42,
                pr_title: "Add example link".to_string(),
                author: "alice".to_string(),
                pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/42".to_string(),
                head_sha: "0123456789abcdef0123456789abcdef01234567".to_string(),
                section: "Project/Tooling Updates".to_string(),
                item: "* [Example](https://example.com/post)".to_string(),
            },
            CiState::Success,
        )];
        let buffer = build_edit_buffer(draft, &submissions).unwrap();
        assert!(buffer.contains("* [Example](https://example.com/post) <!-- ✅ url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr:42"));
        assert!(buffer.contains("title=Add example link -->"));

        let parsed = parse_edited_buffer(&buffer).unwrap();
        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].pr, 42);
        assert_eq!(parsed.included[0].pr_title, "Add example link");
        assert!(!parsed.final_text.contains("submerge-pr:42"));
        assert!(
            parsed
                .final_text
                .contains("* [Example](https://example.com/post)")
        );
    }

    #[test]
    fn parse_edit_buffer_rejects_duplicate_markers() {
        let text = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Example](https://example.com) <!-- ✅ url=https://example.com submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 author=alice -->\n* [Example](https://example.com) <!-- ❌ url=https://example.com submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 author=alice -->\n";
        let err = parse_edited_buffer(text).unwrap_err();
        assert!(err.to_string().contains("duplicate"));
    }

    #[test]
    fn parse_edit_buffer_reconstructs_submission_from_comments() {
        let text = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Edited Title](https://example.com/edited) <!-- ❌ url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 author=alice -->\n";
        let parsed = parse_edited_buffer(text).unwrap();

        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].pr, 42);
        assert_eq!(parsed.included[0].pr_title, "");
        assert_eq!(
            parsed.included[0].pr_url,
            "https://github.com/rust-lang/this-week-in-rust/pull/42"
        );
        assert_eq!(parsed.included[0].section, "Project/Tooling Updates");
        assert!(!parsed.final_text.contains("submerge-pr"));
    }

    #[test]
    fn parse_edit_buffer_reads_marker_pr_title() {
        let text = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* [Edited Title](https://example.com/edited) <!-- ❌ url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 author=alice title=Add a helpful link -->\n";
        let parsed = parse_edited_buffer(text).unwrap();

        assert_eq!(parsed.included.len(), 1);
        assert_eq!(parsed.included[0].pr_title, "Add a helpful link");
    }

    #[test]
    fn marker_comment_value_does_not_close_comment() {
        assert_eq!(
            marker::comment_value("Title --> with\nnewline"),
            "Title -- > with newline"
        );
    }

    #[test]
    fn build_edit_buffer_uses_question_mark_for_unknown_ci() {
        let draft = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n";
        let submissions = vec![(
            Submission {
                pr: 42,
                pr_title: "Add example link".to_string(),
                author: "alice".to_string(),
                pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/42".to_string(),
                head_sha: "0123456789abcdef0123456789abcdef01234567".to_string(),
                section: "Project/Tooling Updates".to_string(),
                item: "* [Example](https://example.com/post)".to_string(),
            },
            CiState::Unknown,
        )];

        let buffer = build_edit_buffer(draft, &submissions).unwrap();

        assert!(buffer.contains("* [Example](https://example.com/post) <!-- ❓ url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr:42"));
    }

    #[test]
    fn parse_edit_buffer_accepts_embedded_or_missing_links() {
        let text = "## Updates from Rust Community\n\n### Project/Tooling Updates\n* Read more in [release notes](https://example.com) today <!-- ❌ url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 author=alice -->\n* Plain community announcement <!-- ❌ url=https://github.com/rust-lang/this-week-in-rust/pull/43 submerge-pr:43 sha=0123456789abcdef0123456789abcdef01234567 author=bob -->\n";
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
        let submissions = vec![(
            Submission {
                pr: 42,
                pr_title: "Add example link".to_string(),
                author: "alice".to_string(),
                pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/42".to_string(),
                head_sha: "0123456789abcdef0123456789abcdef01234567".to_string(),
                section: "Project/Tooling Updates".to_string(),
                item: "* [Example](https://example.com/post)\n    * extra detail".to_string(),
            },
            CiState::Success,
        )];
        let buffer = build_edit_buffer(draft, &submissions).unwrap();
        assert!(buffer.contains("* [Example](https://example.com/post) <!-- ✅ url=https://github.com/rust-lang/this-week-in-rust/pull/42 submerge-pr:42"));
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
        assert!(!parsed.final_text.contains("submerge-pr:42"));
    }

    #[test]
    fn formats_non_eligible_pr_summary_with_changes_url() {
        let skipped = SkippedPr {
            pr: 1234,
            title: "Not a submission".to_string(),
            reason: anyhow!("changes 2 files"),
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
            pr_title: "Add example link".to_string(),
            author: "alice".to_string(),
            pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/7".to_string(),
            head_sha: pr_parent.to_string(),
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
        assert_eq!(commit.parent_count(), 2);
        assert_eq!(
            fs::read_to_string(dir.path().join("draft/2026-06-24-this-week-in-rust.md")).unwrap(),
            "new\n"
        );
        ensure_tracked_worktree_clean(&repo).unwrap();
    }
}
