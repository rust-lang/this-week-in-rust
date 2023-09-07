Title: This Week in Rust 511
Number: 511
Date: 2023-09-06
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
* [Electing New Project Directors](https://blog.rust-lang.org/2023/08/30/electing-new-project-directors.html)
* [Keeping Rust projects secure with cargo-audit 0.18: performance, compatibility and security improvements](https://blog.rust-lang.org/inside-rust/2023/09/04/keeping-secure-with-cargo-audit-0.18.html)
* [crates.io Postmortem: User Uploaded Malware](https://blog.rust-lang.org/inside-rust/2023/09/01/crates-io-malware-postmortem.html)
* [Announcing Freya GUI library](https://marc0.hashnode.dev/freya)

### Foundation

### Newsletters
* [This Month in Rust GameDev #48 - July 2023](https://gamedev.rs/news/048/)

### Project/Tooling Updates
* [Slint 1.2 Released with Enhanced Platform Abstraction](https://slint.dev/blog/slint-1.2-released)
* [Announcing diesel-async 0.4](https://blog.weiznich.de/blog/diesel-async-0-4/)
* [git-cliff 1.3.0 is released! (highly customizable changelog generator)](https://git-cliff.org/blog/1.3.0/)
* [capnp 0.18 release](https://dwrensha.github.io/capnproto-rust/2023/09/04/0.18-release.html)
* [rustc_codegen_gcc: Progress Report #25](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-25)
* [Type-safe Rust-In-Flutter 3.0](https://reddit.com/r/rust/s/glIVUEPb8U)
* [Announcing Nucleo: A fast fuzzy matcher library](https://reddit.com/r/rust/s/GeGknPLPOm)

### Observations/Thoughts
* [Elevate Your Rust Code: The Art of Separating Actions and Calculations](https://rusty-ferris.pages.dev/blog/fp-actions-vs-calculations/)
* [Are we embedded yet?](https://tweedegolf.nl/en/blog/101/are-we-embedded-yet)
* [Pavex DevLog #6: designing safe and ergonomic middlewares](https://www.lpalmieri.com/posts/pavex-progress-report-06/)
* [Using Kani to Validate Security Boundaries in AWS Firecracker](https://model-checking.github.io/kani-verifier-blog/2023/08/31/using-kani-to-validate-security-boundaries-in-aws-firecracker.html)
* [Views on Machine Learning in Rust](https://deepcausality.com/blog/views-on-rust-ml/)
* [video] [5 Strong Opinions On Everyday Rust](https://www.youtube.com/watch?v=8j_FbjiowvE)

* [video] [Core Flight System Applications in Rust with n2o4](https://www.youtube.com/live/H7q-7CNb7gc?si=zgWjLlaCxNHJfRJq&t=8229)

### Rust Walkthroughs
* [How I finally understood async/await in Rust (part 4: why would I ever want to write a future manually?)](https://hegdenu.net/posts/understanding-async-await-4/)
* [Use just to run commands in Rust projects](https://developerlife.com/2023/08/28/justfile/)
* [How to Implement OAuth in Rust](https://www.shuttle.rs/blog/2023/08/30/using-oauth-with-axum)
* [Integrating a large Tokio-based Rust library with Haskell](https://www.iankduncan.com/articles/2023-08-30-integrating-haskell-and-tokio)
* [A series of 3 tutorials that walks one through the world of Actors as implemented by 'ractor' framework ](https://github.com/nsengupta/ractor-tutorial#readme)
* [video] [Raytracing in Rust](https://www.youtube.com/watch?v=6D8WVYm1YwY)
* [Trying out Rust's Async Functions in Traits](https://broch.tech/posts/rust-async-fn-trait/)

### Research
* [Accelerating package expansion in Rust through development of a semantic versioning tool](https://arxiv.org/abs/2308.14623)

### Miscellaneous
* [Why Rust is the most admired language among developers](https://github.blog/2023-08-30-why-rust-is-the-most-admired-language-among-developers/)
* [FFI optimizations and benchmarking](https://godot-rust.github.io/dev/ffi-optimizations-benchmarking/)

* [video] [Creating a spectrogram generator - "peer programming" with the Internet](https://www.youtube.com/watch?v=PoaHybRUC18)

* [Becoming a [Rust] contractor](https://ochagavia.nl/blog/becoming-a-contractor/)

## Crate of the Week

This week's crate is [str0m](https://github.com/algesten/str0m), a synchronous sans-IO WebRTC implementation.

Thanks to [Hugo Tunius](https://users.rust-lang.org/t/crate-of-the-week/2704/1235) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hyperswitch - add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - deserialization error exposes sensitive values in the logs](https://github.com/juspay/hyperswitch/issues/1943)
* [Hyperswitch - remove unnecessary function from Refunds Validate Flow](https://github.com/juspay/hyperswitch/issues/1521)
* [Ockam - Inconsistent behavior of `ockam tcp-outlet create`](https://github.com/build-trust/ockam/issues/5897)
* [Ockam - Incorrect alias handling in outlet creation](https://github.com/build-trust/ockam/issues/5898)
* [Ockam - Investigate Github API error in github actions workflows that use nix](https://github.com/build-trust/ockam/issues/5895)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

357 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-08-28..2023-09-04

* [`rustc_llvm`: Link to `zlib` on dragonfly and solaris](https://github.com/rust-lang/rust/pull/114349)
* [adapt table sizes to the contents, accommodating u64 rmeta offsets](https://github.com/rust-lang/rust/pull/113542)
* [add `ParallelGuard` type to handle unwinding in parallel sections](https://github.com/rust-lang/rust/pull/115144)
* [add alignment to the NPO guarantee](https://github.com/rust-lang/rust/pull/114845)
* [add new interface to smir](https://github.com/rust-lang/rust/pull/115187)
* [add note on non-exhaustiveness when matching on str and nested non-exhaustive enums](https://github.com/rust-lang/rust/pull/115270)
* [add stable provenance](https://github.com/rust-lang/rust/pull/115470)
* [also skip musl checks when `BOOTSTRAP_SKIP_TARGET_SANITY` is set](https://github.com/rust-lang/rust/pull/115471)
* [always add `LC_BUILD_VERSION` for metadata object files](https://github.com/rust-lang/rust/pull/114114)
* [avoid stdout redirection on `curl` executions](https://github.com/rust-lang/rust/pull/115323)
* [capture all lifetimes for TAITs and impl trait in associated types](https://github.com/rust-lang/rust/pull/114616)
* [capture lifetimes for associated type bounds destined to be lowered to opaques](https://github.com/rust-lang/rust/pull/115366)
* [create a SMIR visitor](https://github.com/rust-lang/rust/pull/115444)
* [diagnostics: avoid wrong `unused_parens` on `x as (T) < y`](https://github.com/rust-lang/rust/pull/115424)
* [parser: not insert dummy field in `struct`](https://github.com/rust-lang/rust/pull/114704)
* [permit recursive weak type aliases](https://github.com/rust-lang/rust/pull/113201)
* [replace old private-in-public diagnostic with type privacy lints](https://github.com/rust-lang/rust/pull/113126)
* [return ident for `ExprField` and `PatField` HIR nodes](https://github.com/rust-lang/rust/pull/115400)
* [skip rendering metadata strings from `include_str!/include_bytes!`](https://github.com/rust-lang/rust/pull/115286)
* [suggest removing `impl` in generic trait bound position](https://github.com/rust-lang/rust/pull/115347)
* [work around ICE in diagnostics for local super-universes missing `UniverseInfo`s](https://github.com/rust-lang/rust/pull/115384)
* [don't ICE on layout computation failure](https://github.com/rust-lang/rust/pull/111580)
* [don't forget to normalize the translated message](https://github.com/rust-lang/rust/pull/115513)
* [don't manually compute param indices when adding implicit `Sized` and `ConstParamHasTy`](https://github.com/rust-lang/rust/pull/115361)
* [don't record spans for predicates in coherence](https://github.com/rust-lang/rust/pull/115392)
* [don't suggest adding parentheses to call an inaccessible method](https://github.com/rust-lang/rust/pull/115363)
* [emit unused doc comment warnings for pat and expr fields](https://github.com/rust-lang/rust/pull/115478)
* [fix inlining with -Zalways-encode-mir](https://github.com/rust-lang/rust/pull/115194)
* [fix some issues around ZST handling](https://github.com/rust-lang/rust/pull/115277)
* [fix the repetitive word](https://github.com/rust-lang/rust/pull/115441)
* [inline functions called from `add_coverage`](https://github.com/rust-lang/rust/pull/115058)
* [interpret: fix projecting into an unsized field of a local](https://github.com/rust-lang/rust/pull/114483)
* [lower `Or` pattern without allocating place](https://github.com/rust-lang/rust/pull/111752)
* [make SIGSEGV handler emit nicer backtraces](https://github.com/rust-lang/rust/pull/113565)
* [make `get_return_block()` return `Some` only for HIR nodes in body](https://github.com/rust-lang/rust/pull/115313)
* [make `termcolor` types public in `rustc_errors`](https://github.com/rust-lang/rust/pull/115393)
* [make it more clear what 'Tier 2' (without host tools) means](https://github.com/rust-lang/rust/pull/115472)
* [miri ABI check: fix handling of 1-ZST; don't accept sign differences](https://github.com/rust-lang/rust/pull/115411)
* [miri ABI compatibility check: accept u32 and i32](https://github.com/rust-lang/rust/pull/115182)
* [miri function ABI check: accept `repr(transparent)` wrappers as compatible](https://github.com/rust-lang/rust/pull/115374)
* [miri/diagnostics: don't forget to `print_backtrace` when ICEing on unexpected errors](https://github.com/rust-lang/rust/pull/115272)
* [miri: add '--skip-children' to rustfmt invocation](https://github.com/rust-lang/miri/pull/3048)
* [miri: make sure we test all tier 1 targets](https://github.com/rust-lang/miri/pull/3045)
* [encode `DepKind` as `u16`](https://github.com/rust-lang/rust/pull/115391)
* [do not compute unneeded query results](https://github.com/rust-lang/rust/pull/114908)
* [use `OnceLock` for `SingleCache`](https://github.com/rust-lang/rust/pull/115422)
* [stabilize `os_str_bytes` feature](https://github.com/rust-lang/rust/pull/115443)
* [`impl TryFrom<char> for u16`](https://github.com/rust-lang/rust/pull/114065)
* [const-stabilize `is_ascii`](https://github.com/rust-lang/rust/pull/115449)
* [optimize `Take::{fold, for_each}` when wrapping `TrustedRandomAccess` iterators](https://github.com/rust-lang/rust/pull/115273)
* [implement `Step` for `ascii::Char`](https://github.com/rust-lang/rust/pull/113295)
* [make `Cell::swap` panic if the Cells partially overlap](https://github.com/rust-lang/rust/pull/114795)
* [hashbrown: fix two bugs in `clone_from`](https://github.com/rust-lang/hashbrown/pull/465)
* [hashbrown: migrate from `actions-rs/toolchain@v1` to `dtolnay/rust-toolchain@master` and remove `Cross.toml`](https://github.com/rust-lang/hashbrown/pull/467)
* [regex-automata: reduce regex contention considerably](https://github.com/rust-lang/regex/pull/1080)
* [regex: upgrade to memchr 2.6 to bring in aarch64 improvements](https://github.com/rust-lang/regex/pull/1079)
* [codegen\_gcc: add support for Link-Time Optimization](https://github.com/rust-lang/rustc_codegen_gcc/pull/278)
* [codegen\_gcc: add support for `noalias` function parameters](https://github.com/rust-lang/rustc_codegen_gcc/pull/312)
* [codegen\_gcc: only apply NoAlias attribute if optimization is enabled](https://github.com/rust-lang/rustc_codegen_gcc/pull/324)
* [cargo install: suggest --git when package name is url](https://github.com/rust-lang/cargo/pull/12575)
* [cargo cli: Help users know possible `--target` values](https://github.com/rust-lang/cargo/pull/12607)
* [cargo lints: Fail when overriding inherited lints](https://github.com/rust-lang/cargo/pull/12584)
* [cargo doc: adjust all doc headings one level up](https://github.com/rust-lang/cargo/pull/12595)
* [cargo fewer temporary needless strings](https://github.com/rust-lang/cargo/pull/12604)
* [cargo: add error for unsupported credential provider version](https://github.com/rust-lang/cargo/pull/12590)
* [cargo: improve logout message for asymmetric tokens](https://github.com/rust-lang/cargo/pull/12587)
* [cargo: prepare for partial-version package specs](https://github.com/rust-lang/cargo/pull/12591)
* [cargo: set tracing target for networking messages](https://github.com/rust-lang/cargo/pull/12582)
* [cargo: test: new options of debuginfo are no longer unstable](https://github.com/rust-lang/cargo/pull/12618)
* [rustdoc: Fix type based search](https://github.com/rust-lang/rust/pull/115436)
* [rustdoc: correctly deal with self ty params when eliding default object lifetimes](https://github.com/rust-lang/rust/pull/115276)
* [clippy: `implied_bounds_in_impls`: don't ICE on default generic parameter and move to nursery](https://github.com/rust-lang/rust-clippy/pull/11437)
* [clippy: add config flag for reborrows in `explicit_iter_loop`](https://github.com/rust-lang/rust-clippy/pull/11418)
* [clippy: correctly format `vec!` invocations](https://github.com/rust-lang/rust-clippy/pull/11430)
* [clippy: defaultUnionRepresentation: explain why we only warn about unions with at least 2 non-ZST fields](https://github.com/rust-lang/rust-clippy/pull/11448)
* [clippy: don't pass extra generic arguments in `needless_borrow`](https://github.com/rust-lang/rust-clippy/pull/11259)
* [clippy: fix span when linting `explicit_auto_deref` immediately after `needless_borrow`](https://github.com/rust-lang/rust-clippy/pull/11376)
* [clippy: new lint: `iter_out_of_bounds`](https://github.com/rust-lang/rust-clippy/pull/11396)
* [clippy: new lint: `missing_asserts_for_indexing`](https://github.com/rust-lang/rust-clippy/pull/10692)
* [clippy: rewrite `never_loop` as a strict reachability pass](https://github.com/rust-lang/rust-clippy/pull/11447)
* [clippy: `never_loop` catches `loop { panic!() }`](https://github.com/rust-lang/rust-clippy/pull/11450)
* [clippy: skip `todo!()` in `never_loop`](https://github.com/rust-lang/rust-clippy/pull/11455)
* [clippy: use multipart suggestions for raw string lints](https://github.com/rust-lang/rust-clippy/pull/11416)
* [rustfmt: fix issues with formatting imports with comments](https://github.com/rust-lang/rustfmt/pull/5853)
* [rustfmt: fix: reject leading `.`, `)` without prefix as item marker](https://github.com/rust-lang/rustfmt/pull/5839)

### Rust Compiler Performance Triage


A lot of spurious noise this week from a few benchmarks (bitmaps-3.1.0, libc,
and even cargo-0.60.0 at a few points). Beyond that, we had a few small
improvements associated with the trait system and with parallel-rustc.

Triage done by **@pnkfelix**.
Revision range: [cedbe5c7..15e52b05](https://perf.rust-lang.org/?start=cedbe5c715c1fa9359683c5f108bed2054ac258b&end=15e52b05ca8f63e0da27c808680388717e5b997e&absolute=false&stat=instructions%3Au)

4 Regressions, 7 Improvements, 8 Mixed; 2 of them in rollups
66 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-09-05.md)

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
* [disposition: merge] [rustdoc-search: add support for type parameters](https://github.com/rust-lang/rust/pull/112725)
* [disposition: merge] [Enable -Zdrop-tracking-mir by default](https://github.com/rust-lang/rust/pull/107421)
* [disposition: merge] [Stabilize `PATH` option for `--print KIND=PATH`](https://github.com/rust-lang/rust/pull/114183)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Pre-RFC describing mechanism to remove orphan rule constraints](https://github.com/rust-lang/rfcs/pull/3482)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-09-06 - 2023-10-04 🦀

### Virtual

* 2023-09-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294049877)
* 2023-09-06 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/294343596/)
* 2023-09-07 | Virtual (Ann Arbor, MI, US) | [Michigan Python](https://www.meetup.com/michigan-python/)
    * [**Online MI Python: Improving Python Speed with a Bit of Rust**](https://www.meetup.com/michigan-python/events/294951180/)
* 2023-09-12 - 2023-09-15 | Virtual (Albuquerque, NM, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295635473/)
* 2023-09-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/gqdlgtyfcmbqb/)
* 2023-09-13 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/295011539)
* 2023-09-13 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-09-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732655)
* 2023-09-20 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**SurrealDB for Rustaceans**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/295826608/)
* 2023-09-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057154/)
* 2023-09-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295666673/)
* 2023-09-21 | Virtual (Cologne, DE) | [Cologne AWS User Group #AWSUGCGN](https://www.meetup.com/aws-cologne/)
    * [**AWS User Group Cologne - September Edition: Stefan Willenbrock: Developer Preview: Discovering Rust on AWS**](https://www.meetup.com/aws-cologne/events/294594401/)
* 2023-09-21 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Real Time Multiplayer Game Server in Rust**](https://www.meetup.com/utah-rust/events/294972877/)
* 2023-09-21 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 33rd Edition**](https://www.meetup.com/rust-linz/events/295363887/)
* 2023-09-21 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/295828383/)
* 2023-09-25 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**How we built the SurrealDB Python client in Rust.**](https://www.meetup.com/Rust-Dublin/events/294908596/)
* 2023-09-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcmbjc/)
* 2023-10-03 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295919493/)
* 2023-10-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)

### Asia

* 2023-10-03 | Taipei, TW | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/)
    * [**WebAssembly Meetup (Wasm Empowering AI) in Taipei**](https://www.meetup.com/wasm-rust-meetup/events/295672575/)

### Europe

* 2023-09-12 | Berlin, DE | [Berlin AWS User Group](https://www.meetup.com/berlinawsug/)
    * [**Berlin AWS Group Meetup - September 2023: Luca Zonta // Sustainable Serverless Computing with Rust**](https://www.meetup.com/berlinawsug/events/295565048/)
* 2023-09-12 | Zurich, CH| [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**The Lightning Rust Quiz - September Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/295804450/)
* 2023-09-13 | Cologne, DE | [Rust User Group Cologne](https://rust.cologne/2023/09/13/rare-rust.html)
    * [**Rare Rust**](https://www.meetup.com/rustcologne/events/295869748/) | [**Group Detail Page**](https://rust.cologne/2023/09/13/rare-rust.html)
* 2023-09-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295109905/)
* 2023-09-15 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**OnsiteMeeting**](https://www.meetup.com/rust-community-stuttgart/events/295639296/)
* 2023-09-15 | Tiel, NL | [Rust, Getting Started](https://www.meetup.com/rust-getting-started/)
    * [**Rust Workshop - 2**](https://www.meetup.com/rust-getting-started/events/295880062/)
* 2023-09-16 | Brussels, BE | [HSBXL](https://hsbxl.be/events/software-freedom-day/2023-09-16/)
    * [**Software Freedom Day 2023**](https://www.meetup.com/brussels-hackerspace/events/295912633/)
* 2023-09-19 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Logging and tracing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504245/)
* 2023-09-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus - Rust and Talk at Concordium**](https://www.meetup.com/rust-aarhus/events/294031975/)
* 2023-09-21 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2023 🦀**](https://www.meetup.com/rust-bern/events/295503351/)
* 2023-09-26 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679767/)
* 2023-09-28 | Berlin, DE | [React Berlin](https://www.meetup.com/react-berlin-meetup/)
    * [**React Berlin September Meetup: Creating Videos with React & Remotion & More: Integrating Rust with React Native – Gheorghe Pinzaru**](https://www.meetup.com/react-berlin-meetup/events/295382108/)

### North America

* 2023-09-06 | Bellevue, WA, US | [The Linux Foundation](https://www.linuxfoundation.org/)
    * [**Rust Global**](https://events.linuxfoundation.org/rust-global/)
* 2023-09-07 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/295614871/) | [**Mountain View Rust Meetup Mirror**](https://www.meetup.com/mv-rust-meetup/events/295746992/)
* 2023-09-07 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Weekly Pasadena Python study group**](https://www.meetup.com/thursday-go/events/295818856/)
* 2023-09-09 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295572737/)
* 2023-09-12 - 2023-09-15 | Albuquerque, NM, US  + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**A Panel Discussion on Thriving in a Rust-Driven Workplace**](https://www.meetup.com/rust-nyc/events/295639294)
* 2023-09-12 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/295744114/)
* 2023-09-14 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295484105)
* 2023-09-16 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295579189/)
* 2023-09-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/295545278)
* 2023-09-21 | Mountain View, CA, US| [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295747006/)
* 2023-09-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust on the web! Get started with Leptos**](https://www.meetup.com/music-city-rust-developers/events/295587220/)
* 2023-09-23 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295869150/)
* 2023-09-26 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/295771515)
* 2023-09-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295466314)
* 2023-09-30 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/fktvgtyfcmbnc/)

### Oceania

* 2023-09-13 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**Rust Meetup 2: Lunch & Learn**](https://www.linkedin.com/events/7097356771584880640/) | [**Ticket Link**](https://www.tickettailor.com/events/perthrustusergroup/984771)
* 2023-09-19 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/295602231/)
* 2023-09-26 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/295432237/)

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

> Rusts standard library, and a lot of the popular crates, are like a museum. While it does change, as new exhibitions are added, it is mostly finished. Each painting has a detailed explanation in 7 different languages underneath. Descriptions below each excitation are written beautifully, with detailed drawings, showing how everything works. It is so easy to navigate, one glance at the map is enough to find exactly what you are looking for. It is so convenient, you almost don't notice that you are learning something.
>
> Internals of `rustc` are like a build site of a sprawling factory. You can see the scaffolds everywhere, as more production lines come online, and everything gets faster, better, bigger. Workers move around, knowing the place like the back of their hands. They can glance at the signs on the walls, and instantly tell you: where you are, what this place does and what pitfalls you should avoid. And you are a new hire who has just came for his first day at the new job. You look at the sign, and after some thinking, you too are able to tell roughly in which building you are. The signs almost always tell you what you need, just in short, cryptic sentences. You always can tell what is going on, with some thinking, but it is not effortless. The signs on the walls are not *bad*, just not written for anyone to get right away.

– [FractalFir on their blog](https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_0_1.html)

Thanks to [Alona Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1464) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
