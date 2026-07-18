# Submerge

Aggregate one-link TWiR submission PRs into a local multi-parent merge commit.

## Workflow

First you'll want to install submerge:

```
# (from your this-week-in-rust git checkout)
git remote update
cargo install --path tools/submerge
```

If you don't have a `GITHUB_TOKEN` in your environment, get one from [here](https://github.com/settings/personal-access-tokens/new).
It only needs "Pulic Repository" access and you don't need to add any permissions. This is used for fetching
the this-week-in-rust repository content with higher rate limits.

```
export GITHUB_TOKEN=<your token from github>
```

### Start a clean branch from main

```
git switch -c $(date +%Y-%m-%d)-links origin/main
```

### Fetch the current open community links PRs

```
submerge fetch
```

This will print a list of PRs which are being placed into your local draft copy, as well as the PRs which are not (and a reason for each of the latter).

Note this only fetches PRs which add a single community link and are passing CI. If other cases become common we could extend this!

### Edit the draft to reorder, remove, or adjust submissions
Keep the comment metadata attached to each link as you move/edit. The metadata itself shouldn't be edited.

Deleting a link and its metadata will result in that PR being untouched by later steps (i.e. you will want to manually go respond to or close that PR).

### Run the usual checks

Review the content manually, run these tools:

```
python3 tools/inspect_links.py
python3 tools/inspect_markdown.py
```

### Create the merge commit

```
submerge merge
```

This will create a single commit which merges all the PRs still in the draft together. 

### Open a PR from your branch

Merging this PR will close all the submission PRs which you left in the draft.
