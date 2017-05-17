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

# Crate of the Week

This week's crate of the week is [PX8](https://github.com/Gigoteur/PX8), a Rust implementation of an Open Source fantasy console. Thanks to [hallucino](https://users.rust-lang.org/users/hallucino) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust libz blitz status update 2017-05-12](https://internals.rust-lang.org/t/rust-libz-blitz/5184/29). 11 issues were closed last week and 8 new cookbook examples added. This week, the libz blitz is in need of someone to [lead the walkdir evaluation](https://internals.rust-lang.org/t/rust-libz-blitz/5184/30).
* [rust-url is looking for maintainers](https://users.rust-lang.org/t/help-wanted-maintaining-rust-url/10707).
* [rust: Get test suite working with wasm](https://github.com/rust-lang/rust/issues/38800).
* [easy] [rust-cookbook: Switch error handling setup to quick_main! macro from error-chain](https://github.com/brson/rust-cookbook/issues/59).
* [rustup: Optimize disk access during install](https://github.com/rust-lang-nursery/rustup.rs/issues/904).
* [rustup: Improve error message when override doesn't exist](https://github.com/rust-lang-nursery/rustup.rs/issues/820).
* [easy] [rustup: Use ShellExecute to open docs on windows](https://github.com/rust-lang-nursery/rustup.rs/issues/499).
* [easy] [rustup: No info printed on `rustup target remove`](https://github.com/rust-lang-nursery/rustup.rs/issues/306).
* [easy] [rustup: rustup tries to sync channel updates for non-updatable channels](https://github.com/rust-lang-nursery/rustup.rs/issues/756).
* [easy] [rustup: Windows confirmation prompt not disabled with -y](https://github.com/rust-lang-nursery/rustup.rs/issues/916).
* [easy] [rustup: Please show me the version I'm downloading/updatin](https://github.com/rust-lang-nursery/rustup.rs/issues/1007).
* [easy] [rustup: `target add` and `component add` should succeed if target/component is already installed](https://github.com/rust-lang-nursery/rustup.rs/issues/1009).
* [rustup: Build with panic=abort](https://github.com/rust-lang-nursery/rustup.rs/issues/992).
* [rustup: zero exit code on failing `rustup update`](https://github.com/rust-lang-nursery/rustup.rs/issues/1095).
* [rustup: Teach rustup/rustbuild about optional components](https://github.com/rust-lang-nursery/rustup.rs/issues/1006).
* [rustup: Support dependencies between optional components/extensions](https://github.com/rust-lang-nursery/rustup.rs/issues/1111).
* [easy] [rusoto: SQS: bring back integration tests](https://github.com/rusoto/rusoto/issues/616). Rusoto is an AWS SDK for Rust.
* [easy] [quackin: Better documentation for the recommender module](https://github.com/z1mvader/quackin/issues/3). Quackin is a recommender systems framework for Rust.
* [easy] [quackin: Item based recommender](https://github.com/z1mvader/quackin/issues/1).
* [medium] [quackin: Benchmarks for recommenders](https://github.com/z1mvader/quackin/issues/4).
* [easy] [quackin: Support similarities like Jaccard and Pearson correlation](https://github.com/z1mvader/quackin/issues/5).
* [medium] [quackin: Performance metrics like Precision, Recall, RMSE and MAE](https://github.com/z1mvader/quackin/issues/7).

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

* Bastien Orivel
* Dennis Schridde
* Eduardo Pinho
* faso
* Liran Ringel

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1849: Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: close] [Add Cargo post-build scripts](https://github.com/rust-lang/rfcs/pull/1777).
* [disposition: close] [Reject crates.io uploads which declare a feature named `no_std`](https://github.com/rust-lang/rfcs/pull/1841).
* [disposition: merge] [Expand and stabilize `impl Trait`](https://github.com/rust-lang/rfcs/pull/1951).
* [disposition: close] [Allow any Displayable type for expect](https://github.com/rust-lang/rfcs/pull/1968).
* [disposition: merge] [Make RangeInclusive just a two-field struct (amend 1192)](https://github.com/rust-lang/rfcs/pull/1980).

## New RFCs

* [Const generics](https://github.com/rust-lang/rfcs/pull/2000).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [Spaces around `=` in attributes](https://github.com/rust-lang-nursery/fmt-rfcs/issues/82)
* [Return type on new line](https://github.com/rust-lang-nursery/fmt-rfcs/issues/77) - proposed to close
* [Single-line where](https://github.com/rust-lang-nursery/fmt-rfcs/issues/74) - proposed to close
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

* [May 17. Rust LA May Meetup - Rust Birthday Party](https://www.meetup.com/Rust-Los-Angeles/events/239616841/)!
* [May 17. South Florida Rust - Rust Birthday Party](https://www.meetup.com/South-Florida-Rust-Meetup/events/239036595/)!
* [May 17. Rust Atlanta - Heterogeneous Collections in Rust at Tech Square Labs (Midtown)](https://www.meetup.com/Rust-ATL/events/239205124/).
* [May 17. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/239666001/).
* [May 17. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 17. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 18. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [May 20. Rust Bangalore community meetup](https://www.eventshigh.com/detail/Bangalore/9a49c6be73b6591e770d1cece7eec6fe-Rust-Bangalore-First-Meetup).
* [May 20-21. Hackathon TupperRust (Lyon)](https://public.etherpad-mozilla.org/p/TupperRust-lyon-1705).
* [May 24. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 24. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 30. Rust Oslo - Fearless programming with Rust - Adventures with Asynchronous I/O](https://www.meetup.com/Rust-Oslo/events/238315636/).
* [May 30. Rust Toronto meetup - Hands-on parsing in Rust](https://www.meetup.com/Rust-Toronto/events/239839632/).
* [May 31. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/239889748/).
* [May 31. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 31. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun  1. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at Anixe](https://rustjobs.rs/jobs/21/anixe-rust-developer)
* [Rust Legend at Between Lines](https://rustjobs.rs/jobs/20/between-lines-ltd-rust-legend)
* [Research Engineer (Big Data) at Hadean](https://rustjobs.rs/jobs/18/hadean-research-engineer-big-data)
* [Rust Engineer at Suitable Technologies](https://www.reddit.com/r/rust/comments/6bfjqc/possible_paid_rust_work/).
* [Core Developer for Parity Technologies](https://parity.io/#rust-dev-job)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Spent the last week learning rust. The old martial arts adage applies. Cry in the dojo, laugh in the battlefield.

— [/u/crusoe on Reddit](https://www.reddit.com/r/programming/comments/69g8il/the_horror_in_the_standard_library/dh7fakg/).

Thanks to [Ayose Cazorla](https://users.rust-lang.org/t/twir-quote-of-the-week/328/399) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
