Title: This Week in Rust 460
Number: 460
Date: 2022-09-14
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

This week's crate is [bstr](https://lib.rs/crates/bstr), a fast and featureful byte-string library.

Thanks to [8573](https://users.rust-lang.org/t/crate-of-the-week/2704/1103) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

324 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-09-05..2022-09-12

* [initial implementation of return-position `impl Trait` in traits](https://github.com/rust-lang/rust/pull/101224)
* [check that the types in return position `impl Trait` in traits are well-formed](https://github.com/rust-lang/rust/pull/101676)
* [deny return-position `impl Trait` in traits for object safety](https://github.com/rust-lang/rust/pull/101681)
* [only encode return-position `impl Trait` in trait when parent function has a default body](https://github.com/rust-lang/rust/pull/101682)
* [implement `std::marker::Tuple`, a marker trait for tuples](https://github.com/rust-lang/rust/pull/100251)
* [add `inline-llvm` option for disabling/enabling LLVM inlining](https://github.com/rust-lang/rust/pull/100293)
* [stabilize raw-dylib for non-x86](https://github.com/rust-lang/rust/pull/99916)
* [equate fn outputs when inferring RPITIT hidden types](https://github.com/rust-lang/rust/pull/101614)
* [allow generators to `impl Clone`/`Copy`](https://github.com/rust-lang/rust/pull/101673)
* [avoid infinite loop in function arguments checking](https://github.com/rust-lang/rust/pull/100502)
* [`const_generics`: correctly deal with bound variables](https://github.com/rust-lang/rust/pull/98900)
* [fix LLVM IR type mismatch](https://github.com/rust-lang/rust/pull/101647)
* [fix ICE in `opt_suggest_box_span`](https://github.com/rust-lang/rust/pull/101604)
* [fix ICE report flags display](https://github.com/rust-lang/rust/pull/101595)
* [fix ICE, generalize 'move generics to trait' suggestion for >0 non-rcvr arguments](https://github.com/rust-lang/rust/pull/101427)
* [fix RPIT ICE for implicit HRTB when missing dyn](https://github.com/rust-lang/rust/pull/101468)
* [fix code generation of `Rvalue::Repeat` with 128 bit values](https://github.com/rust-lang/rust/pull/101612)
* [fix compile errors for uwp-windows-msvc targets](https://github.com/rust-lang/rust/pull/101481)
* [normalize before erasing late-bound regions in `equal_up_to_regions`](https://github.com/rust-lang/rust/pull/101437)
* [recover from using `;` as separator between fields](https://github.com/rust-lang/rust/pull/101457)
* [fix the suggestion of format for `asm_sub_register`](https://github.com/rust-lang/rust/pull/101253)
* [adjust and slightly generalize operator error suggestion](https://github.com/rust-lang/rust/pull/101424)
* [add list of recognized repr attributes to the unrecognized repr error](https://github.com/rust-lang/rust/pull/101486)
* [shrink span for bindings with subpatterns](https://github.com/rust-lang/rust/pull/101399)
* [point at type parameter in plain path expr](https://github.com/rust-lang/rust/pull/101425)
* [point out when a callable is not actually callable because its return is not sized](https://github.com/rust-lang/rust/pull/101359)
* [allow `lower_lifetime_binder` receive a closure](https://github.com/rust-lang/rust/pull/101496)
* [do not suggest a semicolon for a macro without `!`](https://github.com/rust-lang/rust/pull/101502)
* [include enum path in variant suggestion](https://github.com/rust-lang/rust/pull/101357)
* [suggest adding array lengths to references to arrays if possible](https://github.com/rust-lang/rust/pull/101492)
* [suggest introducing an explicit lifetime if it does not exist](https://github.com/rust-lang/rust/pull/101445)
* [suggest pub instead of public for const type item](https://github.com/rust-lang/rust/pull/101668)
* [suggest removing unnecessary prefix let in patterns](https://github.com/rust-lang/rust/pull/101362)
* [migrate another part of `rustc_infer` to session diagnostic](https://github.com/rust-lang/rust/pull/101153)
* [migrate `rustc_middle` diagnostic](https://github.com/rust-lang/rust/pull/101021)
* [migrate `rustc_session` to use SessionDiagnostic - Pt. 2](https://github.com/rust-lang/rust/pull/101041)
* [miri: add a Machine hook for inline assembly](https://github.com/rust-lang/rust/pull/101402)
* [shrink `PredicateS`](https://github.com/rust-lang/rust/pull/101432)
* [shrink `hir::Ty` and `hir::Pat`](https://github.com/rust-lang/rust/pull/101467)
* [parameterize `ty::Visibility` over used ID](https://github.com/rust-lang/rust/pull/101498)
* [allow lint passes to be bound by `TyCtxt`](https://github.com/rust-lang/rust/pull/101501)
* [track PGO profiles in depinfo](https://github.com/rust-lang/rust/pull/100801)
* [use `RelocModel::Pic` for UEFI targets](https://github.com/rust-lang/rust/pull/101413)
* [use niche-filling optimization even when multiple variants have data](https://github.com/rust-lang/rust/pull/94075)
* [inline `<T as From<T>>::from`](https://github.com/rust-lang/rust/pull/100733)
* [lower the `assume` intrinsic to a MIR statement](https://github.com/rust-lang/rust/pull/98332)
* [compile `spin_loop_hint` as pause on x86 even without sse2 enabled](https://github.com/rust-lang/rust/pull/101495)
* [reimplement `carrying_add` and `borrowing_sub` for signed integers](https://github.com/rust-lang/rust/pull/93873)
* [optimize thread parking on NetBSD](https://github.com/rust-lang/rust/pull/101482)
* [remove `&[T]` from `vec_deque::Drain`](https://github.com/rust-lang/rust/pull/101299)
* [the `<*const T>::guaranteed_*` methods now return an option for the unknown case](https://github.com/rust-lang/rust/pull/101483)
* [use futex-based locks and thread parker on Hermit](https://github.com/rust-lang/rust/pull/101475)
* [hashbrown: add `HashSet::raw_table`](https://github.com/rust-lang/hashbrown/pull/358)
* [hashbrown: add `RawTable::is_full`](https://github.com/rust-lang/hashbrown/pull/354)
* [git2: implement `IntoIterator` for `Statuses`](https://github.com/rust-lang/git2-rs/pull/880)
* [codegen\_gcc: simd: impl `extract_element` for vector types](https://github.com/rust-lang/rustc_codegen_gcc/pull/215)
* [cargo: specify the max length for crate name](https://github.com/rust-lang/cargo/pull/11051)
* [rustdoc: avoid cleaning modules with duplicate names](https://github.com/rust-lang/rust/pull/101631)
* [rustdoc: correcty handle intra-doc-links to items without HTML page](https://github.com/rust-lang/rust/pull/101633)
* [rustdoc: more accurate struct type](https://github.com/rust-lang/rust/pull/101521)
* [rustdoc: store Variant Fields as their own item](https://github.com/rust-lang/rust/pull/101462)
* [clippy: do not expand macro in `nonminimal_bool` suggestions](https://github.com/rust-lang/rust-clippy/pull/9457)
* [clippy: don't lint `large_stack_array` inside static items](https://github.com/rust-lang/rust-clippy/pull/9466)
* [clippy: don't panic on invalid shift while constfolding](https://github.com/rust-lang/rust-clippy/pull/9464)
* [clippy: fix `FormatArgsExpn` parsing of `FormatSpec` positions](https://github.com/rust-lang/rust-clippy/pull/9469)
* [clippy: fix `range_{plus,minus}_one` bad suggestions](https://github.com/rust-lang/rust-clippy/pull/9446)
* [clippy: fix hang in `vec_init_then_push`](https://github.com/rust-lang/rust-clippy/pull/9441)
* [clippy: rename the `arithmetic` lint](https://github.com/rust-lang/rust-clippy/pull/9443)
* [clippy: suggest `unwrap_or_default` when closure returns `"".to_string`](https://github.com/rust-lang/rust-clippy/pull/9421)
* [clippy: use `visit_expr_field` for `ParamPosition`](https://github.com/rust-lang/rust-clippy/pull/9458)
* [clippy: use macro callsite when creating `Sugg` helper](https://github.com/rust-lang/rust-clippy/pull/9410)
* [clippy: make `Arithmetic` consider literals](https://github.com/rust-lang/rust-clippy/pull/9365)
* [clippy: `assertions_on_result_states`: fix suggestion when `assert!` is not in a statement](https://github.com/rust-lang/rust-clippy/pull/9453)
* [rust-analyzer: add config to unconditionally prefer core imports over std](https://github.com/rust-lang/rust-analyzer/pull/13212)
* [rust-analyzer: build release artifact against older glibc](https://github.com/rust-lang/rust-analyzer/pull/13214)
* [rust-analyzer: filter imports on find-all-references](https://github.com/rust-lang/rust-analyzer/pull/13186)
* [rust-analyzer: new assist: `move_format_string_arg`](https://github.com/rust-lang/rust-analyzer/pull/13216)
* [rust-analyzer: remove the `toggleInlayHints` command from VSCode](https://github.com/rust-lang/rust-analyzer/pull/13215)
* [rust-analyzer: use `proc-macro-srv` from sysroot in rust-project.json workspaces](https://github.com/rust-lang/rust-analyzer/pull/13200)
* [rust-analyzer: make clicking a closing brace inlay hint go to the opening brace](https://github.com/rust-lang/rust-analyzer/pull/13158)
* [rust-analyzer: add semicolon completion to mod](https://github.com/rust-lang/rust-analyzer/pull/13207)
* [rust-analyzer: handle lifetime variables in projection normalization](https://github.com/rust-lang/rust-analyzer/pull/13223)
* [rust-analyzer: handle trait methods as inherent methods for trait-related types](https://github.com/rust-lang/rust-analyzer/pull/13147)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-09-14 - 2022-10-12 🦀

### Virtual

* 2022-09-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121715/)
* 2022-09-08 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Workshop: Introduction To Rust For Artists - by Lisa Passing**](https://www.meetup.com/rust-linz/events/288027737/)
* 2022-09-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #17**](https://www.meetup.com/rust-noris/events/hlvbvsydcmblb/)
* 2022-09-09 | Virtual + Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 25th Edition**](https://www.meetup.com/rust-linz/events/288048260/)
* 2022-09-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-09-10 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059979/)
* 2022-09-12 | Virtual + Dublin, IE | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022 (Sep 12-14)**](https://lpc.events/event/16/sessions/150/)
* 2022-09-13 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/288032274/)
* 2022-09-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcmbrb/)
* 2022-09-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/286485815/)
* 2022-09-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcmbsb/)
* 2022-09-14 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Introduction to Async in Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/288154493/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)
* 2022-09-15 | Virtual (Columbus, OH, US) | [GDG Columbus](https://www.meetup.com/gdg-columbus/)
    * [**Past, Present, and Future of Internet Money! (Custom tokenomics, RUST and CosmWASM library...)**](https://www.meetup.com/gdg-columbus/events/287972746/)
* 2022-09-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcmbtb/)
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
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)

### Europe

* 2022-09-09 | Linz, AT + Virtual | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 25th Edition**](https://www.meetup.com/rust-linz/events/288048260/)
* 2022-09-12 | Dublin, IE + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022 (Sep 12-14)**](https://lpc.events/event/16/sessions/150/)
* 2022-09-13 | Rome, IT | [Rust Roma](https://www.meetup.com/Rust-Roma/)
    * [**Rust learning and hacking evening #Aperitech**](https://www.meetup.com/Rust-Roma/events/288323150/)
* 2022-09-15 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #52**](https://www.meetup.com/rust-paris/events/288136736/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)

### North America

* 2022-09-08 | Columbus, OH, US| [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcmblb/)
* 2022-09-14 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288287206/)
* 2022-09-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcmbbc/)
* 2022-09-22 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Game Prototyping with Rusty Engine with Nathan Stocks and Food!**](https://www.meetup.com/utah-rust/events/rvpgxsydcmbmc/)

### Oceania

* 2022-09-14 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust-Sydney Lightning Talks**](https://www.meetup.com/rust-sydney/events/287979855/)

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

> In Rust We Trust

– [Alexander Sidorov on Medium](https://medium.com/@siberianguy/an-almost-religious-case-for-rust-e4c4764acd8d)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1287) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
