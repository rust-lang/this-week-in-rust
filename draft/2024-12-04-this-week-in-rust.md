Title: This Week in Rust 576
Number: 576
Date: 2024-12-04
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
* [Announcing Rust 1.83.0](https://blog.rust-lang.org/2024/11/28/Rust-1.83.0.html)
* [Rustc Trait System Refactor Initiative Update: Stabilizing `-Znext-solver=coherence` ](https://blog.rust-lang.org/inside-rust/2024/12/04/trait-system-refactor-initiative.html)
* [The wasm32-wasip2 Target Has Reached Tier 2 Support](https://blog.rust-lang.org/2024/11/26/wasip2-tier-2.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [Measuring and Improving rustls's Multithreaded Performance](https://rustls.dev/perf/2024-11-28-threading/)
* [Bevy 0.15](https://bevyengine.org/news/bevy-0-15/)
* [Leptos 0.7.0](https://github.com/leptos-rs/leptos/releases/tag/v0.7.0)
* [Advent of Rust 2024](https://www.rustfinity.com/advent-of-rust)
* [Introducing Uniffi for React Native: Rust-Powered Turbo Modules](https://hacks.mozilla.org/2024/12/introducing-uniffi-for-react-native-rust-powered-turbo-modules/)
* [Revisiting Hubris appconfigs](https://cliffle.com/blog/exhubris/)
* [RVKMS and Rust KMS bindings](https://lwn.net/SubscriberLink/997850/8f1246199581a250/)
* [NonStop discussion around adding Rust to Git](https://lwn.net/SubscriberLink/998115/e9849d6de88348c6/)
* [Rust's incremental compiler architecture](https://lwn.net/SubscriberLink/997784/84e8aae50b88cca6/)
* [What's new in SeaStreamer 0.5](https://www.sea-ql.org/blog/2024-11-30-whats-new-in-sea-streamer-0.5/)

### Observations/Thoughts
* [audio] [GitButler with Scott Chacon and Kiril Videlov](https://corrode.dev/podcast/s03e04-gitbutler/)
* [audio] [A different serde](https://sdr-podcast.com/episodes/a-different-serde/)
* [Streaming Audio APIs in Rust pt. 4: The Model](https://xd009642.github.io/2024/12/03/streaming-audio-APIs-the-model.html)

### Rust Walkthroughs
* [Optimization adventures: making a parallel Rust workload even faster with data-oriented design (and other tricks)](https://gendignoux.com/blog/2024/12/02/rust-data-oriented-design.html)
* [Designing a const `array::from_fn` in stable Rust](https://13ros27.github.io/blog/const-array-from-fn/)
* [Implementing async APIs for microcontroller peripherals](https://beaurivage.io/atsamd-hal-async/)
* [Rust 🦀 on the RP2040](https://baileytownsend.dev/articles/getting-started-with-rust-an-rp2040)
* [Building a real-time chat using WebSockets over HTTP/2 streams](https://c410-f3r.github.io/thoughts/building-a-real-time-chat-using-web-sockets-over-http2-streams)
* [Running Bevy in a Web Worker with Rendering and Physics!](https://allwright.io/#/blog/20241127-bevy-webworker.md)
* [Packaging a Rust library as an XCFramework for iOS](https://stadiamaps.com/news/ferrostar-building-a-cross-platform-navigation-sdk-in-rust-part-2/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [augurs](https://github.com/grafana/augurs), a time-series toolkit for Rust with bindings to JS & Python.

Thanks to [Ben Sully](https://users.rust-lang.org/t/crate-of-the-week/2704/1379) for the suggestion!

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

* [RustWeek 2025](https://www.papercall.io/rust-week) | Closes 2025-01-12 | Utrecht, The Netherlands | Event date: 2025-05-13

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

488 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-26..2024-12-03

* [`rust_analyzer_settings`: force use of 'nightly' toolchain](https://github.com/rust-lang/rust/pull/133712)
* [add `needs-target-has-atomic` directive](https://github.com/rust-lang/rust/pull/133736)
* [allow injecting a profiler runtime into `#![no_core]` crates](https://github.com/rust-lang/rust/pull/133369)
* [bail on more errors in dyn ty lowering](https://github.com/rust-lang/rust/pull/133394)
* [better diagnostic for fn items in variadic functions](https://github.com/rust-lang/rust/pull/133538)
* [changes old intrinsic declaration to new declaration](https://github.com/rust-lang/rust/pull/133106)
* [check `xform_ret_ty` for WF in the new solver to improve method winnowing](https://github.com/rust-lang/rust/pull/133519)
* [check let source before suggesting annotation](https://github.com/rust-lang/rust/pull/133691)
* [check local cache even if global is usable](https://github.com/rust-lang/rust/pull/133626)
* [cleanup: delete `//@ pretty-expanded` directive](https://github.com/rust-lang/rust/pull/133470)
* [constify `Drop` and `Destruct`](https://github.com/rust-lang/rust/pull/133402)
* [coverage: store coverage source regions as `Span` until codegen](https://github.com/rust-lang/rust/pull/133418)
* [coverage: use a query to identify which counter/expression IDs are used](https://github.com/rust-lang/rust/pull/133446)
* [delay a bug when encountering an impl with unconstrained generics in `codegen_select`](https://github.com/rust-lang/rust/pull/133368)
* [disable `avr-rjmp-offset` on Windows for now](https://github.com/rust-lang/rust/pull/133481)
* [do not call `extern_crate` on current trait on crate mismatch errors](https://github.com/rust-lang/rust/pull/133585)
* [do not constrain infer vars in `find_best_leaf_obligation`](https://github.com/rust-lang/rust/pull/133493)
* [do not create trait object type if missing associated types](https://github.com/rust-lang/rust/pull/133660)
* [do not unify dereferences of shared borrows in GVN](https://github.com/rust-lang/rust/pull/133474)
* [don't type error if we fail to coerce `Pin<T>` because it doesnt contain a ref](https://github.com/rust-lang/rust/pull/133358)
* [eliminate magic numbers from expression precedence](https://github.com/rust-lang/rust/pull/133603)
* [enable `-Zshare-generics` for `inline(never)` functions](https://github.com/rust-lang/rust/pull/123244)
* [ensure JSON-defined targets are consistent](https://github.com/rust-lang/rust/pull/133409)
* [fast-reject: add depth check](https://github.com/rust-lang/rust/pull/133566)
* [fix ICE when promoted has layout size overflow](https://github.com/rust-lang/rust/pull/133704)
* [fix `-Zdump-mir-dataflow`](https://github.com/rust-lang/rust/pull/133732)
* [fix `clobber_abi` in RV32E and RV64E inline assembly](https://github.com/rust-lang/rust/pull/133422)
* [fix confusing diagnostic for reserved `##`](https://github.com/rust-lang/rust/pull/133487)
* [fix handling of x18 in AArch64 inline assembly on ohos/trusty or with -Zfixed-x18](https://github.com/rust-lang/rust/pull/133463)
* [gce: fix `typing_mode` mismatch](https://github.com/rust-lang/rust/pull/133471)
* [get rid of HIR const checker](https://github.com/rust-lang/rust/pull/133321)
* [improve span handling in `parse_expr_bottom`](https://github.com/rust-lang/rust/pull/133623)
* [improvements on initial sysroot and libdir finding logics](https://github.com/rust-lang/rust/pull/132782)
* [make `adjust_fulfillment_errors` work with `HostEffectPredicate` and `const_conditions`](https://github.com/rust-lang/rust/pull/133403)
* [make `compare_impl_item` into a query](https://github.com/rust-lang/rust/pull/133365)
* [only error raw lifetime followed by `\'` in edition 2021+](https://github.com/rust-lang/rust/pull/133482)
* [only ignore windows-gnu in avr-jmp-offset](https://github.com/rust-lang/rust/pull/133513)
* [print generated doc paths](https://github.com/rust-lang/rust/pull/133550)
* [properly pass linker arguments that contain commas](https://github.com/rust-lang/rust/pull/132974)
* [respect verify-llvm-ir option in the backend](https://github.com/rust-lang/rust/pull/133499)
* [robustify and genericize return-type-notation resolution in `resolve_bound_vars`](https://github.com/rust-lang/rust/pull/132047)
* [show `forbidden_lint_groups` in future-compat reports](https://github.com/rust-lang/rust/pull/133535)
* [support `clobber_abi` in AVR inline assembly](https://github.com/rust-lang/rust/pull/131323)
* [support input/output in vector registers of PowerPC inline assembly](https://github.com/rust-lang/rust/pull/131551)
* [support predicate registers (clobber-only) in Hexagon inline assembly](https://github.com/rust-lang/rust/pull/133452)
* [support revealing defined opaque post borrowck](https://github.com/rust-lang/rust/pull/133501)
* [target `check_consistency`: ensure target feature string makes some basic sense](https://github.com/rust-lang/rust/pull/133410)
* [the emscripten OS no longer exists on non-wasm targets](https://github.com/rust-lang/rust/pull/133411)
* [use edition of `macro_rules` when compiling the macro](https://github.com/rust-lang/rust/pull/133274)
* [use stores of the correct size to set discriminants](https://github.com/rust-lang/rust/pull/131698)
* [miri: implement `TlsFree`](https://github.com/rust-lang/rust/pull/133457)
* [miri: filesystem support for solarish: stat](https://github.com/rust-lang/miri/pull/4031)
* [miri: move FdTable to a common location and split off Unix behavior](https://github.com/rust-lang/miri/pull/4045)
* [miri: remove ctrlc, unused](https://github.com/rust-lang/miri/pull/4064)
* [stop cloning `Context` so much](https://github.com/rust-lang/rust/pull/133345)
* [recover some lost performence](https://github.com/rust-lang/rust/pull/133509)
* [stabilize `const_maybe_uninit_write`](https://github.com/rust-lang/rust/pull/131713)
* [stabilize `extended_varargs_abi_support`](https://github.com/rust-lang/rust/pull/116161)
* [stabilize `ptr::fn_addr_eq`](https://github.com/rust-lang/rust/pull/133678)
* [stabilize unsigned and float variants of `num_midpoint` feature](https://github.com/rust-lang/rust/pull/131784)
* [`thread::available_parallelism` for wasm32-wasip1-threads](https://github.com/rust-lang/rust/pull/133496)
* [add `BTreeSet` entry APIs to match `HashSet`](https://github.com/rust-lang/rust/pull/133548)
* [btree: add `{Entry,VacantEntry}::insert_entry`](https://github.com/rust-lang/rust/pull/133042)
* [fix chaining `carrying_add`s](https://github.com/rust-lang/rust/pull/133674)
* [mark `slice::copy_from_slice` unstably const](https://github.com/rust-lang/rust/pull/131416)
* [std: expose `const_io_error!` as `const_error!`](https://github.com/rust-lang/rust/pull/133449)
* [std: refactor `pthread`-based synchronization](https://github.com/rust-lang/rust/pull/128184)
* [fix and undeprecate `home_dir()`](https://github.com/rust-lang/rust/pull/132515)
* [support ranges in `<[T]>::get_many_mut()`](https://github.com/rust-lang/rust/pull/133136)
* [cargo: toml: Allow adding/removing from cargo scripts](https://github.com/rust-lang/cargo/pull/14857)
* [cargo: build-std: always link to std when testing proc-macros](https://github.com/rust-lang/cargo/pull/14850)
* [cargo: fix: Migrate cargo script manifests across editions](https://github.com/rust-lang/cargo/pull/14864)
* [cargo: build-std: download deps first](https://github.com/rust-lang/cargo/pull/14861)
* [cargo: pgo: determine test runnability at compile time](https://github.com/rust-lang/cargo/pull/14874)
* [cargo: pgo: ensure PGO works](https://github.com/rust-lang/cargo/pull/14859)
* [cargo: pgo: only run on nightly](https://github.com/rust-lang/cargo/pull/14887)
* [cargo: add future-incompat warning against keywords in cfgs and add raw-idents](https://github.com/rust-lang/cargo/pull/14671)
* [cargo: fix: remove default registry reference in `info` cmd docs](https://github.com/rust-lang/cargo/pull/14880)
* [cargo: git-fetch-with-cli: set `GIT_DIR` for bare repository compatibility](https://github.com/rust-lang/cargo/pull/14860)
* [cargo: test: `requires` attribute accepts string literals for cmds](https://github.com/rust-lang/cargo/pull/14875)
* [rustdoc-json: include safety of `static`s](https://github.com/rust-lang/rust/pull/133715)
* [rustdoc: Change impl items indent](https://github.com/rust-lang/rust/pull/131718)
* [bindgen: add support for unsafe extern blocks](https://github.com/rust-lang/rust-bindgen/pull/3015)
* [bindgen: consolidate dependency version management](https://github.com/rust-lang/rust-bindgen/pull/3008)
* [bindgen: extend parse callbacks to expose discovered composite types and aliases](https://github.com/rust-lang/rust-bindgen/pull/2658)
* [bindgen: generate C-String literals `c"example"` instead of unsafe code](https://github.com/rust-lang/rust-bindgen/pull/2996)
* [bindgen: improve debug str generator](https://github.com/rust-lang/rust-bindgen/pull/3010)
* [bindgen: introduce `--rust-edition`](https://github.com/rust-lang/rust-bindgen/pull/3002)
* [bindgen: use v2 cargo resolver](https://github.com/rust-lang/rust-bindgen/pull/2999)
* [bindgen: wrap the array representation of opaque types in a `#[repr(C)] struct`](https://github.com/rust-lang/rust-bindgen/pull/2880)
* [rustfmt: use preinterned `path` symbol](https://github.com/rust-lang/rustfmt/pull/6404)
* [clippy: `bad_bit_mask` Fix false positive on proc macros](https://github.com/rust-lang/rust-clippy/pull/13736)
* [clippy: `doc_nested_refdefs`: new lint for suspicious list syntax](https://github.com/rust-lang/rust-clippy/pull/13707)
* [clippy: add more cases to the `useless_conversion` lint](https://github.com/rust-lang/rust-clippy/pull/13756)
* [clippy: add new `literal_string_with_formatting_args` lint](https://github.com/rust-lang/rust-clippy/pull/13410)
* [clippy: fix `needless_match` FP on if-lets](https://github.com/rust-lang/rust-clippy/pull/13646)
* [clippy: fix `shadow_unrelated`'s behaviour with closures](https://github.com/rust-lang/rust-clippy/pull/13677)
* [clippy: fix lifetimes elision suggestion in where clauses](https://github.com/rust-lang/rust-clippy/pull/13752)
* [clippy: fix: use `multipart_suggestion` for `derivable_impls`](https://github.com/rust-lang/rust-clippy/pull/13717)
* [clippy: handle repetition of associated constant constraint as well](https://github.com/rust-lang/rust-clippy/pull/13723)
* [rust-analyzer: advertise completions and inlay hints resolve server capabilities based on the client capabilities](https://github.com/rust-lang/rust-analyzer/pull/18589)
* [rust-analyzer: fix debug configuration querying not inheriting environment](https://github.com/rust-lang/rust-analyzer/pull/18586)
* [rust-analyzer: fix syntax fixup inserting unnecessary semicolons](https://github.com/rust-lang/rust-analyzer/pull/18587)
* [rust-analyzer: re-add `rust-analyzer.cargo.sysrootQueryMetadata`](https://github.com/rust-lang/rust-analyzer/pull/18511)
* [rust-analyzer: remove redundant associated type bounds from `dyn TypeFolder`](https://github.com/rust-lang/rust-analyzer/pull/18577)

### Rust Compiler Performance Triage

Busy week with more PRs impacting performance than is typical. Luckily performance improvements outweighed regressions in real world benchmarks with the largest single performance gain coming from a change to no longer unconditionally do LLVM IR verification in debug builds which was just wasted work.

Triage done by **@rylev**.
Revision range: [7db7489f..490b2cc0](https://perf.rust-lang.org/?start=7db7489f9bc274cb60c4956bfa56de0185eb1b9b&end=490b2cc09860dd62a7595bb07364d71c12ce4e60&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.2%, 1.9%]   | 58    |
| Regressions ❌ <br /> (secondary)  | 1.1%  | [0.2%, 5.1%]   | 85    |
| Improvements ✅ <br /> (primary)   | -2.3% | [-8.2%, -0.2%] | 116   |
| Improvements ✅ <br /> (secondary) | -2.5% | [-8.9%, -0.1%] | 55    |
| All ❌✅ (primary)                 | -1.4% | [-8.2%, 1.9%]  | 174   |


6 Regressions, 6 Improvements, 5 Mixed; 5 of them in rollups
49 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/fcd028e6e8117a881b7ffab448f549410c1c0dde/triage/2024-12-03.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [crates.io: Remove version deletions from the "crate deletions" RFC](https://github.com/rust-lang/rfcs/pull/3731)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [crates.io: Trusted Publishing Support](https://github.com/rust-lang/rfcs/pull/3691)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Fix ICE when multiple supertrait substitutions need assoc but only one is provided](https://github.com/rust-lang/rust/pull/133392)
* [disposition: merge] [rework winnowing to sensibly handle global where-bounds](https://github.com/rust-lang/rust/pull/132325)
* [disposition: merge] [Always display first line of impl blocks even when collapsed](https://github.com/rust-lang/rust/pull/132155)
* [disposition: merge] [Tracking Issue for `const_nonnull_new`](https://github.com/rust-lang/rust/issues/93235)
* [disposition: merge] [Lint on combining `#[no_mangle]` and `#[export_name]`](https://github.com/rust-lang/rust/pull/131558)
* [disposition: merge] [Add Extend impls for tuples of arity 1 through 12](https://github.com/rust-lang/rust/pull/132187)
* [disposition: merge] [[discussion] `ErrorKind::FilesystemQuotaExceeded` from `io_error_more`](https://github.com/rust-lang/rust/issues/130190)
* [disposition: merge] [[discussion] `ErrorKind::CrossesDevices` from `io_error_more`](https://github.com/rust-lang/rust/issues/130191)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [[RFC] field projections v2](https://github.com/rust-lang/rfcs/pull/3735)

## Upcoming Events

Rusty Events between 2024-12-04 - 2025-01-01 🦀

### Virtual
* 2024-11-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 2024-11-28 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-11-29 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304477903/)
* 2024-12-02 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Advent of Code - Kick Off!**](https://www.meetup.com/women-in-rust/events/304668776/)
* 2024-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)
* 2024-12-03 | Virtual (San Francisco, CA, US) | [Blockchain Center SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust in Web3: Developer Series**](https://www.meetup.com/blockchaincentersf/events/304510595/)
* 2024-12-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031652)
* 2024-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633275/)
* 2024-12-05 | Virtual (Miami, FL) | [Miami Java User Group Events](https://www.meetup.com/miami-java-user-group)
    * [**Introduction to Rust for Java Developers**](https://www.meetup.com/miami-java-user-group/events/304717903/)
* 2024-12-06 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304369723/)
* 2024-12-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-12-08 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Reading JSON files in Rust - קריאת קבצי ג'ייסון בראסט**](https://www.meetup.com/rust-tlv/events/304685546/)
* 2024-12-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346988/)
* 2024-12-11 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 2024-12-12 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 2024-12-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-12-12 | Hybrid: In-Person and Virtual (Seattle, WA, US) | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**December Meetup**](https://www.meetup.com/Seattle-Rust-Meetup/)
* 2024-12-13 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304730560/)
* 2024-12-17 | Virtual (San Francisco, CA, US) | [Blockchain Center SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust in Web3: Developer Series**](https://www.meetup.com/blockchaincentersf/events/kwnzntygcqbwb/)
* 2024-12-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346972/)
* 2024-12-17 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate**](https://www.meetup.com/code-mavens/events/304824684/)
* 2024-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633276/)
* 2024-12-19 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 2024-12-20 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 2024-12-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)

### Africa
* 2024-12-10 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Hello World... again**](https://www.meetup.com/johannesburg-rust-meetup/events/304649358/)

### Asia
* 2024-11-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**RustTechX Summit 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 2024-11-30 | Tokyo, JP | [Rust Tokyo](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)
* 2024-12-03 | Ra'anana, IL | [Abra R&D Meetups](https://www.meetup.com/abra-rnd-solutions/)
    * [**Rust in the Linux Kernel**](https://www.meetup.com/abra-rnd-solutions/events/304302596/)

### Europe
* 2024-11-27 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 2024-11-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 2024-11-28 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 2024-11-28 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421381/)
* 2024-11-28 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #53 sponsored by Microsoft**](https://www.meetup.com/copenhagen-rust-community/events/304608747/)
* 2024-11-28 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #5**](https://www.meetup.com/rust-gdansk/events/304462668/)
* 2024-11-28 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn with Mainmatter & Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 2024-11-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/304556866/)
* 2024-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Prague (November 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 2024-11-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #7**](https://www.meetup.com/stockholm-rust/events/304722627/)
* 2024-12-03 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #11: Advent of Code**](https://www.meetup.com/copenhagen-rust-community/events/304427710/)
* 2024-12-04 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Aprenem junts Rust / Learn Rust Together**](https://lu.ma/pypwr0m7)
* 2024-12-04 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in December: Advent of Code**](https://www.meetup.com/rustcologne/events/304760521/)
* 2024-12-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 2024-12-04 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #73**](https://www.meetup.com/rust-paris/events/304685955/)
* 2024-12-05 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #6**](https://www.meetup.com/rust-gdansk/events/304773705/)
* 2024-12-05 | Zlin, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Rust Moravia Meetup (December 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 2024-12-06 | Moscow, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Russia**](https://rustcon.ru/)
* 2024-12-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 2024-12-12 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 2024-12-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Types, Traits und Best Practices**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)

### North America
* 2024-11-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)
* 2024-12-05 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**December Monthly Social**](https://www.meetup.com/rust-montreal/events/304778367/)
* 2024-12-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Rust Strings**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)
* 2024-12-12 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 2024-12-12 | Hybrid: In-Person and Virtual (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/)
    * [**December Meetup**](https://www.meetup.com/join-srug/events/304806455/)
* 2024-12-16 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 2024-12-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)
* 2024-12-23 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcqbfc/)

### Oceania
* 2024-12-04 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/events/)
    * [**2024 🦀 Encore ✨ Talks**](https://www.meetup.com/rust-sydney/events/304625921/)
* 2024-12-08 | Canberra, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**CRUG Xmas party**](https://www.meetup.com/rust-canberra/events/304282046/)

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

> "self own" sounds like a rust thing

– [ionchy on Mastodon](https://types.pl/@ionchy/113567387219906256)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1636) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
