Title: This Week in Rust 563
Number: 563
Date: 2024-09-04
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
* [Security advisory for the standard library (CVE-2024-43402)](https://blog.rust-lang.org/2024/09/04/cve-2024-43402.html)

### Newsletters
* [This Month in Rust OSDev: August 2024](https://rust-osdev.com/this-month/2024-08/)
* [This Week In Bevy - Required Components, Curves, and the Bevy CLI Working Group](https://thisweekinbevy.com/issue/2024-09-02-required-components-curves-and-the-bevy-cli-working-group)

### Project/Tooling Updates
* [FreeBSD considers Rust in the base system](https://lwn.net/Articles/985210/)
* [GCC Rust - August 2024 Monthly report](https://rust-gcc.github.io/2024/09/03/2024-08-monthly-report.html)
* [This month in Servo: tabbed browsing, Windows buffs, devtools, and more!](https://servo.org/blog/2024/08/31/this-month-in-servo/)
* [iroh 0.24.0 - Upgrading to Quinn 11](https://www.iroh.computer/blog/iroh-0-24-0-quinn-11)
* [Swiftide 0.9 - Fluvio, Lancedb and RAGAS support](https://bosun.ai/posts/swiftide-0-9/)
* [Next-gen builder macro Bon 2.1 release 🎉. Compilation is faster by 36% 🚀](https://elastio.github.io/bon/blog/bon-builder-v2-1-release)
* [Nutype 0.5.0: the newtype with guarantees supports custom errors now](https://github.com/greyblake/nutype/releases/tag/v0.5.0)
* [BackON Reaches v1](https://xuanwo.io/2024/08-backon-reaches-v1/)

### Observations/Thoughts
* [Defeating Coherence in Rust with Tacit Trait Parameters](https://willcrichton.net/notes/defeating-coherence-rust/)
* [Rust On Illumos](https://wegmueller.it/blog/posts/2024-09-02-rust-on-illumos)
* [Rust for Linux revisited](https://drewdevault.com/2024/08/30/2024-08-30-Rust-in-Linux-revisited.html)
* [Async Rust can be a pleasure to work with (without `Send + Sync + 'static`)](https://emschwartz.me/async-rust-can-be-a-pleasure-to-work-with-without-send-sync-static/)
* [Rust - A low-level echo server using io_uring](https://www.thespatula.io/rust/rust_io_uring_echo_server/)
* [Deploying Rust in Existing Firmware Codebases](https://security.googleblog.com/2024/09/deploying-rust-in-existing-firmware.html)
* [Your own little memory strategy](https://blog.morj.men/posts/rust-arena.html)
* [Code Generation with GraphQL in Rust](https://omarabid.com/rust-graphql)
* [How to deadlock Tokio application in Rust with just a single mutex](https://turso.tech/blog/how-to-deadlock-tokio-application-in-rust-with-just-a-single-mutex)
* [Is this trait sealed, or not sealed — that is the question](https://predr.ag/blog/is-this-trait-sealed-or-not-sealed/)
* [Beyond Ctrl-C: The dark corners of Unix signal handling](https://sunshowers.io/posts/beyond-ctrl-c-signals/)
* [Rust to Assembly: Understanding the Inner Workings of Rust](https://eventhelix.com/rust/)
* [K-Means Image Compression](https://www.rdiachenko.com/posts/ml/k-means-image-compression/)
* [Hey Rustaceans: Rust freelancers do exist!](https://blog.veeso.dev/blog/en/hey-rustaceans-rust-freelancers-do-exist/)
* [Why Rust needs scoped generics](http://ais523.me.uk/blog/scoped-generics.html)

### Rust Walkthroughs
* [series] [video] [Rust for Beginners in Arabic](https://youtube.com/playlist?list=PLjQ2-_RIGIVBPbV2H0ng0N4nuN7DsPkPa&si=EQrpkGhEsPCUq9Fu)
* [video] [Crafting an Interpreter in Rust #01: Basic Bytecode Chunks](https://tube.xy-space.de/w/weDkpD5BB8mAWcyUQXzxhu)
* [video] [Crafting an Interpreter in Rust #02: First Virtual Machine](https://tube.xy-space.de/w/1uSn42TmjCUpR2Ta23CgMA)
* [video] [Explore Linux TTY, process, signals w/ Rust - Part 2/3 (signal, proc, IPC egs)](https://www.youtube.com/watch?v=58_9yjLI4WA)

### Miscellaneous
* [Whither the Apple AGX graphics driver?](https://lwn.net/SubscriberLink/988438/4171601a819405c4/)
* [Counting iterations - count() vs collect().len()](https://rust.code-maven.com/counting-iterations-count-vs-collect-len)
* [audio] [Learn Rust, Train Doctors – Interview With Caroline Morton](https://timclicks.dev/podcast/learn-rust-train-doctors-interview-with-caroline-morton)
* [video] [Creating a modding system with Rust and WebAssembly [Voxel Devlog #21]](https://www.youtube.com/watch?v=fvxOI0nQsTA)
* [video] [From Zero to Async in Embedded Rust](https://www.youtube.com/watch?v=wni5h5vIPhU)

## Crate of the Week

This week's crate is [vimania-uri-rs](https://github.com/sysid/vimania-uri-rs), a VIM plugin for file and URI handling.

Thanks to [sysid](https://users.rust-lang.org/t/crate-of-the-week/2704/1334) for the self-suggestion!

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

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

416 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-08-27..2024-09-03

* [ABI compat check: detect unadjusted ABI mismatches](https://github.com/rust-lang/rust/pull/129649)
* [`rustc_target`: Add various aarch64 features](https://github.com/rust-lang/rust/pull/128192)
* [`ub_checks` intrinsics: fall back to `cfg(ub_checks)`](https://github.com/rust-lang/rust/pull/129551)
* [add `aarch64_unknown_nto_qnx700` target - QNX 7.0 support for aarch64le](https://github.com/rust-lang/rust/pull/127897)
* [add `needs-unwind` compiletest directive to `libtest-thread-limit` and replace some `Path` with `path` in `run-make`](https://github.com/rust-lang/rust/pull/129690)
* [add an ability to convert between `Span` and `visit::Location`](https://github.com/rust-lang/rust/pull/129170)
* [add missing `needs-llvm-components` directives for run-make tests that need target-specific codegen](https://github.com/rust-lang/rust/pull/129605)
* [add repr to the allowlist for naked functions](https://github.com/rust-lang/rust/pull/129421)
* [const fn stability checking: also check declared language features](https://github.com/rust-lang/rust/pull/129659)
* [const-eval: do not make UbChecks behavior depend on current crate's flags](https://github.com/rust-lang/rust/pull/129608)
* [coverage: rename `CodeRegion` to `SourceRegion`](https://github.com/rust-lang/rust/pull/129686)
* [create opaque definitions in resolver](https://github.com/rust-lang/rust/pull/129493)
* [debug-fmt-detail option](https://github.com/rust-lang/rust/pull/123940)
* [deny `wasm_c_abi` lint to nudge the last 25%](https://github.com/rust-lang/rust/pull/129534)
* [deny imports of `rustc_type_ir::inherent` outside of type ir + new trait solver](https://github.com/rust-lang/rust/pull/129678)
* [do not call `source_span` when not tracking dependencies](https://github.com/rust-lang/rust/pull/129513)
* [don't make statement nonterminals match pattern nonterminals](https://github.com/rust-lang/rust/pull/120221)
* [don't use `TyKind` in a lint](https://github.com/rust-lang/rust/pull/129527)
* [emit specific message for `time<=0.3.35`](https://github.com/rust-lang/rust/pull/129343)
* [enable Miri to pass pointers through FFI](https://github.com/rust-lang/rust/pull/129684)
* [exit: explain our expectations for the exit handlers registered in a Rust program](https://github.com/rust-lang/rust/pull/129581)
* [expand NLL MIR dumps](https://github.com/rust-lang/rust/pull/129711)
* [fix LLVM ABI NAME for riscv64imac-unknown-nuttx-elf](https://github.com/rust-lang/rust/pull/129842)
* [get rid of `predicates_defined_on`](https://github.com/rust-lang/rust/pull/129546)
* [implement a first version of RFC 3525: `struct` target features](https://github.com/rust-lang/rust/pull/127537)
* [interpret, codegen: tweak some comments and checks regarding Box with custom allocator](https://github.com/rust-lang/rust/pull/129812)
* [interpret/visitor: make memory order iteration slightly more efficient](https://github.com/rust-lang/rust/pull/129751)
* [interpret: add missing alignment check in `raw_eq`](https://github.com/rust-lang/rust/pull/129666)
* [interpret: do not make const-eval query result depend on tcx.sess](https://github.com/rust-lang/rust/pull/129613)
* [linker: synchronize native library search in rustc and linker](https://github.com/rust-lang/rust/pull/129366)
* [lint that warns when an elided lifetime ends up being a named lifetime (`elided_named_lifetimes`)](https://github.com/rust-lang/rust/pull/129207)
* [llvm-wrapper: adapt for LLVM API changes](https://github.com/rust-lang/rust/pull/129749)
* [make decoding non-optional `LazyArray` panic if not set](https://github.com/rust-lang/rust/pull/129829)
* [make it possible to enable `const_precise_live_drops` per-function](https://github.com/rust-lang/rust/pull/129507)
* [make the "detect-old-time" UI test more representative](https://github.com/rust-lang/rust/pull/129760)
* [make the const-unstable-in-stable error more clear](https://github.com/rust-lang/rust/pull/129818)
* [more `unreachable_pub`](https://github.com/rust-lang/rust/pull/129648)
* [move `'tcx` lifetime off of impl and onto methods for `CrateMetadataRef`](https://github.com/rust-lang/rust/pull/129689)
* [move the Windows `remove_dir_all` impl into a module and make it more race resistant](https://github.com/rust-lang/rust/pull/129800)
* [process.rs: remove "Basic usage" text where not useful](https://github.com/rust-lang/rust/pull/129916)
* [re-enable android tests/benches in alloc/core](https://github.com/rust-lang/rust/pull/129640)
* [refactor: standardize duplicate processes in parser](https://github.com/rust-lang/rust/pull/128641)
* [rename `BikeshedIntrinsicFrom` to `TransmuteFrom`](https://github.com/rust-lang/rust/pull/129657)
* [replace walk with visit so we dont skip outermost expr kind in def collector](https://github.com/rust-lang/rust/pull/129858)
* [rewrite `lint_expectations` in a single pass](https://github.com/rust-lang/rust/pull/127313)
* [riscv64imac: allow shadow call stack sanitizer](https://github.com/rust-lang/rust/pull/129316)
* [separate core search logic with search ui](https://github.com/rust-lang/rust/pull/126183)
* [simplify some extern providers](https://github.com/rust-lang/rust/pull/129723)
* [std: move allocators to `sys`](https://github.com/rust-lang/rust/pull/128134)
* [stop storing a special inner body for the coroutine by-move body for async closures](https://github.com/rust-lang/rust/pull/128506)
* [stop using `ty::GenericPredicates` for `non-predicates_of` queries](https://github.com/rust-lang/rust/pull/129725)
* [tweak some attributes to improve `panic_immediate_abort`](https://github.com/rust-lang/rust/pull/129589)
* [use a reduced recursion limit in the MIR inliner's cycle breaker](https://github.com/rust-lang/rust/pull/129714)
* [use equality when relating formal and expected type in arg checking](https://github.com/rust-lang/rust/pull/129317)
* [use unsafe extern blocks throughout the compiler](https://github.com/rust-lang/rust/pull/129635)
* [wasi: fix sleeping for `Duration::MAX`](https://github.com/rust-lang/rust/pull/129754)
* [miri: add tokio io test](https://github.com/rust-lang/miri/pull/3848)
* [miri: make TB tree traversal bottom-up](https://github.com/rust-lang/miri/pull/3843)
* [miri: make Tree Borrows Provenance GC compact the tree](https://github.com/rust-lang/miri/pull/3837)
* [miri: support blocking for epoll](https://github.com/rust-lang/miri/pull/3804)
* [apply size optimizations to panic machinery and some cold functions](https://github.com/rust-lang/rust/pull/129063)
* [`derive(SmartPointer)`: assume pointee from the single generic and better error messages](https://github.com/rust-lang/rust/pull/129467)
* [add `fmt::Debug` to `sync::Weak<T, A>`](https://github.com/rust-lang/rust/pull/129673)
* [add missing `read_buf` stub for `x86_64-unknown-l4re-uclibc`](https://github.com/rust-lang/rust/pull/129913)
* [allow `BufReader::peek` to be called on unsized types](https://github.com/rust-lang/rust/pull/129675)
* [core: use `compare_bytes` for more slice element types](https://github.com/rust-lang/rust/pull/128495)
* [fix `Pin::set` bounds regression](https://github.com/rust-lang/rust/pull/129668)
* [improved `checked_isqrt` and `isqrt` methods](https://github.com/rust-lang/rust/pull/128166)
* [partially stabilize `feature(new_uninit)`](https://github.com/rust-lang/rust/pull/129401)
* [hashbrown: add `HashTable::iter_hash`, `HashTable::iter_hash_mut`](https://github.com/rust-lang/hashbrown/pull/549)
* [cargo: resolve: Report incompatible-with-rustc when MSRV-resolver is disabled](https://github.com/rust-lang/cargo/pull/14459)
* [cargo: resolve: Report incompatible packages with precise Rust version](https://github.com/rust-lang/cargo/pull/14457)
* [cargo: pkgid: Allow open namespaces in PackageIdSpec's](https://github.com/rust-lang/cargo/pull/14467)
* [cargo: fix elided lifetime](https://github.com/rust-lang/cargo/pull/14487)
* [rustfmt: implement 2024 expression overflowing](https://github.com/rust-lang/rustfmt/pull/6260)
* [clippy: extend `implicit_saturating_sub` lint](https://github.com/rust-lang/rust-clippy/pull/12476)
* [clippy: new lint: `zombie_processes`](https://github.com/rust-lang/rust-clippy/pull/11476)
* [clippy: remove `feature=cargo-clippy` argument](https://github.com/rust-lang/rust-clippy/pull/13246)
* [rust-analyzer: extra sugar auto-completion `async fn ...` in `impl trait` for `async fn in trait` that's defined in desugar form](https://github.com/rust-lang/rust-analyzer/pull/17737)
* [rust-analyzer: fix handling of `for` in `impl T for A` in function body](https://github.com/rust-lang/rust-analyzer/pull/18005)
* [rust-analyzer: add explicit `enum` discriminant assist](https://github.com/rust-lang/rust-analyzer/pull/17985)
* [rust-analyzer: do not report missing unsafe on `addr_of[_mut]!(EXTERN_OR_MUT_STATIC)`](https://github.com/rust-lang/rust-analyzer/pull/18003)
* [rust-analyzer: create an assist to convert closure to freestanding fn](https://github.com/rust-lang/rust-analyzer/pull/17940)
* [rust-analyzer: implement cast typecheck and diagnostics](https://github.com/rust-lang/rust-analyzer/pull/17984)
* [rust-analyzer: implement object safety and its hovering hint](https://github.com/rust-lang/rust-analyzer/pull/17814)
* [rust-analyzer: suggest name in completion for `let_stmt` and `fn_param`](https://github.com/rust-lang/rust-analyzer/pull/18031)
* [rust-analyzer: support fn-ptr and fn-path types for lifetime elision hints](https://github.com/rust-lang/rust-analyzer/pull/18010)
* [rust-analyzer: fix incorrect symbol definitions in SCIP output](https://github.com/rust-lang/rust-analyzer/pull/17988)
* [rust-analyzer: `std::error::Error` is object unsafe](https://github.com/rust-lang/rust-analyzer/pull/17999)
* [rust-analyzer: consider field attributes when converting from tuple to named `struct` and the opposite](https://github.com/rust-lang/rust-analyzer/pull/17993)
* [rust-analyzer: consider indentation in the "Generate impl" and "Generate trait impl" assists](https://github.com/rust-lang/rust-analyzer/pull/17982)
* [rust-analyzer: don't add reference when it isn't needed for the "Extract variable" assist](https://github.com/rust-lang/rust-analyzer/pull/17991)
* [rust-analyzer: fix `TokenStream::to_string` implementation dropping quotation marks](https://github.com/rust-lang/rust-analyzer/pull/17994)
* [rust-analyzer: fix lifetime elision inlay hints breaking for ranged requests](https://github.com/rust-lang/rust-analyzer/pull/18012)
* [rust-analyzer: fix name resolution of shadowed builtin macro](https://github.com/rust-lang/rust-analyzer/pull/17987)
* [rust-analyzer: handle attributes correctly in "Flip comma"](https://github.com/rust-lang/rust-analyzer/pull/18015)
* [rust-analyzer: lifetime hint panic in non generic defs](https://github.com/rust-lang/rust-analyzer/pull/18028)
* [rust-analyzer: use Result type aliases in "Wrap return type in Result" assist](https://github.com/rust-lang/rust-analyzer/pull/18016)
* [rust-analyzer: provide an option to hide deprecated items from completion](https://github.com/rust-lang/rust-analyzer/pull/18006)
* [rust-analyzer: recategorize config classes](https://github.com/rust-lang/rust-analyzer/pull/17945)

### Rust Compiler Performance Triage

This week we had some trouble with our performance bot, but luckily the issue has been resolved.
In the end, we saw much more improvements than regressions.

Triage done by **@kobzol**.
Revision range: [acb4e8b6..6199b69c](https://perf.rust-lang.org/?start=acb4e8b6251f1d8da36f08e7a70fa23fc581839e&end=6199b69c53a8c275ca3cd59647ea0af5ca29aae2&absolute=false&stat=instructions%3Au)

**Summary**:

|         (instructions:u)          | mean  |     range      | count |
|:---------------------------------:|:-----:|:--------------:|:-----:|
|  Regressions ❌ <br /> (primary)   | 0.3%  |  [0.2%, 0.4%]  |   8   |
| Regressions ❌ <br /> (secondary)  | 0.7%  |  [0.2%, 1.5%]  |   9   |
|  Improvements ✅ <br /> (primary)  | -0.8% | [-3.4%, -0.2%] |  158  |
| Improvements ✅ <br /> (secondary) | -0.7% | [-2.3%, -0.2%] |  96   |
|         All ❌✅ (primary)          | -0.7% | [-3.4%, 0.4%]  |  166  |


2 Regressions, 3 Improvements, 1 Mixed; 3 of them in rollups
19 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/ee7cd7e680ebf9d588ad23cc0f1ba32b3caf1813/triage/2024-09-03.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add "crates.io: Crate Deletions" RFC](https://github.com/rust-lang/rfcs/pull/3660)
* [Merge RFC 3529: Add named path bases to cargo](https://github.com/rust-lang/rfcs/pull/3529)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Update `catch_unwind` doc comments for `c_unwind`](https://github.com/rust-lang/rust/pull/128321)
* [disposition: merge] [stabilize `const_extern_fn`](https://github.com/rust-lang/rust/pull/129753)
* [disposition: merge] [stabilize const_float_bits_conv](https://github.com/rust-lang/rust/pull/129555)
* [disposition: merge] [Make destructors on `extern "C"` frames to be executed](https://github.com/rust-lang/rust/pull/129582)
* [disposition: merge] [Don't warn empty branches unreachable for now](https://github.com/rust-lang/rust/pull/129103)
* [disposition: merge] [Tracking Issue for `char::MIN`](https://github.com/rust-lang/rust/issues/114298)
* [disposition: merge] [Tracking issue for `#![feature(entry_insert)]`](https://github.com/rust-lang/rust/issues/65225)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [[RFC] code name support](https://github.com/rust-lang/rfcs/pull/3688)
* [new] [Generic Integers V2: It's Time](https://github.com/rust-lang/rfcs/pull/3686)
* [new] [Simplify lightweight clones, including into closures and async blocks](https://github.com/rust-lang/rfcs/pull/3680)

## Upcoming Events

Rusty Events between 2024-09-04 - 2024-10-02 🦀

### Virtual
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Typestate Pattern in Rust: With a Strict Builder Example**](https://www.meetup.com/indyrs/events/300328029/)
* 2024-09-05 | Virtual (Buenos Aires, AR) | [LambdaClass](https://lu.ma/user/usr-dkk9KnFvsvZEb7k)
    * [**Meetup Rust Septiembre [Spanish]**](https://lu.ma/uh1qpox0)
* 2024-09-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897957/)
* 2024-09-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820268/)
* 2024-09-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346981/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-12 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633268/)
* 2024-09-12 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #6**](https://www.meetup.com/bevy-game-development/events/302841892/)
* 2024-09-16 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302827971/)
* 2024-09-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346969/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-18 - 2024-09-20 | Hybrid - Virtual and In-Person (Vienna, AT) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024**](https://lpc.events/event/18/sessions/186/)
* 2024-09-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897973/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585670/)
* 2024-09-26 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rusty secure communication on embedded devices**](https://www.meetup.com/charlottesville-rust-meetup/events/303159380/)
* 2024-10-02 | Virtual (Vancouver, BC, CA) | [Vancouver Postgres](https://www.meetup.com/vancouver-postgres/)
    * [**Leveraging a PL/RUST extension to protect sensitive data in PostgreSQL**](https://www.meetup.com/vancouver-postgres/events/302160672/)

### Africa
* 2024-09-06 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587/)

### Asia
* 2024-09-07 - 2024-09-08 | Shanghai, CN | [Rust China](https://rustcc.cn/)
    * [**Rust China Conf: Shanghai**](https://rustcc.cn/2024conf/)
* 2024-09-09 | Ramat Gan, IL | [Coralogix](https://coralogix.com/)
    * [**Rust as Scale**](https://coralogix.com/rust-coralogix-meetup/)
* 2024-09-14 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**September 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)

### Europe
* 2024-09-04 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**More Rust - Generics, constraints, safety.**](https://www.meetup.com/oxford-rust-meetup-group/events/302848276/)
* 2024-09-11 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302833564/)
* 2024-09-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425049/)
* 2024-09-17 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Making AI-models perform tasks, in Rust!**](https://www.meetup.com/rust-trondheim/events/302957040/)
* 2024-09-18 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Rust Moravia Meetup (September 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 2024-09-18 | Vienna, AT + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024 (Sep 18-20)**](https://lpc.events/event/18/sessions/186/)
* 2024-09-21 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #5**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)
* 2024-09-24 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust)
    * [**Rust meetup #70**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-26 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night**](https://www.meetup.com/rust-aarhus/events/301522991/)
* 2024-09-27 | Mannheim, DE | [Hackerstolz e.V.](https://www.hackerstolz.de/en/)
    * [**Hackerstolz Stammtisch Rhein-Neckar**](https://www.hackerstolz.de/en/)
* 2024-10-02 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/Stockholm-Rust/events/303213095)

### North America
* 2024-09-05 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Shooting Stars: Create a game from scratch in 25 minutes!**](https://www.meetup.com/utah-rust/events/303204006/)
* 2024-09-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302723843/)
* 2024-09-05 | Portland, OR, US | [PDX Rust](https://www.meetup.com/pdxrust/)
    * [**PDX Rust September!**](https://www.meetup.com/pdxrust/events/302588479/)
* 2024-09-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/hdzdmtygcmbhb/)
* 2024-09-07 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976293/)
* 2024-09-08 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/302498734/)
    * [**Northeastern Rust Lunch, Sep 8**](https://www.meetup.com/bostonrust/events/302498706/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Boulder Elixir Meetup**](hhttps://www.meetup.com/boulder-elixir/events/302991078/)
* 2024-09-16 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Somerville Union Square Rust Lunch, Sep 16**](https://www.meetup.com/bostonrust/events/302498750/)
* 2024-09-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638248/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-21 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976355/)
* 2024-09-24 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/zfcbntygcmbgc/)
* 2024-09-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/302274449/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ecdzp2/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I'm pretty sure I'm the only person ever to single handedly write a complex GPU kernel driver that has never had a memory safety kernel panic bug (itself) in production, running on thousands of users' systems for 1.5 years now.
>
> Because I wrote it in Rust.

– [Asahi Lina on vt.social](https://vt.social/@lina/113045456734886438)

Thanks to [Ludwig Stecher](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1604) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1f9ezq8/this_week_in_rust_563/)</small>
