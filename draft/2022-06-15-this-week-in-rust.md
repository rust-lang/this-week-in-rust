Title: This Week in Rust 447
Number: 447
Date: 2022-06-15
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

* [Caches In Rust](https://matklad.github.io/2022/06/11/caches-in-rust.html)
* [video] [Async I/O in Depth: State Machines, Event Loops and Non-Blocking I/O System Calls in Rust (Part 2)](https://www.youtube.com/watch?v=_3LpJ6I-tzc)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [dxf-rs](https://github.com/IxMilia/dxf-rs), a library to parse AutoCAD files.

Thanks to [cosj](https://users.rust-lang.org/t/crate-of-the-week/2704/1067) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

368 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-06-06..2022-06-13

* [add support for emitting functions with `coldcc` to LLVM](https://github.com/rust-lang/rust/pull/97512)
* [allow unstable items to be re-exported unstably without requiring the feature be enabled](https://github.com/rust-lang/rust/pull/97301)
* [don't suggest adding `let` in certain `if` conditions](https://github.com/rust-lang/rust/pull/97856)
* [fix ICEs from zsts within unsized types with non-zero offsets](https://github.com/rust-lang/rust/pull/97738)
* [fix precise field capture of univariant enums](https://github.com/rust-lang/rust/pull/97325)
* [never regard macro rules with `compile_error!` invocations as unused](https://github.com/rust-lang/rust/pull/97903)
* [use more targeted suggestion when confusing i8 with std::i8](https://github.com/rust-lang/rust/pull/97845)
* [`ValuePairs::PolyTraitRefs` should be called "trait"s in type error diagnostics](https://github.com/rust-lang/rust/pull/98012)
* [use precise spans for recursive const evaluation](https://github.com/rust-lang/rust/pull/97740)
* [suggest escaping `box` as identifier](https://github.com/rust-lang/rust/pull/97857)
* [suggest to swap a struct and a trait in trait impls](https://github.com/rust-lang/rust/pull/97812)
* [suggest using `iter()` or `into_iter()` for `Vec`](https://github.com/rust-lang/rust/pull/97871)
* [tidy up miscellaneous bounds suggestions](https://github.com/rust-lang/rust/pull/97778)
* [cleanup bound variable handling](https://github.com/rust-lang/rust/pull/97648)
* [do not introduce bindings for types and consts in higher-ranked trait bounds](https://github.com/rust-lang/rust/pull/97927)
* [folding revamp](https://github.com/rust-lang/rust/pull/97447)
* [make `Encodable` and `Encoder` infallible](https://github.com/rust-lang/rust/pull/94732)
* [re-use the type op instead of calling the `implied_outlives_bounds` query directly](https://github.com/rust-lang/rust/pull/97081)
* [remove redundant calls to `reserve` in `impl Write for VecDeque`](https://github.com/rust-lang/rust/pull/97922)
* [remove unnecessary `to_string` and `String::new`](https://github.com/rust-lang/rust/pull/98043)
* [miri: make scheduler preemptive](https://github.com/rust-lang/miri/pull/2208)
* [stabilize `$$` in Rust 1.63.0](https://github.com/rust-lang/rust/pull/95860)
* [stabilize the `bundle` native library modifier](https://github.com/rust-lang/rust/pull/95818)
* [stabilize `explicit_generic_args_with_impl_trait`](https://github.com/rust-lang/rust/pull/96868)
* [stabilize `const_intrinsic_copy`](https://github.com/rust-lang/rust/pull/97276)
* [stabilize scoped threads](https://github.com/rust-lang/rust/pull/97992)
* [allow `ptr_from_addr_cast` to fail](https://github.com/rust-lang/rust/pull/97763)
* [add the `Provider` api to `core::any`](https://github.com/rust-lang/rust/pull/91970)
* [`BTreeSet`: avoid intermediate sorting when collecting sorted iterators](https://github.com/rust-lang/rust/pull/97868)
* [`impl Read` and `Write` for `VecDeque<u8>`](https://github.com/rust-lang/rust/pull/95632)
* [change `Direction::{is_forward,is_backward}` functions into constants](https://github.com/rust-lang/rust/pull/97832)
* [fix infinite recursion in x86\_64 memcmp if SSE2 is not present](https://github.com/rust-lang/compiler-builtins/pull/471)
* [fix `Termination` impl panic on closed stderr](https://github.com/rust-lang/rust/pull/97970)
* [hashbrown: add `Extend<&'a (K, V)> for HashMap<K, V, S, A>`](https://github.com/rust-lang/hashbrown/pull/340)
* [hashbrown: editing `do_alloc` for reducing LLVM IR](https://github.com/rust-lang/hashbrown/pull/341)
* [codegen\_gcc: feature/more simd](https://github.com/rust-lang/rustc_codegen_gcc/pull/176)
* [clippy: fix `iter_overeager_cloned` false positive](https://github.com/rust-lang/rust-clippy/pull/8960)
* [clippy: fix some `#[expect]` lint interaction](https://github.com/rust-lang/rust-clippy/pull/8976)
* [clippy: fix `derive_partial_eq_without_eq`](https://github.com/rust-lang/rust-clippy/pull/8950)
* [clippy: check const context](https://github.com/rust-lang/rust-clippy/pull/8907)
* [rustfmt: dedup `imports_granularity = "Item"` (#4737)](https://github.com/rust-lang/rustfmt/pull/5380)
* [rust-analyzer: more precise proc-macro errors](https://github.com/rust-lang/rust-analyzer/pull/12514)
* [rust-analyzer: restart server automatically on settings changes](https://github.com/rust-lang/rust-analyzer/pull/12477)
* [rust-analyzer: add proc macro ABI for rustc 1.63](https://github.com/rust-lang/rust-analyzer/pull/12492)
* [rust-analyzer: on assoc item name hover, render trait decl docs](https://github.com/rust-lang/rust-analyzer/pull/12519)
* [rust-analyzer: add label to loop](https://github.com/rust-lang/rust-analyzer/pull/12481)
* [rust-analyzer: fix inline variable produce mismatched type](https://github.com/rust-lang/rust-analyzer/pull/12464)
* [rust-analyzer: don't respond to cancelled requests when retrying them](https://github.com/rust-lang/rust-analyzer/pull/12508)
* [rustup: Visual Studio: add the English language pack](https://github.com/rust-lang/rustup/pull/3006)
* [rustup: Visual Studio: let the user choose install method](https://github.com/rust-lang/rustup/pull/3008)
* [rustup: improve handling of Visual Studio errors](https://github.com/rust-lang/rustup/pull/3004)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-06-15 - 2022-07-13 🦀

### Virtual

* 2022-06-08 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcjblb/)
* 2022-06-08 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Study Session - Macros**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/286079097/)
* 2022-06-09 | Dublin, IE | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Verus — Verified Rust for low-level systems code**](https://www.meetup.com/Rust-Dublin/events/286018947/)
* 2022-06-09 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydcjbmb/)
* 2022-06-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust June 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/285952122/)
* 2022-06-09 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydcjbmb/)
* 2022-06-11 | Online | [Rust Gamedev](https://arewegameyet.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://discord.gg/j6QJsMd)
* 2022-06-14 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcjbsb/)
* 2022-06-14 | Rostock, DE | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/286138086/)
* 2022-06-14 | Saarbrücken, DE | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 22u16**](https://www.meetup.com/rust-saar/events/286291318/)
* 2022-06-15 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbtb/)
* 2022-06-15 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Nushell**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcjbtb/)
* 2022-06-21 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydcjbcc/)
* 2022-06-28 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydcjblc/)
* 2022-06-29 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcjbmc/)
* 2022-06-30 | Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 23th Edition**](https://www.meetup.com/Rust-Linz/events/286029968/)
* 2022-07-05 | Austin, TX, US | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting #10**](https://www.meetup.com/webassembly-and-wasmedge/events/zzdnrsydckbhb/)
* 2022-07-05 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/jbfnrsydckbhb/)
* 2022-07-05 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydckbhb/)
* 2022-07-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydckbjb/)

### North America

* 2022-06-08 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcjblb/)
* 2022-06-09 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcjbmb/)
* 2022-06-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcjbcc/)
* 2022-06-29 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - June 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)

### Europe

* 2022-06-09 | Edinburgh, UK | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**Rust Night June, Edinburgh**](https://www.meetup.com/rust-edi/events/286080531/)
* 2022-06-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Introduction to axum - An ergonomic and modular web framework by David Pedersen**](https://www.meetup.com/Rust-Oslo/events/286006378/)
* 2022-06-14 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**On Site Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/286137650/)
* 2022-06-16 | Bristol City, UK | [Rust Bristol](https://www.meetup.com/rust-bristol/)
    * [**Talks - Serverless WASM & Graphics in Rust**](https://www.meetup.com/rust-bristol/events/286391025/)
* 2022-06-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Async Rust and Embedded**](https://www.meetup.com/Rust-Oslo/events/286236751/)
* 2022-06-22 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/286305083/)

### Oceania

* 2022-06-14 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**Canberra Rust User Group**](https://www.meetup.com/rust-canberra/events/285918739/)
* 2022-06-17 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
    * [**June 2022 Meetup**](https://www.meetup.com/Rust-Melbourne/events/285962368/)
* 2022-06-23 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**June Meetup**](https://www.meetup.com/rust-brisbane/events/286385515/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Because lower-level software has more operational constraints than higher-level software (e.g. it typically cannot tolerate a runtime or memory management via garbage collection), developing a memory safe language suitable for systems software is particularly challenging. The Rust language has met that challenge, however, and is an excellent candidate for replacing C in many systems applications.
>
> We plan to invest in the tools that allow systems engineers to move their software to Rust. This means investing in improving package management, compilers, and Foreign Function Interface (FFI) generators. In many cases this will include providing interfaces compatible with existing widely-used components to enable transition. With these tools, adoption of a memory safe alternative will scale much faster without replication of efforts.

– [The White House Open Source Software Mobilization Plan, multiple authors](https://8112310.fs1.hubspotusercontent-na1.net/hubfs/8112310/OpenSSF/White%20House%20OSS%20Mobilization%20Plan.pdf?utm_referrer=https%3A%2F%2Fopenssf.org%2Foss-security-mobilization-plan%2F) (PDF link)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/125) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
