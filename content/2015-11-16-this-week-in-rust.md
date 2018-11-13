Title: This Week in Rust 105
Number: 105
Date: 2015-11-16
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

* [**Redox is serious**](http://dictator.redox-os.org/index.php?controller=post&action=view&id_post=17). Redox is an upcoming OS written in Rust.
* [Index 1,600,000,000 keys with Automata and Rust](http://blog.burntsushi.net/transducers/).
* [Writing an OS in Rust: Allocating frames](http://os.phil-opp.com/allocating-frames.html).
* [Making Popcorn: Adding a disk to a Rust Rumprun Unikernel](https://polyfractal.com/post/adding-a-disk-to-a-rust-rumprun-unikernel/).
* [Rust in detail: Sending and receiving messages](https://nbaksalyar.github.io/2015/11/09/rust-in-detail-2.html).
* [Rust impressions from a C++/D programmer, part 1](https://atilanevesoncode.wordpress.com/2015/11/09/rust-impressions-from-a-cd-programmer-part-1/).
* [Rust impressions from a C++/D programmer, part 2](https://atilanevesoncode.wordpress.com/2015/11/16/rust-impressions-from-a-cd-programmer-part-2/).
* [Bare metal Rust 2: Retarget your compiler so interrupts are not evil](http://www.randomhacks.net/2015/11/11/bare-metal-rust-custom-target-kernel-space/).
* [This week in Servo 41](http://blog.servo.org/2015/11/09/twis-41/).

## Notable New Crates & Projects

* [rustfmt](https://github.com/rust-lang-nursery/rustfmt) is now part of [Rust Nursery](https://github.com/rust-lang-nursery).
* [Leaf](https://github.com/autumnai/leaf). Open Machine Intelligence Framework.
* [lrs](https://github.com/lrs-lang/lib). An experimental, linux-only standard library.

# Updates from Rust Core

95 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-11-09..2015-11-16

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-wed-nov-11-2015/2892
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-11-06/2886

## Notable changes

* [Add a MIR pass to simplify the control flow graph](https://github.com/rust-lang/rust/pull/29757).
* [BinaryHeap: Simplify sift down](https://github.com/rust-lang/rust/pull/29811).
* [Allow none Sized types in assert_eq!](https://github.com/rust-lang/rust/pull/29770).
* [Add `-Zinput-stats`](https://github.com/rust-lang/rust/pull/29764).
* [Suggest `mut` for mutability errors](https://github.com/rust-lang/rust/pull/29738).
* [Cargo: Add a rustdoc subcommand](https://github.com/rust-lang/cargo/pull/2129).
* [Syntax: Merge parsing code for structures and variants](https://github.com/rust-lang/rust/pull/29714).
* [Report errors at macro definition, not expansion](https://github.com/rust-lang/rust/pull/29828).

## New Contributors

* corentih
* Danilo Bargen
* Eric Findlay
* Erik Davidson
* Kohei Hasegawa
* Sebastian Hahn

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

* [Amend `recover` with a `PanicSafe` bound](https://github.com/rust-lang/rfcs/pull/1323).
* [Document and expand the open options](https://github.com/rust-lang/rfcs/pull/1252).
* [Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [`#[deprecated]` for Everyone](https://github.com/rust-lang/rfcs/pull/1270).
* [Improvements to the Time APIs](https://github.com/rust-lang/rfcs/pull/1288).
* [Define the general semantics of intrinsic functions](https://github.com/rust-lang/rfcs/pull/1300).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Add `#[repr(align = "N")]`](https://github.com/rust-lang/rfcs/pull/1358).
* [Add `CommandExt::{exec, before_exec}`](https://github.com/rust-lang/rfcs/pull/1359).
* [Improve Cargo target-specific dependencies](https://github.com/rust-lang/rfcs/pull/1361).
* [Transparently use verbatim paths on Windows](https://github.com/rust-lang/rfcs/pull/1370).

# Upcoming Events

* [11/17. Rust Hack and Learn Hamburg](http://www.meetup.com/Rust-Meetup-Hamburg/events/226298232/?a=ea1_grp&rv=ea1&_af=event&_af_eid=226298232&https=off).
* [11/18. Rust Los Angeles Monthly Meetup](http://www.meetup.com/Rust-Los-Angeles/events/226074704/).
* [11/19. Wellington Rust Meetup](http://www.meetup.com/Wellington-Rust-Meetup/events/226584413/).
* [11/25. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [11/25. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla. Join the core Rust team!
* [Open Source Software Engineer](https://www.talented.scot/Advert/180-Open-Source-Software-Engineer-%28C--C--Ruby--Java--Rust%29-Jobs-Scotland-Ayrshire.aspx) at MaidSafe.
* [Systems Engineer](https://twitter.com/jarrednicholls/status/664446704410861568) at IronNet Cybersecurity.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [Hyper](https://github.com/hyperium/hyper) which offers a Rust HTTP(S) implementation for both clients and servers.

Thanks to [DanielKeep](https://users.rust-lang.org/users/DanielKeep) for this week's suggestion. [Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
