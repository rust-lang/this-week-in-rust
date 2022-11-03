Title: This Week in Rust 466
Number: 466
Date: 2022-10-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
### Foundation
* [Implementing the Network Time Protocol (NTP) in Rust](https://foundation.rust-lang.org/news/implementing-the-network-time-protocol-ntp-in-rust/)

### Project/Tooling Updates
* [rust-analyzer changelog #152](https://rust-analyzer.github.io/thisweek/2022/10/24/changelog-152.html)
* [IntelliJ Rust Changelog #181](https://intellij-rust.github.io/2022/10/24/changelog-181.html)
* [mirrord 3.0 is out!](https://metalbear.co/blog/mirrord-3.0-is-out/)
* [gix credential and diffing with similar](https://github.com/Byron/gitoxide/discussions/564)
* [New release – gtk-rs](https://gtk-rs.org/blog/2022/10/18/new-release.html)
* [Zellij 0.32.0: YAML => KDL, actions through CLI, command panes and a new layout system](https://zellij.dev/news/config-command-layouts/)
* [RPITs, RPITITs and AFITs and their relationship](https://santiagopastorino.com/2022/10/20/what-rpits-rpitits-and-afits-and-their-relationship)
* [Evaluating Build Scripts in the IntelliJ Rust Plugin](https://blog.jetbrains.com/rust/2022/10/24/evaluating-build-scripts-in-the-intellij-rust-plugin/)

### Observations/Thoughts
* [Adding Ada to Rust](https://blog.adacore.com/adding-ada-to-rust)
* [Adding a JavaScript interpreter to your Rust project](https://boa-dev.github.io/posts/2022-10-24-boa-usage/)
* [Rust in the Linux Kernel: Just the Beginning](https://www.memorysafety.org/blog/rust-in-linux-just-the-beginning/)
* [A deeper look into the GCC Rust front-end](https://lwn.net/Articles/909887/)
* [The HTTP crash course nobody asked for](https://fasterthanli.me/articles/the-http-crash-course-nobody-asked-for)
* [Making Rust attractive for writing GTK applications](https://belmoussaoui.com/blog/15-making-rust-attractive-for-writing-gtk-applications/)
* [Adventures In Cross Compilation](https://kentiklabs.com/blog/adventures-in-cross-compilation/)
* [Compiling Brainfuck code - Part 1: A Optimized Interpreter](https://rodrigodd.github.io/2022/10/21/bf_compiler-part1.html)
* [Rust Embedded Graphics with the MAX7219](https://apollolabsblog.hashnode.dev/rust-embedded-graphics-with-the-max7219)
* [Buffers on the edge: Python and Rust · Alex Gaynor](https://alexgaynor.net/2022/oct/23/buffers-on-the-edge/)
* [Writing Better Integration Tests with RAII](https://notado.substack.com/p/writing-better-integration-tests)
* [Contention on multi-threaded regex matching](https://morestina.net/blog/1827/multi-threaded-regex)

### Rust Walkthroughs
* [Serde by Example 2: OpenStreetMap](https://blog.dzejkop.space/posts/serde-by-example-2.html)
* [Enums and Pattern Matching in Rust](https://serokell.io/blog/enums-and-pattern-matching)
* [Creating a minimal RESTful song request API using Rocket](https://imajindevon.hashnode.dev/rust-rocket-song-request-api)
* [Compiling Rust libraries for Android apps: a deep dive](https://gendignoux.com/blog/2022/10/24/rust-library-android.html)
* [Inline Crates](https://blog.yoshuawuyts.com/inline-crates/)
* [Writing a HashMap in Rust without unsafe](https://ecton.dev/writing-a-hashmap-without-unsafe/)
* [A Rust web app with HTML templates](https://woile.dev/posts/web-app-with-template-in-rust/)
* [Nine Rules for Creating Procedural Macros in Rust: Practical Lessons from anyinput, a New Macro for Easily Accepting String/Path/Iterator/Array-Like Inputs](https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff)
* [series] [Sqlite File Parser Pt 4](https://freemasen.com/blog/sqlite-parser-pt4/index.html)
* [MacroKata: Rustlings style exercises for learning macros](https://tfpk.github.io/macrokata/)

### Miscellaneous
* [video] [Web-native Rust apps (what will YOU build?)](https://www.youtube.com/watch?v=y10jJX35shE)
* [video] [Rapid Prototyping in Rust: Write fast like Python; Run fast like C](https://www.youtube.com/watch?v=X7Dsu0oRk0U)
* [video] [Let our rusty crab explore the depths of the C by Yvan Sraka](https://www.youtube.com/watch?v=GnrtYcqPOI8)
* [video] [Case Study: Rust in axle OS by Philip Tennen](https://www.youtube.com/watch?v=OAGuoKa1Gm8)
* [video] [Aya: Extending the Linux Kernel with eBPF and Rust by Michal Rostecki](https://www.youtube.com/watch?v=t996ZkbWnEs)
* [video] [Building a Lightweight IR and Backend for YJIT / Maxime Chevalier-Boisvert](https://www.youtube.com/watch?v=BbLGqTxTRp0)
* [video] [RustcContributor::explore: @compiler-errors session - RPITIT deep dive](https://www.youtube.com/watch?v=TGxio7A0SfI)
* [Step-by-step guide to building a web-crawler](https://www.youtube.com/watch?v=oZa6UY9gzLw)
* [SQLx in 12 minutes - Rust + Actix Web + PostgreSQL](https://www.youtube.com/watch?v=vLcoW408QvE)
* [Bevy Basics Scenes(re-upload)](https://www.youtube.com/watch?v=bbBh3oKibkE)

## Crate of the Week

This week's crate is [humantime](https://lib.rs/crates/humantime), a parser and formatter for `std::time::`{`Duration`, `SystemTime`}.

Thanks to [Aleksey Kladov](https://users.rust-lang.org/t/crate-of-the-week/2704/1116) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [zerocopy - Optimize caching in CI](https://github.com/google/zerocopy/issues/85)
* [boa - hacktoberfest issues](https://github.com/boa-dev/boa/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22%2CHacktoberfest%2CE-Easy%2C%22good+first+issue%22+no%3Aassignee)
* [boa - bugs that cause boa to panic](https://github.com/boa-dev/boa/issues?q=is%3Aissue+is%3Aopen+panic+no%3Aassignee)
* [Ockam - Show "help" output when no args passed on subscription show clap command](https://github.com/build-trust/ockam/issues/3739)
* [Ockam - Add argument to node create clap command to terminate on EOF on STDIN](https://github.com/build-trust/ockam/issues/3701)
* [Ockam - Extract duplicated code into a shared helper function](https://github.com/build-trust/ockam/issues/3742)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

398 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-10-17..2022-10-24

* [linker: fix weak lang item linking with combination windows-gnu + LLD + LTO](https://github.com/rust-lang/rust/pull/103092)
* [recover when unclosed char literal is parsed as a lifetime in some positions](https://github.com/rust-lang/rust/pull/101293)
* [allow `#[unstable]` impls for fn() with unstable abi](https://github.com/rust-lang/rust/pull/103239)
* [allow `Vec::leak` when using `no_global_oom_handling`](https://github.com/rust-lang/rust/pull/103153)
* [allow semicolon after closure within parentheses in macros](https://github.com/rust-lang/rust/pull/103224)
* [change `unknown_lint` applicability to `MaybeIncorrect`](https://github.com/rust-lang/rust/pull/103399)
* [require `Drop` impls to have the same constness on its bounds as the bounds on the struct have](https://github.com/rust-lang/rust/pull/103351)
* [require lifetime bounds for opaque types in order to allow hidden types to capture said lifetimes](https://github.com/rust-lang/rust/pull/102417)
* [add default trait implementations for "c-unwind" ABI function pointers](https://github.com/rust-lang/rust/pull/101263)
* [filtering spans when emitting json](https://github.com/rust-lang/rust/pull/102922)
* [suggest let for assignment, and some code refactor](https://github.com/rust-lang/rust/pull/101908)
* [do not suggest trivially false const predicates](https://github.com/rust-lang/rust/pull/103328)
* [standardize "use parentheses to call" suggestions between typeck and trait selection](https://github.com/rust-lang/rust/pull/102863)
* [escape string literals when fixing overlong char literal](https://github.com/rust-lang/rust/pull/103354)
* [handle return-position `impl Trait` in traits properly in `register_hidden_type`](https://github.com/rust-lang/rust/pull/103355)
* [improve "`~const` is not allowed here" message](https://github.com/rust-lang/rust/pull/103319)
* [add diagnostic for calling a function with the same name with unresolved Macro](https://github.com/rust-lang/rust/pull/103140)
* [chalk: consider ADT's generic parameters](https://github.com/rust-lang/chalk/pull/781)
* [miri: fix ICE when trying to GC a Stack with an unknown bottom](https://github.com/rust-lang/miri/pull/2600)
* [miri: add `scalar-abi-only` field retagging option](https://github.com/rust-lang/miri/pull/2613)
* [erase regions before checking for `Default` in uninitialized binding error](https://github.com/rust-lang/rust/pull/103276)
* [introduce deduced parameter attributes, and use them for deducing `readonly` on indirect immutable freeze by-value function parameters](https://github.com/rust-lang/rust/pull/103172)
* [let expressions on RHS shouldn't be terminating scopes](https://github.com/rust-lang/rust/pull/103034)
* [make diagnostic for unsatisfied `Termination` bounds more precise](https://github.com/rust-lang/rust/pull/103142)
* [make `order_dependent_trait_objects` show up in future-breakage reports](https://github.com/rust-lang/rust/pull/102635)
* [reduce false positives in msys2 detection](https://github.com/rust-lang/rust/pull/103360)
* [enable LTO for `rustc_driver.so`](https://github.com/rust-lang/rust/pull/101403)
* [remove byte swap of valtree hash on big endian](https://github.com/rust-lang/rust/pull/103231)
* [remove more attributes from metadata](https://github.com/rust-lang/rust/pull/98450)
* [use Set instead of Vec in `transitive_relation`](https://github.com/rust-lang/rust/pull/103214)
* [sort tests at compile time, not at startup](https://github.com/rust-lang/rust/pull/99939)
* [use already checked RHS ty for LHS deref suggestions](https://github.com/rust-lang/rust/pull/103223)
* [stabilize `proc_macro::Span::source_text`](https://github.com/rust-lang/rust/pull/103197)
* [stabilize `duration_checked_float`](https://github.com/rust-lang/rust/pull/102271)
* [stabilize `asm_sym`](https://github.com/rust-lang/rust/pull/103168)
* [make transpose const and inline](https://github.com/rust-lang/rust/pull/103127)
* [mark `std::os::wasi::io::AsFd` etc. as stable](https://github.com/rust-lang/rust/pull/103308)
* [eliminate 280-byte memset from `ReadDir` iterator](https://github.com/rust-lang/rust/pull/103137)
* [optimize `slice_iter.copied().next_chunk()`](https://github.com/rust-lang/rust/pull/103166)
* [implement `String::leak`](https://github.com/rust-lang/rust/pull/103280)
* [adjust argument type for mutable `with_metadata_of`](https://github.com/rust-lang/rust/pull/103346)
* [hashbrown: add support for 16-bit targets](https://github.com/rust-lang/hashbrown/pull/368)
* [futures: do not store items field in `ReadyChunks`](https://github.com/rust-lang/futures-rs/pull/2656)
* [cargo: fix publishing with a dependency on a sparse registry](https://github.com/rust-lang/cargo/pull/11268)
* [cargo: improve the error message if `publish` is `false` or empty list](https://github.com/rust-lang/cargo/pull/11280)
* [cargo: publish: check remote git registry more than once post-publish](https://github.com/rust-lang/cargo/pull/11255)
* [rustdoc: eliminate uses of `EarlyDocLinkResolver::all_traits`](https://github.com/rust-lang/rust/pull/103192)
* [rustdoc: do not filter out cross-crate `Self: Sized` bounds](https://github.com/rust-lang/rust/pull/103254)
* [crates.io: introduce daily limit of published versions per crate](https://github.com/rust-lang/crates.io/pull/5294)
* [docs.rs: perf: change the link in the topbar to avoid a redirect](https://github.com/rust-lang/docs.rs/pull/1887)
* [bindgen: avoid suppressing panic messages](https://github.com/rust-lang/rust-bindgen/pull/2323)
* [bindgen: use panic hooks instead of using `catch_unwind`](https://github.com/rust-lang/rust-bindgen/pull/2317)
* [clippy: add `missing_trait_methods` lint](https://github.com/rust-lang/rust-clippy/pull/9670)
* [clippy: add lint to tell about let else pattern](https://github.com/rust-lang/rust-clippy/pull/8437)
* [clippy: enable test `no_std_main_recursion`](https://github.com/rust-lang/rust-clippy/pull/9654)
* [clippy: fix `allow_attributes_without_reason` applying to external crate macros](https://github.com/rust-lang/rust-clippy/pull/9630)
* [clippy: fix ICE due to out-of-bounds array access](https://github.com/rust-lang/rust-clippy/pull/9635)
* [clippy: improvement for  `equatable_if_let`](https://github.com/rust-lang/rust-clippy/pull/9368)
* [clippy: `collapsible_match` specify field name when destructuring structs](https://github.com/rust-lang/rust-clippy/pull/9685)
* [clippy: `unwrap_used`, `expect_used` do not lint in `test` cfg](https://github.com/rust-lang/rust-clippy/pull/9686)
* [clippy: `ref_option_ref` do not lint when inner reference is mutable](https://github.com/rust-lang/rust-clippy/pull/9684)
* [clippy: add `from_raw_with_void_ptr` lint](https://github.com/rust-lang/rust-clippy/pull/9690)
* [clippy: fix `box-default` ignoring trait objects' types](https://github.com/rust-lang/rust-clippy/pull/9622)
* [clippy: support `map_or` for `or_fun_call` lint](https://github.com/rust-lang/rust-clippy/pull/9689)
* [rust-analyzer: support const generics for builtin derive macro](https://github.com/rust-lang/rust-analyzer/pull/13463)
* [rust-analyzer: workaround the python vscode extension's polyfill](https://github.com/rust-lang/rust-analyzer/pull/13448)
* [rust-analyzer: add multiple getters mode in `generate_getter`](https://github.com/rust-lang/rust-analyzer/pull/13365)
* [rust-analyzer: don't catch the server activation error](https://github.com/rust-lang/rust-analyzer/pull/13444)
* [rust-analyzer: don't respond with an error when requesting a shutdown while starting](https://github.com/rust-lang/rust-analyzer/pull/13476)
* [rust-analyzer: fix `DidSaveDocument` requests blocking the server on startup](https://github.com/rust-lang/rust-analyzer/pull/13447)
* [rust-analyzer: fix standard flycheck command not being executed in the workspace it is being invoked for](https://github.com/rust-lang/rust-analyzer/pull/13478)
* [rust-analyzer: handle multiple projects sharing dependency correctly in `once` strategy](https://github.com/rust-lang/rust-analyzer/pull/13471)

### Rust Compiler Performance Triage

An amazing week. We saw more wins than losses; I want to call out specifically
the wins from removing attributes from metadata (up to 8.2% faster builds for 18
benchmarks) and from enabling LTO for rustc_driver.so (up to 9.6% faster builds
for an epic 230 benchmarks, with *zero* regressions).

Triage done by **@pnkfelix**.
Revision range: [e0f8e60d..629a414d](https://perf.rust-lang.org/?start=e0f8e60dddfecfc9093ee9d9f42557d8260c0355&end=629a414d7ba4caa3ca28b0a46c478e2ecb4c0059&absolute=false&stat=instructions%3Au)

2 Regressions, 6 Improvements, 2 Mixed; 2 of them in rollups
53 artifact comparisons made in total

See [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-10-26.md) for details.

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Make PROC_MACRO_DERIVE_RESOLUTION_FALLBACK a hard error](https://github.com/rust-lang/rust/pull/84022)
* [disposition: merge] [Elaborate supertrait obligations when deducing closure signatures](https://github.com/rust-lang/rust/pull/101834)
* [disposition: merge] [Tracking Issue for Integer::{ilog,ilog2,ilog10}](https://github.com/rust-lang/rust/issues/70887)
* [disposition: close] [Propagate deref coercion into block](https://github.com/rust-lang/rust/pull/83850)
* [disposition: merge] [Derive `Eq` and `Hash` for `ControlFlow`](https://github.com/rust-lang/rust/pull/103084)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Warning on unintended implicit drops](https://github.com/rust-lang/rfcs/pull/3335)
* [new] [Niches](https://github.com/rust-lang/rfcs/pull/3334)
* [new] [Deprecate PhantomData dropck](https://github.com/rust-lang/rfcs/pull/3331)

## Upcoming Events

Rusty Events between 2022-10-26 - 2022-11-23 🦀

### Virtual

* 2022-10-26 | Virtual (Redmond, WA, US / New York, NY, US / Toronto, CA / Stockholm, SE / London, UK) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Your First Rust Project: Rust Basics**](https://www.meetup.com/microsoft-reactor-redmond/events/288475815/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475839/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475818/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/288475819/) | [**London Mirror**](https://www.meetup.com/microsoft-reactor-london/events/288475821/)
* 2022-10-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Using Applicative Functors to parse command line options**](https://www.meetup.com/charlottesville-rust-meetup/events/288867237/)
* 2022-10-27 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA) (various topics, from C++ to Rust...)**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/288972651/)
* 2022-10-27 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 26th Edition**](https://www.meetup.com/rust-linz/events/289212637/)
* 2022-10-29 | Virtual (Ludwigslust, DE) | [Ludwigslust Rust Meetup](https://www.meetup.com/ludwigslust-rust-meetup/)
    * [**Von Nullen und Einsen | Rust Meetup Ludwigslust #1**](https://www.meetup.com/ludwigslust-rust-meetup/events/289168194/)
* 2022-11-01 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/events/289066646/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/289066646/)
* 2022-11-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcpbcb/)
* 2022-11-02 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust and C++ Cardiff Virtual Meet**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289052285/)
* 2022-11-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcpbdb/)
* 2022-11-02 | Virtual (Redmond, WA, US / San Francisco, SF, US / New York, NY, US / Toronto, CA / London, UK) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Getting Started with Rust: From Java Dev to Rust Developer**](https://www.meetup.com/microsoft-reactor-redmond/events/288475833/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475838/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475839/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475836/) | [**London Mirror**](https://www.meetup.com/microsoft-reactor-london/events/288475832/) 
* 2022-11-02 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/289092511/)
* 2022-11-08 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289211414/)
* 2022-11-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcpblb/)
* 2022-11-08 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/289352420/)
* 2022-11-08 | Virtual (Stockholm, SE) | [Func Prog Sweden](https://www.meetup.com/func-prog-sweden/)
    * [**Tenth Func Prog Sweden MeetUp 2022 – Online (with "Ready for Rust" by Erik Dörnenburg)**](https://www.meetup.com/func-prog-sweden/events/288323896/)
* 2022-11-09 | Virtual (Malaysia, MY) | [Rust Malaysia](https://forms.gle/zWXcMDAnnibiL4ni9)
    * [**Rust Meetup November 2022 - a couple of lightning talks**](https://discord.gg/9Xj8H2EXTD)
* 2022-11-10 | Virtual (Budapest, HU) | [HWSW free!](https://www.meetup.com/hwswfree/)
    * [**RUST! RUST! RUST! meetup (online formában!)**](https://www.meetup.com/hwswfree/events/289044458/)
* 2022-11-12 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-11-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc//)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/289015883/)
* 2022-11-15 | Virtual (Nairobi, KE / New York, NY, US)| [Data Umbrella Africa](https://www.meetup.com/data-umbrella-africa2/)
    * [**Online: Introduction to Rust Programming**](https://www.meetup.com/data-umbrella-africa2/events/289308825/) | [**New York Mirror**](https://www.meetup.com/data-umbrella/events/289308172/)
* 2022-11-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcpbvb/)
* 2022-11-17 | Virtual (Amsterdam, NL) | [ITGilde Tech-Talks](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups)
    * [**Introduction “Rust” an ITGilde Tech Talk delivered by Pascal van Dam**](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups/events/289167373/)
* 2022-11-21 | Virtual (Paris, FR) | [Meetup Paris - École Supérieure de Génie Informatique (ESGI)](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique)
    * [**Découverte de WebAssembly**](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique/events/289112753/)

### Asia

* 2022-11-08 | Bangkok, TH | [Tech@Agoda](https://www.meetup.com/techatagoda/)
    * [**Rustacean Bangkok 5.0.0**](https://www.meetup.com/techatagoda/events/289329464/)

### Europe

* 2022-10-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks October 2022: Host by Amazon Prime Video**](https://www.meetup.com/rust-london-user-group/events/289023932/)
* 2022-10-26 | Bristol, UK | [Rust and C++ Cardiff/Rust Bristol](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Programming Veloren & Rust for a living**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289204085/)
* 2022-10-27 | København, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #30**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179125/)
* 2022-11-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Initial Meet and Greet Rust meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/289028178/)    
### North America

* 2022-10-27 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Bevy Crash Course with Nathan and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsydcnbkc/)
* 2022-11-10 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcpbnb/)

### Oceania

* 2022-11-09 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**RustAU Sydney - Last physical for 2022 !**](https://www.meetup.com/rust-sydney/events/289061840/)
* 2022-11-22 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/288615873/)

### South America

* 2022-11-05 | São Paulo, SP, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/)
    * [**Rust-SP meetup Outubro 2022**](https://www.meetup.com/rust-sao-paulo-meetup/events/289176073/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/xldzbl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Also, I don't know how much of this is because Rust is special or because BurntSushi is a national treasure and his CSV library is impeccably constructed and documented.

– [Gabe Durazo on github](https://github.com/losvedir/transit-lang-cmp/#rust)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1319) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/yeghtr/this_week_in_rust_466/)</small>
