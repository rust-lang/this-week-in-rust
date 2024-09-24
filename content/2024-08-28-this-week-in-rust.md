Title: This Week in Rust 562
Number: 562
Date: 2024-08-28
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
* [2024 Leadership Council Survey](https://blog.rust-lang.org/2024/08/26/council-survey.html)
* [Embedded Working Group Community Micro Survey](https://blog.rust-lang.org/inside-rust/2024/08/22/embedded-wg-micro-survey.html)

### Foundation
* [What's in Store at RustConf 2024](https://foundation.rust-lang.org/news/what-s-in-store-at-rustconf-2024/)

### Project/Tooling Updates
* [git-cliff 2.5.0 is released!](https://git-cliff.org/blog/2.5.0)
* [Ferrocene 24.08.0 now available!](https://ferrous-systems.com/blog/ferrocene-24-08-0/)
* [Go wild: Wildcard support in Rules and a new open-source wildcard crate](https://blog.cloudflare.com/wildcard-rules/)
* [Why am I writing a Rust compiler in C?](https://notgull.net/announcing-dozer/)
* [Current zlib-rs performance](https://tweedegolf.nl/en/blog/134/current-zlib-rs-performance)
* [Bon builder generator 2.0 release 🎉](https://elastio.github.io/bon/blog/bon-builder-generator-v2-release)
* [Meilisearch 1.10](https://blog.meilisearch.com/meilisearch-1-10/)

### Observations/Thoughts
* [Rust dylib rabbit holes](https://davidlattimore.github.io/posts/2024/08/27/rust-dylib-rabbit-holes.html)
* [I sped up serde_json strings by 20%](https://purplesyringa.moe/blog/i-sped-up-serde-json-strings-by-20-percent/)
* [Using StringView / German Style Strings to Make Queries Faster: Part 1 - Reading Parquet](https://www.influxdata.com/blog/faster-queries-with-stringview-part-one-influxdb/)
* [Rust as My First Language](https://www.shuttle.rs/blog/2024/08/23/rust-as-my-first-language)
* [Doctests - How were they improved?](https://blog.guillaume-gomez.fr/articles/2024-08-17+Doctests+-+How+were+they+improved%3F)
* [MiniJinja: Learnings from Building a Template Engine in Rust](https://lucumr.pocoo.org/2024/8/27/minijinja/)
* [Standards for use of unsafe Rust in the kernel](https://lwn.net/Articles/982868/)
* [Rust vs C++: A Real-World Perspective (interview with Tyler Weaver)](https://corrode.dev/blog/cpp-rust-interop/)
* [A Piece of UNIX History in Rust](https://howdytx.technology/unix-history-the-dc-calculator/)
* [Why Amazon, Cloudflare and Discord are building servers in Rust but you should probably not](https://kerkour.com/rust-is-bad-for-servers)

### Rust Walkthroughs
* [Let the API protect you](https://www.snoyman.com/blog/2024/08/let-the-api-protect-you/)
* [Packaging a Rust Program for Fedora](https://jrfernandez.com/packaging-rust-program-for-fedora)
* [Building a Bevy Plugin for Rolling Dice](https://blog.erikhorton.com/2024/08/25/building-a-bevy-plugin-for-rolling-dice.html)
* [video] [Implementing a Lox interpreter in Rust](https://www.youtube.com/watch?v=mNOLaw-_Buc)
* [video] [Command line tools: Implementing wc in Rust](https://www.youtube.com/watch?v=eQuSA9kSLAY)
* [video] [Explore Linux TTY, process, signals w/ Rust - Part 1/3 (background info)](https://www.youtube.com/watch?v=bolScvh4x7I)

### Research
* [On the Impact of Memory Safety on Fast Network I/O](https://ieeexplore.ieee.org/abstract/document/10635971)

### Miscellaneous
* [Can DARPA’s TRACTOR Pull C to Rust for Memory-Safe Overhaul?](https://thenewstack.io/can-darpas-tractor-pull-c-to-rust-for-memory-safe-overhaul/)
* [Crafting a Blockchain in Go and Rust: A Comparative Journey — Private keys, Public Keys and Signatures [Part 1]](https://hashblog.thepolyglotprogrammer.com/crafting-a-blockchain-in-go-and-rust-a-comparative-journey-private-keys-public-keys-and-signatures-part-1)
* [video] [A Multiplatform Mobile Navigation SDK with Rust at the Core](https://www.youtube.com/watch?v=WL0jY51PQR8)


## Crate of the Week

This week's crate is [wtx](https://github.com/c410-f3r/wtx), a batteries-included web application framework.

Thanks to [Caio](https://users.rust-lang.org/t/crate-of-the-week/2704/1333) for the self-suggestion!

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

429 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-08-20..2024-08-27

* [add Trusty OS as tier 3 target](https://github.com/rust-lang/rust/pull/129490)
* [CFI: erase regions when projecting ADT to its transparent non-1zst field](https://github.com/rust-lang/rust/pull/129179)
* [add missing module flags for CFI and KCFI sanitizers](https://github.com/rust-lang/rust/pull/129373)
* [`repr_transparent_external_private_fields`: special-case some std types](https://github.com/rust-lang/rust/pull/129487)
* [add a special case for `CStr`/`CString` in the `improper_ctypes` lint](https://github.com/rust-lang/rust/pull/128735)
* [avoid extra `cast()`s after `CStr::as_ptr()`](https://github.com/rust-lang/rust/pull/129332)
* [const checking: properly compute the set of transient locals](https://github.com/rust-lang/rust/pull/129508)
* [ctfe: make CompileTimeInterpCx type alias public](https://github.com/rust-lang/rust/pull/129309)
* [detect `*` operator on `!Sized` expression](https://github.com/rust-lang/rust/pull/128467)
* [do not ICE on non-ADT rcvr type when looking for crate version collision](https://github.com/rust-lang/rust/pull/129250)
* [don't consider locals to shadow inner items' generics](https://github.com/rust-lang/rust/pull/129270)
* [don't generate functions with the `rustc_intrinsic_must_be_overridden` attribute](https://github.com/rust-lang/rust/pull/129239)
* [don't trigger refinement lint if predicates reference errors](https://github.com/rust-lang/rust/pull/129417)
* [fix extern crates not being hidden with `doc(hidden)`](https://github.com/rust-lang/rust/pull/129414)
* [fix handling of macro arguments within the `dropping_copy_types` lint](https://github.com/rust-lang/rust/pull/129408)
* [implement `-Z embed-source` (DWARFv5 source code embedding extension)](https://github.com/rust-lang/rust/pull/126985)
* [improve diagnostic-related lints: `untranslatable_diagnostic` & `diagnostic_outside_of_impl`](https://github.com/rust-lang/rust/pull/128941)
* [interpret: immTy: tighten sanity checks in offset logic](https://github.com/rust-lang/rust/pull/129505)
* [lint on tail expr drop order change in Edition 2024](https://github.com/rust-lang/rust/pull/128662)
* [llvm-wrapper: adapt for LLVM 20 API changes](https://github.com/rust-lang/rust/pull/129353)
* [make `ArgAbi::make_indirect_force` more specific](https://github.com/rust-lang/rust/pull/129339)
* [make `writes_through_immutable_pointer` a hard error](https://github.com/rust-lang/rust/pull/129199)
* [more work on `zstd` compression](https://github.com/rust-lang/rust/pull/128935)
* [mv `build_reduced_graph_for_external_crate_res` into Resolver](https://github.com/rust-lang/rust/pull/129597)
* [pal/hermit: correctly round up microseconds in `Thread::sleep`](https://github.com/rust-lang/rust/pull/129588)
* [panicking: improve hint for Miri's `RUST_BACKTRACE` behavior](https://github.com/rust-lang/rust/pull/129501)
* [point at explicit `'static` obligations on a trait](https://github.com/rust-lang/rust/pull/129070)
* [pretty-print own args of existential projections (dyn-Trait w/ GAT constraints)](https://github.com/rust-lang/rust/pull/129395)
* [print the generic parameter along with the variance in dumps](https://github.com/rust-lang/rust/pull/129429)
* [remove invalid `TyCompat` relation for effects](https://github.com/rust-lang/rust/pull/129500)
* [safe transmute: gracefully bubble-up layout errors](https://github.com/rust-lang/rust/pull/129364)
* [skip updating when external binding is existed](https://github.com/rust-lang/rust/pull/128932)
* [use `assert_unsafe_precondition!` in `AsciiChar::digit_unchecked`](https://github.com/rust-lang/rust/pull/129374)
* [use a LocalDefId in ResolvedArg](https://github.com/rust-lang/rust/pull/129386)
* [use old ctx if has same expand environment during decode span](https://github.com/rust-lang/rust/pull/127279)
* [use subtyping for `UnsafeFnPointer` coercion, too](https://github.com/rust-lang/rust/pull/129288)
* [miri: `provenance_gc`: fix comment](https://github.com/rust-lang/miri/pull/3827)
* [miri: `readdir_r` shim: assume FreeBSD v12+](https://github.com/rust-lang/miri/pull/3748)
* [miri: avoid extra copy by using `retain_mut` and moving the deletion into the closure](https://github.com/rust-lang/miri/pull/3835)
* [miri: disable tree traversal optimization that is wrong due to lazy nodes](https://github.com/rust-lang/miri/pull/3847)
* [miri: epoll: add a EINVAL case](https://github.com/rust-lang/miri/pull/3836)
* [miri: epoll: handle edge case for `epoll_ctl`](https://github.com/rust-lang/miri/pull/3829)
* [miri: fix a misleading comment in `tests/pass/tree_borrows/tree-borrows.rs`](https://github.com/rust-lang/miri/pull/3834)
* [miri: fix calling pipe, pipe2, socketpair with a pointer-to-array](https://github.com/rust-lang/miri/pull/3840)
* [miri: implement SHA256 SIMD intrinsics on x86](https://github.com/rust-lang/miri/pull/3752)
* [miri: make Tree Borrows Provenance GC no longer produce stack overflows](https://github.com/rust-lang/miri/pull/3833)
* [miri weak memory emulation: put previous value into initial store buffer](https://github.com/rust-lang/rust/pull/128942)
* [stabilize opaque type precise capturing (RFC 3617)](https://github.com/rust-lang/rust/pull/127672)
* [stabilize `const_fn_floating_point_arithmetic`](https://github.com/rust-lang/rust/pull/128596)
* [stabilize `iter::repeat_n`](https://github.com/rust-lang/rust/pull/129294)
* [stabilize feature `char_indices_offset`](https://github.com/rust-lang/rust/pull/129276)
* [implement `debug_more_non_exhaustive`](https://github.com/rust-lang/rust/pull/127945)
* [add `Box::as_ptr` and `Box::as_mut_ptr` methods](https://github.com/rust-lang/rust/pull/129091)
* [add `const_cell_into_inner` to `OnceCell`](https://github.com/rust-lang/rust/pull/129382)
* [add `f16` and `f128` inline ASM support for `aarch64`](https://github.com/rust-lang/rust/pull/129536)
* [add a precondition check for `Layout::from_size_align_unchecked`](https://github.com/rust-lang/rust/pull/126556)
* [add implementations for `unbounded_shl`/`unbounded_shr`](https://github.com/rust-lang/rust/pull/129377)
* [change neutral element of `<fNN` as `iter::Sum>` to `neg_zero`](https://github.com/rust-lang/rust/pull/129321)
* [library: move unstable API of `new_uninit` to new features](https://github.com/rust-lang/rust/pull/129416)
* [fix `thread::sleep` Duration-handling for ESP-IDF](https://github.com/rust-lang/rust/pull/129232)
* [fix: `fs::remove_dir_all`: treat internal ENOENT as success](https://github.com/rust-lang/rust/pull/127623)
* [put `Pin::as_deref_mut` in impl `Pin<Ptr>` / rearrange Pin methods](https://github.com/rust-lang/rust/pull/129449)
* [implement `ptr::fn_addr_eq`](https://github.com/rust-lang/rust/pull/129323)
* [hashbrown: deprecate the raw entry API in favor of `HashTable`](https://github.com/rust-lang/hashbrown/pull/534)
* [hashbrown: rework the Entry API](https://github.com/rust-lang/hashbrown/pull/535)
* [cargo: mdman: Normalize newlines when rendering options](https://github.com/rust-lang/cargo/pull/14428)
* [cargo resolve: Dont show locking workspace members](https://github.com/rust-lang/cargo/pull/14445)
* [cargo: be more permissive while packaging unpublishable crates](https://github.com/rust-lang/cargo/pull/14408)
* [cargo: add `matches_prerelease` semantic](https://github.com/rust-lang/cargo/pull/14305)
* [cargo: -Cmetadata includes whether extra rustflags is same as host](https://github.com/rust-lang/cargo/pull/14432)
* [cargo: doctest respects Cargo's color options](https://github.com/rust-lang/cargo/pull/14425)
* [cargo: limiting pre-release match semantics to use only on `OptVersionReq::Req`](https://github.com/rust-lang/cargo/pull/14412)
* [cargo: log details of failure if no errors were seen](https://github.com/rust-lang/cargo/pull/14453)
* [cargo: more helpful missing feature error message](https://github.com/rust-lang/cargo/pull/14436)
* [rustdoc-search: use tighter json for names and parents](https://github.com/rust-lang/rust/pull/129426)
* [rustdoc: animate the `:target` highlight](https://github.com/rust-lang/rust/pull/129284)
* [rustdoc: show exact case-sensitive matches first](https://github.com/rust-lang/rust/pull/129430)
* [rustdoc: Generate source link on impl associated types](https://github.com/rust-lang/rust/pull/129560)
* [clippy: `declare_interior_mutable_const`: Ignore pointer types](https://github.com/rust-lang/rust-clippy/pull/13290)
* [clippy: add new `too_long_first_doc_paragraph` first paragraph lint](https://github.com/rust-lang/rust-clippy/pull/12993)
* [clippy: add new lint: `used_underscore_items`](https://github.com/rust-lang/rust-clippy/pull/13294)
* [clippy: check `std::panic::panic_any` in panic lint](https://github.com/rust-lang/rust-clippy/pull/13300)
* [clippy: diverging subexpression lint should not fire on `todo!()`](https://github.com/rust-lang/rust-clippy/pull/13285)
* [clippy: fix `manual_range_patterns` case with one element at OR](https://github.com/rust-lang/rust-clippy/pull/13311)
* [clippy: fix confusing message in `double_must_use` lint](https://github.com/rust-lang/rust-clippy/pull/13241)
* [clippy: fix suggestion `unnecessary_lazy_eval`](https://github.com/rust-lang/rust-clippy/pull/13299)
* [clippy: ignore underscore-prefixed args for `needless_pass_by_value` lint](https://github.com/rust-lang/rust-clippy/pull/13113)
* [clippy: rewrite `empty_line_after_doc_comments` and `empty_line_after_outer_attr`, move them from `nursery` to `suspicious`](https://github.com/rust-lang/rust-clippy/pull/13091)
* [clippy: start removing `snippet_opt` in favor of `get_source_text`](https://github.com/rust-lang/rust-clippy/pull/13244)
* [rust-analyzer: add new assist `toggle_macro_delimiter`](https://github.com/rust-lang/rust-analyzer/pull/17757)
* [rust-analyzer: allow declaring cfg groups in rust-project.json, to help sharing common cfgs](https://github.com/rust-lang/rust-analyzer/pull/17857)
* [rust-analyzer: add workspace level config to ratoml](https://github.com/rust-lang/rust-analyzer/pull/17913)
* [rust-analyzer: always show error lifetime arguments as `'_`](https://github.com/rust-lang/rust-analyzer/pull/17963)
* [rust-analyzer: don't enable the search fast path for short associated functions when a search scope is set](https://github.com/rust-lang/rust-analyzer/pull/17955)
* [rust-analyzer: expand proc-macros in workspace root, not package root](https://github.com/rust-lang/rust-analyzer/pull/17973)
* [rust-analyzer: fix "Unwrap block" assist with block modifiers](https://github.com/rust-lang/rust-analyzer/pull/17970)
* [rust-analyzer: fix Return Type Syntax to include `..` (i.e. `method(..)` and not `method()`) as specified in the RFC](https://github.com/rust-lang/rust-analyzer/pull/17962)
* [rust-analyzer: fix metadata retrying eating original errors](https://github.com/rust-lang/rust-analyzer/pull/17956)
* [rust-analyzer: fix trait method completions not acknowledging `Deref` impls](https://github.com/rust-lang/rust-analyzer/pull/17958)
* [rust-analyzer: improve proc-macro panic message and workspace loading failure diagnostic](https://github.com/rust-lang/rust-analyzer/pull/17943)
* [rust-analyzer: run flycheck without `rev_deps` when target is specified](https://github.com/rust-lang/rust-analyzer/pull/17912)
* [rust-analyzer: rust-analyzer should watch build files from rust-project.json](https://github.com/rust-lang/rust-analyzer/pull/17949)
* [rust-analyzer: wrong `Self: Sized` predicate for trait assoc items](https://github.com/rust-lang/rust-analyzer/pull/17948)
* [rust-analyzer: wrong `Sized` predicate for `generic_predicates_for_param`](https://github.com/rust-lang/rust-analyzer/pull/17939)
* [rust-analyzer: implement floating point casts in const eval](https://github.com/rust-lang/rust-analyzer/pull/17942)
* [rust-analyzer: perf: speed up search for short associated functions, especially very common identifiers such as `new`](https://github.com/rust-lang/rust-analyzer/pull/17927)
* [rust-analyzer: remove the ability to configure the user config path](https://github.com/rust-lang/rust-analyzer/pull/17930)

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
* [disposition: merge] [crates.io: Remove dev-dependencies from the index](https://github.com/rust-lang/rfcs/pull/3674)
* [disposition: merge] [Mergeable rustdoc cross-crate info](https://github.com/rust-lang/rfcs/pull/3662)
* [disposition: merge] [Guard Patterns](https://github.com/rust-lang/rfcs/pull/3637)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Proposal: stabilize `const_refs_to_static`](https://github.com/rust-lang/rust/issues/128183)
* [disposition: merge] [Check WF of source type's signature on fn pointer cast](https://github.com/rust-lang/rust/pull/129021)
* [disposition: merge] [rustdoc: add header map to the table of contents](https://github.com/rust-lang/rust/pull/120736)
* [disposition: merge] [doc: Make block of inline Deref methods foldable](https://github.com/rust-lang/rust/pull/127474)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [`#[derive(Default)]` on enum variants with fields](https://github.com/rust-lang/rfcs/pull/3683)
* [new] [[RFC] Default field values](https://github.com/rust-lang/rfcs/pull/3681)
* [new] [Simplify lightweight clones, including into closures and async blocks](https://github.com/rust-lang/rfcs/pull/3680)

## Upcoming Events

Rusty Events between 2024-08-28 - 2024-09-25 🦀

### Virtual
* 2024-08-28 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633267/)
* 2024-08-29 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
* 2024-09-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007365/)
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Typestate Pattern in Rust: With a Strict Builder Example**](https://www.meetup.com/indyrs/events/300328029/)
* 2024-09-05 | Virtual | [LambdaClass](https://lu.ma/user/usr-dkk9KnFvsvZEb7k)
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

### Africa
* 2024-09-06 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587/)

### Asia
* 2024-09-09 | Ramat Gan, IL | [Coralogix](https://coralogix.com/)
    * [**Rust as Scale**](https://coralogix.com/rust-coralogix-meetup/)
* 2024-09-14 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**September 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)

### Europe
* 2024-08-28 | Frankfurt (Main), DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Frankfurt WebAssembly**](https://www.meetup.com/rust-rhein-main/events/302738445/)
* 2024-08-29 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - The social Beergarden.**](https://www.meetup.com/rust-berlin/events/299421378/)
* 2024-08-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #50 sponsored by Adapt Agency**](https://www.meetup.com/copenhagen-rust-community/events/303040544/)
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
* 2024-09-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)

### North America
* 2024-08-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygclblc/)
* 2024-08-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : placeholder**](https://www.meetup.com/music-city-rust-developers/events/301420110/)
* 2024-08-29 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night/Happy Hour**](https://www.meetup.com/deep-dish-rust/events/302940777/)
* 2024-08-31 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Central Cambridge Rust Lunch, Aug 31**](https://www.meetup.com/bostonrust/events/302498706/)
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

# Oceania
* 2024-08-28 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Stuff ⚡ And Crabs 🦀**](https://www.meetup.com/rust-sydney/events/302951173/)

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

> ... opaque number sequences (\<GitHub\> "issue numbers") are not very informative about what is behind that pointer, and pretending they are is harmful. People could provide, instead, actual reasons for things, which do not require dereferencing random pointers, which thrashes cache.

– [Jubilee on rust-internals](https://internals.rust-lang.org/t/type-inference-breakage-in-1-80-has-not-been-handled-well/21374/29)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1603) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1f3rlhp/this_week_in_rust_562_this_week_in_rust/)</small>
