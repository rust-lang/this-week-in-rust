Title: This Week in Rust 535
Number: 535
Date: 2024-02-21
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
* [2023 Annual Rust Survey Results](https://blog.rust-lang.org/2024/02/19/2023-Rust-Annual-Survey-2023-results.html)
* [Rust participates in Google Summer of Code 2024](https://blog.rust-lang.org/2024/02/21/Rust-participates-in-GSoC-2024.html)
* [Leadership Council March Representative Selections](https://blog.rust-lang.org/inside-rust/2024/02/19/leadership-council-repr-selection.html)

### Foundation
* [Save the Date: RustConf 2024 – September 10-13](https://foundation.rust-lang.org/news/save-the-date-rustconf-2024-september-10-13/)
* [Second Security Initiative Report Details Rust Security Advancements](https://foundation.rust-lang.org/news/second-security-initiative-report-details-rust-security-advancements/)

### Newsletters

### Project/Tooling Updates
* [Bevy 0.13](https://bevyengine.org/news/bevy-0-13/)
* [Bevy XPBD 0.4: Collider Agnosticism, Layer Rework, and Bevy 0.13](https://joonaa.dev/blog/05/bevy-xpbd-0-4-0)
* [uv: Python packaging in Rust](https://astral.sh/blog/uv)
* [git-cliff: What's new in 2.0.0? (highly customizable changelog generator)](https://git-cliff.org/blog/2.0.0/)
* [rustc_codegen_gcc: Progress Report #30](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-30)
* [RustRover Q4 2023 Feature Updates Retrospective](https://blog.jetbrains.com/rust/2024/02/14/q4-2023-features-retrospective/)
* [rust-analyzer changelog #221](https://rust-analyzer.github.io/thisweek/2024/02/19/changelog-221.html)
- [Anouncing Stabby 3.0](https://www.reddit.com/r/rust/comments/1amjknw/anouncing_stabby_300_and_rustconf_video_available/)

* [argmin_testfunctions 0.2.0: test functions for optimization problems in Rust and Python](https://argmin-rs.org/blog/argmin-testfunctions-v0-2-0/)

- [vscode-rustup released: interface with rustup as a VSCode extension](https://github.com/emberian/vscode-rustup)

- [OpenPGP-card-tools 0.10.0 released. The project provides the `oct` commandline tool for inspecting, configuring and using OpenPGP card devices such as Nitrokey or Yubikey.](https://codeberg.org/openpgp-card/openpgp-card-tools)

[Rust Client for Timeplus Proton SQL Streaming](https://www.timeplus.com/post/rust-client-for-proton)

### Observations/Thoughts
* [FuturesUnordered and the order of futures](https://without.boats/blog/futures-unordered/)
* [Rust Tooling: 8 tools that will increase your productivity](https://www.shuttle.rs/blog/2024/02/15/best-rust-tooling)
* [Writing down my mental model of unsafe](https://gist.github.com/ia0/820ab50d4c5f0f5e3aeb841cef8e6792)
* [How can Rust be so fast in the TechEmpower Web Framework Benchmarks?](https://kerkour.com/rust-fast-techempower-web-framework-benchmarks)

### Rust Walkthroughs
* [From 1s to 4ms](https://registerspill.thorstenball.com/p/from-1s-to-4ms)
* [Translating OpenStreetMap data to HTML5 Canvas with Rust and WebAssembly](https://mary.codes/blog/programming/translating_openstreetmaps_to_HTML5_canvas_rust_wasm/)
* [macros_rule!](https://auroranssolis.github.io/rust/2024/02/14/macros-rule.html)
* [Implementing JWT Authentication in Rust](https://www.shuttle.rs/blog/2024/02/21/using-jwt-auth-rust)
+ [FR] [Les closures en Rust](https://lafor.ge/closure/)
* [Deploying Axum to Lambda and ECS, using Lambda Web Adapter](https://medium.com/@sam.van.overmeire/deploying-axum-to-lambda-and-ecs-using-lambda-web-adapter-2273bd56bb81)
* [Rust/C++ Interop Part 3 - Cxx](https://tylerjw.dev/posts/rust-cmake-interop-part-3-cxx/)
* [video] [Safe Rust AIN'T SAFE!? (cve-rs)](https://www.youtube.com/watch?v=vfMpIsJwpjU)


### Research

### Miscellaneous
* [video] [Release-plz: releasing crates like it's 2023 (RustLab 2023)](https://www.youtube.com/watch?v=kXPBVGDkQSs)

## Crate of the Week

This week's crate is [kind](https://github.com/wingbackapp/kind/), a helper crate for typed UUIDs.

Thanks to [Denys Séguret](https://users.rust-lang.org/t/crate-of-the-week/2704/1290) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

#### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [RFC: Checking conditional compilation at compile time](https://github.com/rust-lang/rfcs/pull/3013)
    * [Testing steps](https://github.com/rust-lang/rfcs/pull/3013#issuecomment-1936648479)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Syntax highlighting for fenced code blocks, in command help output, on Linux works](https://github.com/build-trust/ockam/issues/7471)
* [Ockam - Shut down Worker/Processor if initialization failed](https://github.com/build-trust/ockam/issues/7575)
* [Ockam - Output for ockam project ticket is improved and information is not opaque](https://github.com/build-trust/ockam/issues/7478)

* [ Hyperswitch - [FEATURE] : add `offset` field to disputes list](https://github.com/juspay/hyperswitch/issues/3749)
* [ Hyperswitch - [FEATURE]: add`offset` field to mandates list](https://github.com/juspay/hyperswitch/issues/3748)
* [ Hyperswitch - [FEATURE]: add pagination support for customers list](https://github.com/juspay/hyperswitch/issues/3746)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker. 

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

508 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-02-13..2024-02-20

* [implicitly enable evex512 if avx512 is enabled](https://github.com/rust-lang/rust/pull/121088)
* [`const_mut_refs`: allow mutable pointers to statics](https://github.com/rust-lang/rust/pull/120932)
* [`macro_rules`: Preserve all metavariable spans in a global side table](https://github.com/rust-lang/rust/pull/119673)
* [add APIs for fetching foreign items](https://github.com/rust-lang/rust/pull/120982)
* [add `rust.frame-pointers` config option](https://github.com/rust-lang/rust/pull/121203)
* [add more checks for `unnamed_fields` during HIR analysis](https://github.com/rust-lang/rust/pull/121198)
* [allow AST and HIR visitors to return `ControlFlow`](https://github.com/rust-lang/rust/pull/121256)
* [allow mutable references in const values when they point to no memory](https://github.com/rust-lang/rust/pull/121179)
* [always evaluate free constants and statics, even if previous errors occurred](https://github.com/rust-lang/rust/pull/121087)
* [avoid an ICE in diagnostics](https://github.com/rust-lang/rust/pull/121020)
* [be less confident when `dyn` suggestion is not checked for object safety](https://github.com/rust-lang/rust/pull/120530)
* [check normalized call signature for WF in mir typeck](https://github.com/rust-lang/rust/pull/118882)
* [consider principal trait ref's auto-trait super-traits in dyn upcasting](https://github.com/rust-lang/rust/pull/119338)
* [continue compilation after `check_mod_type_wf` errors](https://github.com/rust-lang/rust/pull/120847)
* [continue compilation even if inherent impl checks fail](https://github.com/rust-lang/rust/pull/121113)
* [continue reporting remaining errors instead of silently dropping them](https://github.com/rust-lang/rust/pull/121032)
* [detect when method call on argument could be removed to fulfill failed trait bound](https://github.com/rust-lang/rust/pull/121100)
* [fix an ICE in the recursion lint](https://github.com/rust-lang/rust/pull/121181)
* [ignore unsized types when trying to determine the size of the original type](https://github.com/rust-lang/rust/pull/121104)
* [make `ConstPropLint` lint run on promoteds](https://github.com/rust-lang/rust/pull/119432)
* [make `async Fn` trait kind errors better](https://github.com/rust-lang/rust/pull/121119)
* [properly deal with weak alias types as self types of impls](https://github.com/rust-lang/rust/pull/120780)
* [rename `ConstPropLint` to `KnownPanicsLint`](https://github.com/rust-lang/rust/pull/121286)
* [store static initializers in metadata instead of the MIR of statics](https://github.com/rust-lang/rust/pull/116564)
* [suggest `into_iter()` when `Iterator` method called on `impl IntoIterator`](https://github.com/rust-lang/rust/pull/119928)
* [trigger `unsafe_code` lint on invocations of `global_asm`](https://github.com/rust-lang/rust/pull/121318)
* [use fulfillment in next trait solver coherence](https://github.com/rust-lang/rust/pull/121193)
* [miri: implement x86 AVX intrinsics](https://github.com/rust-lang/miri/pull/3192)
* [optimize `delayed_bug` handling](https://github.com/rust-lang/rust/pull/121015)
* [optimize away poison guards when std is built with panic=abort](https://github.com/rust-lang/rust/pull/100603)
* [overhaul `Diagnostic` and `DiagnosticBuilder`](https://github.com/rust-lang/rust/pull/120576)
* [implement `Instant` for UEFI](https://github.com/rust-lang/rust/pull/120889)
* [implement `Default` for `AsciiChar`](https://github.com/rust-lang/rust/pull/121024)
* [store `core::str::CharSearcher::utf8_size` as u8](https://github.com/rust-lang/rust/pull/119808)
* [make `File::read_to_end` less special](https://github.com/rust-lang/rust/pull/120538)
* [implement `NonZero` traits generically](https://github.com/rust-lang/rust/pull/121241)
* [make `NonZero::get` generic](https://github.com/rust-lang/rust/pull/120563)
* [make `io::BorrowedCursor::advance` safe](https://github.com/rust-lang/rust/pull/120741)
* [make `is_nonoverlapping #[inline]`](https://github.com/rust-lang/rust/pull/121311)
* [specialize flattening iterators with only one inner item](https://github.com/rust-lang/rust/pull/121204)
* [specialize some methods of `io::Chain`](https://github.com/rust-lang/rust/pull/105917)
* [rename `MaybeUninit::write_slice`](https://github.com/rust-lang/rust/pull/116385)
* [don't use `mem::zeroed` in `vec::IntoIter`](https://github.com/rust-lang/rust/pull/120952)
* [optimize `VecDeque::drain` for (half-)open ranges](https://github.com/rust-lang/rust/pull/118264)
* [fix BTreeMap's `Cursor::remove_{next,prev}`](https://github.com/rust-lang/rust/pull/120505)
* [add `Future` and `IntoFuture` to the 2024 prelude](https://github.com/rust-lang/rust/pull/121041)
* [hashbrown: inline tweaks to `HashTable`](https://github.com/rust-lang/hashbrown/pull/505)
* [hashbrown: make `HashSet::insert` return OccupiedEntry](https://github.com/rust-lang/hashbrown/pull/495)
* [codegen\_gcc: correctly handle `--use-system-gcc`](https://github.com/rust-lang/rustc_codegen_gcc/pull/429)
* [codegen\_gcc: implement dummy emit=llvm-ir](https://github.com/rust-lang/rustc_codegen_gcc/pull/437)
* [codegen\_gcc: use the default rust mangling](https://github.com/rust-lang/rustc_codegen_gcc/pull/440)
* [codegen\_cranelift: fix `simd_select_bitmask` on big-endian systems](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1457)
* [codegen\_cranelift: fix download hash check on big-endian systems](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1458)
* [cargo add: Ensure users know a feature is being created](https://github.com/rust-lang/cargo/pull/13434)
* [cargo add: Remove inconsistent period](https://github.com/rust-lang/cargo/pull/13446)
* [cargo test: Suggest `--` for libtest arguments](https://github.com/rust-lang/cargo/pull/13448)
* [cargo: respect `rust-version` when generating lockfile](https://github.com/rust-lang/cargo/pull/12861)
* [rustdoc: correctly handle long crate names on mobile](https://github.com/rust-lang/rust/pull/120526)
* [rustdoc: cross-crate re-exports: correctly render late-bound params in source order even if early-bound params are present](https://github.com/rust-lang/rust/pull/121022)
* [rustdoc: fix handling of `doc_auto_cfg` feature for cfg attributes on glob reexport](https://github.com/rust-lang/rust/pull/120548)
* [rustfmt: fix error trying to format unnormalized UTF8](https://github.com/rust-lang/rustfmt/pull/6073)
* [format `async` trait bounds in rustfmt](https://github.com/rust-lang/rust/pull/121035)
* [add clippy into the known `cfg` list](https://github.com/rust-lang/rust/pull/121137)
* [clippy: add new lint `deprecated_clippy_cfg_attr`](https://github.com/rust-lang/rust-clippy/pull/12292)
* [clippy: `case_sensitive_file_extension_comparisons`: Don't trigger on digits-only extensions](https://github.com/rust-lang/rust-clippy/pull/12293)
* [clippy: `implied_bounds_in_impls`: avoid linting on overlapping associated tys](https://github.com/rust-lang/rust-clippy/pull/11881)
* [clippy: `incompatible_msrv`: allow expressions that come from desugaring](https://github.com/rust-lang/rust-clippy/pull/12275)
* [clippy: `new_without_default`: Now emits on const fns](https://github.com/rust-lang/rust-clippy/pull/10903)
* [clippy: allow negative literals in `redundant_guards`](https://github.com/rust-lang/rust-clippy/pull/11641)
* [clippy: check trait items in `min_ident_chars`](https://github.com/rust-lang/rust-clippy/pull/12294)
* [clippy: default test output conflict handling to error](https://github.com/rust-lang/rust-clippy/pull/12314)
* [clippy: ensure ASM syntax detect `global_asm!` and `asm!` only on x86 architectures](https://github.com/rust-lang/rust-clippy/pull/12305)
* [clippy: add check for identical guards in lint `match_same_arms`](https://github.com/rust-lang/rust-clippy/pull/12059)
* [clippy: make `#[allow]` work on field for `pub_underscore_fields`](https://github.com/rust-lang/rust-clippy/pull/12309)
* [clippy: ignore imported items in `min_ident_chars`](https://github.com/rust-lang/rust-clippy/pull/12285)
* [rust-analyzer: activate on top level `Cargo.toml` and `rust-project.json` files](https://github.com/rust-lang/rust-analyzer/pull/16550)
* [rust-analyzer: deduplicate references when some of them are in macro expansions](https://github.com/rust-lang/rust-analyzer/pull/16358)
* [rust-analyzer: create alias when renaming an import](https://github.com/rust-lang/rust-analyzer/pull/16489)
* [rust-analyzer: add non-exhaustive-let diagnostic](https://github.com/rust-lang/rust-analyzer/pull/16303)
* [rust-analyzer: add unresolved-ident diagnostic](https://github.com/rust-lang/rust-analyzer/pull/16589)
* [rust-analyzer: support multiple tab stops for completions in VSCode](https://github.com/rust-lang/rust-analyzer/pull/16475)
* [rust-analyzer: add basic support for `become` expr/tail calls](https://github.com/rust-lang/rust-analyzer/pull/15003)
* [rust-analyzer: don't add `\` before `{`](https://github.com/rust-lang/rust-analyzer/pull/16618)
* [rust-analyzer: don't show type mismatches for `{unknown}` to non-`{unknown}` mismatches](https://github.com/rust-lang/rust-analyzer/pull/16583)
* [rust-analyzer: fix "needless return" diagnostic for trailing item declarations](https://github.com/rust-lang/rust-analyzer/pull/16574)
* [rust-analyzer: fix build scripts not being rebuilt in some occasions](https://github.com/rust-lang/rust-analyzer/pull/16247)
* [rust-analyzer: fix false positives for "unnecessary else" diagnostic](https://github.com/rust-lang/rust-analyzer/pull/16590)
* [rust-analyzer: fix snippets being placed leftwards of where they should be](https://github.com/rust-lang/rust-analyzer/pull/16579)
* [rust-analyzer: improve recover on `=` for record field initializer and pattern](https://github.com/rust-lang/rust-analyzer/pull/16553)
* [rust-analyzer: only complete traits in `impl .. for`](https://github.com/rust-lang/rust-analyzer/pull/16544)
* [rust-analyzer: place snippets correctly in multi-edit assists](https://github.com/rust-lang/rust-analyzer/pull/16569)
* [rust-analyzer: server hanging up on build script task](https://github.com/rust-lang/rust-analyzer/pull/16616)

### Rust Compiler Performance Triage


Relatively few PRs affecting performance, but massive improvements thanks to the
update to LLVM 18 (PR #12005), as well as the merging of two related compiler
queries (PR #120919) and other small improvements from a rollup (PR #121055).

Triage done by **@pnkfelix**.
Revision range: [74c3f5a1..5af21304](https://perf.rust-lang.org/?start=74c3f5a146860c94ff4d179fc3bfa34f879adf41&end=5af2130440c198afefbe5b8099342057cf272ef4&absolute=false&stat=instructions%3Au)

3 Regressions, 1 Improvements, 6 Mixed; 1 of them in rollups
65 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/d18e18944c4ab14988ca5219b17530454d133474/triage/2024-02-20.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [eRFC: Iterate on and stabilize libtest's programmatic output](https://github.com/rust-lang/rfcs/pull/3558)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [change equate for binders to not rely on subtyping](https://github.com/rust-lang/rust/pull/118247)
* [disposition: merge] [Implement RFC 3373: Avoid non-local definitions in functions](https://github.com/rust-lang/rust/pull/120393)
* [disposition: merge] [Tracking Issue for `waker_getters`](https://github.com/rust-lang/rust/issues/96992)
* [disposition: merge] [Stabilize the `#[diagnostic]` namespace and `#[diagnostic::on_unimplemented]` attribute](https://github.com/rust-lang/rust/pull/119888)
* [disposition: merge] [Tracking Issue for cfg-target-abi](https://github.com/rust-lang/rust/issues/80970)
* [disposition: merge] [make non-PartialEq-typed consts as patterns a hard error](https://github.com/rust-lang/rust/pull/120805)
* [disposition: close] [Allow ?-converting from `Result<T, E>` in functions returning `Option<Result<T, E>>`](https://github.com/rust-lang/rust/pull/99333)
* [disposition: merge] [Add Read Impl for &Stdin](https://github.com/rust-lang/rust/pull/99153)
* [disposition: merge] [Make `Barrier::new()` const ](https://github.com/rust-lang/rust/pull/119536)
* [disposition: close] [Implement `Future` for `Option<F>`](https://github.com/rust-lang/rust/pull/109691)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [`is` operator for pattern-matching and binding](https://github.com/rust-lang/rfcs/pull/3573)

## Upcoming Events

Rusty Events between 2024-02-21 - 2024-03-20 🦀

### Virtual

* 2024-02-21 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 2 - Types**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298991687/)
* 2024-02-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 2024-02-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)
* 2024-02-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/299068302/)
* 2024-02-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457901/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/02/29/rust-hack-and-learn.html)
* 2024-02-29 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Surfing the Rusty Wireless Waves with the ESP32-C3 Board**](https://www.meetup.com/charlottesville-rust-meetup/events/298372724/)
* 2024-03-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047891/)
* 2024-03-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)
* 2024-03-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341582/)
* 2024-03-12 | Hybrid (Virtual + In-person) Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Web Frontend Co-Learning (online)**](https://www.meetup.com/opentechschool-berlin/events/298406445/)
* 2024-03-21 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 2024-03-26 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Stream](https://www.youtube.com/@bcnrust)

### Europe

* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 2024-02-22 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)
* 2024-02-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Season start 2024**](https://www.meetup.com/rust-berlin/events/299190389/)
* 2024-03-12 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 2024-03-20 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introduction to programming Microcontrollers with Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 2024-03-26 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-26, 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024**](https://www.rustnationuk.com/)

### North America

* 2024-02-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Evening Boston Rust Meetup at Microsoft, February 21**](https://www.meetup.com/bostonrust/events/299054786/)
* 2024-02-22 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043763/)
* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)
* 2024-03-07 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)

### Oceania

* 2024-02-27 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/297650401/)
* 2024-02-27 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)
* 2024-02-29 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**February Meetup**](https://www.meetup.com/rust-brisbane/events/299304438/)
* 2024-03-05 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Introduction to Embedded Rust + The State of Rust UI**](https://www.meetup.com/rust-akl/events/299158887/)

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

> Shared mutable state is evil, and you can solve it by forbidding mutation, or by forbidding sharing. Rust supports both.

– [kornel on Lobste.rs](https://lobste.rs/s/fud3fk/from_1s_4ms#c_relksr)

Thanks to [Aleksey Kladov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1535) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
