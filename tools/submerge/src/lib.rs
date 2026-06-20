use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use git2::{Commit, FetchOptions, Oid, Repository, Signature, StatusOptions};
use octocrab::Octocrab;
use regex::Regex;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::Builder;

const DEFAULT_REMOTE: &str = "rust-lang/this-week-in-rust";
const DEFAULT_BASE: &str = "main";
const COMMUNITY_HEADING: &str = "## Updates from Rust Community";
const MARKER_PREFIX: &str = "<!-- submerge-pr:";

#[derive(Debug, Parser)]
#[command(
    name = "submerge",
    about = "Aggregate one-link TWiR submission PRs into a local multi-parent merge commit"
)]
pub struct Args {
    /// GitHub repository to inspect, as owner/name.
    #[arg(long, default_value = DEFAULT_REMOTE)]
    pub repo: String,

    /// Base branch to inspect.
    #[arg(long, default_value = DEFAULT_BASE)]
    pub base: String,

    /// Draft file to update. Defaults to the latest draft/YYYY-MM-DD-this-week-in-rust.md.
    #[arg(long)]
    pub draft: Option<PathBuf>,

    /// Editor command. Defaults to $EDITOR.
    #[arg(long)]
    pub editor: Option<String>,

    /// Allow tracked local modifications before running.
    #[arg(long)]
    pub allow_dirty: bool,

    /// Build the editable merge buffer and print a summary without opening an editor or committing.
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Submission {
    pub pr: u64,
    pub title: String,
    pub author: String,
    pub pr_url: String,
    pub url: String,
    pub head_sha: String,
    pub section: String,
    pub line: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedPr {
    pub pr: u64,
    pub title: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
struct PullSummary {
    number: u64,
    title: String,
    author: String,
    url: String,
    head_sha: String,
    draft: bool,
    base_ref: String,
}

#[derive(Debug, Clone)]
struct ChangedFile {
    filename: String,
    additions: u64,
    deletions: u64,
    patch: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiPull {
    number: u64,
    title: String,
    html_url: String,
    draft: Option<bool>,
    user: ApiUser,
    head: ApiBranch,
    base: ApiBranch,
}

#[derive(Debug, Deserialize)]
struct ApiUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct ApiBranch {
    #[serde(rename = "ref")]
    ref_name: String,
    sha: String,
}

#[derive(Debug, Deserialize)]
struct ApiFile {
    filename: String,
    additions: u64,
    deletions: u64,
    patch: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditedBuffer {
    pub final_text: String,
    pub included_prs: BTreeSet<u64>,
}

pub async fn run(args: Args) -> Result<()> {
    let repo = Repository::discover(".").context("open git repository")?;
    if !args.allow_dirty {
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
    let draft_text = fs::read_to_string(workdir.join(&draft_rel))
        .with_context(|| format!("read {}", draft_rel.display()))?;

    let (owner, name) = parse_repo_name(&args.repo)?;
    let client = github_client()?;
    let pulls = fetch_open_pulls(&client, owner, name, &args.base).await?;
    let mut submissions = Vec::new();
    let mut skipped = Vec::new();

    for pull in pulls {
        if pull.draft {
            skipped.push(SkippedPr {
                pr: pull.number,
                title: pull.title,
                reason: "draft PR".to_string(),
            });
            continue;
        }
        if pull.base_ref != args.base {
            skipped.push(SkippedPr {
                pr: pull.number,
                title: pull.title,
                reason: format!("targets {}, not {}", pull.base_ref, args.base),
            });
            continue;
        }

        let files = fetch_pr_files(&client, owner, name, pull.number).await?;
        match classify_pr(&pull, &files, &draft_rel) {
            Ok(submission) => submissions.push(submission),
            Err(reason) => skipped.push(SkippedPr {
                pr: pull.number,
                title: pull.title,
                reason,
            }),
        }
    }

    submissions.sort_by_key(|submission| submission.pr);
    print_summary(&submissions, &skipped);

    if submissions.is_empty() {
        bail!("no eligible one-link community submissions found");
    }

    let buffer = build_edit_buffer(&draft_text, &submissions)?;
    if args.dry_run {
        println!("\n--- editable buffer preview ---\n{buffer}");
        return Ok(());
    }

    let edited = edit_buffer(&buffer, args.editor.as_deref())?;
    let parsed = parse_edited_buffer(&edited)?;
    if parsed.included_prs.is_empty() {
        bail!("edited buffer did not retain any PR markers");
    }

    let submission_by_pr: HashMap<u64, Submission> = submissions
        .iter()
        .map(|submission| (submission.pr, submission.clone()))
        .collect();
    let included: Vec<Submission> = parsed
        .included_prs
        .iter()
        .map(|pr| {
            submission_by_pr
                .get(pr)
                .cloned()
                .ok_or_else(|| anyhow!("edited buffer references unknown PR #{pr}"))
        })
        .collect::<Result<_>>()?;

    let parent_oids = fetch_and_verify_pr_heads(&repo, &args.repo, &included)?;
    let commit_oid = create_merge_commit(
        &repo,
        workdir,
        &draft_rel,
        &parsed.final_text,
        &included,
        &skipped,
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

fn github_client() -> Result<Octocrab> {
    let token = env::var("GITHUB_TOKEN")
        .or_else(|_| env::var("GH_TOKEN"))
        .ok();
    let mut builder = Octocrab::builder();
    if let Some(token) = token {
        builder = builder.personal_token(token);
    }
    builder.build().context("build GitHub client")
}

async fn fetch_open_pulls(
    client: &Octocrab,
    owner: &str,
    name: &str,
    base: &str,
) -> Result<Vec<PullSummary>> {
    #[derive(serde::Serialize)]
    struct Params<'a> {
        state: &'a str,
        base: &'a str,
        per_page: u8,
        page: u32,
    }

    let mut page = 1;
    let mut pulls = Vec::new();
    loop {
        let route = format!("/repos/{owner}/{name}/pulls");
        let params = Params {
            state: "open",
            base,
            per_page: 100,
            page,
        };
        let batch: Vec<ApiPull> = client
            .get(route, Some(&params))
            .await
            .with_context(|| format!("list open PRs for {owner}/{name}"))?;
        if batch.is_empty() {
            break;
        }
        pulls.extend(batch.into_iter().map(|pull| PullSummary {
            number: pull.number,
            title: pull.title,
            author: pull.user.login,
            url: pull.html_url,
            head_sha: pull.head.sha,
            draft: pull.draft.unwrap_or(false),
            base_ref: pull.base.ref_name,
        }));
        page += 1;
    }
    Ok(pulls)
}

async fn fetch_pr_files(
    client: &Octocrab,
    owner: &str,
    name: &str,
    pr: u64,
) -> Result<Vec<ChangedFile>> {
    #[derive(serde::Serialize)]
    struct Params {
        per_page: u8,
        page: u32,
    }

    let mut page = 1;
    let mut files = Vec::new();
    loop {
        let route = format!("/repos/{owner}/{name}/pulls/{pr}/files");
        let params = Params {
            per_page: 100,
            page,
        };
        let batch: Vec<ApiFile> = client
            .get(route, Some(&params))
            .await
            .with_context(|| format!("list files for PR #{pr}"))?;
        if batch.is_empty() {
            break;
        }
        files.extend(batch.into_iter().map(|file| ChangedFile {
            filename: file.filename,
            additions: file.additions,
            deletions: file.deletions,
            patch: file.patch,
        }));
        page += 1;
    }
    Ok(files)
}

fn classify_pr(
    pull: &PullSummary,
    files: &[ChangedFile],
    draft_rel: &Path,
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
    if file.deletions != 0 {
        return Err(format!("has {} deletions", file.deletions));
    }
    if file.additions == 0 {
        return Err("has no additions".to_string());
    }

    let patch = file
        .patch
        .as_deref()
        .ok_or_else(|| "GitHub did not include a parseable patch".to_string())?;

    let mut community = false;
    let mut current_section: Option<String> = None;
    let mut candidates = Vec::new();
    let mut non_whitespace_additions = Vec::new();

    for raw in patch.lines() {
        if raw.starts_with("@@") || raw.starts_with("---") || raw.starts_with("+++") {
            continue;
        }
        let Some(marker) = raw.as_bytes().first().copied() else {
            continue;
        };
        if !matches!(marker, b' ' | b'+') {
            continue;
        }
        let line = &raw[1..];
        if line.trim() == COMMUNITY_HEADING {
            community = true;
            current_section = None;
        } else if line.starts_with("## ") {
            community = false;
            current_section = None;
        } else if line.starts_with("### ") {
            let section = line.trim_start_matches("### ").trim().to_string();
            if community || is_known_community_section(&section) {
                community = true;
                current_section = Some(section);
            } else {
                current_section = None;
            }
        }

        if marker == b'+' {
            if line.trim().is_empty() {
                continue;
            }
            non_whitespace_additions.push(line.to_string());
            if let Some((title, url)) = parse_submission_line(line) {
                let section = current_section
                    .clone()
                    .ok_or_else(|| "link is not inside a community subsection".to_string())?;
                if !is_known_community_section(&section) {
                    return Err(format!(
                        "link is in unsupported community section {section:?}"
                    ));
                }
                candidates.push((title, url, section, line.to_string()));
            }
        }
    }

    if candidates.len() != 1 {
        if candidates.is_empty() && !non_whitespace_additions.is_empty() {
            return Err("does not add a valid one-line markdown list item".to_string());
        }
        return Err(format!("adds {} valid link items", candidates.len()));
    }
    if non_whitespace_additions.len() != 1 {
        return Err(format!(
            "adds {} non-whitespace lines, not one",
            non_whitespace_additions.len()
        ));
    }

    let (title, url, section, line) = candidates.remove(0);
    Ok(Submission {
        pr: pull.number,
        title,
        author: pull.author.clone(),
        pr_url: pull.url.clone(),
        url,
        head_sha: pull.head_sha.clone(),
        section,
        line,
    })
}

fn parse_submission_line(line: &str) -> Option<(String, String)> {
    if line != line.trim_start() {
        return None;
    }
    let trimmed = line.trim_end();
    if !trimmed.starts_with("* ") {
        return None;
    }

    let re = Regex::new(
        r#"^\* (?:(?:\[(?:video|audio|series|[A-Z]{2,5})\])\s+)*\[(?P<title>[^\]\n]+)\]\((?P<url>https?://[^)\s]+)\)\s*$"#,
    )
    .ok()?;
    let captures = re.captures(trimmed)?;
    Some((
        captures.name("title")?.as_str().to_string(),
        captures.name("url")?.as_str().to_string(),
    ))
}

fn is_known_community_section(section: &str) -> bool {
    matches!(
        section,
        "Official"
            | "Foundation"
            | "Newsletters"
            | "Project/Tooling Updates"
            | "Observations/Thoughts"
            | "Rust Walkthroughs"
            | "Research"
            | "Miscellaneous"
    )
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
                "<!-- submerge-pr:{} sha={} url={} author={} -->",
                submission.pr, submission.head_sha, submission.pr_url, submission.author
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

fn parse_edited_buffer(text: &str) -> Result<EditedBuffer> {
    let marker_re = Regex::new(
        r"^<!-- submerge-pr:(?P<pr>\d+) sha=(?P<sha>[0-9a-fA-F]{40}) url=(?P<url>\S+) author=(?P<author>.*?) -->$",
    )?;
    let mut included = BTreeSet::new();
    let mut output = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut idx = 0;

    while idx < lines.len() {
        let line = lines[idx];
        if line.trim_start().starts_with(MARKER_PREFIX) {
            let captures = marker_re
                .captures(line.trim())
                .ok_or_else(|| anyhow!("malformed submerge marker: {line}"))?;
            let pr: u64 = captures["pr"].parse()?;
            if !included.insert(pr) {
                bail!("duplicate marker for PR #{pr}");
            }

            let mut probe = idx + 1;
            while probe < lines.len() && lines[probe].trim().is_empty() {
                probe += 1;
            }
            if probe >= lines.len() {
                bail!("PR #{pr} marker is not followed by a list item");
            }
            if parse_submission_line(lines[probe]).is_none() {
                bail!(
                    "PR #{pr} marker is followed by an invalid list item: {}",
                    lines[probe]
                );
            }
        } else {
            output.push(line.to_string());
        }
        idx += 1;
    }

    let trailing_newline = if text.ends_with('\n') { "\n" } else { "" };
    Ok(EditedBuffer {
        final_text: format!("{}{}", output.join("\n"), trailing_newline),
        included_prs: included,
    })
}

fn edit_buffer(initial: &str, editor: Option<&str>) -> Result<String> {
    let editor = editor
        .map(str::to_string)
        .or_else(|| env::var("EDITOR").ok())
        .ok_or_else(|| anyhow!("$EDITOR is not set; pass --editor or use --dry-run"))?;

    let mut file = Builder::new()
        .prefix("submerge-")
        .suffix(".md")
        .tempfile()
        .context("create editor temp file")?;
    file.write_all(initial.as_bytes())?;
    file.flush()?;

    let status = Command::new(editor)
        .arg(file.path())
        .status()
        .context("run editor")?;
    if !status.success() {
        bail!("editor exited with status {status}");
    }

    fs::read_to_string(file.path()).context("read edited buffer")
}

fn fetch_and_verify_pr_heads(
    repo: &Repository,
    remote_repo: &str,
    submissions: &[Submission],
) -> Result<Vec<Oid>> {
    let remote_url = format!("https://github.com/{remote_repo}.git");
    let mut remote = repo
        .remote_anonymous(&remote_url)
        .with_context(|| format!("open anonymous remote {remote_url}"))?;

    let mut fetch_options = FetchOptions::new();

    let mut oids = Vec::new();
    for submission in submissions {
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
            message.push_str(&format!(
                "- #{} {}: {}\n",
                skipped.pr, skipped.title, skipped.reason
            ));
        }
    }
    message
}

fn ensure_tracked_worktree_clean(repo: &Repository) -> Result<()> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(false)
        .recurse_untracked_dirs(false)
        .renames_head_to_index(true);
    let statuses = repo.statuses(Some(&mut opts)).context("read git status")?;
    if statuses.is_empty() {
        return Ok(());
    }

    let paths = statuses
        .iter()
        .filter_map(|entry| entry.path().ok().map(str::to_string))
        .take(10)
        .collect::<Vec<_>>()
        .join(", ");
    bail!("tracked worktree is not clean: {paths}");
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
    println!("skipped PRs: {}", skipped.len());
    for skipped in skipped {
        println!("  #{} {}: {}", skipped.pr, skipped.title, skipped.reason);
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
            base_ref: "main".to_string(),
        }
    }

    fn file(patch: &str, additions: u64, deletions: u64) -> ChangedFile {
        ChangedFile {
            filename: "draft/2026-06-24-this-week-in-rust.md".to_string(),
            additions,
            deletions,
            patch: Some(patch.to_string()),
        }
    }

    #[test]
    fn classifies_one_link_submission() {
        let patch = r#"@@ -44,6 +44,7 @@ and just ask the editors to select the category.
 ### Project/Tooling Updates
+* [Ratatui 0.30.2 is released](https://ratatui.rs/highlights/v0302/)
 
 ### Observations/Thoughts"#;
        let submission = classify_pr(
            &pull(1),
            &[file(patch, 1, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
        )
        .unwrap();
        assert_eq!(submission.section, "Project/Tooling Updates");
        assert_eq!(submission.title, "Ratatui 0.30.2 is released");
    }

    #[test]
    fn allows_one_link_plus_blank_line() {
        let patch = r#"@@ -44,6 +44,8 @@ and just ask the editors to select the category.
 ### Project/Tooling Updates
+* [m-vis 0.4.0-rc1 is released](https://github.com/SickleFire/m-vis)
+
 
 ### Observations/Thoughts"#;
        let submission = classify_pr(
            &pull(2),
            &[file(patch, 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
        )
        .unwrap();
        assert_eq!(submission.pr, 2);
    }

    #[test]
    fn skips_event_blocks() {
        let patch = r#"@@ -254,6 +254,8 @@ Rusty Events between 2026-06-24 - 2026-07-22
 * 2026-06-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
+* 2026-06-25 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse/)
+    * [**Rust Toulouse Meetup - Bevy & ESP32**](https://www.meetup.com/rust-community-toulouse/events/314947457/)
 * 2026-07-02 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)"#;
        let err = classify_pr(
            &pull(3),
            &[file(patch, 2, 0)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
        )
        .unwrap_err();
        assert!(err.contains("non-whitespace") || err.contains("valid one-line"));
    }

    #[test]
    fn skips_deletions() {
        let err = classify_pr(
            &pull(4),
            &[file("", 1, 1)],
            Path::new("draft/2026-06-24-this-week-in-rust.md"),
        )
        .unwrap_err();
        assert!(err.contains("deletions"));
    }

    #[test]
    fn build_and_parse_edit_buffer() {
        let draft = "## Updates from Rust Community\n\n### Project/Tooling Updates\n\n### Observations/Thoughts\n";
        let submissions = vec![Submission {
            pr: 42,
            title: "Example".to_string(),
            author: "alice".to_string(),
            pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/42".to_string(),
            url: "https://example.com/post".to_string(),
            head_sha: "0123456789abcdef0123456789abcdef01234567".to_string(),
            section: "Project/Tooling Updates".to_string(),
            line: "* [Example](https://example.com/post)".to_string(),
        }];
        let buffer = build_edit_buffer(draft, &submissions).unwrap();
        assert!(buffer.contains("submerge-pr:42"));

        let parsed = parse_edited_buffer(&buffer).unwrap();
        assert!(parsed.included_prs.contains(&42));
        assert!(!parsed.final_text.contains("submerge-pr:42"));
        assert!(parsed
            .final_text
            .contains("* [Example](https://example.com/post)"));
    }

    #[test]
    fn parse_edit_buffer_rejects_duplicate_markers() {
        let text = "<!-- submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 url=https://example.com author=alice -->\n* [Example](https://example.com)\n<!-- submerge-pr:42 sha=0123456789abcdef0123456789abcdef01234567 url=https://example.com author=alice -->\n* [Example](https://example.com)\n";
        let err = parse_edited_buffer(text).unwrap_err();
        assert!(err.to_string().contains("duplicate"));
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
            author: "alice".to_string(),
            pr_url: "https://github.com/rust-lang/this-week-in-rust/pull/7".to_string(),
            url: "https://example.com/post".to_string(),
            head_sha: pr_parent.to_string(),
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
