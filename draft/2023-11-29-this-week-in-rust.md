Title: This Week in Rust 523
Number: 523
Date: 2023-11-29
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

### Foundation

### Newsletters

### Project/Tooling Updates
* [Announcing axum 0.7.0](https://tokio.rs/blog/2023-11-27-announcing-axum-0-7-0)
* [Announcing general availability of the AWS SDK for Rust](https://aws.amazon.com/blogs/developer/announcing-general-availability-of-the-aws-sdk-for-rust/)
* [ripgrep 14.0.0](https://github.com/BurntSushi/ripgrep/releases/tag/14.0.0)
* [Improved Multithreading in wgpu - Arcanization Lands on Trunk](https://gfx-rs.github.io/2023/11/24/arcanization.html)
* [Introducing SALT: A Rust error tutor, seeking participants in a study of errors!](https://marketplace.visualstudio.com/items?itemName=kale-lab.salt)
* [Bionic GPT - Chat GPT replacement built in Rust](https://github.com/bionic-gpt/bionic-gpt)
* [cargo-run-bin: Why does everyone install crates globally?](https://dustinblackman.com/posts/why-does-everyone-install-crates-globally/)
* [CXX-Qt: On the Road to Stability, improved signals support and more in release 0.6](https://www.kdab.com/cxx-qt-0-6/)
* [Oatmeal: Terminal UI to chat with large language models (LLM) using different model backends, and integrations with your favourite editors!](https://dustinblackman.com/posts/oatmeal/)

### Observations/Thoughts
* [Project Goals](https://smallcultfollowing.com/babysteps/blog/2023/11/28/project-goals/)
* [poll_next](https://without.boats/blog/poll-next/)
* [video] [but what is 'a lifetime?](https://www.youtube.com/watch?v=gRAVZv7V91Q)
* [audio] [Recruiting in Rust with Cedric Sellmann](https://rustacean-station.org/episode/cedric-sellmann/)

### Rust Walkthroughs
* [Designing a SIMD Algorithm from Scratch](https://mcyoung.xyz/2023/11/27/simd-base64/)
* [Why Enums in Rust feel so much better](https://www.shuttle.rs/blog/2023/11/23/enums-in-rust)
* [How I Use Declarative Macros in Rust](https://flinect.com/blog/quick-tips-rust-declarative-macros)
* [Embassy on ESP: Getting Started](https://apollolabsblog.hashnode.dev/embassy-on-esp-getting-started)
* [Intro to LLVM and MLIR with Rust and Melior](https://edgarluque.com/blog/mlir-with-rust/)
* [Rust Course (Part 2) - YouTube](https://www.youtube.com/watch?v=Yj2aANykEgM) (Bahasa Indonesia).
* [Investigating crazy compile times](https://blog.adamchalmers.com/crazy-compile-time/)

### Research
* [Refinement Proofs in Rust Using Ghost Locks](https://arxiv.org/abs/2311.14452)

* [Semantic fuzzing of the Rust compiler and interpreter](https://ethz.ch/content/dam/ethz/special-interest/infk/inst-pls/plf-dam/documents/StudentProjects/MasterTheses/2023-Andy-Thesis.pdf)

### Miscellaneous
* [Read arbitrary YAML files in Rust](https://rust.code-maven.com/read-arbitrary-yaml)

* [Create a Lambda in Rust using Terraform](https://maahl.net/blog/rust-aws-lambda/)

## Crate of the Week

This week's crate is [tokio-graceful](https://docs.rs/tokio-graceful), a library for graceful shutdown of tokio-based async servers.

Thanks to [Glen De Cauwsemaecker](https://users.rust-lang.org/t/crate-of-the-week/2704/1266) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - [REFACTOR]: [Nuvei] MCA metadata validation](https://github.com/juspay/hyperswitch/issues/2910)
* [Hyperswitch - [Features]: [Noon] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2904)
* [Hyperswitch - [Features]: [Payme] Sync with Hyperswitch Reference](https://github.com/juspay/hyperswitch/issues/2906)
* [Hyperswitch - [BUG]: MCA metadata deserialization failures should be 4xx](https://github.com/juspay/hyperswitch/issues/2899)
* [Hyperswitch - [REFACTOR]: [Stripe] Error Message For Connector Implementation](https://github.com/juspay/hyperswitch/issues/2910)
* [Ockam - Library - Slim down the `NodeManagerWorker` for `node / node status`](https://github.com/build-trust/ockam/issues/6707)
* [Ockam - Command - refactor to use typed interfaces to implement commands for `kafka services`](https://github.com/build-trust/ockam/issues/6706)
* [Ockam - Library - Validate CBOR structs according to the cddl schema for `nodes/models/transport` and `nodes/models/workers`](https://github.com/build-trust/ockam/issues/6694)
* [r3bl-open-core - [tuify] API change the return type of `select_from_list()` 3](https://github.com/r3bl-org/r3bl-open-core/issues/200)
* [r3bl-open-core - Improve "Bug report" issue template](https://github.com/r3bl-org/r3bl-open-core/issues/248)
  
If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

405 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-20..2023-11-27

* [stabilize dyn upcasting coercion](https://github.com/rust-lang/rust/pull/118133) (RFC [#3324](https://rust-lang.github.io/rfcs/3324-dyn-upcasting.html))
* [add an experimental feature gate for function delegation](https://github.com/rust-lang/rust/pull/117978)
* [enable Rust to use the EHCont security feature of Windows](https://github.com/rust-lang/rust/pull/118013)
* [enable profiler in dist-powerpc64-linux](https://github.com/rust-lang/rust/pull/118100)
* [enable the Arm Cortex-A53 errata mitigation on aarch64-unknown-none](https://github.com/rust-lang/rust/pull/118095)
* [remove now deprecated target `x86_64-sun-solaris`](https://github.com/rust-lang/rust/pull/118091)
* [`EvalCtxt::commit_if_ok` don't inherit nested goals](https://github.com/rust-lang/rust/pull/118243)
* [`intercrate_ambiguity_causes`: handle self ty infer + reservation impls](https://github.com/rust-lang/rust/pull/118089)
* [add `Span` to `TraitBoundModifier`](https://github.com/rust-lang/rust/pull/118245)
* [add `debug_assert_nounwind` and convert `assert_unsafe_precondition`](https://github.com/rust-lang/rust/pull/110303)
* [add allow-by-default lint for unit bindings](https://github.com/rust-lang/rust/pull/112380)
* [allow defining opaques in `check_coroutine_obligations`](https://github.com/rust-lang/rust/pull/118161)
* [call `FileEncoder::finish` in rmeta encoding](https://github.com/rust-lang/rust/pull/117301)
* [coverage: simplify building coverage expressions based on sums](https://github.com/rust-lang/rust/pull/117651)
* [do not erase late bound regions when selecting inherent associated types](https://github.com/rust-lang/rust/pull/118118)
* [don't ICE when ambiguity is found when selecting `Index` implementation in typeck](https://github.com/rust-lang/rust/pull/118112)
* [don't ICE when encountering placeholders in implied bounds computation](https://github.com/rust-lang/rust/pull/118290)
* [don't consider generic args of supertrait in `deref_into_dyn_supertrait` lint](https://github.com/rust-lang/rust/pull/118026)
* [don't require intercrate mode for negative coherence](https://github.com/rust-lang/rust/pull/117992)
* [eagerly compute `output_filenames`](https://github.com/rust-lang/rust/pull/117584)
* [fix early param lifetimes in `generic_const_exprs`](https://github.com/rust-lang/rust/pull/118035)
* [fixes error count display is different when there's only one error left](https://github.com/rust-lang/rust/pull/118138)
* [improve tool-only help for multiple `#[default]` variants](https://github.com/rust-lang/rust/pull/118131)
* [make PlaceholderReplacer `shallow_resolver` and recur when infer vars](https://github.com/rust-lang/rust/pull/118261)
* [note about object lifetime defaults in does not live long enough error](https://github.com/rust-lang/rust/pull/117835)
* [print query map for deadlock when using parallel front end](https://github.com/rust-lang/rust/pull/118169)
* [relate Inherent Associated Types using eq](https://github.com/rust-lang/rust/pull/118262)
* [remove `--check-cfg` checking of command line `--cfg` args](https://github.com/rust-lang/rust/pull/117522)
* [remove `HirId` from `QPath::LangItem`](https://github.com/rust-lang/rust/pull/118199)
* [separate `NaN`/`Inf` floats with `_`](https://github.com/rust-lang/rust/pull/118271)
* [suggest swapping the order of `ref` and `box`](https://github.com/rust-lang/rust/pull/118359)
* [the unadjusted ABI needs to pass aggregates by-value](https://github.com/rust-lang/rust/pull/118127)
* [tighten up link attributes for llvm-wrapper bindings](https://github.com/rust-lang/rust/pull/118142)
* [typeck break expr even if break is illegal](https://github.com/rust-lang/rust/pull/118010)
* [use an absolute path to the NUL device](https://github.com/rust-lang/rust/pull/118060)
* [when failing to import `core`, suggest `std`](https://github.com/rust-lang/rust/pull/118065)
* [add `VarDebugInfo` to Stable MIR](https://github.com/rust-lang/rust/pull/117972)
* [add support for global allocation in smir](https://github.com/rust-lang/rust/pull/118012)
* [fix smir's `Ty::Ref` pretty printing](https://github.com/rust-lang/rust/pull/118274)
* [expand Miri's BorTag GC to a Provenance GC](https://github.com/rust-lang/rust/pull/118029)
* [validate there are no critical call edges in optimized MIR](https://github.com/rust-lang/rust/pull/118075)
* [miri: GC the `dead_alloc_map` too](https://github.com/rust-lang/rust/pull/118073)
* [miri: check that target features required by LLVM intrinsics are enabled](https://github.com/rust-lang/miri/pull/3180)
* [miri: refactor `float_to_int_checked` to remove its generic parameter and reduce code duplication a bit](https://github.com/rust-lang/miri/pull/3185)
* [cache flags for `ty::Const`](https://github.com/rust-lang/rust/pull/118189)
* [indicate that multiplication in `Layout::array` cannot overflow](https://github.com/rust-lang/rust/pull/118228)
* [rewrite exhaustiveness in one pass](https://github.com/rust-lang/rust/pull/117611)
* [`AmbiguityCause` should not eagerly format strings](https://github.com/rust-lang/rust/pull/118267)
* [specialize `SpecFromElem` for `()`](https://github.com/rust-lang/rust/pull/118094)
* [refactor `binary_search_by` to use conditional moves](https://github.com/rust-lang/rust/pull/117722)
* [stabilize `ptr::addr_eq`](https://github.com/rust-lang/rust/pull/117968)
* [add `BufRead::skip_until`](https://github.com/rust-lang/rust/pull/98943)
* [kmc-solid: I/O safety](https://github.com/rust-lang/rust/pull/115159)
* [add `Duration::abs_diff`](https://github.com/rust-lang/rust/pull/117619)
* [non null convenience ops](https://github.com/rust-lang/rust/pull/117697)
* [hashbrown: specialize `fold` implementation of iterators](https://github.com/rust-lang/hashbrown/pull/480)
* [cranelift: implement another batch of vendor intrinsics](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1431)
* [cargo: `refactor(toml)`: Better abstract inheritance details](https://github.com/rust-lang/cargo/pull/13021)
* [cargo: exited with hard error when custom build file no existence or not in package](https://github.com/rust-lang/cargo/pull/12995)
* [cargo: add `CARGO_RUSTC_CURRENT_DIR` (unstable)](https://github.com/rust-lang/cargo/pull/12996)
* [cargo: use custom error instead of anyhow](https://github.com/rust-lang/cargo/pull/13050)
* [cargo: review and remove ignored tests in rustfix](https://github.com/rust-lang/cargo/pull/13047)
* [cargo: try running on windows](https://github.com/rust-lang/cargo/pull/13042)
* [rustdoc-search: add support for traits and associated types](https://github.com/rust-lang/rust/pull/116085)
* [rustdoc-search: avoid infinite where clause unbox](https://github.com/rust-lang/rust/pull/118251)
* [rustdoc-search: simplify `checkPath` and `sortResults`](https://github.com/rust-lang/rust/pull/118109)
* [rustdoc: sort unstable items last instead of first](https://github.com/rust-lang/rust/pull/118224)
* [rustfix: add notice that rustfix has moved](https://github.com/rust-lang/rustfix/pull/225)
* [clippy: `TypeckResults::node_type()` can be used inside of bodies](https://github.com/rust-lang/rust-clippy/pull/11877)
* [clippy: `deprecated_semver`: Allow `#[deprecated(since = "TBD")]`](https://github.com/rust-lang/rust-clippy/pull/11850)
* [clippy: `manual_try_fold`: check that `fold` is really `Iterator::fold`](https://github.com/rust-lang/rust-clippy/pull/11879)
* [clippy: `missing_asserts_for_indexing`: work with bodies instead of blocks separately](https://github.com/rust-lang/rust-clippy/pull/11859)
* [clippy: `needless_return_with_question_mark`: don't lint if never type is used for coercion](https://github.com/rust-lang/rust-clippy/pull/11627)
* [clippy: `ptr_arg`: recognize methods that also exist on slices](https://github.com/rust-lang/rust-clippy/pull/11817)
* [clippy: add new `check_private_items` config](https://github.com/rust-lang/rust-clippy/pull/11842)
* [clippy: create new lint `option_map_or_err_ok`](https://github.com/rust-lang/rust-clippy/pull/11864)
* [clippy: don't suggest `a.mul_add(b, c)` if parameters are not float](https://github.com/rust-lang/rust-clippy/pull/11836)
* [clippy: extend `result_map_or_into_option` lint to handle `Result::map_or_else(|_| None, Some)`](https://github.com/rust-lang/rust-clippy/pull/11845)
* [clippy: fix `box_default` behaviour with empty `vec![]` coming from macro arg](https://github.com/rust-lang/rust-clippy/pull/11875)
* [clippy: fix `iter_kv_map` false positive `into_keys` and `into_values` suggestion](https://github.com/rust-lang/rust-clippy/pull/11757)
* [clippy: improve error messages format](https://github.com/rust-lang/rust-clippy/pull/11860)
* [clippy: remove underscore check for `manual_non_exhaustive` lint](https://github.com/rust-lang/rust-clippy/pull/11844)
* [clippy: suggest alternatives to iterate an array of ranges](https://github.com/rust-lang/rust-clippy/pull/11862)
* [clippy: use absolute path for `declare_tool_lint` in `declare_clippy_lint`](https://github.com/rust-lang/rust-clippy/pull/11870)
* [rust-analyzer: cancelable initialization](https://github.com/rust-lang/rust-analyzer/pull/15894)
* [rust-analyzer: editor/code: add option to suppress internal error notifications](https://github.com/rust-lang/rust-analyzer/pull/15846)
* [rust-analyzer: ensure renames happen after edit](https://github.com/rust-lang/rust-analyzer/pull/15940)
* [rust-analyzer: fix variant resolve for type alias](https://github.com/rust-lang/rust-analyzer/pull/15970)
* [rust-analyzer: fix: add fallback for completion label details](https://github.com/rust-lang/rust-analyzer/pull/15962)
* [rust-analyzer: fix: better resolve assoc item with type bound](https://github.com/rust-lang/rust-analyzer/pull/15825)
* [rust-analyzer: fix: dedup duplicate crates with differing origins in CrateGraph construction](https://github.com/rust-lang/rust-analyzer/pull/15754)
* [rust-analyzer: fix: remove parenthesis should ensure space](https://github.com/rust-lang/rust-analyzer/pull/15857)
* [rust-analyzer: improve completion label details display](https://github.com/rust-lang/rust-analyzer/pull/15956)
* [rust-analyzer: replace `option.map(cond) == Some(true)` with `option.is_some_and(cond)`](https://github.com/rust-lang/rust-analyzer/pull/15960)

### Rust Compiler Performance Triage

A good week, despite a few PRs that pnkfelix opted not to mark as triaged. In
particular, a broad set of primary benchmarks improved, due to improvements to
resolve (PR #118188) and a one-pass rewrite of exhaustiveness (PR #117611).

Triage done by **@pnkfelix**.
Revision range: [4f3da903..df0295f0](https://perf.rust-lang.org/?start=4f3da903a43f22ea33d2ca4435a24b42fc1f842a&end=df0295f07175acc7325ce3ca4152eb05752af1f2&absolute=false&stat=instructions%3Au)

1 Regressions, 5 Improvements, 5 Mixed; 2 of them in rollups
84 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/20911b7f28a4b88d36ecd5b13414f26feac49d4d/triage/2023-11-28.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Macro fragment specifiers edition policy](https://github.com/rust-lang/rfcs/pull/3531)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [[rustdoc] Add highlighting for comments in items declaration](https://github.com/rust-lang/rust/pull/117869)
* [disposition: merge] [generalize: handle occurs check failure in aliases](https://github.com/rust-lang/rust/pull/117088)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: constants in patterns](https://github.com/rust-lang/rfcs/pull/3535)
* [Add RFC combining Infra and Release teams](https://github.com/rust-lang/rfcs/pull/3533)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-11-29 - 2023-12-27 🦀

### Virtual

* 2023-11-23 | Virtual (Edmonton, AB, CA) | [Edmonton R User Group - Yegrug](https://www.meetup.com/edmonton-r-user-group-yegrug/)
    * [**Edmonton R User Group Meetup: R and Rust, like a match made in heaven**](https://www.meetup.com/edmonton-r-user-group-yegrug/events/296605221/)
* 2023-11-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)
* 2023-11-28 | Virtual (Europe / Africa) | [Rust for Lunch](https://lunch.rs/)
    * [**Rust Meet-up**](https://lunch.rs/meetups/2023-11-28/)
* 2023-11-29 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Atomics & Locks Book Club Final Chapter! (Chapter 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 2023-11-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 2023-11-30 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Automating expertise with cargo-semver-checks**](https://www.meetup.com/rust-dublin/events/296346693/)
* 2023-12-01 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 2023-12-02 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 2023-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Mirror**](https://berline.rs/)
* 2023-12-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)
* 2023-12-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/297172140)
* 2023-12-12 | Virtual | [Mainmatter](https://mainmatter.com)
    * [**Workshop: Telemetry for Rust applications**](https://rust-telemetry-workshop.mainmatter.com)
* 2023-12-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcqbqb/)
* 2023-12-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 2023-12-18 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763506/)

### Europe

* 2023-11-23 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)
* 2023-11-28 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks November 2023 with Helsing.ai**](https://www.meetup.com/rust-london-user-group/events/297257712/)
* 2023-11-30 | Brussels, BE | [Lambda Brussels](https://lambda-brussels.glitch.me/)
    * [**Lambda Brussels**](https://lambda-brussels.glitch.me/)
* 2023-11-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #42 sponsored by Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/297405705/)
* 2023-11-30 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - November**](https://www.meetup.com/rust-vienna/events/297382145/)
* 2023-11-30 | Zurich, CH| [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**November Meetup**](https://www.meetup.com/rust-zurich/events/297312190/)
* 2023-12-06 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**December Meetup**](https://www.meetup.com/rustcologne/events/297100007/)
* 2023-12-07 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 2023-12-07 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 2023-12-14 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 2023-12-18 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, an Electron-alternative**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### North America

* 2023-11-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)
* 2023-11-28 | Pasadena, CA, US | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/297062186/)
* 2023-11-29 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/296657831/)
* 2023-12-12 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 2023-12-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)

### Oceania

* 2023-11-28 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/296391733/)
* 2023-12-05 | Aukland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Advanced Async Tricks + Interruptible Software**](https://www.meetup.com/rust-akl/events/297271684/)
* 2023-12-11 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust End of Year Event**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

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

> I'd like to report that Rust's compile times were OK today and yesterday and the day before.
>
> I'll keep you posted.

– [ZiCog about slow Rust compile times on rust-users](https://users.rust-lang.org/t/is-rust-compile-time-really-that-slow/102863/15)

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1491) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
