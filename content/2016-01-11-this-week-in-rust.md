Title: This Week in Rust 113
Number: 113
Date: 2016-01-11
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

* [My thoughts on Rust in 2016](http://www.ncameron.org/blog/my-thoughts-on-rust-in-2016/). By Nick Cameron.
* [Making your open-source project newcomer-friendly](https://manishearth.github.io/blog/2016/01/03/making-your-open-source-project-newcomer-friendly/).
* [This Week In Servo 46](http://blog.servo.org/2016/01/04/twis-46/).
* [Compiling Rust to an Android target](https://ghotiphud.github.io/rust/android/cross-compiling/2016/01/06/compiling-rust-to-android.html).
* [ArcadeRS 1.11: Shooting bullets](https://jadpole.github.io/arcaders/arcaders-1-11/). Part of the series [ArcadeRS 1.0: The project](https://jadpole.github.io/arcaders/arcaders-1-0/) - a series whose objective is to explore the Rust programming language and ecosystem through the development of a simple, old-school shooter.
* [GC and Rust part 2: The roots of the problem](http://blog.pnkfx.org/blog/2016/01/01/gc-and-rust-part-2-roots-of-the-problem/).
* [slides] [Why I ❤ Rust](https://speakerdeck.com/jvns/why-i-rust). By Julia Evans.
* [Discovering hardware topology in Rust](http://nitschinger.at/Discovering-Hardware-Topology-in-Rust/).
* [The scope of `unsafe`](https://www.ralfj.de/blog/2016/01/09/the-scope-of-unsafe.html).
* [Two weeks of Rust - building a Memcache clone](http://www.matusiak.eu/numerodix/blog/2016/1/10/two-weeks-rust/).
* [Abstract return types, aka `impl Trait`](http://www.ncameron.org/blog/abstract-return-types-aka-%60impl-trait%60/).
* [RustBelt: Logical Foundations for the Future of Safe Systems Programming](http://plv.mpi-sws.org/rustbelt/).

## Notable New Crates & Project Updates

* [Robigalia](https://robigalia.org/). A Rust userland and POSIX layer built on seL4.
* [Flac](https://github.com/sourrust/flac). An implementation of FLAC, free lossless audio codec, written in Rust.
* [Imageproc](https://github.com/PistonDevelopers/imageproc). An image processing library, in Rust.
* [Heroku Buildpack for Rust](https://hoverbear.org/2016/01/04/Heroku-Rust-Buildpack/).
* [YouCompleteMe now supports Rust](http://blog.jwilm.io/youcompleteme-rust). YouCompleteMe is a fast, fuzzy, as-you-type code-completion engine built originally for Vim.

# Updates from Rust Core

61 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-01-04..2016-01-11

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-tue-jan-05-2016/3052
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2016-01-08/3067

## Notable changes

* [Feature gate defaulted type parameters appearing outside of types](https://github.com/rust-lang/rust/pull/30724).
* [Make `".".parse::<f32>()` and `".".parse::<f64>()` return `Err`](https://github.com/rust-lang/rust/pull/30681).
* [Add `std::panic::propagate`](https://github.com/rust-lang/rust/pull/30557).
* [Cross item dependencies, take 2](https://github.com/rust-lang/rust/pull/30532). Adds dependency graph for incremental compilation.
* [libstd: unix process spawning: fix bug with setting stdio](https://github.com/rust-lang/rust/pull/30490).
* [Add `OpAssign` to `Wrapping<T>`, etc. in `core::num::wrapping`](https://github.com/rust-lang/rust/pull/30523).
* [[MIR] Refine representation and translation of calls](https://github.com/rust-lang/rust/pull/30481).
* [Refactor and improve: `Arena`, `TypedArena`](https://github.com/rust-lang/rust/pull/27807).
 
## New Contributors

* Anders Granlund
* BChip
* jonastepe
* Lawrence Woodman
* Matt Kraai
* Michael F. Lamb
* Mike Anderson
* Nathan
* Zach Panzarino

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

* [`src/grammar` for the canonical grammar of the Rust language](https://github.com/rust-lang/rfcs/pull/1331).
* [Improve Cargo target-specific dependencies](https://github.com/rust-lang/rfcs/pull/1361).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Allow eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Provide native support for C-compatible unions, defined via a new keyword `untagged_union`](https://github.com/rust-lang/rfcs/pull/1444).
* [Restrict constants in patterns](https://github.com/rust-lang/rfcs/pull/1445).
* [Add language support for bitfields](https://github.com/rust-lang/rfcs/pull/1449).
* [Variant types and untagged enums](https://github.com/rust-lang/rfcs/pull/1450).
* [Rewrite the `for` loop desugaring to use language items instead of hardcoded paths](https://github.com/rust-lang/rfcs/pull/1457).
* [Extend atomic `compare_and_swap`](https://github.com/rust-lang/rfcs/pull/1443).

# Upcoming Events

* [1/12. Eat, Drink, Rust! San Diego Downtown Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/227308164/).
* [1/13. Copenhagen hackathon](https://cph.rs/).
* [1/13. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [1/13. Los Angeles Monthly Meetup - Happy New Year Hack Night](http://www.meetup.com/Rust-Los-Angeles/events/227438139/).
* [1/14. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [1/14. Rust São Paulo Meetup](http://www.meetup.com/Rust-Sao-Paulo-Meetup/events/227499416/).
* [1/15. Rhein-Main Rust Meetup](http://www.meetup.com/de/Rust-Rhein-Main/events/227808685/).
* [1/18. Rust Paris](http://www.meetup.com/Rust-Paris).
* [1/19. Rust Hack and Learn Hamburg @ Ponton](http://www.meetup.com/Rust-Meetup-Hamburg/events/227838367/).
* [1/21. SF Bay Area: Rust Concurrency and Parallelism](http://www.meetup.com/Rust-Bay-Area/events/227841778/).

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

This week's Crate of the Week is [crossbeam](https://crates.io/crates/crossbeam). This is a library of non-blocking data structures and synchronization primitives that makes writing concurrent programs easier and more efficient (both in terms of code and runtime).

Thanks to [DroidLogician](https://users.rust-lang.org/users/droidlogician) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

>> Rustaceans are not very imaginative at naming things.
>
> We try! and try!, but sometimes, we Err.

— [steveklabnik1 on /r/rust](https://www.reddit.com/r/rust/comments/3zledh/concurrency_in_rust/cyn4352).

Thanks to [msiemens](https://users.rust-lang.org/users/msiemens) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
