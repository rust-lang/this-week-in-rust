Title: This Week in Rust 623
Number: 623
Date: 2025-10-29
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

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Newsletters
* [The Embedded Rustacean Issue #57](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-57)

### Project/Tooling Updates
* [Introducing Apache Fory™](https://fory.apache.org/blog/fory_rust_versatile_serialization_framework/)
* [SeaORM 2.0: Entity First Workflow](https://www.sea-ql.org/blog/2025-10-30-sea-orm-2.0/)
* [Rari v0.3.0: Rust-based React framework with SSR - 12x faster, 10x higher throughput than Next.js](https://ryanskinner.com/posts/the-rari-ssr-breakthrough-12x-faster-10x-higher-throughput-than-nextjs)
* [Burn 0.19.0 Release: Quantization, Distributed Training, and LLVM Backend](https://burn.dev/blog/release-0.19.0/)
* [Yelken's Last Alpha Release and Intro to Yelken Cloud](https://blog.yelken.io/last-alpha-and-yelken-cloud/)
* [Capnproto 0.22 — async methods](https://dwrensha.github.io/capnproto-rust/2025/10/27/0.22-release.html)
* [Fyrox 1.0.0-rc.1](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0-rc-1/)
* [Boa release v0.21](https://boajs.dev/blog/2025/10/22/boa-release-21)
* [Typst: Typst 0.14: Now accessible](https://typst.app/blog/2025/typst-0.14/)
* [iroh-blobs 0.95](https://www.iroh.computer/blog/iroh-blobs-0-95-new-features)
* [This week in Heave (2025.10.24)](https://www.rustydonkey.dev/blog/2025.10.24_this_week_in_heave/)

### Observations/Thoughts
* [Explicit capture clauses](https://smallcultfollowing.com/babysteps/blog/2025/10/22/explicit-capture-clauses/)
* [Closure captures](https://andwass.github.io/rust/2025/10/23/closure-captures.html)
* [Recent Rust Changes](https://www.ncameron.org/blog/recent-rust-changes/)
* [How Signal uses Rust to secure the communications of millions of people](https://kerkour.com/signal-app-rust)
* [A hard rain's a-gonna fall: decoding JSON in Rust](https://bitfieldconsulting.com/posts/hard-rain-json-rust)
* [GSoC ‘25 Work Product: Parallel Macro Expansion](https://lorrens.me/2025/10/26/GSoC-Parallel-Macro-Expansion.html)
* [When O3 is 2x slower than O2](https://cat-solstice.github.io/test-pqueue/)
* [The (rust) Clippy Changelog Cat Contest, a brief retrospective](https://blog.goose.love/posts/history-of-clippy-changelog-cat/)
* [audio] [Netstack.FM — Episode 11 – Modern networking in Firefox with Max Inden](https://netstack.fm/#episode-11)
* [audio] [What's New in Rust 1.81 through 1.84](https://rustacean-station.org/episode/rust-1.81-1.82-1.83-1.84/)

### Rust Walkthroughs
* [Data Analysis in Rust](https://ericfecteau.ca/data/rust-data-analysis/index.html)
* [How to Avoid Fighting Rust Borrow Checker](https://qouteall.fun/qouteall-blog/2025/How%20to%20Avoid%20Fighting%20Rust%20Borrow%20Checker)
* [Rust Unit Testing: Mocks and flexible verification](https://jorgeortiz.dev/posts/rust_unit_testing_test_doubles_mock/)
* [Building a Coding Agent in Rust: Introduction](https://blog.0xshadow.dev/posts/coding-agent-in-rust/coding-agent-in-rust-introduction/)
* [Teddy Bear Trash Compactor](https://internet.place/content/teddy-bear-trash-compactor/)
* [Rust for JavaScript Engineers - Connect-4 Interactivity](https://www.afloat.boats/posts/rust-for-javascript-engineers-interactivity)
* [Cleanup your lifetime annotations in Rust with Rc and Arc](https://kerkour.com/rust-lifetimes-rc-arc)
* [Vibe Validation with Lean, ChatGPT-5, & Claude 4.5: Nine Rules for Proving (Rust) Algorithms Correct Without Knowing Formal Methods (Part 2)](https://medium.com/@carlmkadie/081e0f06886d)
* [video] [Rust Axum 0.8 Backend Engineering | Hello World](https://www.youtube.com/watch?v=Imb6vJkD0Vc)
* [video] [Building Coding Agent in Rust | Project Setup](https://www.youtube.com/watch?v=tQJTuYkZ4u8&t=1s)

### Research
* [Supporting `VIEW`s in Diesel](https://blog.weiznich.de/blog/diesel-infer-sql-nullablity/)

## Crate of the Week

This week's crate is [tower-resilience](https://github.com/joshrotenberg/tower-resilience), a library offering resilience features for tower.

Thanks to [Josh Rotenberg](https://users.rust-lang.org/t/crate-of-the-week/2704/1483) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing),
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
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Diesel - https://github.com/diesel-rs/diesel/issues/4840](https://github.com/diesel-rs/diesel/issues/4840)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

463 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-10-21..2025-10-28

#### Compiler
* [`-Znext-solver` instantiate predicate binder without recanonicalizing goal](https://github.com/rust-lang/rust/pull/146725)
* [`hir_analysis`: add missing sizedness bounds](https://github.com/rust-lang/rust/pull/142712)
* [add NonNull pattern types](https://github.com/rust-lang/rust/pull/142339)
* [add a fast path for lowering trivial consts](https://github.com/rust-lang/rust/pull/148040) (great speedup!)
* [do not lifetime-extend array/slice indices](https://github.com/rust-lang/rust/pull/147083)
* [deduce `captures(none)` for a return place and parameters](https://github.com/rust-lang/rust/pull/147890)
* [privacy: introduce some caching to type visiting in `DefIdVisitorSkeleton`](https://github.com/rust-lang/rust/pull/147486)

#### Library
* [add `FromIterator` impls for `ascii::Char`s to `String`s](https://github.com/rust-lang/rust/pull/141445)
* [add `String::replace_first` and `String::replace_last`](https://github.com/rust-lang/rust/pull/134316)
* [add new `inherit_handles` flag to CommandExt trait](https://github.com/rust-lang/rust/pull/115501)
* [const Cell methods](https://github.com/rust-lang/rust/pull/147788)
* [const `select_unpredictable`](https://github.com/rust-lang/rust/pull/145939)
* [create UTF-8 version of `OsStr`/`OsString`](https://github.com/rust-lang/rust/pull/147932)

#### Cargo
* [git: support shallow fetch for Git CLI backend](https://github.com/rust-lang/cargo/pull/16156)
* [make shell completion variables private](https://github.com/rust-lang/cargo/pull/16144)

#### Rustdoc
* [Check `doc(cfg())` even of private/hidden items](https://github.com/rust-lang/rust/pull/147991)
* [`--emit=depinfo` output to stdout via `-`](https://github.com/rust-lang/rust/pull/147762)

#### Clippy
* [`manual_let_else`: wrap expressions ending with `'}'`](https://github.com/rust-lang/rust-clippy/pull/15919)
* [`match_as_ref`: suggest `as_ref` when the reference needs to be cast](https://github.com/rust-lang/rust-clippy/pull/15934)
* [`needless_if`: don't expand macro invocations in the suggestion](https://github.com/rust-lang/rust-clippy/pull/15960)
* [`manual_option_as_slice`: improve diagnostics](https://github.com/rust-lang/rust-clippy/pull/15926)
* [`match_as_ref`: improve diagnostics](https://github.com/rust-lang/rust-clippy/pull/15928)
* [`unnecessary_{find,filter}_map`: make diagnostic spans more precise](https://github.com/rust-lang/rust-clippy/pull/15929)
* [`{option,result}_map_unit_fn`: fix and clean-up tests, make suggestions multiline](https://github.com/rust-lang/rust-clippy/pull/15871)
* [consider labels of inline asm as conditionally executed](https://github.com/rust-lang/rust-clippy/pull/15676)
* [fix `len_zero` false positive on unstable methods](https://github.com/rust-lang/rust-clippy/pull/15894)
* [lint precedence possible ambiguity between closure and method call](https://github.com/rust-lang/rust-clippy/pull/14421)

#### Rust-Analyzer
* [add an Extension Config API](https://github.com/rust-lang/rust-analyzer/pull/20837)
* [avoid calling `specializes()` query on crates that do not define `#![feature(specialization)]`](https://github.com/rust-lang/rust-analyzer/pull/20921)
* [add "Flip range expression" assist](https://github.com/rust-lang/rust-analyzer/pull/20705)
* [add "Remove `else` branches" assist](https://github.com/rust-lang/rust-analyzer/pull/19918)
* [provide an option to not show derives near the ADT for "Goto Implementations" or "Implementations" codelens](https://github.com/rust-lang/rust-analyzer/pull/20186)
* [when renaming `self` to other name, change callers method method call syntax to assoc fn syntax](https://github.com/rust-lang/rust-analyzer/pull/20369)
* [add `#[doc = include_str!("…")]` completion](https://github.com/rust-lang/rust-analyzer/pull/20755)
* [add shorthand record field completions](https://github.com/rust-lang/rust-analyzer/pull/20831)
* [add type keyword completions](https://github.com/rust-lang/rust-analyzer/pull/20571)
* [complete `else` in more expressions](https://github.com/rust-lang/rust-analyzer/pull/20658)
* [complete `let` before expression in `if`](https://github.com/rust-lang/rust-analyzer/pull/20912)
* [consider all matches for flyimport even when searched with a qualifier](https://github.com/rust-lang/rust-analyzer/pull/20919)
* [fix array inhabitedness check](https://github.com/rust-lang/rust-analyzer/pull/20905)
* [fix casts and use typed syntax tree API in `convert_to_guarded_return`](https://github.com/rust-lang/rust-analyzer/pull/20759)
* [handle `if`-`let` in `convert_to_guarded_return`](https://github.com/rust-lang/rust-analyzer/pull/20764)
* [handle shorthand field patterns in `destructure_tuple_binding`](https://github.com/rust-lang/rust-analyzer/pull/20712)
* [implement `Interner::impl_specializes()`](https://github.com/rust-lang/rust-analyzer/pull/20893)
* [improve field completion parentheses heuristic](https://github.com/rust-lang/rust-analyzer/pull/20889)
* [improve handling of missing names in `MethodCallExpr`](https://github.com/rust-lang/rust-analyzer/pull/20886)
* [improve handling of the `env!` macro](https://github.com/rust-lang/rust-analyzer/pull/20554)
* [improve incomplete statement heuristic](https://github.com/rust-lang/rust-analyzer/pull/20670)
* [lower async block/closures correctly](https://github.com/rust-lang/rust-analyzer/pull/20895)
* [offer `add_braces` on assignments](https://github.com/rust-lang/rust-analyzer/pull/20844)
* [offer `invert_if` on `else`](https://github.com/rust-lang/rust-analyzer/pull/20771)
* [place new module outside `impl` block in `extract_module`](https://github.com/rust-lang/rust-analyzer/pull/20589)
* [support `let`-chains in `replace_is_method_with_if_let_method`](https://github.com/rust-lang/rust-analyzer/pull/20913)
* [reduce `client_commands` allocations in proto conversion](https://github.com/rust-lang/rust-analyzer/pull/20922)
* [remove `hir-ty/src/next_solver/mapping.rs`](https://github.com/rust-lang/rust-analyzer/pull/20896)
* [semantic type for logical not](https://github.com/rust-lang/rust-analyzer/pull/20891)

### Rust Compiler Performance Triage

Mostly negative week, coming almost entirely from adding sizedness bounds in [#142712](https://github.com/rust-lang/rust/pull/142712). Other than that, we got a nice win for async code from state transform optimization in [#147493](https://github.com/rust-lang/rust/pull/147493) and quite a few smaller improvements from codegen optimization in [#147890](https://github.com/rust-lang/rust/pull/147890).

Triage done by **@panstromek**.
Revision range: [4068bafe..23fced0f](https://perf.rust-lang.org/?start=4068bafedd8ba724e332a5221c06a6fa531a30d2&end=23fced0fcc5e0ec260d25f04a8b78b269e5e90f0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.2%, 3.7%]    | 113   |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.1%, 1.7%]    | 75    |
| Improvements ✅ <br /> (primary)   | -0.4% | [-0.7%, -0.2%]  | 3     |
| Improvements ✅ <br /> (secondary) | -2.3% | [-20.8%, -0.1%] | 30    |
| All ❌✅ (primary)                 | 0.7%  | [-0.7%, 3.7%]   | 116   |


2 Regressions, 2 Improvements, 7 Mixed; 2 of them in rollups
42 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/0d28673aa9dacd2fe062baa1cd5336247f373d57/triage/2025/2025-10-27.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [FCW for repr(C) enums whose discriminant values do not fit into a c_int](https://github.com/rust-lang/rust/pull/147017)
* [Tracking Issue for maybe_uninit_write_slice](https://github.com/rust-lang/rust/issues/79995)
* [Add some conversion trait impls](https://github.com/rust-lang/rust/pull/145504)
* [Tracking Issue for `Duration::from_nanos_u128`](https://github.com/rust-lang/rust/issues/139201)
* [Tracking Issue for `core_slice_as_array`.](https://github.com/rust-lang/rust/issues/133508)
* [TryFrom\<integer\> for bool](https://github.com/rust-lang/rust/pull/147400)
* [Tracking Issue for slice::array_windows](https://github.com/rust-lang/rust/issues/75027)
* [Tracking issue for `#![feature(maybe_uninit_slice)]`](https://github.com/rust-lang/rust/issues/63569)
* [Tracking Issue for `lazy_get`](https://github.com/rust-lang/rust/issues/129333)
* [add Iterator::contains](https://github.com/rust-lang/rust/pull/141994)
* [Tracking Issue for inherent unchecked integer methods](https://github.com/rust-lang/rust/issues/85122)
* [Stabilize s390x `vector` target feature and `is_s390x_feature_detected!` macro](https://github.com/rust-lang/rust/pull/145656)
* [Update bundled musl to 1.2.5](https://github.com/rust-lang/rust/pull/142682)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Use v0 mangling by default on nightly](https://github.com/rust-lang/compiler-team/issues/938)
* [Use `annotate-snippets` as the default emitter on nightly](https://github.com/rust-lang/compiler-team/issues/937)
* [Creating a new maintainable debuginfo test suite](https://github.com/rust-lang/compiler-team/issues/936)
* [Switch to v0 mangling for symbols exceeding 64KB characters on targets generated PDB debug info](https://github.com/rust-lang/compiler-team/issues/934)
* [Promote `riscv64a23-unknown-linux-gnu` to Tier 2 without host tools](https://github.com/rust-lang/compiler-team/issues/932)
* [Omit suggestions when spans overlap](https://github.com/rust-lang/compiler-team/issues/929)
* [Replace `rustc_target::specTarget::arch` string with enum](https://github.com/rust-lang/compiler-team/issues/926)
* [Run main rust-analyzer tests in rust-lang/rust CI](https://github.com/rust-lang/compiler-team/issues/923)
* [Turn emscripten-wasm-eh unwinding ABI on by default](https://github.com/rust-lang/compiler-team/issues/920)
* [target tier 3 support for hexagon-unknown-qurt](https://github.com/rust-lang/compiler-team/issues/919)
* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Give integer literals a sign instead of relying on negation expressions](https://github.com/rust-lang/compiler-team/issues/835)
* [Also enable ICE file dumps on stable](https://github.com/rust-lang/compiler-team/issues/809)
* [New Tier-3 target proposal: `loongarch64-linux-android`](https://github.com/rust-lang/compiler-team/issues/806)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period)
* [Proposal: Require all project team members to have Zulip IDs](https://github.com/rust-lang/leadership-council/issues/228)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Adding a crates.io Security tab](https://github.com/rust-lang/rfcs/pull/3872)

## Upcoming Events

Rusty Events between 2025-10-29 - 2025-11-26 🦀

### Virtual
* 2025-10-29 | Virtual (Boulder, CO, US) | [Boulder Elixir](https://www.meetup.com/boulder-elixir/events/)
    * [**Integrating Elixir and Apache DataFusion with Rustler**](https://www.meetup.com/boulder-elixir/events/310996627/)
* 2025-10-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Hybrid event with Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311721465/)
* 2025-10-29 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/311669921/)
* 2025-10-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/t8yovmmm)
* 2025-11-01 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763868657)
* 2025-11-02 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109173/)
* 2025-11-04 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**Monthly WasmEdge Community Meeting, the runtime for LLM/AGI**](https://www.meetup.com/wasm-rust-meetup/events/311759399/)
* 2025-11-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-11-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/311574520/)
* 2025-11-05 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Mastering Error Handling in Rust: From Panics to thiserror & anyhow**](https://www.eventbrite.com/e/mastering-error-handling-in-rust-from-panics-to-thiserror-anyhow-tickets-1849030121869)
* 2025-11-06 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646021/)
* 2025-11-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109175/)
* 2025-11-10 || [BetterCode](https://www.bettercode.eu/)
    * $[**betterCode() Industrielle Anwendungen mit Rust**](https://dev.events/conferences/better-code-rust-i6inve6t)
* 2025-11-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361536/)
* 2025-11-11 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/311068632/)
* 2025-11-12 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yhe1xrhe)
* 2025-11-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310849154/)
* 2025-11-16 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109181/)
* 2025-11-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002262/)
* 2025-11-19 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/l2xukapz)
* 2025-11-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926564/)
* 2025-11-20 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046642/)
* 2025-11-20 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Tock, a Rust based Operating System Part #1**](https://www.meetup.com/charlottesville-rust-meetup/events/311705915/)
* 2025-11-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109183/)
* 2025-11-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361446/)
* 2025-11-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Data-Intensive Systems in Rust: Safety, Speed, Concurrency**](https://www.meetup.com/women-in-rust/events/311534469/)
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)

### Africa
* 2025-11-11 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**More on Rust types**](https://www.meetup.com/johannesburg-rust-meetup/events/311726345/)

### Asia
* 2025-11-15 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**November 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/november-2025-rustacean-meetup//)

### Europe
* 2025-10-29 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup October 2025**](https://www.meetup.com/rust-dortmund/events/311251545/)
* 2025-10-29 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London Code Dojo: Intro to Async Embedded Rust**](https://www.meetup.com/rust-london-user-group/events/311670686/)
* 2025-10-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 008**](https://www.meetup.com/rust-berlin/events/311752307/)
* 2025-10-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #62 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/311405044/)
* 2025-10-30 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague (October 2025)**](https://www.meetup.com/rust-prague/events/310967094/)
* 2025-11-01 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #19**](https://www.meetup.com/stockholm-rust/events/311582259/)
* 2025-11-02 - 2025-11-04 | Florence, IT | [Rustlab 2025](https://rustlab.it/)
    * $[**Rustlab 2025**](https://rustlab.it/)
* 2025-11-03 | Bern, CH | [Guild42](https://www.meetup.com/it-IT/guild42ch/)
    * [**Moving out of systems programming into Kubernetes: is it time to adopt Rust ?**](https://www.meetup.com/it-IT/guild42ch/events/307260207/)
* 2025-11-04 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Talk**](https://www.meetup.com/rust-manchester/events/310921632/)
* 2025-11-04 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/events/)
    * [**Optimizing matrix multiplication and building Python packages with Rust**](https://www.meetup.com/rust-trondheim/events/311595023/)
* 2025-11-05 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.club)
    * [**Rust Girona Hack & Learn 11 2025**](https://luma.com/xl8ob0tn)
* 2025-11-05 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/310601872/)
* 2025-11-05 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Halloween special.**](https://www.meetup.com/oxford-rust-meetup-group/events/311569796/)
* 2025-11-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #11**](https://www.meetup.com/rust-gdansk/events/310924266/)
* 2025-11-06 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/events/)
    * [**Season 2 Kickoff | at metalab 🦀**](https://www.meetup.com/rust-vienna/events/311679397/)
* 2025-11-11 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London x Zed Meetup**](https://www.meetup.com/rust-london-user-group/events/311737021/)
* 2025-11-12 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/311583721/)
* 2025-11-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944050/)
* 2025-11-13 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-11-13 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #80**](https://www.meetup.com/rust-paris/events/311461594/)
* 2025-11-18 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592257/)
* 2025-11-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Social Night**](https://www.meetup.com/rust-aarhus/events/311502123/)
* 2025-11-20 | Luzern, CH | [Rust Luzern]((https://www.meetup.com/rust-luzern/)
    * [**2025 Rust Talks Luzern #3: Crate Walkthroughs @ Noser Engineering AG**](https://www.meetup.com/rust-luzern/events/311410681/)

### North America
* 2025-10-29 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Scalable static analysis: confronting the halting problem**](https://www.meetup.com/rust-nyc/events/311541108/)
* 2025-10-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308675988/)
* 2025-10-30 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/311273832/)
* 2025-11-01 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Chinatown Rust Lunch, Nov 1**](https://www.meetup.com/bostonrust/events/311039492/)
* 2025-11-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**SIUE students on wasm 3D animations**](https://www.meetup.com/stl-rust/events/307251982/)
* 2025-11-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, Nov 8**](https://www.meetup.com/bostonrust/events/311039501/)
* 2025-11-13 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Ipmap: Building Desktop Apps with Tauri**](https://www.meetup.com/utah-rust/events/311613658/)
* 2025-11-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865806/)
* 2025-11-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**November, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351673/)
* 2025-11-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457310/)


### Oceania
* 2025-10-29 | Barton, AC, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**October Meetup**](https://www.meetup.com/rust-canberra/events/311234237/)
* 2025-11-11 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/311685331/)

### South America
* 2025-10-30 | Florianopolis, BR | [Rust Brasil](https://luma.com/calendar/cal-iOloL5ZqswCO5Mm)
    * [**Rust Floripa**](https://luma.com/lky7an18)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1nknaii/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Petition to add an `unwise` keyword in Rust

– [James Logan on hachyderm.io](https://hachyderm.io/@ponderingpothos/115403971956993021)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1724) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
