Title: This Week in Rust 451
Number: 451
Date: 2022-07-13
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
* [Announcing Rustup 1.25.0](https://blog.rust-lang.org/2022/07/11/Rustup-1.25.0.html)
* [Announcing Rustup 1.25.1](https://blog.rust-lang.org/2022/07/12/Rustup-1.25.1.html)
* [Changes in the Core Team](https://blog.rust-lang.org/2022/07/12/changes-in-the-core-team.html)

### Newsletters
* [This Month in Rust OSDev: June 2022](https://rust-osdev.com/this-month/2022-06/)
* [This Month in Rust GameDev #35 - June 2022](https://gamedev.rs/news/035/)

### Project/Tooling Updates
* [Tauri 1.0 Release](https://tauri.app/blog/tauri_1_0/)
* [Lyon 1.0](https://nical.github.io/posts/lyon-1-0.html)
* [rust-analyzer changelog #137](https://rust-analyzer.github.io/thisweek/2022/07/11/changelog-137.html)
* [IntelliJ Rust Changelog #174](https://intellij-rust.github.io/2022/07/11/changelog-174.html)
* [rustc_codegen_gcc: Progress Report #13](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-13)
* [Accouncing Fornjot 0.7 (code-first CAD in Rust)](https://www.fornjot.app/blog/fornjot-0.7/)
* [Announcing Experimental Rust Filesystem and Path Support for Xous PDDB!](https://xobs.io/experimental-rust-filesystem-and-path-support-for-xous/)
* [This week in Fluvio #38: The programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0038/)
* [HexoSynth (modular synthesizer) - Devlog #4 - The Tracker Sequencer is Back](https://m8geil.de/posts/hexosynth-4/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-07-11.html)
* [Black Hat Rust updated for Rust 1.62 (and announcing the online course)](https://kerkour.com/black-hat-rust-update-2022-56)
* [This week in Databend #50: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-07-13-databend-weekly/)

### Observations/Thoughts
* [Announcement: Rust-Edu](https://rust-edu.org/news/announcement/)
* [A Pleasing Symmetry in Rust — Sympolymathesy, by Chris Krycho](https://v5.chriskrycho.com/journal/pleasing-symmetry-in-rust/)
* [Unsafe Syntax](https://blog.yoshuawuyts.com/unsafe-syntax/)
* [Almost Rules](https://matklad.github.io/2022/07/10/almost-rules.html)
* [Pitfalls of fallible Iterators](https://swatinem.de/blog/fallible-iterators/)
* [Minimalist Guide to Poem](https://tech.marksblogg.com/poem-rust-web-framework.html)
* [audio] [High Assurance Rust with Tiemoko Ballo](https://rustacean-station.org/episode/tiemoko-ballo/)
* [ZH] [audio] [004. 与迟先生聊 Rust 与数据库系统](https://rusttalk.github.io/podcast/004/)

### Rust Walkthroughs
* [Procedural macros under the hood: Part II | The IntelliJ Rust Blog](https://blog.jetbrains.com/rust/2022/07/07/procedural-macros-under-the-hood-part-ii/)
* [Bare-Metal Rust on ESP32: A Brief Overview](https://beta7.io/posts/bare-metal-rust-on-esp32/)
* [A Rust app in a Cloudflare Worker](https://logankeenan.com/posts/running-a-rust-server-in-a-cloudflare-worker/)
* [Runtime errors: Come again? Rust macros to the rescue](https://tech.nextroll.com/blog/dev/2022/06/21/rust-lua.html)
* [Using the Kani Rust Verifier on a Firecracker Example](https://model-checking.github.io/kani-verifier-blog/2022/07/13/using-the-kani-rust-verifier-on-a-firecracker-example.html)
* [video] [How AWS is building the Rust SDK and how you can use it today - Zelda Hessler](https://www.youtube.com/watch?v=N0XMjokwTIM)
* [video] [Constant fun (const fn) with Rust - Rainer Stropek](https://www.youtube.com/watch?v=Vw8BFScm0K0)

### Research
* [Flux: Liquid Types for Rust](https://arxiv.org/abs/2207.04034)

### Miscellaneous
* [GCC Rust Approved By Steering Committee, Likely To Land For GCC 13](https://www.phoronix.com/scan.php?page=news_item&px=GCC-Rust-SC-Approved)

## Crate of the Week

This week's crate is [shame](https://github.com/RayMarch/shame), a shader EDSL for writing render and compute pipelines in rust.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/1077) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

363 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-07-04..2022-07-11

* [implement support for DWARF version 5](https://github.com/rust-lang/rust/pull/98350)
* [sess: stabilize `-Zterminal-width` as `--diagnostic-width`](https://github.com/rust-lang/rust/pull/95635)
* [macros: `LintDiagnostic` derive](https://github.com/rust-lang/rust/pull/98884)
* [proc_macro: fix `expand_expr` expansion of `bool` literals](https://github.com/rust-lang/rust/pull/98463)
* [shorten `def_span` of closures to just their header](https://github.com/rust-lang/rust/pull/98482)
* [improve spans for specialization error](https://github.com/rust-lang/rust/pull/98782)
* [do not mention private types from other crates as impl candidates](https://github.com/rust-lang/rust/pull/99091)
* [adjust dangling-int-ptr error message](https://github.com/rust-lang/rust/pull/98860)
* [highlight conflicting param-env candidates](https://github.com/rust-lang/rust/pull/98794)
* [suggest `#[derive(Default)]` to enums with `#[default]`](https://github.com/rust-lang/rust/pull/98873)
* [suggest using block for `extern "abi" fn` with no body](https://github.com/rust-lang/rust/pull/98827)
* [suggest adding a derive for `#[default]` applied to variants](https://github.com/rust-lang/rust/pull/99002)
* [fix "wrap closure in parenthesis" suggestion for `async` closure](https://github.com/rust-lang/rust/pull/98879)
* [don't succeed `evaluate_obligation` query if new opaque types were registered](https://github.com/rust-lang/rust/pull/98614)
* [lints: mostly translatable diagnostics](https://github.com/rust-lang/rust/pull/98624)
* [deny float const params even when `adt_const_params` is enabled](https://github.com/rust-lang/rust/pull/98907)
* [allow arithmetic and certain bitwise ops on `AtomicPtr`](https://github.com/rust-lang/rust/pull/96935)
* [more `need_type_info` improvements](https://github.com/rust-lang/rust/pull/98761)
* [miri: handle `Box` with allocators](https://github.com/rust-lang/miri/pull/2323)
* [improve soundness of `rustc_arena`](https://github.com/rust-lang/rust/pull/97711)
* [fix `ProjectionElem` validation](https://github.com/rust-lang/rust/pull/96856)
* [compilation speed optimization for `pest-2.1.3`](https://github.com/rust-lang/rust/pull/98654)
* [use less string interning](https://github.com/rust-lang/rust/pull/98638)
* [more derive output improvements](https://github.com/rust-lang/rust/pull/98758)
* [implement `SourceMap::is_span_accessible`](https://github.com/rust-lang/rust/pull/99140)
* [use a bitset instead of a hash map in HIR ID validator](https://github.com/rust-lang/rust/pull/98841)
* [miscellaneous inlining improvements](https://github.com/rust-lang/rust/pull/99028)
* [partially stabilize `const_slice_from_raw_parts`](https://github.com/rust-lang/rust/pull/97522)
* [return a `FxIndexSet` in `is_late_bound` query](https://github.com/rust-lang/rust/pull/98959)
* [split `TypeVisitable` from `TypeFoldable`](https://github.com/rust-lang/rust/pull/98206)
* [implement `ExitCodeExt` for Windows](https://github.com/rust-lang/rust/pull/97917)
* [implement `FusedIterator` for `std::net::`(`Into`)`Incoming`](https://github.com/rust-lang/rust/pull/97300)
* [Windows: fallback for overlapped I/O](https://github.com/rust-lang/rust/pull/98950)
* [stabilize `into_future`](https://github.com/rust-lang/rust/pull/98718)
* [futures: add `push_front` and `push_back` to `FuturesOrdered`](https://github.com/rust-lang/futures-rs/pull/2591)
* [regex: add `ExactSizeIterator` to `SubCaptureMatches`](https://github.com/rust-lang/regex/pull/857)
* [cargo: fix corrupted git checkout recovery](https://github.com/rust-lang/cargo/pull/10829)
* [cargo: fix publishing to crates.io with `-Z sparse-registry`](https://github.com/rust-lang/cargo/pull/10831)
* [cargo: add a cache for discovered workspace roots](https://github.com/rust-lang/cargo/pull/10776)
* [rustdoc: add more semantic information to impl IDs](https://github.com/rust-lang/rust/pull/98939)
* [rustdoc: filter `'_` lifetimes from `ty::Generics`](https://github.com/rust-lang/rust/pull/98911)
* [clippy: allow `let_unit_value` in more cases](https://github.com/rust-lang/rust-clippy/pull/9056)
* [clippy: extend `unnecessary_lazy_eval` to cover `bool::then` -> `bool::then_some`](https://github.com/rust-lang/rust-clippy/pull/9099)
* [clippy: fix ICE in `sugg::DerefDelegate` with (named) closures](https://github.com/rust-lang/rust-clippy/pull/9120)
* [clippy: fix `needless_borrow` changing called trait impls on method receivers](https://github.com/rust-lang/rust-clippy/pull/9096)
* [clippy: fix `undocumented_unsafe_blocks` in closures](https://github.com/rust-lang/rust-clippy/pull/9117)
* [clippy: fix span for `or_fun_call`](https://github.com/rust-lang/rust-clippy/pull/9144)
* [clippy: ignore `into_iter` in `significant_drop_in_scrutinee`](https://github.com/rust-lang/rust-clippy/pull/9140)
* [clippy: lint `shadow_*` lints in anon const blocks](https://github.com/rust-lang/rust-clippy/pull/9124)
* [clippy: lint simple expressions in `manual_filter_map`, `manual_find_map`](https://github.com/rust-lang/rust-clippy/pull/8958)
* [clippy: maybe trait bound on type repetition](https://github.com/rust-lang/rust-clippy/pull/9132)
* [rust-analyzer: complete type param/associated type in trait generic arg per arg index](https://github.com/rust-lang/rust-analyzer/pull/12695)
* [rust-analyzer: implement `ignore`  and `index` metavar expression](https://github.com/rust-lang/rust-analyzer/pull/12745)
* [rust-analyzer: extract Function misses locals used in closures](https://github.com/rust-lang/rust-analyzer/pull/12706)
* [rust-analyzer: extract function from trait impl](https://github.com/rust-lang/rust-analyzer/pull/12676)
* [rust-analyzer: improve suggested names for extracted variables](https://github.com/rust-lang/rust-analyzer/pull/12727)
* [rust-analyzer: use `SmallVec` to slightly shrink `ModPath` size](https://github.com/rust-lang/rust-analyzer/pull/12704)

### Rust Compiler Performance Triage

A fairly noisy week (many entries below dropped and untagged as regressions),
largely driven by tt-muncher and html5ever. Our sensitivity assessment currently
takes roughly a week since new noise starts to learn the noise level, so it can
take some time for oscillations to stop reporting somewhat spurious results.

Otherwise, this week had a number of solid improvements, and was overall
positive, with improvements across many benchmarks.

Triage done by **@simulacrum**.
Revision range: [880646c..b3f4c3](https://perf.rust-lang.org/?start=880646ca9c6dc21e04efe2f1940369a45b71ff2d&end=b3f4c3119957aa0a250cab08ab586b7a9a680ef1&absolute=false&stat=instructions%3Au)

3 Regressions, 6 Improvements, 3 Mixed; 3 of them in rollups
53 artifact comparisons made in total

For details, see the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-07-12.md).

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

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Stabilize Windows `FileTypeExt` with is_`symlink_dir` and is_`symlink_file`](https://github.com/rust-lang/rust/pull/98583)
* [disposition: merge?] [Implement `TryFrom<&OsStr>` for `&str`](https://github.com/rust-lang/rust/issues/99031)
* [disposition: merge] [Tracking Issue for core::task::ready! macro](https://github.com/rust-lang/rust/issues/70922)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: resolve crates.io source replacement ambiguity](https://github.com/rust-lang/rfcs/pull/3289)

## Upcoming Events

Rusty Events between 2022-07-13 - 2022-08-10 🦀

### Virtual

* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-13 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/rFzwUjh5YT1pVci6A)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-14 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust July 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/287024976/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)
* 2022-07-20 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Discussion - Building a Multithreaded Web Server**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121101/)
* 2022-07-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydckbcc/)
* 2022-07-26 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydckbjc/)
* 2022-07-27 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Using Rust to read the Little Schemer**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121637/)
* 2022-07-29 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286993342/)
* 2022-07-31 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)
* 2022-08-02 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydclbdb/)
* 2022-08-03 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)

### Europe

* 2022-07-20 | Wrocław, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**Rust Warsaw #5**](https://www.meetup.com/rust-warsaw/events/287093615/)
* 2022-07-21 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #27**](https://www.meetup.com/rust-wroclaw/events/287023750/)
* 2022-06-22 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/287019877/)

### North America

* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-13 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/287009519/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)
* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)

### Oceania

* 2022-07-19 | Phillip, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**Canberra July Meetup**](https://www.meetup.com/rust-canberra/events/286884699/)
* 2022-07-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**July Meetup**](https://www.meetup.com/rust-brisbane/events/286889804/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has removed the jobs posting section. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/voglel/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Learning Rust has taught me something - "There are really no problems, just adventure and opportunities"

– [Adeoye Adefemi on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1263)

Thanks to [Adeoye Adefemi and Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1266) for the suggestion as well as [Christopher Durham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1267) for the leniency.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/vyntoj/this_week_in_rust_451/)</small>
