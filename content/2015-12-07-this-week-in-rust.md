Title: This Week in Rust 108
Number: 108
Date: 2015-12-07
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

* [Rustaceans: Please keep a changelog](https://blog.dbrgn.ch/2015/12/1/rust-crates-keep-a-changelog/)!
* [Building a simple JIT in Rust](http://www.jonathanturner.org/2015/12/building-a-simple-jit-in-rust.html).
* [Build an API in Rust with JWT authentication using Nickel.rs](https://auth0.com/blog/2015/11/30/build-an-api-in-rust-with-jwt-authentication-using-nickelrs/).
* [Using Wayland from Rust, part 1](http://blog.levans.fr/rust_wayland_1-en.html).
* [Language of the month: Rust, the results](https://gergely.imreh.net/blog/2015/12/lotm-rust-results/).
* [embed_lang - An embeddable language written in Rust](https://marwes.github.io/2015/12/06/embed_lang-An-embeddable-language-written-in-Rust.html).
* [Implementing a SuperCollider UGen in Rust](http://www.andrewchristophersmith.com/blog/implementing_a_supercollider_external_in_rust/).
* [This week in Redox OS 7](http://www.redox-os.org/news/this-week-in-redox-7/).
* [This week in Servo 43](http://blog.servo.org/2015/11/30/twis-43/).

## Notable New Crates & Project Updates

* [MIO released v0.5 which includes support for Windows, NetBSD, and Android](https://github.com/carllerche/mio/blob/v0.5.x/CHANGELOG.md).
* [Rust by Example](http://rustbyexample.com/) now has a new [error handling section](http://rustbyexample.com/error.html) via this [PR](https://github.com/rust-lang/rust-by-example/pull/674).
* [notty](https://github.com/withoutboats/notty). A new kind of terminal.
* [Collenchyma](https://github.com/autumnai/collenchyma). High Performance Computation for CUDA, OpenCL and common CPU.
* [gaol](https://github.com/pcwalton/gaol). Cross-platform application sandboxing for Rust.
* [RobotS](https://github.com/gamazeps/RobotS). Actor system for Rust, inspired by Akka & Erlang.
* [kaws](https://github.com/InQuicker/kaws). A tool for deploying multiple Kubernetes clusters in AWS using CoreOS, GPG, and Terraform.
* [Kinglet](https://github.com/pyfisch/kinglet). A modern asynchronous HTTP server for Rust.
* [Rugra](https://github.com/Ticki/rugra). An ultra minimal graphics engine in Rust.
* [inlinable_string](https://github.com/fitzgen/inlinable_string). An owned, grow-able UTF-8 string that stores small strings inline and avoids heap-allocation.

# Updates from Rust Core

92 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-11-30..2015-12-07

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-mon-dec-07-2015/2970
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-12-04/2963

## Notable changes

* [Implement calling of `const fn`-methods in true constants](https://github.com/rust-lang/rust/pull/30084).
* [Add suggestion of similar macro names to `macro undefined` error message](https://github.com/rust-lang/rust/pull/30064).
* [Fix various bugs around empty structs and patterns](https://github.com/rust-lang/rust/pull/29383).
* [Allow constant evaluation of index operations on constant arrays and repeat expressions](https://github.com/rust-lang/rust/pull/25570).
* [Implement RFC 16, attributes on statements and expressions](https://github.com/rust-lang/rust/pull/29850).
* [std: Stabilize APIs for the 1.6 release](https://github.com/rust-lang/rust/pull/30187).
* [Cargo: Allow build scripts to specify dependencies](https://github.com/rust-lang/cargo/pull/2134).

## New Contributors

* Adam Badawy
* Bhargav Patel
* Christopher Sumnicht
* Mihaly Barasz
* Mika Attila
* Ori Avtalion
* Paul A. Jungwirth
* Sean Griffin

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

* [Allocators, take III](https://github.com/rust-lang/rfcs/pull/1398).
* [Add `#[repr(pack = "N")]`](https://github.com/rust-lang/rfcs/pull/1399).
* [Add `try_some` macro equivalent to `try!`](https://github.com/rust-lang/rfcs/pull/1394).
* [Expand `try!` macro with additional case](https://github.com/rust-lang/rfcs/pull/1393).
* [Add a `pod` language item and marker trait](https://github.com/rust-lang/rfcs/pull/1387).

# Upcoming Events

* [12/8. San Diego Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/226694618/).
* [12/9. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/).
* [12/10. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [12/11. Rhein-Main Area Rust Meetup](http://www.meetup.com/de/Rust-Rhein-Main/events/226858571/).
* [12/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [12/15. Rust - Rethinking Systems Programming](http://www.meetup.com/de/NewStore/events/225945950/).
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

This week's Crate of the Week is [cargo-count](https://github.com/kbknapp/cargo-count) – a neat way to summarize line counts for cargo projects.

Thanks to [lizida](https://users.rust-lang.org/users/lizida) who suggested it back in September. [Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> The major philosophic difference between Rust today and Swift-as-I-envision-it is that Rust forces you to think about ownership everywhere, but Swift-as-I-envision-it should only force you to think about single ownership & borrowing if you want to optimize performance or guarantee that you have no encounters with the runtime.

> If it helps, think of the extant Swift "inout" parameter modifier as being equivalent to "&mut", and imagine the logical swift extensions to support the rest of the Rust model.

> This is a really important area for us to develop, but it also isn't the highest priority of the team. That means that Rust will maintain a lead in this area of applicability... unless someone motivated and capable from the open source community decides that it is really important to them, and makes it happen sooner.

— Chris Lattner on [/r/rust](https://www.reddit.com/r/rust/comments/3vadg8/swift_is_open_source/cxnu2kk).

Thanks to [llogiq](https://github.com/llogiq) for the tip.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
