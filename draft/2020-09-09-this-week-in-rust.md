Title: This Week in Rust 355
Number: 355
Date: 2020-09-09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Tooling

### Newsletters

### Observations/Thoughts
* [Retrospective of my first useful Rust project](http://jamesmcm.github.io/blog/2020/09/05/vopono/#en)

### Learn Standard Rust
* [TR] [Rust Turu](https://tourofrust.com/00_tr.html)
* [video] [Choosing Rust - Intro to Rust and Ownership](https://www.youtube.com/watch?v=DMAnfOlhSpU)

### Learn More Rust
* [Linux System Call `fork()` in Rust ](https://blog.knoldus.com/linux-system-call-fork-in-rust/)
* [PL] [CrabbyBird #2 Poruszanie kamerą](https://postacnormalna.pl/ruch-kamery/)

### Project Updates

### Miscellaneous
* [Create an amazing Rust GitHub project in no time](https://www.marcoieni.com/2020/09/create-an-amazing-rust-github-project-in-no-time/)

# Crate of the Week

This week's crate is [serde-query](https://github.com/pandaman64/serde-query/), an efficient query language for Serde.

Thanks to [Vlad Frolov](https://users.rust-lang.org/t/crate-of-the-week/2704/810) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

332 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-31..2020-09-07

* [inliner: avoid query cycles when optimizing generators](https://github.com/rust-lang/rust/pull/76245)
* [diagnostics: shorten paths of unique symbols](https://github.com/rust-lang/rust/pull/73996)
* [add `-Z proc-macro-backtrace` to allow showing proc-macro panics](https://github.com/rust-lang/rust/pull/75082)
* [suggest `if let x = y` when encountering `if x = y`](https://github.com/rust-lang/rust/pull/75931)
* [MIR peephole optimize {Ne, Eq}(_1, false) into _1](https://github.com/rust-lang/rust/pull/76067)
* [miri: move panic payload state from Machine to Thread](https://github.com/rust-lang/miri/pull/1532)
* [eliminate some other bound checks when index comes from an enum](https://github.com/rust-lang/rust/pull/75529)
* [improve recovery on malformed `format!` call](https://github.com/rust-lang/rust/pull/76160)
* [specialize some collection and iterator operations to run in-place](https://github.com/rust-lang/rust/pull/70793)
* [stabilize `deque_make_contiguous`](https://github.com/rust-lang/rust/pull/74559)
* [add `slice::check_range`](https://github.com/rust-lang/rust/pull/75207)
* [BTreeMap: introduce marker::ValMut and reserve Mut for unique access](https://github.com/rust-lang/rust/pull/75200)
* [add `[T; N]::as_[mut_]slice`](https://github.com/rust-lang/rust/pull/76120)
* [implement `Seek::stream_position()` for `BufReader`](https://github.com/rust-lang/rust/pull/74366)
* [`impl Rc::new_cyclic`](https://github.com/rust-lang/rust/pull/75994)
* [make `cow_is_borrowed` methods const](https://github.com/rust-lang/rust/pull/76139)
* [compiler-builtins: greatly improve division performance for u128 and other cases](https://github.com/rust-lang/compiler-builtins/pull/332)
* [stdarch: bye bye MMX!](https://github.com/rust-lang/stdarch/pull/890)
* [stdarch: AVX512](https://github.com/rust-lang/stdarch/pull/891)
* [futures-rs: implement `FusedStream` for `FuturesOrdered`](https://github.com/rust-lang/futures-rs/pull/2205)
* [futures-rs: fix UB due to missing `'static` on `task::waker`](https://github.com/rust-lang/futures-rs/pull/2206)
* [hashbrown: use the alloc crate on stable Rust](https://github.com/rust-lang/hashbrown/pull/197)
* [hashbrown: remove `from_key_hashed_nocheck`'s `Q: Hash`](https://github.com/rust-lang/hashbrown/pull/200)

## Rust Compiler Performance Triage

* [2020-09-08](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-09-08.md):
  3 regressions, 0 improvements.
  
A few small compile-time regressions this week. The first was
[#70793](https://github.com/rust-lang/rust/pull/70793), which added some
specializations to the standard library in order to increase runtime
performance. The second was
[#73996](https://github.com/rust-lang/rust/pull/73996), which adds an option to
the diagnostics code to print only the names of types and traits when they are
unique instead of the whole path. The third was
[#75200](https://github.com/rust-lang/rust/pull/75200), which refactored part
of `BTreeMap` to avoid aliasing mutable references.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Named arguments](https://github.com/rust-lang/rfcs/pull/2964)
* [eRFC: Add JSON backend to Rustdoc](https://github.com/rust-lang/rfcs/pull/2963)
* [RFC: Reading into uninitialized buffers](https://github.com/rust-lang/rfcs/pull/2930)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Add `#[cfg(panic = '...')]`](https://github.com/rust-lang/rust/pull/74754)
* [disposition: merge][Allow Weak::as_ptr and friends for unsized T](https://github.com/rust-lang/rust/pull/74160)
* [disposition: merge][Update stdarch](https://github.com/rust-lang/rust/pull/73166)
* [disposition: merge][Tracking issue for #[doc(alias = "...")]](https://github.com/rust-lang/rust/issues/50146)

## New RFCs

* [Safer Transmute](https://github.com/rust-lang/rfcs/pull/2981)
* [[RFC]: Libs Team Governance](https://github.com/rust-lang/rfcs/pull/2979)
* [[RFC]: Portable SIMD Libs Project Group](https://github.com/rust-lang/rfcs/pull/2977)

# Upcoming Events

### Online
* [September 8. Saarbrücken, DE - Rust-Saar Meetup - `3u16.map_err(...)`](https://www.meetup.com/Rust-Saar/events/272522454/)
* [September 8. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybcmblb/)
* [September 9. East Coast, US - Rust East Coast Virtual Talks + Q&A](https://www.meetup.com/Rust-NYC/events/272982073/)
* [September 11. Russia - Russian Rust Online Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/272798484/)

### North America
* [September 9. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcmbmb/)
* [September 10. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers (Sept / Second Round)](https://www.meetup.com/utah-rust/events/268567961/)

### Asia Pacific
* [September 7. Auckland, NZ - Rust AKL - Finally, good Rust IDE support in VSCode: rust-analyzer](https://www.meetup.com/rust-akl/events/266876702/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> It's amazing how frequent such "rare edge cases" can be. Especially when there are millions of people using billions of files originating from God know what operating systems. Far better things are checked properly if one want robust code. As Rust uses do.

- [ZiCog on rust-users](https://users.rust-lang.org/t/disappointed-with-path/48148/5)

Thanks to [Edoardo Morandi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/938) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/imoogj/this_week_in_rust_354/)</small>
