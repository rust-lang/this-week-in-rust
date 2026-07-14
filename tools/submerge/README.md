# Submerge

Aggregate one-link TWiR submission PRs into a local multi-parent merge commit.

## Workflow

First you'll want to install submerge, which (temporarily) is done like this:

```
# (from your this-week-in-rust git checkout)
git remote add jder https://github.com/jder/this-week-in-rust.git
git remote update jder
git switch --detach jder/submerge
cargo install --path tools/submerge
```

### Start a clean branch from main

```
git remote update
git switch -c $(date +%Y-%m-%d)-links origin/main
```

### Fetch the current open community links PRs

```
submerge fetch
```

This will print a list of PRs which are being placed into your local draft copy, as well as the PRs which are not (and a reason for each of the latter).

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
