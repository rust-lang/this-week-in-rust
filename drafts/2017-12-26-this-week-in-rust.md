Title: This Week in Rust 214
Number: 214
Date: 2017-12-26
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

- [podcast] [New Rustacean: Increasing Rust's Reach – Anna Liao](http://www.newrustacean.com/show_notes/interview/irr_2017/anna_liao/index.html), on Anna's experience learning Rust while porting a Raspberry Pi Python project as part of the Increasing Rust’s Reach 2017 program.

# Crate of the Week

This week's crate is [crossbeam-channel](https://crates.io/crates/crossbeam-channel), a crate that improves multi-producer multi-consumer channels compared to what the standard library offers. Thanks to [leodasvacas](https://users.rust-lang.org/u/leodasvacas) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Use cargo-contribute to find issues in your dependencies to contribute to](https://github.com/Xion/cargo-contribute).
* [Rusoto, an AWS SDK, is looking for maintainers](https://github.com/rusoto/rusoto/issues/593).
* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [good first issue] [Criterion.rs: Replace rustc_serialize with Serde](https://github.com/japaric/criterion.rs/issues/91).
* [good first issue] [Criterion.rs: Replace Floaty with num-traits](https://github.com/japaric/criterion.rs/issues/95).
* [medium] [allocators-rs: object-alloc-test: Finish `corruption::mapped::is_mapped_range` on Windows](https://github.com/ezrosent/allocators-rs/issues/137).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

118 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-12-18..2017-12-25

* [`feature(nll)`](https://github.com/rust-lang/rust/pull/46862)
* [prevent unwinding past FFI boundaries](Prevent unwinding past FFI boundaries) (finally!)
* [prevent rustc overwriting input files](https://github.com/rust-lang/rust/pull/46814)
* [`-C incremental`](https://github.com/rust-lang/rust/pull/46751) and
  [enable incremental by default](https://github.com/rust-lang/cargo/pull/4817) (this effectively stabilizes incremental compilation)
* [do not emit type errors on recovered blocks](https://github.com/rust-lang/rust/pull/46732)
* [kill borrows on a local variable whenever we assign over it](https://github.com/rust-lang/rust/pull/46752)
* [MIR borrowck: no "move occurs because `X` is not Copy` error](https://github.com/rust-lang/rust/pull/46949)
* [MIR: terminate unreachable blocks in `construct_const`](https://github.com/rust-lang/rust/pull/46877)
* [rustc: set release mode cgus to 16 by default](https://github.com/rust-lang/rust/pull/46910)
* [rustc: sort codegen units before merging](https://github.com/rust-lang/rust/pull/46918) (fix non-determinism)
* [rustc: do not raise the alignment of optimized enums to the niche's alignment](https://github.com/rust-lang/rust/pull/46809)
* [rustc: ensure optimized enums have a properly aligned size](https://github.com/rust-lang/rust/pull/46808)
* [rustc: work around `DICompileUnit` bugs in LLVM](https://github.com/rust-lang/rust/pull/46772)
* [fix ICE when calling non-functions within closures](https://github.com/rust-lang/rust/pull/46780)
* [work towards thread safety in rustc](https://github.com/rust-lang/rust/pull/46779)
* [fix -Z lower_128bit_ops handling of statics](https://github.com/rust-lang/rust/pull/46583)
* [type privacy polishing](https://github.com/rust-lang/rust/pull/46083)
* [only mark unions as uninhabited if all of their fields are uninhabited](https://github.com/rust-lang/rust/pull/46859)
* [`visible_parent_map` now sorts by crate num](https://github.com/rust-lang/rust/pull/46838)
* [convert warning about `*const _` to a future-compat lint](https://github.com/rust-lang/rust/pull/46914)
* [lint against single-use lifetime names](https://github.com/rust-lang/rust/pull/46441)
* [set the dwarf linkage_name to the mangled name](https://github.com/rust-lang/rust/pull/46899)
* [fix debuginfo scoping of let-statements](https://github.com/rust-lang/rust/pull/46896)
* [add a feature gate for nested uses of `impl Trait`](https://github.com/rust-lang/rust/pull/46888)
* [ensure separate activations only occur for assignments to locals](https://github.com/rust-lang/rust/pull/46887)
* [move `PhantomData<T>` from `Shared<T>` to users of both `Shared` and `#[may_dangle]`](https://github.com/rust-lang/rust/pull/46749)
* [add Hash impl for SystemTime and Instant](https://github.com/rust-lang/rust/pull/46828)
* [capture `Command` environment at spawn](https://github.com/rust-lang/rust/pull/46789)
* [add more Duration methods for consistency](https://github.com/rust-lang/rust/pull/46508)
* [reject superfluous `::` in IPv6 addresses](https://github.com/rust-lang/rust/pull/46671)
* [stablize `RefCell::`{`replace`, `swap`}](https://github.com/rust-lang/rust/pull/46517)
* [rustdoc: const-eval array lengths](https://github.com/rust-lang/rust/pull/46894)

## New Contributors

* David Teller
* Felix Schütt
* Nika Layzell
* qres
* varkor

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

* [Change impls of `PartialEq` and friends in libstd to be more generic](https://github.com/rust-lang/rfcs/pull/2245).
* [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).

# Upcoming Events

* [Dec 21. Cambridge Rust Meetup #6](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnywqbcc/).
* [Dec 27. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Dec 27. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Dec 28. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jan  2. Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxcbdb/).
* [Jan  3. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxcbfb/).
* [Jan  3. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jan  3. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at ANIXE Wrocław Poland](http://anixe.pl/jobs/rust_dev/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
