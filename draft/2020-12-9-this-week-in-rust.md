Title: This Week in Rust 368
Number: 368
Date: 2020-12-9
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No project updates this week.

### Official

### Newsletters

### Tooling

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

# Crate of the Week

This week's crate is [kira](https://github.com/tesselode/kira), a library for expressive game audio with many bells and whistles (pardon the pun).

Thanks to [Alexis Bourget](https://users.rust-lang.org/t/crate-of-the-week/2704/849) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

289 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-11-23..2020-11-30

* [upgrade the coverage map to Version 4](https://github.com/rust-lang/rust/pull/79365)
* [allow using generic trait methods in `const fn`](https://github.com/rust-lang/rust/pull/79287)
* [allow Trait inheritance with cycles on associated types](https://github.com/rust-lang/rust/pull/79209)
* [do not visit `ForeignItemRef` for HIR indexing and validation](https://github.com/rust-lang/rust/pull/79511)
* [only create `OnDiskCache` in incremental compilation mode](https://github.com/rust-lang/rust/pull/79216)
* [cache pretty-print/retokenize result to avoid compile time blowup](https://github.com/rust-lang/rust/pull/79338)
* [stabilize `const_int_pow`](https://github.com/rust-lang/rust/pull/76829)
* [compiler-builtins: fix division on SPARC](https://github.com/rust-lang/compiler-builtins/pull/393)
* [libtest: print the total time taken to execute a test suite](https://github.com/rust-lang/rust/pull/75752)
* [accept '!' in intra-doc links](https://github.com/rust-lang/rust/pull/79321)
* [cleanup more of rustdoc](https://github.com/rust-lang/rust/pull/79372)
* [bindgen: struct_layout: fix field offset computation for packed(n) structs](https://github.com/rust-lang/rust-bindgen/pull/1935)
* [miri: add simple data-race detector](https://github.com/rust-lang/miri/pull/1617)
* [clippy: add `suspicious_operation_groupings` lint](https://github.com/rust-lang/rust-clippy/pull/6086)

## Rust Compiler Performance Triage

* [2020-11-24](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-24.md):
1 Regression, 2 Improvements, 2 mixed

This week saw landing of [#79237](https://github.com/rust-lang/rust/pull/79237) which by itself provides no wins but opens the door to support for split debuginfo on macOS. This'll eventually show huge wins as we can likely avoid re-collecting debuginfo while retaining support for lldb and Rust backtraces. [#79361](https://github.com/rust-lang/rust/issues/79361) tracks the stabilization of the rustc flag, but the precise rollout to stable users is not yet 100% clear.

Triage done by @jyn514 and @simulacrum.

4 regressions, 4 improvements, 2 mixed results.
5 of them in rollups.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-24.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Plan to make core and std's panic identical](https://github.com/rust-lang/rfcs/pull/3007)
* [Stabilize Cargo's new feature resolver](https://github.com/rust-lang/rfcs/pull/2957)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [Use true previous lint level when detecting overriden forbids](https://github.com/rust-lang/rust/pull/78864)
* [Apply `unused_doc_comments` lint to inner items](https://github.com/rust-lang/rust/pull/78367)
* [remove this weird special case from promotion](https://github.com/rust-lang/rust/pull/78363)

## New RFCs

* [Allow "artifact dependencies" on bin, cdylib, and staticlib crates](https://github.com/rust-lang/rfcs/pull/3028)
* [Infallible promotion](https://github.com/rust-lang/rfcs/pull/3027)

# Upcoming Events

### Online
* [December 2, Johannesburg, ZA - Monthly Joburg Rust Chat - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/274734310/)
* [December 2, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/jhfstrybcqbdb/)
* [December 8, Saarbücken, Saarland, DE - Meetup: 6u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/274592167)
* [December 8, Stuttgart, DE - TALK: Running Multi-Module Heterogenous WASM Assemblies - Rust Community Stuttgart](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/274921745/)
* [December 8, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybcqblb/)
* [December 10, Stuttgart, DE - Hack & Learn - Directions for 2021 - Rust Community Stuttgart](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/274892215/)
* [December 10, San Diego, CA, US - San Diego Rust December 2020 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/274757235/)
* [December 15, Russia - Russian Rust Online Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/274924961/)
 
### North America
* [December 9, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgrybcqbmb/)
* [December 10, Provo, UT, US - Mob Programming: Add `--tree -d` to `lsd`](https://www.meetup.com/utah-rust/events/273530244/)

### Asia Pacific
* [December 7, Auckland, NZ - Rust AKL - Show and Tell + Introduction to RUst II](https://www.meetup.com/rust-akl/events/266876724/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Let’s be clear: We understand that we are net beneficiaries of the exceptional work that others have done to make Rust thrive. AWS didn’t start Rust or make it the success that it is today, but we’d like to contribute to its future success.

– [Matt Asay on the AWS Open Source blog](https://aws.amazon.com/blogs/opensource/why-aws-loves-rust-and-how-wed-like-to-help/)

Thanks to [Alice Ryhl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/969) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
