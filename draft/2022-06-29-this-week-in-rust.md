Title: This Week in Rust 449
Number: 449
Date: 2022-06-29
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

This week's crate is [cap-std](https://github.com/bytecodealliance/cap-std) a std-replacement that introduces capabilities to facilitate defense-in-depth sandboxing.

Thanks to [Kinrany](https://users.rust-lang.org/t/crate-of-the-week/2704/1073) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

388 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-06-20..2022-06-27

* [LLVM on AVR: fix expanding `MOVW` for overlapping registers](https://github.com/rust-lang/llvm-project/pull/141)
* [work around llvm 12's memory ordering restrictions](https://github.com/rust-lang/rust/pull/98385)
* [proc_macro/bridge: cache static spans in proc_macro's client thread-local state](https://github.com/rust-lang/rust/pull/98187)
* [remove (transitive) reliance on sorting by `DefId` in pretty-printer](https://github.com/rust-lang/rust/pull/96955)
* [avoid an ICE and instead let the compiler report a useful error](https://github.com/rust-lang/rust/pull/98329)
* [greatly improve error reporting for futures and generators in `note_obligation_cause_code`](https://github.com/rust-lang/rust/pull/98259)
* [improve memory ordering diagnostics](https://github.com/rust-lang/rust/pull/97389)
* [improve suggestion for calling fn-like expr on type mismatch](https://github.com/rust-lang/rust/pull/98280)
* [diagnostics: consider parameter count when suggesting smart pointers](https://github.com/rust-lang/rust/pull/98509)
* [mitigate MMIO stale data vulnerability](https://github.com/rust-lang/rust/pull/98126)
* [suggest defining variable as mutable on `&mut _` type mismatch in pats](https://github.com/rust-lang/rust/pull/98431)
* [use correct substs in enum discriminant cast](https://github.com/rust-lang/rust/pull/98429)
* [improve `derive(Debug)`](https://github.com/rust-lang/rust/pull/98190)
* [miri: `freebsd-target-support`](https://github.com/rust-lang/miri/pull/2221)
* [miri: handle wildcard pointers in Stacked Borrows](https://github.com/rust-lang/miri/pull/2196)
* [miri: require local annotations for local diagnostics](https://github.com/rust-lang/miri/pull/2250)
* [miri: add `-Zmiri-report-progress` to regularly print a stacktrace of what we are executing](https://github.com/rust-lang/miri/pull/2272)
* [miri: fix ICE when const refers to extern static](https://github.com/rust-lang/miri/pull/2241)
* [miri: make sure a thread is joined](https://github.com/rust-lang/miri/pull/2276)
* [expand expressions where possible](https://github.com/rust-lang/rust/pull/98148) (RFC [#2011](https://rust-lang.github.io/rfcs/2011-generic-assert.html))
* [partial stabilization of "nonzero_checked_ops"](https://github.com/rust-lang/rust/pull/97547)
* [stabilize `NonZero*` checked operations constness](https://github.com/rust-lang/rust/pull/97908)
* [add `Iterator::next_chunk`](https://github.com/rust-lang/rust/pull/93700)
* [leak `pthread_`{`mutex`, `rwlock`}`_t` if it's dropped while locked](https://github.com/rust-lang/rust/pull/98194)
* [make `RwLockReadGuard` covariant](https://github.com/rust-lang/rust/pull/96820)
* [windows: Iterative `remove_dir_all`](https://github.com/rust-lang/rust/pull/96412)
* [portable-simd: change `Simd::splat` to not generate a loop](https://github.com/rust-lang/portable-simd/pull/284)
* [cargo: fetch GitHub commits by long hash more efficiently](https://github.com/rust-lang/cargo/pull/10079)
* [cargo: stabilize config-cli](https://github.com/rust-lang/cargo/pull/10755)
* [compiletest: strip debuginfo by default for mode=ui](https://github.com/rust-lang/rust/pull/98140)
* [clippy: add `manual_rem_euclid` lint](https://github.com/rust-lang/rust-clippy/pull/9031)
* [clippy: add `cargo dev deprecate`](https://github.com/rust-lang/rust-clippy/pull/8871)
* [clippy: check for `--force-warn` in Clippy's driver run condition](https://github.com/rust-lang/rust-clippy/pull/9036)
* [clippy: fix `extra_unused_lifetimes` false positive](https://github.com/rust-lang/rust-clippy/pull/9037)
* [clippy: fix `let_undescore_lock` false-positive when binding without locking](https://github.com/rust-lang/rust-clippy/pull/8990)
* [clippy: lint `single_match` on `Option` matches](https://github.com/rust-lang/rust-clippy/pull/8985)
* [clippy: suggest `pointer::cast` when possible in `transmute_ptr_to_ref`](https://github.com/rust-lang/rust-clippy/pull/8939)
* [clippy: add `manual_find` lint for function return case](https://github.com/rust-lang/rust-clippy/pull/8649)
* [clippy: add `vec.capacity()` to `slow_vec_initialization` detection](https://github.com/rust-lang/rust-clippy/pull/8953)
* [clippy: confirm using chain in `collapsible_span_lint_calls`](https://github.com/rust-lang/rust-clippy/pull/9028)
* [clippy: `enum_variant_names` should ignore when all prefixes are `_`](https://github.com/rust-lang/rust-clippy/pull/9032)
* [clippy: new lint `manual_retain`](https://github.com/rust-lang/rust-clippy/pull/8972)
* [clippy: put parentheses around `neg_multiply` suggestion if needed](https://github.com/rust-lang/rust-clippy/pull/9026)
* [rust-analyzer: correct `target_feature` completion](https://github.com/rust-lang/rust-analyzer/pull/12635)
* [rust-analyzer: fix completions for locals not working properly inside macro calls](https://github.com/rust-lang/rust-analyzer/pull/12643)
* [rust-analyzer: improve proc macro errors a bit](https://github.com/rust-lang/rust-analyzer/pull/12629)
* [rust-analyzer: completes non exhaustive variant within the defining crate](https://github.com/rust-lang/rust-analyzer/pull/12625)
* [rust-analyzer: deduplicate cfg completions](https://github.com/rust-lang/rust-analyzer/pull/12642)
* [rust-analyzer: fix `doc_links` link type - Determine link type at start](https://github.com/rust-lang/rust-analyzer/pull/12605)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-06-29 - 2022-07-27 🦀

### Virtual

* 2022-06-22 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Study Session - Procedural Macros**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/286594851/)
* 2022-06-24 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286688186/)
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
* 2022-07-07 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust, nalgebra, and Fourier Optics**](https://www.meetup.com/charlottesville-rust-meetup/events/285818136/)
* 2022-07-09 | Virtual | [Rust Game Dev](https://github.com/rust-gamedev/wg)
    * [**Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-14 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/swgrssydckbsb/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)

### Asia

* 2022-07-04 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**Rust Interop, Rewrites and fun**](https://www.meetup.com/rust-tlv/events/286610368/)

### Europe

* 2022-06-22 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/286305083/)
* 2022-06-23 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #26**](https://www.meetup.com/rust-wroclaw/events/286415834/)
* 2022-06-28 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks June 2022: Community Showcase**](https://www.meetup.com/rust-london-user-group/events/286489185/)
* 2022-06-29 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Get started with Rust and stories from the frontlines**](https://www.meetup.com/rust-nederland/events/286582960/)
* 2022-06-30 | Copenhagen, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #26**](https://cph.rs/)

### North America

* 2022-06-29 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - June 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)
* 2022-06-29 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/286596997/)
* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)

### Oceania

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

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> JG: mem::replace / mem::swap ===
> ![Indiana Jones swapping the artifact for a bag of sand in a temple](https://c.tenor.com/eqLNYv0A9TQAAAAC/swap-indiana-jones.gif)
<!-- TODO: download and re-host the image to avoid trouble -->

> CV: except rustc would tell Indy that's a type mismatch
<!-- -->

> JG: Yes, that would be the boulder, I assume.
>
> Older compilers were more aggressive in error reporting.

[Jake Goulding and Cuviper on the Rust Zulip](https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Clone.20implementation.20for.20Box)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1262) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
