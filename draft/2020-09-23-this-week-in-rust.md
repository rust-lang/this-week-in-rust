Title: This Week in Rust 357
Number: 357
Date: 2020-09-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official

### Tooling

### Observations/Thoughts

### Learn Standard Rust

### Learn More Rust
 * [Dynamic Iterators](https://hole.tuziwo.info/dyn-iterator.html)

### Project Updates

### Miscellaneous

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

# Crate of the Week

This week's crate is [gitoxide](https://github.com/Byron/gitoxide), an idiomatic, modern, lean, fast, safe & pure Rust implementation of git.

Thanks again to [Vlad Frolov](https://users.rust-lang.org/t/crate-of-the-week/2704/812) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

336 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-09-07..2020-09-14

* [add rust-dev component to support rustc development](https://github.com/rust-lang/rust/pull/76332)
* [properly encode spans with a dummy location and non-root `SyntaxContext`](https://github.com/rust-lang/rust/pull/76658)
* [add `const_item_mutation` lint](https://github.com/rust-lang/rust/pull/75573)
* [more structured suggestions for boxed trait objects instead of impl Trait on non-coerceable tail expressions](https://github.com/rust-lang/rust/pull/75608)
* [add help note when using type in place of const](https://github.com/rust-lang/rust/pull/75611)
* [do not promote `&mut` of a non-ZST ever](https://github.com/rust-lang/rust/pull/75585)
* [chalk: simplify lowering](https://github.com/rust-lang/chalk/pull/602)
* [inliner: emit storage markers for introduced arg temporaries](https://github.com/rust-lang/rust/pull/76123)
* [enable the `SimplifyArmIdentity` MIR optimization at `mir-opt-level=1`](https://github.com/rust-lang/rust/pull/76308)
* [stabilize `doc_alias`](https://github.com/rust-lang/rust/pull/75740)
* [stabilize `core::future::`{`pending`,`ready`}](https://github.com/rust-lang/rust/pull/74328)
* [add saturating methods for `Duration`](https://github.com/rust-lang/rust/pull/76114)
* [add `slice::array_chunks_mut`](https://github.com/rust-lang/rust/pull/75021)
* [eliminate mut reference UB in `Drop` impl for `Rc<T>`](https://github.com/rust-lang/rust/pull/76530)
* [`BTreeMap` mutable iterators should not take any reference to visited nodes during iteration](https://github.com/rust-lang/rust/pull/73971)
* [`BTreeMap`: move up reference to map's root from `NodeRef`](https://github.com/rust-lang/rust/pull/74437)
* [add `drain_filter` method to `HashMap` and `HashSet`](https://github.com/rust-lang/rust/pull/76458)
* [arch: AVX512F](https://github.com/rust-lang/stdarch/pull/896)
* [add `MaybeUninit::assume_init_drop`](https://github.com/rust-lang/rust/pull/76484)
* [remove internal and unstable `MaybeUninit::UNINIT`](https://github.com/rust-lang/rust/pull/76527)
* [cargo: fix non-determinism with new feature resolver](https://github.com/rust-lang/cargo/pull/8701)

## Rust Compiler Performance Triage

* [2020-09-15](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-09-15.md):
  0 regressions, 2 improvements.

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

* [eRFC: Add JSON backend to Rustdoc](https://github.com/rust-lang/rfcs/pull/2963)

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [[RFC]: Portable SIMD Libs Project Group](https://github.com/rust-lang/rfcs/pull/2977)
* [Get type of an arbitrary expression](https://github.com/rust-lang/rfcs/pull/2706)
* [Add generalized arity tuples](https://github.com/rust-lang/rfcs/pull/2702)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)
* [disposition: merge][Add `[T; N]: TryFrom<Vec<T>>` (insta-stable)](https://github.com/rust-lang/rust/pull/76310)
* [disposition: merge][Implementation of Write for some immutable ref structs](https://github.com/rust-lang/rust/pull/76275)
* [disposition: merge][Permit uninhabited enums to cast into ints](https://github.com/rust-lang/rust/pull/76199)
* [disposition: merge][Stabilize some Result methods as const](https://github.com/rust-lang/rust/pull/76136)
* [disposition: merge][Stabilize some Option methods as const](https://github.com/rust-lang/rust/pull/76135)
* [disposition: merge][Stabilize move_ref_pattern](https://github.com/rust-lang/rust/pull/76119)
* [disposition: merge][Explicitly document the size guarantees that Option makes.](https://github.com/rust-lang/rust/pull/75454)
* [disposition: merge][Stabilize intra-doc links](https://github.com/rust-lang/rust/pull/74430)
* [disposition: merge][target-feature 1.1: should closures inherit target-feature annotations?](https://github.com/rust-lang/rust/issues/73631)
* [disposition: merge][might_permit_raw_init: also check aggregate fields](https://github.com/rust-lang/rust/pull/71274)

## New RFCs

* [rustdoc edit links](https://github.com/rust-lang/rfcs/pull/2985)

# Upcoming Events

### Online
* [September 16. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/gbzjxrybcmbvb/)
* [September 17. Linz, AT - Rust Linz - Rust Meetup Linz - Meaghan Lewis on Rust, Embedded Rust with Roland Ruckerbauer](https://www.meetup.com/de-DE/Rust-Linz/events/271857244/)
* [September 17. Berlin, DE - Berline.rs - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcmbwb/)
* [September 29. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcmbmc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> When you have a lifetime `<'a>` on a struct, that lifetime denotes references to values stored *outside* of the struct. If you try to store a reference that points inside the struct rather than outside, you will run into a compiler error when the compiler notices you **lied** to it.

- [Alice Ryhl on rust-users](https://users.rust-lang.org/t/how-to-resolve-error-e0499-cannot-borrow-as-mutable-more-than-once-at-a-time-in-this-case/48815/3)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/939) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/iu3ge0/this_week_in_rust_356/)</small>
