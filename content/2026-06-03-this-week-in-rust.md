Title: This Week in Rust 654
Number: 654
Date: 2026-06-03
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

* [Launching the Rust Foundation Maintainers Fund](https://blog.rust-lang.org/2026/06/02/launching-the-rust-foundation-maintainers-fund)

### Project/Tooling Updates

* [One year of Roto, the compiled scripting language for Rust](https://blog.nlnetlabs.nl/one-year-of-roto-the-compiled-scripting-language-for-rust/)
* [xa11y: cross-platform desktop automation via native accessibility APIs](https://crowecawcaw.github.io/general/2026/05/30/accessibility-for-computer-use.html)
* [halloy 2026.7 - now supports IRCv3 reply, redact, metadata, bot mode and more!](https://github.com/squidowl/halloy/releases/tag/2026.7)
* [Building a Native Markdown Previewer for AI-Generated Docs with Rust and WebView](https://vorojar.github.io/md-preview/rust-webview-ai-docs.html)
* [BPF in the agentic era](https://lwn.net/SubscriberLink/1075067/6e0bbea2010794b8/)

### Observations/Thoughts

* [Nine Ways to Do Inheritance in Rust, a Language Without Inheritance](https://medium.com/@carlmkadie/nine-ways-to-do-inheritance-in-rust-a-language-without-inheritance-14825bf1e215?v=1)
* [Async Rust: deep dive into cooperative scheduling and Tokio's architecture](https://kerkour.com/async-rust-cooperative-scheduling-tokio)

### Rust Walkthroughs

* [ZK snarks for Rust developers: R1CS vs Plonkish vs AIR](https://rustarians.com/r1cs-plonkish-air)
* [Learn Rust Closures By Building a Tiny Rule-Based Linter](https://blog.sheerluck.dev/posts/learn-rust-closures-by-building-a-tiny-linter/)
* [Learn Bevy States, Timers, and Grid Movement by Building Snake](https://blog.sheerluck.dev/posts/learn-bevy-states-timers-by-building-snake/)
* [video] [RustCurious lesson 8: Generics and Monomorphization](https://www.youtube.com/watch?v=WTmjbKk1EIk)

### Research

* [Counterfactuals via the Causal Monad in Rust](https://www.deepcausality.com/blog/counterfactuals-via-the-causal-monad/)

## Crate of the Week

This week's crate is [remyx](https://github.com/manuelgdlvh/remyx), a framework for building TUIs on top of Ratatui.

Thanks to [Manuel Garcia de la Vega](https://users.rust-lang.org/t/crate-of-the-week/2704/1608) for the self-suggestion!

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
<!-- or if none - *No Calls for participation were submitted this week.* -->
* [MD Preview - Package MD Preview for Homebrew Cask](https://github.com/vorojar/md-preview/issues/19)
* [OpenSlate - Test Health Check Endpoint](https://github.com/MrSheerluck/openslate/issues/7)
* [OpenSlate - Test Login Endpoint](https://github.com/MrSheerluck/openslate/issues/8)
* [OpenSlate - Test Notes CRUD Endpoint](https://github.com/MrSheerluck/openslate/issues/9)
* [OpenSlate - Test Search Endpoint](https://github.com/MrSheerluck/openslate/issues/10)
* [OpenSlate - Test Preference Endpoint](https://github.com/MrSheerluck/openslate/issues/11)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**Scientific Computing in Rust 2026**](https://scientificcomputing.rs/2026/submit-talk)| 2026-06-05 | Virtual | 2026-07-08 - 2026-07-10

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

500 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-05-26..2026-06-02

#### Compiler
* [expand async drops during drop elaboration](https://github.com/rust-lang/rust/pull/156649)
* [`offload_kernel` macro expansion](https://github.com/rust-lang/rust/pull/156642)
* [`std::offload` sharedmem](https://github.com/rust-lang/rust/pull/154835)

#### Library
* [constify Iterator-related methods and functions](https://github.com/rust-lang/rust/pull/156390)
* [move `IoSlice` and `IoSliceMut` to `core::io`](https://github.com/rust-lang/rust/pull/155849)
* [specialize Clone of array IntoIter](https://github.com/rust-lang/rust/pull/156634)
* [stabilize `Path::is_empty`](https://github.com/rust-lang/rust/pull/157065)
* [stop needing an alloca for `catch_unwind`](https://github.com/rust-lang/rust/pull/156867)

#### Cargo
* [`diag`: Add the `'cargo::default'` group](https://github.com/rust-lang/cargo/pull/17033)
* [`diag`: Report summaries for `unused_deps`](https://github.com/rust-lang/cargo/pull/17034)
* [add `--output-format=json` to cargo doc as an unstable option](https://github.com/rust-lang/cargo/pull/17025)
* [add edition for scripts anytime we mutate the manifest](https://github.com/rust-lang/cargo/pull/17038)

#### Rustdoc
* [avoid ICE when rendering body-less type consts](https://github.com/rust-lang/rust/pull/156851)
* [correctly propagate cfgs for glob reexports](https://github.com/rust-lang/rust/pull/157039)
* [deterministic sorting for `doc_cfg` badges](https://github.com/rust-lang/rust/pull/156401)
* [fix ICE on delegated async functions](https://github.com/rust-lang/rust/pull/157223)
* [optimize impl sorting](https://github.com/rust-lang/rust/pull/157179)
* [separate the caches for synthetic auto trait & blanket impls](https://github.com/rust-lang/rust/pull/157171)

#### Clippy
* [add `unused_async_trait_impl` lint](https://github.com/rust-lang/rust-clippy/pull/16244)
* [add new lint: `for_unbounded_range`](https://github.com/rust-lang/rust-clippy/pull/16257)
* [added new lint for `map_or(..., identity)`](https://github.com/rust-lang/rust-clippy/pull/16052)
* [`redundant_pattern_match`: improve suggestions](https://github.com/rust-lang/rust-clippy/pull/17116)
* [faster `has_arg`](https://github.com/rust-lang/rust-clippy/pull/17112)
* [fold all early lint passes into one statically-combined pass](https://github.com/rust-lang/rust-clippy/pull/17132)
* [fold all late lint passes into one statically-combined pass](https://github.com/rust-lang/rust-clippy/pull/17124)
* [memoize `first_node_in_macro` for consecutive queries](https://github.com/rust-lang/rust-clippy/pull/17134)
* [skip disabled off-by-default doc reparses](https://github.com/rust-lang/rust-clippy/pull/17126)

#### Rust-Analyzer
* [always use crates from sysroot in proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/22500)
* [enable salsa feature for syntax-bridge](https://github.com/rust-lang/rust-analyzer/pull/22504)
* [also consider library features internal](https://github.com/rust-lang/rust-analyzer/pull/22498)
* [do not fill both `drop()` and `pin_drop()` in the "fill missing members" assist](https://github.com/rust-lang/rust-analyzer/pull/22508)
* [fix extract variable in token tree replace range](https://github.com/rust-lang/rust-analyzer/pull/22447)
* [port block and loop inference from rustc](https://github.com/rust-lang/rust-analyzer/pull/22473)
* [try to improve completion ranking](https://github.com/rust-lang/rust-analyzer/pull/22503)
* [use add deref in assign instead add `&mut` for value](https://github.com/rust-lang/rust-analyzer/pull/22457)
* [kill proc-macro-srv processes on shutdown](https://github.com/rust-lang/rust-analyzer/pull/22506)
* [remove direct use of make constructor with editor make](https://github.com/rust-lang/rust-analyzer/pull/22477)
* [remove make from rename and prettify macro expansion](https://github.com/rust-lang/rust-analyzer/pull/22484)

### Rust Compiler Performance Triage

This week we saw nice wins across the board thanks to merging several compiler queries together ([#155678](https://github.com/rust-lang/rust/pull/155678)), and also substantial improvements in `doc` performance thanks to
doing less work when sorting trait impls ([#157179](https://github.com/rust-lang/rust/pull/157179)).

Triage done by **@Kobzol**.
Revision range: [783eb8c8..4804ad7e](https://perf.rust-lang.org/?start=783eb8c8682ddde0807c60ed8293670ef523794f&end=4804ad7e93e1b31f4605b7083871d0d3d85a2afe&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.3%  | [0.1%, 0.7%]   | 14    |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.1%, 0.9%]   | 39    |
| Improvements ✅ <br /> (primary)   | -0.9% | [-6.8%, -0.2%] | 111   |
| Improvements ✅ <br /> (secondary) | -1.1% | [-2.9%, -0.1%] | 53    |
| All ❌✅ (primary)                 | -0.8% | [-6.8%, 0.7%]  | 125   |

3 Regressions, 1 Improvement, 2 Mixed; 4 of them in rollups
35 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/4a082d37cfd5006c8313e55bab306ea41f091714/triage/2026/2026-06-01.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Tracking Issue for `strip_circumfix`](https://github.com/rust-lang/rust/issues/147946)
* [Tracking issue for CommandExt::show_window](https://github.com/rust-lang/rust/issues/127544)
* [Tracking Issue for `path_set_times`](https://github.com/rust-lang/rust/issues/147455)
* [Tracking Issue for `nonzero_from_str_radix`](https://github.com/rust-lang/rust/issues/152193)
* [Tracking Issue for LoongArch CRC Intrinsics](https://github.com/rust-lang/rust/issues/156908)
* [Tracking Issue for `Vec::from_fn`](https://github.com/rust-lang/rust/issues/149698)
* [Add `Step::forward/backward_overflowing` to enable RangeInclusive loop optimizations](https://github.com/rust-lang/rust/pull/155114)
* [Stabilize `core::range::{legacy, RangeFull, RangeTo}`](https://github.com/rust-lang/rust/pull/156629)
* [Tracking Issue for box_as_ptr](https://github.com/rust-lang/rust/issues/129090)
* [Tracking Issue for explicit-endian String::from_utf16](https://github.com/rust-lang/rust/issues/116258)
* [Reduce `unreachable-code` churn after `todo!()`](https://github.com/rust-lang/rust/pull/149543)
* [make repr_transparent_non_zst_fields a hard error](https://github.com/rust-lang/rust/pull/155299)
* [Tracking Issue for algebraic floating point methods](https://github.com/rust-lang/rust/issues/136469)
* [riscv: promote d, e, and f target_features to CfgStableToggleUnstable](https://github.com/rust-lang/rust/pull/156188)
* [Tracking Issue for `PathBuf::into_string`](https://github.com/rust-lang/rust/issues/156203)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Desugar async blocks in HIR instead of MIR](https://github.com/rust-lang/compiler-team/issues/997)
* [Test new solver and polonius alpha on CI](https://github.com/rust-lang/compiler-team/issues/996)
* [Add `-Zllvm-target-feature target` \*modifier\* to directly set LLVM-level target features, and deprecate doing that with `-Ctarget-feature`](https://github.com/rust-lang/compiler-team/issues/994)
* [Set requirements for windows-gnu](https://github.com/rust-lang/compiler-team/issues/993)
* [Create a new Tier 3 target: `powerpc64le-unknown-none`](https://github.com/rust-lang/compiler-team/issues/988)
* [Add flag to pass MSRV/`package.rust-version` for use by lints](https://github.com/rust-lang/compiler-team/issues/950)
* [Optimize `repr(Rust)` enums by omitting tags in more cases involving uninhabited variants.](https://github.com/rust-lang/compiler-team/issues/922)
* [Promote tier 3 riscv32 ESP-IDF targets to tier 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Can references to uninhabited types ever be valid?](https://github.com/rust-lang/unsafe-code-guidelines/issues/413)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [BTF relocations](https://github.com/rust-lang/rfcs/pull/3966)
* [`--allow-unstable-flags`: Allow unstable flags on stable](https://github.com/rust-lang/rfcs/pull/3965)

## Upcoming Events

Rusty Events between 2026-06-03 - 2026-07-01 🦀

### Virtual
* 2026-06-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/314691782/)
* 2026-06-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455930/)
* 2026-06-04 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345241/)
* 2026-06-04 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Exploring FalkorDB - Learning to use a Graph Database in Rust**](https://www.meetup.com/code-mavens/events/314979560/)
* 2026-06-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-06-07 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095285/)
* 2026-06-08 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**RustWeek Reflections**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315009040/)
* 2026-06-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254780/)
* 2026-06-09 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315060947/)
* 2026-06-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/3bcnx1jb)
* 2026-06-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjcjbvb/)
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
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjckbcb/)

### Africa
* 2026-06-09 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust by Example - Lifetimes**](https://www.meetup.com/johannesburg-rust-meetup/events/315070733/)

### Europe
* 2026-06-03 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Join us live and INPERSON for Rust 261**](https://www.meetup.com/rust-dublin/events/314689875/)
* 2026-06-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 06 2026**](https://luma.com/4bmlc7qd)
* 2026-06-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/313791798/)
* 2026-06-11 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-06-12 - 2026-06-14 | Kraków, PL | [Rustmeet](https://rustmeet.eu/)
    * [**Rustmeet**](https://rustmeet.eu/)
* 2026-06-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Interactive: Everything is Open Source**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813937/)
* 2026-06-16 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Real-time planning in Rust: SolverForge & SERIO**](https://www.meetup.com/rust-language-milan/events/314766950/)
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)
* 2026-06-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #86**](https://www.meetup.com/rust-paris/events/315040676/)
* 2026-06-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396600/)

### North America
* 2026-06-04 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/314983693/)
* 2026-06-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Testing, Coverage, Tracey & Mutations**](https://www.meetup.com/stl-rust/events/314106244/)
* 2026-06-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston Common Rust Lunch, June 6**](https://www.meetup.com/bostonrust/events/314480539/)
* 2026-06-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust June Meetup**](https://www.meetup.com/utah-rust/events/314696643/)
* 2026-06-11 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314825006/)
* 2026-06-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust June Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721899/)
* 2026-06-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314989012/)
* 2026-06-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcjbvb/)
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcjbgc/)
* 2026-06-24 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust-Based Constraint Solvers in 2D Sketching with Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-26 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Big Summer Social**](https://www.meetup.com/rust-nyc/events/315014582/)

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

> If memory safety bugs were Waldo (Wally): finding them in C programs is a "Where's Waldo?" game, and Rust's `unsafe` simplifies it to "Is *this* Waldo?"

– [kornel on rust-users](https://users.rust-lang.org/t/is-unsafe-rust-worse-than-c/140286/25)

Thanks to [Moy2010](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1776) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1twbzid/this_week_in_rust_654/)</small>
