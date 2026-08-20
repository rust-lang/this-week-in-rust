Title: This Week in Rust 665
Number: 665
Date: 2026-08-19
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
* [Experiment in reducing target directory size on nightly](https://blog.rust-lang.org/inside-rust/2026/08/18/reducing-target-dir-size-on-nightly/)

### Newsletters
* [The Embedded Rustacean Issue #78](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-78)

### Project/Tooling Updates
* [OxiSH: a modern, memory-safe SSH server](https://dirkjan.ochtman.nl/writing/2026/08/13/announcing-oxish.html)
* [A critical review of Xilem in 2026](https://hackmd.io/@s_haMSbyTAOWfoXc1aYNUg/Hka74gCwZg)
* [rama v0.4.0](https://github.com/plabayo/rama/releases/tag/rama-0.4.0)

### Observations/Thoughts
* [From Go to Rust](https://rolandsdev.blog/posts/from-go-to-rust/)
* [What Zig felt like, coming from Rust](https://besok.github.io/posts/what-zig-felt-like-coming-from-rust/)
* [I want extern "fil-c"](https://domenkozar.com/2026/08/13/i-want-extern-fil-c/)
* [Four levels of in-place initialization](https://blog.yoshuawuyts.com/four-levels-of-in-place-initialization/)
* [Protecting the Rust standard library from accidental breakage](https://predr.ag/blog/protecting-the-rust-stdlib-from-breakage/)
* [Zero-copy wgpu rendering inside an Electron app](https://murlet.com/blog/rendering-wgpu-under-electron/)
* [The Lint That Would Have Caught It Is Off by Default](https://ai2rules.dev/blog/the-lint-that-was-off-by-default/)
* [video] [series] [Implementing State Machines (Part 1)](https://youtu.be/q4GdfJNKI-M)

### Rust Walkthroughs
* [A gentle introduction to Embedded Rust](https://niss36.github.io/blog/01-gentle-intro-to-embedded-rust/)
* [Building a Containerized RESTful API](https://learning-rust.github.io/labs/building-a-containerized-restful-api/)

### Research
* [GPU Offload in Rust: Portable, Safe, and Fast](https://arxiv.org/pdf/2608.13759)

### Miscellaneous
* [Zerocopy with Joshua Liebow-Feeser](https://joshlf.com/posts/netstack-fm-ep-10/)

## Crate of the Week

This week's crate is [tokio_with_wasm](https://crates.io/crates/tokio_with_wasm), a crate that lets a single tokio codebase run both natively and in web browsers.

Thanks to [Kim Dong-Hyun](https://users.rust-lang.org/t/crate-of-the-week/2704/1654) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)
* [Tracking Issue for `-Zembed-metadata`](https://github.com/rust-lang/cargo/issues/15495)

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.


## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [sysknife - Export the audit chain rows, not just the verify verdict](https://github.com/lacs-project/sysknife/issues/215)
* [sysknife - Expose the read-only actions as MCP tools without exposing AptUpdate](https://github.com/lacs-project/sysknife/issues/216)
* [sysknife - Record a current Fedora Atomic validation run](https://github.com/lacs-project/sysknife/issues/217)
* [YantrikDB - Migrate the 7 remaining manual SAVEPOINT sites to SavepointGuard (panic-unwind hole + 7 hand-rolled copies of the unwind rule)](https://github.com/yantrikos/yantrikdb/issues/100)
* [RustAPI - chore: issue templates, drop missing triage label, MSRV 1.85 (easy)](https://github.com/Tuntii/RustAPI/issues/261)
* [KayaDB - test: one extra named malformed WAL / command-frame decoder case (easy)](https://github.com/Tuntii/KayaDB/issues/46)
* [Cordial - GameActivity.getWaterfallInsets has the wrong JNI descriptor](https://github.com/luohoa97/cordial/issues/11)
* [Cordial - ro.soc.manufacturer is answered with an empty string](https://github.com/luohoa97/cordial/issues/12)
* [Cordial - Map which FLog channels take a number and which take a severity name](https://github.com/luohoa97/cordial/issues/13)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

613 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-08-11..2026-08-18

#### Compiler
* [inline some hot new-solver functions](https://github.com/rust-lang/rust/pull/160892)
* [stabilize `-Zprofile-sample-use`](https://github.com/rust-lang/rust/pull/155942)
* [stabilize `extern "custom"`](https://github.com/rust-lang/rust/pull/158504)
* [make `ShardedHashMap::with_capacity` split capacity between shards](https://github.com/rust-lang/rust/pull/161127)
* [three new-solver speedups](https://github.com/rust-lang/rust/pull/160605)
* [use `TyOrConstInferVar` in the next solver, fix #158441](https://github.com/rust-lang/rust/pull/158436)

#### Library
* [add `core::num::Complex`](https://github.com/rust-lang/rust/pull/158885)
* [`Arc`: remove unnecessary `fmt::Display` use for overflow assertion](https://github.com/rust-lang/rust/pull/160731)
* [core/num: implement feature `float_nan_to`](https://github.com/rust-lang/rust/pull/161250)
* [core: implement `FusedIterator` for `StepBy`](https://github.com/rust-lang/rust/pull/159963)
* [experiment: add `core::cmp::smallest` and `core::cmp::largest`](https://github.com/rust-lang/rust/pull/160687)
* [`Iterator::{min,max}(_by_key)` should use overridden `min`/`max`/`lt`](https://github.com/rust-lang/rust/pull/160203)
* [`offload!` function-like macro](https://github.com/rust-lang/rust/pull/161055)
* [optimize CStr backing slice bounds checks](https://github.com/rust-lang/rust/pull/161040)
* [single-byte ASCII searcher for `StrSearcherImpl(pattern.rs)`](https://github.com/rust-lang/rust/pull/160408)

#### Cargo
* [`trim-paths`: honor workspace prefix override from env](https://github.com/rust-lang/cargo/pull/17349)
* [`frontmatter`: Don't panic on a short closing fence before a non-ASCII char](https://github.com/rust-lang/cargo/pull/17274)
* [`min-publish-age`: remove `registry.min-publish-age`](https://github.com/rust-lang/cargo/pull/17353)
* [enable `-Zembed-metadata=no` by default on nightly Cargo](https://github.com/rust-lang/cargo/pull/17267)
* [re-stabilize build-dir layout v2](https://github.com/rust-lang/cargo/pull/17354)
* [remove unremap file when running cargo clean -p in new build-dir layout](https://github.com/rust-lang/cargo/pull/17356)

#### Rustdoc
* [add basic `splat` support to `rustdoc`](https://github.com/rust-lang/rust/pull/160882)
* [add new `unused_footnote_definition` rustdoc lint](https://github.com/rust-lang/rust/pull/137858)
* [also warn if an invalid `doc` attribute is used on a macro invocation](https://github.com/rust-lang/rust/pull/161003)

#### Clippy
* [add `option_zip_none` lint](https://github.com/rust-lang/rust-clippy/pull/17465)
* [clean-up `used_underscore_*`](https://github.com/rust-lang/rust-clippy/pull/17308)
* [fix ICE on `unnecessary_rest_pattern` for TyAlias](https://github.com/rust-lang/rust-clippy/pull/17557)
* [fix `unfulfilled_lint_expectations` incorrectly triggered by `#[expect(clippy::let_and_return)]`](https://github.com/rust-lang/rust-clippy/pull/17045)
* [fix duplicate diagnostics for `min_rust_version_invalid_attr`](https://github.com/rust-lang/rust-clippy/pull/17396)
* [perf: check fn kind before the expansion walk in `missing_const_for_thread_local`](https://github.com/rust-lang/rust-clippy/pull/17581)
* [perf: resolve the callee before the expansion walk in `VecArgs::hir`](https://github.com/rust-lang/rust-clippy/pull/17582)
* [perf: run `in_external_macro` after the cheap checks in five hot lint paths](https://github.com/rust-lang/rust-clippy/pull/17276)

#### Rust-Analyzer
* [parser: frontmatter error path for UTF-8](https://github.com/rust-lang/rust-analyzer/pull/23159)
* [avoid panic for mismatched associated type parameters](https://github.com/rust-lang/rust-analyzer/pull/23118)
* [check original type for `replace_arith_op`](https://github.com/rust-lang/rust-analyzer/pull/22225)
* [consider loop containing `break expr` to diverge if `expr` is diverging](https://github.com/rust-lang/rust-analyzer/pull/23127)
* [do not panic when defined in macro from input](https://github.com/rust-lang/rust-analyzer/pull/23122)
* [don't error on tail comma for some macro](https://github.com/rust-lang/rust-analyzer/pull/23134)
* [emit E0600 when unary `!`/`-` is applied to unsupported type](https://github.com/rust-lang/rust-analyzer/pull/23147)
* [every workspace should have a proc-macro server](https://github.com/rust-lang/rust-analyzer/pull/23111)
* [fix `rustc_private` support for `rustc_proc_macro`](https://github.com/rust-lang/rust-analyzer/pull/23140)
* [lower range expressions in hir lowering](https://github.com/rust-lang/rust-analyzer/pull/23115)
* [return an error const to the solver when consteval fails](https://github.com/rust-lang/rust-analyzer/pull/23138)
* [offer `replace_arith` on references to ints](https://github.com/rust-lang/rust-analyzer/pull/23109)
* [support Reborrow and CoerceShared built-in derives](https://github.com/rust-lang/rust-analyzer/pull/22325)

### Rust Compiler Performance Triage

There were almost no regressions this week, while the next trait solver saw several significant performance
improvements!

Triage done by **@kobzol**.
Revision range: [771916f9..8fa1c96c](https://perf.rust-lang.org/?start=771916f9028e7fe56d2685f2c4f698de5d7d6a45&end=8fa1c96cfd489e4c27654c144ae871ce2c4db6c6&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 0.5%]    | 6     |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.2%, 1.0%]    | 17    |
| Improvements ✅ <br /> (primary)   | -0.5% | [-1.7%, -0.2%]  | 166   |
| Improvements ✅ <br /> (secondary) | -2.3% | [-16.0%, -0.1%] | 219   |
| All ❌✅ (primary)                 | -0.5% | [-1.7%, 0.5%]   | 172   |

0 Regressions, 6 Improvements, 7 Mixed; 4 of them in rollups
50 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/ca70287715cb2c2b10aed04506acb0ee5574c3fe/triage/2026/2026-08-18.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Refactor the libs team](https://github.com/rust-lang/rfcs/pull/3984)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Always escape grapheme extenders in `str::escape_debug`](https://github.com/rust-lang/rust/pull/158303)
* [feat: add symmetric PartialEq impls for Vec, &[T], &mut [T] versus Cow<'_, [T]>](https://github.com/rust-lang/rust/pull/156160)
* [stabilize smart pointer map functions](https://github.com/rust-lang/rust/pull/160534)
* [Stabilize `windows_process_extensions_main_thread_handle`](https://github.com/rust-lang/rust/pull/160108)
* [Add `Default` implementation for `std::sync::Once`](https://github.com/rust-lang/rust/pull/160136)
* [target_features: sse (or at least avx2) is incompatible with soft-float ABI](https://github.com/rust-lang/rust/pull/160302)
* [Remove `From<!> for T` *reservation* impl](https://github.com/rust-lang/rust/pull/160705)
* [stabilize `Box::take`](https://github.com/rust-lang/rust/pull/160436)
* [Extend `dropping_{references,copy_types}` lints to `drop_in_place`](https://github.com/rust-lang/rust/pull/160229)
* [lint on more incorrect usages of `core::ffi::c_void`](https://github.com/rust-lang/rust/pull/159986)
* [Make let-else respect macro_rules expr metavariable grouping](https://github.com/rust-lang/rust/pull/158515)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(resolver): Stabilize min-publish-age](https://github.com/rust-lang/cargo/pull/17335)
* [feat(diag): Stabilize cargo-lints ](https://github.com/rust-lang/cargo/pull/17298)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Expose `target_abi = "v8plus"` on sparc-unknown-linux-gnu](https://github.com/rust-lang/compiler-team/issues/1028)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Change `i686-pc-windows-msvc` from Tier 1 with host tools => Tier 1 without host tools](https://github.com/rust-lang/rfcs/pull/3999)
* [Owning references (`&own T`)](https://github.com/rust-lang/rfcs/pull/4000)

<!-- Call for Testing Message (post in GH `issue` and remove `call-for-testing` label) -->
This RFC will appear in the **Call for Testing** section of the next issue (#) of This Week in Rust (TWiR).
You may remove the `call-for-testing` label.  Please feel free to leave the `call-for-testing` label in place if you would like this RFC to appear again in another issue of TWiR.

## Upcoming Events

Rusty Events between 2026-08-19 - 2026-09-16 🦀

### Virtual
* 2026-08-19 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Dealing with Dependencies**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #5 — Wireless Communication with the IEEE 802.15.4 protocol**](https://www.meetup.com/charlottesville-rust-meetup/events/315733791/)
* 2026-08-21 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/1bm27cah)
* 2026-08-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254775/)
* 2026-08-26 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Lottery and Multi-CPU Scheduling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316083375/)
* 2026-08-27 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/313345334/)
* 2026-08-28 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/arkkrcj5)
* 2026-08-31 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Workshop: Add tests to an open source Rust project**](https://luma.com/nwfmsdtf)
* 2026-09-01 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Tauri: Cross-Platform desktop applications with Rust and web technologies**](https://luma.com/d9w26vav)
* 2026-09-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcmbdb/)
* 2026-09-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/316107210/)
* 2026-09-04 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/sqf4ux01)
* 2026-09-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316133872/)
* 2026-09-08 - 2026-09-11 | Hybrid (Montreal, CA) | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026**](https://rustconf.com/)
* 2026-09-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254774/)
* 2026-09-08 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315859305/)
* 2026-09-10 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Solving Real-World Planning Problems in Rust with SolverForge**](https://luma.com/rfbzk3ae)
* 2026-09-10 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315691423/)
* 2026-09-10 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619611/)
* 2026-09-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcmbtb/)
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314233757/)

### Africa
* 2026-09-08 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
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
* 2026-08-20 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Summer Party!**](https://www.meetup.com/rust-berlin/events/316151073/)
* 2026-08-20 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Building an acoustic camera with egui and embassy**](https://www.meetup.com/rust-rhein-main/events/315855368/)
* 2026-08-21 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/316115136/)
* 2026-08-26 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Third Meetup**](https://pretix.eu/rust-dresden/on-location-3)
* 2026-08-27 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester August Talks**](https://www.meetup.com/rust-manchester/events/315891530/)
* 2026-08-29 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #29**](https://www.meetup.com/stockholm-rust/events/316130996/)
* 2026-09-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/316169040/)
* 2026-09-14 - 2026-09-16 | Berlin, DE | [Oxidize 2026](https://oxidizeconf.com/)
    * [**Oxidize 2026**](https://oxidizeconf.com/)
* 2026-09-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Reproducing scientific papers - with Rust & "AI"**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816477/)

### North America
* 2026-08-19 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Dealing with Dependencies**](https://www.meetup.com/vancouver-rust/events/314105333/)
* 2026-08-19 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust August Meetup**](https://luma.com/00f2s7q9)
* 2026-08-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315929355/)
* 2026-08-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: 'Data Shapes Your Memory' and 'Rust in Peace'**](https://www.meetup.com/rust-nyc/events/316056830/)
* 2026-08-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315171660/)
* 2026-08-26 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA August! Rust in Quantum Computing**](https://www.meetup.com/rust-los-angeles/events/315963062/)
* 2026-08-27 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539331/)
* 2026-09-03 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/316124372/)
* 2026-09-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Cryptography + Quantum Computers**](https://www.meetup.com/stl-rust/events/315603673/)
* 2026-09-08 - 2026-09-11 | Hybrid (Montreal, CA) | [RustConf](https://rustconf.com/)
    * [**RustConf**](https://rustconf.com/)
* 2026-09-08 | Montreal, QC, CA | [Rust Foundation](https://rustfoundation.org/)
    * [**Rust Teams Health Summit**](https://rustfoundation.org/event/rust-teams-health-summit/)
* 2026-09-08 - 2026-09-11 | Montreal, QC, CA | [RustConf 2026](https://rustconf.com/)
    * [**RustConf 2026, Hosted by the Rust Foundation**](https://rustconf.com/schedule/)
* 2026-09-09 | Montreal, CA | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**RustConf Coffee Break Meetup**](https://www.meetup.com/women-in-rust/events/315773005/)
* 2026-09-10 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust September Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/315601104/)
* 2026-09-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314997217/)
* 2026-09-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314233757/)

### Oceania
* 2026-08-27 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne August 2026**](https://www.meetup.com/rust-melbourne/events/315039490/)
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

> ass-bind is peak

– [Clar Fon on rust-zulip discussion "assumptions on binders" abbreviations](https://rust-lang.zulipchat.com/#narrow/channel/326132-t-types.2Fmeetings/topic/2026-08-11/near/615874481)

Thanks to [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1790) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1vt8nni/this_week_in_rust_665/)</small>
