Title: This Week in Rust 111
Number: 111
Date: 2015-12-28
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

* [Neon: Node + Rust = 💖](http://calculist.org/blog/2015/12/23/neon-node-rust/). Neon is a set of APIs and tools for making it super easy to write native Node modules in Rust.
* [podcast] [New Rustacean podcast episode 08](http://www.newrustacean.com/show_notes/e008/). Generics, traits, and shared behavior in Rust.

## Notable New Crates & Project Updates

* [clang-rs](https://github.com/KyleMayes/clang-rs). An idiomatic Rust wrapper for libclang.

# Updates from Rust Core

56 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-12-21..2015-12-28

## Notable changes

* [configure: Enable `-C rpath` by default](https://github.com/rust-lang/rust/pull/30353).
* [std: Remove rust_builtin C support library](https://github.com/rust-lang/rust/pull/30175).
* [Add note when item accessed from module via `m.i` rather than `m::i`](https://github.com/rust-lang/rust/pull/30413).
* [std: Update jemalloc version](https://github.com/rust-lang/rust/pull/30434).
* [Implement RFC 1328 Custom Panic Handlers](https://github.com/rust-lang/rust/pull/30485).
 
## New Contributors

* Jeff Walden
* Kai Noda
* lnmx

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

* [Feature gate extern fn methods](https://github.com/rust-lang/rfcs/pull/1429).
* [Placement in/box refinement](https://github.com/rust-lang/rfcs/pull/1426).
* [Statically dispatched methods for trait objects with associated data](https://github.com/rust-lang/rfcs/pull/1431).

# Upcoming Events

* [1/11. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307).

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

This week's Crate of the Week is [handlebars-rust](https://github.com/sunng87/handlebars-rust), an implementation of the Handlebars templating language for Rust. It also allows custom helpers and template inheritance for real-world usage. Thanks to [@sunng](https://users.rust-lang.org/users/sunng) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> You have to think about it. You don't have to worry about it.

— SamReidHughes on [manual memory management in Rust](https://news.ycombinator.com/item?id=10711997).

Thanks to [Barosl](https://users.rust-lang.org/users/barosl) for the tip.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
