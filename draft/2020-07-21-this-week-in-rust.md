Title: This Week in Rust 348
Number: 348
Date: 2020-07-21
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/022-twir-347/)

# Updates from Rust Community

## News & Blog Posts

* [Efficient representation of Ultimate Tic Tac Toe](https://www.minimax.dev/docs/ultimate/efficient-representation/) (using Rust)

# Crate of the Week

This week's crate is [nnnoiseless](https://jneem.github.io/nnnoiseless), a filter for audio noise removal ported from C.

Thanks to [mmmmib](https://users.rust-lang.org/t/crate-of-the-week/2704/790) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

273 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-07-06..2020-07-13

* [shrink `ParamEnv` to 16 bytes](https://github.com/rust-lang/rust/pull/73978)
* [stabilize const `mem::forget`](https://github.com/rust-lang/rust/pull/73887)
* [typeck: adding type information to projection](https://github.com/rust-lang/rust/pull/73870)
* [clippy: some accuracy lints for floating point operations](https://github.com/rust-lang/rust-clippy/pull/5443)
* [correctly mark the ending span of a match arm](https://github.com/rust-lang/rust/pull/74125)
* [only allow `repr(i128/u128)` on enum](https://github.com/rust-lang/rust/pull/74109)
* [hide `&mut self` methods from Deref in sidebar if there are no `DerefMut` impl for the type](https://github.com/rust-lang/rust/pull/74107)
* [only add CFGuard on `windows-msvc` targets](https://github.com/rust-lang/rust/pull/74103)
* [add `VecDeque::range*` methods](https://github.com/rust-lang/rust/pull/74099)
* [add `read_exact_at` and `write_all_at` to WASI's `FileExt`](https://github.com/rust-lang/rust/pull/74076)
* [clippy: new lint: `match_like_matches_macro`](https://github.com/rust-lang/rust-clippy/pull/5769)
* [Optimize `is_ascii` for `str` and `[u8]`](https://github.com/rust-lang/rust/pull/74066)
* [arch: added `f32` and `f64` unaligned stores and loads from avx512f set](https://github.com/rust-lang/stdarch/pull/873)
* [hashbrown: add `HashSet::drain_filter` method](https://github.com/rust-lang/hashbrown/pull/179)

## Rust Compiler Performance Triage

* [2020-07-14](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-07-14.md). Twelve revisions checked. Zero regressions. One improvement.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Remove trait `LengthAtMost32`](https://github.com/rust-lang/rust/pull/74060)
* [disposition: merge] [Stabilize control-flow-guard codegen option](https://github.com/rust-lang/rust/pull/73893)
* [disposition: merge] [Impl Default for ranges](https://github.com/rust-lang/rust/pull/73197)
* [disposition: merge] [Tracking issue for `core::{f32,f64}::consts::TAU` ](https://github.com/rust-lang/rust/issues/66770)

## New RFCs

* [Opt-in Stable Trait VTables](https://github.com/rust-lang/rfcs/pull/2955)

# Upcoming Events

### Online
* [July 14. Dallas, TX, US - Dallas Rust - Second Tuesday](https://www.meetup.com/Dallas-Rust/events/mzzfsrybckbsb/)
* [July 16. Turin, IT - Rust Italia - Gruppo di studio di Rust](https://community.mozilla.org/events/gruppo-di-studio-di-rust-3/)
* [July 27 - August 8. Rusty Days Virtual Rust Conference](https://rusty-days.org/)

### North America
* [July 15. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybckbtb/)
* [July 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybckbkc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Ownership in Rust is entirely a type system fiction.

— dodomorandi

> I'm not sure what is meant there. "ownership" in many languages is a very real thing to me.

– and [ZiCog on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/900)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/903) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hnkws3/this_week_in_rust_346/)</small>
