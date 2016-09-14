Title: This Week in Rust 148
Number: 148
Date: 2016-09-20
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

## New Crates & Project Updates

# Crate of the Week

This week's crate of the week is [tokio](https://github.com/tokio-rs/tokio), a high-level asynchronous IO library based on futures. Thanks to [notriddle](https://users.rust-lang.org/users/notriddle) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [hard] [rust: Support Apple app store bitcode](https://github.com/rust-lang/rust/issues/35968).
* [hard] [rust: Missed opportunities to eliminate bounds checks](https://github.com/rust-lang/rust/issues/35981).
* [easy] [tempdir: make directory removal robust on windows](https://github.com/rust-lang-nursery/tempdir/issues/15). This bug lets you publish a replacement for the unreliable `std::fs::remove_dir_all` fn.
* [moderate] [rust: Create official .deb packages](https://github.com/rust-lang/rust/issues/28307).
* [easy] [rust-www: Better front-page example](https://github.com/rust-lang/rust-www/issues/180).
  The front page example on the website isn't so special. Make it shine.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

84 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-09-05..2016-09-12

* [Don't double-count nested struct prefixes in DST size calculation](https://github.com/rust-lang/rust/pull/36351)
* [Individual MIR passes now show up in `-Z time-passes`](https://github.com/rust-lang/rust/pull/36296)
* [Refs to associated sized types no longer result in ICE](https://github.com/rust-lang/rust/pull/36281)
* [`impl Trait` now correctly reports the empty lifetime](https://github.com/rust-lang/rust/pull/36333)
* [Errors in macros now note the correct location](https://github.com/rust-lang/rust/pull/36308)
* [Suggest `self::_` when missing item in current module](https://github.com/rust-lang/rust/pull/36289)
* [`save-analysis` changes variable value output](https://github.com/rust-lang/rust/pull/36288)
* [Item-like imports are no longer reported as unused](https://github.com/rust-lang/rust/pull/36276)
* [Compiler controllers can now access the Registry](https://github.com/rust-lang/rust/pull/36240)
* [Macros are now stacklessly expanded](https://github.com/rust-lang/rust/pull/36214)
* [Cargo is now Macros-1.1-ready](https://github.com/rust-lang/cargo/pull/3064)
* [Accessing external statics now requires `unsafe`](https://github.com/rust-lang/rust/pull/36173)
* [Cyclic traits no longer allow arbitrary traits to be synthesized](https://github.com/rust-lang/rust/pull/35745) (which lead to unsoundness)
* [Rustdoc filters out `Deref`fed methdos on `&mut self` unless `self impl`s `DerefMut`](https://github.com/rust-lang/rust/pull/36266)

## New Contributors

* Cobrand
* Jake Goldsborough
* John Firebaugh
* Justin LeFebvre
* Kylo Ginsberg
* Nicholas Nethercote
* orbea
* Richard Janis Goldschmidt
* Ulrich Weigand

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1620: regex 1.0](https://github.com/rust-lang/rfcs/pull/1620).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [`mem::discriminant()`](https://github.com/rust-lang/rfcs/pull/1696). Add a function that extracts the discriminant from an enum variant as a comparable, hashable, printable, but (for now) opaque and unorderable type.
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).

## New RFCs

* [Check future-proofing of `macro_rules!` using FIRST sets](https://github.com/rust-lang/rfcs/pull/1746).

# Upcoming Events

* **[9/17. Rustfest Europe Conference](http://www.rustfest.eu/)**.
* [9/19. Paris - Rust Paris](https://www.meetup.com/Rust-Paris/events/230111512/).
* [9/20. Rust NYC Meetup](https://www.meetup.com/Rust-NYC/events/233756447/).
* [9/21. Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/events/233463725/).
* 9/21. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [9/22. RustPH Mentors Meeting](http://www.rustph.tech/).
* 9/22. Rust release triage at #rust-triage on irc.mozilla.org.
* [9/26. São Paulo Meetup](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/233713814/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust engineer at MaidSafe](http://maidsafe.net/careers.html#rust_engineer).
* [Rust developer at ANIXE](http://anixe.pl/rust_dev/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
