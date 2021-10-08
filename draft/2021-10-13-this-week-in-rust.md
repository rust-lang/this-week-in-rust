Title: This Week in Rust 412
Number: 412
Date: 2021-10-13
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

### Newsletters

### Observations/Thoughts

### Rust Walkthroughs

## Crate of the Week

This week's crate is [pubgrub](https://crates.io/crates/pubgrub), a Rust implementation of the state of the art version solving algorithm.

Thanks to [Louis Pilfold](https://users.rust-lang.org/t/crate-of-the-week/2704/968) for the suggestion!

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

266 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-09-27..2021-10-04

* [make *const (), *mut () okay for FFI](https://github.com/rust-lang/rust/pull/84267)
* [resolve: cache module loading for all foreign modules](https://github.com/rust-lang/rust/pull/89239)
* [improve error message for missing angle brackets in `[_]::method`](https://github.com/rust-lang/rust/pull/89447)
* [avoid nondeterminism in `trimmed_def_paths`](https://github.com/rust-lang/rust/pull/89408)
* [improve error message for printf-style format strings](https://github.com/rust-lang/rust/pull/89340)
* [pick one possible lifetime in case there are multiple choices](https://github.com/rust-lang/rust/pull/89327)
* [suggest using the path separator for tuple struct](https://github.com/rust-lang/rust/pull/89293)
* [suggest similarly named associated items in trait impls](https://github.com/rust-lang/rust/pull/89248)
* [improve cause information for NLL higher-ranked errors](https://github.com/rust-lang/rust/pull/89249)
* [hide `<...> defined here` note if the source is not available](https://github.com/rust-lang/rust/pull/89233)
* [fix incorrect disambiguation suggestion for associated items](https://github.com/rust-lang/rust/pull/89255)
* [fix unsound optimization with explicit variant discriminants](https://github.com/rust-lang/rust/pull/89489)
* [don't anonymize bound region names during typeck](https://github.com/rust-lang/rust/pull/89250)
* [pass real crate-level attributes to `pre_expansion_lint`](https://github.com/rust-lang/rust/pull/89214)
* [use larger span for adjustment THIR expressions](https://github.com/rust-lang/rust/pull/89110)
* [coerce const FnDefs to implement const Fn traits](https://github.com/rust-lang/rust/pull/88963)
* [constify ?-operator for `Result` and `Option`](https://github.com/rust-lang/rust/pull/86853)
* [partially stabilize `array_methods`](https://github.com/rust-lang/rust/pull/88353)
* [avoid spurious "previous iteration of loop" errors](https://github.com/rust-lang/rust/pull/87998)
* [include the length in `BTree` hashes](https://github.com/rust-lang/rust/pull/89443)
* [optimize unnecessary check in `Vec::retain`](https://github.com/rust-lang/rust/pull/88060)
* [`VecDeque`: improve performance for `From<[T; N]`>](https://github.com/rust-lang/rust/pull/88452)
* [optimize `is_sorted` for `Range` and `RangeInclusive`](https://github.com/rust-lang/rust/pull/89335)
* [optimize `str::from_utf8()` validation when slice contains multibyte chars and `str.chars().count()` in all * [Fix `read_to_end` to not grow an exact size buffer](https://github.com/rust-lang/rust/pull/89165)
* [make `<[T]>::split_at_unchecked` and `<[T]>::split_at_mut_unchecked` public](https://github.com/rust-lang/rust/pull/87870)
* [mark unsafe methods `NonZero*::unchecked_`{`add`, `mul`} as const](https://github.com/rust-lang/rust/pull/87910)
* [const fn for `Option::`{`copied`, `take`, `replace`}](https://github.com/rust-lang/rust/pull/86828)
cases](https://github.com/rust-lang/rust/pull/88834)
* [hashbrown: relax the bounds on `HashSet`: `Debug`](https://github.com/rust-lang/hashbrown/pull/296)
* [clippy: correctly handle signs in exponents in `numeric_literal::format()`](https://github.com/rust-lang/rust-clippy/pull/7747)
* [clippy: make `if_then_panic` handle situation of `BinOpKind::And || BinOpKind::Or`](https://github.com/rust-lang/rust-clippy/pull/7741)
* [clippy: re-write `shadow` lints](https://github.com/rust-lang/rust-clippy/pull/7338)
* [clippy: make `doc_unsafe` warn on unsafe traits as well](https://github.com/rust-lang/rust-clippy/pull/7734)
* [clippy: fix bug for `large_enum_variants`](https://github.com/rust-lang/rust-clippy/pull/7677)
* [clippy: add new 'while_let_some_result' linting](https://github.com/rust-lang/rust-clippy/pull/7608)
* [clippy: add lint `equatable_if_let`](https://github.com/rust-lang/rust-clippy/pull/7762)
* [clippy: fix ICE in `implicit_hasher`](https://github.com/rust-lang/rust-clippy/pull/7761)
* [clippy: exclude enum from derivable impls](https://github.com/rust-lang/rust-clippy/pull/7755)

### Rust Compiler Performance Triage

A fairly busy week, with a relatively high percentage of PRs landing with
regressions and improvements. The overall trajectory is fairly neutral for this
week though.

Triage done by **@simulacrum**.
Revision range: [83f147b..25ec82](https://perf.rust-lang.org/?start=83f147b3baf21acfc367a6da1045d212cd3957e4&end=25ec8273855fde2d72ae877b397e054de5300e10&absolute=false&stat=instructions%3Au)

5 Regressions, 5 Improvements, 5 Mixed; 1 of them in rollups

43 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-10-05.md)

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

* [disposition: merge] [Partially stabilize duration_consts_2](https://github.com/rust-lang/rust/pull/89542)
* [disposition: merge] [Stabilize unreachable_unchecked as const fn](https://github.com/rust-lang/rust/pull/89509)
* [disposition: merge] [Add `#[repr(i8)]` to Ordering](https://github.com/rust-lang/rust/pull/89507)
* [disposition: merge] [Fix ctrl-c causing reads of stdin to return empty on Windows.](https://github.com/rust-lang/rust/pull/89433)
* [disposition: merge] [Tracking Issue for saturating_div](https://github.com/rust-lang/rust/issues/89381)
* [disposition: merge] [Avoid allocations and copying in `Vec::leak`](https://github.com/rust-lang/rust/pull/89337)
* [disposition: merge] [linux/aarch64 Now() should be actually_monotonic()](https://github.com/rust-lang/rust/pull/88652)
* [disposition: merge] [Stabilise unix_process_wait_more, extra ExitStatusExt methods](https://github.com/rust-lang/rust/pull/88300)
* [disposition: merge] [Make all proc-macro back-compat lints deny-by-default](https://github.com/rust-lang/rust/pull/88041)
* [disposition: merge] [Windows: Resolve process::Command program without using the current directory](https://github.com/rust-lang/rust/pull/87704)
* [disposition: merge] [Implement RefUnwindSafe for `Rc<T>`](https://github.com/rust-lang/rust/pull/87467)
* [disposition: merge] [Reject octal zeros in IPv4 addresses](https://github.com/rust-lang/rust/pull/86984)
* [disposition: merge] [Tracking Issue for methods to go from nul-terminated `Vec<u8>` to CString ](https://github.com/rust-lang/rust/issues/73179)
* [disposition: merge] [Tracking issue for `proc_macro::is_available()`](https://github.com/rust-lang/rust/issues/71436)
* [disposition: close] [Tracking issue for `alloc::prelude`](https://github.com/rust-lang/rust/issues/58935)

### New RFCs

* [Cargo --crate-type CLI Argument](https://github.com/rust-lang/rfcs/pull/3180)

## Upcoming Events

### Online

* [October 9, 2021 - Rust Gamedev Discord - Rust Gamedev Monthly Meetup](https://discord.gg/yNtPTb2)
* [October 12, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [October 12, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccnbqb/)
* [October 13, 2021 - betterCode Rust](https://rust.bettercode.eu/)
* [October 13, 2021 - C++/Rust: Learning from Each Other - MUC++](https://www.meetup.com/MUCplusplus/events/281231257)
* [October 13, 2021, Los Angeles, CA, US - Processing shaders in Rust with Dzmitry Malyshau - Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/280981968/)
* [October 20, 2021, Buffalo, NY, US - Buffalo Rust User Group, Alternate Day - Buffalo Rust](https://www.meetup.com/Buffalo-Rust-Meetup/events/281236385/)
* [October 20, 2021, Vancouver, BC, CA - WASM plugin for Istio - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccnbbc/)

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

> There's a common trope among people unfamiliar with rust where they assume that if you use unsafe at all, then it's just as unsafe as C and rust provided no benefit. Comparing C's approach to safety vs Rust's is like comparing an [open world assumption](https://en.wikipedia.org/wiki/Open-world_assumption) to a closed world assumption in formal logic systems. In C, you publish your api if it's possible to use correctly (open world). In Rust, you publish a safe api if it's **im** possible to use **in** correctly (closed world). Rust's key innovation here is that it enables you to build a 'bridge' from open world (unsafe) to a closed world (safe), a seemingly impossible feat that feels like somehow pairwise reducing an uncountable infinity with a countable infinity. Rust's decision to design an analogous closed-world assumption for safe code is extremely powerful, but it seems very hard for old school C programmers to wrap their head around it.

– [/u/infogulch on /r/rust](https://www.reddit.com/r/rust/comments/pzo1v9/comment/hf2thv2/?utm_source=reddit&utm_medium=web2x&context=3)

Thanks to [Alice Ryhl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1122) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
