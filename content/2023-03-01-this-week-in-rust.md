Title: This Week in Rust 484
Number: 484
Date: 2023-03-01
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
* [Keyword Generics Progress Report: February 2023](https://blog.rust-lang.org/inside-rust/2023/02/23/keyword-generics-progress-report-feb-2023.html)

### Newsletters
* [This Month in Rust GameDev #42 - January 2023](https://gamedev.rs/news/042/)

### Project/Tooling Updates
* [rust-analyzer Changelog #170](https://rust-analyzer.github.io/thisweek/2023/02/27/changelog-170.html)
* [IntelliJ Rust Changelog #189](https://intellij-rust.github.io/2023/02/27/changelog-189.html)
* [gitoxide - [January]: The first `cargo` integration review](https://github.com/Byron/gitoxide/discussions/757)
* [Dioxus 0.3 - Templates, Hot Reloading, LiveView, and more](https://dioxuslabs.com/blog/release-030/)
* [Rust on Espressif chips - 2023 Roadmap](https://mabez.dev/blog/posts/esp-rust-24-02-2023/)
* [Announcing transitive 0.4.1, for better transitivity between types](https://www.reddit.com/r/rust/comments/119roow/announcing_transitive_041_for_better_transitivity/)
* [Announcing `compact_str` version 0.7! A small string optimization for Rust](https://www.reddit.com/r/rust/comments/1192ord/announcing_compact_str_version_07_a_small_string/)
* [Introducing Ambient 0.1](https://www.ambient.run/post/introducing-ambient)
* [This Week in Fyrox #16](https://fyrox.rs/blog/post/twif16/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - Still More To Do](https://www.fornjot.app/blog/weekly-release/2023-w09/)
* [video] [Cargo's New Sparse Index w/special guest Arlo Siemsen](https://www.youtube.com/watch?v=LWef_2IaFDk)
* [Kani Internship Projects 2022: Function Stubbing](https://model-checking.github.io/kani-verifier-blog/2023/02/28/kani-internship-projects-2022-stubbing.html)
* [Announcing Magmide Month!](https://github.com/magmide/magmide/discussions/28) (proof language for/using Rust)

### Observations/Thoughts
* [video] [Rust and RAII Memory Management - Computerphile](https://www.youtube.com/watch?v=pTMvh6VzDls)
* [All the Problems of Mutation](https://samsartor.com/guis-3/)
* [Learning Rust Part 1: A kitten's guide to Options and Results](https://cherrykitten.dev/blog/rust-1-options-results/)
* [When Rust hurts](https://mmapped.blog/posts/15-when-rust-hurts.html)
* [Time Travel Debugging in Rust](https://www.travelneil.com/time-travel-debugging-in-rust.html)
* [What I learned from contributing to Rust's linter](https://blog.nindalf.com/posts/clippy/)
* [Rust Programming #1 - Getting started & basic concepts](https://www.youtube.com/watch?v=2NZlLK6NGmY&list=PLA3GOqJSZytqgPc76nqHc3QAr77bwj3p9)
* [ROFL with a LOL: rewriting an NGINX module in Rust](https://blog.cloudflare.com/rust-nginx-module/)
* [Pretty Rust backtraces in raw terminal mode](https://werat.dev/blog/pretty-rust-backtraces-in-raw-terminal-mode/)
* [A Little Bit of Rust](https://betterprogramming.pub/a-little-bit-of-rust-d9f2afc09238)
* [How Rust and Wasm power Cloudflare's 1.1.1.1](https://blog.cloudflare.com/big-pineapple-intro/)
* [STM32F4 Embedded Rust at the PAC: SysTick Delay](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-systick-delay)
* [Write a First Person Game in 2KB With Rust](https://grantshandy.github.io/posts/raycasting/)

### Rust Walkthroughs
* [Trait Implementations and References](https://www.judy.co.uk/blog/rust-traits-and-references/)
* [Some exercises to accompany the Rust Book](https://www.hyperexponential.com/blog/rust-language-exercises-at-hx)
* [video] [Compression from scratch: Huffman coding tutorial](https://www.youtube.com/watch?v=ek_HJX8sQqk)
* [Create a launchbar app using Tauri & Rust](https://blog.spyglass.fyi/posts/2023/creating-a-launcher-in-tauri/)

### Miscellaneous
* [EuroRust 2023 announced for Oct. 12-13 in Brussels, BE](https://eurorust.eu/)
* [Introducing MailCrab!](https://tweedegolf.nl/en/blog/86/introducing-mailcrab)
* [Rust Nation 2023 Review](https://matt.si/2023-02/rust-nation/)
* [Announcing wasi-threads](https://bytecodealliance.org/articles/wasi-threads)
* [WAI is the Answer !!!](https://wasmer.io/posts/WAI-is-the-answer)
* [video] [Tutorial: Building a Blog in Rust - Static File Server](https://youtube.com/watch?v=9uAy8skUVsc)
* [video] [Embedded-exploration: Debugging Rust start-up code via JTAG](https://www.youtube.com/watch?v=QeiAl35bAIQ)
* [video] [Oxidise Your Life](https://www.youtube.com/watch?v=dFkGNe4oaKk)
* [video] [Improve your Rust APIs with the type state pattern](https://www.youtube.com/watch?v=_ccDqRTx-JU)
* [(Possibly) Emulate PinePhone with Unicorn Emulator](https://lupyuen.github.io/articles/unicorn)
* [Video] [Generic Traits, Impls, and Slices in Rustlang](https://www.youtube.com/watch?v=ykQbsTHqKFo)

## Crate of the Week

This week's crate is [goku](https://github.com/jcaromiq/goku/), a HTTP load tester.

Thanks to [Joaquín Caro](https://users.rust-lang.org/t/crate-of-the-week/2704/1161) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [barricade - Add support for OpenID Connect](https://github.com/purton-tech/barricade/issues/1)
* [config-rs - CFP for the config-rs-ng experiment for a new implementation of the config-rs crate](https://beyermatthias.de/call-for-participation-config-rs-and-config-rs-ng)
* [Ockam - Remove the check_credential arguments from clap command ockam tcp-inlet create](https://github.com/build-trust/ockam/issues/4308)
* [Ockam - Implement clap command ockam tcp-listener delete](https://github.com/build-trust/ockam/issues/4367)
* [Ockam - Improve install.sh to install in path - like rustup](https://github.com/build-trust/ockam/issues/4359)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

381 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-02-20..2023-02-27

* [MIR-Validate StorageLive](https://github.com/rust-lang/rust/pull/108175)
* [`rustc_infer`: consolidate obligation elaboration de-duplication](https://github.com/rust-lang/rust/pull/108424)
* [add an `InstCombine` for redundant casts](https://github.com/rust-lang/rust/pull/108246)
* [add check for invalid `#[macro_export]` arguments](https://github.com/rust-lang/rust/pull/107911)
* [add inlining attributes for query system functions](https://github.com/rust-lang/rust/pull/108375)
* [apply query response: actually define opaque types](https://github.com/rust-lang/rust/pull/108342)
* [ban associated type bounds in bad positions](https://github.com/rust-lang/rust/pull/108063)
* [correctly handle aggregates in DataflowConstProp](https://github.com/rust-lang/rust/pull/108208)
* [diagnostics: if `AssocFn` has self argument, describe as method](https://github.com/rust-lang/rust/pull/108324)
* [do not lint ineffective unstable trait impl for unresolved trait](https://github.com/rust-lang/rust/pull/108449)
* [don't delay `ReError` bug during lexical region resolve](https://github.com/rust-lang/rust/pull/108176)
* [don't project specializable RPITIT projection](https://github.com/rust-lang/rust/pull/108319)
* [don't trigger error for ReError when other region is empty](https://github.com/rust-lang/rust/pull/108502)
* [emit the enum discriminant separately for the Encodable macro](https://github.com/rust-lang/rust/pull/108440)
* [fix `is_terminal`'s handling of long paths on Windows](https://github.com/rust-lang/rust/pull/108391)
* [fix handling of reexported macro in doc hidden items](https://github.com/rust-lang/rust/pull/108241)
* [fix overlapping spans in removing extra arguments](https://github.com/rust-lang/rust/pull/108239)
* [give the resolver access to `TyCtxt`](https://github.com/rust-lang/rust/pull/105462)
* [implement `-Zlink-directives=yes/no`](https://github.com/rust-lang/rust/pull/107675)
* [implement const iterator using `rustc_do_not_const_check`](https://github.com/rust-lang/rust/pull/106541)
* [lint against `Iterator::map` receiving a callable that returns `()`](https://github.com/rust-lang/rust/pull/107890)
* [lint dead code in closures and generators](https://github.com/rust-lang/rust/pull/108315)
* [lint: don't suggest `MaybeUninit::assume_init` for uninhabited types](https://github.com/rust-lang/rust/pull/108000)
* [make object bound candidates sound in the new trait solver](https://github.com/rust-lang/rust/pull/108333)
* [make query keys `Copy`](https://github.com/rust-lang/rust/pull/108169)
* [make sure `test_type_match` doesn't ICE with late-bound types](https://github.com/rust-lang/rust/pull/108202)
* [merge `diagnostic_items` duplicate diagnostics](https://github.com/rust-lang/rust/pull/108486)
* [migrate `rustc_hir_analysis` to session diagnostic Part One](https://github.com/rust-lang/rust/pull/108434)
* [miri: basic dyn* support](https://github.com/rust-lang/rust/pull/107728)
* [move IpAddr, SocketAddr and V4+V6 related types to `core`](https://github.com/rust-lang/rust/pull/104265)
* [parser: provide better suggestions and errors on closures with braces missing](https://github.com/rust-lang/rust/pull/108388)
* [print a backtrace when query forcing fails](https://github.com/rust-lang/rust/pull/91742)
* [rebuild BinaryHeap on unwind from retain](https://github.com/rust-lang/rust/pull/106918)
* [remove dead unwinds before drop elaboration](https://github.com/rust-lang/rust/pull/106430)
* [rustdoc: avoid including `<li>` tags in item table short desc](https://github.com/rust-lang/rust/pull/108410)
* [treat `str` as containing `[u8]` for auto trait purposes](https://github.com/rust-lang/rust/pull/107941)
* [use `ThinVec` more in the AST](https://github.com/rust-lang/rust/pull/104754)
* [use a lock-free datastructure for `source_span`](https://github.com/rust-lang/rust/pull/108300)
* [miri: get Miri working on ARM](https://github.com/rust-lang/miri/pull/2798)
* [hashbrown: `raw_table` + `raw_table_mut`](https://github.com/rust-lang/hashbrown/pull/404)
* [hashbrown: fix last bug in `RawTable::clone_from_impl`](https://github.com/rust-lang/hashbrown/pull/407)
* [codegen\_gcc: simd scatter gather](https://github.com/rust-lang/rustc_codegen_gcc/pull/254)
* [cargo: addition of support for -F as an alias for --features](https://github.com/rust-lang/cargo/pull/11774)
* [cargo: error message for transitive artifact dependencies with targets the package doesn't directly interact with](https://github.com/rust-lang/cargo/pull/11643)
* [cargo: fix Cargo removing the sparse+ prefix from sparse URLs in .crates.toml](https://github.com/rust-lang/cargo/pull/11756)
* [cargo: reuse url encoding from `url` crate, don't use separate `percent-encoding`](https://github.com/rust-lang/cargo/pull/11750)
* [cargo: suggest cargo add when installing library crate](https://github.com/rust-lang/cargo/pull/11410)
* [clippy: add `impl_trait_in_params` lint](https://github.com/rust-lang/rust-clippy/pull/10197)
* [clippy: add new lint `no_mangle_with_rust_abi`](https://github.com/rust-lang/rust-clippy/pull/10369)
* [clippy: add configuration to lint missing docs of `pub(crate)` items](https://github.com/rust-lang/rust-clippy/pull/10303)
* [clippy: do not panic when analyzing the malformed origin of a format string](https://github.com/rust-lang/rust-clippy/pull/10401)
* [clippy: do not suggest to derive `Default` on generics with implicit arguments](https://github.com/rust-lang/rust-clippy/pull/10399)
* [clippy: do not suggest using Self in const generic parameters](https://github.com/rust-lang/rust-clippy/pull/10375)
* [clippy: fix more false positives for `extra_unused_type_parameters`](https://github.com/rust-lang/rust-clippy/pull/10392)
* [clippy: fix test function checker in `unwrap_used`, `expect_used`](https://github.com/rust-lang/rust-clippy/pull/10391)
* [clippy: ignore lifetimes from differing contexts in `needless_lifetimes`](https://github.com/rust-lang/rust-clippy/pull/10380)
* [clippy: normalize projections types when checking `explicit_auto_deref`](https://github.com/rust-lang/rust-clippy/pull/10386)
* [rust-analyzer: add openDocs command to context menu in VS Code extension](https://github.com/rust-lang/rust-analyzer/pull/14175)
* [rust-analyzer: add check for extra path segments after a fully qualified one](https://github.com/rust-lang/rust-analyzer/pull/14203)
* [rust-analyzer: add a case in which remainig is None in resolveing types when resolving hir path](https://github.com/rust-lang/rust-analyzer/pull/14208)
* [rust-analyzer: respect `$CARGO_HOME` when looking up toolchains](https://github.com/rust-lang/rust-analyzer/pull/14207)
* [fix outdated doc](https://github.com/rust-lang/ena/pull/37)
* [rust-installer: adjust xz compression settings](https://github.com/rust-lang/rust-installer/pull/123)

### Rust Compiler Performance Triage

Some noisy benchmarks impeded performance review this week. There was a notable
improvement to a broad range of primary benchmarks, first from PR #108440, which
revised the encodable proc macro to handle the discriminant separately from its
fields, and second from PR #108375, which inlined a number of methods that had
only a single caller. Both of these PR's were authored by the same contributor;
many thanks Zoxc!

Triage done by **@pnkfelix**.
Revision range: [3fee48c1..31f858d9](https://perf.rust-lang.org/?start=3fee48c161a48b0c142d3998fff56faee96bd56c&end=31f858d9a511f24fedb8ed997b28304fec809630&absolute=false&stat=instructions%3Au)

5 Regressions, 4 Improvements, 6 Mixed; 6 of them in rollups
39 artifact comparisons made in total

[Full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-02-28.md) 
 
### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Move std::net::IpAddr types into core::net.](https://github.com/rust-lang/rfcs/pull/2832)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [Rename {Option, Result}::expect to unwrap_or_panic](https://github.com/rust-lang/rfcs/pull/3218)
* [disposition: close] [take on bool](https://github.com/rust-lang/rfcs/pull/3189)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [io: soften ‘at most one write attempt’ requirement in io::Write::write](https://github.com/rust-lang/rust/pull/107200)
* [disposition: merge] [Make `unused_allocation` lint against `Box::new` too](https://github.com/rust-lang/rust/pull/104363)
* [disposition: merge] [Properly allow macro expanded format_args invocations to uses captures](https://github.com/rust-lang/rust/pull/106505)
* [disposition: merge] [Add `NonZero{I,U}{8,16,32,64,128,size}::{MIN,MAX}`](https://github.com/rust-lang/rust/issues/89065)
* [disposition: merge] [allow negative numeric literals in concat!](https://github.com/rust-lang/rust/pull/106844)
* [disposition: merge] [Tracking issue for atomic_mut_ptr](https://github.com/rust-lang/rust/issues/66893)
* [disposition: merge] [Add documentation about the memory layout of `Cell`](https://github.com/rust-lang/rust/pull/106921)
* [disposition: merge] [Tracking issue for `PathBuf::as_mut_os_string` and `Path::as_mut_os_str`](https://github.com/rust-lang/rust/issues/105021)
* [disposition: merge] [Use `partial_cmp` to implement tuple `lt`/`le`/`ge`/`gt`](https://github.com/rust-lang/rust/pull/108157)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Create RFC for bundling local images in rustdoc output](https://github.com/rust-lang/rfcs/pull/3397)
* [new] [Extern types v2](https://github.com/rust-lang/rfcs/pull/3396)
* [new] [format-dynamic-fill](https://github.com/rust-lang/rfcs/pull/3394)
* [new] [Add RFC on governance, establishing the Leadership Council](https://github.com/rust-lang/rfcs/pull/3392)
* [new] [RFC: result_ffi_guarantees](https://github.com/rust-lang/rfcs/pull/3391)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-03-01 - 2023-03-29 🦀

### Virtual

* 2023-03-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Michael Baykov on Category Theory & Argument Parsing**](https://www.meetup.com/indyrs/events/qwtdjsyfcfbcb/)
* 2023-03-02 | Virtual (Raleigh, NC, US) | [Triangle BitDevs](https://www.meetup.com/triangle-bitdevs/)
    * [**Rust for Bitcoiners**](https://www.meetup.com/triangle-bitdevs/events/291710295/)
* 2023-03-02 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 30th Edition**](https://www.meetup.com/rust-linz/events/291483339/)
* 2023-03-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcfbkb/)
* 2023-03-07 | Virtual (Santa Clara, CA, US) | [UCSC Extension Community](https://www.meetup.com/ucsc-extension-community/)
    * [**Programming with Rust**](https://www.meetup.com/ucsc-extension-community/events/290906954/)
* 2023-03-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcfblb/)
* 2023-03-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-03-14 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2023/03/14/rust-hack-and-learn.html)
* 2023-03-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/291809763/)
* 2023-03-14 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep1**](https://www.meetup.com/microsoft-reactor-redmond/events/291676352/)
* 2023-03-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Injecting Rust Hooks into a 1999 game binary (unsafe)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291354288/)
* 2023-03-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/wqchctyfcfbtb/)
* 2023-03-16 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Introduction to WebAssembly (WASM) with Rust and WASMEdge**](https://www.meetup.com/microsoft-reactor-redmond/events/291681809/)
* 2023-03-16 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/291847774/)
* 2023-03-21 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep2**](https://www.meetup.com/microsoft-reactor-redmond/events/291676961/)
* 2023-03-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Rust+Tell Lightning Talks**](https://www.meetup.com/rustdc/events/vdhxgsyfcfbcc/)
* 2023-03-22 | Virtual (Richmond, VA, US) | [Rustaceans RVA](https://www.meetup.com/rustaceans-rva/)
    * [**Rustaceans RVA - March Meetup**](https://www.meetup.com/rustaceans-rva/events/291963911/)
* 2023-03-28 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep3**](https://www.meetup.com/microsoft-reactor-redmond/events/291677113/)
* 2023-03-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Writing your own rust 'book' with mdBook**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291892487/)

### Asia

* 2023-03-04 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Fn vs FnMut vs FnOnce**](https://www.meetup.com/kansai-rust/events/291614614/)

### Europe

* 2023-03-01 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Rust traits for Fn and profit**](https://www.meetup.com/rustcologne/events/291774935/)
* 2023-03-02 | Barcelona, ES | [BcnRust](https://bcnrust.github.io/)
    * [**9th BcnRust Meetup: Full Stack**](https://www.meetup.com/es-ES/bcnrust/events/291754590/)
* 2023-03-02 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #32**](https://www.meetup.com/rust-wroclaw/events/291776357/)
* 2023-03-07 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/291657555/)   
* 2023-03-09 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Rust Meetup #7**](https://www.meetup.com/rust-basel/events/291228934/)
* 2023-03-09 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401965/)
    * [**Student track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401778/)
* 2023-03-09 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #2**](https://www.meetup.com/fr-FR/rust-lyon/events/291727241/)
* 2023-03-15 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk around Embedded World Exhibition**](https://www.meetup.com/rust-noris/events/291623203/)
* 2023-03-17 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfcfbwb/)
* 2023-03-28 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
    * [**High performance concurrent data structures in Rust - March Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)

### North America

* 2023-03-01 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/291619816/)
* 2023-03-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Trails, Triumphs, & Travails of Yet-Another-Database-Crate with PJ and Food!**](https://www.meetup.com/utah-rust/events/rrwbctyfcfbmb/)

### Oceania

* 2023-03-01 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - We are back!**](https://www.meetup.com/rust-sydney/events/291265163/)


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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/10nmtew/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> You've probably come across unsafe. So "unsafe" is a keyword that sort of unlocks super powers and segfaults.

– [Arthur Cohen during FOSDEM '23](https://fosdem.org/2023/schedule/event/rust_a_deep_dive_inside_the_rust_frontend_for_gcc/)

Thanks to [blonk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1375) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/11g1wkc/this_week_in_rust_484/)</small>
