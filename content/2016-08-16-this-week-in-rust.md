Title: This Week in Rust 143
Number: 143
Date: 2016-08-16
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

# Updates from Rust Community

## News & Blog Posts

- [Security Advisory for crates.io, 2016-08-15](https://users.rust-lang.org/t/security-advisory-for-crates-io-2016-08-15/6907). Please read, especially if you've renamed your GitHub account.
- [Shape of errors to come](https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html). A sneak peek at new & improved error formats in Rust.
- [What’s new with “The Rust Programming Language”](http://words.steveklabnik.com/whats-new-with-the-rust-programming-language)? Second edition of TRPL book is coming up with [Carol (Nichols || Goulding)](https://github.com/carols10cents) as a co-author.
- [Zero-cost futures in Rust](https://aturon.github.io/blog/2016/08/11/futures/). futures-rs is now ready for prime time!
- [Futures in Rust](http://www.ishbir.com/post/2016-08-14-futures-in-rust/). Writing an Async Web API Wrapper - An excercise in learning Rust.
- [Rust compiler walk-through: Introduction](https://gchp.ie/2016/08/09/rust-compiler-walkthrough-introduction/).
- [Asynchronous servers in Rust](https://gkbrk.com/2016/08/asynchronous-servers-in-rust/). Using tokio-rs to build an asynchronous server.
- [Parsing strategies in Rust](https://willcrichton.net/notes/parsing-strategies-in-rust/). Comparing two competing parsing frameworks in Rust: nom, a parser combinator, and LALRPOP, an LR(1) parser generator.
- [Xero and Sandstorm.io have been added to the Friends of Rust page](https://www.rust-lang.org/en-US/friends.html). Xero is using Rust for infrastructure, and Sandstorm.io has a [collections application](https://sandstorm.io/news/2016-08-09-collections-app) in Rust.

## New Crates & Project Updates

- [error-chain 0.5.0 released](https://users.rust-lang.org/t/announcing-error-chain-a-library-for-consistent-and-reliable-rust-error-handling/6133/21), changing how backtraces are handled.
- [rust-skeptic 0.6.1 released](https://users.rust-lang.org/t/rust-skeptic-test-your-rust-markdown-documentation-via-cargo/2163/4), with fixes and optimizations.
- The official [nano-config](https://github.com/rust-lang/nano-config) repo is _undeprecated_. It now contains a configuration that is compatible with older versions of the editor.
- [Way Cooler](https://github.com/Immington-Industries/way-cooler). Customizable Wayland compositor (window manager) written in Rust.
- [_ring_](https://github.com/briansmith/ring). Safe, fast, small crypto using Rust.
- [ralloc](https://github.com/redox-os/ralloc). A fast & memory efficient pure-Rust memory allocator.
- [Threshold Secret Sharing](https://github.com/snipsco/rust-threshold-secret-sharing).  A pure-Rust implementation of various threshold secret sharing schemes.
- [WS-RS](https://github.com/housleyjk/ws-rs). Lightweight, event-driven WebSockets for Rust.
- [alexa-rs](https://github.com/neil-lobracco/alexa-rs). Rust library for building Alexa skills.
- [Tantivy](https://github.com/fulmicoton/tantivy). A text search engine library written in Rust.
- [filters](https://github.com/matthiasbeyer/filters) - a crate for building predicates/filters with the builder pattern.
- [derive_builder](https://github.com/colin-kiegel/rust-derive-builder). Derive builder implementation for Rust structs.
- [This week in Servo 74](https://blog.servo.org/2016/08/08/twis-74/).
* [This week in Rust docs 17](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-17).
- [TiKV weekly 2016-08-12](http://www.pingcap.com/tikv/2016/08/12/tikv-weekly/).
- [Talking Tock week 1](http://www.tockos.org/blog/2016/talking-tock-1/). Tock is a safe, multitasking operating system for low-power, low-memory microcontrollers.
- [Talking Tock week 2](http://www.tockos.org/blog/2016/talking-tock-2/).
- [What's coming up in imag 13](http://beyermatthias.de/blog/2016/08/12/what-s-coming-up-in-imag-13/).

# Crate of the Week

This week's Crate of the Week is Raph Levien's [font-rs](https://github.com/google/font-rs), yet another pure Rust font renderer, which is incomplete, but *very* fast. Thanks [StefanoD](https://users.rust-lang.org/users/StefanoD) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [moderate] [rust-www: Add a section to the front page with 3 rotating 'friends'](https://github.com/rust-lang/rust-www/issues/477).
  This is an important change to how we present Rust that everybody will see.
* [easy] [rust: Error code list which need to be updated to new format](https://github.com/rust-lang/rust/issues/35233).
* [easy] [servo: Do not define Pipeline::setup_common when on Windows](https://github.com/servo/servo/issues/12856).
* [easy] [rustup: Don't capture backtraces without RUST_BACKTRACE=1](https://github.com/rust-lang-nursery/rustup.rs/issues/591#issuecomment-236235677).
  An easy fix, done in two steps, first modifying error-chain, then upgrading it in rustup.
* [easy] [rustup: Add command to install shell
  completions](https://github.com/rust-lang-nursery/rustup.rs/issues/387#issuecomment-234675568).
* [moderate] [rust: improve error message when resolution via Deref
  actually required
  DerefMut](https://github.com/rust-lang/rust/issues/28419). Good
  first type system bug.
* [hard] [imag: implement bindings to lua/lisp (ketos)/rhai for the filter
  library](https://github.com/matthiasbeyer/imag/issues/245)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

135 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-08-08..2016-08-15

* [`impl trait` in return type position](https://github.com/rust-lang/rust/pull/35091) Yay! @eddyb, you're a hero!
* [It is now an error to use private items in public code](https://github.com/rust-lang/rust/pull/34206) (formerly a warning)
* [Also, `private_in_public` checker now substitutes type aliases](https://github.com/rust-lang/rust/pull/34193)
* [`--test-threads=`N argument for tests](https://github.com/rust-lang/rust/pull/35414)
* [Slow test warning](https://github.com/rust-lang/rust/pull/35405)
* [Extend MIR to emit LLVM lifetime statements](https://github.com/rust-lang/rust/pull/35409)
* [MIR: new statement kind for enum deaggregation](https://github.com/rust-lang/rust/pull/35348)
* [Incremental Compilation: Fixed some ICEs](https://github.com/rust-lang/rust/pull/35166)
* [Strict Version Hashes for Crates improved](https://github.com/rust-lang/rust/pull/35079)
* [Improved {H,C}ashing for dep-graphs](https://github.com/rust-lang/rust/pull/35406)
* [Some artificial restrictions regarding zero-sized structs/enum variants lifted](https://github.com/rust-lang/rust/pull/35138) (RFC 1506)
* [Introduce `as_slice`/`as_mut_slice` methods on `std::vec::IntoIter` struct](https://github.com/rust-lang/rust/pull/35447)
* [Optimize `std::panic::catch_unwind(_)` slightly](https://github.com/rust-lang/rust/pull/35444) (don't use it anyway unless you have to)
* [`impl From<T> for` {ε, `Ref`, `Unsafe`}`Cell<T>`](https://github.com/rust-lang/rust/pull/35392)...
* [`impl FromIterator<_> for Cow<str>`](https://github.com/rust-lang/rust/pull/35064)
* [Macro expansion, expanded](https://github.com/rust-lang/rust/pull/34811)
* [Macros: hygienic metavariables](https://github.com/rust-lang/rust/pull/35453)
* [`binary_search_by_key(..)` now more flexible with specified lifetime](https://github.com/rust-lang/rust/pull/34762) (what a small lifetime annotation can do)
* [Being smart about concatenating TokenStreams](https://github.com/rust-lang/rust/pull/35539)
* [Unchanged github repos are no longer re-downloaded](https://github.com/rust-lang/cargo/pull/2974) (uses github API instead)
* [Test improvements for emscripten port](https://github.com/rust-lang/rust/pull/35574)
* [New errors (+ JSON mode) now active by default](https://github.com/rust-lang/rust/pull/35401) Yay! And kudos, Jonathan!
* [Better error messages on missing parenthesis when calling fields](https://github.com/rust-lang/rust/pull/35456) (e.g. `(x.y)()`)
* [Better {`&`, `*`}`ptr` printing in error messages](https://github.com/rust-lang/rust/pull/35611)
* Another large batch of changes to error messages


## New Contributors

* Andrii Dmytrenko
* Cameron Hart
* Cengiz Can
* Chiu-Hsiang Hsu
* Christophe Vu-Brugier
* Clement Miao
* crypto-universe
* Felix Rath
* hank-der-hafenarbeiter
* José manuel Barroso Galindo
* Krzysztof Garczynski
* Luke Hinds
* Marco A L Barbosa
* Mark-Simulacrum
* Matthew Piziak
* Michael Gattozzi
* Patrick McCann
* Pietro Albini
* ShyamSundarB
* srdja
* Stephen Lazaro

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1643: Dedicated strike team to resolve unsafe code guidelines](https://github.com/rust-lang/rfcs/pull/1643).
* [RFC 1607: RFC process for formatting style and Rustfmt defaults](https://github.com/rust-lang/rfcs/pull/1607).
* [RFC 1683: Create a team responsible for documentation for the Rust project](https://github.com/rust-lang/rfcs/pull/1683).
* [RFC 1581: `FusedIterator` marker trait and `iter::Fuse` specialization](https://github.com/rust-lang/rfcs/pull/1581).
* [RFC 1649: Add extra access methods for atomic types](https://github.com/rust-lang/rfcs/pull/1649).
* [RFC 1576: Add a `literal` fragment specifier for `macro_rules!` patterns that matches literal constants](https://github.com/rust-lang/rfcs/pull/1576).
* [RFC 1506: Clarify the relationships between various kinds of structs and variants](https://github.com/rust-lang/rfcs/pull/1506).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Allow deriving `Deref` and `DerefMut`](https://github.com/rust-lang/rfcs/pull/1694).
* [Procedural macros 1.1](https://github.com/rust-lang/rfcs/pull/1681).
* [Add "panic-safe" or "total" alternatives to the existing panicking indexing syntax](https://github.com/rust-lang/rfcs/pull/1679).
* [Add `checked_sub()` already known from various primitive types to the `Duration` struct](https://github.com/rust-lang/rfcs/pull/1640).
* [Omit `'static` lifetimes to reference or generics lifetime values in `static` or `const` declarations](https://github.com/rust-lang/rfcs/pull/1623).
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).
* [Specify Rust compatibility of nursery crates](https://github.com/rust-lang/rfcs/pull/1619).
* [Define a best practices procedure for making bug fixes in the compiler](https://github.com/rust-lang/rfcs/pull/1589).
* [Add `parse_generics!` and `parse_where!` macros](https://github.com/rust-lang/rfcs/pull/1583).
* [Support code generators with source maps and multiple source directories](https://github.com/rust-lang/rfcs/pull/1573).
* [Macro naming and modularisation](https://github.com/rust-lang/rfcs/pull/1561).
* [Propose `Interior<T>` data-type, to allow moves out of the dropped value during the drop hook](https://github.com/rust-lang/rfcs/pull/1180).

## New RFCs

* [Use `#[link(kind)]` to fix imports from native libs on Windows](https://github.com/rust-lang/rfcs/pull/1717).
* [Add "meta tags" to Rust documentation conventions and to the rustdoc tool](https://github.com/rust-lang/rfcs/pull/1713).
* [Add 'else match' blocks to if expressions](https://github.com/rust-lang/rfcs/pull/1712).

# Upcoming Events

* [8/17. Boston Rust Meetup: Hack Night](http://www.meetup.com/BostonRust/events/233260730/).
* 8/17. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [8/17. Rust Los Angeles Meetup](https://www.meetup.com/Rust-Los-Angeles/events/232933613/).
* 8/24. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 8/25. Rust release triage at #rust-triage on irc.mozilla.org.
* [8/29. Rust Sthlm: Rust on the Web](http://www.meetup.com/ruststhlm/events/232054490/).
* [8/29. Rust on the web Rust Meetup Stockholm #2](http://www.meetup.com/ruststhlm/events/232054490/).
* 9/9. Rust Table of Regulars Darmstadt

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

> The best way to learn Rust is to just `try!` and see what works (or is this to just see what works`?` now?)!

— [llogiq on /r/rust](https://www.reddit.com/r/rust/comments/4xuds0/sharing_coloring_books_with_friends_in_rust/d6jecnz).

Thanks to [UtherII](https://users.rust-lang.org/users/utherii) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
