Title: This Week in Rust 331
Number: 331
Date: 2020-03-24
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

This week's crates is [beef](https://github.com/maciejhirsz/beef), an alternative memory-compact Clone on Write (CoW) implementation.

Thanks to [Vlad Frolov](https://users.rust-lang.org/t/crate-of-the-week/2704/740) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [This Week in Rust is looking for a new maintainer](https://blog.rust-lang.org/inside-rust/2020/03/13/twir-new-lead.html).
* [The swc project needs some help from community](https://swc-project.github.io/blog/2020/03/16/roadmap-and-call-for-help).
* [The RustConf 2020 CFP is now open](https://cfp.rustconf.com/events/rustconf-2020). We'd love to hear from you at RustConf!

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

309 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-03-09..2020-03-16

* [change `DIBuilderCreateEnumerator` signature to match LLVM 9](https://github.com/rust-lang/rust/pull/69734)
* [add support for LLVM globals corresponding to miri allocations should be named `alloc123`](https://github.com/rust-lang/rust/pull/69155)
* [emit 1-based column numbers in debuginfo](https://github.com/rust-lang/rust/pull/69357)
* [improve expression & attribute parsing](https://github.com/rust-lang/rust/pull/69760)
* [resolve: fix two issues in fresh binding disambiguation](https://github.com/rust-lang/rust/pull/70006)
* [don't store locals in generators that are immediately overwritten with the resume argument](https://github.com/rust-lang/rust/pull/69716)
* [make `PlaceRef` take just one lifetime](https://github.com/rust-lang/rust/pull/69714)
* [use `TypeRelating` for instantiating query responses](https://github.com/rust-lang/rust/pull/69591)
* [perf: reuse a `Vec` in mir simplification](https://github.com/rust-lang/rust/pull/68551)
* [exhaustiveness checking, `Matrix::push`: recursively expand or-patterns](https://github.com/rust-lang/rust/pull/69891)
* [miri: use a session variable instead of checking for an env var always](https://github.com/rust-lang/rust/pull/69888)
* [`panic_bounds_check`: use caller_location, like `PanicFnLangItem`](https://github.com/rust-lang/rust/pull/69850)
* [check if output is immediate value](https://github.com/rust-lang/rust/pull/69836)
* [fix memory leak when `vec::IntoIter` panics during drop](https://github.com/rust-lang/rust/pull/69828)
* [optimize `catch_unwind` to match C++ try/catch](https://github.com/rust-lang/rust/pull/67502)
* [make `mem::discriminant` const](https://github.com/rust-lang/rust/pull/69825)
* [allow zero-sized types in `AllocRef`](https://github.com/rust-lang/rust/pull/69799)
* [`mem::zeroed`/`uninit`: panic on types that do not permit zero-initialization](https://github.com/rust-lang/rust/pull/66059)
* [add `Display` and `Error` impls for `proc_macro::LexError`](https://github.com/rust-lang/rust/pull/68899)
* [implement `Error` for `TryReserveError`](https://github.com/rust-lang/rust/pull/69792)
* [implement `nth`, `last`, and `count` for `iter::Copied`](https://github.com/rust-lang/rust/pull/69625)
* [add undo_leak to reset `RefCell` borrow state](https://github.com/rust-lang/rust/pull/69528)
* [implement `Copy` for `IoSlice`](https://github.com/rust-lang/rust/pull/69403)
* [stabilize const for `integer `{`to`, `from`}`_`{`be`, `le`, `ne`}`_bytes` methods](https://github.com/rust-lang/rust/pull/69373)
* [implement `From<&mut str>` for `String`](https://github.com/rust-lang/rust/pull/69661)
* [hashbrown: optimize `Clone` implementation](https://github.com/rust-lang/hashbrown/pull/146)
* [futures: use `once_cell` for sound `&'static Waker` instances](https://github.com/rust-lang/futures-rs/pull/2095)
* [cargo: avoid buffering large amounts of rustc output](https://github.com/rust-lang/cargo/pull/7838)
* [cargo: clippy integration changes](https://github.com/rust-lang/cargo/pull/7533)
* [cargo: add "Updating" status for git submodules](https://github.com/rust-lang/cargo/pull/7989)
* [docs.rs: allow crates to opt-in to building a single target](https://github.com/rust-lang/docs.rs/pull/632)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Allow obtaining &mut OsStr](https://github.com/rust-lang/rust/pull/70048).
* [disposition: merge] [`is_x86_feature_detected!("avx512f")` fails to build on beta and nightly](https://github.com/rust-lang/rust/issues/68905).
* [disposition: merge] [impl From<[T; N]> for Vec<T>](https://github.com/rust-lang/rust/pull/68692).
* [disposition: merge] [Amend Rc/Arc::from_raw() docs regarding unsafety](https://github.com/rust-lang/rust/pull/68099).

## New RFCs

* [Placement by return](https://github.com/rust-lang/rfcs/pull/2884).

# Upcoming Events

### Online

* [Mar 20. Berlin, DE - Oxidize 1K: A Remote Conference](https://oxidizeconf.com/oxidize-1k/).
* [Mar 25. Portland, OR, US - PDXRust - Lightning Talks - Remote Event](https://www.meetup.com/PDXRust/events/269447550/).
* [Mar 23. Durham, NC, US - Triangle Rustaceans - Online gathering](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcfbfc/).

### Europe

* [Mar 26. Edinburgh, GB - Rust Edinburgh - Rust Meetup Short Talk Night](https://www.meetup.com/rust-edi/events/267810816).
* [Apr  2. Stockholm, SE - Stockholm Rust Meetup](https://www.goto10.se/evenemang/stockholm-rust-meetup/).
* [Apr  2. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcgbdb/).

### North America

* [Mar 25. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Mar 25. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscrybcfbhc/).
* [Mar 31. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcfbpc/).
* [Apr  1. Indianapolis, IN, US - Indy.rs - WebAssembly 101](https://www.meetup.com/indyrs/events/dtqwprybcgbcb/).
* [Apr  8. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcgblb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I thought up a clever qotw bait one liner to stick in here that prompted me to actually write it then forgot it while writing the post in favor of being genuine... whoops

– [Christopher Durham confessing to rust-users](https://users.rust-lang.org/t/the-confessional-thread-parts-of-rust-that-i-still-dont-get-after-all-this-time/39022/14)

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/t/twir-quote-of-the-week/328/835) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
