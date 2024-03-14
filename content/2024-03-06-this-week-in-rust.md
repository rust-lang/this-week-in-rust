Title: This Week in Rust 537
Number: 537
Date: 2024-03-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official
* [Clippy: Deprecating `feature = "cargo-clippy"`](https://blog.rust-lang.org/2024/02/28/Clippy-deprecating-feature-cargo-clippy.html)
* [Updated baseline standards for Windows targets](https://blog.rust-lang.org/2024/02/26/Windows-7.html)

### Project/Tooling Updates
* [Replacing PyO3's API without breaking everything downstream](https://polar.sh/davidhewitt/posts/replacing-pyo3-api-pt1)
* [rust-analyzer changelog #223](https://rust-analyzer.github.io/thisweek/2024/03/04/changelog-223.html)
* [rust-gcc February 2024 Monthly report](https://rust-gcc.github.io/2024/03/05/2024-02-monthly-report.html)
* [godot-rust February 2024 dev update](https://godot-rust.github.io/dev/february-2024-update/)
* [Fluvio v0.11.5 Release Updates](https://www.fluvio.io/news/this-week-in-fluvio-0059/)
* [sshd-openpgp-auth version 0.3.0](https://crates.io/crates/sshd-openpgp-auth) and [ssh-openpgp-auth version 0.2.2](https://crates.io/crates/ssh-openpgp-auth) as updates to the tooling providing [OpenPGP based authentication for SSH host keys](https://codeberg.org/wiktor/ssh-openpgp-auth)

### Observations/Thoughts
* [Borrow checking without lifetimes](https://smallcultfollowing.com/babysteps/blog/2024/03/04/borrow-checking-without-lifetimes/)
* [How to speed up the Rust compiler in March 2024](https://nnethercote.github.io/2024/03/06/how-to-speed-up-the-rust-compiler-in-march-2024.html)
* [Code review in the Rust compiler](https://nnethercote.github.io/2024/03/05/code-review-in-the-rust-compiler.html)
* [Rust `thread_local!`s are surprisingly expensive](https://swatinem.de/blog/slow-thread-local/)
* [Designing an Async Runtime for WASI 0.2](https://blog.yoshuawuyts.com/building-an-async-runtime-for-wasi/)

### Rust Walkthroughs
* [Storing borrowed data in trait objects](https://ferrous-systems.com/blog/rustls-borrow-checker-p3/)
* [Rust's early vs. late lifetime binding](https://blog.the-pans.com/rusts-early-vs-late-lifetime-binding/)
* [How moving from Pandas to Polars made me write better code without writing better code](https://dev.to/check/how-moving-from-pandas-to-polars-made-me-write-better-code-without-writing-better-code-52bl)
* [A Full Stack SaaS Template with Loco](https://www.shuttle.rs/blog/2024/02/29/fullstack-loco-rust)
* [Async Rust in a Nutshell](https://www.shuttle.rs/blog/2024/02/29/async-rust)
* [Real-time Streaming Analytics with Fluvio, DeepCausality, and Rust](https://infinyon.com/blog/2024/02/fluvio-deep-causality-rs/)
* [video] [Modern All Rust Stack - Dioxus, Axum, Warp, SurrealDB](https://www.youtube.com/watch?v=Pr6T0Phjvgc)
* [video] [Serverless Data Pipelines in Rust by Michele Vigilante](https://www.youtube.com/watch?v=PK_FKzgPDWg)
* [[FR] [video] Rust Lyon Meetup #8 - Impl Snake for Micro:bit — Cyril MARPAUD](https://www.youtube.com/watch?v=8_Pj6q_mVQw)

### Miscellaneous

* [Czech Rust community index](https://rustlang.cz/)
* [Launching RustRover: JetBrains’ Investment in Rust](https://mainmatter.com/blog/2024/02/29/launching-rustrover/)
* [audio] [RustShip: Rust in Art with Lisa Passing](https://ieni.dev/2024/03/%EF%B8%8F-rust-in-art-with-lisa-passing-rustship-6/)

## Crate of the Week

This week's crate is [srgn](https://github.com/alexpovel/srgn), a mix of tr, sed, rip-grep and tree-sitter.

Thanks to [Alex Povel](https://users.rust-lang.org/t/crate-of-the-week/2704/1294) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Output for both `ockam project ticket` and `ockam project enroll` is improved, with support for `--output json`](https://github.com/build-trust/ockam/issues/7473)
* [Ockam - Syntax highlighting for fenced code blocks, in command help output, on Linux works](https://github.com/build-trust/ockam/issues/7471)
* [Ockam - Command - refactor to use typed interfaces to implement commands for `kafka services`](https://github.com/build-trust/ockam/issues/6706)
* [ZeroCopy - Don't generate warning when deriving on deprecated type](https://github.com/google/zerocopy/issues/553)
* [ZeroCopy - Test the output of zerocopy-derive](https://github.com/google/zerocopy/issues/367)
* [ZeroCopy - [CI] Check semver compatibility with all target platforms, not just the host platform](https://github.com/google/zerocopy/issues/357)
* [ZeroCopy - Inline many trait methods (in zerocopy and in derive-generated code)](https://github.com/google/zerocopy/issues/7)
* [Fluvio - fvm switch fails on some systems with running local cluster](https://github.com/infinyon/fluvio/issues/3765)
* [Fluvio - Add new command fluvio cluster resume](https://github.com/infinyon/fluvio/issues/3810)

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [Oxidize 2024](https://oxidizeconf.com/) [CFP](https://pretalx.com/oxidize-berlin-2024/cfp) closes 2024-03-24 | Berlin, Germany | Event date: 2024-05-28 - 2024-05-30
* [RustConf 2024](https://foundation.rust-lang.org/news/the-rustconf-2024-call-for-talk-proposals-is-open/) | CFP closes 2024-04-25 | Montreal, Canada | 2024-09-10
* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| CFP closes 2024-06-03 | Vienna, Austria & online | Event on 2024-10-10

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

488 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-02-27..2024-03-05

* [`ffi_unwind_calls`: treat RustIntrinsic like regular Rust calls](https://github.com/rust-lang/rust/pull/121548)
* [`pattern_analysis`: rework how we hide empty private fields](https://github.com/rust-lang/rust/pull/121000)
* [rustc: fix wasm64 metadata object files](https://github.com/rust-lang/rust/pull/121464)
* [add a proper `with_no_queries` to printing](https://github.com/rust-lang/rust/pull/121927)
* [add a way to add constructors for `rustc_type_ir` types](https://github.com/rust-lang/rust/pull/121703)
* [add initial support for DataFlowSanitizer](https://github.com/rust-lang/rust/pull/120761)
* [add new `pattern_complexity` attribute to add possibility to limit and check recursion in pattern matching](https://github.com/rust-lang/rust/pull/121917)
* [add platform-specific function to get the error number for HermitOS](https://github.com/rust-lang/rust/pull/121765)
* [add profiling support to AIX](https://github.com/rust-lang/rust/pull/121730)
* [add proper cfg to keep only one AlignmentEnum definition for different `target_pointer_widths`](https://github.com/rust-lang/rust/pull/121753)
* [allow statics pointing to mutable statics](https://github.com/rust-lang/rust/pull/121782)
* [always generate GEP i8 / ptradd for `struct` offsets](https://github.com/rust-lang/rust/pull/121665)
* [avoid collecting into vecs in some places](https://github.com/rust-lang/rust/pull/121895)
* [avoid using unnecessary queries when printing the query stack in panics](https://github.com/rust-lang/rust/pull/121993)
* [consider middle segments of paths in `unused_qualifications`](https://github.com/rust-lang/rust/pull/121528)
* [correctly generate item info of trait items](https://github.com/rust-lang/rust/pull/121855)
* [delete line if suggestion would replace it with an empty line](https://github.com/rust-lang/rust/pull/120305)
* [display short types for unimplemented trait](https://github.com/rust-lang/rust/pull/121739)
* [don't grab variances in `TypeRelating` relation if we're invariant](https://github.com/rust-lang/rust/pull/121864)
* [don't panic when waiting on poisoned queries](https://github.com/rust-lang/rust/pull/121913)
* [fix incorrect suggestion for uninitialized binding in pattern](https://github.com/rust-lang/rust/pull/120646)
* [fix issues in suggesting importing extern crate paths](https://github.com/rust-lang/rust/pull/121226)
* [fix link generation for foreign macro in jump to definition feature](https://github.com/rust-lang/rust/pull/121680)
* [implement missing ABI structures in StableMIR](https://github.com/rust-lang/rust/pull/121824)
* [improve renaming suggestion when item starts with underscore](https://github.com/rust-lang/rust/pull/121792)
* [made `INVALID_DOC_ATTRIBUTES` lint deny by default](https://github.com/rust-lang/rust/pull/111505)
* [make `ZeroablePrimitive` trait unsafe](https://github.com/rust-lang/rust/pull/121850)
* [make the success arms of `if lhs || rhs` meet up in a separate block](https://github.com/rust-lang/rust/pull/121784)
* [match lowering: Lower bindings in a predictable order](https://github.com/rust-lang/rust/pull/121716)
* [match lowering: Separate the `bool` case from other integers in `TestKind`](https://github.com/rust-lang/rust/pull/121750)
* [match lowering: pre-simplify or-patterns too](https://github.com/rust-lang/rust/pull/121715)
* [never say `"`Trait is implemented for {type error}`"`](https://github.com/rust-lang/rust/pull/121803)
* [normalizes-to: handle negative impls](https://github.com/rust-lang/rust/pull/121853)
* [opportunistically resolve regions when processing region outlives obligations](https://github.com/rust-lang/rust/pull/121743)
* [pattern analysis: Don't panic when encountering unexpected constructor](https://github.com/rust-lang/rust/pull/121735)
* [pattern analysis: abort on arity mismatch](https://github.com/rust-lang/rust/pull/121987)
* [preserve same vtable pointer when cloning raw waker, to fix `Waker::will_wake`](https://github.com/rust-lang/rust/pull/121622)
* [process alias-relate obligations in CoerceUnsized loop](https://github.com/rust-lang/rust/pull/121702)
* [properly deal with GATs when looking for method chains to point at](https://github.com/rust-lang/rust/pull/121912)
* [safe Transmute: Revise safety analysis](https://github.com/rust-lang/rust/pull/121681)
* [skip unnecessary comparison with half-open range patterns](https://github.com/rust-lang/rust/pull/121376)
* [split `rustc_type_ir` to avoid `rustc_ast` from depending on it](https://github.com/rust-lang/rust/pull/121695)
* [style library/core/src/error.rs](https://github.com/rust-lang/rust/pull/121888)
* [suggest moving definition if non-found `macro_rules!` is defined later](https://github.com/rust-lang/rust/pull/121130)
* [suggest removing superfluous semicolon when statements used as expression](https://github.com/rust-lang/rust/pull/121153)
* [the ordinary lowering of `thir::ExprKind::Let` is unreachable](https://github.com/rust-lang/rust/pull/121892)
* [use volatile access instead of `#[used]` for `on_tls_callback`](https://github.com/rust-lang/rust/pull/121596)
* [miri: add -Zmiri-track-alloc-accesses to readme and fix its wording](https://github.com/rust-lang/miri/pull/3346)
* [miri: log when we change the active thread, and fix logging for concurrency](https://github.com/rust-lang/miri/pull/3348)
* [miri: print thread name in miri error backtraces; add option to track read/write accesses](https://github.com/rust-lang/miri/pull/3338)
* [miri: tree Borrows diagnostic improvements](https://github.com/rust-lang/miri/pull/3343)
* [miri: windows: support getting the thread name](https://github.com/rust-lang/miri/pull/3345)
* [add ASCII fast-path for `char::is_grapheme_extended`](https://github.com/rust-lang/rust/pull/121138)
* [perf: improve `write_fmt` to handle simple strings](https://github.com/rust-lang/rust/pull/121001)
* [add `display` method to `OsStr`](https://github.com/rust-lang/rust/pull/120051)
* [have `String` use `SliceIndex` impls from `str`](https://github.com/rust-lang/rust/pull/120291)
* [use the OS thread name by default if `THREAD_INFO` has not been initialized](https://github.com/rust-lang/rust/pull/121666)
* [add missing `get_name` for `wasm::thread`](https://github.com/rust-lang/rust/pull/121933)
* [remove `Mutex::unlock` Function](https://github.com/rust-lang/rust/pull/121736)
* [implement unwind safety for Condvar on all platforms](https://github.com/rust-lang/rust/pull/121768)
* [make `ReentrantLock` public](https://github.com/rust-lang/rust/pull/110543)
* [codegen\_gcc: debuginfo: Add support for debuginfo, without scope support](https://github.com/rust-lang/rustc_codegen_gcc/pull/455)
* [codegen\_gcc: switch to the new `set_special_chars_allowed_in_func_names` API](https://github.com/rust-lang/rustc_codegen_gcc/pull/462)
* [cargo add: Fallback to `rustc -v` when no MSRV is set](https://github.com/rust-lang/cargo/pull/13516)
* [cargo toml: Warn on unset Edition](https://github.com/rust-lang/cargo/pull/13505)
* [cargo msrv: Report all incompatible packages, not just a random one](https://github.com/rust-lang/cargo/pull/13514)
* [cargo rustc: Always pass --edition to rustc](https://github.com/rust-lang/cargo/pull/13499)
* [cargo toml: Don't warn on unset Edition if only 2015 is compatible](https://github.com/rust-lang/cargo/pull/13533)
* [cargo: add all unit's children recursively for `doc.extern-map` option](https://github.com/rust-lang/cargo/pull/13481)
* [cargo: add "-Zpublic-dependency" for public-dependency feature](https://github.com/rust-lang/cargo/pull/13340)
* [cargo: silently ignore `cargo::rustc-check-cfg` to avoid MSRV annoyance when stabilizing `-Zcheck-cfg`](https://github.com/rust-lang/cargo/pull/13438)
* [cargo: stabilize global cache data tracking](https://github.com/rust-lang/cargo/pull/13492)
* [rustdoc: Prevent inclusion of whitespace character after `macro_rules` ident](https://github.com/rust-lang/rust/pull/121689)
* [rustfmt: ensure space around binary exprs](https://github.com/rust-lang/rustfmt/pull/6085)
* [clippy: `identity_op`: Fix duplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12409)
* [clippy: `let_underscore_untyped`: fix false positive on async function](https://github.com/rust-lang/rust-clippy/pull/12400)
* [clippy: `map_entry`: Check insert expression for map use](https://github.com/rust-lang/rust-clippy/pull/12362)
* [clippy: `misrefactored_assign_op`: fix duplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12413)
* [clippy: `redundant_closure_call`: don't lint closure originating from a macro](https://github.com/rust-lang/rust-clippy/pull/12380)
* [clippy: `unnecessary_cast`: avoid breaking precedence](https://github.com/rust-lang/rust-clippy/pull/12365)
* [clippy: add `assigning_clones` lint](https://github.com/rust-lang/rust-clippy/pull/12077)
* [clippy: add `mixed_attributes_style` lint](https://github.com/rust-lang/rust-clippy/pull/12354)
* [clippy: added msrv to threadlocal initializer check](https://github.com/rust-lang/rust-clippy/pull/12405)
* [clippy: check for try blocks in `question_mark` more consistently](https://github.com/rust-lang/rust-clippy/pull/12341)
* [clippy: dedup `std_instead_of_core` by using first segment span for uniqueness](https://github.com/rust-lang/rust-clippy/pull/12406)
* [clippy: don't emit "missing backticks" lint if the element is wrapped in `<code>` HTML tags](https://github.com/rust-lang/rust-clippy/pull/12423)
* [clippy: fix false positive in `threadlocal!` when falling back to `os_local`](https://github.com/rust-lang/rust-clippy/pull/12276)
* [clippy: fix `derive_partial_eq_without_eq` false positive on trait projection](https://github.com/rust-lang/rust-clippy/pull/12393)
* [clippy: fix `nonminimal_bool` lint regression](https://github.com/rust-lang/rust-clippy/pull/12372)
* [clippy: fix `manual_memcpy` wrong indexing for multi dimensional arrays](https://github.com/rust-lang/rust-clippy/pull/12010)
* [clippy: handle plural acronyms in `doc_markdown`](https://github.com/rust-lang/rust-clippy/pull/12419)
* [clippy: improve `is_lint_level` code](https://github.com/rust-lang/rust-clippy/pull/12375)
* [clippy: lower `bstr` version requirement to `1.6.0`](https://github.com/rust-lang/rust-clippy/pull/12363)
* [clippy: pointers cannot be converted to integers at compile time](https://github.com/rust-lang/rust-clippy/pull/12403)
* [rust-analyzer: add hover display for trait assoc items](https://github.com/rust-lang/rust-analyzer/pull/15938)
* [rust-analyzer: add basic support for native debug](https://github.com/rust-lang/rust-analyzer/pull/16719)
* [rust-analyzer: autocomplete constants inside format strings](https://github.com/rust-lang/rust-analyzer/pull/16723)
* [rust-analyzer: don't destructure `struct` with no public fields](https://github.com/rust-lang/rust-analyzer/pull/16752)
* [rust-analyzer: don't highlight related assoc items of super traits](https://github.com/rust-lang/rust-analyzer/pull/16727)
* [rust-analyzer: goto definition for `deref_mut`](https://github.com/rust-lang/rust-analyzer/pull/16696)
* [rust-analyzer: goto definition for `index_mut`](https://github.com/rust-lang/rust-analyzer/pull/16709)
* [rust-analyzer: goto-definition for constants inside range pattern](https://github.com/rust-lang/rust-analyzer/pull/16759)
* [rust-analyzer: ignore generic arguments in intra doc link path resolution](https://github.com/rust-lang/rust-analyzer/pull/16702)
* [rust-analyzer: put style lints behind disabled-by-default config](https://github.com/rust-lang/rust-analyzer/pull/16757)
* [rust-analyzer: fix rust-project.json projects not preferring sysroot rustc](https://github.com/rust-lang/rust-analyzer/pull/16693)
* [rust-analyzer: fix wrong closure kind deduction for closures with predicates](https://github.com/rust-lang/rust-analyzer/pull/16630)
* [futures: parse rhs of `select!` arms using match-arm rules](https://github.com/rust-lang/futures-rs/pull/2832)

### Rust Compiler Performance Triage

A bunch of noise this week which has been dropped from the report (but may be
present in the summary figures). As a result, the week is pretty busy in amount
of changes, but the net effect is nearly neutral to a slight regression for
most workloads.

Triage done by **@simulacrum**.
Revision range: [71ffdf7..41d97c8](https://perf.rust-lang.org/?start=71ffdf7ff7ac6df5f9f64de7e780b8345797e8a0&end=41d97c8a5dea2731b0e56fe97cd7cb79e21cff79&absolute=false&stat=instructions%3Au)

2 Regressions, 0 Improvements, 10 Mixed; 4 of them in rollups
51 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-03-05.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: patchable-function-entry](https://github.com/rust-lang/rfcs/pull/3543)
* [RFC: Add native code coverage support in Cargo](https://github.com/rust-lang/rfcs/pull/3287)
* [RFC: Packages as (optional) namespaces](https://github.com/rust-lang/rfcs/pull/3243)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [alloc: implement `FromIterator` for `Box<str>`](https://github.com/rust-lang/rust/pull/99969)
* [disposition: merge] [rework opaque type region inference](https://github.com/rust-lang/rust/pull/116891)
* [disposition: merge] [Make `impl<Fd: AsFd>` impl take `?Sized`](https://github.com/rust-lang/rust/pull/114655)
* [disposition: merge] [Tracking issue for Allow a re-export for `main` (RFC 1260)](https://github.com/rust-lang/rust/issues/28937)
* [disposition: merge] [Update Windows platform support ](https://github.com/rust-lang/rust/pull/115141)
* [disposition: close] [Resolve region bounds from components of type projection](https://github.com/rust-lang/rust/pull/121602)
* [disposition: merge] [Propagate temporary lifetime extension into if and match.](https://github.com/rust-lang/rust/pull/121346)
* [disposition: merge] [more eagerly instantiate binders](https://github.com/rust-lang/rust/pull/119849)
* [disposition: merge] [`E0492: borrow of an interior mutable value may end up in the final value` during const eval when no inner mutability is involved](https://github.com/rust-lang/rust/issues/121250)
* [disposition: merge] [align_offset, align_to: no longer allow implementations to spuriously fail to align](https://github.com/rust-lang/rust/pull/121201)
* [disposition: merge] [Soft-destabilize `RustcEncodable` & `RustcDecodable`, remove from prelude in next edition](https://github.com/rust-lang/rust/pull/116016)
* [disposition: merge] [`impl From<TryReserveError>` for `io::Error`](https://github.com/rust-lang/rust/pull/121403)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for rustc `--check-cfg` integration](https://github.com/rust-lang/cargo/issues/10554)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Public Key Infrastructure for Rust Project](https://github.com/rust-lang/rfcs/pull/3579)

## Upcoming Events

Rusty Events between 2024-03-06 - 2024-04-03 🦀

### Virtual

* 2024-03-06 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**An intro to `nom`, parsing made easy for Rustaceans**](https://www.meetup.com/rust-dublin/events/299358988/)
* 2024-03-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047891/)
* 2024-03-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)
* 2024-03-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341582/)
* 2024-03-12 | Hybrid (Virtual + In-person) Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-13 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 3 - Designing Interfaces**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299505703/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Web Frontend Co-Learning (online)**](https://www.meetup.com/opentechschool-berlin/events/298406445/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457903/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/03/14/rust-hack-and-learn.html)
* 2024-03-14 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945252/)
* 2024-03-19 | Virtual (Washinigton, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299335006/)
* 2024-03-20 | Virtual (Vancouver, BC, CA)| [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763494/)
* 2024-03-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368793/)
* 2024-03-26 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Stream](https://www.youtube.com/@bcnrust)
* 2024-03-28 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457904/)
* 2024-04-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/mrnrktygcgbdb/)
* 2024-04-03 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 4 - Error Handling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299507234/)
* 2024-04-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047892/)

### Asia

* 2024-03-09 | Karnataka, Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**March 2024 Rustacean Meetup**](https://hasgeek.com/rustbangalore/march-2024-rustacean-meetup/)
* 2024-03-12 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Building the Isograph Compiler in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/299607311/)

### Europe

* 2024-03-06 | Cologne / Köln, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Macros by simple Examples**](https://www.meetup.com/rustcologne/events/299530888/)
* 2024-03-06 | Zürich, CH | [Rust Zürisee](https://www.meetup.com/rust-zurich/)
    * [**How to (partial) Migration - March Meetup**](https://www.meetup.com/rust-zurich/events/299380190/)
* 2024-03-07 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust Hack Night #3: Embedded on Espressif's ESP32C3**](https://www.meetup.com/copenhagen-rust-community/events/299451605/)
* 2024-03-12 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-13 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.com/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-830340830777)
* 2024-03-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/298533419/)
* 2024-03-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 2024-03-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Rust Interactive Session**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/299309224/)
* 2024-03-19 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/299515169/)
    * [**Rust Meetup @ Charles University**](https://www.meetup.com/rust-prague/events/299515169/)
* 2024-03-20 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introduction to programming Microcontrollers with Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 2024-03-20 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #9**](https://www.meetup.com/fr-FR/rust-lyon/events/299527560/)
* 2024-03-21 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #6**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/299354449/)
* 2024-03-21 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #6: Du RSS et de L'ECS !**](https://www.meetup.com/meetup-group-zgphbyet/events/299295547/)
* 2024-03-26 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-27 - 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024 - Conference**](https://www.rustnationuk.com/)
* 2024-03-28 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell**](https://www.meetup.com/rust-berlin/events/299288961/)

### North America

* 2024-03-07 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)
* 2024-03-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Meetup**](https://www.meetup.com/rust-nyc/events/299619615/)
* 2024-03-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Northeastern Rust Lunch**](https://www.meetup.com/bostonrust/events/299262009/)
* 2024-03-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186823/)
* 2024-03-21 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 2024-03-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299220136/)
* 2024-03-27 | Hawthorne, CA, US | [Freeform](https://freeform.co/)
    * [**Rust in the Physical World 🦀 Tech Talk Event at Freeform**](https://freeformxrust.rsvpify.com/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1arr8xi/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> My experience with C++ is that, as I’ve become more of an expert in the language, I’ve become more disillusioned with it. It’s incredibly hard to do things that you should be able to do in software. And, it’s a huge problem for me to constantly be helping other engineers debug the same bugs over and over. It’s always another use after free. I’ve probably debugged 300 of those. \[...\]
>
> In our experience using the Rust ecosystem for almost three years now, I don't think we found a bug in a single Rust crate that we've pulled off the shelf. We found a bug in one of them and that was a Rust crate wrapping a C library and the bug was in the C library. The software quality that you kind of get for free is amazing.

– [Carter Schultz interviewed on the filtra blog](https://filtra.io/rust-amp-feb-24)

Thanks to [George Barwood](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1543) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1b8o0ms/this_week_in_rust_537/)</small>
