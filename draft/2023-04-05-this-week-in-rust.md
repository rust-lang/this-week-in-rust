Title: This Week in Rust 489
Number: 489
Date: 2023-04-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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
* [Rust Foundation Joins Open Infrastructure Foundation as Associate Member](https://foundation.rust-lang.org/news/rust-foundation-joins-open-infrastructure-foundation-as-associate-member/)
* [Rust Identified as Safer Coding Tool by NIST](https://foundation.rust-lang.org/news/rust-identified-as-safer-coding-tool-by-nist/)
* [Rewarding Resilience: Rust & the U.S. National Cybersecurity Strategy](https://foundation.rust-lang.org/news/rewarding-resilience-rust-the-u-s-national-cybersecurity-strategy/)

### Newsletters

### Project/Tooling Updates
* [Helix editor 23.03 released](https://helix-editor.com/news/release-23-03-highlights/)
* [Rustls 0.21.0 Released With Exciting New Features](https://www.memorysafety.org/blog/rustls-new-features/)
* [Slint 1.0: The Next-Generation Native GUI Toolkit Matures](https://slint-ui.com/blog/announcing-slint-1.0.html)
* [rust-analyzer changelog #175](https://rust-analyzer.github.io/thisweek/2023/04/03/changelog-175.html)
* [Introducing SeaStreamer - a stream processing toolkit for Kafka and Redis Streams](https://www.sea-ql.org/blog/2023-04-03-intro-sea-streamer/)
* [youki 0.0.5 has been released](https://github.com/containers/youki/releases/tag/v0.0.5)
* [Fluvio Connector Development Kit released](https://www.fluvio.io/news/this-week-in-fluvio-0050/)


### Observations/Thoughts
* [Thoughts on async closures](https://smallcultfollowing.com/babysteps/blog/2023/03/29/thoughts-on-async-closures/)
* [How to speed up the Rust compiler in March 2023](https://nnethercote.github.io/2023/03/24/how-to-speed-up-the-rust-compiler-in-march-2023.html)
* [Two things that Rust does better than C++](https://getdozer.io/blog/rust-cpp-move-and-dispatch/)
* [Rust Tidbits #1](https://www.thecodedmessage.com/posts/rust-tidbits-1/)
* [Follow up on cracking ZIP archives in Rust](https://agourlay.github.io/follow-up-cracking-zip-rust/)
* [audio] [AI-NC with Tom Miles](https://rustacean-station.org/episode/tom-miles/)
* [audio] [Servo with Josh Matthews](https://rustacean-station.org/episode/josh-matthews/)

### Rust Walkthroughs
* [Building a Classic Mac OS App in Rust](https://www.wezm.net/v2/posts/2023/rust-classic-mac-os-app/)
* [Cross Compiling Rust Projects in GitHub Actions](https://blog.urth.org/2023/03/05/cross-compiling-rust-projects-in-github-actions/)
* [Rust on the CH32V003](https://noxim.xyz/blog/rust-ch32v003/)
* [Build your own CountMinSketch in Rust](https://www.arunma.com/2023/04/02/build-your-own-countminsketch-in-rust/)
* [ZH] [Build a Lua interpreter in Rust](https://wubingzheng.github.io/build-lua-in-rust/zh/)
* [Nine Rules for Creating Fast, Safe, and Compatible Data Structures in Rust (Part 1): Lessons from RangeSetBlaze](https://towardsdatascience.com/nine-rules-for-creating-fast-safe-and-compatible-data-structures-in-rust-part-1-c0973092e0a3)
* [A definitive guide to sealed traits in Rust](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/)

### Research

### Miscellaneous
* [video] [Rust Wasm-Bindgen in 2 Minutes: Bridging the Gap Between WASM & JavaScript](https://youtu.be/y_nGGbM2eaU)
* [Rust for C#/.NET Developers](https://microsoft.github.io/rust-for-dotnet-devs/latest/)

## Crate of the Week

This week's crate is [keshvar](https://github.com/pouriya/keshvar), a library providing a host of information on every country.

Thanks to [Pouriya](https://users.rust-lang.org/t/crate-of-the-week/2704/1182) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

390 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-03-27..2023-04-03

* [initial support for return type notation (RTN)](https://github.com/rust-lang/rust/pull/109010)
* [specialization involving RPITITs is broken so ignore the diagnostic differences](https://github.com/rust-lang/rust/pull/109708)
* [add a builtin `FnPtr` trait that is implemented for all function pointers](https://github.com/rust-lang/rust/pull/108080)
* [middle: emit error rather than delay bug when reaching limit](https://github.com/rust-lang/rust/pull/109856)
* [closures always implement `FnOnce` in new solver](https://github.com/rust-lang/rust/pull/109739)
* [correctly substitute GAT's type used in `normalize_param_env` in `check_type_bounds`](https://github.com/rust-lang/rust/pull/109470)
* [do not consider elaborated projection predicates for objects in new solver](https://github.com/rust-lang/rust/pull/109675)
* [don't ICE on `DiscriminantKind` projection in new solver](https://github.com/rust-lang/rust/pull/109748)
* [don't ICE on placeholder consts in deep reject](https://github.com/rust-lang/rust/pull/109740)
* [erase impl regions when checking for impossible to eagerly monomorphize items](https://github.com/rust-lang/rust/pull/109321)
* [freshen normalizes-to hack goal RHS in the evaluate loop](https://github.com/rust-lang/rust/pull/109679)
* [improve error message when writer is forgotten in write and writeln macro](https://github.com/rust-lang/rust/pull/109149)
* [make init mask lazy for fully initialized/uninitialized const allocations](https://github.com/rust-lang/rust/pull/109670)
* [numeric vars can only be unified with numerical types in deep reject](https://github.com/rust-lang/rust/pull/109750)
* [simplify transmutes in MIR InstCombine](https://github.com/rust-lang/rust/pull/109612)
* [stable MIR: Add basic MIR body datastructures](https://github.com/rust-lang/rust/pull/109224)
* [stop special-casing `'static` in evaluation](https://github.com/rust-lang/rust/pull/102472)
* [suggest ..= when someone tries to create an overflowing range](https://github.com/rust-lang/rust/pull/109554)
* [drop array patterns using subslices](https://github.com/rust-lang/rust/pull/109008)
* [add `IndexSlice` to go with `IndexVec`](https://github.com/rust-lang/rust/pull/109787)
* [use `&IndexSlice` instead of `&IndexVec` where possible](https://github.com/rust-lang/rust/pull/109819)
* [partial stabilization of `once_cell`](https://github.com/rust-lang/rust/pull/105587)
* [optimize `LazyCell` size](https://github.com/rust-lang/rust/pull/109483)
* [drop unstable `Option::contains`, Result::contains`, Result::contains_err`](https://github.com/rust-lang/rust/pull/108095)
* [use span of placeholders in `format_args!()` expansion](https://github.com/rust-lang/rust/pull/109664)
* [change `advance(_back)_by` to return the remainder instead of the number of processed elements](https://github.com/rust-lang/rust/pull/92284)
* [`binary_heap`: Optimize Extend implementation](https://github.com/rust-lang/rust/pull/108448)
* [stabilize `binary_heap_retain`](https://github.com/rust-lang/rust/pull/109701)
* [hashbrown: optimize insertion to only use a single lookup](https://github.com/rust-lang/hashbrown/pull/277)
* [codegen\_gcc: optimize bitreverse codegen](https://github.com/rust-lang/rustc_codegen_gcc/pull/257)
* [cargo: add delays to network retries](https://github.com/rust-lang/cargo/pull/11881)
* [rustdoc + rustdoc-json support for `feature(non_lifetime_binders)`](https://github.com/rust-lang/rust/pull/108335)
* [rustdoc: run more HIR validation to mirror rustc](https://github.com/rust-lang/rust/pull/108576)
* [clippy: add large future lint](https://github.com/rust-lang/rust-clippy/pull/10414)
* [clippy: added the `unnecessary_box_returns` lint](https://github.com/rust-lang/rust-clippy/pull/9102)
* [clippy: flag `bufreader.lines().filter_map(Result::ok)` as suspicious](https://github.com/rust-lang/rust-clippy/pull/10534)
* [clippy: add suggestions to `extra_unused_type_parameters`](https://github.com/rust-lang/rust-clippy/pull/10536)
* [clippy: `arithmetic_side_effects`: correctly handle division and module when the right-hand-side is unknown](https://github.com/rust-lang/rust-clippy/pull/10585)
* [clippy: fix `nonminimal_bool #[allow]` attributes](https://github.com/rust-lang/rust-clippy/pull/10588)
* [clippy: fix allow attribute, items from macros in `items_after_statements`](https://github.com/rust-lang/rust-clippy/pull/10542)
* [clippy: ignore `file!()` macro in `print_literal`, `write_literal`](https://github.com/rust-lang/rust-clippy/pull/10573)
* [clippy: in uninit checking, add fallback for polymorphic types](https://github.com/rust-lang/rust-clippy/pull/10553)
* [rust-analyzer: expand Macro Recursively: don't append "!" to non-bang macro name](https://github.com/rust-lang/rust-analyzer/pull/14468)
* [rust-analyzer: feat: pop a notification prompting the user to add a Cargo.toml of unlinked file to the linkedProjects](https://github.com/rust-lang/rust-analyzer/pull/14366)
* [rust-analyzer: fix stack overflow in `is_ty_uninhabited_from`](https://github.com/rust-lang/rust-analyzer/pull/14426)
* [rust-analyzer: add missing autoborrow adjustment for index expressions](https://github.com/rust-lang/rust-analyzer/pull/14435)
* [rust-analyzer: allow new, subsequent `rust-project.json`-based workspaces to get proc macro expansion](https://github.com/rust-lang/rust-analyzer/pull/14427)
* [rust-analyzer: canonicalize rust-project.json manifest path](https://github.com/rust-lang/rust-analyzer/pull/14430)
* [rust-analyzer: handle box and raw pointers correctly in `builtin_deref`](https://github.com/rust-lang/rust-analyzer/pull/14440)
* [rust-analyzer: lower adjusts in simple index except the last one](https://github.com/rust-lang/rust-analyzer/pull/14464)
* [rust-analyzer: properly handle local trait impls](https://github.com/rust-lang/rust-analyzer/pull/14424)
* [rust-analyzer: recover from `pub()` visibility modifier](https://github.com/rust-lang/rust-analyzer/pull/14449)
* [rust-analyzer: use `struct_tail_without_normalization` in `Expectation::rvalue_hint`](https://github.com/rust-lang/rust-analyzer/pull/14434)
* [rust-analyzer: use async block in async fn type inference](https://github.com/rust-lang/rust-analyzer/pull/14461)
* [rust-analyzer: limited syntax support for return type notations (RTN)](https://github.com/rust-lang/rust-analyzer/pull/14465)
* [rust-analyzer: missing runnable env on debug target](https://github.com/rust-lang/rust-analyzer/pull/14444)

### Rust Compiler Performance Triage

A large improvement in const evaluation (particularly for large types) in
[#109670](https://github.com/rust-lang/rust/pull/109670) and a large
improvement to many-paged rustdoc workloads in
[#109876](https://github.com/rust-lang/rust/pull/109876) by removing quadratic
behavior. Regressions are comparatively limited this week.

Triage done by **@simulacrum**.
Revision range: [cbc064b341be231403d181402a786cce7f1c73f1..7c96e40da81165beef4f273f44e96eeef5a1bd30](https://perf.rust-lang.org/?start=cbc064b341be231403d181402a786cce7f1c73f1&end=7c96e40da81165beef4f273f44e96eeef5a1bd30&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 2.3%  | [0.4%, 8.4%]    | 18    |
| Regressions ❌ <br /> (secondary)  | 1.6%  | [0.3%, 10.7%]   | 26    |
| Improvements ✅ <br /> (primary)   | -2.5% | [-77.6%, -0.3%] | 63    |
| Improvements ✅ <br /> (secondary) | -4.0% | [-27.9%, -0.3%] | 52    |
| All ❌✅ (primary)                 | -1.4% | [-77.6%, 8.4%]  | 81    |


1 Regressions, 4 Improvements, 3 Mixed; 2 of them in rollups
54 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-04-04.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [standard lazy types](https://github.com/rust-lang/rfcs/pull/2788)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [rustdoc-search: add support for nested generics](https://github.com/rust-lang/rust/pull/109802)
* [disposition: merge] [Tracking Issue for const_ptr_read](https://github.com/rust-lang/rust/issues/80377)
* [disposition: merge] [Uplift `clippy::{drop,forget}_{ref,copy}` lints](https://github.com/rust-lang/rust/pull/109732)
* [disposition: close] [Tracking issue for the #[alloc_error_handler] attribute (for no_std + liballoc)](https://github.com/rust-lang/rust/issues/51540)
* [disposition: merge] [Implement Neg for signed non-zero integers.](https://github.com/rust-lang/rust/pull/102341)
* [disposition: merge] [Tracking Issue for IsTerminal / is_terminal](https://github.com/rust-lang/rust/issues/98070)
* [disposition: close] [Tracking Issue for array_zip](https://github.com/rust-lang/rust/issues/80094)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [ABI-stabilize `core::task::Waker`](https://github.com/rust-lang/rfcs/pull/3404)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-04-05 - 2023-05-03 🦀

### Virtual

* 2023-04-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/291967741/)
* 2023-04-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcgbhb/)
* 2023-04-06 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Rust Iran Meetup #9 - Let's Write An Async Executor**](https://rust-meetup.ir/2023/04/06/9th-meetup.html)
* 2023-04-08 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-04-08 | Virtual | [Ukrainian Rust Community](https://github.com/rust-lang-ua/learn_rust_together#-ukrainian-rust-community-)
    * [**UA Rust Conference 2023**](https://www.uarust.com/)
* 2023-04-11 | Virtual (Berlin, DE) | [Berline.rs - OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/292236794/)
* 2023-04-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfcgbpb/)
* 2023-04-11 | Virtual | [Rust Live](https://www.eventbrite.com/cc/rust-live-1876809)
    * [**Rust Live: Asynchronous Rust**](https://www.eventbrite.com/e/rust-live-asynchronous-rust-tickets-575865518267?aff=ebdssbonlinesearch&keep_tld=1)
* 2023-04-11 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 28u16**](https://www.meetup.com/rust-saar/events/292549070/)
* 2023-04-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcgbqb/)
* 2023-04-12 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Atomics and Locks Book Club Launch!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292410256/)
* 2023-04-13 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Learning Rust By Building Small CLI Tools!**](https://www.meetup.com/charlottesville-rust-meetup/events/292674779/)
* 2023-04-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsyfcgbrb/)
* 2023-04-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Introducing duplicate! and the peculiarities of proc macros**](https://www.meetup.com/rustdc/events/291830834/)
* 2023-04-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/-0)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/lqkkctyfcgbzb/)
* 2023-04-20 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/291965920/)
* 2023-04-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfcgbbc/)
* 2023-04-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcgbhc/)
* 2023-04-26 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust-friendly websites and web apps**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292559177/)
* 2023-04-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Testing Tock, how unit tests in Rust improve and teach**](https://www.meetup.com/charlottesville-rust-meetup/events/292193436/)
* 2023-04-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 3: Protohackers Exercises Mob Coding (as far as we get)**](https://www.meetup.com/rust-noris/events/292149688/)
* 2023-05-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfchbdb/)
* 2023-05-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfchbfb/)

### Asia

* 2023-04-08 | Beijing, CN | [Rust Chinese Group](https://www.meetup.com/rust-chinese-group/)
    * [**Rust Meetup Beijing**](https://www.meetup.com/rust-chinese-group/events/292379002/) 
* 2023-04-08 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Demystifying Closures**](https://www.meetup.com/kansai-rust/events/292202435/) 
* 2023-04-12 | Kuala Lumpur, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/); [Telegram](https://t.me/golangmalaysia)
    * [**Rust Meetup Malaysia April 2023: How far is Dioxus from React? by Ivan Tham**](https://www.google.com/calendar/event?eid=MWI0bWJzY21qZTI2NWsyZDgzOG0xb2JkdTkgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles) | [Map](https://goo.gl/maps/w2ogftac6mqpBbvt5)
* 2023-04-18 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Rewriting Relay's GraphQL Compiler in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/292549607/)

### Europe

* 2023-04-05 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Embedded Rust**](https://www.meetup.com/rustcologne/events/292561864/)
* 2023-04-06 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #3**](https://www.meetup.com/fr-FR/rust-lyon/events/292283973/)
* 2023-04-13 | Roma, IT | [Rust Roma](https://www.meetup.com/rust-roma/)
    * [**Rules engine: from good to awesome (and beyond) with Rust**](https://www.meetup.com/rust-roma/events/292684621/)
* 2023-04-13 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #33**](https://www.meetup.com/rust-wroclaw/events/292581415/)
* 2023-04-19 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #58**](https://www.meetup.com/rust-paris/events/292575461/)
* 2023-04-19 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Rust Embedded with MicroBit:V2**](https://www.meetup.com/rust-trondheim/events/292680021/)
* 2023-04-19 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**sett: data encryption and transfer made easy(ier)**](https://www.meetup.com/de-DE/rust-zurich/events/292151879/)
* 2023-04-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus meetup #1 at Geanix**](https://www.meetup.com/rust-aarhus/events/292185072/)
* 2023-04-20 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/291965920/)
* 2023-04-20 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**First Rust Bern Meetup!**](https://www.meetup.com/de-DE/rust-bern/events/292206056/)
* 2023-04-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfcgbcc/)
* 2023-05-02 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/292297784/)
* 2023-05-10 | Amsterdam, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2023**](https://2023.rustnl.org/)

### North America

* 2023-04-08 | Durham, NC, US | [Triangle Rust](https://www.meetup.com/triangle-rust/)
    * [**Rust Social / Coffee Chat at Boxyard RTP**](https://www.meetup.com/triangle-rust/events/292665089/)
* 2023-04-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Autometrics: Easily add metrics in Rust and understand them in Prometheus**](https://www.meetup.com/rust-nyc/events/292430796/)
* 2023-04-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcgbxb/)

### Oceania

* 2023-04-13 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - 🐰 April Thingy 😊**](https://www.meetup.com/rust-sydney/events/292163549/)

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

> As usual, the borrow checker is correct: we are doing memory crimes.

– [Ohad Ravid on his blog](https://ohadravid.github.io/posts/2023-03-rusty-python/)

Thanks to [Jelte Fennema](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1392) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
