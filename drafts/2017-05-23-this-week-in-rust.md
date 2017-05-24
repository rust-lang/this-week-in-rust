Title: This Week in Rust 183
Number: 183
Date: 2017-05-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [Announcing the Infrastructure, Cargo, and Dev Tools teams; disbanding the Tools team](https://internals.rust-lang.org/t/announcing-the-infrastructure-cargo-and-dev-tools-teams-disbanding-the-tools-team/5256).
* [Rust and CSV parsing](http://blog.burntsushi.net/csv/).
* [How to: Run Rust code on your NVIDIA GPU](https://github.com/japaric/nvptx#nvptx).
* [System programming in Rust: Beyond safety](https://www.sigops.org/hotos/hotos17/papers/hotos17-final92.pdf).
* [TLA + Rust for provably correct and safe distributed systems](https://github.com/spacejam/tla-rust/blob/master/README.md).
* [Building graphical applications to JS in stable Rust](https://gregkatz.github.io/2017-05-20-rust-emscripten.html).
* [Walkthrough: Rocket, Diesel and a Postgres database on AWS using Rusoto](https://matthewkmayer.github.io/blag/public/post/rusoto-rds-walkthrough/).
* [Enforcing documentation in a medium-size Rust project](https://medium.com/@Razican/enforcing-documentation-in-a-medium-size-rust-project-7b6a2a47b6d6).
* [My first PR for a Rust project](https://medium.com/adventures-in-rust/my-first-pr-for-a-rust-project-364d8c02220b).
* [This week in Rust docs 57](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-57).

# Crate of the Week

This week's crate of the week is [PX8](https://github.com/Gigoteur/PX8), a Rust implementation of an Open Source fantasy console. Thanks to [hallucino](https://users.rust-lang.org/users/hallucino) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust libz blitz status update 2017-05-19](https://internals.rust-lang.org/t/rust-libz-blitz/5184/34). Contribution opportunities are available.
* [Rust cookbook needs a lot of help](https://github.com/brson/rust-cookbook/issues?q=is%3Aissue+is%3Aopen+label%3Aexample)!
* [Help wanted: Rust for embedded development: Where we are and what’s missing](https://users.rust-lang.org/t/rust-for-embedded-development-where-we-are-and-whats-missing/10861).
* [rust-url is looking for maintainers](https://users.rust-lang.org/t/help-wanted-maintaining-rust-url/10707).
* The log crate just completed its [evaluation](https://internals.rust-lang.org/t/crate-evaluation-for-2017-05-16-log/5185/50), generating a ton of [easy-tagged issues](https://github.com/rust-lang-nursery/log/issues?utf8=%E2%9C%93&q=is%3Aissue%20is%3Aopen%20label%3A%22help%20wanted%22%20label%3Aeasy).
* [easy] [i3status-rust is looking for contributors to make i3 window manager more awesome](https://github.com/XYunknown/i3status-rust)!
* [rust: Teach .pkg, .msi, .exe installers about RLS](https://github.com/rust-lang/rust/issues/42157).
* [rust: Get test suite working with wasm](https://github.com/rust-lang/rust/issues/38800).
* [gutenberg: Make a SyntaxSet pool](https://github.com/Keats/gutenberg/issues/70). Gutenberg is an opinionated static site generator written in Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

125 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-05-08..2017-05-15

* [disallow `._` in float literal](https://github.com/rust-lang/rust/pull/41946) (breaking change, but...who does this?), also
  [`illegal_floating_point_literal_pattern` compat lint](https://github.com/rust-lang/rust/pull/41293)
* [enforce well-formedness after generalizing](https://github.com/rust-lang/rust/pull/41716)
* [`eprint!(..)` / `eprintln!(..)`](https://github.com/rust-lang/rust/pull/41192)
* [faster `[u8].reverse()`](https://github.com/rust-lang/rust/pull/41764)
* [polymorphic `span_label`](https://github.com/rust-lang/rust/pull/41745) (reduces plugin ceremony)
* [diagnostics: allow multiple suggestions](https://github.com/rust-lang/rust/pull/41876)
* [remove no longer needed libsyntax features](https://github.com/rust-lang/rust/pull/41729)
* [remove unused macros](https://github.com/rust-lang/rust/pull/41934)
* [more crate metadata queries](https://github.com/rust-lang/rust/pull/41724)
* [include crate root in `save-analysis`](https://github.com/rust-lang/rust/pull/41919)
* [finer crate hashing for incremental compilation](https://github.com/rust-lang/rust/pull/41709)
* [Windows io::Error improvements](https://github.com/rust-lang/rust/pull/41684)
* [`impl Clone for .split_whitespace()`](https://github.com/rust-lang/rust/pull/41659)
* [allow bare CR in doc comments](https://github.com/rust-lang/rust/pull/41827)
* [box large MIR variants](https://github.com/rust-lang/rust/pull/41926)
* [fix exponential LLVM code growth on inlining drops](https://github.com/rust-lang/rust/pull/41920)
* [RLS crash fixed](https://github.com/rust-lang/rust/pull/41969)
* [fix lvalue ops](https://github.com/rust-lang/rust/pull/41939)
* [fix error with `-Z treat-err-as-bug`](https://github.com/rust-lang/rust/pull/41942)
* [don't deny outer type parameters in embedded constants](https://github.com/rust-lang/rust/pull/41939)
* [equate items ignoring variance](https://github.com/rust-lang/rust/pull/41913)
* [new `-Z force-unstable-if-unmarked` flag](https://github.com/rust-lang/rust/pull/41847)
* [stabilize `-C target-feature=+crt-static`](https://github.com/rust-lang/rust/pull/41757)
* [cargo fetches only registry index master branch](https://github.com/rust-lang/cargo/pull/4024)
* [cargo now ignores malformed manifests on git deps](https://github.com/rust-lang/cargo/pull/3998)
* [cargo build script search path fixed](https://github.com/rust-lang/cargo/pull/3974)
* [cargo now retries requests that failed with 5XX](https://github.com/rust-lang/cargo/pull/4032)
* [cargo no longer checks out the whole crates.io index](https://github.com/rust-lang/cargo/pull/4026)
* [(mostly) rewrite the Rust installer in Rust](https://github.com/rust-lang/rust-installer/pull/62)

## New Contributors

* Anders Papitto
* Daniel Lockyer
* David LeGare
* Ivan Dardi
* Michael Kohl
* Mike Lubinets
* Venkata Giri Reddy

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amendment to RFC 1192: Make RangeInclusive just a two-field struct](https://github.com/rust-lang/rfcs/pull/1980).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Finalize syntax and parameter scoping for `impl Trait`, while expanding it to arguments](https://github.com/rust-lang/rfcs/pull/1951).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: close] [Add Cargo post-build scripts](https://github.com/rust-lang/rfcs/pull/1777).

## New RFCs

* [Support of alternate registries in Cargo](https://github.com/rust-lang/rfcs/pull/2006).
* [Match ergonomics using default binding modes](https://github.com/rust-lang/rfcs/pull/2005).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [Spaces around `=` in attributes](https://github.com/rust-lang-nursery/fmt-rfcs/issues/82)
* [Return type on new line](https://github.com/rust-lang-nursery/fmt-rfcs/issues/77) - proposed to close
* [Attribute/doc comment ordering](https://github.com/rust-lang-nursery/fmt-rfcs/issues/72)
* [Ordering of types of groups within a module](https://github.com/rust-lang-nursery/fmt-rfcs/issues/71)
* [Function calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/65) and [chains of calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/66)
* [Combining opening and closing delims](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)
* [Where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)

# Upcoming Events

* [May 24. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 24. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 30. Rust Oslo - Fearless programming with Rust - Adventures with Asynchronous I/O](https://www.meetup.com/Rust-Oslo/events/238315636/).
* [May 30. Rust Toronto meetup - Hands-on parsing in Rust](https://www.meetup.com/Rust-Toronto/events/239839632/).
* [May 31. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/239889748/).
* [May 31. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 31. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun  1. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jun  5. Rust Prague Meetup #4](https://www.meetup.com/rust-prague/events/240025447/).
* [Jun  6. Mozilla Rust Roadshow @ GA Boston: Rust! Hack Without Fear](https://generalassemb.ly/education/ga-mozilla-developer-roadshow-presents-rust-hack-without-fear/boston/36069).
* [Jun  7. Rust Cologne - Rust 2nd Anniversary Reloaded](http://rust.cologne/2017/06/07/rust-2nd-aniversary-part-2.html).
* [Jun  7. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/240072184/).
* [Jun  7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jun  7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun 8. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/240198831/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at 1aim](https://rustjobs.rs/jobs/22/1aim-gmbh-rust-developer).
* [Rust Developer at Anixe](https://rustjobs.rs/jobs/21/anixe-rust-developer)
* [Rust Legend at Between Lines](https://rustjobs.rs/jobs/20/between-lines-ltd-rust-legend)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
