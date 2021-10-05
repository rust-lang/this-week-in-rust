Title: This Week in Rust 411
Number: 411
Date: 2021-10-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Project/Tooling Updates

### Research and Papers
* [SyRust: automatic testing of Rust libraries with semantic-aware program synthesis](https://dl.acm.org/doi/abs/10.1145/3453483.3454084)

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [miette](https://crates.io/crates/miette), a library for error handling that is beautiful both in code and output.

Thanks to [Kat Marchán](https://users.rust-lang.org/t/crate-of-the-week/2704/965) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

265 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-09-20..2021-09-27

* [suggest both of immutable and mutable trait implementations](https://github.com/rust-lang/rust/pull/89263)
* [give better error for `macro_rules! name!`](https://github.com/rust-lang/rust/pull/89221)
* [validate builtin attributes for macro args](https://github.com/rust-lang/rust/pull/88680)
* [implement `#[must_not_suspend]`](https://github.com/rust-lang/rust/pull/88865)
* [support `#[track_caller]` on closures and generators](https://github.com/rust-lang/rust/pull/87064)
* [make `#[track_caller]` actually do stuff in `Steal::borrow`](https://github.com/rust-lang/rust/pull/89237)
* [revise never type fallback algorithm](https://github.com/rust-lang/rust/pull/88804)
* [don't use projection cache or candidate cache in intercrate mode](https://github.com/rust-lang/rust/pull/89125)
* [don't normalize opaque types with escaping late-bound regions](https://github.com/rust-lang/rust/pull/89285)
* [disable visible path calculation for `PrettyPrinter` in `Ok` path of compiler](https://github.com/rust-lang/rust/pull/89120)
* [enable new pass manager with LLVM 13](https://github.com/rust-lang/rust/pull/88243)
* [simplify `scoped_thread`](https://github.com/rust-lang/rust/pull/89104)
* [stabilize `Iterator::map_while`](https://github.com/rust-lang/rust/pull/89086)
* [use ZST for `fmt` unsafety](https://github.com/rust-lang/rust/pull/89139)
* [rustfmt: trailing comma on match block goes missing when guard is on its own line](https://github.com/rust-lang/rustfmt/pull/4998)
* [rustfmt: simplify and speed up search for local path based deps with `cargo fmt --all`](https://github.com/rust-lang/rustfmt/pull/4997)
* [clippy: demote `float_cmp` to pedantic](https://github.com/rust-lang/rust-clippy/pull/7692)
* [clippy: new lint `if_then_panic`](https://github.com/rust-lang/rust-clippy/pull/7669)
* [clippy: stop `excessive_precision` from suggesting a float truncation that is not shorter](https://github.com/rust-lang/rust-clippy/pull/7722)
* [clippy: don't lint `suspicious_else_formatting` inside proc-macros](https://github.com/rust-lang/rust-clippy/pull/7707)

### Rust Compiler Performance Triage

The largest story for the week are the massive improvements that come from enabling the new pass manager in LLVM which leads to consistent 5% to 30% improvements across almost all test cases. The regressions were mostly minor with clear paths for addressing the ones that were not made with some specific trade off in mind.

Triage done by **@rylev**.
Revision range: [7743c9..83f147](https://perf.rust-lang.org/?start=7743c9fadd64886d537966ba224b9c20e6014a59&end=83f147b3baf21acfc367a6da1045d212cd3957e4&absolute=false&stat=instructions%3Au)

4 Regressions, 4 Improvements, 3 Mixed; 0 of them in rollups

43 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-28.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking Issue for saturating_div](https://github.com/rust-lang/rust/issues/89381)
* [disposition: merge] [Avoid allocations and copying in `Vec::leak`](https://github.com/rust-lang/rust/pull/89337)
* [disposition: merge] [Stabilize RFC 2345: Allow panicking in constants](https://github.com/rust-lang/rust/issues/89006)
* [disposition: merge] [Perform type inference in range pattern](https://github.com/rust-lang/rust/pull/88090)
* [disposition: merge] [Tracking issue for `proc_macro::is_available()`](https://github.com/rust-lang/rust/issues/71436)
* [disposition: close] [Tracking issue for `alloc::prelude`](https://github.com/rust-lang/rust/issues/58935)

### New RFCs

*No new RFCs were proposed this week.*

## Upcoming Events

### Online

* [September 30, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [October 2, 2021 - Rust Graphics meetup](https://github.com/gfx-rs/meetup)
* [October 5, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/280628523/)
* [October 7, 2021, Zürich, CH - Rust & GUI: egui - Rust Zürichsee](https://www.meetup.com/Rust-Zurich/events/280399418/)
* [October 9, 2021 - Rust Gamedev Discord - Rust Gamedev Monthly Meetup](https://discord.gg/yNtPTb2)
* [October 12, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccnbqb/)
* [October 13, 2021 - betterCode Rust](https://rust.bettercode.eu/)

### North America

* [October 13, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccnbrb/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

This week we have two great quotes!

> The signature of your function is your contract with not only the compiler, but also users of your function.

– [Quine Dot on rust-users](https://users.rust-lang.org/t/why-rust-lifetime-elision-cannot-inference-the-proper-lifetime-annotations-on-functions/65106/3)

> Do you want to know what was harder than learning lifetimes? Learning the same lessons through twenty years of making preventable mistakes.

– [Zac Burns in his RustConf talk](https://www.youtube.com/watch?v=4_Jg-rLDy-Y&t=1658s)

Thanks to [Daniel H-M](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1116) and [Erik Zivkovic](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1117) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
