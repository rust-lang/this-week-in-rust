Title: This Week in Rust 567
Number: 567
Date: 2024-10-02
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

[rPGP 0.14.0 (a pure Rust implementation of OpenPGP) now supports the new RFC 9580](https://fosstodon.org/@hko/113198947595455844)

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

* [This Development-cycle in Cargo: 1.82](https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html)
* [Return type notation MVP: Call for testing!](https://blog.rust-lang.org/inside-rust/2024/09/26/rtn-call-for-testing.html)

### Project/Tooling Updates

* [Two weeks of binsider](https://binsider.dev/blog/v020/)
* [egui 0.29](https://github.com/emilk/egui/releases/tag/0.29.0)
* [pantheon: Parsing command line arguments](https://traxys.me/sheshat_pantheon_3.html)
* [System76 COSMIC Alpha 2 Released](https://blog.system76.com/post/cosmic-alpha-2-press-release)
* [Linus and Dirk on succession, Rust, and more](https://lwn.net/SubscriberLink/990534/a059b651e416a0c5/)
* [What the Nova GPU driver needs](https://lwn.net/Articles/990736/)
* [Getting PCI driver abstractions upstream](https://lwn.net/SubscriberLink/990918/ee00284446ec8d85/)
* [Coccinelle for Rust](https://lwn.net/SubscriberLink/991399/8bfb2ec24205dbaa/)
* [An update on gccrs development](https://lwn.net/SubscriberLink/991199/b109442b923b3909/)
* [BTF, Rust, and the kernel toolchain](https://lwn.net/SubscriberLink/991719/3fecd51a9a75f011/)
* [tokio-graceful 0.2.0: support shutdown trigger delay and forceful shutdown](https://github.com/plabayo/tokio-graceful/releases/tag/0.2.0)
* [Cargo Watch 8.5.3: the final update, as the project goes dormant](https://github.com/watchexec/cargo-watch/releases/tag/v8.5.3)

### Observations/Thoughts

* [Best practices for error handling in kernel Rust](https://lwn.net/SubscriberLink/990489/eab6106fa595052e/)
* [A discussion of Rust safety documentation](https://lwn.net/Articles/990273/)
* [(Re)Using rustc components in gccrs](https://rust-gcc.github.io/2024/09/20/reusing-rustc-components.html)
* [Whence '\n'?](https://rodarmor.com/blog/whence-newline/)
* [Should you use Rust in LLM based tools for performance?](https://bosun.ai/posts/rust-for-genai-performance/)
* [Code Generation in Rust vs C++26](https://brevzin.github.io/c++/2024/09/30/annotations/)
* [Rust adventure to develop a Game Boy emulator — Part 3: CPU Instructions](https://medium.com/@wolferxy/rust-adventure-to-develop-a-game-boy-emulator-part-3-cpu-instructions-d6d1d727026f)
* [Improved Turso (libsql) ergonomics in Rust](https://codethoughts.io/posts/2024-10-01-improved-turso-ergonomics-in-rust/)
* [Rewriting Rust](https://josephg.com/blog/rewriting-rust/)
* [Making overwrite opt-in #crazyideas](https://smallcultfollowing.com/babysteps/blog/2024/09/26/overwrite-trait/)
* [Rust needs a web framework for lazy developers](https://ntietz.com/blog/rust-needs-a-web-framework-for-lazy-developers/)
* [Safety Goggles for Alchemists](https://jack.wrenn.fyi/blog/safety-goggles-for-alchemists/)
* [Beyond multi-core parallelism: faster Mandelbrot with SIMD](https://pythonspeed.com/articles/optimizing-with-simd/)
* [Nine Rules for Running Rust on WASM WASI](https://medium.com/towards-data-science/nine-rules-for-running-rust-on-wasm-wasi-550cd14c252a)
* [Rust needs an extended standard library](https://kerkour.com/rust-stdx)

### Rust Walkthroughs

* [New Book: "100 Exercises to Learn Rust: A hands-on course by Mainmatter"](https://rust-exercises.com/100-exercises/).
* [Rust interop in practice: speaking Python and Javascript](https://tweedegolf.nl/en/blog/136/rust-interop-in-practice-speaking-python-and-javascript)
* [Series] [Mastering Dependency Injection in Rust: Despatma with Lifetimes](https://chesedo.me/blog/despatma-with-singleton-and-scoped-support/)
* [Sqlx4k - Interoperability between Kotlin and Rust, using FFI (Part 1)](https://smyrgeorge.github.io/posts/sqlx4k---interoperability-between-kotlin-and-rust-using-ffi-part-1/)
* [Serde for Trait objects](https://voelklmichael.github.io/Blog/2024/10/01/serde-trait-part1.html)
* [video] [Build with Naz : Rust clap colorization](https://www.youtube.com/watch?v=lzMYDA6St0s)

### Miscellaneous

* [Resources for learning Rust for kernel development](https://lwn.net/SubscriberLink/990619/cb5f47f5d88818e4/)

## Crate of the Week

This week's crate is [binsider](https://binsider.dev), a terminal UI tool for analyzing binary files.

Despite yet another week without suggestions, llogiq is appropriately pleased with his choice.

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
* [**Rustikon CFP**](https://sessionize.com/rustikon-2025) | [Event Page](https://www.rustikon.dev/) | Closes 2024-10-13 | Warsaw, PL | Event 2025-03-26

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

451 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-09-24..2024-10-01

* [add new Tier-3 target: `loongarch64-unknown-linux-ohos`](https://github.com/rust-lang/rust/pull/130750)
* [add RISC-V vxworks targets](https://github.com/rust-lang/rust/pull/130549)
* [`cfg_match` Generalize inputs](https://github.com/rust-lang/rust/pull/130313)
* [add InProgress ErrorKind gated behind `io_error_inprogress` feature](https://github.com/rust-lang/rust/pull/130789)
* [allow instantiating object trait binder when upcasting](https://github.com/rust-lang/rust/pull/130866)
* [allow instantiating trait object binder in ptr-to-ptr casts](https://github.com/rust-lang/rust/pull/130944)
* [ban combination of GCE and new solver](https://github.com/rust-lang/rust/pull/130787)
* [collect relevant item bounds from trait clauses for nested rigid projections](https://github.com/rust-lang/rust/pull/120752)
* [diagnostics: wrap fn cast suggestions in parens when needed](https://github.com/rust-lang/rust/pull/130911)
* [don't trap into the debugger on panics under Linux](https://github.com/rust-lang/rust/pull/130810)
* [enable compiler fingerprint logs in verbose mode](https://github.com/rust-lang/rust/pull/131056)
* [fix `adt_const_params` leaking `{type error}` in error msg](https://github.com/rust-lang/rust/pull/131038)
* [fix diagnostics for coroutines with () as input](https://github.com/rust-lang/rust/pull/130820)
* [fix error span if arg to `asm!()` is a macro call](https://github.com/rust-lang/rust/pull/130917)
* [fix the misleading diagnostic for `let_underscore_drop` on type without `Drop` implementation](https://github.com/rust-lang/rust/pull/130833)
* [fix: ices on virtual-function-elimination about principal trait](https://github.com/rust-lang/rust/pull/130734)
* [implement trim-paths sysroot changes - take 2](https://github.com/rust-lang/rust/pull/129687) (RFC [#3127](https://rust-lang.github.io/rfcs/3127-trim-paths.html))
* [improve compile errors for invalid ptr-to-ptr casts with trait objects](https://github.com/rust-lang/rust/pull/130234)
* [initial std library support for NuttX](https://github.com/rust-lang/rust/pull/130595)
* [make `clashing_extern_declarations` considering generic args for ADT field](https://github.com/rust-lang/rust/pull/130924)
* [mark some more types as having insignificant dtor](https://github.com/rust-lang/rust/pull/130914)
* [on implicit `Sized` bound on fn argument, point at type instead of pattern](https://github.com/rust-lang/rust/pull/130912)
* [only add an automatic SONAME for Rust dylibs](https://github.com/rust-lang/rust/pull/130960)
* [pass Module Analysis Manager to Standard Instrumentations](https://github.com/rust-lang/rust/pull/130850)
* [pass correct HirId to `late_bound_vars` in diagnostic code](https://github.com/rust-lang/rust/pull/130879)
* [preserve brackets around if-lets and skip while-lets](https://github.com/rust-lang/rust/pull/131035)
* [properly elaborate effects implied bounds for super traits](https://github.com/rust-lang/rust/pull/129499)
* [reference UNSPECIFIED instead of `INADDR_ANY` in `join_multicast_v4`](https://github.com/rust-lang/rust/pull/130922)
* [reject leading unsafe in `cfg!(...)` and `--check-cfg`](https://github.com/rust-lang/rust/pull/131057)
* [rename `standalone` doctest attribute into `standalone_crate`](https://github.com/rust-lang/rust/pull/130931)
* [reorder stack spills so that constants come later](https://github.com/rust-lang/rust/pull/130329)
* [separate collection of crate-local inherent impls from error tracking](https://github.com/rust-lang/rust/pull/130764)
* [simple validation for unsize coercion in MIR validation](https://github.com/rust-lang/rust/pull/130735)
* [check vtable projections for validity in miri](https://github.com/rust-lang/rust/pull/130727)
* [miri: implements `arc4random_buf` shim for freebsd/solarish platforms](https://github.com/rust-lang/miri/pull/3918)
* [miri: make returning io errors more uniform and convenient](https://github.com/rust-lang/miri/pull/3929)
* [miri: refactor `return_read_bytes_and_count` and `return_written_byte_count_or_error`](https://github.com/rust-lang/miri/pull/3923)
* [miri: switch custom target JSON test to a less exotic target](https://github.com/rust-lang/miri/pull/3915)
* [skip query in `get_parent_item` when possible](https://github.com/rust-lang/rust/pull/130618)
* [stabilize `const_cell_into_inner`](https://github.com/rust-lang/rust/pull/130972)
* [stabilize `const_intrinsic_copy`](https://github.com/rust-lang/rust/pull/130762)
* [stabilize `const_refs_to_static`](https://github.com/rust-lang/rust/pull/129759)
* [stabilize `option_get_or_insert_default`](https://github.com/rust-lang/rust/pull/129087)
* [improve autovectorization of `to_lowercase` / `to_uppercase` functions](https://github.com/rust-lang/rust/pull/123778)
* [add `File` constructors that return files wrapped with a buffer](https://github.com/rust-lang/rust/pull/130803)
* [add `must_use` attribute to `len_utf8` and `len_utf16`](https://github.com/rust-lang/rust/pull/130819)
* [add `optimize_for_size` variants for stable and unstable sort as well as `select_nth_unstable`](https://github.com/rust-lang/rust/pull/129587)
* [fix `read_buf` uses in `std`](https://github.com/rust-lang/rust/pull/125404)
* [make ptr metadata functions callable from stable const fn](https://github.com/rust-lang/rust/pull/130966)
* [mark `make_ascii_uppercase` and `make_ascii_lowercase` in `[u8]` and `str` as const](https://github.com/rust-lang/rust/pull/130738)
* [fix some cfg logic around `optimize_for_size` and 16-bit targets](https://github.com/rust-lang/rust/pull/130832)
* [hook up `std::net` to wasi-libc on wasm32-wasip2 target](https://github.com/rust-lang/rust/pull/129638)
* [compute `RUST_EXCEPTION_CLASS` from native-endian bytes](https://github.com/rust-lang/rust/pull/130897)
* [hashbrown: change signature of `get_many_mut` APIs](https://github.com/rust-lang/hashbrown/pull/562)
* [regex: add `SetMatches::matched_all`](https://github.com/rust-lang/regex/pull/1228)
* [cargo timings: support dark color scheme in HTML output](https://github.com/rust-lang/cargo/pull/14588)
* [cargo toml: Add `autolib`](https://github.com/rust-lang/cargo/pull/14591)
* [cargo rustc: give trailing flags higher precedence on nightly](https://github.com/rust-lang/cargo/pull/14587)
* [cargo config: Don't double-warn about `$CARGO_HOME/config`](https://github.com/rust-lang/cargo/pull/14579)
* [cargo compiler: zero-copy deserialization when possible](https://github.com/rust-lang/cargo/pull/14608)
* [cargo: add `CARGO_MANIFEST_PATH` env variable](https://github.com/rust-lang/cargo/pull/14404)
* [cargo: lockfile path implies --locked on cargo install](https://github.com/rust-lang/cargo/pull/14556)
* [cargo: make lockfile v4 the default](https://github.com/rust-lang/cargo/pull/14595)
* [cargo: correct error count for `cargo check --message-format json`](https://github.com/rust-lang/cargo/pull/14598)
* [cargo perf: improve quality of completion performance traces](https://github.com/rust-lang/cargo/pull/14592)
* [cargo test: add support for features in the sat resolver](https://github.com/rust-lang/cargo/pull/14583)
* [cargo test: relax compiler panic assertions](https://github.com/rust-lang/cargo/pull/14618)
* [cargo test: relax panic output assertion](https://github.com/rust-lang/cargo/pull/14602)
* [rustdoc perf: clone `clean::Item` less](https://github.com/rust-lang/rust/pull/130857)
* [rustdoc: do not animate :target when user prefers reduced motion](https://github.com/rust-lang/rust/pull/130862)
* [rustdoc: inherit parent's stability where applicable](https://github.com/rust-lang/rust/pull/130798)
* [rustdoc: rewrite stability inheritance as a doc pass](https://github.com/rust-lang/rust/pull/131076)
* [rustdoc: copy correct path to clipboard for modules/keywords/primitives](https://github.com/rust-lang/rust/pull/131023)
* [rustdoc: redesign toolbar and disclosure widgets](https://github.com/rust-lang/rust/pull/129545)
* [rustdoc toolbar: Adjust spacings and sizing to improve behavior with over-long names](https://github.com/rust-lang/rust/pull/131002)
* [add `field@` and `variant@` doc-link disambiguators](https://github.com/rust-lang/rust/pull/130587)
* [rustfmt: add `style_edition` 2027](https://github.com/rust-lang/rustfmt/pull/6324)
* [clippy: `wildcard_in_or_patterns` will no longer be triggered for types annotated with `#[nonexhaustive]`](https://github.com/rust-lang/rust-clippy/pull/13456)
* [clippy: `invalid_null_ptr_usage`: fix false positives for `std::ptr::slice_from_raw_parts` functions](https://github.com/rust-lang/rust-clippy/pull/13452)
* [clippy: add reasons for or remove some `//@no-rustfix` annotations](https://github.com/rust-lang/rust-clippy/pull/13442)
* [clippy: extend `needless_lifetimes` to suggest eliding `impl` lifetimes](https://github.com/rust-lang/rust-clippy/pull/13286)
* [clippy: specifying reason in `expect(clippy::needless_return)` no longer triggers false positive](https://github.com/rust-lang/rust-clippy/pull/13393)
* [clippy: ignore `--print`/`-Vv` requests in `clippy-driver`](https://github.com/rust-lang/rust-clippy/pull/13468)
* [clippy: remove method call receiver special casing in `unused_async` lint](https://github.com/rust-lang/rust-clippy/pull/13471)
* [clippy: suggest `Option<&T>` instead of `&Option<T>`](https://github.com/rust-lang/rust-clippy/pull/13336)
* [clippy: convert `&Option<T>` to `Option<&T>`](https://github.com/rust-lang/rust-clippy/pull/13469)
* [clippy: use `std_or_core` to determine the correct prefix](https://github.com/rust-lang/rust-clippy/pull/13453)
* [rust-analyzer: building before a debugging session was restarted](https://github.com/rust-lang/rust-analyzer/pull/17923)
* [rust-analyzer: index workspace symbols at startup rather than on the first symbol search](https://github.com/rust-lang/rust-analyzer/pull/18180)
* [rust-analyzer: provide an config option to not set `cfg(test)`](https://github.com/rust-lang/rust-analyzer/pull/18085)
* [rust-analyzer: ambiguity with CamelCase diagnostic messages, align with rustc warnings](https://github.com/rust-lang/rust-analyzer/pull/18207)
* [rust-analyzer: better support references in consuming postfix completions](https://github.com/rust-lang/rust-analyzer/pull/18161)
* [rust-analyzer: consider lifetime GATs object unsafe](https://github.com/rust-lang/rust-analyzer/pull/18162)
* [rust-analyzer: don't report a startup error when a discover command is configured](https://github.com/rust-lang/rust-analyzer/pull/18193)
* [rust-analyzer: fix a bug in span map merge, and add explanations of how span maps are stored](https://github.com/rust-lang/rust-analyzer/pull/18166)
* [rust-analyzer: fix name resolution when an import is resolved to some namespace and then later in the algorithm another namespace is added](https://github.com/rust-lang/rust-analyzer/pull/18160)
* [rust-analyzer: fix resolution of label inside macro](https://github.com/rust-lang/rust-analyzer/pull/18210)
* [rust-analyzer: handle block exprs as modules when finding their parents](https://github.com/rust-lang/rust-analyzer/pull/18206)
* [rust-analyzer: pass all-targets for build scripts in more cli commands](https://github.com/rust-lang/rust-analyzer/pull/18184)

### Rust Compiler Performance Triage

A quiet week without too many perf. changes, although there was a nice perf. win on documentation
builds thanks to [#130857](https://github.com/rust-lang/rust/. Overall the results were positive.

Triage done by **@kobzol**.
Revision range: [4cadeda9..c87004a1](https://perf.rust-lang.org/?start=4cadeda932d5c261a9a0b1bbd25c4486e4e0a4c6&end=c87004a1f5be671e3f03f69fb13d8915bdbb6a52&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.2%, 0.8%]    | 11    |
| Regressions ❌ <br /> (secondary)  | 0.3%  | [0.2%, 0.6%]    | 19    |
| Improvements ✅ <br /> (primary)   | -1.2% | [-14.9%, -0.2%] | 21    |
| Improvements ✅ <br /> (secondary) | -1.0% | [-2.3%, -0.3%]  | 5     |
| All ❌✅ (primary)                 | -0.6% | [-14.9%, 0.8%]  | 32    |

3 Regressions, 4 Improvements, 3 Mixed; 2 of them in rollups
47 artifact comparisons made in total

[Full report here](https://github.com/Kobzol/rustc-perf/blob/ca0e3756481095ac4cce9305d5911f832e8d3b80/triage/2024-10-01.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: postpone] [Make `cargo install` respect lockfiles by default](https://github.com/rust-lang/rfcs/pull/3585)
* [disposition: postpone] [RFC: Templating `CARGO_TARGET_DIR` to make it the parent of all target directories](https://github.com/rust-lang/rfcs/pull/3371)
* [disposition: postpone] [Cargo: providing artifacts (for artifact dependencies) via build.rs](https://github.com/rust-lang/rfcs/pull/3035)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for constify-ing non-trait `Duration` methods](https://github.com/rust-lang/rust/issues/72440)
* [disposition: merge] [Tracking Issue for const `Result` methods](https://github.com/rust-lang/rust/issues/82814)
* [disposition: merge] [Tracking issue for const `Option` functions](https://github.com/rust-lang/rust/issues/67441)
* [disposition: merge] [Tracking Issue for `slice_first_last_chunk` feature (`slice::{split_,}{first,last}_chunk{,_mut}`)](https://github.com/rust-lang/rust/issues/111774)
* [disposition: merge] [Partially stabilize const_pin](https://github.com/rust-lang/rust/pull/130136)
* [disposition: merge] [Check elaborated projections from dyn don't mention unconstrained late bound lifetimes](https://github.com/rust-lang/rust/pull/130367)
* [disposition: merge] [Stabilize the `map`/`value` methods on `ControlFlow`](https://github.com/rust-lang/rust/pull/130518)
* [disposition: merge] [Do not consider match/let/ref of place that evaluates to `!` to diverge, disallow coercions from them too](https://github.com/rust-lang/rust/pull/129392)
* [disposition: merge] [Tracking issue for const slice::from_raw_parts_mut (const_slice_from_raw_parts_mut)](https://github.com/rust-lang/rust/issues/67456)
* [disposition: merge] [Stabilize `const {slice,array}::from_mut`](https://github.com/rust-lang/rust/pull/130538)
* [disposition: merge] [Tracking Issue for `feature(const_slice_split_at_mut)`](https://github.com/rust-lang/rust/issues/101804)
* [disposition: merge] [Tracking Issue for `str::from_utf8`_unchecked_mut as a `const fn`](https://github.com/rust-lang/rust/issues/91005)
* [disposition: merge] [Tracking Issue for `#![feature(const_unsafecell_get_mut)]`](https://github.com/rust-lang/rust/issues/88836)
* [disposition: merge] [Tracking Issue for const_maybe_uninit_assume_init](https://github.com/rust-lang/rust/issues/86722)
* [disposition: merge] [Tracking issue for `#![feature(const_float_classify)]`](https://github.com/rust-lang/rust/issues/72505)
* [disposition: merge] [Tracking Issue for const_str_as_mut](https://github.com/rust-lang/rust/issues/130086)
* [disposition: merge] [Tracking Issue for `pin_deref_mut`](https://github.com/rust-lang/rust/issues/86918)
* [disposition: merge] [Tracking Issue for `UnsafeCell::from_mut`](https://github.com/rust-lang/rust/issues/111645)
* [disposition: merge] [Tracking Issue for `BufRead::skip_until`](https://github.com/rust-lang/rust/issues/111735)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [docs(charter): Declare new Intentional Artifacts as 'small' changes](https://github.com/rust-lang/cargo/pull/14599)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* [disposition: merge] [Meeting proposal: rename "object safety" to "dyn compatibility"](https://github.com/rust-lang/lang-team/issues/286)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [num::WrappingFrom trait for conversions between integers](https://github.com/rust-lang/rfcs/pull/3703)
* [new] [Add helper methods on primitive pointer types for pointer tagging](https://github.com/rust-lang/rfcs/pull/3700)

## Upcoming Events

Rusty Events between 2024-10-02 - 2024-10-30 🦀

### Virtual
* 2024-10-02 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 8 - Asynchronous Programming**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-10-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Ezra Singh on Rust's HashMap**](https://www.meetup.com/indyrs/events/302031649/)
* 2024-10-02 | Virtual (Vancouver, BC, CA) | [Vancouver Postgres](https://www.meetup.com/vancouver-postgres/)
    * [**Leveraging a PL/RUST extension to protect sensitive data in PostgreSQL**](https://www.meetup.com/vancouver-postgres/events/302160672/)
* 2024-10-03 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 1 of 4 - Rust Essentials: Build Your First API**](https://www.meetup.com/women-in-rust/events/303213069/)
* 2024-10-03 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897992/)
* 2024-10-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346983/)
* 2024-10-10 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 2 of 4 - Navigating Rust Web Frameworks: Axum, Actix, and Rocket**](https://www.meetup.com/women-in-rust/events/303213792/)
* 2024-10-10 | Virtual (Barcelona, ES) | [BcnRust](https://bcnrust.github.io) + [Codurance](https://www.codurance.com/) + [Heavy Duty Builders](https://heavyduty.builders/)
    * [**15th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/303443195/)
* 2024-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633270/)
* 2024-10-10 | Virtual (Girona, ES) | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Leveraging Rust to Improve Your Programming Fundamentals & De Rust A Solana**](https://www.meetup.com/rust-girona/events/303484509/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-14 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302828025/)
* 2024-10-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346970/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 3 of 4 - Hackathon Ideation Lab**](https://www.meetup.com/women-in-rust/events/303213817/)
* 2024-10-17| Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898023)
* 2024-10-22 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcnbdc/)
* 2024-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633271/)
* 2024-10-26 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 2024-10-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585671/)

### Africa
* 2024-10-05 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 2024-10-09 | Subang Jaya / Kuala Lumpur, Selangor, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup - Traits and How to Read Trait (October 2024)**](https://docs.google.com/forms/d/e/1FAIpQLScNS5IWmnzTTJAOw-RIxdj4_BWbxB5NVmAVO30XHr_viMbLqQ/viewform)
* 2024-10-17 - 2024-10-18 | Beijing, CN | [Global Open-Source Innovation Meetup (GOSIM)](https://www.gosim.org/)
    * [**GOSIM 2024**](https://china2024.gosim.org/)
* 2024-10-19 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**October 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/october-2024-rustacean-meetup/)

### Europe
* 2024-10-02 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Rust for Rustaceans Book Club: Chapter 11: Foreign Function Interfaces**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/303445033/)
* 2024-10-02 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/Stockholm-Rust/events/303213095)
* 2022-10-03 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820273/)
* 2024-10-03 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154268/)
* 2024-10-09 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcnbmb/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 2024-10-17 | Darmstadr, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Code Together**](https://www.meetup.com/rust-rhein-main/events/303240000/)
* 2024-10-15 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/303606799/)
* 2024-10-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 2024-10-15 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/303273953/)
* 2024-10-16 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks October - Leptos and Crux**](https://www.meetup.com/rust-manchester/events/303658240/)
* 2024-10-17 | Barcelona, ES | [BcnRust](https://bcnrust.github.io)
    * [**16th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/303792888/)
* 2024-10-17 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #3**](https://www.meetup.com/rust-bern/events/303617330/)
* 2024-10-22 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**New Rust Warsaw Meetup #2**](https://www.meetup.com/rust-warsaw/events/303619536/)
* 2024-10-28 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 2024-10-29 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/303479865)
* 2024-10-30 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn October 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)

### North America
* 2024-10-03 | Boston, MA, US | [SquiggleConf](https://2024.squiggleconf.com/)
    * [**SquiggleConf 2024: "Oxc: Pluggable Next-Gen Tooling At Rust Speed", Don Isaac**](https://2024.squiggleconf.com/schedule)
* 2024-10-03 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/)
    * [**October Social**](https://www.meetup.com/rust-montreal/events/303609215/)
* 2024-10-03 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/303768323/)
* 2024-10-03 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Iterators in Rust**](https://www.meetup.com/stl-rust/events/302371456/)
* 2024-10-04 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust pt1. Prerequisitos**](https://www.meetup.com/rust-mx/events/303480458/)
* 2024-10-05 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Davis Square Rust Lunch, Oct 5**](https://www.meetup.com/bostonrust/events/303708325/)
* 2024-10-08 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcnblb/)
* 2024-10-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638250/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**October Meetup**](https://www.meetup.com/join-srug/events/303545170/)
* 2024-10-19 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Oct 19**](https://www.meetup.com/bostonrust/events/303708335/)
* 2024-10-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 2024-10-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, Oct 27**](https://www.meetup.com/bostonrust/events/303708359/)

### Oceania
* 2024-10-29 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/303128131/)

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

> Just to provide another perspective: if you can write the programs you want to write, then all is good. You don't have to use every single tool in the standard library.
>
> I co-authored the Rust book. I have twelve years experience writing Rust code, and just over thirty years of experience writing software. I have written a macro_rules macro exactly one time, and that was 95% taking someone else's macro and modifying it. I have written one proc macro. I have used Box::leak once. I have never used Arc::downgrade. I've used Cow a handful of times.
>
> Don't stress yourself out. You're doing fine.

– [Steve Klabnik on r/rust](https://www.reddit.com/r/rust/comments/1fofg43/comment/lopwnyd/)

Thanks to [Jacob Finkelman](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1614) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
