Title: This Week in Rust 77
Date: 2015-04-13
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

104 pull requests were [merged in the last week][merged], and 6 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-06..2015-04-13
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-06..2015-04-13

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://rawgit.com/mrmonday/bitrust/gh-pages/index.html

# Breaking Changes

* [Set `CLOEXEC` for all fds on Unix by default](https://github.com/rust-lang/rust/pull/24034)
* [Convert lifetime shadowing into a hard error](https://github.com/rust-lang/rust/pull/24057)
* [Use discriminant_value intrinsic for `derive(PartialOrd)`](https://github.com/rust-lang/rust/pull/24270)

# Other Changes

* [Phil Dawes refactored a bunch of the parser to return `Result` insteadof panicking](https://github.com/rust-lang/rust/pull/23857)
* [Implemented remaining string pattern API](https://github.com/rust-lang/rust/pull/23952)
* [Implement reentrant mutexes and make stdio use them](https://github.com/rust-lang/rust/pull/24029)
* [Prefer ObjectCandidate to ImplCandidate if both apply](https://github.com/rust-lang/rust/pull/24056)
* [Michael Sproul added some more extended errors and improved their display](https://github.com/rust-lang/rust/pull/24143)
* [Implement io::Seek on BufReader and BufWriter](https://github.com/rust-lang/rust/pull/24176)
* [Allow plugins to register LLVM passes](https://github.com/rust-lang/rust/pull/24207)
* [Stabilize clone_from](https://github.com/rust-lang/rust/pull/24215)
* [Stabilize Error::from_raw_os_error](https://github.com/rust-lang/rust/pull/24216)

# New Contributors

* Ben Ashford
* Christopher Chambers
* Dominick Allen
* Hajime Morrita
* Igor Strebezhev
* Josh Triplett
* Luke Gallagher
* Michael Alexander
* Michael Macias
* Oak
* Remi Rampin
* Sean Bowe
* Tibor Benke
* Will Hipschman
* Xue Fuqiao

# Approved RFCs

* [RFC 218: Empty struct with braces](https://github.com/rust-lang/rfcs/pull/218)
* [RFC 639: discriminant_value intrinsic](https://github.com/rust-lang/rfcs/pull/639)
* [RFC 888: Compiler fences](https://github.com/rust-lang/rfcs/blob/master/text/0888-compiler-fence-intrinsics.md)
* [RFC 911: Const functions and inherent methods](https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn.md)

# New RFCs

* [Expand the scope of `std::fs`](https://github.com/rust-lang/rfcs/pull/1044)
* [Rename `soft_link` to `symlink`](https://github.com/rust-lang/rfcs/pull/1048)
* [Clarify cast rules, especially regarding fat pointers](https://github.com/rust-lang/rfcs/pull/1052)
* [Rename or replace `str::words`](https://github.com/rust-lang/rfcs/pull/1054)
* [Redirecting stdio of child processes to open file handles](https://github.com/rust-lang/rfcs/pull/1055)
* [Add `Sync` to `io::Error`](https://github.com/rust-lang/rfcs/pull/1057)
* [Replace `tail`/`init`](https://github.com/rust-lang/rfcs/pull/1058)

# Notable Links

* [Fearless concurrency with Rust](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)
* [Featherweight musings: arrays and vectors in Rust](http://featherweightmusings.blogspot.com/2015/04/new-tutorial-arrays-and-vectors-in-rust.html).
* [Newcomer to Rust: my experience](http://internals.rust-lang.org/t/newcomer-to-rust-my-experience/1816/1)
* [Memcpy is
  backwards](http://internals.rust-lang.org/t/memcpy-is-backwards/1797). There
  was a big silently-breaking change to `copy_memory` right before the
  beta.
* [Bay Area Rust Meetup: Data Science](https://air.mozilla.org/bay-area-rust-meetup-april-2015/). Video from the SF meetup.
* [Weekly-meetings/2015-04-07][mtg]: beta, abs, rustdoc, wiki, docs
* [Steve Klabnik and Yehuda Katz talk about Rust on The Changelog #151](https://thechangelog.com/151/)
* [Aaron Turon's Stanford talk (video)](https://www.youtube.com/watch?v=O5vzLKg7y-k).
* [A page of useful links for new contributors](http://www.ncameron.org/rust.html).

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-04-07.md

# Project Updates

* [Racer progress update 5 (cargo support)](http://phildawes.net/blog/2015/04/05/racer5/).
* [multirust](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html). The Rust toolchain manager, updated with new features and bugfixes.
* [reforge](https://github.com/tedsta/reforge). A multiplayer sandbox space combat MMO.
* [The Hat Backup System](https://github.com/google/hat-backup).
* [Piston 1.0 is released](http://blog.piston.rs/2015/04/07/piston-0.1/).
* [gag](https://crates.io/crates/gag). Redirect stderr/stdout.
* [afl.rs](https://github.com/kmcallister/afl.rs). Integrating American Fuzzy-Lop with Rust.
* [hyper on beta](http://seanmonstar.com/post/115873169212/hyper-on-beta).
* [Raft: Small status update](http://hoverbear.org/2015/04/09/raft-the-next-generation-3/).
* [This Week in Servo 30](http://blog.servo.org/2015/04/09/twis-30/).
* [A Chef comunity cookbook for Rust](https://supermarket.chef.io/cookbooks/rustlang).
* [A Docker image for Rust](https://registry.hub.docker.com/u/jimmycuadra/rust/).
* [Rust_Classifier](https://github.com/jackm321/Rust_Classifier). A naive Bayes classifier.
* [Pool](https://github.com/carllerche/pool). A pool for reusable values, from carllerche.
* [ggp-rs](https://www.reddit.com/r/rust/comments/3272b8/ggprs_a_library_for_creating_general_game_players/). A library for [General Game Playing](https://class.coursera.org/ggp-003).
* [timer](https://www.reddit.com/r/rust/comments/326x5p/a_betacompatible_timer/). A timer to make up for the lack of one in std.
* [retry](https://github.com/jimmycuadra/retry). Retry an operation until a condition is satisfied.
* [query_rs](https://github.com/Mr-Byte/query_rs). LINQ-like macros.
* [analit](https://github.com/jswrenn/analit). 'Analog' literals for geometric types.
* [Google APIs for Rust - Dev Diary #1: How to write 78 APIs in 5s (video)](https://youtu.be/2U3SpepKaBE).
* [All crates of the RustAudio project work with beta](https://www.reddit.com/r/rust/comments/32b74a/all_rustaudio_crates_now_build_on_the_stable/).
* [eventual_io](https://github.com/carllerche/eventual_io). Async I/O with mio and eventual.
* [serde, the serialization library, is beta-compatible](https://erickt.github.io/blog/2015/04/12/serde-0-dot-3-1-now-compatible-with-beta/).
* [nickel, the web framework, is on crates.io](https://www.reddit.com/r/rust/comments/32dhwb/nickel_is_on_cratesio/).
* [coroutine-rs](https://github.com/rustcc/coroutine-rs). Coroutines.
* [rust-sdl2 is beta-compatible](https://github.com/AngryLawyer/rust-sdl2/commit/00f7df56570b2e7e57df63813692bb7ef53d10a0).

# Upcoming Events

* [4/13 Seattle](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)
* [4/14 Sydney](http://www.meetup.com/Rust-Sydney/events/221388677/)
* [4/20 Paris](http://www.meetup.com/Rust-Paris)
* [4/22 Columbus Rust Society](http://www.meetup.com/columbus-rs/). Their inaugural event.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

```
<frankmcsherry> rust is like a big bucket of solder and wire, with the promise that you can't electrocute yourself.
```

From #rust.

Thanks to BurntSushi for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
