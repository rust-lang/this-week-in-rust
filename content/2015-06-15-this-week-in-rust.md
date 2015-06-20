Title: This Week in Rust 83
Date: 2015-06-15
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

This week's edition was edited by: Brian Anderson.

# Notable Links

* [A graph-based higher-order intermediate representation](http://compilers.cs.uni-saarland.de/papers/lkh15_cgo.pdf). A novel intermediate representation for a dialect of Rust.
* [Interview with Mozilla's Aaron Turon](http://www.pl-enthusiast.net/2015/06/09/interview-with-mozillas-aaron-turon/).
* [Out of curiosity - Who is currently focusing on improving compile times? If so, how is it coming along?](http://www.reddit.com/r/rust/comments/39f21l/out_of_curiosity_who_is_currently_focusing_on/). People are working on compile time.
* [Hodor esolang as a Rust macro](http://www.reddit.com/r/rust/comments/39wvrm/hodor_esolang_as_a_rust_macro/).
* [dispatcher](https://github.com/timonv/rdispatcher). Many-to-many channels.
* ['When you have no room for error'](http://www.commitstrip.com/en/2015/06/15/when-you-have-no-room-for-error/). The CommitStrip comic mentions Rust.
* [Fundraiser for a multi-language (incl. Rust) teaching tent at Chaos Communication Camp](https://www.betterplace.org/en/projects/30076-an-assembly-on-chaos-communication-camp-providing-free-teaching).

# Project Updates

* [Homu, a gatekeeper for your commits](http://homu.io/). Barosl is now offering homu-as-a-service. This is the CI system Rust and Servo both use, and it is compatible with Travis CI.
* [rustorm](http://www.reddit.com/r/rust/comments/395hwl/ivancerasrustorm_a_simple_orm_for_rust/). A simple ORM.
* [snake-piston](http://www.reddit.com/r/rust/comments/398azz/snake_game_in_rust_using_piston/). A snake game written with Piston.
* [yaml-rust](http://chyh1990.github.io/yaml-rust/). A pure-Rust YAML 1.2 parser.
* [crust](http://www.reddit.com/r/rust/comments/39elgj/crust_reliable_p2p_network_connections_in_rust/). Reliable p2p with NAT traversal.
* [aho-corasick](https://github.com/BurntSushi/aho-corasick). Fast multi-substring nmatching.
* [pcapng-rs](https://github.com/richo/pcapng-rs). A [pcapng](https://github.com/pcapng/pcapng) parser written with [nom](https://github.com/Geal/nom).
* [deuterium](https://github.com/deuterium-orm/deuterium). A fully-typed SQL query builder.
* [regex](https://github.com/rust-lang/regex/pull/91). It got faster. A lot faster.

# What's cooking on master?

160 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-06-07..2015-06-15

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [Prevent raw pointers from being used as explicit
  self](https://github.com/rust-lang/rust/pull/26225). Probably nobody
  has ever tried to write such code, and the current implemented
  behavior is bonkers.

# Other Changes

* [Add `Result::expect`](https://github.com/rust-lang/rust/pull/25359/files).
* [Add
  `CString::from_ptr/into_ptr`](https://github.com/rust-lang/rust/pull/25777). For
  transfering ownership of C strings across the FFI.
* [Implement `str::split_at`](https://github.com/rust-lang/rust/pull/25839).
* [Stabilize a number of new `fs` features](https://github.com/rust-lang/rust/pull/25844).
* [Parallel code generation works
  again](https://github.com/rust-lang/rust/pull/26018). Pass `rustc -C
  codegen-units=4` to try.
* [The `to_uppercase` and `to_lowercase` functions now support complex
  case mapping](https://github.com/rust-lang/rust/pull/26039). This
  changes the behavior of the *stable*
  `char::to_uppercase/to_lowercase` and also stabilizes
  `str::to_uppercase/to_lowercase`.
* [Implement `Extend<&T> where: T: Copy` for a variety of collection
  types](https://github.com/rust-lang/rust/pull/25989).
* [The unstable `String::from_str` is
  deprecated](https://github.com/rust-lang/rust/pull/26077). Use
  `String::from`.
* [Heuristics for detecting identifier typos are improved](https://github.com/rust-lang/rust/pull/26087).

# New Contributors

* ben fleis
* David Voit
* Eli Friedman
* frankamp
* funkill
* Johann Tuffe
* joliv
* Joshua Landau
* Leo Correa
* marcell
* Marcel Müller
* Matthew Astley
* Nathan Long
* Nick Fitzgerald
* Russell McClellan
* saml
* simplex
* swgillespie

# Approved RFCs

* [RFC 1105. Policy on API
  evolution](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md). Describes
  how the Rust project evaluates changes to the libraries, and which are allowed when.
* [RFC 1119. `Result::expect`](https://github.com/rust-lang/rfcs/pull/1119).
* [RFC 1122. Semantic
  versioning](https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md). Describes
  how the language itself is allowed to change.
* [RFC 1123. Introduce `str::split_at`](https://github.com/rust-lang/rfcs/blob/master/text/1123-str-split-at.md)
* [RFC 1131. `likely` intrinsic](https://github.com/rust-lang/rfcs/blob/master/text/1131-likely-intrinsic.md). For hinting hot and cold branches.

# New RFCs

* [Adjust default object
bounds](https://github.com/rust-lang/rfcs/pull/1156). This fixes some
dumb rules that made it into 1.0, but is a breaking change that
affects relatively little code.
* [Expand the `std::net` module](https://github.com/rust-lang/rfcs/pull/1158).

# Upcoming Events

* [6/17. Montreal](http://www.meetup.com/Montreal-Rust-Language-Meetup/events/223045701/)
* [6/17. Los Angeles](http://www.meetup.com/Rust-Los-Angeles/events/222656434/)
* [6/24. Columbus Rust Society](http://www.meetup.com/columbus-rs/)
* [6/29. Sydney](http://www.meetup.com/Rust-Sydney/events/222811456/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

```text
<Quxxy> I had a fun one in cargo script: there's currently no way in Rust
to get a file's mtime and the system time in the same time format
<Quxxy> (On Windows)
<Quxxy> You can get one in UNIX time, the other in Windows time
<Quxxy> Which have different scales and different epochs
<Quxxy> Rust: Buy Your Own Damn Batteries; What Are You, A Communist?
```

Quxxy discovers Rust's stance toward the inclusion of batteries.

Thanks to cmr for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
