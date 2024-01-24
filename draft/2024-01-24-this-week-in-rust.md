Title: This Week in Rust 531
Number: 531
Date: 2024-01-24
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
* [Q4 2023 Recap from Rebecca Rumbul](https://foundation.rust-lang.org/news/q4-2023-recap-from-rebecca-rumbul/)

### Newsletters

### Project/Tooling Updates
* [Ruffle 2023 in review](https://ruffle.rs/blog/2024/01/14/2023-in-review)
* [Four challenges cargo-semver-checks has yet to tackle](https://predr.ag/blog/four-challenges-cargo-semver-checks-has-yet-to-tackle/)
* [rustc_codegen_gcc: Progress Report #29](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-29)
* [Roadmap for the Xilem backend in 2024](https://linebender.org/blog/xilem-backend-roadmap/#2)
* [rust-analyzer changelog #217](https://rust-analyzer.github.io/thisweek/2024/01/22/changelog-217.html)

### Observations/Thoughts
* [Making Rust binaries smaller by default](https://kobzol.github.io/rust/cargo/2024/01/23/making-rust-binaries-smaller-by-default.html)
* [My Best and Worst Deadlock in Rust](https://www.snoyman.com/blog/2024/01/best-worst-deadlock-rust/)
* [Why SQL hang for exactly 940s? TCP and Async Rust!](https://xuanwo.io/2024/01-why-sql-hang-for-exactly-940s/)
* [Making Async Rust Reliable](https://tmandry.gitlab.io/blog/posts/making-async-reliable/)
* [Identifying Rust’s collect::() memory leak footgun](https://blog.polybdenum.com/2024/01/17/identifying-the-collect-vec-memory-leak-footgun.html)
* [video] [embassy is now on crates.io](https://www.youtube.com/watch?v=o7okEkXPuIA)
* [video] [Rust full stack web frameworks have a bright future](https://www.youtube.com/watch?v=tq3-M7QJiWg)
* [video] [Rust Halifax - Rust & Tell #1](https://www.youtube.com/watch?v=MH-7xnv9CMI)
* [video] [Why Rust will keep growing in 2024](https://www.youtube.com/watch?v=Q4VNRgxMQ6I)

### Rust Walkthroughs
* [Using `mem::take` to reduce heap allocations](https://ferrous-systems.com/blog/rustls-borrow-checker-p1/)
* [Writing your own Rust linter](https://blog.guillaume-gomez.fr/articles/2024-01-18+Writing+your+own+Rust+linter)
* [Using Serde in Rust](https://www.shuttle.rs/blog/2024/01/23/using-serde-rust)
* [Parsing JSON in Rust](https://www.shuttle.rs/blog/2024/01/18/parsing-json-rust)
* [Billion-row challenge: Rust walkthrough](https://aminediro.com/posts/billion_row/)
* [Embassy on ESP: Timers](https://apollolabsblog.hashnode.dev/embassy-on-esp-timers)

### Research
* [Profiling Programming Language Learning](https://arxiv.org/abs/2401.01257)
* [Rust-lancet: Automated Ownership-Rule-Violation Fixing with Behavior Preservation](https://songlh.github.io/paper/lancet.pdf)

### Miscellaneous

## Crate of the Week

This week's crate is [apistos](https://github.com/netwo-io/apistos), an OpenAPI documentation tool.

Thanks to [Romain Lebran](https://users.rust-lang.org/t/crate-of-the-week/2704/1279) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Have a single `SqlxDatabase` instance per process](https://github.com/build-trust/ockam/issues/7313)
* [Ockam - Improve database migrations to pair sql and rust migration code](https://github.com/build-trust/ockam/issues/7311)
* [Ockam - Make install.sh not fail during upgrade process](https://github.com/build-trust/ockam/issues/7118)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker. 

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

453 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-16..2024-01-23

* [`HashMap`/`HashSet`: forward `fold` implementations of iterators](https://github.com/rust-lang/rust/pull/117756)
* [`dead_code` treats `#[repr(transparent)]` the same as `#[repr(C)]`](https://github.com/rust-lang/rust/pull/120107)
* [`fix(rust-analyzer)`: use new pkgid spec to compare](https://github.com/rust-lang/rust/pull/120084)
* [`large_assignments`: Lint on specific large args passed to functions](https://github.com/rust-lang/rust/pull/116520)
* [`maybe_lint_impl_trait`: separate `is_downgradable` from `is_object_safe`](https://github.com/rust-lang/rust/pull/120164)
* [`never_patterns`: Count `!` bindings as diverging](https://github.com/rust-lang/rust/pull/120104)
* [`never_patterns`: typecheck never patterns](https://github.com/rust-lang/rust/pull/120009)
* [`pat_analysis`: Don't rely on contiguous `VariantId`s outside of rustc](https://github.com/rust-lang/rust/pull/120039)
* [`pattern_analysis`: Remove `Ty: Copy` bound](https://github.com/rust-lang/rust/pull/120027)
* [`proc_macro`: Add `Literal::c_string` constructor](https://github.com/rust-lang/rust/pull/119651)
* [`single_use_lifetimes`: Don't suggest deleting lifetimes with bounds](https://github.com/rust-lang/rust/pull/120148)
* [add `#[track_caller]` to the "From implies Into" impl](https://github.com/rust-lang/rust/pull/119807)
* [add `Ipv6Addr::is_ipv4_mapped`](https://github.com/rust-lang/rust/pull/119081)
* [add `PatKind::Err` to AST/HIR](https://github.com/rust-lang/rust/pull/119967)
* [add help message for `exclusive_range_pattern` error](https://github.com/rust-lang/rust/pull/120152)
* [add private `NonZero<T>` type alias](https://github.com/rust-lang/rust/pull/119990)
* [add way to express that no values are expected with check-cfg](https://github.com/rust-lang/rust/pull/119930)
* [added `NonZeroXxx::from_mut(_unchecked)?`](https://github.com/rust-lang/rust/pull/103730)
* [allow any `const` expression blocks in `thread_local!`](https://github.com/rust-lang/rust/pull/120181)
* [always use RevealAll for const eval queries](https://github.com/rust-lang/rust/pull/119821)
* [avoid ICEs in trait names without `dyn`](https://github.com/rust-lang/rust/pull/119752)
* [consolidate logic around resolving built-in coroutine trait impls](https://github.com/rust-lang/rust/pull/120143)
* [deny braced macro invocations in let-else](https://github.com/rust-lang/rust/pull/119062)
* [detect `NulInCStr` error earlier](https://github.com/rust-lang/rust/pull/119172)
* [improve `let_underscore_lock`](https://github.com/rust-lang/rust/pull/119710)
* [improved `collapse_debuginfo` attribute, added command-line flag](https://github.com/rust-lang/rust/pull/119828)
* [make `unsafe_op_in_unsafe_fn` migrated in edition 2024](https://github.com/rust-lang/rust/pull/119948)
* [restrict access to the private field of newtype indexes](https://github.com/rust-lang/rust/pull/120134)
* [simplify `closure_env_ty` and `closure_env_param`](https://github.com/rust-lang/rust/pull/119969)
* [suggest `.swap()` when encountering conflicting borrows from `mem::swap` on a slice](https://github.com/rust-lang/rust/pull/120126)
* [undeprecate lint `unstable_features` and make use of it in the compiler](https://github.com/rust-lang/rust/pull/118639)
* [make MIR pass name a compile-time constant](https://github.com/rust-lang/rust/pull/120161)
* [make `stable_mir::with_tables` sound](https://github.com/rust-lang/rust/pull/120128)
* [SMIR: make the remaining "private" fields actually private](https://github.com/rust-lang/rust/pull/120135)
* [use an interpreter in MIR jump threading](https://github.com/rust-lang/rust/pull/119461)
* [use implied bounds compat mode in MIR borrowck](https://github.com/rust-lang/rust/pull/120123)
* [validate AggregateKind types in MIR](https://github.com/rust-lang/rust/pull/120137)
* [sandwich MIR optimizations between DSE](https://github.com/rust-lang/rust/pull/119672)
* [cache local DefId-keyed queries without hashing](https://github.com/rust-lang/rust/pull/119977)
* [pack u128 in the compiler to mitigate new alignment](https://github.com/rust-lang/rust/pull/120080)
* [use UnhashMap for a few more maps](https://github.com/rust-lang/rust/pull/120076)
* [fold arithmetic identities in GVN](https://github.com/rust-lang/rust/pull/119670)
* [optimize large array creation in const-eval](https://github.com/rust-lang/rust/pull/120069)
* [implement iterator specialization traits on more adapters](https://github.com/rust-lang/rust/pull/85528)
* [optimize `EscapeAscii`'s Display and `CStr`'s Debug](https://github.com/rust-lang/rust/pull/113142)
* [stabilise `bound_map`](https://github.com/rust-lang/rust/pull/118361)
* [stabilize `round_ties_even`](https://github.com/rust-lang/rust/pull/120150)
* [stabilize `slice_first_last_chunk`](https://github.com/rust-lang/rust/pull/117561)
* [stabilize single-field `offset_of!`](https://github.com/rust-lang/rust/pull/118799)
* [implement strict integer operations that panic on overflow](https://github.com/rust-lang/rust/pull/116090)
* [core: introduce `split_at{,_mut}_checked`](https://github.com/rust-lang/rust/pull/118578)
* [un-hide `iter::repeat_n`](https://github.com/rust-lang/rust/pull/120045)
* [fix deallocation with wrong allocator in `(A)Rc::from_box_in`](https://github.com/rust-lang/rust/pull/119801)
* [use `bool` instead of `PartialOrd` as return value of the comparison closure in `{slice,Iterator}::is_sorted_by`](https://github.com/rust-lang/rust/pull/118811)
* [regex: make `Input::new` guard against incorrect `AsRef` implementations](https://github.com/rust-lang/regex/pull/1154)
* [cargo-rustdoc: use same path by output format logic everywhere](https://github.com/rust-lang/cargo/pull/13325)
* [cargo: use pkgid spec in in JSON messages](https://github.com/rust-lang/cargo/pull/13311)
* [cargo: remap common prefix only](https://github.com/rust-lang/cargo/pull/13210)
* [cargo doc: add a heading to highlight "How to find features enabled on dependencies"](https://github.com/rust-lang/cargo/pull/13305)
* [cargo: inherit jobserver from env for all kinds of runner](https://github.com/rust-lang/cargo/pull/12776)
* [cargo: fix precise-prerelease tracking link](https://github.com/rust-lang/cargo/pull/13320)
* [cargo: go back to passing an empty `values()` when no features are declared](https://github.com/rust-lang/cargo/pull/13316)
* [cargo: improve GitHub Actions CI config](https://github.com/rust-lang/cargo/pull/13317)
* [rustdoc: Allows links in headings](https://github.com/rust-lang/rust/pull/117662)
* [rustdoc: hide modals when resizing the sidebar](https://github.com/rust-lang/rust/pull/119746)
* [rustfmt: check that a token can begin a nonterminal kind before parsing it as a macro arg](https://github.com/rust-lang/rust/pull/120218)
* [rustfmt: add config option `generated_marker_line_search_limit`](https://github.com/rust-lang/rustfmt/pull/5993)
* [clippy: `blocks_in_conditions`: do not warn if condition comes from macro](https://github.com/rust-lang/rust-clippy/pull/12173)
* [clippy: `default_numeric_fallback`: improve const context detection](https://github.com/rust-lang/rust-clippy/pull/12168)
* [clippy: `no_effect_underscore_binding: _` prefixed variables can be used](https://github.com/rust-lang/rust-clippy/pull/12172)
* [clippy: `unused_io_amount` captures `Ok(_)`s](https://github.com/rust-lang/rust-clippy/pull/12005)
* [clippy: add `suspicious_open_options` lint](https://github.com/rust-lang/rust-clippy/pull/11608)
* [clippy: correctly handle type relative in `trait_duplication_in_bounds` lint](https://github.com/rust-lang/rust-clippy/pull/12155)
* [clippy: don't emit `derive_partial_eq_without_eq` lint if the type has the `non_exhaustive` attribute](https://github.com/rust-lang/rust-clippy/pull/12153)
* [clippy: find function path references early in the same lint pass](https://github.com/rust-lang/rust-clippy/pull/12147)
* [clippy: fix FP on `semicolon_if_nothing_returned`](https://github.com/rust-lang/rust-clippy/pull/12167)
* [clippy: fix `multiple_crate_versions` to correctly normalize package names to avoid missing the local one](https://github.com/rust-lang/rust-clippy/pull/12146)
* [clippy: fix warning span for `no_effect_underscore_binding`](https://github.com/rust-lang/rust-clippy/pull/12125)
* [clippy: respect `#[allow]` attributes in `single_call_fn` lint](https://github.com/rust-lang/rust-clippy/pull/12183)
* [clippy: improve wording and fix dead link in description of `arc_with_non_send_sync` lint](https://github.com/rust-lang/rust-clippy/pull/11945)
* [rust-analyzer: add "One" import granularity](https://github.com/rust-lang/rust-analyzer/pull/16372)
* [rust-analyzer: add a new config to allow renaming of non-local defs](https://github.com/rust-lang/rust-analyzer/pull/16391)
* [rust-analyzer: goto type actions for notable trait hovers](https://github.com/rust-lang/rust-analyzer/pull/16375)
* [rust-analyzer: show additional value information when hovering over literals](https://github.com/rust-lang/rust-analyzer/pull/16370)
* [rust-analyzer: show notable implemented traits on hover](https://github.com/rust-lang/rust-analyzer/pull/16374)
* [rust-analyzer: add error recovery for `use_tree_list` parsing](https://github.com/rust-lang/rust-analyzer/pull/16349)
* [rust-analyzer: fix panic when extracting `struct` from `enum` variant](https://github.com/rust-lang/rust-analyzer/pull/16396)
* [rust-analyzer: fix progress reporting getting stuck](https://github.com/rust-lang/rust-analyzer/pull/16383)
* [rust-analyzer: handle `SelfParam` better in "Inline call"](https://github.com/rust-lang/rust-analyzer/pull/16378)
* [rust-analyzer: include `for` construct in convert to guarded return conditions](https://github.com/rust-lang/rust-analyzer/pull/16405)
* [rust-analyzer: infer `OUT_DIR` when workspace root contains a symlink](https://github.com/rust-lang/rust-analyzer/pull/15868)
* [rust-analyzer: make `value_ty` query fallible](https://github.com/rust-lang/rust-analyzer/pull/16367)
* [rust-analyzer: parse `macro_rules` as macro name](https://github.com/rust-lang/rust-analyzer/pull/16314)

### Rust Compiler Performance Triage

This week saw a bunch of regressions caused by correctness fixes and in general doing more work
in the compiler. These were offset by many improvements (especially around hashing in the compiler)
that improved performance by ~2% across a large number of benchmarks. Don't get too excited about the
large 45+% wins though, these were only for tiny benchmarks like helloworld. They were caused by a
[change in Cargo](https://github.com/rust-lang/cargo/pull/13257) which introduces stripping of debug
symbols from Rust release binaries by default, and in turn also improves compilation time for small
crates.

Triage done by **@kobzol**.
Revision range: [f9c2421a..d6b151fc](https://perf.rust-lang.org/?start=f9c2421a2a6e34f3756900dd7d600704c08bfccb&end=d6b151fc77e213bf637db0f12c1965ace3ffe255&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.2%, 1.5%]    | 11    |
| Regressions ❌ <br /> (secondary)  | 2.2%  | [0.2%, 9.9%]    | 26    |
| Improvements ✅ <br /> (primary)   | -3.2% | [-47.5%, -0.2%] | 191   |
| Improvements ✅ <br /> (secondary) | -7.9% | [-46.5%, -0.1%] | 123   |
| All ❌✅ (primary)                 | -3.0% | [-47.5%, 1.5%]  | 202   |


4 Regressions, 4 Improvements, 9 Mixed; 4 of them in rollups
48 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/dc3605e34203a3513f589868a161b8818b30adca/triage/2024-01-23.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: close] [Add a default flag for enum documentation](https://github.com/rust-lang/rust/pull/115575)
* [disposition: merge] [impl `From<&[T; N]>` for `Cow<[T]>`](https://github.com/rust-lang/rust/pull/113489)
* [disposition: merge] [Tracking Issue for array_methods](https://github.com/rust-lang/rust/issues/76118)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [eRFC: Iterate on and stabilize libtest's programmatic output](https://github.com/rust-lang/rfcs/pull/3558)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2024-01-24 - 2024-02-21 🦀

### Virtual

* 2024-01-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763502/)
* 2024-01-23 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/298011202/)
* 2024-01-24 | Virtual (Berlin, DE) | [WeAreDevelopers Community](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Rust Day**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 2024-01-25 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
* 2024-01-25 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Iniciando 2024 con Rust**](https://www.meetup.com/rust-mx/events/298439198/)
* 2024-01-28 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 2024-01-30 | Virtual | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #1**](https://www.meetup.com/bevy-game-development/events/298399958/)
* 2024-01-30 | Virtual (Buffalo, NY, US) | [Buffalo Rust User Group](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 2024-01-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)
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
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygcdbkb/)
* 2024-02-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 2024-02-08 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 2024-02-10 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 2024-02-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341575/)

### Europe

* 2024-01-17 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Que és Rust i els seus beneficis / What's Rust and its advantages**](https://www.meetup.com/rust-girona/events/294080437/)
* 2024-01-17 | Praha / Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Reloaded 2024**](https://www.meetup.com/rust-prague/events/298005196/) 
* 2024-01-17 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**How to Community - January Meetup**](https://www.meetup.com/rust-zurich/events/298066842/)
* 2024-01-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack and Learn**](https://www.meetup.com/rust-aarhus/events/297463730/)
* 2024-01-23 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #64**](https://mobilizon.fr/events/0fce31cd-3578-43f2-abf4-ffecd8d16da2)
* 2024-01-24 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/Zagreb-Rust-Meetup/)
    * [**Rust Meetup 2024/01: WebGPU intro using Rust**](https://www.meetup.com/zagreb-rust-meetup/events/298540606/)
* 2024-01-25 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #5: Async Part2 and Async in action**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/298008068/)
* 2024-01-25 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - January - Blockchains and Data Pipelines**](https://www.meetup.com/rust-vienna/events/298504153/)
* 2024-02-01 | Hybrid (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**12th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/297439924/)
* 2024-02-03 | Brussels, BE | [FOSDEM '24](https://fosdem.org/2024/)
    * [**FOSDEM '24 Conference: Rust devroom - talks**](https://fosdem.org/2024/schedule/track/rust/) | [**Rust Aarhus FOSDEM Meetup**](https://www.meetup.com/rust-aarhus/events/295946777/)
* 2024-02-06 | Bremen, DE | [Rust Meetup Bremen](https://www.linkedin.com/company/rust-meetup-bremen/)
    * [**Rust Meetup Bremen [1]**](https://www.linkedin.com/events/rustmeetupbremen-17153350929486868481/)
* 2024-02-07 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 2024-02-08 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀**](https://www.meetup.com/rust-bern/events/298488858/)

### North America

* 2024-01-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)
* 2024-01-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298304117/)
* 2024-01-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch**](https://www.meetup.com/bostonrust/events/297634962/)
* 2024-01-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 2024-01-27-28 | Calgary, AB, CA | [Rust Calgary](https://www.eventbrite.ca/o/rust-calgary-63449860593)
    * [**Harnessing Rust for Real-World Problems hackathon: Day 1**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-1-tickets-794992302377?aff=ebdsoporgprofile)
    * [**Harnessing Rust for Real-World Problems hackathon: Day 2**](https://www.eventbrite.ca/e/harnessing-rust-for-real-world-problems-hackathon-day-2-tickets-794994147897?aff=ebdsoporgprofile)  
* 2024-01-30 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)
* 2024-02-07 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/297635028/)
* 2024-02-12 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Open Source Contrib Hackathon & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760219/)
* 2024-02-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer**](https://www.meetup.com/rust-nyc/events/298593474/)
* 2024-02-13 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564994/)

### Oceania

* 2024-02-06 | Perth, WA, AU | [Perth Rust Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust Feb 2024 Meetup**](https://www.meetup.com/perth-rust-meetup-group/events/297330668/)

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

> The functional ML roots of the language, Graydon's first Rust compiler was written in OCaml, shine through, influencing it right from the start.
>
> It's not "C++ but better".
>
> It's Haskell standing on Lisp's shoulders, hiding in C's coat to sneak into PRDCTN. (The fancy nightclub where all the popular language's hang out)

– [tris on his "No Boilerplate" Youtube channel](https://www.youtube.com/watch?v=voRBS0r4EyI&t=317)

Thanks to [PrototypeNM1](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1512) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
