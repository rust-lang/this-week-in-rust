Title: This Week in Rust 656
Number: 656
Date: 2026-06-17
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
### Project/Tooling Updates
* [cuTile Rust - Fearless Concurrency on the GPU, memory-safe, data-race-free GPU kernels, B200 benchmarks](https://arxiv.org/abs/2606.15991)
* [Iroh 1.0 - Dial Keys, not IPs](https://www.iroh.computer/blog/v1)
* [Diplomat - Multi-language FFI for Rust libraries](https://manishearth.github.io/blog/2026/06/14/diplomat-multi-language-ffi-for-rust-libraries/)
* [I built EVM from scratch. Again.](https://sergey-melnychuk.github.io/2026/05/23/yevm/)
* [processkit 1.0 - async process tree management](https://zelanton.github.io/processkit/)
* [litchee: Rust Lichess API client](https://github.com/obazin/litchee/releases/tag/v0.1.0)
* [Basin - Numerical Optimization in Rust](https://jolars.co/blog/2026-06-10-basin/)
* [Carboxyl 0.1.0-rc - A servo-based browser for the terminal](https://github.com/carboxyl-rs/carboxyl/releases/tag/v0.1.0-servo-rc.1)
* [kache 0.6.0 - a shareable Rust + C/C++ build cache](https://github.com/kunobi-ninja/kache/releases/tag/v0.6.0)
* [numax v0.1.0 - first stable release of the numax distributed WASM runtime](https://github.com/GianIac/numax/releases/tag/v0.1.0)
* [ZamSync - offline-first Rust sync engine](https://dev.to/etoile_bleu/-i-built-a-sync-engine-for-clinics-that-run-on-2g-and-lose-power-mid-transfer-here-is-why-and-18od)
* [Ktav - a quote-free config format](https://dev.to/phpcraftdream/ktav-i-got-fed-up-with-every-config-format-so-i-built-one-with-no-quotes-no-commas-no-54an)

### Observations/Thoughts
* [zlib-rs in Firefox](https://trifectatech.org/blog/zlib-rs-in-firefox/)
* [Rust Prevents Data Races, Not Race Conditions](https://corrode.dev/blog/rust-prevents-data-races-not-race-conditions/)
* [Build your project Zig-style](https://fnordig.de/2026/06/16/build-your-project-zig-style/)
* [How memory safety CVEs differ between Rust and C/C++](https://kobzol.github.io/rust/2026/06/15/how-memory-safety-cves-differ-between-rust-and-c-cpp.html)
* [Why stdx is not on crates.io](https://kerkour.com/stdx-cratesio)
* [videos] [RustWeek 2026 by RustNL, all talks playlist](https://www.youtube.com/watch?v=PrfMpCaIh0k&list=PL8Q1w7Ff68DBpmF38rcIAf8Z9Gj2TnlgM)
* [The iPad was on Tailscale](https://www.p2claw.com/blog/2026-06-09-the-ipad-was-on-tailscale/)

### Rust Walkthroughs
* [Learn Rust Concurrency By Building a Thread Pool](https://blog.sheerluck.dev/posts/learn-rust-concurrency-by-building-a-thread-pool/)
* [There Is Life Before Main in Rust](https://grack.com/blog/2026/06/11/life-before-main/)
* [Async Task Locals From Scratch](https://wolfgirl.dev/blog/2026-06-16-async-task-locals-from-scratch/)
* [Fearless Embedded Rust: Driving a Lego Car with a Pico W](https://dystroy.org/blog/picomobile/)
* [Building a provider-agnostic LLM layer in Rust with Rig](https://smista.ai/blog/how-we-built-a-provider-agnostic-llm-layer-in-rust-with-rig)

### Miscellaneous
* [video] [RustWeek 2026 talk recordings](https://2026.rustweek.org/blog/2026-06-10-rustweek-recordings-published/)

## Crate of the Week

This week's crate is [marser](https://github.com/ArneCode/marser), a parser combinator library with a twist.

Thanks to [Arne Code](https://users.rust-lang.org/t/crate-of-the-week/2704/1611) for the self-suggestion!

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
<!-- * [ - ]() -->
* [solana-infra-doctor - List exit codes in `sol-doctor --help`](https://github.com/satyakwok/solana-infra-doctor/issues/77)
* [solana-infra-doctor - Make the invalid-URL error suggest the expected scheme](https://github.com/satyakwok/solana-infra-doctor/issues/78)
* [solana-infra-doctor - Add a glossary of RPC readiness terms](https://github.com/satyakwok/solana-infra-doctor/issues/79)
* [openslate - add unit tests for slugify() in api/src/notes.rs](https://github.com/MrSheerluck/openslate/issues/38)
* [openslate - add integration tests for notes CRUD in api/src/notes.rs](https://github.com/MrSheerluck/openslate/issues/70)
* [openslate - add integration tests for auth flow in api/src/users.rs](https://github.com/MrSheerluck/openslate/issues/96)
* [openslate - add unit tests for build_fts_query() in api/src/search.rs](https://github.com/MrSheerluck/openslate/issues/89)
* [openslate - add integration tests for auth middleware and logout in api/src/auth.rs](https://github.com/MrSheerluck/openslate/issues/106)
* [openslate - add integration tests for media endpoints (DB layer) in api/src/media.rs](https://github.com/MrSheerluck/openslate/issues/85)
* [openslate - add unit tests for ext_from_mime() and filename_from_url() in api/src/media.rs](https://github.com/MrSheerluck/openslate/issues/40)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

527 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-09..2026-06-16

#### Compiler
* [`obligations_for_self_ty`: skip irrelevant goals (recompute `sub_root` from `stalled_vars)`](https://github.com/rust-lang/rust/pull/156187)
* [`codegen_ssa`: peel trans. wrappers on scalable vecs](https://github.com/rust-lang/rust/pull/157768)
* [add a check for impossible predicates to `trivial_const`](https://github.com/rust-lang/rust/pull/156934)
* [add unstable loop unrolling hint attributes](https://github.com/rust-lang/rust/pull/156816)
* [improve polymorphization of raw pointer formatting](https://github.com/rust-lang/rust/pull/157714)
* [introduce `#[diagnostic::on_type_error(message)]`](https://github.com/rust-lang/rust/pull/155200)
* [perf: reuse green-marking's edge walk when promoting a node](https://github.com/rust-lang/rust/pull/157781)

#### Library
* [add `or_try_*` variants for `HashMap` and `BTreeMap` Entry APIs](https://github.com/rust-lang/rust/pull/157355)
* [make `BorrowedBuf` and `BorrowedCursor` generic over the data](https://github.com/rust-lang/rust/pull/149749)
* [replace printables table with `unicode_data.rs` tables](https://github.com/rust-lang/rust/pull/155527)
* [stabilize `#![feature(box_as_ptr)]`](https://github.com/rust-lang/rust/pull/157876)
* [stabilize `core::range::{legacy, RangeFull, RangeTo}`](https://github.com/rust-lang/rust/pull/156629)
* [stabilize `int_format_into` feature](https://github.com/rust-lang/rust/pull/152544)
* [stabilize `nonzero_from_str_radix`](https://github.com/rust-lang/rust/pull/157877)
* [stabilize feature `float_algebraic`](https://github.com/rust-lang/rust/pull/157029)

#### Cargo
* [`trim-paths`: emit `CARGO_TRIM_PATHS_REMAP` for build.rs](https://github.com/rust-lang/cargo/pull/17104)
* [`diag`: Give diagnostics the same display path behavior as rustc](https://github.com/rust-lang/cargo/pull/17101)
* [`diag`: Report all errors, in order](https://github.com/rust-lang/cargo/pull/17095)
* [`publish`: avoid false deadlock when `to_confirm` is non-empty](https://github.com/rust-lang/cargo/pull/17071)
* [`resolver`: move yank policy to resolver layer](https://github.com/rust-lang/cargo/pull/17083)

#### Rustdoc
* [also run lint `unused_doc_comments`](https://github.com/rust-lang/rust/pull/141000)
* [cleanup and (micro-)optimize `print_where_clause`](https://github.com/rust-lang/rust/pull/157874)
* [correct doctest span for trailing semicolon after item](https://github.com/rust-lang/rust/pull/157740)
* [don't strip hidden items in `AliasedNonLocalStripper`](https://github.com/rust-lang/rust/pull/157838)
* [some more lazy formatting](https://github.com/rust-lang/rust/pull/157796)

#### Rustfmt
* [add `doc_comment_code_block_small_heuristics`, to override `use_small_heuristics` in doc code](https://github.com/rust-lang/rustfmt/pull/6616)
* [stabilize `hex_literal_case`](https://github.com/rust-lang/rustfmt/pull/6935)

#### Clippy
* [new `by_ref_peekable_peek` lint](https://github.com/rust-lang/rust-clippy/pull/17042)
* [add `with_capacity_zero` lint](https://github.com/rust-lang/rust-clippy/pull/17192)
* [`mem_replace_with_default`: also emit inside macros](https://github.com/rust-lang/rust-clippy/pull/17191)
* [`infallible_destructuring_match`: clean-up, split off the suggestion from the main message](https://github.com/rust-lang/rust-clippy/pull/17175)
* [`manual_is_variant_and`: lint `result.ok().is_some_and(f)`](https://github.com/rust-lang/rust-clippy/pull/17184)
* [`needless_borrow`: same-name methods false positive](https://github.com/rust-lang/rust-clippy/pull/17171)
* [`unnecessary_lazy_evaluations`: handle closure `->`](https://github.com/rust-lang/rust-clippy/pull/17216)
* [deprecate the `from_iter_instead_of_collect` lint](https://github.com/rust-lang/rust-clippy/pull/17208)
* [remove `is_integer_const`](https://github.com/rust-lang/rust-clippy/pull/17204)
* [do not trigger `ref_patterns` lint on automatically derived code](https://github.com/rust-lang/rust-clippy/pull/17250)
* [enhance never loop](https://github.com/rust-lang/rust-clippy/pull/17145)
* [add profile-specific configuration for disallowed methods and types](https://github.com/rust-lang/rust-clippy/pull/15779)
* [fix `collapsible_match` suggests wrongly when match body has no braces](https://github.com/rust-lang/rust-clippy/pull/16749)
* [fix `unnecessary_sort_by` reverse suggestion using wrong closure parameter name](https://github.com/rust-lang/rust-clippy/pull/16868)
* [fix redundant closure call async false positive](https://github.com/rust-lang/rust-clippy/pull/17107)
* [perf: check `is_in_test` last in `incompatible_msrv`](https://github.com/rust-lang/rust-clippy/pull/17218)
* [perf: check the token kind before extracting source in early literal lints](https://github.com/rust-lang/rust-clippy/pull/17219)
* [perf: match expression shape before MSRV check in `cloned_ref_to_slice_refs`](https://github.com/rust-lang/rust-clippy/pull/17220)
* [perf: skip `doc_markdown` text collection and word scan when the lint is allowed](https://github.com/rust-lang/rust-clippy/pull/17217)
* [perf: skip `single_component_path_imports` module walk when nothing to lint](https://github.com/rust-lang/rust-clippy/pull/17225)

#### Rust-Analyzer
* [create directory for `cargo xtask metrics rustc_tests`](https://github.com/rust-lang/rust-analyzer/pull/22562)
* [don't count C-variadic `...` as a parameter for fn pointers](https://github.com/rust-lang/rust-analyzer/pull/22575)
* [support flyimport exclude variants](https://github.com/rust-lang/rust-analyzer/pull/22549)
* [fix destructuring assignments not introducing moves](https://github.com/rust-lang/rust-analyzer/pull/22566)
* [offer inline macro in macro call and proc macro](https://github.com/rust-lang/rust-analyzer/pull/22584)
* [prefer bench command when target is bench to avoid cargo run](https://github.com/rust-lang/rust-analyzer/pull/22591)
* [supports inline variable in macro](https://github.com/rust-lang/rust-analyzer/pull/22551)
* [use package id as argument to `--package` if package is not unique](https://github.com/rust-lang/rust-analyzer/pull/22574)
* [assist `inline_type_alias` work on ADT definitions](https://github.com/rust-lang/rust-analyzer/pull/22545)
* [perf: defer initial workspace flycheck until cache priming completes](https://github.com/rust-lang/rust-analyzer/pull/22579)
* [remove docs about removed `analysis-bench` command](https://github.com/rust-lang/rust-analyzer/pull/22561)
* [remove unnecessary feature flags from tests](https://github.com/rust-lang/rust-analyzer/pull/22571)
* [use ASCII lowercase for dylib extensions check](https://github.com/rust-lang/rust-analyzer/pull/22585)

### Rust Compiler Performance Triage

This week we had quite a lot of changes, a few small regressions that were a bit tough to diagnose, but the week is largely positive, overall.
Notably, we got one massive improvement on the next-solver benchmark in #[156187](https://github.com/rust-lang/rust/pull/156187),
and a nice speedup for incremental in [#157781](https://github.com/rust-lang/rust/pull/157781).

Triage done by **@panstromek**.
Revision range: [f3ef3bd8..b5d46ecb](https://perf.rust-lang.org/?start=f3ef3bd882dd24a275a60701a67c3bb330edd8c1&end=b5d46ecb51c3e4134b82570cfe718f093daa6390&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 0.6%]    | 22    |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.1%, 2.0%]    | 40    |
| Improvements ✅ <br /> (primary)   | -1.8% | [-5.9%, -0.1%]  | 125   |
| Improvements ✅ <br /> (secondary) | -3.8% | [-69.4%, -0.1%] | 90    |
| All ❌✅ (primary)                 | -1.5% | [-5.9%, 0.6%]   | 147   |


1 Regression, 4 Improvements, 8 Mixed; 5 of them in rollups
28 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/d36b1ad8679b65efbb98252fbb93f72a7d90d4c6/triage/2026/2026-06-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Fix trait method resolution on an adjusted never type](https://github.com/rust-lang/rust/pull/156047)
* [Tracking Issue for atomic_from_mut](https://github.com/rust-lang/rust/issues/76314)
* [stabilize never type](https://github.com/rust-lang/rust/pull/155499)
* [Lint against iterator functions that panic when N is zero](https://github.com/rust-lang/rust/pull/153563)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Single-byte counter support in coverage instrumentation](https://github.com/rust-lang/compiler-team/issues/1002)
* [Rename the compiler files containing struct diagnostics to `diagnostics.rs`](https://github.com/rust-lang/compiler-team/issues/1003)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Delegate Project Grants to the Funding team](https://github.com/rust-lang/leadership-council/issues/301)
* [Allocate budget to the Funding team](https://github.com/rust-lang/leadership-council/issues/304)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Named Fn trait parameters](https://github.com/rust-lang/rfcs/pull/3955)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Structs with no fields or all-ZST fields are ZSTs](https://github.com/rust-lang/reference/pull/2262)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-06-17 - 2026-07-15 🦀

### Virtual
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/ekws5nr4)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: What the heck are monads - and how do we fake them in Rust**](https://www.meetup.com/women-in-rust/events/313767883/)
* 2026-07-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210366/)
* 2026-07-02 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Learning Game Development the Hard Way with Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/315211402/)
* 2026-07-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-05 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315060981/)
* 2026-07-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
    
### Europe
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)
* 2026-06-18 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends comes to Glasgow! (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/315093492/)
* 2026-06-18 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends comes to Glasgow! (evening pub)**](https://www.meetup.com/rust-and-friends/events/315093500/)
* 2026-06-18 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**21st BcnRust Meetup**](https://www.meetup.com/bcnrust/events/315094938/)
* 2026-06-19 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Second Meetup**](https://pretix.eu/rust-dresden/on-location-2)
* 2026-06-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #86**](https://www.meetup.com/rust-paris/events/315040676/)
* 2026-06-23 | Warsaw, PL | [Rust Warsaw](https://luma.com/rust.in.warsaw)
    * [**Rust Warsaw Meetup: June 2026**](https://luma.com/djs7ntfx)
* 2026-06-24 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester June Talks**](https://www.meetup.com/rust-manchester/events/315200163/)
* 2026-06-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396600/)
* 2026-06-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #69**](https://www.meetup.com/copenhagen-rust-community/events/315214426/)
* 2026-07-02 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Bevy, Bits, & Cats (Rust July Talks)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, OV, NL | [Baseflow Tech Meetups](https://www.meetup.com/dutch-rust-meetup/events/)
    * [**AI Summit**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/events/)
    * [**Join us live and INPERSON for Rust 261**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)


### North America
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Community Meetup**](https://www.meetup.com/music-city-rust-developers/events/315213927/)
* 2026-06-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Northeastern Rust Lunch, June 20**](https://www.meetup.com/bostonrust/events/315225854/)
* 2026-06-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315105633/)
* 2026-06-24 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust-Based Constraint Solvers in 2D Sketching with Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-26 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Big Summer Social**](https://www.meetup.com/rust-nyc/events/315014582/)
* 2026-06-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Somerville Union Square Rust Lunch, June 27**](https://www.meetup.com/bostonrust/events/315225857/)
* 2026-07-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Git is easy?**](https://www.meetup.com/stl-rust/events/315103359/)
* 2026-07-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston University Rust Lunch, July 4**](https://www.meetup.com/bostonrust/events/315225861/)
* 2026-07-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Utah Rust July Meetup**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, July 11**](https://www.meetup.com/bostonrust/events/315225865/)

### Oceania
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne June 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)

### South America
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> "The never type is named after the date of its stabilization" was a good joke while it lasted.

– [Sergey "Shnatsel" Davidoff on /r/rust](https://www.reddit.com/r/rust/comments/1u1v53c/the_never_type_is_likely_to_stabilize_soon/oqss8ii/)

Thanks to [Dos Moonen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1780) for the suggestion!

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

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
