Title: This Week in Rust 132
Number: 132
Date: 2016-05-30
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

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts

* 🎈🎉 [Announcing Rust 1.9](http://blog.rust-lang.org/2016/05/26/Rust-1.9.html). 🎉🎈
* [The Path to Rust](https://thesquareplanet.com/blog/the-path-to-rust/). Why Rust might be right for you.
* [How do I convert a `&str` to a `String` in Rust](https://mgattozzi.github.io/2016/05/26/how-do-i-str-string.html)? A series of "How do I do X in Rust?" articles aimed at beginners.
* [Catching Exceptions](http://os.phil-opp.com/catching-exceptions.html). Part of the series [Writing an OS in Rust](http://os.phil-opp.com/).
* [Connecting a webservice to a database in Rust](http://hermanradtke.com/2016/05/23/connecting-webservice-database-rust.html).
* [Unsafe abstractions](http://smallcultfollowing.com/babysteps/blog/2016/05/23/unsafe-abstractions/). Understanding `unsafe` code and the idea of unsafety boundaries.
* [The 'Tootsie Pop' model for unsafe code](http://smallcultfollowing.com/babysteps/blog/2016/05/27/the-tootsie-pop-model-for-unsafe-code/). Niko Matsakis outlines a high-level approach to defining a memory model. Follow-up to "Unsafe abstractions".
* [Async generators](https://dwrensha.github.io/capnproto-rust/2016/05/28/async-generators.html). Making a case for generators based async I/O in Rust.
* [Using Wayland from Rust, Part 2](http://blog.levans.fr/rust_wayland_2-en.html).
* [Using Rust 1.8 stable for building embedded firmware](https://spin.atomicobject.com/2016/05/25/rust-1-8-embedded-firmware/).
* [Attempting to use Rust's type system for statically checked dependency tracking](https://michaelwoerister.github.io/2016/05/28/attempting-to-use-rusts-type-system-for-statically-check-dependency-tracking.html).
* [This week in Servo 64](https://blog.servo.org/2016/05/23/twis-64/).
* [This week in Rust docs 5](https://www.reddit.com/r/rust/comments/4kqlsk/this_week_in_rust_docs_5/).
* [This week in Parity 2](https://ethcore.github.io/twip/content/2016-05-25.html).
* [This week in Ruma 2016-06-29](https://www.ruma.io/news/this-week-in-ruma-2016-05-29/).
* [podcast] [Rust with Steve Klabnik](http://softwareengineeringdaily.com/2016/05/24/rust-steve-klabnik/).
* [The Rust community](http://www.suspectsemantics.com/blog/2016/05/28/the-rust-community/). What it gets right and what it gets wrong.

## New Crates & Project Updates

* [Redox OS: Survey Results](https://docs.google.com/forms/d/1uDndu1eU_KHQdB_OgQfKrWUJ4F5RaVH-X9WnLnygmJo/viewanalytics).
* [bindgen 0.17.0](https://github.com/crabtw/rust-bindgen), a C binding generator. Lots of new features and fixes, with a much cleaner generated code. See the [changelog](https://github.com/crabtw/rust-bindgen/blob/0.17/Changelog.md) for a complete list.
* [wsta](https://github.com/esphen/wsta/). A CLI development tool for WebSocket APIs.
* [keygen](https://github.com/xsznix/keygen). An algorithm for generating optimal keyboard layouts.
* [apply_attr](https://github.com/regexident/apply_attr). A syntax extension providing higher-order attributes to Rust.
* [bytevec](https://github.com/fero23/bytevec). A Rust serialization library that uses byte vectors.

# Crate of the Week

*Sadly, there was no suggestion for this week's Crate of the Week.*

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Participate in 2016 State of Rust Survey](http://blog.rust-lang.org/2016/05/09/survey.html).
* [easy] [clippy: Participate in Rust Clippy Survey](https://docs.google.com/forms/d/1k0wuWgGwDhuUL3q_cONGVxQ6PJSYq5JRZOHKc0itLbg/viewform?c=0&w=1).
* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [medium] [rustup: Make `rustup default x86_64-unknown-linux-gnu` do something smarter](https://github.com/rust-lang-nursery/rustup.rs/issues/411).
* [easy] [servo: Implement more `DOMParser::parseFromString` arguments](https://github.com/servo/servo/issues/11505).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

90 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-05-23..2016-05-30

* [abort with SIGABRT instead of SIGILL on Linux](https://github.com/rust-lang/rust/pull/31457) (breaking change!)
* [process `cfg_attr(..)`s during macro expansion](https://github.com/rust-lang/rust/pull/33706) (breaking change!)
* [allow `concat_idents!(..)` in type position](https://github.com/rust-lang/rust/pull/33735)
* [don't expand erroneous macros](https://github.com/rust-lang/rust/pull/33713) (reduces duplicate error messages)
* [1.10 stabilizations](https://github.com/rust-lang/rust/pull/33699)
* [MIR dataflow fixes](https://github.com/rust-lang/rust/pull/33667) (changed 22 files, merged on first try. Kudos, pnkfelix!)
* [deprecate `f32`/`f64.abs_sub(_)`](https://github.com/rust-lang/rust/pull/33664)
* [remove ExplicitSelf from AST](https://github.com/rust-lang/rust/pull/33644) (plugin-breaking change)
* [implement `..` in tuple (struct) patterns](https://github.com/rust-lang/rust/pull/33639) (plugin-breaking, [RFC #1492](https://github.com/rust-lang/rfcs/blob/master/text/1492-dotdot-in-patterns.md))
* [HIR spans for loop labels](https://github.com/rust-lang/rust/pull/33351) (plugin-breaking)
* [speed up unicode property lookup](https://github.com/rust-lang/rust/pull/33098)
* [refactor autoderef obligation handling](https://github.com/rust-lang/rust/pull/33852) (fixes epic number of issues)
* [make `EscapeUnicode` an `ExactSizeIterator`](https://github.com/rust-lang/rust/pull/33849)
* [use `memalign` instead of `posix_memalign` on older Androids](https://github.com/rust-lang/rust/pull/33832) (yay for portability)
* [cleanup macro expansion](https://github.com/rust-lang/rust/pull/33766)
* [fix FFI argument handling](https://github.com/rust-lang/rust/pull/33872)
* [fix ANTLR grammar verification script](https://github.com/rust-lang/rust/pull/33860) (though there's a newer yacc grammar)
* [`fmt::Error` now implements the `Error` trait](https://github.com/rust-lang/rust/pull/33856)
* [speed up `Ipv4Addr` comparisons](https://github.com/rust-lang/rust/pull/33891)

## New Contributors

* Aaklo Xu
* Alexander Polyakov
* Carlo Teubner
* Daniel Firth
* kennytm
* Sebastian Thiel
* Srinivas Reddy Thatiparthy
* Tamir Bahar
* Ty Coghlan

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

* [Remove the one-type-only restriction on `format_args!` arguments](https://github.com/rust-lang/rfcs/pull/1618).
* [Standardise stream wrappers like compression, encryption](https://github.com/rust-lang/rfcs/pull/1568).
* [Change thread local variables to only accept async-signal-safe types](https://github.com/rust-lang/rfcs/pull/1379).
* [Implement new methods for checked and wrapping casts for potentially lossy integer conversions](https://github.com/rust-lang/rfcs/pull/1218).
* [Normalization for long error codes explanations](https://github.com/rust-lang/rfcs/pull/1567).
* [Add a `lifetime` specifier to `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1590).

## New RFCs

* [Eager expansion of macros](https://github.com/rust-lang/rfcs/pull/1628).
* [Replace synchronization primitives with those from parking_lot](https://github.com/rust-lang/rfcs/pull/1632).
* [Make `mem::uninitialized()` and `mem::zeroed()` `const fn`](https://github.com/rust-lang/rfcs/pull/1633).

# Upcoming Events

* 6/1. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [6/1. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [6/6. Cologne / Germany: Rust Anniversary Meetup - Part II](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/231135785/).
* 6/8. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [6/8. Rust Berlin Meetup](http://www.meetup.com/Rust-Berlin/events/231188250/).
* [6/8. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [6/9. Columbus Rust Society](http://www.meetup.com/columbus-rs/events/230812780/).
* [6/13. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [6/14. Eat – Drink – Rust! Downtown Rust Meetup (San Diego)](http://www.meetup.com/San-Diego-Rust/events/231356534/)

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

> Rust is like doing parkour while suspended on strings & wearing protective gear. Yes, it will sometimes look a little ridiculous, but you'll be able to do all sorts of cool moves without hurting yourself.

— [llogiq on reddit](https://www.reddit.com/r/rust/comments/4l44z3/why_should_i_use_rust/d3k7ayi).

Thanks to [aochagavia](https://users.rust-lang.org/users/aochagavia) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
