Title: This Week in Rust 514
Number: 514
Date: 2023-09-27
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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
* [Increasing the minimum supported Apple platform versions](https://blog.rust-lang.org/2023/09/25/Increasing-Apple-Version-Requirements.html)
* [crates.io Policy Update RFC](https://blog.rust-lang.org/2023/09/22/crates-io-usage-policy-rfc.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [cargo-binstall Release v1.4.1](https://github.com/cargo-bins/cargo-binstall/releases/tag/v1.4.1)

### Observations/Thoughts
* [Interacting with AWS IAM services using the Rust SDK and stand-alone CLI](https://sanjuvi.github.io/Blog/posts/Aws-Iam/)
* [My Pure Rust Wishlist](https://gburghoorn.com/posts/pure-rust-wishlist/)
* [Qualifying Rust without forking](https://ferrous-systems.com/blog/qualifying-rust-without-forking/)
* [Why Not Rust Crypto?](https://briansmith.org/why-not-rustcrypto)
* [Eyra is an interesting Rust project](https://notgull.net/eyra/)
* [Generic trait methods and new auto traits](https://without.boats/blog/generic-trait-methods-and-new-auto-traits/)
* [Rust for Mission Critical Applications](https://ferrous-systems.com/blog/rust-for-mission-critical-applications/)
* [Polonius revisited, part 1](https://smallcultfollowing.com/babysteps/blog/2023/09/22/polonius-part-1/)
* [Scaling Rust Adoption Through Training](https://security.googleblog.com/2023/09/scaling-rust-adoption-through-training.html)

### Rust Walkthroughs
* [Hello, Rust!](https://ariel-miculas.github.io/Hello-Rust/)
* [Rust: The joy of safe zero-copy parsers](https://itnext.io/rust-the-joy-of-safe-zero-copy-parsers-8c8581db8ab2)
* [Cross-compiling Rust on Github Actions](https://blog.timhutt.co.uk/cross-compiling-rust/)
* [video] [Build Your First Game in Bevy and Rust](https://www.youtube.com/watch?v=E9SzRc9HkOg)
* [video] [`no_std: ?no_problem`](https://www.youtube.com/watch?v=Oq-3bOBrpeo)

### Research
* [How do Rust developers use unsafe? - Survey](https://cmu.ca1.qualtrics.com/jfe/form/SV_0k7naTSSk8jaaGi)

### Miscellaneous
* [Using Kani to write and validate Rust code with ChatGPT](https://blog.logrocket.com/using-kani-write-validate-rust-code-chatgpt/)
* [Logging in Rust - How to Get Started](https://www.shuttle.rs/blog/2023/09/20/logging-in-rust)
* [Profiling with perf and DHAT on Rust code in Linux](https://www.justanotherdot.com/posts/profiling-with-perf-and-dhat-on-rust-code-in-linux.html)
* [Adding runtime benchmarks to the Rust compiler benchmark suite](https://kobzol.github.io/rust/rustc/2023/09/23/rustc-runtime-benchmarks.html)
* [ESP Embedded Rust: Multithreading with FreeRTOS Bindings](https://apollolabsblog.hashnode.dev/esp-embedded-rust-multithreading-with-freertos-bindings)
* [A JVM in Rust part 6 - Methods and exceptions](https://andreabergia.com/blog/2023/09/a-jvm-in-rust-part-6-methods-and-exceptions/

## Crate of the Week

This week's crate is [async_fn_traits](https://docs.rs/async_fn_traits), a crate with async function traits to enable using higher ranked trait bounds in async functions.

Thanks to [kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1239) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.


<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - add multiple insert and delete functionality for in-memory cache](https://github.com/juspay/hyperswitch/issues/2373)
* [Hyperswitch - add metrics to cache invalidation miss](https://github.com/juspay/hyperswitch/issues/2377)
* [Hyperswitch - separate payments_session from payments core](https://github.com/juspay/hyperswitch/issues/888)
* [nix-rust - next generation maintainers](https://github.com/nix-rust/nix/issues/2132)
* [DeepCausality - Increase Test Coverage](https://github.com/deepcausality-rs/deep_causality/issues/104)
* [DeepCausality - Polish documentation](https://github.com/deepcausality-rs/sites/issues/71)
* [rust-libp2p - replace `custom-clippy` alias with workspace-wide `[lints]` configuration](https://github.com/libp2p/rust-libp2p/issues/4484)
* [rust-libp2p - Prefer tokio runtime everywhere](https://github.com/libp2p/rust-libp2p/issues/4449)
* [rust-libp2p - misc: make `RwStreamSink` an implementation detail of the memory transport](https://github.com/libp2p/rust-libp2p/issues/4345)
* [time - Missing panic condition on API docs](https://github.com/time-rs/time/issues/623)
* [time - Add comparison to chrono to the docs](https://github.com/time-rs/time/issues/308)
* [time - The Book is hardly readable](https://github.com/time-rs/time/issues/327)
* [Ockam - `ockam status` clap command should provide more user friendly information when a user is *not* enrolled](https://github.com/build-trust/ockam/issues/6064)
* [Ockam - `ockam status` clap command should provide more user friendly information when a user is enrolled](https://github.com/build-trust/ockam/issues/6063)
* [Ockam - Improve `ockam enroll ----help` clap command text by adding doc comment for `verbose` flag](https://github.com/build-trust/ockam/issues/6055)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

402 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-09-18..2023-09-25

* [*breaking change*: Validate crate name in `--extern`](https://github.com/rust-lang/rust/pull/116001)
* [add support for GNU/Hurd](https://github.com/rust-lang/rust/pull/115230)
* [enable ASAN/LSAN/TSAN for *-apple-ios-macabi](https://github.com/rust-lang/rust/pull/115644)
* [raise minimum supported Apple OS versions](https://github.com/rust-lang/rust/pull/104385)
* [`rustc_target/loongarch`: Fix passing of transparent unions with only one non-ZST member](https://github.com/rust-lang/rust/pull/115987)
* [`rustc_target/riscv`: Fix passing of transparent unions with only one non-ZST member](https://github.com/rust-lang/rust/pull/115499)
* [`rustc_hir_analysis`: add a helper to check function the signature mismatches](https://github.com/rust-lang/rust/pull/115897)
* [account for nested `impl Trait` in TAIT](https://github.com/rust-lang/rust/pull/116039)
* [add minimal std implementation for UEFI](https://github.com/rust-lang/rust/pull/105861)
* [add OwnedTargetMachine to manage llvm:TargetMachine](https://github.com/rust-lang/rust/pull/115911)
* [add Zba, Zbb, and Zbs as target features for riscv64-linux-android](https://github.com/rust-lang/rust/pull/116076)
* [add initial libstd support for Xous](https://github.com/rust-lang/rust/pull/104101)
* [adjust `ConstValue::Slice` to work for arbitrary slice types](https://github.com/rust-lang/rust/pull/115870)
* [adjust how closure/generator types are printed](https://github.com/rust-lang/rust/pull/115696)
* [allow `-Z treat-err-as-bug=0`](https://github.com/rust-lang/rust/pull/115690)
* [allow anyone to set llvm-fixed-upstream](https://github.com/rust-lang/rust/pull/115990)
* [allow higher-ranked fn sigs in `ValuePairs`](https://github.com/rust-lang/rust/pull/116073)
* [capture scrutinee of if let guards correctly](https://github.com/rust-lang/rust/pull/115999)
* [check that closure/generator's interior/capture types are sized](https://github.com/rust-lang/rust/pull/116081)
* [command: also print removed env vars](https://github.com/rust-lang/rust/pull/114379)
* [correctly deny late-bound lifetimes from parent in anon consts and TAITs](https://github.com/rust-lang/rust/pull/115486)
* [coverage: don't bother renumbering expressions on the Rust side](https://github.com/rust-lang/rust/pull/114399)
* [coverage: fix an unstable-sort inconsistency in coverage spans](https://github.com/rust-lang/rust/pull/115930)
* [coverage: remove debug code from the instrumentor](https://github.com/rust-lang/rust/pull/115962)
* [dependencies: reduce the amount of crates pulling in atty](https://github.com/rust-lang/rust/pull/115975)
* [detect cycle errors hidden by opaques during monomorphization](https://github.com/rust-lang/rust/pull/115801)
* [diagnostics: avoid mismatch between variance index and hir generic](https://github.com/rust-lang/rust/pull/116045)
* [do not create a DerefLen place for `Box<[T]>`](https://github.com/rust-lang/rust/pull/115794)
* [don't ICE when no bound vars found while doing closure hir type check](https://github.com/rust-lang/rust/pull/113396)
* [don't complain on a single non-exhaustive 1-ZST](https://github.com/rust-lang/rust/pull/115924)
* [don't modify libstd to dump rustc ICEs](https://github.com/rust-lang/rust/pull/115627)
* [don't resolve generic impls that may be shadowed by dyn built-in impls](https://github.com/rust-lang/rust/pull/114941)
* [enable -Zdrop-tracking-mir by default](https://github.com/rust-lang/rust/pull/107421)
* [enable effects for libcore](https://github.com/rust-lang/rust/pull/114776)
* [fall back to `_SC_NPROCESSORS_ONLN` if `sched_getaffinity` returns an empty mask](https://github.com/rust-lang/rust/pull/116038)
* [fall back to the unoptimized implementation in `read_binary_file` if `File::metadata` lies](https://github.com/rust-lang/rust/pull/115549)
* [fix `ui-fulldeps --stage=1` with `-Zignore-directory-in-diagnostics-source-blocks`](https://github.com/rust-lang/rust/pull/116009)
* [fix confusing let chain indentation in `rustc_resolve`](https://github.com/rust-lang/rust/pull/115983)
* [fix debug printing of tuple](https://github.com/rust-lang/rust/pull/116069)
* [give FutureIncompatibilityReason variants more explicit names](https://github.com/rust-lang/rust/pull/116049)
* [implement `Literal::byte_character`](https://github.com/rust-lang/rust/pull/112711)
* [implement `intercrate_ambiguity_causes` in the new solver](https://github.com/rust-lang/rust/pull/115996)
* [improve invalid UTF-8 lint by finding the expression initializer](https://github.com/rust-lang/rust/pull/115257)
* [interpret: more consistently use ImmTy in operators and casts](https://github.com/rust-lang/rust/pull/116010)
* [make unsized casts illegal](https://github.com/rust-lang/rust/pull/116056)
* [match on elem first while building move paths](https://github.com/rust-lang/rust/pull/115770)
* [more accurate suggestion for `self.` and `Self:`:](https://github.com/rust-lang/rust/pull/116086)
* [move `DepKind` to `rustc_query_system` and define it as `u16`](https://github.com/rust-lang/rust/pull/115920)
* [pass name of object file to LLVM so it can correctly emit `S_OBJNAME` in pdb files on Windows](https://github.com/rust-lang/rust/pull/115704)
* [point at cause of expectation of `break` value when possible](https://github.com/rust-lang/rust/pull/116071)
* [prevent promotion of const fn calls in inline consts](https://github.com/rust-lang/rust/pull/115936)
* [suggest desugaring to return-position `impl Future` when an `async fn` in trait fails an auto trait bound](https://github.com/rust-lang/rust/pull/115864)
* [tweak expected message to explain what it's actually signifying](https://github.com/rust-lang/rust/pull/116082)
* [miri: GC the Stacked Borrows allocation history](https://github.com/rust-lang/miri/pull/3083)
* [miri: deprecate -Zmiri-disable-abi-check](https://github.com/rust-lang/miri/pull/3071)
* [miri: implement `llvm.ctpop.v*` intrinsics](https://github.com/rust-lang/miri/pull/3072)
* [miri: issue discovered in TB: spurious reads are not (yet) possible in a concurrent setting](https://github.com/rust-lang/miri/pull/3054)
* [miri: move `llvm.x86.*` shims into `shims::x86` and implement `_addcarry_u32` and `_subborrow_u{32,64}`](https://github.com/rust-lang/miri/pull/3075)
* [open the FileEncoder file for reading and writing](https://github.com/rust-lang/rust/pull/116067)
* [simplify/Optimize FileEncoder](https://github.com/rust-lang/rust/pull/115542)
* [avoid overflow in `IoSlice::advance_slices`](https://github.com/rust-lang/rust/pull/116070)
* [call `panic_display` directly in `const_panic_fmt`](https://github.com/rust-lang/rust/pull/116007)
* [implement `cstr_count_bytes`](https://github.com/rust-lang/rust/pull/114443)
* [panic when encountering an illegal cpumask in `thread::available_parallelism`](https://github.com/rust-lang/rust/pull/115946)
* [add the `cfg_match!` macro](https://github.com/rust-lang/rust/pull/115416)
* [cargo: add some enhancements to `cargo clean`](https://github.com/rust-lang/cargo/pull/12638)
* [cargo: better suggestion for redundant mode in build and install commands](https://github.com/rust-lang/cargo/pull/12693)
* [cargo: buffer console status messages](https://github.com/rust-lang/cargo/pull/12727)
* [cargo: cargo add displays either feature list or summarized count](https://github.com/rust-lang/cargo/pull/12702)
* [cargo: doc: mention unstable flag `-Z asymmetric-token`](https://github.com/rust-lang/cargo/pull/12712)
* [cargo: fix spurious errors with networking tests](https://github.com/rust-lang/cargo/pull/12726)
* [cargo: fix: copy PDBs for EFI targets](https://github.com/rust-lang/cargo/pull/12688)
* [cargo: fix: use channel-specific link for registry auth error](https://github.com/rust-lang/cargo/pull/12709)
* [cargo: infra: add auto-trigger rules for new labels](https://github.com/rust-lang/cargo/pull/12713)
* [cargo: more specific registry index not found msg](https://github.com/rust-lang/cargo/pull/12732)
* [cargo: shortest path](https://github.com/rust-lang/cargo/pull/12678)
* [rustdoc-search: add support for type parameters](https://github.com/rust-lang/rust/pull/112725)
* [rustdoc: correctly render the return type of cross-crate async fns](https://github.com/rust-lang/rust/pull/116084)
* [rustdoc: custom code classes in docs warning](https://github.com/rust-lang/rust/pull/115947)
* [rustfmt: bugfix/comment duplication](https://github.com/rust-lang/rustfmt/pull/5913)
* [clippy: `redundant_guards`: lint if the pattern is on the left side](https://github.com/rust-lang/rust-clippy/pull/11522)
* [clippy: change defaults of `accept-comment-above-statement` and `accept-comment-above-attributes`](https://github.com/rust-lang/rust-clippy/pull/11170)
* [clippy: fix false positive with `needless_raw_string_hashes`](https://github.com/rust-lang/rust-clippy/pull/11518)
* [clippy: fix `cast_lossless` with macro call](https://github.com/rust-lang/rust-clippy/pull/11516)
* [clippy: fix mutably used async function argument in closure for `needless_pass_by_ref_mut`](https://github.com/rust-lang/rust-clippy/pull/11492)
* [clippy: fixed  caused by moving &mut reference inside of a closure](https://github.com/rust-lang/rust-clippy/pull/11551)
* [clippy: prevent ice when threshold is 0 and `enum` has no variants](https://github.com/rust-lang/rust-clippy/pull/11552)
* [clippy: remove most usage of `hir_ty_to_ty`](https://github.com/rust-lang/rust-clippy/pull/11544)
* [rust-analyzer: add `unused_variables` native diagnostic](https://github.com/rust-lang/rust-analyzer/pull/15659)
* [rust-analyzer: add option to show full function signatures in completion docs](https://github.com/rust-lang/rust-analyzer/pull/15582)
* [rust-analyzer: deunwrap `add_missing_match_arms`](https://github.com/rust-lang/rust-analyzer/pull/15594)
* [rust-analyzer: do not resolve inlayHint.textEdit for VSCode client](https://github.com/rust-lang/rust-analyzer/pull/15635)
* [rust-analyzer: bool to `enum` assist](https://github.com/rust-lang/rust-analyzer/pull/15484)
* [rust-analyzer: fix autoimport does nothing when importing trait that is as `_` imports](https://github.com/rust-lang/rust-analyzer/pull/15587)
* [rust-analyzer: fix inlining closures from local variables and functions](https://github.com/rust-lang/rust-analyzer/pull/15651)
* [rust-analyzer: give `unmerge_use` a label explaining what it will affect](https://github.com/rust-lang/rust-analyzer/pull/15621)

### Rust Compiler Performance Triage

A very quiet week with the only large change in performance being improvements brought on by @saethlin's work on cleaning up the `FileEncoder` used in various places like `rustc_metadata` and `rustc_serialize`.

Triage done by **@rylev**.
Revision range: [af78bae..27b4eb9](https://perf.rust-lang.org/?start=af78bae565e85b9c5698ee909af0652674eca6d4&end=27b4eb96d13106332d511be2ea6d0c008a57aa6e&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.3%, 1.1%]   | 15    |
| Regressions ❌ <br /> (secondary)  | 2.0%  | [0.2%, 7.1%]   | 32    |
| Improvements ✅ <br /> (primary)   | -0.7% | [-1.3%, -0.3%] | 70    |
| Improvements ✅ <br /> (secondary) | -0.9% | [-3.5%, -0.2%] | 31    |
| All ❌✅ (primary)                 | -0.4% | [-1.3%, 1.1%]  | 85    |


2 Regressions, 3 Improvements, 4 Mixed; 0 of them in rollups
73 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-09-26.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add Zulip notifications for cargo and crates-io](https://github.com/rust-lang/rfcs/pull/3495)
* [Cargo Check T-lang Policy](https://github.com/rust-lang/rfcs/pull/3477)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Rework negative coherence to properly consider impls that only partly overlap](https://github.com/rust-lang/rust/pull/112875)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Lifetime Capture Rules 2024](https://github.com/rust-lang/rfcs/pull/3498)
* [new] [RFC: `typed-context-injection`](https://github.com/rust-lang/rfcs/pull/3496)
* [new] [RFC: Precise Pre-release `cargo update`](https://github.com/rust-lang/rfcs/pull/3493)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-09-27 - 2023-10-25 🦀

### Virtual

* 2023-10-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295919493/)
* 2023-10-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-04 | Virtual (Various) | [Ferrous Systems](https://www.eventbrite.com/o/ferrous-systems-gmbh-68735392123)
    * [**A Decade of Rust with Ferrous Systems**](https://www.eventbrite.com/e/a-decade-of-rust-with-ferrous-systems-tickets-680492891557?aff=ebdssbdestsearch)
* 2023-10-04 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Understanding the Processor (Atomics & Locks Chapter 7)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296278202/)
* 2023-10-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296135640/)
* 2023-10-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup: Mentorship (First Saturday)**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763617907?aff=erelpanelorg)
* 2023-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/) | [**Mirror**](https://berline.rs/)
* 2023-10-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcnbnb/)
* 2023-10-11| Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcnbpb/)
* 2023-10-12 - 2023-10-13 | Virtual (Brussels, BE) | [EuroRust](https://eurorust.eu)
    * [**EuroRust 2023**](https://eurorust.eu)
* 2023-10-12 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732662/)
* 2023-10-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057159/)
* 2023-10-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcnbzb/)
* 2023-10-19 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679778/) | [**Mirror**](https://berline.rs/)
 
### Asia

* 2023-10-03 | Taipei, TW | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/)
    * [**WebAssembly Meetup (Wasm Empowering AI) in Taipei**](https://www.meetup.com/wasm-rust-meetup/events/295672575/)

### Europe

* 2023-09-28 | Berlin, DE | [React Berlin](https://www.meetup.com/react-berlin-meetup/)
    * [**React Berlin September Meetup: Creating Videos with React & Remotion & More: Integrating Rust with React Native – Gheorghe Pinzaru**](https://www.meetup.com/react-berlin-meetup/events/295382108/)
* 2023-09-28 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/)
    * [**Primer evento Post COVID: ¡Cervezas MadRust!**](https://www.meetup.com/madrust/events/296063394/)
* 2023-09-28 | Paris, FR | [Paris Scala User Group (PSUG)](https://www.meetup.com/paris-scala-user-group-psug/events/296025234/)
    * [**PSUG #114 Comparons Scala et Rust**](https://www.meetup.com/paris-scala-user-group-psug/events/296025234/)
* 2023-09-28 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Dojo 2**](https://www.meetup.com/rust-vienna/events/296254546/)
* 2023-09-30 | Saint Petersburg, RU | [Rust Saint Petersburg meetups](https://t.me/ruRust_spb)
    * [**Rust Community Meetup: A tale about how I tried to make my Blitz Basic - Vitaly; How to use nix to build projects on Rust – Danil; Getting to know tower middleware. General overview – Mikhail**](https://rurust-saint-petersburg-m.timepad.ru/event/2561864/) 
* 2023-10-04 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #6**](https://www.meetup.com/fr-FR/rust-lyon/events/296186641/)
* 2023-10-10 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/)
* 2023-10-12 - 2023-10-13 | Brussels, BE | [EuroRust](https://eurorust.eu)
    * [**EuroRust 2023**](https://eurorust.eu)
* 2023-10-12 | Brussels, BE | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - EuroRust Conference**](https://www.meetup.com/rust-aarhus/events/295673220/)
* 2023-10-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295955356/)
* 2023-10-17 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**Helsinki Rustaceans Meetup**](https://www.meetup.com/finland-rust-meetup/events/295680333/)
* 2023-10-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**SIMD in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504251/)
* 2023-10-25 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Biome, web development tooling with Rust**](https://www.meetup.com/rust-dublin/events/295179534/)

### North America

* 2023-09-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295466314)
* 2023-09-28 | Boulder, CO, US | [Solid State Depot - The Boulder Makerspace](https://www.meetup.com/solidstatedepot/)
    * [**Rust and ROS for Robotics + Happy Hour**](https://www.meetup.com/solidstatedepot/events/295466122/)
* 2023-10-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369949/)
* 2023-10-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**First Meetup - Demo Day and Office Hours**](https://www.meetup.com/boulder-rust-meetup/events/296193722/)
* 2023-10-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**The Actor Model: Fearless Concurrency, Made Easy w/Chris Mena**](https://www.meetup.com/utah-rust/events/295771376/)
* 2023-10-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcnbwb/)
* 2023-10-19 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369976/)
* 2023-10-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust Meetup : Getting to Know search other**](https://www.meetup.com/music-city-rust-developers/events/296254420/)

### Oceania

* 2023-09-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**September Meetup**](https://www.meetup.com/rust-brisbane/events/295946587/)

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

> The problem with Rust it appears,  
> that it leaves programmers in tears  
> if they have to go back  
> to languages that lack  
> in short they've got feature-arrears.

– [llogiq on /r/rust](https://www.reddit.com/r/rust/comments/16mv8bb/comment/k1buhp0/)

Thanks to [Frank Steffahn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1468) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
