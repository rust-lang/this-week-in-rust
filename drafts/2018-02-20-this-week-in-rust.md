Title: This Week in Rust 222
Number: 222
Date: 2018-02-20
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

* 🎈🎉 [Announcing Rust 1.24](https://blog.rust-lang.org/2018/02/15/Rust-1.24.html). 🎉🎈
* [Rayon 1.0 is released](https://github.com/rayon-rs/rayon).
* [Initial support for Rust has landed in quicktype](https://github.com/quicktype/quicktype/pull/512). quicktype generates types and converters from JSON.
* [Announcement: Rust team structure revamp](https://internals.rust-lang.org/t/rust-team-structure-revamp/6776).
* [Embedded Rust: Zero cost stack overflow protection for ARM Cortex-M devices](http://blog.japaric.io/stack-overflow-protection/).
* [Borrow cycles in Rust: arenas v.s. drop-checking](https://exyr.org/2018/rust-arenas-vs-dropck/).
* [Porting Rust to WebAssembly](https://udoprog.github.io/rust/2018-02-19/porting-rust-to-wasm.html).
* [Mutation testing Rust in earnest](https://llogiq.github.io/2018/02/14/mutagen.html).
* [“The expressive C++17 coding challenge (in Rust)” revisited](http://words.steveklabnik.com/the-expressive-c-17-coding-challenge-in-rust-revisited).
* [Write a plugin system and script your app in Rust](http://phaazon.net/blog/spectra_plugins).
* [Sorting in Rust: selection, insertion, and counting sort](https://medium.com/@spyr1014/sorting-in-rust-selection-insertion-and-counting-sort-2c4d3575e364).
* [This week in Rust docs 93](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-93).
* [podcast] [Rusty Spike Podcast - episode 19](https://rusty-spike.blubrry.net/2018/02/15/episode-19-feb-14-2018/). new teams, humble bundle, SIMD, being special, quicktype, and deps.rs.

# Crate of the Week

This week sadly had to go without a crate for lack of votes.

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

117 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-05..2018-02-12

* epochs: [rustc](https://github.com/rust-lang/rust/pull/48014) and [cargo](https://github.com/rust-lang/cargo/pull/5011) (RFC [#2052](https://rust-lang.github.io/rfcs/2052-epochs.html))
* [rustc: upgrade to LLVM 6](https://github.com/rust-lang/rust/pull/47828)
* [customizable extended tools](https://github.com/rust-lang/rust/pull/48015)
* [fix ICE for mismatched args on target without span](https://github.com/rust-lang/rust/pull/48047)
* [proc_macro: don't panic parsing `..=`](https://github.com/rust-lang/rust/pull/48036)
* [implement `?` macro repetition](https://github.com/rust-lang/rust/pull/47752)
* [warn about more ignored bounds in type aliases](https://github.com/rust-lang/rust/pull/48020)
* [do not ignore lifetime bounds in Copy impls](https://github.com/rust-lang/rust/pull/47877)
* [add filtering options to `rustc_on_unimplemented`](https://github.com/rust-lang/rust/pull/47613)
* [rustc: Add `#[rustc_args_required_const]`](https://github.com/rust-lang/rust/pull/48018)
* [rustc_mir: insert a dummy access to places being matched on, when building MIR](https://github.com/rust-lang/rust/pull/48092)
* [emit data::Impl in save-analysis](https://github.com/rust-lang/rust/pull/47657)
* [ui tests: diff from old (expected) to new (actual) instead of backwards](https://github.com/rust-lang/rust/pull/47978)
* [NLL: improve `DefiningTy::Const`](https://github.com/rust-lang/rust/pull/47957)
* [NLL: add false edges out of infinite loops](https://github.com/rust-lang/rust/pull/47802)
* [stabilize `use_nested_groups`](https://github.com/rust-lang/rust/pull/47948)
* [implement `TrustedLen` for `Take<Repeat>` and `Take<RangeFrom>`](https://github.com/rust-lang/rust/pull/47944)
* [override `try_(r)fold` for RangeInclusive](https://github.com/rust-lang/rust/pull/48012)
* [add some APIs to ptr::NonNull](https://github.com/rust-lang/rust/pull/47631)
* [add `-Zteach` documentation](https://github.com/rust-lang/rust/pull/47843)
* [update book](https://github.com/rust-lang/rust/pull/47753)
* [fix rustdoc ICE on macros defined within functions](https://github.com/rust-lang/rust/pull/47959)
* [make resolution backtracking smarter](https://github.com/rust-lang/cargo/pull/4834)
* [do not rename packages on `cargo new`](https://github.com/rust-lang/cargo/pull/5013)

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
* [Ma4  5. London, GB - Rust learning and hacking evening #8](https://www.meetup.com/Rust-London-User-Group/events/247286584/).
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
