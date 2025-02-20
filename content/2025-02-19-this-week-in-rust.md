Title: This Week in Rust 587
Number: 587
Date: 2025-02-19
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
* [2024 State of Rust Survey Results](https://blog.rust-lang.org/2025/02/13/2024-State-Of-Rust-Survey-results.html)

### Newsletters
* [The Embedded Rustacean Issue #39](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-39)
* [Rust Trends Issue #59](https://rust-trends.com/newsletter/rust-for-scientists-async-closures-if-let-chains/)

### Project/Tooling Updates
* [Towards Practical Formal Verification for a General-Purpose OS in Rust](https://asterinas.github.io/2025/02/13/towards-practical-formal-verification-for-a-general-purpose-os-in-rust.html)
* [Meilisearch 1.3 - AI-powered search stabilization, remote federated search, and dumpless version upgrades](https://www.meilisearch.com/blog/meilisearch-1-13)
* [Slack Your REST Client with a couple of Serde Tricks](https://ideas.reify.ing/en/blog/rest-client-slack/)
* [Welcome, Cot: the Rust web framework for lazy developers](https://mackow.ski/blog/cot-the-rust-web-framework-for-lazy-developers/)
* [derive-deftly 1.0.0 - Rust derive macros, the easy way](https://diziet.dreamwidth.org/19395.html)

### Observations/Thoughts
* Scaling PayPay with Rust ([english](https://blog.paypay.ne.jp/en/scaling-paypay-with-rust) / [japanese](https://blog.paypay.ne.jp/scaling-paypay-with-rust))
* [Macro-Less, Highly Integrated OpenAPI Document Generation in Rust with Ohkami](https://medium.com/@kanarus786/macro-less-highly-integrated-openapi-document-generation-in-rust-with-ohkami-912de388adc1)
* [Part 6b: The Types of Lowered Rows](https://thunderseethe.dev/posts/lowering-rows-ty/)
* [eserde: Don't stop at the first deserialization error](https://mainmatter.com/blog/2025/02/13/eserde/)
* [Smuggling arbitrary data through an emoji](https://paulbutler.org/2025/smuggling-arbitrary-data-through-an-emoji/)
* [Why I'm Writing a Scheme Implementation in 2025](https://maplant.com/2025-02-17-Why-I'm-Writing-a-Scheme-Implementation-in-2025-(The-Answer-is-Async-Rust).html)
* [What's in a ring buffer? And using them in Rust](https://ntietz.com/blog/whats-in-a-ring-buffer)
* [How Rust & Embassy Shine on Embedded Devices (Part 1): Insights for Everyone and Nine Rules for Embedded Programmers](https://medium.com/@carlmkadie/how-rust-embassy-shine-on-embedded-devices-part-1-9f4911c92007)
* [Should I pin my Rust toolchain version?](https://swatinem.de/blog/rust-toolchain/)

### Rust Walkthroughs
* [Emjay – a simple JIT that does math](https://andreabergia.com/blog/2025/02/emjay-a-simple-jit-that-does-math/)
* [ioctls from Rust](https://blogsystem5.substack.com/p/ioctls-rust)
* [A length-indexed Vector in Rust](https://rvarago.github.io/a-length-indexed-vector-in-rust/)
* [video] [Rust in Audio: Collections](https://www.youtube.com/watch?v=wFiMtYtHss8&t=11s)

### Miscellaneous
* [Rust’s Async Closures: A New Way to Handle Asynchronous Logic.](https://rust-trends.com/posts/rust-s-async-closures/)
* [Rust’s if/let While Chains: Cleaner Control Flow is Coming!.](https://rust-trends.com/posts/rust-s-if-let-while-chains/)
* [video] [Meet Elusion: New DataFrame Library for Python 🐍 users powered by Rust 🦀  with Borivoj Grujicic](https://www.youtube.com/watch?v=H-GhJIFreHY)

## Crate of the Week

This week's crate is [httpmock](https://crates.io/crates/httpmock), which is quite unsurprisingly a HTTP mocking library for Rust.

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/crate-of-the-week/2704/1409) for the suggestion!

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->
* [**NDC Techtown**](https://ndctechtown.com/call-for-papers) | 2025-04-27 | Kongsberg, Norway | 2025-09-22 to 2025-09-25
* [**Scientific Computing in Rust 2025**](https://scientificcomputing.rs/2025/submit-talk) | Closes 2025-05-02 | virtual | Event date: 2025-06-04 to 2025-06-06

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

498 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-02-11..2025-02-18

* [make `-O` mean `OptLevel::Aggressive`](https://github.com/rust-lang/rust/pull/135439)
* [`rustc_target`: Add the fp16 target feature for AArch32](https://github.com/rust-lang/rust/pull/136813)
* [add x86-sse2 (32bit) ABI that requires SSE2 target feature](https://github.com/rust-lang/rust/pull/137037)
* [add cygwin target](https://github.com/rust-lang/rust/pull/134999)
* [HIR analysis: Remove unnecessary abstraction over list of clauses](https://github.com/rust-lang/rust/pull/137100)
* [AIX: expect `EINVAL` for `pthread_mutex_destroy`](https://github.com/rust-lang/rust/pull/136908)
* [`abi_unsupported_vector_types`: say which type is the problem](https://github.com/rust-lang/rust/pull/137092)
* [`invalid_from_utf8[_unchecked]`: also lint inherent methods](https://github.com/rust-lang/rust/pull/137101)
* [`transmute` should also assume non-null pointers](https://github.com/rust-lang/rust/pull/136735)
* [allow configuring jemalloc per target](https://github.com/rust-lang/rust/pull/137170)
* [cast allocas to default address space](https://github.com/rust-lang/rust/pull/135025)
* [change `swap_nonoverlapping` from lang to library UB](https://github.com/rust-lang/rust/pull/136890)
* [check sig for errors before checking for unconstrained anonymous lifetime](https://github.com/rust-lang/rust/pull/136891)
* [check whole `Unsize` predicate for escaping bound vars](https://github.com/rust-lang/rust/pull/136838)
* [compiler: die immediately instead of handling unknown target codegen](https://github.com/rust-lang/rust/pull/136833)
* [compiler: give `ExternAbi` truly stable `Hash` and `Ord`](https://github.com/rust-lang/rust/pull/136901)
* [compiler: replace `ExternAbi::name` calls with formatters](https://github.com/rust-lang/rust/pull/136900)
* [correctly escape hashtags when running `invalid_rust_codeblocks` lint](https://github.com/rust-lang/rust/pull/136927)
* [debuginfo: set bitwidth appropriately in `enum` variant tags](https://github.com/rust-lang/rust/pull/136895)
* [do not allow attributes on `struct` field rest patterns](https://github.com/rust-lang/rust/pull/136490)
* [eagerly prove WF when resolving fully qualified paths](https://github.com/rust-lang/rust/pull/136928)
* [emit MIR for each bit with on `dont_reset_cast_kind_without_updating_operand`](https://github.com/rust-lang/rust/pull/137007)
* [explain that in paths generics can't be set on both the `enum` and the variant](https://github.com/rust-lang/rust/pull/134981)
* [fix `ensure_monomorphic_enough`](https://github.com/rust-lang/rust/pull/136839)
* [fix const items not being allowed to be called `r#move` or `r#static`](https://github.com/rust-lang/rust/pull/137140)
* [fix diagnostic when using = instead of : in let binding](https://github.com/rust-lang/rust/pull/136869)
* [fix presentation of purely "additive" replacement suggestion parts](https://github.com/rust-lang/rust/pull/136958)
* [i686-linux-android: increase CPU baseline to Pentium 4 (without an actual change](https://github.com/rust-lang/rust/pull/136885)
* [ignore Self in bounds check for associated types with Self:Sized](https://github.com/rust-lang/rust/pull/137097)
* [implement `supertrait_item_shadowing` (v2)](https://github.com/rust-lang/rust/pull/125782) (RFC [#3624](https://rust-lang.github.io/rfcs/3624-supertrait-item-shadowing-v2.html))
* [implement pattern type ffi checks](https://github.com/rust-lang/rust/pull/136193)
* [improve host/cross target checking](https://github.com/rust-lang/rust/pull/136767)
* [in "specify type" suggestion, skip type params that are already known](https://github.com/rust-lang/rust/pull/135965)
* [load all builtin targets at once instead of one by one in check-cfg](https://github.com/rust-lang/rust/pull/137072)
* [made `check_argument_compat` public for use in miri](https://github.com/rust-lang/rust/pull/137056)
* [mark condition/carry bit as clobbered in C-SKY inline assembly](https://github.com/rust-lang/rust/pull/136217)
* [normalize closure instance before eagerly monomorphizing it](https://github.com/rust-lang/rust/pull/137035)
* [overhaul `rustc_middle::limits`](https://github.com/rust-lang/rust/pull/136671)
* [overhaul how contracts are lowered on fn-like bodies](https://github.com/rust-lang/rust/pull/136837)
* [parallel-compiler-related cleanup](https://github.com/rust-lang/rust/pull/136858)
* [properly deeply normalize in the next solver](https://github.com/rust-lang/rust/pull/136074)
* [reject `?Trait` bounds in various places where we unconditionally warned since 1.0](https://github.com/rust-lang/rust/pull/135841)
* [remove SSE ABI from i586-pc-windows-msvc](https://github.com/rust-lang/rust/pull/137149)
* [remove the common prelude module](https://github.com/rust-lang/rust/pull/136886)
* [replace some u64 hashes with Hash64](https://github.com/rust-lang/rust/pull/137095)
* [resolve named regions when reporting type test failures in NLL](https://github.com/rust-lang/rust/pull/136559)
* [rework `name_regions` to not rely on reverse scc graph for non-member-constrain usages](https://github.com/rust-lang/rust/pull/137102)
* [rework rigid alias handling](https://github.com/rust-lang/rust/pull/136863)
* [set both `nuw` and `nsw` in slice size calculation](https://github.com/rust-lang/rust/pull/136575)
* [show supported register classes in error message](https://github.com/rust-lang/rust/pull/136239)
* [try to recover from path sep error in type parsing](https://github.com/rust-lang/rust/pull/136808)
* [unify LLVM version finding logic](https://github.com/rust-lang/rust/pull/136962)
* [use `const_error!` when possible](https://github.com/rust-lang/rust/pull/136844)
* [use a trait to enforce field validity for union fields + `unsafe` fields + `unsafe<>` binder types](https://github.com/rust-lang/rust/pull/136660)
* [use the right binder for rebinding `PolyTraitRef`](https://github.com/rust-lang/rust/pull/136951)
* [valtree performance tuning](https://github.com/rust-lang/rust/pull/136593)
* [miri: apply random float error](https://github.com/rust-lang/miri/pull/4156)
* [miri: remove the build script for miri](https://github.com/rust-lang/miri/pull/4192)
* [stabilize `NonZero::count_ones`](https://github.com/rust-lang/rust/pull/136663)
* [stabilize `const_is_char_boundary` and `const_str_split_at`](https://github.com/rust-lang/rust/pull/134016)
* [stabilize `get_many_mut` as `get_disjoint_mut`](https://github.com/rust-lang/rust/pull/134633)
* [stabilize `target_feature_11`](https://github.com/rust-lang/rust/pull/134090)
* [std: replace the `FromInner` implementation for addresses with private conversion functions](https://github.com/rust-lang/rust/pull/136699)
* [implement `Extend<AsciiChar>` for String](https://github.com/rust-lang/rust/pull/136749)
* [implement `f`{`16`, `32`, `64`, `128`}`::`{`erf`, `erfc`} under `#![feature(float_erf)]`](https://github.com/rust-lang/rust/pull/136324)
* [implement `read*_exact` for `std:io::repeat`](https://github.com/rust-lang/rust/pull/136818)
* [use `slice::fill` in `io::Repeat` implementation](https://github.com/rust-lang/rust/pull/136967)
* [use `tell` for `<File as Seek>::stream_position`](https://github.com/rust-lang/rust/pull/137165)
* [restrict `DerefPure` for `Cow<T>` impl to `T = impl Clone, [impl Clone], str`](https://github.com/rust-lang/rust/pull/137105)
* [forward all default methods for I/O impls](https://github.com/rust-lang/rust/pull/137062)
* [cargo: embedded: Handle more parsing corner cases](https://github.com/rust-lang/cargo/pull/15187)
* [cargo: embedded: Integrate cargo-script logic into main parser](https://github.com/rust-lang/cargo/pull/15168)
* [cargo: implement workspace feature unification](https://github.com/rust-lang/cargo/pull/15157)
* [cargo: util: provide a better error message for invalid SSH URLs](https://github.com/rust-lang/cargo/pull/15185)
* [rustdoc: only apply LTO to rustdoc at stage 2](https://github.com/rust-lang/rust/pull/136586)
* [rustdoc: Move line numbers into the `<code>` directly](https://github.com/rust-lang/rust/pull/136829)
* [rustdoc: improve refdef handling in the unresolved link lint](https://github.com/rust-lang/rust/pull/136363)
* [rustdoc: properly restore search input placeholder](https://github.com/rust-lang/rust/pull/137055)
* [rustdoc: Fixed `Copy Item Path` in rust doc](https://github.com/rust-lang/rust/pull/137068)
* [rustdoc: do more lazy formatting in librustdoc 🦥](https://github.com/rust-lang/rust/pull/136828)
* [rustdoc: nuke `Buffer` abstraction from `librustdoc`, take 2 💣](https://github.com/rust-lang/rust/pull/136784)
* [rustfmt: add option to control trailing zero in floating-point literals](https://github.com/rust-lang/rustfmt/pull/5834)
* [rustfmt: labels on the closure body block can be disappearing](https://github.com/rust-lang/rustfmt/pull/6468)
* [bindgen: distinguish `char16_t`](https://github.com/rust-lang/rust-bindgen/pull/3135)
* [bindgen: report enums in ParseCallbacks](https://github.com/rust-lang/rust-bindgen/pull/3133)
* [clippy: new lint: `manual_contains`](https://github.com/rust-lang/rust-clippy/pull/13817)
* [clippy: new lint: `mem_replace_option_with_some`](https://github.com/rust-lang/rust-clippy/pull/14197)
* [clippy: new lint: `unbuffered_bytes`](https://github.com/rust-lang/rust-clippy/pull/14089)
* [clippy: `declare_interior_mutable_const`, `borrow_interior_mutable_const`: resolve `<T as Trait>::AssocT` projections](https://github.com/rust-lang/rust-clippy/pull/14125)
* [clippy: `doc_link_code`: add check for links with code spans that render weird](https://github.com/rust-lang/rust-clippy/pull/14121)
* [clippy: `just_underscores_and_digits`: fix false positive in error recovery scenario](https://github.com/rust-lang/rust-clippy/pull/14168)
* [clippy: `manual_ok_err`: blockify the replacement of an `else if …`](https://github.com/rust-lang/rust-clippy/pull/14240)
* [clippy: `unnecessary_map_or`: do not consume the comparison value if it does not implement `Copy`](https://github.com/rust-lang/rust-clippy/pull/14207)
* [clippy: `{expect,unwrap}_used`: add options to lint at compilation time](https://github.com/rust-lang/rust-clippy/pull/14200)
* [clippy: add `--allow-no-vcs` to `cargo dev dogfood --fix`](https://github.com/rust-lang/rust-clippy/pull/14227)
* [clippy: add index checks for the slice in `manual_slice_fill`](https://github.com/rust-lang/rust-clippy/pull/14193)
* [clippy: fix `literal_string_with_formatting_args` lint emitted when it should not](https://github.com/rust-lang/rust-clippy/pull/13953)
* [clippy: fix `used_underscore_items` lint uses of foreign functions](https://github.com/rust-lang/rust-clippy/pull/14205)
* [clippy: fix `needless_option_as_deref` FP in trait](https://github.com/rust-lang/rust-clippy/pull/14210)
* [rust-analyzer: add cargo's git checkouts to the list of paths to mark as read-only in vscode](https://github.com/rust-lang/rust-analyzer/pull/19156)
* [rust-analyzer: apply cfg.setTest to json projects](https://github.com/rust-lang/rust-analyzer/pull/19150)
* [rust-analyzer: feat: refactor path lowering and serve a new path diagnostic](https://github.com/rust-lang/rust-analyzer/pull/19127)
* [rust-analyzer: apply adjustments to proper expr when invoking `CoerceMany`](https://github.com/rust-lang/rust-analyzer/pull/19111)
* [rust-analyzer: censor `cfg_attr` for attribute macros](https://github.com/rust-lang/rust-analyzer/pull/19125)
* [rust-analyzer: do not show safety hints for extern items lacking semantics](https://github.com/rust-lang/rust-analyzer/pull/19109)
* [rust-analyzer: don't emit implicit drop inlay hints for macro](https://github.com/rust-lang/rust-analyzer/pull/19117)
* [rust-analyzer: don't show drop hints for other pattern](https://github.com/rust-lang/rust-analyzer/pull/19144)
* [rust-analyzer: fix detection of ref patterns for path patterns](https://github.com/rust-lang/rust-analyzer/pull/19167)
* [rust-analyzer: fix postfix completions inside macros](https://github.com/rust-lang/rust-analyzer/pull/19129)
* [rust-analyzer: fix sorting of runnables](https://github.com/rust-lang/rust-analyzer/pull/19166)
* [rust-analyzer: handle character boundary in search mode](https://github.com/rust-lang/rust-analyzer/pull/18928)
* [rust-analyzer: highlight `extern crate` in doc comments](https://github.com/rust-lang/rust-analyzer/pull/19137)
* [rust-analyzer: improve sort order of runnables](https://github.com/rust-lang/rust-analyzer/pull/19161)
* [rust-analyzer: lower range pattern bounds to expressions](https://github.com/rust-lang/rust-analyzer/pull/18995)
* [rust-analyzer: make `rust-analyzer.files.excludeDirs` work, actually](https://github.com/rust-lang/rust-analyzer/pull/18998)
* [rust-analyzer: propogate error types in mir type projections](https://github.com/rust-lang/rust-analyzer/pull/19143)
* [rust-analyzer: stabilize sort order of `related_tests`](https://github.com/rust-lang/rust-analyzer/pull/19163)
* [rust-analyzer: implement `expand_glob_reexport` assist](https://github.com/rust-lang/rust-analyzer/pull/19158)
* [rust-analyzer: improve error recovery when method-calling a field](https://github.com/rust-lang/rust-analyzer/pull/19148)
* [rust-analyzer: improve error recovery when method-calling an assoc function](https://github.com/rust-lang/rust-analyzer/pull/19160)
* [rust-analyzer: pass `struct` fields to chalk](https://github.com/rust-lang/rust-analyzer/pull/19122)
* [rust-analyzer: simplify `panic_context`](https://github.com/rust-lang/rust-analyzer/pull/19110)

### Rust Compiler Performance Triage

This week's results were dominated by the update to LLVM 20 ([#135763](https://github.com/rust-lang/rust/pull/135763)),
which brought a large number of performance improvements, as usually. There were also two other
significant improvements, caused by improving the representation of `const` values ([#136593](https://github.com/rust-lang/rust/pull/136593)) and doing less work when formatting in `rustdoc` ([#136828](https://github.com/rust-lang/rust/pull/136828)).

Triage done by **@kobzol**.

Revision range: [c03c38d5..ce36a966](https://perf.rust-lang.org/?start=c03c38d5c2368cd2aa0e056dba060b94fc747f4e&end=ce36a966c79e109dabeef7a47fe68e5294c6d71e&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 4.4%  | [0.2%, 35.8%]   | 10    |
| Regressions ❌ <br /> (secondary)  | 1.2%  | [0.2%, 5.0%]    | 13    |
| Improvements ✅ <br /> (primary)   | -1.6% | [-10.5%, -0.2%] | 256   |
| Improvements ✅ <br /> (secondary) | -1.0% | [-4.7%, -0.2%]  | 163   |
| All ❌✅ (primary)                 | -1.3% | [-10.5%, 35.8%] | 266   |

3 Regressions, 2 Improvements, 4 Mixed; 4 of them in rollups
50 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/305a70edd98c32a4ea7388561841e5473b4bb153/triage/2025-02-18.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [[RFC] Target Modifiers](https://github.com/rust-lang/rfcs/pull/3716)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Make `ptr_cast_add_auto_to_object lint` into hard error](https://github.com/rust-lang/rust/pull/136764)
* [Allow `*const W<dyn A> -> *const dyn A` ptr cast](https://github.com/rust-lang/rust/pull/136127)
* [Stabilize `core::str::from_utf8_mut` as `const`](https://github.com/rust-lang/rust/pull/136668)
* [core: Make `Debug` impl of raw pointers print metadata if present](https://github.com/rust-lang/rust/pull/135080)
* [Explain how Vec::with_capacity is faithful](https://github.com/rust-lang/rust/pull/135933)
* [Reduce formatting `width` and `precision` to 16 bits](https://github.com/rust-lang/rust/pull/136932)
* [Run TLS destructors at process exit on all platforms](https://github.com/rust-lang/rust/pull/134085)
* [Tracking Issue for `string_extend_from_within`](https://github.com/rust-lang/rust/issues/103806)
* [Tracking Issue for unbounded_shifts](https://github.com/rust-lang/rust/issues/129375)
* [[discussion] `ErrorKind::InvalidFilename` from io_`error_more`](https://github.com/rust-lang/rust/issues/130192)
* [Tracking Issue for `unsigned_is_multiple_of`](https://github.com/rust-lang/rust/issues/128101)
* [Tracking issue for HashMap::extract_if and HashSet::extract_if](https://github.com/rust-lang/rust/issues/59618)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Add `must-use-output` attribute](https://github.com/rust-lang/rfcs/pull/3773)
* [RFC: Deprecate the per-build-target `edition` field in `Cargo.toml`](https://github.com/rust-lang/rfcs/pull/3772)
* [RFC: Demote i686-pc-windows-gnu to Tier 2](https://github.com/rust-lang/rfcs/pull/3771)

## Upcoming Events

Rusty Events between 2025-02-19 - 2025-03-19 🦀

### Virtual
* 2025-02-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Pointer Provenance**](https://www.meetup.com/vancouver-rust/events/304051783)
* 2025-02-20 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**February, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 2025-02-21 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntyhcdbcc)
* 2025-02-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361428)
* 2025-02-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: The complicated world of Strings in Rust**](https://www.meetup.com/women-in-rust/events/305716182)
* 2025-02-25 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful—Everett Pompeii presents Bencher 🐰**](https://www.meetup.com/rustdc/events/305170682)
* 2025-02-27 | Virtual (US) | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Intro to Rust**](https://www.eventbrite.com/e/intro-to-rust-tickets-1237517059839)
* 2025-02-27 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820295)
* 2025-02-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Parsing command line options with category theory and async**](https://www.meetup.com/charlottesville-rust-meetup/events/305948365)
* 2025-03-01 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-62876317658/)
* 2025-03-04 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Building Integration Libraries and parsing with Winnow in Rust with Kenny Flegal**](https://www.meetup.com/code-mavens/events/305793122/)
* 2025-03-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031659)
* 2025-03-06 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820281/)
* 2025-03-06 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #9**](https://www.meetup.com/bevy-game-development/events/306131557)
* 2025-03-06 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Ratatui - Terminal User Interfaces in Rust with Orhun Parmaksız**](https://www.meetup.com/code-mavens/events/305750365/)
* 2025-03-09 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Creating A Mock Blockchain in Rust with Sourav Mishra**](https://www.meetup.com/code-mavens/events/305587087/)
* 2025-03-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/303522529)
* 2025-03-11 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/305716839)
* 2025-03-13 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820296)
* 2025-03-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170694)
* 2025-03-18 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**crum: Complex Numbers and Complex Matrices in Rust with Frans Slabber**](https://www.meetup.com/code-mavens/events/305823397/)
* 2025-03-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/305470139)

### Asia
* 2025-02-24 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust February 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/305570131)
* 2025-03-01 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Zig & Rust Meetup**](https://lu.ma/460w8v58)
* 2025-03-19 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust March 2025 at Jit in Tel Aviv**](https://www.meetup.com/rust-tlv/events/305697580)

### Europe
* 2025-02-19 - 2025-02-20 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2025**](https://www.rustnationuk.com/)
* 2025-02-20 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #1 @Puzzle ITC**](https://www.meetup.com/rust-bern/events/305597994)
* 2025-02-21 | London, UK | [Rust Global: London 2025](https://rustfoundation.org/event/rust-global-london-2025/)
    * [**Rust Global: London 2025**](https://rustfoundation.org/event/rust-global-london-2025/)
* 2025-02-22 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #9**](https://www.meetup.com/stockholm-rust/events/305848723)
* 2025-02-25 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/events/)
    * [**Rust desde cero: Cargo y tipos**](https://www.meetup.com/madrust/events/305896258)
* 2025-02-26 | Darmstadt, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Rust Compiler Tuning**](https://www.meetup.com/rust-rhein-main/events/305990886/)
2025-02-26 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2025**](https://lu.ma/iwu6mlcj)
* 2025-02-27 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809675)
* 2025-02-27 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #75**](https://www.meetup.com/rust-paris/events/305791655)
* 2025-03-01 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Speyer**](https://www.meetup.com/rust-noris/events/305361732/)
* 2025-03-05 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**17th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/305887675)
* 2025-03-07 | Prague, CZ | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/events/)
    * [**Rust meetup in Braiins offices**](https://www.meetup.com/rust-czech-republic/events/306237623)
* 2025-03-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045445)
* 2025-03-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045445)
* 2025-03-13 | Biel, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #2 @ BFH Biel**](https://www.meetup.com/rust-bern/events/306169573) 
* 2025-03-14 | Paris, FR | [Rust in Paris](https://www.rustinparis.com/)
    * [**Rust in Paris 2025**](https://www.rustinparis.com/schedule)
* 2025-03-18 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #10 @ MDPI Basel**](https://www.meetup.com/rust-basel/events/306121044)
* 2025-03-18 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729673)

### North America
* 2025-02-20 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/306087854)
* 2025-02-20 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 2: Basics of Game Development**](https://www.meetup.com/music-city-rust-developers/events/304333047)
* 2025-02-20 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**February, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658424)
* 2025-02-20 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/events/)
    * [**Monthly Meetup: Celebrating A Year of Spokane Rust**](https://www.meetup.com/spokane-rust/events/306179775)
* 2025-02-21 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y ciencia de datos**](https://www.meetup.com/rust-mx/events/305793010)
* 2025-02-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Somerville Union Square Rust Lunch, Feb 22**](https://www.meetup.com/bostonrust/events/305805059)
* 2025-02-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcdbjc)
* 2025-02-27 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Starting the meetup again**](https://www.meetup.com/rust-atl/events/305776081)
* 2025-03-02 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Beacon Hill Rust Lunch, Mar 2**](https://www.meetup.com/bostonrust/events/305805164)
* 2025-03-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**CRDTs in Rust**](https://www.meetup.com/stl-rust/events/305187783)
* 2025-03-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, Mar 10**](https://www.meetup.com/bostonrust/events/305805192)
* 2025-03-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person **](https://www.meetup.com/san-francisco-rust-study-group/events/302638264)

### Oceania
* 2025-02-24 | Collingwood, VI, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**February 2025 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/306040785)
* 2025-02-25 | Barton, AC, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/306090406)
* 2025-02-27 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/events/)
    * [**Rust:7 Years Maintaining a Commercial Unicode Tool + Peace: Automation Framework**](https://www.meetup.com/rust-akl/events/306198434)
* 2025-03-04 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**How Orica is using Rust in their workplace**](https://www.meetup.com/perth-rust-meetup-group/events/306131753)
* 2025-03-11 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/306262384)

### South America:

* 2025-03-15 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na CloudWalk**](https://www.meetup.com/rust-sao-paulo-meetup/events/306034427)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1hynsw7/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I have found that many automated code review tools, including LLMs, catch 10 out of 3 bugs.

– [Josh Triplett on r/rust](https://old.reddit.com/r/rust/comments/1ink5qf/niko_matsakis_how_i_learned_to_stop_worrying_and/mcdkg36/)

Despite a lamentable lack of suggestions, llogiq is properly pleased with his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1itnl16/this_week_in_rust_587/)</small>
