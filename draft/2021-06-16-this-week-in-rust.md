Title: This Week in Rust 395
Number: 395
Date: 2021-06-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-sort](https://github.com/DevinR528/cargo-sort), a cargo subcommand to sort your `Cargo.toml`'s dependencies and workspace members.

Thanks to [jplatte](https://users.rust-lang.org/t/crate-of-the-week/2704/921) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

267 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-31..2021-06-07

* [BPF target support](https://github.com/rust-lang/rust/pull/79608)
* [support for force-warns](https://github.com/rust-lang/rust/pull/85788)
* [improve debugging experience for enums on windows-msvc](https://github.com/rust-lang/rust/pull/85292)
* [parser: ensure that all nonterminals have tokens after parsing](https://github.com/rust-lang/rust/pull/84995)
* [don't suggest unsized indirection in where-clauses](https://github.com/rust-lang/rust/pull/85979)
* [rustc: allow safe `#[target_feature]` on wasm](https://github.com/rust-lang/rust/pull/84988)
* [always go through the `expn_that_defined` query](https://github.com/rust-lang/rust/pull/86002)
* [perf: miscellaneous inlining improvements](https://github.com/rust-lang/rust/pull/85892)
* [perf: only compute the trait map once](https://github.com/rust-lang/rust/pull/85905)
* [stabilize `vecdeque_binary_search`](https://github.com/rust-lang/rust/pull/83362)
* [update standard library for `IntoIterator` implementation of arrays](https://github.com/rust-lang/rust/pull/85930)
* [clippy: don't warn about `cfg!(..)` as a constant in assertions](https://github.com/rust-lang/rust-clippy/pull/7319)
* [clippy: fix `needless_collect` with binding shadowing](https://github.com/rust-lang/rust-clippy/pull/7289)
* [clippy: add lint `manual_str_repeat`](https://github.com/rust-lang/rust-clippy/pull/7265)

### Rust Compiler Performance Triage


Some good improvements, and a few regressions. No large changes.

Triage done by **@simulacrum**.
Revision range: [1160cf..a50d721](https://perf.rust-lang.org/?start=1160cf864f2a0014e3442367e1b96496bfbeadf4&end=a50d72158e08e02cfc051b863017bdbd2c45b637&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 1 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-08.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No new RFCs were proposed this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [Add the boxed!() macro to "de-magic" box syntax](https://github.com/rust-lang/rfcs/pull/3057)
* [disposition: merge] [RFC: Supertrait item shadowing](https://github.com/rust-lang/rfcs/pull/2845)
* [disposition: merge] [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize span_open() and span_close().](https://github.com/rust-lang/rust/pull/86136)
* [disposition: merge] [Stabilize ops::ControlFlow (just the type)](https://github.com/rust-lang/rust/pull/85608)
* [disposition: merge] [Re-add support for parsing (and pretty-printing) inner-attributes in match body](https://github.com/rust-lang/rust/pull/85193)
* [disposition: merge] [Ignore derived Clone and Debug implementations during dead code analysis](https://github.com/rust-lang/rust/pull/85200)
* [disposition: merge] [Tracking Issue for const core::str::from_utf8_unchecked](https://github.com/rust-lang/rust/issues/75196)

### New RFCs

* [RFC: let-else statements](https://github.com/rust-lang/rfcs/pull/3137)

## Upcoming Events

### Online

* [June 15, 2021, Washington, DC, US - In-kernel, fast-path packet processing with AF_XDP - Rust DC](https://www.meetup.com/RustDC/events/vdhxgsyccjbtb)
* [June 16, 2021, Vancouver, BC, CA - Rust in Mozilla's Data Platform - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/fqpkjsyccjbvb/)
* [June 17, 2021, Denver, CO, US - Learning Rust as a Python/Javascript developer by Juhis - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/277575285/)
* [June 18, 2021, Online - Learn Rust Fundamentals | Rust 101 - KubeDaily](https://www.youtube.com/watch?v=DIxjk0HTx5U)
* [June 29. 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccjbmc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> As the tradeoffs in software engineering change over time, so does the ideal solution. Some 40 years ago when the first C standards were written down, by people no less competent than those that work on Rust today, the design of the language and the list of behaviours not defined likely made much more sense in context of back then than they do right now. It is not all that unlikely that some years down the line the choices made by Rust won't make all that much of sense as they do today, too.

– [Simonas on rust-internals](https://users.rust-lang.org/t/why-deference-maybeuninit-unint-as-mut-ptr-is-safe/60344/19)

Thanks to [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1055) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
