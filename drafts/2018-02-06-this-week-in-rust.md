Title: This Week in Rust 220
Number: 220
Date: 2018-02-06
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

- [podcast] [New Rustacean: e022 – `Send` and `Sync`: The “marker” traits Rust uses for safe concurrency](http://www.newrustacean.com/show_notes/e022/)

## #Rust2018

Find all #Rust2018 posts at [Read Rust](http://readrust.net/rust2018/).

# Crate of the Week

This week's crate is [datafusion](https://www.datafusion.rs), a query planner/execution framework for Big Data processing. Thanks to [andygrove](https://users.rust-lang.org/u/andygrove) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [Help Rayon prepare for 1.0](https://users.rust-lang.org/t/rayon-1-0-on-feb-14/14950).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

115 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-29..2018-02-05

* [syntax: lower priority of `+` in `impl Trait`/`dyn Trait`](https://github.com/rust-lang/rust/pull/45294)
* [improve char escaping in lexer messages](https://github.com/rust-lang/rust/pull/47914)
* [stabilize `feature(match_beginning_vert)`](https://github.com/rust-lang/rust/pull/47947)
* [add a `-Z no-index-update` for crater and benchmarking](https://github.com/rust-lang/cargo/pull/4990)
* [fix ICE when reading non-UTF-8 input from stdin](https://github.com/rust-lang/rust/pull/47895)
* [use a range to identify SIGSEGV in stack guards](https://github.com/rust-lang/rust/pull/47912)
* [fix overflow when performing drop check calculations in NLL](https://github.com/rust-lang/rust/pull/47920)
* [fix ref-to-ptr coercions not working with NLL in certain cases](https://github.com/rust-lang/rust/pull/47873)
* [fix ICE when assigning references to a static mut with NLL](https://github.com/rust-lang/rust/pull/47898)
* [make region inference use a dirty list](https://github.com/rust-lang/rust/pull/47766)
* [add approximate suggestions for rustfix](https://github.com/rust-lang/rust/pull/47540)
* [add line numbers and columns to error messages spanning multiple files](https://github.com/rust-lang/rust/pull/47780)
* [don't lint unnecessary parens in function or method arguments inside of nested macros](https://github.com/rust-lang/rust/pull/47896)
* [avoid underflow in render_source_line](https://github.com/rust-lang/rust/pull/47677)
* [minimize weird spans involving macro context](https://github.com/rust-lang/rust/pull/47942)
* [tweak presentation on lifetime trait mismatch](https://github.com/rust-lang/rust/pull/47791)
* [suggest removing value from `break` when invalid](https://github.com/rust-lang/rust/pull/47829)
* [fix regression: account for trait methods in arg count mismatch error](https://github.com/rust-lang/rust/pull/47844)
* [cleanup the shim code](https://github.com/rust-lang/rust/pull/47865)
* [implement `Send` for `process::Command` on Unix](https://github.com/rust-lang/rust/pull/47760)
* [specialize `StepBy::nth`](https://github.com/rust-lang/rust/pull/47552)
* [move `Duration` to libcore](https://github.com/rust-lang/rust/pull/46666)
* [rustbuild: per-stage `RUSTFLAGS`](https://github.com/rust-lang/rust/pull/47836)
* [cargo: allow configuration of LTO in `[profile]`](https://github.com/rust-lang/cargo/pull/4984)
* [rustdoc: fix const evaluation ICE](https://github.com/rust-lang/rust/pull/47862)
* [rustdoc: fix link title rendering with hoedown](https://github.com/rust-lang/rust/pull/47855)

## New Contributors

* evelynmitchell
* Gilad Naaman

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2091: Implicit caller location](https://github.com/rust-lang/rfcs/pull/2091).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [impl-only-use](https://github.com/rust-lang/rfcs/pull/2166). The `use …::{… as …}` syntax can now accept `_` as alias to a trait to only import the implementations of such a trait.
* [disposition: merge] [or-patterns in if / while let expressions](https://github.com/rust-lang/rfcs/pull/2175).
* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [Rust 2018 roadmap](https://github.com/rust-lang/rfcs/pull/2314).
* [Tighter coupling of Cargo workspaces](https://github.com/rust-lang/rfcs/pull/2315).
* [DynSized without ?DynSized — Lint against use of `extern type` in `size_of_val`, and more](https://github.com/rust-lang/rfcs/pull/2310).
* [Overconstraining and omitting `unsafe` in impls of `unsafe` trait methods](https://github.com/rust-lang/rfcs/pull/2316).

# Upcoming Events

* [Feb  2 - Feb 4. Multiple locations - Rust Roadshow Brazil 2018](https://mozillabr.org/2017/12/anunciando-o-rust-roadshow-brasil-2018-para-mobilizadores-de-todo-o-brasil/).
* [Feb  3 - Feb 4. Brussels - FOSDEM Rust Devroom](https://fosdem.org/2018/schedule/track/rust/)
* [Feb  4. Rust Bangalore - Rust for newbies (Part 2 of 12)](https://www.meetup.com/rustox/events/247201900/).
* [Feb  6. Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/).
* [Feb  7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb  7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb  7. Indianapolis - Indy.rs - February 2018](https://www.meetup.com/indyrs/events/246726699/).
* [Feb  7. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxdbkb/).
* [Feb  8. Helsinki - Finland Rust-lang Group](https://www.meetup.com/Finland-Rust-Meetup/events/246866694/).
* [Feb  8. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxdblb/).
* [Feb  8. San Diego Rust February Meetup](https://www.meetup.com/San-Diego-Rust/events/246906809/).
* [Feb  8. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb  9. Rust Table of Regulars Darmstadt / Germany](https://www.meetup.com/Rust-Rhein-Main/events/246744631)
* [Feb 10. Mangalore, India - RUSTCON2k18](https://www.rustcon2k18.in/).
* [Feb 12. Rust London User Group - LDN Talks: February 2018](https://www.meetup.com/Rust-London-User-Group/events/246860921/).
* [Feb 12. Rust Amsterdam - Perl FFI && Long-term reliability in Rust projects](https://www.meetup.com/Rust-Amsterdam/events/247120013/).
* [Feb 12. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxdbqb/).
* [Feb 14. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 14. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 15. Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnyxdbtb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at MaidSafe](https://maidsafe.net/careers.html#rust_engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Failure is not an OPTION<T\>.
> It’s a Result<T, E\>.

— [llogiq on Twitter](https://twitter.com/llogiq/status/956051804374134785).

Thanks to [nasa42 for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/484)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
