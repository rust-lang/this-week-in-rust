Title: This Week in Rust 97
Number: 97
Date: 2015-09-21
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* 🎈🎉 [Announcing Rust 1.3](http://blog.rust-lang.org/2015/09/17/Rust-1.3.html). 🎉🎈
* [Asynchronous IO in Rust](https://blog.skcript.com/asynchronous-io-in-rust-36b623e7b965).
* [An introduction to timely dataflow](https://github.com/frankmcsherry/blog/blob/7a1174e80134c8ce5338d5c62864a2aae835da04/posts/2015-09-14.md). Dataflow computational model written in Rust.
* [An introduction to timely dataflow, part 2.](https://github.com/frankmcsherry/blog/blob/master/posts/2015-09-18.md). Writing super-powerful operators that do whatever you want to the streams.
* [A Pythonista's irresistible attraction to Rust](http://hardcoded.net/articles/pythonistas-irresistible-attraction-rust).
* [Python Modules in Rust](https://ehiggs.github.io/2015/07/Python-Modules-In-Rust/).
* [Fine-grained timing and energy profiling in Servo](http://blog.servo.org/2015/09/11/timing-energy/).
* [Seeking Compilation-Independent Type IDs in Rust](http://davidlegare.ghost.io/seeking-compilation-independent-type-ids-in-rust/).
* [Running Rust code on the 3DS 2: Electric Boogaloo](http://www.idolagames.com/running-rust-code-on-the-3ds-2-electric-boogaloo/).
* [Specialize to reuse](https://aturon.github.io/blog/2015/09/18/reuse/). [Specialization](https://github.com/rust-lang/rfcs/pull/1210) supports clean, inheritance-like patterns out of the box. This post explains how, and discusses the interaction with the “virtual structs” saga.
* [Eliminating branches in Rust](http://kamalmarhubi.com/blog/2015/09/15/eliminating-branches-in-rust-for-fun-but-not-much-profit/).
* [Rust and the Monad trait - Not just higher kinded types](https://m4rw3r.github.io/rust-and-monad-trait/).

## Notable New Crates

* [rustlings](https://github.com/carols10cents/rustlings). Small exercises to get you used to reading and writing Rust code. Includes practice reading and responding to compiler messages!
* [LALRPOP - an LR(1) parser generator for Rust](http://smallcultfollowing.com/babysteps/blog/2015/09/14/lalrpop/).
* [Cuprum Pi](https://github.com/cuprumpi/cupi). A GPIO access library written on Rust for the Raspberry Pi.
* [hostblock](https://github.com/cgag/hostblock). Rust cli app for managing sites blocked via the hosts file.
* [Tokei](https://github.com/Aaronepower/tokei). A CLOC (Count Lines Of Code) program, written in Rust.
* [rust.fish](https://github.com/bheesham/rust.fish). Tab completions for `rustc` and `cargo` in fish shell.
* [rust-xdg](https://github.com/whitequark/rust-xdg). A library that makes it easy to follow the X Desktop Group specifications.

# Updates from Rust Core

116 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-09-14..2015-09-21

## Notable changes

* [Feature-gate `#[no_debug]` and `#[omit_gdb_pretty_printer_section]`](https://github.com/rust-lang/rust/pull/28522).
* [Warn on `pub extern crate`](https://github.com/rust-lang/rust/pull/28486).
* [Remove Visibility field from enum variants](https://github.com/rust-lang/rust/pull/28442).
* [Add a blanket `impl` for `&mut std::fmt::Write`](https://github.com/rust-lang/rust/pull/28368).
* [Implement overload-able augmented/compound assignments, like `a += b` via the `AddAssign` trait](https://github.com/rust-lang/rust/pull/28345).
* [Implement empty structs with braces](https://github.com/rust-lang/rust/pull/28336).
* [Split up lint interface into pre- and post-expansion lints](https://github.com/rust-lang/rust/pull/28349).

## New Contributors

* Bastien Dejean
* Colin Wallace
* David Szotten
* Dongie Agnir
* Michael McConville
* Peter Reid
* whitequark

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [1199: Lay the ground work for building powerful SIMD functionality.](https://github.com/rust-lang/rfcs/pull/1199).
* [1224: Update the RFC process with sub-teams, amongst other things.](https://github.com/rust-lang/rfcs/pull/1224).
* [1238: Revise the Drop Check (`dropck`) part of Rust's static analyses](https://github.com/rust-lang/rfcs/pull/1238).
* [1240: References into `repr(packed)` structs should be `unsafe`](https://github.com/rust-lang/rfcs/pull/1240).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Forbid wildcard dependencies on crates.io](https://github.com/rust-lang/rfcs/pull/1241).
* [Amend #911 const-fn to allow unsafe const functions](https://github.com/rust-lang/rfcs/pull/1245).
* [Place left arrow syntax (`place <- expr`)](https://github.com/rust-lang/rfcs/pull/1228).

## New RFCs

* [Machine-readable output of tests](https://github.com/rust-lang/rfcs/pull/1284).
* [Improvements to the Time APIs](https://github.com/rust-lang/rfcs/pull/1288).

# Upcoming Events

* [9/23. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [9/25. Rust Wellington](http://www.meetup.com/Wellington-Rust-Meetup/events/225362848/).
* [9/28. Rust Sydney Meetup](http://www.meetup.com/Rust-Sydney/events/225175121/).
* 9/30. RustBerlin Hack and Learn.
* [10/1. Rust Meetup Hamburg: Rusty Project Presentations](http://www.meetup.com/Rust-Meetup-Hamburg/events/225391520/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This is a new part of this weekly installation, where we will write about a crate that some of you may not know.
Please nominate a crate of your choice at the [rust-users thread](https://users.rust-lang.org/t/crate-of-the-week/2704/15) so we can write about it next week.

This week, Crate of the Week is **[lazy_static](https://crates.io/crates/lazy_static)**. Thanks go to [stebalien](https://users.rust-lang.org/users/stebalien) for the suggestion.

This week's Crate of the Week does something some won't know is possible, and does it in a natural way. You know how in some languages you have to run through hoops to correctly lazily instantiate stuff? Not in Rust, for thanks to the [lazy_static](https://crates.io/crates/lazy_static) crate you just put your static values inside a `lazy_static! { ... }` block, pay a modest runtime cost on lookup and be done.

Btw, this aligns very well with the C++ adage *"... and what you pay for, you couldn't have written any better"*.

# Quote of the Week

*Transmute is taking a dog, sawing its front legs off, gluing on a pair of buffalo wings and telling it it's a duck so it damn well better start quacking
You should not be surprised when you end up with a pile of gore and a dead dog instead of an actual duck :-P* — Quxxy

Thanks to [Ms2ger](https://users.rust-lang.org/users/Ms2ger) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
