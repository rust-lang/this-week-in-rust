Title: This Week in Rust 558
Number: 558
Date: 2024-07-31
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crates are [static-keys](https://crates.io/crates/static-keys), a nightly-only mechanism to speed up checks of rarely changed features and [bon](https://elastio.github.io/bon/docs/guide/overview), yet another crate to autogenerate builders from functions, methods and structs.

Thanks to [EvianZhang](https://users.rust-lang.org/t/crate-of-the-week/2704/1325) and [Veetaha](https://users.rust-lang.org/t/crate-of-the-week/2704/1326) for the suggestions!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

434 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-07-23..2024-07-30

* [compiler: replace ASCII control chars with Unicode Control Pictures](https://github.com/rust-lang/rust/pull/127528)
* [`#[naked]`: report incompatible attributes](https://github.com/rust-lang/rust/pull/127853)
* [`#[naked]`: use an allowlist for allowed options on `asm!` in naked functions](https://github.com/rust-lang/rust/pull/128138)
* [add NuttX based targets for RISC-V and ARM](https://github.com/rust-lang/rust/pull/127755)
* [add `select_unpredictable` to force LLVM to use CMOV](https://github.com/rust-lang/rust/pull/128250)
* [add a label to point to the lacking macro name definition](https://github.com/rust-lang/rust/pull/127557)
* [add basic Serde serialization capabilities to Stable MIR](https://github.com/rust-lang/rust/pull/126963)
* [add migration lint for 2024 prelude additions](https://github.com/rust-lang/rust/pull/125889)
* [always set `result` during `finish()` in debug builders](https://github.com/rust-lang/rust/pull/127946)
* [bootstrap command refactoring: make command output API more bulletproof (step 7)](https://github.com/rust-lang/rust/pull/127799)
* [build-manifest: allow building manifests for formats that only have xz compression](https://github.com/rust-lang/rust/pull/128091)
* [compiler: never `debug_assert` in codegen](https://github.com/rust-lang/rust/pull/127995)
* [do not try to reveal hidden types when trying to prove auto-traits in the defining scope](https://github.com/rust-lang/rust/pull/122192)
* [do not use global caches if opaque types can be defined](https://github.com/rust-lang/rust/pull/126024)
* [don't ICE if HIR and middle types disagree in borrowck error reporting](https://github.com/rust-lang/rust/pull/128172)
* [don't ICE when auto trait has assoc ty in old solver](https://github.com/rust-lang/rust/pull/128160)
* [don't ICE when encountering error regions when confirming object method candidate](https://github.com/rust-lang/rust/pull/128239)
* [don't elaborate associated types with Sized bounds in `trait_object_ty` in cfi](https://github.com/rust-lang/rust/pull/127882)
* [don't manually implement `PartialEq` for some types in `rustc_type_ir`](https://github.com/rust-lang/rust/pull/128246)
* [don't record trait aliases as marker traits](https://github.com/rust-lang/rust/pull/128174)
* [exhaustiveness: explain why a given pattern is considered unreachable](https://github.com/rust-lang/rust/pull/128034)
* [extend rules of dead code analysis for impls for adts to impls for types refer to adts](https://github.com/rust-lang/rust/pull/127017)
* [fix a span error when parsing a wrong param of function](https://github.com/rust-lang/rust/pull/128185)
* [fix malformed suggestion for repeated maybe unsized bounds](https://github.com/rust-lang/rust/pull/127717)
* [fix supertrait associated type unsoundness](https://github.com/rust-lang/rust/pull/126090)
* [fix: UWP compilation issue with `change_time` refactored type](https://github.com/rust-lang/rust/pull/128211)
* [graciously handle `Drop` impls introducing more generic parameters than the ADT](https://github.com/rust-lang/rust/pull/127220)
* [handle `no_std` targets on std builds](https://github.com/rust-lang/rust/pull/128182)
* [implement `Copy`/`Clone` for async closures](https://github.com/rust-lang/rust/pull/128201)
* [implement lint against ambiguous negative literals](https://github.com/rust-lang/rust/pull/121364)
* [improve `extern "<abi>" unsafe fn()` error message](https://github.com/rust-lang/rust/pull/128229)
* [improve error message when `global_asm!` uses `asm!` options](https://github.com/rust-lang/rust/pull/128207)
* [improve spans on evaluated `cfg_attr`s](https://github.com/rust-lang/rust/pull/128133)
* [inject arm32 shims into Windows metadata generation](https://github.com/rust-lang/rust/pull/127999)
* [isolate the diagnostic code that expects `thir::Pat` to be printable](https://github.com/rust-lang/rust/pull/128304)
* [let InstCombine remove Clone shims inside Clone shims](https://github.com/rust-lang/rust/pull/128165)
* [make `missing_fragment_specifier` an error in edition 2024](https://github.com/rust-lang/rust/pull/128006)
* [make sure that args are compatible in `resolve_associated_item`](https://github.com/rust-lang/rust/pull/128171)
* [mark `missing_fragment_specifier` as `FutureReleaseErrorReportInDeps`](https://github.com/rust-lang/rust/pull/128122)
* [match exhaustiveness: Expand or-patterns as a separate step](https://github.com/rust-lang/rust/pull/128015)
* [note closure captures when reporting cast to fn ptr failed](https://github.com/rust-lang/rust/pull/128082)
* [perform instsimplify before inline to eliminate some trivial calls](https://github.com/rust-lang/rust/pull/128265)
* [skip assoc type during infer source visitor](https://github.com/rust-lang/rust/pull/128337)
* [stop using `unsized_const_parameters` in core/std](https://github.com/rust-lang/rust/pull/128150)
* [support ?Trait bounds in supertraits and dyn Trait under a feature gate](https://github.com/rust-lang/rust/pull/121676)
* [support lists and stylings in more places for `rustc --explain`](https://github.com/rust-lang/rust/pull/126994)
* [tweak "wrong # of generics" suggestions](https://github.com/rust-lang/rust/pull/127374)
* [miri: add `flock` shim](https://github.com/rust-lang/miri/pull/3759)
* [miri: add `gettid` support](https://github.com/rust-lang/miri/pull/3756)
* [miri: fix `offset_from` behavior on wildcard pointers](https://github.com/rust-lang/rust/pull/128277)
* [miri: better diagnostics for Tree Borrows + int2ptr casts](https://github.com/rust-lang/miri/pull/3766)
* [miri: show warning when Stacked Borrows skips a reborrow due to 'extern type'](https://github.com/rust-lang/miri/pull/3701)
* [stabilize `const_waker`](https://github.com/rust-lang/rust/pull/128228)
* [stabilize `is_sorted`](https://github.com/rust-lang/rust/pull/128279)
* [stabilize `offset_of_nested`](https://github.com/rust-lang/rust/pull/128284)
* [stabilize const `{integer}::from_str_radix` i.e. `const_int_from_str`](https://github.com/rust-lang/rust/pull/124941)
* [initial implementation of `anonymous_pipe` API](https://github.com/rust-lang/rust/pull/127153)
* [add `elem_offset` and related methods](https://github.com/rust-lang/rust/pull/126770)
* [add `is_multiple_of` for unsigned integer types](https://github.com/rust-lang/rust/pull/128103)
* [implement `unsigned_signed_diff`](https://github.com/rust-lang/rust/pull/126042)
* [allow optimizing `u32::from::<char>`](https://github.com/rust-lang/rust/pull/124905)
* [bitwise and bytewise methods on `NonZero`](https://github.com/rust-lang/rust/pull/128282)
* [implement `unsigned_signed_diff`](https://github.com/rust-lang/rust/pull/126042)
* [optimize empty case in `Vec::retain`](https://github.com/rust-lang/rust/pull/128234)
* [remove generic lifetime parameter of trait `Pattern`](https://github.com/rust-lang/rust/pull/127481)
* [replace `io::Cursor::{remaining_slice, is_empty}`](https://github.com/rust-lang/rust/pull/109174)
* [cargo: dont call wrap in a no-op `source_id::with*`](https://github.com/rust-lang/cargo/pull/14318)
* [cargo: fix: remove rustc probe for `--check-cfg` support](https://github.com/rust-lang/cargo/pull/14302)
* [cargo: package workspaces](https://github.com/rust-lang/cargo/pull/13947)
* [rustdoc: Add copy code feature](https://github.com/rust-lang/rust/pull/125779)
* [rustdoc: change title of search results](https://github.com/rust-lang/rust/pull/128210)
* [rustdoc: use strategic ThinVec/Box to shrink `clean::ItemKind`](https://github.com/rust-lang/rust/pull/128263)
* [rustdoc: word wrap CamelCase in the item list table and sidebar](https://github.com/rust-lang/rust/pull/126247)
* [rustfmt: implement Style Edition support](https://github.com/rust-lang/rustfmt/pull/6247)
* [rustfmt: modify ListItem to hold RewriteResult as the item field](https://github.com/rust-lang/rustfmt/pull/6244)
* [rustfmt: track configs set from cli flags](https://github.com/rust-lang/rustfmt/pull/6253)
* [clippy: `missing_trait_methods`: lint methods in definition order](https://github.com/rust-lang/rust-clippy/pull/13159)
* [clippy: `needless_borrows_for_generic_args`: Fix for &mut](https://github.com/rust-lang/rust-clippy/pull/12892)
* [clippy: add `BTreeSet` detection to the `set_contains_or_insert` lint](https://github.com/rust-lang/rust-clippy/pull/13053)
* [clippy: add possibility to focus on search input using keyboard](https://github.com/rust-lang/rust-clippy/pull/13178)
* [clippy: emit `if_let_mutex` in presence of other mutexes](https://github.com/rust-lang/rust-clippy/pull/13174)
* [clippy: fix `while_let_on_iterator` dropping loop label when applying fix](https://github.com/rust-lang/rust-clippy/pull/13149)
* [clippy: fix display of configs in clippy doc page](https://github.com/rust-lang/rust-clippy/pull/13166)
* [clippy: fix under loop may dropping loop label when applying fix](https://github.com/rust-lang/rust-clippy/pull/13176)
* [clippy: fix handling of `Deref` in `assigning_clones`](https://github.com/rust-lang/rust-clippy/pull/12473)
* [clippy: lint casts to `u128` in `cast_lossless`](https://github.com/rust-lang/rust-clippy/pull/13146)
* [clippy: lintcheck: support underscores replacement in URL crate names](https://github.com/rust-lang/rust-clippy/pull/13148)
* [clippy: make `BindInsteadOfMap` a `struct`](https://github.com/rust-lang/rust-clippy/pull/13153)
* [clippy: make `std_instead_of_core` somewhat MSRV aware](https://github.com/rust-lang/rust-clippy/pull/13168)
* [clippy: make restriction lint's use `span_lint_and_then` (i → p)](https://github.com/rust-lang/rust-clippy/pull/13144)
* [clippy: suggest `.cast`/`.cast_const`/`.cast_mut` in `transmute_ptr_as_ptr`](https://github.com/rust-lang/rust-clippy/pull/13143)
* [rust-analyzer: run unit tests at the crate level not workspace](https://github.com/rust-lang/rust-analyzer/pull/17472)
* [rust-analyzer: Invalid RA diagnostic error: expected 2 arguments, found 1](https://github.com/rust-lang/rust-analyzer/pull/17511)
* [rust-analyzer: add preliminary support for `+ use<..> precise_capturing` syntax](https://github.com/rust-lang/rust-analyzer/pull/17676)
* [rust-analyzer: introduce workspace `rust-analyzer.toml`s](https://github.com/rust-lang/rust-analyzer/pull/17735)
* [rust-analyzer: use spans for builtin and declarative macro expansion errors](https://github.com/rust-lang/rust-analyzer/pull/17707)
* [rust-analyzer: don't retry inlay hint and cole lens requests](https://github.com/rust-lang/rust-analyzer/pull/17742)
* [rust-analyzer: early exit if unresolved field is an index](https://github.com/rust-lang/rust-analyzer/pull/17713)
* [rust-analyzer: explictly show `async` keyword on `impl trait` methods](https://github.com/rust-lang/rust-analyzer/pull/17736)
* [rust-analyzer: fix builtin includes rejecting raw string literals](https://github.com/rust-lang/rust-analyzer/pull/17741)
* [rust-analyzer: fix includes not working with expr fragment inputs](https://github.com/rust-lang/rust-analyzer/pull/17706)
* [rust-analyzer: let glob imports override other globs' visibility](https://github.com/rust-lang/rust-analyzer/pull/17715)
* [rust-analyzer: support new cargo config get env format](https://github.com/rust-lang/rust-analyzer/pull/17697)
* [rust-analyzer: flip the naming of the doc comment to comment assist](https://github.com/rust-lang/rust-analyzer/pull/17720)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2024-07-31 - 2024-08-28 🦀

### Virtual
* 2024-07-17| Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/302139632/)
* [**Rust for Rustaceans Book Club: Chapter 10: Concurrency (and Parallelism)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-07-17 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488824/)
* 2024-07-18 | Virtual (IL) | [Rust in Israel](https://www.meetup.com/rust-in-israel/)
    * [**Threads Rust (Virtual) - תהליכונים בראסט - מפגש בזום**](https://www.meetup.com/rust-in-israel/events/302219468/)
* 2024-07-18 | Virtual (Rotterdam, NL)| [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #5**](https://www.meetup.com/bevy-game-development/events/301711262/)
* 2024-07-23 | Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/301062840/)
* 2024-07-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Exploring Rust API Use Cases**](https://www.meetup.com/women-in-rust/events/301730780/)
* 2024-07-25 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897865/)
* 2024-07-27 | Hybrid - Virtual and In-Person (Kyiv, UA) | [UA Rust](https://uarust.com/)
    * [**UARust Conference 2024**](https://uarust.com/)
* 2024-07-27 | Virtual | [Leptos Monthly Meetup](https://lu.ma/user/leptos)
    * [**Leptos Monthly Meetup: Pavex with Luca Palmieri**](https://lu.ma/3ouqapsr)
* 2024-07-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585665/)
* 2024-07-31 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633265/)
* 2024-08-06 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn! (Virtual)**](https://www.meetup.com/women-in-rust/events/300994574/)
* 2024-08-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191718/)
* 2024-08-06 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Web development in Rust using Rocket - part 2 (English)**](https://www.meetup.com/code-mavens/events/301736709/)
* 2024-08-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328027/)
* 2024-08-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897918/)
* 2024-08-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300787793/)
* 2024-08-08 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
* 2024-08-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346978/)

### Africa
* 2024-08-02 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)

### Asia
* 2024-07-20 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2024 Rustacean meetup 🤝 C4GT**](https://hasgeek.com/rustbangalore/july-2024-rustacean-meetup-c4gt/)

### Europe
* 2024-07-17 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/302024746/)
* 2024-07-18 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2024**](https://www.meetup.com/rust-bern/events/301952761/)
* 2024-07-23 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester July Code Night**](https://www.meetup.com/rust-manchester/events/301461206/)
* 2024-07-23 | Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-07-25 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #8**](https://www.meetup.com/rust-meetup-augsburg/events/301642385/)
* 2024-07-25 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288967/)
* 2024-07-27 | Hybrid - Virtual and In-Person (Kyiv, UA) | [UA Rust](https://uarust.com/)
    * [**UARust Conference 2024**](https://uarust.com/)
* 2024-07-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #9**](https://www.meetup.com/rust-basel/events/301459503/)
* 2024-08-14 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302153005/)

### North America
* 2024-07-17 | Hybrid - Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : holding pattern**](https://www.meetup.com/music-city-rust-developers/events/301411794/)
* 2024-07-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/301883176/)
* 2024-07-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, July 21**](https://www.meetup.com/bostonrust/events/301550076/)
* 2024-07-24 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygckbgc/)
* 2024-07-25 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302066981/)
* 2024-07-29 | Cambridge, MA, US| [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch, July 29**](https://www.meetup.com/bostonrust/events/301550289/)
* 2024-08-01 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/301697569/)
* 2024-08-08 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302067008/)

# Oceania
* 2024-08-01 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**August Meetup**](https://www.meetup.com/rust-brisbane/events/302244260/)

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

> Man, rust crates are so allergic to 1.0 that they'll skip all the way to version 22.

– [/u/darkpyro2 on /r/rust](https://www.reddit.com/r/rust/comments/1e6j8sk/comment/ldv61bm/)

Thanks to [Erich Gubler](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1593) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
