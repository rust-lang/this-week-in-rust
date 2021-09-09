Title: This Week in Rust 407
Number: 407
Date: 2021-09-08
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

* [Had a blast porting one of my serverless applications from Go to Rust - some things I learned](https://twitter.com/mlafeldt/status/1433414474589954048)

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov), a cargo subcommand for LLVM-based code coverage.

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/crate-of-the-week/2704/948) for the suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

296 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-08-23..2021-08-30

* [fix debugger stepping behavior with match expressions](https://github.com/rust-lang/rust/pull/87832)
* [improve liveness analysis for generators](https://github.com/rust-lang/rust/pull/84333)
* [handle match statements with non exhaustive variants in closures](https://github.com/rust-lang/rust/pull/88280)
* [`ast_lowering`: introduce `lower_span` for catching all spans entering HIR](https://github.com/rust-lang/rust/pull/88208)
* [PGO for LLVM builds on `x86_64-unknown-linux-gnu` in CI](https://github.com/rust-lang/rust/pull/88069)
* [`Cow`'ify some `pprust` methods](https://github.com/rust-lang/rust/pull/88262)
* [polonius: move to a fully hand-written parser to improve compile / iteration times](https://github.com/rust-lang/polonius/pull/173)
* [warn about unreachable code following an expression with an uninhabited type](https://github.com/rust-lang/rust/pull/85556)
* [normalize projections under binders](https://github.com/rust-lang/rust/pull/85499)
* [stabilize and document `--force-warn`](https://github.com/rust-lang/rust/pull/87472)
* [stabilise `BufWriter::into_parts`](https://github.com/rust-lang/rust/pull/88299)
* [add `Cell::as_array_of_cells`](https://github.com/rust-lang/rust/pull/87944)
* [add `Saturating` type (based on `Wrapping` type)](https://github.com/rust-lang/rust/pull/87921)
* [stdarch: update codegen for simd wasm intrinsics with LLVM 13](https://github.com/rust-lang/stdarch/pull/1203)
* [futures: add `Peekable::`{`peek_mut`, `poll_peek_mut`}](https://github.com/rust-lang/futures-rs/pull/2488)
* [cargo: show description of well known subcommands (fmt, clippy) in `cargo --list`](https://github.com/rust-lang/cargo/pull/9848)
* [clippy: fix `option_if_let_else`](https://github.com/rust-lang/rust-clippy/pull/7573)
* [clippy: add `module_style` lint to style](https://github.com/rust-lang/rust-clippy/pull/7543)
* [clippy: don't report function calls as unnecessary operation if used in array index](https://github.com/rust-lang/rust-clippy/pull/7453)

### Rust Compiler Performance Triage

A very busy week with relatively even amounts of regressions and improvements (albeit with improvements outweighing regressions). The largest win was the use of profile-guided optimization (PGO) builds on x86_64 linux builds which brings fairly large improvements in real-world crates. There were 2 regressions that caused fairly large (~3.5%) regressions in real-world crates which need to be investigated.

Triage done by **@rylev**.
Revision range: [33fdb..fe379](https://perf.rust-lang.org/?start=33fdb797f59421c7bbecaa4588ed5d7a31a9494a&end=fe37929e4cba2c5c21e6805805769630c736bc3d&absolute=false&stat=instructions%3Au)

5 Regressions, 4 Improvements, 5 Mixed; 0 of them in rollups
56 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-01.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [Proposal: Else clauses for for and while loops](https://github.com/rust-lang/rfcs/pull/3163)
* [disposition: close] [RFC: let-expression](https://github.com/rust-lang/rfcs/pull/3159)
* [disposition: merge] [Scrape code examples from examples/ directory for Rustdoc](https://github.com/rust-lang/rfcs/pull/3123)
* [disposition: merge] [Rust-lang crate ownership policy](https://github.com/rust-lang/rfcs/pull/3119)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Partially stabilize array_methods](https://github.com/rust-lang/rust/pull/88353)
* [disposition: merge] [Stabilize std::os::unix::fs::chroot](https://github.com/rust-lang/rust/pull/88177)
* [disposition: merge] [Stabilize reserved prefixes](https://github.com/rust-lang/rust/issues/88140)
* [disposition: merge] [stabilize disjoint capture in closures (RFC 2229)](https://github.com/rust-lang/rust/issues/88126)
* [disposition: merge] [Stabilize try_reserve](https://github.com/rust-lang/rust/pull/87993)
* [disposition: merge] [Support #[track_caller] on closures and generators](https://github.com/rust-lang/rust/pull/87064)

### New RFCs

*No new RFCs were proposed this week.*

## Upcoming Events

### Online

* [September 2, 2021, Zurich, CH - Exciting new Rustdoc features landing in 1.55.0 - Hybrid Meetup (Livestream!) - Rust Zurich](https://www.meetup.com/Rust-Zurich/events/280295950/)
* [September 2, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [September 7, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/280433831/)
* [September 8, 2021, Denver, CO, US - Rust Q&A - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/279407152/)
* [September 14, 2021, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccmbsb/)
* [September 15, 2021, Vancouver, BC, CA - Considering Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccmbtb/)

### North America

* [September 8, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccmblb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Anyway: the standard library docs say "check the nomicon"  
> then the nomicon says "here is some advice and ultimately we don't know, maybe check UCG"  
> then UCG says "ultimately we don't know it's probably like this but there's no RFC yet"  
> then Ralf says "probably it should be allowed if the layout matches".

– [Lokathor on the Rust Zulip](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/rustc.20warn.20against.20repr.20rust.20transmutes/near/250735818)

Thanks to [Riccardo D'Ambrosio](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1097) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
