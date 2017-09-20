Title: This Week in Rust 200
Number: 200
Date: 2017-09-19
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

- [podcast] [New Rustacean: Crates You Should Know: Rayon](http://www.newrustacean.com/show_notes/cysk/rayon/)

# Crate of the Week

This week's crate is [pikkr](https://github.com/pikkr/pikkr), a JSON parser that can extract values without tokenization and is blazingly fast using AVX2 instructions,
Thank you, [bstrie](https://users.rust-lang.org/u/bstrie) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [less easy] [bindgen: Allow specifying certain types we shouldn't derive/impl Debug for](https://github.com/rust-lang-nursery/rust-bindgen/issues/961).
* [less easy] [bindgen: Allow specifying certain types we shouldn't derive/impl Copy for](https://github.com/rust-lang-nursery/rust-bindgen/issues/962).
* [less easy] [bindgen: Allow specifying certain types we shouldn't derive/impl Default for](https://github.com/rust-lang-nursery/rust-bindgen/issues/963).
* [less easy] [bindgen: Allow specifying certain types we shouldn't derive/impl Hash for](https://github.com/rust-lang-nursery/rust-bindgen/issues/964).
* [less easy] [bindgen: Allow specifying certain types we shouldn't derive/impl PartialEq for](https://github.com/rust-lang-nursery/rust-bindgen/issues/965).
* [easy] [bindgen: bindgen generates erroneous rust code: error: expected identifier, found `_`](https://github.com/rust-lang-nursery/rust-bindgen/issues/631).
* [less easy] [bindgen: Rewrite `is_unsized` as a fix-point analysis](https://github.com/rust-lang-nursery/rust-bindgen/issues/768).
* [less easy] [bindgen: Large bitfield causes compilation failures re: `derive(Debug)`](https://github.com/rust-lang-nursery/rust-bindgen/issues/982).
* [easy] [bindgen: Add a `Builder::blacklist_type` method; deprecate `Builder::hide_type`](https://github.com/rust-lang-nursery/rust-bindgen/issues/984).
* [easy] [bindgen: Add `Builder::whitelist_function`; deprecate `Builder::whitelisted_function`](https://github.com/rust-lang-nursery/rust-bindgen/issues/985).
* [easy] [bindgen:  Add `Builder::whitelist_var`; deprecate `Builder::whitelisted_var`](https://github.com/rust-lang-nursery/rust-bindgen/issues/986).
* [easy] [bindgen: Add `Builder::whitelist_type`; deprecate `Builder::whitelisted_type`](https://github.com/rust-lang-nursery/rust-bindgen/issues/987).
* [easy] [bindgen: TemplateParameters' methods should just return a Vec, not an Option<Vec>](https://github.com/rust-lang-nursery/rust-bindgen/issues/960).
* [easy] [bindgen: Make BindgenOptions be pub(crate)](https://github.com/rust-lang-nursery/rust-bindgen/issues/958).
* [medium] [allocators-rs: elfmalloc: Handle thread-local storage on platforms without `#[thread_local]`](https://github.com/ezrosent/allocators-rs/issues/54).
* [medium] [allocators-rs: slab-alloc: Create mechanism for supporting time in no-std and no-os](https://github.com/ezrosent/allocators-rs/issues/3).
* [medium] [allocators-rs: bsalloc: Support allocation failures](https://github.com/ezrosent/allocators-rs/issues/4).
* [easy] [allocators-rs: Add links in documentation](https://github.com/ezrosent/allocators-rs/issues/1).
* [medium] [allocators-rs: Travis: Spurious test failures on Mac](https://github.com/ezrosent/allocators-rs/issues/36).
* [easy] [allocators-rs: AppVeyor: Mark appveyor.sh files executable](https://github.com/ezrosent/allocators-rs/issues/67).
* [easy] [allocators-rs: test-scripts/check-copyright-comments.sh: Fix comment typo](https://github.com/ezrosent/allocators-rs/issues/70).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

99 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-09-04..2017-09-11

* [improved message on `&str` vs. `&[u8]` literals](https://github.com/rust-lang/rust/pull/44361)
* [hint on overlapping inter-crate ambiguities](https://github.com/rust-lang/rust/pull/43426)
* [`#![feature(drop_types_in_const)]`](https://github.com/rust-lang/rust/pull/44212)
* [proper expansion info for more builtin macros](https://github.com/rust-lang/rust/pull/44248)
* [macros 2.0 now have their `Span` include visibility modifiers](https://github.com/rust-lang/rust/pull/44375)
* [use `NodeId`/`HirId` instead of `DefId` for local vars](https://github.com/rust-lang/rust/pull/44316)
* [avoid hashing every `HirId` in existence](https://github.com/rust-lang/rust/pull/44335)
* [`SplitWhitespace` now implements `Debug`](https://github.com/rust-lang/rust/pull/44303)
* [`_.clamp(min, max)` for `Ord` types](https://github.com/rust-lang/rust/pull/44097)
* [panic on overflow in `Instance` ± `Duration`](https://github.com/rust-lang/rust/pull/44220)
* [the `LocalKey` facade of `thread_local!` is now inlineable cross-crate](https://github.com/rust-lang/rust/pull/43931)
* [avoid creating `static`s for each `panic`](https://github.com/rust-lang/rust/pull/44312)
* [MIR no longer emits `EndRegion`s by default](https://github.com/rust-lang/rust/pull/44249)
* [MIR: restrict `ProjectionElem::Index` and `Storage`{`Live`, `Dead`} to `Local`](https://github.com/rust-lang/rust/pull/44308)
* [MIR: no longer inlines trait methods](https://github.com/rust-lang/rust/pull/44383)
* [MIR: inliner bug fixed](https://github.com/rust-lang/rust/pull/44362)
* [macOS users can get backtraces again](https://github.com/rust-lang/rust/pull/44251)
* [fix macOs segfault](https://github.com/rust-lang/rust/pull/44384)
* [more metadata methods are queries](https://github.com/rust-lang/rust/pull/44142)
* [`rustc_metadata` no longer needs `DepGraph` handling, remove it](https://github.com/rust-lang/rust/pull/44418) (now querified)
* [rustc now deals correctly with really long linker commands](https://github.com/rust-lang/rust/pull/44094)
* [rustbuild can use hardlinks again, reduces copies](https://github.com/rust-lang/rust/pull/44260) (my small SSD rejoices)
* [cargo: hash dependencies of metadata into lib's metadata](https://github.com/rust-lang/cargo/pull/4469)
* [cargo: don't loop forever on cyclical features](https://github.com/rust-lang/cargo/pull/4473)
* [cargo: support vendoring git repositories](https://github.com/rust-lang/cargo/pull/3992)

## New Contributors

* bgermann
* Douglas Campos
* Ethan Dagner
* Jacob Kiesel
* John Colanduoni
* Lance Roy
* Mark
* MarkMcCaskey
* Max Comstock
* toidiu
* Zaki Manian

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2115: In-band lifetime bindings](https://github.com/rust-lang/rfcs/pull/2115).
* [RFC 2128: Nested groups in imports](https://github.com/rust-lang/rfcs/pull/2128).
* [RFC 2089: Implied bounds](https://github.com/rust-lang/rfcs/pull/2089).
* [RFC 2132: Copy/Clone closures](https://github.com/rust-lang/rfcs/pull/2132).
* [RFC 2086: Allow Irrefutable Patterns in if-let and while-let statements](https://github.com/rust-lang/rfcs/pull/2086).
* [RFC 2043: Add `align_offset` intrinsic and `[T]::align_to` function](https://github.com/rust-lang/rfcs/pull/2043).
* [RFC 2070: stable mechanism to specify the behavior of panic! in no-std applications](https://github.com/rust-lang/rfcs/pull/2070).
* [RFC 2093: Infer `T: 'x` outlives requirements on structs](https://github.com/rust-lang/rfcs/pull/2093).
* [RFC 2133: Compiler-generated Clone impls for arrays and tuples](https://github.com/rust-lang/rfcs/pull/2133).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [`dyn Trait` syntax for trait objects: Take 2](https://github.com/rust-lang/rfcs/pull/2113).
* [disposition: merge] [Introduce a public/private distinction to crate dependencies](https://github.com/rust-lang/rfcs/pull/1977).
* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Autoreferencing `Copy` types](https://github.com/rust-lang/rfcs/pull/2111).
* [disposition: merge] [Attributes for tools, 2.0](https://github.com/rust-lang/rfcs/pull/2103).
* [disposition: merge] [Add impl Trait type alias and variable declarations](https://github.com/rust-lang/rfcs/pull/2071).
* [disposition: merge] [Evolving Rust through Epochs](https://github.com/rust-lang/rfcs/pull/2052).
* [disposition: merge] [`cfg!(target_feature)` and `#[target_feature]`](https://github.com/rust-lang/rfcs/pull/2045).
* [disposition: postpone] [Allow comparisons between integers of different types](https://github.com/rust-lang/rfcs/pull/2021).
* [disposition: merge] [Const generics](https://github.com/rust-lang/rfcs/pull/2000).
* [disposition: merge] [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [disposition: merge] [Change the default URL of doc.rust-lang.org](https://github.com/rust-lang/rfcs/pull/1826).
* [disposition: merge] [Clarify and streamline paths and visibility](https://github.com/rust-lang/rfcs/pull/2126).

## New RFCs

* [Add support to Cargo for alternative registries](https://github.com/rust-lang/rfcs/pull/2141).
* [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).
* [Allow autoderef and autoref in operators](https://github.com/rust-lang/rfcs/pull/2147).
* [Adding unsafe modules and unsafe blocks outside functions](https://github.com/rust-lang/rfcs/pull/2148).
* [Add match/in statements](https://github.com/rust-lang/rfcs/pull/2144).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

We're currently writing up the discussions, we'd love some help. Check out [the tracking issue](https://github.com/rust-lang-nursery/fmt-rfcs/issues/89) for details.

PRs:

* [ranges and blocks](https://github.com/rust-lang-nursery/fmt-rfcs/pull/91)

# Upcoming Events

* [Sep 14. Rust Washington, DC - Hacktember](https://www.meetup.com/RustDC/events/242847065/).
* [Sep 14. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/242459785/).
* [Sep 14. St. Petersburg, Russia - Game Development in Rust](https://www.meetup.com/Rust-%D0%B2-%D0%9F%D0%B8%D1%82%D0%B5%D1%80%D0%B5/events/242219775/).
* [Sep 20. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep 20. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep 20. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/242793549/).
* [Sep 21. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Sep 24. Rust Indonesia - Level up Your Knowledge on Rust](https://www.eventbrite.com/e/level-up-your-knowledge-on-rust-tickets-36755777520).
* [Sep 25. Rust Paris Meetup #38](https://www.meetup.com/Rust-Paris/events/243110057/).
* [Sep 27. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep 27. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep 28. Mozilla Community Dresden - Rust Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/242610304/).
* [Sep 30 - Oct 1. RustFest Zürich](http://zurich.rustfest.eu).
* [Sep 30. Rust Mexico #8: Taller Introductorio a Rust y Rocket](https://www.meetup.com/Rust-MX/events/243334902/).
* [Oct  2 - Oct 3. Impl Days at RustFest Zürich](https://github.com/RustFestEU/blog.rustfest.eu/issues/29).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

>> When programmers are saying that there are a lot of bicycles in code that means that it contains reimplementations of freely available libraries instead of using them
>
> Presumably the metric for this would be bicyclomatic complexity?

— [/u/tomwhoiscontrary on reddit](https://www.reddit.com/r/rust/comments/6zdvza/my_experience_participating_in_highload_cup_re/dmuoydx/).

Thanks to [Matt Ickstadt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/443) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
