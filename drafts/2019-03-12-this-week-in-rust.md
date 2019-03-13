Title: This Week in Rust 277
Number: 277
Date: 2019-03-12
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

This week's crate is [validator](https://github.com/Keats/validator), a crate offering simple validation for Rust structs. Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/500) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [LambdAle (FP conference): CfP is now open](https://www.papercall.io/lambdale-2019).
* [Rust: Erroneous compilation failure with associated constant](https://github.com/rust-lang/rust/issues/54822).
* [TiKV: Set compile-time env vars in build script instead of Makefile](https://github.com/tikv/tikv/issues/4051)
* [TiKV: Use breakpad + symbolic to generate and interpret minidump-format core dumps](https://github.com/tikv/tikv/issues/4202)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

173 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-03-04..2019-03-11

* [Make the rustc driver and interface demand driven](https://github.com/rust-lang/rust/pull/56732)
* [Make the lifetime parameters of tcx consistent](https://github.com/rust-lang/rust/pull/58926)
* [Don't promote function calls to nonpromotable things](https://github.com/rust-lang/rust/pull/58784)
* [HirIdification: almost there](https://github.com/rust-lang/rust/pull/58915)
* [Monomorphize generator field types for debuginfo](https://github.com/rust-lang/rust/pull/58906)
* [Always emit unclosed delimiter diagnostics](https://github.com/rust-lang/rust/pull/58903)
* [Mention `unwind(aborts)` in diagnostics for `#[unwind]`](https://github.com/rust-lang/rust/pull/58762)
* [Add const generics to ty (and transitive dependencies)](https://github.com/rust-lang/rust/pull/58583)
* [Make the Entry API of HashMap<K, V> Sync and Send](https://github.com/rust-lang/rust/pull/58369)
* [On return type `impl Trait` for block with no expr point at last semi](https://github.com/rust-lang/rust/pull/58204)
* [Create a derive macro for HashStable and allow proc macros in rustc](https://github.com/rust-lang/rust/pull/58013)
* [Move index updates off the web server](https://github.com/rust-lang/crates.io/pull/1588)
* [Make `Unique::as_ptr`, `NonNull::dangling` and `NonNull::cast` const](https://github.com/rust-lang/rust/pull/58750)
* [MaybeUninit: add read_initialized, add examples](https://github.com/rust-lang/rust/pull/58660)
* [Add `as_slice()` to `slice::IterMut` and `vec::Drain`](https://github.com/rust-lang/rust/pull/58924)
* [cargo: Emit warning on misspelled environment variables](https://github.com/rust-lang/cargo/pull/6694)
* [rustdoc: add option to calculate "documentation coverage"](https://github.com/rust-lang/rust/pull/58626)

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

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add FromStr impl for NonZero types](https://github.com/rust-lang/rust/pull/58717).
* [disposition: merge] [Cosmetic changes to compiler comments and docs](https://github.com/rust-lang/rust/issues/58619).

## New RFCs

* [Initial pipeline rfc](https://github.com/rust-lang/rfcs/pull/2656).
* [Add Destructuring assignment](https://github.com/rust-lang/rfcs/pull/2649).

# Upcoming Events

### Online

* [Mar 13. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 20. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Asia Pacific

* [Mar 15. Noida, IN - Lifetime in Rust-Knoldus](https://www.meetup.com/Reactive-Application-Programmers-in-Delhi-NCR/events/259354521/).
* [Mar 20. Tokyo, JP - Rust Game-bot Challenge](https://connpass.com/event/122171/).
* [Mar 25. Auckland, NZ - Rust AKL Meetup](https://www.meetup.com/rust-akl/events/259480499/).

### Europe

* [Mar 11. Stockholm, SE - Rust Meetup Stockholm First one for 2019](https://www.meetup.com/ruststhlm/events/259387426/).
* [Mar 14. Brno, CZ - Rust Brno Meetup at Masaryk University](https://rust-brno.github.io/)
* [Mar 14. Göteborg, SE - Rust Gothenburg](https://www.meetup.com/rustgbg/events/259386306/).
* [Mar 19. Nijmegen, NL - Rust Nijmegen: Rust for the (Inter)Net - API's, HTTP/3 and Tide](https://www.meetup.com/Rust-Nijmegen/events/258758167).
* [Mar 19. Paris, FR - Paris - Rust Paris](http://www.meetup.com/Rust-Paris).
* [Mar 20. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/find/events/?allMeetups=false&keywords=Rust+Hack+and+Learn+OpenTechSchool&radius=25&userFreeform=Berlin%2C+Germany&mcName=Berlin%2C+DE&eventFilter=all).

### North America

* [Mar  9. 200 University Ave W, Waterloo, CN - Workshop: Introduction to Game Development in Rust](https://www.meetup.com/Rust-KW/events/259335419/).
* [Mar 11. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/nzfspqyzfbpb/).
* [Mar 13. Ciudad de México, MX - Study group RustMX](https://www.meetup.com/Rust-MX/events/259473143/).
* [Mar 14. Columbus, US - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dbcfrpyzfbsb/).
* [Mar 14. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/258703993/).
* [Mar 20. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/gqbksqyzfbbc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Tech Ambassador at Parity, Berlin, DE](https://www.parity.io/jobs/#berlin-tech-ambassador).
* [Rust Software Consultant at Knoldus, Noida, IN](https://www.knoldus.com/careers/rust-software-consultant.knol).
* [Summer Internship at Brave Software, San Francisco, US](https://www.reddit.com/r/rust/comments/av50om/rustrelated_summer_internship_at_brave_software/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> – Rust’s ownership is difficult.
> – Yes, ownership is difficult. For such a difficult thing, you (are going to) ever check by hand rather than having machine do it for you.

– @Cryolite [on twitter](https://twitter.com/Cryolite/status/1104325100881375232) (translated from Japanese)

Thanks to [Xidorn Quan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/629) for the suggestions!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
