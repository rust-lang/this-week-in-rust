Title: This Week in Rust 134
Number: 134
Date: 2016-06-13
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

This week's edition was edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts

* [Baby steps: Slowly porting musl to Rust](http://blog.adamperry.me/rust/2016/06/11/baby-steps-porting-musl-to-rust/).
* [Why Rust for Low-level Linux programming](http://groveronline.com/2016/06/why-rust-for-low-level-linux-programming/)?
* [Lambda crabs (part 1): A mathematical introduction to lifetimes and regions](https://ticki.github.io/blog/lambda_crabs_1/).
* [Lambda crabs (part 2): Region inference is (not) magic](https://ticki.github.io/blog/lambda_crabs_2/).
* [Lambda crabs (part 3): Region-based alias analysis](https://ticki.github.io/blog/lambda_crabs_3/).
* [Interior mutability in Rust: what, why, how](https://ricardomartins.cc/2016/06/08/interior-mutability)?
* [Heterogeneous linked lists in Rust](http://src.codes/typed-linked-lists.html).
* [Procedural macros vs. macros](https://llogiq.github.io/2016/06/11/expand.html).
* [Beware upcoming Dyon v0.8 - It will be awesome](http://blog.piston.rs/2016/06/07/beware-dyon/)! Dyon is a dynamically typed programming language written in Rust.
* [Sandboxing code in Rust](https://insanitybit.github.io/2016/06/11/sandboxing-code-in-rust).
* [Recordings & slides from Rust Anniversary – Part II meetup](http://rustaceans.cologne/2016/06/06/rust-anniversary-part-2.html). Featuring [alexcrichton](https://github.com/alexcrichton), [llogiq](https://github.com/llogiq), and [flanfly](https://github.com/flanfly).
* [Rust out your C](https://github.com/carols10cents/rust-out-your-c-talk). [Carol](https://github.com/carols10cents) talks about her experience in rewriting [Zopfli](https://github.com/google/zopfli) from C to Rust.

## New Crates & Project Updates

* [Rust Project changelog for 2016-06-10](https://users.rust-lang.org/t/rust-project-changelog-for-2016-06-10/6181).
  Updates to bitflags, libc, nano-config, regex, rustfmt, rust-mode,
  rustup, rust.vim, sublime-rust, uuid, www.rustbyexample.com,
  www.rust-lang.org.
* [error-chain](https://users.rust-lang.org/t/announcing-error-chain-a-library-for-consistent-and-reliable-rust-error-handling/6133),
  a library for consistent and reliable error handling. Initial release.
* [rust-skeptic 0.5 released](https://users.rust-lang.org/t/rust-skeptic-test-your-rust-markdown-documentation-via-cargo/2163/2?u=brson).
  A tool for testing Rust documentation.
* [mayda](https://github.com/harharkh/mayda). A Rust library to compress integer arrays.
* [rust-metrics](https://github.com/posix4e/rust-metrics). Multi reporter metrics library.
* [process-viewer](https://github.com/GuillaumeGomez/process-viewer). A process viewer GUI in Rust.
* [strfmt](https://github.com/vitiral/strfmt). Rust library for formatting dynamic strings.
* [This week in Servo 66](https://blog.servo.org/2016/06/06/twis-66/).
* [This week in Rust docs 7](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-7).
* [This week in Ruma 2016-06-05](https://www.ruma.io/news/this-week-in-ruma-2016-06-05/).

# Crate of the Week

This week's Crate of the Week is [petgraph](https://crates.io/crates/petgraph), which provides graph structures and algorithms. Thanks to [/u/diwic](https://reddit.com/user/diwic) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

110 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-06-06..2016-06-13

* [Implement RFC #495 for slice patterns](https://github.com/rust-lang/rust/pull/32202) ([RFC text](https://github.com/rust-lang/rfcs/blob/master/text/0495-array-pattern-changes.md))
* [Rust now requires LLVM3.7 or newer](https://github.com/rust-lang/rust/pull/34104) (3.6 was also [removed from wrappers](https://github.com/rust-lang/rust/pull/34178))
* [Outdated -Z gc and -Z count_type_sizes no longer exist](https://github.com/rust-lang/rust/pull/34124) (potential script-breaking change)
* [Some MIR edge case fixes](https://github.com/rust-lang/rust/pull/34128)
* [ABI return casts now always use memcpy](https://github.com/rust-lang/rust/pull/34141) (will be in next beta)
* [MIR now supports 16-bit pointers](https://github.com/rust-lang/rust/pull/34174)
* [Visitors now visit Statement and Expression Attributes](https://github.com/rust-lang/rust/pull/34199)
* [Support `#[macro_use]` on macro-expanded crates](https://github.com/rust-lang/rust/pull/34032)
* [Skip `#[test]`-annotated elements during non-test build](https://github.com/rust-lang/rust/pull/34002)
* [MIR no longer double-rounds float consts](https://github.com/rust-lang/rust/pull/34006) (also TIL that NaNs are signed)
* [Decorators are now run post-expansion](https://github.com/rust-lang/rust/pull/34010)
* [Old follow checking removed](https://github.com/rust-lang/rust/pull/33982) (So we'll get errors instead of warnings for things like `$($x:expr),* ...` in macros)
* [MIR scopes are now more similar to the lexical structure](https://github.com/rust-lang/rust/pull/33989)

## New Contributors

* Andrew Brinker
* Chris Tomlinson
* Hendrik Sollich
* Horace Abenga
* Jacob Clark
* Jakob Demler
* James Alan Preiss
* James Lucas
* Joachim Viide
* Mark Côté
* Mathieu De Coster
* Michael Necio
* Morten H. Solvang
* Wojciech Nawrocki

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

* [Implement new methods for checked and wrapping casts for potentially lossy integer conversions](https://github.com/rust-lang/rfcs/pull/1218).
* [Change thread local variables to only accept async-signal-safe types](https://github.com/rust-lang/rfcs/pull/1379).
* [Add a initial, minimal form of `impl Trait`](https://github.com/rust-lang/rfcs/pull/1522).
* [Normalization for long error codes explanations](https://github.com/rust-lang/rfcs/pull/1567).
* [Standardise stream wrappers like compression, encryption](https://github.com/rust-lang/rfcs/pull/1568).
* [Add a `lifetime` specifier to `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1590).
* [Remove the one-type-only restriction on `format_args!` arguments](https://github.com/rust-lang/rfcs/pull/1618).

## New RFCs

* [Add `CStr::with_ptr` and deprecate `CStr::as_ptr`](https://github.com/rust-lang/rfcs/pull/1642).
* [Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [Default and expanded errors for rustc](https://github.com/rust-lang/rfcs/pull/1644).
* [Add Cortex-M targets to the compiler + binary releases of `core`](https://github.com/rust-lang/rfcs/pull/1645).
* [Add `&move` pointers, the `DerefMove` trait, and the unsafe `DerefPure` traits](https://github.com/rust-lang/rfcs/pull/1646).
* [Allow `Self` to appear in the where clause of trait impls](https://github.com/rust-lang/rfcs/pull/1647).

# Upcoming Events

* [6/14. Eat – Drink – Rust! Downtown Rust Meetup (San Diego)](http://www.meetup.com/San-Diego-Rust/events/231356534/)
* 6/15. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [6/15. Rust Los Angeles Monthly Meetup - Hack Night](http://www.meetup.com/Rust-Los-Angeles/events/231587506/).
* [6/16. London Rust Meetup](http://www.meetup.com/Rust-London-User-Group/events/231332388/).
* [6/17. Rhein-Main Rust Regulars' Table](http://www.meetup.com/Rust-Rhein-Main/events/231344035/).
* 6/22. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 6/23. Rust release triage at #rust-triage on irc.mozilla.org.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

>> Isn’t rust too difficult to be widely adopted?

> I believe in people.

— [Steve Klabnik on TRPLF](https://users.rust-lang.org/t/isnt-rust-too-difficult-to-be-widely-adopted/6173/2)

Thanks to [Steven Allen](https://users.rust-lang.org/users/stebalien) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
