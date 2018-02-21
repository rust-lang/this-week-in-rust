Title: This Week in Rust 223
Number: 223
Date: 2018-02-27
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

This week's crate is [afl.rs](https://github.com/rust-fuzz/afl.rs), a by now pretty well-known fuzzing tool for Rust. Thanks to [Philipp Hansch](https://users.rust-lang.org/u/phansch) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [Mutagen: A mutation testing framework](https://github.com/llogiq/mutagen/issues) needs help with some beginner-friendly issues.
* [good first issue] [errno: Port library to winapi 0.3](https://github.com/lfairy/rust-errno/issues/14).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

95 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-12..2018-02-19

* [don't promote the result of dereferences to `'static`](https://github.com/rust-lang/rust/pull/47408)
* [rustc: persist LLVM's `Linker` in Fat LTO](https://github.com/rust-lang/rust/pull/48163)
* [incr.comp.: run cache directory garbage collection before loading dep-graph](https://github.com/rust-lang/rust/pull/48181)
* [continue parsing function after finding `...` arg](https://github.com/rust-lang/rust/pull/48154)
* [remove allocation from width of character function](https://github.com/rust-lang/rust/pull/48167)
* [unimplement Send/Sync for ::env::{Args,ArgsOs,Vars,VarsOs}](https://github.com/rust-lang/rust/pull/48005)
* [support `default impl` for specialization](https://github.com/rust-lang/rust/pull/45404)
* [`PanicInfo` and `Location` API changes](https://github.com/rust-lang/rust/pull/47687) (RFC #2070)
* [optimize `Vec::retain`](https://github.com/rust-lang/rust/pull/48065)
* [early exit for empty HashMap](https://github.com/rust-lang/rust/pull/48035)
* [add `Range(Inclusive)::is_empty`](https://github.com/rust-lang/rust/pull/48087)
* [add `std`/`core::iter::repeat_with`](https://github.com/rust-lang/rust/pull/48156)
* [`cargo new` defaults to bin](https://github.com/rust-lang/cargo/pull/5029)
* [cargo conflict tracking](https://github.com/rust-lang/cargo/pull/5037)
* [remove hoedown from rustdoc](https://github.com/rust-lang/rust/pull/48274)
* [compiletest: delete the compiled program once its test is done](https://github.com/rust-lang/rust/pull/48144)

And my personal favourite:

* [this is the ideal FileType on Windows. You may not like it, but this is what peak performance looks like](https://github.com/rust-lang/rust/pull/47956)

## New Contributors

* Alex Crawford
* Antoni Boucher
* Artyom Pavlov
* Brad Gibson
* Jacob Hughes
* Mazdak Farrokhzad
* Paolo Teti
* Pramod Bisht
* roblabla
* Ross Light
* Shaun Steenkamp

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2195: Formally define repr(u32, i8, etc...) and repr(C) on enums with payloads](https://github.com/rust-lang/rfcs/pull/2195).
* [RFC 2056: Allow trivial constraints to appear in where clauses](https://github.com/rust-lang/rfcs/pull/2056).
* [RFC 2175: or-patterns in if / while let expressions](https://github.com/rust-lang/rfcs/pull/2175).
* [RFC 2166: impl-only-use](https://github.com/rust-lang/rfcs/pull/2166). The `use …::{… as …}` syntax can now accept `_` as alias to a trait to only import the implementations of such a trait.
* [Issues are not feature requests](https://github.com/rust-lang/rfcs/pull/2299).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Rust 2018 roadmap](https://github.com/rust-lang/rfcs/pull/2314).
* [disposition: merge] [Stable SIMD in Rust](https://github.com/rust-lang/rfcs/pull/2325).
* [disposition: merge] [Cargo profile dependencies](https://github.com/rust-lang/rfcs/pull/2282).
* [disposition: merge] [Prior art](https://github.com/rust-lang/rfcs/pull/2333). A section to the RFC template where RFC authors may discuss the experience of other programming languages.
* [disposition: merge] [Amend RFC 0141 Lifetime elision: Mention deduplicated lifetimes](https://github.com/rust-lang/rfcs/pull/2330).
* [disposition: merge] [Hexadecimal integers with fmt::Debug, including within larger types](https://github.com/rust-lang/rfcs/pull/2226).
* [disposition: merge] [Raw identifiers](https://github.com/rust-lang/rfcs/pull/2151). Add a raw identifier format `r#ident`, so crates written in future language epochs/versions can still use an older API that overlaps with new keywords.
* [disposition: merge] [label-break-value](https://github.com/rust-lang/rfcs/pull/2046). Allow a break not only out of `loop`, but of labelled blocks with no loop
* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [`?` repetition in macro rules](https://github.com/rust-lang/rfcs/pull/2298).
* [disposition: postpone] [Immovable types](https://github.com/rust-lang/rfcs/pull/1858). Add built-in trait `Move` which all existing types will implement. Types which do not implement it cannot move after they have been borrowed
* [disposition: postpone] [Proper tail calls](https://github.com/rust-lang/rfcs/pull/1888). Explicit proper tail calls to Rust via the `become` keyword.
* [disposition: postpone] [Movable array iterators](https://github.com/rust-lang/rfcs/pull/2185).
* [disposition: close] [Implement Add for OsString](https://github.com/rust-lang/rfcs/pull/2020).

## New RFCs

* [Allow `if` and `match` in constants](https://github.com/rust-lang/rfcs/pull/2342).
* [Allow locals and destructuring in const fn](https://github.com/rust-lang/rfcs/pull/2341).
* [Type alias enum variants](https://github.com/rust-lang/rfcs/pull/2338).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Feb 22. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb 22. London, GB - LDN Talks: February 2018](https://www.meetup.com/Rust-London-User-Group/events/246860921/).
* [Feb 22. Minneapolis, US - February 2018 Meetup](https://www.meetup.com/RustMN/events/247512052/).
* [Feb 25. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxdbxb/).
* [Feb 26. Durham, US - Triangle Rustaceans - Rust 102 -- Choose Your Own Adventure](https://www.meetup.com/triangle-rustaceans/events/kkjnpnyxdbjc/).
* [Feb 27. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-content).
* [Feb 28. Milano, IT - Rust Language Milano - Da Rust a Firefox Quantum passando da Servo](https://www.meetup.com/rust-language-milano/events/247930375/).
* [Feb 28. Denver, US - denver.rs reactivate!()](https://www.meetup.com/Rust-Boulder-Denver/events/247751967/).
* [Feb 28. Moscow, RU - Rust Meetup from Exonum & Parity developers](https://bitfury.timepad.ru/event/665119/).
* [Feb 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 28. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 28. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar  4. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbgb/).
* [Mar  5. London, GB - Rust learning and hacking evening #8](https://www.meetup.com/Rust-London-User-Group/events/247286584/).
* [Mar  6. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxfbjb/).
* [Mar  7. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlyxfbkb/).
* [Mar  7. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxfbkb/).
* [Mar  7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  8. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxfblb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer at Facebook](https://www.facebook.com/careers/jobs/a0I1H00000LCTYYUA5/.)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
