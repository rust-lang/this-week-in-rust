Title: This Week in Rust 115
Number: 115
Date: 2016-01-25
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

* 🎈🎉 [Announcing Rust 1.6](http://blog.rust-lang.org/2016/01/21/Rust-1.6.html). 🎉🎈
* [Rust, BigData and my laptop](http://www.poumeyrol.fr/2016/01/15/Awkward-zone/).
* [pdf][You can't spell trust without Rust](https://cdn.rawgit.com/Gankro/thesis/master/thesis.pdf). Analysis of the semantics and expressiveness of Rust’s type system.
* [Libmacro - an API for procedural macros to interact with the compiler](http://www.ncameron.org/blog/libmacro/).
* [Rust and the Blub Paradox](http://www.jonathanturner.org/2016/01/rust-and-blub-paradox.html). And the [follow-up](http://www.jonathanturner.org/2016/01/rethinking-the-blub-paradox.html).
* [video] [Ferris Makes Emulators](https://www.youtube.com/channel/UC4mpLlHn0FOekNg05yCnkzQ/videos). Live stream of Ferris developing a N64 emulator in Rust (also on [Twitch](http://www.twitch.tv/ferrisstreamsstuff/profile)).

## Notable New Crates & Project Updates

* [Are we concurrent yet](http://areweconcurrentyet.com/)?
* [GFX](https://github.com/gfx-rs/gfx) epic rewrite for the Pipeline State Objects paradigm has [landed](https://github.com/gfx-rs/gfx/pull/828), described [on the blog](http://gfx-rs.github.io/2016/01/22/pso.html).
* [Herbie](https://github.com/mcarton/rust-herbie-lint). A rustc plugin to check for numerical instability.
* [Dynamo](http://blog.piston.rs/2016/01/23/dynamo/). A rusty dynamically typed scripting language.
* [rust-vnc](https://github.com/whitequark/rust-vnc). An implementation of VNC protocol, client state machine and a client.

# Updates from Rust Core

129 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-01-18..2016-01-25

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-mon-jan-25-2016/3111
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2016-01-22/3106

## Notable changes

* [Implement RFC 1252 expanding the OpenOptions structure](https://github.com/rust-lang/rust/pull/30872).
* [Book: First draft of 'ownership'](https://github.com/rust-lang/book/pull/58).
* [Cargo: Add convenience syntax to install current crate](https://github.com/rust-lang/cargo/pull/2205).
* [Cargo: Introduce cargo metadata subcommand](https://github.com/rust-lang/cargo/pull/2196).
* [Cargo: Implement `cargo init`](https://github.com/rust-lang/cargo/pull/2081).
* [Cargo: Emit a warning when manifest specifies empty dependency constraints](https://github.com/rust-lang/cargo/pull/2270).
* [Change name when outputting staticlibs on Windows](https://github.com/rust-lang/rust/pull/29520).
* [Make `btree_set::{IntoIter, Iter, Range}` covariant](https://github.com/rust-lang/rust/pull/30998).
* [Avoid bounds checking at `slice::binary_search`](https://github.com/rust-lang/rust/pull/30917).
* [`std::sync::mpsc`: Add `fmt::Debug` stubs](https://github.com/rust-lang/rust/pull/30894).
* [resolve: Fix variant namespacing](https://github.com/rust-lang/rust/pull/30882).

## New Contributors

* Adrian Heine
* Andrea Bedini
* Guillaume Bonnet
* Kamal Marhubi
* Keith Yeung
* Marc Bowes
* Martin
* mopp
* Olaf Buddenhagen
* Paul Dicker
* Peter Kolloch
* Stephen (Ziyun) Li

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amendment to RFC 550: Add `[` to the FOLLOW(ty) in macro future-proofing rules](https://github.com/rust-lang/rfcs/pull/1462).
* [Amendment to RFC 1192: Amend `RangeInclusive` to use an enum](https://github.com/rust-lang/rfcs/pull/1320).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Trait-based exception handling](https://github.com/rust-lang/rfcs/pull/243).
* [Improve Cargo target-specific dependencies](https://github.com/rust-lang/rfcs/pull/1361).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Allow eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Add compiler support for generic atomic operations](https://github.com/rust-lang/rfcs/pull/1477).
* [Translate undefined generic intrinsics to an LLVM `unreachable` and a lint](https://github.com/rust-lang/rfcs/pull/1478).

# Upcoming Events

* [1/27. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [1/28. Tokyo Rust Meetup #2](http://www.meetup.com/Tokyo-Rust-Meetup/events/227871840/).
* [2/3. Rust Berlin: Leaf and Collenchyma](http://www.meetup.com/Rust-Berlin/events/227321071/).
* [2/3. Rust Meetup in Cologne / Germany](http://www.meetup.com/de/Rust-Cologne-Bonn/events/227534456/).
* [2/8. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [2/12. Embedded Rust Workshop Frankfurt](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/228170051/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust Engineer](http://maidsafe.net/rust_engineer.html) at MaidSafe.
* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [racer](https://github.com/phildawes/racer) which powers code completion in all Rust development environments.

Thanks to [Steven Allen](https://users.rust-lang.org/users/stebalien) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> Memory errors are fundamentally state errors, and Rust's move semantics, borrowing, and aliasing XOR mutating help enormously for me to reason about how my program changes state as it executes, to avoid accidental shared state and side effects at a distance. Rust more than any other language I know enables me to do compiler driven design. And internalizing its rules has helped me design better systems, even in other languages.

— [desiringmachines on /r/rust](https://www.reddit.com/r/rust/comments/4275gz/rust_and_the_blub_paradox/cz8akv9).

Thanks to [dikaiosune](https://users.rust-lang.org/users/dikaiosune) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
