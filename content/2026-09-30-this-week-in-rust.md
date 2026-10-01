Title: This Week in Rust 671
Number: 671
Date: 2026-09-30
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

### Newsletters

* [Scientific Computing in Rust #22 (September 2026)](https://scientificcomputing.rs/monthly/2026-09)
* [The Embedded Rustacean Issue #81](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-81)

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

* [Can safe Rust ever beat Google's C Brotli?](https://mnwa.hashnode.dev/can-safe-rust-ever-beat-google-s-c-brotli)
* [Building a DMA based driver for the RP2350 I2C (safety not included)](https://micro-rust.github.io/posts/001-i2c-dma-handler/)
* [Advanced soft-bodies for games with the Rapier physics engine](https://dimforge.com/blog/2026/09/25/advanced-soft-bodies-for-games-in-the-rapier-physics-engine/)
* [Supporting native Rust in Workers with the new Emscripten target for wasm-bindgen](https://blog.cloudflare.com/rust-workers-emscripten-target/)
* [Rusty thoughts on "Parse, don't validate"](https://eli.thegreenplace.net/2026/rusty-thoughts-on-parse-dont-validate/)
* [The state of SIMD in Rust in 2026](https://shnatsel.github.io/state-of-simd-rust-2026/)
* [How do you stop being a Rust novice?](https://www.jochen.fyi/posts/how-do-you-stop-being-a-rust-novice)
* [We Have Named Arguments at Home](https://corrode.dev/blog/named-arguments-at-home/): a reply to the blog post *Arguing about arguments* mentioned in the last issue
* [Upstream Rust maintenance report (August-September 2026)](https://kobzol.github.io/rust/2026/09/30/stf-august-september-2026.html)
* [Rust in the kernel? What about Rust without the kernel!](https://kerkour.com/rust-kernel)
* [Compiling the kernel with gccrs](https://lwn.net/SubscriberLink/1095553/7f34252658f8b8d1/)
* [Listening to the radio with Rust](https://lwn.net/SubscriberLink/1095721/e1d863e5fd827753/)
* [Native support for Rust on the GPU](https://lwn.net/SubscriberLink/1095731/a5ecc9da2388b8ec/)

### Rust Walkthroughs

* [How to speed up the Rust compiler in September 2026](https://nnethercote.github.io/2026/09/30/how-to-speed-up-the-rust-compiler-in-september-2026.html)
* [Building Real-Time Notifications with SSE and Pub/Sub](https://oluseun.dev/blogs/real-time-notifications-sse-pubsub.html)
* [Green Threads from Scratch](https://dzania.github.io/green-threads-from-scratch/)
* [A Type Stronger than the Sum of its Components](https://www.schneems.com/2026/09/24/a-type-stronger-than-the-sum-of-its-components/)
* [Deser: Rethinking Rust Serialization](https://lucumr.pocoo.org/2026/9/29/deser/)
* [Dropping Swift from our Bevy iOS crates](https://rustunit.com/blog/2026/09-04-bevy-ios-crates-objc2/)
* [Pining for Arc Downcasting in Rust](https://wolfgirl.dev/blog/2026-09-29-pining-for-arc-downcasting-in-rust/)
* [Topcoat is pushing the boundary of server applications with Rust](https://tokio.rs/blog/2026-09-24-topcoat-server-applications)
* [Rust Reborrowing, Aliasing, and Mutable References](https://developerlife.com/2026/09/25/rust-reborrowing/)
* [A very condensed introduction of the basics of Rust](https://liw.fi/distilled-rust/)
* [video] [Making Our GPUI App Interactive with State and Events](https://youtu.be/bs8bpAZ10SM)
* [ES] [Domain–Flow–Effects (DFE): an architecture designed for Rust](https://codigolinea.com/domain-flow-effects-dfe-arquitectura-rust/)

### Miscellaneous

* [DE][Rust & Linux Community Event – November 20–21, 2026 @ TUXEDO, Augsburg – Help us choose the workshop topic](https://cryptpad.fr/form/#/2/form/view/ppm1DazKFfZxfLB8Fw6-7W0vBC1kFV9KgkHQWqY7UU0/)

## Crate of the Week

This week's crate is [ying-profiler](https://github.com/velvia/ying-profiler), a native Rust sampling memory profiler.

Thanks to [Evan Chan](https://users.rust-lang.org/t/crate-of-the-week/2704/1683) for the self-suggestion!

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
<!-- or if none - *No Calls for participation were submitted this week.* -->
* [unsynced - strace frontend fails on pwritev2 with offset -1 (current file offset)](https://github.com/zaydmulani09/unsynced/issues/1)
* [unsynced - Model hard links (link/linkat) instead of warning](https://github.com/zaydmulani09/unsynced/issues/2)
* [unsynced - Add an ext4 data=writeback persistence profile](https://github.com/zaydmulani09/unsynced/issues/3)
* [MemoryWhale - Cover friendly errors for incomplete `mw` arguments](https://github.com/wuisabel-gif/MemWhale/issues/253)
* [MemoryWhale - Lock down `mw --help` output](https://github.com/wuisabel-gif/MemWhale/issues/254)
* [dataprof - Remote Parquet refusal messages should say to download the file when the server ignores Range](https://github.com/AndreaBozzo/dataprof/issues/840)
* [dataprof - `ScoreBounds::dimension_scores` docs still list estimated key counts as unbounded](https://github.com/AndreaBozzo/dataprof/issues/827)
* [dataprof - The progress `finished` event under-counts rows when a row cap stops the incremental engine](https://github.com/AndreaBozzo/dataprof/issues/824)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

546 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-22..2026-09-29

#### Compiler
* [computing `crate_hash` from metadata encoding instead of HIR (implements #94878)](https://github.com/rust-lang/rust/pull/154724)
* [detect missing else in let statement](https://github.com/rust-lang/rust/pull/156949)
* [give noalias back to refs in closures](https://github.com/rust-lang/rust/pull/162361)
* [implement forced keywords (`k#`)](https://github.com/rust-lang/rust/pull/161775)
* [use SmallVec in LocalizedConstraintGraph](https://github.com/rust-lang/rust/pull/163190)

#### Library
* [add `Div` and `Mul` for `Complex<{float}>`](https://github.com/rust-lang/rust/pull/162832)
* [alloc: stabilise `Allocator`](https://github.com/rust-lang/rust/pull/156882)
* [additional `NonZero` conversions](https://github.com/rust-lang/rust/pull/129036)
* [allow elided (`'static`) lifetimes in `thread_local!`](https://github.com/rust-lang/rust/pull/159564)
* [implement `PartialEq<VecDeque<U>>` for `Vec<T>`, `&[T]`, `&mut [T]`, `[T; N]`, `&[T; N]` and `&mut [T; N]`](https://github.com/rust-lang/rust/pull/152972)
* [make dropping an empty BTreeMap free](https://github.com/rust-lang/rust/pull/161791)
* [stabilize SyncView](https://github.com/rust-lang/rust/pull/163366)
* [stabilize `Box::take`](https://github.com/rust-lang/rust/pull/160436)
* [stabilize `Result::into_{ok,err}`](https://github.com/rust-lang/rust/pull/161712)
* [stabilize `funnel_shifts` (including `const`)](https://github.com/rust-lang/rust/pull/161015)
* [stabilize `mem::conjure_zst`](https://github.com/rust-lang/rust/pull/161710)
* [stabilize `vec_try_remove`](https://github.com/rust-lang/rust/pull/163459)
* [use wrapping arithmetic in `from_str_radix`](https://github.com/rust-lang/rust/pull/163099)

#### Cargo
* [`builtin-deps`: add builtin dependencies manifest syntax](https://github.com/rust-lang/cargo/pull/17498)
* [`config`: add build.profile, install.profile](https://github.com/rust-lang/cargo/pull/17215)
* [`metadata`: mirror package features in `features_v2`](https://github.com/rust-lang/cargo/pull/17517)
* [`builtin-deps`: fix builtin dependencies manifest validation](https://github.com/rust-lang/cargo/pull/17499)
* [`diag`: Don't report unused normal deps when static libs are skipped](https://github.com/rust-lang/cargo/pull/17515)
* [`package`: preserve feature metadata in normalized manifests](https://github.com/rust-lang/cargo/pull/17509)

#### Rustdoc
* [correctly check that an item is not `doc(hidden)` with `--generate-link-to-definition`](https://github.com/rust-lang/rust/pull/163268)
* [fix intra doc link resolution when a doc comment is composed of both inner and outer doc comment](https://github.com/rust-lang/rust/pull/162862)
* [fix invalid jump to def link when `#[rustc_allow_incoherent_impl]` is involved](https://github.com/rust-lang/rust/pull/163133)
* [fix quadratic naming of duplicate sidebar links](https://github.com/rust-lang/rust/pull/162976)

#### Clippy
* [`while_let_loop`: detect the pattern when the loop has a label](https://github.com/rust-lang/rust-clippy/pull/17614)
* [add new `try_from_instead_of_from_str` lint](https://github.com/rust-lang/rust-clippy/pull/17030)
* [don't suggest `Box::leak` in `nonnull_unchecked_on_box_ptr`](https://github.com/rust-lang/rust-clippy/pull/17752)
* [fix `collapsible_match` consuming/mutation checking](https://github.com/rust-lang/rust-clippy/pull/16951)
* [fix `match_str_case` matching str inside or patterns](https://github.com/rust-lang/rust-clippy/pull/17759)
* [improve doc attr span tracking for proc-macro](https://github.com/rust-lang/rust-clippy/pull/17678)

#### Rust-Analyzer
* [don't fail extension activation when the server fails to start](https://github.com/rust-lang/rust-analyzer/pull/23386)
* [add `type_match` relevance for type-alias](https://github.com/rust-lang/rust-analyzer/pull/23406)
* [coercion safe fn to unsafe fn](https://github.com/rust-lang/rust-analyzer/pull/23416)
* [complete 'false' in cfg attribute](https://github.com/rust-lang/rust-analyzer/pull/23382)
* [complete attr value inside string without quotes](https://github.com/rust-lang/rust-analyzer/pull/23414)
* [const eval cast of single-variant `enum`](https://github.com/rust-lang/rust-analyzer/pull/23185)
* [deduplicate 'derive' and 'test' attribute macro](https://github.com/rust-lang/rust-analyzer/pull/23413)
* [do not type match unknown type](https://github.com/rust-lang/rust-analyzer/pull/23408)
* [don't clear semantic tokens cache on refresh](https://github.com/rust-lang/rust-analyzer/pull/23410)
* [hover show impl header when impl with trait](https://github.com/rust-lang/rust-analyzer/pull/23365)
* [panic in async closures with higher-ranked trait bounds](https://github.com/rust-lang/rust-analyzer/pull/23409)
* [return UB instead of panicking when reading the discriminant of an uninhabited `enum`](https://github.com/rust-lang/rust-analyzer/pull/23421)

### Rust Compiler Performance Triage

This week was fairly positive. We had no pure regressions, and most of the results came from a few architectural improvements with mixed or mostly positive impact. Some improvements also come from addressing previously triaged regression caused by missing no_alias annotation for references in closures.

The biggest improvement this week is in rustdoc, from tackling quadratic behaviour when generating sidebar links. This was reported by a user, but the effect didn't show up in our benchmarks, so we added a special stress test for it.

Triage done by **@panstromek**.
Revision range: [3670d253..c1070d69](https://perf.rust-lang.org/?start=3670d2532bdf51abbe0b8fea22284d7ca340ffe3&end=c1070d69382b8d2f2eb65119c738a77d9e324c9e&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.2%, 0.8%]    | 8     |
| Regressions ❌ <br /> (secondary)  | 1.4%  | [0.1%, 5.8%]    | 30    |
| Improvements ✅ <br /> (primary)   | -0.6% | [-1.7%, -0.2%]  | 192   |
| Improvements ✅ <br /> (secondary) | -1.5% | [-82.5%, -0.1%] | 101   |
| All ❌✅ (primary)                 | -0.6% | [-1.7%, 0.8%]   | 200   |


0 Regressions, 2 Improvements, 6 Mixed; 3 of them in rollups
26 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/7409c0adce96db29bbfa5030136401590f768577/triage/2026/2026-09-29.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Stop using dlltool for generating import libraries on MinGW](https://github.com/rust-lang/rust/pull/157712)
* [Stabilize `ptr::try_cast_aligned`](https://github.com/rust-lang/rust/pull/154170)
* [disposition: close] [1.99 beta crater regression: overflow evaluating the requirement](https://github.com/rust-lang/rust/issues/161916)
* [implement FCW for `rustc_allowed_through_unstable_modules` items](https://github.com/rust-lang/rust/pull/163161)
* [fix `VisibleForLeakCheck` in `RegionOutlives` fast path](https://github.com/rust-lang/rust/pull/163267)
* [Stabilize `debug_closure_helpers`](https://github.com/rust-lang/rust/pull/146099)
* [ Support type-relative assoc item paths in generic param defaults & const param types](https://github.com/rust-lang/rust/pull/161998)
* [FCW for `#[panic_handler]` on `unsafe fn`.](https://github.com/rust-lang/rust/pull/162974)
* [Stabilize `optimize` attribute](https://github.com/rust-lang/rust/pull/157273)
* [Allow unary operand types to be inferred later](https://github.com/rust-lang/rust/pull/159744)
* [Feat - `#[inline(always)] + #[target_feature(enable = "....")]` #2](https://github.com/rust-lang/rust/pull/162460)
* [Tracking Issue for `CStr::display`](https://github.com/rust-lang/rust/issues/139984)
* [Syntactically reject leading parenthesized precise capturing lists in bare trait object types (`(use<…>)+`)](https://github.com/rust-lang/rust/pull/162652)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [`f16b` type](https://github.com/rust-lang/rfcs/pull/3983)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(trim-paths): stabilize `profile.trim-paths`](https://github.com/rust-lang/cargo/pull/17488)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Create new Tier 3 Target for QTEE: `aarch64-unknown-qtee`](https://github.com/rust-lang/compiler-team/issues/1038)

*No Items entered Final Comment Period this week for
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Make trait methods callable in const contexts, take III](https://github.com/rust-lang/rfcs/pull/4008)

## Upcoming Events

Rusty Events between 2026-09-30 - 2026-10-28 🦀

### Virtual
* 2026-09-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Segmentation and Introduction to Paging**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316486941/)
* 2026-10-01 | Virtual | [Rust Foundation & JetBrains](https://rustfoundation.org/event/livestream-smarter-coding-agents-for-rust-with-symposium/)
    * [**Livestream: Smarter Coding Agents for Rust with Symposium**](https://info.jetbrains.com/rustrover-livestream-october01-2026.html#form)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yqxvguts)
* 2026-10-03 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #14**](https://www.meetup.com/bevy-game-development/events/316736369/)
* 2026-10-04 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316134009/)
* 2026-10-06 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773044/)
* 2026-10-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)
* 2026-10-08 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315907995/)
* 2026-10-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619617/)
* 2026-10-10 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/316381946/)
* 2026-10-10 | Hybrid (Kuala Lumpur, Malaysia) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup October 2026**](https://forms.gle/721DxqrPeHXY6omP9)
* 2026-10-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254772/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-18 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563013/)
* 2026-10-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcnbbc/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Reasoning with Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)

### Asia
* 2026-10-09 | Hybrid (Kuala Lumpur, MY) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup August 2026**](https://forms.gle/721DxqrPeHXY6omP9)

### Europe
* 2026-09-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-09-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/316661690/)
* 2026-10-01 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 – Edition 018**](https://www.meetup.com/rust-berlin/events/316763107/)
* 2026-10-01 | Oxford, GB | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Embedded Rust for Duffers**](https://www.meetup.com/oxford-rust-meetup-group/events/316708765/)
* 2026-10-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)
* 2026-10-08 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-08 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22nd bcnrust session**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-20 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)

### North America
* 2026-10-01 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Building a Minimal, Rootless Container in Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 2026-10-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Oct 3**](https://www.meetup.com/bostonrust/events/316378820/)
* 2026-10-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Lightning Talks N' Chill**](https://www.meetup.com/utah-rust/events/316708351/)
* 2026-10-08 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Zero Knowledge Proofs & GPUs in Rendering**](https://www.meetup.com/rust-nyc/events/316698929/)
* 2026-10-08 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust October Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316319730/)
* 2026-10-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, Oct 10**](https://www.meetup.com/bostonrust/events/316378823/)
* 2026-10-14 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust w/ Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)
* 2026-10-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783988/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Embedded Meetup**](https://luma.com/ur4pm34i)
* 2026-10-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316655403/)

### South America
* 2026-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**WebApp Ergonomics y Secretos Distribuidos.**](https://www.meetup.com/rust-argentina/events/316664266/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1wo5btb/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> The community is obnoxiously helpful. I asked a simple question on the Rust community Discord server. What is the best way to read a file in Rust? I expected a straightforward response. Instead, I got back a 2,000-word essay on the inner workings of IO, buffering, error handling, and ownership, plus links to four different blog posts and three different approaches depending on file size, and a working code example.
>
> ...
>
> The Rust community has weaponized education against me. I'm now a better engineer than I was yesterday against my will.

– [tris on youtube](https://youtu.be/B2gmKy3pHkw?si=4QRLux5X55fTx8c8&t=196)

Thanks to [MusicalNinjaDad](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1806) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1wuo2o9/this_week_in_rust_671/)</small>
