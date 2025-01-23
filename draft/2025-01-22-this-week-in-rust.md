Title: This Week in Rust 583
Number: 583
Date: 2025-01-22
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X (formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [This Development-cycle in Cargo: 1.85](https://blog.rust-lang.org/inside-rust/2025/01/17/this-development-cycle-in-cargo-1.85.html)

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #37](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-37)

### Project/Tooling Updates
* [gitoxide - January 2025](https://github.com/GitoxideLabs/gitoxide/discussions/1791)
* [musi lili 0.2 released](https://codeberg.org/vivi-ui/lili/src/branch/main/CHANGELOG.md#0-2)

### Observations/Thoughts
* [Comparing Rust Actor Libraries: Actix, Coerce, Kameo, Ractor, and Xtra](https://theari.dev/blog/comparing-rust-actor-libraries/)
* [Improve Rust Compile Time by 108X](https://burn.dev/blog/improve-rust-compile-time-by-108x)
* [Branchless UTF-8 Encoding](https://cceckman.com/writing/branchless-utf8-encoding/)
* [The hunt for error -22](https://tweedegolf.nl/en/blog/145/the-hunt-for-error--22)
* [Automatic Server Reloading in Rust on Change: What is listenfd/systemfd?](https://lucumr.pocoo.org/2025/1/19/what-is-systemfd/)
* [Investigating a Strange Out-of-Memory Error](https://www.qovery.com/blog/rust-investigating-a-strange-out-of-memory-error/)
* [Comparing 13 Rust Crates for Extracting Text from HTML](https://emschwartz.me/comparing-13-rust-crates-for-extracting-text-from-html/)
* [Typesafe Frontend Routing in Rust/Leptos](https://dnaaun.github.io/posts/typesafe-routing-in-rust-leptos/)
* [Interview with passionate Rust Developer Radu Marias](https://blog.rust.careers/post/radu_interview/)
* [Introducing RealtimeSanitizer for Rust](https://steck.tech/posts/rtsan-in-rust/)
* [The HARM Stack (HTMX, Axum/AlpineJS, Rust, Maud) Considered Unharmful](https://nguyenhuythanh.com/posts/the-harm-stack-considered-unharmful/)
* [Type Inference in Rust and C++](https://herecomesthemoon.net/2025/01/type-inference-in-rust-and-cpp/)
* [video] [Rust is not Complicated - An architects perspective](https://www.youtube.com/watch?v=bYKPKBPo7EY)
* [Path canonicalization is unergonomic](https://tesujimath.org/path-canonicalization-is-unergonomic/)

### Rust Walkthroughs
* [A Gentle Introduction to WebAssembly in Rust (2025 Edition)](https://dev.to/marktolmacs/a-gentle-introduction-to-webassembly-in-rust-2025-edition-1iac)
* [Adding A New Fake To The Fake Crate](https://xd009642.github.io/2025/01/15/adding-a-new-fake-to-the-fake-crate.html)
* [Streaming Audio Apis: The Axum Server](https://xd009642.github.io/2025/01/20/streaming-audio-APIs-the-axum-server.html)
* [Prototyping in Rust](https://corrode.dev/blog/prototyping/)

### Research

### Miscellaneous
* [2024 in review - seanmonstar](https://seanmonstar.com/blog/2024-in-review/)
* ["We never update unless forced to" — cargo-semver-checks 2024 Year in Review](https://predr.ag/blog/cargo-semver-checks-2024-year-in-review/)
* [Year in review: 2024 highlights and a peek at 2025](https://graphite.rs/blog/year-in-review-2024-highlights-and-a-peek-at-2025)
* [Why Rust is not a functional language](https://serokell.io/blog/rust-is-not-a-functional-language)
* [How I think about Zig and Rust](https://lewiscampbell.tech/blog/250117.html)
* [video] [Breaking Rust’s memory safety with 1 line of code](https://youtu.be/HGERkwG0ViI)
* [audio] [Crates We Love | Oxide and Friends](https://share.transistor.fm/s/d3cea637)

## Crate of the Week

This week's crate is [resvg](https://crates.io/crates/resvg), an SVG rendering library.

Thanks to [David Mason](https://users.rust-lang.org/t/crate-of-the-week/2704/1389) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No calls for testing were issued this week.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No calls for testing were issued this week.*

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

* [rama - first version of rama-unix](https://github.com/plabayo/rama/issues/370)
* [rama - add web Router service to rama-http](https://github.com/plabayo/rama/issues/396)
* [rama - add full-stack rama benchmarks](https://github.com/plabayo/rama/issues/374)
* [rama - add rama to "the-benchmarker"](https://github.com/plabayo/rama/issues/398)
* [rama - add rama server benchmark to sharkbench](https://github.com/plabayo/rama/issues/390)
* [rama - add rama to TechEmpower's FrameworkBenchmark](https://github.com/plabayo/rama/issues/389)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

397 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-01-14..2025-01-21

* [`cfg_match`: Adjust syntax](https://github.com/rust-lang/rust/pull/133720)
* [`rustc_resolve`: use structured fields in traces](https://github.com/rust-lang/rust/pull/135676)
* [add Profile Override for Non-Git Sources](https://github.com/rust-lang/rust/pull/135433)
* [add cache to `AmbiguityCausesVisitor`](https://github.com/rust-lang/rust/pull/135618)
* [add gpu-kernel calling convention](https://github.com/rust-lang/rust/pull/135047)
* [add license-metadata.json to rustc-src tarball](https://github.com/rust-lang/rust/pull/135588)
* [allow coercing safe-to-call `target_feature` functions to safe fn pointers](https://github.com/rust-lang/rust/pull/135504)
* [always force non-trimming of path in `unreachable_patterns` lint](https://github.com/rust-lang/rust/pull/135310)
* [cleanup promoteds move check](https://github.com/rust-lang/rust/pull/134455)
* [consider more erroneous layouts as `LayoutError::ReferencesError` to suppress spurious errors](https://github.com/rust-lang/rust/pull/135264)
* [consolidate ad-hoc MIR lints into real pass-manager-based MIR lints](https://github.com/rust-lang/rust/pull/135705)
* [const traits: remove some known-bug that do not seem to make sense](https://github.com/rust-lang/rust/pull/135523)
* [const-eval: detect more pointers as definitely not-null](https://github.com/rust-lang/rust/pull/133700)
* [convert `struct FromBytesWithNulError` into `enum`](https://github.com/rust-lang/rust/pull/134143)
* [coverage: completely overhaul counter assignment, using node-flow graphs](https://github.com/rust-lang/rust/pull/135481)
* [detect if-else chains with a missing final else in type errors](https://github.com/rust-lang/rust/pull/135558)
* [disallow `A { .. }` if `A` has no fields](https://github.com/rust-lang/rust/pull/135703)
* [do not consider traits that have unsatisfied const conditions to be conditionally const](https://github.com/rust-lang/rust/pull/135425)
* [don't skip argument parsing when running `rustc` with no arguments](https://github.com/rust-lang/rust/pull/135716)
* [eagerly mono drop for structs with lifetimes](https://github.com/rust-lang/rust/pull/135313)
* [encode constraints that hold at all points as logical edges in location-sensitive polonius](https://github.com/rust-lang/rust/pull/135290)
* [enforce syntactical stability of const traits in HIR](https://github.com/rust-lang/rust/pull/135423)
* [fix ICE in resolving associated items as non-bindings](https://github.com/rust-lang/rust/pull/135663)
* [fix overflows in the implementation of `overflowing_literals` lint's help](https://github.com/rust-lang/rust/pull/135249)
* [fix suggestion to convert dereference of raw pointer to ref](https://github.com/rust-lang/rust/pull/135601)
* [for purely return-type based searches, deprioritize clone-like functions](https://github.com/rust-lang/rust/pull/135302)
* [fully de-stabilize all custom inner attributes](https://github.com/rust-lang/rust/pull/134276)
* [further improve `panic_immediate_abort` by removing rtprintpanic! messages](https://github.com/rust-lang/rust/pull/135446)
* [implement `use` associated items of traits](https://github.com/rust-lang/rust/pull/134754)
* [improve `DispatchFromDyn` and `CoerceUnsized` impl validation](https://github.com/rust-lang/rust/pull/135228)
* [leak check in `impossible_predicates` to avoid monomorphizing impossible instances](https://github.com/rust-lang/rust/pull/135466)
* [location-sensitive polonius prototype: endgame](https://github.com/rust-lang/rust/pull/134980)
* [make `missing_abi` lint warn-by-default](https://github.com/rust-lang/rust/pull/132397)
* [make sure to scrape region constraints from deeply normalizing type outlives assumptions in borrowck](https://github.com/rust-lang/rust/pull/134940)
* [make sure we actually use the right trivial lifetime substs when eagerly monomorphizing drop for ADTs](https://github.com/rust-lang/rust/pull/135520)
* [make sure we can produce `ConstArgHasWrongType` errors for valtree consts](https://github.com/rust-lang/rust/pull/135380)
* [methods of const traits are const](https://github.com/rust-lang/rust/pull/135541)
* [mir borrowck: cleanup late-bound region handling](https://github.com/rust-lang/rust/pull/135479)
* [new solver: prefer trivial builtin impls](https://github.com/rust-lang/rust/pull/135639)
* [only treat plain literal patterns as short](https://github.com/rust-lang/rust/pull/135251)
* [outline panicking code for `LocalKey::with`](https://github.com/rust-lang/rust/pull/135224)
* [Replace extern "rust-intrinsic" with `#[rustc_intrinsic]` across the codebase](https://github.com/rust-lang/rust/pull/135333)
* [prefer lower `TraitUpcasting` candidates in selection](https://github.com/rust-lang/rust/pull/135498)
* [provide structured suggestion for `#![feature(..)]` in more cases](https://github.com/rust-lang/rust/pull/134858)
* [reexport likely/unlikely in `std::hint`](https://github.com/rust-lang/rust/pull/133695)
* [respect --sysroot for rustc -vV and -Cpasses=list](https://github.com/rust-lang/rust/pull/135330)
* [stable Hash: Ignore all HirIds that just identify the node itself](https://github.com/rust-lang/rust/pull/135329)
* [treat other items as functions for the purpose of type-based search](https://github.com/rust-lang/rust/pull/131806)
* [treat safe `target_feature` functions as unsafe by default (less invasive variant)](https://github.com/rust-lang/rust/pull/134353)
* [use a C-safe return type for `__rust_[ui]128_*` overflowing intrinsics](https://github.com/rust-lang/rust/pull/134338)
* [use indirect return for `i128` and `f128` on wasm32](https://github.com/rust-lang/rust/pull/135534)
* [use trait definition cycle detection for trait alias definitions, too](https://github.com/rust-lang/rust/pull/134504)
* [when LLVM's location discriminator value limit is exceeded, emit locations with dummy spans instead of dropping them entirely](https://github.com/rust-lang/rust/pull/135643)
* [use a C-safe return type for `__rust_[ui]128_*` overflowing intrinsics](https://github.com/rust-lang/compiler-builtins/pull/735)
* [stabilize `float_next_up_down`](https://github.com/rust-lang/rust/pull/135661)
* [std: lazily allocate the main thread handle](https://github.com/rust-lang/rust/pull/132654)
* [made `Path::name` only have item name rather than full name](https://github.com/rust-lang/rust/pull/134880)
* [move `std::pipe::*` into `std::io`](https://github.com/rust-lang/rust/pull/135583)
* [less unsafe in `dangling`/`without_provenance`](https://github.com/rust-lang/rust/pull/135344)
* [cargo: created a function for user defined aliases](https://github.com/rust-lang/cargo/pull/15076)
* [cargo: fix benchsuite issue with newer versions of git](https://github.com/rust-lang/cargo/pull/15069)
* [cargo: fix: wrong concat and field name](https://github.com/rust-lang/cargo/pull/15074)
* [cargo: took the functionality of the third party subcommand from the `list_commands` function](https://github.com/rust-lang/cargo/pull/15075)
* [rustdoc: remove `AttributesExt` trait magic that added needless complexity](https://github.com/rust-lang/rust/pull/135428)
* [rustdoc: Replace module list items `ul`/`li` with `dl`/`dd`/`dt` elements](https://github.com/rust-lang/rust/pull/135641)
* [clippy: add a new lint for `repeat().take()` that can be replaced with `repeat_n()`](https://github.com/rust-lang/rust-clippy/pull/13858)
* [clippy: change `literal_string_with_formatting_args` lint category to nursery](https://github.com/rust-lang/rust-clippy/pull/14014)
* [clippy: emit `missing_const_for_fn` for `CONST_MUT_REFS`](https://github.com/rust-lang/rust-clippy/pull/13839)
* [clippy: fix: correct suggestion for `significant_drop_in_scrutinee` in expressions](https://github.com/rust-lang/rust-clippy/pull/14019)
* [clippy: new lint `useless-nonzero-new_unchecked`](https://github.com/rust-lang/rust-clippy/pull/13993)
* [clippy: new lint: `unnecessary_semicolon`](https://github.com/rust-lang/rust-clippy/pull/14032)
* [clippy: rust 1.81 and later support elision with explicit self types](https://github.com/rust-lang/rust-clippy/pull/13992)
* [clippy: suggest `manual_div_ceil` even when right operand is a constant](https://github.com/rust-lang/rust-clippy/pull/13951)
* [clippy: use clearer multipart suggestions for `unnecessary_map_or` lint](https://github.com/rust-lang/rust-clippy/pull/13998)
* [rust-analyzer: add missing `#[rust_analyzer::rust_fixture]` annotations](https://github.com/rust-lang/rust-analyzer/pull/18951)
* [rust-analyzer: add missing `Win32_Foundation` feature](https://github.com/rust-lang/rust-analyzer/pull/18963)
* [rust-analyzer: extract variable assist triggers less eagerly](https://github.com/rust-lang/rust-analyzer/pull/18982)
* [rust-analyzer: add dereferencing autocomplete](https://github.com/rust-lang/rust-analyzer/pull/18917)
* [rust-analyzer: add smart completions that skip `await` or `iter()` and `into_iter()`](https://github.com/rust-lang/rust-analyzer/pull/18927)
* [rust-analyzer: add the ability to jump from `into` to `from` definitions](https://github.com/rust-lang/rust-analyzer/pull/18934)
* [rust-analyzer: complete raw, const keyword](https://github.com/rust-lang/rust-analyzer/pull/18952)
* [rust-analyzer: render type parameter projection target bounds in inlays](https://github.com/rust-lang/rust-analyzer/pull/18925)
* [rust-analyzer: show go-to-type-def actions for func param and trait bound when hovering](https://github.com/rust-lang/rust-analyzer/pull/18946)
* [rust-analyzer: `cargo rustc --print` needs `unstable-options`](https://github.com/rust-lang/rust-analyzer/pull/18968)
* [rust-analyzer: detect missing errors for } braces before else in let...else statements](https://github.com/rust-lang/rust-analyzer/pull/18908)
* [rust-analyzer: don't return inlay hints outside requested range](https://github.com/rust-lang/rust-analyzer/pull/18922)
* [rust-analyzer: fix a bug where `enum` variants were not considered properly in type ns resolution](https://github.com/rust-lang/rust-analyzer/pull/18976)
* [rust-analyzer: fix another bug when reaching macro expansion limit caused a stack overflow](https://github.com/rust-lang/rust-analyzer/pull/18929)
* [rust-analyzer: fix missing upmapping in trait impls completion](https://github.com/rust-lang/rust-analyzer/pull/18977)
* [rust-analyzer: fix semantics not always correctly caching file roots](https://github.com/rust-lang/rust-analyzer/pull/18940)
* [rust-analyzer: make `test_runner::TestState::stdout` optional to fix parsing cargo test json output](https://github.com/rust-lang/rust-analyzer/pull/18897)
* [rust-analyzer: flip on typing config to be opt-in, better defaults](https://github.com/rust-lang/rust-analyzer/pull/18939)
* [rust-analyzer: generalize some type walking in hover type actions](https://github.com/rust-lang/rust-analyzer/pull/18950)
* [rust-analyzer: lsp-server: drop outgoing messages on background thread](https://github.com/rust-lang/rust-analyzer/pull/18972)
* [rust-analyzer: proc-macro-srv: make usage of `RTLD_DEEPBIND` portable](https://github.com/rust-lang/rust-analyzer/pull/18981)
* [rust-analyzer: properly record meaningful imports as re-exports in symbol index](https://github.com/rust-lang/rust-analyzer/pull/18967)

### Rust Compiler Performance Triage

A very quiet week for performance, with small improvements essentially on all benchmarks.

Triage done by **@simulacrum**.
Revision range: [1ab85fbd..9a1d156f](https://perf.rust-lang.org/?start=1ab85fbd7474e8ce84d5283548f21472860de3e2&end=9a1d156f38c51441ee51e5a068f1d0caf4bb0f27&absolute=false&stat=instructions%3Au)

0 Regression, 1 Improvement, 2 Mixed; 0 of them in rollups
40 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-01-20.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Supertrait item shadowing v2](https://github.com/rust-lang/rfcs/pull/3624)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [[RFC] Target Modifiers](https://github.com/rust-lang/rfcs/pull/3716)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [remove long-deprecated no-op attributes no_start and crate_id](https://github.com/rust-lang/rust/pull/134300)
* [Stabilize target_feature_11](https://github.com/rust-lang/rust/pull/134090)
* [Add `File already exists` error doc to `hard_link` function](https://github.com/rust-lang/rust/pull/135415)
* [Tracking Issue for `once_wait`](https://github.com/rust-lang/rust/issues/127527)
* [fix(libtest): Deprecate '--logfile'](https://github.com/rust-lang/rust/pull/134283)
* [Windows: remove readonly files](https://github.com/rust-lang/rust/pull/134679)
* [make rustc_encodable_decodable feature properly unstable](https://github.com/rust-lang/rust/pull/134272)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: project goals for 2025h1](https://github.com/rust-lang/rfcs/pull/3764)

## Upcoming Events

Rusty Events between 2025-01-22 - 2025-02-19 🦀

### Virtual
* 2025-01-22 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #8**](https://www.meetup.com/bevy-game-development/events/305111151)
* 2025-01-23 & 2025-01-24 | Virtual | [Mainmatter Rust Workshop](https://ti.to/mainmatter/)
    * [**Remote Workshop: Testing for Rust projects – going beyond the basics**](https://ti.to/mainmatter/rust-testing-jan-2025)
* 2025-01-24 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305425309/)
* 2025-01-26 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Rust and embedded programming with Leon Vak (online in Hebrew)**](https://www.meetup.com/rust-tlv/events/304971264)
* 2025-01-27 | Virtual (London, UK) | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/)
    * [**using traits in Rust for flexibility, mocking/ unit testing, and more**](https://www.meetup.com/london-rust-project-group/events/305211672/)
* 2025-01-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/305361243)
* 2025-01-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/299468340)
* 2025-01-30 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Quantum Computers Can’t Rust-Proof This!**](https://www.meetup.com/charlottesville-rust-meetup/events/305391474)
* 2025-01-30 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller**](https://www.meetup.com/code-mavens/events/305382647)
* 2025-01-31 | Virtual (Delhi, IN) | [Hackathon Raptors Association](https://www.meetup.com/hackathon-raptors-association/)
    * [**Blazingly Fast Rust Hackathon**](https://www.meetup.com/hackathon-raptors-association/events/305435372/)
* 2025-01-31 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/305560416/)
* 2025-02-01 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-02-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304216)
* 2025-02-04 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Rust Nation UK Talks**](https://www.meetup.com/women-in-rust/events/305647334)
* 2025-02-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031658)
* 2025-02-07 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbkb/)
* 2025-02-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/302815036)
* 2025-02-11 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Meet Elusion: New DataFrame Library powered by Rust 🦀 with Borivoj Grujicic**](https://www.meetup.com/code-mavens/events/305513416)
* 2025-02-13 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/299468342)
* 2025-02-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170682)
* 2025-02-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Pointer Provenance**](https://www.meetup.com/vancouver-rust/events/304051783)

### Europe
* 2025-01-22 | London, GB | [Rust London User Group](https://www.meetup.com/rust-london-user-group/events/)
    * [**Rust London's New Years Party & Community Swag Drop**](https://www.meetup.com/rust-london-user-group/events/305588703)
* 2025-01-22 | Oberursel, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust 2024 Edition and Beyond**](https://www.meetup.com/rust-rhein-main/events/305330873)
* 2025-01-23 | Barcelona, ES | [Barcelona Free Software](https://www.meetup.com/barcelona-free-software/events/)
    * [**Why Build a New Browser Engine in Rust?**](https://www.meetup.com/barcelona-free-software/events/305179554)
* 2025-01-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #74**](https://www.meetup.com/rust-paris/events/305455221)
* 2025-01-24 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/305204279)
* 2025-01-27 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust Meetup Prague (January 2025)**](https://www.meetup.com/rust-prague/events/305455153)
* 2025-01-27 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/events/)
    * [**Meetup 2025/01: Optimizacija buildanja C++ i usporedba s Rustom (GUI alat)**](https://www.meetup.com/zagreb-rust-meetup/events/305624480)
* 2025-01-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Advent of Code**](https://www.meetup.com/rust-aarhus/events/304487851)
* 2025-01-28 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester January Code Night**](https://www.meetup.com/rust-manchester/events/305496243)
* 2025-01-28 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/events/)
    * [**Rust Warsaw Meetup #4**](https://www.meetup.com/rust-warsaw/events/305628338)
* 2025-01-30 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #11: Hypermedia-driven development in Rust**](https://rust-augsburg.github.io/meetup/Meetup_11.html)
* 2025-01-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421383)
* 2025-01-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #54 sponsored by Google**](https://www.meetup.com/copenhagen-rust-community/events/305453880)
* 2025-02-01 | Brussels, BE | [FOSDEM 2025](https://fosdem.org/2025/)
    * [**FOSDEM Rust Devroom**](https://fosdem.org/2025/schedule/track/rust/)
* 2025-02-01 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/events/)
    * [**February Meetup**](https://www.meetup.com/finland-rust-meetup/events/305666104)
* 2025-02-01 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Sinsheim**](https://www.meetup.com/rust-noris/events/305361544)
* 2025-02-05 | Oxford, GB | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123401)
* 2025-02-06 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #7**](https://www.meetup.com/rust-gdansk/events/305742562)
* 2025-02-12 | Reading, GB | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045444)
* 2025-02-18 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Introduction to Context-Generic Programming in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729528)
* 2025-02-19 - 2025-02-20 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2025**](https://www.rustnationuk.com/)

### North America
* 2025-01-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/305325657)
* 2025-01-23 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305414182) | [**Rust Meetup at Hacker Dojo - Mountain View Rust Meetup Page**](https://www.meetup.com/mv-rust-meetup/events/305564600)
* 2025-01-28 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/events/)
    * [**From Basics to Advanced: Testing**](https://www.meetup.com/boulder-rust-meetup/events/305597961)
* 2025-02-06 | Mountain view, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/305517476)
* 2025-02-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Async, the Future of Futures**](https://www.meetup.com/stl-rust/events/304959018)
* 2025-02-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/305720765)
* 2025-02-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638261)

### Oceania
* 2025-02-04 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/events/)
    * [**Rust AKL: How We Learn Rust**](https://www.meetup.com/rust-akl/events/305583693)

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

> Memory safety issues mean you can’t trust what you’re seeing in your source code anymore.

– [Someone from Antithesis on the shuttle blog](https://www.shuttle.dev/blog/2025/01/14/the-appeal-of-rust)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1651) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
