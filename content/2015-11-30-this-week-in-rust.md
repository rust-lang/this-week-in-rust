Title: This Week in Rust 107
Number: 107
Date: 2015-11-30
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [nasa42](https://github.com/nasa42), [brson](https://github.com/brson), and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [Announcing Diesel — A safe, extensible ORM and query builder for Rust](https://medium.com/@sgrif/announcing-diesel-a-safe-extensible-orm-and-query-builder-for-rust-fdf57966a16c).
* [Where are you From::from, And what have you turned Into::into](https://llogiq.github.io/2015/11/27/from-into.html)? When and for what to use `From` and `Into` traits.
* [Experimenting with Rust](https://www.polidea.com/blog/Experimenting_with_Rust/).
* [Language of the month: Rust](https://gergely.imreh.net/blog/2015/11/language-of-the-month-rust/).
* [Macro hygiene in all its guises and variations](http://www.ncameron.org/blog/macro-hygiene-in-all-its-guises-and-variations/).
* [Macro plans, overview](http://www.ncameron.org/blog/macro-plans-overview/). An overview of the changes that Nick plans to propose for the macro system.
* [Parser Combinators: The road to Chomp 0.1](https://m4rw3r.github.io/parser-combinators-road-chomp-0-1/).

## Notable New Crates & Projects

* [Diesel](https://github.com/sgrif/diesel). A safe, extensible ORM and Query Builder for Rust.
* [Chomp](https://github.com/m4rw3r/chomp). Fast parser combinator library for Rust.
* [libkeccak-tiny](https://github.com/debris/tiny-keccak). A tiny implementation of SHA-3, SHAKE, Keccak, and sha3sum in Rust.
* [Waitout](https://github.com/softprops/waitout). Simple interface for tracking and awaiting the completion of multiple asynchounous tasks.

# Updates from Rust Core

69 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-11-23..2015-11-30

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-wed-nov-25-2015/2940
[subteam]: https://github.com/rust-lang/subteams/blob/master/tools/reports/2015-11-23.md

## Notable changes

* [Make unreachable_code lint warn on diverging call arguments as well](https://github.com/rust-lang/rust/pull/30000).
* [Rename `#[deprecated]` to `#[rustc_deprecated]`](https://github.com/rust-lang/rust/pull/29952).
* [Add rustc UX guidelines](https://github.com/rust-lang/rust/pull/29687).
* [Fix various bugs around empty structs and patterns](https://github.com/rust-lang/rust/pull/29383).
* [Update bash-completion](https://github.com/rust-lang/cargo/pull/2176).
* [Add suggestion of similar macro names to `macro undefined` error message](https://github.com/rust-lang/rust/pull/30064).
* [Split `rustc::metadata` to a rustc_metadata crate](https://github.com/rust-lang/rust/pull/30043).
* [Remove `#[staged_api]`](https://github.com/rust-lang/rust/pull/30015).

## New Contributors

* androm3da
* ebadf
* Ivan Stankovic
* Jack Fransham
* Jeffrey Seyfried
* Josh Austin
* Kevin Yeh
* Matthias Bussonnier
* Philipp Matthias Schäfer
* xd1le

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1252: Document and expand the open options](https://github.com/rust-lang/rfcs/pull/1252).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Allow eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196).
* [Add thread-local custom panic handlers to customize the behavior of thread panics](https://github.com/rust-lang/rfcs/pull/1100).
* [Allow a custom panic handler](https://github.com/rust-lang/rfcs/pull/1328).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Change thread local variables to only accept async-signal-safe types](https://github.com/rust-lang/rfcs/pull/1379).
* [Amend RFC 550 with (expanded) abstract specification rather than algorithm](https://github.com/rust-lang/rfcs/pull/1384).

# Upcoming Events

* [12/1. Rust Sydney Meetup](http://www.meetup.com/Rust-Sydney/events/226832397/).
* [12/2. Boston: Concurrency in Rust Tutorial](http://www.meetup.com/Boston-Rust-Meetup-25317522aNpHwZdw/events/226759437/).
* [12/8. San Diego Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/226694618/).
* [12/9. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/).
* [12/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [Open Source Software Engineer](http://maidsafe.net/careers) at MaidSafe.
* [Software Engineer - Sensor Development](https://gethired.com/apply/ac642822-b445-4fee-9d44-65827f0381f5) at IronNet Cybersecurity.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [Chrono](https://github.com/lifthrasiir/rust-chrono), a crate that offers very handy timezone-aware `Duration` and `Date`/`Time` types.

Thanks to [Ygg01](https://users.rust-lang.org/users/Ygg01) for the suggestion. [Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
