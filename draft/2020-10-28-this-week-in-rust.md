Title: This Week in Rust 362
Number: 362
Date: 2020-10-28
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
* Contributing to the IntelliJ Rust plugin: Implementing a refactoring](https://kobzol.github.io/rust/intellij/2020/10/19/contributing-4-introduce-constant-refactoring.html)

### Observations/Thoughts

### Learn Simple Rust

### Learn More Rust

### Project Updates

### Miscellaneous

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

# Crate of the Week

This week's crate is [icu4x](https://github.com/unicode-org/icu4x), the Unicode Consortium's official crate for dealing with i18n in resource constrained environments.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/828) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

398 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-10-12..2020-10-19

* [make set_span take `mut self`](https://github.com/rust-lang/rust/pull/78047)
* [resolve: further improvements to "try using the enum's variant" diagnostic](https://github.com/rust-lang/rust/pull/77855)
* [`min_const_generics` diagnostics improvements](https://github.com/rust-lang/rust/pull/77825)
* [make sure arenas don't allocate bigger than `HUGE_PAGE`](https://github.com/rust-lang/rust/pull/78058)
* [make `ObligationForest` more efficient](https://github.com/rust-lang/rust/pull/77908)
* [add `std::thread::available_concurrency`](https://github.com/rust-lang/rust/pull/74480)
* [remove `shrink_to_fit` from default `ToString::to_string` implementation](https://github.com/rust-lang/rust/pull/77997)
* [add `str::`{`Split`, `RSplit`, `SplitN`, `RSplitN`, `SplitTerminator`, `RSplitTerminator`, `SplitInclusive`}`::as_str` methods](https://github.com/rust-lang/rust/pull/75265)
* [liballoc: `VecDeque`: Add binary search functions](https://github.com/rust-lang/rust/pull/77751)
* [BTreeMap: fix gdb provider on `BTreeMap` with ZST keys or values](https://github.com/rust-lang/rust/pull/77788)
* [hashbrown: remove the need for unwrap when using `ProbeSeq`](https://github.com/rust-lang/hashbrown/pull/208)

## Rust Compiler Performance Triage

* [2020-10-21](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-21.md):
4 Regressions, 7 Improvements, 0 Mixed

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-21.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Promote aarch64-unknown-linux-gnu to a Tier-1 Rust target](https://github.com/rust-lang/rfcs/pull/2959)
* [YieldSafe auto trait](https://github.com/rust-lang/rfcs/pull/2890)
* [Access to traits' associated functions and constants from trait objects](https://github.com/rust-lang/rfcs/pull/2886)
* [Variadic tuples](https://github.com/rust-lang/rfcs/pull/2775)
* [RFC for a match based surface syntax to get pointer-to-field](https://github.com/rust-lang/rfcs/pull/2666)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)
* [disposition: merge] [Allow making `RUSTC_BOOTSTRAP` conditional on the crate name](https://github.com/rust-lang/rust/pull/77802)
* [disposition: merge] [stop promoting union field accesses in 'const'](https://github.com/rust-lang/rust/pull/77526)
* [disposition: merge] [passes: `check_attr` on more targets](https://github.com/rust-lang/rust/pull/77015)
* [disposition: merge] [Stabilize `Poll::is_ready` and `is_pending` as const](https://github.com/rust-lang/rust/pull/76227)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [October 22. Edinburgh, UK - Fluence: interface-types for server-side WebAssembly modules - Rust Edinburgh](https://www.meetup.com/rust-edi/events/273685985)
* [October 27. Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcnbkc/)
* [October 29. Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbmc/)

 # Asia Pacific
* [November 1. Auckland, NZ - Rust meetup - Introduction to Rust - Rust AKL](https://www.meetup.com/rust-akl/events/266876718/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> And it's true that a lot of stuff requires a "sufficiently smart compiler" but really it's 2020, if your compiler isn't serving you breakfast in bed you need to be upping your expectations.

- [Jubilee on the Rust Zulip](https://rust-lang.zulipchat.com/#narrow/stream/257879-project-portable-simd/topic/The.20movemasquerade/near/212794818)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/949) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/iu3ge0/this_week_in_rust_356/)</small>
