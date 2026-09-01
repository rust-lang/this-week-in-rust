#!/usr/bin/env python3
"""Generate the Calls for Testing / Approved RFCs / FCP / New RFCs sections for TWiR.

Queries GitHub via the `gh` CLI and writes draft/cft_rfc_fcp_draft_template.md.
Copy the generated sections into this week's draft.

Meant to be run on Tuesday morning: the reporting window is the previous
Tuesday through the most recent Tuesday (the run day), both inclusive, so
activity from the morning of the run is picked up. Items only count if their
label was added, the PR merged, or the RFC created within that window.
Because consecutive windows share the Tuesday boundary, an item from last
Tuesday can appear two weeks running - cross-check against last week's issue.

Usage: tools/generate_cft_fcp.py [--end YYYY-MM-DD]
  --end  end of the reporting window, a Tuesday (default: most recent Tuesday)
"""

import argparse
import datetime
import json
import subprocess
import sys
from pathlib import Path
from typing import TypedDict, cast

TEMPLATE_PATH = Path(__file__).resolve().parent.parent / "draft" / "cft_rfc_fcp_draft_template.md"

# static boilerplate appended after the generated sections
TAIL = """\
<!-- Call for Testing Message (post in GH `issue` and remove `call-for-testing` label) -->
This RFC will appear in the **Call for Testing** section of the next issue (#) of This Week in Rust (TWiR).
You may remove the `call-for-testing` label.  Please feel free to leave the `call-for-testing` label in place if you would like this RFC to appear again in another issue of TWiR.

<!-- Commit message -->
Update CFT, FCP, MCP and RFC sections for TWiR-xxx
"""


class Label(TypedDict):
    name: str


class Item(TypedDict):
    """The fields of a GitHub search result item that we use."""
    number: int
    title: str
    html_url: str
    labels: list[Label]


class LabelEvent(TypedDict):
    name: str
    at: str

# (display name, repo, extra header markdown) with the exact section URLs used in past issues
CFT_REPOS = [
    ("Rust", "rust-lang/rust",
     "https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen"),
    ("Cargo", "rust-lang/cargo",
     "https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen"),
    ("Rustup", "rust-lang/rustup",
     "https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen"),
    ("Rust language RFCs", "rust-lang/rfcs",
     "https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen"),
]

# (display name, repo, extra search qualifiers, section URL, header suffix)
FCP_GROUPS = [
    ("Rust", "rust-lang/rust", "",
     "https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen", ""),
    ("Rust RFCs", "rust-lang/rfcs", "",
     "https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen", ""),
    ("Cargo", "rust-lang/cargo", "",
     "https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen", ""),
    ("Compiler Team", "rust-lang/compiler-team", "label:major-change",
     "https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen",
     " [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)"),
    ("Language Team", "rust-lang/lang-team", "",
     "https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen", ""),
    ("Language Reference", "rust-lang/reference", "",
     "https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen", ""),
    ("Leadership Council", "rust-lang/leadership-council", "",
     "https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen", ""),
    ("Unsafe Code Guidelines", "rust-lang/unsafe-code-guidelines", "",
     "https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen", ""),
]


def search(query: str) -> list[Item]:
    """Run a GitHub issue search, newest-updated first, and return the items."""
    out = subprocess.run(
        ["gh", "api", "-X", "GET", "search/issues",
         "-f", f"q={query}", "-f", "sort=updated", "-f", "order=desc",
         "-f", "per_page=100", "-f", "advanced_search=true"],
        capture_output=True, text=True, check=False)
    if out.returncode != 0:
        sys.exit(f"gh search failed for {query!r}:\n{out.stderr}")
    return cast("list[Item]", cast("dict[str, object]", json.loads(out.stdout))["items"])


def labeled_in_window(repo: str, item: Item, label: str,
                      start: datetime.date, end: datetime.date) -> bool:
    """True if `label` was added to the item within [start, end], both inclusive."""
    out = subprocess.run(
        ["gh", "api", "--paginate",
         f"repos/{repo}/issues/{item['number']}/events",
         "--jq", '.[] | select(.event == "labeled") | {name: .label.name, at: .created_at}'],
        capture_output=True, text=True, check=False)
    if out.returncode != 0:
        sys.exit(f"gh events failed for {repo}#{item['number']}:\n{out.stderr}")
    for line in out.stdout.splitlines():
        event = cast("LabelEvent", json.loads(line))
        if event["name"] == label and start.isoformat() <= event["at"][:10] <= end.isoformat():
            return True
    return False


def labeled_search(repo: str, label: str, extra: str,
                   start: datetime.date, end: datetime.date) -> list[Item]:
    """Open items with `label`, keeping only those labeled within the window."""
    items = search(f"repo:{repo} state:open label:{label} {extra}".strip())
    return [i for i in items if labeled_in_window(repo, i, label, start, end)]


def bullet(item: Item) -> str:
    return f"* [{item['title']}]({item['html_url']})"


def oxford_join(lines: list[str]) -> str:
    """Join '[Name](url)' lines: comma-newline separated, 'or' before the last."""
    if len(lines) == 1:
        return lines[0]
    return ",\n".join(lines[:-1]) + " or\n" + lines[-1]


def cft_section(out: list[str], start: datetime.date, end: datetime.date) -> None:
    out.append("## Calls for Testing")
    out.append("An important step for RFC implementation is for people to experiment with the")
    out.append("implementation and give feedback, especially before stabilization.")
    out.append("")
    out.append("If you are a feature implementer and would like your RFC to appear in this list, add a")
    out.append("`call-for-testing` label to your RFC along with a comment providing testing instructions and/or")
    out.append("guidance on which aspect(s) of the feature need testing.")
    out.append("")
    empty: list[str] = []
    for name, repo, url in CFT_REPOS:
        items = labeled_search(repo, "call-for-testing", "", start, end)
        if items:
            out.append(f"##### [{name.strip()}]({url})")
            out.extend(bullet(item) for item in items)
            out.append("")
        else:
            empty.append(f"[{name.strip()}]({url})")
    if empty:
        out.append(f"*No calls for testing were issued this week by\n{oxford_join(empty)}.*")
        out.append("")
    out.append("[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.")
    out.append("")


def approved_rfcs_section(out: list[str], start: datetime.date, end: datetime.date) -> None:
    out.append("---")
    out.append("")
    out.append("### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)")
    out.append("")
    out.append("Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These")
    out.append("are the RFCs that were approved for implementation this week:")
    out.append("")
    items = search(f"repo:rust-lang/rfcs is:pr is:merged merged:{start}..{end}")
    if items:
        out.extend(bullet(i) for i in items)
    else:
        out.append("* *No RFCs were approved this week.*")
    out.append("")


def disposition_prefix(item: Item) -> str:
    for label in item["labels"]:
        name = label["name"]
        if name.startswith("disposition-") and name != "disposition-merge":
            return f"[disposition: {name.removeprefix('disposition-')}] "
    return ""


def fcp_section(out: list[str], start: datetime.date, end: datetime.date) -> None:
    out.append("### Final Comment Period")
    out.append("")
    out.append("Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs")
    out.append("which are reaching a decision. Express your opinions now.")
    out.append("")
    out.append("#### Tracking Issues & PRs")
    out.append("")
    empty: list[str] = []
    for name, repo, extra, url, suffix in FCP_GROUPS:
        items = labeled_search(repo, "final-comment-period", extra, start, end)
        if items:
            out.append(f"##### [{name.strip()}]({url}){suffix}")
            out.extend(f"* {disposition_prefix(i)}[{i['title']}]({i['html_url']})" for i in items)
            out.append("")
        else:
            empty.append(f"[{name.strip()}]({url})")
    if empty:
        out.append(f"*No Items entered Final Comment Period this week for\n{oxford_join(empty)}.*")
    out.append("Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.")
    out.append("")


def new_rfcs_section(out: list[str], start: datetime.date, end: datetime.date) -> None:
    out.append("### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)")
    items = search(f"repo:rust-lang/rfcs is:pr state:open created:{start}..{end}")
    if items:
        out.extend(bullet(i) for i in items)
    else:
        out.append("* *No New or Updated RFCs were created this week.*")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    _ = parser.add_argument("--end", help="end of reporting window (a Tuesday), YYYY-MM-DD (default: most recent Tuesday)")
    args = parser.parse_args()
    end_arg = cast("str | None", args.end)
    if end_arg:
        end = datetime.date.fromisoformat(end_arg)
    else:
        today = datetime.datetime.now(tz=datetime.timezone.utc).astimezone().date()
        end = today - datetime.timedelta(days=(today.weekday() - 1) % 7)  # most recent Tuesday
    start = end - datetime.timedelta(days=7)
    print(f"Reporting window: {start} to {end}, both inclusive", file=sys.stderr)

    out: list[str] = []
    cft_section(out, start, end)
    approved_rfcs_section(out, start, end)
    fcp_section(out, start, end)
    new_rfcs_section(out, start, end)
    generated = "\n".join(out) + "\n"
    _ = TEMPLATE_PATH.write_text(generated + "\n" + TAIL)
    print(f"Wrote {TEMPLATE_PATH}", file=sys.stderr)


if __name__ == "__main__":
    main()
