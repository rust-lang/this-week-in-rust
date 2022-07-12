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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

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

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-07-13 - 2022-08-10 🦀

### Virtual

* 2022-07-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydckbjb/)
* 2022-07-07 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust, nalgebra, and Fourier Optics**](https://www.meetup.com/charlottesville-rust-meetup/events/285818136/)
* 2022-07-09 | Virtual | [Rust Game Dev](https://github.com/rust-gamedev/wg)
    * [**Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-07-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Rust Meetup - "Lifetimes" and Social Hour**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286994065/)
* 2022-07-12 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydckbqb/)
* 2022-07-12 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/286935763/)
* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-13 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/rFzwUjh5YT1pVci6A)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)
* 2022-07-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydckbcc/)
* 2022-07-26 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydckbjc/)
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

* 2022-07-06 | Paris, FR | [Stockly](https://www.welcometothejungle.com/fr)
    * [**Rust Meetup in Paris**](https://www.eventbrite.com/e/rust-meetup-in-paris-hosted-by-stockly-tickets-358592809747)
* 2022-07-12 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/286935763/)

### North America

* 2022-07-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Rust Meetup - "Lifetimes" and Social Hour**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286994065/)
* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)
* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)

### Oceania

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

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Learning Rust has taught me something - "There are really no problems, just adventure and opportunities"

– [Adeoye Adefemi on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1263)

Thanks to [Adeoye Adefemi and Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1266) for the suggestion as well as [Christopher Durham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1267) for the leniency.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
