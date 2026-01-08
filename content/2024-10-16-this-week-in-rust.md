Title: This Week in Rust 569
Number: 569
Date: 2024-10-16
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

### Foundation
* [Announcing the Rust Foundation’s 2024 Fellows](https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-2024-fellows/)

### Newsletters
* [This Week In Bevy - Animation Events, Curves, and no_std](https://thisweekinbevy.com/issue/2024-10-14-animation-events-curves-and-nostd)

### Project/Tooling Updates
* [On Rust in enterprise kernels](https://lwn.net/SubscriberLink/993337/68c7e5af573545e6/)
* [FFI type mismatches in Rust for Linux](https://lwn.net/SubscriberLink/993163/3c2b06af07814cd2/)
* [Zapping pointers out of thin air](https://lwn.net/SubscriberLink/993484/385b176aa8939e7b/)
* [Using LKMM atomics in Rust](https://lwn.net/SubscriberLink/993785/cbe8cf5846d6864a/)
* ["pigg" (the Raspberry Pi GPIO GUI) 0.4.0 released](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.4.0)

### Observations/Thoughts
* [Why `Pin` is a part of trait signatures (and why that's a problem)](https://blog.yoshuawuyts.com/why-pin/)
* [The `Overwrite` trait and `Pin`](https://smallcultfollowing.com/babysteps/blog/2024/10/14/overwrite-and-pin)
* [Improve an algorithm performance step by step](https://blog.mapotofu.org/blogs/rabitq-bench/)
* [Replacing nginx with axum](https://felix-knorr.net/posts/2024-10-13-replacing-nginx-with-axum.html)
* [An experiment in async Rust](https://ochagavia.nl/blog/an-experiment-in-async-rust/)
* [Designing A Fast Concurrent Hash Table](https://ibraheem.ca/posts/designing-papaya/)
* [Rethinking Builders… with Lazy Generics](https://geo-ant.github.io/blog/2024/rust-rethinking-builders-lazy-generics/)
* [IPC in Rust](https://pranitha.dev/posts/rust-ipc-ping-pong/)
* [Serde Trait - Part 3: Deserialization](https://voelklmichael.github.io/Blog/serde-trait-part3.html)
* [Memory for Nothing: Why Vec&lt;usize&gt; is (probably) a bad idea](https://pwy.io/posts/memory-for-nothing/)
* [Upgrade the Logging in your Rust Tests](https://tylerjw.dev/posts/20241012-rust-logging-in-tests/)
* [Nine Rules for Running Rust on Embedded Systems](https://medium.com/towards-data-science/nine-rules-for-running-rust-on-embedded-systems-b0c247ee877e)
* [Why Rust is taking the data engineering world by storm](https://kerkour.com/rust-data-engineering)

### Rust Walkthroughs
* [Reading a Remote File Using Rust](https://crustc.com/reading-a-remote-file-rust/)
* [video] [Build with Naz : Rust memory address and size](https://www.youtube.com/watch?v=ivqIty5EOf8)

## Crate of the Week

This week's crate is [bacon](https://dystroy.org/bacon), a terminal application to run your cargo tasks on change in background.

Thanks to [Denys Séguret](https://users.rust-lang.org/t/crate-of-the-week/2704/1351) for the self-suggestion!
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

*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

468 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-10-08..2024-10-15

* [reserve guarded string literals](https://github.com/rust-lang/rust/pull/123951) (RFC [#3593](https://rust-lang.github.io/rfcs/3593-unprefixed-guarded-strings.html))
* [ABI: pass aggregates by value on AIX](https://github.com/rust-lang/rust/pull/131208)
* [`codegen_ssa`: consolidate tied target checks](https://github.com/rust-lang/rust/pull/130308)
* [`rustc_target`: Add sme-b16b16 as an explicit aarch64 target feature](https://github.com/rust-lang/rust/pull/130741)
* [add `&pin (mut|const) T` type position sugar](https://github.com/rust-lang/rust/pull/130635)
* [add suggestion for removing invalid path sep `:`: in fn def](https://github.com/rust-lang/rust/pull/130870)
* [also use outermost const-anon for impl items in `non_local_defs` lint](https://github.com/rust-lang/rust/pull/131660)
* [check ABI target compatibility for function pointers](https://github.com/rust-lang/rust/pull/128784)
* [don't assume traits used as type are trait objs in 2021 edition](https://github.com/rust-lang/rust/pull/131239)
* [dont ICE when encountering post-mono layout cycle error](https://github.com/rust-lang/rust/pull/131420)
* [emit an error for unstable attributes that reference already stable features](https://github.com/rust-lang/rust/pull/131567)
* [fix `clobber_abi` and disallow SVE-related registers in Arm64EC inline assembly](https://github.com/rust-lang/rust/pull/131332)
* [improve error messages for `C-cmse-nonsecure-entry` functions](https://github.com/rust-lang/rust/pull/130747)
* [introduce SolverRelating type relation to the new solver](https://github.com/rust-lang/rust/pull/131263)
* [make `unused_parens's` suggestion considering expr's attributes](https://github.com/rust-lang/rust/pull/131546)
* [move dummy commit logic into `x86_64-gnu-llvm-18`](https://github.com/rust-lang/rust/pull/131531)
* [move polarity into `PolyTraitRef` rather than storing it on the side](https://github.com/rust-lang/rust/pull/131652)
* [precise capturing in traits](https://github.com/rust-lang/rust/pull/131033)
* [shallowly match opaque key in storage](https://github.com/rust-lang/rust/pull/131599)
* [support `clobber_abi` in MSP430 inline assembly](https://github.com/rust-lang/rust/pull/131310)
* [suppress import errors for traits that couldve applied for method lookup error](https://github.com/rust-lang/rust/pull/131702)
* [use throw intrinsic from stdarch in wasm libunwind](https://github.com/rust-lang/rust/pull/131418)
* [miri: do not store synchronization primitive IDs in adressable memory](https://github.com/rust-lang/miri/pull/3966)
* [miri: ensure that a macOS `os_unfair_lock` that is moved while being held is not implicitly unlocked](https://github.com/rust-lang/miri/pull/3973)
* [miri: epoll event adding: no need to join, there's no old clock here](https://github.com/rust-lang/miri/pull/3961)
* [miri: fix behavior of `release_clock()`](https://github.com/rust-lang/miri/pull/3951)
* [miri: fix over synchronization of epoll](https://github.com/rust-lang/miri/pull/3946)
* [miri: fixed `pthread_getname_np` impl for glibc](https://github.com/rust-lang/miri/pull/3953)
* [miri: fixed get/set thread name implementations for macOS and FreeBSD](https://github.com/rust-lang/miri/pull/3957)
* [miri: use new `check_min_arg_count` helper in more places](https://github.com/rust-lang/miri/pull/3974)
* [impossible obligations fast path](https://github.com/rust-lang/rust/pull/131491)
* [remove unnecessary sorts in `rustc_hir_analysis`](https://github.com/rust-lang/rust/pull/131328)
* [stabilise `const_char_encode_utf8`](https://github.com/rust-lang/rust/pull/131463)
* [stabilise `const_make_ascii`](https://github.com/rust-lang/rust/pull/131496)
* [stabilize `Pin::as_deref_mut()`](https://github.com/rust-lang/rust/pull/129424)
* [stabilize `ci_rustc_if_unchanged_logic` test](https://github.com/rust-lang/rust/pull/131444)
* [stabilize `const_option`](https://github.com/rust-lang/rust/pull/131120)
* [stabilize `const_result`](https://github.com/rust-lang/rust/pull/131287)
* [stabilize `debug_more_non_exhaustive`](https://github.com/rust-lang/rust/pull/131109)
* [stabilize `duration_consts_float`](https://github.com/rust-lang/rust/pull/131289)
* [stabilize const `ptr::write*` and `mem::replace`](https://github.com/rust-lang/rust/pull/130954)
* [stabilize const `{slice,array}::from_mut`](https://github.com/rust-lang/rust/pull/130538)
* [optimize `escape_ascii` using a lookup table](https://github.com/rust-lang/rust/pull/125679)
* [migrate lib's `&Option<T>` into `Option<&T>`](https://github.com/rust-lang/rust/pull/130962)
* [intrinsics fmuladdf{32,64}: expose llvm.fmuladd.* semantics](https://github.com/rust-lang/rust/pull/124874)
* [add `#[track_caller]` to allocating methods of `Vec` & `VecDeque`](https://github.com/rust-lang/rust/pull/126557)
* [core/net: add `Ipv[46]Addr::from_octets, Ipv6Addr::from_segments`](https://github.com/rust-lang/rust/pull/130629)
* [decouple WASIp2 sockets from WasiFd](https://github.com/rust-lang/rust/pull/131449)
* [fix bug where `option_env!` would return `None` when env var is present but not valid Unicode](https://github.com/rust-lang/rust/pull/122670)
* [implemented `FromStr` for `CString` and `TryFrom<CString>` for `String`](https://github.com/rust-lang/rust/pull/130608)
* [library: const-stabilize `MaybeUninit::assume_init_mut`](https://github.com/rust-lang/rust/pull/131274)
* [std: fix stdout-before-main](https://github.com/rust-lang/rust/pull/131233)
* [hashbrown: ci: test the MSRV with minimal dependency versions](https://github.com/rust-lang/hashbrown/pull/572)
* [hashbrown: revert "feat: borsh serde"](https://github.com/rust-lang/hashbrown/pull/570)
* [cargo: complete: Don't complete files for any value](https://github.com/rust-lang/cargo/pull/14653)
* [cargo: git: dont fetch tags by default](https://github.com/rust-lang/cargo/pull/14688)
* [cargo: resolver: share conflict cache between activation retries](https://github.com/rust-lang/cargo/pull/14692)
* [cargo: add more SAT resolver tests](https://github.com/rust-lang/cargo/pull/14614)
* [cargo: docs: tools should only interpret a line starting with `{` as JSON](https://github.com/rust-lang/cargo/pull/14677)
* [cargo: feat: add custom completer for completing registry name](https://github.com/rust-lang/cargo/pull/14656)
* [cargo: fix panic when running cargo tree on a package with a cross compiled bindep](https://github.com/rust-lang/cargo/pull/14593)
* [cargo: fix: avoid inserting duplicate `dylib_path_envvar` when calling `cargo run` recursively](https://github.com/rust-lang/cargo/pull/14464)
* [cargo: improve resolver speed](https://github.com/rust-lang/cargo/pull/14663)
* [cargo: initial version of checksum based freshness](https://github.com/rust-lang/cargo/pull/14137)
* [cargo: remove the support for `Cargo.toml` of the cargo-script](https://github.com/rust-lang/cargo/pull/14670)
* [cargo: support package selection options like `--exclude` in `cargo publish`](https://github.com/rust-lang/cargo/pull/14659)
* [rustdoc-json: change item ID's repr from a string to an int](https://github.com/rust-lang/rust/pull/130078)
* [rustdoc: add space between `struct` fields and their descriptions](https://github.com/rust-lang/rust/pull/131394)
* [rustfmt: hint which `style_edition` to use instead of version](https://github.com/rust-lang/rustfmt/pull/6361)
* [clippy: `implicit_saturating_sub`: fix suggestion with a less volatile approach](https://github.com/rust-lang/rust-clippy/pull/13542)
* [clippy: `module_name_repetitions`: don't warn if the item is in a private module](https://github.com/rust-lang/rust-clippy/pull/13444)
* [clippy: add `manual_ignore_cast_cmp` lint](https://github.com/rust-lang/rust-clippy/pull/13334)
* [clippy: check `MethodCall`/`Call` arg count earlier or at all](https://github.com/rust-lang/rust-clippy/pull/13540)
* [clippy: check for needless raw strings in `format_args!()` template as well](https://github.com/rust-lang/rust-clippy/pull/13504)
* [clippy: don't warn on proc macro generated code in `needless_return`](https://github.com/rust-lang/rust-clippy/pull/13464)
* [clippy: fix `large_stack_arrays` triggering when nesting const items](https://github.com/rust-lang/rust-clippy/pull/13534)
* [clippy: fix lint `manual_slice_size_calculation` when a slice is ref more than once](https://github.com/rust-lang/rust-clippy/pull/13487)
* [clippy: fix span issue on `implicit_saturating_sub`](https://github.com/rust-lang/rust-clippy/pull/13533)
* [clippy: mark `unnecessary_first_then_check` and `byte_char_slices` as `Applicable`](https://github.com/rust-lang/rust-clippy/pull/13537)
* [clippy: move `clippy::module_name_repetitions` to `restriction` (from `pedantic`)](https://github.com/rust-lang/rust-clippy/pull/13541)
* [clippy: move `too_long_first_doc_paragraph` to `nursery`](https://github.com/rust-lang/rust-clippy/pull/13551)
* [clippy: only emit `manual_c_str_literals` in ≥ Edition 2021](https://github.com/rust-lang/rust-clippy/pull/13532)
* [clippy: use correct std/core prefix in lint output](https://github.com/rust-lang/rust-clippy/pull/13454)
* [rust-analyzer: add support for LLDB-DAP](https://github.com/rust-lang/rust-analyzer/pull/18265)
* [rust-analyzer: correctly parse `use` in generic parameters](https://github.com/rust-lang/rust-analyzer/pull/18226)
* [rust-analyzer: do not consider match/let/ref of place that evaluates to ! to diverge, disallow coercions from them too](https://github.com/rust-lang/rust-analyzer/pull/18278)
* [rust-analyzer: handle self-param outside of methods when renaming](https://github.com/rust-lang/rust-analyzer/pull/18292)
* [rust-analyzer: highlight exit points of async blocks](https://github.com/rust-lang/rust-analyzer/pull/18152)
* [rust-analyzer: respect `references.exclude_tests` in call-hierarchy](https://github.com/rust-lang/rust-analyzer/pull/18291)
* [rust-analyzer: fix panic when json project has relative buildfile paths](https://github.com/rust-lang/rust-analyzer/pull/18289)
* [rust-analyzer: comment out cast checks for unknown ptr kind](https://github.com/rust-lang/rust-analyzer/pull/18217)
* [rust-analyzer: do not consider mutable usage of deref to `*mut T` as `deref_mut`](https://github.com/rust-lang/rust-analyzer/pull/18252)
* [rust-analyzer: fix `prettify_macro_expansion()` when the node isn't the whole file](https://github.com/rust-lang/rust-analyzer/pull/18246)
* [rust-analyzer: include description in label details when detail field is marked for …](https://github.com/rust-lang/rust-analyzer/pull/18245)
* [rust-analyzer: incorrect autofix for missing wrapped unit in return expr](https://github.com/rust-lang/rust-analyzer/pull/18299)
* [rust-analyzer: join rustfmt overrideCommand with project root](https://github.com/rust-lang/rust-analyzer/pull/18229)
* [rust-analyzer: hir-ty: change `struct` + `enum` variant constructor formatting](https://github.com/rust-lang/rust-analyzer/pull/18269)
* [rust-analyzer: lsp: fix `completion_item something_to_resolve` not being a latch to true](https://github.com/rust-lang/rust-analyzer/pull/18247)
* [rust-analyzer: run subprocesses async in vscode extension](https://github.com/rust-lang/rust-analyzer/pull/18281)
* [rust-analyzer: skip `#[test_case]` expansion](https://github.com/rust-lang/rust-analyzer/pull/18275)

### Rust Compiler Performance Triage

No major changes this week.

Triage done by **@simulacrum**.
Revision range: [e6c46db4..5ceb623a](https://perf.rust-lang.org/?start=e6c46db4e9fd11e3183c397a59d946731034ede6&end=5ceb623a4abd66e91e7959d25caaf0523f1a7f7c&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-10-14.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [stabilize Strict Provenance and Exposed Provenance APIs](https://github.com/rust-lang/rust/pull/130350)
* [disposition: merge] [make unsupported_calling_conventions a hard error](https://github.com/rust-lang/rust/pull/129935)
* [disposition: merge] [Decide on name for `derive(SmartPtr)`](https://github.com/rust-lang/rust/issues/129104)
* [disposition: merge] [Stabilize `shorter_tail_lifetime`](https://github.com/rust-lang/rust/issues/131445)
* [disposition: merge] [Finish stabilization of `result_ffi_guarantees`](https://github.com/rust-lang/rust/pull/130628)
* [disposition: merge] [Implement edition 2024 match ergonomics restrictions](https://github.com/rust-lang/rust/pull/131381)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: No (opsem) Magic Boxes](https://github.com/rust-lang/rfcs/pull/3712)
* [new] [`#[inline(required)]`](https://github.com/rust-lang/rfcs/pull/3711)
* [new] [RFC: introduce the flavor syntactic design pattern](https://github.com/rust-lang/rfcs/pull/3710)

## Upcoming Events

Rusty Events between 2024-10-16 - 2024-11-13 🦀

### Virtual
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 3 of 4 - Hackathon Ideation Lab**](https://www.meetup.com/women-in-rust/events/303213817/)
* 2024-10-17| Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898023)
* 2024-10-17 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/303987275/)
* 2024-10-22 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcnbdc/)
* 2024-10-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 4 of 4 - Hackathon Showcase: Final Projects and Presentations**](https://www.meetup.com/women-in-rust/events/303213835/)
* 2024-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633271/)
* 2024-10-26 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 2024-10-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585671/)
* 2024-10-31| Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898048/)
* 2022-10-31 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)
* 2024-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633272/)
* 2024-11-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346985/)


### Africa
* 2024-11-02 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 2024-10-17 - 2024-10-18 | Beijing, CN | [Global Open-Source Innovation Meetup (GOSIM)](https://www.gosim.org/)
    * [**GOSIM 2024**](https://china2024.gosim.org/)
* 2024-10-19 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**October 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/october-2024-rustacean-meetup/)

### Europe
* 2024-10-16 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks October - Leptos and Crux**](https://www.meetup.com/rust-manchester/events/303658240/)
* 2024-10-17 | Barcelona, ES | [BcnRust](https://bcnrust.github.io)
    * [**16th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/303792888/)
* 2024-10-17 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #3**](https://www.meetup.com/rust-bern/events/303617330/)
* 2024-10-17 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #4**](https://www.meetup.com/rust-gdansk/events/303370455/)
* 2024-10-22 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust London x London Python: *SOLD OUT***](https://www.meetup.com/rust-london-user-group/events/303922750/)
* 2024-10-22 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**New Rust Warsaw Meetup #2**](https://www.meetup.com/rust-warsaw/events/303619536/)
* 2024-10-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #6**](https://www.meetup.com/stockholm-rust/events/303918943/)
* 2024-10-28 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 2024-10-29 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/303479865)
* 2024-10-30 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn October 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 2024-10-31 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/300820289/)
* 2024-11-06 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)

### North America
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night/Happy Hour**](https://www.meetup.com/deep-dish-rust/events/303919296/)
* 2024-10-17 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**October Meetup**](https://www.meetup.com/join-srug/events/303545170/)
* 2024-10-19 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Oct 19**](https://www.meetup.com/bostonrust/events/303708335/)
* 2024-10-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 2024-10-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, Oct 27**](https://www.meetup.com/bostonrust/events/303708359/)
* 2024-10-28 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Generics from the Ground Up**](https://www.meetup.com/boulder-rust-meetup/events/303766925/)
* 2024-10-28 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/303909299/)
* 2024-10-28 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/303884468/)
* 2024-10-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : State of the Group and Expectations for 2025**](https://www.meetup.com/music-city-rust-developers/events/301425524/)
* 2024-11-04 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Nov 4**](https://www.meetup.com/bostonrust/events/303708387/)
* 2024-11-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031651/)
* 2024-11-07 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Game development with Rust and the Bevy engine**](https://www.meetup.com/stl-rust/events/302371464/)
* 2024-11-12 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)

### Oceania
* 2024-10-28 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**October 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/304034898/)
* 2024-10-29 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/303128131/)
* 2024-10-31 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Rust on AWS: Sustainability + Peace: Zero Stress Automation**](https://www.meetup.com/rust-akl/events/303824919/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1fa0tf6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> We'd have buttons on the screen to control the fans of the car. I had to write a lot of code before I could compile it all, a big jenga tower. But once it compiled, the fans started to work! Very impressed.

– [Julius Gustavsson on the Tweedegolf blog](https://tweedegolf.nl/en/blog/137/rust-is-rolling-off-the-volvo-assembly-line)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1619) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1g5iqq8/this_week_in_rust_569/)</small>
