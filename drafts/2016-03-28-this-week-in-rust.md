Title: This Week in Rust 124
Number: 124
Date: 2016-03-28
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

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts

* [OneSignal now sends iOS push notifications 100x faster](https://onesignal.com/blog/announcing-our-new-delivery-backend/). Using Rust of course!
* [String types in Rust](https://andrewbrinker.github.io/blog/2016/03/27/string-types-in-rust/). Andrew explains the difference between `String`, `str`, `OsStr`, `OsString`, `CStr`, and `CString`.
* [Installing Rust on Windows](https://facility9.com/2016/03/installing-rust-on-windows/).
* [Async hyper](http://seanmonstar.com/post/141495445652/async-hyper). Asynchronous IO is coming in hyper.
* [Exploring Rust](http://nblumhardt.com/2016/03/exploring-rust/) from a C# developer's perspective.
* [Linear Algebra written entirely in Rust](https://athemathmo.github.io/2016/03/23/linear-algebra-in-rust.html).
* [Rust mutation testing](https://llogiq.github.io/2016/03/24/mutest.html).
* [This week in Servo 56](http://blog.servo.org/2016/03/21/twis-56/).
* [These weeks in Amethyst 8](https://www.amethyst.rs/_posts/twia-8.html).

## Notable New Crates & Project Updates

* [Erick Tryzelaar joins the core team](https://internals.rust-lang.org/t/announcement-erick-tryzelaar-joins-the-core-team/3273)!
* [Jonathan Turner joins Mozilla’s Rust team](https://internals.rust-lang.org/t/announcement-jonathan-turner-joins-mozillas-rust-team/3278)!
* [GFX now supports D3D11 (DX11) as a backend](https://github.com/gfx-rs/gfx/pull/861).
* Parity (an Ethereum client in Rust) has [released v1.0](https://blog.ethcore.io/1-0-is-here/).
* [BitSparrow](http://bitsparrow.io/). Binary serializer to exchange data between Rust and JavaScript.
* [Emit](https://github.com/nblumhardt/emit). An experimental structured event emitter for Rust.
* [Gracer](https://github.com/isamert/gracer). A Rust code completion plugin for gedit.
* [untry](https://github.com/japaric/untry). Convert `try!()` into `?`s.
* [cargo-deadlinks](https://github.com/deadlinks/cargo-deadlinks). Cargo subcommand for checking your documentation for broken links.

# Crate of the Week

This week's Crate of the Week is [tempfile](https://crates.io/crates/tempfile), a crate that does exactly what it says on the tin. Thanks to [Steven Allen](https://users.rust-lang.org/users/stebalien) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [medium] [buildengine5: Test `net::test::client_server_send` fails](https://github.com/Ameliorate/buildengine5/issues/10).
* [less easy] [Vulkano: Add a memory pool](https://github.com/tomaka/vulkano/issues/14).
* [easy] [Servo: Implement `HTMLScriptElement.type` and similar attributes](https://github.com/servo/servo/issues/10227).
* [easy] [Servo: Stop re-exporting webrender_traits WebGL types from canvas_traits](https://github.com/servo/servo/issues/10211).
* [easy] [Servo: Using viewport percentage lengths (e.g. vh, vw) causes unnecessary style recalc on resize](https://github.com/servo/servo/issues/10104).
* [easy] [Servo: Enable CCACHE for AppVeyor Windows builds](https://github.com/servo/servo/issues/9874).
* [easy] [`cargo add`: Target specifications](https://github.com/killercup/cargo-edit/issues/13).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

103 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-14..2016-03-21

## Notable changes

## New Contributors

* Alejandro Wainzinger
* Andrew Horton
* Cyryl Płotnicki-Chudyk
* David Henningsson
* ituxbag
* Kevin Brothaler
* nicholasf
* Novotnik, Petr

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1201: Add support for naked functions](https://github.com/rust-lang/rfcs/pull/1201).
* [RFC 1422: Expand the current pub/non-pub categorization of items with the ability to say "make this item visible solely to a (named) module tree"](https://github.com/rust-lang/rfcs/pull/1422).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add a contains method to VecDeque and LinkedList](https://github.com/rust-lang/rfcs/pull/1552).
* [Add libutil to scope of libc crate on Linux](https://github.com/rust-lang/rfcs/pull/1529).
* [Stabilize implementing panics as aborts](https://github.com/rust-lang/rfcs/pull/1513).
* [Add a standard allocator interface and support for user-defined allocators](https://github.com/rust-lang/rfcs/pull/1398).
* [Remove some kinds of doc comments](https://github.com/rust-lang/rfcs/pull/1373).
* [Unsafe expressions](https://github.com/rust-lang/rfcs/pull/1346).
* [Prevent unstable items from causing name resolution conflicts with downstream code](https://github.com/rust-lang/rfcs/pull/1321).
* [Amend RFC 1228 with operator fixity and precedence](https://github.com/rust-lang/rfcs/pull/1319).

## New RFCs

* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).

# Upcoming Events

* [3/31. Tokyo Rust Meetup #4](http://www.meetup.com/Tokyo-Rust-Meetup/events/229260081/).
* [4/6. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [4/6. Germany/Cologne - Hack and Learn](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/229919455/).
* [4/6. Rust São Paulo Meetup](http://www.meetup.com/Rust-Sao-Paulo-Meetup/events/229377422/).
* [4/11. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [4/15. Frankfurt/Main Rust Lint Workshop](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/229564640/?eventId=229564640)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.
* [Rust developer](http://rust.jobboard.io/jobs/125594-rust-developer-at-the-blackbird) at The Blackbird.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
