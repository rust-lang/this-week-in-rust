Title: This Week in Rust 164
Number: 164
Date: 2017-01-10
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

* [The Rust programming language, in the words of its practitioners](https://brson.github.io/fireflowers/).
* [Replacing the jet engine while still flying](http://jensimmons.com/post/jan-4-2017/replacing-jet-engine-while-still-flying). Project Quantum: An effort to incrementally add Servo technologies in Firefox.
* [Exploring _double faults_](http://os.phil-opp.com/double-faults.html). Part of the series [Writing an OS in Rust](http://os.phil-opp.com/).
* [Announcing Alacritty, a GPU-accelerated terminal emulator](http://blog.jwilm.io/announcing-alacritty/).
* [Piston: The image library is now pure Rust](http://blog.piston.rs/2017/01/08/the-image-library-is-now-pure-rust/).
* [A guide to compiling Rust to WebAssembly](https://hackernoon.com/compiling-rust-to-webassembly-guide-411066a69fde).
* [Should you convert your C project to Rust](http://jamey.thesharps.us/2017/01/which-projects-should-convert-to-rust.html)?
* [american fuzzy lop’ing Rust](http://blog.troutwine.us/2017/01/06/american-fuzzy-loping-rust/). [american fuzzy lop](https://en.wikipedia.org/wiki/American_fuzzy_lop_(fuzzer)) is a fuzzer that employs genetic algorithms in order to efficiently increase code coverage of the test cases.
* [Inner workings of a search engine written in Rust](http://fulmicoton.com/posts/behold-tantivy).
* [SoA (Structure of Arrays) in Rust with macros 1.1](https://maikklein.github.io/post/soa-rust/).
* [Rust makes implicit invariants explicit](https://medium.com/@robertgrosse/rust-makes-implicit-invariants-explicit-baf4cf17ae50).
* [The Rust module system is too confusing](https://withoutboats.github.io/blog/rust/2017/01/04/the-rust-module-system-is-too-confusing.html).
* [Rust at OneSignal](https://onesignal.com/blog/rust-at-onesignal/). OneSignal shares its experience of using Rust in production for more than a year.
* [Librsvg 2.41.0 requires Rust](https://mail.gnome.org/archives/desktop-devel-list/2017-January/msg00001.html).

## Other Weeklies from Rust Community

* [This week in Rust docs 38](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-38).
* [video] [Ferris makes emulators 14](https://www.youtube.com/watch?v=d-dm4z5fJ1U).

# Crate of the Week

This week's Crate of the Week is [rocket](https://crates.io/crates/rocket), an experimental web framework (will need a nightly Rust!) with a focus on ease-of-use, expressability and speed. Thanks to Vikrant for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rust: Make Rust on wasm + emscripten a reliable, 1st class Rust target](https://github.com/rust-lang/rust/issues/38805).
* [easy] [rust: Rvalue static promotion](https://github.com/rust-lang/rust/issues/38865).
* [easy] [Diesel: Refactorings using macros in type position](https://github.com/diesel-rs/diesel/issues/521).
* [easy] [Diesel: Deny missing docs](https://github.com/diesel-rs/diesel/issues/563).
* [android-rs-glue: Add more arguments and use clap to parse the arguments](https://github.com/tomaka/android-rs-glue/issues/115).
* [tokei: Add package repositories](https://github.com/Aaronepower/tokei/issues/92).
* [RustCrypto/hashes: Missing hash functions](https://github.com/RustCrypto/hashes/issues/1).
* [RustCrypto/block-ciphers: Missing block ciphers](https://github.com/RustCrypto/block-ciphers/issues/1).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

109 pull requests were [merged in the last week][merged]. This contains a good number of plugin-breaking changes.

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-12-26..2016-01-02

* [`u128`/`i128` support!](https://github.com/rust-lang/rust/pull/38482) (RFC [#1504](https://github.com/rust-lang/rfcs/blob/master/text/1504-int128.md)), also [vim](https://github.com/rust-lang/rust.vim/pull/133)
* [new `min_atomic_width` target option](https://github.com/rust-lang/rust/pull/38579)
* [`--crate-type=metadata` is now `--emit=metadata`](https://github.com/rust-lang/rust/pull/38571)
* [Rust can now compile to PTX (cuda)](https://github.com/rust-lang/rust/pull/38559)
* [`fastcall` calling convention fixed](https://github.com/rust-lang/rust/pull/38542)
* [`pub(restricted)` visibilities are now handled by `DefId`](https://github.com/rust-lang/rust/pull/38490) (potentially plugin-breaking)
* [rustbuild now build's twice (instead of thrice) by default](https://github.com/rust-lang/rust/pull/38631) (hooray for faster builds!)
* [A new `DroplessArena` for some speedups](https://github.com/rust-lang/rust/pull/38653)
* [Debuggers can now pretty-print unions](https://github.com/rust-lang/rust/pull/38753)
* [Custom derives now can work on structs with macros](https://github.com/rust-lang/rust/pull/38737)
* [sparc64-linux support](https://github.com/rust-lang/rust/pull/38726)
* [`impl TrustedLen for `{`Empty`, `Once`}](https://github.com/rust-lang/rust/pull/38713)

## New Contributors

* derekdreery
* F001
* Kyle Aleshire
* Mina Naguib
* Yamakaky

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1774: Roadmap for 2017](https://github.com/rust-lang/rfcs/pull/1774).
* [RFC 1647: Allow `Self` to appear in the where clause of trait impls](https://github.com/rust-lang/rfcs/pull/1647).
* [RFC 1414: Add Rvalue-static-promotion](https://github.com/rust-lang/rfcs/pull/1414).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [crates.io] [What categories should be available on crates.io](https://github.com/rust-lang/crates.io/pull/488)?
* [Abort by default v2](https://github.com/rust-lang/rfcs/pull/1765). Specify abort-by-default in `Cargo.toml` when the user does `cargo new --bin`, as well as various other refinements to the panick strategy system.
* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Add syntax for expressing tuples as a head and tail pair, similar to a Lisp cons cell](https://github.com/rust-lang/rfcs/pull/1582).
* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).

## New RFCs

* [`'fn` lifetime ascription](https://github.com/rust-lang/rfcs/pull/1847). Add a `'fn` lifetime that is bound to the scope of the body of the current innermost function or closure.
* [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [From<&[T]> for Rc<[T]> + From<&str> for Rc<str>](https://github.com/rust-lang/rfcs/pull/1845).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

Ready for PR:

There's [a lot of them](https://github.com/rust-lang-nursery/fmt-rfcs/issues?q=is%3Aopen+is%3Aissue+label%3Aready-for-PR) right now, contributions here would be very welcome. If you want advice or help getting started, please ping nrc, or any other member of the style team, in #rust-style.

Issues in final comment period:

* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [Against braces always demanding rightward drift](https://github.com/rust-lang-nursery/fmt-rfcs/issues/50).
* [Disable trailing comma by default](https://github.com/rust-lang-nursery/fmt-rfcs/issues/42).
* [Conventions for Cargo.toml files (FCP)](https://github.com/rust-lang-nursery/fmt-rfcs/pull/41).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).
* [Customisation of Rustfmt should be allowed](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).

# Upcoming Events

* [1/12. Rust Utrecht](https://www.meetup.com/Rust-Utrecht/events/235444678/).
* [1/12. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [1/18. Rust Cologne: Ruby meets Rust](https://www.meetup.com/RustCologne/events/235877954/).
* [1/18. Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/236735645/).
* [1/18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/18. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/19. Rust Paris: Rust meetup #35](https://www.meetup.com/Rust-Paris/events/236727277/).
* [1/19. GPU enhanced terminals, Counting Votes, and Converting C to Rust](https://www.meetup.com/Rust-Bay-Area/events/236668916/).
* [1/20. Rust Rhein-Main: Rust Table of Regulars Darmstadt](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/236456912/?eventId=236456912).
* [1/24. Mozilla Meetup Switzerland: Rust January Meetup @ Coredump.ch](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/236277734/?eventId=236277734).
* [1/25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/25. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/25. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658932/).
* [1/26. Rust Stockholm: REST in Rust and Rust Hack Night](https://www.meetup.com/ruststhlm/events/236791788/).
* [1/26. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

* [Senior backend developer at OneSignal](https://angel.co/onesignal/jobs/128684-senior-backend-developer).
* [Rust backend developer at 1aim.com](https://news.ycombinator.com/item?id=13302210).
* [Rust systems programmer at Hadean](https://news.ycombinator.com/item?id=13301893).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is the first version to have Rust code in it. The public API
remains unchanged. Apologies in advance to distros who will have to
adjust their build systems for Rust - it's like taking a one-time
vaccine; you'll be better off in the end for it.

— [Federico Mena Quintero announcing librsvg 2.41.0](https://mail.gnome.org/archives/desktop-devel-list/2017-January/msg00001.html).

Thanks to [Zbigniew Siciarz](https://users.rust-lang.org/users/zsiciarz) for the [suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/338).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
