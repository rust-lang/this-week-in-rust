Title: This Week in Rust 663
Number: 663
Date: 2026-08-05
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
* [Enabling the next iteration of the borrow checker on nightly](https://blog.rust-lang.org/2026/08/04/enabling-polonius-alpha-on-nightly/)
* [Funding team progress update](https://blog.rust-lang.org/inside-rust/2026/08/04/funding-team-progress-update-july-2026/)
* [rust-lang/rust is adopting an LLM policy](https://blog.rust-lang.org/inside-rust/2026/08/05/rust-langrust-is-adopting-an-llm-policy/)
* [All Hands 2026 retrospective](https://blog.rust-lang.org/inside-rust/2026/07/31/all-hands-2026-retrospective/)

### Newsletters
* [The Embedded Rustacean Issue #77](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-77)

### Project/Tooling Updates
* [Kevat 0.4.0 — fast, resumable copy and move to external drives, now with a GUI on all three platforms](https://kevat.app/)
* [kache 0.13.0: keying the env vars proc-macros read](https://github.com/kunobi-ninja/kache/releases/tag/v0.13.0)
* [kobe 101: lease a Kubernetes cluster, don't create one](https://kunobi.ninja/blog/kobe-101-leasing-kubernetes-clusters)
* [Rewriting FalkorDB in Rust: Make It Work, Make It Stable](https://www.falkordb.com/blog/rewriting-falkordb-in-rust/)
* [Announcing `webrtc` v0.20.0: Async-Friendly, Runtime-Agnostic WebRTC on Sans-I/O Core `rtc`](https://webrtc.rs/blog/2026/07/31/announcing-webrtc-v0.20.0.html)
* [Proxelar 0.5.0: sessions, rules, and more ways to capture traffic](https://micheletti.io/proxelar-050/)
* [mirador 1.0.0: a personal terminal dashboard](https://github.com/jchultarsky/mirador/releases/tag/v1.0.0)
* [BitFun 0.2.15: an open-source desktop AI agent built on a Rust runtime](https://github.com/GCWing/BitFun/releases/tag/v0.2.15)
* [mvis v0.5.0: CI/CD Profiling & Allocation Histograms](https://dev.to/sicklefire/mvis-v050-new-release-5997)
* [multicalc 0.9.0: scientific computation for embedded and robotics systems](https://github.com/kmolan/multicalc-rust/releases/tag/v0.9.0)
* [Auto-correcting wrong-layout typing on Wayland is nearly impossible. We did it anyway](https://poltertype.com/blog/wrong-layout-typing-on-wayland/)
* [wimux 0.1.0: a native Windows terminal multiplexer](https://github.com/fabperso/wimux/releases/tag/v0.1.0)
* [amtr: a btop-style context-window monitor for Claude Code sessions, and the forensic autopsy of its own 153-hour build](https://github.com/arian-shamaei/anthropometer/tree/main/docs/autopsy)
* [RSigma v0.20.0 release](https://github.com/timescale/rsigma/releases/tag/v0.20.0)
* [The State of RSigma](https://mostafa.dev/the-state-of-rsigma-7ba0a99020d9), and [Part Two: The Loop](https://mostafa.dev/the-state-of-rsigma-part-two-the-loop-c114f379dd78)

### Observations/Thoughts
* [How Firecracker microVMs work under the hood to sandbox untrusted code and AI agents](https://kerkour.com/firecracker-sandboxing-rust)
* [Faster floating point math with Rust’s new API](https://pythonspeed.com/articles/faster-float-math-rust/)
* [Rust MEMS drivers: 3 reasons to try and adopt our new sensor driver](https://blog.st.com/rust-mems-drivers/)
* [The Bedrock of Software Design | Alex Fedoseev](https://alex.draftist.io/blog/the-bedrock-of-software-design-ycqvcedsj)
* [Tail-Call Interpreters in Rust](https://lordgoati.us/blog/tail-call/)
* [How to speed up the Rust compiler in July 2026](https://nnethercote.github.io/2026/07/31/how-to-speed-up-the-rust-compiler-in-july-2026.html)
* [Sovereign Tech Fellowship for Rust maintenance (June-July 2026 report)](https://kobzol.github.io/rust/2026/08/03/stf-june-july-2026.html)
* [An old-new take on argument parsing in Rust](https://jmmv.dev/2026/07/hello-getoptsargs.html)
* [Stateless Servers, Stateful Payloads: Sessions vs Continuations, Measured in Rust](https://dmitrii.app/stateless-servers-stateful-payloads-sessions-vs-continuations-measured-in-rust/)
* [video] [Rust in the age of Generative AI with Niko, Allen & Zeeshan](https://www.youtube.com/watch?v=2937MGszrak)
* [audio] [Rust in Production S06 E09: JetBrains with Orhun Parmaksız](https://corrode.dev/podcast/s06e09-jetbrains/)
* [Work-Stealing vs. Executor-Per-Thread: Evaluating different HTTP server workloads with Tokio, Smol and Glommio](https://c410-f3r.github.io/thoughts/work-stealing-vs-executor-per-thread-evaluating-different-http-server-workloads-with-tokio-smol-and-glommio/)
* [Your `#[target_feature(enable = "avx2")]` does nothing on `x86_64-unknown-uefi`](https://github.com/Aefinity-AI/alice-aegis/blob/main/docs/posts/2026-08-05_uefi-soft-float-deletes-your-avx2.md)
* [Three bugs my AI agents couldn't fix](https://dev.to/fabperso/three-bugs-my-ai-agents-couldnt-fix-13bn)


### Rust Walkthroughs
* [Blinking an LED on STM32 Blue Pill (STM32F103C8T6) with Embedded Rust](https://blog.implrust.com/posts/2026/08/blinky-with-stm32f103c8t6-embedded-rust/)
* [Branchless Rust: Making a Filter 4x Faster by Removing an `if`](https://www.greyblake.com/blog/branchless-rust/)
* [Why modern font metrics cannot reproduce Word pagination](https://oxi-dd65f4.gitlab.io/articles/word-pagination-gdi-rounding.html)
* [Building hooklog on a six-day-old framework](https://github.com/JuanMarchetto/hooklog/blob/main/ARTICLE.md)

## Crate of the Week

This week's crate is [index_type](https://crates.io/crates/index_type), a crate for providing strongly typed indices for collections.

Thanks to [Roee Shoshani](https://users.rust-lang.org/t/crate-of-the-week/2704/1638) for the self-suggestion!

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
* [Cordial - Unify the two implementations of the profile lock](https://github.com/luohoa97/cordial/issues/6)
* [Cordial - Fullscreen clips and letterboxes until the workspace is switched away and back](https://github.com/luohoa97/cordial/issues/7)
* [Dofigen - Extend Dockerfiles](https://github.com/lenra-io/dofigen/issues/481)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

630 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-28..2026-08-04

#### Compiler
* [improve CFG traversal](https://github.com/rust-lang/rust/pull/160193)
* [perf: avoid a heap allocation per basic block in MoveData's location maps](https://github.com/rust-lang/rust/pull/160245)
* [stabilize passing 128-bit integers via vector registers with `asm!` on x86](https://github.com/rust-lang/rust/pull/159525)

#### Library
* [a bit optimize four-digit chunks in integer formatting](https://github.com/rust-lang/rust/pull/159130)
* [add NEON support for `is_ascii` and `eq_ignore_ascii_case`](https://github.com/rust-lang/rust/pull/160143)
* [add semver check test command for checking API compatibility of stdlib](https://github.com/rust-lang/rust/pull/159671)
* [allow only implementing `Read::read_buf`](https://github.com/rust-lang/rust/pull/106643)
* [core: implement bounded random sampling](https://github.com/rust-lang/rust/pull/159592)
* [iter: specialize `Take::count` using `advance_by`](https://github.com/rust-lang/rust/pull/160139)
* [iter: specialize `advance_by` method of `Fuse`](https://github.com/rust-lang/rust/pull/160342)
* [make atomic operations const](https://github.com/rust-lang/rust/pull/160079)
* [move `std::io::copy` to `alloc::io`](https://github.com/rust-lang/rust/pull/158548)
* [stabilize `size_of_val_raw, align_of_val_raw, Layout::for_value_raw`](https://github.com/rust-lang/rust/pull/157572)

#### Cargo
* [add a suggestion when adding `[lints]` to a workspace to use `[workspace.lints]` instead](https://github.com/rust-lang/cargo/pull/17300)
* [avoid parsing unchanged lockfiles](https://github.com/rust-lang/cargo/pull/17301)
* [completions: complete paths for cargo run arguments](https://github.com/rust-lang/cargo/pull/17284)
* [fix `manual_readme` lint for lower-priority README files](https://github.com/rust-lang/cargo/pull/17208)
* [git: make checkout names independent of git config](https://github.com/rust-lang/cargo/pull/17289)
* [make `__CARGO_TEST_FORCE_ARGFILE` available in distributed builds](https://github.com/rust-lang/cargo/pull/17293)
* [pass rustdoc flags to final CCI merge step](https://github.com/rust-lang/cargo/pull/17269)
* [prevent panic when `package.build` is empty](https://github.com/rust-lang/cargo/pull/17268)
* [reworked how we enable the new build-dir layout on nightly](https://github.com/rust-lang/cargo/pull/17272)
* [trim-paths: unambiguous and reversible remap rules](https://github.com/rust-lang/cargo/pull/17302)

#### Rustdoc
* [label badge for notable traits](https://github.com/rust-lang/rust/pull/157058)
* [rustdoc-json: make `Stability` compatible with non-self-describing serde formats](https://github.com/rust-lang/rust/pull/160032)
* [fix ICE when a grapheme cluster joins a Prepend-class character to `_` or `:`](https://github.com/rust-lang/rust/pull/160232)
* [fix crash when trying to list attributes on an opaque type](https://github.com/rust-lang/rust/pull/160208)
* [only analyze head of self type when deciding impl inlining](https://github.com/rust-lang/rust/pull/159854)

#### Rustfmt
* [format `cfg_select!`](https://github.com/rust-lang/rust/pull/154202)

#### Clippy
* [`manual_div_ceil`: avoid suggestions that change evaluation count](https://github.com/rust-lang/rust-clippy/pull/17468)
* [fix `no_effect_underscore_binding` false positive on proc-macro generated code](https://github.com/rust-lang/rust-clippy/pull/17473)
* [add check for image with embedded link to `doc_paragraphs_missing_punctuation`](https://github.com/rust-lang/rust-clippy/pull/16773)
* [lint for UFCS call in `clone_on_copy`](https://github.com/rust-lang/rust-clippy/pull/16972)
* [trigger `float_cmp_const` for `assert_eq!` with const floats](https://github.com/rust-lang/rust-clippy/pull/17024)

#### Rust-Analyzer
* [allow `self` as the last segment of a path](https://github.com/rust-lang/rust-analyzer/pull/23014)
* [correctly handle unlinked module edge cases](https://github.com/rust-lang/rust-analyzer/pull/22977)
* [support `CovariantUnsafeCell`](https://github.com/rust-lang/rust-analyzer/pull/22959)
* [add `-Zjson-target-spec` on cargo calls where needed](https://github.com/rust-lang/rust-analyzer/pull/21846)
* [add reference for same name param coerce matches](https://github.com/rust-lang/rust-analyzer/pull/23003)
* [allow diverging rhs in destructuring assignments](https://github.com/rust-lang/rust-analyzer/pull/23017)
* [avoid panic when checking `Copy` for hrtb closure arguments](https://github.com/rust-lang/rust-analyzer/pull/22938)
* [detect the rust-analyzer component in a multi-line components array](https://github.com/rust-lang/rust-analyzer/pull/22996)
* [do not alloc anon consts for bare paths in blocks](https://github.com/rust-lang/rust-analyzer/pull/22965)
* [don't panic on a self-referential `impl Trait` function](https://github.com/rust-lang/rust-analyzer/pull/22992)
* [double stack size for threads to 16MiB](https://github.com/rust-lang/rust-analyzer/pull/22956)
* [exclude unknown types from term search](https://github.com/rust-lang/rust-analyzer/pull/23015)
* [fix lookup `MACRO_CALL@...` in this Semantics due to include!](https://github.com/rust-lang/rust-analyzer/pull/22933)
* [fix `ExprScopes` handling of exprs inside patterns](https://github.com/rust-lang/rust-analyzer/pull/23008)
* [fix glob import shadowing bug](https://github.com/rust-lang/rust-analyzer/pull/22886)
* [make mir debug execution work fot bitflags items](https://github.com/rust-lang/rust-analyzer/pull/22948)
* [mark auto traits as coinductive](https://github.com/rust-lang/rust-analyzer/pull/22943)
* [no hint with similar name raw-ident arg](https://github.com/rust-lang/rust-analyzer/pull/22957)
* [parse postfix range inside closure in access](https://github.com/rust-lang/rust-analyzer/pull/23004)
* [recognize format arguments after a backslash in raw strings](https://github.com/rust-lang/rust-analyzer/pull/22993)
* [resolve assignment lhs in its expression scope](https://github.com/rust-lang/rust-analyzer/pull/23016)
* [show qualified paths when type names collide in E0308](https://github.com/rust-lang/rust-analyzer/pull/22964)
* [hir-ty, ide-diagnostics: use E0057/E0061 for arg-count mismatch (was E0107)](https://github.com/rust-lang/rust-analyzer/pull/22947)
* [perf: avoid having a separate query for defined opaques](https://github.com/rust-lang/rust-analyzer/pull/22966)
* [perf: save an allocation in lifetime handling](https://github.com/rust-lang/rust-analyzer/pull/23001)
* [report a config error for postfix snippets with item scope](https://github.com/rust-lang/rust-analyzer/pull/22937)
* [`vfs`: use component-based path prefix matching for virtual paths](https://github.com/rust-lang/rust-analyzer/pull/22940)

### Rust Compiler Performance Triage
A lot of optimizations landed this week. Some big improvements to rustdoc in [#159854](https://github.com/rust-lang/rust/pull/159854), one big improvement in control flow graph traversal for `cranelift-codegen`, few more improvements to next-solver benchmarks and various other micro-optimizations, bringing the total to a nice round number of 10 improvements this week.

Triage done by **@panstromek**.
Revision range: [ad0c9dce..65dd30fb](https://perf.rust-lang.org/?start=ad0c9dce27a22416b65946bc0010edaf22ac6c83&end=65dd30fb9e882a7e8f0be10caca62936db2a98b8&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.3%  | [0.2%, 0.5%]    | 18    |
| Regressions ❌ <br /> (secondary)  | 2.1%  | [0.1%, 16.8%]   | 64    |
| Improvements ✅ <br /> (primary)   | -3.3% | [-39.8%, -0.2%] | 97    |
| Improvements ✅ <br /> (secondary) | -6.1% | [-39.6%, -0.1%] | 111   |
| All ❌✅ (primary)                 | -2.7% | [-39.8%, 0.5%]  | 115   |


1 Regression, 5 Improvements, 11 Mixed; 6 of them in rollups
32 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/c41ca2a96f74761503b333d9f416eb7012eef858/triage/2026/2026-08-03.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Tracking Issue for `core_io_borrowed_buf`](https://github.com/rust-lang/rust/issues/117693)
* [Tracking Issue for `derive_macro_global_path`](https://github.com/rust-lang/rust/issues/154645)
* [stabilize `c_variadic_naked_functions`](https://github.com/rust-lang/rust/pull/159746)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Implement a naming convention for lint/diagnostic-only `rustc_` attrs](https://github.com/rust-lang/compiler-team/issues/1021)
* [Encode OpenBSD `-current` version in targets' `target_env`](https://github.com/rust-lang/compiler-team/issues/1018)
* [Add `target_feature_available_at_call_site`](https://github.com/rust-lang/compiler-team/issues/1010)
* [Promote `wasm32-wasip3` to Tier 2](https://github.com/rust-lang/compiler-team/issues/1001)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-08-05 - 2026-09-02 🦀

### Virtual
* 2026-08-05 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Execution and Scheduling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315880365/)
* 2026-08-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210367/)
* 2026-08-07 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ii2jrwva)
* 2026-08-10 | Hybrid (Kuala Lumpur, Malaysia) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup August 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfwGMGqDit9jn9INA1EROWTbvnjTAZAO1oUQaEwqmao7AYy1A/viewform)
* 2026-08-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254776/)
* 2026-08-13 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/313345333/)
* 2026-08-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619609/)
* 2026-08-14 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/f2hnzrug)
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
* 2026-08-21 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/arkkrcj5)
* 2026-09-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)

### Africa
* 2026-08-11 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust's extended standard library**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-10 | Hybrid (Kuala Lumpur, MY) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup August 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfwGMGqDit9jn9INA1EROWTbvnjTAZAO1oUQaEwqmao7AYy1A/viewform)
* 2026-08-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/august-2026-rustacean-meetup/)
* 2026-08-22 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi X SciPy India Meetup**](https://www.meetup.com/rustdelhi/events/315185336/)
* 2026-08-22 | Noida, IN | [SciPy India](https://scipy.in/)
    * [**Scientific Computing in Rust and Python**](https://scipy.in/sci-py-rs/)
* 2026-08-29 | Pune, IN | [Rust Pune](https://hasgeek.com/rustpune/)
    * [**Rust Pune Meetup: August 2026**](https://hasgeek.com/rustpune/meetup-august-2026/)

### Europe
* 2026-08-05 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in August: Don't panic! …or_else?**](https://www.meetup.com/rustcologne/events/315910506/)
* 2026-08-06 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 016**](https://www.meetup.com/rust-berlin/events/315966137/)
* 2026-08-06 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**ACCU/Rust Summer social**](https://www.meetup.com/oxford-rust-meetup-group/events/315863373/)
* 2026-08-13 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night: Trust but verify the LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)
* 2026-08-20 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Building an acoustic camera with egui and embassy**](https://www.meetup.com/rust-rhein-main/events/315855368/)
* 2026-08-27 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)

### North America
* 2026-08-06 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315590399/)
* 2026-08-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Shipping Temporal: How a Global Rust Ecosystem Built Chrome’s Newest Web API**](https://www.meetup.com/stl-rust/events/314701905/)
* 2026-08-11 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: 'An intro to wgpu' and 'Let's Talk Generics!'**](https://www.meetup.com/rust-nyc/events/315963710/)
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
* 2026-08-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315171660/)
* 2026-08-26 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust LA August! Rust in Quantum Computing**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 2026-08-27 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539331/)

### Oceania
* 2026-08-27 | Melbourne, AU | [Rust Melbourne](https://luma.com/rustmelbourne)
    * [**Rust Melbourne Meetup**](https://luma.com/d0rndgyv)

### South America
* 2026-08-08 | São Paulo, SP | [Rust-SP](https://luma.com/calendar/cal-bif2oHITU1aVvsr)
    * [**Rust SP - Aug/2026**](https://luma.com/41oiyhtk)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ttbtf5/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> … but I gave up on the idea as the macro rules were turning into a turing complete rust syntax parser

– [Koosha on rust-users](https://users.rust-lang.org/t/crate-of-the-week/2704/1637)

Thanks to [miro](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1787) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1vgv7sn/this_week_in_rust_663)</small>
