Title: This Week in Rust 661
Number: 661
Date: 2026-07-22
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
* [Announcing Rust 1.97.1](https://blog.rust-lang.org/2026/07/16/Rust-1.97.1/)

### Newsletters
* [The Embedded Rustacean Issue #76](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-76)

### Project/Tooling Updates
* [Announcing Topcoat: a framework for building full-stack reactive web apps with Rust](https://tokio.rs/blog/2026-07-22-announcing-topcoat)
* [Syn 3.0.0](https://github.com/dtolnay/syn/releases/tag/3.0.0)
* [What’s New in RustRover 2026.2](https://blog.jetbrains.com/rust/2026/07/22/whats-new-in-rustrover-2026-2/)
* [kobe 0.35.0: readiness gates and cert recycling](https://github.com/kunobi-ninja/kobe/releases/tag/v0.35.0)
* [Comhad v0.1.0: a ranger-style tui cyberduck replacement for browsing S3](https://github.com/Eoin-McMahon/comhad/releases/tag/v0.1.0)
* [Nova v0.2.1: computer-use MCP server](https://github.com/bigduu/Nova/releases/tag/v0.2.1)
* [winit now has comprehensive cross-platform drag-and-drop support, exposing most of the power of the underlying OS APIs](https://github.com/rust-windowing/winit/pull/4571)
* [crimson-crab v0.1.0 - a production-grade Rust SDK for the Claude API (streaming, tool use, prompt caching, batches)](https://github.com/singhpratech/crimson-crab/releases/tag/v0.1.0)
* [ferrovec: dependency-light HNSW vector search in Rust, compiled to WebAssembly for private in-browser semantic search](https://singhpratech.github.io/ferrovec/)
* [OrdoFP 0.1.0 released — a functional-programming toolbelt for Rust (HList, GAT type classes, optics, effects, monad transformers)](https://github.com/ordokr/ordofp/releases/tag/v0.1.0)
* [Freya 0.4](https://freyaui.dev/posts/0.4)
* [buildline: merging cargo and ninja's build profiling into one timeline](https://dev.to/nabsei/buildline-merging-cargo-and-ninjas-build-profiling-into-one-timeline-2373)
* [cochlea 0.3.0: melody read-back, MFCC timbre, a master limiter, and MIDI import for the deterministic agent-audio engine](https://richer-richard.github.io/cochlea/determinism.html#030-additions-2026-07-22)
* [flodl 0.6.0: multi-host heterogeneous DDP - mismatched GPUs across hosts beat the fastest card alone](https://flodl.dev/blog/then-the-cpu-died)
* [hwatu: a daemon-based WebKitGTK browser for tiling WMs with ~13ms window spawn](https://hongnoul.github.io/hwatu/)
* [kache 0.11.0: broader compiler coverage and libc-aware keys](https://github.com/kunobi-ninja/kache/releases/tag/v0.11.0)
* [`tracing-reload` - reload layer without panics](https://mladedav.github.io/blog/blog/tracing-reload/)
* [Introducing OpenTypeless: Voice Input That Actually Works](https://www.opentypeless.com/en/blog/introducing-talkmore)
* [Reading a Rust crate's capabilities out of its compiled symbols](https://dev.to/booyaka101/reading-a-rust-crates-capabilities-out-of-its-compiled-symbols-58pb)

### Observations/Thoughts
* [Battery packs: Let's talk about crates, baby](https://smallcultfollowing.com/babysteps/blog/2026/07/15/battery-packs/)
* [Capture Clauses as Effects](https://blog.yoshuawuyts.com/capture-clauses-as-effects)
* [Hardening Rust Code For Production](https://corrode.dev/blog/hardening-rust/)
* [Tokio Gives Progress, Not Ordering: Scheduling 1M Tasks](https://pranitha.dev/posts/tokio-gives-progress-not-ordering/)
* [Rust service hardening and production checklist](https://kerkour.com/rust-service-hardening-and-production-checklist)
* [audio] [The Rust Foundation with Rebecca Rumbul, Lori Lorusso, and David Wood, Rust Foundation leadership and board](https://corrode.dev/podcast/s06e08-rust-foundation/)
* [video] [Jon Gjengset: Open Source Maintenance 2026-07-18](https://www.youtube.com/watch?v=bAINppA0BSU)
* [video] [Rust Release Changelog - 1.97.0](https://www.youtube.com/watch?v=lUoQ3uGSQA0)
* [video] [Livestream: Rust in Ubuntu](https://www.youtube.com/live/Doqwh1b4QyA)

### Rust Walkthroughs
* [I hash-chained my agent's audit log. Then I found 13 breaks in it — all mine, all benign.](https://kriyanative.com/blog/13-chain-breaks/)
* [Two tricky bugs in a Rust daemon](https://dev.to/scripthpp/two-bugs-i-only-found-by-running-my-rust-sync-daemon-against-real-infrastructure-4278)
* [video] [Backend Concepts in Rust: Securely Managing App Secrets](https://www.youtube.com/watch?v=u91eX3J6lPU)
* [video] [Build with Naz - Ep 21: High Performance Flat 2D Arrays in Rust (SIMD, L1 cache)](https://www.youtube.com/watch?v=tIrSvJFRxAg)

## Crate of the Week

This week's crate is [xan](https://github.com/medialab/xan), a TUI toolkit to work with CSV files.

Thanks to [Simeon H.K. Fitch](https://users.rust-lang.org/t/crate-of-the-week/2704/1630) for the suggestion!

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
- *No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->
- *No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

576 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-14..2026-07-21

#### Compiler
* [account for async closures when pointing at lifetime in return type](https://github.com/rust-lang/rust/pull/159256)
* [comptime inherent impls](https://github.com/rust-lang/rust/pull/157824)
* [`dep_graph`: deduplicate task reads with an epoch-filtered index recorder](https://github.com/rust-lang/rust/pull/159115)
* [eagerly check for ambiguity in macro parsing](https://github.com/rust-lang/rust/pull/158976)
* [implement `#[diagnostic::opaque]` attribute to hide backtraces of macros](https://github.com/rust-lang/rust/pull/158608)
* [shrink `ast::Expr64`](https://github.com/rust-lang/rust/pull/158720)

#### Library
* [add explicit `Iterator::count` impl for `str::EncodeUtf16`](https://github.com/rust-lang/rust/pull/159467)
* [implement `bool::toggle`](https://github.com/rust-lang/rust/pull/159296)
* [implement `const_binary_search`](https://github.com/rust-lang/rust/pull/159528)
* [implement `Debug` helpers via `Cell`](https://github.com/rust-lang/rust/pull/159302)
* [implement `VecDeque::truncate_to_range`](https://github.com/rust-lang/rust/pull/156220)
* [make `pin!()` more foolproof](https://github.com/rust-lang/rust/pull/158061)
* [move `std::io::BufRead` to `alloc::io`](https://github.com/rust-lang/rust/pull/158546)
* [move `std::io::Read` to `alloc::io`](https://github.com/rust-lang/rust/pull/158544)
* [move `std::io::read_to_string` to `alloc::io`](https://github.com/rust-lang/rust/pull/158545)

#### Cargo
* [use PGO for Cargo](https://github.com/rust-lang/rust/pull/159149)
* [`timings`: only report units the job queue actually ran](https://github.com/rust-lang/cargo/pull/17238)
* [do not include proc-macro deps in rustc search path args](https://github.com/rust-lang/cargo/pull/17236)
* [include SBOM outputs in fingerprints](https://github.com/rust-lang/cargo/pull/17216)
* [lazily initialize git2 fetch transports](https://github.com/rust-lang/cargo/pull/17226)

#### Rustdoc
* [fix auto trait normalization env](https://github.com/rust-lang/rust/pull/159194)
* [use PGO for rustdoc](https://github.com/rust-lang/rust/pull/159091)

#### Clippy
* [add `block_scrutinee` lint](https://github.com/rust-lang/rust-clippy/pull/16855)
* [avoid invalid `ref_as_ptr` suggestions in const/static initializers](https://github.com/rust-lang/rust-clippy/pull/17415)
* [detect `== 0` on unsigned types as a `manual_clamp` lower bound](https://github.com/rust-lang/rust-clippy/pull/16800)
* [fix `if_not_else` linting on macro expanded conditions](https://github.com/rust-lang/rust-clippy/pull/17405)
* [fix `needless_collect` suggests a suggestion that cannot be typed](https://github.com/rust-lang/rust-clippy/pull/17383)
* [`non_zero_suggestions`: don't lint signed integer div/rem as NonZero](https://github.com/rust-lang/rust-clippy/pull/17385)
* [`manual_filter`: don't eat comments in the `and_then` suggestion](https://github.com/rust-lang/rust-clippy/pull/17377)
* [require the use of `as _` for indirectly used traits in clippy sources](https://github.com/rust-lang/rust-clippy/pull/17369)
* [rewrite `min_ident_chars`](https://github.com/rust-lang/rust-clippy/pull/17362)
* [use `#[must_use]` determination from the compiler](https://github.com/rust-lang/rust-clippy/pull/16633)

#### Rust-Analyzer
* [avoid index panic when flycheck list is empty](https://github.com/rust-lang/rust-analyzer/pull/22634)
* [add capture hints to coroutines](https://github.com/rust-lang/rust-analyzer/pull/22811)
* [add handler for E0572](https://github.com/rust-lang/rust-analyzer/pull/22813)
* [do not assume array destructuring assignments with rest pattern are constant-sized](https://github.com/rust-lang/rust-analyzer/pull/22483)
* [eagerly normalize `.await`'s `IntoFuture::Output`](https://github.com/rust-lang/rust-analyzer/pull/22852)
* [enable auto trait inference](https://github.com/rust-lang/rust-analyzer/pull/22791)
* [extract variable preserving whitespace from macro input](https://github.com/rust-lang/rust-analyzer/pull/22792)
* [fix coroutines not recording binding owners correctly](https://github.com/rust-lang/rust-analyzer/pull/22832)
* [fix crashes in assists due to `.unwrap()` calls in SyntaxFactory](https://github.com/rust-lang/rust-analyzer/pull/22759)
* [fix `hir` crate leaking bound variables from skipped binders](https://github.com/rust-lang/rust-analyzer/pull/22810)
* [fix `InferenceContext:identity_args` using the wrong DefId](https://github.com/rust-lang/rust-analyzer/pull/22855)
* [fix syntax bridge panic when spilting float](https://github.com/rust-lang/rust-analyzer/pull/22849)
* [handle `enum` variants in next-solver `generics`](https://github.com/rust-lang/rust-analyzer/pull/22857)
* [implement lowering of HRTB](https://github.com/rust-lang/rust-analyzer/pull/22818)
* [invalid `pattern_matching_variant` lowering due to recovery](https://github.com/rust-lang/rust-analyzer/pull/22789)
* [merge `WherePredicate::ForLifetimes` into `WherePredicate::TypeBound`](https://github.com/rust-lang/rust-analyzer/pull/22867)
* [only write anon const ty in parent's inference result if it doesn't have its own inference](https://github.com/rust-lang/rust-analyzer/pull/22804)
* [panic with a function item and a proc macro item having a duplicate name](https://github.com/rust-lang/rust-analyzer/pull/22822)
* [parser to error on macro type bound](https://github.com/rust-lang/rust-analyzer/pull/22827)
* [spawn proc-macro servers on requests clearing the client cache](https://github.com/rust-lang/rust-analyzer/pull/22865)
* [use quote! inside `ast::make::expr_call()`](https://github.com/rust-lang/rust-analyzer/pull/22782)
* [use `Result` for the lsp-server `Response` payload type](https://github.com/rust-lang/rust-analyzer/pull/22793)
* [record expressions in types in `ExprScope`](https://github.com/rust-lang/rust-analyzer/pull/22861)

### Rust Compiler Performance Triage

The two most notable changes this week were [#159115](https://github.com/rust-lang/rust/pull/159115),
which resulted in pretty nice instruction count wins for full incremental builds on several benchmarks,
and [#159091](https://github.com/rust-lang/rust/pull/159091), which enabled PGO for rustdoc, which
makes it ~3-4% faster across the board.

There were two large rollups with tiny performance regressions, which made it difficult to find
the offending PRs.

Triage done by **@Kobzol**.
Revision range: [5503df87..d527bc9b](https://perf.rust-lang.org/?start=5503df87342a73d0c29126a7e08dc9c1255c46ad&end=d527bc9bfa297ca7fd7f5ae93781eeec42073170&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 1.0%]   | 40    |
| Regressions ❌ <br /> (secondary)  | 0.7%  | [0.2%, 4.6%]   | 69    |
| Improvements ✅ <br /> (primary)   | -2.0% | [-6.2%, -0.2%] | 136   |
| Improvements ✅ <br /> (secondary) | -2.6% | [-8.4%, -0.2%] | 119   |
| All ❌✅ (primary)                 | -1.4% | [-6.2%, 1.0%]  | 176   |

2 Regressions, 3 Improvements, 6 Mixed; 4 of them in rollups
34 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/189822607d8d09acd85c234b2c245e817591ca67/triage/2026/2026-07-21.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Tracking Issue for `bool::toggle`](https://github.com/rust-lang/rust/issues/159298)
* [Tracking Issue for vec_try_remove](https://github.com/rust-lang/rust/issues/146954)
* [Avoid computing layout of enums with non-int discriminants](https://github.com/rust-lang/rust/pull/157562)
* [Tracking Issue for const_btree_len](https://github.com/rust-lang/rust/issues/71835)
* [Add `raw_borrows_via_references` lint](https://github.com/rust-lang/rust/pull/138230)
* [stabilize size_of_val_raw, align_of_val_raw, Layout::for_value_raw](https://github.com/rust-lang/rust/pull/157572)
* [rustc_passes: lint unused `#[path]` attributes on inline modules](https://github.com/rust-lang/rust/pull/158835)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Emit `note` when calling `rustc` without specifying an edition](https://github.com/rust-lang/compiler-team/issues/1019)
* [Let the OS handle stack growth](https://github.com/rust-lang/compiler-team/issues/1011)
* [Add `target_feature_available_at_call_site`](https://github.com/rust-lang/compiler-team/issues/1010)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Deallocate post-2026 funds from PM and compiler-ops](https://github.com/rust-lang/leadership-council/pull/314)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Do the bytes of a pointer have to stay in the same order?](https://github.com/rust-lang/unsafe-code-guidelines/issues/558)
  
*No Items entered Final Comment Period this week for
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
  [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Refactor the libs team](https://github.com/rust-lang/rfcs/pull/3984)

## Upcoming Events

Rusty Events between 2026-07-22 - 2026-08-19 🦀

### Virtual
* 2026-07-24 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/hd8mlw56)
* 2026-07-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254777/)
* 2026-07-28 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/315279653/)
* 2026-07-30 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/312045928/)
* 2026-07-31 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/uo5ek1f4)
* 2026-08-01 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-08-02 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095294/)
* 2026-08-04 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315213885/)
* 2026-08-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210367/)
* 2026-08-07 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ii2jrwva)
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

### Africa
* 2026-08-11 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust's extended standard library**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-07-25 | Mumbai, IN | [Rust Mumbai](https://luma.com/mumbai)
    * [**​Rust Mumbai — July Meetup 🦀**](https://luma.com/7ksabwbm/)
* 2026-07-26 | Pune, IN | [Rust Pune](https://www.meetup.com/rust-pune)
    * [**Rust Pune: July 2026**](https://www.meetup.com/rust-pune/events/315651505/)

### Europe
* 2026-07-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/315484101/)
* 2026-07-23 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**LDN Talks: July 2026 Antithesis Takeover**](https://www.meetup.com/rust-london-user-group/events/315612916/)
* 2026-07-23 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Rama modular service framework for Rust**](https://www.meetup.com/london-rust-project-group/events/315366453/)
* 2026-07-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #87**](https://www.meetup.com/rust-paris/events/315309633/)
* 2026-07-25 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #28**](https://www.meetup.com/stockholm-rust/events/315749994/)
* 2026-07-27 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #20: Julian Dickert - Supply chain security in Rust: Evaluating crates for production**](https://rust-augsburg.github.io/meetup/Meetup_20.html)
* 2026-07-29 | Poland, PL | [Rust Poland](https://www.meetup.com/rust-poland-meetup)
    * [**Rust Poland x Kraków #10**](https://www.meetup.com/rust-poland-meetup/events/315582674/)
* 2026-07-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #70**](https://www.meetup.com/copenhagen-rust-community/events/315767999/)
* 2026-07-30 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester July Code Night**](https://www.meetup.com/rust-manchester/events/315037685/)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night: Trust but verify the LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)

### North America
* 2026-07-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjckbdc/)
* 2026-07-22 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust in Distributed Systems with Flight Science!**](https://www.meetup.com/rust-los-angeles/events/315376271/)
* 2026-07-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Write A Custom Coding Agent and wasm_zero**](https://www.meetup.com/rust-nyc/events/315636854/)
* 2026-07-23 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315418155/)
* 2026-07-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, July 25**](https://www.meetup.com/bostonrust/events/315582650/)
* 2026-07-25 | Brooklyn, NY, US | [Flower](https://flowercomputer.com/)
    * [**BOG-A-THON 2**](https://partiful.com/e/Vq9fyDNCMSO7ia4ulK5b)
* 2026-07-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539329/)
* 2026-08-01 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Chinatown Rust Lunch, Aug 1**](https://www.meetup.com/bostonrust/events/315582653/)
* 2026-08-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Evening Boston Rust Meetup at Red Hat, Aug 4**](https://www.meetup.com/bostonrust/events/314660176/)
* 2026-08-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Shipping Temporal: How a Global Rust Ecosystem Built Chrome’s Newest Web API**](https://www.meetup.com/stl-rust/events/314701905/)
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

### Oceania
* 2026-07-23 | Perth, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group)
    * [**Rust Perth: July Meetup!**](https://www.meetup.com/perth-rust-meetup-group/events/315451138/)
* 2026-07-30 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne July 2026**](https://www.meetup.com/rust-melbourne/events/315039480/)

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

> We were planning on publishing a blog post announcing this at the same time as making the repo public, but ran out of private repo CI usage 😭.

– [Carl Lerche on r/rust](https://www.reddit.com/r/rust/comments/1uzknzl/tokiorstopcoat_a_batteriesincluded_framework_for/oy8k2nn/) about the launch of topcoat

Despite a lamentable lack of suggestions, llogiq is glad to have found this quote.

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1v41dgv/this_week_in_rust_661/)</small>
