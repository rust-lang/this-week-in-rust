Title: This Week in Rust 302
Number: 302
Date: 2019-09-03
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

* [Introduction to Rust Web Applications](https://erwabook.com/intro/)
[Low Power NB-IoT on STM32 Blue Pill with Apache Mynewt and Embedded Rust](https://medium.com/@ly.lee/low-power-nb-iot-on-stm32-blue-pill-with-apache-mynewt-and-embedded-rust-cef5a3ecdd90).
- [Semantic validation in Rust](https://slowtec.de/posts/2019-09-03-semantic-validation-with-rust.html)
* [This Month in Rust GameDev #1 - August 2019](https://rust-gamedev.github.io/2019/09/02/newsletter-001.html)

# Crate of the Week

This week's crate is [cargo-udeps](https://crates.io/crates/cargo-udeps), a cargo subcommand to find unused dependencies.

Thanks to [Christopher Durham](https://users.rust-lang.org/t/crate-of-the-week/2704/613) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

214 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-08-26..2019-09-02

* [rustc: Handle modules in "fat" LTO more robustly](https://github.com/rust-lang/rust/pull/63956)
* [Add default serialization for `Ident`s](https://github.com/rust-lang/rust/pull/63853)
* [Correctly suggest adding bounds to `impl Trait` argument](https://github.com/rust-lang/rust/pull/63811)
* [Strip code to the left and right in diagnostics for long lines](https://github.com/rust-lang/rust/pull/63402)
* [Do not complain about unused code when used in `impl` `Self` type](https://github.com/rust-lang/rust/pull/63317)
* [Simplify eager normalization of constants](https://github.com/rust-lang/rust/pull/63820)
* [miri: Stacked Borrows: Don't read from memory during retagging](https://github.com/rust-lang/miri/pull/931)
* [miri: detect too large dynamically sized objects](https://github.com/rust-lang/rust/pull/64014)
* [Small improvement for `Ord` implementation of integers](https://github.com/rust-lang/rust/pull/63992)
* [Improve Rustdoc's handling of procedural macros](https://github.com/rust-lang/rust/pull/62855)
* [crates.io: Show right-hand column for yanked versions to crate owners](https://github.com/rust-lang/crates.io/pull/1759)

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

* [disposition: merge] [hint::bench_black_box](https://github.com/rust-lang/rfcs/pull/2360).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Constify LinkedList new function](https://github.com/rust-lang/rust/pull/63684).
* [disposition: merge] [Test that Wrapping arithmetic ops are implemented for all int types](https://github.com/rust-lang/rust/pull/63692).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Africa

* [Sep  4. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzmbgb/).

### Asia Pacific

* [Sep  2. Auckland, NZ - Rust AKL - Introduction to Rust (session 1 of 3)](https://www.meetup.com/rust-akl/events/259481026/).
* [Sep 11. Selangor, MY - Rust Malaysia Meetup September 2019](https://docs.google.com/forms/d/e/1FAIpQLScsqK0kH3o6ti12AEc9Fn4To-W0rXo9Q-frLmZ3JZUWc8yjjw/viewform).

### Europe

* [Sep  4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzmbgb/).
* [Sep  5. Hamburg, DE - Rust Hack & Learn September 2019](https://www.meetup.com/Rust-Meetup-Hamburg/events/264102479/).

### North America

* [Sep  4. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzmbgb/).
* [Sep  4. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzmbgb/).
* [Sep  5. Seattle, WA, US - Seattle Rust Meetup - Physical Computing Workshop](https://www.meetup.com/Seattle-Rust-Meetup/events/264245990/).
* [Sep 10. Detroit, MI, US - Detroit Rust - September Detroit Rust at the Altimetrik Collider](https://www.meetup.com/detroitrust/events/264251923/).
* [Sep 12. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzmbqb/).
* [Sep 12. San Diego, CA, US - San Diego Rust September Meetup](https://www.meetup.com/San-Diego-Rust/events/264062555/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer - Rust at IOHK, Remote](https://iohk.recruiterbox.com/jobs/fk03udp/).
* [Senior Software Engineer at ConsenSys R&D, Remote](https://consensys.net/open-roles/1792013/).
* [Rust/Core Developer at Parity, Berlin, DE](https://www.parity.io/jobs/#berlin-rust-core-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Threads are for working in parallel, async is for waiting in parallel.

– [ssokolow on /r/rust](https://reddit.com/r/rust/comments/cws788/is_await_only_useful_is_my_code_doesnt_do_much/eyfg4va/)

Thanks to [Philipp Oppermann](https://users.rust-lang.org/t/twir-quote-of-the-week/328/686) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
