Title: This Week in Rust 206
Number: 206
Date: 2017-10-31
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

# Crate of the Week

This week's crate is [cargo-outdated](https://crates.io/crates/cargo-outdated), a cargo subcommand that shows outdated dependencies including latest compatible
and latest version. Thanks to [Colin Kiegel](https://users.rust-lang.org/u/colin_kiegel) for the suggestion.

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

111 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-10-23..2017-10-30

* [ci: upgrade Android SDK/NDK and refactor to use sdkmanager/avdmanager](https://github.com/rust-lang/rust/pull/45580)
* [implement RFC 1861: Extern types](https://github.com/rust-lang/rust/pull/44295)
* [avoid unnecessary copies of arguments that are simple bindings](https://github.com/rust-lang/rust/pull/45380)
* [add short error message-format](https://github.com/rust-lang/rust/pull/44636)
* [add several lints into `unused` lint group](https://github.com/rust-lang/rust/pull/45424)
* [resolve types properly in const eval](https://github.com/rust-lang/rust/pull/45488)
* [create NormalizeTy query](https://github.com/rust-lang/rust/pull/44984)
* [`crate` shorthand visibility modifier](https://github.com/rust-lang/rust/pull/45401)
* [move Generics from MethodSig to TraitItem and ImplItem](https://github.com/rust-lang/rust/pull/44766)
* [add generics to LateContext](https://github.com/rust-lang/rust/pull/45611)
* [don't emit the same compiler diagnostic twice](https://github.com/rust-lang/rust/pull/45519)
* [improve diagnostics when list of tokens has incorrect separators](https://github.com/rust-lang/rust/pull/45503)
* [use 128 bit instead of Symbol for crate disambiguator](https://github.com/rust-lang/rust/pull/45476)
* [remove dependency tracking for variance computation](https://github.com/rust-lang/rust/pull/45473)
* [implement Hash for raw pointers to unsized types](https://github.com/rust-lang/rust/pull/45483)
* [visit attribute tokens in `DefCollector` and `BuildReducedGraphVisitor`](https://github.com/rust-lang/rust/pull/45464)
* [remove deprecated `collections` crate](https://github.com/rust-lang/rust/pull/45446)
* [fix 32- vs 64-bit platform instability in StableHasher](https://github.com/rust-lang/rust/pull/45522)
* [std: optimize thread park/unpark implementation](https://github.com/rust-lang/rust/pull/45524)
* [std: disable usage of mmap allocator in libbacktrace ](https://github.com/rust-lang/rust/pull/45523)
* [add current_pid function](https://github.com/rust-lang/rust/pull/45059)
* [cargo: add unit test checking to `cargo check`](https://github.com/rust-lang/cargo/pull/4592)
* [cargo: improving the error message for when a patched dependency does not resolve to anything](https://github.com/rust-lang/cargo/pull/4607)
* [cargo: alternative registries](https://github.com/rust-lang/cargo/pull/4506)
* [rustdoc: Show src button and function version on mobile version ](https://github.com/rust-lang/rust/pull/45502)

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
* [Nov  2. Live AMA with Rust Core Team members](https://hashnode.com/ama/with-rust-language-team-cj99mv7s101yw4rwtk5zntk8k).
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
