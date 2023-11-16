Title: This Week in Rust 521
Number: 521
Date: 2023-11-15
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
* [Faster compilation with the parallel front-end in nightly](https://blog.rust-lang.org/2023/11/09/parallel-rustc.html)
* [November 2023 Leadership Council Update](https://blog.rust-lang.org/inside-rust/2023/11/13/leadership-council-update.html)
* [Our Vision for the Rust Specification](https://blog.rust-lang.org/inside-rust/2023/11/15/spec-vision.html)

### Foundation
* [The Rust Foundation to Develop Training and Certification Program](https://foundation.rust-lang.org/news/the-rust-foundation-to-develop-training-and-certification-program/)

### Newsletters

### Project/Tooling Updates
* [Slint 1.3 Released with Revamped Native Styles and JavaScript API](https://slint.dev/blog/slint-1.3-released)
* [rustc_codegen_gcc: Progress Report #27](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-27)
* [rust-analyzer changelog #207](https://rust-analyzer.github.io/thisweek/2023/11/13/changelog-207.html)

- [`breadcrumbs 0.1.4`](https://crates.io/crates/breadcrumbs) released - now the smallest logging library on crates.io, supporting `#[no_std]` and concurrency.

### Observations/Thoughts
* [Why Rust in Production?](https://corrode.dev/why-rust/)
* [Building a Web App in Rust](https://www.yieldcode.blog/post/building-a-webapp-in-rust/)
* [Rust without crates.io](https://thomask.sdf.org/blog/2023/11/14/rust-without-crates-io.html)
* [How I Improved My Rust Compile Times by 75%](https://benw.is/posts/how-i-improved-my-rust-compile-times-by-seventy-five-percent)
* [Iterator as an Alias](https://blog.yoshuawuyts.com/iterator-as-an-alias/)
* [What is a team?](https://blog.yoshuawuyts.com/what-is-a-team/)

### Rust Walkthroughs
* [Building and Deploying A Static Site Generator](https://www.shuttle.rs/blog/2023/11/15/ssg-in-rust)
* [Tracking the current active process in Windows with Rust](https://hellocode.co/blog/post/tracking-active-process-windows-rust/)
* [Edge IoT with Rust on ESP: MQTT Subscriber](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-mqtt-subscriber)
* [Building a Central Authentication Server with Rust, PostgreSQL, Kafka and gRPC](https://medium.com/@adefemiadeoye/building-a-central-authentication-server-with-rust-postgresql-kafka-and-grpc-f1b44de099ea)
* [video] [An Introduction to Veilid, by Christien Rioux](https://www.youtube.com/watch?v=h288gZTjJOM)
* [video] [Code in Rust with RustRover, by Vitaly Bragilevsky](https://www.youtube.com/watch?v=pnFS0YIKUJ8)
* [video] [Create a Dummy GitHub CLI in Rust!](https://www.youtube.com/watch?v=pyeUkQg8z9A)

### Research

### Miscellaneous
* [audio] [RustShip: Corrode.dev and lychee with Matthias Endler](https://ieni.dev/2023/11/%EF%B8%8F-corrode.dev-and-lychee-with-matthias-endler-rustship-5/)

## Crate of the Week

This week's crate is [cargo-msrv](https://github.com/foresterre/cargo-msrv), a cargo subcommand to find out the Minimum Supported Rust Version (MSRV) of your crate.

llogiq is a bit worried about not having received suggestions for two weeks in a row, but still offers you his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

364 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-06..2023-11-13

* [speed up `x clean`](https://github.com/rust-lang/rust/pull/117723)
* [llvm-wrapper: remove include of non-existant Vectorize.h](https://github.com/rust-lang/rust/pull/117675)
* [`rustc_llvm`: Link to libkstat on Solaris/SPARC](https://github.com/rust-lang/rust/pull/114224)
* [add -Zcross-crate-inline-threshold=yes](https://github.com/rust-lang/rust/pull/117650)
* [add `std::hash::{DefaultHasher, RandomState}` exports](https://github.com/rust-lang/rust/pull/115694)
* [add a new `download-ci-llvm = if-unchanged` option and enable it by default for `profile = codegen`](https://github.com/rust-lang/rust/pull/116881)
* [allow configuring the parent GitHub repository](https://github.com/rust-lang/rust/pull/117122)
* [build a better MIR body when errors are encountered](https://github.com/rust-lang/rust/pull/117418)
* [compute layout with spans for better cycle errors in coroutines](https://github.com/rust-lang/rust/pull/117858)
* [compute polonius loan scopes over the region graph](https://github.com/rust-lang/rust/pull/117560)
* [coverage: avoid creating malformed macro name spans](https://github.com/rust-lang/rust/pull/117827)
* [coverage: rename the `run-coverage` test mode to `coverage-run`](https://github.com/rust-lang/rust/pull/117700)
* [deny more `~const` trait bounds](https://github.com/rust-lang/rust/pull/117817)
* [extend builtin/auto trait args with error when they have \>1 argument](https://github.com/rust-lang/rust/pull/117645)
* [format macro const literals with pretty printer](https://github.com/rust-lang/rust/pull/115485)
* [generator layout: ignore fake borrows](https://github.com/rust-lang/rust/pull/117712)
* [give a better diagnostic for missing parens in Fn* bounds](https://github.com/rust-lang/rust/pull/117297)
* [handle the case when the change-id isn't found](https://github.com/rust-lang/rust/pull/117263)
* [improve diagnostic for const ctors in array repeat expressions](https://github.com/rust-lang/rust/pull/113925)
* [make `FatalErrorMarker` lower priority than other panics](https://github.com/rust-lang/rust/pull/117557)
* [on method chain expression failure, look for missing method in earlier segments of the chain](https://github.com/rust-lang/rust/pull/115229)
* [only instantiate binder during dyn's built-in trait candidate probe once](https://github.com/rust-lang/rust/pull/117610)
* [only use `normalize_param_env` when normalizing predicate in `check_item_bounds`](https://github.com/rust-lang/rust/pull/117542)
* [patterns: reject raw pointers that are not just integers](https://github.com/rust-lang/rust/pull/116930)
* [recover from incorrectly ordered/duplicated function keywords](https://github.com/rust-lang/rust/pull/117282)
* [reorder checks to make sure potential missing expect on Option/Result…](https://github.com/rust-lang/rust/pull/117695)
* [restore rustc shim error message](https://github.com/rust-lang/rust/pull/117724)
* [catch stray `{` in let-chains](https://github.com/rust-lang/rust/pull/117770)
* [suggest removing `;` for `;` within let-chains](https://github.com/rust-lang/rust/pull/117743)
* [thir unsafeck fixes](https://github.com/rust-lang/rust/pull/117229)
* [warn when using an unstable feature with -Ctarget-feature](https://github.com/rust-lang/rust/pull/117616)
* [when not finding assoc fn on type, look for builder fn](https://github.com/rust-lang/rust/pull/117006)
* [miri: `data_race`: link to docs for 'unusual' race conditions](https://github.com/rust-lang/miri/pull/3155)
* [miri: freebsd adding getentropy interception support](https://github.com/rust-lang/miri/pull/3161)
* [miri: implement round.ps and round.pd SSE4.1 intrinsics](https://github.com/rust-lang/miri/pull/3159)
* [miri: share getentropy shim across various unixes](https://github.com/rust-lang/miri/pull/3162)
* [miri: treat thread-local statics on main thread as static roots for leakage analysis](https://github.com/rust-lang/miri/pull/2931)
* [emit `#[inline]` on `derive(Debug)`](https://github.com/rust-lang/rust/pull/117727)
* [stabilize `result_option_inspect`](https://github.com/rust-lang/rust/pull/116866)
* [move `BorrowedBuf` and `BorrowedCursor` from `std:io` to `core::io`](https://github.com/rust-lang/rust/pull/117694)
* [closure-consuming helper functions for `fmt::Debug` helpers](https://github.com/rust-lang/rust/pull/117730)
* [don't panic in `<BorrowedCursor as io::Write>::write`](https://github.com/rust-lang/rust/pull/115460)
* [futures: provide a non-destructive mechanism to determine if a sink/stream are paired](https://github.com/rust-lang/futures-rs/pull/2797)
* [codegen-cranelift: implement AArch64 intrinsics necessary for simd-json](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1416)
* [codegen-cranelift: implement AES-NI and SHA256 crypto intrinsics using inline asm](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1425)
* [codegen-cranelift: implement a lot of SIMD intrinsics](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1417)
* [codegen\_gcc: do not emit `.eh_frame` section if using -Cpanic=abort](https://github.com/rust-lang/rustc_codegen_gcc/pull/374)
* [cargo: `query`{`_vec`} use `IndexSummary`](https://github.com/rust-lang/cargo/pull/12970)
* [cargo: add better error message when it can not find the search section](https://github.com/rust-lang/cargo/pull/12865)
* [cargo: add cache garbage collection](https://github.com/rust-lang/cargo/pull/12634)
* [cargo credential: include license files in all published crates](https://github.com/rust-lang/cargo/pull/12953)
* [cargo: do not allow empty feature name](https://github.com/rust-lang/cargo/pull/12928)
* [cargo: make browser links out of HTML file paths](https://github.com/rust-lang/cargo/pull/12889)
* [cargo: filter `cargo-credential-*` dependencies by OS](https://github.com/rust-lang/cargo/pull/12949)
* [cargo: fix `--quiet` being used with nested subcommands](https://github.com/rust-lang/cargo/pull/12959)
* [cargo: fix non-deterministic behavior in last-use repopulation](https://github.com/rust-lang/cargo/pull/12958)
* [cargo: do not panic when failed to parse rustc commit-hash](https://github.com/rust-lang/cargo/pull/12965)
* [cargo: preserve jobserver file descriptors on rustc invocation in `fix_exec_rustc`](https://github.com/rust-lang/cargo/pull/12951)
* [cargo: report more detailed semver errors](https://github.com/rust-lang/cargo/pull/12924)
* [rustdoc: properly elide cross-crate host effect args](https://github.com/rust-lang/rust/pull/117531)
* [clippy: `arc_with_non_send_sync` Improve suggested resolution](https://github.com/rust-lang/rust-clippy/pull/11772)
* [clippy: `map_identity`: respect match ergonomics](https://github.com/rust-lang/rust-clippy/pull/11792)
* [clippy: `mod_module_files` Don't emit lint for mod.rs in tests](https://github.com/rust-lang/rust-clippy/pull/11779)
* [clippy: add type details to `unnecessary_fallible_conversions` note](https://github.com/rust-lang/rust-clippy/pull/11767)
* [clippy: destructure `Conf` in `register_lints`](https://github.com/rust-lang/rust-clippy/pull/11790)
* [clippy: disable `vec_box` when using different allocators](https://github.com/rust-lang/rust-clippy/pull/11780)
* [clippy: don't check for late-bound vars, check for escaping bound vars](https://github.com/rust-lang/rust-clippy/pull/11760)
* [clippy: fixes to `manual_let_else`'s divergence check](https://github.com/rust-lang/rust-clippy/pull/11787)
* [clippy: lint `needless_borrow` and `explicit_auto_deref` on most union field accesses](https://github.com/rust-lang/rust-clippy/pull/11508)
* [clippy: move `suspicious_doc_comments` to doc pass](https://github.com/rust-lang/rust-clippy/pull/11798)
* [clippy: replace `if_chain` with let chains](https://github.com/rust-lang/rust-clippy/pull/11750)
* [rust-analyzer: add config for preferring / ignoring prelude modules when inserting imports](https://github.com/rust-lang/rust-analyzer/pull/15871)
* [rust-analyzer: preview adt field when hover](https://github.com/rust-lang/rust-analyzer/pull/15847)
* [rust-analyzer: find `Self` reference](https://github.com/rust-lang/rust-analyzer/pull/15864)
* [rust-analyzer: ignore `doc(hidden)` attr if no body is present](https://github.com/rust-lang/rust-analyzer/pull/15854)
* [rust-analyzer: truncate closure capture place for raw pointer](https://github.com/rust-lang/rust-analyzer/pull/15860)
* [rust-analyzer: improve check for include macro](https://github.com/rust-lang/rust-analyzer/pull/15866)

### Rust Compiler Performance Triage

A week dominated by one particular perf improvement that lead to huge performance gains - an avg of 5% improvement across 121 test cases! The perf improvement comes from adding an `#[inline]` hint to the output from `#[derive(Debug)]` which presumably allows the compiler to more easily do deadcode elimination reducing the binary size and the amount of code that actually needs to be code-gened.

Triage done by **@rylev**.
Revision range: [7b97a5ca..173b6e68](https://perf.rust-lang.org/?start=7b97a5ca8422d1495a8918106d3249aa405812d4&end=173b6e686b158dbad7d072c64bef3ced2052312b&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 0.9%]    | 10    |
| Regressions ❌ <br /> (secondary)  | 1.9%  | [0.2%, 3.6%]    | 12    |
| Improvements ✅ <br /> (primary)   | -5.6% | [-49.2%, -0.1%] | 111   |
| Improvements ✅ <br /> (secondary) | -3.5% | [-25.0%, -0.2%] | 155   |
| All ❌✅ (primary)                 | -5.1% | [-49.2%, 0.9%]  | 121   |


2 Regressions, 2 Improvements, 3 Mixed; 3 of them in rollups
55 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/e036aa707afc1495783004ee018aada4dfa9d192/triage/2023-11-14.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Superseding public/private dependencies](https://github.com/rust-lang/rfcs/pull/3516)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for mutex_unpoison](https://github.com/rust-lang/rust/issues/96469)
* [disposition: merge] [Tracking issue for dyn upcasting coercion](https://github.com/rust-lang/rust/issues/65991)
* [disposition: merge] [rustdoc-search: add support for traits and associated types](https://github.com/rust-lang/rust/pull/116085)

#### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

#### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guidelines entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Add named path bases to cargo (v2)](https://github.com/rust-lang/rfcs/pull/3529)
* [new] [RFC: Associated const underscore](https://github.com/rust-lang/rfcs/pull/3527)
* [new] [Add forbidden function casts RFC](https://github.com/rust-lang/rfcs/pull/3526)
* [new] [Struct target features RFC](https://github.com/rust-lang/rfcs/pull/3525)
* [new] [Create 0000-cargo-dns.md](https://github.com/rust-lang/rfcs/pull/3523)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-11-15 - 2023-12-13 🦀

### Virtual

* 2023-11-15 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Building Our Own Locks (Atomics & Locks Chapter 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 2023-11-15 | Virtual (Richmond, VA, US) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Nightly Night: impl Trait in Type Aliases**](https://www.meetup.com/vancouver-rust/events/296600976/)
* 2023-11-16 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833657/)
* 2023-11-16 | Virtual (San Diego, CA, US) | [San Diego Rust](https://www.meetup.com/san-diego-rust/)
    * [**San Diego Rust November 2023 Tele-Meetup**](https://www.meetup.com/san-diego-rust/events/297376463/)
* 2023-11-16 | Virtual (Vilnius, LT) | [Vilnius Rust and Go Meetup Group](https://www.meetup.com/vilnius-rust-go-meetup-group/)
    * [**Enjoy our first Rust event**](https://www.meetup.com/vilnius-rust-go-meetup-group/events/297133832/)
* 2023-11-21 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679794/)
* 2023-11-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/296807537/)
* 2023-11-23 | Virtual (Edmonton, AB, CA) | [Edmonton R User Group - Yegrug](https://www.meetup.com/edmonton-r-user-group-yegrug/)
    * [**Edmonton R User Group Meetup: R and Rust, like a match made in heaven**](https://www.meetup.com/edmonton-r-user-group-yegrug/events/296605221/)
* 2023-11-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcpblc/)
* 2023-11-29 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Atomics & Locks Book Club Final Chapter! (Chapter 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 2023-11-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 2023-11-30 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Automating expertise with cargo-semver-checks**](https://www.meetup.com/rust-dublin/events/296346693/)
* 2023-12-01 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 2023-12-02 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 2023-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Mirror**](https://berline.rs/)
* 2023-12-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)


### Europe

* 2023-11-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**GPU processing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504264/)
* 2023-11-23 | Biel/Bienne, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Talks Bern @ Biel: Embedded Edition**](https://www.meetup.com/rust-bern/events/296556498/)
* 2023-12-07 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 2023-11-30 | Brussels, BE | [Lambda Brussels](https://lambda-brussels.glitch.me/)
    * [**Lambda Brussels**](https://lambda-brussels.glitch.me/)

### North America

* 2023-11-15 | Richmond, VA, US + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)
* 2023-11-16 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297062689/)
* 2023-11-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Python loves Rust!**](https://www.meetup.com/music-city-rust-developers/events/296916567/)
* 2023-11-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295483924)
* 2023-11-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/296917625/)
* 2023-11-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcpbdc/)
* 2023-11-28 | Pasadena, CA, US | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/297062186/)
* 2023-12-12 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)

### Oceania

* 2023-11-21 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296819540/)
* 2023-11-28 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/296391733/)
* 2023-12-11 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust End of Year Event**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

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

> I decided to keep learning Rust because I liked the syntax. I liked the speed. I liked the community. I liked it all. It felt like a breath of fresh air: a syntax more intuitive than Python, JavaScript, or C, yet still faster.

– [Goren Barak on their blog](https://digital-goobers.vercel.app/posts/learning-rust)

Thanks to [Goren Barak](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1488) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
