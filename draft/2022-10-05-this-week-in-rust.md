Title: This Week in Rust 463
Number: 463
Date: 2022-10-05
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

### Official
* [Announcing the Rust Style Team](https://blog.rust-lang.org/inside-rust/2022/09/29/announcing-the-rust-style-team.html)

### Foundation
* [Rust Foundation Project Grants are open for applications](https://foundation.rust-lang.org/news/2022-10-03-project-grants-open-for-applications/)

### Newsletters

### Project/Tooling Updates
* [cargo careful: run your Rust code with extra careful debug checking](https://www.ralfj.de/blog/2022/09/26/cargo-careful.html)
* [Async UI: a Rust UI Library where Everything is a Future](https://wishawa.github.io/posts/async-ui-intro/)
* [rust-analyzer changelog #149](https://rust-analyzer.github.io/thisweek/2022/10/03/changelog-149.html)

### Observations/Thoughts
* [How (and why) nextest uses tokio, part 1](https://sunshowers.io/posts/nextest-and-tokio-1/)
* [in-place constructors](https://y86-dev.github.io/blog/safe-pinned-initialization/in-place.html)
* [Quirks of Rust’s token representation](https://nnethercote.github.io/2022/10/05/quirks-of-rusts-token-representation.html)
* [Brute forcing protected ZIP archives in Rust](https://agourlay.github.io/brute-forcing-protected-zip-rust/)

### Rust Walkthroughs
* [How to call a C function from Rust (A simple FFI tutorial)](https://github.com/vanjacosic/rust-ffi-to-c)
* [Rewriting the Modern Web in Rust](https://implfuture.dev/blog/rewriting-the-modern-web-in-rust)
* [Implementing truly safe semaphores in rust](https://neosmart.net/blog/2022/implementing-truly-safe-semaphores-in-rust/)
* [Model an ALU in Rust](https://www.superperfundo.tech/articles/alu-model)
* [6 things you can do with the Cow 🐄 in Rust 🦀](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55)
* [Platform Agnostic Drivers in Rust: MAX7219 Naive Code Refactoring](https://apollolabsblog.hashnode.dev/platform-agnostic-drivers-in-rust-max7219-naive-code-refactoring)
* [Last mile DynamoDB: Deno Deploy edition](https://artofserverless.com/dynamodb-deno-deploy/)

### Research

### Miscellaneous
* [The Initial Rust Infrastructure Has Been Merged Into Linux 6.1](https://www.phoronix.com/news/Rust-Is-Merged-Linux-6.1)

## Crate of the Week

This week's crate is [humansize](https://lib.rs/crates/humansize), a size formatting crate. Now in version 2.0, it features an updated API. 

Thanks, [Leopold Arkham](https://users.rust-lang.org/u/leopoldarkham/summary) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

## Updates from the Rust Project

367 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-09-26..2022-10-03

* [cargo: Expose guide to adding a new edition as rustdoc](https://github.com/rust-lang/cargo/pull/11157)
* [cargo: Expose libgit2-sys/vendored feature as vendored-libgit2](https://github.com/rust-lang/cargo/pull/11162)
* [cargo: Indicate how Cargo locates the manifest](https://github.com/rust-lang/cargo/pull/10770)
* [cargo: Iteratively construct target cfg](https://github.com/rust-lang/cargo/pull/11114)
* [cargo: Provide a better error message when mixing dep: with /](https://github.com/rust-lang/cargo/pull/11172)
* [cargo: Reduce references to `[project]` within cargo](https://github.com/rust-lang/cargo/pull/11135)
* [cargo: Remove `multitarget` from -Zhelp](https://github.com/rust-lang/cargo/pull/11158)
* [cargo: Remove lingering unstable flag `-Zfeatures`](https://github.com/rust-lang/cargo/pull/11168)
* [cargo: Remove outdated comments](https://github.com/rust-lang/cargo/pull/11155)
* [cargo: Tweak wording](https://github.com/rust-lang/cargo/pull/11164)
* [cargo: build-scripts.md: Use em dash consistently](https://github.com/rust-lang/cargo/pull/11150)
* [cargo: refactor(cli): Upgrade to clap v4](https://github.com/rust-lang/cargo/pull/11159)
* [Split fallible infallible folding](https://github.com/rust-lang/chalk/pull/772)
* [weak Linkage for "ARM Alias" functions](https://github.com/rust-lang/compiler-builtins/pull/495)
* [ci: Replace `volta-cli/action` with builtin functionality from `actions/setup-node`](https://github.com/rust-lang/crates.io/pull/5262)
* [docs.rs: Link to a consistent owners page](https://github.com/rust-lang/docs.rs/pull/1868)
* [docs.rs: new cache-policy & cache middleware structure to support full page caching](https://github.com/rust-lang/docs.rs/pull/1856)
* [note that a commit is not a descendant of itself](https://github.com/rust-lang/git2-rs/pull/886)
* [libc: CPU_SETSIZE constant to dragonflybsd](https://github.com/rust-lang/libc/pull/2925)
* [libc: Deploy GitHub Pages via GitHub Actions](https://github.com/rust-lang/libc/pull/2933)
* [libc: Prepare 0.2.134 release](https://github.com/rust-lang/libc/pull/2932)
* [libc: Use `::Option` and not `Option` for `pthread_jit_write_callback_t`](https://github.com/rust-lang/libc/pull/2931)
* [libc: add major/minor/makedev on apple OSes](https://github.com/rust-lang/libc/pull/2937)
* [libc: fs: add NSFS_MAGIC constant](https://github.com/rust-lang/libc/pull/2928)
* [libc: linux add ptrace_rseq_configuration](https://github.com/rust-lang/libc/pull/2923)
* [libc: mmap/madvise specific solaris additional flags](https://github.com/rust-lang/libc/pull/2922)
* [use wrapping_neg() to avoid fma errors on underflow](https://github.com/rust-lang/libm/pull/269)
* [miri: Add flag to specify the number of cpus](https://github.com/rust-lang/miri/pull/2562)
* [miri: Merge rustc changes back into the miri repo](https://github.com/rust-lang/miri/pull/2568)
* [miri: Rustup](https://github.com/rust-lang/miri/pull/2577)
* [miri: Rustup](https://github.com/rust-lang/miri/pull/2573)
* [miri: relax vergen dependency](https://github.com/rust-lang/miri/pull/2574)
* [miri: use rustc-build-sysroot instead of xargo](https://github.com/rust-lang/miri/pull/2564)
* [miri: use rustc_tools_util instead of vergen](https://github.com/rust-lang/miri/pull/2576)
* [one sentence, one line Patterns chapter](https://github.com/rust-lang/reference/pull/1275)
* [account for use of index-based lifetime names in print of binder](https://github.com/rust-lang/rust/pull/102399)
* [add `#[rustc_safe_intrinsic]`](https://github.com/rust-lang/rust/pull/100719)
* [add a known-bug test for #102498](https://github.com/rust-lang/rust/pull/102566)
* [add a label to struct/enum/union ident name](https://github.com/rust-lang/rust/pull/102314)
* [add a niche to `Duration`, unix `SystemTime`, and non-apple `Instant`](https://github.com/rust-lang/rust/pull/102368)
* [add diagnostic struct for const eval error in `rustc_middle`](https://github.com/rust-lang/rust/pull/102486)
* [add long description and test for E0311](https://github.com/rust-lang/rust/pull/100747)
* [add negation methods for signed non-zero integers](https://github.com/rust-lang/rust/pull/102342)
* [added more const_closure functionality](https://github.com/rust-lang/rust/pull/102276)
* [adjust the s390x data layout for LLVM 16](https://github.com/rust-lang/rust/pull/102499)
* [allow passing rustix_use_libc cfg using RUSTFLAGS](https://github.com/rust-lang/rust/pull/102353)
* [avoid ICE in printing RPITIT type](https://github.com/rust-lang/rust/pull/102597)
* [avoid LLVM-deprecated `Optional::hasValue`](https://github.com/rust-lang/rust/pull/102337)
* [change `is_some_and` to take by value](https://github.com/rust-lang/rust/pull/98354)
* [change argument handling in `remote-test-server` and add new flags](https://github.com/rust-lang/rust/pull/102193)
* [cleanup some error code explanations](https://github.com/rust-lang/rust/pull/102615)
* [code refactoring smart_resolve_report_errors](https://github.com/rust-lang/rust/pull/102085)
* [compute lint levels by definition](https://github.com/rust-lang/rust/pull/102236)
* [declare `main` as visibility hidden on targets that default to hidden](https://github.com/rust-lang/rust/pull/102424)
* [deny associated type bindings within associated type bindings](https://github.com/rust-lang/rust/pull/102338)
* [do not overwrite lifetime binders for another HirId](https://github.com/rust-lang/rust/pull/101454)
* [do not panic when a test function returns Result::Err](https://github.com/rust-lang/rust/pull/100451)
* [document that Display automatically implements ToString](https://github.com/rust-lang/rust/pull/102322)
* [document the conditional existence of `alloc::sync` and `alloc::task`](https://github.com/rust-lang/rust/pull/98218)
* [don't export `__heap_base` and `__data_end` on wasm32-wasi](https://github.com/rust-lang/rust/pull/102385)
* [don't export `__wasm_init_memory` on WebAssembly](https://github.com/rust-lang/rust/pull/102426)
* [don't lower assoc bindings just to deny them](https://github.com/rust-lang/rust/pull/102492)
* [enable inline stack probes on PowerPC and SystemZ](https://github.com/rust-lang/rust/pull/102328)
* [enable inline stack probes on X86 with LLVM 16](https://github.com/rust-lang/rust/pull/102503)
* [even more lexer improvements](https://github.com/rust-lang/rust/pull/102508)
* [fix ICE #101739](https://github.com/rust-lang/rust/pull/102613)
* [fix ICE in const_trait check code](https://github.com/rust-lang/rust/pull/102361)
* [fix `#[derive(Default)]` on a generic `#[default]` enum adding unnecessary `Default` bounds](https://github.com/rust-lang/rust/pull/101040)
* [fix `format_args` capture for macro expanded format strings](https://github.com/rust-lang/rust/pull/102519)
* [fix associated type bindings with anon const in GAT position](https://github.com/rust-lang/rust/pull/102336)
* [fix duplicate usage of `a` article](https://github.com/rust-lang/rust/pull/102591)
* [fix integer overflow in `format!("{:.0?}", Duration::MAX)`](https://github.com/rust-lang/rust/pull/102484)
* [fix perf regression from TypeVisitor changes](https://github.com/rust-lang/rust/pull/101893)
* [fix search result colors](https://github.com/rust-lang/rust/pull/102369)
* [fix span of byte-escaped left format args brace](https://github.com/rust-lang/rust/pull/102214)
* [flush delayed bugs before codegen](https://github.com/rust-lang/rust/pull/102373)
* [generate synthetic region from `impl` even in closure body within an associated fn](https://github.com/rust-lang/rust/pull/102490)
* [get rid of exclude-list for Windows-only tests](https://github.com/rust-lang/rust/pull/102305)
* [give `def_span` the same SyntaxContext as `span_with_body`](https://github.com/rust-lang/rust/pull/102538)
* [group together more size assertions](https://github.com/rust-lang/rust/pull/102493)
* [improve E0585 help](https://github.com/rust-lang/rust/pull/102351)
* [improve `File::set_times` error handling](https://github.com/rust-lang/rust/pull/101675)
* [improve `FromStr` example](https://github.com/rust-lang/rust/pull/102569)
* [improve code example for Option::unwrap_or_default](https://github.com/rust-lang/rust/pull/102283)
* [improve documentation of `slice::{from_ptr_range, from_ptr_range_mut}`](https://github.com/rust-lang/rust/pull/102607)
* [improve errors for incomplete functions in struct definitions](https://github.com/rust-lang/rust/pull/102350)
* [improve example of Iterator::reduce](https://github.com/rust-lang/rust/pull/102435)
* [improve the COPYRIGHT file](https://github.com/rust-lang/rust/pull/102195)
* [inline a few functions](https://github.com/rust-lang/rust/pull/102387)
* [make `feature(const_btree_len)` implied by `feature(const_btree_new)`](https://github.com/rust-lang/rust/pull/102556)
* [make `std::os::fd` public](https://github.com/rust-lang/rust/pull/98368)
* [make fmt downloaded on every invocation of bootstrap](https://github.com/rust-lang/rust/pull/101969)
* [make the `c` feature for `compiler-builtins` an explicit opt-in](https://github.com/rust-lang/rust/pull/101833)
* [manually order `DefId` on 64-bit big-endian](https://github.com/rust-lang/rust/pull/102382)
* [mark Cell::replace() as #[inline]](https://github.com/rust-lang/rust/pull/102548)
* [migrate `.stab` elements style to CSS variables](https://github.com/rust-lang/rust/pull/102620)
* [migrate more of rustc_parse to SessionDiagnostic](https://github.com/rust-lang/rust/pull/101619)
* [migrate rustc_codegen_gcc to SessionDiagnostics](https://github.com/rust-lang/rust/pull/101075)
* [migrate sidebar links color to CSS variables and unify themes with ayu](https://github.com/rust-lang/rust/pull/102237)
* [more lexer improvements](https://github.com/rust-lang/rust/pull/102302)
* [move lint level source explanation to the bottom](https://github.com/rust-lang/rust/pull/101986)
* [reinstate `hir-stats.rs` test for stage 1](https://github.com/rust-lang/rust/pull/102495)
* [remove `expr_parentheses_needed` from `ParseSess`](https://github.com/rust-lang/rust/pull/102500)
* [remove a FIXME whose code got moved away in #62883](https://github.com/rust-lang/rust/pull/102405)
* [remove a couple lifetimes that can be infered](https://github.com/rust-lang/rust/pull/102592)
* [rewrite and refactor format_args!() builtin macro](https://github.com/rust-lang/rust/pull/100996)
* [rustdoc-Json: List impls for primitives](https://github.com/rust-lang/rust/pull/102321)
* [serialize return-position `impl Trait` in trait hidden values in foreign libraries](https://github.com/rust-lang/rust/pull/102164)
* [shrink `hir::def::Res`](https://github.com/rust-lang/rust/pull/101887)
* [some more cleanup for rustc_codegen_ssa](https://github.com/rust-lang/rust/pull/102551)
* [specify `DynKind::Dyn`](https://github.com/rust-lang/rust/pull/102506)
* [split collect.rs](https://github.com/rust-lang/rust/pull/102461)
* [split out the error reporting logic into a separate function](https://github.com/rust-lang/rust/pull/102476)
* [stabilize `#![feature(mixed_integer_ops)]`](https://github.com/rust-lang/rust/pull/101555)
* [stabilize bench_black_box](https://github.com/rust-lang/rust/pull/102232)
* [structured suggestion for missing `mut`/`const` in raw pointer](https://github.com/rust-lang/rust/pull/102284)
* [suggest unwrapping `???<T>` if a method cannot be found on it but is present on `T`](https://github.com/rust-lang/rust/pull/102288)
* [tell LLVM that `partition_point` returns a valid fencepost](https://github.com/rust-lang/rust/pull/102535)
* [tweak `FpCategory` example order](https://github.com/rust-lang/rust/pull/100470)
* [tweak `FulfillProcessor`](https://github.com/rust-lang/rust/pull/102348)
* [unescaping cleanups](https://github.com/rust-lang/rust/pull/102347)
* [upgrade dist-i586-gnu-i586-i686-musl to ubuntu:22.04](https://github.com/rust-lang/rust/pull/102530)
* [use already resolved `self_ty` in `confirm_fn_pointer_candidate`](https://github.com/rust-lang/rust/pull/102378)
* [use let-chaining in `WhileTrue::check_expr`](https://github.com/rust-lang/rust/pull/102455)
* [`HirId` for `deferred_transmute_checks`](https://github.com/rust-lang/rust/pull/102617)
* [create def ids for impl traits during ast lowering](https://github.com/rust-lang/rust/pull/102483)
* [docs: Improve AsRef / AsMut docs on blanket impls](https://github.com/rust-lang/rust/pull/99460)
* [docs: be less harsh in wording for Vec::from_raw_parts](https://github.com/rust-lang/rust/pull/99216)
* [fix issue with x.py setup running into explicit panic](https://github.com/rust-lang/rust/pull/102557)
* [fix minor ungrammatical sentence](https://github.com/rust-lang/rust/pull/102452)
* [fix: use git-commit-info for version information](https://github.com/rust-lang/rust/pull/100557)
* [introduce `{char, u8}::is_ascii_octdigit`](https://github.com/rust-lang/rust/pull/101308)
* [macros: diagnostic derive on enums](https://github.com/rust-lang/rust/pull/102189)
* [make invalid_value lint a bit smarter around enums](https://github.com/rust-lang/rust/pull/102281)
* [re-add git-commit-hash file to tarballs](https://github.com/rust-lang/rust/pull/102610)
* [remove FIXME, improve documentation](https://github.com/rust-lang/rust/pull/102416)
* [remove outdated coherence hack](https://github.com/rust-lang/rust/pull/102304)
* [remove the unused :: between trait and type to give user correct diag…](https://github.com/rust-lang/rust/pull/102421)
* [rename rustc_typeck to rustc_hir_analysis](https://github.com/rust-lang/rust/pull/102306)
* [rustdoc: Remove `clean::TraitWithExtraInfo` and queryify `is_notable_trait`](https://github.com/rust-lang/rust/pull/102384)
* [rustdoc: add method spacing to trait methods](https://github.com/rust-lang/rust/pull/102447)
* [rustdoc: add missing margin to no-docblock methods](https://github.com/rust-lang/rust/pull/102521)
* [rustdoc: clean up "normalize.css 8" input override CSS](https://github.com/rust-lang/rust/pull/102436)
* [rustdoc: cut margin-top from first header in docblock](https://github.com/rust-lang/rust/pull/102437)
* [rustdoc: give `.line-number` / `.line-numbers` meaningful names](https://github.com/rust-lang/rust/pull/102325)
* [rustdoc: merge CSS `table` rules into `.docblock`](https://github.com/rust-lang/rust/pull/102319)
* [rustdoc: re-sugar more cross-crate trait bounds](https://github.com/rust-lang/rust/pull/102439)
* [rustdoc: remove bad CSS font-weight on `.impl`, `.method`, etc](https://github.com/rust-lang/rust/pull/102442)
* [rustdoc: remove no-op CSS `.srclink { font-weight; font-size }`](https://github.com/rust-lang/rust/pull/102330)
* [rustdoc: remove no-op CSS `h3.variant, .sub-variant h4 { border-bottom: none }`](https://github.com/rust-lang/rust/pull/102505)
* [rustdoc: remove no-op CSS on `.impl, .method` etc](https://github.com/rust-lang/rust/pull/102550)
* [rustdoc: remove no-op source sidebar `opacity`](https://github.com/rust-lang/rust/pull/102491)
* [rustdoc: remove orphaned link on array bracket](https://github.com/rust-lang/rust/pull/102525)
* [rustdoc: remove redundant `#help-button` CSS](https://github.com/rust-lang/rust/pull/102367)
* [rustdoc: remove redundant mobile `.source > .sidebar` CSS](https://github.com/rust-lang/rust/pull/102380)
* [rustdoc: remove unneeded CSS `.rust-example-rendered { position }`](https://github.com/rust-lang/rust/pull/102481)
* [rustdoc: remove unused CSS selector `a.source`](https://github.com/rust-lang/rust/pull/102533)
* [rustdoc: use CSS containment to speed up render](https://github.com/rust-lang/rust/pull/102253)
* [session: remove now-unnecessary lint `#[allow]`s](https://github.com/rust-lang/rust/pull/102356)
* [tidy: make rustc dependency error less confusing](https://github.com/rust-lang/rust/pull/102468)
* [rust-analyzer: Add RequestFailed error code, as per spec 3.17](https://github.com/rust-lang/rust-analyzer/pull/13280)
* [rust-analyzer: Add config for supplying sysroot path](https://github.com/rust-lang/rust-analyzer/pull/13327)
* [rust-analyzer: Add proc-macro dependency to rustc crates](https://github.com/rust-lang/rust-analyzer/pull/13328)
* [rust-analyzer: Amalgamate file changes for the same file ids in process_changes](https://github.com/rust-lang/rust-analyzer/pull/13237)
* [rust-analyzer: Do not use the sysroot proc-macro server when a server path is given explicitly](https://github.com/rust-lang/rust-analyzer/pull/13326)
* [rust-analyzer: Don't retry requests that have already been cancelled](https://github.com/rust-lang/rust-analyzer/pull/13202)
* [rust-analyzer: Emit unconfigured code diagnostics for enum variants and struct/union fields](https://github.com/rust-lang/rust-analyzer/pull/13189)
* [rust-analyzer: Fix PackageInformation having the crate name instead of package name](https://github.com/rust-lang/rust-analyzer/pull/13296)
* [rust-analyzer: Fix annotations not resolving when lens location is set to whole item](https://github.com/rust-lang/rust-analyzer/pull/13318)
* [rust-analyzer: Fix find_path using the wrong module for visibility calculations](https://github.com/rust-lang/rust-analyzer/pull/13275)
* [rust-analyzer: Fix move_format_string_arg being tokentree unaware](https://github.com/rust-lang/rust-analyzer/pull/13321)
* [rust-analyzer: Fix requests not being retried anymore](https://github.com/rust-lang/rust-analyzer/pull/13319)
* [rust-analyzer: Fix trait impl item completions using macro file text ranges](https://github.com/rust-lang/rust-analyzer/pull/13324)
* [rust-analyzer: Fix type alias hovers not rendering generic parameters](https://github.com/rust-lang/rust-analyzer/pull/13320)
* [rust-analyzer: Make assist tests panic again on empty source changes](https://github.com/rust-lang/rust-analyzer/pull/13301)
* [rust-analyzer: Prioritize restart messages in flycheck](https://github.com/rust-lang/rust-analyzer/pull/13338)
* [rust-analyzer: Remove obsolete in-rust-tree feature from sourcegen](https://github.com/rust-lang/rust-analyzer/pull/13295)
* [rust-analyzer: Use cfg(any()) instead of cfg(FALSE) for disabling proc-macro test](https://github.com/rust-lang/rust-analyzer/pull/13300)
* [rust-analyzer: docs(guide): fix Analysis and AnalysisHost doc links](https://github.com/rust-lang/rust-analyzer/pull/13272)
* [rust-analyzer: fix: infer for-loop item type with `IntoIterator` and `Iterator`](https://github.com/rust-lang/rust-analyzer/pull/13311)
* [rust-analyzer: fix: treat enum variants as generic item on their own](https://github.com/rust-lang/rust-analyzer/pull/13339)
* [rust-analyzer: internal: change generic parameter order](https://github.com/rust-lang/rust-analyzer/pull/13335)
* [address clippy lints](https://github.com/rust-lang/rust-bindgen/pull/2291)
* [don't traverse through special-cased <stdint.h> types](https://github.com/rust-lang/rust-bindgen/pull/2287)
* [enables blocklisting of Objective-C methods](https://github.com/rust-lang/rust-bindgen/pull/2146)
* [make `BindgenOptions` clonable](https://github.com/rust-lang/rust-bindgen/pull/2285)
* [move codegen postprocessing to its own module](https://github.com/rust-lang/rust-bindgen/pull/2282)
* [clippy: Add `cargo lintcheck --recursive` to check dependencies of crates](https://github.com/rust-lang/rust-clippy/pull/9510)
* [clippy: Don't lint `*_interior_mutable_const` on unions due to potential ICE](https://github.com/rust-lang/rust-clippy/pull/9539)
* [clippy: Don't lint unstable moves in `std_instead_of_core`](https://github.com/rust-lang/rust-clippy/pull/9545)
* [clippy: Fix #9544](https://github.com/rust-lang/rust-clippy/pull/9559)
* [clippy: Fix and improve `match_type_on_diagnostic_item`](https://github.com/rust-lang/rust-clippy/pull/7962)
* [clippy: Implement `manual_clamp` lint](https://github.com/rust-lang/rust-clippy/pull/9484)
* [clippy: Remove unused `.fixed` files, only run asm_syntax doctests on x86](https://github.com/rust-lang/rust-clippy/pull/9574)
* [clippy: Replace `expr_visitor` with  `for_each_expr`](https://github.com/rust-lang/rust-clippy/pull/8762)
* [clippy: Rustup](https://github.com/rust-lang/rust-clippy/pull/9516)
* [clippy: Silence [`question_mark`] in const context](https://github.com/rust-lang/rust-clippy/pull/9487)
* [clippy: [`drop_copy`]: Do not lint idiomatic in match arm](https://github.com/rust-lang/rust-clippy/pull/9491)
* [clippy: [`manual_assert`]: Preserve comments in the suggestion](https://github.com/rust-lang/rust-clippy/pull/9479)
* [clippy: [`needless_return`] Recursively remove unneeded semicolons](https://github.com/rust-lang/rust-clippy/pull/9497)
* [clippy: [`redundant_closure`] Fix suggestion causes error for `impl FnMut`](https://github.com/rust-lang/rust-clippy/pull/9556)
* [clippy: [`should_implement_trait`] Also lint `default` method](https://github.com/rust-lang/rust-clippy/pull/9546)
* [clippy: [`unnecessary_cast`] add parenthesis when negative number uses a method](https://github.com/rust-lang/rust-clippy/pull/9577)
* [clippy: [`unnecessary_lazy_eval`] Do not lint in external macros](https://github.com/rust-lang/rust-clippy/pull/9486)
* [clippy: [`unnecessary_lazy_evaluations`] Do not suggest switching to early evaluation when type has custom `Drop`](https://github.com/rust-lang/rust-clippy/pull/9551)
* [clippy: [arithmetic-side-effects] Consider references](https://github.com/rust-lang/rust-clippy/pull/9507)
* [clippy: `suboptimal_flops` lint for multiply and subtract](https://github.com/rust-lang/rust-clippy/pull/9581)
* [clippy: add `box-default` lint](https://github.com/rust-lang/rust-clippy/pull/9511)
* [clippy: avoid doc-link-with-quotes in code blocks](https://github.com/rust-lang/rust-clippy/pull/9567)
* [clippy: doc: make the usage of clippy.toml more clear](https://github.com/rust-lang/rust-clippy/pull/9557)
* [clippy: fix [`needless_borrow`], [`explicit_auto_deref`] FPs on unions](https://github.com/rust-lang/rust-clippy/pull/9490)
* [clippy: let `upper_case_acronyms` check the enum name](https://github.com/rust-lang/rust-clippy/pull/9580)
* [clippy: let unnecessary_cast work for trivial non_literal expressions](https://github.com/rust-lang/rust-clippy/pull/9576)
* [clippy: lint nested patterns and slice patterns in `needless_borrowed_reference`](https://github.com/rust-lang/rust-clippy/pull/9573)
* [clippy: new `implicit_saturating_add` lint](https://github.com/rust-lang/rust-clippy/pull/9549)
* [clippy: use `is_integer_literal` more](https://github.com/rust-lang/rust-clippy/pull/9571)
* [clippy: use rustc_tools_util dependency from crates.io instead of this repo](https://github.com/rust-lang/rust-clippy/pull/9569)
* [ignore non-error compiler output](https://github.com/rust-lang/rust.vim/pull/445)
* [link to the correct page in "about this guide"](https://github.com/rust-lang/rustc-dev-guide/pull/1456)
* [add a filter for try commits in graphs, compare page and triage](https://github.com/rust-lang/rustc-perf/pull/1452)
* [add triage 2022-09-27](https://github.com/rust-lang/rustc-perf/pull/1449)
* [fix link in perf-runner.md](https://github.com/rust-lang/rustc-perf/pull/1451)
* [codegen\_gcc: Add CI tests with a sysroot compiled in release mode](https://github.com/rust-lang/rustc_codegen_gcc/pull/224)
* [codegen\_gcc: Fix warnings](https://github.com/rust-lang/rustc_codegen_gcc/pull/227)
* [codegen\_gcc: Implement llvm.prefetch](https://github.com/rust-lang/rustc_codegen_gcc/pull/226)
* [codegen\_gcc: simd: enable simd_as intrinsic](https://github.com/rust-lang/rustc_codegen_gcc/pull/228)
* [codegen\_gcc: simd: implement float math intrinsics](https://github.com/rust-lang/rustc_codegen_gcc/pull/219)
* [allow users to debug their processes](https://github.com/rust-lang/simpleinfra/pull/119)
* [fix formatting of Terraform file](https://github.com/rust-lang/simpleinfra/pull/121)
* [set a filesystem quota for dev desktop users](https://github.com/rust-lang/simpleinfra/pull/120)

### Rust Compiler Performance Triage

A great week, with 170 primary benchmark scenarios seeing improvement. Every PR
flagged by perf provided at least some wins, and perhaps more impressive: No
rollup PR's were flagged by perf this week! Furthermore, cjgillot fixed an issue
where incremental compilation was being unnecessarily hindered by our span and
lint system. Great work everyone!

Triage done by **@pnkfelix**.
Revision range: [d9297d22..02cd79af](https://perf.rust-lang.org/?start=d9297d22ad9edc2b56f0dd8734c1187a0c88be69&end=02cd79afb8080fce8c8ce35533c54d8ecf8f390e&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-10-04.md)

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

* [disposition: merge] [make const_err a hard error](https://github.com/rust-lang/rust/pull/102091)
* [disposition: merge] [Elaborate supertrait bounds when triggering unused_`must_use` on `impl Trait`](https://github.com/rust-lang/rust/pull/102287)
* [disposition: merge] [Stabilize proc_macro Span::source_text](https://github.com/rust-lang/rust/issues/101991)
* [disposition: merge] [`const`-stablilize `NonNull::as_ref`](https://github.com/rust-lang/rust/pull/102198)
* [disposition: merge] [Add documentation about the memory layout of `UnsafeCell<T>`](https://github.com/rust-lang/rust/pull/101717)
* [disposition: merge] [Handle projections as uncovered types during coherence check](https://github.com/rust-lang/rust/pull/100555)
* [disposition: merge] [Never panic in `thread::park` and `thread::park_timeout`](https://github.com/rust-lang/rust/pull/102412)
* [disposition: merge] [Stabilize `nonzero_bits`](https://github.com/rust-lang/rust/pull/101514)
* [disposition: merge] [`EscapeAscii` is not an `ExactSizeIterator`](https://github.com/rust-lang/rust/pull/99880)
* [disposition: merge] [Change default level of INVALID_HTML_TAGS to warning and stabilize it](https://github.com/rust-lang/rust/pull/101720)
* [disposition: merge] [Add `Box<[T; N]>: TryFrom<Vec<T>>`](https://github.com/rust-lang/rust/pull/101837)
* [disposition: merge] [add `no_compile` doctest attribute](https://github.com/rust-lang/rust/pull/96573)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-10-05 - 2022-11-02 🦀

### Virtual

* 2022-09-28 | Virtual (London, UK) | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-30 | Virtual (Minneapolis, MN, US) | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/minneapolis-rust-meetup/events/288601893/)
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)
* 2022-10-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #18**](https://www.meetup.com/rust-noris/events/hlvbvsydcnbrb/)
* 2022-10-08 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-10-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcnbpb/)
* 2022-10-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-12 | Virtual (Erlangen, DE) | [Rust Franken](https://www.meetup.com/rust-nerf/)
    * [**Rust Franken Meetup #4**](https://www.meetup.com/rust-nerf/events/288723552/)
* 2022-10-12 | Virtual (San Francisco, CA, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/)
* 2022-10-13 | Virtual (Berlin, DE) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-15 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 2 (CuteCopter): Reverse Engineering a tiny drone**](https://www.meetup.com/rust-noris/events/287347851/)
* 2022-10-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Impractical Rust: The HATETRIS World Record**](https://www.meetup.com/rustdc/events/vdhxgsydcnbxb/)
* 2022-10-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcnbzb/)
* 2022-10-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcnbbc/)
* 2022-10-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcnbhc/)

### Asia

* 2022-10-11 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Cost-Efficient Rust in Practice**](https://www.meetup.com/tokyo-rust-meetup/events/288597490/)

### Europe

* 2022-09-28 | London, UK + Virtual | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-09-30 | Berlin, DE | [RustFi Hackathon](https://rustfi.keyrock.com/)
    * [**RustFi Hackathon 30 Sept - 2 Oct**](https://rustfi.keyrock.com/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)
* 2022-10-03 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Microsoft Reactor**](https://www.meetup.com/Stockholm-Rust/events/288453360/)
* 2022-10-04 | Helsinki, FI | [Finland Rust Meetup](https://www.meetup.com/Finland-Rust-Meetup/)
    * [**October meetup**](https://www.meetup.com/Finland-Rust-Meetup/events/288724370/)
* 2022-10-06 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #29**](https://www.meetup.com/rust-wroclaw/events/288667400/)
* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)
* 2022-10-13 | Berlin, DE + Virtual | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-18 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #53**](https://www.meetup.com/rust-paris/events/288735204/)

### North America

* 2022-09-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288590318/)
* 2022-09-29 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Zola o como la comunidad de RustMX tiene página**](https://www.meetup.com/rust-mx/events/288388973/)
* 2022-10-13 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcnbrb/)
* 2022-10-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcnbxb/)
* 2022-10-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Anyhow ? Turbofish ::<> / HTTP calls and errors in Rust.**](https://www.meetup.com/rust-nyc/events/288756215/)
* 2022-10-20 | New York, NY, US | [Cloud Native New York](https://www.meetup.com/cloud-native-new-york/)
    * [**Cloud-native Search Engine for Log Management and Analytics.**](https://www.meetup.com/cloud-native-new-york/events/288818963/)
* 2022-10-25 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**Rust DHCP**](https://www.meetup.com/rust-toronto/events/288589539/)

### Oceania

* 2022-10-10 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust Lightning Talks**](https://www.meetup.com/rust-sydney/events/288746516/)
* 2022-10-20 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Tune Up Edition: software engineering management**](https://www.meetup.com/rust-wellington/events/288738684/)

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
