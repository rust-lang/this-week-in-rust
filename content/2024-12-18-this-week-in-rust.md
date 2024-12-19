Title: This Week in Rust 578
Number: 578
Date: 2024-12-18
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
* [November project goals update](https://blog.rust-lang.org/2024/12/16/project-goals-nov-update.html)
* [This Development-cycle in Cargo: 1.84](https://blog.rust-lang.org/inside-rust/2024/12/13/this-development-cycle-in-cargo-1.84.html)
* [December 2024 Project Director Update](https://blog.rust-lang.org/inside-rust/2024/12/17/project-director-update.html)

### Newsletters
* [This Month in Rust OSDev: November 2024](https://rust-osdev.com/this-month/2024-11/)

### Project/Tooling Updates
* [hyper Roadmap 2025](https://seanmonstar.com/blog/hyper-roadmap-2025/)
* [Bevy Fallible Systems, Bindless, and Immutable Components](https://thisweekinbevy.com/issue/2024-12-09-fallible-systems-bindless-and-immutable-components)
* [Sequoia PGP: A Sapling Matures: Meet sq 1.0](https://sequoia-pgp.org/blog/2024/12/16/202412-sq-1.0/)
* [Release 0.30.0 - egui_kittest and modals](https://github.com/emilk/egui/releases/tag/0.30.0)
* [fish-shell 4.0b1, now in Rust](https://fishshell.com/blog/fish-4b/)
* [Introducing Limbo: A complete rewrite of SQLite in Rust](https://turso.tech/blog/introducing-limbo-a-complete-rewrite-of-sqlite-in-rust)
* [Streaming data analytics, Fluvio 0.14.0 release](https://www.fluvio.io/news/this-week-in-fluvio-0067)
* [Announcing Hex Display: A modern `xxd` alternative](https://users.rust-lang.org/t/announcing-hex-display-a-modern-alternative-to-xxd/122523)
* [Diesel: Window functions](https://blog.weiznich.de/blog/diesel-nl-net-grant-window-functions/)
* [Rerun 0.21.0 - Graph view, 3D Grid & UI/UX improvements](https://rerun.io/blog/graphs)
* [Introducing Fjädra — a Rust port of `d3-force` for graph layouts](https://github.com/grtlr/fjadra)

### Observations/Thoughts
* [What are temporal and spatial memory safety?](https://blog.yoshuawuyts.com/temporal-spatial-memory-safety/)
* [Reducing WASM binary size](https://www.warp.dev/blog/reducing-wasm-binary-size)
* [Crash recovery in 256 bytes](https://cliffle.com/blog/exhubris-super/)
* [Rust crate feature debugging](https://rustunit.com/blog/2024/12-16-rust-feature-debugging/)
* [audio] [Building Rust: An interview with Nell Shamrell-Harrington](https://timclicks.dev/podcast/nell-shamrell-harrington)

### Rust Walkthroughs
* [Read the Code: Using Drop Safely in Rust](https://v5.chriskrycho.com/journal/read-the-code/using-drop-safely-in-rust/)
* [Thoughts on Rust hashing](https://purplesyringa.moe/blog/thoughts-on-rust-hashing/)
* [Scheme to the Spec Part I: Concurrent Cycle Collection](https://maplant.com/2024-12-13-Scheme-to-the-Spec-Part-I:-Concurrent-Cycle-Collection.html)
* [Solving Advent of Code at Compile Time with Rust Macros](https://doublefree.bearblog.dev/solving-advent-of-code-at-compile-time-with-rust-macros/)
* [video] [Rust code reading: The thousands crate](https://www.youtube.com/watch?v=ITTj7ByNStE)
* [video] [Building a Lua package manager in Rust (Arabic)](https://youtu.be/YsupdHTAKDw)

### Miscellaneous
* [What is shift-left ⬅️ programming?](https://dev.to/szabgab/what-is-shift-left-programming-5601)
* [Rust social status update 2024.12](https://rust.code-maven.com/rust-update-2024-12-17)
* [So many tokens, so little time: Introducing a faster, more flexible byte-pair tokenizer](https://github.blog/ai-and-ml/llms/so-many-tokens-so-little-time-introducing-a-faster-more-flexible-byte-pair-tokenizer/)

## Crate of the Week

This week's crate is [cmd_lib](https://crates.io/crates/cmd_lib), a library of command-line macros and utilities to write shell-script like tasks easily in Rust.

Thanks to [Remo Senekowitsch](https://users.rust-lang.org/t/crate-of-the-week/2704/1382) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

437 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-10..2024-12-17

* [(Re-)Implement `impl_trait_in_bindings`](https://github.com/rust-lang/rust/pull/134185)
* [`rustc_borrowck`: Stop suggesting the invalid syntax `&mut raw const`](https://github.com/rust-lang/rust/pull/134244)
* [`rustc_mir_dataflow` cleanups, including some renamings](https://github.com/rust-lang/rust/pull/133938)
* [`rustc_target`: ppc64 target string fixes for LLVM 20](https://github.com/rust-lang/rust/pull/134115)
* [add AST support for unsafe binders](https://github.com/rust-lang/rust/pull/134140)
* [add unpolished, experimental support for AFIDT (async fn in dyn trait)](https://github.com/rust-lang/rust/pull/133122)
* [allow `symbol_intern_string_literal` lint in test modules](https://github.com/rust-lang/rust/pull/134173)
* [arbitrary self types v2: main compiler changes](https://github.com/rust-lang/rust/pull/132961)
* [change `GetManyMutError` to match T-libs-api decision](https://github.com/rust-lang/rust/pull/133598)
* [check for array lengths that aren't actually `usize`](https://github.com/rust-lang/rust/pull/134371)
* [codegen `#[naked]` functions using global asm](https://github.com/rust-lang/rust/pull/128004)
* [correctly handle comments in attributes in doctests source code](https://github.com/rust-lang/rust/pull/134260)
* [don't ICE when encountering never in range pattern](https://github.com/rust-lang/rust/pull/134103)
* [don't make a def id for `impl_trait_in_bindings`](https://github.com/rust-lang/rust/pull/134313)
* [don't show the full linker args unless `--verbose` is passed](https://github.com/rust-lang/rust/pull/133633)
* [don't use `AsyncFnOnce::CallOnceFuture` bounds for signature deduction](https://github.com/rust-lang/rust/pull/134017)
* [encode coroutine-closures in SMIR](https://github.com/rust-lang/rust/pull/134295)
* [exercise const trait interaction with default fields](https://github.com/rust-lang/rust/pull/134136)
* [fix ICE on type error in promoted](https://github.com/rust-lang/rust/pull/134010)
* [fix ICE when multiple supertrait substitutions need assoc but only one is provided](https://github.com/rust-lang/rust/pull/133392)
* [fix `trimmed_def_paths` ICE in the function ptr comparison lint](https://github.com/rust-lang/rust/pull/134357)
* [fix our `llvm::Bool` typedef to be signed, to match `LLVMBool`](https://github.com/rust-lang/rust/pull/134204)
* [interpret: reduce usage of `TypingEnv::fully_monomorphized`](https://github.com/rust-lang/rust/pull/134058)
* [jsondocck: parse, don't validate commands](https://github.com/rust-lang/rust/pull/133478)
* [keep track of parse errors in `mod`s and don't emit resolve errors for paths involving them](https://github.com/rust-lang/rust/pull/133937)
* [lint on combining `#[no_mangle]` and `#[export_name]`](https://github.com/rust-lang/rust/pull/131558)
* [make `Copy` unsafe to implement for ADTs with `unsafe` fields](https://github.com/rust-lang/rust/pull/134008)
* [make sure to use normalized ty for unevaluated const in default `struct` value](https://github.com/rust-lang/rust/pull/134314)
* [modifies the index instruction from `gep [0 x %Type]` to `gep %Type`](https://github.com/rust-lang/rust/pull/134117)
* [properly consider APITs for never type fallback ascription fix](https://github.com/rust-lang/rust/pull/134144)
* [remove `Lexer`'s dependency on `Parser`](https://github.com/rust-lang/rust/pull/134192)
* [remove queries from the driver interface](https://github.com/rust-lang/rust/pull/134302)
* [rudimentary heuristic to insert parentheses when needed for RPIT overcaptures lint](https://github.com/rust-lang/rust/pull/134142)
* [some asm! diagnostic adjustments and a papercut fix](https://github.com/rust-lang/rust/pull/134070)
* [some trait method vs impl method signature difference diagnostic cleanups](https://github.com/rust-lang/rust/pull/134386)
* [suggest using deref in patterns](https://github.com/rust-lang/rust/pull/132939)
* [suppress field expr with generics error message if it's a method](https://github.com/rust-lang/rust/pull/134154)
* [try to evaluate constants in legacy mangling](https://github.com/rust-lang/rust/pull/134081)
* [tweak multispan rendering to reduce output length](https://github.com/rust-lang/rust/pull/134181)
* [use SourceMap to load debugger visualizer files](https://github.com/rust-lang/rust/pull/134041)
* [use a more precise span in `placeholder_type_error_diag`](https://github.com/rust-lang/rust/pull/134256)
* [use newly added exceptions to non default branch warning](https://github.com/rust-lang/rust/pull/134089)
* [validate `--skip` and `--exclude` paths](https://github.com/rust-lang/rust/pull/134209)
* [validate self in host predicates correctly](https://github.com/rust-lang/rust/pull/134105)
* [bounds-check with PtrMetadata instead of Len in MIR](https://github.com/rust-lang/rust/pull/133734)
* [miri: TB Optimization: Skip subtrees based on the subtree's root node's permissions](https://github.com/rust-lang/miri/pull/4008)
* [miri: `localtime_r`: deduplicate timezone name allocation](https://github.com/rust-lang/miri/pull/4069)
* [miri: use clap in miri-script](https://github.com/rust-lang/miri/pull/4036)
* [stabilize `const_nonnull_new`](https://github.com/rust-lang/rust/pull/134116)
* [stabilize async closures](https://github.com/rust-lang/rust/pull/132706) (RFC [#3668](https://rust-lang.github.io/rfcs/3668-async-closures.html))
* [stabilize the Rust 2024 prelude](https://github.com/rust-lang/rust/pull/134178)
* [`UniqueRc` trait impls](https://github.com/rust-lang/rust/pull/133223)
* [`std::net`: Solaris supports `SOCK_CLOEXEC` as well since 11.4](https://github.com/rust-lang/rust/pull/130361)
* [add value accessor methods to `Mutex` and `RwLock`](https://github.com/rust-lang/rust/pull/133406)
* [de-duplicate and improve definition of `core::ffi::c_char`](https://github.com/rust-lang/rust/pull/132975)
* [run TLS destructors for wasm32-wasip1-threads](https://github.com/rust-lang/rust/pull/133472)
* [wasi/fs: improve stopping condition for `<ReadDir` as `Iterator>::next`](https://github.com/rust-lang/rust/pull/133184)
* [codegen\_gcc: stabilize `lang_tests_common` config parsing logic](https://github.com/rust-lang/rustc_codegen_gcc/pull/576)
* [codegen\_gcc: use casts instead of bitcast between pointers and integers to fix issues when using the lld linker](https://github.com/rust-lang/rustc_codegen_gcc/pull/577)
* [cargo: build-script: Pass `CARGO_CFG_FEATURE`](https://github.com/rust-lang/cargo/pull/14902)
* [cargo: SourceId: use stable hash from rustc-stable-hash](https://github.com/rust-lang/cargo/pull/14917)
* [cargo: base: Support bases in patches in virtual manifests](https://github.com/rust-lang/cargo/pull/14931)
* [cargo: build-rs: Implicitly report rerun-if-env-changed for input](https://github.com/rust-lang/cargo/pull/14911)
* [cargo: resolver: Don't report all versions as rejected](https://github.com/rust-lang/cargo/pull/14921)
* [cargo: resolver: In errors, show rejected versions over alt versions](https://github.com/rust-lang/cargo/pull/14923)
* [cargo: resolver: Report invalid index entries](https://github.com/rust-lang/cargo/pull/14927)
* [cargo: resolver: Report unmatched versions, rather than saying no package](https://github.com/rust-lang/cargo/pull/14897)
* [cargo: script: Don't override the release profile](https://github.com/rust-lang/cargo/pull/14925)
* [cargo: a faster hash for ActivationsKey](https://github.com/rust-lang/cargo/pull/14915)
* [cargo: implement `--depth workspace` for `cargo tree` command](https://github.com/rust-lang/cargo/pull/14928)
* [cargo: `emit_serialized_unit_graph` uses the configured shell](https://github.com/rust-lang/cargo/pull/14926)
* [rustdoc-search: fix mismatched path when parent re-exported twice](https://github.com/rust-lang/rust/pull/134231)
* [rustdoc-search: handle `impl Into<X>` better](https://github.com/rust-lang/rust/pull/134277)
* [rustdoc: fix self cmp](https://github.com/rust-lang/rust/pull/134214)
* [clippy: allow `needless_option_take` to report for more cases](https://github.com/rust-lang/rust-clippy/pull/13684)
* [clippy: better help message for `comparison_chain` lint](https://github.com/rust-lang/rust-clippy/pull/13762)
* [clippy: correct `single_match` lint suggestion](https://github.com/rust-lang/rust-clippy/pull/13824)
* [clippy: correct suggestion for `unnecessary_sort_by` in `no_std`](https://github.com/rust-lang/rust-clippy/pull/13836)
* [clippy: correctly handle string indices in `literal_string_with_formatting_arg`](https://github.com/rust-lang/rust-clippy/pull/13841)
* [clippy: detect shadowing in pattern field](https://github.com/rust-lang/rust-clippy/pull/13797)
* [clippy: do not suggest using `Error` in `no_std` before Rust 1.81](https://github.com/rust-lang/rust-clippy/pull/13834)
* [clippy: fix `must_use_unit` suggestion when there're multiple attributes](https://github.com/rust-lang/rust-clippy/pull/13830)
* [clippy: fix `single_match` lint being emitted when it should not](https://github.com/rust-lang/rust-clippy/pull/13765)
* [clippy: initial impl of `repr_packed_without_abi`](https://github.com/rust-lang/rust-clippy/pull/13398)
* [rust-analyzer: add an assist to extract an expression into a constant](https://github.com/rust-lang/rust-analyzer/pull/18652)
* [rust-analyzer: add diagnostic fix to remove unnecessary wrapper in type mismatch](https://github.com/rust-lang/rust-analyzer/pull/18458)
* [rust-analyzer: preserve order of parameters in `extract_functions`](https://github.com/rust-lang/rust-analyzer/pull/18656)
* [rust-analyzer: report unresolved idents for implicit captures in `format_args!()`](https://github.com/rust-lang/rust-analyzer/pull/18696)
* [rust-analyzer: fix publish workflow link in manual](https://github.com/rust-lang/rust-analyzer/pull/18666)
* [rust-analyzer: copied proc-macros not being cleaned up on exit](https://github.com/rust-lang/rust-analyzer/pull/18660)
* [rust-analyzer: fix a panic with a diagnostics fix when a keyword is used as a field](https://github.com/rust-lang/rust-analyzer/pull/18700)
* [rust-analyzer: fix path qualified auto-importing completions not working with re-exports](https://github.com/rust-lang/rust-analyzer/pull/18699)
* [rust-analyzer: fix proc-macro dylib names on windows](https://github.com/rust-lang/rust-analyzer/pull/18693)
* [rust-analyzer: fix sourceroot construction for virtual manifests](https://github.com/rust-lang/rust-analyzer/pull/18668)
* [rust-analyzer: generate implementation with items even if snippet text edit is disabled](https://github.com/rust-lang/rust-analyzer/pull/18667)
* [rust-analyzer: improve name suggestion for `destructure_tuple_binding`](https://github.com/rust-lang/rust-analyzer/pull/18695)
* [rust-analyzer: panic when displaying generic params with defaults, again](https://github.com/rust-lang/rust-analyzer/pull/18675)
* [rust-analyzer: swallow rustfmt parsing panics](https://github.com/rust-lang/rust-analyzer/pull/18663)
* [rust-analyzer: use string literal contents as a name when extracting into variable](https://github.com/rust-lang/rust-analyzer/pull/18690)
* [rust-analyzer: hash completion items to properly match them during /resolve](https://github.com/rust-lang/rust-analyzer/pull/18653)
* [rust-analyzer: properly handle different defaults for severity of lints](https://github.com/rust-lang/rust-analyzer/pull/18466)
* [rust-analyzer: show expansion errors in `expand_macro` feature](https://github.com/rust-lang/rust-analyzer/pull/18674)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### Tracking Issues & PRs
<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: -->
<!-- * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2024-12-18 - 2025-01-15 🦀

### Virtual
* 2024-12-19 | Virtual | [Scandio GmBH](https://www.eventbrite.de/o/scandio-gmbh-75623231843)
    * [**Einführung in Rust: Für eine nachhaltige Zukunft / Introduction to Rust: For a Sustainable Future**](https://www.eventbrite.com/e/einfuhrung-in-rust-fur-eine-nachhaltige-zukunft-tickets-1106203667949)
* 2024-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633276/)
* 2024-12-19 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina/events/)
    * [**Despedida de Año 🎉🎉**](https://www.meetup.com/rust-argentina/events/305095113)
* 2024-12-19 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 2024-12-20 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 2024-12-22 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller**](https://www.meetup.com/rust-tlv/events/304937982)
* 2024-12-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)
* 2024-12-26 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898145)
* 2025-01-02| Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633277/)
* 2025-01-04 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-01-06 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**logger.info(f"Don't Give your {secrets} away") by Tamar Galer (Virtual, English)**](https://www.meetup.com/code-mavens/events/305045436)
* 2025-01-07 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Microdosing Rust to your organization with Aviram Hassan (Virtual, English)**](https://www.meetup.com/code-mavens/events/304883841)
* 2025-01-08 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**BlockMesh Network implemented in Rust with Ohad Dahan (Virtual, English)**](https://www.meetup.com/code-mavens/events/304951805)
* 2025-01-09 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898167)
* 2025-01-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/302815031)
* 2025-01-14 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**An introduction to WASM in Rust with Márk Tolmács (Virtual, English)**](https://www.meetup.com/code-mavens/events/305064546)
* 2025-01-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)

### Asia
* 2025-01-12 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust January 2025 at Abra in Raanana**](https://www.meetup.com/rust-tlv/events/304898730/)

### Europe
* 2024-12-18 | Ghent, BE | [Systems Programming Ghent](https://sysghent.be)
    * [**Launch of new community for Rust and C++ developers**](https://sysghent.be)
* 2025-01-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305038426)
* 2025-01-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820279/)
* 2025-01-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154281)

### North America
* 2024-12-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Harvard Square Rust Lunch, Dec 22**](https://www.meetup.com/bostonrust/events/304951457)
* 2024-12-26 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)
* 2025-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Lechmere Rust Lunch, Jan 10**](https://www.meetup.com/bostonrust/events/304951467)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1h2zwpx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> She said yes!! (And so did I!)

– [Amos on Mastodon](https://hachyderm.io/@fasterthanlime/113639047728482697) proving that Rustaceans *do* have a life outside of Rust. Congratulations, Amos!

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1642) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
