Title: This Week in Rust 446
Number: 446
Date: 2022-06-08
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

* [Rust Compiler June 2022 Steering Cycle](https://blog.rust-lang.org/inside-rust/2022/06/03/jun-steering-cycle.html)

### Newsletters

* [This Month in Rust GameDev #34 - May 2022](https://gamedev.rs/news/034/)

### Project/Tooling Updates

* [rust-analyzer changelog #132](https://rust-analyzer.github.io/thisweek/2022/06/06/changelog-132.html)
* [This week in Databend #44: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-06-01-databend-weekly/)
* [This week in Databend #45: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-06-08-databend-weekly/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-06-06.html)
* [This week in Fluvio #35: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0035/)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W22](https://www.fornjot.app/blog/weekly-dev-log/2022-w22/)
* [GCC Rust Monthly Report #17 May 2022](https://thephilbert.io/2022/06/06/gcc-rust-monthly-report-17-may-2022/)
* [Zellij 0.30.0: use your $EDITOR to search and save your scrollback](https://zellij.dev/news/edit-scrollback-compact/)

### Observations/Thoughts

* [Stacked Futures and why they are impossible](https://conradludgate.com/posts/async-stack)
* [Rust Is Hard, Or: The Misery of Mainstream Programming](https://hirrolot.github.io/posts/rust-is-hard-or-the-misery-of-mainstream-programming.html)
* [Trivia About Rust Types: An (Authorized) Transcription of Jon Gjengset’s Twitter Thread](https://www.thecodedmessage.com/posts/trivia-rust-types/)
* [Hyper vs Rocket - Low Level vs Batteries included](https://www.shuttle.rs/blog/2022/06/01/hyper-vs-rocket)
* [(async) Rust doesn't have to be hard](https://itsallaboutthebit.com/async-simple/)
* [Rust language’s explosive popularity comes with challenges](https://thestack.technology/rust-language-explosive-growth-challenges-rust-governance/)
* [audio] [egui with Emil Ernerfeldt](https://rustacean-station.org/episode/emil-ernerfeldt/)

### Rust Walkthroughs

* [Introduction to Rust generics (2/2): Trait Objects - Static vs Dynamic dispatch](https://kerkour.com/rust-generics-trait-objects)
* [Beginner-Intermediate Errror Handling in Rust](https://desmondwillowbrook.github.io/blog/rust-error-handling/)
* [Macro Patterns - A match made in heaven](https://conradludgate.com/posts/macros_match)
* [Using the Kani Rust Verifier on a Rust Standard Library CVE](https://model-checking.github.io/kani-verifier-blog/2022/06/01/using-the-kani-rust-verifier-on-a-rust-standard-library-cve.html)
* [The entrait pattern](https://audunhalland.github.io/blog/entrait-pattern/)
* [Web Scraping with Rust](https://www.scrapingbee.com/blog/web-scraping-rust/)
* [Logan Keenan - A Rust Server App Compiled to WASM as an SPA](https://logankeenan.com/posts/a-rust-server-app-compiled-to-wasm-as-an-spa/)
* [video] [Async I/O in Depth: Creating a High Performance HTTP Web Server in Rust (Part 1)](https://www.youtube.com/watch?v=fdxhcDne2Ww)
* [video] [Stream Ingestion with Kafka & Kinesis](https://www.youtube.com/watch?v=05pS-m6iuQ4)
* [video] [is salvo really the simplest rust web framework?](https://www.youtube.com/watch?v=tf9x97eTcpk)
* [video] [Rust wgpu Graphics Programming Tutorial](https://www.youtube.com/playlist?list=PL_UrKDEhALdJS0VrLPn7dqC5A4W1vCAUT)

### Research

* [End-to-End Security for Distributed Event-Driven Enclave Applications on Heterogeneous TEEs](https://arxiv.org/abs/2206.01041)

### Miscellaneous

## Crate of the Week

This week's crate is [osmpbf](https://lib.rs/crates/osmpbf) an OpenStreetMap pbf-file reader.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1065) for the suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [mirrord - Handle Job/Pod creation errors in mirrord-layer](https://github.com/metalbear-co/mirrord/issues/112)
* [mirrord - mirrord-layer panics without any relevant info if the agent exits](https://github.com/metalbear-co/mirrord/issues/110)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

385 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-05-30..2022-06-06

* [support the `#[expect]` attribute on fn parameters](https://github.com/rust-lang/rust/pull/97715) (RFC #[2383](https://rust-lang.github.io/rfcs/2383-lint-reasons.html))
* [tighten spans for bad fields in struct deriving `Copy`](https://github.com/rust-lang/rust/pull/97722)
* [add a suggestion to replace parentheses with angle brackets on associated trait constraint](https://github.com/rust-lang/rust/pull/97656)
* [diagnose anonymous lifetimes errors more uniformly between async and regular fns](https://github.com/rust-lang/rust/pull/97023)
* [do not ICE when failing to normalize during inlining](https://github.com/rust-lang/rust/pull/97696)
* [fail gracefully when encountering an HRTB in APIT](https://github.com/rust-lang/rust/pull/97683)
* [fix reachability analysis for const methods](https://github.com/rust-lang/rust/pull/97716)
* [fix wrong suggestion for adding where clauses](https://github.com/rust-lang/rust/pull/97640)
* [suggest `extern crate foo` when failing to resolve `use foo`](https://github.com/rust-lang/rust/pull/97264)
* [suggest adding `{}` for `'label: non_block_expr`](https://github.com/rust-lang/rust/pull/97759)
* [suggest adding a semicolon to a closure without block](https://github.com/rust-lang/rust/pull/97371)
* [suggest `?` when method is missing on `Result<T, _>` but found on `T`](https://github.com/rust-lang/rust/pull/96271)
* [use strict provenance APIs](https://github.com/rust-lang/rust/pull/97764)
* [provide more context when denying invalid type params](https://github.com/rust-lang/rust/pull/97471)
* [optimize the diagnostic generation for `extern unsafe`](https://github.com/rust-lang/rust/pull/97172)
* [improve soundness of `rustc_data_structures`](https://github.com/rust-lang/rust/pull/97707)
* [on E0204 suggest missing type param bounds](https://github.com/rust-lang/rust/pull/97664)
* [interpret: better control over whether we read data with provenance](https://github.com/rust-lang/rust/pull/97684)
* [miri: add support for `_COARSE` clocks](https://github.com/rust-lang/miri/pull/2165)
* [miri: save a created event for zero-size reborrows](https://github.com/rust-lang/miri/pull/2145)
* [miri: weak memory emulation using store buffers](https://github.com/rust-lang/miri/pull/1963)
* [miri: adjust for better provenance control](https://github.com/rust-lang/miri/pull/2183)
* [miri: make Miri's scheduler proper round-robin](https://github.com/rust-lang/miri/pull/2197)
* [iterate over `maybe_unused_trait_imports` when checking dead trait imports](https://github.com/rust-lang/rust/pull/97609)
* [inline `bridge::Buffer` methods](https://github.com/rust-lang/rust/pull/97604)
* [lazify `SourceFile::lines`](https://github.com/rust-lang/rust/pull/97575)
* [lazily allocate and initialize pthread locks](https://github.com/rust-lang/rust/pull/97647)
* [add windows application manifest to rustc-main](https://github.com/rust-lang/rust/pull/96737)
* [Implement faster memcmp for x86_64](https://github.com/rust-lang/compiler-builtins/pull/467)
* [inline `Vec`'s `Deref/DerefMut`](https://github.com/rust-lang/rust/pull/97553)
* [add unicode fast path to `is_printable`](https://github.com/rust-lang/rust/pull/97526)
* [additional `*mut [T]` methods](https://github.com/rust-lang/rust/pull/95594)
* [expose `get_many_mut` and `get_many_unchecked_mut` to `HashMap`](https://github.com/rust-lang/rust/pull/94647)
* [implement `[OsStr]::join`](https://github.com/rust-lang/rust/pull/96881) &nbsp;
* [make `from{,_mut}_ptr_range` const](https://github.com/rust-lang/rust/pull/97419)
* [stabilize `box_into_pin`](https://github.com/rust-lang/rust/pull/97397)
* [stabilize `toowned_clone_into`](https://github.com/rust-lang/rust/pull/97455)
* [stabilize {slice, array}`::from_ref`](https://github.com/rust-lang/rust/pull/97366)
* [hashbrown: add shortcircuit in iteration if we yielded all elements](https://github.com/rust-lang/hashbrown/pull/338)
* [futures: fix orderings in `LocalPool` waker](https://github.com/rust-lang/futures-rs/pull/2608)
* [futures: switch to `FuturesOrdered` dynamically in `try_join_all`](https://github.com/rust-lang/futures-rs/pull/2556)
* [futures: stream: fix `Chunk` adapters size hints](https://github.com/rust-lang/futures-rs/pull/2611)
* [cargo: emit warning upon encountering multiple packages with the same name](https://github.com/rust-lang/cargo/pull/10701)
* [cargo: expose rust-version through env var](https://github.com/rust-lang/cargo/pull/10713)
* [rustdoc: improve calculation of "Impls on Foreign Types"](https://github.com/rust-lang/rust/pull/97613)
* [rustdoc: also index impl trait and raw pointers](https://github.com/rust-lang/rust/pull/97592)
* [rustdoc: avoid including impl blocks with filled-in generics](https://github.com/rust-lang/rust/pull/97130)
* [clippy: clippy book](https://github.com/rust-lang/rust-clippy/pull/7359)
* [clippy: add lint `almost_complete_letter_range`](https://github.com/rust-lang/rust-clippy/pull/8918)
* [clippy: add lint `needless_parens_on_range_literals`](https://github.com/rust-lang/rust-clippy/pull/8933)
* [clippy: add lint `mismatching_type_param_order`](https://github.com/rust-lang/rust-clippy/pull/8831)
* [clippy: new lint `swap_ptr_to_ref`](https://github.com/rust-lang/rust-clippy/pull/8916)
* [clippy: new lint: `borrow_deref_ref`](https://github.com/rust-lang/rust-clippy/pull/7930)
* [clippy: add `as_underscore` lint](https://github.com/rust-lang/rust-clippy/pull/8934)
* [clippy: improve `for_loops_over_fallibles` to detect the usage of iter, iter_mut and into_iterator](https://github.com/rust-lang/rust-clippy/pull/8941)
* [clippy: fix `use_self` false negative with on struct and tuple struct patterns](https://github.com/rust-lang/rust-clippy/pull/8899)
* [clippy: fix `manual_range_contains` false negative with chains of `&&` and `||`](https://github.com/rust-lang/rust-clippy/pull/8884)
* [clippy: when setting suggestion for significant_drop_in_scrutinee, add suggestion for MoveAndClone for non-ref](https://github.com/rust-lang/rust-clippy/pull/8902)
* [clippy: `needless_return` checks for macro expr in return stmts](https://github.com/rust-lang/rust-clippy/pull/8932)
* [clippy: don't lint `useless_transmute` on types with erased regions](https://github.com/rust-lang/rust-clippy/pull/8564)
* [clippy: `cast_abs_to_unsigned`: do not remove cast if it's required](https://github.com/rust-lang/rust-clippy/pull/8876)
* [clippy: remove `large_enum_variant` suggestion for `Copy` types](https://github.com/rust-lang/rust-clippy/pull/8906)
* [clippy: fix `manual_find_map` and `manual_filter_map`: check clone method](https://github.com/rust-lang/rust-clippy/pull/8930)
* [clippy: fix ICE in shadow lints](https://github.com/rust-lang/rust-clippy/pull/8913)
* [clippy: needless_late_init: fix ICE when all branches return the never type](https://github.com/rust-lang/rust-clippy/pull/8912)
* [clippy: set correct `ParamEnv` for `derive_partial_eq_without_eq`](https://github.com/rust-lang/rust-clippy/pull/8869)
* [rust-analyzer: increase worker thread stack and name them](https://github.com/rust-lang/rust-analyzer/pull/12466)
* [rust-analyzer: move trait_impl completion analysis into CompletionContext](https://github.com/rust-lang/rust-analyzer/pull/12461)
* [rust-analyzer: order auto-imports by relevance](https://github.com/rust-lang/rust-analyzer/pull/12333)
* [rust-analyzer: cleaer status bar bg color / command  when server status returns to OK](https://github.com/rust-lang/rust-analyzer/pull/12435)
* [rust-analyzer: support `$$` in macros](https://github.com/rust-lang/rust-analyzer/pull/12451)
* [rust-analyzer: `Merge imports` assist can merge multiple selected imports](https://github.com/rust-lang/rust-analyzer/pull/12452)
* [rust-analyzer: don't remove diagnostic with empty message](https://github.com/rust-lang/rust-analyzer/pull/12440)
* [rust-analyzer: resolving import panics and improve import resolution](https://github.com/rust-lang/rust-analyzer/pull/12347)
* [rust-analyzer: type-mismatch when using equals w/ a trait bound](https://github.com/rust-lang/rust-analyzer/pull/12336)
* [rust-analyzer: cleanup output channels when restarting server](https://github.com/rust-lang/rust-analyzer/pull/12470)
* [rust-analyzer: fix VSCode config patching incorrectly patching some configs](https://github.com/rust-lang/rust-analyzer/pull/12427)
* [rust-analyzer: fix completions disappearing when typing two keys in quick succession](https://github.com/rust-lang/rust-analyzer/pull/12431)
* [rust-analyzer: fix match to if let assist for wildcard pats](https://github.com/rust-lang/rust-analyzer/pull/12467)
* [rust-analyzer: fix trait impl completions using wrong insert position](https://github.com/rust-lang/rust-analyzer/pull/12475)
* [rust-analyzer: restart the server instead of reloading the window when config changes](https://github.com/rust-lang/rust-analyzer/pull/12471)
* [rust-analyzer: float display impl](https://github.com/rust-lang/rust-analyzer/pull/12425)
* [rust-analyzer: parsing of `?` opt-out trait bounds](https://github.com/rust-lang/rust-analyzer/pull/12444)

### Rust Compiler Performance Triage

A busy week in compiler performance, but fortunately improvements outweighed regressions. The biggest improvements came from @nnethercote's work on making the decoding of `SourceFile::lines` lazy which significantly cuts the costs of decoding crate metadata. The biggest regressions came from the removal of json handling in `rustc_serialize` which has been a multi-month effort to improve the maintainability of json (de-)serialization in the compiler.

Triage done by **@rylev**.
Revision range: [0a43923a..bb55bd](https://perf.rust-lang.org/?start=0a43923a86c3b8f11d005884871b152f59b746f7&end=bb55bd449e65e611da928560d948982d73e50027&absolute=false&stat=instructions%3Au)

**Summary**:

|            | mean | max | count |
|:----------:|:----:|:---:|:-----:|
| Regressions 😿 <br /> (primary) | 0.5% | 3.2% | 36    |
| Regressions 😿 <br /> (secondary) | 0.3% | 0.9% | 15    |
| Improvements 🎉 <br /> (primary) | -1.3% | -15.1% | 124   |
| Improvements 🎉 <br /> (secondary) | -2.7% | -13.5% | 182   |
| All 😿🎉 (primary) | -0.9% | -15.1% | 160   |


2 Regression, 6 Improvements, 5 Mixed; 4 of them in rollups
48 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-06-07.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [RFC: 'C-unwind' ABI](https://github.com/rust-lang/rfcs/pull/2945)
    * [Testing instructions](https://github.com/rust-lang/rfcs/pull/2945#issuecomment-1134003542)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Rolling co-lead roles for T-compiler](https://github.com/rust-lang/rfcs/pull/3262)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Make {Mutex, Condvar, RwLock}::new() const. ](https://github.com/rust-lang/rust/pull/97791)
* [disposition: merge] [Make std::mem::needs_drop accept ?Sized](https://github.com/rust-lang/rust/pull/97675)
* [disposition: merge] [Tracking Issue for scoped threads](https://github.com/rust-lang/rust/issues/93203)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-06-08 - 2022-07-06 🦀

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

**Spire Global**

* [Software Engineers + Engineering Managers (Glasgow, UK, Luxembourg - Relocation Available)](https://spire.com/careers/job-openings/job/?gh_jid=4226230)

**Meilisearch**

* [Senior Software Engineer (Remote, European timezone)](https://jobs.lever.co/meili/5b9f780a-0c80-41a7-aba0-bd86834e6823)

**Stockly**

* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)

**Nikulipe**

* [Software Engineer – Full Stack Rust (Vilnius, Vilniaus, LT)](https://www.linkedin.com/jobs/view/3107060725/)

**GMEX Group**

* [Senior Blockchain MPC Developer / Blockchain MPC Developer (Various, Remote)](https://www.linkedin.com/jobs/view/3099462018/)

**Enso**

* [Senior Rust IDE Developer (Kraków, PL or Remote)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-ide-developer.md)
* [Senior Cloud Rust Engineer (Kraków, PL or Remote)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-cloud-developer.md)

**Kraken**

* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I wrote a bespoke time-series database in Rust a few years ago, and it has had exactly one issue since I stood it up in production, and that was due to pessimistic filesystem access patterns, rather than the language. This thing is handling hundreds of thousands of inserts per second, and it's even threaded.
>
> Given that I've been programming professionally for over a decade in Python, Perl, Ruby, C, C++, Javascript, Java, and Rust, I'll pick Rust absolutely any time that I want something running that I won't get called at 3 AM to fix. It probably took me 5 times as long to write it as if I did it in Go or Python, but I guarantee it's saved me 10 times as much time I would have otherwise spent triaging, debugging, and running disaster recovery.

– [Taywee on hacker news](https://news.ycombinator.com/item?id=31616966)

Thanks to [Erich Gubler](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1250) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/v88hmw/this_week_in_rust_446/)</small>
