Title: This Week in Rust 660
Number: 660
Date: 2026-07-15
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
* [Announcing Rust 1.97.0](https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/)
* [crates.io: development update](https://blog.rust-lang.org/2026/07/13/crates-io-development-update/)

### Foundation

### Newsletters

### Project/Tooling Updates
* [Rewriting Bun in Rust](https://bun.com/blog/bun-in-rust)
* [Announcing BullMQ for Rust](https://bullmq.io/news/260712/rust-release/)
* [prost-protovalidate 0.6 — buf.validate (protovalidate) for prost and buffa: compile-time codegen + runtime CEL, 2872/2872 conformance](https://github.com/zs-dima/prost-protovalidate/releases/tag/v0.6.0)
* [plaza 1.0: a ratatui package-manager TUI that searches pacman, the AUR, apt, dnf, and Flatpak at once](https://github.com/StaszeKrk/plaza/releases/tag/v1.0.0)
* [Danube v0.15.1: native Apache Iceberg integration for streaming-to-lakehouse export](https://github.com/danube-messaging/danube/releases/tag/v0.15.1)
* [Guardian Sentinel. The Terminal User Interface for Guardian Decentralized Database - P2P](https://www.willsearch.com.br/sentinel/)
* [kobe 0.33.0: a Rust operator for instant CI Kubernetes clusters](https://github.com/kunobi-ninja/kobe/releases/tag/v0.33.0)
* [Elara Mesh: what the black box for AI agents actually does](https://navigatorbuilds.github.io/elara-mesh/blog/black-box-for-ai-agents.html)
* [kache 0.10.0: instant download dedup, no more polling](https://github.com/kunobi-ninja/kache/releases/tag/v0.10.0)

* [cochlea 0.1.0: a headless, deterministic audio engine for AI agents](https://richer-richard.github.io/cochlea/)

### Observations/Thoughts
* Open Source Security Podcast: Rust Foundation Maintainers Fund with Lori and Niko (https://opensourcesecurity.io/2026/2026-07-rfmf-lori-niko/)
* [Moving a Rust WebRTC SFU to thread-per-core](https://pulsebeam.dev/blog/moving-to-thread-per-core)
* [Faster Rust tests in CI with parallel steps](https://abundance.build/blog/2026-07-11-faster-rust-tests-in-ci-with-parallel-steps/)
* [video] [The Only Diagram You Need to Understand Rust Ownership](https://www.youtube.com/watch?v=fugcSHD-9Jw)
* [We compiled our TypeScript parser to WASM](https://encore.dev/blog/typescript-parser-wasm)
* [Understanding the Rust hype for the busy developer](https://kerkour.com/rust-hype)
* [I red-teamed my own LLM security gateway (Rust) in four passes — every detection gap and how I closed it](https://dev.to/akavlabs_69/i-red-teamed-my-own-llm-security-gateway-in-four-passes-heres-every-gap-i-found-5cl9)

### Rust Walkthroughs
* [video] [Backend Concepts in Rust: HTTP Servers](https://www.youtube.com/watch?v=DJhhy6YQe8k)
* [Fearless Embedded Rust: A FPV Lego car](https://dystroy.org/blog/picamobile/)
* [What I learned building a self-corrupting file format in Rust](https://www.aravpanwar.com/writing/building-decayfmt-in-rust/)
* [Come Async You Are](https://corentin-core.github.io/posts/ruxe-async-runtime-agnostic/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [dashu](https://crates.io/crates/dashu), a pure Rust set of libraries of arbitrary precision numbers.

Thanks to [JacobZ](https://users.rust-lang.org/t/crate-of-the-week/2704/1628) for the self-suggestion!

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

* [Nika - showcase: CSV → chart PNG → markdown report (nika:chart has no example yet)](https://github.com/supernovae-st/nika/issues/424)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

550 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-07..2026-07-14

#### Compiler
* [inline some `Symbol` functions](https://github.com/rust-lang/rust/pull/158931)
* [predicate/clause cleanups](https://github.com/rust-lang/rust/pull/157104)
* [remove some AST `tokens` fields](https://github.com/rust-lang/rust/pull/158942)
* [resolver: wrap arenas in `WorkerLocal`](https://github.com/rust-lang/rust/pull/159019)
* [rework read deduplication with pooled read recorders](https://github.com/rust-lang/rust/pull/158794)
* [shrink `mir::Statement` to 40 bytes](https://github.com/rust-lang/rust/pull/159012)
* [shrink no-op drop elaboration](https://github.com/rust-lang/rust/pull/157491)
* [specialize common `(1, 1)` case for arg unification](https://github.com/rust-lang/rust/pull/158865)
* [use SmallVec for return places in MIR](https://github.com/rust-lang/rust/pull/158842)

#### Library
* [add explicit `Iterator::count` impl for `ChunkBy`](https://github.com/rust-lang/rust/pull/158866)
* [allow `Allocator`s to be used as `#[global_allocator]`s](https://github.com/rust-lang/rust/pull/157153)
* [fix multiple logic bugs in `Arc::make_mut`](https://github.com/rust-lang/rust/pull/158876)
* [implement feature `char_to_u32`](https://github.com/rust-lang/rust/pull/158940)
* [make volatile operations const](https://github.com/rust-lang/rust/pull/159092)
* [move `std::io::Write` to `core::io`](https://github.com/rust-lang/rust/pull/158541)
* [stabilize `String::from_utf8_lossy_owned`](https://github.com/rust-lang/rust/pull/159099)
* [stabilize `VecDeque::retain_back` from `truncate_front`](https://github.com/rust-lang/rust/pull/151379)

#### Cargo
* [`install`: Move --debug to Compilation options](https://github.com/rust-lang/cargo/pull/17199)
* [`source`: incorrect duplicate package warning](https://github.com/rust-lang/cargo/pull/17204)
* [fix manifest schema generation: `TomlDebugInfo` enum-variants doesn't renamed](https://github.com/rust-lang/cargo/pull/17202)
* [dont apply host-config gating to stable behavior](https://github.com/rust-lang/cargo/pull/17198)
* [reduce library search path length in new build dir layout](https://github.com/rust-lang/cargo/pull/17191)
* [reduce rustc `-L` args used in the new `build-dir` layout](https://github.com/rust-lang/cargo/pull/17168)
* [rename `-Zno-embed-metadata` to `-Zembed-metadata=no`](https://github.com/rust-lang/cargo/pull/17149)
* [test: fix race in `cargo_compile_with_invalid_code_in_deps`](https://github.com/rust-lang/cargo/pull/17203)

#### Clippy
* [add new lints: `rest_pattern_accessible_field` and `unnecessary_rest_pattern`](https://github.com/rust-lang/rust-clippy/pull/15000)
* [new lint: `definition_in_module_root`](https://github.com/rust-lang/rust-clippy/pull/16965)
* [`arbitrary_source_item_ordering`: add configurable trait impl item ordering modes](https://github.com/rust-lang/rust-clippy/pull/17343)
* [`tests_outside_test_module`: put code in backticks in the lint message](https://github.com/rust-lang/rust-clippy/pull/17387)
* [count length of the first paragraph by its text](https://github.com/rust-lang/rust-clippy/pull/17215)
* [fix `suboptimal_flops` false negative with ambiguous float literals](https://github.com/rust-lang/rust-clippy/pull/16980)
* [partly disable `unneeded_wildcard_pattern` when `rest_pattern_accessible_field` is enabled](https://github.com/rust-lang/rust-clippy/pull/17416)
* [respect the configured MSRV in `implicit_saturating_sub`'s `if x != 0 { x -= 1 }` rewrite](https://github.com/rust-lang/rust-clippy/pull/17404)
* [trigger `single_element_loop` if the block contains only a final expression](https://github.com/rust-lang/rust-clippy/pull/16513)
* [optimize `nonstandard_macro_braces` by 99.9683% (1.1b → 351K)](https://github.com/rust-lang/rust-clippy/pull/16808)
* [perf: bail out of the `disallowed_methods` rule if the disallowed list is empty](https://github.com/rust-lang/rust-clippy/pull/17381)

#### Rust-Analyzer
* [ask for disclosure in AI contributions](https://github.com/rust-lang/rust-analyzer/pull/22771)
* [add fixes for array length for `type_mismatch`](https://github.com/rust-lang/rust-analyzer/pull/22734)
* [add parens in transformed dyn type in ref type](https://github.com/rust-lang/rust-analyzer/pull/22741)
* [avoid panic in merge imports on trailing path separator](https://github.com/rust-lang/rust-analyzer/pull/22736)
* [change some things for `#[doc = macro!()]` expansion](https://github.com/rust-lang/rust-analyzer/pull/22654)
* [clamp cttz const-eval result to type width](https://github.com/rust-lang/rust-analyzer/pull/22770)
* [correctly handled cfg'ed tail expr, take 2](https://github.com/rust-lang/rust-analyzer/pull/22751)
* [crash on code actions when an unresolved module is present](https://github.com/rust-lang/rust-analyzer/pull/22749)
* [crash when computing diagnostics with MIR and error types](https://github.com/rust-lang/rust-analyzer/pull/22707)
* [don't complete default in default impl](https://github.com/rust-lang/rust-analyzer/pull/22744)
* [early late classification of lifetimes](https://github.com/rust-lang/rust-analyzer/pull/22283)
* [fix `render_const_using_debug_impl` constructing outdated std layouts](https://github.com/rust-lang/rust-analyzer/pull/22583)
* [fix proc macros `TokenStream::from_str()` for doc comments](https://github.com/rust-lang/rust-analyzer/pull/22735)
* [hide private fields on hover depending on context](https://github.com/rust-lang/rust-analyzer/pull/22464)
* [make lsp-server `Response` type closer aligned to JSON-RPC](https://github.com/rust-lang/rust-analyzer/pull/22753)
* [pretty assoc const when trait in macro](https://github.com/rust-lang/rust-analyzer/pull/22535)
* [reimplement `crate_supports_no_std` syntactic heuristic](https://github.com/rust-lang/rust-analyzer/pull/22747)
* [resolve non-plain paths in blocks correctly](https://github.com/rust-lang/rust-analyzer/pull/22773)
* [support Cargo 1.97.0 lockfile path setting](https://github.com/rust-lang/rust-analyzer/pull/22683)
* [hir-ty: walk container exprs for `unused_must_use`](https://github.com/rust-lang/rust-analyzer/pull/22405)
* [fix onEnter erroneously deleting/interpreting `$foo`](https://github.com/rust-lang/rust-analyzer/pull/22768)
* [suggest code action fixes produced from diagnostics under cursor, even if they have effects elsewhere](https://github.com/rust-lang/rust-analyzer/pull/22726)
* [treat library files as truly client immutable](https://github.com/rust-lang/rust-analyzer/pull/22777)
* [turn `BlockLoc` into a tracked struct, take 3](https://github.com/rust-lang/rust-analyzer/pull/22534)

### Rust Compiler Performance Triage

This week many new optimizations landed, making this a very good week for performance.
The only real regression was a fix for a miscompile that will likely be re-landed in the future.

Triage done by **@JonathanBrouwer**.
Revision range: [3659db0d..5503df87](https://perf.rust-lang.org/?start=3659db0d3e2cd634c766fcda79ed118eca31a9fd&end=5503df87342a73d0c29126a7e08dc9c1255c46ad&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.3%  | [0.2%, 0.4%]    | 3     |
| Regressions ❌ <br /> (secondary)  | 0.9%  | [0.1%, 2.5%]    | 25    |
| Improvements ✅ <br /> (primary)   | -1.2% | [-9.9%, -0.2%]  | 195   |
| Improvements ✅ <br /> (secondary) | -3.4% | [-92.1%, -0.1%] | 174   |
| All ❌✅ (primary)                 | -1.2% | [-9.9%, 0.4%]   | 198   |


2 Regressions, 10 Improvements, 10 Mixed; 7 of them in rollups
36 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/212da2d63f1edf2ab22293547a99f0fbf8cb68a8/triage/2026/2026-07-13.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Named `Fn` trait parameters](https://github.com/rust-lang/rfcs/pull/3955)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [enable `unreachable_cfg_select_predicates` lint as part of `unused` lint group](https://github.com/rust-lang/rust/pull/159179)
* [Stabilize `dyn Allocator`](https://github.com/rust-lang/rust/issues/156906)
* [Tracking Issue for vec_try_remove](https://github.com/rust-lang/rust/issues/146954)
* [Partially stabilize `box_vec_non_null`](https://github.com/rust-lang/rust/pull/157226)
* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Enable `-Zpolonius=next` on nightly](https://github.com/rust-lang/compiler-team/issues/1015)
* [Enable `-Znext-solver` on nightly by default for testing](https://github.com/rust-lang/compiler-team/issues/1014)
* [Stabilizing the state of the debuginfo test suite](https://github.com/rust-lang/compiler-team/issues/1012)
* [Optimize `repr(Rust)` enums by omitting tags in more cases involving uninhabited variants.](https://github.com/rust-lang/compiler-team/issues/922)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [bf16 primitive type](https://github.com/rust-lang/rfcs/pull/3983)

## Upcoming Events

Rusty Events between 2026-07-15 - 2026-08-12 🦀

### Virtual
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
* 2026-07-21 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/315676843/)
* 2026-07-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/315279653/)
* 2026-07-22 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/hd8mlw56)
* 2026-07-23 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315418155/)
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
* 2026-08-05 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/f2hnzrug)
* 2026-08-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210367/)
* 2026-08-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254776/)
* 2026-08-12 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/f2hnzrug)
* 2026-07-19 | Virtual (Bangalore, IN) | [Embedded Rust Discord](https://discord.gg/VJyv3NfVdw)
    * [**Silicon Sundays**](https://discord.gg/6gwCNpFP?event=1526087936234225814)

### Asia
* 2026-07-18 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/july-2026-rustacean-meetup/)
* 2026-07-19 | Virtual (Bangalore, IN) | [Embedded Rust Discord](https://discord.gg/VJyv3NfVdw)
    * [**Silicon Sundays**](https://discord.gg/6gwCNpFP?event=1526087936234225814)
* 2026-07-25 | Mumbai, IN | [Rust Mumbai](https://luma.com/mumbai)
    * [**​Rust Mumbai — July Meetup 🦀**](https://luma.com/7ksabwbm/)
* 2026-07-26 | Pune, MA, IN | [Rust Pune](https://www.meetup.com/rust-pune/events/)
    * [**Rust Pune: July 2026**](https://www.meetup.com/rust-pune/events/315651505/)

### Europe
* 2026-07-15 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Teach and Hack at Projektspeicher**](https://www.meetup.com/rust-dortmund/events/315496876/)
* 2026-07-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Supercharge Rust funcs with implicit arguments and context-generic programming**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816470/)
* 2026-07-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework for Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**LDN Talks: July 2026 Antithesis Takeover**](https://www.meetup.com/rust-london-user-group/events/315612916/)
* 2026-07-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)
* 2026-07-29 | Poland, PL | [Rust Poland](https://www.meetup.com/rust-poland-meetup)
    * [**Rust Poland x Kraków #10**](https://www.meetup.com/rust-poland-meetup/events/315582674/)
* 2026-07-30 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester July Code Night**](https://www.meetup.com/rust-manchester/events/315037685/)

### North America
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
* 2026-07-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Write A Custom Coding Agent and wasm_zero**](https://www.meetup.com/rust-nyc/events/315636854/)
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
* 2026-08-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Shipping Temporal: How a Global Rust Ecosystem Built Chrome’s Newest Web API**](https://www.meetup.com/stl-rust/events/314701905/)

### South America
* 2026-08-08 | São Paulo, SP | [Rust-SP](https://luma.com/calendar/cal-bif2oHITU1aVvsr)
    * [**Rust SP - Aug/2026**](https://luma.com/41oiyhtk)

### Oceania
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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Thank you for your PR, but please edit the description like you are a chainsaw-wielding maniac that just discovered the sentences are young adults who came to the lake at summer camp after sunset.

– [workingjubilee on Rust github](https://github.com/rust-lang/rust/pull/159039#issuecomment-4931084997)

Thanks to [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1786) for the suggestion!

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
