Title: This Week in Rust 116
Number: 116
Date: 2016-02-01
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

* [The `?` operator and `try` vs `do`](https://m4rw3r.github.io/rust-questionmark-operator/). A follow-up on [Trait-based exception handling](https://github.com/rust-lang/rfcs/pull/243) RFC.
* [Rust vs. C++: Fine-grained Performance](http://cantrip.org/rust-vs-c++.html).
* [Stateful: A Rust experimental syntax extension for generators and more](https://erickt.github.io/blog/2016/01/27/stateful-in-progress-generators/).
* [Rust and BigData](http://www.poumeyrol.fr/2016/01/25/The-Rust-is-in-there/).
* [Rust Guide to CHANGELOG.md](https://medium.com/@autumn_eng/guide-to-changelog-md-in-rust-6eb349808fa4).
* [Rules-based Network programming with Mio and Rust](http://www.lshift.net/blog/2016/01/27/rules-based-mio-chat-example/).
* [Eliminating the Garbage Collector: The RAII Way](http://www.toptal.com/software/eliminating-garbage-collector).
* [These weeks in Servo 48](https://blog.servo.org/2016/01/25/twis-48/).
* [Rust for the web](https://medium.com/@eugeniyoz/restful-api-in-rust-impressions-63250d611d15). RESTful API in Rust.

## Notable New Crates & Project Updates

* [clippy now has 100 different lints](https://llogiq.github.io/2016/01/28/hundred-lints.html). clippy is a collection of lints to catch common mistakes and improve your Rust code.
* [clap-rs 2.0 is released](https://twentyfives.net/clap-rs-2-0-is-released/). Clap is a full featured, fast Command Line Argument Parser for Rust.
* [Palette 0.2.0 is released](https://ogeon.github.io/2016/01/31/palette-0-2-0-a-different-shade.html). Palette is a Rust library for linear color calculations and conversion.
* [scoped-pool](https://github.com/reem/rust-scoped-pool). A flexible thread pool providing scoped and static threads.
* [rusty-blockparser](https://github.com/gcarq/rusty-blockparser). Multi-threaded blockchain parser written in Rust language.
* [kirk](https://github.com/kinghajj/kirk). A highly-flexible thread pool for Rust.

# Updates from Rust Core

114 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-01-25..2016-02-01

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-mon-feb-01-2016/3142
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2016-02-01/3141

## Notable changes

* [Implement unwinding for i686 MSVC](https://github.com/rust-lang/rust/pull/30448).
* [Explicitly disable stack execution on linux and bsd](https://github.com/rust-lang/rust/pull/30859).
* [Improve error message for non-exhaustive patterns](https://github.com/rust-lang/rust/pull/31020).
* [mk: Move from `-D warnings` to `#![deny(warnings)]`](https://github.com/rust-lang/rust/pull/31120).
* [Implement RFC amendment 1462 (add `[` to FOLLOW(ty))](https://github.com/rust-lang/rust/pull/31152).
* [Implement the translation item collector](https://github.com/rust-lang/rust/pull/30900).
* [std: Stabilize custom hasher support in HashMap](https://github.com/rust-lang/rust/pull/31081).
* [Update expression span when transcribing macro args](https://github.com/rust-lang/rust/pull/31089).
* [Cargo: Introduce cargo metadata subcommand](https://github.com/rust-lang/cargo/pull/2196).
* [Cargo: Add convenience syntax to install current crate](https://github.com/rust-lang/cargo/pull/2205).
* [Initial work towards abort-free compilation](https://github.com/rust-lang/rust/pull/31206).
* [Implement MultiSpan error reporting](https://github.com/rust-lang/rust/pull/30411).
* [Expect all help/note messages are specified in a cfail test](https://github.com/rust-lang/rust/pull/30778).
* [rustc_resolve: Refactor away NameBindings and ImportResolutionPerNamespace](https://github.com/rust-lang/rust/pull/30843).
* [Extend save-analysis to support generated code](https://github.com/rust-lang/rust/pull/31097).
* [Add support for the crosstool-ng Raspberry Pi 2 toolchain](https://github.com/rust-lang/rust/pull/30948).
* [Add support for mips(el)-unknown-linux-musl](https://github.com/rust-lang/rust/pull/31298).

## New Contributors

* Ali Clark
* Daan Sprenkels
* ggomez
* tgor
* Thomas Wickham
* Tomasz Miąsko
* Vincent Esche

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1361: Improve Cargo target-specific dependencies](https://github.com/rust-lang/rfcs/pull/1361).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Restrict constants in patterns](https://github.com/rust-lang/rfcs/pull/1445).
* [Amend RFC 1270 to describe actual implementation](https://github.com/rust-lang/rfcs/pull/1423).
* [Deprecate type aliases in `std::os::*::raw`](https://github.com/rust-lang/rfcs/pull/1415).
* [Add `CommandExt::{exec, before_exec}`](https://github.com/rust-lang/rfcs/pull/1359).
* [Add `retain_mut` to `Vec` and `VecDeque`](https://github.com/rust-lang/rfcs/pull/1353).
* [Rust Language Server (IDE support)](https://github.com/rust-lang/rfcs/pull/1317).
* [Add a `SharedSender` to `std::sync::mpsc` that implements `Sync`](https://github.com/rust-lang/rfcs/pull/1299).
* [Propose a design for _specialization_, which permits multiple `impl` blocks to apply to the same type/trait](https://github.com/rust-lang/rfcs/pull/1210).
* [Let specified generic type parameter lists be abbreviated](https://github.com/rust-lang/rfcs/pull/1196).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Trait-based exception handling](https://github.com/rust-lang/rfcs/pull/243).

## New RFCs

* [Unix socket support in the standard library](https://github.com/rust-lang/rfcs/pull/1479).
* [Proposal for thread affinity](https://github.com/rust-lang/rfcs/pull/1480).

# Upcoming Events

* [2/3. Rust Meetup in Cologne / Germany](http://www.meetup.com/de/Rust-Cologne-Bonn/events/227534456/).
* [2/8. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [2/10. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [2/10. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [2/11. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [2/12. Embedded Rust Workshop Frankfurt](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/228170051/).
* [2/15. Rust Paris](http://www.meetup.com/Rust-Paris).
* [2/15. Rust Sydney Meetup](http://www.meetup.com/Rust-Sydney/events/228043858/).
* [2/17. Rust Berlin: Leaf and Collenchyma](http://www.meetup.com/Rust-Berlin/events/227321071/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [herbie-lint](https://crates.io/crates/herbie-lint), a miraculous compiler plugin to check the numerical stability of floating-point operations in the code. Another reason to have a nightly Rust handy.

Thanks to redditor [protestor](https://www.reddit.com/user/protestor) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> imo: the opinionated version of mio

— [durka42 on #rust](https://botbot.me/mozilla/rust/2016-02-01/?msg=59153775&page=20)

Thanks to [Steve Klabnik](https://users.rust-lang.org/users/steveklabnik) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
