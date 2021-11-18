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

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [starship](https://github.com/starship/starship), a fast featureful customizable UNIX terminal prompt.

Thanks to [matchai](https://users.rust-lang.org/t/crate-of-the-week/2704/984) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

273 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-11-08..2021-11-15

* [proc_macro: add an expand_expr method to TokenStream](https://github.com/rust-lang/rust/pull/87264) (literals only for now)
* [type inference for inline consts](https://github.com/rust-lang/rust/pull/89561)
* [add support for specifying multiple clobber_abi in `asm!`](https://github.com/rust-lang/rust/pull/89316)
* [LLVM: fix nondeterminism in debuginfo generation](https://github.com/rust-lang/llvm-project/pull/118)
* [don't abort compilation after giving a lint error](https://github.com/rust-lang/rust/pull/87337)
* [do not emit overlap errors for impls failing the orphan check](https://github.com/rust-lang/rust/pull/89550)
* [implement diagnostic for `String` conversion](https://github.com/rust-lang/rust/pull/90645)
* [miri: detect uninitialized integers and floats](https://github.com/rust-lang/rust/pull/88670)
* [re-enable `copy`(`_nonoverlapping`) debug-checks](https://github.com/rust-lang/rust/pull/90041)
* [specialize array cloning for `Copy` types](https://github.com/rust-lang/rust/pull/90755)
* [replace `Copy`/`Clone` compiler magic on arrays with library impls](https://github.com/rust-lang/rust/pull/86041)
* [optimize `BinaryHeap::extend` from `Vec`](https://github.com/rust-lang/rust/pull/88282)
* [optimize `Eq` and `Hash` for `Path`/`PathBuf`](https://github.com/rust-lang/rust/pull/90596)
* [optimize pattern matching](https://github.com/rust-lang/rust/pull/90746)
* [stabilize `const_raw_ptr_deref` for `*const T`](https://github.com/rust-lang/rust/pull/89551)
* [stabilize format args capture](https://github.com/rust-lang/rust/pull/90473)
* [extend the const swap feature](https://github.com/rust-lang/rust/pull/90644)
* [don't destructure args tuple in `format_args!`](https://github.com/rust-lang/rust/pull/90485)
* [portable-simd: use new bitmask intrinsics with byte arrays](https://github.com/rust-lang/portable-simd/pull/159)
* [portable-simd: add `Simd::from_slice`](https://github.com/rust-lang/portable-simd/pull/177)
* [portable-simd: rotate_{left,right} -> rotate_lanes_{left,right}](https://github.com/rust-lang/portable-simd/pull/181)
* [clippy: add Clippy version to Clippy's lint list](https://github.com/rust-lang/rust-clippy/pull/7813)
* [clippy: add minimum supported Rust version to `deprecated_cfg_attr`](https://github.com/rust-lang/rust-clippy/pull/7944)
* [clippy: fix `explicit_counter_loop` suggestion for non-`usize` types](https://github.com/rust-lang/rust-clippy/pull/7950)
* [clippy: fix `semicolon_if_nothing_returned` FP on `let-else` stmts](https://github.com/rust-lang/rust-clippy/pull/7955)
* [clippy: fix suggestion for deref expressions in `redundant_pattern_matching`](https://github.com/rust-lang/rust-clippy/pull/7949)
* [clippy: lint for bool to integer casts in `cast_lossless`](https://github.com/rust-lang/rust-clippy/pull/7948)
* [clippy: make `let_underscore_lock` also detect `parking_lot` locks](https://github.com/rust-lang/rust-clippy/pull/7957)
* [clippy: new lint `index_refutable_slice` to avoid slice indexing](https://github.com/rust-lang/rust-clippy/pull/7643)
* [clippy: `swap` lints now check if there is `no_std` or `no_core` attribute](https://github.com/rust-lang/rust-clippy/pull/7877)
* [clippy: `option_if_let_else`: don't expand macros in suggestion](https://github.com/rust-lang/rust-clippy/pull/7974)
* [rustup: optimization: parse manifest only once](https://github.com/rust-lang/rustup/pull/2898)

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

> If a normal add is [a waffle iron ](https://en.wikipedia.org/wiki/Waffle_iron), SIMD add is a
> double or quadruple waffle iron. You can make 2 or 4 or more waffles at the same time.
>
> In case of waffles it would be called SIMW: **S** ingle **I** ron, **M** ultiple **W** affles.
>
> It's not multithreading - because you open and close the waffle iron for all the waffles at the
> same time.

– [/u/EarthyFeet on /r/rust](https://www.reddit.com/r/rust/comments/qucind/stdsimd_is_now_available_on_nightly/hkpy4y4/)

Editors note: Do yourself a favor, click the link and read the whole thread, it's pure gold (*chef's kiss*).

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1137) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [marriannegoldin](https://github.com/marriannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
