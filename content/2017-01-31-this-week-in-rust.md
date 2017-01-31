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

* [An overview of macros in Rust](http://words.steveklabnik.com/an-overview-of-macros-in-rust).
* [Lowering Rust traits to logic](http://smallcultfollowing.com/babysteps/blog/2017/01/26/lowering-rust-traits-to-logic/).
* [Lazy initialization in Rust](http://blog.kylehuey.com/post/156464146312/lazy-initialization-in-rust).
* [How-to optimize Rust programs on Linux](https://jbendig.github.io/fix-rs/2017/01/24/how-to-optimize-rust-programs-on-linux/).
* [A guide to porting C/C++ to Rust](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/).
* [Building high performance REST APIs with Rust and Rocket](https://github.com/sebasmagri/rocket-loadavg-api/blob/master/README.md).
* [Building an asynchronous Hyper server](https://mgattozzi.com/hyper-async).
* [The struggle with Rust](https://ayende.com/blog/176801/the-struggle-with-rust).
* [Setting expectations for Rust's difficulty](http://www.suspectsemantics.com/blog/2017/01/26/setting-expectations-for-rusts-difficulty/).
* [Initial Rust support has landed in Apache Thrift](https://www.reddit.com/r/rust/comments/5qbhat/announcement_initial_rust_support_for_thrift/).
* [Serde 0.9.0](https://github.com/serde-rs/serde/releases/tag/v0.9.0) and [serde_json 0.9.0](https://github.com/serde-rs/json/releases/tag/v0.9.0) released with redesigned error-reporting API, better no_std support, a `json!` macro, and some important breaking changes.

## Other Weeklies from Rust Community

* [This week in Rust docs 41](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-41).
* [This week in Servo 90](https://blog.servo.org/2017/01/30/twis-90/).
* [video] [Ferris makes Emulators 18](https://www.youtube.com/watch?v=CsQ9WOiHcUQ): Sweep and Mod.

# Crate of the Week

This week's crate of the week is [tantivy](https://crates.io/crates/tantivy), a full text search engine, akin to Lucene. Thanks to [Jos van den Oever](https://users.rust-lang.org/users/vandenoever) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [kafka-rust: Parallel communication to brokers](https://github.com/spicavigo/kafka-rust/issues/63).
* [kafka-rust: Integration testing](https://github.com/spicavigo/kafka-rust/issues/138).
* [easy] [clippy: Lint for redundant cast](https://github.com/Manishearth/rust-clippy/issues/1497).
* [easy] [clippy: Exclude self-by-value trait methods implemented on Box<T> from boxed_local](https://github.com/Manishearth/rust-clippy/issues/1478).
* [easy] [clippy: Writing out a `&Box<T>` type](https://github.com/Manishearth/rust-clippy/issues/1480).
* [easy] [clippy: Lint on method/struct fields sharing the same name](https://github.com/Manishearth/rust-clippy/issues/1481).
* [easy] [Diesel: Refactorings using macros in type position](https://github.com/diesel-rs/diesel/issues/521).
* [android-rs-glue: Add more arguments and use clap to parse the arguments](https://github.com/tomaka/android-rs-glue/issues/115).
* [tokei: Add package repositories](https://github.com/Aaronepower/tokei/issues/92).

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

* Bryant Mairs
* Caleb Reach
* Collin J. Sutton
* Denis Andrejew
* Ömer Sinan Ağacan
* Raphael Das Gupta
* Segev Finer
* Tatsuyuki Ishi
* Zack Weinberg

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1584: Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Rust bookshelf](https://github.com/rust-lang/rfcs/pull/1828). Create a "Rust Bookshelf" of learning resources for Rust.
* [Variant types and untagged enums](https://github.com/rust-lang/rfcs/pull/1450).
* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).

## Closed RFCs

Following proposals were rejected by [the team](https://www.rust-lang.org/team.html) after their 'final comment period' elapsed.

*No RFCs were closed this week!*

## New RFCs

* [Fix the handling of uninhabited types in pattern matching](https://github.com/rust-lang/rfcs/pull/1872).
* [Add `SafeDeref` and `SafeDerefMut`, equivalent to `Deref` and `DerefMut` but which are guaranteed to always return the same object](https://github.com/rust-lang/rfcs/pull/1873).
* [Allow `extern crate` to take a list of crates](https://github.com/rust-lang/rfcs/pull/1875).

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

* [2/1. Rust User Group Cologne - Macros 1.1](http://rust.cologne/2017/02/01/proc-macros.html).
* [2/1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/2. TupperRust - Lyon, France](http://www.agendadulibre.org/events/12769).
* [2/2. Stockholm Google Developer Group - Rust Talk](https://www.meetup.com/Stockholm-Google-Developer-Group/events/236959999/).
* [2/4 - 2/5: FOSDEM 2017 Belgium - Meeting for Rustaceans](https://fosdem.org/2017/schedule/event/rust_bof/).
* [2/8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/9. Rust Bay Area: Security and Crypto with RustTLS and Macaroons](https://www.meetup.com/Rust-Bay-Area/events/237115024/).
* [2/9. Rust Boulder/Denver - Redox OS with Denver Open Source OS](https://www.meetup.com/Rust-Boulder-Denver/events/237016107/).
* [2/9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/236907254/).
* [2/9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [2/13. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/236811856/).
* [2/15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/16. Rust DC Learn + Try: clap.rs](https://www.meetup.com/RustDC/events/236719329/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Postdoc positions for RustBelt project](http://lists.seas.upenn.edu/pipermail/types-announce/2017/006485.html) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Training opportunities

Three day Rust course at [LinuxHotel](http://www.linuxhotel.de/kurs/rust/). (German)

# Quote of the Week

> Clippy is for those of you who have become desensitized to the constant whining of the Rust compiler and need a higher dosage of whininess to be kept on their toes. Clippy is for those perfectionists amongst you who want to know every minute thing wrong with their code so that they can fix it. But really, Clippy is for everyone.

— [Manishearth in a blog post](https://manishearth.github.io/blog/2017/01/21/mitigating-underhandedness-clippy/).

Thanks to [Johan Sigfrids for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/348).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
