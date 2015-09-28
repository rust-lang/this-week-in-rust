Title: This Week in Rust 98
Number: 98
Date: 2015-09-28
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

This week's edition was edited by: [nasa42](https://github.com/nasa42), [brson](https://github.com/brson), and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [If you use unsafe, you should be using compiletest](https://erickt.github.io/blog/2015/09/22/if-you-use-unsafe/).
* [Running Rust on the Rumprun unikernel](https://gandro.github.io/2015/09/27/rust-on-rumprun/).
* [Survey of licenses used by Rust projects on crates.io](http://paul.woolcock.us/posts/crates-io-license-survey.html).
* [An introduction to timely dataflow, part 3](https://github.com/frankmcsherry/blog/blob/master/posts/2015-09-21.md). Learn more about timely dataflow by writing a breadth-first search on random graphs.
* [These weeks in Servo 34](http://blog.servo.org/2015/09/21/twis-34/).
* [Get data from a URL in Rust](http://hermanradtke.com/2015/09/21/get-data-from-a-url-rust.html).
* [Debuger state machine in Rust](http://system.joekain.com/2015/09/16/rust-state-machine.html).

## Notable New Crates

* [rust-todomvc](https://github.com/tcr/rust-todomvc). Implementation of TodoMVC in Rust in the browser.
* [zas](https://github.com/juanibiapina/zas). A tool to help with local web development, inspired by Pow.
* [Serve](https://github.com/aochagavia/Serve). Command line utility to serve the files in the current directory.
* [Rodio](https://github.com/tomaka/rodio). Rust audio playback library.
* [io-providers](https://github.com/pshendry/io-providers). Defines "provider" traits and implementations for different types of I/O operations.
* [rust-sorty](https://github.com/Wafflespeanut/rust-sorty). A Rust lint to help with the sorting of uses, mods & crate declarations.
* [walkdir](https://github.com/BurntSushi/walkdir). Rust library for walking directories recursively.

# Updates from Rust Core

88 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-09-21..2015-09-28

## Notable changes

* [Correctly walk import lists in AST visitors](https://github.com/rust-lang/rust/pull/28364).
* [Remove region variable leaks from `higher_ranked_sub()`](https://github.com/rust-lang/rust/pull/28369).
* [Always pass `/DEBUG` flag to MSVC linker](https://github.com/rust-lang/rust/pull/28505).
* [Do not `drop_in_place` elements of `Vec<T>` if `T` doesn't need dropping](https://github.com/rust-lang/rust/pull/28531).
* [Make function pointers implement traits for up to 12 parameters](https://github.com/rust-lang/rust/pull/28560).
* [Use `BufWriter` in fasta-redux for a nice speedup](https://github.com/rust-lang/rust/pull/28562).
* [Upgrade hoedown to 3.0.5](https://github.com/rust-lang/rust/pull/28574).
* [Add `no_default_libraries` target linker option](https://github.com/rust-lang/rust/pull/28578).
* [Remove the deprecated `box(PLACE)` syntax](https://github.com/rust-lang/rust/pull/28608).
* [Implement `AsMut` for `Vec`](https://github.com/rust-lang/rust/pull/28663).

## New Contributors

* Amit Aryeh Levy
* David Elliott
* DenisKolodin
* Reza Akhavan
* Sebastian Wicki
* Xavier Shay

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [1241: Forbid wildcard dependencies on crates.io](https://github.com/rust-lang/rfcs/pull/1241).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Amend #911 const-fn to allow unsafe const functions](https://github.com/rust-lang/rfcs/pull/1245).
* [Place left arrow syntax (`place <- expr`)](https://github.com/rust-lang/rfcs/pull/1228).

## New RFCs

* [Promote the `libc` crate from the nursery](https://github.com/rust-lang/rfcs/pull/1291).
* [Add an internationalization framework to the Rust compiler](https://github.com/rust-lang/rfcs/pull/1292).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

# Upcoming Events

* 9/30. RustBerlin Hack and Learn.
* [10/1. Rust Meetup Hamburg: Rusty Project Presentations](http://www.meetup.com/Rust-Meetup-Hamburg/events/225391520/).
* [10/12. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This Week, Crate of the Week is BurntSushi's [quickcheck](https://github.com/BurntSushi/quickcheck). Out of all [quickcheck](https://en.wikipedia.org/wiki/Quickcheck) implementations, this is probably one of the more impressive ones. Thanks to DanielKeep, who had [this](https://users.rust-lang.org/t/crate-of-the-week/2704/40) to say:

    It helps write property-based tests: you define some property and how to test it, and quickcheck feeds your test random inputs as it tries to narrow down the ranges within which the property fails to hold. Handy when the set of possible test cases is very large.

I'd like to add an appeal to all supporters of "repeatable tests". Don't let the worthy goal of repeatability override the worthier goal of actually finding bugs. Your deterministic tests usually cannot even make a dent in the vast space of possible inputs. With a bit of randomness thrown in, you can greatly improve you chances and thus make your tests more valuable. Also with quickcheck, you get to see a _minimized_ input that makes your test fail, which you can then turn into a repeatable test easily.

# Quote of the Week

*If one regards Rust as a critique to C++, it certainly should be seen as a constructive critique.* — [llogiq on /r/cpp](https://www.reddit.com/r/cpp/comments/3m0d41/writing_good_c14_by_default_herb_sutter/cvdipz7).

Thanks to [msiemens](https://users.rust-lang.org/users/msiemens) for the tip. [Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
