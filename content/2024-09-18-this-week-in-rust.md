Title: This Week in Rust 565
Number: 565
Date: 2024-09-18
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

### Foundation
* [Latest Rust Foundation Report Details Technical Accomplishments](https://foundation.rust-lang.org/news/latest-rust-foundation-report-details-technical-accomplishments/)

### Project/Tooling Updates
* [This Month in Xilem, August 2024](https://linebender.org/blog/tmix-08/)
* [Next-gen builder macro Bon 2.3 release](https://elastio.github.io/bon/blog/bon-builder-v2-3-release)
* [RootAsRole 3.0 Release](https://github.com/LeChatP/RootAsRole/discussions/57)
* [Maelstrom 0.12.0](https://maelstrom-software.com/blog/0.12.0/)
* [Announcing vf2](https://github.com/OwenTrokeBillard/vf2/discussions/1)

### Observations/Thoughts
* [I landed my dream job making a Rust game engine. Now what?](https://bevyengine.org/news/dream-job/)
* [audio] [Talking to Microcontrollers with Postcard-RPC](https://sdr-podcast.com/episodes/postcard-rpc/)

### Rust Walkthroughs
* [Understanding Memory Ordering in Rust](https://emschwartz.me/understanding-memory-ordering-in-rust/)
* [video] [Build with Naz: Subtyping and variance](https://www.youtube.com/watch?v=HRlpYXi4E-M)

### Research
* [Towards Modified Condition/Decision Coverage of Rust](https://arxiv.org/abs/2409.08708)
* [Blindsided by Rust's Subtyping and Variance](https://nullderef.com/blog/rust-variance/)

### Miscellaneous
* [August '24 Rust Jobs Report](https://filtra.io/rust-aug-24)
* [video] [QnA with Friends: Orhun Parmaksiz on Open Source, the Rust Community and TUI's in Rust](https://www.youtube.com/watch?v=bqYyPVdTvxk)

## Crate of the Week

This week's crate is [lepton-jpeg-rust](https://github.com/microsoft/lepton_jpeg_rust), Microsoft's port of Dropbox' lepton space-saving JPEG compressor library to Rust.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1346) for the suggestion!

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

* [zerocopy - Make derive macros hygienic](https://github.com/google/zerocopy/issues/11)
* [zerocopy - Ensure all safety comments quote and cite a specific version of the documentation](https://github.com/google/zerocopy/issues/1655)
* [zerocopy - Inline trait methods in derive-generated code](https://github.com/google/zerocopy/issues/7)
* [zerocopy - Configure OpenSSF Scorecard's `Pinned-Dependencies` check to block CI](https://github.com/google/zerocopy/issues/1579)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

351 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-09-10..2024-09-17

* [deprecate -Csoft-float because it is unsound (and not fixable)](https://github.com/rust-lang/rust/pull/129897)
* [add -Z small-data-threshold](https://github.com/rust-lang/rust/pull/117465)
* [fix conflicting negative impl marker](https://github.com/rust-lang/rust/pull/130311)
* [add a machine-applicable suggestion to "unreachable pattern"](https://github.com/rust-lang/rust/pull/128991)
* [add system libs for LLVM when cross compiling for Windows](https://github.com/rust-lang/rust/pull/130398)
* [also emit `missing_docs` lint with `--test` to fulfil expectations](https://github.com/rust-lang/rust/pull/130025)
* [avoid crashing on variadic functions when producing arg-mismatch errors](https://github.com/rust-lang/rust/pull/130437)
* [const-eval interning: accept interior mutable pointers in final value](https://github.com/rust-lang/rust/pull/128543)
* [const: don't ICE when encountering a mutable ref to immutable memory](https://github.com/rust-lang/rust/pull/130394)
* [correctly account for niche-optimized tags in `rustc_transmute`](https://github.com/rust-lang/rust/pull/130371)
* [disallow `naked_asm!` outside of `#[naked]` functions](https://github.com/rust-lang/rust/pull/130195)
* [do precise capturing arg validation in resolve](https://github.com/rust-lang/rust/pull/130414)
* [don't ICE when RPITIT captures more method args than trait definition](https://github.com/rust-lang/rust/pull/130412)
* [don't ICE when generating `Fn` shim for async closure with borrowck error](https://github.com/rust-lang/rust/pull/130410)
* [don't leave debug locations for constants sitting on the builder indefinitely](https://github.com/rust-lang/rust/pull/130052)
* [don't suggest adding return type for closures with default return type](https://github.com/rust-lang/rust/pull/129260)
* [don't use `typeck_root_def_id` in codegen for finding closure's root](https://github.com/rust-lang/rust/pull/129716)
* [don't warn empty branches unreachable for now](https://github.com/rust-lang/rust/pull/129103)
* [enumerate lint expectations using AttrId](https://github.com/rust-lang/rust/pull/130050)
* [fix `Parser::break_up_float`'s right span](https://github.com/rust-lang/rust/pull/130349)
* [fix anon const def-creation when macros are involved](https://github.com/rust-lang/rust/pull/129137)
* [fix crash when labeling arguments for `call_once` and friends](https://github.com/rust-lang/rust/pull/129320)
* [fix default/minimum deployment target for Aarch64 simulator targets](https://github.com/rust-lang/rust/pull/129367)
* [fix false positive with `missing_docs` and `#[test]`](https://github.com/rust-lang/rust/pull/130219)
* [fix linking error when compiling for 32-bit watchOS](https://github.com/rust-lang/rust/pull/130077)
* [fix lint levels not getting overridden by attrs on `Stmt` nodes](https://github.com/rust-lang/rust/pull/130293)
* [generalize: track relevant info in cache key](https://github.com/rust-lang/rust/pull/130194)
* [layout computation: gracefully handle unsized types in unexpected locations](https://github.com/rust-lang/rust/pull/129970)
* [compute Rust exception class from its string repr](https://github.com/rust-lang/rust/pull/130381)
* [limit `libc::link` usage to `nto70` target only, not NTO OS](https://github.com/rust-lang/rust/pull/130248)
* [linker: allow MSVC to use import libraries following the Meson/MinGW convention](https://github.com/rust-lang/rust/pull/123436)
* [make basic allocation functions `track_caller` in Miri for nicer backtraces](https://github.com/rust-lang/rust/pull/130245)
* [make dist vendoring configurable](https://github.com/rust-lang/rust/pull/130110)
* [make some lint doctests compatible with `--stage=0`](https://github.com/rust-lang/rust/pull/130353)
* [map `ERROR_CANT_RESOLVE_FILENAME` to `ErrorKind::FilesystemLoop`](https://github.com/rust-lang/rust/pull/130207)
* [map `WSAEDQUOT` to `ErrorKind::FilesystemQuotaExceeded`](https://github.com/rust-lang/rust/pull/130206)
* [more eagerly discard constraints on overflow](https://github.com/rust-lang/rust/pull/130273)
* [properly report error on `const gen fn`](https://github.com/rust-lang/rust/pull/130252)
* [report the `note` when specified in `diagnostic::on_unimplemented`](https://github.com/rust-lang/rust/pull/130123)
* [rescope temp lifetime in if-let into IfElse with migration lint](https://github.com/rust-lang/rust/pull/107251)
* [simplify the canonical clone method and the copy-like forms to copy](https://github.com/rust-lang/rust/pull/128299)
* [some fixes for `clashing_extern_declarations` lint](https://github.com/rust-lang/rust/pull/130301)
* [suggest the correct pattern syntax on usage of unit variant pattern for a `struct` variant](https://github.com/rust-lang/rust/pull/129520)
* [ban non-array SIMD](https://github.com/rust-lang/rust/pull/129403)
* [`simd_shuffle`: require index argument to be a vector](https://github.com/rust-lang/rust/pull/130268)
* [interpret, miri: fix dealing with overflow during slice indexing and allocation](https://github.com/rust-lang/rust/pull/130342)
* [interpret: `get_ptr_alloc_mut`: lookup allocation only once](https://github.com/rust-lang/rust/pull/130148)
* [interpret: simplify SIMD type handling](https://github.com/rust-lang/rust/pull/130215)
* [notify miri when intrinsics are changed](https://github.com/rust-lang/rust/pull/130228)
* [miri: fix overflow detection for unsigned pointer offset](https://github.com/rust-lang/rust/pull/130239)
* [miri: treat non-memory local variables properly for data race detection](https://github.com/rust-lang/rust/pull/129828)
* [miri: /miri run: directly run binary instead of using 'cargo run'](https://github.com/rust-lang/miri/pull/3881)
* [miri: add Android pthread support](https://github.com/rust-lang/miri/pull/3889)
* [miri: add non-portable linux pthread initializers to layout sanity check](https://github.com/rust-lang/miri/pull/3880)
* [miri: detect when `pthread_cond_t` is moved](https://github.com/rust-lang/miri/pull/3884)
* [miri: support pthread primitives on FreeBSD](https://github.com/rust-lang/miri/pull/3886)
* [stabilize `&mut` (and `*mut`) as well as &Cell` (and `*const Cell`) in const`](https://github.com/rust-lang/rust/pull/129195)
* [stabilize `const_extern_fn`](https://github.com/rust-lang/rust/pull/129753)
* [stabilize `entry_insert`](https://github.com/rust-lang/rust/pull/130290)
* [stabilize most of `io_error_more`](https://github.com/rust-lang/rust/pull/128316)
* [implement `PartialEq` for `ExitCode`](https://github.com/rust-lang/rust/pull/127633)
* [move `Option::unwrap_unchecked` into `const_option` feature gate](https://github.com/rust-lang/rust/pull/130118)
* [add `NonNull` convenience methods to `Box` and `Vec`](https://github.com/rust-lang/rust/pull/130061)
* [add `core::panic::abort_unwind`](https://github.com/rust-lang/rust/pull/130339)
* [properly handle EOF in `BufReader::peek`](https://github.com/rust-lang/rust/pull/130042)
* [implement feature `string_from_utf8_lossy_owned` for lossy conversion from `Vec<u8>` to `String` methods](https://github.com/rust-lang/rust/pull/129439)
* [futures: `#[inline(always)]` on `clone_arc_raw`](https://github.com/rust-lang/futures-rs/pull/2865)
* [futures: add accessors for the inner of `stream::Iter`](https://github.com/rust-lang/futures-rs/pull/2875)
* [cargo: `fix(vendor)`: trust crate version only when coming from registries](https://github.com/rust-lang/cargo/pull/14530)
* [cargo: disable the `shell_completions` tests](https://github.com/rust-lang/cargo/pull/14546)
* [cargo: add custom completer for `cargo -Z <TAB>`](https://github.com/rust-lang/cargo/pull/14536)
* [cargo: add custom completer for completing bin names](https://github.com/rust-lang/cargo/pull/14533)
* [cargo: add custom completer for completing installed binaries](https://github.com/rust-lang/cargo/pull/14534)
* [cargo: add native comlpetion with CompleteEnv under the nightly](https://github.com/rust-lang/cargo/pull/14493)
* [rustdoc rfc#3662 changes under unstable flags](https://github.com/rust-lang/rust/pull/129337)
* [rustdoc: add two regression tests](https://github.com/rust-lang/rust/pull/130173)
* [rustdoc: rename `issue-\d+.rs` tests to have meaningful names (part 9)](https://github.com/rust-lang/rust/pull/130287)
* [rustdoc: unify the short-circuit on all lints](https://github.com/rust-lang/rust/pull/129975)
* [rustfmt: `config_proc_macro`: reduce syn's features](https://github.com/rust-lang/rustfmt/pull/6237)
* [rustfmt: format trailing where clauses in type aliases](https://github.com/rust-lang/rustfmt/pull/5887)
* [rustfmt: non-panicking `fmt::Display` for `FileName`](https://github.com/rust-lang/rustfmt/pull/6328)
* [rustfmt: refactor - show file path in error message when parsing config from toml](https://github.com/rust-lang/rustfmt/pull/6323)
* [clippy: look at adjusted types instead of fn signature types in `ptr_arg`](https://github.com/rust-lang/rust-clippy/pull/13313)
* [clippy: not trigger `duplicated_attributes` on duplicate reasons](https://github.com/rust-lang/rust-clippy/pull/13386)
* [clippy: special-case suggestions for null pointers constness cast](https://github.com/rust-lang/rust-clippy/pull/13369)
* [clippy: consider msrv for const context for `const_float_bits_conv`](https://github.com/rust-lang/rust/pull/130305)
* [rust-analyzer: add command to report unresolved references](https://github.com/rust-lang/rust-analyzer/pull/17904)
* [rust-analyzer: assist: ensure `replace_qualified_name_with_use` applies to the first path segment](https://github.com/rust-lang/rust-analyzer/pull/18050)
* [rust-analyzer: automatically add semicolon when completing unit-returning functions](https://github.com/rust-lang/rust-analyzer/pull/18018)
* [rust-analyzer: generate names for tuple-struct in add-missing-match-arms](https://github.com/rust-lang/rust-analyzer/pull/18038)
* [rust-analyzer: render patterns in params for hovering](https://github.com/rust-lang/rust-analyzer/pull/18075)
* [rust-analyzer: correctly escape strings in our quote macro](https://github.com/rust-lang/rust-analyzer/pull/18092)
* [rust-analyzer: don't emit empty inlay hint parts](https://github.com/rust-lang/rust-analyzer/pull/18107)
* [rust-analyzer: don't report typed hole error in asm! out ops](https://github.com/rust-lang/rust-analyzer/pull/18106)
* [rust-analyzer: faulty notifications should not bring down the server](https://github.com/rust-lang/rust-analyzer/pull/18105)
* [rust-analyzer: fix `inline_const_as_literal` error when the number \>= 10](https://github.com/rust-lang/rust-analyzer/pull/18052)
* [rust-analyzer: fix inference of literals when the expectation is Castable](https://github.com/rust-lang/rust-analyzer/pull/18101)
* [rust-analyzer: fix printing of constants greater than `i128::MAX`](https://github.com/rust-lang/rust-analyzer/pull/18119)
* [rust-analyzer: immutable tree panic in `generate_delegate_trait`](https://github.com/rust-lang/rust-analyzer/pull/18073)
* [rust-analyzer: skip checks for cast to dyn traits](https://github.com/rust-lang/rust-analyzer/pull/18093)
* [rust-analyzer: use more correct handling of lint attributes](https://github.com/rust-lang/rust-analyzer/pull/18099)

### Rust Compiler Performance Triage

A relatively quiet week, with overall neutral performance across our set of key
metrics (instructions, cycles, memory).

Triage done by **@simulacrum**.
Revision range: [263a3aee..170d6cb8](https://perf.rust-lang.org/?start=263a3aeeb8f2d0e9cc85eee61774d1f5f23dc3f5&end=170d6cb845c8c3f0dcec5cdd4210df9ecf990244&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 4 Mixed; 2 of them in rollups
54 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-09-16.md)

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
* *No RFCs entered Final Comment Period this week.*

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
[disposition: merge] [Allow build scripts to report error messages through `cargo::error`](https://github.com/rust-lang/cargo/pull/14435)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Allow boolean literals as cfg predicates](https://github.com/rust-lang/rfcs/pull/3695)
* [new] [Expose std support via --print](https://github.com/rust-lang/rfcs/pull/3693)
* [new] [RFC: Give users control over `feature unification`](https://github.com/rust-lang/rfcs/pull/3692)
* [new] [Trusted Publishing Support on Crates.io](https://github.com/rust-lang/rfcs/pull/3691)

## Upcoming Events

Rusty Events between 2024-09-18 - 2024-10-16 🦀

### Virtual
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
* 2024-09-25 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Reflections on RustConf 2024**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/303444953/)
* 2024-09-26 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633269/)
* 2024-09-26 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rusty secure communication on embedded devices**](https://www.meetup.com/charlottesville-rust-meetup/events/303159380/)
* 2024-10-02 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 8 - Asynchronous Programming**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-10-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031649/)
* 2024-10-02 | Virtual (Vancouver, BC, CA) | [Vancouver Postgres](https://www.meetup.com/vancouver-postgres/)
    * [**Leveraging a PL/RUST extension to protect sensitive data in PostgreSQL**](https://www.meetup.com/vancouver-postgres/events/302160672/)
* 2024-10-03 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897992/)
* 2024-10-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346983/)
* 2024-10-10 | Virtual (Barcelona, ES) | [BcnRust](https://bcnrust.github.io) + [Codurance](https://www.codurance.com/)
    * [**15th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/303443195/)
* 2024-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633270/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
    
### Africa
* 2024-10-05 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-10-09 | Subang Jaya / Kuala Lumpur, Selangor, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup - Traits and How to Read Trait (October 2024)**](https://docs.google.com/forms/d/e/1FAIpQLScNS5IWmnzTTJAOw-RIxdj4_BWbxB5NVmAVO30XHr_viMbLqQ/viewform)

### Asia
* 2024-09-21 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**September 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)
* 2024-09-24 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Building Tangible Things With Rust**](https://www.meetup.com/tokyo-rust-meetup/events/303402114/)

### Europe
* 2024-09-18 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Rust Moravia Meetup (September 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 2024-09-18 | Vienna, AT + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024 (Sep 18-20)**](https://lpc.events/event/18/sessions/186/)
* 2024-09-20 | Hamburg, DE | [Code.Talks](https://codetalks.com/)
    * [**Code.Talks Conference 2024: "Journey to Fullstack Mobile Game Development in Rust" (Stephan Dilly)**](https://codetalks.com/program?talkId=2290)
* 2024-09-21 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #5**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)
* 2024-09-24 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #70**](https://www.meetup.com/rust-paris/events/303212378/)
* 2024-09-24 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust)
    * [**Rust meetup #70**](https://www.meetup.com/Stockholm-Rust/events/303210419)
* 2024-09-24 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/)
    * [**Rust Drinkup 2024/09**](https://www.meetup.com/zagreb-rust-meetup/events/303484490/)
* 2024-09-26 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Talk Night at Alexandra Instituttet**](https://www.meetup.com/rust-aarhus/events/301522991/)
* 2024-09-26 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Rust Meetup #9: From loops to folds**](https://www.meetup.com/rust-meetup-augsburg/events/302437593)
* 2024-09-26 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421380/)
* 2024-09-26 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (September 2024)**](https://www.meetup.com/rust-prague/events/303346494/)
* 2024-09-27 | Mannheim, DE | [Hackerstolz e.V.](https://www.hackerstolz.de/en/)
    * [**Hackerstolz Stammtisch Rhein-Neckar**](https://www.hackerstolz.de/en/)
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

### North America
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
* 2024-09-26 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Develpers : Community Presentations**](https://www.meetup.com/music-city-rust-developers/events/301420118/)
* 2024-09-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch, Sep 27**](https://www.meetup.com/bostonrust/events/302498761/)
* 2024-10-03 | Boston, MA, US | [SquiggleConf](https://2024.squiggleconf.com/)
    * [**SquiggleConf 2024: "Oxc: Pluggable Next-Gen Tooling At Rust Speed", Don Isaac**](https://2024.squiggleconf.com/schedule)
* 2024-10-03 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Iterators in Rust**](https://www.meetup.com/stl-rust/events/302371456/)
* 2024-10-04 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust pt1. Prerequisitos**](https://www.meetup.com/rust-mx/events/303480458/)
* 2024-10-08 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcnblb/)
* 2024-10-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346970/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)

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

> \[Rust\] is a conspiracy to popularize programing language concepts from the 80s

– [Esteban Küber on hachyderm](https://hachyderm.io/@ekuber/113130426545931814)

[llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1610) is quite thankful to himself for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1fklulu/this_week_in_rust_565_this_week_in_rust/)</small>
