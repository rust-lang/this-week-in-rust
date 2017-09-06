Title: This Week in Rust 198
Number: 198
Date: 2017-09-05
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

* [Generating C bindings for Rust crates with cbindgen](http://dreamingofbits.com/post/generating-c-bindings-for-rust-crates-with-cbindgen/)

# Crate of the Week

This week's crate is [brain](https://crates.io/crates/brain), a programming language transpiler to brainfuck of all things!
Thank you, [icefoxen](https://users.rust-lang.org/u/icefoxen) for the weird suggestion. It's appreciated!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Libz blitz: crate evaluation for 2017-08-29: rayon](https://internals.rust-lang.org/t/crate-evaluation-for-2017-08-29-rayon/5795).
* [easy] [Diesel: Crash with special timestamp value in mysql (`0000-00-00 00:00:00`) through chrono](https://github.com/diesel-rs/diesel/issues/1130).
* [Diesel: Deny missing docs](https://github.com/diesel-rs/diesel/issues/563).
* [less easy] [bindgen: Add something like rustc's -Z time-passes](https://github.com/rust-lang-nursery/rust-bindgen/issues/933).
* [mio: Discuss platform support in the crate-level doc](https://github.com/carllerche/mio/issues/686).
* [mio: Link to MSDN doc on completion ports](https://github.com/carllerche/mio/issues/685).
* [mio: Consider deprecating only_v6 and set_only_v6](https://github.com/carllerche/mio/issues/684).
* [mio: Consistent case convention for name of the crate](https://github.com/carllerche/mio/issues/679).
* [mio: Clarify BSD support](https://github.com/carllerche/mio/issues/678).
* [mio: Expose EPOLLWAKEUP](https://github.com/carllerche/mio/issues/677).
* [mio: Expose EPOLLEXCLUSIVE](https://github.com/carllerche/mio/issues/676).
* [mio: Bound but not registered](https://github.com/carllerche/mio/issues/675).
* [mio: Single Evented on more than on Poll](https://github.com/carllerche/mio/issues/674).
* [mio: Multithreaded behavior of Poll](https://github.com/carllerche/mio/issues/673).
* [mio: Accept ToSocketAddrs](https://github.com/carllerche/mio/issues/668).
* [mio: SocketAddr argument to UdpSocket methods](https://github.com/carllerche/mio/issues/667).
* [mio: Rustdoc examples for mio::net methods](https://github.com/carllerche/mio/issues/665).
* [mio: Implement Hash for Event, PollOpt, Ready](https://github.com/carllerche/mio/issues/663).
* [mio: Consider using associated constants for the bitflag-like types](https://github.com/carllerche/mio/issues/661).
* [mio: Upgrade to a stable version of iovec](https://github.com/carllerche/mio/issues/660).
* [mio: Winapi in the public API of mio](https://github.com/carllerche/mio/issues/658).
* [mio: Document that UdpSocket::recv and recv_from do not read from the buffer](https://github.com/carllerche/mio/issues/657).
* [env_logger: Implement Debug for Logger and Filter](https://github.com/sebasmagri/env_logger/issues/14).
* [env_logger: Should we add new separate filter_module and filter_level to Builder](https://github.com/sebasmagri/env_logger/issues/13)?
* [env_logger: Define and implement missing impls for Logger](https://github.com/sebasmagri/env_logger/issues/7).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

120 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-08-28..2017-09-04

* [mark allocation functions as `nounwind`](https://github.com/rust-lang/rust/pull/44049)
* [`String::splice` no longer returns a `Splice`](https://github.com/rust-lang/rust/pull/44044)
* [libsyntax `Span` fields are now private](https://github.com/rust-lang/rust/pull/43968) (plugin-breaking)
* [`align_offset` intrinsic](https://github.com/rust-lang/rust/pull/43903)
* [clippy is now a submodule](https://github.com/rust-lang/rust/pull/43886)
* [`method::probe` no longer does hacky trait selection](https://github.com/rust-lang/rust/pull/43880) (some breakage, see [#44224](https://github.com/rust-lang/rust/issues/44224))
* [`errors::Handler::reset_err_count`](https://github.com/rust-lang/rust/pull/43778)
* [`core::option::Option<&mut T>::cloned`](https://github.com/rust-lang/rust/pull/43705)
* [fix `alloc_jemalloc` `debug` feature](https://github.com/rust-lang/rust/pull/43648)
* [warn when rustdoc HTML rendering differs](https://github.com/rust-lang/rust/pull/41991)
* [fix `proc_macro` expansion on trait methods](https://github.com/rust-lang/rust/pull/44089)
* [fix reachability with cross-crate generators](https://github.com/rust-lang/rust/pull/44202)
* [generalize `on_unimplemented`](https://github.com/rust-lang/rust/pull/44191) (allows to generate better compiler errors)
* [`CodeExtent` now uses `ItemLocalId` instead of `NodeId`](https://github.com/rust-lang/rust/pull/44171)
* [Allow `|` prefix in match arms](https://github.com/rust-lang/rust/pull/44108) (RFC [#1925](https://github.com/rust-lang/rfcs/blob/master/text/1925-optional-match-vert.md))
* [stabilize `mem::discriminant`](https://github.com/rust-lang/rust/pull/44263)
* [flag 128-bit integers as FFI-unsafe](https://github.com/rust-lang/rust/pull/44261)

## New Contributors

* David Ross
* Evgeniy A. Dushistov
* Jouan Amate
* Matthew Hammer
* Michal 'vorner' Vaner
* Samuel Holland
* Sebastian Humenda

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2008: Future-proofing enums/structs with `#[non_exhaustive]` attribute](https://github.com/rust-lang/rfcs/pull/2008).
* [RFC 1925: Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [RFC 2025: Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [disposition: merge] [Clamp function for primitive types](https://github.com/rust-lang/rfcs/pull/1961).
* [disposition: merge] [Generic associated types (associated type constructors)](https://github.com/rust-lang/rfcs/pull/1598).
* [disposition: merge] [Evolving Rust through checkpoints](https://github.com/rust-lang/rfcs/pull/2052).
* [disposition: merge] [Allow Irrefutable Patterns in if-let and while-let statements](https://github.com/rust-lang/rfcs/pull/2086).

## New RFCs

* [Clarify and streamline paths and visibility](https://github.com/rust-lang/rfcs/pull/2126).
* [Provide a community-wide banner server](https://github.com/rust-lang/rfcs/pull/2127).
* [Nested groups in imports](https://github.com/rust-lang/rfcs/pull/2128).
* [Crate changelogs](https://github.com/rust-lang/rfcs/pull/2129). Add changelog support to `cargo` and [crates.io](https://crates.io/).
* [Copy/Clone closures](https://github.com/rust-lang/rfcs/pull/2132). Implement `Clone` and `Copy` for closures where possible.
* [Compiler-generated Clone impls for arrays and tuples](https://github.com/rust-lang/rfcs/pull/2133).
* [Add `Option::filter` to the standard library](https://github.com/rust-lang/rfcs/pull/2124).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

We're currently writing up the discussions, we'd love some help. Check out [the tracking issue](https://github.com/rust-lang-nursery/fmt-rfcs/issues/89) for details.

PRs:

* [ranges and blocks](https://github.com/rust-lang-nursery/fmt-rfcs/pull/91)



# Upcoming Events

* [Aug 31. Mozilla Community Dresden - Rust Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/242028333/).
* [Aug 31. Rust NYC - Come learn about Rust](https://www.meetup.com/Rust-NYC/events/241866546/).
* [Aug 31. Rust London - Rust learning and hacking evening #2](https://www.meetup.com/Rust-London-User-Group/events/242378000/).
* [Aug 31. Cambridge Rust Meetup - Rust Study Group](https://www.meetup.com/Cambridge-Rust-Meetup/events/242409356/).
* [Aug 31. Rust Roma Italy - Rust learning and hacking evening #1](https://www.meetup.com/Rust-Roma/events/242709171/).
* [Aug 31. Rust Bay Area - GRPC and Turing Tarpits](https://www.meetup.com/Rust-Bay-Area/events/242797574/).
* [Sep  4. Rust Zurich - September Community Meetup](https://www.meetup.com/de-DE/Rust-Zurich/events/242032911/).
* [Sep  6. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/241813161/).
* [Sep  6. Rust Cologne - RFC Lightning Talks](https://www.meetup.com/RustCologne/events/242597353/).
* [Sep  6. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep  6. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep  6. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywmbjb/).
* [Sep  7. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Sep 11. Seattle Rust Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/242563613/).
* [Sep 12. Rust Berlin Meetup - September 2017](https://www.meetup.com/Rust-Berlin/events/242564404/).
* [Sep 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep 13. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep 14. Rust Washington, DC - Hacktember](https://www.meetup.com/RustDC/events/242847065/).
* [Sep 14. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/242459785/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Systems Engineer at Immunant](http://immunant.com/page/002_rustacean/).
* [Sr. Software Development Engineer at Amazon](https://www.amazon.jobs/en/jobs/559813/sr-software-development-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Abomonation has no safe methods. […] If you are concerned about safety, it may be best to avoid Abomonation all together. It does several things that may be undefined behavior, depending on how undefined behavior is defined.

— [Frank McSherry in Abomination docs](http://www.frankmcsherry.org/abomonation/abomonation/trait.Abomonation.html).

Thanks to [Adwhit](https://users.rust-lang.org/t/twir-quote-of-the-week/328/435) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
