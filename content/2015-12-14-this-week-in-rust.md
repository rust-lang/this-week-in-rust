Title: This Week in Rust 109
Number: 109
Date: 2015-12-14
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

* 🎈🎉 [Announcing Rust 1.5](http://blog.rust-lang.org/2015/12/10/Rust-1.5.html). 🎉🎈
* [Building an iOS app in Rust, Part 3](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-3/). Passing owned objects between Rust and iOS.
* [Writing an OS in Rust: Accessing and modifying page tables](http://os.phil-opp.com/modifying-page-tables.html). Part of the series [Writing an OS in Rust](http://os.phil-opp.com/).
* [Rust programming: Creating a Phoronix reader application](https://www.gitbook.com/book/mmstick/rust-programming-phoronix-reader-how-to/details)
* [Macro plans - syntax](http://www.ncameron.org/blog/macro-plans-syntax/). Proposal to improve the syntax for Rust macros.
* [Zero-runtime-cost mixed list in Rust](https://nercury.github.io/rust/interesting/2015/12/12/typed-arrays.html).
* [ArcadeRS 1.10: Asteroid attack!](https://jadpole.github.io/arcaders/arcaders-1-10/). Part of the series [ArcadeRS 1.0: The project](https://jadpole.github.io/arcaders/arcaders-1-0/) - a series whose objective is to explore the Rust programming language and ecosystem through the development of a simple, old-school shooter.
* [Rust profiling with instruments and FlameGraph on OSX: CPU/Time](http://carol-nichols.com/2015/12/09/rust-profiling-on-osx-cpu-time/).
* [Type-Level Shenanigans](https://llogiq.github.io/2015/12/12/types.html).
* [This Week in Redox OS 8](http://www.redox-os.org/news/this-week-in-redox-8/).

## Notable New Crates & Project Updates

* [rusty-cheddar](https://github.com/Sean1708/rusty-cheddar). A Rust plugin to automatically generate C-style header files.
* [Iomrascálaí](https://github.com/ujh/iomrascalai). An AI for the game of Go/Weiqi/Baduk written in Rust.

# Updates from Rust Core

78 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-11-30..2015-12-07

## Notable changes

* [Store MIR in crate metadata](https://github.com/rust-lang/rust/pull/30301).
* [Cargo: Bump to 0.8.0](https://github.com/rust-lang/cargo/pull/2204).
* [std: Remove deprecated functionality from 1.5](https://github.com/rust-lang/rust/pull/30182).
* [std: Rename `thread::catch_panic` to `panic::recover`](https://github.com/rust-lang/rust/pull/29937).
* [Adjust the pointer to an unsized field to the correct alignment](https://github.com/rust-lang/rust/pull/30245).

## New Contributors

* Brian Bowman
* Daniel Campbell
* Letheed
* petevine
* Tianyi Wang
* Xmasreturns

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

* [Amend RFC 550 with (expanded) abstract specification rather than algorithm](https://github.com/rust-lang/rfcs/pull/1384).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Allow eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196).
* [Add thread-local custom panic handlers to customize the behavior of thread panics](https://github.com/rust-lang/rfcs/pull/1100).
* [Allow a custom panic handler](https://github.com/rust-lang/rfcs/pull/1328).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Provide a syntactic sugar to automatically implement a given trait `Tr` using a pre-existing type implementing `Tr`](https://github.com/rust-lang/rfcs/pull/1406).
* [Amend 0809 to reduce the number of traits](https://github.com/rust-lang/rfcs/pull/1401).

# Upcoming Events

* [12/15. Rust - Rethinking Systems Programming](http://www.meetup.com/de/NewStore/events/225945950/).
* [12/16. SSD Rust meetup - servo + TBD topics](http://www.meetup.com/SolidStateDepot/events/227170190/).
* [12/21. Paris - Rust Paris](http://www.meetup.com/Rust-Paris).

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

This week's Crate of the Week is [Quick Error](https://github.com/tailhook/quick-error) – a rust-macro which makes errors easy to write.

Thanks to [killercup](https://users.rust-lang.org/users/killercup) for the suggestion. [Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> how do cats arbitrate access to a shared typesetting system?

> with a mew-TeX

— Nathaniel Theis on [Twitter](https://twitter.com/XMPPwocky/status/653963665431891968)

Thanks to [Paul](https://twitter.com/pauldwoolcock/status/673281326414344192) for the tip.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
