Title: This Week in Rust 308
Number: 308
Date: 2019-10-15
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

This week's crate is [algebraics](https://crates.io/crates/algebraics), a pure-Rust algebraic numbers library for infinite-precision computation.

Thanks to [Jacob Lifshay](https://users.rust-lang.org/t/crate-of-the-week/2704/629) and [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/639) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hacktoberfest issues from EmbarkStudios](https://github.com/search?q=user:EmbarkStudios+label:hacktoberfest+state:open).
* [rustc: Deprecation warning emitted from derive without a span](https://github.com/rust-lang/rust/issues/56195).
* [async-std: Add `task::yield_now`](https://github.com/async-rs/async-std/issues/290).
* [async-std: Add `sync::CondVar`](https://github.com/async-rs/async-std/issues/217).
* [async-std: Add `path::{Path,PathBuf}`](https://github.com/async-rs/async-std/issues/183).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

338 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-09-30..2019-10-07

* [Only add sanitizer runtimes when linking an executable](https://github.com/rust-lang/rust/pull/64780)
* [LLVM/wasm: Fix conflict between ret legalization and sjlj](https://github.com/rust-lang/llvm-project/pull/25)
* [LLVM/wasm: Restore defaults for stores per memop](https://github.com/rust-lang/llvm-project/pull/24)
* [When encountering chained operators use heuristics to recover from bad turbofish](https://github.com/rust-lang/rust/pull/64909)
* [Make visit projection iterative](https://github.com/rust-lang/rust/pull/65056)
* [Make re-export collection deterministic](https://github.com/rust-lang/rust/pull/65043)
* [Deduplicate closure type errors](https://github.com/rust-lang/rust/pull/64937)
* [Suggest fix for type mismatch based on operator precendence](https://github.com/rust-lang/rust/pull/64933)
* [syntax: improve parameter without type suggestions](https://github.com/rust-lang/rust/pull/64959)
* [Polonius: use the fx hasher when interning](https://github.com/rust-lang/polonius/pull/131)
* [extract expected return type for async fn generators](https://github.com/rust-lang/rust/pull/64999)
* [async/await: improve not-send errors](https://github.com/rust-lang/rust/pull/64895)
* [const-prop: Fix ICE when trying to eval polymorphic promoted MIR](https://github.com/rust-lang/rust/pull/65066)
* [const-prop: Correctly handle locals that can't be propagated](https://github.com/rust-lang/rust/pull/64991)
* [Still more `ObligationForest` improvements](https://github.com/rust-lang/rust/pull/64805)
* [Avoid `chain()` in `find_constraint_paths_between_regions()`](https://github.com/rust-lang/rust/pull/64801)
* [Optimize integral pattern matching](https://github.com/rust-lang/rust/pull/65089)
* [proc_macro API: Expose `macro_rules` hygiene](https://github.com/rust-lang/rust/pull/64690)
* [Deprecate `#![plugin]` & `#[plugin_registrar]`](https://github.com/rust-lang/rust/pull/64675)
* [metadata: Some crate loading cleanup](https://github.com/rust-lang/rust/pull/65026)
* [Do not ICE when dereferencing non-Copy raw pointer](https://github.com/rust-lang/rust/pull/65011)
* [Fix zebra-striping in generic dataflow visualization](https://github.com/rust-lang/rust/pull/64974)
* [Don't mark borrows of zero-sized arrays as indirectly mutable](https://github.com/rust-lang/rust/pull/64967)
* [Deny specializing items not in the parent impl](https://github.com/rust-lang/rust/pull/64564)
* [Add feature gate for raw_dylib](https://github.com/rust-lang/rust/pull/63948)
* [Stabilize macros in some more positions](https://github.com/rust-lang/rust/pull/63931)
* [syntax: Support modern attribute syntax in the `meta` matcher](https://github.com/rust-lang/rust/pull/63674)
* [Use `PlaceBuilder` to avoid a lot of slice → vec → slice convertions](https://github.com/rust-lang/rust/pull/64922)
* [use `try_fold` instead of `try_for_each` to reduce compile time](https://github.com/rust-lang/rust/pull/64885)
* [Stabilize `UdpSocket::peer_addr`](https://github.com/rust-lang/rust/pull/64728)
* [Stabilize `Option::as_deref` and `Option::as_deref_mut`](https://github.com/rust-lang/rust/pull/64708)
* [Stabilize `todo!(..)` macro](https://github.com/rust-lang/rust/pull/61879)
* [hashbrown: Add `RustcVacantEntry::insert_entry`](https://github.com/rust-lang/hashbrown/pull/118)
* [`BTreeSet` `intersection`, `is_subset` & `difference` optimizations](https://github.com/rust-lang/rust/pull/64820)
* [Implement `Clone::clone_from` for `LinkedList`](https://github.com/rust-lang/rust/pull/64975)
* [Inline {`min`,`max`}`_value` even in debug builds](https://github.com/rust-lang/rust/pull/64941)
* [cargo: Support for named profiles](https://github.com/rust-lang/cargo/pull/6989) (RFC 2678)
* [cargo: Disable preserving mtimes on archives](https://github.com/rust-lang/cargo/pull/7465)
* [rustup: Cleaned up error messages](https://github.com/rust-lang/rustup.rs/pull/2035)
* [rustbuild: Make all alt builders produce parallel-enabled compilers](https://github.com/rust-lang/rust/pull/64722)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize proc macros generating `macro_rules` items](https://github.com/rust-lang/rust/pull/64035).
* [disposition: merge] [Stabilize `slice::repeat` (feature `repeat_generic_slice`)](https://github.com/rust-lang/rust/pull/64877).

## New RFCs

* [Variadic tuples](https://github.com/rust-lang/rfcs/pull/2775).
* [Scope prints in diagnostics](https://github.com/rust-lang/rfcs/pull/2777).
* [Initial cargo-plugin-fields](https://github.com/rust-lang/rfcs/pull/2776).

# Upcoming Events

### Europe

* [Oct 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbvb/).
* [Oct 17. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/265509739/)
* [Oct 18. Stuttgart, DE - Rust Meetup Hack and Learn](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/265526369/).
* [Oct 23. Stockholm, SE - Stockholm Rust - Rust Meetup @Embark Studios](https://www.meetup.com/Stockholm-Rust/events/265322700/).
* [Oct 24. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/265207841).

### North America

* [Oct 16. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbvb/).
* [Oct 16. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryznbfc/).
* [Oct 18 & 19. Dayton, OH, US - Rust Belt Rust](https://www.rust-belt-rust.com/).
* [Oct 23. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/265043014/).
* [Oct 30. San Francisco, US - Rust in Blockchain Workshop Day (SFBW)](https://www.meetup.com/Rust-in-Blockchain-San-Francisco/events/265362152/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Blockchain Engineer at Nervos, Hangzhou, CN (Remote available)](https://angel.co/company/nervos-1/jobs/589230-senior-blockchain-engineer).
* [Rust/Core Developer at Parity, Berlin, DE (Remote available)](https://www.parity.io/jobs/#berlin-rust-core-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> "Rust compilation is so slow that I can fix the bugs while it still compiles the crates"

– [Rustafarian on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/705)

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
