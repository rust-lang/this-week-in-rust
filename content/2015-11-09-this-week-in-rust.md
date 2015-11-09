Title: This Week in Rust 104
Number: 104
Date: 2015-11-09
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

* [New version of kernel32-sys breaks multiple crates](https://github.com/retep998/winapi-rs/issues/238).
* [Bare metal Rust: Low-level CPU I/O ports](http://www.randomhacks.net/2015/11/09/bare-metal-rust-cpu-port-io/).
* [Macros in Rust - Part 1](http://ncameron.org/blog/macros-in-rust-pt1/).
* [Macros in Rust - Part 2](http://www.ncameron.org/blog/macros-in-rust-pt2/). Procedural macros.
* [Macros in Rust - Part 3](http://www.ncameron.org/blog/macros-in-rust-pt3/). Macro hygiene in Rust.
* [Using rustfmt in Vim](http://johannh.me/blog/rustfmt-vim.html).
* [Writing my first Rust crate: jsonwebtoken](https://blog.wearewizards.io/writing-my-first-rust-crate-jsonwebtoken).
* [Learning to 'try!' things in Rust](http://www.jonathanturner.org/2015/11/learning-to-try-things-in-rust.html).
* [video] [Concurrency in Rust](https://vimeo.com/144809407).
* [This week in Redox 5](http://www.redox-os.org/news/this-week-in-redox-5/).
* [This week in Servo 40](http://blog.servo.org/2015/11/02/twis-40/).

## Notable New Crates & Projects

* [Organn](https://github.com/monsieursquirrel/organn). A simple drawbar organ in Rust.
* [libloading](https://github.com/nagisa/rust_libloading). A safer binding to platform’s dynamic library loading utilities.

# Updates from Rust Core

104 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-11-02..2015-11-09

See the [triage digest][triage] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-tue-nov-03-2015/2865

## Notable changes

* [Library FCP Issues for the 1.6 cycle](https://internals.rust-lang.org/t/library-fcp-issues-for-the-1-6-cycle/2872).
* [Deprecate `_ms` functions that predate the `Duration` API](https://github.com/rust-lang/rust/pull/29604).
* [Implement `IntoIterator` for `&{Path, PathBuf}`](https://github.com/rust-lang/rust/pull/29514).
* [Use guard-pages also on DragonFly/FreeBSD](https://github.com/rust-lang/rust/pull/29510).
* [Make Windows directory layout uniform with everything else](https://github.com/rust-lang/rust/pull/29500).
* [Expose `drop_in_place` as `ptr::drop_in_place`](https://github.com/rust-lang/rust/pull/29475).
* [Make all integer intrinsics generic](https://github.com/rust-lang/rust/pull/29316).
* [Check whether a supertrait contains Self even without being it](https://github.com/rust-lang/rust/pull/29259).
* [Improve error handling in libsyntax](https://github.com/rust-lang/rust/pull/29285).

## New Contributors

* Amanieu d'Antras
* Amit Saha
* Bruno Tavares
* Daniel Trebbien
* Ivan Kozik
* Jake Worth
* jrburke
* Kyle Mayes
* Oliver Middleton
* Rizky Luthfianto

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1298: Enable the compiler to cache incremental workproducts](https://github.com/rust-lang/rfcs/pull/1298).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [`#[deprecated]` for Everyone](https://github.com/rust-lang/rfcs/pull/1270).
* [Improvements to the Time APIs](https://github.com/rust-lang/rfcs/pull/1288).
* [Define the general semantics of intrinsic functions](https://github.com/rust-lang/rfcs/pull/1300).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Add `retain_mut` to `Vec` and `VecDeque`](https://github.com/rust-lang/rfcs/pull/1353).
* [Add `#[cfg(...)]` attribute to enable conditional compilation dependent on size and alignment of FFI types](https://github.com/rust-lang/rfcs/pull/1354).

# Upcoming Events

* [11/9. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [11/10. San Diego Rust Meetup #10](http://www.meetup.com/San-Diego-Rust/events/226048826/).
* [11/11. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/).
* [11/13. Rust Rhein-Main](http://www.meetup.com/Rust-Rhein-Main/events/225850710/).
* [11/14. Rust at the Hungarian Web Conference](http://webconf.hu/2015/program/index.php#i2015_03).
* [11/16. Rust Paris](http://www.meetup.com/Rust-Paris).
* [11/17. Rust Hack and Learn Hamburg](http://www.meetup.com/Rust-Meetup-Hamburg/events/226298232/?a=ea1_grp&rv=ea1&_af=event&_af_eid=226298232&https=off).
* [11/18. Rust Los Angeles Monthly Meetup](http://www.meetup.com/Rust-Los-Angeles/events/226074704/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [ramp](https://crates.io/crates/ramp). Ramp supplies some high performance low memory easy to use big integral types.

Whenever you need integers too large for a `u64` and cannot afford to lose precision, ramp has just what you need.

Thanks to [zcdziura](https://users.rust-lang.org/users/zcdziura) for this week's suggestion. [Submit your suggestions for next week][submit_crate]!

# Quote of the Week

> with **unsafe**
> .... **if** you have to ask, then you probably shouldn't **be** doing it
> basically

— Steve Klabnik on [#rust IRC](https://botbot.me/mozilla/rust/msg/53505463/).

Thanks to [Oliver Schneider](https://users.rust-lang.org/users/oli_obk) for the tip.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
