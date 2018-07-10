Title: This Week in Rust 242
Number: 242
Date: 2018-07-10
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

This week's crate is [datafrog](https://crates.io/crates/datafrog), the lightweight embeddable datalog engine that powers Rust's non-lexical lifetimes (NLL). Thanks to [Jules Kerssemakers](https://users.rust-lang.org/u/juleskers) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [good first issue] [wasm-bindgen: Expose bindings to ALL the global JS things](https://github.com/rustwasm/wasm-bindgen/issues/275).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

174 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-06-25..2018-07-02

* [did you mean to block nightlies on clippy?](https://github.com/rust-lang/rust/pull/51122) (a big leap forward on the way to stable clippy!)
* [llvm: add DWARF for discriminated unions](https://github.com/rust-lang/llvm/pull/118)
* [libc: simplify the stdbuild section](https://github.com/rust-lang/libc/pull/1019)
* [allow irrefutable `let` patterns](https://github.com/rust-lang/rust/pull/49469) (RFC [#2086](https://rust-lang.github.io/rfcs/2086-allow-if-let-irrefutables.html))
* [loosened rules involving statics mentioning other statics](https://github.com/rust-lang/rust/pull/51110)
* [obligation forest cleanup](https://github.com/rust-lang/rust/pull/51613)
* [suggestion for 'static impl Trait return](https://github.com/rust-lang/rust/pull/51444)
* [fix incorrect type mismatch label pointing at return type](https://github.com/rust-lang/rust/pull/46720)
* [use literal span for concrete type suggestion](https://github.com/rust-lang/rust/pull/51920)
* [improve the error message when `#[panic_implementation]` is missing](https://github.com/rust-lang/rust/pull/51921)
* [point to lifetime spans on lifetime errors](https://github.com/rust-lang/rust/pull/51862)
* [add modifier keyword spans to hir::Visibility; improve unreachable-pub, private-no-mangle lint suggestions](https://github.com/rust-lang/rust/pull/51866)
* [provide existing ref suggestions for more E0308 errors](https://github.com/rust-lang/rust/pull/51822)
* [`Self` in where clauses may not be object safe](https://github.com/rust-lang/rust/pull/50966)
* [suggest correct comparison against negative literal](https://github.com/rust-lang/rust/pull/51883)
* [lint to favor `..=` over `...` range patterns; migrate to `..=` throughout codebase](https://github.com/rust-lang/rust/pull/51149)
* [detect overflows of non u32 shifts](https://github.com/rust-lang/rust/pull/51839)
* [`HirId`-ification, continued](https://github.com/rust-lang/rust/pull/51321)
* [optimize `places_conflict` to avoid complex vectors etc.](https://github.com/rust-lang/rust/pull/51849)
* [NLL: better move errors](https://github.com/rust-lang/rust/pull/51729)
* [NLL: bad error message when converting anonymous lifetime to `'static`](https://github.com/rust-lang/rust/pull/51536)
* [NLL: introduce dirty list to liveness, eliminate `ins` vector](https://github.com/rust-lang/rust/pull/51896)
* [convert NLL ops to caches](https://github.com/rust-lang/rust/pull/51538)
* [avoid needless allocations in `liveness_of_locals`](https://github.com/rust-lang/rust/pull/51869)
* [speed up compilation of large constant arrays](https://github.com/rust-lang/rust/pull/51833)
* [implement `#[macro_export(local_inner_macros)]`](https://github.com/rust-lang/rust/pull/51496)
* [use `Ident`s in HIR and remove emulation of hygiene with gensyms](https://github.com/rust-lang/rust/pull/51492)
* [always check type_dependent_defs](https://github.com/rust-lang/rust/pull/51882)
* [fix ICEs when using continue as an array length inside closures (inside loop conditions)](https://github.com/rust-lang/rust/pull/51731)
* [add error for using null characters in `#[export_name]`](https://github.com/rust-lang/rust/pull/51747)
* [don't inspect the generated existential type items](https://github.com/rust-lang/rust/pull/51773)
* [don't ICE when performing `lower_pattern_unadjusted` on a `TyError`](https://github.com/rust-lang/rust/pull/51789)
* [make the public API of the `alloc` crate a subset of `std`](https://github.com/rust-lang/rust/pull/51569)
* [new safe associated functions for `PinMut`](https://github.com/rust-lang/rust/pull/51730)
* [make custom trait object for `Future` generic](https://github.com/rust-lang/rust/pull/51944)
* [optimize sum of Durations by using custom function](https://github.com/rust-lang/rust/pull/51598)
* [add `str::split_ascii_whitespace`](https://github.com/rust-lang/rust/pull/49987)
* [`Arc`: remove unused allocation from `Weak::new()`](https://github.com/rust-lang/rust/pull/50357)
* [make `BTreeMap::clone()` not allocate when cloning an empty tree](https://github.com/rust-lang/rust/pull/51893)
* [make `FileMap::`{`lines`, `multibyte_chars`, `non_narrow_chars`} non-mutable](https://github.com/rust-lang/rust/pull/50997)
* [implement `PartialEq` between `&str` and `OsString`](https://github.com/rust-lang/rust/pull/51178)
* [`park`/`park_timeout`: prohibit spurious wakeups in next `park`](https://github.com/rust-lang/rust/pull/51290)
* [fix possibly endless loop in `ReadDir` iterator](https://github.com/rust-lang/rust/pull/50630)
* [stabilize `Iterator::flatten`](https://github.com/rust-lang/rust/pull/51511)
* [stabilize `to_bytes` and `from_bytes` for integers](https://github.com/rust-lang/rust/pull/51835)
* [cargo: remove all 4 `Rc` clones in `min_candidates`](https://github.com/rust-lang/cargo/pull/5625)
* [cargo: display a one line progress of what crates are currently built](https://github.com/rust-lang/cargo/pull/5620)
* [cargo: remove redundant hashmap](https://github.com/rust-lang/cargo/pull/5619)
* [rustdoc: minify css](https://github.com/rust-lang/rust/pull/51791)
* [crates.io: `dyn` all the things](https://github.com/rust-lang/crates.io/pull/1441)
* [crates.io: forbid tarballs with hard links being uploaded](https://github.com/rust-lang/crates.io/pull/1448)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2345: Allow panicking in constants](https://github.com/rust-lang/rfcs/pull/2345).
* [RFC 2296: Add `Option::replace` to the core library](https://github.com/rust-lang/rfcs/pull/2296).
* [RFC 2344: Allow `loop` in constant evaluation](https://github.com/rust-lang/rfcs/pull/2344).
* [RFC 2302: Tuple struct construction with `Self(v1, v2, ..)`](https://github.com/rust-lang/rfcs/pull/2302).
* [RFC 2397: Introduce `#[do_not_recommend]` to control errors for trait impls](https://github.com/rust-lang/rfcs/pull/2397).
* [Amend RFC 0430: Allow underscores between numbers in CamelCase names](https://github.com/rust-lang/rfcs/pull/2478).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: postpone] [String reflection](https://github.com/rust-lang/rfcs/pull/2233).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [Tracking issue for `Rc::downcast` (`rc_downcast` feature)](https://github.com/rust-lang/rust/issues/44608).
* [disposition: merge] [Respect -Z no-verify during LTO](https://github.com/rust-lang/rust/pull/51230).
* [disposition: merge] [impl Clone for Box<CStr>, Box<OsStr>, Box<Path>](https://github.com/rust-lang/rust/pull/51912).
* [disposition: merge] [Add `#[repr(transparent)]` to some libcore types](https://github.com/rust-lang/rust/pull/51395).
* [disposition: close] [Add `TryFrom<{integer}>` for `bool`](https://github.com/rust-lang/rust/pull/50597).

## New RFCs

* [Existential types with external definition](https://github.com/rust-lang/rfcs/pull/2492).
* [Add a replace_with method to Option](https://github.com/rust-lang/rfcs/pull/2490).

# Upcoming Events

### Online

* [Jul 11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jul 17. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jul 18. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jul 18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).

### Asia

* [Jul 19. Tokyo, JP - Rust Meetup #9 in Shibuya](https://www.meetup.com/Tokyo-Rust-Meetup/events/252145423/).

### Europe

* [Jul  6. Darmstadt, DE - Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/251928672).
* [Jul  9. Karlsruhe, DE - Rust Meetup](https://www.meetup.com/de-DE/Rust-Hack-Learn-Karlsruhe/events/252267570/)
* [Jul 11. Zurich, CH - Actix.rs - July Meetup](https://www.meetup.com/Rust-Zurich/events/250386292/).
* [Jul 11. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/xkdlvpyxkbpb/).
* [Jul 19. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxkbzb/).
* [Jul 20. Barcelona, ES - BcnRust 1st meetup with Ashley Williams & Steve Klabnik](https://www.meetup.com/es-ES/BcnRust/events/251237895/).


### North America

* [Jul  8. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkblb/).
* [Jul  9. Seattle, US - Monthly Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxkbmb/).
* [Jul 11. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbkc/).
* [Jul 12. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxkbqb/).
* [Jul 12. Utah Valley, Utah, US - Utah Rust - Monthly Meeting](https://www.meetup.com/utahrust/events/251816575/).
* [Jul 15. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkbtb/).
* [Jul 18. Standford, US - Rust Bay Area - [@ Stanford] Munching Macros and Facebook's Mononoke](https://www.meetup.com/Rust-Bay-Area/events/251862242/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).** Registration is now open.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. San Francisco, US](https://news.ycombinator.com/item?id=17442861).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Freedom to shoot yourself in the foot is not a rust marketing point 😉

– [eugene2k](https://users.rust-lang.org/u/eugene2k) on [rust-users](https://users.rust-lang.org/t/why-cant-i-increment-a-variable-like-this/18287/14)

Thanks to [DPC](https://users.rust-lang.org/u/dylan.dpc) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
