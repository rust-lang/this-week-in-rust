Title: This Week in Rust 662
Number: 662
Date: 2026-07-29
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

* [Scientific Computing in Rust #20 (July 2026)](https://scientificcomputing.rs/monthly/2026-07)

### Project/Tooling Updates

* [afrim 0.7.0: a generic input method framework](https://github.com/fodydev/afrim/releases/tag/v0.7.0)
* [Sharing Rust build work across Cargo worktrees with cargo-reapi](https://github.com/TamedTornado/cargo-reapi/blob/main/docs/introducing-cargo-reapi.md)
* [exiftool-rs 0.7.0: localizing ExifTool's PrintConv values, not just its labels](https://github.com/Le-Syl21/exiftool-rs/releases/tag/v0.7.0)
* [Announcing SeaORM 2.0](https://www.sea-ql.org/blog/2026-07-27-sea-orm-2.0/)
* [kobe 0.37.0: easier to deploy and install](https://github.com/kunobi-ninja/kobe/releases/tag/v0.37.0)
* [kache 0.12.0: pluggable remotes, smarter GC, sharper diagnostics](https://github.com/kunobi-ninja/kache/releases/tag/v0.12.0)
* [Progress toward compiling Linux with gccrs](https://lwn.net/SubscriberLink/1083202/f1ba926cd57ac5c5/)
* [flodl 0.7.0: one dashboard view, repeated at every level](https://flodl.dev/blog/then-i-looked-at-it)
* [samkhya 1.2.1 — the join-cardinality ceiling becomes provable](https://github.com/singhpratech/samkhya/releases/tag/v1.2.1)
* [BrewFS: a Rust and JuiceFS-like distributed filesystem](https://brewfs.ai/en/blog/introducing-brewfs) 

### Observations/Thoughts

* [Improving std::simd::swizzle_dyn](https://shnatsel.github.io/improving-std-simd-swizzle-dyn/)
* [Query cycles: A compiler murder mystery](https://ferrous-systems.com/blog/query-cycles-a-compiler-murder-mystery/)
* [GDPatch: a versatile Godot mod loader](https://notnite.com/blog/gdpatch)
* [Memory Safety Absolutists](https://itsallaboutthebit.com/memory-safety-absolutists/)
* [C++ to Rust Migration](https://blog.jetbrains.com/rust/2026/07/27/cpp-to-rust-migration/)
* [High-Performance Flat 2D Arrays in Rust with SIMD, L1 Cache](https://developerlife.com/2026/07/14/build-high-performance-flat-2d-arrays-in-rust/)
* [Building Java–Rust Microservices with TeaQL: Models, Events, and Audit Intent](https://teaql.io/blog/java-rust-microservice-integration-with-teaql/)
* [How We Cut a Trading Bot's Reaction Time from ~2 Seconds to Milliseconds — by Moving Only the Hot Path to Rust](https://www.99francs.agency/blog/python-to-rust-trading-bot-migration)
* [ESP32 Server: Distributing HTTP/2 streams over TLS](https://c410-f3r.github.io/thoughts/esp32-server-distributing-http2-streams-over-tls)
* [video] [Rust Berlin Talks · 23/07/2026](https://www.youtube.com/watch?v=ut5EHZ2FK0c)

### Rust Walkthroughs

* [No Tokens Yet Does Not Mean a Rust LLM Stream Is Safe to Retry](https://ai-router.hashnode.dev/rust-llm-stream-retry-safety)
* [series] [Rama 101.2: Core Concepts](https://plabayo.tech/blog/rama-101-2-core-concepts)
* [video] [series] [What's Inside Axum?](https://www.youtube.com/watch?v=rBzPw6WurN0)

## Crate of the Week

This week's crate is [cargo-efmt](https://codeberg.org/filmroellchen/cargo-efmt), a drop-in replacement for cargo fmt to support `.editorconfig`.

Thanks to [kleines Filmröllchen](https://users.rust-lang.org/t/crate-of-the-week/2704/1632) for the self-suggestion!

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

570 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-07-21..2026-07-28

#### Compiler
* [apply RemoveNoopLandingPads post-monomorphization](https://github.com/rust-lang/rust/pull/143208)
* [closures inherit `#[optimize]` from the enclosing function by default](https://github.com/rust-lang/rust/pull/158901)
* [fix `bool` calling convention for aarch64, etc](https://github.com/rust-lang/rust/pull/159317)
* [optimize `escape_string_symbol()`](https://github.com/rust-lang/rust/pull/159609)
* [`proc_macro`: Fix `cfg_attr` inner attrs in file modules](https://github.com/rust-lang/rust/pull/159695)
* [resolve: more preperation work for parallelizing the import resolution loop](https://github.com/rust-lang/rust/pull/159440)
* [stabilize c-variadic function definitions](https://github.com/rust-lang/rust/pull/155697)

#### Library
* [constify `vec![1, 2, 3]` macro](https://github.com/rust-lang/rust/pull/155795)
* [core: implement `Rng` for references](https://github.com/rust-lang/rust/pull/159435)
* [define a `Simd` type in `minicore`](https://github.com/rust-lang/rust/pull/159656)
* [implement `CovariantUnsafeCell`](https://github.com/rust-lang/rust/pull/159738)
* [implement `str::copy_from_str`](https://github.com/rust-lang/rust/pull/159846)
* [iter: extend `step_by` specialization to cover `StepBy<RangeIter<{integer}>>`](https://github.com/rust-lang/rust/pull/159518)
* [move `std::io::buffered` to `alloc::io`](https://github.com/rust-lang/rust/pull/158547)
* [num: improve error messages for `TryFromIntError`](https://github.com/rust-lang/rust/pull/156225)
* [str: add ASCII fast path to `word_to_titlecase`](https://github.com/rust-lang/rust/pull/159271)
* [switch implementations of `thread_local!` for WASI](https://github.com/rust-lang/rust/pull/159733)

#### Cargo
* [add haiku's dylib path](https://github.com/rust-lang/cargo/pull/17248)
* [`diag`: bound transitive unused dependency traversal](https://github.com/rust-lang/cargo/pull/17251)
* [`git`: Hide git fetch output without progress](https://github.com/rust-lang/cargo/pull/17243)
* [`git`: Suggest libgit2 if git-cli fails](https://github.com/rust-lang/cargo/pull/17252)
* [`test`: gate trim-paths tests on split debuginfo support](https://github.com/rust-lang/cargo/pull/17256)
* [`toml`: warn on hyphenated lint names and duplicates](https://github.com/rust-lang/cargo/pull/17051)
* [allow setting `-Zembed-metadata` value from the config](https://github.com/rust-lang/cargo/pull/17266)
* [enable build-dir layout v2 on nightly by default](https://github.com/rust-lang/cargo/pull/17258)
* [zsh completion: Add `-p` and `--package` flags for `cargo add`](https://github.com/rust-lang/cargo/pull/17247)

#### Rustfmt
* [allow file not found errors for external mods annotated with `#[my_macro]`](https://github.com/rust-lang/rust/pull/159737)
* [discover modules via `cfg_select!`](https://github.com/rust-lang/rust/pull/158372)

#### Rustdoc
* [add paths for linked associated items](https://github.com/rust-lang/rust/pull/156474)
* [Retrieve `cfg_attr` information for derived impls for `doc_cfg` feature](https://github.com/rust-lang/rust/pull/159722)
* [only build extern trait impls if needed](https://github.com/rust-lang/rust/pull/159623)
* [only inline impls for local primitives](https://github.com/rust-lang/rust/pull/159721)

#### Clippy
* [add `EULER_GAMMA` and `GOLDEN_RATIO` to `approx_constant`](https://github.com/rust-lang/rust-clippy/pull/17441)
* [add `assert_is_empty` lint](https://github.com/rust-lang/rust-clippy/pull/17149)
* [apply safety comment to compound assignment statement](https://github.com/rust-lang/rust-clippy/pull/17044)
* [`blocks_in_conditions`: Don't lint if the block creates temporarie…](https://github.com/rust-lang/rust-clippy/pull/17420)
* [call `in_external_macro` after running other checks in various places](https://github.com/rust-lang/rust-clippy/pull/17294)
* [do not trigger `clippy::exit` when expression comes from an external macro](https://github.com/rust-lang/rust-clippy/pull/17105)
* [`duration_suboptimal_units`: print the complete method name in the suggestion](https://github.com/rust-lang/rust-clippy/pull/17002)
* [extend `branches_sharing_code` to match arms with a shared tail](https://github.com/rust-lang/rust-clippy/pull/17313)
* [`min_ident_chars` lint short idents even if follows trait naming](https://github.com/rust-lang/rust-clippy/pull/16741)
* [`multiple_unsafe_ops_per_block`: false positive in with taking an reference to a static, but not reading/writing it](https://github.com/rust-lang/rust-clippy/pull/17461)
* [fix `four_forward_slashes` false positive on inner doc comments](https://github.com/rust-lang/rust-clippy/pull/17448)
* [`lint-page`: add accessible labels to filters](https://github.com/rust-lang/rust-clippy/pull/17434)
* [new lint: `nonnull_unchecked_on_box_ptr`](https://github.com/rust-lang/rust-clippy/pull/17336)
* [perf: avoid per-call type and path work in `unnecessary_mut_passed`](https://github.com/rust-lang/rust-clippy/pull/17227)
* [perf: find tab groups in doc comments without allocating](https://github.com/rust-lang/rust-clippy/pull/17410)
* [rewrite `EndianBytes` lint pass](https://github.com/rust-lang/rust-clippy/pull/17363)

#### Rust-Analyzer
* [add diagnostic for `struct` patterns which don't specify sub-patterns for its fields](https://github.com/rust-lang/rust-analyzer/pull/22851)
* [add parentheses for invert general expression](https://github.com/rust-lang/rust-analyzer/pull/22898)
* [attach db on worker threads in parallel analysis-stats inference](https://github.com/rust-lang/rust-analyzer/pull/22905)
* [change unsupported toolchain version to match reality](https://github.com/rust-lang/rust-analyzer/pull/22876)
* [discover protocol should only parse stdout](https://github.com/rust-lang/rust-analyzer/pull/22903)
* [do not detect `#[rust_analyzer]` as `#[rust_analyzer::rust_fixture]`](https://github.com/rust-lang/rust-analyzer/pull/22881)
* [don't offer `replace_qualified_name_with_use` on an unqualified path](https://github.com/rust-lang/rust-analyzer/pull/22919)
* [don't panic on a qualified path whose trait is not a trait](https://github.com/rust-lang/rust-analyzer/pull/22930)
* [don't pick a discriminant type larger than typeck's](https://github.com/rust-lang/rust-analyzer/pull/22932)
* [fix stale lock file](https://github.com/rust-lang/rust-analyzer/pull/22908)
* [fix `.zip(None)` call](https://github.com/rust-lang/rust-analyzer/pull/22924)
* [give `impl_trait_with_diagnostics` a cycle result](https://github.com/rust-lang/rust-analyzer/pull/22923)
* [make analysis-stats progress bar Unicode-safe](https://github.com/rust-lang/rust-analyzer/pull/22909)
* [`merge_imports` panic on invalid paths](https://github.com/rust-lang/rust-analyzer/pull/22892)
* [panic on macro-defined structs with unknown fields](https://github.com/rust-lang/rust-analyzer/pull/22843)
* [prefer `alloc` over `std` paths when `preferNoStd` is set](https://github.com/rust-lang/rust-analyzer/pull/22918)
* [record obligation chain for unimplemented trait diagnostics and show it](https://github.com/rust-lang/rust-analyzer/pull/22854)
* [replace detach with delete for `ast::IdentPat`](https://github.com/rust-lang/rust-analyzer/pull/22916)
* [resolve path on all namespace on `resolve_path`](https://github.com/rust-lang/rust-analyzer/pull/22743)
* [respect `references.exclude[Tests/Imports]` in references lens](https://github.com/rust-lang/rust-analyzer/pull/22660)
* [scoped lazy priming](https://github.com/rust-lang/rust-analyzer/pull/22587)
* [support inactive-code diagnostic in macros](https://github.com/rust-lang/rust-analyzer/pull/22306)
* [uses bool instead pat ty in guard](https://github.com/rust-lang/rust-analyzer/pull/22896)

### Rust Compiler Performance Triage

Several large improvements landed in the past week:

* rustdoc is on average roughly 16% faster across all of our doc benchmarks:
  * [rustdoc: Only inline impls for local primitives](https://github.com/rust-lang/rust/pull/159721), 7% faster doc builds
  * [rustdoc: Only synthesize auto/blanket impls for documented items](https://github.com/rust-lang/rust/pull/159779), another 7% faster doc builds
  * [rustdoc: Only build extern trait impls if needed](https://github.com/rust-lang/rust/pull/159623), another 10% faster doc builds
* [Early removal of no-op panic handling in debug builds](https://github.com/rust-lang/rust/pull/143208). This speeds up Cargo by ~4% in cycle count.
* [Optimize escape_string_symbol()](https://github.com/rust-lang/rust/pull/159609) sped
  up large `include_bytes!`/`include_str!` through changes to string escaping, avoiding a regression in upcoming LLVM 23 upgrade.

Great to see so many improvements!

Triage done by **@simulacrum**.
Revision range: [d527bc9b..ad0c9dce](https://perf.rust-lang.org/?start=d527bc9bfa297ca7fd7f5ae93781eeec42073170&end=ad0c9dce27a22416b65946bc0010edaf22ac6c83&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-07-27.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Shallow resolve ty and const vars to their root vars, attempt 2](https://github.com/rust-lang/rust/pull/158447)
* [Ensure inferred let pattern types are well-formed](https://github.com/rust-lang/rust/pull/157841)
* [stabilize `c_variadic_naked_functions`](https://github.com/rust-lang/rust/pull/159746)
* [lint against repeated repr attributes](https://github.com/rust-lang/rust/pull/157036)
* [Stabilize passing 128-bit integers via vector registers with `asm!` on x86](https://github.com/rust-lang/rust/pull/159525)
* [Add new `invalid_markdown_table` rustdoc lint](https://github.com/rust-lang/rust/pull/159583)
* [allocations: document that they can be read-only](https://github.com/rust-lang/rust/pull/159503)
* [allocations are allowed to grow (but not shrink)](https://github.com/rust-lang/rust/pull/159729)
* [Tracking Issue for `bool::toggle`](https://github.com/rust-lang/rust/issues/159298)
* [Tracking Issue for const_btree_len](https://github.com/rust-lang/rust/issues/71835)
* [Add `raw_borrows_via_references` lint](https://github.com/rust-lang/rust/pull/138230)

* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Wasm proc macro support](https://github.com/rust-lang/compiler-team/issues/1017)
* [group target modifier options under -T](https://github.com/rust-lang/compiler-team/issues/980)

* [Optimize repr(Rust) enums by omitting tags in more cases involving uninhabited variants.](https://github.com/rust-lang/compiler-team/issues/922)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Refactor the libs team](https://github.com/rust-lang/rfcs/pull/3984)
* [Cargo: `hints.min-opt-level`](https://github.com/rust-lang/rfcs/pull/3924)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(profile): Add built-in profile debug](https://github.com/rust-lang/cargo/pull/17214)
* [feat(toml): allow overriding inherited default-features in 2024](https://github.com/rust-lang/cargo/pull/17126)

*No Items entered Final Comment Period this week for
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Externref lang item for Wasm targets](https://github.com/rust-lang/rfcs/pull/3987)

## Upcoming Events

Rusty Events between 2026-07-29 - 2026-08-26 🦀

### Virtual
* 2026-07-30 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/312045928/)
* 2026-07-31 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/uo5ek1f4)
* 2026-08-01 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-08-02 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095294/)
* 2026-08-03 | Virtual (Global) | [Rust Maven](https://luma.com/rust-maven)
    * [**Workshop: Add tests to an open source Rust project**](https://luma.com/nwfmsdtf)
* 2026-08-04 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315213885/)
* 2026-08-04 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/315800760/)
* 2026-08-05 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Execution and Scheduling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/315880365/)
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
* 2026-08-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-20 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #5 — Wireless Communication with the IEEE 802.15.4 protocol**](https://www.meetup.com/charlottesville-rust-meetup/events/315733791/)
* 2026-08-21 | Virtual (Girona, ES) | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/1bm27cah)
* 2026-08-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254775/)

### Africa
* 2026-08-11 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust's extended standard library**](https://www.meetup.com/johannesburg-rust-meetup/events/315750593/)

### Asia
* 2026-08-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/august-2026-rustacean-meetup/)
* 2026-08-22 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi X SciPy India Meetup**](https://www.meetup.com/rustdelhi/events/315185336/)
* 2026-08-22 | Noida, IN | [SciPy India](https://scipy.in/)
    * [**Scientific Computing in Rust and Python**](https://scipy.in/sci-py-rs/)

### Europe
* 2026-07-29 | Poland, PL | [Rust Poland](https://www.meetup.com/rust-poland-meetup)
    * [**Rust Poland x Kraków #10**](https://www.meetup.com/rust-poland-meetup/events/315582674/)
* 2026-07-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #70**](https://www.meetup.com/copenhagen-rust-community/events/315767999/)
* 2026-07-30 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester July Code Night**](https://www.meetup.com/rust-manchester/events/315037685/)
* 2026-08-06 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**ACCU/Rust Summer social**](https://www.meetup.com/oxford-rust-meetup-group/events/315863373/)
* 2026-08-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night: Trust but verify the LLM**](https://www.meetup.com/rust-aarhus/events/315683629/)
* 2026-08-18 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816474/)
* 2026-08-20 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Building an acoustic camera with egui and embassy**](https://www.meetup.com/rust-rhein-main/events/315855368/)

### North America
* 2026-07-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539329/)
* 2026-08-01 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Chinatown Rust Lunch, Aug 1**](https://www.meetup.com/bostonrust/events/315582653/)
* 2026-08-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Evening Boston Rust Meetup at Red Hat, Aug 4**](https://www.meetup.com/bostonrust/events/314660176/)
* 2026-08-06 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/315590399/)
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
* 2026-08-19 | San Francisco, CA, US | [Rust Bay Area](https://luma.com/bayarearust)
    * [**Rust Bay Area August Meetup**](https://luma.com/00f2s7q9)
* 2026-08-20 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**August, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314520814/)
* 2026-08-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315171660/)

### Oceania
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

> So let's talk about what the process has looked like for Netstack3. For 11 months, the team has been ramping up a dogfooding program. At peak, that program has seen about 60 devices running nearly 24/7 in developers' homes.
>
> Again, if this were any other netstack, we would have expected to uncover a giant mountain of bugs in that time. So, over the past year, how many bugs did the team uncover in the field?
>
> Three.

– [Josh Liebow-Feeser on his blog](https://joshlf.com/posts/safety-unsafe-world/)

llogiq again has no one to thank for a suggestion, so he is thankful to himself for finding this quote instead.

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1vaibge/this_week_in_rust_662/)</small>
