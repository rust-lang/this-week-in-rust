Title: This Week in Rust 393
Number: 393
Date: 2021-06-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No official blog posts, newsletters, or research papers this week.

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

# Crate of the Week

This week's crate is [typed-index-collections](https://github.com/zheland/typed-index-collections), a crate that lets you make Vecs with custom-typed indices.

Thanks to [Tim](https://users.rust-lang.org/t/crate-of-the-week/2704/913) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

*No issues were proposed for CfP*.

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

280 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-17..2021-05-24

* [implement more `Iterator` methods on `core::iter::Repeat`](https://github.com/rust-lang/rust/pull/85338)
* [override `clone_from` for some types](https://github.com/rust-lang/rust/pull/85176)
* [stabilize `const_fn_unsize`](https://github.com/rust-lang/rust/pull/85078)
* [implement the new desugaring from `try_trait_v2`](https://github.com/rust-lang/rust/pull/84767)
* [impl `FromStr` for `proc_macro::Literal`](https://github.com/rust-lang/rust/pull/84717)
* [stabilize `extended_key_value_attributes`](https://github.com/rust-lang/rust/pull/83366)
* [fix auto-hide for implementations and implementors](https://github.com/rust-lang/rust/pull/85575)
* [add `rustc_mir::interpret::Machine::enforce_abi()`](https://github.com/rust-lang/rust/pull/85557)
* [check for more things in THIR unsafeck](https://github.com/rust-lang/rust/pull/85555)
* [suppress spurious errors inside `async fn`](https://github.com/rust-lang/rust/pull/85393)
* [avoid zero-length `memcpy` in formatting](https://github.com/rust-lang/rust/pull/85391)
* [always produce sub-obligations when using cached projection result](https://github.com/rust-lang/rust/pull/85382)
* [CTFE core engine allocation & memory API improvemenets](https://github.com/rust-lang/rust/pull/85376)
* [CTFE `get_alloc_extra_mut`: also provide ref to `MemoryExtra`](https://github.com/rust-lang/rust/pull/85578)
* [fix missing lifetimes diagnostics](https://github.com/rust-lang/rust/pull/85375)
* [suggest borrowing if a trait implementation is found for `&`/`&mut T`](https://github.com/rust-lang/rust/pull/85369)
* [remove `InPlaceIterable` marker from `Peekable` due to unsoundness](https://github.com/rust-lang/rust/pull/85340)
* [extend `rustc_on_implemented` to improve more `?` error messages](https://github.com/rust-lang/rust/pull/85596)ippy/pull/7264)
* [cargo: add `cargo:rustc-link-arg-bin` flag](https://github.com/rust-lang/cargo/pull/9486)
* [rustdoc: don't hide inherent implementations by default](https://github.com/rust-lang/rust/pull/85602)
* [clippy: fix ICE in `implicit_return`](https://github.com/rust-lang/rust-clippy/pull/7242)
* [clippy: fix invalid syntax in `from_iter_instead_of_collect` suggestion](https://github.com/rust-lang/rust-cl
* [clippy: fix `needless_borrow` suggestion](https://github.com/rust-lang/rust-clippy/pull/7105)
* [clippy: fix `redundant_closure` for `vec![]` macro in a closure with arguments](https://github.com/rust-lang/rust-clippy/pull/7263)
* [clippy: don't lint `multiple_inherent_impl` with generic arguments](https://github.com/rust-lang/rust-clippy/pull/7089)
* [clippy: early return from `LintPass` registration when collecting metadata](https://github.com/rust-lang/rust-clippy/pull/7253)
* [clippy: adding the default lint level to the metadata collection](https://github.com/rust-lang/rust-clippy/pull/7246)

## Rust Compiler Performance Triage

A somewhat quiet week. Some PRs had performance runs performed on them, but the changes were merged despite this. Also, we still have issues with certain benchmarks being noisy. 

Triage done by **@rylev**.
Revision range: [25a277..cdbe2](https://perf.rust-lang.org/?start=25a277f03df7e44643ddfcc240d034409cb2f505&end=cdbe2888979bb8797b05f0d58a6f6e60753983d2&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 1 Mixed
0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-05-25.md).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [A new prelude for the 2021 edition (trait-only edition)](https://github.com/rust-lang/rfcs/pull/3114)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: 2021 Edition](https://github.com/rust-lang/rfcs/pull/3085)
* [disposition: merge] [RFC: Overconstraining and omitting unsafe in impls of unsafe trait methods](https://github.com/rust-lang/rfcs/pull/2316)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [rustc: Allow safe #[target_feature] on wasm](https://github.com/rust-lang/rust/pull/84988)
* [disposition: merge] [Show test type during prints](https://github.com/rust-lang/rust/pull/84863)
* [disposition: merge] [Tracking Issue for VecDeque binary search functions](https://github.com/rust-lang/rust/issues/78021)
* [disposition: merge] [Tracking issue for WebAssembly SIMD support](https://github.com/rust-lang/rust/issues/74372)
* [disposition: merge] [Use try_reserve in Vec's io::Write](https://github.com/rust-lang/rust/pull/84612)

## New RFCs

* [ArrayBuilder struct for safe/efficient dynamic array initialisation](https://github.com/rust-lang/rfcs/pull/3131)
* [RFC: I/O Safety](https://github.com/rust-lang/rfcs/pull/3128)
* [A Cargo profile option trim-path to sanitise absolute paths](https://github.com/rust-lang/rfcs/pull/3127)

# Upcoming Events

### Online
* [May 27, 2021, London/Remote, UK - Runtime reflection, gRPC at scale, and more](https://www.meetup.com/Rust-London-User-Group/events/278045628/)
* [May 27, 2021, Montréal, QC, CN - Rust MTL: Building a Scrabble AI with the fst crate - Rust Montréal](https://www.meetup.com/Rust-Montreal/events/278011978/)
* [June 1, 2021, Dublin, IE - June Remote Meetup - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/278409501/)
* [June 1, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/jxfdjsyccjbcb/)

### North America
* [June 9, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccjbmb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Ok, you wanted it. Let's go full meta:

> This time, there were two crates and one quote, which is not much, but ok. Keep it up, folks!

– [llogiq on reddit](https://www.reddit.com/r/rust/comments/ngp41e/this_week_in_rust_391/gysis5e)

Thanks to [Patrice Peterson](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1051) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
