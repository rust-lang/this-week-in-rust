Title: This Week in Rust 414
Number: 414
Date: 2021-10-27
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Foundation

### Project/Tooling Updates

* [SixtyFPS (GUI crate) weekly report for 24th of October 2021](https://sixtyfps.io/thisweek/2021-10-25.html)
* [Releasing Gloo v0.4.0: Gloo is a modular toolkit for building fast, reliable Web applications and libraries with Rust and Wasm ](https://gloo-rs.web.app/blog/release-0.4.0/)

### Newsletters

### Observations/Thoughts

* [Async on Embedded: Present & Future](https://tweedegolf.nl/blog/63/async-on-embedded-present-and-future)
* [No Time for Chrono](https://passcod.name/technical/no-time-for-chrono.html)

### Rust Walkthroughs

* [series] [Build Your Text Editor With Rust! Final Part](https://medium.com/@otukof/build-your-text-editor-with-rust-final-part-4c841a649900)

* [Custom Logging in Rust using tracing and tracing-subscriber, part 2](https://burgers.io/custom-logging-in-rust-using-tracing-part-2)

### Miscellaneous

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

* [wahn/rs_pbrt - [parse_blend_file] Attic Close-up](https://github.com/wahn/rs_pbrt/issues/136)

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

A week where improvements outweigh regressions. The highlight of the week is the change to split out LLVM profile guided optimization (PGO) and using clang 13 to compile LLVM which led to improvements in many real world crates (e.g., cargo) in the range of 10%. Most regressions were limited and at most in the less than 1% range. We are seeing more performance changes in rollups which are supposed to be performance neutral. We'll have to decide how to best address this.

Triage done by **@rylev**.
Revision range: [9475e609..d45ed750](https://perf.rust-lang.org/?start=9475e609b8458fff9e444934a6017d2e590642cf&end=d45ed7502ad225739270a368528725930f54b7b6&absolute=false&stat=instructions%3Au)

3 Regressions, 4 Improvements, 2 Mixed; 2 of them in rollups;
34 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-10-19.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Thread local Cell methods.](https://github.com/rust-lang/rfcs/pull/3184)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Update `std::env::temp_dir` to use GetTempPath2 on Windows when available.](https://github.com/rust-lang/rust/pull/89999)
* [disposition: merge] [Implement `Termination` for `Result<Infallible, E>`](https://github.com/rust-lang/rust/pull/88601)
* [disposition: close] [Port clippy lint `redundant_field_names` to compiler](https://github.com/rust-lang/rust/pull/87512)
* [disposition: merge] [Stabilize `File::options()`](https://github.com/rust-lang/rust/pull/85766)
* [disposition: merge] [Tracking Issue for relaxed struct unsizing rules](https://github.com/rust-lang/rust/issues/81793)
* [disposition: merge] [Tracking Issue for `option_result_unwrap_unchecked`](https://github.com/rust-lang/rust/issues/81383)
* [disposition: merge] [Tracking Issue for `destructuring_assignment`](https://github.com/rust-lang/rust/issues/71126)
* [disposition: close] [Tracking issue for `slice_concat_ext` stabilization](https://github.com/rust-lang/rust/issues/27747)

### New RFCs

*No new RFCs were proposed this week.*

## Upcoming Events

### Online

* [October 27, 2021, London, UK - Rust London Ockam Takeover - Rust London User Group](https://skillsmatter.com/meetups/13606-rust-london-october2021#community)
* [October 27, 2021, Phoenix, AZ - Desert Rust Halloween - Desert Rust](https://www.meetup.com/Desert-Rustaceans/events/281215858/)
* [October 28, 2021, Copenhagen, DK - Hack Night #22 - Copenhagen Rust Group](https://cph.rs/)
* [November 2, 2021, Buffalo, NY, US - First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/281558952/)
* [November 9, 2021, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccpbmb/)
* [November 10, 2021, Malaysia - Rust Meetup - Rust Malaysia](https://discord.gg/9Xj8H2EXTD)

### North America

* [November 10, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccpbnb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Snapview**

* [(Senior) Rust Backend Engineer (m/f/d (Remote)](https://snapview.jobs.personio.de/job/381815?display=en&language=en)

**Quickwit**

* [Senior Rust Engineer (Remote: UTC-1..=UTC+9)](https://quickwit.io/jobs/distributed-software-engineer)

**Seaplane**

* [Rust Software Engineer (Remote)](https://apply.workable.com/seaplane/j/CEFCB5F5B8/)

**Kraken**

* [Several Rust Engineering Positions Available](https://jobs.lever.co/kraken?team=Engineering)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The biggest failure in Rust‘s communication strategy has been the inability to explain to non-experts that unsafe abstractions are the point, not a sign of failure.

– [withoutboats on twitter](https://mobile.twitter.com/withoutboats/status/1447512045558149122)

Thanks to [Alice Ryhl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1124) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
