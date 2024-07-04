Title: This Week in Rust 554
Number: 554
Date: 2024-07-03
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Types Team Update and Roadmap](https://blog.rust-lang.org/2024/06/26/types-team-update.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [Rustlings Rewrite](https://mo8it.com/blog/rustlings-rewrite/)
* [iroh 0.19.0 - Make it your own](https://iroh.computer/blog/iroh-0-19-make-it-your-own)
* [Announcing Polars 1.0](https://pola.rs/posts/announcing-polars-1/)
* [rust-analyzer changelog #240](https://rust-analyzer.github.io/thisweek/2024/07/01/changelog-240.html)
* [r3bl_cmdr v0.0.14 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v0014-2024-06-29)
* [r3bl_tui v0.5.6 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v056-2024-06-29)
* [Danube - Pub/Sub messaging broker](https://dev-state.com/posts/danube_intro/) - Intro to Danube - a distributed Pub/Sub message broker developed in Rust.
* [what-the-time 1.0.0](https://github.com/sdball/what-the-time/releases/tag/v1.0.0): calculate
  time diffs between JSON log lines
* [Meilisearch releases v1.9](https://blog.meilisearch.com/meilisearch-1-9/)

### Observations/Thoughts
* [Ergonomic Self-Referential Types for Rust](https://blog.yoshuawuyts.com/self-referential-types/)
* [More thoughts on claiming](https://smallcultfollowing.com/babysteps/blog/2024/06/26/claim-followup-1/)
* [Async closures](https://hackmd.io/@compiler-errors/async-closures)
* [Rust has three reference types!](https://ssbr.xyz/blog/rust-has-three-reference-types/)
* [audio] [OxidOS with Alexandru Radovici](https://corrode.dev/podcast/s02e05-oxidos/)

### Rust Walkthroughs
* [Resolving Rust Symbols](https://blog.shrirambalaji.com/posts/resolving-rust-symbols/)
* [#![doc = include_str!()] with intra-doc links](https://linebender.org/blog/doc-include/)
- [The Minimal Rust-Wasm Setup](https://dzfrias.dev/blog/rust-wasm-minimal-setup/)
* [Build with Naz : Markdown parser in Rust and nom from r3bl_tui](https://developerlife.com/2024/06/28/md-parser-rust-from-r3bl-tui/)
* [Adding compile-time safety to the AWS SDK with syn's Visit trait](https://medium.com/@sam.van.overmeire/adding-compile-time-safety-to-the-aws-sdk-with-syns-visit-trait-57bfbbac8677)
* [Adding GraphQL Support to Loco with Seaography](https://www.sea-ql.org/blog/2024-07-01-graphql-support-with-loco-seaography/)
* [Rust patterns: Don't put any code in mod.rs or lib.rs files](https://kerkour.com/rust-patterns-dont-put-code-in-lib-mod-files)

### Research

### Miscellaneous
* [video] [Zelda Hessler discusses the AWS SDK for Rust](https://www.youtube.com/watch?v=-PTSJbUZ_Jo)

* [FizzBuzz Multithreaded - synchronization with rendezvous channels](https://firedbg.sea-ql.org/blog/2024-06-30-fizzbuzz-multithread/)

## Crate of the Week

This week's crate is [asak](https://github.com/chaosprint/asak), a terminal-based audio recording/playback TUI.

Despite a lamentable lack of suggestions this week, llogiq is reasonably pleased with his choice.

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

* [diesel- Add support for currently unsupported range operators and methods](https://github.com/diesel-rs/diesel/issues/4092)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

408 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-06-25..2024-07-02

* [`rustc_data_structures`: Explicitly check for 64-bit atomics support](https://github.com/rust-lang/rust/pull/127075)
* [add `()` to the `marker_impls` macro for `ConstParamTy`](https://github.com/rust-lang/rust/pull/127070)
* [add `.ignore` file to make `config.toml` searchable in vscode](https://github.com/rust-lang/rust/pull/126876)
* [add more constants, functions, and tests for `f16` and `f128`](https://github.com/rust-lang/rust/pull/126608)
* [ast: standardize visiting order](https://github.com/rust-lang/rust/pull/126993)
* [ast: standardize visiting order for attributes and node IDs](https://github.com/rust-lang/rust/pull/125741)
* [automatically taint InferCtxt when errors are emitted](https://github.com/rust-lang/rust/pull/126996)
* [avoid MIR bloat in inlining](https://github.com/rust-lang/rust/pull/127113)
* [avoid cloning jump threading state when possible](https://github.com/rust-lang/rust/pull/127024)
* [coverage: avoid getting extra unexpansion info when we don't need it](https://github.com/rust-lang/rust/pull/127157)
* [coverage: make `#[coverage(..)]` apply recursively to nested functions](https://github.com/rust-lang/rust/pull/126721)
* [de-duplicate all consecutive native libs regardless of their options](https://github.com/rust-lang/rust/pull/126943)
* [delegation: ast lowering refactor](https://github.com/rust-lang/rust/pull/126947)
* [deny `use<>` for RPITITs](https://github.com/rust-lang/rust/pull/126746)
* [detect unused structs which derived Default](https://github.com/rust-lang/rust/pull/126302)
* [do not ICE when suggesting dereferencing closure arg](https://github.com/rust-lang/rust/pull/126884)
* [don't ICE during RPITIT refinement checking for resolution errors after normalization](https://github.com/rust-lang/rust/pull/126968)
* [don't suggest awaiting in closure patterns](https://github.com/rust-lang/rust/pull/126915)
* [eliminate the distinction between `PREC_POSTFIX` and `PREC_PAREN` precedence level](https://github.com/rust-lang/rust/pull/126893)
* [enable const casting for `f16` and `f128`](https://github.com/rust-lang/rust/pull/127032)
* [fix `x86_64` code being produced for bare-metal LoongArch targets' `compiler_builtins`](https://github.com/rust-lang/rust/pull/127150)
* [fix an error suggestion for E0121 when using placeholder `_` as return types on function signature](https://github.com/rust-lang/rust/pull/127110)
* [fix bad replacement for unsafe extern block suggestion](https://github.com/rust-lang/rust/pull/126973)
* [prefer `(*p).clone` to `p.clone` if the `p` is a raw pointer](https://github.com/rust-lang/rust/pull/127114)
* [ignore `llvm::Lld` if lld is not enabled](https://github.com/rust-lang/rust/pull/126701)
* [implement new effects desugaring](https://github.com/rust-lang/rust/pull/120639)
* [improve unsafe extern blocks diagnostics](https://github.com/rust-lang/rust/pull/127106)
* [introduce a `rustc_` attribute to dump all the `DefId` parents of a `DefId`](https://github.com/rust-lang/rust/pull/127181)
* [less `maybe_whole_expr`, take 2](https://github.com/rust-lang/rust/pull/126571)
* [let's `#[expect]` some lints: Stabilize `lint_reasons` (RFC 2383)](https://github.com/rust-lang/rust/pull/120924)
* [linker: refactor interface for passing arguments to linker](https://github.com/rust-lang/rust/pull/126832)
* [make `feature(effects)` require `-Znext-solver`](https://github.com/rust-lang/rust/pull/127176)
* [make cargo submodule optional](https://github.com/rust-lang/rust/pull/126470)
* [mark assoc tys live only if the corresponding trait is live](https://github.com/rust-lang/rust/pull/126618)
* [move binder and polarity parsing into `parse_generic_ty_bound`](https://github.com/rust-lang/rust/pull/127103)
* [not use offset when there is not ends with brace](https://github.com/rust-lang/rust/pull/126868)
* [patchable-function-entry: add unstable compiler flag and attribute](https://github.com/rust-lang/rust/pull/124741)
* [print `TypeId` as a `u128` for `Debug`](https://github.com/rust-lang/rust/pull/127134)
* [un-unsafe the `StableOrd` trait](https://github.com/rust-lang/rust/pull/126326)
* [remove (deprecated & unstable) `{to,from}_bits` pointer methods](https://github.com/rust-lang/rust/pull/127071)
* [remove `f16` and `f128` ICE paths from smir](https://github.com/rust-lang/rust/pull/126983)
* [remove more `PtrToPtr` casts in GVN](https://github.com/rust-lang/rust/pull/126844)
* [remove unnecessary SeqCst in `impl fmt::Pointer for AtomicPtr`](https://github.com/rust-lang/rust/pull/127073)
* [resolve: tweak some naming around import ambiguities](https://github.com/rust-lang/rust/pull/126954)
* [show used attribute's kind for user when find it isn't applied to a `static` variable](https://github.com/rust-lang/rust/pull/127118)
* [stall computing instance for drop shim until it has no unsubstituted const params](https://github.com/rust-lang/rust/pull/127068)
* [support fetching `Attribute` of items](https://github.com/rust-lang/rust/pull/127022)
* [switch back `non_local_definitions` lint to allow-by-default](https://github.com/rust-lang/rust/pull/127015)
* [tighten `fn_decl_span` for async blocks](https://github.com/rust-lang/rust/pull/127058)
* [transmute size check: properly account for alignment](https://github.com/rust-lang/rust/pull/125740)
* [tweak `FlatPat::new` to avoid a temporarily-invalid state](https://github.com/rust-lang/rust/pull/126932)
* [tweak a confusing comment in `create_match_candidates`](https://github.com/rust-lang/rust/pull/126926)
* [unify `dylib` and `bin_helpers` and create `shared_helpers::parse_value_from_args`](https://github.com/rust-lang/rust/pull/127108)
* [use `clang-format` in `tidy` to check the C++ code style under `llvm-wrapper`](https://github.com/rust-lang/rust/pull/123918)
* [use full expr span for return suggestion on type error/ambiguity](https://github.com/rust-lang/rust/pull/127129)
* [various `rustc_codegen_ssa` cleanups](https://github.com/rust-lang/rust/pull/123237)
* [various refactorings to `rustc_interface`](https://github.com/rust-lang/rust/pull/126834)
* [miri: `iter_exported_symbols`: also walk used statics in local crate](https://github.com/rust-lang/miri/pull/3723)
* [miri: remove GetCurrentProcessId's `frame_in_std` check](https://github.com/rust-lang/miri/pull/3716)
* [stabilize `PanicInfo::message()` and `PanicMessage`](https://github.com/rust-lang/rust/pull/126732)
* [stabilize `duration_abs_diff`](https://github.com/rust-lang/rust/pull/127128)
* [mark `Hasher::finish` as `#[must_use]`](https://github.com/rust-lang/rust/pull/127055)
* [std: separate TLS key creation from TLS access](https://github.com/rust-lang/rust/pull/126953)
* [cargo: allow `unexpected_builtin_cfgs` lint in `user_specific_cfgs` test](https://github.com/rust-lang/cargo/pull/14153)
* [cargo: gix: remove `revision` feature from cargo](https://github.com/rust-lang/cargo/pull/14160)
* [cargo: make it clear that `CARGO_CFG_TARGET_FAMILY` is multi-valued](https://github.com/rust-lang/cargo/pull/14165)
* [cargo: test: fix several assertions](https://github.com/rust-lang/cargo/pull/14167)
* [cargo: test: omit target-dir name](https://github.com/rust-lang/cargo/pull/14142)
* [cargo: test: replace glob with explicit unordered calls](https://github.com/rust-lang/cargo/pull/14166)
* [rustdoc: check if the disambiguator matches its suffix](https://github.com/rust-lang/rust/pull/127016)
* [clippy: `doc_lazy_continuation`: blank comment line for gap](https://github.com/rust-lang/rust-clippy/pull/13002)
* [clippy: add error message to `manual_inspect` lint](https://github.com/rust-lang/rust-clippy/pull/13006)
* [clippy: don't lint `assertions_on_constants` on any const assertions](https://github.com/rust-lang/rust-clippy/pull/12840)
* [clippy: fix `doc_markdown` DevOps false positive](https://github.com/rust-lang/rust-clippy/pull/12995)
* [clippy: implement a lint to replace manual bit rotations with `rotate_left/rot…`](https://github.com/rust-lang/rust-clippy/pull/12983)
* [rust-analyzer: do not normalize `use foo::{self}` to `use foo`](https://github.com/rust-lang/rust-analyzer/pull/17494)
* [rust-analyzer: add `bool_to_enum` assist for parameters](https://github.com/rust-lang/rust-analyzer/pull/17467)
* [rust-analyzer: completions after async kw](https://github.com/rust-lang/rust-analyzer/pull/17513)
* [rust-analyzer: fix expression scope calculation when within macro expansions](https://github.com/rust-lang/rust-analyzer/pull/17518)
* [rust-analyzer: pass cargo extra args when debugging](https://github.com/rust-lang/rust-analyzer/pull/17495)
* [rust-analyzer: quality of life improvements to term search](https://github.com/rust-lang/rust-analyzer/pull/17516)
* [rust-analyzer: use proper `ImplTraits` in `insert_inference_vars_for_impl_trait`](https://github.com/rust-lang/rust-analyzer/pull/17505)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:
* [RFC: Unblock Cargo feature metadata](https://github.com/rust-lang/rfcs/pull/3416)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [RFC: #[derive(SmartPointer)]](https://github.com/rust-lang/rfcs/pull/3621)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking issue for const char conversions](https://github.com/rust-lang/rust/issues/89259)
* [disposition: merge] [Don't make statement nonterminals match pattern nonterminals](https://github.com/rust-lang/rust/pull/120221)
* [disposition: merge] [Allow `#[deny]` inside `#[forbid]` as a no-op](https://github.com/rust-lang/rust/pull/121560)
* [disposition: merge] [Tracking issue for function attribute `#[coverage]`](https://github.com/rust-lang/rust/issues/84605)
* [disposition: merge] [Bump `elided_lifetimes_in_associated_constant` to deny](https://github.com/rust-lang/rust/pull/124211)
* [disposition: merge] [Tracking Issue for `const_cstr_from_ptr`](https://github.com/rust-lang/rust/issues/113219)
* [disposition: merge] [Deny keyword lifetimes pre-expansion](https://github.com/rust-lang/rust/pull/126762)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference Tracking Issues or PRs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Async closures](https://github.com/rust-lang/rfcs/pull/3668)

## Upcoming Events

Rusty Events between 2024-07-03 - 2024-07-31 🦀

### Virtual
* 2024-06-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 2024-07-02 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 2024-07-02 | Hybrid - Virtual and In-person (Los Angeles, CA, US) | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/)
    * [**Rust LA Reboot**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 2024-07-03 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Build Web Apps with Rust and Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 2024-07-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328025/)
* 2024-07-04 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488820/)
* 2024-07-06 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-07-09 | Virtual | [Rust for Lunch](https://lunch.rs/)
    * [**July 2024 Rust for Lunch**](https://lunch.rs/meetups/2024-07-09/)
* 2024-07-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346976/)
* 2024-07-10 | Virtual | [Centre for eResearch](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Research Computing With The Rust Programming Language**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)
* 2024-07-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897842/)
* 2024-07-11 | Hybrid - Virtual and In-person (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/298076822/)
* 2024-07-11 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Reading JSON files in Rust (English)**](https://www.meetup.com/code-mavens/events/301636580/)
* 2024-07-16 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Web development in Rust using Rocket - part 2 (English)**](https://www.meetup.com/code-mavens/events/301736709/)
* 2024-07-17 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488824/)
* 2024-07-23| Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-07-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Exploring Rust API Use Cases**](https://www.eventbrite.com/e/lunch-learn-exploring-rust-api-use-cases-tickets-928424531767)

### Asia
* 2024-06-30 | Kyoto, JP | [Kyoto Rust](https://www.meetup.com/kyoto-rust/)
    * [**Rust Talk: Cross Platform Apps**](https://www.meetup.com/kyoto-rust/events/301499550/)
* 2024-07-03 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/ja-JP/tokyo-rust-meetup/)
    * [**I Was Understanding WASM All Wrong!**](https://www.meetup.com/ja-JP/tokyo-rust-meetup/events/301807832/)

### Europe
* 2024-06-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288965/)
* 2024-06-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)
* 2024-07-10 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup - July**](https://www.meetup.com/reading-rust-workshop/events/301359031/)
* 2024-07-11 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (July 2024)**](https://www.meetup.com/rust-prague/events/301227195)
* 2024-07-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Building a REST API in Rust using Axum, SQLx and SQLite**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/301716171/)
* 2024-07-16 | Mannheim, DE | [Hackschool - Rhein-Neckar](https://www.meetup.com/hackschool-rhein-neckar)
    * [**Nix Your Bugs & Rust Your Engines #4**](https://www.meetup.com/hackschool-rhein-neckar/events/301504325/)
* 2024-07-23| Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-07-27 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #8: Einführung in Machine Learning mit Rust**](https://www.meetup.com/rust-meetup-augsburg/events/301642385/)

### North America
* 2024-06-26 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 2024-06-27 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613483/)
* 2024-06-27 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)
* 2024-06-27 | St. Louis, MO, US | [STl Rust](https://www.meetup.com/stl-rust/)
    * [**Meet and Greet Hacker session**](https://www.meetup.com/stl-rust/events/301321974/)
* 2024-07-02 | Hybrid - Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/)
    * [**Rust LA Reboot**](https://www.meetup.com/rust-los-angeles/events/301645611/)
* 2024-07-05 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch, July 5**](https://www.meetup.com/bostonrust/events/301549737/)
* 2024-07-11 | Hybrid - Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Programación de sistemas con Rust**](https://www.meetup.com/rust-mx/events/301740677/)
* 2024-07-11 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/301613495/)
* 2024-07-17 | Hybrid - Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : holding pattern**](https://www.meetup.com/music-city-rust-developers/events/301411794/)
* 2024-07-24 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygckbgc/)


### Oceania

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

> the compiler usually doesn't complain about \[you\] doing undefined behavior because it doesn't know that you're doing undefined behavior.

– [Nilstrieb on GitHub](https://github.com/rust-lang/rust/issues/125658#issuecomment-2135511362)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1589) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
