Title: This Week in Rust 445
Number: 445
Date: 2022-06-01
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

* [Concluding the events of last November](https://blog.rust-lang.org/inside-rust/2022/05/26/Concluding-events-mods.html)

### Project/Tooling Updates

* [rust-analyzer changelog #131](https://rust-analyzer.github.io/thisweek/2022/05/30/changelog-131.html)
* [Helix editor 22.05 released](https://helix-editor.com/news/release-22-05-highlights/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-05-30.html)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W21](https://www.fornjot.app/blog/weekly-dev-log/2022-w21/)
* [IntelliJ Rust Changelog #171](https://intellij-rust.github.io/2022/05/30/changelog-171.html)

### Observations/Thoughts

* [Introducing the Ferrocene Language Specification](https://ferrous-systems.com/blog/ferrocene-language-specification/)
* [The Rust Jobs Market](https://www.rustjobs.com/blog/the-rust-jobs-market.html)
* [video] [Rust makes you feel like a GENIUS](https://www.youtube.com/watch?v=0rJ94rbdteE)
* [audio] [This Week in Rust Issue 443](https://rustacean-station.org/episode/twir-443/)

### Rust Walkthroughs

* [Impl-tools: beyond derive](https://kas-gui.github.io/blog/impl-tools.html)
* [Introduction to Rust generics (1/2): Traits](https://kerkour.com/rust-generics-traits)
* [Builder Lite](https://matklad.github.io/2022/05/29/builder-lite.html)
* [The curse of strong typing](https://fasterthanli.me/articles/the-curse-of-strong-typing)
* [Simple rust interview questions](https://flakm.github.io/posts/rust_interview_questions/)
* [Profiling heap allocation in rust](https://flakm.github.io/posts/heap_allocation/)
* [Arc and Mutex in Rust](https://itsallaboutthebit.com/arc-mutex/)
* [How I speeded up my Rust builds on GitHub ~30 times](https://ectobit.com/blog/speed-up-github-actions-rust-pipelines/)
* [Some notes on internal working of profiler](https://inspektor.cloud/blog/how-profiler-works/)
* [Rewriting the lexer benchmark in Rust ](https://eli.thegreenplace.net/2022/rewriting-the-lexer-benchmark-in-rust/)
* [Parallel programming design with BipBuffer](https://dev-random.io/parallel-programming-with-bip-buffer/)
* [A Neophyte's Introduction to the async/await landscape in Rust](https://www.geekabyte.io/2022/05/a-neophytes-introduction-to-asyncawait.html)


### Research

* [Hardware/Software Co-Assurance using the Rust Programming Language and ACL2](https://arxiv.org/abs/2205.11709)

### Miscellaneous

* [RustConf 2022 Presentation Schedule](https://rustconf.com/schedule)

## Crate of the Week

This week's crate is [pgfplots](https://github.com/DJDuque/pgfplots), a crate to generate publication-quality figures (with or without LaTeX).

Thanks to [Daniel Duque](https://users.rust-lang.org/t/crate-of-the-week/2704/1066) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [mirrord - extract panics on non-existent directories](https://github.com/metalbear-co/mirrord/issues/53)
* [mirrord - Support MIRRORD_AGENT_TTL as an argument in the CLI](https://github.com/metalbear-co/mirrord/issues/62)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

361 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-05-23..2022-05-30

* [prepare Rust for opaque pointers](https://github.com/rust-lang/rust/pull/94214)
* [parse expression after `else` as a condition if followed by `{`](https://github.com/rust-lang/rust/pull/97298)
* [macros: introduce `fluent_messages` macro](https://github.com/rust-lang/rust/pull/97327)
* [improve error message for E0081](https://github.com/rust-lang/rust/pull/97456)
* [minor improvement on else-no-if diagnostic](https://github.com/rust-lang/rust/pull/97370)
* [output correct type responsible for structural match violation](https://github.com/rust-lang/rust/pull/97351)
* [miri: adjust Miri to also require return places everywhere](https://github.com/rust-lang/miri/pull/2138)
* [miri: avoid error patterns matching themselves](https://github.com/rust-lang/miri/pull/2158)
* [miri: enable number validity checking and `ptr::invalid` checking by default](https://github.com/rust-lang/miri/pull/2151)
* [do writeback of Closure params before visiting the parent expression](https://github.com/rust-lang/rust/pull/97302)
* [implement `Hash` for `core::alloc::Layout`](https://github.com/rust-lang/rust/pull/97034)
* [refactor call terminator to always include destination place](https://github.com/rust-lang/rust/pull/96098)
* [split out the various responsibilities of `rustc_metadata::Lazy`](https://github.com/rust-lang/rust/pull/97291)
* [try to cache `region_scope_tree` as a query](https://github.com/rust-lang/rust/pull/97383)
* [add a deep `fast_reject` routine](https://github.com/rust-lang/rust/pull/97345)
* [extend `ptr::null` and `null_mut` to all thin (including extern) types](https://github.com/rust-lang/rust/pull/94954)
* [stabilize `cell_filter_map`](https://github.com/rust-lang/rust/pull/97308)
* [partially stabilize (`const_`)`slice_ptr_len` feature by stabilizing `NonNull::len`](https://github.com/rust-lang/rust/pull/94640)
* [use rounding instead of truncation in float to Duration conversion methods](https://github.com/rust-lang/rust/pull/96051)
* [improve case conversion happy path](https://github.com/rust-lang/rust/pull/97046)
* [rustc: fix ICE in native library error reporting](https://github.com/rust-lang/rust/pull/97328)
* [codegen\_gcc: Define immutable statics with const qualified types](https://github.com/rust-lang/rustc_codegen_gcc/pull/165)
* [libcore: add `iter::from_generator` which is like `iter::from_fn`, but for coroutines instead of functions](https://github.com/rust-lang/rust/pull/96298)
* [builtins: faster float conversion operations](https://github.com/rust-lang/compiler-builtins/pull/464)
* [clippy: add new lint `unused_rounding`](https://github.com/rust-lang/rust-clippy/pull/8866)
* [clippy: add `doc_link_with_quotes` lint](https://github.com/rust-lang/rust-clippy/pull/8385)
* [clippy: new lint about use first() instead of get(0)](https://github.com/rust-lang/rust-clippy/pull/8882)
* [clippy: new lint `no_effect_replace`](https://github.com/rust-lang/rust-clippy/pull/8754)
* [clippy: support `Weak` in `rc_clone_in_vec_init`](https://github.com/rust-lang/rust-clippy/pull/8885)
* [clippy: `get_last_with_len`: lint `VecDeque` and any deref to slice](https://github.com/rust-lang/rust-clippy/pull/8862)
* [clippy: fix `empty_line_after_outer_attribute` false positive](https://github.com/rust-lang/rust-clippy/pull/8892)
* [clippy: `identity_op`: add parenthesis to suggestions where required](https://github.com/rust-lang/rust-clippy/pull/8786)
* [clippy: introduce `allow-dbg-in-tests` config value](https://github.com/rust-lang/rust-clippy/pull/8897)
* [rust-analyzer: add implicit static lifetime hints](https://github.com/rust-lang/rust-analyzer/pull/12416)
* [rust-analyzer: generate variant: insert code in file with enum definition](https://github.com/rust-lang/rust-analyzer/pull/12384)
* [rust-analyzer: fix overflow during type inference for tuple struct patterns](https://github.com/rust-lang/rust-analyzer/pull/12409)
* [rust-analyzer: correct single-file module rename](https://github.com/rust-lang/rust-analyzer/pull/12387)
* [rust-analyzer: clear native diagnostics for files when they are deleted](https://github.com/rust-lang/rust-analyzer/pull/12383)
* [rust-analyzer: retrigger visibility completion after parentheses](https://github.com/rust-lang/rust-analyzer/pull/12412)
* [rust-analyzer: `f32` and `f64` representation during lowering](https://github.com/rust-lang/rust-analyzer/pull/12395)
* [rust-analyzer: make `files.excludeDirs` work](https://github.com/rust-lang/rust-analyzer/pull/12341)

### Rust Compiler Performance Triage

A good week: The regressions were small; some have follow-up PR's in flight to
address them; and we saw a big improvement from PR
[#97345](https://github.com/rust-lang/rust/pull/97345), which adds more fast
paths for quickly exiting comparisons between two types (such as `BitsImpl<M>`
and `BitsImpl<N>` for const integers `M` and `N`). This improved compile-times
for the `bitmaps` benchmark by 50-65% in some cases (including the trunk
`nalgebra`, according to independent investigation from nnethercote). That same
PR had more modest improvements (1% to 2%) to the compile-times for a number of
other crates. Many thanks to lcnr and nnethercote for some excellent work here!

Triage done by **@pnkfelix**.
Revision range: [43d9f385..0a43923a](https://perf.rust-lang.org/?start=43d9f3859e0204e764161ee085a360274b5f3e9a&end=0a43923a86c3b8f11d005884871b152f59b746f7&absolute=false&stat=instructions%3Au)

3 Regressions, 1 Improvements, 9 Mixed; 0 of them in rollups
59 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-05-31.md) 

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

* [Allow using for<'a> syntax when declaring closures](https://github.com/rust-lang/rfcs/pull/3216)
* [Create a types team](https://github.com/rust-lang/rfcs/pull/3254)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Tracking Issue for scoped threads](https://github.com/rust-lang/rust/issues/93203)
* [disposition: merge] [Put a bound on collection misbehavior](https://github.com/rust-lang/rust/pull/97316)
* [disposition: merge] [Tracking Issue for RFC 3128: I/O Safety](https://github.com/rust-lang/rust/issues/87074)
* [disposition: merge] [Lang: Stabilize usage of rustc_nonnull_optimization_guaranteed on -1](https://github.com/rust-lang/rust/issues/97122)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [notice] [Mention about removal of crate visibility specifier](https://github.com/rust-lang/rfcs/pull/3273)

## Upcoming Events

Rusty Events between 2022-06-01 - 2022-06-29 🦀

### Virtual

* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)
* 2022-06-07 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydcjbkb/)
* 2022-06-07 | Berlin, DE | [Berline.rs](https://berline.rs/) | [Open Tech School Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsydcjbkb/)
* 2022-06-07 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**June Meetup: Intro to monoids and semi groups using Frunk**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcjbkb/)
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

### North America

* 2022-06-01 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/285973465/)
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
* 2022-06-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Async Rust and Embedded**](https://www.meetup.com/Rust-Oslo/events/286236751/)
### Oceania

* 2022-06-17 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
    * [**June 2022 Meetup**](https://www.meetup.com/Rust-Melbourne/events/285962368/)

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

**Element**

* [Software Engineer - VoIP (Go/Rust) (Remote)](https://apply.workable.com/elementio/j/5BD58AFB6F/)

**Quickwit**

* [Senior Software Engineer, Rust & distributed systems (Remote, European/Asian time zones)](https://quickwit.io/jobs/distributed-software-engineer)

**Micropelt**

* [Embedded Engineer (Freiburg, DE)](http://www.micropelt.com/en/micropelt/jobs)

**Stockly**

* [Back-end developer (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly/jobs/back-end-developer-rust-grpc-postgresql_paris)

**Audiotool**

* [Rust / Audio DSP Developer](https://www.audiotool.com/jobs#rust-audio-dsp-developer)

**Zcash Foundation**

* [Core Engineer (Remote)](https://cryptojobslist.com/jobs/core-engineer-zcash-foundation-no-restrictions)

**Bolt Labs**

* [Cryptographic Engineer at Bolt Labs (Remote, US)](https://hackmd.io/@bolt/H1gYR6NEq)
* [Senior Software Engineer (Rust) at Bolt Labs (Remote, US)](https://hackmd.io/@bolt/S1wOGzS4q)
* [Software Engineer (Rust) at Bolt Labs (Remote, US)](https://hackmd.io/@bolt/SknjzfB49)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is a perfect language for a dad like me, who every day puts kids to sleep, and tired after long day of work and chores, can sit down and possibly write some code for the hobby open source project, even when he's already just half awake. And it usually just works, tend to be robust and make the day feel extra productive.

– [Dawid Ciężarkiewicz on /r/rust](https://www.reddit.com/r/rust/comments/uxx7w8/this_week_in_rust_444/ia1cwn6)

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/v2wdgj/this_week_in_rust_445/)</small>
