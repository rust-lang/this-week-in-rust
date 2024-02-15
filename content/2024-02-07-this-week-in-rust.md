Title: This Week in Rust 533
Number: 533
Date: 2024-02-07
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
* [crates.io: API status code changes](https://blog.rust-lang.org/2024/02/06/crates-io-status-codes.html)

### Foundation
* [Google Contributes $1M to Rust Foundation to Support C++/Rust "Interop Initiative"](https://foundation.rust-lang.org/news/google-contributes-1m-to-rust-foundation-to-support-c-rust-interop-initiative/)

### Project/Tooling Updates
* [Announcing the Tauri v2 Beta Release](https://beta.tauri.app/blog/tauri-2-0-0-beta/)
* [Polars — Why we have rewritten the string data type](https://pola.rs/posts/polars-string-type/)
* [rust-analyzer changelog #219](https://rust-analyzer.github.io/thisweek/2024/02/05/changelog-219.html)
* [Ratatui 0.26.0 - a Rust library for cooking up terminal user interfaces](https://ratatui.rs/highlights/v026/)

### Observations/Thoughts
* [Will it block?](https://blog.yoshuawuyts.com/what-is-blocking/)
* [Embedded Rust in Production ..?](https://blog.lohr.dev/embedded-rust)
* [Let futures be futures](https://without.boats/blog/let-futures-be-futures/)
* [Compiling Rust is testing](https://kobzol.github.io/rust/2024/02/04/compiling-rust-is-testing.html)
* [Rust web frameworks have subpar error reporting](https://www.lpalmieri.com/posts/rust-web-frameworks-have-subpar-error-reporting/)
* [video] [Proving Performance - FOSDEM 2024 - Rust Dev Room](https://www.youtube.com/watch?v=P87C4jNakGs)
* [video] [Stefan Baumgartner - Trials, Traits, and Tribulations](https://www.youtube.com/watch?v=DH9HIBbpktY)
* [video] [Rainer Stropek - Memory Management in Rust](https://www.youtube.com/watch?v=5yy64sy2oSM)
* [video] [Shachar Langbeheim - Async & FFI - not exactly a love story](https://www.youtube.com/watch?v=z3tpB94VKwU)
* [video] [Massimiliano Mantione - Object Oriented Programming, and Rust](https://www.youtube.com/watch?v=XkCHjuF5Qps)
* [audio] [Unlocking Rust's power through mentorship and knowledge spreading, with Tim McNamara](https://rustacean-station.org/episode/tim-mcnamara/)
* [audio] [Asciinema with Marcin Kulik](https://rustacean-station.org/episode/marcin-kulik/)
* [Non-Affine Types, ManuallyDrop and Invariant Lifetimes in Rust - Part One](https://asyncmove.com/blog/2024/02/non-affine-types-manuallydrop-and-invariant-lifetimes-in-rust-part-one/)
* [Nine Rules for Accessing Cloud Files from Your Rust Code: Practical lessons from upgrading Bed-Reader, a bioinformatics library](https://towardsdatascience.com/nine-rules-for-accessing-cloud-files-from-your-rust-code-d456c1e2ceb4)

### Rust Walkthroughs
* [AsyncWrite and a Tale of Four Implementations](https://medium.com/@razieh.behjati/asyncwrite-and-a-tale-of-four-implementations-e63aef8397f7)
* [Garbage Collection Without Unsafe Code](https://fitzgeraldnick.com/2024/02/06/safe-gc.html)
* [Fragment specifiers in Rust Macros](https://anoopelias.github.io/posts/fragment-specifiers-in-rust-macros/)
* [Writing a REST API in Rust](https://www.shuttle.rs/blog/2024/01/31/write-a-rest-api-rust)
* [video] [Traits and operators](https://www.youtube.com/watch?v=q6T5MsVmz7g)
* [Write a simple netcat client and server in Rust](https://developerlife.com/2024/01/13/write-simple-netcat-in-rust/)

### Miscellaneous
* [RustFest 2024 Announcement](https://rustfest.ch/posts/2024-02-01/announcement/)
* [All EuroRust 2023 talks ordered by the view count](https://techtalksweekly.substack.com/p/all-eurorust-2023-talks-ordered-by)

## Crate of the Week

This week's crate is [embedded-cli-rs](https://github.com/funbiscuit/embedded-cli-rs), a library that makes it easy to create CLIs on embedded devices.

Thanks to [Sviatoslav Kokurin](https://users.rust-lang.org/t/crate-of-the-week/2704/1286) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Fluvio - Build a new python wrapping for the fluvio client crate](https://github.com/infinyon/fluvio/issues/3842)
* [Fluvio - MQTT Connector: Prefix auto generated Client ID to prevent connection drops](https://github.com/infinyon/fluvio/issues/3825)
* [Ockam - Implement events in `SqlxDatabase`](https://github.com/build-trust/ockam/issues/7116)
* [Ockam - Output for both `ockam project ticket` and `ockam project enroll` is improved, with support for `--output json`](https://github.com/build-trust/ockam/issues/7473)
* [Ockam - Output for ockam project ticket is improved and information is not opaque ](https://github.com/build-trust/ockam/issues/7478)
* [Hyperswitch - [FEATURE]: Setup code coverage for local tests & CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - [FEATURE]: Have get_required_value to use ValidationError in OptionExt](https://github.com/juspay/hyperswitch/issues/860)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker. 

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [RustNL 2024](https://2024.rustnl.org/) [CFP](https://cryptpad.fr/form/#/2/form/view/WtBXS48XjwAvYXIjhgEY6yJ0EOpULX+zByvkHds2oUA/) closes 2024-02-19 | Delft, The Netherlands | Event date: 2024-05-07 & 2024-05-08
* [NDC Techtown](https://ndctechtown.com/call-for-papers) CFP closes 2024-04-14 | Kongsberg, Norway | Event date: 2024-09-09 to 2024-09-12

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

309 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-01-30..2024-02-06

* [add avx512fp16 to x86 target features](https://github.com/rust-lang/rust/pull/119543)
* [riscv only supports `split_debuginfo=off` for now](https://github.com/rust-lang/rust/pull/120518)
* [target: default to the medium code model on LoongArch targets](https://github.com/rust-lang/rust/pull/120661)
* [`#![feature(inline_const_pat)]` is no longer incomplete](https://github.com/rust-lang/rust/pull/120547)
* [actually abort in -Zpanic-abort-tests](https://github.com/rust-lang/rust/pull/120326)
* [add missing `potential_query_instability` for keys and values in hashmap](https://github.com/rust-lang/rust/pull/120485)
* [avoid ICE when `is_val_statically_known` is not of a supported type](https://github.com/rust-lang/rust/pull/120484)
* [be more careful about interpreting a label/lifetime as a mistyped char literal](https://github.com/rust-lang/rust/pull/120460)
* [check `RUST_BOOTSTRAP_CONFIG` in `profile_user_dist` test](https://github.com/rust-lang/rust/pull/120207)
* [correctly check `never_type` feature gating](https://github.com/rust-lang/rust/pull/120552)
* [coverage: improve handling of function/closure spans](https://github.com/rust-lang/rust/pull/120569)
* [coverage: use normal `edition`: headers in coverage tests](https://github.com/rust-lang/rust/pull/120566)
* [deduplicate more sized errors on call exprs](https://github.com/rust-lang/rust/pull/120293)
* [`pattern_analysis`: Gracefully abort on type incompatibility](https://github.com/rust-lang/rust/pull/120313)
* [`pattern_analysis`: cleanup manual impls](https://github.com/rust-lang/rust/pull/120516)
* [`pattern_analysis`: cleanup the contexts](https://github.com/rust-lang/rust/pull/120321)
* [fix BufReader unsoundness by adding a check in `default_read_buf`](https://github.com/rust-lang/rust/pull/120607)
* [fix ICE on field access on a tainted type after const-eval failure](https://github.com/rust-lang/rust/pull/120616)
* [hir: refactor getters for owner nodes](https://github.com/rust-lang/rust/pull/120346)
* [hir: remove the generic type parameter from `MaybeOwned`](https://github.com/rust-lang/rust/pull/120610)
* [improve the diagnostics for unused generic parameters](https://github.com/rust-lang/rust/pull/120556)
* [introduce support for `async` bound modifier on `Fn*` traits](https://github.com/rust-lang/rust/pull/120392)
* [make matching on NaN a hard error, and remove the rest of `illegal_floating_point_literal_pattern`](https://github.com/rust-lang/rust/pull/116284)
* [make the coroutine def id of an async closure the child of the closure def id](https://github.com/rust-lang/rust/pull/120402)
* [miscellaneous diagnostics cleanups](https://github.com/rust-lang/rust/pull/120571)
* [move UI issue tests to subdirectories](https://github.com/rust-lang/rust/pull/120439)
* [move predicate, region, and const stuff into their own modules in middle](https://github.com/rust-lang/rust/pull/120497)
* [never patterns: It is correct to lower `!` to `_`](https://github.com/rust-lang/rust/pull/120517)
* [normalize region obligation in lexical region resolution with next-gen solver](https://github.com/rust-lang/rust/pull/119101)
* [only suggest removal of `as_*` and `to_` conversion methods on E0308](https://github.com/rust-lang/rust/pull/120473)
* [provide more context on derived obligation error primary label](https://github.com/rust-lang/rust/pull/120469)
* [suggest changing type to const parameters if we encounter a type in the trait bound position](https://github.com/rust-lang/rust/pull/120570)
* [suppress unhelpful diagnostics for unresolved top level attributes](https://github.com/rust-lang/rust/pull/118533)
* [miri: normalize `struct` tail in ABI compat check](https://github.com/rust-lang/rust/pull/120587)
* [miri: moving out `sched_getaffinity` interception from linux'shim, FreeBSD su…](https://github.com/rust-lang/miri/pull/3283)
* [miri: switch over to rustc's `tracing` crate instead of using our own `log` crate](https://github.com/rust-lang/miri/pull/2956)
* [revert unsound libcore changes](https://github.com/rust-lang/rust/pull/120562)
* [fix some `Arc` allocator leaks](https://github.com/rust-lang/rust/pull/120445)
* [use `<T, U>` for array/slice equality `impl`s](https://github.com/rust-lang/rust/pull/120384)
* [improve `io::Read::read_buf_exact` error case](https://github.com/rust-lang/rust/pull/120523)
* [reject infinitely-sized reads from `io::Repeat`](https://github.com/rust-lang/rust/pull/119991)
* [`thread_local::register_dtor` fix proposal for FreeBSD](https://github.com/rust-lang/rust/pull/120430)
* [add LocalWaker and ContextBuilder types to core, and LocalWake trait to alloc](https://github.com/rust-lang/rust/pull/118960)
* [codegen\_gcc: improve iterator for files suppression](https://github.com/rust-lang/rustc_codegen_gcc/pull/416)
* [cargo: Don't panic on empty spans](https://github.com/rust-lang/cargo/pull/13375)
* [cargo: Improve map/sequence error message](https://github.com/rust-lang/cargo/pull/13376)
* [cargo: apply `-Zpanic-abort-tests` to doctests too](https://github.com/rust-lang/cargo/pull/13388)
* [cargo: don't print rustdoc command lines on failure by default](https://github.com/rust-lang/cargo/pull/13387)
* [cargo: stabilize lockfile v4](https://github.com/rust-lang/cargo/pull/12852)
* [cargo: fix markdown line break in cargo-add](https://github.com/rust-lang/cargo/pull/13400)
* [cargo: use spec id instead of name to match package](https://github.com/rust-lang/cargo/pull/13335)
* [rustdoc: fix footnote handling](https://github.com/rust-lang/rust/pull/120443)
* [rustdoc: correctly handle attribute merge if this is a glob reexport](https://github.com/rust-lang/rust/pull/120501)
* [rustdoc: prevent JS injection from localStorage](https://github.com/rust-lang/rust/pull/120250)
* [rustdoc: trait.impl, type.impl: sort impls to make it not depend on serialization order](https://github.com/rust-lang/rust/pull/120641)
* [clippy: `redundant_locals`: take by-value closure captures into account](https://github.com/rust-lang/rust-clippy/pull/12227)
* [clippy: new lint: `manual_c_str_literals`](https://github.com/rust-lang/rust-clippy/pull/11919)
* [clippy: add `lint_groups_priority` lint](https://github.com/rust-lang/rust-clippy/pull/11832)
* [clippy: add new lint: `ref_as_ptr`](https://github.com/rust-lang/rust-clippy/pull/12087)
* [clippy: add configuration for `wildcard_imports` to ignore certain imports](https://github.com/rust-lang/rust-clippy/pull/11979)
* [clippy: avoid deleting labeled blocks](https://github.com/rust-lang/rust-clippy/pull/12219)
* [clippy: fixed FP in `unused_io_amount` for Ok(lit), unrachable! and unwrap de…](https://github.com/rust-lang/rust-clippy/pull/12217)
* [rust-analyzer: "Normalize import" assist and utilities for normalizing use trees](https://github.com/rust-lang/rust-analyzer/pull/16417)
* [rust-analyzer: enable excluding refs search results in test](https://github.com/rust-lang/rust-analyzer/pull/16441)
* [rust-analyzer: support for GOTO def from *inside* files included with `include!` macro](https://github.com/rust-lang/rust-analyzer/pull/16439)
* [rust-analyzer: emit parser error for missing argument list](https://github.com/rust-lang/rust-analyzer/pull/16493)
* [rust-analyzer: swap `Subtree::token_trees` from `Vec` to boxed slice](https://github.com/rust-lang/rust-analyzer/pull/16482)

### Rust Compiler Performance Triage

Rust's CI was down most of the week, leading to a much smaller collection of
commits than usual. Results are mostly neutral for the week.

Triage done by **@simulacrum**.
Revision range: [5c9c3c78..0984bec](https://perf.rust-lang.org/?start=5c9c3c7871d603ba13d38372830eca0c9013e575&end=0984becf01112cbd3583c796541760b65340c8db&absolute=false&stat=instructions%3Au)

0 Regressions, 2 Improvements, 1 Mixed; 1 of them in rollups
17 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-02-05.md)

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
* [disposition: merge] [Consider principal trait ref's auto-trait super-traits in dyn upcasting](https://github.com/rust-lang/rust/pull/119338)
* [disposition: merge] [remove `sub_relations` from the `InferCtxt`](https://github.com/rust-lang/rust/pull/119989)
* [disposition: merge] [Optimize away poison guards when std is built with panic=abort](https://github.com/rust-lang/rust/pull/100603)
* [disposition: merge] [Check normalized call signature for WF in mir typeck](https://github.com/rust-lang/rust/pull/118882)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Nested function scoped type parameters](https://github.com/rust-lang/rfcs/pull/3562)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2024-02-07 - 2024-03-06 🦀

### Virtual

* 2024-02-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Ezra Singh - How Rust Saved My Eyes**](https://www.meetup.com/indyrs/events/298641965/)
* 2024-02-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251149/)
* 2024-02-08 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945246/)
* 2024-02-10 | Virtual (Krakow, PL) | [Stacja IT Kraków](https://www.meetup.com/stacja-it-krakow/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-krakow/events/298303129/)
* 2024-02-10 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-wroclaw/events/298303130/)
* 2024-02-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341575/)
* 2024-02-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack n Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 2024-02-15 | Virtual + In person (Praha, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-19 | Virtual (Melbourne, VIC, AU) | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**February 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 2024-02-20 | Virtual | [Rust for Lunch](https://lunch.rs/about/)
    * [**Lunch**](https://lunch.rs/meetups/2024-02-20/)
* 2024-02-21 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 2 - Types**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298991687/)
* 2024-02-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 2024-02-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)

### Asia

* 2024-02-10 | Hyderabad, IN | [Rust Language Hyderabad](https://www.meetup.com/rust-hyderabad/)
    * [**Rust Language Develope BootCamp**](https://www.meetup.com/rust-hyderabad/events/298687498/)

### Europe

* 2024-02-07 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Embedded Abstractions**](https://www.meetup.com/rustcologne/events/298913201/) | [**Event page**](https://rust.cologne/2024/02/07/embedded-hal.html)
* 2024-02-07 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust for the Web — Mainmatter x Shuttle Takeover**](https://www.meetup.com/rust-london-user-group/events/298413388/)
* 2024-02-08 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern/)
    * [**Rust Bern Meetup #1 2024 🦀**](https://www.meetup.com/rust-bern/events/298488858/)
* 2024-02-08 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Rust-based banter**](https://www.meetup.com/rust-oslo/events/298861296/)
* 2024-02-13 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Building Games with Rust: Dive into the Bevy Framework**](https://www.meetup.com/rust-trondheim/events/298838682/)
* 2024-02-15 | Praha, CZ - Virtual + In-person | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 2024-02-22 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)

### North America

* 2024-02-07 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/297635028/)
* 2024-02-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**BEAST: Recreating a classic DOS terminal game in Rust**](https://www.meetup.com/utah-rust/events/298888955/)
* 2024-02-12 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust: Open Source Contrib Hackathon & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760219/)
* 2024-02-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer**](https://www.meetup.com/rust-nyc/events/298593474/)
* 2024-02-13 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564994/)
* 2024-02-15 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, Feb 15**](https://www.meetup.com/bostonrust/events/297635043/)
* 2024-02-15 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631774/)
* 2024-02-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/298603354/)
* 2024-02-22 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043763/)
* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)

### Oceania

* 2024-02-19 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**February 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 2024-02-27 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/297650401/)
* 2024-02-27 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/18t4wtx/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> My take on this is that you cannot use async Rust correctly and fluently without understanding Arc, Mutex, the mutability of variables/references, and how async and await syntax compiles in the end. Rust forces you to understand how and why things are the way they are. It gives you minimal abstraction to do things that could’ve been tedious to do yourself.
> 
> I got a chance to work on two projects that drastically forced me to understand how async/await works. The first one is to transform a library that is completely sync and only requires a sync trait to talk to the outside service. This all sounds fine, right? Well, this becomes a problem when we try to port it into browsers. The browser is single-threaded and cannot block the JavaScript runtime at all! It is arguably the most weird environment for Rust users. It is simply impossible to rewrite the whole library, as it has already been shipped to production on other platforms.
> 
> What we did instead was rewrite the network part using async syntax, but using our own generator. The idea is simple: the generator produces a future when called, and the produced future can be awaited. But! The produced future contains an arc pointer to the generator. That means we can feed the generator the value we are waiting for, then the caller who holds the reference to the generator can feed the result back to the function and resume it. For the browser, we use the native browser API to derive the network communications; for other platforms, we just use regular blocking network calls. The external interface remains unchanged for other platforms.
> 
> Honestly, I don’t think any other language out there could possibly do this. Maybe C or C++, but which will never have the same development speed and developer experience.
> 
> I believe people have already mentioned it, but the current asynchronous model of Rust is the most reasonable choice. It does create pain for developers, but on the other hand, there is no better asynchronous model for Embedded or WebAssembly.
                    

– [/u/Top_Outlandishness78 on /r/rust](https://reddit.com/r/rust/comments/1ai1a97/let_futures_be_futures/korxtar/)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1521) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1alofun/this_week_in_rust_533/)</small>
