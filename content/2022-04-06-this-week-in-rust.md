Title: This Week in Rust 437
Number: 437
Date: 2022-04-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official
* [Changes at the Cargo Team](https://blog.rust-lang.org/inside-rust/2022/03/31/cargo-team-changes.html)
* [Rust Lang Roadmap for 2024](https://blog.rust-lang.org/inside-rust/2022/04/04/lang-roadmap-2024.html)

### Foundation
* [The Rust Foundation Community Grants Program 2022 Opens for Applications](https://foundation.rust-lang.org/news/2022-03-31-cgp-is-open-announcement/)

### Newsletters
* [Rust Nigeria Newsletter #4](https://rustnigeria.curated.co/issues/4)

### Project/Tooling Updates
* [What's new in axum 0.5](https://tokio.rs/blog/2022-03-whats-new-in-axum-0-5)
* [rust-analyzer changelog #123](https://rust-analyzer.github.io/thisweek/2022/04/04/changelog-123.html)
* [Slint (GUI crate) weekly update](https://slint-ui.com/thisweek/2022-04-05.html)
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W13](https://www.fornjot.app/blog/weekly-dev-log/2022-w13/)
* [Introducing StarfishQL - visualizing the dependency network on crates.io](https://www.sea-ql.org/SeaORM/blog/2022-04-04-introducing-starfish-ql/)
* [This week in Fluvio #27: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0027/)
* [Rust on Espressif chips - 04-04-2022](https://mabez.dev/blog/posts/esp-rust-04-04-2022/)
* [GCC Rust Monthly Report #15 March 2022](https://thephilbert.io/2022/04/04/gcc-rust-monthly-report-15-march-2022/)
* [Progress Report #10: rustc_codegen_gcc can now bootstrap rustc!](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-10)
* [youki 0.0.3 has been released, with WASM support](https://github.com/containers/youki/releases/tag/v0.0.3)
* [Coppers - A test harness that measures the energy usage of your Rust projects](https://github.com/ThijsRay/coppers)

### Observations/Thoughts
* [The Tower of Weakenings: Memory Models For Everyone - Faultlore](https://gankra.github.io/blah/tower-of-weakenings/)
* [hyper 1.0 roadmap](https://seanmonstar.com/post/680802159018803200/hyper-10-roadmap)
* [Why Rust mutexes look like they do](https://cliffle.com/blog/rust-mutexes/)
* [Improving Python S3 Client Performance with Rust](https://joshua-robinson.medium.com/improving-python-s3-client-performance-with-rust-e9639359072f)
* [Qiskit now includes Rust for better performance](https://medium.com/qiskit/new-weve-started-using-rust-in-qiskit-for-better-performance-a3676433ca8c)
* [Why We’re Porting Our Database Drivers to Async Rust](https://thenewstack.io/why-were-porting-our-database-drivers-to-async-rust/)
* [The Chronicles of a Web3 philosopher.](https://dev.to/rustnigeria/the-chronicles-of-a-web3-philosopher-3a43)
* [audio] [Rusty Engine](https://rustacean-station.org/episode/062-nathan-stocks/)

### Rust Walkthroughs
* [Futures Nostalgia](https://fasterthanli.me/articles/futures-nostalgia)
* [A Rust web server / frontend setup like it's 2022 (with axum and yew)](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/)
* [Rust's fearless concurrency in practice](https://kerkour.com/rust-fearless-concurrency)
* [video] [Rust multi-threading code review](https://www.youtube.com/watch?v=jkHqrkcEHRc)

### Miscellaneous
* [A Computer Science Curriculum with Rust flavor](https://github.com/AbdesamedBendjeddou/Rusty-CS)
* [Tracking the JWST: Programming/Language Issues](https://arachnoid.com/tracking_the_JWST/section2.html)
* [Ferrous Systems and Espressif’s Rust Training on ESP32](https://www.espressif.com/en/news/ESP_RUST_training)
* [RUST! #[proc_macros] zine](https://github.com/lrlna/smol-zines/blob/fc925ac8168599915dd179babd858c53d6f57aab/zines/rust-proc-macros.md)

## Crate of the Week

This week's crate is [bet](https://github.com/Canop/bet), a library of binary expression trees.

Thanks to [Denys Séguret](https://users.rust-lang.org/t/crate-of-the-week/2704/1050) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

311 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-03-28..2022-04-04

* [include a header in .rlink files](https://github.com/rust-lang/rust/pull/95589)
* [make lowering pull-based](https://github.com/rust-lang/rust/pull/90204)
* [make GATs object safe under generic_associated_types_extended feature](https://github.com/rust-lang/rust/pull/94911)
* [lazy type-alias-impl-trait take two](https://github.com/rust-lang/rust/pull/94081)
* [allow large Size again](https://github.com/rust-lang/rust/pull/95456)
* [reduce max hash in raw strings from `u16` to `u8`](https://github.com/rust-lang/rust/pull/95251)
* [a new matcher representation for use in `parse_tt`](https://github.com/rust-lang/rust/pull/95555)
* [yet more `parse_tt` improvements](https://github.com/rust-lang/rust/pull/95425)
* [simplify `MatcherPos` some more](https://github.com/rust-lang/rust/pull/95509)
* [reduce unnecessary escaping in `proc_macro::Literal::character`/`string`](https://github.com/rust-lang/rust/pull/95343)
* [remember mutability in `DefKind::Static`](https://github.com/rust-lang/rust/pull/95436)
* [ast\_lowering: stop wrapping `ident` matchers into groups](https://github.com/rust-lang/rust/pull/95571)
* [convert more `DefId`s to `LocalDefId`](https://github.com/rust-lang/rust/pull/95560)
* [miri: add `-Zmiri-strict-provenance`](https://github.com/rust-lang/miri/pull/2045)
* [add error message suggestion for missing `noreturn` in naked function](https://github.com/rust-lang/rust/pull/95544)
* [add note to the move size diagnostic](https://github.com/rust-lang/rust/pull/95478)
* [add suggestion to borrow `Fn` and `FnMut` params/opaque/closures instead of move](https://github.com/rust-lang/rust/pull/95257)
* [add the `generic_associated_types_extended` feature](https://github.com/rust-lang/rust/pull/94869)
* [better suggestions for `Fn`-family trait selection errors](https://github.com/rust-lang/rust/pull/95260)
* [don't ICE when opaque types get their hidden type constrained again](https://github.com/rust-lang/rust/pull/95471)
* [don't emit non-asm contents error for naked function composed of errors](https://github.com/rust-lang/rust/pull/95553)
* [fix double drop of allocator in `IntoIter  impl of Vec`](https://github.com/rust-lang/rust/pull/95298)
* [improve method name suggestions](https://github.com/rust-lang/rust/pull/95119)
* [specialize infinite-type "insert some indirection" suggestion for Option](https://github.com/rust-lang/rust/pull/91416)
* [suggest `i += 1` when we see `i++` or `++i`](https://github.com/rust-lang/rust/pull/88672)
* [suggest borrowing when trying to coerce unsized type into `dyn Trait`](https://github.com/rust-lang/rust/pull/95609)
* [suggest wrapping patterns in enum variants](https://github.com/rust-lang/rust/pull/95386)
* [suggest wrapping single-expr blocks in square brackets](https://github.com/rust-lang/rust/pull/95293)
* [codegen\_gcc: add intrinsic translation for x86 arch](https://github.com/rust-lang/rustc_codegen_gcc/pull/151)
* [codegen\_gcc: add missing vendor intrinsics](https://github.com/rust-lang/rustc_codegen_gcc/pull/150)
* [codegen\_gcc: add support for target builtins](https://github.com/rust-lang/rustc_codegen_gcc/pull/149)
* [codegen\_gcc: feature/packed struct](https://github.com/rust-lang/rustc_codegen_gcc/pull/148)
* [codegen\_gcc: feature/simd](https://github.com/rust-lang/rustc_codegen_gcc/pull/129)
* [stabilize native library modifier syntax and the `whole-archive` modifier specifically](https://github.com/rust-lang/rust/pull/93901)
* [stabilize `windows_process_extensions_raw_arg`](https://github.com/rust-lang/rust/pull/92942)
* [stabilize `Termination` and `ExitCode`](https://github.com/rust-lang/rust/pull/93840)
* [stabilize feature `vec_retain_mut` on `Vec` and `VecDeque`](https://github.com/rust-lang/rust/pull/95491)
* [stabilize `thread::is_finished`](https://github.com/rust-lang/rust/pull/95130)
* [stabilize `total_cmp`](https://github.com/rust-lang/rust/pull/95431)
* [add `SyncUnsafeCell`](https://github.com/rust-lang/rust/pull/95438)
* [strict Provenance MVP](https://github.com/rust-lang/rust/pull/95241)
* [implement provenance preserving methods on `NonNull`](https://github.com/rust-lang/rust/pull/95556)
* [async: give predictable name to binding generated from .await expressions](https://github.com/rust-lang/rust/pull/95011)
* [portable-simd: move comparisons to traits](https://github.com/rust-lang/portable-simd/pull/265)
* [rustdoc: fix invalid DOM generation](https://github.com/rust-lang/rust/pull/95568)
* [rustdoc: fix multiline attributes handling in doctests](https://github.com/rust-lang/rust/pull/95590)
* [rustdoc: fix rustdoc attribute display](https://github.com/rust-lang/rust/pull/95613)
* [rustdoc: only show associated consts from inherent impls in sidebar](https://github.com/rust-lang/rust/pull/95475)
* [rustfmt: preserve semicolon after macro call inside foreign mod](https://github.com/rust-lang/rustfmt/pull/5282)
* [rustfmt: fix struct field formatting with doc comments present](https://github.com/rust-lang/rustfmt/pull/5217)
* [clippy: add `crate_in_macro_def` lint](https://github.com/rust-lang/rust-clippy/pull/8576)
* [clippy: do not fire `panic` in a constant environment](https://github.com/rust-lang/rust-clippy/pull/8592)
* [clippy: don't lint `cast_ptr_alignment` when used for unaligned reads and writes](https://github.com/rust-lang/rust-clippy/pull/8632)
* [clippy: don't warn int-to-char transmutes in const contexts](https://github.com/rust-lang/rust-clippy/pull/8610)
* [clippy: fix ICE for `iter_overeager_cloned`](https://github.com/rust-lang/rust-clippy/pull/8602)
* [clippy: handle relative paths in module_files lints](https://github.com/rust-lang/rust-clippy/pull/8611)
* [clippy: provide suggestion context in map_unit_fn](https://github.com/rust-lang/rust-clippy/pull/8584)
* [clippy: rework `undocumented_unsafe_blocks`](https://github.com/rust-lang/rust-clippy/pull/8450)
* [clippy: add `empty_structs_with_brackets`](https://github.com/rust-lang/rust-clippy/pull/8594)
* [clippy: single_element_loop: handle arrays for Edition 2021](https://github.com/rust-lang/rust-clippy/pull/8616)
* [cargo-bisect: add support for git tags in `--start` and `--end`](https://github.com/rust-lang/cargo-bisect-rustc/pull/147)
* [rustc-perf: use a summary table for PR comments](https://github.com/rust-lang/rustc-perf/pull/1245)

### Rust Compiler Performance Triage

A somewhat quiet week with only a few improvements and regressions, but with improvements ever so slightly edging out regressions. The biggest regression was in a rollup which makes investigation difficult though it looks like its in trait resolution which impacts crates that do a lot of that such as diesel. The biggest improvement comes from work done by the performance team (more specifically @nnethercote) to improve `macro_rules` parsing which can lead to sizeable performance gains for crates using the ["token munching"](https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html) pattern in `macro_rules`.

Triage done by **@rylev**.
Revision range: [3e75146..949b98c](https://perf.rust-lang.org/?start=3e7514670db841a7f0d7656f3b13b1c8b2c11599&end=949b98cab8a186b98bf87e64374b8d0848c55271&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 1 Mixed; 2 of them in rollups
37 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-04-05.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Cargo authenticating users without sending secrets over the network](https://github.com/rust-lang/rfcs/pull/3231)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Allow using for<'a> syntax when declaring closures](https://github.com/rust-lang/rfcs/pull/3216)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [\[let_chains\] Forbid let inside parentheses](https://github.com/rust-lang/rust/pull/95008)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: multiple_crate_versions](https://github.com/rust-lang/rfcs/pull/3251)

## Upcoming Events

Rusty Events between 2022-04-06 - 2022-05-04 🦀

### Virtual

* 2022-04-06 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydcgbjb/)
* 2022-04-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcgbjb/)
* 2022-04-06 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qnxdtsydcgbjb/)
* 2022-04-07 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #12**](https://www.meetup.com/rust-noris/events/zgfnssydcgbsb/)
* 2022-04-11 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Mob Programming: Rome Tools**](https://www.meetup.com/RustPhilly/events/bgqgtsydcgbpb/)
* 2022-04-11 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly Meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydcgbqb/)
* 2022-04-12 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399977/)
* 2022-04-12 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcgbqb/)
* 2022-04-12 | Rostock, DE | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**6. Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/283819122)
* 2022-04-12 | Saarbrücken, DE | [Rust-Saar](https://www.meetup.com/Rust-Saar/)
    * [**Meetup: 20u16**](https://www.meetup.com/Rust-Saar/events/284753673/)
* 2022-04-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcgbrb/)
* 2022-04-13 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydcgbrb/)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/283790509/)
* 2022-04-14 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydcgbsb/)
* 2022-04-18 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Mob Programming: Rome Tools**](https://www.meetup.com/RustPhilly/events/bgqgtsydcgbxb/)
* 2022-04-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydcgbzb/)
* 2022-04-20 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/284644487)
* 2022-04-20 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust April 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/284705301/)
* 2022-04-20 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Movie/Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcgbbc/)
* 2022-04-21 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**The Rusty Boss--directing the world with Rust, Bluetooth, and a Raspberry Pi**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/284922970)
* 2022-04-23 | Various - EMEA | [Rustfest](https://rustfest.world/)
    * [**Rust EMEA Conference**](https://rustfest.world/news/twirf-latam-emea-announcement)
* 2022-04-26 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399979/)
* 2022-04-27 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcgbkc/)
* 2022-05-03 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/284996307/)

### Europe
* 2022-04-06 | Amsterdam, NL | [Rust Developers Amsterdam Group, Part of Rust Amsterdam Network](https://www.meetup.com/rust-amsterdam-group/) | [Alt link](https://www.meetup.com/pro/rust-amsterdam-network/)
    * [**Rust Developer Meetup: Serverless Rust and Rust on a Pi**](https://www.meetup.com/rust-amsterdam-group/events/284647946/)
* 2022-04-06 | Bristol, UK | [Rust Bristol](https://www.meetup.com/rust-bristol/)
    * [**Rust Bristol 🦀 Kickoff (1/2) - Intro & Embedded**](https://www.meetup.com/rust-bristol/events/284703797/)
* 2022-04-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/Rust-Berlin/)
    * [**Rust and Tell - an onsite event**](https://www.meetup.com/Rust-Berlin/events/284512764/) | [**Alt Link**](https://berline.rs/2022/04/12/rust-and-tell.html)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/283790509/)
* 2022-04-13 | Paris, FR | [Stockly](https://www.welcometothejungle.com/fr/companies/stockly-1)
    * [**Rust Meetup in Paris - hosted by Stockly, Qovery & Meilisearch**](https://www.eventbrite.com/e/rust-meetup-in-paris-hosted-by-stockly-qovery-meilisearch-tickets-277690869867)
* 2022-04-14 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/)
    * [**Cambridge Rust Reboot 2**](https://www.meetup.com/Cambridge-Rust-Meetup/events/284505361/)
* 2022-04-19 | Bristol, UK | [Rust Bristol](https://www.meetup.com/rust-bristol/)
    * [**Rust Bristol 🦀 Kickoff (2/2)**](https://www.meetup.com/rust-bristol/events/284704573/)

### North America
* 2022-04-13 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcgbrb/)
* 2022-04-14 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcgbsb/)
* 2022-04-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcgbzb/)
* 2022-04-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Boston Rust Meetup at Amazon**](https://www.meetup.com/BostonRust/events/284808948)

### Oceania
* 2022-04-21 | Melbourne, AUS | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
    * [**Rust Melbourne is back!**](https://www.meetup.com/Rust-Melbourne/events/284327357/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**KidsLoop**

* [Senior Audio/ Video Backend Engineer (Seoul, KR)](https://kidsloop.bamboohr.com/jobs/view.php?id=397)

**Amazon**

* [Software Development Engineer (Remote)](https://www.amazon.jobs/en/jobs/1262951/software-development-engineer)

**NXLog**

* [Rust Developer (Remote, Europe or worldwide)](https://application.nxlog.org/jobs/detail/rust-developer-39)

**Timescale**

* [Senior Rust Engineer - TimescaleDB Toolkit (Remote: UTC-5 to UTC-8)](https://www.timescale.com/careers/5920911002?gh_jid=5920911002)

**Kollider**

* [Senior Frontend Engineer - Rust (Remote)](https://careers.kollider.xyz/senior-frontend-engineer/en)
* [Junior Backend Engineer - Rust (Remote)](https://careers.kollider.xyz/junior-backend-engineer/en)

**Stockly**

* [Back-end developer - TechOps (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-rust-grpc-postgresql_paris)
* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)
* [Back-end developer - Freelance (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-freelance-rust-grpc-postgresql_paris)

**Kraken**

* [Site Reliability Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/1c6b290f-e430-430d-9b40-a258d07686b0)
* [Rust Engineer, Desktop GUI - Cryptowatch (Remote)](https://jobs.lever.co/kraken/2442ee5c-56b6-4a73-a477-8cdda2b218d5)
* [Engineering Manager - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/53def500-b146-40da-89a8-98adfd7e84e4)

**Tempus Ex**

* [Several full-time Rust positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US, and Remote)](https://tempus-ex.com/careers)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I've seen similar sentiments echoed before, elsewhere. The point it's making is the same one that's argued whenever people say you should learn LISP because it'll make you a better programmer.
>
> There's no such thing as a perfectly intuitive programming language because algorithmic thinking isn't something that comes to us intuitively. That's why the first language is always the hardest.
>
> It's helpful and mind-expanding to learn new paradigms and force yourself out of old cognitive ruts. Thus, from an "improving your ability to solve problems and function as a programmer" perspective, what makes Rust difficult is valuable because it's forcing you to learn to think about problems in new ways.
>
> That's the distinction between necessary complexity and complexity due to ill-considered design. (Similar to how, in video games, there's a difference between genuine difficulty and difficulty caused by something like a crappy control scheme.)

– [Stephan Sokolow on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1207) (in our quotes thread!)

Thanks to [Christopher Durham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1208) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/ty1asi/this_week_in_rust_437/)</small>
