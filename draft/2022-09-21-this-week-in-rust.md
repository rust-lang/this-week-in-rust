Title: This Week in Rust 461
Number: 461
Date: 2022-09-21
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

This week's crate is [match\_deref](https://crates.io/crates/match_deref), a macro crate to implement deref patterns on stable Rust.

Thanks to [meithecatte](https://users.rust-lang.org/t/crate-of-the-week/2704/1106) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

347 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-09-12..2022-09-19

* [stabilize `let else`](https://github.com/rust-lang/rust/pull/93628) (RFC #[3137](https://rust-lang.github.io/rfcs/3137-let-else.html))
* [stabilize generic associated types](https://github.com/rust-lang/rust/pull/96709) (RFC #[1598](https://rust-lang.github.io/rfcs/1598-generic_associated_types.html))
* [initial implementation of dyn*](https://github.com/rust-lang/rust/pull/101212)
* [fix `#[link kind="raw-dylib"]` to respect `#[link_name]`](https://github.com/rust-lang/rust/pull/101738)
* [disallow defaults on type GATs](https://github.com/rust-lang/rust/pull/101807)
* [compute lint levels by definition](https://github.com/rust-lang/rust/pull/101620)
* [deny return-position `impl Trait` in traits for object safety](https://github.com/rust-lang/rust/pull/101681)
* [do not suggest a placeholder to const and static without a type](https://github.com/rust-lang/rust/pull/101790)
* [avoid infinite loop in function arguments checking](https://github.com/rust-lang/rust/pull/100502)
* [fix ICE in `opt_suggest_box_span`](https://github.com/rust-lang/rust/pull/101604)
* [be careful about `expr_ty_adjusted` when noting block tail type](https://github.com/rust-lang/rust/pull/101629)
* [check that the types in return position `impl Trait` in traits are well-formed](https://github.com/rust-lang/rust/pull/101676)
* [impove diagnostic for `.await`ing non-futures](https://github.com/rust-lang/rust/pull/101723)
* [suggest pub instead of public for const type item](https://github.com/rust-lang/rust/pull/101668)
* [suggest associated const for incorrect use of let in traits](https://github.com/rust-lang/rust/pull/101843)
* [miri: implement a garbage collector for tags](https://github.com/rust-lang/miri/pull/2479)
* [miri: make `sleep` work with isolation enabled](https://github.com/rust-lang/miri/pull/2506)
* [miri: run the GC more often on Linux, not MacOS](https://github.com/rust-lang/miri/pull/2543)
* [do not fetch HIR node when iterating to find lint](https://github.com/rust-lang/rust/pull/101862)
* [extend list of targets that support dyanmic linking for llvm tools](https://github.com/rust-lang/rust/pull/101781)
* [normalize struct field types in `confirm_builtin_unsize_candidate`](https://github.com/rust-lang/rust/pull/101831)
* [streamline `register_res`](https://github.com/rust-lang/rust/pull/101830)
* [`rustc_error`, `rustc_private`: switch to stable hash containers](https://github.com/rust-lang/rust/pull/99334)
* [change `AccessLevels` representation](https://github.com/rust-lang/rust/pull/101713)
* [change `FnMutDelegate` to trait objects](https://github.com/rust-lang/rust/pull/101857)
* [change rlib format to distinguish native dependencies](https://github.com/rust-lang/rust/pull/100101)
* [ssa: implement `#[collapse_debuginfo]`](https://github.com/rust-lang/rust/pull/99556)
* [translations: migrate `rustc_session` to use `SessionDiagnostic` - Final](https://github.com/rust-lang/rust/pull/101266)
* [constify `PartialEq` for `Ordering`](https://github.com/rust-lang/rust/pull/101810)
* [constify impl Fn* &(mut) Fn*](https://github.com/rust-lang/rust/pull/101802)
* [constify some `CStr` methods](https://github.com/rust-lang/rust/pull/100291)
* [use `DisplayBuffer` for socket addresses](https://github.com/rust-lang/rust/pull/100640)
* [simplify `const` `memchr`](https://github.com/rust-lang/rust/pull/101784)
* [implement `std::marker::Tuple`](https://github.com/rust-lang/rust/pull/100251)
* [implement `simd_as` for pointers](https://github.com/rust-lang/rust/pull/98441)
* [stdarch: riscv: P extension intrinsics for packed SIMD (part 1)](https://github.com/rust-lang/stdarch/pull/1332)
* [cargo: expose cargo add internals as edit API](https://github.com/rust-lang/cargo/pull/11059)
* [cargo: take priority into account within the pending queue](https://github.com/rust-lang/cargo/pull/11032)
* [rustdoc: use more precise URLs for jump-to-definition links](https://github.com/rust-lang/rust/pull/101868)
* [clippy: Add `iter_kv_map` lint](https://github.com/rust-lang/rust-clippy/pull/9409)
* [clippy: Do not lint `use_self` in proc macro expansion](https://github.com/rust-lang/rust-clippy/pull/9454)
* [clippy: Don't lint `large_stack_array` inside static items](https://github.com/rust-lang/rust-clippy/pull/9466)
* [clippy: Don't panic on invalid shift while constfolding](https://github.com/rust-lang/rust-clippy/pull/9464)
* [clippy: Fix `FormatArgsExpn` parsing of `FormatSpec` positions](https://github.com/rust-lang/rust-clippy/pull/9469)
* [clippy: Fix `almost_complete_letter_range` false positive](https://github.com/rust-lang/rust-clippy/pull/9467)
* [clippy: Fix `unused_peekable` closure and `f(&mut peekable)` false positives](https://github.com/rust-lang/rust-clippy/pull/9465)
* [clippy: Make `derivable_impls` machine applicable](https://github.com/rust-lang/rust-clippy/pull/9429)
* [clippy: Make module-style lints resilient to `--remap-path-prefix`](https://github.com/rust-lang/rust-clippy/pull/9475)
* [clippy: Migrate write.rs to a late pass](https://github.com/rust-lang/rust-clippy/pull/8518)
* [clippy: Use `visit_expr_field` for `ParamPosition`](https://github.com/rust-lang/rust-clippy/pull/9458)
* [clippy: `arithmetic-side-effects` More non-overflowing ops](https://github.com/rust-lang/rust-clippy/pull/9474)
* [clippy: `arithmetic-side-effects` Finish non-overflowing ops](https://github.com/rust-lang/rust-clippy/pull/9483)
* [clippy: `bool_to_int_with_if` inverse case patch](https://github.com/rust-lang/rust-clippy/pull/9476)
* [rust-analyzer: Add a new configuration settings to set env vars when running cargo, rustc, etc. commands: cargo.extraEnv and checkOnSave.extraEnv](https://github.com/rust-lang/rust-analyzer/pull/13058)
* [rust-analyzer: Add config to unconditionally prefer core imports over std](https://github.com/rust-lang/rust-analyzer/pull/13212)
* [rust-analyzer: Allow configuration of annotation location](https://github.com/rust-lang/rust-analyzer/pull/13221)
* [rust-analyzer: Complete variants and assoc items in path pattern through type aliases](https://github.com/rust-lang/rust-analyzer/pull/13242)
* [rust-analyzer: Ensure at least one trait bound in `TyKind::DynTy`](https://github.com/rust-lang/rust-analyzer/pull/13264)
* [rust-analyzer: Filter imports on find-all-references](https://github.com/rust-lang/rust-analyzer/pull/13186)
* [rust-analyzer: Fix add reference action on macros](https://github.com/rust-lang/rust-analyzer/pull/13239)
* [rust-analyzer: Fix prelude injection](https://github.com/rust-lang/rust-analyzer/pull/13235)
* [rust-analyzer: Move reference imports filtering into `to_proto` layer](https://github.com/rust-lang/rust-analyzer/pull/13228)
* [rust-analyzer: New assist: `move_format_string_arg`](https://github.com/rust-lang/rust-analyzer/pull/13216)
* [rust-analyzer: Fix a crash](https://github.com/rust-lang/rust-analyzer/pull/13257)
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

Rusty Events between 2022-09-21 - 2022-10-19 🦀

### Virtual

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
* 2022-09-22 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Rust Iran Meetup #9 - Let's Write An Async Executor**](https://rust-meetup.ir/2022/09/22/9th-meetup.html)
* 2022-09-23 | Virtual (Tokyo, JP) | [Rust Tokyo](https://rust.tokyo)
    * [**Rust Tokyo 2022**](https://rust.tokyo/2022)
* 2022-09-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcmbkc/)
* 2022-09-28 | Virtual (London, UK) | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)
* 2022-10-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #18**](https://www.meetup.com/rust-noris/events/hlvbvsydcnbrb/)
* 2022-10-08 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-10-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-12 | Virtual (San Francisco, CA, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/)

### Europe

* 2022-09-15 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #52**](https://www.meetup.com/rust-paris/events/288136736/)
* 2022-09-27 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Rust at RU**](https://www.meetup.com/rust-nederland/events/288376119/)
    * [**Student track: Rust at RU**](https://www.meetup.com/rust-nederland/events/288440591/)
* 2022-09-28 | London, UK + Virtual | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)
* 2022-10-03 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Microsoft Reactor**](https://www.meetup.com/Stockholm-Rust/events/288453360/)
* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)

### North America

* 2022-09-14 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288287206/)
* 2022-09-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcmbbc/)
* 2022-09-22 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Game Prototyping with Rusty Engine with Nathan Stocks and Food!**](https://www.meetup.com/utah-rust/events/rvpgxsydcmbmc/)
* 2022-09-29 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Nuestra RustMX comunidad tiene página**](https://www.meetup.com/rust-mx/events/288388973/)

### Oceania

* 2022-09-14 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust-Sydney Lightning Talks**](https://www.meetup.com/rust-sydney/events/287979855/)
* 2022-09-20 | Phillip, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/288450836/)

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

> At the #LinuxPlumbers Rust MC: "I'm Matthew Wilcox, I'm one of the authors of the NVMe spec, I'm the one who suggested you make an NVMe driver to demonstrate the value of Rust. You have succeeded beyond my wildest expectations. These performance numbers are phenomenal."

– [Josh Triplett paraphrasing Matthew Wilcox as spoken at the Linux Plumbers Conference Q&A session](https://twitter.com/josh_triplett/status/1569363148985233414)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1291) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
