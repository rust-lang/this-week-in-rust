Title: This Week in Rust 166
Number: 166
Date: 2017-01-24
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

Since there were no nominations, this week has to go without a Crate of the Week. Sorry.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust::from(lang)](https://github.com/mgattozzi/rust-from-lang) is a project to help people transition from other languages to Rust with articles that show how to do something in one language and then how to do it in Rust and comparing the two. You can help by writing examples or request for articles on problems you need help with.
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

139 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-16..2017-01-23

* [1.15 release notes](https://github.com/rust-lang/rust/pull/38966)
* [implement placement-in on `BinaryHeap`](https://github.com/rust-lang/rust/pull/39062)
* [implement `#[proc_macro_attribute]`](https://github.com/rust-lang/rust/pull/38842) (RFC [#1566](https://github.com/nrc/rfcs/blob/proc-macros/text/0000-proc-macros.md))
* [`&Void`'s uninhabitedness now feature gated](https://github.com/rust-lang/rust/pull/39151)
* [fix ICE on `fn f<T: ?for<'a> Sized>() {}`](https://github.com/rust-lang/rust/pull/39138)
* [warn, not ICE on unreachable patterns](https://github.com/rust-lang/rust/pull/39127)
* [refactor parser to consume token trees](https://github.com/rust-lang/rust/pull/39118)
* [merge `ObjectSum` and `PolyTraitRef` in AST/HIR](https://github.com/rust-lang/rust/pull/39110)
* [rename `ExprKind::Vec` to `Array` in HIR/HAIR](https://github.com/rust-lang/rust/pull/39090)
* [incremental compilation cleans up more garbage files](https://github.com/rust-lang/rust/pull/39109)
* [fix UB in test helpers](https://github.com/rust-lang/rust/pull/39095)
* [fix `u128`/`i128` bugs on big endian systems](https://github.com/rust-lang/rust/pull/39094)
* [deprecate `#[unsafe_destructor_blind_to_params]`](https://github.com/rust-lang/rust/pull/38970)
* [highlight code in diagnostics](https://github.com/rust-lang/rust/pull/38955)
* [fix jemalloc for OS X 10.2](https://github.com/rust-lang/jemalloc/pull/16) [and introduce it in Rust](https://github.com/rust-lang/rust/pull/39166)
* [fix linker failure on windows](https://github.com/rust-lang/rust/pull/38949)
* [`Duration` now implements `Sum`](https://github.com/rust-lang/rust/pull/38712)
* [`rand` types now implement `Debug`](https://github.com/rust-lang/rust/pull/39156)
* [`IpAddr`s are now comparable to `Ipv`{4, 6}`Addr`s](https://github.com/rust-lang/rust/pull/38464)
* [epic slice iteration search speedups](https://github.com/rust-lang/rust/pull/37972)
* [compile rmeta crates faster](https://github.com/rust-lang/rust/pull/39184)
* [fix regression in parsing trait object types](https://github.com/rust-lang/rust/pull/39179)
* [remove unused ABIs (`Os`/`Architecture`) from libsyntax](https://github.com/rust-lang/rust/pull/39218)
* [extra bounds in trait impls are now denied](https://github.com/rust-lang/rust/pull/39195)
* [lint attributes now work below item level](https://github.com/rust-lang/rust/pull/38806)
* [better unused `extern crate` and `#[macro_use]` warnings](https://github.com/rust-lang/rust/pull/39060)
* [building cargo is now reproducible](https://github.com/rust-lang/cargo/pull/3554)
* [examples can now be libraries](https://github.com/rust-lang/cargo/pull/3556)
* [procedural macro crates can now be doctested](https://github.com/rust-lang/cargo/pull/3552)
* [`cargo new` no longer allows numerical named crates](https://github.com/rust-lang/cargo/pull/3542)
* [`cargo publish` now uploads CI badge information](https://github.com/rust-lang/cargo/pull/3546) so [crates.io can show the badges](https://github.com/rust-lang/crates.io/pull/504)
* [`cargo publish` now uploads](https://github.com/rust-lang/cargo/pull/3301) the [categories on crates.io](https://github.com/rust-lang/crates.io/pull/488) (Warning: Huge bikeshedding)

And my favorite PR title: ["travis: Move glibc backwards in time"](https://github.com/rust-lang/rust/pull/39198)

## New Contributors

* Behnam Esfahbod
* Benjamin Saunders
* Ben Wiederhake
* Bjorn Tipling
* Christopher Armstrong
* Craig Macomber
* Djzin
* Jeff Waugh
* Tyler Julian

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).
* [Const-dependent type system (also known as, Π-types and value-types)](https://github.com/rust-lang/rfcs/pull/1657).
* [Extend `Cell` to work with non-`Copy` types](https://github.com/rust-lang/rfcs/pull/1651).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Add syntax for expressing tuples as a head and tail pair, similar to a Lisp cons cell](https://github.com/rust-lang/rfcs/pull/1582).
* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).

## Closed RFCs

Following proposals were rejected by [the team](https://www.rust-lang.org/team.html) after their 'final comment period' elapsed.

* [Abort by default v2](https://github.com/rust-lang/rfcs/pull/1765). Specify abort-by-default in `Cargo.toml` when the user does `cargo new --bin`, as well as various other refinements to the panick strategy system.

## New RFCs

*No new RFCs were proposed this week.*

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

* [1/18. Rust Cologne: Ruby meets Rust](https://www.meetup.com/RustCologne/events/235877954/).
* [1/18. Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/236735645/).
* [1/18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/18. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/19. Rust Paris: Rust meetup #35](https://www.meetup.com/Rust-Paris/events/236727277/).
* [1/19. GPU enhanced terminals, Counting Votes, and Converting C to Rust](https://www.meetup.com/Rust-Bay-Area/events/236668916/).
* [1/20. Rust Rhein-Main: Rust Table of Regulars Darmstadt](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/236456912/?eventId=236456912).
* [1/24. Mozilla Meetup Switzerland: Rust January Meetup @ Coredump.ch](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/236277734/?eventId=236277734).
* [1/25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/25. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/25. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658932/).
* [1/26. Rust Stockholm: REST in Rust and Rust Hack Night](https://www.meetup.com/ruststhlm/events/236791788/).
* [1/26. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [1/28. Rust MX - Rust Meetup in Mexico City](https://www.meetup.com/Rust-MX/events/236642131/).
* [2/1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/2. Stockholm Google Developer Group - Rust Talk](https://www.meetup.com/Stockholm-Google-Developer-Group/events/236959999/).
* [2/4 - 2/5: FOSDEM 2017 Belgium - Meeting for Rustaceans](https://fosdem.org/2017/schedule/event/rust_bof/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior backend developer at OneSignal](https://angel.co/onesignal/jobs/128684-senior-backend-developer).
* [Rust backend developer at 1aim.com](https://news.ycombinator.com/item?id=13302210).
* [Rust systems programmer at Hadean](https://news.ycombinator.com/item?id=13301893).
* [Embedded software engineer at ATS](http://advancedtelematic.com/en/careers/embedded-software-engineer.html)
* [Rust engineer at MaidSafe](https://maidsafe.net/careers.html#rust_engineer)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Training opportunities

Three day Rust course at [LinuxHotel](http://www.linuxhotel.de/kurs/rust/). (German)

# Quote of the Week

> I really hate the phrase "fighting". Calling it a fight doesn't do justice to the conversations you have with the borrow checker when you use Rust every day. You don't fight with the borrow checker, because there isn't a fight to win. It's far more elegant, more precise. It's fencing; you fence with the borrow checker, with ripostes and parries and well-aimed thrusts. And sometimes, you get to the end and you realize you lose anyway because the thing you were trying to do was fundamentally wrong. And it's okay, because it's just fencing, and you're a little wiser, a little better-honed, a little more practiced for your next bout.

— [kaosjester on Hacker News](https://news.ycombinator.com/item?id=13413021).

Thanks to [Manishearth](https://users.rust-lang.org/users/manishearth) for the [suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/346).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
