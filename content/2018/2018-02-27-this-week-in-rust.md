Title: This Week in Rust 223
Number: 223
Date: 2018-02-27
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

* [Announcing the Embedded Devices Working Group](https://internals.rust-lang.org/t/announcing-the-embedded-devices-working-group/6839).
* [Announcing stdweb 0.4 - export arbitrary functions to JavaScript](https://www.reddit.com/r/rust/comments/7z8imh/announcing_stdweb_04_now_with_support_for_parcel/).
* [Speed without wizardry](http://fitzgeraldnick.com/2018/02/26/speed-without-wizardry.html). Rust + WebAssembly and performance.
* [Programming an ARM microcontroller in Rust at four different levels of abstraction](http://pramode.in/2018/02/20/programming-a-microcontroller-in-rust-at-four-levels-of-abstraction/).
* [Should you Rust in embedded yet](https://kazlauskas.me/entries/rust-embedded-ready.html)?
* [Reasoning with types in Rust](https://aaronweiss.us/posts/2018-02-26-reasoning-with-types-in-rust.html).
* [Typestates in Rust](https://yoric.github.io/post/rust-typestate/).
* [Discovery: Discover the world of microcontrollers through Rust! - 2018 edition](https://japaric.github.io/discovery/).
* [Oxidizing Fedora: Try Rust and its applications in Fedora 27](https://fedoramagazine.org/oxidizing-fedora-try-rust-applications-today/).
* [Debian: Rust architecture status](https://lists.alioth.debian.org/pipermail/pkg-rust-maintainers/2018-February/001215.html).
* [RustConf CFP is now open. Deadline is 13 April](https://cfp.rustconf.com/events/rustconf-2018/).
* [RustFest CFP is now open. Deadline is 17 March](https://cfp.rustfest.eu/events/rustfest-paris).
* [This week in Rust docs 94](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-94).
* [podcast] [Rusty Spike Podcast - episode 20](https://rusty-spike.blubrry.net/2018/02/22/episode-20-feb-21-2018/). Rust 1.24, Rayon, Facebook, Fedora, Snips, and benchmarks.

# Crate of the Week

This week's crate is [fselect](https://github.com/jhspetersson/fselect), a crate to find files by SQL-like queries. Thanks to [Jhspetersson](https://users.rust-lang.org/u/jhspetersson) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [doc] [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide): A guide to how rustc works and how to contribute to it [has issues that need help](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22). The difficulty ranges from "copy the contents of the rustc READMEs" or "proofreading" to "write about the dark bowls of X in rustc". Prior knowledge of compiler internals is _not_ required.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

127 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-19..2018-02-26

* [fix exponential projection complexity on nested types](https://github.com/rust-lang/rust/pull/48296)
* [avoid ICE in arg mistmatch error for tuple variants](https://github.com/rust-lang/rust/pull/48246)
* [fix parsing of extern paths in types and poly-traits](https://github.com/rust-lang/rust/pull/48441)
* [make ".e0" not parse as 0.0](https://github.com/rust-lang/rust/pull/48235)
* [use sparse bitsets instead of dense ones for NLL results](https://github.com/rust-lang/rust/pull/48245)
* [reset default binding mode when we pass through a `&` pattern](https://github.com/rust-lang/rust/pull/48448)
* [add nonstandard_style alias for bad_style](https://github.com/rust-lang/rust/pull/48386)
* [fix span of visibility](https://github.com/rust-lang/rust/pull/47799)
* [overhaul improper_ctypes output](https://github.com/rust-lang/rust/pull/48221)
* [inform user where to give a type annotation](https://github.com/rust-lang/rust/pull/48198)
* [handle custom diagnostic for `&str + String`](https://github.com/rust-lang/rust/pull/48392)
* [fix nested impl trait lifetimes](https://github.com/rust-lang/rust/pull/48072)
* [error on nested impl Trait and path projections from impl Trait](https://github.com/rust-lang/rust/pull/48084)
* [remove "static item recursion checking" in favor of relying on cycle checks in the query engine](https://github.com/rust-lang/rust/pull/47987)
* [improve tuple struct field access hygiene](https://github.com/rust-lang/rust/pull/48083)
* [macros: improve struct constructor field hygiene, fix span bug](https://github.com/rust-lang/rust/pull/48082)
* [do not run the default panic hook inside procedural macros](https://github.com/rust-lang/rust/pull/47933)
* [introduce UnpackedKind](https://github.com/rust-lang/rust/pull/48452)
* [do not run MIR type checker twice](https://github.com/rust-lang/rust/pull/48061)
* [MIR: gather move at SwitchInt, Assert terminators](https://github.com/rust-lang/rust/pull/48232)
* [allow two-phase borrows of &mut self in ops](https://github.com/rust-lang/rust/pull/48197)
* [incr.comp.: Don't keep RefCells in on-disk-cache borrowed in order to allow for recursive invocations](https://github.com/rust-lang/rust/pull/48185)
* [rustc_trans: rewrite mips64 ABI code](https://github.com/rust-lang/rust/pull/47964)
* [termination trait in tests](https://github.com/rust-lang/rust/pull/48143)
* [detect wrong number of args when type-checking a closure](https://github.com/rust-lang/rust/pull/48123)
* [add Iterator::try_for_each](https://github.com/rust-lang/rust/pull/48157)
* [add Iterator::flatten](https://github.com/rust-lang/rust/pull/48115)
* [add Condvar APIs not susceptible to spurious wake](https://github.com/rust-lang/rust/pull/47970)
* [stabilize 'entry_and_modify' feature](https://github.com/rust-lang/rust/pull/48166)
* [stabilize Box::leak](https://github.com/rust-lang/rust/pull/48110)
* [Fix borrow checker unsoundness with unions](https://github.com/rust-lang/rust/pull/47689)
* [Derive std::cmp::Reverse as Copy or Clone](https://github.com/rust-lang/rust/pull/47379)
* [When encountering invalid token after `unsafe`, mention `{`](https://github.com/rust-lang/rust/pull/48356)
* [Implement implied shortcut links for intra-rustdoc-links](https://github.com/rust-lang/rust/pull/48335)
* [rename rdrnd target feature to rdrand](https://github.com/rust-lang/rust/pull/48369)
* [book: Second edition is now the definitive edition](https://github.com/rust-lang/book/pull/1180)
* [rustc explain](https://github.com/rust-lang/rust/pull/48337)
* [rustdoc: generate documentation for auto-trait impls](https://github.com/rust-lang/rust/pull/47833)
* [rustdoc: don't crash when an external trait's docs needs to import another trait](https://github.com/rust-lang/rust/pull/48415)
* [fix rustdoc test ICE](https://github.com/rust-lang/rust/pull/48382)
* [make cargo-the-binary version the same as the Rust version](https://github.com/rust-lang/cargo/pull/5083)
* [cargo: warn Windows 7 users about old TLS](https://github.com/rust-lang/cargo/pull/5069)
* [cargo: display path to custom commands with `--list -v`](https://github.com/rust-lang/cargo/pull/5041)

## New Contributors

* Andreas Streichardt
* Anthony Deschamps
* boats
* Bryan Drewery
* csmoe
* Dale Wijnand
* Dan Aloni
* Federico Poli
* hedgehog1024
* Hidehito Yabuuchi
* Jakub Adam Wieczorek
* Jimmy Brush
* moe
* Nathan Ringo
* newpavlov
* Vitali Lovich

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2325: Stable SIMD in Rust](https://github.com/rust-lang/rfcs/pull/2325).
* [RFC 2226: Hexadecimal integers with fmt::Debug, including within larger types](https://github.com/rust-lang/rfcs/pull/2226).
* [RFC 2151: Raw identifiers](https://github.com/rust-lang/rfcs/pull/2151). Add a raw identifier format `r#ident`, so crates written in future language epochs/versions can still use an older API that overlaps with new keywords.
* [RFC 2298: `?` repetition in macro rules](https://github.com/rust-lang/rfcs/pull/2298).
* [RFC 2046: label-break-value](https://github.com/rust-lang/rfcs/pull/2046). Allow a break not only out of `loop`, but of labelled blocks with no loop
* [RFC 2333: Prior art](https://github.com/rust-lang/rfcs/pull/2333). A section to the RFC template where RFC authors may discuss the experience of other programming languages.


## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Rust 2018 roadmap](https://github.com/rust-lang/rfcs/pull/2314).
* [disposition: merge] [Cargo profile dependencies](https://github.com/rust-lang/rfcs/pull/2282).
* [disposition: merge] [Allow locals and destructuring in const fn](https://github.com/rust-lang/rfcs/pull/2341).
* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [Amend RFC 0141 Lifetime elision: Mention deduplicated lifetimes](https://github.com/rust-lang/rfcs/pull/2330).
* [disposition: merge] [Update the disambiguation handling in RFC 1946 (intra-rustdoc-links) to match impl concerns](https://github.com/rust-lang/rfcs/pull/2285).

## New RFCs

* [Standard library API for immovable types](https://github.com/rust-lang/rfcs/pull/2349).
* [Allow `loop` in constant evaluation](https://github.com/rust-lang/rfcs/pull/2344).
* [Allow panicking in constants](https://github.com/rust-lang/rfcs/pull/2345).
* [`#[derive_no_bound(..)]` and `#[derive_field_bound(..)]`](https://github.com/rust-lang/rfcs/pull/2353).
* [Add `is_sorted` to the standard library](https://github.com/rust-lang/rfcs/pull/2351).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Mar  4. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbgb/).
* [Mar  5. London, GB - Rust learning and hacking evening #8](https://www.meetup.com/Rust-London-User-Group/events/247286584/).
* [Mar  6. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxfbjb/).
* [Mar  7. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlyxfbkb/).
* [Mar  7. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxfbkb/).
* [Mar  7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  8. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxfblb/).
* [Mar  8. San Diego, US - San Diego Rust March Meetup](https://www.meetup.com/San-Diego-Rust/events/248229805/).
* [Mar 11. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbpb/).
* [Mar 12. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxfbqb/).
* [Mar 13. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-content).
* [Mar 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 13. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 13. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 15. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/fmwshpyxfbtb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer at Vistaprint](https://careers.vistaprint.com/job-description/?id=23901).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
