Title: This Week in Rust 532
Number: 532
Date: 2024-01-31
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters

### Project/Tooling Updates
* [Palette 0.7.4](https://ogeon.github.io/2024/01/28/palette-0.7.4.html)
* [Fyrox Game Engine 0.33](https://fyrox.rs/blog/post/fyrox-game-engine-0-33/)
* [Two months in Servo: better inline layout, stable Rust, and more!](https://servo.org/blog/2024/01/26/two-months-in-servo/)

* [Ownership and data flow in GPUI](https://zed.dev/blog/gpui-ownership)

### Observations/Thoughts

* [How to benchmark Rust code with Criterion](https://bencher.dev/learn/benchmarking/rust/criterion/)
* [Playing with Nom and parser combinators](https://andreabergia.com/blog/2024/01/playing-with-nom-and-parser-combinators/)
* [Where Does the Time Go? Rust's Problem with Slow Compiles](https://thenewstack.io/where-does-the-time-go-rusts-problem-with-slow-compiles/)
* [ESP32 Embedded Rust at the HAL: I2C Scanner](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-i2c-scanner)
* [We build X.509 chains so you don’t have to](https://blog.trailofbits.com/2024/01/25/we-build-x-509-chains-so-you-dont-have-to/)
* [Process spawning performance in Rust](https://kobzol.github.io/rust/2024/01/28/process-spawning-performance-in-rust.html)
* [Introducing Foundations - our open source Rust service foundation library](https://blog.cloudflare.com/introducing-foundations-our-open-source-rust-service-foundation-library)
* [High performance vector graphic video games](https://simbleau.github.io/rust/graphics/2023/11/20/using-vello-for-video-games.html)

### Rust Walkthroughs
* [Rust Memory Leak Diagnosing Guides using Flame Graphs](https://www.greptime.com/blogs/2024-01-18-memory-leak)
* [WebSockets - The Beginner’s Guide](https://vaktibabat.github.io/posts/websockets/)
* [Writing Cronjobs in Rust](https://www.shuttle.rs/blog/2024/01/24/writing-cronjobs-rust)

* [Fearless concurrency with Rust, cats, and a few Raspberry PIs](https://manuel.bernhardt.io/posts/2024-01-26-rust-fearless-concurrency-cats-raspberry-pi/)

### Research

### Miscellaneous
* [audio] [Arroyo - Micah Wylde, Co-Founder and CEO](https://corrode.dev/podcast/s01e04-arroyo)

## Crate of the Week

This week's crate is [Apache Iceberg Rust](https://github.com/apache/iceberg-rust/), a Rust implementation of a table format for huge analytic datasets.

Thanks to [Renjie Liu](https://users.rust-lang.org/t/crate-of-the-week/2704/1283) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [GreptimeTeam - Fix a minor bug in `join_path` for more elegant code](https://github.com/GreptimeTeam/greptimedb/issues/3212)
* [GreptimeTeam - Add tests for `MetaPeerClientRef` to enhance GreptimeDB's stability](https://github.com/GreptimeTeam/greptimedb/issues/3044)
* [Ockam - Syntax highlighting for fenced code blocks, in command help output, on Linux works](https://github.com/build-trust/ockam/issues/7471)
* [Ockam - Output for `ockam project ticket` is improved and information is not opaque](https://github.com/build-trust/ockam/issues/7478)
* [Ockam - Output for both `ockam project ticket` and `ockam project enroll` is improved, with support for `--output json`](https://github.com/build-trust/ockam/issues/7473)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

409 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-23..2024-01-30

* [`pattern_analysis`: let `ctor_sub_tys` return any Iterator they want](https://github.com/rust-lang/rust/pull/120317)
* [`pattern_analysis`: reuse most of the `DeconstructedPat Debug` impl](https://github.com/rust-lang/rust/pull/120318)
* [add `#[coverage(off)]` to closures introduced by `#[test]` and `#[bench]`](https://github.com/rust-lang/rust/pull/120183)
* [add the `min_exhaustive_patterns` feature gate](https://github.com/rust-lang/rust/pull/118803)
* [add the unstable option to reduce the binary size of dynamic library…](https://github.com/rust-lang/rust/pull/118636)
* [always normalize `LoweredTy` in the new solver](https://github.com/rust-lang/rust/pull/120378)
* [assert that a single scope is passed to `for_scope`](https://github.com/rust-lang/rust/pull/120230)
* [avoid ICE in trait without `dyn` lint](https://github.com/rust-lang/rust/pull/120275)
* [borrow check inline const patterns](https://github.com/rust-lang/rust/pull/120390)
* [classify closure arguments in refutable pattern in argument error](https://github.com/rust-lang/rust/pull/120382)
* [const-eval interning: get rid of type-driven traversal](https://github.com/rust-lang/rust/pull/119044)
* [coverage: dismantle `Instrumentor` and flatten span refinement](https://github.com/rust-lang/rust/pull/120292)
* [coverage: don't instrument `#[automatically_derived]` functions](https://github.com/rust-lang/rust/pull/120185)
* [coverage: never emit improperly-ordered coverage regions](https://github.com/rust-lang/rust/pull/119460)
* [do not normalize closure signature when building `FnOnce` shim](https://github.com/rust-lang/rust/pull/120139)
* [don't call `walk_` functions directly if there is an equivalent `visit_` method](https://github.com/rust-lang/rust/pull/120316)
* [don't fire `OPAQUE_HIDDEN_INFERRED_BOUND` on sized return of AFIT](https://github.com/rust-lang/rust/pull/120360)
* [don't manually resolve async closures in `rustc_resolve`](https://github.com/rust-lang/rust/pull/120322)
* [emit suggestion when trying to write exclusive ranges as `..<`](https://github.com/rust-lang/rust/pull/119342)
* [fix assume and assert in jump threading](https://github.com/rust-lang/rust/pull/120171)
* [fix: correct suggestion arg for impl trait](https://github.com/rust-lang/rust/pull/119957)
* [improve handling of expressions in patterns](https://github.com/rust-lang/rust/pull/118625)
* [improve handling of numbers in `IntoDiagnosticArg`](https://github.com/rust-lang/rust/pull/120398)
* [make `#![allow_internal_unstable(..)]` work with `stmt_expr_attributes`](https://github.com/rust-lang/rust/pull/117420)
* [manually implement derived `NonZero` traits](https://github.com/rust-lang/rust/pull/120160)
* [modify GenericArg and Term structs to use strict provenance rules](https://github.com/rust-lang/rust/pull/119955)
* [move condition enabling the pass to `is_enabled`](https://github.com/rust-lang/rust/pull/120280)
* [normalize field types before checking validity](https://github.com/rust-lang/rust/pull/120277)
* [only assemble alias bound candidates for rigid aliases](https://github.com/rust-lang/rust/pull/119744)
* [properly recover from trailing attr in body](https://github.com/rust-lang/rust/pull/118182)
* [provide more context on recursive `impl` evaluation overflow](https://github.com/rust-lang/rust/pull/119389)
* [riscv32im-risc0-zkvm-elf: add target](https://github.com/rust-lang/rust/pull/117958)
* [scopeTree: remove `destruction_scopes` as unused](https://github.com/rust-lang/rust/pull/120386)
* [split Diagnostics for Uncommon Codepoints: Add List to Display Characters Involved](https://github.com/rust-lang/rust/pull/120259)
* [split tait and impl trait in assoc items logic](https://github.com/rust-lang/rust/pull/119766)
* [stop using derivative in `rustc_pattern_analysis`](https://github.com/rust-lang/rust/pull/120420)
* [subtree sync for `rustc_codegen_cranelift`](https://github.com/rust-lang/rust/pull/120395)
* [suggest `array::from_fn` for array initialization](https://github.com/rust-lang/rust/pull/119805)
* [use `assert_unchecked` instead of `assume` intrinsic in the standard library](https://github.com/rust-lang/rust/pull/119892)
* [interpret: `project_downcast`: do not ICE for uninhabited variants](https://github.com/rust-lang/rust/pull/120367)
* [return a finite number of AllocIds per ConstAllocation in Miri](https://github.com/rust-lang/rust/pull/118336)
* [miri: add `__cxa_thread_atexit_impl` on freebsd](https://github.com/rust-lang/miri/pull/3277)
* [miri: add portable-atomic-util bug to "bugs found" list](https://github.com/rust-lang/miri/pull/3233)
* [miri: freebsd add *stat calls interception support](https://github.com/rust-lang/miri/pull/3181)
* [only use dense bitsets in dataflow analyses](https://github.com/rust-lang/rust/pull/116152)
* [remove all ConstPropNonsense](https://github.com/rust-lang/rust/pull/119627)
* [remove StructuralEq trait](https://github.com/rust-lang/rust/pull/116167)
* [boost iterator `intersperse(_with)` performance](https://github.com/rust-lang/rust/pull/111379)
* [stabilise array methods](https://github.com/rust-lang/rust/pull/103522)
* [std: make `HEAP` initializer never inline](https://github.com/rust-lang/rust/pull/120205)
* [add `AsyncFn` family of traits](https://github.com/rust-lang/rust/pull/119305)
* [add `ErrCode`](https://github.com/rust-lang/rust/pull/119972)
* [add `NonZero*::count_ones`](https://github.com/rust-lang/rust/pull/118326)
* [add `str::Lines::remainder`](https://github.com/rust-lang/rust/pull/107464)
* [adjust Behaviour of `read_dir` and `ReadDir` in Windows Implementation: Check Whether Path to Search In Exists](https://github.com/rust-lang/rust/pull/120373)
* [core: add `From<core::ascii::Char>` implementations](https://github.com/rust-lang/rust/pull/120311)
* [handle out of memory errors in `io:Read::read_to_end()`](https://github.com/rust-lang/rust/pull/117925)
* [impl `From<&[T; N]>` for `Cow<[T]>`](https://github.com/rust-lang/rust/pull/113489)
* [rc,sync: do not create references to uninitialized values](https://github.com/rust-lang/rust/pull/119433)
* [initial implementation of `str::from_raw_parts[_mut]`](https://github.com/rust-lang/rust/pull/119466)
* [remove special-case handling of `vec.split_off(0)`](https://github.com/rust-lang/rust/pull/119917)
* [rewrite the `BTreeMap` cursor API using gaps](https://github.com/rust-lang/rust/pull/118208)
* [specialize `Bytes` on `StdinLock<'_>`](https://github.com/rust-lang/rust/pull/120053)
* [stabilize `slice_group_by`](https://github.com/rust-lang/rust/pull/117678)
* [switch `NonZero` alias direction](https://github.com/rust-lang/rust/pull/120165)
* [regex: make additional prefilter metadata public](https://github.com/rust-lang/regex/pull/1156)
* [cargo: `docs(ref)`: Try to improve reg auth docs](https://github.com/rust-lang/cargo/pull/13351)
* [cargo: `fix(cli)`: Improve errors related to cargo script](https://github.com/rust-lang/cargo/pull/13346)
* [cargo: `fix(config)`: Deprecate non-extension files](https://github.com/rust-lang/cargo/pull/13349)
* [cargo: `refactor(shell)`: Use new fancy anstyle API](https://github.com/rust-lang/cargo/pull/13368)
* [cargo: doc: replace version with `latest` for jobserver link](https://github.com/rust-lang/cargo/pull/13366)
* [cargo: fix list option description starting with uppercase](https://github.com/rust-lang/cargo/pull/13344)
* [cargo: refactor: remove unnecessary Option in `Freshness::Dirty`](https://github.com/rust-lang/cargo/pull/13361)
* [cargo: test: data layout fix for `x86_64-unknown-none-gnu`](https://github.com/rust-lang/cargo/pull/13362)
* [rustfmt: wrap macro that starts with nested body blocks](https://github.com/rust-lang/rustfmt/pull/5582)
* [rustfmt: format diff line to be easily clickable](https://github.com/rust-lang/rustfmt/pull/5971)
* [clippy: add `to_string_trait_impl` lint](https://github.com/rust-lang/rust-clippy/pull/12122)
* [clippy: add new `unnecessary_result_map_or_else` lint](https://github.com/rust-lang/rust-clippy/pull/12169)
* [clippy: false positive: `needless_return_with_question_mark` with implicit Error Conversion](https://github.com/rust-lang/rust-clippy/pull/12021)
* [clippy: `redundant_closure_for_method_calls` Suggest relative paths for local modules](https://github.com/rust-lang/rust-clippy/pull/11370)
* [clippy: `multiple_crate_versions`: add a configuration option for allowed duplicate crates](https://github.com/rust-lang/rust-clippy/pull/12179)
* [clippy: `never_loop`: recognize desugared `try` blocks](https://github.com/rust-lang/rust-clippy/pull/12206)
* [clippy: avoid linting redundant closure when callee is marked `#[track_caller]`](https://github.com/rust-lang/rust-clippy/pull/12202)
* [clippy: don't warn about modulo arithmetic when comparing to zero](https://github.com/rust-lang/rust-clippy/pull/12178)
* [clippy: assert* in multi-condition after unrolling will cause lint `nonminimal_bool` emit warning](https://github.com/rust-lang/rust-clippy/pull/12083)
* [clippy: fix incorrect suggestions generated by `manual_retain` lint](https://github.com/rust-lang/rust-clippy/pull/12084)
* [clippy: false positive in `redundant_closure_call` when closures are passed to macros](https://github.com/rust-lang/rust-clippy/pull/12082)
* [clippy: suggest existing configuration option if one is found](https://github.com/rust-lang/rust-clippy/pull/12180)
* [clippy: warn if an item coming from more recent version than MSRV is used](https://github.com/rust-lang/rust-clippy/pull/12160)
* [rust-analyzer: add postfix completion for let else](https://github.com/rust-lang/rust-analyzer/pull/15730)
* [rust-analyzer: filter out cfg-disabled fields when lowering record patterns](https://github.com/rust-lang/rust-analyzer/pull/16427)
* [rust-analyzer: replaced `adjusted_display_range` with `adjusted_display_range_new` in `mismatched_arg_count`](https://github.com/rust-lang/rust-analyzer/pull/16431)

### Rust Compiler Performance Triage

This was a very quiet week with only one PR having any real impact on overall compiler performance. The removal of the internal `StructuralEq` trait saw a roughly 0.4% improvement on average across nearly 50 real-world benchmarks. 

Triage done by **@rylev**.
Revision range: [d6b151fc7..5c9c3c7](https://perf.rust-lang.org/?start=d6b151fc77e213bf637db0f12c1965ace3ffe255&end=5c9c3c7871d603ba13d38372830eca0c9013e575&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.3%, 0.7%]   | 5     |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.2%, 1.4%]   | 10    |
| Improvements ✅ <br /> (primary)   | -0.5% | [-1.5%, -0.2%] | 48    |
| Improvements ✅ <br /> (secondary) | -2.3% | [-7.7%, -0.4%] | 36    |
| All ❌✅ (primary)                 | -0.4% | [-1.5%, 0.7%]  | 53    |


0 Regressions, 4 Improvements, 4 Mixed; 3 of them in rollups
37 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/54a18b2515048a5695aa61e79cbf12b5ed9a118d/triage/2024-01-30.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Avoid non-local definitions in functions](https://github.com/rust-lang/rfcs/pull/3373)
* [RFC: constants in patterns](https://github.com/rust-lang/rfcs/pull/3535)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Include `Future` and `IntoFuture` in the 2024 prelude](https://github.com/rust-lang/rfcs/pull/3509)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [static mut: allow mutable reference to arbitrary types, not just slices and arrays](https://github.com/rust-lang/rust/pull/117614)
* [disposition: merge] [Make it so that async-fn-in-trait is compatible with a concrete future in implementation](https://github.com/rust-lang/rust/pull/120103)
* [disposition: merge] [Decision: semantics of the `#[expect]` attribute](https://github.com/rust-lang/rust/issues/115980)
* [disposition: merge] [style-guide: When breaking binops handle multi-line first operand better](https://github.com/rust-lang/rust/pull/119838)
* [disposition: merge] [style-guide: Tweak `Cargo.toml` formatting to not put description last](https://github.com/rust-lang/rust/pull/120072)
* [disposition: merge] [style-guide: Format single associated type where clauses on the same line](https://github.com/rust-lang/rust/pull/119515)
* [disposition: merge] [PartialEq, PartialOrd: update and synchronize handling of transitive chains](https://github.com/rust-lang/rust/pull/115386)
* [disposition: merge] [`std::error::Error` -> Trait Implementations: lifetimes consistency improvement](https://github.com/rust-lang/rust/pull/113833)


### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Deprecate then remove static mut](https://github.com/rust-lang/rfcs/pull/3560)
* [RFC: Rust Has Provenance](https://github.com/rust-lang/rfcs/pull/3559)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2024-01-31 - 2024-02-28 🦀

### Virtual

* 2024-01-31 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club launch!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298563633/)
* 2024-02-01 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/) - [Stream](https://www.youtube.com/@bcnrust)
* 2024-02-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457951/)
* 2024-02-03 | Virtual + In-person (Brussels, BE) | [FOSDEM 2024](https://fosdem.org/2024/)
    * [**FOSDEM Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/)
* 2024-02-03 | Virtual (Kampala, UG) | [Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)
* 2024-02-04 | Virtual | [Rust Maven](https://meet-os.com/group/1)
    * [**Web development with Rocket - In English**](https://meet-os.com/event/1)
* 2024-02-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Ezra Singh - How Rust Saved My Eyes**](https://www.meetup.com/indyrs/events/298641965/)
* 2024-02-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 2024-02-08 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 2024-02-10 | Virtual (Krakow, PL) | [Stacja IT Kraków](https://www.meetup.com/stacja-it-krakow/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-krakow/events/298303129/)
* 2024-02-10 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 2024-02-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341575/)
* 2024-02-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 2024-02-15 |  Virtual + In person (Praha, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 2024-02-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)

### Asia

* 2024-02-10 | Hyderabad, IN | [Rust Language Hyderabad](https://www.meetup.com/rust-hyderabad/)
    * [**Rust Language Develope BootCamp**](https://www.meetup.com/rust-hyderabad/events/298687498/)

### Europe

* 2024-02-01 | Hybrid (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 2024-02-03 | Brussels, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**FOSDEM '24 Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/) | [**Rust Aarhus FOSDEM Meetup**](https://www.meetup.com/rust-aarhus/events/295946777/)
* 2024-02-03 | Nürnberg, BY, DE | [Paessler Rust Camp 2024](https://www.meetup.com/paessler-rust-camp-2024/)
    * [**Paessler Rust Camp 2024**](https://www.meetup.com/paessler-rust-camp-2024/events/298603948)
* 2024-02-05 | Brussels, BE | [Belgium Rust user group](https://www.meetup.com/fr-FR/belgium-rust-user-group/)
    * [**Post-FOSDEM Rust meetup @ Vrije Universiteit Brussel**](https://www.meetup.com/fr-FR/belgium-rust-user-group/events/298754029/)
* 2024-02-06 | Bremen, DE | [Rust Meetup Bremen](https://www.linkedin.com/company/rust-meetup-bremen/)
    * [**Rust Meetup Bremen [1]**](https://www.linkedin.com/events/rustmeetupbremen-17153350929486868481/)
* 2024-02-07 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Embedded Abstractions**](https://www.meetup.com/rustcologne/events/298913201/) | [**Event page**](https://rust.cologne/2024/02/07/embedded-hal.html)
* 2024-02-07 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 2024-02-08 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀**](https://www.meetup.com/rust-bern/events/298488858/)
* 2024-02-15 | Praha, CZ - Virtual + In-person | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 2024-02-22 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)

### North America

* 2024-02-07 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/297635028/)
* 2024-02-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**BEAST: Recreating a classic DOS terminal game in Rust**](https://www.meetup.com/utah-rust/events/298888955/)
* 2024-02-12 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Open Source Contrib Hackathon & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760219/)
* 2024-02-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer**](https://www.meetup.com/rust-nyc/events/298593474/)
* 2024-02-13 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564994/)
* 2024-02-15 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, Feb 15**](https://www.meetup.com/bostonrust/events/297635043/)
* 2024-02-15 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631774/)
* 2024-02-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/298603354/)
* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)

### Oceania

* 2024-02-06 | Perth, WA, AU | [Perth Rust Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust Feb 2024 Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)
* 2024-02-27 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/297650401/)
* 2024-02-27 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> The sheer stability of this program is what made me use rust for everything going forward. The social-service has a 100% uptime for almost 2.5 years now. It’s processed 12.9TB of traffic and is still using 1.5mb of ram just like the day we ran it 2.5 years ago. The resource usage is so low it brings tears to my eyes. As someone who came from Java, the lack of OOM errors or GC problems has been a huge benefit of rust and I don’t ever see myself using any other programming language. I’m a big fan of the mindset “build it once, but build it the right way” which is why rust is always my choice.

– [/u/Tiflotin on /r/rust](https://reddit.com/r/rust/comments/1ach3ir/what_were_some_of_the_first_useful_applications/)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1519) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
