Title: This Week in Rust 106
Number: 106
Date: 2015-11-23
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

* [Rust on AWS Lambda](http://julienblanchard.com/2015/rust-on-aws-lambda/).
* [Async IO for Rust (part II)](https://medium.com/@paulcolomiets/async-io-for-rust-part-ii-33b9a7274e67).
* [GC and Rust Part 0: Garbage collection background](http://blog.pnkfx.org/blog/2015/10/27/gc-and-rust-part-0-how-does-gc-work/).
* [GC and Rust Part 1: Specifying the problem](http://blog.pnkfx.org/blog/2015/11/10/gc-and-rust-part-1-specing-the-problem/).
* [Lessons learned from Rust: The Result Monad in Ruby](http://www.codethatgrows.com/lessons-learned-from-rust-the-result-monad/).
* [Bare Metal Rust 3: Configure your PIC to handle interrupts correctly](http://www.randomhacks.net/2015/11/16/bare-metal-rust-configure-your-pic-interrupts/).
* [`concat_idents!` and macros in ident position](http://ncameron.org/blog/untitledconcat_idents-and-macros-in-ident-position/).
* [This week in Piston](http://blog.piston.rs/2015/11/17/what-is-happening/).
* [This week in Servo 42](http://blog.servo.org/2015/11/16/twis-42/).
* [This week in Redox OS 6](http://www.redox-os.org/news/this-week-in-redox-6/).
* [Russian] [Software render in Rust: Cubic texture](http://reangdblog.blogspot.com/2015/11/software-render-rust.html). ([Translated version](https://translate.google.com/translate?u=http://reangdblog.blogspot.com/2015/11/software-render-rust.html)).

## Notable New Crates & Projects

* [nom 1.0 is released](https://www.clever-cloud.com/blog/engineering/2015/11/16/nom-1-0/).
* [Freepass](https://github.com/myfreeweb/freepass). The free password manager for power users.
* [Barcoders](https://github.com/buntine/barcoders). A barcode encoding library for the Rust programming language.
* [fst](https://github.com/BurntSushi/fst). Fast implementation of ordered sets and maps using finite state machines.
* [Rusty Code](https://github.com/saviorisdead/RustyCode). Advanced language support for the Rust language in Visual Studio Code.
* [Dybuk](https://github.com/Ticki/dybuk). Prettify the ugly Rustc messages (inspired by Elm).
* [Substudy](https://github.com/emk/substudy). Use SRT subtitle files to study foreign languages.

# Updates from Rust Core

99 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-11-16..2015-11-23

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-wed-nov-18-2015/2916
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-11-16/2910

## Notable changes

* [RFC 1288: Add `Instant` and `SystemTime` to `std::time`](https://github.com/rust-lang/rust/pull/29894).
* [Detect confusing unicode characters and show the alternative](https://github.com/rust-lang/rust/pull/29837).
* [Rework stability annotation pass](https://github.com/rust-lang/rust/pull/29083).
* [Report errors at macro definition, not expansion](https://github.com/rust-lang/rust/pull/29828).
* [Rename `ImplItem_::*ImplItem` to `ImplItem_::*`](https://github.com/rust-lang/rust/pull/29766).
* [liballoc: implement `From` for `Box`, `Rc`, `Arc`](https://github.com/rust-lang/rust/pull/29580).
* [Look up macro names as well when suggesting replacements for function resolve error](https://github.com/rust-lang/rust/pull/29968).
* [Ignore malformed environment strings like glibc does](https://github.com/rust-lang/rust/pull/29297).
* [Store items out-of-line in the HIR](https://github.com/rust-lang/rust/pull/29903).
* [Cargo: Make all working directories refer to `Config::cwd()`](https://github.com/rust-lang/cargo/pull/2058).
* [crates.io: Allow per-crate max upload sizes](https://github.com/rust-lang/crates.io/pull/218).

## New Contributors

* Alexander Bulaev
* Ashkan Kiani
* Devon Hollowood
* Doug Goldstein
* Jean Maillard
* Joshua Holmer
* Matthias Kauer
* Ole Krüger
* Ravi Shankar

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1270: `#[deprecated]` for Everyone](https://github.com/rust-lang/rfcs/pull/1270).
* [RFC 1268: Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [RFC 1288: Improvements to the Time APIs](https://github.com/rust-lang/rfcs/pull/1288).
* [RFC 1300: Define the general semantics of intrinsic functions](https://github.com/rust-lang/rfcs/pull/1300).
* [RFC 1323: Amend `recover` with a `PanicSafe` bound](https://github.com/rust-lang/rfcs/pull/1323).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Add thread-local custom panic handlers to customize the behavior of thread panics](https://github.com/rust-lang/rfcs/pull/1100).
* [Allow a custom panic handler](https://github.com/rust-lang/rfcs/pull/1328).
* [Document and expand the open options](https://github.com/rust-lang/rfcs/pull/1252).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Remove some kinds of doc comments](https://github.com/rust-lang/rfcs/pull/1373).

# Upcoming Events

* [11/25. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [11/25. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/).
* [12/1. Rust Sydney Meetup](http://www.meetup.com/Rust-Sydney/events/226832397/).
* [12/2. Boston: Concurrency in Rust Tutorial](http://www.meetup.com/Boston-Rust-Meetup-25317522aNpHwZdw/events/226759437/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla. Join the core Rust team!
* [Open Source Software Engineer](http://maidsafe.net/careers) at MaidSafe.
* [Systems Engineer](https://twitter.com/jarrednicholls/status/664446704410861568) at IronNet Cybersecurity.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [nom](https://crates.io/crates/nom), a library of fast zero-copy parser combinators, which has already been used to create safe, high-performance parsers for a number of formats both binary and textual. nom just reached version 1.0, too, so congratulations for both the major version and the CotW status!

Thanks to Reddit user [gbersac](https://www.reddit.com/user/gbersac) for the [nom-ination](https://www.reddit.com/r/rust/comments/3t10f3/nom_just_reached_10_cleaner_parsers_more_generic/cx2or2m)! [Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
