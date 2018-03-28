Title: This Week in Rust 227
Number: 227
Date: 2018-03-27
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [Touring a Fast, Safe, and Complete(ish) Web Service in Rust](https://brandur.org/rust-web)

* [Designing a fast CLI join tool with rust](https://milancio42.github.io/blog/2018-03-22-rjoin/)

# Crate of the Week

This week's crate is [fui](https://crates.io/crates/fui), a crate to add both a command-line interface and text forms to your program. Thanks to [musicmatze](https://users.rust-lang.org/u/musicmatze) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is more of a writing project than a programming project, but there are a bunch of things that need doing. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

178 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-03-19..2018-03-26

* [prepare the 1.25.0 stable release](https://github.com/rust-lang/rust/pull/49340)
* [rustc: add a `#[wasm_custom_section]` attribute](https://github.com/rust-lang/rust/pull/48883)
* [add basic PGO support](https://github.com/rust-lang/rust/pull/48346) (Hooray!)
* [improve lint for type alias bounds](https://github.com/rust-lang/rust/pull/48909)
* [stabilize the copy_closures and clone_closures features](https://github.com/rust-lang/rust/pull/49299)
* [stabilize impl Trait](https://github.com/rust-lang/rust/pull/49255) (Huzzah!)
* [detect illegal hidden lifetimes in `impl Trait`](https://github.com/rust-lang/rust/pull/49041)
* [NLL should identify and respect the lifetime annotations that the user wrote](https://github.com/rust-lang/rust/pull/48482)
* [fix DefKey lookup for proc-macro crates](https://github.com/rust-lang/rust/pull/49273)
* [always print `aborting due to n previous error(s)`](https://github.com/rust-lang/rust/pull/49046)
* [better diagnostics for '..' pattern fragment not in the last position](https://github.com/rust-lang/rust/pull/49268)
* [support elision in impl headers](https://github.com/rust-lang/rust/pull/49251) (`'_`)
* [fix type_dependent_defs ICE on method calls](https://github.com/rust-lang/rust/pull/49244)
* [pass attributes to hir::TyParam](https://github.com/rust-lang/rust/pull/49242)
* [produce nice array lengths on a best effort basis](https://github.com/rust-lang/rust/pull/49262)
* [remove slow HashSet during miri stack frame creation](https://github.com/rust-lang/rust/pull/49274)
* [refactor the `BorrowckErrors` trait to take `fn(self)`](https://github.com/rust-lang/rust/pull/48902)
* [don't check interpret_interner when accessing a static to fix miri mutable statics](https://github.com/rust-lang/rust/pull/49216)
* [implement "Implemented-From-Env" Chalk lowering rule](https://github.com/rust-lang/rust/pull/49211)
* [implement Raw Identifiers](https://github.com/rust-lang/rust/pull/48942) (RFC #[2151](https://rust-lang.github.io/rfcs/2151-raw-identifiers.html))
* [fix the conversion between bit representations and i128 representations](https://github.com/rust-lang/rust/pull/49210)
* [encode/decode extern statics in metadata and incremental cache](https://github.com/rust-lang/rust/pull/49200)
* [extend stable hasher to support `CanonicalTy`](https://github.com/rust-lang/rust/pull/49091)
* [`Pin`, `Unpin`, `PinBox`](https://github.com/rust-lang/rust/pull/49058) (immovable types for generators)
* [make resuming generators unsafe instead of the creation of immovable generators](https://github.com/rust-lang/rust/pull/49194)
* [add hexadecimal formatting of integers with fmt::Debug](https://github.com/rust-lang/rust/pull/48978)
* [reduce the diagnostic spam when multiple fields are missing in pattern](https://github.com/rust-lang/rust/pull/49160)
* [add a -Z flag for LLVM align attributes on arguments](https://github.com/rust-lang/rust/pull/49122)
* [deprecate the AsciiExt trait in favor of inherent methods](https://github.com/rust-lang/rust/pull/49109)
* [stabilize termination_trait, split out termination_trait_test](https://github.com/rust-lang/rust/pull/49162)
* [stabilise FromUtf8Error::as_bytes](https://github.com/rust-lang/rust/pull/49121)
* [stabilize 128-bit integers](https://github.com/rust-lang/rust/pull/49101)
* [replace `convert::Infallible` with `!`](https://github.com/rust-lang/rust/pull/49038)
* [add `simd_select` intrinsic](https://github.com/rust-lang/rust/pull/49141)
* [add `BufReader::buffer`](https://github.com/rust-lang/rust/pull/49139)
* [suggest removing `&`s](https://github.com/rust-lang/rust/pull/48834)
* [implement Integer methods for Wrapping](https://github.com/rust-lang/rust/pull/48810)
* [stabilize slice patterns without `..`](https://github.com/rust-lang/rust/pull/48516)
* [implement `get_key_value` for {`HashMap`, `BTreeMap`}](https://github.com/rust-lang/rust/pull/49346)
* [fix vector fmin/fmax non-fast/fast intrinsics NaN handling](https://github.com/rust-lang/rust/pull/49231)
* [add 12 num::NonZero* types for primitive integers, deprecate core::nonzero](https://github.com/rust-lang/rust/pull/48265)
* [cargo: faster resolver: use a inverse-index to not activate the causes of conflict](https://github.com/rust-lang/cargo/pull/5213)
* [fix ordering of auto-generated trait bounds in rustdoc output](https://github.com/rust-lang/rust/pull/49196)

## New Contributors

* Alan Du
* Alexandre Martin
* Alex Butler
* Boris-Chengbiao Zhou
* Dileep Bapat
* dragan.mladjenovic
* Eric Huss
* snf
* Yukio Siraichi

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2349: Standard library API for immovable types](https://github.com/rust-lang/rfcs/pull/2349).
* [RFC 2307: Add std::num::NonZeroU32 and friends, deprecate core::nonzero](https://github.com/rust-lang/rfcs/pull/2307).
* [RFC 2169: Add Euclidean modulo & division functionality for integers](https://github.com/rust-lang/rfcs/pull/2169).
* [RFC 2203: Constants in array repeat expressions](https://github.com/rust-lang/rfcs/pull/2203).
* [RFC 2342: Allow `if` and `match` in constants](https://github.com/rust-lang/rfcs/pull/2342).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [`Self` in type definitions allowing `enum List<T> { Nil, Cons(T, Box<Self>) }`](https://github.com/rust-lang/rfcs/pull/2300).
* [disposition: postpone] [Implement parent items with child traits](https://github.com/rust-lang/rfcs/pull/2303).
* [disposition: close] [Make the `as` keyword consider `Into` Trait implementations](https://github.com/rust-lang/rfcs/pull/2308).
* [disposition: close] [Quick `dbg!(expr)` macro](https://github.com/rust-lang/rfcs/pull/2173).

## New RFCs

* [Custom self types](https://github.com/rust-lang/rfcs/pull/2362).
* [Allow arbitrary enums to have explicit discriminants](https://github.com/rust-lang/rfcs/pull/2363).
* [Formalise reborrows](https://github.com/rust-lang/rfcs/pull/2364).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Mar 22. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 25. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbhc/).
* [Mar 21. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247387953/).
* [Mar 27. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Mar 27. Kitchener, CA - An Introduction To Rust & Writing a Crate (Kahan Sums)](https://www.meetup.com/Rust-KW/events/247661794/).
* [Mar 28. Milano, IT - Let's play with Procedural Macros in Rust](https://www.meetup.com/rust-language-milano/events/248725926/).
* [Mar 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Mar 28. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 29. Helsinki, FI - March Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/248805420/).
* [Apr  1. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbcb/).
* [Apr  3. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxgbfb/).
* [Apr  4. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxgbgb/).
* [Apr  4. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxgbgb/).
* [Apr  4. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/cpvshpyxgbgb/).
* [Apr  4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247388074/).
* [Apr  4. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr  5. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Imagine going back in time and telling the reporter “this bug will get fixed 16 years from now, and the code will be written in a systems programming language that doesn’t exist yet”.

— [Nicholas Nethercote](https://blog.mozilla.org/nnethercote/2018/03/09/a-new-preferences-parser-for-firefox/).

Thanks to [jleedev](https://users.rust-lang.org/t/twir-quote-of-the-week/328/501)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
