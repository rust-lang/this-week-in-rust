Title: This Week in Rust 112
Number: 112
Date: 2016-01-04
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

* [This week in Servo 45](http://blog.servo.org/2015/12/28/twis-45/).
* [What is happening in Piston 2](http://blog.piston.rs/2015/12/31/what-is-happening-2/).
* [Async IO in Rust (part III)](https://medium.com/@paulcolomiets/async-io-in-rust-part-iii-cbfd10f17203).
* [Writing an OS in Rust: Remap the Kernel](http://os.phil-opp.com/remap-the-kernel.html). Part of the series [Writing an OS in Rust](http://os.phil-opp.com/).
* [Aha! Understanding lifetimes in Rust](http://codrspace.com/buntine/aha-understanding-lifetimes-in-rust/).
* [Distributed rendering with Rust and Mio](http://www.willusher.io/2016/01/02/distributed-rendering-with-rust-and-mio/).
* [Racer progress update](http://phildawes.net/blog/2015/12/29/racer-update-6/). Changes and notes around Racer 1.1.0 release.

## Notable New Crates & Project Updates

* [rusty-machine](https://github.com/AtheMathmo/rusty-machine). Machine Learning library for Rust.
* [Shiplift](https://github.com/softprops/shiplift). Rust interface for maneuvering docker containers.
* [Hyperlocal](https://github.com/softprops/hyperlocal). Hyper bindings for local unix domain sockets.

# Updates from Rust Core

63 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-12-28..2016-01-04

## Notable changes

* [Link with ld.gold by default](https://github.com/rust-lang/rust/pull/29974).
* [[MIR] Implement `as` casting (Misc cast kind)](https://github.com/rust-lang/rust/pull/30586).
 
## New Contributors

* Aaron Keen
* Chris Buchholz
* Christoffer Buchholz
* Daniel Collin
* defyrlt
* Denis Kolodin
* est31
* James Mantooth
* Luke Jones
* Natha

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week!*

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Improve Cargo target-specific dependencies](https://github.com/rust-lang/rfcs/pull/1361).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Allow eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Add a replace_slice method to Vec<T> and String while removes a range of elements, and replaces it in place with a given sequence of values](https://github.com/rust-lang/rfcs/pull/1432).
* [Implement a method, `contains()`, for `Range`, `RangeFrom`, and `RangeTo`, checking if a number is in the range](https://github.com/rust-lang/rfcs/pull/1434).
* [Add a byte escape `\e` as shorthand for 0x1B (ESC), similar to GCC's `\e`](https://github.com/rust-lang/rfcs/pull/1437).
* [Allow Drop types in statics/const functions](https://github.com/rust-lang/rfcs/pull/1440).

# Upcoming Events

* [1/11. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307).
* [1/12. Eat, Drink, Rust! San Diego Downtown Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/227308164/).
* [1/13. Copenhagen hackathon](https://cph.rs/).
* [1/13. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [1/13. Log Angeles Monthly Meetup - Happy New Year Hack Night](http://www.meetup.com/Rust-Los-Angeles/events/227438139/).
* [1/14. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [1/15. Rhein-Main Rust Meetup](http://www.meetup.com/de/Rust-Rhein-Main/events/227808685/).
* [1/18. Rust Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Technical Lead/Manager](https://ebip.co.uk/careers) at EBI Portfolios.
* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [Multiple positions](http://rust.jobboard.io/employers/6824-ironnet-cybersecurity) at IronNet Cybersecurity.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [rustfmt](https://crates.io/crates/rustfmt/), because it's nice to Rust with style.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> I think the "standard library" is really (forgive me) a brand.

[jimb on rust-internals](https://internals.rust-lang.org/t/thoughts-on-rust-stdlib-and-c-interfacing/3036/14)

Thanks to llogiq for the belated suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
