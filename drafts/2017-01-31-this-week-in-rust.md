Title: This Week in Rust 167
Number: 167
Date: 2017-01-31
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

## Other Weeklies from Rust Community

# Crate of the Week

This week's crate of the week is [tantivy](https://crates.io/crates/tantivy), a full text search engine, akin to Lucene. Thanks to [Jos van den Oever](https://users.rust-lang.org/users/vandenoever) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Removal of the lang feature gate tests whitelist](https://github.com/rust-lang/rust/issues/39059).
* [rust: Make Rust on wasm + emscripten a reliable, 1st class Rust target](https://github.com/rust-lang/rust/issues/38805).
* [easy] [rust: Rvalue static promotion](https://github.com/rust-lang/rust/issues/38865).
* [easy] [Diesel: Refactorings using macros in type position](https://github.com/diesel-rs/diesel/issues/521).
* [easy] [Diesel: Deny missing docs](https://github.com/diesel-rs/diesel/issues/563).
* [android-rs-glue: Add more arguments and use clap to parse the arguments](https://github.com/tomaka/android-rs-glue/issues/115).
* [tokei: Add package repositories](https://github.com/Aaronepower/tokei/issues/92).
* [RustCrypto/hashes: Missing hash functions](https://github.com/RustCrypto/hashes/issues/1).
* [RustCrypto/block-ciphers: Missing block ciphers](https://github.com/RustCrypto/block-ciphers/issues/1).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

103 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-23..2017-01-30

* [stabilizations for the 1.16.0 release](https://github.com/rust-lang/rust/pull/39307)
* [stabilize `Self` and associated types in struct exprs and patterns](https://github.com/rust-lang/rust/pull/39282)
* [`Self` in impl headers](https://github.com/rust-lang/rust/pull/38920) (Partially implements RFC [#1647](https://github.com/rust-lang/rfcs/blob/master/text/1647-allow-self-in-where-clauses.md))
* [More lenient bounds parsing](https://github.com/rust-lang/rust/pull/39158) (to ease macros writing)
* [add `std::process::Command::envs(_)`](https://github.com/rust-lang/rust/pull/38856)
* [`impl ToSocketAddrs for String`](https://github.com/rust-lang/rust/pull/39048)
* [binary ops will now note invalid double refs](https://github.com/rust-lang/rust/pull/38617)
* [remove frequently wrong "add lifetime parameter" suggestion](https://github.com/rust-lang/rust/pull/37057)
* [remove obsolete `Reflect` trait](https://github.com/rust-lang/rust/pull/39075)
* [remove recursive `PartialEq` impl](https://github.com/rust-lang/rust/pull/39380)
* [fix another endianness issue with 128 bit integers](https://github.com/rust-lang/rust/pull/39332)
* [use `__SIZEOF__INT128__` to detect 128 bit integer support](https://github.com/rust-lang/rust/pull/39350)
* [fix parsing inconsistency with `return`](https://github.com/rust-lang/rust/pull/39335)
* [`x..y` exclusive range patterns](https://github.com/rust-lang/rust/pull/35712)
* [fix ICE when unprettying MIR](https://github.com/rust-lang/rust/pull/39311)
* [`tcx.map` is now `tcx.hir`](https://github.com/rust-lang/rust/pull/39309) (possibly plugin-breaking)
* [make lifetime elision syntactic](https://github.com/rust-lang/rust/pull/39305)
* [remove temporary lifetime extension borrow hints](https://github.com/rust-lang/rust/pull/39066)
* [`save-analysis` now visits paths explicitly](https://github.com/rust-lang/rust/pull/39286)
* [`save-analysis`: ICE after error averted](https://github.com/rust-lang/rust/pull/39285)
* [`cfg-mods` option to parse `cfg`d out modules](https://github.com/rust-lang/rust/pull/39145)
* [hide more internal symbols](https://github.com/rust-lang/rust/pull/39252)
* [make incremental compilation cross-crate-tracking optional](https://github.com/rust-lang/rust/pull/39281)
* [improve cargo error message on different dependency source paths](https://github.com/rust-lang/cargo/pull/3593)

## New Contributors

* Colm Seale
* Constantin
* Eijebong
* gralpli
* Jack Vickeridge
* Jacob Wahlgren
* Josh
* krdln
* Lin Clark
* Martin Hafskjold Thoresen
* Matthew Dawson
* Richard S. Imaoka
* Stephen E. Baker
* theduke

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1651: Extend `Cell` to work with non-`Copy` types](https://github.com/rust-lang/rfcs/pull/1651).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).

## Closed RFCs

Following proposals were rejected by [the team](https://www.rust-lang.org/team.html) after their 'final comment period' elapsed.

* [Const-dependent type system (also known as, Π-types and value-types)](https://github.com/rust-lang/rfcs/pull/1657).
* [Add syntax for expressing tuples as a head and tail pair, similar to a Lisp cons cell](https://github.com/rust-lang/rfcs/pull/1582).

## New RFCs

* [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [A portability lint](https://github.com/rust-lang/rfcs/pull/1868).
* [Improve the `assert_eq` failure message formatting to increase legibility](https://github.com/rust-lang/rfcs/pull/1866).
* [Add official Gitter and Slack channels to compliment our official IRC channels](https://github.com/rust-lang/rfcs/pull/1865).
* [Add `extern type` declarations for declaring types from external libraries which have an unknown size/layout](https://github.com/rust-lang/rfcs/pull/1861).
* [Include the `ManuallyDrop` wrapper in `core::mem`](https://github.com/rust-lang/rfcs/pull/1860).
* [Extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [Add built-in trait `Move` which all existing types will implement. Types which do not implement it cannot move after they have been borrowed](https://github.com/rust-lang/rfcs/pull/1858).
* [Add metadata to diagnostic messages' json output](https://github.com/rust-lang/rfcs/pull/1855).
* [Stabilize drop order](https://github.com/rust-lang/rfcs/pull/1857).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [match](https://github.com/rust-lang-nursery/fmt-rfcs/pull/56)
* [type aliases](https://github.com/rust-lang-nursery/fmt-rfcs/pull/55)
* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)

Ready for PR:

There's [a lot of them](https://github.com/rust-lang-nursery/fmt-rfcs/issues?q=is%3Aopen+is%3Aissue+label%3Aready-for-PR) right now, contributions here would be very welcome. If you want advice or help getting started, please ping nrc, or any other member of the style team, in #rust-style.

Issues in final comment period:

* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [`..` vs `_`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/49).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).
* [enum declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/31).
* [generics declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/29).

Other significant issues:

* [types](https://github.com/rust-lang-nursery/fmt-rfcs/issues/15)
* [closures](https://github.com/rust-lang-nursery/fmt-rfcs/issues/35)
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)
* [`extern` vs `extern "C"`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/52)

# Upcoming Events

* [1/25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/25. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/25. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658932/).
* [1/26. Rust Stockholm: REST in Rust and Rust Hack Night](https://www.meetup.com/ruststhlm/events/236791788/).
* [1/26. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [1/28. Rust MX - Rust Meetup in Mexico City](https://www.meetup.com/Rust-MX/events/236642131/).
* [2/1. Rust User Group Cologne - Macros 1.1](http://rust.cologne/2017/02/01/proc-macros.html).
* [2/1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/2. Stockholm Google Developer Group - Rust Talk](https://www.meetup.com/Stockholm-Google-Developer-Group/events/236959999/).
* [2/4 - 2/5: FOSDEM 2017 Belgium - Meeting for Rustaceans](https://fosdem.org/2017/schedule/event/rust_bof/).
* [2/8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/9. Rust Boulder/Denver - Redox OS with Denver Open Source OS](https://www.meetup.com/Rust-Boulder-Denver/events/237016107/).
* [2/9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/236907254/).
* [2/9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior backend developer at OneSignal](https://angel.co/onesignal/jobs/128684-senior-backend-developer).
* [Rust backend developer at 1aim.com](https://news.ycombinator.com/item?id=13302210).
* [Rust systems programmer at Hadean](https://news.ycombinator.com/item?id=13301893).
* [Rust engineer at MaidSafe](https://maidsafe.net/careers.html#rust_engineer)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Training opportunities

Three day Rust course at [LinuxHotel](http://www.linuxhotel.de/kurs/rust/). (German)

# Quote of the Week

> Yeah, it's like learning to dance when your partner [borrow checker] already knows all the steps. When you're just getting started, you step on their toes a lot, but over time you get the motions down. Eventually, you can start to anticipate their movements and start to appreciate the music as part of the dance, instead of just concentrating on getting your feet in the right place.

— [QuietMisdreavus on reddit](https://www.reddit.com/r/rust/comments/5okn5y/this_week_in_rust_165/dcl0vv4/).

Thanks to [matthieum for the suggestion](https://www.reddit.com/r/rust/comments/5okn5y/this_week_in_rust_165/dclejnt/).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
