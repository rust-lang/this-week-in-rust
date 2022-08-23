Title: This Week in Rust 457
Number: 457
Date: 2022-08-24
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

This week's crate is [sass-embedded](https://crates.io/crates/sass-embedded), a library to communicate with Embedded Dart Sass.

Thanks to [Ahab](https://users.rust-lang.org/t/crate-of-the-week/2704/1099) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

411 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-08-15..2022-08-22

* [mitigate stale data reads on SGX platform](https://github.com/rust-lang/rust/pull/100383)
* [support 128-bit atomics on all aarch64 targets](https://github.com/rust-lang/rust/pull/100622)
* [`rustc_metadata`: deduplicate strings to prevent multiple copies in rmeta/query cache blow file size](https://github.com/rust-lang/rust/pull/98851)
* [make NOP dyn casts not require anything about the vtable](https://github.com/rust-lang/rust/pull/100208)
* [implied bounds: explicitly state which types are assumed to be wf](https://github.com/rust-lang/rust/pull/100676)
* [never consider unsafe blocks unused if they would be required with deny(unsafe_op_in_unsafe_fn)](https://github.com/rust-lang/rust/pull/100081)
* [do not allow `Drop` impl on foreign fundamental types](https://github.com/rust-lang/rust/pull/99576)
* [don't derive `PartialEq::ne`](https://github.com/rust-lang/rust/pull/98655)
* [lazily decode SourceFile from metadata](https://github.com/rust-lang/rust/pull/100209)
* [make `must_not_suspend` lint see through references when drop tracking is enabled](https://github.com/rust-lang/rust/pull/97962)
* [mention `as_mut` alongside `as_ref` in borrowck error message](https://github.com/rust-lang/rust/pull/100186)
* [point at a type parameter shadowing another type](https://github.com/rust-lang/rust/pull/100643)
* [recover keywords in trait bounds](https://github.com/rust-lang/rust/pull/99915)
* [reenable disabled early syntax gates as future-incompatibility lints](https://github.com/rust-lang/rust/pull/99935)
* [improved diagnostic for function defined with `def`, `fun`, `func`, or `function` instead of `fn`](https://github.com/rust-lang/rust/pull/100750)
* [suggest `fn` if `fun`, `func`, `function` or `def` is used to define a function](https://github.com/rust-lang/rust/pull/100547)
* [suggest `once_cell::Lazy` for non-const statics](https://github.com/rust-lang/rust/pull/100507)
* [suggest adding a reference to a trait assoc item](https://github.com/rust-lang/rust/pull/100769)
* [suggest adding an array length if possible](https://github.com/rust-lang/rust/pull/100590)
* [suggest the right help message for `as_ref`](https://github.com/rust-lang/rust/pull/100617)
* [`UnreachableProp`: preserve unreachable branches for multiple targets](https://github.com/rust-lang/rust/pull/99762)
* [kind-less `SessionDiagnostic` derive](https://github.com/rust-lang/rust/pull/100765)
* [convert diagnostics in parser/expr to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100713)
* [migrate "invalid variable declaration" errors to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100667)
* [migrate emoji identifier diagnostics to `SessionDiagnostic` in `rustc_interface`](https://github.com/rust-lang/rust/pull/100646)
* [migrate lint reports in `typeck::check_unused` to `LintDiagnostic`](https://github.com/rust-lang/rust/pull/100674)
* [migrate more `rustc_borrowck` diagnostics to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100864)
* [migrate `rustc_ast_passes` diagnostics to `SessionDiagnostic` and translatable messages (first part)](https://github.com/rust-lang/rust/pull/100694)
* [migrate typeck's `used` expected symbol diagnostic to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100709)
* [migrations for `rustc_expand` transcribe.rs](https://github.com/rust-lang/rust/pull/100651)
* [migrate some `rustc_borrowck` diagnostic](https://github.com/rust-lang/rust/pull/100798)
* [miri: breaking `posix_memalign` precondition is not UB](https://github.com/rust-lang/miri/pull/2485)
* [miri: improve information sharing across SB diagnostics](https://github.com/rust-lang/miri/pull/2454)
* [miri: add very basic Android support](https://github.com/rust-lang/miri/pull/2493)
* [remove manual implementations of `HashStable` for `hir::Expr` and `hir::Ty`](https://github.com/rust-lang/rust/pull/100237)
* [shrink `ast::Attribute`](https://github.com/rust-lang/rust/pull/100441)
* [box the `MacCall` in various types](https://github.com/rust-lang/rust/pull/100564)
* [use `AttrVec` more](https://github.com/rust-lang/rust/pull/100668)
* [add `IpDisplayBuffer` helper struct](https://github.com/rust-lang/rust/pull/100625)
* [rework `Ipv6Addr::is_global` to check for global reachability rather than global scope](https://github.com/rust-lang/rust/pull/99957)
* [make `slice::reverse` const](https://github.com/rust-lang/rust/pull/100663)
* [refactor iteration logic in the `Flatten` and `FlatMap` iterators](https://github.com/rust-lang/rust/pull/99541)
* [futures: fix incorrect termination of `select_with_strategy` streams](https://github.com/rust-lang/futures-rs/pull/2635)
* [cargo: fix file locking being not supported on Android raising an error](https://github.com/rust-lang/cargo/pull/10975)
* [cargo: improve error message for an array value in the manifest](https://github.com/rust-lang/cargo/pull/10944)
* [cargo: improve error message for wrong target names](https://github.com/rust-lang/cargo/pull/10999)
* [rustdoc: merge source code pages HTML elements together v2](https://github.com/rust-lang/rust/pull/100775)
* [rustdoc: count deref and non-deref as same set of used methods](https://github.com/rust-lang/rust/pull/100731)
* [rustdoc: strategic boxing to reduce the size of ItemKind and Type](https://github.com/rust-lang/rust/pull/100645)
* [rustfmt: Unicode comment align](https://github.com/rust-lang/rustfmt/pull/5505)
* [clippy: add `unused_peekable` lint](https://github.com/rust-lang/rust-clippy/pull/9258)
* [clippy: add `manual_empty_string_creations` lint](https://github.com/rust-lang/rust-clippy/pull/9295)
* [clippy: add new lint `positional_named_format_parameters`](https://github.com/rust-lang/rust-clippy/pull/9040)
* [clippy: don't lint on match pattern-binding in `question_mark`](https://github.com/rust-lang/rust-clippy/pull/9348)
* [clippy: enhance `needless_borrow` to consider trait implementations](https://github.com/rust-lang/rust-clippy/pull/9136)
* [clippy: fix `non_ascii_literal` in tests](https://github.com/rust-lang/rust-clippy/pull/9327)
* [clippy: fix `to_string_in_format_args` false positive](https://github.com/rust-lang/rust-clippy/pull/9259)
* [clippy: fix false positives of `needless_match`](https://github.com/rust-lang/rust-clippy/pull/9092)
* [clippy: lint `collapsible_str_replace`](https://github.com/rust-lang/rust-clippy/pull/9269)
* [clippy: more lint pass merges](https://github.com/rust-lang/rust-clippy/pull/8957)
* [clippy: refactor `FormatArgsExpn`](https://github.com/rust-lang/rust-clippy/pull/9349)
* [clippy: rework `only_used_in_recursion` and move it back to `complexity`](https://github.com/rust-lang/rust-clippy/pull/8804)
* [clippy: `transmute_undefined_repr` fix](https://github.com/rust-lang/rust-clippy/pull/9287)
* [clippy: check for `if-some-or-ok-else-none-or-err`](https://github.com/rust-lang/rust-clippy/pull/8696)
* [clippy: Do not lint `needless_collect` if the target code is inside a loop](https://github.com/rust-lang/rust-clippy/pull/8992)
* [clippy: suggest `map_or` in `case_sensitive_file_extension_comparisons`](https://github.com/rust-lang/rust-clippy/pull/9341)
* [clippy: `unwrap_used` and `expect_used`: trigger on uses of their `_err` variants](https://github.com/rust-lang/rust-clippy/pull/9338)
* [rust-analyzer: consider bounds on inherent impl in method resolution](https://github.com/rust-lang/rust-analyzer/pull/13074)
* [rust-analyzer: implement `IntoFuture` type inference](https://github.com/rust-lang/rust-analyzer/pull/12982)
* [rust-analyzer: implement lsp extension for cancelling running flychecks](https://github.com/rust-lang/rust-analyzer/pull/13063)
* [rust-analyzer: log rustfmt parsing errors as warnings](https://github.com/rust-lang/rust-analyzer/pull/13064)
* [rust-analyzer: pop an error notification when flycheck can't be restarted](https://github.com/rust-lang/rust-analyzer/pull/13088)
* [rust-analyzer: add a setting for keyword hover popups](https://github.com/rust-lang/rust-analyzer/pull/13037)
* [rust-analyzer: add an assist for inlining all type alias uses](https://github.com/rust-lang/rust-analyzer/pull/13036)
* [rust-analyzer: generate static method using `Self::assoc()` syntax](https://github.com/rust-lang/rust-analyzer/pull/13041)
* [rust-analyzer: improved `inline_call` to replace `Self`](https://github.com/rust-lang/rust-analyzer/pull/13061)
* [rust-analyzer: run test mod from anywhere in parent file](https://github.com/rust-lang/rust-analyzer/pull/13045)
* [rust-analyzer: make trait assoc items become inactive due to cfg](https://github.com/rust-lang/rust-analyzer/pull/12965)
* [rust-analyzer: fix panics on GATs involving const generics](https://github.com/rust-lang/rust-analyzer/pull/13021)
* [rust-analyzer: escape keywords used as names in earlier editions](https://github.com/rust-lang/rust-analyzer/pull/13034)
* [rust-analyzer: record completion filtering](https://github.com/rust-lang/rust-analyzer/pull/13028)
* [rust-analyzer: resolve associated types of bare `dyn` types](https://github.com/rust-lang/rust-analyzer/pull/13049)
* [rust-analyzer: resolve path `Self` alone in value namespace](https://github.com/rust-lang/rust-analyzer/pull/13053)
* [tidy: check fluent files for style](https://github.com/rust-lang/rust/pull/100671)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-08-24 - 2022-09-21 🦀

### Virtual

* 2022-08-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**C++ Concepts vs Rust Traits**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287450947/)
* 2022-08-17 | Virtual (Hoboken, NJ, US) | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/287468144/)
* 2022-08-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)
* 2022-08-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydclbxb/)
* 2022-08-18 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #8**](https://rust-meetup.ir/2022/08/18/8th-meetup.html)
* 2022-08-18 | Virtual (Wiesbaden, DE) | [Frontend RheinMain](https://www.meetup.com/frontend_rm/)
    * [**Rust for curious developers**](https://www.meetup.com/frontend_rm/events/287713743/)
* 2022-08-20 | Virtual | [Rust Edu](https://rust-edu.org/workshop)
    * [**Rust Education Workshop 2022 (Submission deadline 2022-08-16)**](https://rust-edu.org/workshop)
* 2022-08-24 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Tech Talk Live Appointment: Customize GitHub Workflow with Serverless Functions - How to use Rust and JavaScript to automate GitHub processes**](https://www.meetup.com/wasm-rust-meetup/events/287876999/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-25 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch: together with the C++ UG KA; various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/287642940/)
* 2022-08-27 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059968/)
* 2022-08-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/287286751/)
* 2022-08-30 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydclbnc/)
* 2022-09-01 | Virtual | [Google Open Source Live](https://www.meetup.com/google-open-source/)
    * [**Rust Day on Google Open Source Live**](https://www.meetup.com/google-open-source/events/287435626/)
* 2022-09-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together**](https://www.meetup.com/rust-noris/events/287092397/)
* 2022-09-03 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059974/)
* 2022-09-03 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 1: Tokio my-redis Tutorial**](https://www.meetup.com/rust-noris/events/287346970/)
* 2022-09-04 | Virtual (Seattle, WA, US) | [Seattle Rust Meetup](https://www.meetup.com/seattle-rust-meetup/)
    * [**September Meetup**](https://www.meetup.com/seattle-rust-meetup/events/287726278/)
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
* 2022-09-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/286485815/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)


### Europe

* 2022-08-18 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #51**](https://www.meetup.com/rust-paris/events/287704177/)
* 2022-08-25 | Stockholm, SE | [StockholmCpp](https://www.meetup.com/stockholmcpp/)
    * [**0x21: Learning from Rust, Typical C++**](https://www.meetup.com/stockholmcpp/events/286854212/)
* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)

### North America

* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)
* 2022-08-25 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Concurrencia & paralelismo con Rust**](https://www.meetup.com/rust-mx/events/287561814/)
* 2022-08-25 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Hello World Cargo Crates Using Github Actions with jojobyte and Food!**](https://www.meetup.com/utah-rust/events/kvrxqsydclbpb/)
* 2022-08-31 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**August Meetup: Rewriting a high performance Vector Database in Rust.**](https://www.meetup.com/rust-nyc/events/287821884/)

### Oceania

* 2022-08-24 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-26 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**August 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/287468753/)

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

> A fast executing language that crashes all the time is like a supercar… that crashes all the time.

– [Tris on youtube](https://youtu.be/4YU_r70yGjQ)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1281) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
