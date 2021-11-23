Title: This Week in Rust 418
Number: 418
Date: 2021-11-18
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

### Foundation

### Project/Tooling Updates

* [SixtyFPS (GUI crate): Changelog for 21th of November 2021](https://sixtyfps.io/thisweek/2021-11-22.html)
* [What's new in SeaORM 0.4.0](https://www.sea-ql.org/SeaORM/blog/2021-11-19-whats-new-in-0.4.0/)

### Observations/Thoughts

* [My Path to Magma: How I slowly became convinced we absolutely have to build a proof checker and bring formal verification to the mainstream.](https://blainehansen.me/post/my-path-to-magma/)
* [Rust Packages vs Crates](https://jeffa.io/rust_packages_vs_crates)
- [Merge Queues with Bors](https://kflansburg.com/posts/merge-queues/)
* [Stack-safety for free?](https://hurryabit.github.io/blog/stack-safety-for-free/)

### Rust Walkthroughs

* [Calling Rust from Python using PyO3](https://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/)
### Miscellaneous

## Crate of the Week

This week's crate is [rustc\_codegen\_nvvm](https://crates.io/crates/rustc_codegen_nvvm), a rustc codegen backend that targets NVIDIA's libnvvm CUDA library.

Thanks to [troiganto](https://users.rust-lang.org/t/crate-of-the-week/2704/987) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

284 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-11-15..2021-11-22

* [stabilize `-Z strip` as `-C strip`](https://github.com/rust-lang/rust/pull/90058)
* [permit const panics in stable const contexts in stdlib](https://github.com/rust-lang/rust/pull/90687)
* [simplify `for` loop desugar](https://github.com/rust-lang/rust/pull/90352)
* [warn on `#[must_use]` use on async fn's](https://github.com/rust-lang/rust/pull/89610)
* [suggest `&str.chars()` on attempt to `&str.iter()`](https://github.com/rust-lang/rust/pull/90803)
* [suggest `await` in more situations where infer types are involved](https://github.com/rust-lang/rust/pull/91022)
* [suggest removal of arguments for unit variant, not replacement](https://github.com/rust-lang/rust/pull/90961)
* [try all stable method candidates first before trying unstable ones](https://github.com/rust-lang/rust/pull/90329)
* [point at source of trait bound obligations in more places](https://github.com/rust-lang/rust/pull/89580)
* [print escaped string if char literal has multiple characters, but only one printable character](https://github.com/rust-lang/rust/pull/90861)
* [improve `ManuallyDrop` suggestion](https://github.com/rust-lang/rust/pull/90901)
* [improve diagnostics when a static lifetime is expected](https://github.com/rust-lang/rust/pull/90667)
* [improve suggestions for compatible variants on type mismatch](https://github.com/rust-lang/rust/pull/90575)
* [fix float ICE](https://github.com/rust-lang/rust/pull/90927)
* [fix await suggestion on non-future type](https://github.com/rust-lang/rust/pull/90933)
* [fix incorrect handling of `TraitRef`s when emitting suggestions](https://github.com/rust-lang/rust/pull/90819)
* [avoid suggesting literal formatting that turns into member access](https://github.com/rust-lang/rust/pull/90989)
* [address performance regression introduced by recent ADT drop requirements fix](https://github.com/rust-lang/rust/pull/90845)
* [optimize `impl Hash for ObligationCauseData` by not hashing `ObligationCauseCode` variant fields](https://github.com/rust-lang/rust/pull/90996)
* [add IEEE 754-2019 minimun and maximum functions for `f32`/`f64`](https://github.com/rust-lang/rust/pull/91008)
* [make char conversion functions unstably `const`](https://github.com/rust-lang/rust/pull/89258)
* [make slice → `str` conversion and related functions `const`](https://github.com/rust-lang/rust/pull/90607)
* [mark `<*const _>::align_offset` and `<*mut _>::align_offset` as `const fn`](https://github.com/rust-lang/rust/pull/90958)
* [mark `Arc::from_inner` / `Rc::from_inner` as `unsafe`](https://github.com/rust-lang/rust/pull/89741)
* [stabilize `File::options`](https://github.com/rust-lang/rust/pull/85766)
* [add `Vec::retain_mut`](https://github.com/rust-lang/rust/pull/90772)
* [implement `Termination` for `Result<Infallible, E>`](https://github.com/rust-lang/rust/pull/88601)
* [implement `clone_from` for `State`](https://github.com/rust-lang/rust/pull/90535)
* [miri: portable SIMD: basic binops](https://github.com/rust-lang/miri/pull/1918)
* [arch: work-around buggy Intel chips erroneously reporting BMI1/BMI2 support](https://github.com/rust-lang/stdarch/pull/1249)
* [arch: complete armv8 instructions](https://github.com/rust-lang/stdarch/pull/1256)
* [log: add `Log` implementation for `&impl Log` and `Arc<impl Log>`](https://github.com/rust-lang/log/pull/471)
* [cargo: add `--message-format` for `install` command](https://github.com/rust-lang/cargo/pull/10107)
* [cargo: enhance error message for target auto-discovery](https://github.com/rust-lang/cargo/pull/10090)
* [cargo: warn when alias shadows external subcommand](https://github.com/rust-lang/cargo/pull/10082)
* [clippy: add new lint `octal_escapes`](https://github.com/rust-lang/rust-clippy/pull/8007)
* [clippy: allow `suboptimal_flops` in `const` functions](https://github.com/rust-lang/rust-clippy/pull/8009)
* [clippy: avoid inline hints with double backticks for `doc-markdown`](https://github.com/rust-lang/rust-clippy/pull/8011)
* [clippy: don't show `no_effect` warning on unit structs implementing `fn_once`](https://github.com/rust-lang/rust-clippy/pull/7898)
* [clippy: fix ICE on `undocumented_unsafe_blocks`](https://github.com/rust-lang/rust-clippy/pull/7988)
* [clippy: fix `manual_map` with unsafe functions](https://github.com/rust-lang/rust-clippy/pull/7968)
* [clippy: fix `needless_collect`'s tendency to suggest code requiring multiple mutable borrows of the same value.](https://github.com/rust-lang/rust-clippy/pull/7982)
* [clippy: fix behavior-changing `manual_split_once` suggestion and add new lint `needless_splitn`](https://github.com/rust-lang/rust-clippy/pull/7896)
* [clippy: fix `shadow_same` false positives for async function arguments](https://github.com/rust-lang/rust-clippy/pull/7997)
* [clippy: improve `needless_borrow` lint](https://github.com/rust-lang/rust-clippy/pull/7977)
* [clippy: improve heuristic for eagerness suggestion](https://github.com/rust-lang/rust-clippy/pull/7639)
* [clippy: fix suggestion in `option_map_or_none`](https://github.com/rust-lang/rust-clippy/pull/7971)
* [rustfmt: preserve normalized comments after last list item](https://github.com/rust-lang/rustfmt/pull/5091)

### Rust Compiler Performance Triage

A large amount of noise in the comparisons this week, likely due to new
probabilistic query hash verification increasing likelihood of changes in each
benchmark; solutions are being tracked in [rustc-perf#1105].

Otherwise, though, the week largely amounted to a neutral one for performance.
There were some regressions, particularly in doc builds, as a result of the
addition of portable SIMD. These are relatively speaking minor and primarily
impact small crates.

[rustc-perf#1105]: https://github.com/rust-lang/rustc-perf/issues/1105

Triage done by **@simulacrum**.
Revision range: [eee8b9c7..934624f](https://perf.rust-lang.org/?start=eee8b9c7bafade55981d155dae71657f1cc55a22&end=934624fe5f66ce3fb8abf0597a6deb079783335f&absolute=false&stat=instructions%3Au)

5 Regressions, 2 Improvements, 6 Mixed; 2 of them in rollups

41 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-11-16.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Add RFC float-next-up-down.](https://github.com/rust-lang/rfcs/pull/3173)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [#![feature(maybe_uninit_extra,const_maybe_uninit_write)]](https://github.com/rust-lang/rust/issues/63567)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* [Add std::inputln()](https://github.com/rust-lang/rfcs/pull/3196)
* [Fix link in RFC 2873](https://github.com/rust-lang/rfcs/pull/3195)
* [return position impl trait in traits](https://github.com/rust-lang/rfcs/pull/3193)

## Upcoming Events

Rusty Events between 11/17-12/01 🦀

### Online

* [November 17, 2021, Vancouver, BC, CA - Borrowing and Lifetimes through Metaphors - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccpbwb/)
* [November 17, 2021, Houston, TX, US - A Functional Introduction to Rust - Houston Functional Programming User Group](https://www.meetup.com/houston-functional-programming-users-group/events/281526282)
* [November 17, 2021, Los Angeles, CA, US - Live Coding Session: Mob Programming a Rust Code Kata - Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/281944639)
* [November 19, 2021, IR - The Second Rust Iran online meetup - Rust Iran Meetup](https://rust-meetup.ir/2021/11/19/second-meetup.html)
* [November 20, 2021, RustFest Global 2021: Rust In Arts Edition - RustFest](https://rustfest.global/)
* [November 23, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [November 30, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccpbnc/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> On the topic of reframing UB, I was reminded of an article about the [mechanics of oaths and vows in historical cultures](https://acoup.blog/2019/06/28/collections-oaths-how-do-they-work/).
>
> When a programmer writes `get_unchecked` , we can imagine them wanting to promise the compiler that they uphold its preconditions. But since the compiler is normally not so trusting of unproven assertions, the programmer swears an *oath* that their argument is in bounds.
>
> The compiler, seeing such a solemn commitment, treats the programmer's word as true and optimizes accordingly. The compiler is so thoroughly convinced that it never even entertains the possibility of doubting the programmer's oath.
>
> But if the programmer has sworn falsely, then they might well suffer divine retribution in the form of nasal demons — or worse, subtly baffling program behaviour.

– [/u/scook0 on /r/rust](https://reddit.com/r/rust/comments/qx168t/undefined_behavior_deserves_a_better_reputation/hl8koel/)

Thanks to [G. Thorondorsen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1142) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [marriannegoldin](https://github.com/marriannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
