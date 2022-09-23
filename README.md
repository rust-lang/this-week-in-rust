This Week in Rust
=================

Content for [this-week-in-rust.org](http://this-week-in-rust.org). Made available under CC-BY-SA.

All code Copyright 2014 Ember Arlynx, made available under [the MIT
license](http://mit-license.org/).

## TWiR Editors

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [cdmistman](https://github.com/cdmistman)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [andrewpollack](https://github.com/andrewpollack)
* [U007D](https://github.com/U007D) 
* [kolharsam](https://github.com/kolharsam)
* [joelmarcey](https://github.com/joelmarcey)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bennyvasquez](https://github.com/bennyvasquez)

### Language Reviewers

* [yuk1ty](https://github.com/yuk1ty) - Japanese
* [rpruizc](https://github.com/rpruizc) - Spanish
* [matheus-consoli](https://github.com/matheus-consoli) - Portuguese
* [MATRIXKOO](https://github.com/MATRIXKOO) - Chinese
* [Folyd](https://github.com/Folyd) - Chinese

## PRs for next issue are now being accepted

To propose content for inclusion in the next newsletter (found in the `drafts/`
folder), create a new [Pull Request](https://github.com/rust-lang/this-week-in-rust/pulls) updating the relevant section in the 
draft.

Alternately, tweet us [@thisweekinrust](https://twitter.com/thisweekinrust).

### What do we look for when considering whether to include something in This Week in Rust?

This Week in Rust is intended to highlight the incredible work of the Rust Community. 

What we are generally looking for includes:

* how-to intros (and advanced deep dives) into Rust concepts and areas
* Rust walkthroughs that explain concepts in different ways than well known resources like [the Rust book](https://doc.rust-lang.org/stable/book/), [Rustlings](https://github.com/rust-lang/rustlings), and [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
* updates on tooling when in long form or framed as a tutorial (for more details, see what we are not looking for below)
* Rust-related podcast episodes
* Rust-related screenshots and videos
* Rust meetup recordings
* Rust meetup announcements
* Presenter slide decks on Rust
* Observations and thoughts on Rust and the Rust community
* Calls for participation in Rust open source projects
* Rust job announcements
* and more!

What we are generally NOT looking for includes:

* Anything that violates the [Rust Community Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)
* Rants or anything degrading to any part or member of the Community. Rather than submitting an article about what is wrong with something, we would much rather you write something that explains how you'd make it better.
* Duplicates of recent posts (even with the wording changed slightly)
* Anything behind a paywall (this includes Medium's paid article / members-only mechanism)
* Anything that requires information to be shared/captured (like an email address) in order to access

**Projects/Tooling Updates**

There are further guidelines for the Projects/Tooling Updates Section

We include:

* Updates on tooling when in long form or framed as a tutorial (this can be through a blog, through GitHub, through a newsletter, and more) - it must have a high amount of Rust specific info (examples in Rust, notes on things learned about Rust in the process of creating/updating the project, etc.)
* Updates on tooling that call out specific contributors - it is wonderful to highlight all the great people contributing to Rust OSS projects (Note - the update still must include a high amount of Rust specific info)

We do not include:
* Updates that are solely a change log without details on the changes, guides to using the changes, etc.
* Links that are solely to a GitHub repo or crate on crates.io. While we would love to include these, there are too many being created/updated every week for us to include them all. We encourage you to write up an introduction to your project with examples, information you learned through creating the project, etc.

Notes:
* A small description of the project or the update in your link is encouraged (for example: FooBar 1.0: adding support for Baz)
* We discourage submitting links and link descriptions that are solely of a commercial/sales nature

These are meant to be guidelines, if you are ever not sure about whether something should be included please feel free to open a pull request anyway and we can discuss it!

The editors of This Week in Rust do reserve the right to make the decision about whether to include something or not, but we intend to do so in a way that is as transparent as possible.

## Link style guidelines:

The link text should be the same as the page's title. If the title seems to need additional context (for example, if the title is "What's New" and should have the project name added), please ask in the PR comments.

Links should use the most canonical form. For example, if `example.tech` redirects to `www.example.com`, then the latter is preferred.

Links should not contain unnecessary tracking parameters, e.g. `utm_source`, `utm_campaign`.

Some prefixes are used, and should be placed to the left of the link.
- `[video]` for videos
- `[audio]` for podcasts or other audio.
- `[series]` for articles that are one of a series.
- 2-letter languages codes (e.g. `[ZH]`, `[ES]`, `[FR]`) for content in a language other than English.

## Community sub-categories

Editors will sort community links into sub-categories. The following sub-categories are currently used:
- **Official** -- rust-lang.org blog posts and other official Rust team communications.
- **Foundation** -- foundation.rust-lang.org blog posts and other official foundation communications.
- **Project/Tooling Updates** -- News about the progress of a Rust project. Must be more informative than just a changelog.
- **Newsletters** -- Regularly scheduled articles about an area of Rust development, e.g. posts titled "This Month in ___".
- **Research** -- Academic Papers that are about Rust or contain significant Rust content.
- **Observations/Thoughts** -- Articles about Rust.
- **Rust Walkthroughs** -- Articles that include a significant amount of Rust source code, that walk the reader through building something.
- **Miscellaneous** -- Links that don't clearly fit in other sub-categories.

Most blog posts about Rust belong in **Rust Walkthroughs** if they show how something is done (including source code), otherwise **Observations/Thoughts**. Articles that don't contain much Rust content, or news articles that mention Rust, won't always be accepted, but when they are they can be placed in the **Miscellaneous** sub-category.

If a set of related links is published (e.g. from a large Rust conference), the editors may choose to invent a new category just for that issue.

## How I get PR lists:

```
git log --author=bors --since='MM/DD/YYYY 12:00PM' --until='MM/DD/YYYY 12:00PM' --pretty=oneline > ~/entropy/twir.txt
# edit in vim to get rid of everything but PR number, copy into clipboard
for pr in $(xsel -ob); do firefox https://github.com/mozilla/rust/pull/$pr; sleep 0.07; done
# wait a long time...
# write TWIR
```

Alternatively use GitHub search:

```
https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-03..2014-11-10
```

## How I get new contributors:

Use the included `new_contribs.sh` script:

  new_contribs.sh 6/21/2014

## Building

Ensure you have SASS installed. The easiest way to get it is via `gem`, the
Ruby package manager.

```
env SASS_BIN=$HOME/.gem/ruby/*/bin/sass pelican content -s pelicanconf.py
```

### To build the newsletter

* Generate the HTML
  ```
  TWIR_NEWSLETTER_THEME=1 pelican --delete-output-directory content
  ```
* Copy the HTML and inline CSS at http://zurb.com/ink/inliner.php - (MailChimp's inliner doesn't remove the CSS from `<head>`).
* Send the newsletter (we currently use MailChimp).