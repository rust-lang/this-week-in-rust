Title: This Week in Rust 66
Date: 2015-01-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on Github](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

119 pull requests were [merged in the last week][merged].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-12..2015-01-18

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* `AtomicInt` and `AtomicUint` have been [renamed][atomic] to
  `AtomicIsize` and `AtomicUsize` to match their corresponding integer
  types.
* To fix a bug in coherence [builtin traits can only be implemented
  for structs and enums][cohere].

[atomic]: https://github.com/rust-lang/rust/pull/20896
[cohere]: https://github.com/rust-lang/rust/pull/21167

## Other Changes

* Certain long error messages of the form 'expected foo found bar' are
  now [split neatly across multiple lines][multiline]. Examples in the
  PR.
* UFCS method calls can now be [qualified by the trait][ufcs] of the
  method.  This can be used to disambiguate method calls when multiple
  applicable methods are in scope, e.g. `<i32 as Add<_>>::add(1, 2)`
  which is equivalent to `1.add(2)`. [RFC][rfcs-rfc].
* Negative impls are [partially implemented][negimpl], though appear
  to still be special-cased to the `Send` and `Sync`
  traits. [RFC][negimpl-rfc].
* Mutexes on Windows are faster now they are [implemented with Slim
  Reader Writer Locks][mutex].
* The `#[rustc_on_unimplemented]` attribute, requiring the
  'on_unimplemented' feature, lets rustc [display custom error
  messages when a trait is expected to be implemented for a type but
  is not][onun].
* [Preliminary support for PowerPC][powerpc].
* Fatal runtime errors are [now suitably boring][bore].

[multiline]: https://github.com/rust-lang/rust/pull/19870
[mutex]: https://github.com/rust-lang/rust/pull/20367
[onun]: https://github.com/rust-lang/rust/pull/20889
[negimpl]: https://github.com/rust-lang/rust/pull/20972
[negimpl-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md
[powerpc]: https://github.com/rust-lang/rust/pull/20980
[ufcs]: https://github.com/rust-lang/rust/pull/21077
[ufcs-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0132-ufcs.md
[bore]: https://github.com/rust-lang/rust/pull/20944

## New Contributors

* Aidan Hobson Sayers
* Alexander Korolkov
* Andrew Barchuk
* Cam Jackson
* CarVac
* Diggory Blake
* Fenhl
* Greg Chapple
* Jeff Belgum
* Lauri Lehmijoki
* nathan dotz
* Paul Crowley
* Seth Faxon
* Toni Cárdenas
* Travis Watkins

# Approved RFC's



# New RFC's



# Community

## From the Team

* [Weekly-meetings/2015-01-13][mtg]. fott; homu; integer overflow;
  I/O; 1.0; comment RFC

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-01-13.md

## Blog Posts

* [Thoughts about Rust from a D programmer][d]. Good
  comparison. [/r/rust][d-r-rust]. [/r/programming][d-r-programming].
* [Recent syntactic changes][syntax]. Nick discusses some of the
  last-minute syntax changes he's recently landed.
* [A Quick Comparison of Nim
  vs. Rust][nim]. [/r/rust][nim-r-rust]. [/r/programming][nim-r-programming].
* [Solving Project Eueler with Rust][euler]. A walk through of the
  first problem.
* [DTrace on Rust][dtrace].

[d]: http://blog.dicebot.lv/2015/01/thoughts-about-rust-from-d-programmer.html
[d-r-rust]: https://www.reddit.com/r/rust/comments/2s7bnt/thoughts_about_rust_from_d_programmer/
[d-r-programming]: https://www.reddit.com/r/programming/comments/2s70mm/thoughts_about_rust_from_a_d_programmer/
[nim-hn]: http://news.ycombinator.com/item?id=8883791
[nim-r-rust]: https://www.reddit.com/r/rust/comments/2sd5rv/a_quick_comparison_of_nim_vs_rust/
[nim-r-programming]: https://www.reddit.com/r/programming/comments/2scodb/a_quick_comparison_of_nim_vs_rust/
[syntax]: http://featherweightmusings.blogspot.co.nz/2015/01/recent-syntactic-changes-to-rust.html
[euler]: http://unlogic.co.uk/2015/01/12/solving-project-euler-with-rust-1/
[dtrace]: https://twitter.com/bcantrill/status/555143487482368000

## Videos

* [Servo: Building a Parallel Web Browser][servo]. Jack Moffit at
  linux.conf.au. [HN][servo-hn]. [/r/programming][servo-r-programming].

[servo]: https://youtu.be/7q9vIMXSTzc
[servo-hn]: https://news.ycombinator.com/item?id=8899812
[servo-r-programming]: https://www.reddit.com/r/programming/comments/2lcap0/an_introduction_to_servo/

## Discussions

* [Does Rust let you develop faster than C++?][fast]. TL;DR ¯\_(ツ)_/¯
* [Using Rust 1.0 for video game development][games]. Yes, please.
* [Rust support in Visual Studio][vs]. Doesn't exist yet, sorry.
* [Rust Berlin community efforts][berlin]. Berlin needs you!

[games]: https://www.reddit.com/r/rust/comments/2s4kp9/using_rust_10_for_video_game_development/
[vs]: https://www.reddit.com/r/rust/comments/2s5d65/rust_support_in_visual_studio/
[fast]: https://www.reddit.com/r/rust/comments/2sa2qx/does_rust_let_you_develop_faster_than_c/
[berlin]: https://www.reddit.com/r/rust/comments/2sbkuo/rustberlin_community_efforts/

## New Projects



## Project Updates

* [This Week in Servo 19][twis].
* [Worklog 2015-01-13: Time zones, chrono, and associated
  types]. Yurume discusses updates to rust-chrono.

[twis]: http://blog.servo.org/2015/01/13/twis-19/
[chrono]: https://lifthrasiir.github.io/rustlog/worklog-2015-01-13.html

## Upcoming Events

* [January 19 - Rust Paris](http://www.meetup.com/Rust-Paris)
* [January 20 - Rust Berlin](http://www.meetup.com/Rust-Berlin/events/219070839/)
* [January 28 - Rust Amsterdam](http://www.meetup.com/Rust-Amsterdam/events/218908906/)
