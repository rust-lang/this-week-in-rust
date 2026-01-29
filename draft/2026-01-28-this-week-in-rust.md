Title: This Week in Rust 636
Number: 636
Date: 2026-01-28
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
* [crates.io: development update | Rust Blog](https://blog.rust-lang.org/2026/01/21/crates-io-development-update/)
* [Announcing Rust 1.93.0 | Rust Blog](https://blog.rust-lang.org/2026/01/22/Rust-1.93.0/)
### Foundation

### Newsletters

### Project/Tooling Updates
* [Vetis: A very tiny, very fast HTTP server in Rust](https://dev.to/rogrio_arajo_55dae16f0d/vetis-a-very-tiny-very-fast-http-server-in-rust-1ldn)
* [cai 0.13: User friendly CLI tool for AI tasks](https://github.com/ad-si/cai/releases/tag/v0.13.0)
* [Nio v0.1.0: Embracing Thread-Per-Core Architecture](https://nurmohammed840.github.io/posts/embracing-thread-per-core-architecture/)
* [🦀 Compile-Time Reflection Is Finally Here](https://weeklyrust.substack.com/p/compile-time-reflection-is-finally)

* [r3bl_tui v0.7.7: modern async TUI lib — readline, md editor, flexbox, SSH-optimized rendering](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.7.7-tui)
* [r3bl-cmdr v0.0.25: TUI productivity apps - giti (git helper) and edi (beautiful md editor)](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.0.25-cmdr)
* [r3bl-build-infra v0.0.1: cargo-rustdoc-fmt — prettier md tables and ref-style links](https://github.com/r3bl-org/r3bl-open-core/releases/tag/v0.0.1-build-infra)

### Observations/Thoughts
* [Deep dive into Turso, the "SQLite rewrite in Rust"](https://kerkour.com/turso-sqlite)
* [Rust at Scale: An Added Layer of Security for WhatsApp](https://engineering.fb.com/2026/01/27/security/rust-at-scale-security-whatsapp/)
* [Rust vs JavaScript & TypeScript: performance, WebAssembly, and developer experience](https://blog.jetbrains.com/rust/2026/01/27/rust-vs-javascript-typescript/)
* [Atomic variables are not only about atomicity](https://sander.saares.eu/2026/01/25/atomic-variables-are-not-only-about-atomicity/)
* [`if let guard` stabilizing path](https://kivooeo.github.io/blog/if-let-guard/)
* [audio] [Netstack.FM episode 24 — WebAssembly and Rust in Practice, A Conversation with Alex Crichton](https://netstack.fm/#episode-24)
* [video] [AI should write rust and only rust ;)](https://www.youtube.com/watch?v=2lhr-QDWv-k)
* [What's New in Rust 1.88 through 1.90 :: Rustacean Station](https://rustacean-station.org/episode/rust-1.88-1.89-1.90/)
* [What's New in Rust 1.85 through 1.87 :: Rustacean Station](https://rustacean-station.org/episode/rust-1.85-1.86-1.87/)
  
### Rust Walkthroughs
* [The complete guide to publishing your first Rust crate to crates.io](https://dev.to/ajitkumar/the-complete-guide-to-publishing-your-first-rust-crate-to-cratesio-14pg)
* [Designing Error Types in Rust Applications](https://home.expurple.me/posts/designing-error-types-in-rust-applications/)
* [series] [Part 4: Training Infrastructure, Building an LLM from Scratch in Rust](https://www.tag1.com/how-to/part4-training-infrastructure-building-an-llm-from-scratch/)
* [Using Oracle db26ai from Rust with the oracle crate (2)](https://jorgeortiz.dev/posts/rust_use_oracle_db26ai_with_oracle_crate_2/)
* [series] [The Impatient Programmer's Guide to Bevy and Rust: Chapter 6 - Let There Be Particles](https://aibodh.com/posts/bevy-rust-game-development-chapter-6/)
* [Building a 24MB Offline AI with Rust + Burn](https://snaetwarre.github.io/My-Portofolio/blog/intelligent-disease-detection.html)
* [The Hidden Bottleneck: Blocking in Async Rust](https://cong-or.xyz/blocking-async-rust.html)
* [Replacing Protobuf with Rust to go 5 times faster](https://pgdog.dev/blog/replace-protobuf-with-rust)
  
### Research

### Miscellaneous

* [I packaged my Rust CLI to too many places, here's what I learned](https://ivaniscoding.github.io/posts/rustpackaging1/)

## Crate of the Week

This week's crate is [dynamodb-crud](https://github.com/dariocurr/dynamodb-crud), a type-safe API for working with DynamoDB tables.

Thanks to [dario curreri](https://users.rust-lang.org/t/crate-of-the-week/2704/1524) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

### [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing)

* [Tracking Issue for timing report SVG render backend](https://github.com/rust-lang/cargo/issues/16440)

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
[Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Diesel - Port diesel-cli argument parsing from clap builders to clap derives](https://github.com/diesel-rs/diesel/issues/4955)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP closes 2026-02-16 | Montreal, Quebec, Canada | 2026-09-08 - 2026-09-11

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

479 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-01-20..2026-01-27

#### Compiler
* [`const` blocks as a `mod` item](https://github.com/rust-lang/rust/pull/149174)
* [improve error message for `assert!()` macro in functions returning bool](https://github.com/rust-lang/rust/pull/151457)
* [inline constant localized typeck constraint computation](https://github.com/rust-lang/rust/pull/149639)

#### Library
* [`ptr_aligment_type`: add more APIs](https://github.com/rust-lang/rust/pull/148764)
* [add `simd_splat` intrinsic](https://github.com/rust-lang/rust/pull/151346)
* [avoid pulling in unicode when calling `io::Error::kind`](https://github.com/rust-lang/rust/pull/151418)
* [avoid tearing `dbg!` prints](https://github.com/rust-lang/rust/pull/149869)
* [constify boolean methods](https://github.com/rust-lang/rust/pull/151489)
* [ensure that the deadline has passed in `sleep_until`](https://github.com/rust-lang/rust/pull/151494)
* [fix `is_ascii` performance regression on AVX-512 CPUs when compiling with -C target-cpu=native](https://github.com/rust-lang/rust/pull/151259)
* [improve `is_ascii` performance on `x86_64` with explicit SSE2 intrinsics](https://github.com/rust-lang/rust/pull/151611)
* [make `simd_insert_dyn` and `simd_extract_dyn` const](https://github.com/rust-lang/rust/pull/151453)
* [optimize `vec.extend(slice.to_vec())`, take 2](https://github.com/rust-lang/rust/pull/151337)
* [use `ByteStr`'s `Display` for `OsStr`](https://github.com/rust-lang/rust/pull/151010)

#### Cargo
* [add -Z json-target-spec](https://github.com/rust-lang/cargo/pull/16557)
* [don't check the specific build-std output](https://github.com/rust-lang/cargo/pull/16551)
* [fix build-std lto test to run on other platforms](https://github.com/rust-lang/cargo/pull/16550)
* [fix: show `implicit_minimum_version_req` emitted source once per package](https://github.com/rust-lang/cargo/pull/16535)
* [increase `cache_lock` test timeout](https://github.com/rust-lang/cargo/pull/16545)
* [lint: Add `redundant_readme` lint](https://github.com/rust-lang/cargo/pull/16552)
* [lints: Add `non_*_case_features`](https://github.com/rust-lang/cargo/pull/16560)
* [lints: Add `non_kebab_case_bin` lint](https://github.com/rust-lang/cargo/pull/16524)
* [lints: Add mutually exclusive `non_{kebab,snake}_case_packages`](https://github.com/rust-lang/cargo/pull/16554)
* [lints: Pluralize `non_kebab_case_bins`](https://github.com/rust-lang/cargo/pull/16553)
* [rm: Suggest table flags when none are specified](https://github.com/rust-lang/cargo/pull/16533)

#### Rustdoc
* [add "Skip to main content" link for keyboard navigation in rustdoc](https://github.com/rust-lang/rust/pull/151482)
* [Make popover menus content scrollable on mobile devices](https://github.com/rust-lang/rust/pull/151216)

#### Clippy
* [also ignore cases with comments in `let_and_return`](https://github.com/rust-lang/rust-clippy/pull/16461)
* [fix `manual_dangling_ptr` false positive when pointee type is not `Sized`](https://github.com/rust-lang/rust-clippy/pull/16469)
* [fix `test_attr_in_doctest` false positive on `test_harness`](https://github.com/rust-lang/rust-clippy/pull/16454)
* [make `manual_is_variant_and` to cover manual `is_none_or`](https://github.com/rust-lang/rust-clippy/pull/16424)
* [`manual_let_else`: add trailing comma to `struct` patterns ending with `..`](https://github.com/rust-lang/rust-clippy/pull/16442)
* [rhs of short-circuit expression doesn't always run](https://github.com/rust-lang/rust-clippy/pull/16463)

#### Rust-Analyzer
* [`hir-ty`: add method `references_only_ty_error` to detect type errors](https://github.com/rust-lang/rust-analyzer/pull/21497)
* [add semicolon for `toggle_macro_delimiter`](https://github.com/rust-lang/rust-analyzer/pull/21522)
* [correct ungrammar path in patch](https://github.com/rust-lang/rust-analyzer/pull/21523)
* [`default_field_values`](https://github.com/rust-lang/rust-analyzer/pull/21408)
* [do not mix the order of builtin/regular derives in "Expand macro recursively"](https://github.com/rust-lang/rust-analyzer/pull/21490)
* [don't offer `apply_demorgan` on `if let`](https://github.com/rust-lang/rust-analyzer/pull/21499)
* [fix not complete 'else' before tuple](https://github.com/rust-lang/rust-analyzer/pull/21495)
* [fix incorrect continue for `convert_range_for_to_while`](https://github.com/rust-lang/rust-analyzer/pull/21514)

### Rust Compiler Performance Triage

This week saw a very nice win from doing overall less work in the compiler (https://github.com/rust-lang/rust/pull/151382). There were a few regressions, but only in artificial stress tests,
we are keeping an eye on them.

Triage done by **@kobzol**.
Revision range: [3d087e60..ebf13cca](https://perf.rust-lang.org/?start=3d087e6044bddc65723bf42c76fe4cc33a0076b0&end=ebf13cca58b551b83133d4895e123f7d1e795111&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.2%, 1.8%]    | 9     |
| Regressions ❌ <br /> (secondary)  | 3.1%  | [0.1%, 19.9%]   | 47    |
| Improvements ✅ <br /> (primary)   | -1.0% | [-3.1%, -0.2%]  | 195   |
| Improvements ✅ <br /> (secondary) | -1.4% | [-10.1%, -0.1%] | 157   |
| All ❌✅ (primary)                 | -1.0% | [-3.1%, 1.8%]   | 204   |

2 Regressions, 2 Improvements, 6 Mixed; 6 of them in rollups
42 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/480fd97adc2ee4f7bbecea3e62e32503ebcc27d7/triage/2026/2026-01-27.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:
* [Trait method impl restrictions (`final` methods)](https://github.com/rust-lang/rfcs/pull/3678)
* [RFC: `#[export_visibility = ...]` attribute](https://github.com/rust-lang/rfcs/pull/3834)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Promote tier 3 riscv32 ESP-IDF targets to tier 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)
* [Give integer literals a sign instead of relying on negation expressions](https://github.com/rust-lang/compiler-team/issues/835)
* [Also enable ICE file dumps on stable](https://github.com/rust-lang/compiler-team/issues/809)
* [New Tier-3 target proposal: loongarch64-linux-android](https://github.com/rust-lang/compiler-team/issues/806)

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Stabilize `core::hint::cold_path`](https://github.com/rust-lang/rust/pull/151576)
* [Tracking Issue for minimal const `ControlFlow` methods (`min_const_control_flow`)](https://github.com/rust-lang/rust/issues/148738)
* [Tracking Issue for `new_range_api` (part of RFC 3550)](https://github.com/rust-lang/rust/issues/125687)
* [Stabilize `assert_matches`](https://github.com/rust-lang/rust/pull/137487)
* [resolve: Report more visibility-related early resolution ambiguities for imports](https://github.com/rust-lang/rust/pull/149596)
* [Add FCW for derive helper attributes that will conflict with built-in attributes](https://github.com/rust-lang/rust/pull/151152)
* [Constify `fmt::from_fn`](https://github.com/rust-lang/rust/pull/150300)
* [Feature-gate `mut ref` patterns in struct pattern field shorthand](https://github.com/rust-lang/rust/pull/151102)
* [Tracking Issue for raw-pointer-to-reference conversion methods](https://github.com/rust-lang/rust/issues/122034)
* [implement PartialEq\<Vec\<U\>\> for \[T; N\] and &\[T; N\]](https://github.com/rust-lang/rust/pull/149045)
* [thread::scope: document how join interacts with TLS destructors](https://github.com/rust-lang/rust/pull/149482)

#### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [docs(report): enhance man pages for `cargo report *`](https://github.com/rust-lang/cargo/pull/16430)

#### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Guarantee repr(C) union field offset](https://github.com/rust-lang/reference/pull/2128)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Natural Method Disambiguation](https://github.com/rust-lang/rfcs/pull/3913)
* [Add `derive(Deref)` RFC](https://github.com/rust-lang/rfcs/pull/3911)
* [Abi Descriptors](https://github.com/rust-lang/rfcs/pull/3910)
* [Cargo mTLS registry authentication](https://github.com/rust-lang/rfcs/pull/3907)
* [Let `Option` derive `#[must_use]`](https://github.com/rust-lang/rfcs/pull/3906)
* [Version-typed cfgs](https://github.com/rust-lang/rfcs/pull/3905)

## Upcoming Events

Rusty Events between 2026-01-28 - 2026-02-25 🦀

### Virtual
* 2026-01-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Stack Safety**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-21 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/e2ea7hxo)
* 2026-01-26 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Rust code reading and open source contribution (UTC 18:00; English)**](https://www.meetup.com/code-mavens/events/312782592/)
* 2026-01-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254790/)
* 2026-01-27 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & learn: Error Handling in Rust**](https://www.meetup.com/women-in-rust/events/312799344/)
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/9h9n0dau)
* 2026-01-28 | Virtual (Lima-Perú, PE) | [Rust Perú](https://rust.pe)
    * [**Meetup: Rust from Hardware to Web (Embedded + Backend from Scratch)**](https://calendar.app.google/jc9DziLWVTUn1qNVA)    
* 2026-01-29 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #12**](https://www.meetup.com/bevy-game-development/events/312681343/)
* 2026-01-29 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455921/)
* 2026-01-29 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #2 - Rusty processes, memory limits, and basic capsules**](https://www.meetup.com/charlottesville-rust-meetup/events/312326112/)
* 2026-02-04 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting started with Rust Part 1: Common Programming Concepts**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312946936/)
* 2026-02-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312187422/)
* 2026-02-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-02-09 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Rust code reading and open source contribution (UTC 18:00; English)**](https://www.meetup.com/code-mavens/events/312985189/)
* 2026-02-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254789/)
* 2026-02-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/312799368/)
* 2026-02-11 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting Started with Rust Part 2: Ownership and Structs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312947249/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/5bu9kas1)
* 2026-02-12 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455922/)
* 2026-02-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312385179/)
* 2026-02-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/312951859/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ir8s81ec)

### Asia
* 2026-02-05 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/312799833/)
* 2026-02-11 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup February 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)

### Europe
* 2026-01-21 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/312749221/)
* 2026-01-22 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 010**](https://www.meetup.com/rust-berlin/events/312962219/)
* 2026-01-26 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #17: Emily Coaca - Entwicklung des Kernels Update für TockOS**](https://rust-augsburg.github.io/meetup/Meetup_17.html)
* 2026-01-27 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #4 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312851079)
* 2026-01-27 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester January Code Night**](https://www.meetup.com/rust-manchester/events/312848708/)
* 2026-01-28 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - January 2026**](https://www.meetup.com/rust-dortmund/events/312485262/)
* 2026-01-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague (January 2026)**](https://www.meetup.com/rust-prague/events/312895761/)
* 2026-01-28 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - WASM & Elegant CLI**](https://www.meetup.com/rust-community-toulouse/events/312782796/)
* 2026-01-29 | Ostrava, CZ | [MeetUpdate Ostrava](https://www.meetup.com/meetupdate-ostrava)
    * [**MeetUpdate Ostrava #28: Rust**](https://www.meetup.com/meetupdate-ostrava/events/312747904/)
* 2026-01-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #23**](https://www.meetup.com/stockholm-rust/events/312919934/)
* 2026-02-04 | Darmstadt, HE, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a newsletter subscription service with axum**](https://www.meetup.com/rust-rhein-main/events/312798996/)
* 2026-02-04 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 1**](https://www.meetup.com/rust-munich/events/312844145/)
* 2026-02-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Beyond the Code: Designing Services That Stand the Test of Time**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)
* 2026-02-05 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/312679714/)
* 2026-02-11 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-14 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 02 2026**](https://luma.com/e0uay6q5)
* 2026-02-18 - 2026-02-19 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)

### North America
* 2026-01-21 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312185794/)
* 2026-01-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Stack Safety**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Evening Boston Rust Meetup with Jiff, January 22**](https://www.meetup.com/bostonrust/events/312598080/)
* 2026-01-22 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312692728/)
* 2026-01-24 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Davis Square Rust Lunch, Jan 24**](https://www.meetup.com/bostonrust/events/312483715/)
* 2026-01-28 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust Los Angeles: Building Git-LFS Replacements in Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)
* 2026-01-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308676002/)
* 2026-01-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Rust Programming 101**](https://www.meetup.com/music-city-rust-developers/events/312038621/)
* 2026-01-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Northeastern Rust Lunch, Jan 31**](https://www.meetup.com/bostonrust/events/312483767/)
* 2026-02-03 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Bevy Rendering & Build Times at Amazon**](https://www.meetup.com/rust-nyc/events/312871242/)
* 2026-02-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Rendering the Mandelbrot set in Rust**](https://www.meetup.com/stl-rust/events/312614666/)
* 2026-02-07 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Allston Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Full Stack Web Development in Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Telling a programmer there's already a library to do X is like telling a songwriter there's already a song about love.

– [Pete Cordell cited by @blonk on rust-users](https://users.rust-lang.org/t/i-am-looking-for-feedback-for-my-own-game-engine-which-have-written-in-rust/137509/4)

Thanks to [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1752) for the suggestion!

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
