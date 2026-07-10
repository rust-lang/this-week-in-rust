Title: This Week in Rust 659
Number: 659
Date: 2026-07-08
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
* [Maintainer spotlight: Gen Li (@rami3l)](https://blog.rust-lang.org/inside-rust/2026/07/07/maintainer-spotlight-gen-li-rami3l/)
* [Together for a healthier Clippy](https://blog.rust-lang.org/inside-rust/2026/07/06/unite-for-clippy/)

### Newsletters
* [The Embedded Rustacean Issue #75](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-75)

### Project/Tooling Updates
* [copper-rs v1.0.0](https://www.copper-robotics.com/whats-new/copper-rs-v100): the open source deterministic robotics OS is now stable.
* [Rayfish: Your own private network. No servers, no setup.](https://rayfish.xyz/blog/01-introducing-rayfish)
- [rama v0.3.0 — network service framework ready to be used by the wider Rust community](https://plabayo.tech/blog/rama-0-3)
* [kache 0.9.0: supply-chain hardening + read-only CI cache](https://github.com/kunobi-ninja/kache/releases/tag/v0.9.0)
* [GuardianDB - PostgreSQL and P2P/Local-First Together](https://www.willsearch.com.br/blog/2026/07/04/meet-guardiandbs-new-postgresql-compatibility-layer/)
* [Nectar: a Rust-like language that compiles your whole web app to WebAssembly](https://buildnectar.com/)
* [logdrain: Fast, Embeddable Log-Template Mining in Rust](https://thekeeper.io/blog/logdrain-log-template-mining-in-rust/)
* [sheathe: Packaging the World's Video in Pure Rust](https://medium.com/@vbasky/packaging-the-worlds-video-in-pure-rust-ff1f6b884fec)
* [wickra: streaming-first technical indicators](https://docs.wickra.org/Quickstart-Rust)
* [Xcelerator Solver v0.1.0 -- deterministic symbolic regression](https://github.com/TeamXcelerator/xcelerator-solver/releases/tag/v0.1.0)
* [dlt-tui 1.1.0 - a fast TUI viewer for automotive DLT (AUTOSAR Diagnostic Log and Trace) files](https://github.com/tkmsikd/dlt-tui/releases/tag/v1.1.0)
* [RSSH v0.2.11 — terminal workflows, safer SSH key import, and observable AI ops](https://github.com/shihuili1218/rssh/releases/tag/v0.2.11)
* [k8s-scale-app-rs: Scale or Restart a Kubernetes Deployment from a CronJob](https://blog.none.at/blog/2026/2026-07-06-k8s-scale-app-rs/)
* [M-vis v0.5.0-rc1 update](https://dev.to/sicklefire/m-vis-v050-rc1-update-11cp)
* [FlareDB: An Apache Beam Native Streaming Database built in Rust](https://ganeshsivakumar.substack.com/p/flaredb)
* [mqtt-typed-client 0.2: a type-safe async MQTT client on rumqttc](https://holovskyi.github.io/blog/typed-mqtt-topics-for-rust/)
* [RootAsRole: v4.0.0 Major release, secure execution, new logo](https://github.com/LeChatP/RootAsRole/releases/tag/v4.0.0)
* [A Cross-Platform Rust UI Framework via Qt’s Bridging Technology](https://www.qt.io/blog/rust-ui-framework-via-bridging-technology)
* [Jam Programming Language](https://rapha.land/jam-programming-language/)
* [Sōzu 2.1.0: UDP load balancing for the programmable edge](https://www.clever.cloud/blog/company/2026/07/01/sozu-2-1-0-udp-load-balancer-programmable-edge/)
* [b0nker: a minimal container runtime written in Rust](https://op3kay.dev/writing/b0nker)

### Observations/Thoughts
* [video] [Rust Berlin Meetup 25/06/2026 Livestream](https://www.youtube.com/watch?v=SGR5qBdwk30)
* [video] [How do you rewrite C/C++ projects to Rust? – JetBrains interview with Luca Palmieri, Mainmatter](https://www.youtube.com/live/_LtgHxuysUo)
* [Investigating why RustCrypto is slow: Deep dive into SIMD instructions and hardware acceleration](https://kerkour.com/rustcrypto-slow-simd-rust)
* [bool as u32](https://parsa.wtf/cast/)
* [A Rust-to-Lean Verification Pipeline with AI Provers: An Experience Report](https://arxiv.org/html/2605.30106)
* [Work In Progress Rust](https://blog.dureuill.net/articles/wip/)
* [video] [OpenAI just spent $600k on Rust](https://www.youtube.com/watch?v=Fk165jYfHpc)
* [audio] [Rising Academies with Dylan Brown - Rust in Production Podcast](https://corrode.dev/podcast/s06e07-rising-academies/)

### Rust Walkthroughs
* [series] [Bevy Tutorial: Build Your First 3D Editor - Create a 3D Space on an Infinite Grid](https://aibodh.com/posts/bevy-tutorial-build-your-first-3d-editor-in-rust/)
* [Learn Axum Basics and Routing by Building a URL Shortener](https://blog.sheerluck.dev/posts/learn-axum-basics-and-routing-by-building-a-url-shortener/)
* [series] [Rama 101.1: HTTPS clients and layers of abstraction](https://plabayo.tech/blog/rama-101-1-https-clients-and-abstractions)

### Miscellaneous
* [Clickable euler diagram of all the Rust week talks](https://seanborg.tech/tiny-blog/rust-week-ven-diagram/)

## Crate of the Week

This week's crate is [apis-saltans](https://crates.io/crates/apis-saltans-core), a Zigbee implementation including a coordinator API.

Thanks to [Richard Neumann](https://users.rust-lang.org/t/crate-of-the-week/2704/1627) for the self-suggestion!

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
* [Protocol - Extend bit-exactness tests to f64 reconstruction targets](https://github.com/name970/Protocol/issues/4)                                                                            
* [Dofigen - No image tag replacement flag for the generate command](https://github.com/lenra-io/dofigen/issues/278)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

598 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-06-30..2026-07-07

#### Compiler
* [enable eager `param_env` norm in new solver](https://github.com/rust-lang/rust/pull/156976)
* [lint on `core::ffi::c_void` as a return type](https://github.com/rust-lang/rust/pull/156379)
* [polish some macro parsing code](https://github.com/rust-lang/rust/pull/158577)
* [resolve: no allocation in `resolve_ident_in(_local)_module_*`](https://github.com/rust-lang/rust/pull/158604)
* [simplify option-iterator flattening in the compiler](https://github.com/rust-lang/rust/pull/158627)
* [stabilize `#[my_macro] mod foo;` (part of `proc_macro_hygiene`)](https://github.com/rust-lang/rust/pull/157857)

#### Library
* [add `std::io::cursor::WriteThroughCursor`](https://github.com/rust-lang/rust/pull/158537)
* [implement `Box::as_non_null()`](https://github.com/rust-lang/rust/pull/157347)
* [implement `DoubleEndedIterator::next_chunk_back`](https://github.com/rust-lang/rust/pull/156737)
* [implement `IntoIterator` for `[&[mut]] Box<[T; N], A>`](https://github.com/rust-lang/rust/pull/134021)
* [implement `ptr::{read,write}_unaligned` via `repr(packed)`](https://github.com/rust-lang/rust/pull/158427)
* [move `SizeHint` and `IoHandle` to `core::io`](https://github.com/rust-lang/rust/pull/158539)
* [move `std::io::Seek` to `core::io`](https://github.com/rust-lang/rust/pull/158540)
* [optimize `ArrayChunks::try_rfold` with `DoubleEndedIterator::next_chunk_back`](https://github.com/rust-lang/rust/pull/158704)
* [stabilize `feature(atomic_from_mut)`](https://github.com/rust-lang/rust/pull/158573)

#### Cargo
* [`bindeps`: register transitive artifact targets](https://github.com/rust-lang/cargo/pull/17135)
* [avoid cloning parsed TOML manifest in `ManifestErrorContext`](https://github.com/rust-lang/cargo/pull/17167)
* [avoid extra clone of parsed TOML manifest](https://github.com/rust-lang/cargo/pull/17176)
* [remove unneeded cloning when parsing package index](https://github.com/rust-lang/cargo/pull/17178)
* [change HashMaps and HashSets in Cargo to use Fxhasher](https://github.com/rust-lang/cargo/pull/17169)
* [do not pass lint rustflags when `--cap-lints=allow` is set](https://github.com/rust-lang/cargo/pull/17174)
* [fixed `Compilation::deps_output` only taking the last dep](https://github.com/rust-lang/cargo/pull/17164)
* [pre-allocate a few vectors](https://github.com/rust-lang/cargo/pull/17177)
* [stabilize `build-dir` layout v2](https://github.com/rust-lang/cargo/pull/16807)
* [use a set when checking visited workspace members](https://github.com/rust-lang/cargo/pull/17180)

#### Rustdoc
* [fix crash when trying to inline foreign item which cannot have attributes](https://github.com/rust-lang/rust/pull/158751)
* [show use-site paths for unevaluated const array lengths](https://github.com/rust-lang/rust/pull/158334)

#### Clippy
* [`chunks_exact_to_as_chunks`: Don't report expressions with const parameters](https://github.com/rust-lang/rust-clippy/pull/17319)
* [`chunks_exact_to_as_chunks`: Don't report expressions with type params](https://github.com/rust-lang/rust-clippy/pull/17360)
* [`missing_trait_methods`: MSRV/unstable awareness](https://github.com/rust-lang/rust-clippy/pull/17309)
* [`vec_init_then_push`: don't lint pushes from a macro expansion](https://github.com/rust-lang/rust-clippy/pull/17289)
* [`inline_modules`: ignore `cfg(test)` modules in test builds](https://github.com/rust-lang/rust-clippy/pull/17346)
* [`match_same_arms`: keep arm-level expectations working under an outer allow](https://github.com/rust-lang/rust-clippy/pull/17345)
* [`unnecessary_operation`: avoid bad `!` suggestions](https://github.com/rust-lang/rust-clippy/pull/17341)
* [`unnecessary_unwrap_unchecked`: don't trigger inside the `_unchecked` fn](https://github.com/rust-lang/rust-clippy/pull/17351)
* [add required parentheses when the `needless_bool` suggestion is an operand](https://github.com/rust-lang/rust-clippy/pull/17348)
* [fix ICE when resolving local in `unnecessary_unwrap_unchecked`](https://github.com/rust-lang/rust-clippy/pull/17353)
* [fix `infinite_loop` false positive inside gen blocks](https://github.com/rust-lang/rust-clippy/pull/17311)
* [fix `manual_c_str_literals` suggestion when the trailing backslash is escaped](https://github.com/rust-lang/rust-clippy/pull/17358)
* [fix `strlen_on_c_strings` incorrect suggestion logic](https://github.com/rust-lang/rust-clippy/pull/17337)
* [fix `suspicious_operation_groupings` duplications](https://github.com/rust-lang/rust-clippy/pull/17323)
* [lint bit width](https://github.com/rust-lang/rust-clippy/pull/16902)
* [optimize `Msrv::meets` calls](https://github.com/rust-lang/rust-clippy/pull/17338)
* [bail out of unicode lint scans when the snippet is pure ASCII](https://github.com/rust-lang/rust-clippy/pull/17273)
* [skip the HIR parent walk in `is_in_test_function` when there are no test items](https://github.com/rust-lang/rust-clippy/pull/17224)
* [place generated impl block after the existing impl block](https://github.com/rust-lang/rust-clippy/pull/17366)
* [refactor `StringAdd` lint pass](https://github.com/rust-lang/rust-clippy/pull/17333)
* [refactor `suspicious_xor_used_as_pow`](https://github.com/rust-lang/rust-clippy/pull/17334)
* [remove `lower_ty` in `uninhabited_reference`](https://github.com/rust-lang/rust-clippy/pull/17293)
* [respect the configured MSRV in `manual_is_variant_and`'s `map() == Some(_)` rewrite](https://github.com/rust-lang/rust-clippy/pull/17328)
* [rewrite `mut_mut`](https://github.com/rust-lang/rust-clippy/pull/17332)
* [rewrite `redundant_else` as a late pass](https://github.com/rust-lang/rust-clippy/pull/17329)
* [rewrite `tuple_array_conversions`](https://github.com/rust-lang/rust-clippy/pull/17354)

#### Rust-Analyzer
* [SCIP: exclude leading/trailing trivia in definition ranges](https://github.com/rust-lang/rust-analyzer/pull/22595)
* [SCIP: remove dead `inlay_hints` field](https://github.com/rust-lang/rust-analyzer/pull/22708)
* [`feat(ide-diagnostics)`: add diagnostics for invalid union patterns (E0784)](https://github.com/rust-lang/rust-analyzer/pull/22433)
* [`internal(query-group-macro)`: remove the arity test](https://github.com/rust-lang/rust-analyzer/pull/22704)
* [add tree top method to Syntax node](https://github.com/rust-lang/rust-analyzer/pull/22668)
* [add handler for E0627](https://github.com/rust-lang/rust-analyzer/pull/22665)
* [supports multi arms for `replace_match_with_if_let`](https://github.com/rust-lang/rust-analyzer/pull/22231)
* [fix UB in `smol_str borsh_non_utf8` test cases](https://github.com/rust-lang/rust-analyzer/pull/22690)
* [fix generic param for `generate_default_from_enum_variant`](https://github.com/rust-lang/rust-analyzer/pull/20362)
* [`walkthrough_create_project` file not packaged](https://github.com/rust-lang/rust-analyzer/pull/22703)
* [assertion failure on closure with unbound function](https://github.com/rust-lang/rust-analyzer/pull/22677)
* [avoid panic in `convert_tuple_struct_to_named_struct` on nested pattern usage](https://github.com/rust-lang/rust-analyzer/pull/22613)
* [configuration syntax for nvim-lsp](https://github.com/rust-lang/rust-analyzer/pull/22649)
* [correct resolution to value when it shares the same name with type](https://github.com/rust-lang/rust-analyzer/pull/22706)
* [exclude impls on the error type from impl enumeration](https://github.com/rust-lang/rust-analyzer/pull/22619)
* [fix crash on `extract_variable` when selecting unresolved macro call](https://github.com/rust-lang/rust-analyzer/pull/22705)
* [fix crash on completion inside macros](https://github.com/rust-lang/rust-analyzer/pull/22715)
* [fix handling of params of coroutine fns](https://github.com/rust-lang/rust-analyzer/pull/22673)
* [handle more cases of cfgs in expr store lowering](https://github.com/rust-lang/rust-analyzer/pull/22675)
* [no generate with default assoc item](https://github.com/rust-lang/rust-analyzer/pull/22488)
* [panics in `unwrap_return_type`, `remove_underscore`, and `promote_local_to_const`](https://github.com/rust-lang/rust-analyzer/pull/22674)
* [hoist attribute qualifier segment collection](https://github.com/rust-lang/rust-analyzer/pull/22711)
* [reduce parser joint-token allocation](https://github.com/rust-lang/rust-analyzer/pull/22709)
* [project-model: don't pass metadata extra args to sysroot](https://github.com/rust-lang/rust-analyzer/pull/22676)
* [project-model: introduce cargo.configPath](https://github.com/rust-lang/rust-analyzer/pull/22679)
* [provide startup time to ready log point and associated benchmark](https://github.com/rust-lang/rust-analyzer/pull/22581)

### Rust Compiler Performance Triage


This week was dominated by wild swings in benchmarks of the new-solver, which is not enabled by default, yet.
Apart from that, we got a very few notable changes, only one unexpected speedup from a bugfix in rustdoc.

Triage done by **@panstromek**.
Revision range: [7dc2c162..3659db0d](https://perf.rust-lang.org/?start=7dc2c162b9c197aaa76a6f9e7534569537830a01&end=3659db0d3e2cd634c766fcda79ed118eca31a9fd&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean   | range           | count |
|:----------------------------------:|:------:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.2%   | [0.2%, 0.2%]    | 3     |
| Regressions ❌ <br /> (secondary)  | 162.1% | [0.2%, 1116.3%] | 20    |
| Improvements ✅ <br /> (primary)   | -1.4%  | [-8.4%, -0.1%]  | 7     |
| Improvements ✅ <br /> (secondary) | -1.1%  | [-8.4%, -0.1%]  | 11    |
| All ❌✅ (primary)                 | -0.9%  | [-8.4%, 0.2%]   | 10    |


1 Regression, 1 Improvement, 4 Mixed; 3 of them in rollups
17 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/9f1bc6e374b5ae202366df1cbef850b79be8c641/triage/2026/2026-07-06.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Lint against invalid POSIX symbol definitions](https://github.com/rust-lang/rust/pull/158522)
* [Document NonNull layout guarantees](https://github.com/rust-lang/rust/pull/158325)
* [Tracking Issue for `slice_split_once`](https://github.com/rust-lang/rust/issues/112811)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Let the OS handle stack growth](https://github.com/rust-lang/compiler-team/issues/1011)
* [Add `target_feature_available_at_call_site`](https://github.com/rust-lang/compiler-team/issues/1010)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Empty repr(Rust) enums are ZSTs](https://github.com/rust-lang/reference/pull/2293)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Update RFC template](https://github.com/rust-lang/rfcs/pull/3982)
* [RFC: Store registry tokens in the OS credential store by default](https://github.com/rust-lang/rfcs/pull/3981)

## Upcoming Events

Rusty Events between 2026-07-08 - 2026-08-05 🦀

### Virtual
* 2026-07-08 | Virtual (Cardiff, GB) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Operating Systems Book Club: Introduction + Processes**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315506435/)
* 2026-07-08 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/jv9lom12)
* 2026-07-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315517604/)
* 2026-07-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/21k797xr)
* 2026-07-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-16 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/312045926/)
* 2026-07-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329045/)
* 2026-07-21 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Learning Rust as First Programming Language**](https://www.meetup.com/women-in-rust/events/315102297/)
* 2026-07-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/315279653/)
* 2026-07-22 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/hd8mlw56)
* 2026-07-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254777/)
* 2026-07-29 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/uo5ek1f4)
* 2026-07-30 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/312045928/)
* 2026-08-02 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095294/)
* 2026-08-04 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315213885/)
* 2026-07-29 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ii2jrwva)
* 2026-08-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210367/)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)

### Africa:
* 2026-07-14 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Debugging a production grade Open Source Rust crate**](https://www.meetup.com/johannesburg-rust-meetup/events/315573758/)

### Europe
* 2026-07-08 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Join us live and INPERSON for Rust 262**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 015**](https://www.meetup.com/rust-berlin/events/315585121/)
* 2026-07-09 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Building Cross Platform Applications with Ply**](https://www.meetup.com/rust-rhein-main/events/315366165/)
* 2026-07-09 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-07-15 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Teach and Hack at Projektspeicher**](https://www.meetup.com/rust-dortmund/events/315496876/)
* 2026-07-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Supercharge Rust funcs with implicit arguments and context-generic programming**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816470/)
* 2026-07-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework for Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)
* 2026-07-30 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester July Code Night**](https://www.meetup.com/rust-manchester/events/315037685/)

### North America
* 2026-07-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust July Meetup**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-09 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315338107/)
* 2026-07-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, July 11**](https://www.meetup.com/bostonrust/events/315225865/)
* 2026-07-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
* 2026-07-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**July, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520812/)
* 2026-07-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, July 18**](https://www.meetup.com/bostonrust/events/315225872/)
* 2026-07-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314997214/)
* 2026-07-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 2026-07-22 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust in Distributed Systems with Flight Science!**](https://www.meetup.com/rust-los-angeles/events/315376271/)
* 2026-07-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Porter Square Rust Lunch, July 25**](https://www.meetup.com/bostonrust/events/315582650/)
* 2026-07-25 | Brooklyn, NY, US | [Flower](https://flowercomputer.com/)
    * [**BOG-A-THON 2**](https://partiful.com/e/Vq9fyDNCMSO7ia4ulK5b)
* 2026-07-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539329/)
* 2026-08-01 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Chinatown Rust Lunch, Aug 1**](https://www.meetup.com/bostonrust/events/315582653/)
* 2026-08-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Evening Boston Rust Meetup at Red Hat, Aug 4**](https://www.meetup.com/bostonrust/events/314660176/)

### Oceania
* 2026-07-09 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/events/)
    * [**Rust Brisbane • July 2026**](https://www.meetup.com/rust-brisbane/events/315563251/)
* 2026-07-21 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**July Meetup**](https://www.meetup.com/rust-canberra/events/315307280/)
* 2026-07-23 | Perth, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group)
    * [**Rust Perth: July Meetup!**](https://www.meetup.com/perth-rust-meetup-group/events/315451138/)
* 2026-07-30 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**Rust Melbourne July 2026**](https://www.meetup.com/rust-melbourne/events/315039480/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> if a ptr is dereferenced in a forest and nobody hears it, is it sound?

– [Kornel on rust-users](https://users.rust-lang.org/t/does-the-indirection-of-a-pointer-immediately-create-a-reference/141071/10)

Thanks to [Cerber-Ursi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1785) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1ureq0r/this_week_in_rust_659/)</small>
