Title: This Week in Rust 347
Number: 347
Date: 2020-07-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*]()

# Updates from Rust Community

## News & Blog Posts
* [Perspective on Rust Community Moderation](https://www.reddit.com/r/rust/comments/hnfnti/where_is_the_rust_community_allowed_to_talk_about/fxf65nf/)
* [Programming Servo: the bird’s-eyes view](https://medium.com/programming-servo/programming-servo-the-birds-eyes-view-201d28220b9a?source=friends_link&sk=b8610f254bf6faf8b81c81729c1b3498)
* [Faster Integer Parsing (Rust port)](https://rust-malaysia.github.io/code/2020/07/11/faster-integer-parsing.html)
* [Learning Rust: Let's Build a Parser](https://codeandbitters.com/lets-build-a-parser/)

# Crate of the Week

This week's crate is [suckit](https://github.com/skallwar/suckit), a tool to recursively download a website.

Thanks to [Martin Schmidt](https://users.rust-lang.org/t/crate-of-the-week/2704/786) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

308 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-29..2020-07-06

* [add `format_args_capture` feature](https://github.com/rust-lang/rust/pull/73670)
* [don't implement Fn* traits for `#[target_feature]` functions](https://github.com/rust-lang/rust/pull/73306)
* [fix wasm32 being broken due to a NodeJS version bump](https://github.com/rust-lang/rust/pull/73885)
* [handle `macro_rules!` tokens consistently across crates](https://github.com/rust-lang/rust/pull/73569)
* [implement `slice_strip` feature](https://github.com/rust-lang/rust/pull/73414)
* [make `likely` and `unlikely` const, gated by feature `const_unlikely`](https://github.com/rust-lang/rust/pull/73778)
* [optimise fast path of checked_ops with `unlikely`](https://github.com/rust-lang/rust/pull/73938)
* [provide more information on duplicate lang item error.](https://github.com/rust-lang/rust/pull/73449)
* [remove `TypeckTables::empty(None)` and make hir_owner non-optional.](https://github.com/rust-lang/rust/pull/73751)
* [remove unnecessary release from Arc::try_unwrap](https://github.com/rust-lang/rust/pull/74025)
* [serialize all foreign `SourceFile`s into proc-macro crate metadata](https://github.com/rust-lang/rust/pull/73706)
* [stabilize `#[track_caller]`.](https://github.com/rust-lang/rust/pull/72445)
* [use WASM's saturating casts if they are available](https://github.com/rust-lang/rust/pull/73724)
* [use `Span`s to identify unreachable subpatterns in or-patterns](https://github.com/rust-lang/rust/pull/73973)
* [Update the rust-lang/llvm-project submodule to include AVR fixes recently merged](https://github.com/rust-lang/rust/pull/73658)
* [mir-opt: Fix mis-optimization and other issues with the SimplifyArmIdentity pass](https://github.com/rust-lang/rust/pull/73949)
* [added `.collect()` into `String` from `Box<str>`](https://github.com/rust-lang/rust/pull/72688)
* [impl `From<char>` for `String`](https://github.com/rust-lang/rust/pull/73466)
* [linker: create `GNU_EH_FRAME` header by default when producing ELFs](https://github.com/rust-lang/rust/pull/73564)
* [resolve: disallow labelled breaks/continues through closures/async blocks](https://github.com/rust-lang/rust/pull/73726)
* [ship rust analyzer](https://github.com/rust-lang/rust/pull/72978)
* [chalk: add type outlives goal](https://github.com/rust-lang/chalk/pull/551)
* [chalk: allow printing lifetime placeholders](https://github.com/rust-lang/chalk/pull/557)
* [chalk: support for ADTs](https://github.com/rust-lang/chalk/pull/524)
* [hashbrown: add RawTable::erase and remove](https://github.com/rust-lang/hashbrown/pull/171)
* [hashbrown: expose RawTable::try_with_capacity](https://github.com/rust-lang/hashbrown/pull/174)
* [hashbrown: improve RawIter re-usability](https://github.com/rust-lang/hashbrown/pull/175)
* [libc: add a bunch of constants and functions which were missing on Android](https://github.com/rust-lang/libc/pull/1795)
* [libc: add more WASI libc definitions.](https://github.com/rust-lang/libc/pull/1811)
* [libc: declare `seekdir` and `telldir` for WASI.](https://github.com/rust-lang/libc/pull/1804)
* [stdarch: fix or equals integer comparisons](https://github.com/rust-lang/stdarch/pull/872)
* [cargo: write GNU tar files, supporting long names.](https://github.com/rust-lang/cargo/pull/8453)
* [crates.io: use default branch alias instead of "master"](https://github.com/rust-lang/crates.io/pull/2601)
* [clippy: added restriction lint: pattern-type-mismatch](https://github.com/rust-lang/rust-clippy/pull/4841)
* [clippy: suggest `Option::map_or`(`_else`) for `if let Some { y } else { x }`](https://github.com/rust-lang/rust-clippy/pull/5301)
* [rustfmt: do not duplicate const keyword on parameters](https://github.com/rust-lang/rustfmt/pull/4294)
* [rustfmt: do not remove fn headers (e.g., async) on extern fn items](https://github.com/rust-lang/rustfmt/pull/4291)
* [rustfmt: pick up comments between trait where clause and open block](https://github.com/rust-lang/rustfmt/pull/4292)

## Rust Compiler Performance Triage

* [2020-07-07](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-07-07). One unimportant regression on a rollup; six improvements, two on rollups.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Add a new `#[instruction_set(...)]` attribute for supporting per-function instruction set changes](https://github.com/rust-lang/rfcs/pull/2867)
* [Inline `const` expressions and patterns](https://github.com/rust-lang/rfcs/pull/2920)
* [Inline assembly](https://github.com/rust-lang/rfcs/pull/2873)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize const mem::forget](https://github.com/rust-lang/rust/pull/73887)
* [disposition: merge] [Stabilize casts and coercions to `&[T]` in const fn](https://github.com/rust-lang/rust/pull/73862)
* [disposition: merge] [mv std libs to std/](https://github.com/rust-lang/rust/pull/73265)
* [disposition: merge] [Stabilize `transmute` in constants and statics but not const fn](https://github.com/rust-lang/rust/pull/72920)
* [disposition: merge] [Stabilize const_type_id feature](https://github.com/rust-lang/rust/pull/72488)
* [disposition: merge] [Accept tuple.0.0 as tuple indexing (take 2)](https://github.com/rust-lang/rust/pull/71322)

## New RFCs

* [RFC: IndexGet and IndexSet](https://github.com/rust-lang/rfcs/pull/2953)

# Upcoming Events

### Online
* [July 9. Berlin, DE - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybckbmb/)
* [July 9. San Diego, CA, US - July 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/271680644/)
* [July 13. Seattle, WA, US - Seattle Rust Meetup - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybckbsb/)
* [July 16. Turin, IT - Rust Italia - Gruppo di studio di Rust](https://community.mozilla.org/events/gruppo-di-studio-di-rust-3/)

### North America
* [July 8. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybckblb/)
* [July 9. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/)
* [July 15. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybckbtb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is like a futuristic laser gun with an almost AI-like foot detector that turns the safety on when it recognises your foot.

– [u/goofbe on reddit](https://www.reddit.com/r/rust/comments/hiyfhq/linus_torvalds_the_kernel_team_is_looking_at/fwk12r6/)

Thanks to [Synek317](https://users.rust-lang.org/t/twir-quote-of-the-week/328/898) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hnkws3/this_week_in_rust_346/)</small>
