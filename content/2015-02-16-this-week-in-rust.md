Title: This Week in Rust 70
Date: 2015-02-16
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

127 pull requests were [merged in the last week][merged], and 18 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-09..2015-02-16
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-09..2015-02-16

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* In types that implement `Drop`, [lifetimes must outlive the
  value][drop]. This will soon make it possible to safely implement
  `Drop` for types where `#[unsafe_destructor]` is now required. Read
  the [gorgeous RFC][drop-rfc] for details.
* The reserved keyword `be`, for invoking guaranteed tail calls, [has
  been replaced with `become`][be], but `be` will remain in our
  hearts. [RFC][be-rfc].
* [`Self` is a keyword][Self].
* `#[unsafe_no_drop_flag]` is now under the
  [`unsafe_no_drop_flag`][flag] feature gate.
* [Some float constants were renamed][float].
* [`std::failure` moved to `std::panicking`][fail]. This is an
  unstable module dealing with unwinding, not likely to cause
  breakage.

[be]: https://github.com/rust-lang/rust/pull/21918
[be-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0601-replace-be-with-become.md
[drop]: https://github.com/rust-lang/rust/pull/21972
[drop-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md
[Self]: https://github.com/rust-lang/rust/pull/22158
[flag]: https://github.com/rust-lang/rust/pull/22178
[float]: https://github.com/rust-lang/rust/pull/22254
[fail]: https://github.com/rust-lang/rust/pull/22347

## Other Changes

* Objects now have [default lifetime bounds][obj], so you don't have
  to write `Box<Trait+'static>` when you don't care about storing
  references. [RFC][obj-rfc].
* The new [`std::fs`][fs] and [`std::process`][proc] modules have arrived! They are much the
  same as the old but follow new conventions as described in [RFC 579] and [RFC 739].
* On Unix Rust will soon be [uninstallable][un] by running
  `/usr/local/lib/rustlib/uninstall.sh`.
* Richo Healey added lints for putting `#[no_mangle]` on [`const`s, as well as on private `static`s][no_mangle]. `#[no_mangle]` only makes sense for things with public symbol names.
* eddyb endured an [epic struggle][epic] with @bors to make borrows of constants `&'static`.

[obj]: https://github.com/rust-lang/rust/pull/22230
[obj-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0599-default-object-bound.md
[epic]: https://github.com/rust-lang/rust/pull/21744
[no_mangle]: https://github.com/rust-lang/rust/pull/21881
[fs]: https://github.com/rust-lang/rust/pull/21936
[proc]: https://github.com/rust-lang/rust/pull/22119
[`Path`]: http://doc.rust-lang.org/nightly/std/path/
[RFC 579]: https://github.com/rust-lang/rfcs/pull/579
[RFC 739]: https://github.com/rust-lang/rfcs/pull/739
[un]: https://github.com/rust-lang/rust/pull/22256

## New Contributors

* Duane Edwards
* Geoffrey Thomas
* Hugo van der Wijst
* Jorge Israel Peña
* Mátyás Mustoha
* Michael Budde
* Pierre Baillet
* Renato Alves
* Thiago Carvalho
* Tim Cuthbertson

## Approved RFCs

We'll catch up on RFCs next week.

# Friend of the Tree

The Rust Team likes to occassionally recognize people who have made outstanding contributions to The Rust Project, its ecosystem, and its community. These people are 'friends of the tree'.

This week's friend of the tree was ... Jonathan Reem (reem)

[Jonathan Reem] has been making an impact on Rust since May 2014. His primary contribution has been as the main author of the prominent Iron web framework, though he has also created several other popular projects including the testing framework stainless. His practical experience with these projects has led to several improvements in upstream Rust, most notably his complete rewrite of the TaskPool type. Reem is doing everything he can to advance the Rust cause.

[Jonathan Reem]: https://github.com/reem/

# Quote of the Week

*"You may be wondering whether this algorithm is correct. The answer is 'sort of'."* [@horse_rust] (after [Niko]).

[Niko]: https://github.com/rust-lang/rust/blob/ad2efdc67d871b18cc984eeb0b8d1d9b38daffb7/src/librustc/middle/infer/higher_ranked/README.md#shortcomings-and-correctness
[@horse_rust]: https://twitter.com/horse_rust/status/566801735750930432

# Notable Links

* [1.0 final timeline][fin]. Promises are made.
  [HN][fin-hn]. [/r/rust][fin-r-rust]. [/r/programming][fin-r-programming].
* The "Author of Unix in Rust abandons Rust in favor of Nim" headline
  spread to [/r/rust][nim-r-rust],
  [/r/programming][nim-r-programming], and
  [HackerNews][nim-hn]. Everybody stay calm!
* [What are Rust's exact auto-dereferencing rules?][deref] Valuable
  intel with which all rusticians should be equipped.
* [The incredible rewards of creating a programming
  language][reward]. Creating a programming language is fun. Join us
  and have fun.
* [Samsung OSG is hiring for Rust/Servo related work][osg].
* [Codius + Rust = ❤][codius]. Codius is creating some far-out smart
  contract things and Rust is part of the secure foundation.
* Hematite, our resident Minecraft clone from the Piston project,
  [made the HackerNews rounds][hema].
* [Duck typing in Piston][duck]. Piston is possessed of some radical
  notions.
* The ACM SIGMOD programming competition [welcomes Rust
  entries][sigmod].
* [License stats for crates.io][lic]. A surprisingly strong showing
  for the WTFPL.
* [Rust needs net/http][needs]. And net/http needs Rust.
* [First look at Cap'n Proto][capn]. Cap'n Proto for Rust is mega
  fast.
* [Getting Rusty][getting]. Watch people code in Rust.

[fin]: http://blog.rust-lang.org/2015/02/13/Final-1.0-timeline.html
[fin-r-rust]: https://www.reddit.com/r/rust/comments/2vsmy3/10_final_timeline/
[fin-hn]: http://news.ycombinator.com/item?id=9046526
[fin-r-programming]: https://www.reddit.com/r/programming/comments/2vsmzi/rust_10_final_timeline/
[nim-r-rust]: https://www.reddit.com/r/rust/comments/2vqy81/author_of_unix_in_rust_abandons_rust_in_favor_of/
[nim-r-programming]: https://www.reddit.com/r/programming/comments/2vvcbm/author_of_unix_in_rust_abandons_rust_in_favour_of/
[nim-hn]: http://news.ycombinator.com/item?id=9049698
[deref]: http://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules
[reward]: http://mikedrivendevelopment.blogspot.com/2015/02/the-incredible-rewards-of-creating.html
[osg]: http://mikedrivendevelopment.blogspot.com/2015/02/the-incredible-rewards-of-creating.html
[codius]: https://codius.org/blog/codius-rust/
[hema]: https://news.ycombinator.com/item?id=9023168
[duck]: http://blog.piston.rs/2015/02/10/duck-typing-in-piston/
[sigmod]: https://www.reddit.com/r/rust/comments/2w2yfo/acm_sigmod_2015_programming_contest_with_rust/
[lic]: https://www.reddit.com/r/rust/comments/2vxv8m/license_stats_for_crates_on_cratesio/
[needs]: https://www.reddit.com/r/rust/comments/2vnxng/rust_needs_nethttp/
[capn]: http://www.hoverbear.org/2015/02/12/capn-proto-in-rust/
[getting]: https://www.youtube.com/watch?v=mQZiDsGutJ8

# Project Updates

* [rust-cbor]. An implementation of [Concise Binary Object
  Representation][cbor].
* [xsv]. A CSV swiss-army knife.
* [rimd]. Library for working with MIDI.
* [rust-jack]. Bindings to the Jack low-latency Linux audio API.
* stomp-rs, an implementation of the [STOMP messaging
  protocol][stomp], has [matured a lot in 6 months][stomp-rs].
* [timely-dataflow]. An implementation of the low-latency dataflow
  model from [Naiad].
* [Trace Quest 5][tq5], the youtube series about building a raytracer
  in Rust, [wrapped up][tq5-final] its final episode.
* [rust-ptrace]: Wrapper for the `ptrace` syscall, from Codius.
* [bytes]. A crate for working with bytes!
* [RustAudio]. A collection of crates for audio processing, from the
  mind of mitchmindtree.
* [Helion]. An [Ambilight] clone for making your TV produce ambient
  lighting in concert with the video. Watch the [test
  video][helion-test].
* [rhex]. A hexagonal roguelike with a live instance you can telnet
  into.
* [comm]: An alternate implementation of channels.
* [This Week in Servo 23][twis]. With screenshots of Servo running on
  FxOS.
* [syntex]: Syntax extensions that work with stable Rust, from Erick
  Tryzelaar.

[rust-cbor]: https://github.com/BurntSushi/rust-cbor
[cbor]: https://tools.ietf.org/html/rfc7049
[xsv]: https://github.com/BurntSushi/xsv
[rimd]: https://github.com/nicklan/rimd
[rust-jack]: https://github.com/nicklan/rust-jack
[stomp]: https://stomp.github.io/
[stomp-rs]: https://github.com/zslayton/stomp-rs
[Naiad]: http://research.microsoft.com/en-us/projects/naiad/
[timely-dataflow]: https://github.com/frankmcsherry/timely-dataflow
[tq5]: https://www.youtube.com/playlist?list=PLMHbQxe1e9MlR80JVZCa0uJf9cz_PxlCY
[tq5-final]: https://www.reddit.com/r/rust/comments/2vva9y/trace_quest_5_season_1_results/
[rust-ptrace]: https://codius.org/blog/rust-ptrace-0-1-released/
[bytes]: https://github.com/carllerche/bytes
[RustAudio]: https://www.reddit.com/r/rust/comments/2vn0xx/rustaudio_a_collection_of_crates_for_audio_and/
[Helion]: https://github.com/bryal/Helion
[Ambilight]: https://en.wikipedia.org/wiki/Ambilight
[helion-test]: https://www.youtube.com/watch?v=3ZARz9ELfA4&feature=youtu.be
[rhex]: https://www.reddit.com/r/rust/comments/2vms2s/rhex_hexagonal_roguelike_in_rust/
[comm]: https://github.com/mahkoh/comm
[twis]: http://blog.servo.org/2015/02/10/twis-23/
[syntex]: http://erickt.github.io/blog/2015/02/09/syntex-syntex-extensions-for-rust-1-dot-0/

# Upcoming Events

* [February 16, Rust Paris][paris].
* [February 19, Rust Bay Area][sf]. Subject is I/O.
* [February 26, Rust NY][ny]. With Steve, Niko and Jack.

[paris]: http://www.meetup.com/Rust-Paris
[sf]: http://www.meetup.com/Rust-Bay-Area/events/219697152/
[ny]: http://www.meetup.com/Rust-NYC/
