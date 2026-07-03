# Submerge

Aggregate one-link TWiR submission PRs into a local multi-parent merge commit.

# Workflow

  1. Start a clean branch from main.
  2. Run `submerge fetch`.
  3. Edit the draft to reorder, remove, or adjust submissions.
  4. Run the usual `tools/inspect*` checks.
  5. Run `submerge merge`.
  6. Push the resulting branch and open a PR.
