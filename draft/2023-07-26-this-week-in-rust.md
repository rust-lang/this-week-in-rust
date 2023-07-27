Title: This Week in Rust 505
Number: 505
Date: 2023-07-26
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
* [crates.io Postmortem: Broken Crate Downloads](https://blog.rust-lang.org/inside-rust/2023/07/21/crates-io-postmortem.html)
* [July 2023 Leadership Council Update](https://blog.rust-lang.org/inside-rust/2023/07/25/leadership-council-update.html)

### Foundation
* [My first three months at the Rust Foundation](https://foundation.rust-lang.org/news/three-months-at-the-rust-foundation/)

### Newsletters

### Project/Tooling Updates
* [IntelliJ Rust Changelog #199](https://intellij-rust.github.io/2023/07/24/changelog-199.html)
* [Fyrox Game Engine 0.31](https://fyrox.rs/blog/post/fyrox-game-engine-0-31/)
* [Writing a Rest HTTP Service with Axum](https://docs.shuttle.rs/tutorials/rest-http-service-with-axum)

### Observations/Thoughts
* [Totality](https://blog.yoshuawuyts.com/totality/)
* [Why the “Null” Lifetime Does Not Exist](https://sabrinajewson.org/blog/null-lifetime)
* [First report: More than 80% of the crates link to their public VCS](https://rust-digger.code-maven.com/news/first-report)
* [video] [Aram Drevekenin – Zellij - A terminal workspace with batteries included](https://www.youtube.com/watch?v=nBL_8KsTA5g)

### Rust Walkthroughs
* [Make invalid states unrepresentable](https://geeklaunch.io/blog/make-invalid-states-unrepresentable/)
* [A visual tree iterator in Rust](https://blog.danieljanus.pl/2023/07/20/iterating-trees/)
* [A gentle introduction to IMAP](https://duesee.dev/p/a-gentle-introduction-to-imap/)
* [ESP32 Standard Library Embedded Rust: UART Communication](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-uart-communication)
* [Shuttle Launchpad #3: Sudoku, Ownership and Error Handling](https://www.shuttle.rs/launchpad/issues/2023-03-07-issue-03-Ownership-Error-Handling)
* [video] [Pragmatic Intro to Rust Web Development: Overview](https://www.youtube.com/watch?v=ZdStbqp0400&list=PL4bcKu1Oi0f5eYmYJAOTYLGN7Lgr_B7mZ)

### Research

* [ResourceGauge: Enabling Resource-Aware Software Components (with Rust)](https://www.ecrts.org/wp-content/uploads/2023/07/ospert23-proceedings.pdf#page=12)

### Miscellaneous
* [How to speed up the Rust compiler: data analysis assistance requested!](https://nnethercote.github.io/2023/07/25/how-to-speed-up-the-rust-compiler-data-analysis-assistance-requested.html)
* [Programming language popularity: Rust](https://szabgab.com/programming-language-popularity-rust)

## Crate of the Week

This week's crate is [tower-async](https://github.com/plabayo/tower-async), a currently nightly-only async library to build network servers, based on [tower](https://docs.rs/tower).

Thanks to [Glen De Cauwsemaecker](https://users.rust-lang.org/t/crate-of-the-week/2704/1218) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - Migrate to enum_dispatch to reduce runtime overhead](https://github.com/juspay/hyperswitch/issues/921)
* [Hyperswitch - Add Create Merchant and Create Merchant Key Store in a DB transaction](https://github.com/juspay/hyperswitch/issues/1793)
* [Hyperswitch - Use proxy exclusion instead of a separate proxied client](https://github.com/juspay/hyperswitch/issues/1039)
* [Hyperswitch - Add scoped error enum for customer error](https://github.com/juspay/hyperswitch/issues/1580)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

406 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-07-17..2023-07-24

* [add mips64r6 and mips32r6 as `target_arch` values](https://github.com/rust-lang/rust/pull/112374)
* [add `riscv64gc-unknown-hermit` target](https://github.com/rust-lang/rust/pull/114004)
* [add `x86_64-unknown-linux-ohos` target](https://github.com/rust-lang/rust/pull/113061)
* [enable chkstk/alloca intrinsics on `x86_64-unknown-uefi`](https://github.com/rust-lang/compiler-builtins/pull/541)
* [Support `.comment` section like GCC/Clang (`!llvm.ident`)](https://github.com/rust-lang/rust/pull/97550)
* [add Alias to smir](https://github.com/rust-lang/rust/pull/113943)
* [add BITS, `from_bits, to_bits` to IP addresses](https://github.com/rust-lang/rust/pull/113746)
* [add FnPtr ty to SMIR](https://github.com/rust-lang/rust/pull/113910)
* [add Foreign, Never, FnDef, Closure and Generator tys to SMIR](https://github.com/rust-lang/rust/pull/113827)
* [add `#[inline]` to core debug assertion helpers](https://github.com/rust-lang/rust/pull/113687)
* [add dynamic for smir](https://github.com/rust-lang/rust/pull/113969)
* [add the `no-builtins` attribute to functions when `no_builtins` is applied at the crate level](https://github.com/rust-lang/rust/pull/113716)
* [add ty convs for smir refs and ptrs](https://github.com/rust-lang/rust/pull/113659)
* [allow opaques to be defined by trait queries, again](https://github.com/rust-lang/rust/pull/113690)
* [always const-prop scalars and scalar pairs](https://github.com/rust-lang/rust/pull/113858)
* [avoid tls access while iterating through mpsc thread entries](https://github.com/rust-lang/rust/pull/113861)
* [better diagnostics for dlltool errors](https://github.com/rust-lang/rust/pull/112591)
* [comment stuff in the new solver](https://github.com/rust-lang/rust/pull/113987)
* [don't translate compiler-internal bug messages](https://github.com/rust-lang/rust/pull/113884)
* [encode shorthands for spans in metadata](https://github.com/rust-lang/rust/pull/113850)
* [error/E0691: include alignment in error message](https://github.com/rust-lang/rust/pull/113913)
* [fix `#[inline(always)]` on closures with target feature 1.1](https://github.com/rust-lang/rust/pull/111836)
* [fix `inline_const` with interpolated block](https://github.com/rust-lang/rust/pull/113803)
* [fix invalid display of inlined re-export when both local and foreign items are inlined](https://github.com/rust-lang/rust/pull/113785)
* [fix removal span calculation of `unused_qualifications` suggestion](https://github.com/rust-lang/rust/pull/113811)
* [fix results search alias display](https://github.com/rust-lang/rust/pull/113823)
* [fix rustc-args passing issue in bootstrap](https://github.com/rust-lang/rust/pull/113948)
* [get `!nonnull` metadata on slice iterators, without `assume`s](https://github.com/rust-lang/rust/pull/113344)
* [get rid of subst-relate incompleteness in new solver](https://github.com/rust-lang/rust/pull/113901)
* [improve error message when closing bracket interpreted as formatting fill character](https://github.com/rust-lang/rust/pull/113774)
* [inline overlap based CGU merging](https://github.com/rust-lang/rust/pull/113777)
* [lint/ctypes: only try normalize](https://github.com/rust-lang/rust/pull/113921)
* [new solver: add a separate cache for coherence](https://github.com/rust-lang/rust/pull/113887)
* [new solver: don't consider blanket impls multiple times](https://github.com/rust-lang/rust/pull/113835)
* [on nightly, dump ICE backtraces to disk](https://github.com/rust-lang/rust/pull/108714)
* [permit pre-evaluated constants in `simd_shuffle`](https://github.com/rust-lang/rust/pull/113529)
* [prototype: add unstable `-Z reference-niches` option](https://github.com/rust-lang/rust/pull/113166)
* [querify unused trait check](https://github.com/rust-lang/rust/pull/113546)
* [refactor vtable encoding and optimize it for the case of multiple marker traits](https://github.com/rust-lang/rust/pull/113856)
* [remove `Scope::Elision` from bound-vars resolution](https://github.com/rust-lang/rust/pull/113950)
* [rename `arg_iter` to `iter_instantiated`](https://github.com/rust-lang/rust/pull/113801)
* [restrict recursive opaque type check](https://github.com/rust-lang/rust/pull/113636)
* [resurrect: `rustc_llvm`: Add a -Z `print-codegen-stats` option to expose LLVM statistics](https://github.com/rust-lang/rust/pull/113723)
* [reuse `codegen_ssa` monomorphization errors in `codegen_gcc`](https://github.com/rust-lang/rust/pull/113877)
* [reuse the MIR validator for MIR inlining](https://github.com/rust-lang/rust/pull/113853)
* [revert "Prototype: Add unstable `-Z reference-niches` option"](https://github.com/rust-lang/rust/pull/113946)
* [safe Transmute: Fix ICE (due to UnevaluatedConst)](https://github.com/rust-lang/rust/pull/113677)
* [substitute types before checking inlining compatibility](https://github.com/rust-lang/rust/pull/113802)
* [support `--print KIND=PATH` command line syntax](https://github.com/rust-lang/rust/pull/113780)
* [support interpolated block for `try` and `async`](https://github.com/rust-lang/rust/pull/112953)
* [turn copy into moves during DSE](https://github.com/rust-lang/rust/pull/113758)
* [tweak spans for self arg, fix borrow suggestion for signature mismatch](https://github.com/rust-lang/rust/pull/112508)
* [use SHA256 source file checksums by default when targeting MSVC](https://github.com/rust-lang/rust/pull/113707)
* [use `features()` over `features_untracked()` where possible](https://github.com/rust-lang/rust/pull/113961)
* [use erased self type when autoderefing for trait error suggestion](https://github.com/rust-lang/rust/pull/113985)
* [use the correct span for displaying the line following a derive sugge…](https://github.com/rust-lang/rust/pull/113871)
* [miri: make full field retagging the default](https://github.com/rust-lang/miri/pull/2985)
* [remove redundant clones](https://github.com/rust-lang/rust/pull/113972)
* [fix `size_hint` for `EncodeUtf16`](https://github.com/rust-lang/rust/pull/113898)
* [allow limited access to `OsString` bytes](https://github.com/rust-lang/rust/pull/113442)
* [make `{Rc,Arc}::allocator` associated functions](https://github.com/rust-lang/rust/pull/113810)
* [stabilize chown functions (`unix_chown`)](https://github.com/rust-lang/rust/pull/113033)
* [remove an allocation in `Path::with_extension`](https://github.com/rust-lang/rust/pull/113106)
* [remove the unstable `core::sync::atomic::ATOMIC_*_INIT` constants](https://github.com/rust-lang/rust/pull/105571)
* [remove lifetime bound for A for `impl Extend<&'a T> for Vec<T, A>`](https://github.com/rust-lang/rust/pull/113224)
* [hashbrown: publicly re-export `Equivalent` from the crate root](https://github.com/rust-lang/hashbrown/pull/446)
* [futures: add `TryStreamExt::try_ready_chunks` as failable version of `StreamExt::ready_chunks`](https://github.com/rust-lang/futures-rs/pull/2757)
* [codegen\_gcc: add instructions on how to generate GIMPLE format](https://github.com/rust-lang/rustc_codegen_gcc/pull/308)
* [codegen\_gcc: add support for `"ffi_const"` function attribute](https://github.com/rust-lang/rustc_codegen_gcc/pull/307)
* [cargo: git: respect scp-like URL for nested submodules](https://github.com/rust-lang/cargo/pull/12359)
* [cargo: credential provider implementation](https://github.com/rust-lang/cargo/pull/12334)
* [cargo: fix "cargo doc --open" crash on WSL2](https://github.com/rust-lang/cargo/pull/12373)
* [cargo: fix: encode URL params correctly for SourceId in Cargo.lock](https://github.com/rust-lang/cargo/pull/12280)
* [cargo: fix: only skip mtime check on `~/.cargo/{git,registry}`](https://github.com/rust-lang/cargo/pull/12369)
* [rustdoc: strip impl if not re-exported and is `doc(hidden)`](https://github.com/rust-lang/rust/pull/113574)
* [rustdoc: fix position of `default` in method rendering](https://github.com/rust-lang/rust/pull/110765)
* [rustdoc: handle cross-crate RPITITs correctly](https://github.com/rust-lang/rust/pull/113956)
* [clippy: `significant_drop_tightening` don't lint literal-returning functions](https://github.com/rust-lang/rust-clippy/pull/11161)
* [clippy: `significant_drop_tightening` fix tuple drop recognition](https://github.com/rust-lang/rust-clippy/pull/11196)
* [clippy: `inherent_to_string`: Don't lint `unsafe` or `extern` fns](https://github.com/rust-lang/rust-clippy/pull/11205)
* [clippy: `manual_filter_map`: lint on `matches` and pattern matching](https://github.com/rust-lang/rust-clippy/pull/10949)
* [clippy: `ptr_arg` should ignore extern functions](https://github.com/rust-lang/rust-clippy/pull/11215)
* [clippy: `redundant_pattern_matching`: include guard in suggestion](https://github.com/rust-lang/rust-clippy/pull/11175)
* [clippy: `unnecessary_literal_unwrap`: fix ICE on `None.unwrap_or_default()`](https://github.com/rust-lang/rust-clippy/pull/11106)
* [clippy: `unused_async`: don't lint if paths reference async fn without immediate call](https://github.com/rust-lang/rust-clippy/pull/11200)
* [clippy: `unwrap_or_else_default` → `unwrap_or_default` and improve resulting lint](https://github.com/rust-lang/rust-clippy/pull/10120)
* [clippy: allow `Self::cmp(self, other)` as a correct impl](https://github.com/rust-lang/rust-clippy/pull/11188)
* [clippy: check for fully qualified paths in `unnecessary_cast`](https://github.com/rust-lang/rust-clippy/pull/10971)
* [clippy: check that the types are equal in `SpanlessEq::eq_expr`](https://github.com/rust-lang/rust-clippy/pull/11214)
* [clippy: fix `unwrap_or_else_default` false positive](https://github.com/rust-lang/rust-clippy/pull/11135)
* [clippy: fix async functions handling for `needless_pass_by_ref_mut` lint](https://github.com/rust-lang/rust-clippy/pull/11184)
* [clippy: fix: false positive for `option_env!` in `ifs_same_cond`](https://github.com/rust-lang/rust-clippy/pull/11195)
* [clippy: make `comparison_to_empty` work on `if let`/`let` chains](https://github.com/rust-lang/rust-clippy/pull/11029)
* clippy: new lints: [`absolute_paths`](https://github.com/rust-lang/rust-clippy/pull/11003),
  [`error_impl_error`](https://github.com/rust-lang/rust-clippy/pull/11107),
  [`four_forward_slashes`](https://github.com/rust-lang/rust-clippy/pull/11140),
  [`iter_skip_zero`](https://github.com/rust-lang/rust-clippy/pull/11046),
  [`needless_return_with_try`](https://github.com/rust-lang/rust-clippy/pull/11031),
  [`redundant_guards`](https://github.com/rust-lang/rust-clippy/pull/10955),
  [`string_lit_chars_any`](https://github.com/rust-lang/rust-clippy/pull/11052),
  [ `redundant_locals`](https://github.com/rust-lang/rust-clippy/pull/10885)
* [clippy: refactor some of `dereference.rs` to util functions](https://github.com/rust-lang/rust-clippy/pull/11166)
* [clippy: remove `#![allow(unused)]` and `--crate-name` from `cargo dev new_lint` generated tests](https://github.com/rust-lang/rust-clippy/pull/11183)
* [clippy: rewrite `tuple_array_conversions`](https://github.com/rust-lang/rust-clippy/pull/11171)
* [rust-analyzer: editor/code: Use notification command links for debugger installation](https://github.com/rust-lang/rust-analyzer/pull/15290)
* [rust-analyzer: fix highlighting of byte escape sequences](https://github.com/rust-lang/rust-analyzer/pull/15303)
* [rust-analyzer: fix: don't follow raw pointer derefs when considering method receiver candidates](https://github.com/rust-lang/rust-analyzer/pull/15312)
* [rust-analyzer: fix: lookup super traits in `is_dyn_method`](https://github.com/rust-lang/rust-analyzer/pull/15317)
* [rust-analyzer: fix: normalize expected ty in call arguments](https://github.com/rust-lang/rust-analyzer/pull/15325)
* [rust-analyzer: fix: report `incorrect-ident-case` for inner items](https://github.com/rust-lang/rust-analyzer/pull/15320)
* [rust-analyzer: limit `change_visibility` assist to applicable items](https://github.com/rust-lang/rust-analyzer/pull/15277)
* [rustfmt: prevent ICE when formatting an empty-ish macro arm](https://github.com/rust-lang/rustfmt/pull/5833)
* [rustfmt: support non-lifetime binders](https://github.com/rust-lang/rustfmt/pull/5848)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

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
* [disposition: merge] [make `noop_method_call` warn by default](https://github.com/rust-lang/rust/pull/111916)
* [disposition: merge] [Infer type in irrefutable slice patterns with fixed length as array](https://github.com/rust-lang/rust/pull/113199)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Add "crates.io Policy Update" RFC](https://github.com/rust-lang/rfcs/pull/3463)
* [new] [RFC: Generic member access for dyn Error trait objects](https://github.com/rust-lang/rfcs/pull/3461)
* [new] [Error Display (std::error::Error::fmt_error)](https://github.com/rust-lang/rfcs/pull/3459)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-07-26 - 2023-08-23 🦀

### Virtual

* 2023-07-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763486)
* 2023-07-20 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #12 - Ownership and Memory management**](https://rust-meetup.ir/2023/07/20/12th-meetup.html)
* 2023-07-24 | Virtual (Toronto, CA) | [Programming Languages Virtual Meetup](https://www.meetup.com/programming-languages-toronto-meetup/)
    * [**Crafting Interpreters Chapter 18: Types of Values**](https://www.meetup.com/programming-languages-toronto-meetup/events/294616842)
* 2023-07-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfckbhc/)
* 2023-07-25 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Ruff. An extremely fast Python linter, written in Rust**](https://www.meetup.com/Rust-Dublin/events/294557256/)
* 2023-07-26 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-07-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfckbkc/)
* 2023-07-28 | Virtual (Tunis, TN) | [Rust Meetup Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number IV**](https://www.meetup.com/rust-tunisia/events/294664236/)
* 2023-08-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfclbcb/)
* 2023-08-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfclbcb//)
* 2023-08-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfclblb/)
* 2023-08-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfclbtb/)
* 2023-08-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/kmhpftyfclbvb/)

### Asia

* 2023-07-27 | Seoul, KR | [Rust Programming Meetup Seoul](https://www.meetup.com/rust-seoul-meetup/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/294832771/)

### Europe

* 2023-07-21 | Nuremberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together #2**](https://www.meetup.com/rust-noris/events/293823522/)

### North America

* 2023-07-27 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294911462/)
* 2023-08-10 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294911475/)
* 2023-08-15 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294804636/)

### Oceania

* 2023-08-09 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**August Meetup**](https://www.tickettailor.com/events/perthrustusergroup/970279)

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

> A rustacean is a programmer that dislikes being told "yes" in situations where they'll regret it later.

– [Predrag Gruevski on mastodon](https://hachyderm.io/@predrag/110720182333519119)

Thanks to [Kevin Mehall](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1452) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
