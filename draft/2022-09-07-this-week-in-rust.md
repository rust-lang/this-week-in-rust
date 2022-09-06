Title: This Week in Rust 459
Number: 459
Date: 2022-09-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [sql-query-builder](https://crates.io/crates/sql_query_builder), a library to write SQL queries in a simple and composable way.

Thanks to [Belchior Oliveira](https://users.rust-lang.org/t/crate-of-the-week/2704/1102) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

417 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-08-29..2022-09-05

* [add tier-3 support for powerpc64 and riscv64 openbsd](https://github.com/rust-lang/rust/pull/101025)
* [support `#[unix_sigpipe = "inherit|sig_dfl"]` on `fn main()` to prevent ignoring `SIGPIPE`](https://github.com/rust-lang/rust/pull/97802)
* [proc\_macro/bridge: send diagnostics over the bridge as a struct](https://github.com/rust-lang/rust/pull/100210)
* [proc\_macro/bridge: use the cross-thread executor for nested proc-macros](https://github.com/rust-lang/rust/pull/101414)
* [do not leak type variables from opaque type relation](https://github.com/rust-lang/rust/pull/99928)
* [attempt to normalize `FnDef` signature in `InferCtxt::cmp`](https://github.com/rust-lang/rust/pull/100473)
* [do not report too many expr field candidates](https://github.com/rust-lang/rust/pull/100898)
* [do not suggest adding `move` to closure when `move` is already used](https://github.com/rust-lang/rust/pull/101285)
* [don't suggest reborrow if usage is inside a closure](https://github.com/rust-lang/rust/pull/101429)
* [suggest `{Option,Result}::{copied,clone}()` to satisfy type mismatch](https://github.com/rust-lang/rust/pull/101367)
* [suggest associated method on deref types when path syntax method fails](https://github.com/rust-lang/rust/pull/100302)
* [suggest moving redundant generic args of an assoc fn to its trait](https://github.com/rust-lang/rust/pull/100838)
* [suggest returning closure as `impl Fn`](https://github.com/rust-lang/rust/pull/101019)
* [add `special_module_name` lint](https://github.com/rust-lang/rust/pull/94467)
* [uplift the `let_underscore` lints from clippy into rustc](https://github.com/rust-lang/rust/pull/97739)
* [strengthen `invalid_value` lint to forbid uninit primitives, adjust docs to say that's UB](https://github.com/rust-lang/rust/pull/98919)
* [forbid mixing `System` with direct sytem allocator calls](https://github.com/rust-lang/rust/pull/101394)
* [use head span for `rustc_on_unimplemented`'s `enclosing_scope` attr](https://github.com/rust-lang/rust/pull/101296)
* [make call suggestions more general and more accurate](https://github.com/rust-lang/rust/pull/101100)
* [make trait bound not satisfied specify kind](https://github.com/rust-lang/rust/pull/100647)
* [miri: adjust for supporting more implicit ptr-to-int transmutation](https://github.com/rust-lang/miri/pull/2516)
* [miri: re-enable FFI support](https://github.com/rust-lang/miri/pull/2529)
* [allow deriving multipart suggestions](https://github.com/rust-lang/rust/pull/100970)
* [replace `rustc_data_structures::thin_vec::ThinVec` with `thin_vec::ThinVec`](https://github.com/rust-lang/rust/pull/100869)
* [separate the receiver from arguments in HIR](https://github.com/rust-lang/rust/pull/101261)
* [shrink `thir::Pat`](https://github.com/rust-lang/rust/pull/101139)
* [shrink suggestion span of argument mismatch error](https://github.com/rust-lang/rust/pull/101364)
* [simplify `hir::PathSegment`](https://github.com/rust-lang/rust/pull/101228)
* [interpret: fix unnecessary allocation in validation visitor](https://github.com/rust-lang/rust/pull/101154)
* [more `clippy::perf` fixes](https://github.com/rust-lang/rust/pull/101391)
* [optimization of access level table construction](https://github.com/rust-lang/rust/pull/100147)
* [migrate `rustc_session` to use `SessionDiagnostic` - Pt. 1](https://github.com/rust-lang/rust/pull/100753)
* [migrate `rustc_metadata` to `SessionDiagnostics`](https://github.com/rust-lang/rust/pull/100928)
* [migrate `rustc_monomorphize` to use `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100730)
* [porting 'compiler/rustc_trait_selection' to translatable diagnostics - Part 1](https://github.com/rust-lang/rust/pull/100814)
* [fix UB from misalignment and provenance widening in `std::sys::windows`](https://github.com/rust-lang/rust/pull/101171)
* [avoid needless buffer zeroing in `std::sys::windows::fs`](https://github.com/rust-lang/rust/pull/101236)
* [avoid zeroing large stack buffers in stdio on Windows](https://github.com/rust-lang/rust/pull/101193)
* [add `AsFd` implementations for stdio types on WASI](https://github.com/rust-lang/rust/pull/100892)
* [implement internal `IsZero` for Wrapping and Saturating for `Vec` optimizations](https://github.com/rust-lang/rust/pull/93455)
* [add `vec::Drain{,Filter}::keep_rest`](https://github.com/rust-lang/rust/pull/95376)
* [fix `into_iter` on ZST](https://github.com/rust-lang/rust/pull/101237)
* [provider API: add additional methods to the `Demand` type](https://github.com/rust-lang/rust/pull/99583)
* [make `ReentrantMutex` movable and `const`](https://github.com/rust-lang/rust/pull/100576)
* [make `char::is_lowercase` and `char::is_uppercase` const](https://github.com/rust-lang/rust/pull/101401)
* [make `const_eval_select` a real intrinsic](https://github.com/rust-lang/rust/pull/100759)
* [hashbrown: `Equivalent` trait](https://github.com/rust-lang/hashbrown/pull/350)
* [cargo: rework test error handling](https://github.com/rust-lang/cargo/pull/11028)
* [clippy: add `--explain` subcommand](https://github.com/rust-lang/rust-clippy/pull/8952)
* [clippy: new lint `bool_to_int_with_if`](https://github.com/rust-lang/rust-clippy/pull/9412)
* [clippy: initial implementation `result_large_err`](https://github.com/rust-lang/rust-clippy/pull/9373)
* [clippy: don't use `hir_ty_to_ty` in `result_large_err`](https://github.com/rust-lang/rust-clippy/pull/9417)
* [clippy: use `approx_ty_size` for `large_enum_variant`](https://github.com/rust-lang/rust-clippy/pull/9400)
* [clippy: fix `mut_mutex_lock` when mutex is behind immutable deref](https://github.com/rust-lang/rust-clippy/pull/9418)
* [clippy: fix `suboptimal_float` not linting on `{const}.powf({const})`](https://github.com/rust-lang/rust-clippy/pull/9404)
* [clippy: fix `unnecessary_to_owned` false positive](https://github.com/rust-lang/rust-clippy/pull/9424)
* [clippy: fix missing parens in `suboptimal_flops` suggestion](https://github.com/rust-lang/rust-clippy/pull/9394)
* [clippy: fix the emission order of `trait_duplication_in_bounds`](https://github.com/rust-lang/rust-clippy/pull/9397)
* [clippy: suggest `Entry::or_default` for `Entry::or_insert(Default::default())`](https://github.com/rust-lang/rust-clippy/pull/9342)
* [rust-analyzer: clarify the state of (extern) preludes for block def maps](https://github.com/rust-lang/rust-analyzer/pull/13175)
* [rust-analyzer: don't store `SyntheticSyntax` in the reverse maps in `BodySourceMap`](https://github.com/rust-lang/rust-analyzer/pull/13173)
* [rust-analyzer: drop the expander borrow in all control flow paths](https://github.com/rust-lang/rust-analyzer/pull/13154)
* [rust-analyzer: fix nested break expressions, expecting unknown types](https://github.com/rust-lang/rust-analyzer/pull/13183)
* [rust-analyzer: highlight namerefs by syntax until proc-macros have been loaded](https://github.com/rust-lang/rust-analyzer/pull/13134)
* [rust-analyzer: lift out the module scope into a field in the resolver](https://github.com/rust-lang/rust-analyzer/pull/13174)
* [rust-analyzer: prefer the type of expression in "Replace turbofish with type"](https://github.com/rust-lang/rust-analyzer/pull/13151)
* [rust-analyzer: properly handle break resolution inside non-breakable expressions](https://github.com/rust-lang/rust-analyzer/pull/13165)
* [rust-analyzer: remove `hir::Expr::MacroStmts`](https://github.com/rust-lang/rust-analyzer/pull/13156)
* [rust-analyzer: remove type alias definition on inline](https://github.com/rust-lang/rust-analyzer/pull/13091)
* [rust-analyzer: suggest struct when completing enum](https://github.com/rust-lang/rust-analyzer/pull/13139)
* [rust-analyzer: add a "Unmerge match arm" assist to split or-patterns inside match expressions](https://github.com/rust-lang/rust-analyzer/pull/13145)
* [rust-analyzer: implement `feature(exhaustive_patterns)` from unstable Rust](https://github.com/rust-lang/rust-analyzer/pull/13167)
* [rust-analyzer: assist to turn `match` into `matches!` invocation](https://github.com/rust-lang/rust-analyzer/pull/13005)
* [rust-analyzer: insert whitespaces into `static` & `const` bodies if they are expanded from macro on hover](https://github.com/rust-lang/rust-analyzer/pull/13185)
* [rust-analyzer: lower float literals with underscores](https://github.com/rust-lang/rust-analyzer/pull/13161)
* [rust-analyzer: only move comments when extracting a struct from an enum variant](https://github.com/rust-lang/rust-analyzer/pull/13051)
* [rust-analyzer: parse `TypePathFn` with preceding `::`](https://github.com/rust-lang/rust-analyzer/pull/13160)
* [rust-analyzer: correct broken logic for return completion](https://github.com/rust-lang/rust-analyzer/pull/13187)
* [rust-analyzer: unescape all occurrences of module name in module resolution](https://github.com/rust-lang/rust-analyzer/pull/13149)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-09-07 - 2022-10-05 🦀

### Virtual

* 2022-09-01 | Virtual (PDT Timezone) | [Conf42](https://www.conf42.com/)
    * [**Conf42: Rustlang 2022**](https://www.conf42.com/rustlang2022)
* 2022-09-01 | Virtual | [Google Open Source Live](https://www.meetup.com/google-open-source/)
    * [**Rust Day on Google Open Source Live**](https://www.meetup.com/google-open-source/events/287435626/)
* 2022-09-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together**](https://www.meetup.com/rust-noris/events/287092397/)
* 2022-09-03 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059974/)
* 2022-09-03 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 1: Tokio my-redis Tutorial**](https://www.meetup.com/rust-noris/events/287346970/)
* 2022-09-04 | Virtual (Seattle, WA, US) | [Seattle Rust Meetup](https://www.meetup.com/seattle-rust-meetup/)
    * [**Nine Rules for Elegant Rust Library APIs**](https://www.meetup.com/seattle-rust-meetup/events/287726278/)
* 2022-09-06 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/286481325/)
* 2022-09-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcmbjb/)
* 2022-09-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121715/)
* 2022-09-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-09-10 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059979/)
* 2022-09-12 | Virtual + Dublin, IE | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022 (Sep 12-14)**](https://lpc.events/event/16/sessions/150/)
* 2022-09-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcmbrb/)
* 2022-09-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/286485815/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)
* 2022-09-14 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Introduction to Async in Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/288154493/)
* 2022-09-14 | Virtual (Sydney, NSW, AU) | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust-Sydney Lightning Talks**](https://www.meetup.com/rust-sydney/events/287979855/)
* 2022-09-15 | Virtual (Columbus, OH, US) | [GDG Columbus](https://www.meetup.com/gdg-columbus/)
    * [**Past, Present, and Future of Internet Money! (Custom tokenomics, RUST and CosmWASM library...)**](https://www.meetup.com/gdg-columbus/events/287972746/)
* 2022-09-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful: Bencher—Catch Performance Regressions in CI—Everett Pompeii**](https://www.meetup.com/rustdc/events/287004599/)
* 2022-09-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out (Call for Participation)**](https://www.meetup.com/vancouver-rust/events/285933975/)
* 2022-09-22 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Rust based Bluetooth tools (BlueR) you can use today**](https://www.meetup.com/charlottesville-rust-meetup/events/288123436/)
* 2022-09-23 | Virtual (Tokyo, JP) | [Rust Tokyo](https://rust.tokyo)
    * [**Rust Tokyo 2022**](https://rust.tokyo/2022)
* 2022-09-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcmbkc/)

### Europe

* 2022-09-01 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - BBQ Edition**](https://www.meetup.com/rust-berlin/events/287813728/)
* 2022-09-12 | Dublin, IE + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022 (Sep 12-14)**](https://lpc.events/event/16/sessions/150/)
* 2022-09-15 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #52**](https://www.meetup.com/rust-paris/events/288136736/)

### North America

* 2022-08-31 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**August Meetup: Rewriting a high performance Vector Database in Rust.**](https://www.meetup.com/rust-nyc/events/287821884/)
* 2022-09-01 | Phoenix, AZ, US | [Phoenix Android - GDG](https://www.meetup.com/phx-android/)
    * [**Phoenix: Rust Day on Google Open Source Live Viewing party**](https://www.meetup.com/phx-android/events/288147058/) | [Main event link](https://opensourcelive.withgoogle.com/events/rust-day-2022)

### South America

* 2022-09-10 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/)
    * [**iFood Tech Day: Rust**](https://www.meetup.com/rust-sao-paulo-meetup/events/288147015/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> So long, and thanks for all the turbofish.

– [moltonel on r/rust](https://www.reddit.com/r/rust/comments/wzuoqz/comment/im4pek6)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1286) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
