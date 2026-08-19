Title: This Week in Rust 664
Number: 664
Date: 2026-08-12
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

* [Call for testing: Restricting trait implementability and field mutability](https://blog.rust-lang.org/inside-rust/2026/08/10/call-for-testing-impl-and-mut-restrictions/)

### Foundation

* [Rust Team Health Summit: September 8 in Montréal](https://rustfoundation.org/media/rust-teams-health-summit-september-8-in-montreal/)
* [Catching Up with the Rust Content Team: RustWeek Interviews and a New Changelog Series](https://rustfoundation.org/media/catching-up-with-the-rust-content-team-rustweek-interviews-and-a-new-changelog-series/)

### Newsletters

* [This Month in Rust OSDev: July 2026](https://rust-osdev.com/this-month/2026-07/)

### Project/Tooling Updates

We are making changes to the Project/Tooling Updates Section - see [here](https://github.com/rust-lang/this-week-in-rust/issues/8575) for more details

* [Bevy's Sixth Birthday](https://bevy.org/news/bevys-sixth-birthday/)
* [fearless_simd v0.7: 64-bit integers, improved generics, SSE2, and upcoming v1.0](https://linebender.org/blog/fearless-simd-0-7/)
* [vairedb 0.1.0 - Cloud Native Distributed Analytical Database](https://github.com/matteobovetti/vairedb/releases/tag/v0.1.0)
* [OXVG 0.0.7: introducing an SVG-to-JSX transformer to the OXVG toolchain](https://github.com/noahbald/oxvg/releases/tag/v0.0.7)
* [HTML, JavaScript, CSS should have died long ago](https://dev.to/zionsati/html-javascript-css-should-have-died-long-ago-1mm3)
* [Even more formal verification for BPF](https://lwn.net/SubscriberLink/1087069/d25c9e5027849a8a/)
* [kache 0.14.0: debuggable restores and cross-clone convergence](https://github.com/kunobi-ninja/kache/releases/tag/v0.14.0)
* [kobe 0.39.0: hardening the cluster-lease lifecycle](https://github.com/kunobi-ninja/kobe/releases/tag/v0.39.0)
* [renew 0.1.1: a deterministic, code-first game engine](https://github.com/renew-engine/renew/releases/tag/v0.1.1)
* [GRIT 1.1: type-check your quantized tensors](https://singhpratech.github.io/grit-datatype/)
* [floDl: Introducing AMD GPU support](https://flodl.dev/blog/making-room)
* [git-cache-proxy: read-only cache for git](https://rolandsdev.blog/posts/caching-git-clones-across-a-slow-network/)

### Observations/Thoughts

* [A Vision for Cargo](https://epage.github.io/blog/2026/08/cargo-vision/)
* [Cyclic trait implementations: motivation](https://smallcultfollowing.com/babysteps/blog/2026/08/10/cyclic-trait-solving/)
* [Rust SIMD on the GPU](https://www.vectorware.com/blog/simd-on-gpu/)
* [Rewriting in Rust: Performance, Failures, 2026 Reality Check](https://blog.jetbrains.com/rust/2026/08/10/rewriting-in-rust/)
* [RangeFrom, Part 1..: History and background](https://erk.dev/2026/08/12/rangefrom-part-1)
* [Typed Conversations: Make Illegal Agent Dialogues Unrepresentable](https://dmitrii.app/typed-conversations-make-illegal-agent-dialogues-unrepresentable/)
* [ECQV: implicit certificates, and why I kept them out of the project that motivated them](https://Abdk4Moura.github.io/post.html?post=2026-08-09-ecqv.md)
* [TLS Handshakes: Measuring the Performance of 4 Cryptography Libraries](https://c410-f3r.github.io/thoughts/tls-handshakes-measuring-the-performance-of-4-cryptography-libraries/)
* [Building scalable backend services with Rust and PostgreSQL](https://kerkour.com/rust-scalable-backend-services)
* [PoC for universal hardware-in-the-loop HAL test suite — Tweede golf](https://tweedegolf.nl/en/blog/240/PoC-for-universal-hardware-in-the-loop-HAL-test-suite/)
* [video] [FLOSS 878 - A Tool With Opinions](https://www.youtube.com/watch?v=ah11nzclXag)

### Rust Walkthroughs

* [Downcasting Arcs in Rust](https://ashdnazg.github.io/articles/26/Downcasting-Arcs-in-Rust)
* [Profiling Rust with hotpath-rs: The Complete Guide - From SQL Queries to CPU Sampling](https://hotpath.rs/blog/profiling-rust-guide)
* [A chip-agnostic architecture for bare-metal embedded Rust](https://aaronqian.com/log/2026-08-01-chip-agnostic-architecture-bare-metal-rust/)

### Research

* [Rust Coreutils: Rebuilding Unix Foundations in a Modern Language](https://arxiv.org/abs/2608.07135)

### Miscellaneous

## Crate of the Week

This week's crate is [literator](https://crates.io/crates/literator), a crate for efficiently displaying the items of an iterator without temporary allocations.

Thanks to [Nora](https://users.rust-lang.org/t/crate-of-the-week/2704/1644) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [Diesel - Improve the documentation of our derives](https://github.com/diesel-rs/diesel/issues/4840)
* [pulse_map - Add cargo-fuzz harness for insert/get/remove sequences](https://github.com/ddsha441981/pulse_map/issues/7)
* [pulse_map - Add 24-hour soak test for ConcurrentPulseMap and ShardedPulseMap](https://github.com/ddsha441981/pulse_map/issues/8)
* [pulse_map - Add loom tests for MetaWord AtomicU64 CAS correctness](https://github.com/ddsha441981/pulse_map/issues/9)
* [pulse_map - Add CI job to verify no_std compilation on thumbv7m-none-eabi](https://github.com/ddsha441981/pulse_map/issues/10)
* [pulse_map - Run Miri on test suite to detect undefined behavior in unsafe code](https://github.com/ddsha441981/pulse_map/issues/11)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

698 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-08-04..2026-08-11

#### Compiler
* [check the fallback map before queueing child in `visible_parent_map` breadth-first search](https://github.com/rust-lang/rust/pull/160464)
* [enable polonius alpha on nightly](https://github.com/rust-lang/rust/pull/159343)
* [improve `canonical_param_env_cache`](https://github.com/rust-lang/rust/pull/160673)
* [interpret: skip deref-projection validity checks when they are not needed](https://github.com/rust-lang/rust/pull/160399)
* [optimize crate resolution for large workspace](https://github.com/rust-lang/rust/pull/159763)
* [optimize handling of solver errors](https://github.com/rust-lang/rust/pull/160160)
* [optimize `try_evaluate_obligations`](https://github.com/rust-lang/rust/pull/160479)
* [perf: cache already-checked types in the privacy visitor](https://github.com/rust-lang/rust/pull/160317)
* [perf: lock-free root fast paths for hygiene queries](https://github.com/rust-lang/rust/pull/160494)
* [perf: skip irrelevant foreign impls when building the specialization graph](https://github.com/rust-lang/rust/pull/157281)
* [perf: store the fulfillment engine inline in ObligationCtxt](https://github.com/rust-lang/rust/pull/160268)
* [shallow resolve ty and const vars to their root vars, attempt 2](https://github.com/rust-lang/rust/pull/158447)
* [speed up `EverInitializedPlaces`](https://github.com/rust-lang/rust/pull/160033)
* [split `apply_primary_terminator_effect`](https://github.com/rust-lang/rust/pull/160555)
* [stabilize `c_variadic_naked_functions`](https://github.com/rust-lang/rust/pull/159746)

#### Library
* [add fast path to `escape_string_symbol`](https://github.com/rust-lang/rust/pull/160453)
* [core: generalize `BorrowedCursor::ensure_init`](https://github.com/rust-lang/rust/pull/160432)
* [hint that memchr returns an in-bounds index](https://github.com/rust-lang/rust/pull/159784)
* [implement `<OnceCell,OnceLock>::new_init`](https://github.com/rust-lang/rust/pull/160881)
* [implement `to_string()` on `ByteStr` and `ByteString`](https://github.com/rust-lang/rust/pull/159300)
* [introduce a `PinSafePointer` trait that generalizes `PinCoerceUnsized`](https://github.com/rust-lang/rust/pull/156935)
* [mark const ptr methods and free functions as `inline(always)` to match *mut](https://github.com/rust-lang/rust/pull/160816)
* [optimize `slice::contains` for one-byte BytewiseEq types](https://github.com/rust-lang/rust/pull/160732)
* [single-pass ASCII lower/upper case conversion](https://github.com/rust-lang/rust/pull/160480)
* [stabilize `fs_set_times`](https://github.com/rust-lang/rust/pull/160820)

#### Cargo
* [`docs(changelog)`: Pull the --verbose item](https://github.com/rust-lang/cargo/pull/17315)
* [`docs(ref)`: Add missing config env variables](https://github.com/rust-lang/cargo/pull/17345)
* [`feat(log)`: emit build-started JSON message with `run_id`](https://github.com/rust-lang/cargo/pull/16632)
* [`feat(profile)`: Add built-in profile debug](https://github.com/rust-lang/cargo/pull/17214)
* [`feat(resolver)`: Report the min-publish-age in the lock message](https://github.com/rust-lang/cargo/pull/17328)
* [`feat(toml)`: allow overriding inherited default-features in 2024](https://github.com/rust-lang/cargo/pull/17126)
* [`feat(trim-paths)`: emit unremap files for final artifacts](https://github.com/rust-lang/cargo/pull/17303)
* [`fix(clean)`: respect target with --doc](https://github.com/rust-lang/cargo/pull/17322)
* [`fix(diag)`: Ensure diagnostic titles work without snippets](https://github.com/rust-lang/cargo/pull/17304)
* [`fix(diag)`: Gate `blanket_hint_mostly_unused` with -Zprofile-hint-mostly-unused](https://github.com/rust-lang/cargo/pull/17313)
* [`fix(diag)`: Remove complexity, perf, and nursery lint groups](https://github.com/rust-lang/cargo/pull/17307)
* [`fix(git)`: Avoid use of git's core.fsmonitor](https://github.com/rust-lang/cargo/pull/17306)
* [`fix(lock)`: Use more accurate 'highest, rather than 'latest'](https://github.com/rust-lang/cargo/pull/17317)
* [`fix(resolver)`: Make min-publish-age relative to --publish-time](https://github.com/rust-lang/cargo/pull/17327)
* [`fix(trim-paths): /cargo/deps` fallback sources](https://github.com/rust-lang/cargo/pull/17338)
* [`fix(trim-paths)`: workspace remap under -Zroot-dir](https://github.com/rust-lang/cargo/pull/17337)
* [`refactor(resolver): resolve()` does not need an optional gctx](https://github.com/rust-lang/cargo/pull/17331)
* [`revert(compiler)`: forward verbose flag to rustc for local crates](https://github.com/rust-lang/cargo/pull/17314)
* [`test(trim-paths)`: exercise unremap files with debuggers](https://github.com/rust-lang/cargo/pull/17326)
* [doc: don't use mergeable info and json together](https://github.com/rust-lang/cargo/pull/17336)
* [fix funding link](https://github.com/rust-lang/cargo/pull/17344)
* [refactor: remove unnecessary mut in sources](https://github.com/rust-lang/cargo/pull/17305)
* [test: handle non-deterministic compilation order](https://github.com/rust-lang/cargo/pull/17347)

#### Rustdoc
* [Create output file after we checked that the standalone markdown file is valid](https://github.com/rust-lang/rust/pull/160576)
* [Do not take `doc(cfg())` into account when filtering doctests](https://github.com/rust-lang/rust/pull/159014)

#### Clippy
* [`cast_possible_truncation`: fix `try_from` suggestion expanding macros](https://github.com/rust-lang/rust-clippy/pull/17530)
* [don't lint `semicolon_if_nothing_returned` in `#[automatically_derived]` …](https://github.com/rust-lang/rust-clippy/pull/17229)
* [fix `needless_range_loop` suggests wrongly for nested index](https://github.com/rust-lang/rust-clippy/pull/16634)
* [`needless_bool`: lint the early-return guard form](https://github.com/rust-lang/rust-clippy/pull/17185)
* [new lint: nonzero operators and methods](https://github.com/rust-lang/rust-clippy/pull/17499)
* [`redundant_pattern_matching`: parenthesize guarded `matches!` suggestion](https://github.com/rust-lang/rust-clippy/pull/17287)
* [`unwrap_or_default`: respect MSRV for raw-pointer Default impls](https://github.com/rust-lang/rust-clippy/pull/17452)

#### Rust-Analyzer
* [account for trailing line continuations in byte strings](https://github.com/rust-lang/rust-analyzer/pull/23032)
* [support `#[rustc_must_implement_one_of]` in the assists](https://github.com/rust-lang/rust-analyzer/pull/23042)
* [add `replace_arith_with_strict` assist](https://github.com/rust-lang/rust-analyzer/pull/23082)
* [`term_search` exclude useless target type](https://github.com/rust-lang/rust-analyzer/pull/22994)
* [add parens on some common cases for `type_mismatch`](https://github.com/rust-lang/rust-analyzer/pull/23117)
* [allow `struct` literals in match guards inside `let` exprs](https://github.com/rust-lang/rust-analyzer/pull/23055)
* [always allocate anon consts for c-strings/byte-strings literals](https://github.com/rust-lang/rust-analyzer/pull/23021)
* [avoid array len type mismatch with string panic](https://github.com/rust-lang/rust-analyzer/pull/23019)
* [avoid escaping bound vars produced by `infer_method_call's skip_binder`](https://github.com/rust-lang/rust-analyzer/pull/23059)
* [bound macro expansion depth across body and block boundaries](https://github.com/rust-lang/rust-analyzer/pull/22974)
* [do not consider locals of `async fn` as upvars of the returned coroutine](https://github.com/rust-lang/rust-analyzer/pull/23103)
* [do not declare the value NS constructor for structs/enum variants if it does not exist](https://github.com/rust-lang/rust-analyzer/pull/23096)
* [don't panic when a lifetime is passed to an `ident` metavariable](https://github.com/rust-lang/rust-analyzer/pull/22922)
* [fix 'no entry found for key' panic in VFS](https://github.com/rust-lang/rust-analyzer/pull/23120)
* [fix upvars query of const block inside closure](https://github.com/rust-lang/rust-analyzer/pull/23040)
* [fixed positive diagnostic for valid code](https://github.com/rust-lang/rust-analyzer/pull/23020)
* [initialize `macro_depth` to the file's macro depth in docs.rs and assoc.rs](https://github.com/rust-lang/rust-analyzer/pull/23034)
* [let `Param::parent_fn` return function for BuiltinDeriveImplMethod methods](https://github.com/rust-lang/rust-analyzer/pull/23071)
* [normalize associated types in orphan checks](https://github.com/rust-lang/rust-analyzer/pull/23078)
* [off-by-one in lifetime binders when lowering `dyn Trait<'a>`](https://github.com/rust-lang/rust-analyzer/pull/23107)
* [optimize memory usage of the item tree](https://github.com/rust-lang/rust-analyzer/pull/23056)
* [parse inline asm with keyword as operand name](https://github.com/rust-lang/rust-analyzer/pull/23054)
* [parse or pattern after range pattern](https://github.com/rust-lang/rust-analyzer/pull/23077)
* [preserve trailing text when `InsertReplaceEdit` is unsupported](https://github.com/rust-lang/rust-analyzer/pull/23028)
* [remove extra spaces in full function signatures](https://github.com/rust-lang/rust-analyzer/pull/23070)
* [support macros in `#[doc]` attributes in IDE features](https://github.com/rust-lang/rust-analyzer/pull/22899)
* [unresolved type variables shouldn't escape impl selection](https://github.com/rust-lang/rust-analyzer/pull/23072)
* [when searching for a `pub macro`, consider it available to reverse dependencies](https://github.com/rust-lang/rust-analyzer/pull/23036)

### Rust Compiler Performance Triage

This week so many new performance improvements landed that we needed to roll 10 of them up together to keep the bors queue manageable, great work!
Also new is the LLVM 23 update which caused massive compile-time, run-time, bootstrap time and artifact size improvements.
These improvements are reduced by the merge of Polonius Alpha on nightly causing a 3.0% regression. There still seems to be some potential to mitigate this regression.

Triage done by **@JonathanBrouwer**.
Revision range: [65dd30fb..771916f9](https://perf.rust-lang.org/?start=65dd30fb9e882a7e8f0be10caca62936db2a98b8&end=771916f9028e7fe56d2685f2c4f698de5d7d6a45&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 4.6%  | [0.2%, 12.8%]   | 24    |
| Regressions ❌ <br /> (secondary)  | 4.3%  | [0.2%, 14.3%]   | 30    |
| Improvements ✅ <br /> (primary)   | -3.3% | [-16.4%, -0.2%] | 251   |
| Improvements ✅ <br /> (secondary) | -5.2% | [-34.8%, -0.2%] | 308   |
| All ❌✅ (primary)                 | -2.6% | [-16.4%, 12.8%] | 275   |


1 Regression, 4 Improvements, 7 Mixed; 5 of them in rollups
25 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/e34d7594ad4dfdd6541038f505ec37d4602171f7/triage/2026/2026-08-09.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Cargo: `hints.min-opt-level`](https://github.com/rust-lang/rfcs/pull/3924)
* [Add `extern "custom"`](https://github.com/rust-lang/rfcs/pull/3980)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [std: map ENOTSUP to ErrorKind::Unsupported- #158580](https://github.com/rust-lang/rust/pull/158580)
* [feat: add symmetric PartialEq impls for `Vec`, `&[T]`, `&mut [T]` versus `Cow<'_, [T]>`](https://github.com/rust-lang/rust/pull/156160)
* [Add `Default` implementation for `std::sync::Once`](https://github.com/rust-lang/rust/pull/160136)
* [Extend `dropping_{references,copy_types}` lints to `drop_in_place`](https://github.com/rust-lang/rust/pull/160229)
* [lint on more incorrect usages of `core::ffi::c_void`](https://github.com/rust-lang/rust/pull/159986)
* [target_features: sse (or at least avx2) is incompatible with soft-float ABI](https://github.com/rust-lang/rust/pull/160302)
* [Make let-else respect macro_rules expr metavariable grouping](https://github.com/rust-lang/rust/pull/158515)
* [PowerPC inline ASM: Fix scalar floats being in the wrong vector lane on little endian](https://github.com/rust-lang/rust/pull/160441)
* [stabilize `Box::take`](https://github.com/rust-lang/rust/pull/160436)
* [make closures act like MaybeDangling](https://github.com/rust-lang/rust/pull/160745)
* [enable next solver by default in orphanck](https://github.com/rust-lang/rust/pull/160668)
* [Error on projection of dyn noncompat type in old trait solver](https://github.com/rust-lang/rust/pull/154992)
* [Stabilize `-Zprofile-sample-use`](https://github.com/rust-lang/rust/pull/155942)

<!-- This item has been hanging around for several weeks.  Ok to delete when it disappears online
* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)
-->

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Stop using dlltool for generating import libraries on MinGW](https://github.com/rust-lang/compiler-team/issues/1029)
* [Promote riscv64a23-unknown-linux-gnu to Tier 2 with host tools](https://github.com/rust-lang/compiler-team/issues/1022)
* [Drop tier 2 `i686-pc-windows-gnu` host tools](https://github.com/rust-lang/compiler-team/issues/1020)

<!-- These items have been hanging around for several weeks.  Ok to delete when they disappear online
* [Add `target_feature_available_at_call_site`](https://github.com/rust-lang/compiler-team/issues/1010)
* [Optimize repr(Rust) enums by omitting tags in more cases involving uninhabited variants.](https://github.com/rust-lang/compiler-team/issues/922)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)
-->

<!-- These items have been hanging around for several weeks.  Ok to delete when they disappear online.
##### Rust RFCs
* [RFC: Refactor the libs team](https://github.com/rust-lang/rfcs/pull/3984)
* [Cargo: `hints.min-opt-level`](https://github.com/rust-lang/rfcs/pull/3924)
-->

<!-- These items have been hanging around for several weeks.  Ok to delete when they disappear online
##### Cargo
* [feat(profile): Add built-in profile debug](https://github.com/rust-lang/cargo/pull/17214)
* [feat(toml): allow overriding inherited default-features in 2024](https://github.com/rust-lang/cargo/pull/17126)
-->

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Allocate more funds for the Funding team in 2026](https://github.com/rust-lang/leadership-council/issues/318)
* [Allocate more funds to 2026 travel budget](https://github.com/rust-lang/leadership-council/issues/316)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Support External Linkers](https://github.com/rust-lang/rfcs/pull/3993)

## Upcoming Events

Rusty Events between 2026-08-12 - 2026-09-09 🦀

### Virtual
* 2026-08-13 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/313345333/)
* 2026-08-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619609/)
* 2026-08-14 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/f2hnzrug)
* 2026-08-16 | Virtual (Bangalore, IN) | [Embedded Rust](https://discord.com/invite/pvYY69PvyS)
    * [**Silicon Sundays #2**](https://discord.gg/tpsNpDHC?event=1536322186829242389)
* 2026-08-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/315604176/)
* 2026-08-19 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Dealing with Dependencies**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #5 — Wireless Communication with the IEEE 802.15.4 protocol**](https://www.meetup.com/charlottesville-rust-meetup/events/315733791/)
* 2026-08-21 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/1bm27cah)
* 2026-08-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254775/)
* 2026-08-27 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/313345334/)
* 2026-08-28 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/arkkrcj5)
* 2026-08-31 | Virtual (Global) | [Rust Maven](https://luma.com/rust-maven)
    * [**Workshop: Add tests to an open source Rust project**](https://luma.com/nwfmsdtf)
* 2026-09-01 | Virtual (Global) | [Rust Maven](https://luma.com/rust-maven)
    * [**Tauri: Cross-Platform desktop applications with Rust and web technologies**](https://luma.com/d9w26vav)
* 2026-09-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)
* 2026-09-04 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/sqf4ux01)
* 2026-09-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254774/)
* 2026-09-08 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315859305/)

### Africa
* 2026-09-08 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Rust's extended standard library**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/august-2026-rustacean-meetup/)
* 2026-08-22 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi X SciPy India Meetup**](https://www.meetup.com/rustdelhi/events/315185336/)
* 2026-08-22 | Noida, IN | [SciPy India](https://scipy.in/)
    * [**Scientific Computing in Rust and Python**](https://scipy.in/sci-py-rs/)
* 2026-08-29 | Pune, IN | [Rust Pune](https://hasgeek.com/rustpune/)
    * [**Rust Pune Meetup: August 2026**](https://hasgeek.com/rustpune/meetup-august-2026/)

### Europe
* 2026-08-13 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night: Trust but verify the LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)
* 2026-08-20 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Building an acoustic camera with egui and embassy**](https://www.meetup.com/rust-rhein-main/events/315855368/)
* 2026-08-27 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)

### North America
* 2026-08-13 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust August Meetup**](https://www.meetup.com/utah-rust/events/314696652/)
* 2026-08-13 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust August Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/315601099/)
* 2026-08-15 | San Francisco, CA, US | [Flower](https://flowercomputer.com/)
    * [**BOG-A-THON 3**](https://partiful.com/e/juWAwRs3XMWP7s9wLNWK)
* 2026-08-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314997215/)
* 2026-08-19 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Dealing with Dependencies**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-19 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust August Meetup**](https://luma.com/00f2s7q9)
* 2026-08-20 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315929355/)
* 2026-08-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: 'Data Shapes Your Memory' and 'Rust in Peace'**](https://www.meetup.com/rust-nyc/events/316056830/)
* 2026-08-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315171660/)
* 2026-08-26 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust LA August! Rust in Quantum Computing**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 2026-08-27 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539331/)
* 2026-09-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cryptography + Quantum Computers**](https://www.meetup.com/stl-rust/events/315603673/)
* 2026-09-08 | Montreal, QC, CA | [Rust Foundation](https://rustfoundation.org/)
    * [**Rust Teams Health Summit**](https://rustfoundation.org/event/rust-teams-health-summit/)
* 2026-09-08 - 2026-09-11 | Montreal, QC, CA | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026, Hosted by the Rust Foundation**](https://rustconf.com/schedule/)
* 2026-09-09 | Montreal, QC, CA | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**RustConf Coffee Break Meetup**](https://www.meetup.com/women-in-rust/events/315773005/)

### Oceania
* 2026-08-27 | Melbourne, AU | [Rust Melbourne](https://luma.com/rustmelbourne)
    * [**Rust Melbourne Meetup**](https://luma.com/d0rndgyv)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> The AI discussion was already put to bed as off topic and derailing. Do not summon the Mods so carelessly, as they are wroth, and filled with cruel vengeance.

– [Simon Buchan on rust-users](https://users.rust-lang.org/t/rust-being-non-standard-affects-compilers/141600/38)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1788) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1vn1ttk/this_week_in_rust_664/)</small>
