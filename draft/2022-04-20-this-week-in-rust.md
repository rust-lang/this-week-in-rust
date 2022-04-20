Title: This Week in Rust 439
Number: 439
Date: 2022-04-20
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

### Foundation

### Project/Tooling Updates

### Newsletters

### Research

### Observations/Thoughts
* [Why You Should Be Worried About the Future of C/C++ in Embedded: A Case for Rust](https://apollolabsblog.hashnode.dev/why-you-should-be-worried-about-the-future-of-cc-in-embedded-a-case-for-rust)

### Rust Walkthroughs

### Miscellaneous
* [35 Rust Learning Resources Every Beginner Should Know in 2022](https://apollolabsblog.hashnode.dev/35-rust-learning-resources-every-beginner-should-know-in-2022)

## Crate of the Week

This week's crate is [hifitime](https://github.com/nyx-space/hifitime), a high fidelity time management library.

Thanks to [Christopher Rabotin](https://users.rust-lang.org/t/crate-of-the-week/2704/1054) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

373 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-04-04..2022-04-11

* [promote `x86_64-unknown-none` target to Tier 2 and distribute build artifacts](https://github.com/rust-lang/rust/pull/95705)
* [track individual proc-macro expansions in the self-profiler](https://github.com/rust-lang/rust/pull/95473)
* [only suggest removing semicolon when expression is compatible with `impl Trait`](https://github.com/rust-lang/rust/pull/95758)
* [stop flagging unexpected inner attributes as outer ones in certain diagnostics](https://github.com/rust-lang/rust/pull/95189)
* [strict provenance lints](https://github.com/rust-lang/rust/pull/95599)
* [suggest adding a local for vector to fix borrowck errors](https://github.com/rust-lang/rust/pull/95807)
* [suggest derivable trait on E0277 error](https://github.com/rust-lang/rust/pull/95525)
* [suggest replacing `typeof(...)` with an actual type](https://github.com/rust-lang/rust/pull/95784)
* [diagnostics: give a special note for unsafe fn / `Fn`/`FnOnce`/`FnMut`](https://github.com/rust-lang/rust/pull/95663)
* [diagnostics: translation infrastructure](https://github.com/rust-lang/rust/pull/95512)
* [diagnostics: use correct span for const generics](https://github.com/rust-lang/rust/pull/95654)
* [`assert_uninit_valid`: ensure we detect at least arrays of uninhabited types](https://github.com/rust-lang/rust/pull/95374)
* [report opaque type mismatches directly during borrowck of the function instead of within the `type_of` query](https://github.com/rust-lang/rust/pull/95767)
* [call `compute_locs` once per rule](https://github.com/rust-lang/rust/pull/95669)
* [shrink `Nonterminal`](https://github.com/rust-lang/rust/pull/95715)
* [cached stable hash cleanups](https://github.com/rust-lang/rust/pull/95524)
* [remove explicit delimiter token trees from `Delimited`](https://github.com/rust-lang/rust/pull/95797)
* [add new `ThinBox` type for 1 stack pointer wide heap allocated trait objects](https://github.com/rust-lang/rust/pull/90066)
* [enhance `ConstGoto` mir-opt by moving up `StorageDead` statements](https://github.com/rust-lang/rust/pull/95723)
* [miri: add support for `FUTEX_`{`WAIT`, `WAKE`}`_BITSET`](https://github.com/rust-lang/miri/pull/2054)
* [miri: for variadic functions, accept arbitrary trailing arguments](https://github.com/rust-lang/miri/pull/2058)
* [add `<[[T; N]]>::flatten`{`_mut`}](https://github.com/rust-lang/rust/pull/95579)
* [fix unsound `File` methods](https://github.com/rust-lang/rust/pull/95469)
* [replace Linux `Mutex` and `Condvar` with futex based ones](https://github.com/rust-lang/rust/pull/95035)
* [stabilize `Stdin::lines`](https://github.com/rust-lang/rust/pull/95185)
* [use bitwise XOR in `to_ascii_uppercase`](https://github.com/rust-lang/rust/pull/95831)
* [fix: `Vec` leak when capacity is 0](https://github.com/rust-lang/rust/pull/95699)
* [portable-SIMD: use SIMD equality for `PartialEq` on SIMD vectors](https://github.com/rust-lang/portable-simd/pull/274)
* [cargo: part 2 of RFC2906 - allow inheriting from a different `Cargo.toml`](https://github.com/rust-lang/cargo/pull/10517)
* [cargo: part 3 of RFC2906 - Add support for inheriting `license-path`, and `depednency.path`](https://github.com/rust-lang/cargo/pull/10538)
* [cargo: part 4 of RFC2906 - Add support for inheriting `readme`](https://github.com/rust-lang/cargo/pull/10548)
* [rustdoc: early doc link resolution fixes and refactorings](https://github.com/rust-lang/rust/pull/95706)
* [rustdoc: fix empty doc comment with backline ICE](https://github.com/rust-lang/rust/pull/95804)
* [rustdoc: reduce allocations in a `html::markdown` function](https://github.com/rust-lang/rust/pull/95868)
* [clippy: add `err_expect` lint](https://github.com/rust-lang/rust-clippy/pull/8606)
* [clippy: add a lint to detect cast to unsigned for `abs()` and suggest `unsigned_abs()`](https://github.com/rust-lang/rust-clippy/pull/8635)
* [clippy: add lints `drop_non_drop` and `forget_non_drop`](https://github.com/rust-lang/rust-clippy/pull/8630)
* [clippy: new lint `is_digit_ascii_radix`](https://github.com/rust-lang/rust-clippy/pull/8624)
* [clippy: don't lint `iter_with_drain` on references](https://github.com/rust-lang/rust-clippy/pull/8668)
* [clippy: don't lint various match lints when expanded by a proc-macro](https://github.com/rust-lang/rust-clippy/pull/8667)
* [clippy: fix `as_deref_mut` false positives in `needless_option_as_deref`](https://github.com/rust-lang/rust-clippy/pull/8646)
* [clippy: fix `same_functions_in_if_condition` false positive](https://github.com/rust-lang/rust-clippy/pull/8673)
* [clippy: fix `unsound_collection_transmute`](https://github.com/rust-lang/rust-clippy/pull/8648)
* [clippy: fix subtraction overflow in `cast_possible_truncation`](https://github.com/rust-lang/rust-clippy/pull/8687)
* [clippy: fix `unnecessary_cast` suggestion for type aliasses](https://github.com/rust-lang/rust-clippy/pull/8596)
* [clippy: remove overlap between `manual_split_once` and `needless_splitn`](https://github.com/rust-lang/rust-clippy/pull/8631)
* [clippy: suggest `from_utf8_unchecked` in const contexts](https://github.com/rust-lang/rust-clippy/pull/8612)
* [clippy: `indexing_slicing` should not fire if a valid array index comes from a constant function that is evaluated at compile-time](https://github.com/rust-lang/rust-clippy/pull/8588)
* [clippy: `unnecessary_owned_empty_strings`](https://github.com/rust-lang/rust-clippy/pull/8660)
* [clippy: fix false positive in `needless_match`](https://github.com/rust-lang/rust-clippy/pull/8549)
* [clippy: ignore `&x | &y` in `unnested_or_patterns`](https://github.com/rust-lang/rust-clippy/pull/8619)

### Rust Compiler Performance Triage

A week with a large amount of changes in rollups, which makes performance triage difficult. The performance team and the infra team are working on finding ways to automate marking PRs as likely a poor choice for rolling up. Otherwise, the week overall saw a ~1% improvement in incremental check builds, with smaller improvements to incremental debug and release builds. A number of benchmarks have been updated in the last few weeks, which has meant a decrease in the automated noise assessment's algorithm performance, but that should settle out to steady state behavior on its own in the next few days.

Triage done by **@simulacrum**.
Revision range: [949b98ca..4e1927d](https://perf.rust-lang.org/?start=949b98cab8a186b98bf87e64374b8d0848c55271&end=4e1927db3c399fa34dc71992bd5dbec09f945c3d&absolute=false&stat=instructions%3Au)

5 Regressions, 4 Improvements, 7 Mixed; 7 of them in rollups
50 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-04-12.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [New Rust attribute to support embedding debugger visualizers](https://github.com/rust-lang/rfcs/pull/3191)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Check if call return type is visibly uninhabited when building MIR](https://github.com/rust-lang/rust/pull/93313)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-04-20 - 2022-05-18 🦀

### Virtual

* 2022-04-20 | Cardiff, UK | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust image viewer from scratch**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/285193324/)
* 2022-04-20 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/284644487)
* 2022-04-20 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust April 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/284705301/)
* 2022-04-20 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Movie/Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcgbbc/)
* 2022-04-21 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**The Rusty Boss--directing the world with Rust, Bluetooth, and a Raspberry Pi**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/284922970)
* 2022-04-26 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399979/)
* 2022-04-26 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/285118431/)
* 2022-04-26 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn**](https://www.meetup.com/Rust-London-User-Group/events/285273624/)
* 2022-04-27 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcgbkc/)
* 2022-04-28 | Linz, AU | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 21st Edition**](https://www.meetup.com/Rust-Linz/events/285248503/)
* 2022-05-03 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydchbfb/)
* 2022-05-03 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/284996307/)
* 2022-05-04 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121667/)
* 2022-05-04 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/285282177/)
* 2022-05-05 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**Dealing with failure: producing and consuming Errors in Rust**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/285078007/)
* 2022-05-10 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399988/)
* 2022-05-10 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydchbnb/)
* 2022-05-10 | Rostock, DE | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**7. Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/283819127/)
* 2022-05-10 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydchbnb/)
* 2022-05-11 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydchbpb/)
* 2022-05-12 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/tzjtssydchbqb/)
* 2022-05-12 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydchbqb/)
* 2022-05-17 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydchbwb/)
* 2022-05-18 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydchbxb/)

### Europe

* 2022-04-26 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn**](https://www.meetup.com/Rust-London-User-Group/events/285273624/)

### North America

* 2022-04-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Boston Rust Meetup at Amazon**](https://www.meetup.com/BostonRust/events/284808948)
* 2022-05-11 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydchbpb/)
* 2022-05-12 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydchbqb/)
* 2022-05-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydchbwb/)

### Oceania

* 2022-04-21 | Melbourne, AUS | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
    * [**Rust Melbourne is back!**](https://www.meetup.com/Rust-Melbourne/events/284327357/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Relying on the programmer to always read, comprehend, and remember the documentation – and then do everything right, every time – is how we get bugs.

– [Cliff Biffle on his blog](http://cliffle.com/blog/rust-mutexes/#conclusions)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1210) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
