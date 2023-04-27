Title: This Week in Rust 492
Number: 492
Date: 2023-04-26
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

* [Announcing Rust 1.69.0 | Rust Blog](https://blog.rust-lang.org/2023/04/20/Rust-1.69.0.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [pavex, a new Rust web framework - Progress report #3](https://www.lpalmieri.com/posts/pavex-progress-report-03/)
* [Changelog #178](https://rust-analyzer.github.io/thisweek/2023/04/24/changelog-178.html)
* [`regex` 1.8.0 release notes](https://github.com/rust-lang/regex/blob/master/CHANGELOG.md#180-2023-04-20)

* [r3bl_tui v0.3.3 TUI engine released](https://crates.io/crates/r3bl_tui)

### Observations/Thoughts

* [Is Rust a worthy contender for web development?](https://joshmo.hashnode.dev/can-rust-beat-javascript-in-2023)
* [Bringing runtime checks to compile time in Rust](https://ktkaufman03.github.io/blog/2023/04/20/rust-compile-time-checks/)
* [video] [Embeddable Rust](https://www.electronicdesign.com/technologies/embedded/software/video/21263235/electronic-design-embeddable-rust)

### Rust Walkthroughs
* [Rust + Embedded: A Development Power Duo](https://blog.espressif.com/rust-embedded-a-development-power-duo-db43dae21206)
* [A blog article and project demonstrating GitHub actions in Rust](https://blog.ngerakines.me/posts/github-actions-in-rust/)
* [Foresterre's place | Using the todo! macro to prototype your API in Rust](https://foresterre.github.io/posts/todo-macro-rust/)
* [Generics and Const Generics in Rust](https://sanjuvi.github.io/Blog/posts/Rust%20Generics/)
* [Writing an NES emulator: Part 1 - The 6502 CPU](https://analog-hors.github.io/site/pones-p1/)
* [ESP32 Embedded Rust at the HAL: GPIO Button Controlled Blinking](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-gpio-button-controlled-blinking?ref=twitter-share)
* [GBA From Scratch: A Basic Executable](https://lokathor.github.io/gba-from-scratch/ex1.html)
* [video] [A Practical Introduction to Declarative Macros in Rust](https://www.youtube.com/watch?v=dHv8FhcAl5U)

- [Guide to Rust procedural macros](https://developerlife.com/2022/03/30/rust-proc-macro/)

### Research

### Miscellaneous

* [Console #154 - An Interview with Giuliano of Sniffnet - Rust app to easily monitor network traffic](https://console.substack.com/p/console-154#§sniffnet)
* [DE] [Programmiersprache: Rust Foundation überarbeitet Trademark-Entwurf](https://www.heise.de/news/Programmiersprache-Rust-Foundation-ueberarbeitet-Trademark-Entwurf-8969427.html)

## Crate of the Week

This week's crate is [system-deps](https://crates.io/crates/system-deps), a crate that will compile your pkg-config-based dependencies for you.

Thanks to [Aleksey Kladov](https://users.rust-lang.org/t/crate-of-the-week/2704/1191) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hyperswitch - add `upsert` endpoint to `cards_info` table](https://github.com/juspay/hyperswitch/issues/994)
* [Hyperswitch - add a route that will invalidate cache](https://github.com/juspay/hyperswitch/issues/995)
* [Hyperswitch - Implement `ApiKeyInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/996)
* [Hyperswitch - Implement `ConfigInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/997)
* [velo - Add ability to switch canvas background - Issue #22 - StaffEngineer/velo - GitHub 3](https://github.com/StaffEngineer/velo/issues/22)
* [velo - Hex color widget - Issue #58 - StaffEngineer/velo - GitHub 1](https://github.com/StaffEngineer/velo/issues/58)
* [ockam - Update CLI documentation for `secure-channel-listener` commands 1](https://github.com/build-trust/ockam/issues/4774)
* [ockam - Update CLI documentation for `identity` commands](https://github.com/build-trust/ockam/issues/4777)
* [ockam - Refactor auto-reconnect replacer 1](https://github.com/build-trust/ockam/issues/4789)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

411 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-04-17..2023-04-24

* [add support for the `x86_64h-apple-darwin` target](https://github.com/rust-lang/rust/pull/108795)
* [support AIX-style archive type](https://github.com/rust-lang/rust/pull/106704)
* [`assume` value ranges in `transmute`](https://github.com/rust-lang/rust/pull/109993)
* [`rustc_metadata`: Remove `Span` from `ModChild`](https://github.com/rust-lang/rust/pull/109772)
* [add suggestion to use closure argument instead of a capture on borrowck error](https://github.com/rust-lang/rust/pull/110061)
* [deduplicate unreachable blocks, for real this time](https://github.com/rust-lang/rust/pull/110569)
* [delay a good path bug on drop for `TypeErrCtxt` (instead of a regular delayed bug)](https://github.com/rust-lang/rust/pull/110476)
* [ensure `mir_drops_elaborated_and_const_checked` when requiring codegen](https://github.com/rust-lang/rust/pull/110107)
* [fix ICE for transmutability in candidate assembly](https://github.com/rust-lang/rust/pull/110510)
* [fix lint regression in `non_upper_case_globals`](https://github.com/rust-lang/rust/pull/110575)
* [fix printing native CPU on cross-compiled compiler](https://github.com/rust-lang/rust/pull/110668)
* [make `impl Debug for Span` not panic on not having session globals](https://github.com/rust-lang/rust/pull/110548)
* [make `non_upper_case_globals` lint not report trait impls](https://github.com/rust-lang/rust/pull/110513)
* [make sysroot finding compatible with multiarch systems](https://github.com/rust-lang/rust/pull/110281)
* [missing blanket impl trait not public](https://github.com/rust-lang/rust/pull/110533)
* [normalize types and consts in MIR opts](https://github.com/rust-lang/rust/pull/110714)
* [panic instead of truncating if the incremental on-disk cache is too big](https://github.com/rust-lang/rust/pull/110632)
* [report allocation errors as panics](https://github.com/rust-lang/rust/pull/109507)
* [report more detailed reason why `Index` impl is not satisfied](https://github.com/rust-lang/rust/pull/110432)
* [set commit information environment variables when building tools](https://github.com/rust-lang/rust/pull/109981)
* [substitute missing trait items suggestion correctly](https://github.com/rust-lang/rust/pull/110555)
* [suggest using integration tests for test crate using own proc-macro](https://github.com/rust-lang/rust/pull/110255)
* [track if EvalCtxt has been tainted, make sure it can't be used to make query responses after](https://github.com/rust-lang/rust/pull/110618)
* [miri: add minimum alignment support for loongarch64](https://github.com/rust-lang/miri/pull/2852)
* [miri: disable preemption in tokio tests again](https://github.com/rust-lang/miri/pull/2848)
* [miri: remove a test that wasn't carrying its weight](https://github.com/rust-lang/miri/pull/2846)
* [don't transmute `&List<GenericArg> <-> &List<Ty>`](https://github.com/rust-lang/rust/pull/110496)
* [enable flatten-format-args by default](https://github.com/rust-lang/rust/pull/109999)
* [rm const traits in libcore](https://github.com/rust-lang/rust/pull/110393)
* [remove the size of locals heuristic in MIR inlining](https://github.com/rust-lang/rust/pull/110705)
* [don't allocate on SimplifyCfg/Locals/Const on every MIR pass](https://github.com/rust-lang/rust/pull/110477)
* [allow to feed a value in another query's cache and remove `WithOptConstParam`](https://github.com/rust-lang/rust/pull/96840)
* [implement `StableHasher::write_u128` via `write_u64`](https://github.com/rust-lang/rust/pull/110410)
* [in `LexicalResolver`, don't construct graph unless necessary](https://github.com/rust-lang/rust/pull/110527)
* [turn on ConstDebugInfo pass](https://github.com/rust-lang/rust/pull/107404)
* [run various queries from other queries instead of explicitly in phases](https://github.com/rust-lang/rust/pull/108118)
* [add `intrinsics::transmute_unchecked`](https://github.com/rust-lang/rust/pull/110706)
* [add `offset_of!` macro ](https://github.com/rust-lang/rust/pull/106934) (RFC [#3308](https://rust-lang.github.io/rfcs/3308-offset_of.html))
* [limit read size in `File::read_to_end` loop](https://github.com/rust-lang/rust/pull/110655)
* [specialize some `io::Read` and `io::Write` methods for `VecDeque<u8>` and `&[u8]`](https://github.com/rust-lang/rust/pull/110608)
* [implement `Neg` for signed non-zero integers](https://github.com/rust-lang/rust/pull/102341)
* [hashbrown: change key to return `&K` rather than `&Q`](https://github.com/rust-lang/hashbrown/pull/425)
* [hashbrown: relax the trait bounds of `HashSet::raw_table{,_mut}`](https://github.com/rust-lang/hashbrown/pull/423)
* [regex: fix prefix literal matching bug](https://github.com/rust-lang/regex/pull/984)
* [portable-simd: lane → element for `core::simd::Simd`](https://github.com/rust-lang/portable-simd/pull/338)
* [portable-simd: implement dynamic byte-swizzle prototype](https://github.com/rust-lang/portable-simd/pull/334)
* [cargo: add the `Win32_System_Console` feature since it is used](https://github.com/rust-lang/cargo/pull/12016)
* [cargo: allow named debuginfo options in Cargo.toml](https://github.com/rust-lang/cargo/pull/11958)
* [cargo: better error message when getting an empty dep table](https://github.com/rust-lang/cargo/pull/11997)
* [cargo: fix: allow win/mac credential managers to build on all platforms](https://github.com/rust-lang/cargo/pull/11993)
* [cargo: improve error message for empty dep](https://github.com/rust-lang/cargo/pull/12001)
* [clippy: `arithmetic_side_effects` cache symbols](https://github.com/rust-lang/rust-clippy/pull/10675)
* [clippy: `arithmetic_side_effects` detect integer methods that can introduce side effects](https://github.com/rust-lang/rust-clippy/pull/10615)
* [clippy: add `items_after_test_module` lint](https://github.com/rust-lang/rust-clippy/pull/10578)
* [clippy: add size-parameter to `unecessary_box_returns`](https://github.com/rust-lang/rust-clippy/pull/10651)
* [clippy: bugfix: ignore `impl Trait`(s) @ `let_underscore_untyped`](https://github.com/rust-lang/rust-clippy/pull/10701)
* [clippy: check for `..` pattern in `redundant_pattern_matching`](https://github.com/rust-lang/rust-clippy/pull/10707)
* [clippy: don't suggest `suboptimal_flops` unavailable in nostd](https://github.com/rust-lang/rust-clippy/pull/10670)
* [clippy: fix `#[allow(clippy::enum_variant_names)]` directly on variants](https://github.com/rust-lang/rust-clippy/pull/10696)
* [clippy: fix false positive in `allow_attributes`](https://github.com/rust-lang/rust-clippy/pull/10683)
* [clippy: ignore `manual_slice_size_calculation` in code from macro expansions](https://github.com/rust-lang/rust-clippy/pull/10667)
* [clippy: ignore `shadow` warns in code from macro expansions](https://github.com/rust-lang/rust-clippy/pull/10697)
* [clippy: make `len_zero` lint not spanning over parenthesis](https://github.com/rust-lang/rust-clippy/pull/10681)
* [clippy: new lint: detect `if` expressions with simple boolean assignments to the same target](https://github.com/rust-lang/rust-clippy/pull/10432)
* [clippy: suppress the triggering of some lints in derived structures](https://github.com/rust-lang/rust-clippy/pull/10203)
* [rust-analyzer: add `#[doc(alias(..))]`-based field and function completions](https://github.com/rust-lang/rust-analyzer/pull/14513)
* [rust-analyzer: don't wavy-underline the whole for loop](https://github.com/rust-lang/rust-analyzer/pull/14644)
* [rust-analyzer: `editor.parameterHints.enabled` not always being respected](https://github.com/rust-lang/rust-analyzer/pull/14618)
* [rust-analyzer: deduplicate passed workspaces by top level cargo workspace they belong to](https://github.com/rust-lang/rust-analyzer/pull/14603)
* [rust-analyzer: fix need-mut large span in closures and a false positive](https://github.com/rust-lang/rust-analyzer/pull/14619)
* [rust-analyzer: fix panic in const eval and parameter destructing](https://github.com/rust-lang/rust-analyzer/pull/14643)
* [rust-analyzer: fix pat fragment handling in 2021 edition](https://github.com/rust-lang/rust-analyzer/pull/14652)
* [rust-analyzer: mbe: fix token conversion for doc comments](https://github.com/rust-lang/rust-analyzer/pull/14625)
* [rust-analyzer: remove extra argument "rustc"](https://github.com/rust-lang/rust-analyzer/pull/14647)
* [rust-analyzer: report remaining macro errors in assoc item collection](https://github.com/rust-lang/rust-analyzer/pull/14634)
* [rust-analyzer: resolve `$crate` in derive paths](https://github.com/rust-lang/rust-analyzer/pull/14610)
* [rust-analyzer: register obligations during path inference](https://github.com/rust-lang/rust-analyzer/pull/14641)
* [rust-analyzer: simple fix for `make::impl_trait`](https://github.com/rust-lang/rust-analyzer/pull/14621)
* [rust-analyzer: specify `--pre-release` when publishing vsce nightly](https://github.com/rust-lang/rust-analyzer/pull/14648)

### Rust Compiler Performance Triage

A week mostly dominated by noise, in particular a persistent bimodality in
keccak and cranelift-codegen. No significant changes outside of that, a
relatively equal mix of regressions and improvements. Most of the bimodality
has been removed in the full report as it's just noise.

Triage done by **@simulacrum**.
Revision range: [74864fa..fdeef3e](https://perf.rust-lang.org/?start=74864fa496997a6498e623f0d2019ccb7eb6dad0&end=fdeef3ed1809aa9bd4ea9ff0fad92010c6de669c&absolute=false&stat=instructions%3Au)

3 Regressions, 6 Improvements, 5 Mixed; 1 of them in rollups
60 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-04-25.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: result_ffi_guarantees](https://github.com/rust-lang/rfcs/pull/3391)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Add RFC on governance, establishing the Leadership Council](https://github.com/rust-lang/rfcs/pull/3392)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Use fulfillment to check Drop impl compatibility](https://github.com/rust-lang/rust/pull/110577)
* [disposition: merge] [Only check outlives goals on impl compared to trait](https://github.com/rust-lang/rust/pull/109356)
* [disposition: merge] [rustdoc: restructure type search engine to pick-and-use IDs](https://github.com/rust-lang/rust/pull/110371)
* [disposition: merge] [Stabilize raw-dylib, link_ordinal, import_name_type and -Cdlltool](https://github.com/rust-lang/rust/pull/109677)
* [disposition: merge] [Add deployment-target --print flag for Apple targets](https://github.com/rust-lang/rust/pull/105354)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Rustdoc configuration via Cargo (includes feature descriptions)](https://github.com/rust-lang/rfcs/pull/3421)
* [new] [RFC: Partial Types](https://github.com/rust-lang/rfcs/pull/3420)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-04-26 - 2023-05-24 🦀

### Virtual

* 2023-04-26 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust-friendly websites and web apps**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292559177/)
* 2023-04-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Testing Tock, how unit tests in Rust improve and teach**](https://www.meetup.com/charlottesville-rust-meetup/events/292193436/)
* 2023-04-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #35 at Google Cloud**](https://www.meetup.com/copenhagen-rust-community/events/292424926/)
* 2023-04-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 3: Protohackers Exercises Mob Coding (as far as we get)**](https://www.meetup.com/rust-noris/events/292149688/)
* 2023-05-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfchbdb/)
* 2023-05-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfchbfb/)
* 2023-05-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfchbmb/)
* 2023-05-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/gmkpctyfchbpb/)
* 2023-05-13 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-05-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/jkxsctyfchbvb/)
* 2023-05-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Atomics and Locks Book Club Chapter 2**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292847157/)
* 2023-05-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/lqkkctyfchbwb/)

### Asia

* 2023-05-06 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
   * [**Rust Talk: Vec, arrays, and slices**](https://www.meetup.com/kansai-rust/events/293010553/)

### Europe

* 2023-04-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn April 2023**](https://www.meetup.com/rust-london-user-group/events/292729308/)
* 2023-04-27 | Bordeaux, FR | [DedoTalk](https://www.meetup.com/dedotalk/)
    * [**#2 DedoTalk 🎙️ : Comment tester son code Rust?**](https://www.meetup.com/dedotalk/events/292842782/)
* 2023-04-27 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna - April - Hosted by Sentry**](https://www.meetup.com/rust-vienna/events/292751465/)
* 2023-05-02 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/292297784/)
* 2023-05-10 | Amsterdam, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2023**](https://2023.rustnl.org/)
* 2023-05-19 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfchbzb/)

### North America

* 2023-04-29 | Durham, NC, US | [Triangle Rust](https://www.meetup.com/triangle-rust/)
    * [**Rust Social / Coffee Chat at Boxyard RTP**](https://www.meetup.com/triangle-rust/events/292833711/)
* 2023-05-03 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/293007744/)
* 2023-05-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Upcoming Event**](https://www.meetup.com/utah-rust/events/rrwbctyfchbpb/)
* 2023-05-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfchbvb/)

### Oceania

* 2023-04-27 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**April Meetup**](https://www.meetup.com/rust-brisbane/events/292965270/)
* 2023-05-03 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/292993051/)

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

> That said, I really like the language. It’s as if someone set out to design a programming language, and just picked all the right answers. Great ecosystem, flawless cross platform, built-in build tools, no “magic”, static binaries, performance-focused, built-in concurrency checks. Maybe these “correct” choices are just laser-targeted at my soul, but in my experience, once you leap over the initial hurdles, it all just works™️, without much fanfare.

– [John Austin on his blog]()

Thanks to [Ivan Tham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1408) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
