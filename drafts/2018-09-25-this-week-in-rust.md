Title: This Week in Rust 253
Number: 253
Date: 2018-09-25
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

This week's crate is [mtpng](https://github.com/brion/mtpng), a parallelized PNG encoder. Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/454) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust office hours with Niko Matsakis](http://smallcultfollowing.com/babysteps/blog/2018/09/12/rust-office-hours/).
* [rust: Panic in `Receiver::recv()`](https://github.com/rust-lang/rust/issues/39364).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

131 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-09-10..2018-09-17

* [temporarily prohibit proc macro attributes placed after derives](https://github.com/rust-lang/rust/pull/54277)
* [add target thumbv7a-pc-windows-msvc](https://github.com/rust-lang/rust/pull/53621)
* [PowerPC: fix the calling convention for i1 arguments on PPC32](https://github.com/rust-lang/llvm/pull/127)
* [allow for opting out of ThinLTO and clean up LTO related cli flag handling](https://github.com/rust-lang/rust/pull/53950)
* [resolve: allow only core, std, meta and --extern in Rust 2018 paths](https://github.com/rust-lang/rust/pull/54116)
* [resolve: do not error on access to proc macros imported with `#[macro_use]`](https://github.com/rust-lang/rust/pull/53461)
* [add inspection and setter methods to proc_macro::Diagnostic](https://github.com/rust-lang/rust/pull/52896)
* [support ascription for patterns in NLL](https://github.com/rust-lang/rust/pull/53873)
* [allow named lifetimes in async functions](https://github.com/rust-lang/rust/pull/54000)
* [suggest && and || instead of 'and' and 'or'](https://github.com/rust-lang/rust/pull/54181)
* [use structured suggestion for "missing mut" label](https://github.com/rust-lang/rust/pull/54157)
* [de-overlap the lifetimes of `flow_inits` and `flow_{un,ever_}inits`](https://github.com/rust-lang/rust/pull/54213)
* [don't compute padding of braces unless they are unmatched](https://github.com/rust-lang/rust/pull/54092)
* [don't suggest extra clone when converting cloned slice to Vec](https://github.com/rust-lang/rust/pull/54080)
* [reexport `CheckLintNameResult`](https://github.com/rust-lang/rust/pull/54106)
* [miri: keep around some information for dead allocations](https://github.com/rust-lang/rust/pull/54254)
* [miri loop detector hashing](https://github.com/rust-lang/rust/pull/54076)
* [fix some uses of pointer intrinsics with invalid pointers](https://github.com/rust-lang/rust/pull/53804)
* [first step towards `u128` instead of `Const` in `PatternKind::Range`](https://github.com/rust-lang/rust/pull/51159)
* [stabilize outlives requirements](https://github.com/rust-lang/rust/pull/53793)
* [stabilize `#[used]`](https://github.com/rust-lang/rust/pull/51363)
* [stabilize slice_align_to](https://github.com/rust-lang/rust/pull/53754)
* [implement `tuple_struct_self_ctor`](https://github.com/rust-lang/rust/pull/53751) (RFC [#2302](https://rust-lang.github.io/rfcs/2302-tuple-struct-self-ctor.html))
* [implement `map_or_else` for `Result<T, E>`](https://github.com/rust-lang/rust/pull/53777)
* [add a implementation of `From` for converting `&'a Option<T>` into `Option<&'a T>`](https://github.com/rust-lang/rust/pull/53218)
* [cargo: add empty ctrlc handler on Windows](https://github.com/rust-lang/cargo/pull/6004)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2361: Simpler alternative `dbg!()` macro](https://github.com/rust-lang/rfcs/pull/2361).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Deny the `overflowing_literals` lint for the 2018 edition](https://github.com/rust-lang/rfcs/pull/2438).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for eRFC 2497, "if- and while-let-chains, take 2", edition changes](https://github.com/rust-lang/rust/issues/53668).
* [disposition: merge] [Fix camel case type warning for types with trailing underscores](https://github.com/rust-lang/rust/pull/54101).
* [disposition: merge] [Support an explicit annotation for marker traits](https://github.com/rust-lang/rust/pull/53693).

## New RFCs

* [Elide array size](https://github.com/rust-lang/rfcs/pull/2545).
* [Make the turbofish syntax redundant](https://github.com/rust-lang/rfcs/pull/2544).
* [Use `T: ToString` for `thread::Builder::name`](https://github.com/rust-lang/rfcs/pull/2541).

# Upcoming Events

### Online

* [Sep 25. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Sep 26. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Sep 26. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Oct 3. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Oct 2. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxnbdb/).

### Asia

* [Oct 3. Kuala Lumpur, MY - Rust Lang Meetup - Project X](https://www.facebook.com/events/190938831689130/).

### Europe

* [Sep 27. Helsinki, FI - Rust is back with Embedded topics](https://www.meetup.com/Finland-Rust-Meetup/events/254758208/).
* [Oct 1. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/BcnRust/events/254655075/).
* [Oct 3. Vilnius, LT - Vilnius Rust Meetup #3 - Network Simulation and WebAssembly](https://www.meetup.com/Rust-in-Vilnius/events/254403141/).
* [Oct 3. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/xkdlvpyxnbfb/).

### North America

* [Sep 23. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbfc/).
* [Sep 24. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxmbgc/).
* [Sep 25. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxmbhc/).
* [Sep 30. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbnc/).
* [Oct 3. Indianopolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxnbfb/).
* [Oct 3. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxnbfb/).
* [Oct 3. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/dqldspyxnbfb/).
* **[Oct 19 & 20. Ann Arbor, US - Rust Belt Rust 2018](https://rust-belt-rust.com/).**


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [RustBelt is looking for postdocs and PhD students](https://plv.mpi-sws.org/rustbelt/#positions).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Sometimes bad designs will fail faster in Rust

– [Catherine West @ Rustconf](https://youtu.be/aKLntZcp27M?t=1444).

Thanks to [kornel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/561) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
