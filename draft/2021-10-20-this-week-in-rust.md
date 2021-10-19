Title: This Week in Rust 413
Number: 413
Date: 2021-10-20
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

### Newsletters

### Observations/Thoughts

### Rust Walkthroughs

* [Custom Logging in Rust using tracing and tracing-subscriber](https://burgers.io/custom-logging-in-rust-using-tracing)

### Miscellaneous

* [Academy Software Foundation Announces Formation of Rust Working Group, Initial Release of OpenEXR Rust Binding](https://www.aswf.io/news/academy-software-foundation-announces-formation-of-rust-working-group-initial-release-of-openexr-rust-binding/)

## Crate of the Week

This week's crate is [serde\_with](https://docs.rs/serde_with), a crate of helper macros to ease implementing serde traits for your types.

Thanks to [piegames](https://users.rust-lang.org/t/crate-of-the-week/2704/971) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [This Week in Rust is looking for additional editors](https://github.com/rust-lang/this-week-in-rust/issues/2469)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

353 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-10-04..2021-10-11

* [fix ICE with `let...else` and `ref mut`](https://github.com/rust-lang/rust/pull/89965)
* [fix an ICE with TAITs and Future](https://github.com/rust-lang/rust/pull/89946)
* [suggest  Box::pin` when `Pin::new` is used instead](https://github.com/rust-lang/rust/pull/89870)
* [fix incorrect `Box::pin` suggestion](https://github.com/rust-lang/rust/pull/89390)
* [nicer error message if the user attempts to do `let`...`else if`](https://github.com/rust-lang/rust/pull/89974)
* [index and hash HIR as part of lowering](https://github.com/rust-lang/rust/pull/89124)
* [polymorphization: shims and predicates](https://github.com/rust-lang/rust/pull/89514)
* [optimize `VecDeque::append`](https://github.com/rust-lang/rust/pull/88717)
* [speedup int `log10` branchless](https://github.com/rust-lang/rust/pull/88788)
* [stabilize `unreachable_unchecked` as `const fn`](https://github.com/rust-lang/rust/pull/89509)
* [use `BCryptGenRandom` instead of `RtlGenRandom` on Windows.](https://github.com/rust-lang/rust/pull/84096)
* [make `Option::as_mut` const](https://github.com/rust-lang/rust/pull/89953)
* [make `Result::as_mut` const](https://github.com/rust-lang/rust/pull/89977)
* [add `slice::swap_unchecked`](https://github.com/rust-lang/rust/pull/88540)
* [add `#[repr(i8)]` to `Ordering`](https://github.com/rust-lang/rust/pull/89507)
* [add `Poll::ready` and revert stabilization of `task::ready!`](https://github.com/rust-lang/rust/pull/89651)
* [avoid allocations and copying in `Vec::leak`](https://github.com/rust-lang/rust/pull/89337)
* [rustdoc: associated consts sidebar](https://github.com/rust-lang/rust/pull/89815)
* [rustfmt: stabilize `disable_all_formatting`](https://github.com/rust-lang/rustfmt/pull/5026)
* [clippy: fix false positive of `implicit_saturating_sub` with `else` clause](https://github.com/rust-lang/rust-clippy/pull/7832)
* [clippy: do not expand macros in `equatable_if_let` suggestion](https://github.com/rust-lang/rust-clippy/pull/7788)
* [clippy: `unnecessary_sort_by` checks if argument implements `Ord` trait](https://github.com/rust-lang/rust-clippy/pull/7824)
* [clippy: allow giving reasons for `disallowed_types`](https://github.com/rust-lang/rust-clippy/pull/7791)
* [clippy: implement `uninit_vec` lint](https://github.com/rust-lang/rust-clippy/pull/7682)
* [clippy: add `format_in_format_args` and `to_string_in_format_args` lints](https://github.com/rust-lang/rust-clippy/pull/7743)
* [clippy: add lint `transmute_num_to_bytes`](https://github.com/rust-lang/rust-clippy/pull/7805)
* [clippy: add `match_str_case_mismatch` lint](https://github.com/rust-lang/rust-clippy/pull/7806)

### Rust Compiler Performance Triage

A relatively quiet week: two smallish regressions, and one largish regression that is isolated to doc builds. A couple of nice small wins as well.

Triage done by **@pnkfelix**.
Revision range: [25ec82..9475e6](https://perf.rust-lang.org/?start=25ec8273855fde2d72ae877b397e054de5300e10&end=9475e609b8458fff9e444934a6017d2e590642cf&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 2 Mixed; 1 of them in rollups
42 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-10-12.md)

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

* [disposition: merge] [Stabilize is_symlink() for Metadata and Path](https://github.com/rust-lang/rust/pull/89677)
* [disposition: merge] [Partially stabilize duration_consts_2](https://github.com/rust-lang/rust/pull/89542)
* [disposition: merge] [Stabilize unreachable_unchecked as const fn](https://github.com/rust-lang/rust/pull/89509)
* [disposition: merge] [Add `#[repr(i8)]` to Ordering](https://github.com/rust-lang/rust/pull/89507)
* [disposition: merge] [Fix ctrl-c causing reads of stdin to return empty on Windows.](https://github.com/rust-lang/rust/pull/89433)
* [disposition: merge] [linux/aarch64 Now() should be actually_monotonic()](https://github.com/rust-lang/rust/pull/88652)
* [disposition: merge] [Stabilise unix_process_wait_more, extra ExitStatusExt methods](https://github.com/rust-lang/rust/pull/88300)
* [disposition: merge] [Make all proc-macro back-compat lints deny-by-default](https://github.com/rust-lang/rust/pull/88041)
* [disposition: merge] [Windows: Resolve process::Command program without using the current directory](https://github.com/rust-lang/rust/pull/87704)
* [disposition: merge] [Implement RefUnwindSafe for `Rc<T>`](https://github.com/rust-lang/rust/pull/87467)
* [disposition: merge] [Make two Paths unequal if they differ in trailing slash](https://github.com/rust-lang/rust/pull/87339)
* [disposition: merge] [Reject octal zeros in IPv4 addresses](https://github.com/rust-lang/rust/pull/86984)
* [disposition: merge] [Automatic exponential formatting in Debug](https://github.com/rust-lang/rust/pull/86479)
* [disposition: merge] [Tracking Issue for methods to go from nul-terminated `Vec<u8>` to CString](https://github.com/rust-lang/rust/issues/73179)

### New RFCs

*No new RFCs were proposed this week.*

## Upcoming Events

### Online

* [October 13, 2021 - betterCode Rust](https://rust.bettercode.eu/)
* [October 13, 2021 - C++/Rust: Learning from Each Other - MUC++](https://www.meetup.com/MUCplusplus/events/281231257)
* [October 13, 2021, Los Angeles, CA, US - Processing shaders in Rust with Dzmitry Malyshau - Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/280981968/)
* [October 20, 2021, Buffalo, NY, US - Buffalo Rust User Group, Alternate Day - Buffalo Rust](https://www.meetup.com/Buffalo-Rust-Meetup/events/281236385/)
* [October 20, 2021, Vancouver, BC, CA - WASM plugin for Istio - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccnbbc/)
* [October 22, 2021, Iran - The First Rust Iran online meetup - Rust Iran Meetup](https://rust-meetup.ir/)

### North America

* [October 13, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccnbrb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

**Bytewax**

* [Senior Software Engineer (Remote)](https://bytewax.notion.site/Senior-Software-Engineer-9d83531eb0704afd8f323e4080e0d620)

# Quote of the Week

> The biggest failure in Rust‘s communication strategy has been the inability to explain to non-experts that unsafe abstractions are the point, not a sign of failure.

– [withoutboats on twitter](https://mobile.twitter.com/withoutboats/status/1447512045558149122)

Thanks to [Alice Ryhl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1124) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
