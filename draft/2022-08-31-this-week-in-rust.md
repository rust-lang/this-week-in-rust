Title: This Week in Rust 458
Number: 458
Date: 2022-08-31
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

This week's crate is [bytehound](https://github.com/koute/bytehound) a memory profiler for Rust.

Thanks to [Aleksey Kladov](https://users.rust-lang.org/t/crate-of-the-week/2704/1101) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

412 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-08-22..2022-08-29

* [add the armv4t-none-eabi target to the supported_targets](https://github.com/rust-lang/rust/pull/100641)
* [stabilize split debuginfo on linux](https://github.com/rust-lang/rust/pull/98051)
* [add GDB/LLDB pretty-printers for `NonZero` types](https://github.com/rust-lang/rust/pull/98301)
* [fix const: dynamic checks for accessing statics](https://github.com/rust-lang/const-eval/pull/70)
* [improve const mismatch `FulfillmentError`](https://github.com/rust-lang/rust/pull/100437)
* [provide structured suggestion for `hashmap[idx] = val`](https://github.com/rust-lang/rust/pull/101002)
* [suggest adding a missing semicolon before an item](https://github.com/rust-lang/rust/pull/100565)
* [suggest alternatives when trying to mutate a `HashMap`/`BTreeMap` via indexing](https://github.com/rust-lang/rust/pull/100906)
* [use smaller span for suggestions](https://github.com/rust-lang/rust/pull/101055)
* [migrate `rustc_attr` crate diagnostics](https://github.com/rust-lang/rust/pull/100836)
* [migrate `rustc_interface` diagnostics](https://github.com/rust-lang/rust/pull/100808)
* [migrate `rustc_lint` errors to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100776)
* [migrate `rustc_plugin_impl` to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100768)
* [migrate `rustc_ty_utils` to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100735)
* [migrate ast lowering to session diagnostic](https://github.com/rust-lang/rust/pull/100724)
* [migrate part of `rustc_infer` to session diagnostic](https://github.com/rust-lang/rust/pull/100843)
* [migrate `rustc_driver` to `SessionDiagnostic`](https://github.com/rust-lang/rust/pull/100890)
* [migrate `rustc_mir_dataflow` to diagnostic structs](https://github.com/rust-lang/rust/pull/100744)
* [miri: adding support for external C functions that have integer (or empty) args and/or returns](https://github.com/rust-lang/miri/pull/2363)
* [miri: skip field retagging on ZSTs, it can take forever](https://github.com/rust-lang/miri/pull/2517)
* [miri: strengthen C++20 SC accesses](https://github.com/rust-lang/miri/pull/2512)
* [avoid cloning a collection only to iterate over it](https://github.com/rust-lang/rust/pull/100497)
* [reduce code size of `assert_matches_failed`](https://github.com/rust-lang/rust/pull/100933)
* [shrink `FnAbi`](https://github.com/rust-lang/rust/pull/100999)
* [shrink `thir::Expr`](https://github.com/rust-lang/rust/pull/100944)
* [stabilize `#![feature(label_break_value)]`](https://github.com/rust-lang/rust/pull/99332)
* [stabilize `const_ptr_offset_from`](https://github.com/rust-lang/rust/pull/96240)
* [stabilize `std::io::read_to_string`](https://github.com/rust-lang/rust/pull/100337)
* [add a `File::create_new` constructor](https://github.com/rust-lang/rust/pull/98801)
* [add `next_up` and `next_down` for `f32`/`f64`](https://github.com/rust-lang/rust/pull/100578)
* [`is_whitespace()` performance improvements](https://github.com/rust-lang/rust/pull/99487)
* [add pointer masking convenience functions](https://github.com/rust-lang/rust/pull/96946)
* [BTree: evaluate static type-related check at compile time](https://github.com/rust-lang/rust/pull/95005)
* [fix `Ipv6Addr::is_unicast_global` to check for unicast global scope rebase](https://github.com/rust-lang/rust/pull/99947)
* [windows: optimize `Wtf8Buf::into_string` for the case where it contains UTF-8](https://github.com/rust-lang/rust/pull/96869)
* [properly forward `ByRefSized::fold` to the inner iterator](https://github.com/rust-lang/rust/pull/100220)
* [make `slice::`{`split_at`, `split_at_unchecked`} `const` functions](https://github.com/rust-lang/rust/pull/100076)
* [`std::io`: migrate `ReadBuf` to `BorrowBuf`/`BorrowCursor`](https://github.com/rust-lang/rust/pull/97015)
* [rustdoc: rewrite error index generator to greatly reduce the size of the pages](https://github.com/rust-lang/rust/pull/100922)
* [clippy: implemented `suspicious_to_owned` lint to check if `to_owned` is called on a `Cow`](https://github.com/rust-lang/rust-clippy/pull/8984)
* [clippy: new lint: Raw slice pointer cast](https://github.com/rust-lang/rust-clippy/pull/9247)
* [clippy: new `multi_assignment` lint](https://github.com/rust-lang/rust-clippy/pull/9379)
* [clippy: don't lint `needless_return` if `return` has attrs](https://github.com/rust-lang/rust-clippy/pull/9381)
* [clippy: don't lint literal `None` from expansion](https://github.com/rust-lang/rust-clippy/pull/9389)
* [clippy: ignore `match_like_matches_macro` when there is comment](https://github.com/rust-lang/rust-clippy/pull/9276)
* [clippy: remove parenthesis from `unnecessary_cast` suggestion](https://github.com/rust-lang/rust-clippy/pull/9385)
* [clippy: rename `manual_empty_string_creation` and move to pedantic](https://github.com/rust-lang/rust-clippy/pull/9366)
* [rust-analyzer: do not substitute `Self` when in same impl block](https://github.com/rust-lang/rust-analyzer/pull/13090)
* [rust-analyzer: move empty diagnostics workaround back into the server](https://github.com/rust-lang/rust-analyzer/pull/13133)
* [rust-analyzer: resolve doc links on impl blocks](https://github.com/rust-lang/rust-analyzer/pull/13100)
* [rustc-perf: add a metric containing the size of generated documentation](https://github.com/rust-lang/rustc-perf/pull/1417)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-08-31 - 2022-09-28 🦀

### Virtual

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
* 2022-09-12 | Virtual + Dublin, IE | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022**](https://lpc.events/event/16/sessions/150/)
* 2022-09-13 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcmbrb/)
* 2022-09-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/286485815/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)
* 2022-09-15 | Virtual (Columbus, OH, US) | [GDG Columbus](https://www.meetup.com/gdg-columbus/)
    * [**Past, Present, and Future of Internet Money! (Custom tokenomics, RUST and CosmWASM library...)**](https://www.meetup.com/gdg-columbus/events/287972746/)
* 2022-09-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/287004599/)
* 2022-09-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out (Call for Participation)**](https://www.meetup.com/vancouver-rust/events/285933975/)


### Europe
* 2022-08-25 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**CPH Hack Night #28**](https://www.meetup.com/copenhagen-rust-meetup-group/events/287635498/)
* 2022-08-25 | Stockholm, SE | [StockholmCpp](https://www.meetup.com/stockholmcpp/)
    * [**0x21: Learning from Rust, Typical C++**](https://www.meetup.com/stockholmcpp/events/286854212/)
* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)
* 2022-09-12 | Dublin, IE + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022**](https://lpc.events/event/16/sessions/150/)

### North America

* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)
* 2022-08-25 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Concurrencia & paralelismo con Rust**](https://www.meetup.com/rust-mx/events/287561814/)
* 2022-08-25 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Using Github Actions to Deploy Cargo Crates with Jordan and Food!**](https://www.meetup.com/utah-rust/events/kvrxqsydclbpb/)
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

> \[W\]e reached a tipping point. **We decided to move our entire codebase to Rust**... . Rust seemed to give us all the capabilities we needed, **however, there was still one *minor* problem - no one on the team knew Rust.** ...
>
> We started with a small team of senior engineers and managers learning Rust and developing the skeleton of the DB and dev environment (for others to build on). Then, slowly, others joined in rewriting and contributing different components until we eventually got rid of the old codebase altogether (I still remember the day my original C modules, from the first days of Pinecone, were taken out). Unbeknownst to most Pinecone customers, the new Rust core was deployed in March this year. And in the process of taking over running workloads, we managed not to drop a single API call!
>
> ... **We all expect\[ed\] performance and dev processes to improve. Those indeed happened.** What we didn’t expect was the extent to which dev velocity increased and operational incidents decreased. **Dev velocity** ... **improved dramatically with Rust. Built-in testing, CI/CD, benchmarking, and an overzealous compiler increased engineers’ confidence in pushing changes, and enabled them to work on the same code sections and contribute simultaneously without breaking the code base.** Most impressively though, **real time operational events dropped almost to zero overnight after the original release.** Sure, there are still surprises here and there but, by and large, the core engine has been shockingly stable and predictable.

– [Edo Liberty on the pinecone blog](https://www.pinecone.io/learn/inside-the-pinecone/)

Thanks to [Erich Gubler](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1283) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
