Title: This Week in Rust 559
Number: 559
Date: 2024-08-07
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

### Newsletters
* [thisweekinbevy - 0.14.1, tracking change detection, and more rendering examples](https://thisweekinbevy.com/issue/2024-08-05-0141-tracking-change-detection-and-more-rendering-examples)

### Project/Tooling Updates
* [Tauri 2.0 Release Candidate](https://v2.tauri.app/blog/tauri-2-0-0-release-candidate/)
* [CGlue 0.3 Future and Beyond](https://blaz.is/blog/post/cglue-0-3/)
* [ratatui - v0.28.0](https://ratatui.rs/highlights/v028/)
* [Pigg 0.3.3](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.3.3) the GUI for RPi GPIO interaction released, with Remote GPIO feature!
* [Announcing SeaORM 1.0](https://www.sea-ql.org/blog/2024-08-04-sea-orm-1.0/)
* [Danube - Queuing and Pub/Sub message patterns](https://dev-state.com/posts/danube_pubsub/)

### Observations/Thoughts
* [Trying and mostly failing to optimize frustum culling in a WebGL + TS + Rust engine](https://blog.paavo.me/demo-engine-part-1/)
* [Panic! At The Async Runtime Shutdown](https://www.mattkeeter.com/blog/2024-08-01-panic/)
* [Debugging a rustc segfault on illumos](https://sunshowers.io/posts/rustc-segfault-illumos/)
* [Tracing my way with tracing-rs](https://j.njsm.de/blog/tracing-rs/)
* [Series] [The Hitchhiker’s Guide to Building a Distributed Filesystem in Rust.](https://systemweakness.com/hitchhikers-guide-to-building-a-distributed-filesystem-in-rust-the-very-beginning-2c02eb7313e7)
* [Best Rust books for 2024](https://bitfieldconsulting.com/posts/best-rust-books)
* [Phantom Menace: memory leak that wasn't there](https://flakm.com/posts/phantom_leak/)
* [Developing a cryptographically secure bootloader for RISC-V in Rust](https://www.codethink.co.uk/articles/2024/secure_bootloader/)
* [Extending the #[diagnostic] tool attribute namespace](https://blog.weiznich.de/blog/diagnostic-namespace-do-not-recommend/)

### Rust Walkthroughs
* [Tracing Tokio Resources](https://hegdenu.net/posts/tracing-tokio-resources/)
* [Series] [Mastering Dependency Injection in Rust: Crafting a Custom Container](https://chesedo.me/blog/manual-dependency-injection-rust/)

### Research
* [The Hitchhiker’s Guide to Building a Distributed Filesystem in Rust.](https://medium.com/@xorio42/the-hitchhikers-guide-to-building-a-distributed-filesystem-in-rust-the-continuation-part-2-ad1fd8bc9cf8) 

### Miscellaneous
* [Rustic: Enhanced Org Babel integration](https://psibi.in/posts/2024-08-04-rustic-babel.html)
* [Efficient Logging - Speeding up production code by logging more efficiently](https://antoniosbarotsis.github.io/posts/efficient-logging/)

## Crate of the Week

This week's crate is [WhenFS](https://github.com/lvkv/whenfs), a FUSE filesystem that misuses your google calendar as storage. And yes, your schedule will look as packed as mine once you store one or two files in there.

Despite yet another week fully devoid of suggestions nor votes, llogiq is reasonably pleased with his choice.

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

* [rencfs - Abstract file access layer](https://github.com/radumarias/rencfs/issues/111)
* [rencfs - Add RustCrypto as a feature](https://github.com/radumarias/rencfs/issues/104)
* [rencfs - File and fs API](https://github.com/radumarias/rencfs/issues/97)
* [rencfs - io API](https://github.com/radumarias/rencfs/issues/90)
* [rfs - Coordinator node API](https://github.com/radumarias/rfs/issues/19))
* [rfs - Data node API](https://github.com/radumarias/rfs/issues/20)
* [rfs - File upload and changes](https://github.com/radumarias/rfs/issues/18)
* [rfs - Communication between Coordinator and Data nodes](https://github.com/radumarias/rfs/issues/15)
* [syncoxiders - Two-way sync](https://github.com/radumarias/syncoxiders/issues/10)
* [syncoxiders - Sync chunks in parallel](https://github.com/radumarias/syncoxiders/issues/9)
* [syncoxiders - Integrate SurrealDB to store metadata](https://github.com/radumarias/syncoxiders/issues/13)
* [syncoxiders - Migrate scripts tests to integration tests](https://github.com/radumarias/syncoxiders/issues/6)
* [rencfs-desktop - Implement daemon](https://github.com/radumarias/rencfs-desktop/issues/4)
* [Proposal: Deprecate Tokio's LocalSet](https://www.github.com/tokio-rs/tokio/issues/6741)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

381 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-07-30..2024-08-06

* [fix vita build of std and forbid unsafe in unsafe in the os/vita module](https://github.com/rust-lang/rust/pull/128315)
* [`derive(SmartPointer)`: require pointee to be maybe sized](https://github.com/rust-lang/rust/pull/128452)
* [add `#[must_use]` to some `into_raw*` functions](https://github.com/rust-lang/rust/pull/127586)
* [add `REDUNDANT_IMPORTS` lint for new redundant import detection](https://github.com/rust-lang/rust/pull/123813)
* [add `f16` and `f128` math functions](https://github.com/rust-lang/rust/pull/127027)
* [allow overwriting the output of `rustc --version`](https://github.com/rust-lang/rust/pull/124339)
* [allow setting `link-shared` and `static-libstdcpp` with CI LLVM](https://github.com/rust-lang/rust/pull/128589)
* [android: remove libstd hacks for unsupported Android APIs](https://github.com/rust-lang/rust/pull/128416)
* [assert that all attributes are actually checked via `CheckAttrVisitor` and aren't accidentally usable on completely unrelated HIR nodes](https://github.com/rust-lang/rust/pull/128581)
* [better handle suggestions for the already present code and fix some suggestions](https://github.com/rust-lang/rust/pull/126818)
* [built-in derive: remove `BYTE_SLICE_IN_PACKED_STRUCT_WITH_DERIVE` hack and lint](https://github.com/rust-lang/rust/pull/127907)
* [cleanup sys module to match house style](https://github.com/rust-lang/rust/pull/128162)
* [create COFF archives for non-LLVM backends](https://github.com/rust-lang/rust/pull/128450)
* [custom MIR: add support for tail calls](https://github.com/rust-lang/rust/pull/128688)
* [delegation: second attempt to improve perf](https://github.com/rust-lang/rust/pull/128441)
* [delegation: support generics for delegation from free functions](https://github.com/rust-lang/rust/pull/125929)
* [detect non-lifetime binder params shadowing item params](https://github.com/rust-lang/rust/pull/128357)
* [do not fire unhandled attribute assertion on multi-segment `AttributeType::Normal` attributes with builtin attribute as first segment](https://github.com/rust-lang/rust/pull/128623)
* [don't re-elaborate already elaborated caller bounds in method probe](https://github.com/rust-lang/rust/pull/128559)
* [elaborate unknowable goals](https://github.com/rust-lang/rust/pull/127574)
* [emit an error if `#[optimize]` is applied to an incompatible item](https://github.com/rust-lang/rust/pull/128458)
* [enforce supertrait outlives obligations hold when confirming impl](https://github.com/rust-lang/rust/pull/124336)
* [fix removed `box_syntax` diagnostic if source isn't available](https://github.com/rust-lang/rust/pull/128496)
* [fix the invalid argument type](https://github.com/rust-lang/rust/pull/128686)
* [ignore `use` declaration reformatting in `.git-blame-ignore-revs`](https://github.com/rust-lang/rust/pull/128478)
* [implement `UncheckedIterator` directly for `RepeatN`](https://github.com/rust-lang/rust/pull/128530)
* [improve error message when `global_asm!` uses `asm!` operands](https://github.com/rust-lang/rust/pull/128305)
* [interpret: on a signed deref check, mention the right pointer in the error](https://github.com/rust-lang/rust/pull/128482)
* [make `///` doc comments compatible with naked functions](https://github.com/rust-lang/rust/pull/128380)
* [mark `Parser::eat`/`check` methods as `#[must_use]`](https://github.com/rust-lang/rust/pull/128376)
* [match LLVM ABI in `extern "C"` functions for `f128` on Windows](https://github.com/rust-lang/rust/pull/128388)
* [match lowering: Hide `Candidate` from outside the lowering algorithm](https://github.com/rust-lang/rust/pull/127159)
* [more unsafe attr verification](https://github.com/rust-lang/rust/pull/127543)
* [normalize when equating `dyn` tails in MIR borrowck](https://github.com/rust-lang/rust/pull/128694)
* [on short error format, append primary span label to message](https://github.com/rust-lang/rust/pull/126804)
* [peel off explicit (or implicit) deref before suggesting clone on move error in borrowck, remove some hacks](https://github.com/rust-lang/rust/pull/128244)
* [properly mark loop as diverging if it has no breaks](https://github.com/rust-lang/rust/pull/128443)
* [remove `crate_level_only` from `ELIDED_LIFETIMES_IN_PATHS`](https://github.com/rust-lang/rust/pull/128412)
* [revert recent changes to dead code analysis](https://github.com/rust-lang/rust/pull/128404)
* [set branch protection function attributes](https://github.com/rust-lang/rust/pull/128141)
* [simplify match based on the cast result of `IntToInt`](https://github.com/rust-lang/rust/pull/127324)
* [structured suggestion for `extern crate foo` when `foo` isn't resolved in import](https://github.com/rust-lang/rust/pull/128151)
* [temporarily switch `ambiguous_negative_literals` lint to allow](https://github.com/rust-lang/rust/pull/128449)
* [the output in stderr expects panic-unwind](https://github.com/rust-lang/rust/pull/128379)
* [turn `invalid_type_param_default` into a `FutureReleaseErrorReportInDeps`](https://github.com/rust-lang/rust/pull/127655)
* [tweak type inference for `const` operands in inline asm](https://github.com/rust-lang/rust/pull/125558)
* [use `object` in `run-make/symbols-visibility`](https://github.com/rust-lang/rust/pull/128607)
* [use a separate pattern type for `rustc_pattern_analysis` diagnostics](https://github.com/rust-lang/rust/pull/128430)
* [miri: add a flag to do recursive validity checking](https://github.com/rust-lang/rust/pull/128531)
* [miri: add `miri_start` support](https://github.com/rust-lang/miri/pull/3769)
* [miri: use Scalar consistently in foreign item emulation](https://github.com/rust-lang/miri/pull/3776)
* [linker: pass fewer search directories to the linker](https://github.com/rust-lang/rust/pull/128370)
* [use Vec in `instantiate_binder_with_fresh_vars`](https://github.com/rust-lang/rust/pull/128336)
* [change output normalization logic to be linear against size of output](https://github.com/rust-lang/rust/pull/128200)
* [check divergence value first before doing span operations in `warn_if_unreachable`](https://github.com/rust-lang/rust/pull/128544)
* [accelerate GVN a little](https://github.com/rust-lang/rust/pull/126991)
* [stabilize Wasm relaxed SIMD](https://github.com/rust-lang/rust/pull/117468)
* [stabilize unsafe extern blocks (RFC 3484)](https://github.com/rust-lang/rust/pull/127921)
* [enable `std::io::copy` specialisation for `std::pipe::{PipeReader, PipeWriter}`](https://github.com/rust-lang/rust/pull/128303)
* [rewrite binary search implementation](https://github.com/rust-lang/rust/pull/128254)
* [implement cursors for `BTreeSet`](https://github.com/rust-lang/rust/pull/128309)
* [implement the `once_wait` feature](https://github.com/rust-lang/rust/pull/127567)
* [configure which platforms have `f16` and `f128` enabled by default](https://github.com/rust-lang/compiler-builtins/pull/652)
* [hashbrown: implement Default for iterators](https://github.com/rust-lang/hashbrown/pull/542)
* [regex: rust nightly removed the lifetime from Pattern](https://github.com/rust-lang/regex/pull/1219)
* [cargo-miri: better error when we seem to run inside bootstrap but something is wrong](https://github.com/rust-lang/rust/pull/128382)
* [cargo: build-std: remove hack on creating virtual std workspace](https://github.com/rust-lang/cargo/pull/14358)
* [cargo: config: Adjust MSRV resolve config field name / values](https://github.com/rust-lang/cargo/pull/14296)
* [cargo: publish: Don't strip non-dev features](https://github.com/rust-lang/cargo/pull/14325)
* [cargo: also build manpage for cargo.md](https://github.com/rust-lang/cargo/pull/14339)
* [rustdoc-json: discard non-local inherent impls for primitives](https://github.com/rust-lang/rust/pull/128385)
* [rustdoc: cleanup `CacheBuilder` code for building search index](https://github.com/rust-lang/rust/pull/128578)
* [rustdoc: fix handling of `Self` type in search index and refactor its representation](https://github.com/rust-lang/rust/pull/128471)
* [rustdoc: make the hover trail for doc anchors a bit bigger](https://github.com/rust-lang/rust/pull/128615)
* [rustdoc: Make the buttons remain when code example is clicked](https://github.com/rust-lang/rust/pull/128339)
* [rustdoc: simplify `body` usage](https://github.com/rust-lang/rust/pull/128573)
* [rustfmt: add repo cloning to check-diff crate](https://github.com/rust-lang/rustfmt/pull/6187)
* [rustfmt: check exit status of git commands spawned by build script](https://github.com/rust-lang/rustfmt/pull/6266)
* [rustfmt: impl `rewrite_result` for Pat, TuplePatField](https://github.com/rust-lang/rustfmt/pull/6262)
* [clippy: check exit status of subcommands spawned by `rustc_tools_util`](https://github.com/rust-lang/rust-clippy/pull/13217)
* [clippy: fix `redundant_closure` false positive with closures has return type contains `'static`](https://github.com/rust-lang/rust-clippy/pull/13108)
* [clippy: fix `redundant_slicing` when the slice is behind a mutable reference](https://github.com/rust-lang/rust-clippy/pull/13126)
* [clippy: fix broken list for lints config](https://github.com/rust-lang/rust-clippy/pull/13177)
* [clippy: fix false positive for `missing_backticks` in footnote references](https://github.com/rust-lang/rust-clippy/pull/13195)
* [clippy: limit number of `nonminimal_bool` ops](https://github.com/rust-lang/rust-clippy/pull/13209)
* [clippy: lintcheck: force warn all lints](https://github.com/rust-lang/rust-clippy/pull/13210)
* [clippy: make restriction lints use `span_lint_and_then` (a → e)](https://github.com/rust-lang/rust-clippy/pull/13136)
* [clippy: make restriction lints use `span_lint_and_then` (q → w)](https://github.com/rust-lang/rust-clippy/pull/13145)
* [clippy: remove `multispan_sugg[_with_applicability]`](https://github.com/rust-lang/rust-clippy/pull/13213)
* [clippy: remove duplicated `peel_middle_ty_refs`](https://github.com/rust-lang/rust-clippy/pull/13115)
* [clippy: simplify lint deprecation](https://github.com/rust-lang/rust-clippy/pull/13180)
* [clippy: use a deterministic number of digits in `rustc_tools_util` commit hashes](https://github.com/rust-lang/rust-clippy/pull/13222)
* [clippy: use a single multipart suggestion for `implicit_hasher`](https://github.com/rust-lang/rust-clippy/pull/13181)
* [rust-analyzer: implement diagnostic for `await` outside of `async`](https://github.com/rust-lang/rust-analyzer/pull/17791)
* [rust-analyzer: load sysroot library via cargo metadata](https://github.com/rust-lang/rust-analyzer/pull/17795)
* [rust-analyzer: support inlay hint for more expr with label](https://github.com/rust-lang/rust-analyzer/pull/17784)
* [rust-analyzer: apply `IndexMut` obligations for non-assigning mutable index usages](https://github.com/rust-lang/rust-analyzer/pull/17755)
* [rust-analyzer: errors on method call inferences with elided lifetimes](https://github.com/rust-lang/rust-analyzer/pull/17747)
* [rust-analyzer: insert a generic arg for `impl Trait` when lowering generic args](https://github.com/rust-lang/rust-analyzer/pull/17789)
* [rust-analyzer: insert a tail `Ok(())` for expr block instead of wrapping with `Ok`](https://github.com/rust-lang/rust-analyzer/pull/17763)
* [rust-analyzer: panic in path transform with default type parameters](https://github.com/rust-lang/rust-analyzer/pull/17805)
* [rust-analyzer: remove AbsPath requirement from linkedProjects](https://github.com/rust-lang/rust-analyzer/pull/17750)
* [rust-analyzer: surpress type mismatches in calls with mismatched arg counts](https://github.com/rust-lang/rust-analyzer/pull/17802)
* [rust-analyzer: improve crate manifests, adding missing `[package.repository]` and `[package.description]` fields](https://github.com/rust-lang/rust-analyzer/pull/17745)
* [rust-analyzer: segregate syntax and semantic diagnostics](https://github.com/rust-lang/rust-analyzer/pull/17775)
* [rust-analyzer: split out syntax-bridge into a separate crate](https://github.com/rust-lang/rust-analyzer/pull/17799)
* [rust-analyzer: when josh-proxy screws up the roundtrip, say what the involved commits are](https://github.com/rust-lang/rust-analyzer/pull/17761)

### Rust Compiler Performance Triage

This week saw several large improvements caused mostly by the [update to LLVM 19](https://github.com/rust-lang/rust/pull/127513). There were some regressions in several pull requests, but most of them were immediately fixed in a follow-up PR.

Triage done by **@kobzol**.
Revision range: [7e3a9718..8c7e0e16](https://perf.rust-lang.org/?start=7e3a971870f23c94f7aceb53b490fb37333150ff&end=8c7e0e160831866bc1a40691a39455aac21271c0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.0%  | [0.2%, 3.8%]    | 91    |
| Regressions ❌ <br /> (secondary)  | 1.9%  | [0.2%, 19.2%]   | 104   |
| Improvements ✅ <br /> (primary)   | -4.4% | [-15.8%, -0.3%] | 120   |
| Improvements ✅ <br /> (secondary) | -3.3% | [-10.4%, -0.2%] | 70    |
| All ❌✅ (primary)                 | -2.1% | [-15.8%, 3.8%]  | 211   |


6 Regressions, 3 Improvements, 5 Mixed; 4 of them in rollups
51 artifact comparisons made in total

[Full report here](https://github.com/Kobzol/rustc-perf/blob/1b5a3bf24549eea0a1a53a3de40a107ef35746fb/triage/2024-08-06.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Merge RFC 3529: Add named path bases to cargo](https://github.com/rust-lang/rfcs/pull/3529)
* [Merge RFC 3668: Async closures](https://github.com/rust-lang/rfcs/pull/3668)
* [Promote aarch64-apple-darwin to Tier 1](https://github.com/rust-lang/rfcs/pull/3671)
* [RFC for project goals](https://github.com/rust-lang/rfcs/pull/3672)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Stabilize `raw_ref_op` (RFC 2582)](https://github.com/rust-lang/rust/pull/127679)
* [disposition: merge] [Tracking Issue for `Ready::into_inner()`](https://github.com/rust-lang/rust/issues/101196)
* [disposition: merge] [Tracking issue for thread::Builder::spawn_unchecked](https://github.com/rust-lang/rust/issues/55132)
* [disposition: merge] [Tracking Issue for is_none_or](https://github.com/rust-lang/rust/issues/126383)
* [disposition: merge] [CloneToUninit impls](https://github.com/rust-lang/rust/pull/126877)
* [disposition: close] [Tracking issue for HashMap `OccupiedEntry::{replace_key, replace_entry}`](https://github.com/rust-lang/rust/issues/44286)
* [disposition: close] [Tracking issue for HashMap::raw_entry](https://github.com/rust-lang/rust/issues/56167)
* [disposition: merge] [Implement DoubleEnded and ExactSize for Take\<Repeat\> and Take\<RepeatWith\>](https://github.com/rust-lang/rust/pull/106943)
* [disposition: merge] [Implement owned ops for `HashSet` and `BTreeSet`](https://github.com/rust-lang/rust/pull/109402)
* [disposition: merge] [Tracking Issue for `Option::get_or_insert_default`](https://github.com/rust-lang/rust/issues/82901)
* [disposition: merge] [Unify run button display with "copy code" button and with mdbook buttons](https://github.com/rust-lang/rust/pull/128394)
* [disposition: merge] [Greatly speed up doctests by compiling compatible doctests in one file](https://github.com/rust-lang/rust/pull/126245)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: \<not specified\>] [Reformat (and only reformat) the inline assembly chapter](https://github.com/rust-lang/reference/pull/1550)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [crates.io: Remove dev-dependencies from the index](https://github.com/rust-lang/rfcs/pull/3674)

## Upcoming Events

Rusty Events between 2024-08-07 - 2024-09-04 🦀

### Virtual
* 2024-08-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328027/)
* 2024-08-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897918/)
* 2024-08-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300787793/)
* 2024-08-08 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
* 2024-08-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday: Typestate Pattern in Rust**](https://www.meetup.com/dallasrust/events/299346978/)
* 2024-08-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633266/)
* 2024-08-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346968/)
* 2024-08-21 | Hybrid - Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631735/)
* 2024-08-22 | Virtual | [Conf42: Online Tech Events](https://www.meetup.com/conf42/)
    * [**Conf42 Rustlang 2024**](https://www.meetup.com/conf42/events/297266825/)
* 2024-08-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897938/)
* 2024-08-22 | Virtual (Karlsruhe, DE) | [Karlsruhe Functional Programmers Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA): various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293937801)
* 2024-08-27 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Fearless Concurrency with Rust**](https://www.eventbrite.com/e/fearless-concurrency-with-rust-tickets-934569591807)
* 2024-08-27 | Virtual (Bordeaux, FR) | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Live coding - A distance #1**](https://www.meetup.com/bordeaux-rust/events/302570681/)
* 2024-08-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585668/)
* 2024-08-27 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Declarative macros in Rust (Virtual) - מקרוים בראסט**](https://www.meetup.com/rust-in-israel/events/302327956/)
* 2024-08-28 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633267/)
* 2024-09-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007365/)
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328029/)

### Europe
* 2024-08-14 | Köln/Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, August**](https://www.meetup.com/rustcologne/events/302674635/)
* 2024-08-14 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302153005/)
* 2024-08-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/301522950/)
* 2024-08-21 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk'n'Talk around Wöhrder See (+ Burritos)**](https://www.meetup.com/rust-noris/events/302080495/)
* 2024-08-22 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks August**](https://www.meetup.com/rust-manchester/events/302419276/)
* 2024-08-26 | Mainz, DE | [Fachschaft Mathematik+Informatik der JGU Mainz](https://rheinneckar.events/@fsmathe_informatik_mainz@rheinmain.social)
    * [**Ferienkurs Rust**](https://rheinneckar.events/events/3f76f860-75c1-4f3a-810f-03fc0cecb691)
* 2024-08-29 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421378/)

### North America
* 2024-08-08 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302067008/)
* 2024-08-08 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/302330477/)
* 2024-08-19 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup: "State of Rust GPU Programming" & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/301428949/)
* 2024-08-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/Rust-NYC/)
    * [**Rust NYC: Doing the Bare Minimum with Isograph (talk)**](https://www.meetup.com/rust-nyc/events/302480064/)
* 2024-08-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/301614081/)
* 2024-08-21 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631735/)
* 2024-08-28 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygclblc/)
* 2024-08-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : placeholder**](https://www.meetup.com/music-city-rust-developers/events/301420110/)

# Oceania
* 2024-08-22 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Dot IX: Diagram Generator + Deep Learning from Scratch in Rust**](https://www.meetup.com/rust-akl/events/302431924/)
* 2024-08-27 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/301887261/)

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

> Want to have a crate with a million features? Host your own registry and revel in the combinatorial explosion of choices!

– [Jake Goulding on rust-users](https://users.rust-lang.org/t/margo-a-simple-cargo-registry-using-static-files/115412)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1599) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1enceo0/this_week_in_rust_559_this_week_in_rust/)</small>
