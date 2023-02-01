Title: This Week in Rust 478
Number: 478
Date: 2023-01-18
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Project/Tooling Updates
* [IntelliJ Rust Changelog #186](https://intellij-rust.github.io/2023/01/16/changelog-186.html)
* [rust-analyzer changelog #164](https://rust-analyzer.github.io/thisweek/2023/01/16/changelog-164.html)
* [This Week in Fyrox](https://fyrox.rs/blog/post/twif10/)
* [clap v4.1](https://epage.github.io/blog/2023/01/clap-v4-1/)
* [Fornjot (code-first CAD in Rust) - Weekly Release](https://www.fornjot.app/blog/weekly-release/2023-w03/)
* [Release of sphrs 0.2.0, a spherical harmonics library](http://argmin-rs.org/blog/sphrs-v0-2-0/)

### Observations/Thoughts
* [Rails developers write some Rust: a review of Axum 0.6](https://thoughtbot.com/blog/axum-from-a-rails-perspective)
* [Rust should own its debugger experience](https://blog.yoshuawuyts.com/rust-should-own-its-debugger-experience/)
* [The Hidden Control Flow — Some Insights on an Async Cancellation Problem in Rust](https://greptime.com/blogs/2023-01-12-hidden-control-flow)
* [Fallible - The Lost Sibling of Result and Option](https://datavirke.dk/posts/fallible-missing-rust-error-handling/)
* [Folding arguments into the macro](https://nnmm.github.io/rust/2023/01/11/folding-arguments-into-the-macro.html)
* [Zero To Production book review](https://rusty-ferris.pages.dev/blog/zero-to-production-book-review/)
* [We Need Type Information, Not Stable ABI](https://blaz.is/blog/post/we-dont-need-a-stable-abi/)
* [Comparison of web frameworks written in Java, nodejs and Rust](https://github.com/bwysocki/rocket-nest-spring/)
* [This year I tried solving AoC using Rust, here are my impressions coming from Python!](https://duarteocarmo.com/blog/on-rust)

### Rust Walkthroughs
* [Create a Rust worker | Wasm Workers Server](https://workers.wasmlabs.dev/docs/tutorials/rust-workers/)
* [Displaying Images on ESP32 with Rust!](https://lilymara.xyz/posts/images-esp32/)
* [Rust FFI and bindgen: Integrating Embedded C Code in Rust](https://apollolabsblog.hashnode.dev/rust-ffi-and-bindgen-integrating-embedded-c-code-in-rust)
* [Finding Nice MD5s Using Rust](https://blog.youmu.moe/posts/finding-nice-md5s-using-rust/)
* [2D game base with Bevy and LDtk (linked wasm)](https://radim.xyz/project/rusty_woods/)
* [Song search in Rust using OpenAI](https://gigapotential.dev/blog/song-search-in-rust-using-openai/)
* [Build a ray tracer, pt. 1 - 2D Image](https://www.superperfundo.tech/articles/ray-tracer-part1)

### Miscellaneous
* [Building an out-of-tree Rust Kernel Module Part Two](https://blog.rnstlr.ch/building-an-out-of-tree-rust-kernel-module-part-two.html)
* [Using Rust to write a Data Pipeline. Thoughts. Musings.](https://www.confessionsofadataguy.com/using-rust-to-write-a-data-pipeline-thoughts-musings/)
* [video] [C++ vs Rust: which is faster?](https://www.youtube.com/watch?v=VMpSYJ_7aYM)
* [video] [Everything You Wanted to Know About Rust Unit Testing (and then some more)](https://youtube.com/watch?v=_jDKeOtOiEo)
* [video] [Introduction to rust operators for Kubernetes](https://www.youtube.com/watch?v=feBYxeO-3cY)
* [DE] [Rust-Framework: Turmoil testet verteilte Systeme](https://www.heise.de/news/Rust-Framework-Turmoil-testet-verteilte-Systeme-7449772.html)
* [DE] [Rust: bis zu 2500 Projekte durch Bibliothek Hyper für DoS verwundbar](https://www.heise.de/news/Rust-bis-zu-2500-Projekte-durch-Bibliothek-Hyper-fuer-DoS-verwundbar-7451019.html)
* [DE] [Ferris Talk #13: Rust-Web-APIs und Mocking mit Axum](https://www.heise.de/hintergrund/Ferris-Talk-13-Rust-Web-APIs-und-Mocking-mit-Axum-7457143.html)
* [DE] [Open-Source-Browser: Google öffnet Chromium für Rust](https://www.heise.de/news/Open-Source-Browser-Google-oeffnet-Chromium-fuer-Rust-7458091.html)

## Crate of the Week

This week's crate is [syntactic-for](https://crates.io/crates/syntactic-for), a syntactic "for" loop Rust macro.

Thanks to [Tor Hovland](https://users.rust-lang.org/t/crate-of-the-week/2704/1148) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - GitHub CI: use global default shell in documentation.yml workflow](https://github.com/build-trust/ockam/issues/3997)
* [Ockam - Modify clap command ockam start to set the node attribute's default value using attributes](https://github.com/build-trust/ockam/issues/4080)
* [Ockam - Add optional --identity argument to clap command secure-channel-listener create and modify its API handler](https://github.com/build-trust/ockam/issues/3907)


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

458 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-01-09..2023-01-16

* [Initial `#[do_not_recommend]` implementation](https://github.com/rust-lang/rust/pull/106645) (RFC [#2397](https://rust-lang.github.io/rfcs/2397-do-not-recommend.html))
* [LSDA Take `ttype_index` into account when taking unwind action](https://github.com/rust-lang/rust/pull/106446)
* [add checks for the signature of the `start` lang item](https://github.com/rust-lang/rust/pull/106092)
* [add log-backtrace option to show backtraces along with logging](https://github.com/rust-lang/rust/pull/104645)
* [add note when `FnPtr` vs. `FnDef` impl trait](https://github.com/rust-lang/rust/pull/106665)
* [adding a hint on iterator type errors](https://github.com/rust-lang/rust/pull/106740)
* [allow codegen to unsize `dyn*` to `dyn`](https://github.com/rust-lang/rust/pull/106532)
* [change flags with a fixed default value from `Option<bool>` to bool](https://github.com/rust-lang/rust/pull/106671)
* [check `impl`'s `where` clauses in `consider_impl_candidate` in experimental solver](https://github.com/rust-lang/rust/pull/106397)
* [collect and emit proper backtraces for `delay_span_bug`s](https://github.com/rust-lang/rust/pull/106321)
* [consider return type when giving various method suggestions](https://github.com/rust-lang/rust/pull/106607)
* [const closures](https://github.com/rust-lang/rust/pull/106004)
* [deprioritize fulfillment errors that come from expansions](https://github.com/rust-lang/rust/pull/106820)
* [detect out of bounds range pattern value](https://github.com/rust-lang/rust/pull/106622)
* [detect struct literal needing parentheses](https://github.com/rust-lang/rust/pull/106620)
* [disable "split dwarf inlining" by default](https://github.com/rust-lang/rust/pull/106709)
* [emit a hint for bad call return types due to generic arguments](https://github.com/rust-lang/rust/pull/106752)
* [emit a single error for contiguous sequences of unknown tokens](https://github.com/rust-lang/rust/pull/106566)
* [emit only one nbsp error per file](https://github.com/rust-lang/rust/pull/106872)
* [enable atomic cas for bpf targets](https://github.com/rust-lang/rust/pull/105708)
* [exclude formatting commit from blame](https://github.com/rust-lang/rust/pull/106778)
* [feed a bunch of queries instead of tracking fields on TyCtxt](https://github.com/rust-lang/rust/pull/106776)
* [fix ICE formatting](https://github.com/rust-lang/rust/pull/106791)
* [fix `aarch64-unknown-linux-gnu_ilp32` target](https://github.com/rust-lang/rust/pull/106646)
* [fix `unused_braces` on generic const expr macro call](https://github.com/rust-lang/rust/pull/106563)
* [fix bad import suggestion with nested `use` tree](https://github.com/rust-lang/rust/pull/106175)
* [fix help docs for -Zallow-features](https://github.com/rust-lang/rust/pull/106653)
* [fix invalid files array re-creation in rustdoc-gui tester](https://github.com/rust-lang/rust/pull/106689)
* [fix invalid syntax and incomplete suggestion in impl Trait parameter type suggestions for E0311](https://github.com/rust-lang/rust/pull/106167)
* [fix linker detection for linker (drivers) with a version postfix (e.g. clang-12 instead of clang)](https://github.com/rust-lang/rust/pull/106489)
* [fix misleading "add dyn keyword before derive macro" suggestion](https://github.com/rust-lang/rust/pull/106072)
* [improve fluent error messages](https://github.com/rust-lang/rust/pull/106427)
* [label `struct/enum constructor` instead of `fn item`, mention that it should be called on type mismatch](https://github.com/rust-lang/rust/pull/106524)
* [mark ZST as FFI-safe if all its fields are PhantomData](https://github.com/rust-lang/rust/pull/106675)
* [move autoderef to `rustc_hir_analysis`](https://github.com/rust-lang/rust/pull/106170)
* [new trait solver: rebase impl substs for gats correctly](https://github.com/rust-lang/rust/pull/106835)
* [cargo: nightly Fix CVE-2022-46176: Missing SSH host key validation](https://github.com/rust-lang/rust/pull/106687)
* [note predicate span on `ImplDerivedObligation`](https://github.com/rust-lang/rust/pull/106703)
* [only suggest adding type param if path being resolved was a type](https://github.com/rust-lang/rust/pull/106909)
* [prefer non-`[type error]` candidates during selection](https://github.com/rust-lang/rust/pull/106309)
* [provide help on closures capturing self causing borrow checker errors](https://github.com/rust-lang/rust/pull/106641)
* [recover from where clauses placed before tuple struct bodies](https://github.com/rust-lang/rust/pull/106537)
* [remove unnecessary lseek syscall when using `std::fs::read`](https://github.com/rust-lang/rust/pull/106664)
* [render missing generics suggestion verbosely](https://github.com/rust-lang/rust/pull/106608)
* [report fulfillment errors in new trait solver](https://github.com/rust-lang/rust/pull/106705)
* [specialize impl of `ToString` on `bool`](https://github.com/rust-lang/rust/pull/106662)
* [stabilize `::{core,std}::pin::pin!`](https://github.com/rust-lang/rust/pull/103800)
* [stabilize `abi_efiapi` feature](https://github.com/rust-lang/rust/pull/105795)
* [stabilize `f16c_target_feature`](https://github.com/rust-lang/rust/pull/106323)
* [stop probing for statx unless necessary](https://github.com/rust-lang/rust/pull/106661)
* [suggest `is_empty` for collections when casting to `bool`](https://github.com/rust-lang/rust/pull/106896)
* [suggest making private tuple struct field public](https://github.com/rust-lang/rust/pull/106579)
* [suggestion for type mismatch when we need a u8 but the programmer wrote a char literal](https://github.com/rust-lang/rust/pull/106859)
* [tweak E0277 `&`-removal suggestions](https://github.com/rust-lang/rust/pull/106360)
* [tweak E0599 and `elaborate_predicates`](https://github.com/rust-lang/rust/pull/106788)
* [support eager subdiagnostics again](https://github.com/rust-lang/rust/pull/105806)
* [libcore: make result of `iter::from_generator` `Clone`](https://github.com/rust-lang/rust/pull/105526)
* [add `AtomicPtr::as_mut_ptr`](https://github.com/rust-lang/rust/pull/106762)
* [leak amplification for `peek_mut()` to ensure BinaryHeap's invariant is always met](https://github.com/rust-lang/rust/pull/105851)
* [fix `mpsc::SyncSender` spinning behavior](https://github.com/rust-lang/rust/pull/106701)
* [futures: fix panic when `Unfold` sink return an error](https://github.com/rust-lang/futures-rs/pull/2686)
* [futures: fix `FuturesOrdered`](https://github.com/rust-lang/futures-rs/pull/2664)
* [cargo: `cargo metadata` supports artifact dependencies](https://github.com/rust-lang/cargo/pull/11550)
* [cargo: support `codegen-backend` and `rustflags` in profiles in config file](https://github.com/rust-lang/cargo/pull/11562)
* [clippy: `cast_possible_truncation` Suggest `TryFrom` when truncation possible](https://github.com/rust-lang/rust-clippy/pull/10038)
* [clippy: `expl_impl_clone_on_copy`: ignore packed structs with type/const params](https://github.com/rust-lang/rust-clippy/pull/10189)
* [clippy: `needless_return`: remove all semicolons on suggestion](https://github.com/rust-lang/rust-clippy/pull/10187)
* [clippy: `unused_self`: don't trigger if the method body contains `todo!()`](https://github.com/rust-lang/rust-clippy/pull/10166)
* [clippy: allow implementing `Hash` with derived `PartialEq` (`derive_hash_xor_eq`)](https://github.com/rust-lang/rust-clippy/pull/10184)
* [clippy: move `unchecked_duration_subtraction` to pedantic](https://github.com/rust-lang/rust-clippy/pull/10194)
* [rust-analyzer: add basic tooltips to adjustment hints](https://github.com/rust-lang/rust-analyzer/pull/13947)
* [rust-analyzer: assist: desugar doc-comment](https://github.com/rust-lang/rust-analyzer/pull/13935)
* [rust-analyzer: comment out disabled code](https://github.com/rust-lang/rust-analyzer/pull/13862)
* [rust-analyzer: derive 'Hash'](https://github.com/rust-lang/rust-analyzer/pull/13919)
* [rust-analyzer: make `unlinked_file` diagnostic quickfixes work for inline modules](https://github.com/rust-lang/rust-analyzer/pull/13934)
* [rust-analyzer: fix panicking Option unwraping in match arm analysis](https://github.com/rust-lang/rust-analyzer/pull/13940)
* [rust-analyzer: fix ty should query impls in nearest block](https://github.com/rust-lang/rust-analyzer/pull/13897)
* [rust-analyzer: check orpat in missing match](https://github.com/rust-lang/rust-analyzer/pull/13945)
* [rust-analyzer: don't generate `PartialEq`/`PartialOrd` methods body when types don't match](https://github.com/rust-lang/rust-analyzer/pull/13961)
* [rust-analyzer: make inlay hint location links work for more types](https://github.com/rust-lang/rust-analyzer/pull/13948)
* [rust-analyzer: interior-mutable types should be `static` rather than `const`](https://github.com/rust-lang/rust-analyzer/pull/13936)
* [rust-analyzer: remove hover inlay tooltips, replace them with location links](https://github.com/rust-lang/rust-analyzer/pull/13946)
* [rust-analyzer: remove recursive `Display` implementations](https://github.com/rust-lang/rust-analyzer/pull/13937)
* [rust-analyzer: split out hir-def attribute handling parts into hir-expand](https://github.com/rust-lang/rust-analyzer/pull/13917)
* [rust-analyzer: unconditionally enable location links in inlay hints again](https://github.com/rust-lang/rust-analyzer/pull/13963)

### Rust Compiler Performance Triage


Nearly all flagged regressions are likely noise, except one rollup with minor
impact on diesel that we will follow up on. We had a broad (albeit small) win
from [#106294](https://github.com/rust-lang/rust/pull/106294).

Triage done by **@pnkfelix**.
Revision range: [0442fbab..1f72129f](https://perf.rust-lang.org/?start=0442fbabe24ec43636a80ad1f40a0ad92a2e38df&end=1f72129ffe5e8c495113f9a2d4e1730f7fad3209&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 1.7%]   | 39    |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.2%, 1.8%]   | 23    |
| Improvements ✅ <br /> (primary)   | -0.4% | [-0.6%, -0.2%] | 7     |
| Improvements ✅ <br /> (secondary) | -0.4% | [-0.6%, -0.2%] | 6     |
| All ❌✅ (primary)                 | 0.3%  | [-0.6%, 1.7%]  | 46    |


4 Regressions, 3 Improvements, 3 Mixed; 4 of them in rollups
50 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-01-18.md)
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

* [disposition: close] [use implied bounds from impl header when comparing trait and impl methods](https://github.com/rust-lang/rust/pull/105548)
* [disposition: merge] [rustdoc: change trait bound formatting"](https://github.com/rust-lang/rust/pull/102842)
* [disposition: merge] [Make ExitStatus implement Default](https://github.com/rust-lang/rust/pull/106425)
* [disposition: merge] [Allow fmt::Arguments::as_str() to return more Some(_).](https://github.com/rust-lang/rust/pull/106823)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: `CARGO_TARGET_DIRECTORIES`, parent of all target directories](https://github.com/rust-lang/rfcs/pull/3371)
* [new] [RFC: (Re)standardise error code documentation](https://github.com/rust-lang/rfcs/pull/3370)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-01-18 - 2023-02-15 🦀

### Virtual

* 2023-01-18 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224523/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224525/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224523/) | [**Sao Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224522/)
* 2023-01-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfccbxb/)
* 2023-01-19 | Virtual (Redmond, WA, US; San Francisco, CA, US; New York, NY, US; Stockholm, SE) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor Stockholm](https://www.meetup.com/microsoft-reactor-stockholm/)
    * [**Crack code interview problems in Rust - Ep. 2**](https://www.meetup.com/microsoft-reactor-redmond/events/290085767/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290085766/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290085770/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290085769/)
* 2023-01-19 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfccbzb/)
* 2023-01-23 | Virtual (Durham, NC, US) | [Triangle Rust](https://www.meetup.com/triangle-rust/)
    * [**Online Code and Chat**](https://www.meetup.com/triangle-rust/events/290712105/)
* 2023-01-23 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 29th Edition**](https://www.meetup.com/rust-linz/events/290995162/)
* 2023-01-23 | Virtual (New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Condiciones con expresiones if/else en Rust**](https://www.meetup.com/microsoft-reactor-new-york/events/290224532/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224533/)
* 2023-01-24 | Virtual (Redmond, WA, US; New York, NY, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Uso de bucles para iterar por datos en Rust**](https://www.meetup.com/microsoft-reactor-redmond/events/290224536/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224538/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224535/)
* 2023-01-25 | Virtual (Redmond, WA, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224544/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224540/)
* 2023-01-26 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust Lightning Talks!**](https://www.meetup.com/charlottesville-rust-meetup/events/290122935/)
* 2023-01-26 | Virtual (Redmond, WA, US; San Francisco, CA, US; New York, NY, US; Stockholm, SE) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor Stockholm](https://www.meetup.com/microsoft-reactor-stockholm/)
    * [**Crack code interview problems in Rust - Ep. 3**](https://www.meetup.com/microsoft-reactor-redmond/events/290086420/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290086421/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290086422/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290086418/)
* 2023-01-30 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Control de errores en Rust**](https://www.meetup.com/microsoft-reactor-redmond/events/290224559/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224558/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224561/)
* 2023-01-31 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/290697014/)
* 2023-01-31 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfccbpc/)
* 2023-01-31 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Compresión de cómo Rust administra la memoria**](https://www.meetup.com/microsoft-reactor-redmond/events/290224861/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224860/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224859/)
* 2023-02-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfcdbcb/)
* 2023-02-01 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224570/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224566/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224570/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224568/)
* 2023-02-01 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcdbcb/)
* 2023-02-06 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224572/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Implementación de tipos y rasgos genéricos**](https://www.meetup.com/microsoft-reactor-redmond/events/290224576/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224572/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224573/)
* 2023-02-07 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/290609896/)
* 2023-02-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcdbkb/)
* 2023-02-07 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Módulos, paquetes y contenedores de terceros**](https://www.meetup.com/microsoft-reactor-redmond/events/290224578/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224577/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224579/)
* 2023-02-08 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224584/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224583/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224585/)
* 2023-02-13 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Escritura de pruebas automatizadas**](https://www.meetup.com/microsoft-reactor-redmond/events/290224610/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224608/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224609/)
* 2023-02-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfcdbsb/)
* 2023-02-14 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust - Creamos un programa de ToDos en la línea de comandos**](https://www.meetup.com/microsoft-reactor-redmond/events/290224616/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224613/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224617/)
* 2023-02-14 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 26u16**](https://www.meetup.com/rust-saar/events/290040138/)
* 2023-02-15 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224621/)
* 2023-02-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfcdbtb/)

### Asia

* 2023-01-15 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Property-Based Testing in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/290667325/)
* 2023-02-01 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust)
    * [**Rust talk: How to implement Iterator on tuples... kind of**](https://www.meetup.com/kansai-rust/events/291020672)

### Europe

* 2023-01-20 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfccbbc/)
* 2023-01-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #55**](https://www.meetup.com/rust-paris/events/290100223/)
* 2023-01-26 | Copenhagen, DK | [Copenhagen Rust Meetup Group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #32**](https://www.meetup.com/copenhagen-rust-meetup-group/events/290037532/)
* 2023-02-02 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn February 2023**](https://www.meetup.com/rust-meetup-hamburg/events/290824576/)
* 2023-02-02 | Lyon, FR | [Rust Lyon](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)
    * [**Rust Lyon meetup #01**](https://mobilizon.fr/events/b8577678-d072-4d9a-9562-974715f1dfbb)


### North America

* 2023-01-20 | New York, NY, US | [Blockchain Center](https://www.meetup.com/blockchaincenter/)
    * [**Rust Tuesdays: Near Workspaces**](https://www.meetup.com/blockchaincenter/events/291016830/)
* 2023-01-26 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Building a Rust Playground with WASM and Lane and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsyfccbjc/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/zpd1qo/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Common arguments against Rust's safety guarantees:
>
> * The library you're binding to can have a segfault in it.
> * RAM can physically fail, causing dangling pointers.
> * The computer the Rust program is running on can be hit by a meteorite.
> * Alan Turing can come back from the dead and tell everyone that he actually made up computer science and none of it is real, thus invalidating every program ever made, including all Rust programs.

– [Ironmask on the phoronix forums](https://www.phoronix.com/forums/forum/phoronix/latest-phoronix-articles/1367544-google-to-allow-rust-code-in-the-chromium-browser?p=1367778#post1367778)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1355) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/10fvmlt/this_week_in_rust_478/)</small>
