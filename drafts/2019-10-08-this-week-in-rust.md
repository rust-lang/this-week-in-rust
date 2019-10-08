Title: This Week in Rust 307
Number: 307
Date: 2019-10-08
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

* [vector: Update `stdin` source to use a thread instead of using the tokio version](https://github.com/timberio/vector/issues/932). Vector is a high-performance observability data router.
* [mundane: Build docs.rs documentation with all features enabled](https://github.com/google/mundane/issues/21). Mundane is a Rust cryptography library backed by BoringSSL.
* [rustfm-scrobble: Issues for Hacktoberfest](https://github.com/bobbo/rustfm-scrobble/issues?q=is%3Aissue+is%3Aopen+label%3AHacktoberfest). rustfm-scrobble is a Last.fm scrobble API library in Rust.
* [Tokamak is looking for maintainers](https://github.com/vertexclique/tokamak/issues/91). Tokamak is a Rust IDE for Atom.
* [Bastion is looking for maintainers](https://github.com/bastion-rs/bastion/issues/32). Bastion is a fault-tolerant runtime for Rust applications

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

* [RFC 2523: `#[cfg(accessible(..) / version(..))]`](https://github.com/rust-lang/rfcs/pull/2523).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Minimum supported Rust version](https://github.com/rust-lang/rfcs/pull/2495).
* [disposition: close] [Project-based Examples for Cargo Projects](https://github.com/rust-lang/rfcs/pull/2517).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for RFC 2008: Future-proofing enums/structs with #[non_exhaustive] attribute](https://github.com/rust-lang/rust/issues/44109).
* [disposition: merge] [Stabilize `Option::as_deref` and `Option::as_deref_mut`](https://github.com/rust-lang/rust/pull/64708).
* [disposition: merge] [Stabilize mem::take (mem_take)](https://github.com/rust-lang/rust/pull/64716).
* [disposition: merge] [Stabilize `slice::repeat` (feature `repeat_generic_slice`)](https://github.com/rust-lang/rust/pull/64877).

## New RFCs

* [RFC 1201 ammendments: Naked function corrections](https://github.com/rust-lang/rfcs/pull/2774).

# Upcoming Events

### Asia Pacific

* [Oct  7. Auckland, NZ - Rust AKL - Introduction to Rust (session 2 of 3)](https://www.meetup.com/rust-akl/events/259481147/).

### Europe

* [Oct  4. Toulouse, FR - Toulouse Rust Meetup - Future<Output = Rust>](https://www.meetup.com/Toulouse-Rust-Meetup/events/264780064).
* [Oct  4. Darmstadt, DE - Hacktoberfest for Rustaceans](https://www.meetup.com/Rust-Rhein-Main/events/265052778).
* [Oct  5. Kharkiv, UA - PeerLab Kharkiv #Rust: AsyncIO](https://dou.ua/calendar/28904/).
* [Oct  9. Zagreb, HR - impl Zagreb for Rust: Rust, FFmpeg i TensorFlow](https://www.meetup.com/Zagreb-Rust-Meetup/events/265307360/).
* [Oct 10. Helsinki, FI - Finland Rust-lang Group - October meetup](https://www.meetup.com/Finland-Rust-Meetup/events/265091401/).
* [Oct 10. Warsaw, PL - Rust Warsaw - reboot](https://www.meetup.com/Rust-Warsaw/events/265091321/).
* [Oct 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbvb/).

### North America

* [Oct  5. Cleveland, OH, US - Cleveland RustBridge](https://coffee.dev/rustbridge).
* [Oct  8. Detroit, MI, US - Detroit Rust - Diving into Rust web frameworks](https://www.meetup.com/detroitrust/events/265090754/).
* [Oct  9. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgryznbmb/).
* [Oct 10. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryznbnb/).
* [Oct 10. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/264768938).
* [Oct 16. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbvb/).
* [Oct 18 & 19. Dayton, OH, US - Rust Belt Rust](https://www.rust-belt-rust.com/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Official /r/rust "Who's Hiring" thread for job-seekers and job-offerers [Rust 1.38]](https://www.reddit.com/r/rust/comments/d9l79d/official_rrust_whos_hiring_thread_for_jobseekers/).
* [Software engineer (New grad) at Embark Studios, Stockholm, SE](https://www.embark-studios.com/jobs/278026-software-engineer-new-grad).
* [Multiple Rust jobs at Matter Labs (Berlin, Kiev, remote](https://medium.com/matter-labs/software-engineering-jobs-at-matter-labs-c456d01b2a02).
* [Rust internship at Tsuru Capital, Tokyo, JP](https://www.reddit.com/r/rust/comments/db7910/job_rust_internship_in_japan/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> "Rust compilation is so slow that I can fix the bugs while it still compiles the crates"

– [Rustafarian on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/705)

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
