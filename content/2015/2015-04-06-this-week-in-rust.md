Title: This Week in Rust 76
Date: 2015-04-06
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

This week's issue covers the previous *two* weeks.

# What's cooking on master?

266 pull requests were [merged in the last *two* weeks][merged], and 10 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-23..2015-04-06
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-03-23..2015-04-06

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://rawgit.com/mrmonday/bitrust/gh-pages/index.html

# Breaking Changes

* [Update orphan and overlap rules for RFC 1023][orphan].
* [Lex binary and octal literals more eagerly][lex].
* [Remove IteratorExt][it].
* [Move `thread_local` into `thread`][thread].
* [Stabilize `std::convert` and related code][convert].
* [Stabilize the rest of Any/BoxAny][any].
* [Remove FromError, use From<E> instead][error].
* [Don't reborrow the target of a `write!()`][write].
* [Remove variances from traits and deprecate `PhantomFn`/`MarkerTrait`][var].
* [Feature gate rust-call ABI][rustcall].
* [Stabilize basic timeout functionality][timeout].
* [Change the meaning of the count to splitn][splitn].

[splitn]: https://github.com/rust-lang/rust/pull/23951
[timeout]: https://github.com/rust-lang/rust/pull/23949
[rustcall]: https://github.com/rust-lang/rust/pull/23948
[var]: https://github.com/rust-lang/rust/pull/23938
[write]: https://github.com/rust-lang/rust/pull/23934
[error]: https://github.com/rust-lang/rust/pull/23879
[any]: https://github.com/rust-lang/rust/pull/23876
[convert]: https://github.com/rust-lang/rust/pull/23875
[lex]: https://github.com/rust-lang/rust/pull/23872
[orphan]: https://github.com/rust-lang/rust/pull/23867
[thread]: https://github.com/rust-lang/rust/pull/23557
[it]: https://github.com/rust-lang/rust/pull/23300

# Other Changes

* [Add associated types support to #[derive(..)]][assoc].
* [Entry API v3: replace Entry::get with Entry::default and Entry::default_with][entry].
* [Add `std::net::lookup_addr` for reverse DNS lookup][look].
* [Add support for `extern crate foo as bar`][extern].
* [Add net::IpAddr, destabilize lookup_host][ip].
* [Arc::try_unique][try_unique], [Rename Arc::try_unique to get_mut][try_harder_unique].
* [Add (unstable) FnBox trait as a nicer replacement for `Thunk`][fnbox].
* [Re-add min_value, max_value methods][min].
* [Work toward a non-panicking parser][pan].

[pan]: https://github.com/rust-lang/rust/pull/23857
[min]: https://github.com/rust-lang/rust/pull/23947
[fnbox]: https://github.com/rust-lang/rust/pull/23939
[try_unique]: https://github.com/rust-lang/rust/pull/23844
[try_harder_unique]: https://github.com/rust-lang/rust/pull/24053
[ip]: https://github.com/rust-lang/rust/pull/23711
[extern]: https://github.com/rust-lang/rust/pull/23546
[look]: https://github.com/rust-lang/rust/pull/23419
[entry]: https://github.com/rust-lang/rust/pull/22930
[assoc]: https://github.com/rust-lang/rust/pull/21237

# New Contributors

* Adenilson Cavalcanti
* Alex Quach
* Andrew Hobden
* Augusto Hack
* bcoopers
* Camille Roussel
* Carlos Galarza
* Ches Martin
* Dan Callahan
* Dan W.
* Darin Morrison
* Drew Crawford
* Emeliov Dmitrii
* Germano Gabbianelli
* github-monoculture
* Huachao Huang
* Jordan Woehr
* Julian Viereck
* kgv
* Liam Monahan
* Mark Mossberg
* Nicholas Mazzuca
* Or Neeman
* ray glover

# Approved RFCs

* [RFC 879: Lex binary and octal literals more eagerly][rfc-879].
* [RFC 888: Compiler fences][rfc-888].
* [RFC 911: Const functions and inherent methods][rfc-911].
* [RFC 979: Align the count parameter of splitn with other languages][rfc-979].
* [RFC 1011: Add std::process::exit][rfc-1011].
* [RFC 1023: Rebalancing coherence][rfc-1023].

[rfc-879]: https://github.com/rust-lang/rfcs/pull/879
[rfc-888]: https://github.com/rust-lang/rfcs/pull/888
[rfc-911]: https://github.com/rust-lang/rfcs/pull/911
[rfc-979]: https://github.com/rust-lang/rfcs/pull/979
[rfc-1011]: https://github.com/rust-lang/rfcs/pull/1011
[rfc-1023]: https://github.com/rust-lang/rfcs/pull/1023

# New RFCs

* [Generic boolean operators](https://github.com/rust-lang/rfcs/pull/1008).
* [Don't panic when stdout doesn't exist](https://github.com/rust-lang/rfcs/pull/1014).
* [Add read_into_buf and get_buf to BufRead](https://github.com/rust-lang/rfcs/pull/1015).
* [Change abs to return unsigned integers](https://github.com/rust-lang/rfcs/pull/1017).
* [Automatically derive {Clone,PartialEq,PartialOrd} when {Copy,Eq,Ord} are derived](https://github.com/rust-lang/rfcs/pull/1028).
* [1.0 prelude additions](https://github.com/rust-lang/rfcs/pull/1030).
* [RFC: Rename `Iterator::size_hint` to `Iterator::len_hint`](https://github.com/rust-lang/rfcs/pull/1034).
* [Duration](https://github.com/rust-lang/rfcs/pull/1040).
* [Explicit multi-char case changes](https://github.com/rust-lang/rfcs/pull/1042).

# Notable Links

* [Rust 1.0.0 beta is here](http://blog.rust-lang.org/2015/04/03/Rust-1.0-beta.html).
* [Newcomer to Rust: My experience](http://internals.rust-lang.org/t/newcomer-to-rust-my-experience/1816).
* [Use rust-gdb and rust-lldb for improved debugging](http://michaelwoerister.github.io/2015/03/27/rust-xxdb.html).
* [Proposal: spend most of the beta period *not* writing new features](http://internals.rust-lang.org/t/proposal-spend-most-of-the-beta-period-not-writing-new-features/1770).
* [TDD example in Rust with Prime Factors kata](http://carol-nichols.com/2015/03/28/tdd-example-in-rust/).
* [Rust FFI: Embedding Rust in projects for safe, concurrent and fast code anywhere](http://oppenlander.me/articles/rust-ffi).
* [Weekly-meetings/2015-03-31][mtg]: unqualified associated items,
  feature-gating unsigned negation, trivial cast lint, trait variance,
  coherence updates, as and overflow, beta.
* [Rust docs on GitHub pages via Travis](https://github.com/kmcallister/travis-doc-upload/blob/master/README.md).
* [Experimenting with Rust on iOS](http://jakerr.github.io/rust/ios/2015/04/02/experimenting-with-rust-ios.html).
* [The Sunset of C and C++][cpp].
* [Collective noun for Rust programmers](https://www.reddit.com/r/rust/comments/31envv/bikeshed_collective_noun_for_rust_programmers/).
* [Creating a PHP extension in Rust](http://jaredonline.svbtle.com/creating-a-php-extension-in-rust).
* [Why did you stop using Rust][why]? Trick question - it's impossible to stop using Rust.
* [Patina: A Formalization of the Rust Programming Language](ftp://ftp.cs.washington.edu/tr/2015/03/UW-CSE-15-03-02.pdf).
* [Servo shell on Ubuntu 15.04 (video)](https://www.youtube.com/watch?v=9tWPkvh1wVk).
* [Modeling graphs in Rust using vector indices](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/).
* [More Rust compared to C++](https://rnestler.github.io/more-rust-compared-to-c.html).
* [rust-lang/rust at 10,000 stars](https://i.imgur.com/gTDPBtF.png).
* [Go vs. Rust vs. D vs. Crystal](https://www.reddit.com/r/programming/comments/31mzu1/go_vs_rust_vs_d_vs_crystal_etc_perlin_noise/).

[why]: https://www.reddit.com/r/rust/comments/31he9f/why_did_or_will_stop_using_rust/
[cpp]: http://blog.biicode.com/c-cpp-biggest-threats-modern-ecosystem/
[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-03-31.md

# Project Updates

* A [rendering of Ifrit][ifrit] in kvarkus's [Claymore].
* [Lightmapping and movement][lm] in XMPPWocky's [vel0city].
* [balisong] is raytracing some 3D voxels.
* [Mindjuice], a two-stage configurable brainfuck interpreter library.
* [The Steam store rendered in Servo](https://www.reddit.com/r/rust/comments/30blul/the_steam_store_rendered_in_servo/).
* [llvm-sys]. Independent bindings to LLVM.
* [contain-rs]. A new collections library that replaces collect-rs.
* [RustNN]. Neural networks.
* [rust-unityscopes]. Rust bindings to Unity scopes.
* [Firefox Gecko and Servo rendering some sites](https://www.youtube.com/watch?v=pZGhnqtXVdc).
* [github-rust]. Client for the GitHub API v3
* clog, a changelog generator, [has updated to version 0.3.1](https://crates.io/crates/clog).
* [eventual]. Futures and streams.
* [serde 0.3.0, erickt's new serialization library, was released](https://www.reddit.com/r/rust/comments/310t6d/serde_generic_serialization_framework_v030/).
* [SolidOak, the IDE in Rust, for Rust, now has binaries for Linux and OS X](https://www.reddit.com/r/rust/comments/313zxz/solidoak_binaries_for_linux_and_os_x_now_available/).
* [delivery-cli]. A CLI for Chef.
* [servo-shell]. An HTML-based UI for Servo.
* [This Week in Servo 29](http://blog.servo.org/2015/04/02/twis-29/).
* [Mozilla's Rust-based Servo browser engine inches forward](http://www.infoworld.com/article/2905688/applications/mozillas-rust-based-servo-browser-engine-inches-forward.html).
* [capstone-rs]. Rust bindings to the [Capstone disassembly framework](http://capstone-engine.org/).
* [rust-smallvec]. Servo's small vector types.

[rust-smallvec]: https://github.com/servo/rust-smallvec
[capstone-rs]: https://github.com/richo/capstone-rs
[servo-shell]: https://twitter.com/adamhjk/status/578627590996496384
[delivery-cli]: https://github.com/chef/delivery-cli
[eventual]: https://github.com/carllerche/eventual
[github-rust]: https://github.com/GlenDC/github-rust
[rust-unityscopes]: https://launchpad.net/rust-unityscopes
[RustNN]: https://github.com/jackm321/RustNN
[contain-rs]: https://www.reddit.com/r/rust/comments/30vn1d/rip_collectrs_longlive_containrs/
[llvm-sys]: https://www.reddit.com/r/rust/comments/30gbs0/llvm_sys_librustcfree_bindings_to_llvm/
[Mindjuice]: https://github.com/daboross/mindjuice-rs
[balisong]: https://github.com/ivanceras/balisong
[lm]: https://www.reddit.com/r/rust_gamedev/comments/30aqb3/vel0city_lightmapping_movement/
[vel0city]: https://www.reddit.com/r/rust_gamedev/comments/30aqb3/vel0city_lightmapping_movement/
[ifrit]: https://raw.githubusercontent.com/kvark/claymore/master/etc/screens/3-model-scene.jpg
[Claymore]: https://github.com/kvark/claymore

# Upcoming Events

* [4/9 Bay Area Rust - Data Science](http://www.meetup.com/Rust-Bay-Area/events/220627544/).
* [4/13 Seattle Rust](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [4/14 Rust Sydney](http://www.meetup.com/Rust-Sydney/events/221388677/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

"Diagnostics are the UX of a compiler, and so they're deserving of the
exact same care that is put into mobile app or Web design."

[Declared by pcwalton on Hacker
News](https://news.ycombinator.com/item?id=9330518).

Thanks to sanxiyn for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
