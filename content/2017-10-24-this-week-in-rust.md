Title: This Week in Rust 205
Number: 205
Date: 2017-10-24
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

* [Building a cross-platform game in Rust](https://www.reddit.com/r/rust/comments/78bowa/hey_this_is_kyren_from_chucklefish_we_make_and/).
* [Rust to WebAssembly, made easy](https://lord.io/blog/2017/wargo/).
* [HolyJit: A JIT for Firefox's and Servo's JS engine written in Rust](https://blog.mozilla.org/javascript/2017/10/20/holyjit-a-new-hope/).
* [Rust with session types for safe & high-performance codecs](https://polysync.io/blog/session-types-for-hearty-codecs/).
* [Nightly rustdoc is changing markdown rendering, here's some common differences and how to fix them](https://blog.guillaume-gomez.fr/articles/2017-09-18+New+rustdoc+rendering+common+errors).
* [How to deploy a Rocket application to Heroku](http://www.duelinmarkers.com/2017/10/21/how-to-deploy-a-rocket-application-to-heroku.html)
* [Test driven development with Rust](https://matthewkmayer.github.io/blag/public/post/tdd-with-rust/).
* [This week in Rust docs 78](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-78).
* [podcast] [Rusty Spike Podcast - Episode 4](https://rusty-spike.blubrry.net/2017/10/18/episode-4-oct-18-2017/). Rust 1.21 release, popularity of the O’Reilly Rust book, GitHub Octoverse, module counts, and Servo adding JetStream.

# Crate of the Week

This week's crate is [colored](https://crates.io/crates/colored) a UNIX-based terminal color crate. Thanks to [Kyle Galloway](https://users.rust-lang.org/u/kylegalloway) for
the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [How to get involved with the new gfx-rs development](https://github.com/gfx-rs/gfx/wiki/Getting-Involved).
* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [CfP is open for FOSDEM 2018](https://rust-fosdem.github.io).
* [semver-parser: Add examples to public items and methods](https://github.com/steveklabnik/semver-parser/issues/17).
* [semver-parser: Add readme, keywords and categories to Cargo.toml](https://github.com/steveklabnik/semver-parser/issues/18).
* [semver-parser: Add html_root_url attribute to crate root](https://github.com/steveklabnik/semver-parser/issues/19).
* [semver-parser: Make structures with public fields non-exhaustive](https://github.com/steveklabnik/semver-parser/issues/20).
* [semver-parser: Rename `Predicate` to `Comparator` and `VersionReq` to `Range`](https://github.com/steveklabnik/semver-parser/issues/21).
* [semver-parser: Support parsing range sets](https://github.com/steveklabnik/semver-parser/issues/22).
* [semver-parser: Add documentation to the crate root and public items](https://github.com/steveklabnik/semver-parser/issues/23).
* [semver-parser: Implement common traits for `Version`](https://github.com/steveklabnik/semver-parser/issues/16).
* [semver-parser: Implement common traits for `VersionReq`](https://github.com/steveklabnik/semver-parser/issues/15).
* [semver: Use `?` instead of `unwrap` in examples](https://github.com/steveklabnik/semver/issues/129).
* [semver: Document error cases on `Version::parse`](https://github.com/steveklabnik/semver/issues/130).
* [semver: Document error cases on `VersionReq::parse`](https://github.com/steveklabnik/semver/issues/131).
* [semver: Turn references to other `semver` items into links](https://github.com/steveklabnik/semver/issues/132).
* [semver: Add keywords and categories to Cargo.toml](https://github.com/steveklabnik/semver/issues/133).
* [semver: Add html_root_url attribute to crate root](https://github.com/steveklabnik/semver/issues/134).
* [semver: Make error types non-exhaustive](https://github.com/steveklabnik/semver/issues/135).
* [semver: Make fields on `Version` private](https://github.com/steveklabnik/semver/issues/136).
* [semver: Support range sets](https://github.com/steveklabnik/semver/issues/137).
* [semver: Rename VersionReq to Range](https://github.com/steveklabnik/semver/issues/138).
* [winit: Wayland metabug](https://github.com/tomaka/winit/issues/306).
* [arrayvec: ArrayString lacks removal features](https://github.com/bluss/arrayvec/issues/66#issuecomment-338413765).
* [easy] [allocators-rs: malloc-bind: Use likely/unlikely](https://github.com/ezrosent/allocators-rs/issues/111).
* [medium] [allocators-rs: elfc: Test in Travis and AppVeyor](https://github.com/ezrosent/allocators-rs/issues/119).
* [medium] [allocators-rs: alloc-fmt: Skip early frames of backtrace](https://github.com/ezrosent/allocators-rs/issues/107).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

121 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-10-16..2017-10-23

* [rustbuild: Compile rustc with ThinLTO](https://github.com/rust-lang/rust/pull/45400)
* [code suggestions for non-shorthand field pattern, no-mangle lints](https://github.com/rust-lang/rust/pull/45232)
* [cleanly error for non-const variable in associated const](https://github.com/rust-lang/rust/pull/45442)
* [remove or encapsulate the remaining non-query data in tcx](https://github.com/rust-lang/rust/pull/44501)
* [make `erase_regions_ty` query anonymous](https://github.com/rust-lang/rust/pull/45364)
* [bump the minimum LLVM to 3.9](https://github.com/rust-lang/rust/pull/45326)
* [backport ThinLTO LLVM 5 fixes](https://github.com/rust-lang/rust/pull/45301)
* [rustc: Move bytecode compression into codegen](https://github.com/rust-lang/rust/pull/45399)
* [rustc: Add `_imp_` symbols later in compilation](https://github.com/rust-lang/rust/pull/45348)
* [incr.comp.: Use 128bit SipHash for fingerprinting](https://github.com/rust-lang/rust/pull/45319)
* [mark block exits as reachable if the block can break](https://github.com/rust-lang/rust/pull/45316)
* [avoid unnecessary allocas for indirect function arguments](https://github.com/rust-lang/rust/pull/44573)
* [std: Update randomness implementation on Windows](https://github.com/rust-lang/rust/pull/45370)
* [fix a few bugs in drop generation](https://github.com/rust-lang/rust/pull/45359)
* [introduce pthread_cancel() for terminating threads](https://github.com/rust-lang/libc/pull/810)
* [fix ABI breakage in syscall constants](https://github.com/rust-lang/libc/pull/815)
* [fix most rendering warnings from switching to CommonMark](https://github.com/rust-lang/rust/pull/45419)
* [allow cargo install --version as well (preferred)](https://github.com/rust-lang/cargo/pull/4637)

## New Contributors

* cjkenn
* clippered
* greg
* Marco Concetto Rudilosso
* Sunjay Varma

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

* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [Cargo versioning](https://github.com/rust-lang/rfcs/pull/2182).
* [Movable array iterators](https://github.com/rust-lang/rfcs/pull/2185).
* [Imply Option](https://github.com/rust-lang/rfcs/pull/2180).
* [Support long path names on all Windows versions](https://github.com/rust-lang/rfcs/pull/2188).

# Upcoming Events

* [Oct 26 & 27. Rust Belt Rust 2017 - Columbus OH](https://www.rust-belt-rust.com/).
* [Oct 26. Finland Rust-lang group October meetup](https://www.meetup.com/Finland-Rust-Meetup/events/243886850/).
* [Oct 26. Mozilla Community Dresden - Rust meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/243915635/).
* [Oct 27. Rust Bangalore - Rust XML JSON and Serialization Workshop](https://www.meetup.com/rustox/events/243387629/).
* [Oct 28 & 29. Get In Touch With Rust! @ MozFest London](http://mozillafestival.org/)
* [Oct 30. Rust Oslo - Code generation and type providers in Rust](https://www.meetup.com/Rust-Oslo/events/244142198/).
* [Oct 31. Rust Zürich - Calling Rust from C and Java - October Community Meetup](https://www.meetup.com/Rust-Zurich/events/243147356/).
* [Nov  1. Boston Rust - Presentation and Hack Night](https://www.meetup.com/BostonRust/events/244260833/).
* [Nov  1. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/243942704/).
* [Nov  1. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywpbcb/).
* [Nov  1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov  1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov  2. Rust Bay Area - Zero Knowledge Proof Macros and Cernan (data pipelining)](https://www.meetup.com/Rust-Bay-Area/events/244156617/).
* [Nov  2. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Nov  4. Rust Bangalore - Rust Concurrency Workshop](https://www.meetup.com/rustox/events/240879563/).
* [Nov  8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov  8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov  9. Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/243672298/).
* [Nov  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/244164143/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Distributed Application Developer at Sphere Identity, Auckland, NZ](https://nz.linkedin.com/jobs/view/distributed-application-developers---blockchain-at-sphere-identity-ltd-442432632).
* [Full-time Rust position at Commure, San Francisco, US](https://news.ycombinator.com/item?id=15387799).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I guess I would say that, it’s not only important that it be possible to do a good design in a given language, but that the language actively encourage it by making the bad design painful. I think rust does a FANTASTIC job of this.

— [/u/kyrenn during an AMA on developing a cross-platform game in Rust](https://www.reddit.com/r/rust/comments/78bowa/hey_this_is_kyren_from_chucklefish_we_make_and/dosq5qr/).

Thanks to [Kyle Strand](https://users.rust-lang.org/t/twir-quote-of-the-week/328/460) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
