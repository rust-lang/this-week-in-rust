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

# Crate of the Week

This week's crate is [if_chain](https://crates.io/crates/if_chain) a macro that helps combat rightwards drift where code nests many `if`s and `if let`s. Since the
latter cannot be contracted with `&&`, this can be really helpful to make code more readable. Thanks to [Michael Budde](https://users.rust-lang.org/u/mbudde) for
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

163 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-10-09..2017-10-16

* [add `x86_64-unknown-linux-gnux32` target](https://github.com/rust-lang/rust/pull/45224)
* [inline `eq_slice` into `str::eq`](https://github.com/rust-lang/rust/pull/45005)
* [MIR-borrowck: moves of prefixes invalidate uses too](https://github.com/rust-lang/rust/pull/45025)
* [MIR borrowck: print lvalues in error messages in the same way that the AST borrowck](https://github.com/rust-lang/rust/pull/44985)
* [MIR-borrowck: add false edges to match arms](https://github.com/rust-lang/rust/pull/45200)
* [MIR-borrowck: migrate remaining AST diagnostics](https://github.com/rust-lang/rust/pull/45167)
* [querify `trans_fulfill_obligation`](https://github.com/rust-lang/rust/pull/44967)
* [querify Vtable methods](https://github.com/rust-lang/rust/pull/45137)
* [check namespaces when resolving associated items in typeck](https://github.com/rust-lang/rust/pull/45297)
* [rustc: Remove `used_mut_nodes` from `TyCtxt`](https://github.com/rust-lang/rust/pull/45283)
* [rustc: Fix some ThinLTO internalization](https://github.com/rust-lang/rust/pull/45215)
* [rustc: Update LLVM with a ThinLTO fix](https://github.com/rust-lang/rust/pull/45203)
* [rustc: Handle `#[inline(always)]` at `-O0`](https://github.com/rust-lang/rust/pull/45202)
* [rustc: Don't inline in CGUs at `-O0`](https://github.com/rust-lang/rust/pull/45075)
* [rustc: Reduce default CGUs to 16](https://github.com/rust-lang/rust/pull/45064)
* [incremental compilation auto assert (with except)](https://github.com/rust-lang/rust/pull/45104)
* [incremental compilation: Bring back output of -Zincremental-info](https://github.com/rust-lang/rust/pull/45063)
* [ensure `std::mem::Discriminant` is `Send + Sync`](https://github.com/rust-lang/rust/pull/45095)
* [fix `TcpStream::connect_timeout` on linux](https://github.com/rust-lang/rust/pull/45269)
* [improve performance of `spsc_queue` and stream](https://github.com/rust-lang/rust/pull/44963)
* [improve raw `Box` conversions](https://github.com/rust-lang/rust/pull/44877)
* [some hashmap cleanups](https://github.com/rust-lang/rust/pull/45263)
* [optimize comparison functions of `Iterator`](https://github.com/rust-lang/rust/pull/45007)
* [compiletest/runtest: format `ErrorKind` with `Display`](https://github.com/rust-lang/rust/pull/45258)
* [implement display_hint in gdb pretty printers](https://github.com/rust-lang/rust/pull/45071)
* [some low-hanging rustdoc optimizations](https://github.com/rust-lang/rust/pull/44613)
* [rustdoc: mobile sidebar improvements](https://github.com/rust-lang/rust/pull/45240)
* [let rustdoc print the crate version into docs](https://github.com/rust-lang/rust/pull/44989)
* [incr.comp.: Introduce `ensure` and `ensure` typeck_tables_of](https://github.com/rust-lang/rust/pull/45228)
* [enable building clippy in CI](https://github.com/rust-lang/rust/pull/45177) (one more step towards stable clippy!)
* [update grammar to parse current rust syntax](https://github.com/rust-lang/rust/pull/45125) (Language lawyers rejoice!)

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
