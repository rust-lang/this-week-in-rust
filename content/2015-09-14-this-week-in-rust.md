Title: This Week in Rust 96
Number: 96
Date: 2015-09-14
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

* [Why Rust?](http://www.oreilly.com/programming/free/files/why-rust.pdf). Free ebook by Jim Blandy (published by O’Reilly Media).
* [“The Rust Programming Language” to be published by No Starch Press](http://words.steveklabnik.com/the-rust-programming-language-will-be-published-by-no-starch-press). All profits from the book will go to [OpenHatch](http://openhatch.org).
* [Parser Combinator Experiments in Rust - Part 3: Performance and impl Trait](https://m4rw3r.github.io/parser-combinator-experiments-part-3/).
* [Building a Build System: Bazel reaches Beta](http://google-opensource.blogspot.com.es/2015/09/building-build-system-bazel-reaches-beta.html). Includes support for Rust.

## Notable New Crates

* [TRust-DNS](http://trust-dns.org/). A Rust based DNS server.
* [mio-websockets](https://github.com/burrows-labs/mio-websockets). Asynchronous websocket server library.
* [Crabby](https://github.com/Johnson-A/Crabby). Chess Engine written using the Rust programming language.
* [sprs](https://github.com/vbarrielle/sprs). Sparse linear algebra library for Rust.
* [oak](https://github.com/ptal/oak). A typed parser generator embedded in Rust code for Parsing Expression Grammar.
* [cbor](https://github.com/pyfisch/cbor). Serde CBOR Serialization Library.
* [ease](https://github.com/SimonPersson/ease). A library for writing REST API clients.
* [rust-bloom-filter](https://github.com/jedisct1/rust-bloom-filter). A fast Bloom filter implementation in Rust.
* [rust-nats](https://github.com/jedisct1/rust-nats). A Rust client library for the NATS message queue.

# Updates from Rust Core

79 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-09-07..2015-09-14

## Notable changes

* [Error Handing guide is rewritten from scratch](https://github.com/rust-lang/rust/pull/28301).
* [std: Stabilize/deprecate features for 1.4](https://github.com/rust-lang/rust/pull/28339).

## New Contributors

* Andre Bogus
* Dong Zhou
* Ryo Munakata
* Simon Mazur

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

* [Expand the `std::net` module to bind more low-level interfaces](https://github.com/rust-lang/rfcs/pull/1158).
* [Add `Box::leak` to leak `Box<T>` to `&'static mut T`](https://github.com/rust-lang/rfcs/pull/1233).
* [Forbid wildcard dependencies on crates.io](https://github.com/rust-lang/rfcs/pull/1241).
* [References into `repr(packed)` structs should be `unsafe`](https://github.com/rust-lang/rfcs/pull/1240).
* [Revise the Drop Check (`dropck`) part of Rust's static analyses](https://github.com/rust-lang/rfcs/pull/1238).
* [Update the RFC process with sub-teams, amongst other things.](https://github.com/rust-lang/rfcs/pull/1224).
* [Lay the ground work for building powerful SIMD functionality.](https://github.com/rust-lang/rfcs/pull/1199).

## New RFCs

* [Support interrupt calling conventions](https://github.com/rust-lang/rfcs/pull/1275).

# Upcoming Events

* [9/15. San Diego Rust Meetup #8](http://www.meetup.com/San-Diego-Rust/events/224577039/).
* 9/16. RustBerlin Hack and Learn.
* [9/17. Rust Meetup Hamburg: Show, Tell and Drink](http://www.meetup.com/Rust-Meetup-Hamburg/events/225116081/).
* [9/21. Rust Paris](http://www.meetup.com/Rust-Paris).
* [9/23. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [9/28. Rust Sydney Meetup](http://www.meetup.com/Rust-Sydney/events/225175121/).
* 9/30. RustBerlin Hack and Learn.

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

This week, Crate of the Week is **[clap](https://github.com/kbknapp/clap-rs)**. Thanks go to [eternaleye](https://users.rust-lang.org/users/eternaleye) for the suggestion.

Quoting [eternaleye](https://users.rust-lang.org/users/eternaleye) verbatim:

*I'm going to say kbknapp's [clap](https://github.com/kbknapp/clap-rs) crate - I have never, in _any_ language I have ever worked in, had an argument parsing library that was so completely painless.
I've found it especially nice for mocking up the skeleton of a tool where all roads lead to panic!(), then splitting it up further and further, pushing the panic!()s down the branching logic of what to actually do, until a whole utility has appeared from nowhere.*

# Quote of the Week

On `#rust-offtopic` IRC

> 03:46 < durka42> rust has a culture of small crates

> 03:47 < XMPPwocky> a Cargo cult, if you will

Thanks to [Manishearth](https://users.rust-lang.org/users/Manishearth) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
