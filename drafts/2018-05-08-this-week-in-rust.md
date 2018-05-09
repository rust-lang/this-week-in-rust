Title: This Week in Rust 233
Number: 233
Date: 2018-05-08
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

* [New Tokio release, now with filesystem support](https://tokio.rs/blog/2018-05-tokio-fs/).
* [Encapsulating lifetime of the field](https://matklad.github.io/2018/05/04/encapsulating-lifetime-of-the-field.html).
* [Rust in production at Figma](https://blog.figma.com/rust-in-production-at-figma-e10a0ec31929).
* [How fast can we compile Rust hello world](http://www.jonathanturner.org/2018/05/how-fast-can-we-compile-rust-hello-world.html)?
* [Refactoring Apache Arrow to use traits and generics](https://andygrove.io/2018/05/apache-arrow-traits-generics/).
* [Introducing gtk-test - a framework to test GTK UI](http://gtk-rs.org/blog/2018/05/02/who-talked-about-testing.html).
* [Introducing seiri — a music manager written in Rust](https://medium.com/@chyyran/introducing-seiri-a-music-manager-for-lots-of-music-990b464b3387).
* [sudo_pair: A sudo plugin from Square that requires another human to approve and monitor privileged sudo sessions](https://github.com/square/sudo_pair).
* [Announcing the codegen working group](https://internals.rust-lang.org/t/announcing-the-codegen-working-group/7434).
* [This week in Rust docs 104](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-104).
* [Networking WG newsletter 1](https://internals.rust-lang.org/t/networking-wg-newletter-1/7431).

# Crate of the Week

This week's crate is [imgref](https://github.com/kornelski/imgref), a trivial Rust struct for interchange of pixel buffers with width, height and stride. Thanks to [Willi Kappler](https://users.rust-lang.org/u/willi_kappler) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Sodium Oxide: Call for maintainers](https://github.com/sodiumoxide/sodiumoxide/issues/203). Sodium Oxide is a fast cryptographic library for Rust (bindings to libsodium).
* [easy] [gfx-rs: Set buffers mutability qualifiers on Metal compute pipelines](https://github.com/gfx-rs/gfx/issues/1999). gfx-rs is a high-performance, bindless graphics API for Rust.
* [easy] [gfx-rs: Set threadGroupSizeIsMultipleOfThreadExecutionWidth on Metal compute pipelines](https://github.com/gfx-rs/gfx/issues/1998).
* [easy] [gfx-rs: Use set_bytes for short temporary data in Metal internal shaders](https://github.com/gfx-rs/gfx/issues/1997).
* [mutagen has some 'good first issues'](https://github.com/llogiq/mutagen/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

140 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-04-23..2018-04-30

* [make incremental compilation thread-safe](https://github.com/rust-lang/rust/pull/49732)
* [display correct unused field suggestion for nested struct patterns](https://github.com/rust-lang/rust/pull/50327)
* [improve error message for `#[repr(align=x)]`](https://github.com/rust-lang/rust/pull/50317)
* [don't ICE on tuple struct ctor with incorrect arg count](https://github.com/rust-lang/rust/pull/50257)
* [warn on all erroneous constants](https://github.com/rust-lang/rust/pull/50110)
* [warn on pointless #[derive] in more places](https://github.com/rust-lang/rust/pull/50092)
* [better error message when trying to write default impls](https://github.com/rust-lang/rust/pull/49372)
* [mark `std::str::replace`(`n`) as `#[must_use]`](https://github.com/rust-lang/rust/pull/50177)
* [allow MIR borrowck to catch unused mutable locals](https://github.com/rust-lang/rust/pull/48605)
* [allow `#[inline]` on closures](https://github.com/rust-lang/rust/pull/50273)
* [rustc: emit `uwtable` for allocator shims](https://github.com/rust-lang/rust/pull/50263)
* [make `dump_`{`alloc`,`allocs`,`local`}`()` no-ops when tracing is disabled](https://github.com/rust-lang/rust/pull/50246)
* [implement LazyBTreeMap and use it in a few places](https://github.com/rust-lang/rust/pull/50240)
* [speed up `nearest_common_ancestor`](https://github.com/rust-lang/rust/pull/50106)
* [use `FxHashMap` in `syntax_pos::symbol::Interner::intern`](https://github.com/rust-lang/rust/pull/50174)
* [make `Vec::new` a `const fn`](https://github.com/rust-lang/rust/pull/50233)
* [fix ICE with erroneous `impl Trait` in a trait impl](https://github.com/rust-lang/rust/pull/50227)
* [use enum for approximate suggestions](https://github.com/rust-lang/rust/pull/50204)
* [core: fix overflow in `int::mod_euc` when `self < 0 && rhs == MIN`](https://github.com/rust-lang/rust/pull/50185)
* [remove hack around comparisons of i1 values](https://github.com/rust-lang/rust/pull/50137)
* [stabilize dyn trait](https://github.com/rust-lang/rust/pull/49968)
* [stabilize `std::hint::unreachable_unchecked`](https://github.com/rust-lang/rust/pull/49906)
* [compiletest: detect non-ICE compiler panics](https://github.com/rust-lang/rust/pull/49891)
* [rustc_driver: catch ICEs on the main thread too](https://github.com/rust-lang/rust/pull/49826)
* [add `Cell::update`](https://github.com/rust-lang/rust/pull/49727)
* [treat `repr(Rust)` univariant fieldless enums as ZSTs](https://github.com/rust-lang/rust/pull/49513)
* [std: inline `DefaultResizePolicy::new`](https://github.com/rust-lang/rust/pull/50306)
* [cargo: add target directory parameter --target-dir](https://github.com/rust-lang/cargo/pull/5393)
* [docs: add "the Rustc book"](https://github.com/rust-lang/rust/pull/49707)

## New Contributors

* Harm Berntsen
* rleungx
* Samuel Wilson

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2388: Reserve `try` for `try { .. }` block expressions](https://github.com/rust-lang/rfcs/pull/2388).
* [RFC 2230: Bury `Error::description()`](https://github.com/rust-lang/rfcs/pull/2230).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Unreserve `proc`](https://github.com/rust-lang/rfcs/pull/2420).
* [disposition: close] [`Result::pass()`, turning `Result<T,E>` into `Result<U,F>`, if `From` is set up](https://github.com/rust-lang/rfcs/pull/1996).
* [disposition: close] [The ConstDefault trait](https://github.com/rust-lang/rfcs/pull/2204).

## New RFCs

* [Reserve `delegate` as a keyword in edition 2018](https://github.com/rust-lang/rfcs/pull/2429).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [May 10. Redwood City, US - Bay Area - Where "Self-Driving" Database Meets a "Rusty" Distributed Key-Value Store](https://www.meetup.com/Bay-Area-NewSQL-Database-Meetup/events/249676562/).
* [May 10. Arlington, US - Rust DC - Learn+Try: parsing with nom](https://www.meetup.com/RustDC/events/249883820).
* [May 10. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/lcsdqpyxhbnb/).
* [May 10. San Diego, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/249783590/).
* [May 13. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbrb/).
* [May 14. Seattle, US - Seattle Rust Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxhbsb/).
* [May 15. Rome, IT - Rust learning and hacking evening #8](https://www.meetup.com/Rust-Roma/events/250581929/).
* [May 16. Orange County, US - Coding Session and Discussion](https://www.meetup.com/oc-rust/events/250342850/).
* [May 16. Denver, US - Rust Boulder/Denver - Rust Denver May Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/249098925/).
* [May 16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/249497881/).
* [May 16. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxhbvb/).
* [May 17. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxhbwb/).
* [May 20. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbbc/).
* [May 22. Paris, FR - Rust Paris meetup #42](https://www.meetup.com/Rust-Paris/events/250587163/).
* [May 22. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [May 23. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May 23. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.
* [May 30/31 Rust/WASM course around JSConf.EU](https://ti.to/asquera-event-ug/rust-wasm-wwwtf-2018/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. San Francisco, US](https://news.ycombinator.com/item?id=16968087).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
