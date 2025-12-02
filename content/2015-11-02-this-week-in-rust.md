Title: This Week in Rust 103
Number: 103
Date: 2015-11-02
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

* 🎈🎉 [Announcing Rust 1.4](http://blog.rust-lang.org/2015/10/29/Rust-1.4.html). 🎉🎈
* [This week in Servo 39](http://blog.servo.org/2015/10/26/twis-39/).
* [This week in Redox 4](http://www.redox-os.org/news/this-week-in-redox-4/).
* [Rreverrse debugging](https://huonw.github.io/blog/2015/10/rreverse-debugging/). Debugging Rust applications with rr.
* [Piston-Meta](http://blog.piston.rs/2015/10/28/new-meta-language/). A DSL parsing library for human readable text documents.
* [Let's uncomplicate!](http://lez-uncomplicate.blogspot.in/2015/10/understanding-rust-build-system-log.html). Demistifying the `rustc` build process.
* [Using Resque with Rust](http://julienblanchard.com/2015/using-resque-with-rust/).
* [Rust lifetimes](http://www.charlesetc.com/rust/2015/10/29/).

## Notable New Crates & Projects

* [A Rust/Piston game developing tutorial](http://piston-tutorial.logdown.com/).
* [Rust Doom](https://github.com/cristicbz/rust-doom). A Doom Renderer written in Rust.
* [httparse](https://github.com/seanmonstar/httparse). A push parser for the HTTP 1.x protocol in Rust.
* [lettre](https://github.com/lettre/lettre). A mailer library for Rust.
* [Pebble.rs](https://github.com/andars/pebble.rs). Develop Pebble smartwatch applications in Rust.

# Updates from Rust Core

94 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-10-26..2015-11-02

See the [subteam report for 2015-10-31][subteam] for details.

[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-10-31/2854

## Notable changes

* [Port the standard crates to PNaCl/NaCl](https://github.com/rust-lang/rust/pull/29289).
* [Fix excessive memory allocation in `RawVec::reserve`](https://github.com/rust-lang/rust/pull/29454).
* [Implement conversion traits for primitive float types](https://github.com/rust-lang/rust/pull/29129).
* [Generalise associative operator parsing](https://github.com/rust-lang/rust/pull/29072).
* [Do not allow a module and tuple struct of the same name to coexist](https://github.com/rust-lang/rust/pull/26421).
* [Allow constant evaluation of function calls](https://github.com/rust-lang/rust/pull/26848).
* [Remove dependency on libgcc-dw2-1.dll from win32 executables](https://github.com/rust-lang/rust/pull/29177).
* [Remove contraction from region inference](https://github.com/rust-lang/rust/pull/29188).

## New Contributors

* angelsl
* Cameron Sun
* Charlotte Spencer
* Corentin Henry
* Daniel Rollins
* Igor Shuvalov
* Ivan Ivaschenko
* Marcell Pardavi
* pierzchalski
* Ricardo Signes

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1291: Promote the `libc` crate from the nursery](https://github.com/rust-lang/rfcs/pull/1291).
* [RFC 1307: Add some additional utility methods to `OsString` and `OsStr`](https://github.com/rust-lang/rfcs/pull/1307).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [Enable the compiler to cache incremental workproducts](https://github.com/rust-lang/rfcs/pull/1298).
* [`#[deprecated]` for Everyone](https://github.com/rust-lang/rfcs/pull/1270).
* [Improvements to the Time APIs](https://github.com/rust-lang/rfcs/pull/1288).

## New RFCs

* [Unsafe expressions](https://github.com/rust-lang/rfcs/pull/1346).

# Upcoming Events

* [11/3. São Paulo Meetup](http://www.meetup.com/Rust-Sao-Paulo-Meetup/events/225844104/).
* [11/3. Rust Hack and Learn Hamburg](http://www.meetup.com/Rust-Meetup-Hamburg/events/226298117/?a=ea1_grp&rv=ea1&_af=event&_af_eid=226298117&https=off).
* [11/4. PDXRust](http://www.meetup.com/PDXRust/events/225745776/).
* [11/9. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [11/10. San Diego Rust Meetup #10](http://www.meetup.com/San-Diego-Rust/events/226048826/).
* [11/11. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/).
* [11/16. Rust Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate & Quote of the Week

This week's Crate of the Week is [alias](https://github.com/huonw/alias) by [Huon W](https://github.com/huonw). Thanks go to Reddit user [notriddle](https://www.reddit.com/user/notriddle) for the suggestion. [Submit your suggestions for next week][submit_crate]!

> _alias_ allows _mutably aliasing values_ – which seems to actually be safe, somewhat surprisingly. Honestly, I'm a bit shocked about it myself.

— [llogiq](https://github.com/llogiq) on the [alias](https://github.com/huonw/alias) crate.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
