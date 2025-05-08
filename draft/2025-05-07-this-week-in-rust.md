Title: This Week in Rust 598
Number: 598
Date: 2025-05-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

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
* [Announcing rustup 1.28.2](https://blog.rust-lang.org/2025/05/05/Rustup-1.28.2/)

### Foundation

### Newsletters

### Project/Tooling Updates
* [Announcing Malai - Share your dev server (and more) over P2P](https://malai.sh/announcing-malai/)
* [Streaming data analytics, Fluvio 0.17.2 release](https://www.fluvio.io/news/this-week-in-fluvio-0074)
* [Leptos v0.8.0](https://github.com/leptos-rs/leptos/releases/tag/v0.8.0)
* [This Month in Redox - April 2025](https://www.redox-os.org/news/this-month-250430/)

### Observations/Thoughts
* [Automatic interleaving of high-level concurrent operations](https://blog.yoshuawuyts.com/automatic-interleaving-of-high-level-concurrent-operations/)
* [Flattening Rust's Learning Curve](https://corrode.dev/blog/flattening-rusts-learning-curve/)
* [The Evolution of Rust](https://ranger-ross.github.io/blog/evolution-of-rust/)
* [std::mem is... interesting](https://blog.veeso.dev/blog/en/std-mem-is-interesting/)
* [What is my fuzzer doing?](https://tweedegolf.nl/en/blog/154/what-is-my-fuzzer-doing)
* [audio] [Svix with Tom Hacohen](https://corrode.dev/podcast/s04e02-svix/)

### Rust Walkthroughs
* [Authentication with Axum](https://mattrighetti.com/2025/05/03/authentication-with-axum)
* [Newtyped Indices are Proofs](https://eikopf.bearblog.dev/newtyped-indices-are-proofs/)
* [What is my fuzzer doing?](https://tweedegolf.nl/en/blog/154/what-is-my-fuzzer-doing)
* [A Rust API Inspired by Python, Powered by Serde](https://ohadravid.github.io/posts/2025-05-serde-reflect/)
* [How to create small and secure Docker images for Rust (FROM scratch)](https://kerkour.com/rust-docker-from-scratch)
* [video] [Rust + SQLite:  Complete Tutorial (Schema, CRUD, JSON & Async)](https://www.youtube.com/watch?v=VlyXm7bwq6k)

### Research
* [An Interactive Debugger for Rust Trait Errors](https://cel.cs.brown.edu/blog/an-interactive-debugger-for-rust-trait-errors/)
* [RustAssistant: Using LLMs to Fix Compilation Errors in Rust Code](https://www.microsoft.com/en-us/research/publication/rustassistant-using-llms-to-fix-compilation-errors-in-rust-code/)

### Miscellaneous
* [Memory-safe sudo to become the default in Ubuntu](https://trifectatech.org/blog/memory-safe-sudo-to-become-the-default-in-ubuntu/)
* [How To Get A Rust Job Part I: Companies Already Using Rust](https://filtra.io/rust/career-help/how-to-get-a-rust-job-I)
* [GOSIM Spotlight Finalists at RustWeek](https://rustweek.org/gosim-spotlight-speakers/)

## Crate of the Week

This week's crate is [structstruck](https://crates.io/crates/structstruck), a proc-macro crate for enabling nested struct/enum definitions.

Thanks to [Julius Michaelis](https://users.rust-lang.org/t/crate-of-the-week/2704/1433) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [Hyperswitch - Move connector-specific utility functions to respective connector modules](https://github.com/juspay/hyperswitch/issues/7926)
* [Hyperswitch - Refactor ACI connector to reuse utilities from `utils.rs`](https://github.com/juspay/hyperswitch/issues/7927)
* [Hyperswitch - Analyze and remove unused functions in `connector/utils.rs`](https://github.com/juspay/hyperswitch/issues/7928)
* [rama - add ffi/rama-rhai: support ability to use services and layers written in rhai](https://github.com/plabayo/rama/issues/533)
* [rama - support (TLS) peetprint in rama-net fingerprinting](https://github.com/plabayo/rama/issues/518)
* [rama - support akamai h2 passive fingerprint and expose in echo + fp services](https://github.com/plabayo/rama/issues/517)
* [rama - add into_stream to BodyExtractExt trait](https://github.com/plabayo/rama/issues/536)
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

447 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-04-29..2025-05-06

#### Compiler

* [handle paren in macro expand for let-init-else expr](https://github.com/rust-lang/rust/pull/134034)
* [implement or-patterns for pattern types](https://github.com/rust-lang/rust/pull/139909)
* [initial support for dynamically linked crates](https://github.com/rust-lang/rust/pull/134767)
* [mir-opt: execute MatchBranchSimplification after GVN](https://github.com/rust-lang/rust/pull/140115)
* [refactor `rustc_on_unimplemented's` filter parser](https://github.com/rust-lang/rust/pull/140307)
* [perf: optimize the codegen for `Span::from_expansion`](https://github.com/rust-lang/rust/pull/140485)
* [perf: delay checking of `#[rustc_no_implicit_autorefs]` in autoref lint](https://github.com/rust-lang/rust/pull/140406)
* [perf: simplify `LazyAttrTokenStream`](https://github.com/rust-lang/rust/pull/127516)
* [perf: use a closure instead of three chained iterators](https://github.com/rust-lang/rust/pull/140464)
* [transmutability: merge contiguous runs with a common destination](https://github.com/rust-lang/rust/pull/140509)
* [transmutability: uninit transition matches unit byte only](https://github.com/rust-lang/rust/pull/140380)

#### Library

* [avoid redundant WTF-8 checks in `PathBuf`](https://github.com/rust-lang/rust/pull/140159)
* [delegate to inner `vec::IntoIter` from `env::ArgsOs`](https://github.com/rust-lang/rust/pull/139847)
* [implement `Iterator::last` for `vec::IntoIter`](https://github.com/rust-lang/rust/pull/139773)
* [stabilize `ptr::swap_nonoverlapping` in const](https://github.com/rust-lang/rust/pull/137280)
* [stabilize `select_unpredictable`](https://github.com/rust-lang/rust/pull/140550)
* [streamline the `format` macro](https://github.com/rust-lang/rust/pull/140188)

#### Cargo

* [cargo add: suggest similarly named features](https://github.com/rust-lang/cargo/pull/15438)
* [in package-workspace, keep dev-dependencies if they have a version](https://github.com/rust-lang/cargo/pull/15470)

#### Rustdoc

* [fix doctest heuristic for main fn wrapping](https://github.com/rust-lang/rust/pull/140420)

#### Rustfmt

* [also allow bool literals as first item of let chain](https://github.com/rust-lang/rust/pull/140486)

#### Clippy

* [don't warn about unloaded crates](https://github.com/rust-lang/rust-clippy/pull/14733)
* [fix `collapsible_if` false positive on block stmt before expr](https://github.com/rust-lang/rust-clippy/pull/14730)
* [fix `manual_unwrap_or_default` false positive on ref binding](https://github.com/rust-lang/rust-clippy/pull/14731)
* [fix: `manual_slice_fill` false positive on `IndexMut` overload](https://github.com/rust-lang/rust-clippy/pull/14719)
* [fix: `unused_async` false positive on default impl](https://github.com/rust-lang/rust-clippy/pull/14720)
* [gate `collapsible_if let_chains` lints on edition 2024 and MSRV](https://github.com/rust-lang/rust-clippy/pull/14723)

#### Rust-Analyzer

* [add PGO support to install](https://github.com/rust-lang/rust-analyzer/pull/19685)
* [better handle parallelism in cache priming](https://github.com/rust-lang/rust-analyzer/pull/19721)
* [disable fixpoint for variance computation temporarily](https://github.com/rust-lang/rust-analyzer/pull/19739)
* [add an assist to unwrap a type with a generic arg](https://github.com/rust-lang/rust-analyzer/pull/19740)
* [correct assoc ty bound var starting index](https://github.com/rust-lang/rust-analyzer/pull/19732)
* [correct span info for `mir::Operand`](https://github.com/rust-lang/rust-analyzer/pull/19247)
* [don't panic on some weird code](https://github.com/rust-lang/rust-analyzer/pull/19738)
* [fix `move_bounds` assists not working for lifetimes](https://github.com/rust-lang/rust-analyzer/pull/19747)
* [fix incorrect handling of unresolved non-module imports in name resolution](https://github.com/rust-lang/rust-analyzer/pull/19742)
* [fix proc-macro API creating malformed negative literals](https://github.com/rust-lang/rust-analyzer/pull/19746)
* [implement mut to const ptr cast for method resolution](https://github.com/rust-lang/rust-analyzer/pull/19733)
* [improve parser recovery a bit](https://github.com/rust-lang/rust-analyzer/pull/19723)
* [negative nums in `concat!` expansion](https://github.com/rust-lang/rust-analyzer/pull/19434)
* [remove unnecessary token length check for macros in renaming](https://github.com/rust-lang/rust-analyzer/pull/19750)
* [improve the let code snippet](https://github.com/rust-lang/rust-analyzer/pull/19735)
* [render more lifetimes](https://github.com/rust-lang/rust-analyzer/pull/19581)
* [support environment variable `CARGO_MANIFEST_PATH`](https://github.com/rust-lang/rust-analyzer/pull/19751)

### Rust Compiler Performance Triage

A relatively noisy week due to addition of new benchmarks as part of our [2025
benchmark update], and a number of large regressions in a rollup landing late
in the week (and so not yet investigated).

[2025 benchmark update]: https://github.com/rust-lang/rustc-perf/issues/2024

Triage done by **@simulacrum**.
Revision range: [25cdf1f6..62c5f58f](https://perf.rust-lang.org/?start=25cdf1f67463c9365d8d83778c933ec7480e940b&end=62c5f58f57670ce65e7fec34f8c4ba00c27da2d9&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 6 Mixed; 3 of them in rollups
31 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-05-04.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Temporary lifetime extension through tuple struct and tuple variant constructors](https://github.com/rust-lang/rust/pull/140593)
* [Stabilize the avx512 target features](https://github.com/rust-lang/rust/pull/138940)
* [Make `missing_fragment_specifier` an unconditional error](https://github.com/rust-lang/rust/pull/128425)
* [Error on recursive opaque ty in HIR typeck](https://github.com/rust-lang/rust/pull/139419)
* [Add `std::io::Seek instance` for `std::io::Take`](https://github.com/rust-lang/rust/pull/138023)
* [remove intrinsics::drop_in_place](https://github.com/rust-lang/rust/pull/140151)
* [Stabilize `tcp_quickack`](https://github.com/rust-lang/rust/pull/129121)
* [Change the desugaring of `assert!` for better error output](https://github.com/rust-lang/rust/pull/122661)
* [Tracking Issue for `non_null_from_ref]`(https://github.com/rust-lang/rust/issues/130823)
* [Make well-formedness predicates no longer coinductive](https://github.com/rust-lang/rust/pull/140208)
* [Fix parameter order for `_by()` variants of `min` / `max`/ `minmax` in `std::cmp`](https://github.com/rust-lang/rust/pull/139357)
* [Finalize repeat expr inference behaviour with inferred repeat counts](https://github.com/rust-lang/rust/pull/139635)
* [Implement (part of) ACP 429: add `DerefMut` to `Lazy[Cell/Lock]`](https://github.com/rust-lang/rust/pull/129334)

#### Other Areas

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Stabilize doctest-xcompile](https://github.com/rust-lang/cargo/pull/15462)

#### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: map_or_default in Option and Result](https://github.com/rust-lang/rfcs/pull/3148)

*No Items entered Final Comment Period this week for
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: enable derive(From) for single-field structs](https://github.com/rust-lang/rfcs/pull/3809)
* [`#![register_{attribute,lint}_tool]`](https://github.com/rust-lang/rfcs/pull/3808)
* [RFC: Add an attribute for raising the alignment of various items](https://github.com/rust-lang/rfcs/pull/3806)

## Upcoming Events

Rusty Events between 2025-05-07 - 2025-06-04 🦀

### Virtual
* 2025-05-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031663)
* 2025-05-07 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #10**](https://www.meetup.com/bevy-game-development/events/307354690)
* 2025-05-08 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820300)
* 2025-05-08 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/2lmcm9iq)
* 2025-05-08 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/307635288)
* 2025-05-08 | Virtual (Zürich, CH) | [Rust Zürisee](https://www.meetup.com/rust-zurich/events/)
    * [**🦀 Celebrating 10 years of Rust 1.0 (co-event with berline.rs) 🦀**](https://www.meetup.com/rust-zurich/events/307696718)
* 2025-05-10 | Virtual | [Leptos Community](https://lu.ma/3b7solx0)
    * [**Leptos Meetup: 0.8 Release and Server Fn Websockets Demo**](https://www.youtube.com/watch?v=CTIeERU1hns)
* 2025-05-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307648682)
* 2025-05-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbpb)
* 2025-05-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305020415)
* 2025-05-15 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**🦀 Celebrating 10 years of Rust 1.0 🦀**](https://www.meetup.com/rust-berlin/events/307293317)
* 2025-05-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/eeqmobhz)
* 2025-05-18 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbxb)
* 2025-05-19 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Tauri: Cross-Platform desktop applications with Rust and web technologies**](https://www.meetup.com/rust-tlv/events/307178592)
* 2025-05-20 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Threading through lifetimes of borrowing - the Rust way**](https://www.meetup.com/women-in-rust/events/307522540)
* 2025-05-20 | Virtual (Tel Aviv, Israel) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Rust at Work a conversation with Ran Reichman Co-Founder & CEO of Flarion**](https://www.meetup.com/code-mavens/events/307635734/)
* 2025-05-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170826)
* 2025-05-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307184332)
* 2025-05-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820302)
* 2025-05-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/8zabmc3w)
* 2025-05-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307668643)
* 2025-05-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhchbhc)
* 2025-05-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361435)
* 2025-05-27 | Virtual (Tel Aviv, Israel) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversation with Eli Shalom & Igal Tabachnik of Eureka Labs**](https://www.meetup.com/code-mavens/events/307673680/)
* 2025-06-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbcb)
* 2025-06-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031665)

### Asia
* 2025-05-17 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/events/)
    * [**Rust Delhi Meetup #10**](https://www.meetup.com/rustdelhi/events/307657584)
* 2025-05-24 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2025-rustacean-meetup/)

### Europe
* 2025-05-07 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 05 2025**](https://lu.ma/k4nscy5q)
* 2025-05-07 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in May: FFI**](https://www.meetup.com/rustcologne/events/307548402)
* 2025-05-07 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/events/)
    * [**VII Lenguajes, VII Perspectivas, I Problema**](https://www.meetup.com/madrust/events/307030185)
* 2025-05-07 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541571)
* 2025-05-08 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #8**](https://www.meetup.com/rust-gdansk/events/307281434)
* 2025-05-08 | London, GB | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Adopting Rust (Hosted by Lloyds bank)**](https://www.meetup.com/london-rust-project-group/events/307085179)
* 2025-05-12 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Bowling at Rust Week**](https://www.meetup.com/rust-nederland/events/307676003)
* 2025-05-12 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Create your rusty steel Rust logo!**](https://www.meetup.com/rust-nederland/events/307679174)
* 2025-05-12 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Walking Tour around Utrecht - Monday (afternoon)**](https://www.meetup.com/rust-nederland/events/307661412)
* 2025-05-12 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Walking Tour around Utrecht - Monday**](https://www.meetup.com/rust-nederland/events/307521994)
* 2025-05-13 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**RustWeek 2025 announcement**](https://www.meetup.com/rust-nederland/events/305227330)
* 2025-05-14 | Reading, GB | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045447)
* 2025-05-15 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust 10-year anniversary @ Appear**](https://www.meetup.com/rust-oslo/events/307427014)
* 2025-05-16 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Rust Week Hackathon**](https://www.meetup.com/rust-nederland/events/307107584)
* 2025-05-16 | Utrecht, NL | [Rust NL Meetup Group](https://www.meetup.com/rust-nederland/)
    * [**RustWeek Hackathon**](https://www.meetup.com/rust-nederland/events/307107584/)
* 2025-05-17 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Walking Tour around Utrecht - Saturday**](https://www.meetup.com/rust-nederland/events/307522004)
* 2025-05-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/307289572)
* 2025-05-20 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741635)
* 2025-05-22 | Augsburg, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Rust meetup #13**](https://rust-augsburg.github.io/meetup/Meetup_13.html)
* 2025-05-22 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #3 @zentroom**](https://www.meetup.com/rust-bern/events/307559606)
* 2025-05-22 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/307565141)
* 2025-05-22 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/307653223)
* 2025-05-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #11 @ Letsboot Basel**](https://www.meetup.com/rust-basel/events/307567083)
* 2025-05-29 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 2025-06-04 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-04 | Oxford, GB | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)

### North America
* 2025-05-07 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/307557852)
* 2025-05-08 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Calculando con el compilador: Compiler time vs Run time. Introducción a uv**](https://www.meetup.com/rust-mx/events/307015601)
* 2025-05-08 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Apache DataFusion: A Fast, Extensible, Modular Analytic Query Engine in Rust**](https://www.meetup.com/pdxrust/events/307288436)
* 2025-05-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Porter Square Rust Lunch, May 11**](https://www.meetup.com/bostonrust/events/306845728)
* 2025-05-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Multi-Platform App in Rust @ Warp.dev && Verifying Rust's Stdlib @ CMU**](https://www.meetup.com/rust-nyc/events/307675512)
* 2025-05-15 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Using Rust For Web Series 2 : Why you, Yes You. Should use Hyperscript!**](https://www.meetup.com/music-city-rust-developers/events/304333101)
* 2025-05-15 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**May, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307337307)
* 2025-05-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhchblc)
* 2025-05-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)

### South America
* 2025-05-28 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/events/)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 2025-05-31 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)

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

> Well, the answer is basically yes. Our firmware is all Rust. Every component of our autonomy stack is Rust. Our app is 50% in Rust. And, our visualization tools are in Rust. Our production tools are in rust. The production QC software, which we ship to China, is in rust. Our internal websites are in rust. It's rust all over. We’ve drank the Rust Kool-Aid. In fact, there is no Python installed on the robots. This is not to dis Python at all, but it’s just simply not there.
>
> We use Python for neural network training. But Python is boxed to that. Everything else is Rust. And, the advantage of using Rust exponentially builds up.

– [Vivek Bagaria on filtra.io](https://filtra.io/rust/interviews/matic-apr-25)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1683) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
