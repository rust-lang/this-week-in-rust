Title: This Week in Rust 94
Number: 94
Date: 2015-08-31
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

# From the Blogosphere

* [The New MongoDB Rust Driver](https://www.mongodb.com/blog/post/the-new-mongodb-rust-driver).
* [Lock-freedom without garbage collection](https://aturon.github.io/blog/2015/08/27/epoch/).
* [SIMD in Rust](https://huonw.github.io/blog/2015/08/simd-in-rust/).
* [An Optimising BF Compiler](http://www.wilfred.me.uk/blog/2015/08/29/an-optimising-bf-compiler/). Writing a highly optimised Brainfuck compiler in Rust.
* [Another Year With Conrod](http://blog.piston.rs/2015/08/26/another-year-with-conrod/). An update on Conrod - a GUI written in Rust.
* [Learning Rust Modules](http://www.walkercoderanger.com/blog/2015/08/learning-rust-modules/) (from a C# developer's perspective).
* [Parser Combinator Experiments in Rust - Part 2: Error handling](https://m4rw3r.github.io/parser-combinator-experiments-errors/).

# New Releases & Project Updates

* [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison). A comparison of some web frameworks written in Rust.
* [eco](https://github.com/PistonDevelopers/eco). A tool for reasoning about breaking changes in Rust ecosystems.
* [mioco](https://github.com/dpc/mioco). Scalable, asynchronous IO coroutine-based handling for Rust (aka MIO COroutines).
* [cargo-vendor](https://github.com/alexcrichton/cargo-vendor). Cargo subcommand to vendor crates.io dependencies.
* [regex-dfa](https://github.com/jneem/regex-dfa). Proof of concept for fast regexes in Rust (a regex -> DFA compiler).
* [rust-gc](https://github.com/Manishearth/rust-gc). Simple tracing (mark and sweep) garbage collector for Rust.
* [torch](https://github.com/mrhooray/torch). Generate CPU FlameGraphs based on DWARF Debug Info.
* [libwebpki](https://github.com/briansmith/webpki). Web PKI Certificate Validation in Rust.
* [sokoban-rs](https://github.com/swatteau/sokoban-rs). An implementation of Sokoban in Rust.
* [urdict](https://github.com/sunng87/urdict). Command line client for Urban Dictionary.
* [power-assert-rs](https://github.com/gifnksm/power-assert-rs). Power Assert in Rust. Provides better assertion message.
* [rust-passert](https://github.com/manuel-woelker/rust-passert). Pretty/Power assertion macros for Rust.
* [colerr](https://github.com/dpc/colerr). Wrap a given process and colorize it's standard error output.
* [minesweeper-rs](https://github.com/Vinatorul/minesweeper-rs). Simple minesweeper in Rust.

# What's cooking on nightly?

104 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-08-24..2015-08-31

# New Contributors

* Adam Crume
* Artem Shitov
* Cesar Eduardo Barros
* Daan Rijks
* Jake Shadle
* Matěj Grabovský
* Michael Choate
* Nikolay Kondratyev
* Overmind JIANG
* Tim JIANG

# Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1212: Change all functions dealing with reading "lines" to treat both '\n' and '\r\n'](https://github.com/rust-lang/rfcs/pull/1212).
* [RFC 1242: Policy for rust-lang crates](https://github.com/rust-lang/rfcs/pull/1242).
* [RFC 1194: Add item recovery collection APIs](https://github.com/rust-lang/rfcs/pull/1194).
* [RFC 1236: Stabilize `catch_panic`](https://github.com/rust-lang/rfcs/pull/1236).

# Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-08-24..2015-08-31

* [Add the family of `[Op]Assign` traits to allow overloading assignment operations like `a += b`](https://github.com/rust-lang/rfcs/pull/953).
* [Implement raw fat pointer comparisons](https://github.com/rust-lang/rfcs/pull/1135).
* [Expand the `std::net` module to bind more low-level interfaces](https://github.com/rust-lang/rfcs/pull/1158).
* [Allow a `x...y` expression to create an inclusive range](https://github.com/rust-lang/rfcs/pull/1192).
* [Add `Box::leak` to leak `Box<T>` to `&'static mut T`](https://github.com/rust-lang/rfcs/pull/1233).
* [Forbid wildcard dependencies on crates.io](https://github.com/rust-lang/rfcs/pull/1241).

# New RFCs

* [Add a new `ToOpt` trait and have `bool` implement it](https://github.com/rust-lang/rfcs/issues/1265).
* [`macro_rules!` should support gensym for creating items](https://github.com/rust-lang/rfcs/issues/1266).
* [Implement `.drain(range)` and `.drain()` respectively as appropriate on collections](https://github.com/rust-lang/rfcs/pull/1257).
* [TRPLF] [Pauseless Concurrent Garbage Collector](https://users.rust-lang.org/t/rfc-pauseless-concurrent-garbage-collector/2624).

# Upcoming Events

* 9/2. Rust Berlin Hack & Learn.
* [9/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [9/15. San Diego Rust Meetup #8](http://www.meetup.com/San-Diego-Rust/events/224577039/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*"And God said, Noah you must transport these animals across a large body of water... but they are not Send. And Noah replied, I shall build a great Arc!"* — [durka42 on #rust](https://botbot.me/mozilla/rust/2015-08-28/?msg=48406438&page=23)

Thanks to [tomprogrammer](https://users.rust-lang.org/users/tomprogrammer) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
