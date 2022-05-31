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

### Foundation

### Newsletters

### Project/Tooling Updates

* [Helix editor 22.05 released](https://helix-editor.com/news/release-22-05-highlights/)

### Observations/Thoughts

### Rust Walkthroughs

* [Simple rust interview questions](https://flakm.github.io/posts/rust_interview_questions/)
* [Profiling heap allocation in rust](https://flakm.github.io/posts/heap_allocation/)
* [Arc and Mutex in Rust](https://itsallaboutthebit.com/arc-mutex/)
* [How I speeded up my Rust builds on GitHub ~30 times](https://ectobit.com/blog/speed-up-github-actions-rust-pipelines/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [pgfplots](https://github.com/DJDuque/pgfplots), a crate to generate publication-quality figures (with or without LaTeX).

Thanks to [Daniel Duque](https://users.rust-lang.org/t/crate-of-the-week/2704/1066) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-06-01 - 2022-06-29 🦀

### Virtual

* 2022-05-25 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydchbhc/)
* 2022-05-26 | Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 22th Edition**](https://www.meetup.com/Rust-Linz/events/286006468/)
* 2022-05-31 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydchbpc/)
* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)
* 2022-06-07 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydcjbkb/)
* 2022-06-07 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcjbkb/)
* 2022-06-08 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcjblb/)
* 2022-06-09 | Dublin, IE | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Verus — Verified Rust for low-level systems code**](https://www.meetup.com/Rust-Dublin/events/286018947/)
* 2022-06-09 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydcjbmb/)
* 2022-06-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust June 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/285952122/)
* 2022-06-09 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydcjbmb/)
* 2022-06-14 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcjbsb/)
* 2022-06-15 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbtb/)
* 2022-06-15 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Nutshell**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcjbtb/)
* 2022-06-21 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydcjbcc/)

### North America

* 2022-05-25 | New York, NY, US | [Rust NYC](https://www.meetup.com/Rust-NYC/)
    * [**May Lightning Talks: State machines for sync, BonsaiDB, WASM Cloudflare Workers**](https://www.meetup.com/Rust-NYC/events/285925838/)
* 2022-05-31 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Happy Hour and Planning (everyone welcome)**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/285954876/)
* 2022-06-01 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/285973465/)
* 2022-06-08 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcjblb/)
* 2022-06-09 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcjbmb/)
* 2022-06-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person **](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcjbcc/)

### Europe

* 2022-05-30 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks May 2022 *Quickwit Takeover* RSVP @Skills Matter**](https://www.meetup.com/Rust-London-User-Group/events/285740296/)
* 2022-05-31 | Rome, IT | [Rust Roma](https://www.meetup.com/Rust-Roma/)
    * [**When Rocket is fueled by Diesel #Aperitech**](https://www.meetup.com/Rust-Roma/events/285587293/)
* 2022-06-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Introduction to axum - An ergonomic and modular web framework by David Pedersen**](https://www.meetup.com/Rust-Oslo/events/286006378/)

### Oceania

* 2022-05-26 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/Rust-Brisbane/)
    * [**May Meetup**](https://www.meetup.com/Rust-Brisbane/events/285665676/)
* 2022-06-17 | Melbourne, VI, AU | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
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

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is a perfect language for a dad like me, who every day puts kids to sleep, and tired after long day of work and chores, can sit down and possibly write some code for the hobby open source project, even when he's already just half awake. And it usually just works, tend to be robust and make the day feel extra productive.

– [Dawid Ciężarkiewicz on /r/rust](https://www.reddit.com/r/rust/comments/uxx7w8/this_week_in_rust_444/ia1cwn6)

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
