Title: This Week in Rust 370
Number: 370
Date: 2020-12-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Newsletters

### Tooling

### Observations/Thoughts

### Rust Walkthroughs

 * [Async: What is blocking?](https://ryhl.io/blog/async-what-is-blocking/)

### Project Updates

### Miscellaneous

# Crate of the Week

This week's crate is [thermite](https://github.com/raygon-renderer/thermite), a SIMD struct-of-arrays-algorithms library.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/857) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

300 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-12-07..2020-12-14

* [fixes to Rust coverage](https://github.com/rust-lang/rust/pull/79818)
* [properly re-use def path hash in incremental mode](https://github.com/rust-lang/rust/pull/79721)
* [add some `core::cmp::Ordering` helpers](https://github.com/rust-lang/rust/pull/79656)
* [constify some `MaybeUninit` methods](https://github.com/rust-lang/rust/pull/79621)
* [Windows TLS: `ManuallyDrop` instead of `mem::forget`](https://github.com/rust-lang/rust/pull/79893)
* [use `is_write_vectored` to optimize the `write_vectored` implementation for `BufWriter`](https://github.com/rust-lang/rust/pull/78768)
* [enforce no-move rule of `ReentrantMutex` using `Pin` and fix UB in stdio](https://github.com/rust-lang/rust/pull/77801)
* [hashbrown: enable specialization with aHash](https://github.com/rust-lang/hashbrown/pull/207)
* [future: `SinkExt::feed`](https://github.com/rust-lang/futures-rs/pull/2155)
* [futures-util: migrate from pin-project to pin-project-lite](https://github.com/rust-lang/futures-rs/pull/2273)
* [cargo: check if rerun-if-changed points to a directory](https://github.com/rust-lang/cargo/pull/8973)
* [cargo: workaround fs issue in `cargo publish`](https://github.com/rust-lang/cargo/pull/8950)
* [clippy: add MSRV to more lints](https://github.com/rust-lang/rust-clippy/pull/6424)
* [rustfmt: don't force a newline after an empty where clause](https://github.com/rust-lang/rustfmt/pull/4557)

## Rust Compiler Performance Triage

* [2020-12-15](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-15.md):
6 Regressions, 1 Improvements, 2 Mixed

This week was fairly quite with lots of small regressions. Most of the regressions were either for fixes to changes that yielded large performance wins in previous weeks or small performance losses where there is already a plan for how to gain those losses back.

Triage done by @rylev.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-15.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [added secret types rfc](https://github.com/rust-lang/rfcs/pull/2859)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Implement `From<char>` for u64 and u128.](https://github.com/rust-lang/rust/pull/79502)
* [disposition: merge] [Stabilize `unsafe_cell_get_mut`](https://github.com/rust-lang/rust/pull/79485)
* [disposition: merge] [Move `{f32,f64}::clamp` to core](https://github.com/rust-lang/rust/pull/79473)
* [disposition: merge] [Stabilize all stable methods of `Ipv4Addr`, `Ipv6Addr` and `IpAddr` as const](https://github.com/rust-lang/rust/pull/79342)
* [disposition: merge] [Acknowledge that `[CONST; N]` is stable](https://github.com/rust-lang/rust/pull/79270)
* [disposition: merge] [Deprecate atomic compare_and_swap method](https://github.com/rust-lang/rust/pull/79261)
* [disposition: merge] [Stabilize `core::slice::fill`](https://github.com/rust-lang/rust/pull/79213)
* [disposition: close] [Made matches! more useful by adding mapping support](https://github.com/rust-lang/rust/pull/79188)
* [disposition:merge] [stabilize `#![feature(min_const_generics)]`](https://github.com/rust-lang/rust/pull/79135)
* [disposition:merge] [Add `impl Div<NonZeroU{0}> for u{0}` which cannot panic](https://github.com/rust-lang/rust/pull/79134)
* [disposition: merge] [passes: prohibit invalid attrs on generic params](https://github.com/rust-lang/rust/pull/79073)
* [disposition: merge] [stabilize deque_range](https://github.com/rust-lang/rust/pull/79022)
* [disposition: merge] [Rename `overlapping_patterns` lint](https://github.com/rust-lang/rust/pull/78242)
* [disposition: merge] [Stabilize or_insert_with_key](https://github.com/rust-lang/rust/pull/78083)
* [disposition: close] [Add built-in implementations of `Default` for function definition and… ](https://github.com/rust-lang/rust/pull/77688)
* [disposition: merge] [Mark `-1` as an available niche for file descriptors](https://github.com/rust-lang/rust/pull/74699)
* [disposition: merge] [Stabilize the Wake trait](https://github.com/rust-lang/rust/pull/74304)
* [disposition: merge] [Tracking issue for map_ok and map_err method for `Poll<Option<Result<T, E>>>`](https://github.com/rust-lang/rust/issues/63514)

## New RFCs

* [Add min_target_api_version cfg predicate](https://github.com/rust-lang/rfcs/pull/3036)
* [Cargo: providing artifacts (for artifact dependencies) via build.rs](https://github.com/rust-lang/rfcs/pull/3035)

# Upcoming Events

### Online
* [December 16, Vancouver, BC, US - Are Results just Checked Exceptions? - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsybcqbvb/)
* [December 29, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcqbmc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Engineering is not about "not doing mistakes". Engineering is about designing systems that ensure fewer mistakes occur.
>
> Rust is such a system.

– [amos on his blog](https://fasterthanli.me/articles/aiming-for-correctness-with-types)

Thanks to [Joshua Nelson](https://users.rust-lang.org/t/twir-quote-of-the-week/328/972) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
