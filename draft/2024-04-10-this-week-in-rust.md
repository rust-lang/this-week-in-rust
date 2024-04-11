Title: This Week in Rust 542
Number: 542
Date: 2024-04-10
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
* [Announcing Rust 1.77.2](https://blog.rust-lang.org/2024/04/09/Rust-1.77.2.html)
* [Security advisory for the standard library (CVE-2024-24576)](https://blog.rust-lang.org/2024/04/09/cve-2024-24576.html)
* [Changes to Rust's WASI targets](https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html)

### Foundation

### Newsletters

### Rust Nation UK
* [Hannah Aubrey - A Web of Rust: The Future of the Internet Depends on Trust](https://www.youtube.com/watch?v=mM8TiAoPdQQ)
* [JD Nose - Rust Infrastructure: What it takes to keep Rust running](https://www.youtube.com/watch?v=GnLZMJ2r7sk)
* [Amanieu D'Antras - The path to a stable ABI for Rust](https://www.youtube.com/watch?v=MY5kYqWeV1Q)
* [Luca Palmieri - Pavex: re-imaging API development in Rust](https://www.youtube.com/watch?v=cMea6IMRk2s)
* [Lachezar Lechev - Typed for Safety](https://www.youtube.com/watch?v=pnloY3pDgk4)
* [Marco Concetto Rudilosso - Building a profiler for web assembly](https://www.youtube.com/watch?v=sMN9q4RkcuI)
* [Jon Gjengset - Towards Impeccable Rust](https://www.youtube.com/watch?v=qfknfCsICUM)
* [Nicholas Yang - Porting Turborepo From Go To Rust](https://www.youtube.com/watch?v=RILymfTIcoo)
* [David Haig - What’s that behind your ear? An open source hearing aid in Rust.](https://www.youtube.com/watch?v=GKMIYXK1I74)
* [Frédéric Ameye - Renault want to sell cars with rust!](https://www.youtube.com/watch?v=Z1xMvm3eS4k)
* [Nikita Lapkov - Type-safe and fault-tolerant mesh services with Rust](https://www.youtube.com/watch?v=8rZJY9ps4ZE)
* [Andre Bogus - Easy Mode Rust](https://www.youtube.com/watch?v=33FG6O3qejM)
* [Lars Bergstrom - Beyond Safety and Speed: How Rust Fuels Team Productivity](https://www.youtube.com/watch?v=QrrH2lcl9ew)
* [Tim McNamara - Unwrapping unsafe](https://www.youtube.com/watch?v=mdaWeql7C3k)
* [Nicholas Matsakis - Rust 2024 and beyond](https://www.youtube.com/watch?v=04gTQmLETFI)

### Project/Tooling Updates
* [Shipping Jco 1.0, WASI 0.2](https://blog.yoshuawuyts.com/jco-1-0-wasi-0-2/)
* [This month in Pavex, #10](https://www.lpalmieri.com/posts/this-month-in-pavex-10/)
* ["Containerize" individual functions in Rust with extrasafe](https://harrystern.net/extrasafe-user-namespaces.html)
* [rust-analyzer changelog #228](https://rust-analyzer.github.io/thisweek/2024/04/08/changelog-228.html)
* [Rerun 0.15.0 - Blueprints from Python · rerun-io/rerun](https://github.com/rerun-io/rerun/releases/tag/0.15.0)
* [Bevy 0.13.2, Curves, Gizmos, and Games](https://thisweekinbevy.com/issue/2024-04-08-bevy-0-13-2-curves-gizmos-and-games)

### Observations/Thoughts
* [Ownership in Rust](https://smallcultfollowing.com/babysteps/blog/2024/04/05/ownership-in-rust/)
* [Thoughts on the xz backdoor: an lzma-rs perspective](https://gendignoux.com/blog/2024/04/08/xz-backdoor.html)
* [hyper HTTP/2 Continuation Flood](https://seanmonstar.com/blog/hyper-http2-continuation-flood/)
* [Leaky Abstractions and a Rusty Pin](https://medium.com/itnext/leaky-abstractions-and-a-rusty-pin-fbf3b84eea1f)
* [audio] [Launching RustRover: JetBrains' Investment in Rust](https://rustacean-station.org/episode/vitaly-bragilevsky/)
* [audio] [Pavex with Luca Palmieri](https://rustacean-station.org/episode/luca-palmieri-pavex/)
* [video] [Decrusting the tokio crate](https://www.youtube.com/watch?v=o2ob8zkeq2s)
* [video] [Rust 1.77.0: 70 highlights in 30 minutes](https://www.youtube.com/watch?v=A6NJfq5pPaw)
* [video] [Simulate the three body problem in #rustlang](https://www.youtube.com/watch?v=SNnXP4TBc7g)

### Rust Walkthroughs
* [Working with OpenAPI using Rust](https://www.shuttle.rs/blog/2024/04/04/using-openapi-rust)
* [Zed Decoded: Async Rust](https://zed.dev/blog/zed-decoded-async-rust)
* [Writing a Unix-like OS in Rust](https://vmm.dev/en/rust/osinrust.md)
* [Fivefold Slower Compared to Go? Optimizing Rust's Protobuf Decoding Performance](https://www.greptime.com/blogs/2024-04-09-rust-protobuf-performance)
* [Write Cleaner, More Maintainable Rust Code with PhantomData](https://aayushyavajpayee.substack.com/p/coming-soon)

### Research
* ["Against the Void": An Interview and Survey Study on How Rust Developers Use Unsafe Code](https://arxiv.org/abs/2404.02230)
* [Sound Borrow-Checking for Rust via Symbolic Semantics](https://arxiv.org/abs/2404.02680)

### Miscellaneous
* [Rust Meetup and user groups (updated)](https://rust.code-maven.com/user-groups)
* [Embedding the Servo Web Engine in Qt](https://www.kdab.com/embedding-servo-in-qt/)
* [A memory model for Rust code in the kernel](https://lwn.net/SubscriberLink/967049/0ffb9b9ed8940013/)

## Crate of the Week

This week's crate is [archspec-rs](https://github.com/prefix-dev/archspec-rs), a library to track system architecture aspects.

Thanks to [Orhun Parmaksız](https://users.rust-lang.org/t/crate-of-the-week/2704/1302) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

431 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-04-02..2024-04-09

* [CFI: change type transformation to use TypeFolder](https://github.com/rust-lang/rust/pull/123212)
* [CFI: fix ICE in KCFI non-associated function pointers](https://github.com/rust-lang/rust/pull/123635)
* [CFI: restore `typeid_for_instance` default behavior](https://github.com/rust-lang/rust/pull/123487)
* [CFI: support function pointers for trait methods](https://github.com/rust-lang/rust/pull/123052)
* [CFI: support non-general coroutines](https://github.com/rust-lang/rust/pull/123368)
* [MSVC targets should use COFF as their archive format](https://github.com/rust-lang/rust/pull/123467)
* [actually use the inferred `ClosureKind` from signature inference in coroutine-closures](https://github.com/rust-lang/rust/pull/123350)
* [add `Ord::cmp` for primitives as a `BinOp` in MIR](https://github.com/rust-lang/rust/pull/118310)
* [add a debug asserts call to `match_projection_projections` to ensure invariant](https://github.com/rust-lang/rust/pull/123559)
* [add aarch64-apple-visionos and aarch64-apple-visionos-sim tier 3 targets](https://github.com/rust-lang/rust/pull/121419)
* [add consistency with phrases "meantime" and "mean time"](https://github.com/rust-lang/rust/pull/122807)
* [assert `FnDef` kind](https://github.com/rust-lang/rust/pull/123382)
* [assert that args are actually compatible with their generics, rather than just their count](https://github.com/rust-lang/rust/pull/123240)
* [avoid ICEing without the `pattern_types` feature gate](https://github.com/rust-lang/rust/pull/123648)
* [avoid expanding to unstable internal method](https://github.com/rust-lang/rust/pull/123182)
* [avoid panicking unnecessarily on startup](https://github.com/rust-lang/rust/pull/123389)
* [better reporting on generic argument mismatchs](https://github.com/rust-lang/rust/pull/121595)
* [cleanup: rename `HAS_PROJECTIONS` to `HAS_ALIASES` etc](https://github.com/rust-lang/rust/pull/123464)
* [do not ICE in `fn forced_ambiguity` if we get an error](https://github.com/rust-lang/rust/pull/123477)
* [do not ICE on field access check on expr with `ty::Error`](https://github.com/rust-lang/rust/pull/123516)
* [do not ICE when calling incorrectly defined `transmute` intrinsic](https://github.com/rust-lang/rust/pull/123526)
* [fix `ByMove` coroutine-closure shim (for 2021 precise closure capturing behavior)](https://github.com/rust-lang/rust/pull/123518)
* [fix capture analysis for by-move closure bodies](https://github.com/rust-lang/rust/pull/123349)
* [fix diagnostic for qualifier in extern block](https://github.com/rust-lang/rust/pull/123397)
* [hir: use `ItemLocalId::ZERO` in a couple more places](https://github.com/rust-lang/rust/pull/123454)
* [impl `get_mut_or_init` and `get_mut_or_try_init` for OnceCell and OnceLock](https://github.com/rust-lang/rust/pull/114788)
* [implement T-types suggested logic for perfect non-local impl detection](https://github.com/rust-lang/rust/pull/122747)
* [implement minimal, internal-only pattern types in the type system](https://github.com/rust-lang/rust/pull/120131)
* [instantiate higher ranked goals outside of candidate selection](https://github.com/rust-lang/rust/pull/119820)
* [link against libc++abi and libunwind as well when building LLVM wrappers on AIX](https://github.com/rust-lang/rust/pull/123359)
* [make inductive cycles always ambiguous](https://github.com/rust-lang/rust/pull/122791)
* [make sure to insert `Sized` bound first into clauses list](https://github.com/rust-lang/rust/pull/123302)
* [match ergonomics: implement "`&`pat everywhere"](https://github.com/rust-lang/rust/pull/123311)
* [match lowering: make false edges more precise](https://github.com/rust-lang/rust/pull/123324)
* [more postfix match fixes](https://github.com/rust-lang/rust/pull/123395)
* [move check for error in impl header outside of reporting](https://github.com/rust-lang/rust/pull/122894)
* [only allow `compiler_builtins` to call LLVM intrinsics, not any `link_name` function](https://github.com/rust-lang/rust/pull/123347)
* [only inspect user-written predicates for privacy concerns](https://github.com/rust-lang/rust/pull/123377)
* [pass list of defineable opaque types into canonical queries](https://github.com/rust-lang/rust/pull/122077)
* [pattern analysis: fix union handling](https://github.com/rust-lang/rust/pull/123301)
* [postfix match fixes](https://github.com/rust-lang/rust/pull/123394)
* [privacy: stabilize lint `unnameable_types`](https://github.com/rust-lang/rust/pull/120144)
* [put checks that detect UB under their own flag below `debug_assertions`](https://github.com/rust-lang/rust/pull/123411)
* [revert removing miri jobserver workaround](https://github.com/rust-lang/rust/pull/123500)
* [safe Transmute: Compute transmutability from `rustc_target::abi::Layout`](https://github.com/rust-lang/rust/pull/123367)
* [sanitizers: create the `rustc_sanitizers` crate](https://github.com/rust-lang/rust/pull/123620)
* [split hir ty lowerer's error reporting code in check functions to mod errors](https://github.com/rust-lang/rust/pull/122865)
* [teach MIR inliner query cycle avoidance about `const_eval_select`](https://github.com/rust-lang/rust/pull/123444)
* [transforms match into an assignment statement](https://github.com/rust-lang/rust/pull/120614)
* [use the more informative generic type inference failure error on method calls on raw pointers](https://github.com/rust-lang/rust/pull/122768)
* [add missing `?Sized` bounds for `HasInterner` impls](https://github.com/rust-lang/chalk/pull/810)
* [introduce `Lifetime::Error`](https://github.com/rust-lang/chalk/pull/809)
* [perf: cache type info for ParamEnv](https://github.com/rust-lang/rust/pull/123058)
* [encode dep graph edges directly from the previous graph when promoting](https://github.com/rust-lang/rust/pull/122070)
* [remove debuginfo from rustc-demangle too](https://github.com/rust-lang/rust/pull/123608)
* [stabilize `const_caller_location` and `const_location_fields`](https://github.com/rust-lang/rust/pull/122291)
* [stabilize `proc_macro_byte_character` and `proc_macro_c_str_literals`](https://github.com/rust-lang/rust/pull/123431)
* [stabilize const `Atomic*::into_inner`](https://github.com/rust-lang/rust/pull/123522)
* [de-LLVM the unchecked shifts](https://github.com/rust-lang/rust/pull/123226)
* [rename `expose_addr` to `expose_provenance`](https://github.com/rust-lang/rust/pull/122964)
* [rename `ptr::from_exposed_addr` → `ptr::with_exposed_provenance`](https://github.com/rust-lang/rust/pull/122935)
* [remove `rt::init` allocation for thread name](https://github.com/rust-lang/rust/pull/123433)
* [use `unchecked_sub` in str indexing](https://github.com/rust-lang/rust/pull/123561)
* [don't emit divide-by-zero panic paths in `StepBy::len`](https://github.com/rust-lang/rust/pull/123564)
* [add fn const `BuildHasherDefault::new`](https://github.com/rust-lang/rust/pull/123198)
* [add invariant to `VecDeque::pop_*` that len `<` cap if pop successful](https://github.com/rust-lang/rust/pull/123089)
* [add `Context::ext`](https://github.com/rust-lang/rust/pull/123203)
* [provide `cabi_realloc` on wasm32-wasip2 by default](https://github.com/rust-lang/rust/pull/122411)
* [vendor `rustc_codegen_gcc`](https://github.com/rust-lang/rust/pull/122334)
* [cargo: Build script not rerun when target rustflags change](https://github.com/rust-lang/cargo/pull/13560)
* [cargo add: Stabilize MSRV-aware version req selection](https://github.com/rust-lang/cargo/pull/13608)
* [cargo toml: Decouple target discovery from Target creation](https://github.com/rust-lang/cargo/pull/13701)
* [cargo toml: Split out an explicit step to resolve `Cargo.toml`](https://github.com/rust-lang/cargo/pull/13693)
* [cargo metadata: Show behavior with TOML-specific types](https://github.com/rust-lang/cargo/pull/13703)
* [cargo: don't depend on `?` affecting type inference in weird ways](https://github.com/rust-lang/cargo/pull/13706)
* [cargo: fix github fast path redirect](https://github.com/rust-lang/cargo/pull/13718)
* [cargo: maintain sorting of dependency features](https://github.com/rust-lang/cargo/pull/13682)
* [cargo: switch to using gitoxide by default for listing files](https://github.com/rust-lang/cargo/pull/13696)
* [rustdoc-search: shard the search result descriptions](https://github.com/rust-lang/rust/pull/122614)
* [rustdoc: default to light theme if JS is enabled but not working](https://github.com/rust-lang/rust/pull/123407)
* [rustdoc: heavily simplify the synthesis of auto trait impls](https://github.com/rust-lang/rust/pull/123340)
* [rustdoc: synthetic auto trait impls: accept unresolved region vars for now](https://github.com/rust-lang/rust/pull/123375)
* [clippy: `manual_swap` auto fix](https://github.com/rust-lang/rust-clippy/pull/12340)
* [clippy: `manual_unwrap_or_default`: check for `Default` trait implementation in initial condition when linting and use `IfLetOrMatch`](https://github.com/rust-lang/rust-clippy/pull/12610)
* [clippy: allow `cast` lints in macros](https://github.com/rust-lang/rust-clippy/pull/12631)
* [clippy: avoid an ICE in `ptr_as_ptr` when getting the `def_id` of a local](https://github.com/rust-lang/rust-clippy/pull/12617)
* [clippy: correct parentheses for `needless_borrow` suggestion](https://github.com/rust-lang/rust-clippy/pull/12630)
* [clippy: do not suggest `assigning_clones` in `Clone` impl](https://github.com/rust-lang/rust-clippy/pull/12615)
* [clippy: fix ice reporting in lintcheck](https://github.com/rust-lang/rust-clippy/pull/12439)
* [clippy: fix incorrect suggestion for `!(a as type >= b)`](https://github.com/rust-lang/rust-clippy/pull/12626)
* [clippy: reword `arc_with_non_send_sync` note and help messages](https://github.com/rust-lang/rust-clippy/pull/12609)
* [clippy: type certainty: clear `DefId` when an expression's type changes to non-adt](https://github.com/rust-lang/rust-clippy/pull/12591)
* [rust-analyzer: apply cargo flags in test explorer](https://github.com/rust-lang/rust-analyzer/pull/17016)
* [rust-analyzer: fix off-by-one error converting to LSP UTF8 offsets with multi-byte char](https://github.com/rust-lang/rust-analyzer/pull/17003)
* [rust-analyzer: consider `exported_name="main"` functions in test modules as tests](https://github.com/rust-lang/rust-analyzer/pull/17014)
* [rust-analyzer: fix `patch_cfg_if` not applying with stitched sysroot](https://github.com/rust-lang/rust-analyzer/pull/16997)
* [rust-analyzer: set the right postfix snippets competion source range](https://github.com/rust-lang/rust-analyzer/pull/17000)

### Rust Compiler Performance Triage

A quiet week; all the outright regressions were already triaged (the one biggish one was #122077, which is justified as an important bug fix). There was a very nice set of improvements from PR #122070, which cleverly avoids a lot of unnecessary allocator calls when building an incremental dep graph by reusing the old edges from the previous graph.

Triage done by **@pnkfelix**.
Revision range: [3d5528c2..86b603cd](https://perf.rust-lang.org/?start=3d5528c287860b918e178a34f04ff903325571b3&end=86b603cd792b3f6172ba4f676d7b586c1af7630a&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 7 Mixed; 1 of them in rollups
78 artifact comparisons made in total

See full report [here](https://github.com/rust-lang/rustc-perf/pull/1890/files?short_path=20043ae#diff-20043aeaa0842acfd2c504cfc1b0ee53479877678534960356f244c156250849)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Merge RFC 3513: Add gen blocks](https://github.com/rust-lang/rfcs/commit/bc01ed83c19bd96fdd2eb7b7c83c0f1e45f877a9)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [RFC: Drop temporaries in tail expressions before local variables](https://github.com/rust-lang/rfcs/pull/3606)
* [disposition: merge] [RFC: Reserve unprefixed guarded string literals in Edition 2024](https://github.com/rust-lang/rfcs/pull/3593)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Always display stability version even if it's the same as the containing item ](https://github.com/rust-lang/rust/pull/118441)
* [disposition: merge] [Tracking Issue for `cstr_count_bytes`](https://github.com/rust-lang/rust/issues/114441)
* [disposition: merge] [rustdoc-search: single result for items with multiple paths](https://github.com/rust-lang/rust/pull/119912)
* [disposition: merge] [Tracking Issue for `#![feature(const_io_structs)]`](https://github.com/rust-lang/rust/issues/78812)
* [disposition: merge] [Tracking Issue for `alloc::collections::BinaryHeap::as_slice`](https://github.com/rust-lang/rust/issues/83659)
* [disposition: merge] [Tracking Issue for fs_try_exists](https://github.com/rust-lang/rust/issues/83186)
* [disposition: merge] [stabilize `-Znext-solver=coherence`](https://github.com/rust-lang/rust/pull/121848)
* [disposition: merge] [Document overrides of clone_from() in core/std](https://github.com/rust-lang/rust/pull/122201)
* [disposition: merge] [Stabilise inline_const](https://github.com/rust-lang/rust/pull/104087)
* [disposition: merge] [Tracking Issue for RFC 3013: Checking conditional compilation at compile time](https://github.com/rust-lang/rust/issues/82450)
* [disposition: merge] [sess: stabilize `-Zrelro-level` as `-Crelro-level`](https://github.com/rust-lang/rust/pull/121694)
* [disposition: merge] [Implement `FromIterator` for `(impl Default + Extend, impl Default + Extend)`](https://github.com/rust-lang/rust/pull/107462)
* [disposition: close] [Return the delimiter from slice::split_once](https://github.com/rust-lang/rust/pull/119799)
* [disposition: merge] [Support type '/' to search](https://github.com/rust-lang/rust/pull/123355)
* [disposition: merge] [Tracking Issue for `Seek::seek_relative`](https://github.com/rust-lang/rust/issues/117374)
* [disposition: merge] [Tracking Issue for generic `NonZero`](https://github.com/rust-lang/rust/issues/120257)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Add an expression for direct access to an enum's discriminant](https://github.com/rust-lang/rfcs/pull/3607)
* [new] [RFC: Drop temporaries in tail expressions before local variables](https://github.com/rust-lang/rfcs/pull/3606)

## Upcoming Events

Rusty Events between 2024-04-10 - 2024-05-08 🦀

### Virtual

* 2024-04-11 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477689/)
* 2024-04-11 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945256/)
* 2024-04-15 & 2024-04-16 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [**Remote Workshop: Testing for Rust projects – going beyond the basics**](https://ti.to/mainmatter/rust-testing-april-2024)
* 2024-04-16 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**A reverse proxy with Tower and Hyperv1**](https://www.meetup.com/rust-dublin/events/300144192/)
* 2024-04-16 | Virtual (Washinigton, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346486/)
* 2024-04-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-04-18 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368799/)
* 2024-04-21 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/) 
    * [**Using AstroNvim for Rust development (in Hebrew)**](https://www.meetup.com/code-mavens/events/300265648/)
* 2024-04-25 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477692/)
* 2024-04-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcgbnc/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)
* 2024-05-07 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)

### Africa

* 2024-05-04 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Europe

* 2024-04-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Rust Meetup Reboot 3**](https://www.meetup.com/cambridge-rust-meetup/events/299730322/)
* 2024-04-10 | Cologne/Köln, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, April**](https://www.meetup.com/rustcologne/events/300191375/)
* 2024-04-10 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester April 2024**](https://www.meetup.com/rust-manchester/events/299887934/)
* 2024-04-10 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/299488225/)
* 2024-04-11 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #2 : Présentations**](https://www.meetup.com/bordeaux-rust/events/299628716/)
* 2024-04-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/299694473/)
* 2024-04-15 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/)
    * [**Rust Meetup 2024/04: Building cargo projects with NIX**](https://www.meetup.com/zagreb-rust-meetup/events/299905015/)
* 2024-04-16 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #5**](https://www.meetup.com/bratislava-rust-meetup-group/events/299302952/)
* 2024-04-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**winnow/nom**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/300024630/)
* 2024-04-16 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-04-17 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/)
    * [**Lær Rust med Conways Game of Life**](https://www.meetup.com/bergen-html-css-meetup-group/events/300031586/)
* 2024-04-17 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/)
    * [**TechMeetup: RUST**](https://www.meetup.com/techmeetupostrava/events/299912212/)
* 2024-04-20 | Augsburg, DE | [Augsburger Linux-Infotag 2024](https://www.luga.de/static/LIT-2024/)
   * [**Augsburger Linux-Infotag 2024: Workshop Einstieg in Embedded Rust mit dem Raspberry Pico WH**](https://www.luga.de/static/LIT-2024/talks/einstieg_in_embedded_rust_mit_dem_raspberry_pico_wh/)
* 2024-04-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust'n'Tell - Rust for the Web**](https://www.meetup.com/rust-berlin/events/300047151/)
* 2024-04-23 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #67**](https://mobilizon.fr/events/4ba93021-c43a-4e4a-b3e5-09c1c0d0a957)
* 2024-04-25 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at MFT Energy**](https://www.meetup.com/rust-aarhus/events/299564517/)
* 2024-04-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust'n'Tell - Rust for the Web**](https://www.meetup.com/rust-berlin/events/300047151/)
* 2024-04-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - TBD**](https://www.meetup.com/rust-berlin/events/299288960/)
* 2024-04-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Fullstack Rust - Workshop #2 (Register by 23 April)**](https://www.meetup.com/rust-basel/events/299933581/)

### North America

* 2024-04-10 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Rust Meetup: Better Builds w/ Flox + Hangs**](https://www.meetup.com/boulder-rust-meetup/events/300019409/)
* 2024-04-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Interactive Storytelling using Yarn Spinner with Rex Magana**](https://www.meetup.com/utah-rust/events/300264363/)
* 2024-04-11 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509326/)
* 2024-04-11 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300019993/)
* 2024-04-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Davis Square Rust Lunch, Apr 15**](https://www.meetup.com/bostonrust/events/300116673/)
* 2024-04-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186907/)
* 2024-04-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group: Meet Servo and Robius Open Source Projects**](https://www.meetup.com/seattle-rust-user-group/events/299908469/)
* 2024-04-18 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803586/)
* 2024-04-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299960315/)
* 2024-04-25 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers - Async Rust on Embedded**](https://www.meetup.com/music-city-rust-developers/events/299976876/)
* 2024-04-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Apr 26**](https://www.meetup.com/bostonrust/events/300116689/)

### Oceania

* 2024-04-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**April Meetup**](https://www.meetup.com/rust-canberra/events/300023000/)

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

> As a former JavaScript plebeian who has only been semi-recently illuminated by the suspiciously pastel pink, white and blue radiance of Rust developers, NOT having to sit in my web console debugger for hours pushing some lovingly crafted `[object Object]` or `undefined` is a blessing.

– [Julien Robert rage-blogging against bevy](https://oneirical.github.io/bevyrage)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1558) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
