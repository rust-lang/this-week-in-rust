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
* [Rust Guide: Generics Part 2](https://jeffa.io/rust_guide_generics_demystified_part_2)

### Miscellaneous

* [Academy Software Foundation Announces Formation of Rust Working Group, Initial Release of OpenEXR Rust Binding](https://www.aswf.io/news/academy-software-foundation-announces-formation-of-rust-working-group-initial-release-of-openexr-rust-binding/)

## Crate of the Week

This week's crate is [flutter\_rust\_bridge](https://github.com/fzyzcjy/flutter_rust_bridge), a memory-safe binding generator for Flutter/Dart ↔ Rust.

Thanks to [fzyzcjy](https://users.rust-lang.org/t/crate-of-the-week/2704/972) for the suggestion!

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

* [add new tier-3 target: armv7-unknown-linux-uclibceabihf](https://github.com/rust-lang/rust/pull/88952)
* [perform type inference in range pattern](https://github.com/rust-lang/rust/pull/88090)
* [add abs_diff for integer types](https://github.com/rust-lang/rust/pull/88780)
* [implement #85440 (Random test ordering)](https://github.com/rust-lang/rust/pull/89082)
* [correctly handle supertraits for min_specialization](https://github.com/rust-lang/rust/pull/89413)
* [consider unfulfilled obligations in binop errors](https://github.com/rust-lang/rust/pull/89323)
* [add `deref_into_dyn_supertrait` lint](https://github.com/rust-lang/rust/pull/89461)
* [note specific regions involved in 'borrowed data escapes' error](https://github.com/rust-lang/rust/pull/89501)
* [fix suggestion to borrow when casting from pointer to reference](https://github.com/rust-lang/rust/pull/89528)
* [feature gate the non_exhaustive_omitted_patterns lint](https://github.com/rust-lang/rust/pull/89428)
* [fix ICE caused by non_exaustive_omitted_patterns struct lint](https://github.com/rust-lang/rust/pull/89423)
* [perf: only check for `rustc_trivial_field_reads` attribute on traits, not items, impls, etc.](https://github.com/rust-lang/rust/pull/89454)
* [perf: introduce `tcx.get_diagnostic_name`](https://github.com/rust-lang/rust/pull/89534)
* [improved help message for `suspicious_map`](https://github.com/rust-lang/rust-clippy/pull/7770)
* [emit item no type error even if type inference fails](https://github.com/rust-lang/rust/pull/89585)
* [optimize File::read_to_end and read_to_string](https://github.com/rust-lang/rust/pull/89582)
* [prevent error reporting from outputting a recursion error if it finds an ambiguous trait impl during suggestions](https://github.com/rust-lang/rust/pull/89576)
* [create more accurate debuginfo for vtables.](https://github.com/rust-lang/rust/pull/89597)
* [make cfg imply doc(cfg)](https://github.com/rust-lang/rust/pull/89596)
* [show detailed expected/found types in error message when trait paths are the same](https://github.com/rust-lang/rust/pull/89633)
* [fix docblock code display on mobile](https://github.com/rust-lang/rust/pull/89632)
* [use correct edition for panic in (`debug_`)`assert!()`](https://github.com/rust-lang/rust/pull/89622)
* [add `core::array::from_fn` and `core::array::try_from_fn`](https://github.com/rust-lang/rust/pull/75644)
* [add `Ipv6Addr::is_benchmarking`](https://github.com/rust-lang/rust/pull/86434)
* [add functions to add unsigned and signed integers](https://github.com/rust-lang/rust/pull/87601)
* [implement advance_(back_)_by on more iterators](https://github.com/rust-lang/rust/pull/87091)
* [array `.len()` MIR optimization pass](https://github.com/rust-lang/rust/pull/86525)
* [`path.push()` should work as expected on windows verbatim paths](https://github.com/rust-lang/rust/pull/89270)
* [use get_unchecked in `str::`(`r`)`split_once`](https://github.com/rust-lang/rust/pull/89219)
* [stabilize `try_reserve`](https://github.com/rust-lang/rust/pull/87993)
* [stabilize `proc_macro::is_available`](https://github.com/rust-lang/rust/pull/89735)
* [stabilize `const_panic`](https://github.com/rust-lang/rust/pull/89508)
* [stabilize `command_access`](https://github.com/rust-lang/rust/pull/88436)
* [futures: make `futures::task::noop_waker_ref` available without `std`.](https://github.com/rust-lang/futures-rs/pull/2505)
* [`rustc_codegen_gcc`: add missing cast and change some bitcasts to casts to avoid a gimple verification failure](https://github.com/rust-lang/rustc_codegen_gcc/pull/100)
* [rustfmt: stabilize `match_block_trailing_comma`](https://github.com/rust-lang/rustfmt/pull/5020)
* [rustfmt: wrap long array and slice patterns.](https://github.com/rust-lang/rustfmt/pull/4994)
* [rustdoc: migrate to table so the gui can handle >2k constants](https://github.com/rust-lang/rust/pull/88816)
* [clippy: add `undocumented_unsafe_blocks` lint](https://github.com/rust-lang/rust-clippy/pull/7748)
* [clippy: fix false positive in external macros for `mut_mut` lint](https://github.com/rust-lang/rust-clippy/pull/7795)
* [clippy: fix false positive when `Drop` and `Copy` involved in `field_reassign_with_default` lint](https://github.com/rust-lang/rust-clippy/pull/7794)
* [clippy: handle intra-doc links in `doc_markdown`](https://github.com/rust-lang/rust-clippy/pull/7772)
* [clippy: refactor `clippy::match_ref_pats` to check for multiple reference patterns](https://github.com/rust-lang/rust-clippy/pull/7800)
* [clippy: make `shadow_reuse` suggestion less verbose](https://github.com/rust-lang/rust-clippy/pull/7782)
* [clippy: add option to `new_lint` to generate MSRV enabled lint](https://github.com/rust-lang/rust-clippy/pull/7793)
* [clippy: drop exponent on suggestion when exponent value is zero](https://github.com/rust-lang/rust-clippy/pull/7774)

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
* [disposition: merge] [Tracking Issue for methods to go from nul-terminated `Vec<u8>` to CString ](https://github.com/rust-lang/rust/issues/73179)

### New RFCs

*No new RFCs were proposed this week.*

## Upcoming Events

### Online

* [October 13, 2021 - betterCode Rust](https://rust.bettercode.eu/)
* [October 13, 2021 - C++/Rust: Learning from Each Other - MUC++](https://www.meetup.com/MUCplusplus/events/281231257)
* [October 13, 2021, Los Angeles, CA, US - Processing shaders in Rust with Dzmitry Malyshau - Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/280981968/)
* [October 20, 2021, Buffalo, NY, US - Buffalo Rust User Group, Alternate Day - Buffalo Rust](https://www.meetup.com/Buffalo-Rust-Meetup/events/281236385/)
* [October 20, 2021, Vancouver, BC, CA - WASM plugin for Istio - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccnbbc/)

### North America

* [October 13, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccnbrb/)
* [October 22, 2021, IR - The First Rust Iran online meetup - Rust Iran Meetup](https://rust-meetup.ir/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is the language where you get the hangover first.

– [unattributed via Niko Matsakis' RustConf keynote](https://www.youtube.com/watch?v=ylOpCXI2EMM&t=565s)

Thanks to [Alice Ryhl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1122) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
