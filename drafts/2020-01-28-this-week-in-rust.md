Title: This Week in Rust 323
Number: 323
Date: 2020-01-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

# Crate of the Week

This week's crate is [fasteval](https://crates.io/crates/fasteval), a crate for fast and safe evaluation of algebraic expressions.

Thanks to [Christopher Sebastian](https://users.rust-lang.org/t/crate-of-the-week/2704/705) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Papers: Rust LATAM Mexico 2020](https://www.reddit.com/r/rust/comments/em0ru8/rust_2020_a_conference_in_latin_america).
* [time: Implement function returning the local UTC offset](https://github.com/time-rs/time/issues/203).
* [rusty-celery: Multiple issues marked "Status: Help Wanted", good place to start for newcomers](https://github.com/rusty-celery/rusty-celery/issues?q=is%3Aissue+is%3Aopen+label%3A%22Status%3A+Help+Wanted%22).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

270 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-01-13..2020-01-20

* [array repeat expression lengths must be monomorphic at MIR building time](https://github.com/rust-lang/rust/pull/68285)
* [make sure that all upstream generics get re-exported from Rust dylibs](https://github.com/rust-lang/rust/pull/68277)
* [forbid elided lifetimes within const generic parameter types](https://github.com/rust-lang/rust/pull/68143)
* [do not ICE on malformed suggestion spans](https://github.com/rust-lang/rust/pull/68256)
* [untangle ZST validation from integer validation and generalize it to all zsts](https://github.com/rust-lang/rust/pull/68219)
* [don't try to `force_ptr` pointers to zsts](https://github.com/rust-lang/rust/pull/68088)
* [perf: eagerly convert literals to consts](https://github.com/rust-lang/rust/pull/68118)
* [update compiler_builtins with changes to fix 128 bit integer remainder for aarch64 windows](https://github.com/rust-lang/rust/pull/68233)
* [handle recursive instantiation of drop shims](https://github.com/rust-lang/rust/pull/67731)
* [add unreachable propagation mir optimization pass](https://github.com/rust-lang/rust/pull/66329)
* [rebase LLVM onto 9.0.1](https://github.com/rust-lang/rust/pull/68030)
* [don't run const propagation on items with inconsistent bounds](https://github.com/rust-lang/rust/pull/67914)
* [don't use f64 shims for f32 cmath functions on non 32-bit x86 MSVC](https://github.com/rust-lang/rust/pull/68033)
* [stabilize slice patterns](https://github.com/rust-lang/rust/pull/67712)
* [reset Formatter flags on exit from pad_integral](https://github.com/rust-lang/rust/pull/67784)
* [optimize size/speed of Unicode datasets](https://github.com/rust-lang/rust/pull/68232)
* [stabilize `Condvar::`{`wait_while`, `wait_timeout_while`}](https://github.com/rust-lang/rust/pull/67076)
* [stabilize `ManuallyDrop::take`](https://github.com/rust-lang/rust/pull/68066)
* [make `iter::Empty<T>` `Send` and `Sync` for any `T`](https://github.com/rust-lang/rust/pull/68348)
* [implement `DebugStruct::non_exhaustive`](https://github.com/rust-lang/rust/pull/66716)
* [implement `Cursor` for linked lists](https://github.com/rust-lang/rust/pull/68123) (RFC #[2570](https://rust-lang.github.io/rfcs/2570-linked-list-cursors.html))

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Introduce the ASM project group](https://github.com/rust-lang/rfcs/pull/2836).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `ptr::slice_from_raw_parts[_mut]`](https://github.com/rust-lang/rust/pull/68234).
* [disposition: merge] [Stabilize `#[repr(transparent)]` on `enum`s in Rust 1.42.0](https://github.com/rust-lang/rust/pull/68122).
* [disposition: merge] [Stabilize the debug_map_key_value feature](https://github.com/rust-lang/rust/pull/68200).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Europe

* [Jan 23. Paris, FR - Rust Paris meetup #49](https://www.meetup.com/Rust-Paris/events/267250053/).
* [Jan 23. Warsaw, PL - Rust Warsaw 3](https://www.meetup.com/Rust-Warsaw/events/267525144/).
* [Feb  2. Brussels, BE - Rust devroom @ FOSDEM](https://fosdem.org/2020/schedule/track/rust/).
* [Jan 22. Hamburg, DE - Rust Hack & Learn January 2020](https://www.meetup.com/Rust-Meetup-Hamburg/events/267692684/).
* [Jan 23. Zagreb, HR - impl Zagreb for Rust - Rust Meetup 2020/01: Uvod u lock-free Rust](https://www.meetup.com/Zagreb-Rust-Meetup/events/267742601).
* [Jan 29. Copenhagen, DK - Copenhagen Rust Hack Night #21](https://cph.rs/).
* [Feb  5. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgrybcdbhb/).

### North America

* [Jan 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybccbkc/).
* [Jan 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybccblc/).
* [Jan 29. Chicago, IL, US - Chicago Rust Meetup - Byte Sized Rust - AWS Lambdas & Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/267616019/).
* [Jan 29. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Feb  5. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/qgvxlrybcdbhb/).
* [Feb  5. Atlanta, GA, US - Rust Atlanta - Static Assertions Internals](https://www.meetup.com/Rust-ATL/events/qxqdgrybcdbqb/).
* [Feb  5. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpybcdbhb/).

### South America

* [Jan 23. Buenos Aires, AR - Rust Argentina - Encuentro de Enero](https://www.meetup.com/Rust-Argentina/events/267904544/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> `Rc<RefCell>` is like duct tape.
>
> It's very versatile, and can fix a multitude of problems in a pinch. For some problems, it's even the best thing to use. But if the thing you're building is more than about 10% wrapped in duct tape, you might want to reconsider your design process!

– [trentj on rust-users](https://users.rust-lang.org/t/why-do-all-docs-say-refcell-is-bad/37086/22)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/798) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
