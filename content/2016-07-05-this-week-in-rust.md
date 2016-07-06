Title: This Week in Rust 137
Number: 137
Date: 2016-07-05
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

This week's edition was edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).

# Updates from Rust Community

## News & Blog Posts

* [State of Rust survey 2016](http://blog.rust-lang.org/2016/06/30/State-of-Rust-Survey-2016.html).
* [rust-brotli: Lossless compression with Brotli in Rust](https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/).
* [A toy front-end for LLVM, written in Rust](http://blog.ulysse.io/2016/07/03/llvm-getting-started.html).
* [First steps in Rust](https://floooh.github.io/2016/06/27/first-steps-in-rust.html). Early impressions of Rust from the perspective of a C, C++ and Python coder.
* [Why your first FizzBuzz implementation may not work](https://chrismorgan.info/blog/rust-fizzbuzz.html). An exploration into some initially surprising but great parts of Rust.
* [Attention! Span](https://llogiq.github.io/2016/06/28/span.html). Llogiq on `syntax::codemap::Span`, Rust's interface between macros and lints.
* [Implementing an IMAP client in Rust](https://insanitybit.github.io/2016/06/28/implementing-an-imap-client-in-rust).
* [podcast] [New Rustacean interview](http://www.newrustacean.com/show_notes/interview/_2/part_2/). Raph Levien on Rust's current strengths and places it can improve.
* [video] [Shar Bringer](https://www.youtube.com/watch?v=40DGf1eKb_Y). Demo of a video game written entirely in Rust. ([Discussion thread](https://www.reddit.com/r/rust_gamedev/comments/4qlftu/look_our_game_writen_entirely_in_rust/)).

## New Crates & Project Updates

* [Servo nightly builds are now available](https://blog.servo.org/2016/06/30/servo-nightlies/).
* [clap-rs can now generate Bash tab-completion scripts at compile time](https://kbknapp.github.io/clap-rs/clap/struct.App.html#method.gen_completions)
* [Thrussh](https://pijul.org/thrussh/). Portable SSH client and server library.
* [Rust Playground Reimplementation](https://github.com/integer32llc/rust-playground). An alternate implementation of the Rust Playground.
* [Rust on Tessel](https://github.com/tessel/tessel-rust). Example of using Rust development on Tessel 2.
* [S.U.P.E.R. Android Analyzer](https://github.com/Razican/super). An Android applications vulnerability analyzer written in Rust.
* [Takkerus](https://github.com/cdbfoster/takkerus). A Tak AI and board in the Rust language.
* [envy](https://github.com/softprops/envy). Deserialize env vars into typesafe structs.
* [This week in Rust docs 11](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-11).
* [This week in Ruma - July 03, 2016](https://www.ruma.io/news/this-week-in-ruma-2016-07-03/).
* [What’s coming up in imag (10)](http://beyermatthias.de/blog/2016/06/30/what-s-coming-up-in-imag-10/).

# Crate of the Week

This week's Crate of the Week is Kerosene2000's [reustmann](https://crates.io/crates/reustmann) a Von-Neumann Architecture written in Rust.
This is presumably useful as a base substrate to train genetic algorithms on. Thanks, Kerosene2000!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Make `-O` and `-C opt-level` override each other](https://github.com/rust-lang/rust/issues/7493#issuecomment-228892615).
* [easy] [rust: Fix error about multiple `--emit` targets](https://github.com/rust-lang/rust/issues/20130#issuecomment-228900232).
* [easy] [rust: Move some tests into run-pass-valgrind](https://github.com/rust-lang/rust/issues/21696).
* [moderate] [rust: Convert compiler-rt builtins to a Rust crate](https://github.com/rust-lang/rust/issues/34400#issuecomment-230059689).
* [moderate] [rust: Teach rustc to print CPU, etc. features](https://github.com/rust-lang/rust/issues/30961#issuecomment-228905399).
* [easy] [rustfmt: Overlong function signatures](https://github.com/rust-lang-nursery/rustfmt/issues/1049).
* [easy] [rustfmt: Overlong impl signatures](https://github.com/rust-lang-nursery/rustfmt/issues/1048).
* [easy] [rust-by-example: Add a Mutex chapter](https://github.com/rust-lang/rust-by-example/issues/105).
* [easy] [rust-by-exapmle: Add an Arc chapter](https://github.com/rust-lang/rust-by-example/issues/104).
* [easy] [rustup: Print new rustup version during `self update`](https://github.com/rust-lang-nursery/rustup.rs/issues/542).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

103 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-06-27..2016-07-04

* [Release Notes for 1.10](https://github.com/rust-lang/rust/pull/34591) (if you want to know early)
* [Beta is now more lenient with regards to Clang version](https://github.com/rust-lang/rust/pull/34589)
* [MultiDecorators are now a special case of MultiModifiers](https://github.com/rust-lang/rust/pull/34446)
  (also [some](https://github.com/rust-lang/rust/pull/33943) [cleanup](https://github.com/rust-lang/rust/pull/34459))
* [Methods that require `Self: Sized` are no longer in the vtable](https://github.com/rust-lang/rust/pull/34419) (breaking change, also on beta)
* [There can be only one...`Path`](https://github.com/rust-lang/rust/pull/34368) (plugin-breaking)
* [Paren-expressions now share their child node's IDs](https://github.com/rust-lang/rust/pull/34355) (simplifies lookup)
* [`ThinAttributes` are now a `ThinVec&lt;Attributes&gt;`](https://github.com/rust-lang/rust/pull/34339) (plugin-breaking)
* [`Cow`s by `Default` now own their target type's `Default`](https://github.com/rust-lang/rust/pull/34305)
* [Trait items can now be macro-expanded](https://github.com/rust-lang/rust/pull/34213)
* [MIR: Dominators (control flow graph)](https://github.com/rust-lang/rust/pull/34169)
* [Looking up hosts is now more picky about addresses](https://github.com/rust-lang/rust/pull/34067)
* [`HashMap`s now use SipHash-1-3 hasher by default](https://github.com/rust-lang/rust/pull/33940)
* [obligation errors now transitive](https://github.com/rust-lang/rust/pull/34605) (fix some memory leaks)
* [More robust metadata lock](https://github.com/rust-lang/rust/pull/34604) (also in beta)

## New Contributors

* Aaronepower
* Alexander Merritt
* CensoredUsername
* gnzlbg
* Jonathan L
* Jonathan Price
* Jupp Müller
* Peter Landoll
* Tatsuya Kawano
* Will Crichton
* 吴冉波

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Default and expanded errors for rustc](https://github.com/rust-lang/rfcs/pull/1644).
* [RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [Introduce more conventions around documenting Rust projects](https://github.com/rust-lang/rfcs/pull/1574).
* [Add space-friendly arguments](https://github.com/rust-lang/rfcs/pull/1509). Add `-C link-arg` and `-C llvm-arg` which allow you to pass along argument with spaces.
* [Add a used attribute to prevent symbols from being discarded](https://github.com/rust-lang/rfcs/pull/1459).
* [Refine the unguarded-escape-hatch from RFC 1238 (nonparametric dropck)](https://github.com/rust-lang/rfcs/pull/1327).

## New RFCs

* [Propose asserts](https://github.com/rust-lang/rfcs/pull/1662). This rfc proposes that the following macros be added: `assert_gt`, `assert_lt`, `assert_ge`, and `assert_le`.
* [Clarify behaviour when writing to a union field that implements Drop](https://github.com/rust-lang/rfcs/pull/1663).
* [Windows subsystem support](https://github.com/rust-lang/rfcs/pull/1665).
* [Extend the `Hasher` trait with `fn delimit` to support one-shot hashing](https://github.com/rust-lang/rfcs/pull/1666).

# Upcoming Events

* 7/6. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [7/6. Rust Cologne/Bonn: Open Night](http://www.meetup.com/Rust-Cologne-Bonn/events/232274957/).
* [7/6. Symfony User Group Cologne - Rust für PHP Entwickler](http://www.meetup.com/sfugcgn/events/232051942/?eventId=232051942).
* [7/7. Rust DC: Ownership and Borrowing](http://www.meetup.com/RustDC/events/231562147/).
* [7/11. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [7/12. (San Diego) Eat– Drink– Rust! Downtown Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/232039818/).
* 7/13. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [7/13. Rust Boulder/Denver - Hello, Rust!](http://www.meetup.com/Rust-Boulder-Denver/events/232328647/).
* 7/14. Rust release triage at #rust-triage on irc.mozilla.org.
* [7/14. Columbus Rust Society: Monthly Meeting](http://www.meetup.com/columbus-rs/events/231678481/).
* [7/18. Rust Paris Meetup #30](http://www.meetup.com/Rust-Paris/events/230111506/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Rust developer at The Blackbird](https://rust.jobboard.io/jobs/394482-rust-developer-at-the-blackbird).
* [Engineering positions at Zcash mention Rust](https://z.cash/blog/hiring.html).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
