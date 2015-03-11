Title: This Week in Rust 73
Date: 2015-03-09
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

123 pull requests were [merged in the last week][merged], and 4 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-02..2015-03-09
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-02..2015-03-09

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* [Rename `should_fail` to `should_panic`][panic].
* [Implement arithmetic overflow changes][overflow].
* [Audit `core::num` module for `int`/`uint`][num].
* [Feature gate `static_assert`][assert].

[panic]: https://github.com/rust-lang/rust/pull/21824
[overflow]: https://github.com/rust-lang/rust/pull/22532
[num]: https://github.com/rust-lang/rust/pull/22600
[assert]: https://github.com/rust-lang/rust/pull/22960

## Other Changes

* [Implement stdio for `std::io`][stdio].
* [Debugging code inlined from the standard library should work correctly][dbg].
* [Stabilize portions of the `io` module][io].
* [Stabilize the `process` module][process].
* [Stabilize the `ffi` module][ffi].
* [The Grand Metadata Reform][meta]. lifthrasiir reduces binary sizes
   by 27% by improving metadata encoding.
* [Make `#[derive(Anything)]` desugar to
  `#[derive_Anything]`][derive]. Feature-gated.

[stdio]: https://github.com/rust-lang/rust/pull/22797
[dbg]: https://github.com/rust-lang/rust/pull/22235
[process]: https://github.com/rust-lang/rust/pull/22882
[meta]: https://github.com/rust-lang/rust/pull/22971
[ffi]: https://github.com/rust-lang/rust/pull/22975
[io]: https://github.com/rust-lang/rust/pull/23010
[derive]: https://github.com/rust-lang/rust/pull/23137

## New Contributors

* Alan Cutter
* Amol Mundayoor
* awlnx
* Camille TJHOA
* Chloe
* Daniel Lobato García
* David King
* Eric Platon
* Ivan Radanov Ivanov
* Łukasz Niemier
* Matt Cox
* Paul Osborne
* Pyry Kontio

## Approved RFCs

* [RFC 495: Array pattern adjustments][rfc-495].
* [RFC 574: Replace Vec::drain by a method that accepts a range parameter][rfc-574].
* [RFC 640: Debug improvements][rfc-640].

[rfc-495]: https://github.com/rust-lang/rfcs/blob/master/text/0495-array-pattern-changes.md
[rfc-574]: https://github.com/rust-lang/rfcs/blob/master/text/0574-drain-range.md
[rfc-640]: https://github.com/rust-lang/rfcs/blob/master/text/0640-debug-improvements.md

## New RFCs

* [Tweaks to std::net address types][net].
* [Relax the ExactSizeIterator trait][relax].
* [Extract math from Float trait][math].
* [Split the Copy trait into two traits][copy].
* [`if let` multiple patterns][ifwhile].
* [The `{:?}` format specifier should not print values in release builds][not].
* [Disallow hyphens in crate names][disallow].
* [Macro escape char][char].
* [Retire RFC 8 (intrinsics) without implementing][retire].
* [Add a newtype keyword][newtype].
* [Ammend I/O reform RFC to address issues with flush][flush].
* [Don't allow numeric types with fallback in varargs position][var].
* [Overloaded compound assingment][ass].
* [Specify that bool is compatible with _Bool][bool].
* [Statementize looping forms][loop].
* [Unsafe comparison traits][cmp].

[cmp]: https://github.com/rust-lang/rfcs/pull/956
[loop]: https://github.com/rust-lang/rfcs/pull/955
[bool]: https://github.com/rust-lang/rfcs/pull/954
[ass]: https://github.com/rust-lang/rfcs/pull/953
[var]: https://github.com/rust-lang/rfcs/pull/951
[flush]: https://github.com/rust-lang/rfcs/pull/950
[newtype]: https://github.com/rust-lang/rfcs/pull/949
[retire]: https://github.com/rust-lang/rfcs/pull/948
[char]: https://github.com/rust-lang/rfcs/pull/944
[disallow]: https://github.com/rust-lang/rfcs/pull/940
[not]: https://github.com/rust-lang/rfcs/pull/938
[ifwhile]: https://github.com/rust-lang/rfcs/pull/937
[copy]: https://github.com/rust-lang/rfcs/pull/936
[math]: https://github.com/rust-lang/rfcs/pull/925
[relax]: https://github.com/rust-lang/rfcs/pull/924
[net]: https://github.com/rust-lang/rfcs/pull/923

# Friend of the Tree

The Rust Team likes to occassionally recognize people who have made
outstanding contributions to The Rust Project, its ecosystem, and its
community. These people are 'friends of the tree'.

This week's friend of the tree was ... Manish Goregaokar!

Manish started working on Servo as part of the GSoC program in 2014, where he implemented XMLHttpRequest. Since then he's become in integral part of the Servo team while finishing his university studies and organizing Rust community events. In 2015 he took an interest in bors' queue and started making rollup PRs to accelerate the integration process. Nursing the PR queue is the kind of time-consuming labor that creates friends of the tree like Manish, the rollup friend of the tree.

# Notable Links

* [Weekly-meetings/2015-03-03][mtg]. fott, filling drop, type ascription.
* [Starting off in Rust: Trying to write a shell][shell].
* [When should my type be Copy?][copy].
* [Optimizing by default][opt].
* [Completing rustfmt and the Rust style guidelines][rustfmt]. A call to arms.
* [Mozilla rejected for Google Summer of Code 2015][gsoc].
* [Getting aquainted with MIO][mio]. mio is the unix async I/O library by Carl Lerche.
* [MaidSafe discusses Rust vs. C++][maidsafe]. Security-minded
  projects are eyeing us intently.
* [Shrinking Rust Distribution for Fun and Profit][shrink].
* [Rust, Travis, and GutHub Pages][travis]. Some advice on
  administering Rust projects.
* [Mozilla Research is hiring][hire].
* [Rust homepage over time][home]. More nostalgia.

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-03-03.md
[home]: https://www.reddit.com/r/rust/comments/2ydfok/rust_homepage_over_time/
[hire]: https://www.reddit.com/r/rust/comments/2yackc/rustrelated_job_openings_at_mozilla_research/
[travis]: http://www.hoverbear.org/2015/03/07/rust-travis-github-pages/
[shrink]: https://lifthrasiir.github.io/rustlog/worklog-2015-03-06.html
[copy]: https://www.reddit.com/r/rust/comments/2xxjda/when_should_my_type_be_copy/
[maidesafe]: https://www.reddit.com/r/rust/comments/2xwe4r/maidsafe_discusses_rust_vs_c/
[mio]: http://www.hoverbear.org/2015/03/04/getting-acquainted-with-mio/
[shell]: http://blog.achernya.com/2015/03/starting-off-in-rust-trying-to-write.html
[gsoc]: https://www.reddit.com/r/rust/comments/2xr57s/mozilla_rejected_for_google_summer_of_code_2015/
[opt]: http://internals.rust-lang.org/t/optimizing-by-default/1532/1
[rustfmt]: http://internals.rust-lang.org/t/completing-rustfmt-and-the-rust-style-guidelines/1685/11

# Project Updates

* [cargo-build]. Compile to JS with Emscripten.
* [rust-snake, emscripten, and WebGL][snake]. rust-snake is built on
  [Piston], and it runs on the web.
* [Device abstraction in gfx-rs][gfx]. gfx-rs was recently rearchitected to
  support more graphics backends than just OpenGL.
* [gstreamer1.0-rs]. Bindings to GStreamer.
* [rust-pcre]. Recently updated but needs a maintainer.
* [synth]. A polyphonic Synth type whose multiple oscillators generate
  sound via amplitude and frequency envelopes, implemented in Rust.
* [pcg]. A Rust implementation of the PCG PRNG.
* [rust-quicklook]. An OS X QuickLook plugin for Rust source files.
* [marching-squares]. A demo of [noise-rs] plus [rustbox].
* [This Week in Servo 26][twis].
* [Benchmark: Rust/nom VS Haskell/attoparsec VS C/hammer][bench]. [TL;DR][bench-tldr] the [nom]
  parser combinator is very fast.
* [tessel]. A modular hardware prototyping board that will support Rust.
* [fdringbuf-rs]. Ringbuffer with fd signalling - fast IPC without memory copies.
* [ecr-rs: (Ab)using macros to get rid of unsafe code and reduce runtime checks][ecr].
* [Binary Turk]. A chess engine.
* [rusty-tags]. create ctags/etags for Cargo projects.
* [podio]. Write integers and floats in the specified endianness.
* [rust-tcl]. Embed TCL in Rust.
* [oyashio]. Single-producer, multiple-consumer channels, built with [rust-promise].
* [spinlock.rs]. A spinlock!

[spinlock.rs]: https://www.reddit.com/r/rust/comments/2yg4l1/a_spinlock_implementation_in_rust/
[rust-promise]: https://github.com/viperscape/rust-promise
[oyashio]: https://github.com/viperscape/oyashio
[rust-tcl]: https://github.com/AngryLawyer/rust-tcl
[podio]: http://mvdnes.github.io/podio/podio/index.html
[rusty-tags]: https://www.reddit.com/r/rust/comments/2yc37l/rustytags_create_ctagsetags_for_a_cargo_project/
[Binary Turk]: https://github.com/theemathas/binary_turk
[ecr]: http://heroesgravedevelopment.tumblr.com/post/112919710664
[fdringbuf-rs]: https://github.com/diwic/fdringbuf-rs
[tessel]: https://www.reddit.com/r/rust/comments/2y2enz/new_microcontroller_that_aims_to_support_rust_as/
[nom]: https://github.com/Geal/nom
[bench-tldr]: https://github.com/Geal/nom_benchmarks#after-some-optimizations
[bench]: https://www.reddit.com/r/rust/comments/2y0bas/benchmark_rustnom_vs_haskellattoparsec_vs_chammer/
[twis]: http://blog.servo.org/2015/03/04/twis-26/
[Piston]: http://pistondevelopers.github.io/
[snake]: https://www.reddit.com/r/rust_gamedev/comments/2yjkn8/rustsnake_emscripten_and_webgl/
[rustbox]: https://github.com/gchp/rustbox
[noise-rs]: https://github.com/bjz/noise-rs
[marching-squares]: https://github.com/crespyl/marching-squares
[rust-quicklook]: https://github.com/yingDev/rust-quicklook
[pcg]: https://github.com/codahale/pcg
[cargo-build]: http://users.rust-lang.org/t/rust-to-js-with-emscripten/587/4
[gstreamer1.0-rs]: http://users.rust-lang.org/t/gstreamer-bindings/591
[rust-pcre]: http://users.rust-lang.org/t/pcre-crate-in-rust/553
[gfx]: https://gfx-rs.github.io/2015/03/01/device.html
[synth]: https://www.reddit.com/r/rust/comments/2xruhg/synth_a_polyphonic_synth_type_whose_multiple/

# Upcoming Events

* [3/16. Rust Paris](http://www.meetup.com/Rust-Paris)
* [3/18. Copenhagen Tech Polyglot Meetup](http://www.meetup.com/Copenhagen-Tech-Polyglots/events/220800093/)
* The Bay Area meetup for March has been cancelled due to scheduling difficulties.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*"Fear not, this is Rust, not some scruffy loosely-typed, garbage-collected, non-blocking language!"*

[Andrew Hobden, on getting acquainted with mio][mio].

[mio]: http://www.hoverbear.org/2015/03/04/getting-acquainted-with-mio/

Thanks to Johan Sigfrids for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

