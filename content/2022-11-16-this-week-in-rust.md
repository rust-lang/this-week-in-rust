Title: This Week in Rust 469
Number: 469
Date: 2022-11-16
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
* [Rust Foundation Included in Fastly's Fast Forward Program](https://foundation.rust-lang.org/news/rust-foundation-included-in-fastly-s-fast-forward-program/)
* [Welcoming Seth Markle to the Rust Foundation Board](https://foundation.rust-lang.org/news/welcoming-seth-markle-to-the-rust-foundation-board/)

### Newsletters
* [This Month in Rust GameDev #39 - October 2022](https://gamedev.rs/news/039/)

### Project/Tooling Updates
* [hyper Polish Period](https://seanmonstar.com/post/701008919383932928/hyper-polish-period)
* [Changelog #155](https://rust-analyzer.github.io/thisweek/2022/11/14/changelog-155.html)
* [This Week in Fyrox](https://fyrox.rs/blog/post/twif2/)
* [Bevy - Bevy 0.9](https://bevyengine.org/news/bevy-0-9/)
* [video] [Bevy 0.9 Release Train](https://www.youtube.com/watch?v=mxYMduEC7xs)
* [Fornjot (code-first CAD in Rust) - Progress Report](https://www.fornjot.app/blog/progress-report-2022-11/)
* [What's new in SeaORM 0.10.x](https://www.sea-ql.org/blog/2022-11-10-whats-new-in-0.10.x/)

### Observations/Thoughts
* [Error docs](https://www.ncameron.org/blog/error-docs/)
* [Rust enum-match code generation](https://www.eventhelix.com/rust/rust-to-assembly-enum-match/)
* [Implementation Details of async Rust](https://swatinem.de/blog/async-codegen/)
* [Stop writing Rust linked list libraries!](https://diziet.dreamwidth.org/13476.html)
* [video] [Rust GCC Front-end - Philip Herron, David Faust](https://www.youtube.com/watch?v=D8FX1enoZnM)
* [video] [Rust Is Boring (Why Rust?) [RUST-13]](https://www.youtube.com/watch?v=oY0XwMOSzq4)
* [audio] [cargo-auditable with Sergey Davidoff](https://rustacean-station.org/episode/sergey-davidoff/)

### Rust Walkthroughs
* [Rust and C++ Interoperability](https://slint-ui.com/blog/rust-and-cpp.html)
* [4-Step Primer on Navigating Embedded Rust HAL Documentation](https://apollolabsblog.hashnode.dev/4-step-primer-on-navigating-embedded-rust-hal-documentation)
* [Five simple steps to use any Arduino C++ library in a Rust project](https://dev.to/kgrech/five-simple-steps-to-use-any-arduino-c-library-in-a-rust-project-1k78)
* [Tower, Episode 3: Readiness](https://heikoseeberger.de/2022/11/07/2022-11-08-tower-3/)

### Miscellaneous
* [ARP Injection in Rust on Linux](https://leshow.github.io/post/linux_arp_injection/)

## Crate of the Week

This week's crate is [lngcnv](https://crates.io/crates/lngcnv), a linguistic command line tool.

Thanks to [Piotr Bajdek](https://users.rust-lang.org/t/crate-of-the-week/2704/1124) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [hackdose-sml-parser - automate generation of string and list types](https://github.com/torfmaster/hackdose-sml-parser/issues/2)
* [hackdose-sml-parser - add more obis codes](https://github.com/torfmaster/hackdose-sml-parser/issues/3)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

373 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-11-07..2022-11-14

* [add new tier-3 target `loongarch64`](https://github.com/rust-lang/rust/pull/101939)
* [add tier 3 `no_std AArch64/x86_64` support for the QNX Neutrino RTOS](https://github.com/rust-lang/rust/pull/102701)
* [promote `{aarch64,i686,x86_64}-unknown-uefi` to Tier 2](https://github.com/rust-lang/rust/pull/103933)
* [linker: refactoring and fixes to native library linking](https://github.com/rust-lang/rust/pull/103311)
* [limit efiapi calling convention to supported arches](https://github.com/rust-lang/rust/pull/104020)
* [implement the `+whole-archive` modifier for `wasm-ld`](https://github.com/rust-lang/rust/pull/102215)
* [allow specialized const trait impls](https://github.com/rust-lang/rust/pull/95292)
* [parser: recover from using colon as path separator in imports](https://github.com/rust-lang/rust/pull/103443)
* [recover wrong-cased keywords that start items](https://github.com/rust-lang/rust/pull/99918)
* [resolve: more detailed effective visibility tracking for imports](https://github.com/rust-lang/rust/pull/103965)
* [emit error in `collecting_trait_impl_trait_tys` on mismatched signatures](https://github.com/rust-lang/rust/pull/104214)
* [fix auto-application of associated generic functions with placeholders](https://github.com/rust-lang/rust/pull/101990)
* [suggest `is_some` when we've found `Option` but expected `bool`](https://github.com/rust-lang/rust/pull/104036)
* [suggest calling the instance method of the same name when method not found](https://github.com/rust-lang/rust/pull/103531)
* [suggest removing unnecessary `.` to use a floating point literal](https://github.com/rust-lang/rust/pull/104144)
* [improve spans with `use crate::{self}`](https://github.com/rust-lang/rust/pull/104315)
* [tighten the 'introduce new binding' suggestion](https://github.com/rust-lang/rust/pull/104186)
* [fix `rustc_parse_format` spans following escaped utf-8 multibyte chars](https://github.com/rust-lang/rust/pull/103651)
* [consider `#[must_use]` annotation on `async fn` as also affecting the `Future::Output`](https://github.com/rust-lang/rust/pull/100633)
* [better error message for HRTB error from generator interior](https://github.com/rust-lang/rust/pull/103171)
* [`#[test]`: point at return type if `Termination` bound is unsatisfied](https://github.com/rust-lang/rust/pull/103445)
* [`rustc_codegen_ssa`: better code generation for niche discriminants](https://github.com/rust-lang/rust/pull/102872)
* [add support for custom mir](https://github.com/rust-lang/rust/pull/103464)
* [add the `#[derive_const]` attribute](https://github.com/rust-lang/rust/pull/102049)
* [delay `include_bytes` to AST lowering](https://github.com/rust-lang/rust/pull/103812)
* [don't normalize constants unless they need normalization](https://github.com/rust-lang/rust/pull/104063)
* [resolve lifetimes independently for each item-like](https://github.com/rust-lang/rust/pull/103530)
* [improve performance of `rem_euclid()` for signed integers](https://github.com/rust-lang/rust/pull/103913)
* [make `Sized` coinductive, again](https://github.com/rust-lang/rust/pull/100386)
* [make `Hash`, `Hasher` and `BuildHasher` `#[const_trait]` and make `Sip` const `Hasher`](https://github.com/rust-lang/rust/pull/104060)
* [const Compare for Tuples](https://github.com/rust-lang/rust/pull/104125)
* [merge crossbeam-channel into `std::sync::mpsc`](https://github.com/rust-lang/rust/pull/93563)
* [specialize `iter::ArrayChunks::fold` for TrustedRandomAccess iterators](https://github.com/rust-lang/rust/pull/103446)
* [stabilize integer logarithms](https://github.com/rust-lang/rust/pull/103570)
* [hashbrown: `rawTable::allocation_info`](https://github.com/rust-lang/hashbrown/pull/371)
* [backtrace: C/C++ module file and line number are not looked up:  addr2line 0.18.0 fixes it](https://github.com/rust-lang/backtrace-rs/pull/492)
* [impl `TryFrom<&[T]>` for Simd](https://github.com/rust-lang/portable-simd/pull/314)
* [detect CPU features with Linux methods on Android for non-Intel CPUs](https://github.com/rust-lang/stdarch/pull/1351)
* [cargo: propagate change of artifact bin dep to its parent fingerprint](https://github.com/rust-lang/cargo/pull/11353)
* [rustdoc: fix missing reexports' doc comments](https://github.com/rust-lang/rust/pull/104292)
* [rustdoc: use `ThinVec` and `Box<str>` to shrink `clean::ItemKind`](https://github.com/rust-lang/rust/pull/104013)
* [bindgen: fix duplicated function names](https://github.com/rust-lang/rust-bindgen/pull/2341)
* [bindgen: fix inline function identification](https://github.com/rust-lang/rust-bindgen/pull/2340)
* [bindgen: handle the `const struct *` and `struct *` patterns](https://github.com/rust-lang/rust-bindgen/pull/2304)
* [clippy: `fn_params_excessive_bools` Make it possible to allow the lint at the method level](https://github.com/rust-lang/rust-clippy/pull/9698)
* [clippy: `result_large_err` show largest variants in err msg](https://github.com/rust-lang/rust-clippy/pull/9662)
* [clippy: add `manual_is_ascii_check` lint](https://github.com/rust-lang/rust-clippy/pull/9765)
* [clippy: add `unnecessary_safety_doc` lint](https://github.com/rust-lang/rust-clippy/pull/9822)
* [clippy: Fix two `needless_borrow` false positives](https://github.com/rust-lang/rust-clippy/pull/9791)
* [clippy: avoid linting unsized mutable reference](https://github.com/rust-lang/rust-clippy/pull/9835)
* [clippy: fix `explicit_auto_deref` false positives](https://github.com/rust-lang/rust-clippy/pull/9813)
* [clippy: fix `is_async_fn` to check `FnKind::Method`](https://github.com/rust-lang/rust-clippy/pull/9836)
* [clippy: fix `never_loop` false positive](https://github.com/rust-lang/rust-clippy/pull/9837)
* [clippy: fix `vec-box-size-threshold` off-by-one error](https://github.com/rust-lang/rust-clippy/pull/9848)
* [clippy: fix: `cognitive_complexity` for async fn](https://github.com/rust-lang/rust-clippy/pull/9828)
* [clippy: make `bool_to_int_with_if` a pedantic lint](https://github.com/rust-lang/rust-clippy/pull/9830)
* [clippy: make it clear that `or_fun_call` can be a false positive](https://github.com/rust-lang/rust-clippy/pull/9829)
* [rust-analyzer: nest Cargo.lock under Cargo.toml in Code](https://github.com/rust-lang/rust-analyzer/pull/13582)
* [rust-analyzer: fix `tt::Punct`'s spacing calculation](https://github.com/rust-lang/rust-analyzer/pull/13548)
* [rust-analyzer: add trait alias grammar to rust.ungram](https://github.com/rust-lang/rust-analyzer/pull/13606)
* [rust-analyzer: check visibility of each path segment](https://github.com/rust-lang/rust-analyzer/pull/13602)
* [rust-analyzer: fix hover in attributed items not preferring similar kinded tokens](https://github.com/rust-lang/rust-analyzer/pull/13604)
* [rust-analyzer: fix item completions not working properly after unit structs and outline modules](https://github.com/rust-lang/rust-analyzer/pull/13581)
* [rust-analyzer: fix panic when computing signature of generic `FnOnce` callable](https://github.com/rust-lang/rust-analyzer/pull/13584)
* [rust-analyzer: fix r-a eagerly showing no discovered workspace errors](https://github.com/rust-lang/rust-analyzer/pull/13605)
* [rust-analyzer: send status notification if there are no found workspaces](https://github.com/rust-lang/rust-analyzer/pull/13603)
* [crates.io downloads: use `recent_downloads` as `downloads` if it is higher](https://github.com/rust-lang/crates.io/pull/5426)

### Rust Compiler Performance Triage

A light week for triage. The biggest of the three regressions has a (hopeful)
fix up already. The second biggest is a regression we are accepting for sake
of correctness of incremental-compilation. The third regression is small and may
well be removed as the type system internals are improved. max-rss seems stable.

Triage done by **@pnkfelix**.
Revision range: [57d3c58e..96ddd32c](https://perf.rust-lang.org/?start=57d3c58ed6e0faf89a62411f96c000ffc9fd3937&end=96ddd32c4bfb1d78f0cd03eb068b1710a8cebeef&absolute=false&stat=instructions%3Au)

3 Regressions, 4 Improvements, 3 Mixed; 2 of them in rollups
40 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-11-14.md)


### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Restrictions](https://github.com/rust-lang/rfcs/pull/3323)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Remove drop order twist of && and || and make them associative](https://github.com/rust-lang/rust/pull/103293)
* [disposition: merge] [Unreserve braced enum variants in value namespace](https://github.com/rust-lang/rust/pull/103578)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: `c"…"` string literals](https://github.com/rust-lang/rfcs/pull/3348)
* [new] [Allow requiring "at least one feature"](https://github.com/rust-lang/rfcs/pull/3347)

## Upcoming Events

Rusty Events between 2022-11-16 - 2022-12-14 🦀

### Virtual

* 2022-11-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcpbvb/)
* 2022-11-17 | Virtual (Amsterdam, NL) | [ITGilde Tech-Talks](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups)
    * [**Introduction “Rust” an ITGilde Tech Talk delivered by Pascal van Dam**](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups/events/289167373/)
* 2022-11-17 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: delta-rs at Back Market: Python and Rust, the best of both worlds**](https://www.meetup.com/data-ai-online/events/289623322/)
* 2022-11-17 | Virtual (San Francisco, CA, US) | [WebAssembly North America](https://www.meetup.com/wasmna/)
    * [**Rust 🤝 Wasm with Alex Crichton**](https://www.meetup.com/wasmna/events/289623268/)
* 2022-11-17 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcpbwb/)
* 2022-11-21 | Virtual (Paris, FR) | [Meetup Paris - École Supérieure de Génie Informatique (ESGI)](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique)
    * [**Découverte de WebAssembly**](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique/events/289112753/)
* 2022-11-22 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/11/22/rust-hack-and-learn.html)
* 2022-11-24 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 27th Edition**](https://www.meetup.com/rust-linz/events/289251460/)
* 2022-11-28 | Virtual | [Rust Formal Methods Interest Group](https://www.eventbrite.com/o/rust-formal-methods-interest-group-34404947509)
    * [**MiniRust with Ralf Jung**](https://www.eventbrite.com/e/minirust-with-ralf-jung-tickets-460741328717)
* 2022-11-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcpbmc/)
* 2022-11-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Common crates and their usage**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289645553/)
* 2022-11-30 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)
* 2022-12-01 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Exploring USB with Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/289563986/)
* 2022-12-06 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/12/06/rust-hack-and-learn.html)
* 2022-12-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcqbjb/)
* 2022-12-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/287027660/)
* 2022-12-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #20**](https://www.meetup.com/rust-noris/events/hlvbvsydcqblb/)
* 2022-12-08 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: The Genesis of Delta Rust with QP Hou**](https://www.meetup.com/data-ai-online/events/289672886/)
* 2022-12-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-12-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/289352426/)
* 2022-12-13 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 25u16**](https://www.meetup.com/rust-saar/events/289663288/)
* 2022-12-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcqbsb/)

### Asia

* 2022-11-22 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Fullstack Web Dev in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/289661246/)

### Europe

* 2022-11-16 | Paris, FR | [Stockly](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-444152621447?aff=ebdssbdestsearch)
* 2022-11-22 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/)
    * [**Rust Meet-up**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/289674810/)
* 2022-11-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Initial Meet and Greet Rust meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/289028178/)
* 2022-11-24 | København, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #31**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179132/)
* 2022-11-28 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust London Code Dojo: Rust with Front-End Web Assembly**](https://www.meetup.com/rust-london-user-group/events/289631916/)
* 2022-11-30 | Amsterdam, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust in Critical Infrastructure**](https://www.meetup.com/rust-nederland/events/289204146/)
* 2022-11-30 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet)
    * [**Rust Lille #1**](https://www.meetup.com/meetup-group-zgphbyet/events/289620614/)
* 2022-11-30 | Milan, IT | [Rust Language Milano](https://www.meetup.com/rust-language-milano/)
    * [**Welcome GAT!!**](https://www.meetup.com/rust-language-milano/events/289767176/)
* 2022-11-30 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)
* 2022-12-01 | Edinburgh, UK | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**December Talks + Rust Book Raffle**](https://www.meetup.com/rust-edi/events/289582990/)
* 2022-12-07 | Zurich, CH | [Rust Zurich](https://www.meetup.com/Rust-Zurich/)
    * [**Next generation i18n with rust (icu4x) and zero-copy deserialization**](https://www.meetup.com/rust-zurich/events/289518586/)
* 2022-12-12 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Rust Meetup - Subject TBA**](https://www.meetup.com/dutch-rust-meetup/events/289021643/)

### North America

* 2022-11-29 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**Atx Rustaceans Meetup**](https://www.meetup.com/atx-rustaceans/events/289594614/)
* 2022-12-08 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcqblb/)

### Oceania

* 2022-11-22 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/288615873/)
* 2022-11-24 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**November Meetup**](https://www.meetup.com/rust-brisbane/events/289539610/)
* 2022-12-08 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**December 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/289745823/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/ymepy8/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> What you are essentially saying is: "Doctor, I'm writing C in Rust, and it hurts." To which the doctor will reply: "Then don't write C in Rust, and it won't hurt!"

– [Árpád Goretity on rust-users](https://users.rust-lang.org/t/rust-applicability-to-small-embedded-codebase-getting-discouraged/84049/17)

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1332) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/yxe6ex/this_week_in_rust_469/)</small>
