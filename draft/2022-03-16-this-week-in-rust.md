Title: This Week in Rust 434
Number: 434
Date: 2022-03-16
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
* [Lang team March update](https://blog.rust-lang.org/inside-rust/2022/03/09/lang-team-mar-update.html)
* [Rust Compiler March 2022 Steering Cycle](https://blog.rust-lang.org/inside-rust/2022/03/11/mar-steering-cycle.html)

### Newsletters
* [This Month in Rust GameDev #31 - February 2022](https://gamedev.rs/news/031/)

### Project/Tooling Updates
* [Rust-Analyzer Changelog #120](https://rust-analyzer.github.io/thisweek/2022/03/14/changelog-120.html)
* [Tantivy 0.17 is out | Quickwit](https://quickwit.io/blog/tantivy-0.17/)
* [Knurling-rs changelog #34](https://ferrous-systems.com/blog/knurling-changelog-34/)
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W10](https://www.fornjot.app/blog/weekly-dev-log/2022-w10/)
* [Slint Version 0.2.1 released -- weekly update](https://slint-ui.com/thisweek/2022-03-14.html)
* [Arti 0.1.0 is released: Your somewhat-stable API is here!](https://blog.torproject.org/arti_010_released/)
* [Dioxus v0.2 Release: TUI, Router, Fermi, and Tooling](https://dioxuslabs.com/blog/release-020)

### Observations/Thoughts
* [Introducing Rustler Precompiled - Dashbit Blog](https://dashbit.co/blog/rustler-precompiled)
* [Announcing Savage, a computer algebra system written in Rust](https://www.reddit.com/r/rust/comments/tcxrs3/announcing_savage_a_computer_algebra_system/)
* [Oxide at Home: Propolis says Hello](https://artemis.sh/2022/03/14/propolis-oxide-at-home-pt1.html)
* [Things I hate about Rust, redux](https://blog.yossarian.net/2022/03/10/Things-I-hate-about-Rust-redux)
* [Async IO with completion-model IO systems](https://www.ncameron.org/blog/async-io-with-completion-model-io-systems/)
* [Transcript: What's New in Rust 1.58 and 1.59](https://rustacean-station.org/transcript/058-rust-1.58-1.59/)
* [video] [Rust Gamedev Meetup #14: March 2022](https://www.youtube.com/watch?v=dQPkyjbd36Y)
* [video] [Rust fuzzing using cargo-libafl (LibAFL-based fuzzer) - Rust Security #4](https://www.youtube.com/watch?v=0gpGA80DA0s)
* [audio] [Interview with Fish Fight | Rust Game Dev](https://rustgamedev.com/episodes/interview-with-fish-fight)
* [audio] [PancakeDB with Martin Loncaric :: Rustacean Station](https://rustacean-station.org/episode/059-martin-loncaric/)

### Rust Walkthroughs
* [An In-Depth Introduction To Idempotency | A learning journal](https://www.lpalmieri.com/posts/idempotency/)
* [A Rust Gem: The Rust Map API](https://www.thecodedmessage.com/posts/rust-map-entry/)
* [RPATH, or why lld doesn’t work on NixOS](https://matklad.github.io/2022/03/14/rpath-or-why-lld-doesnt-work-on-nixos.html)
* [Rust WebAssembly OCR experiments](https://hugopeixoto.net/articles/rust-wasm-ocr-experiments.html)

### Miscellaneous
* [AUTOSAR announces WG for Rust in automotive](https://www.autosar.org/news-events/details/autosar-announces-new-working-group-for-programming-language-rust-in-automotive-software-context-202/)
* [audio] [Nick Cameron on Juggling Open Source Work and Parenthood](https://anchor.fm/building-with-rust/episodes/Nick-Cameron-on-Juggling-Open-Source-Work-and-Parenthood-e1fhfc7)
* [audio] [Devtools podcast: building a new terminal in Rust](https://console.dev/podcast/s02e10-terminal-tools-michell-lim-zach-lloyd-warp/)

## Crate of the Week

This week's crate is [noline](https://crates.io/crates/noline), a small no-std compatible readline-like line editor.

A lack of suggestions notwithstanding, llogiq is pretty pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

302 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-03-07..2022-03-14

* [diagnostics: do not spuriously claim something is "not an iterator"](https://github.com/rust-lang/rust/pull/94870)
* [diagnostics: single colon within `<>` probably, not type ascription](https://github.com/rust-lang/rust/pull/94865)
* [improve suggestion when casting `usize` to (possibly) wide pointer](https://github.com/rust-lang/rust/pull/92150)
* [warn users about `||` in `let` chain expressions](https://github.com/rust-lang/rust/pull/94754)
* [suggest `if let`/`let_else` for refutable pat in `let`](https://github.com/rust-lang/rust/pull/94739)
* [suggest using double colon when a struct field type include single colon](https://github.com/rust-lang/rust/pull/94839)
* [miri: implement `simd_`{`shuffle`, `gather`, `scatter`}](https://github.com/rust-lang/miri/pull/2013)
* [CTFE/Miri: detect out-of-bounds pointers in `offset_from`](https://github.com/rust-lang/rust/pull/94827)
* [change several `HashMap`s to `IndexMap` to improve incremental hashing performance](https://github.com/rust-lang/rust/pull/90253)
* [improve `AdtDef` interning](https://github.com/rust-lang/rust/pull/94733)
* [optimize `ascii::escape_default`](https://github.com/rust-lang/rust/pull/94776)
* [make some `Clone` impls `const`](https://github.com/rust-lang/rust/pull/91804)
* [remove argument from closure in `thread::Scope::spawn`](https://github.com/rust-lang/rust/pull/94559)
* [use `MaybeUninit` in `VecDeque` to remove the undefined behavior of slice](https://github.com/rust-lang/rust/pull/94472)
* [constify `Index`{,`Mut`} for `[T]`, `str`, and `[T; N]`](https://github.com/rust-lang/rust/pull/94657)
* [fix soundness issue in scoped threads](https://github.com/rust-lang/rust/pull/94644)
* [implement `BITS` constant for non-zero integers](https://github.com/rust-lang/rust/pull/93292)
* [implement `MIN`/`MAX` constants for non-zero integers](https://github.com/rust-lang/rust/pull/93293)
* [add `Result::`{`ok`, `err`, `and`, `or`, `unwrap_or`} as `const`](https://github.com/rust-lang/rust/pull/92385)
* [add `Atomic`*`::get_mut_slice`](https://github.com/rust-lang/rust/pull/94816)
* [add `core::hint::must_use`](https://github.com/rust-lang/rust/pull/94723)
* [unix: reduce the size of `DirEntry`](https://github.com/rust-lang/rust/pull/94750)
* [portable-simd: add `.min` and `.max` for integers](https://github.com/rust-lang/portable-simd/pull/260)
* [compiler-builtins: add support for Apple watchOS](https://github.com/rust-lang/compiler-builtins/pull/456)
* [futures: add `Mutex::lock_owned` and `Mutex::try_lock_owned`](https://github.com/rust-lang/futures-rs/pull/2571)
* [rustfmt: improve mod resolution error for mods with multiple candidate files](https://github.com/rust-lang/rustfmt/pull/5243)
* [clippy: improve styles of filtering options for Clippy's lint list](https://github.com/rust-lang/rust-clippy/pull/8070)
* [clippy: new lint that detects useless match expression](https://github.com/rust-lang/rust-clippy/pull/8471)
* [clippy: new lint: `only_used_in_recursion`](https://github.com/rust-lang/rust-clippy/pull/8422)
* [clippy: allow `single_component_path_imports` for all macros](https://github.com/rust-lang/rust-clippy/pull/8537)
* [clippy: make `search_is_some`s suggestion `MachineApplicable`](https://github.com/rust-lang/rust-clippy/pull/8536)

### Rust Compiler Performance Triage

Largely a quiet week. The perf improvement highlight is the use of real world crates such as `syn`, `cargo`, and `serde` in the collecting of profile guided optimization (PGO) profiles. Previously only `libcore` was used. This led to some decent improvement in compilation of real world crates (upwards of 5.5%). 

On the regression side, the regressions were all largely small but contained inside of rollups making them hard to diagnose and correct. The perf team continues to work on process improvements that make changes to the compiler land through CI quickly while minimizing perf regressions that can sneak through.

Triage done by **@rylev**.
Revision range: [10dccdc..3ba1eb](https://perf.rust-lang.org/?start=10dccdc7fcbdc64ee9efe2c1ed975ab8c1d61287&end=3ba1ebea122238d1a5c613deb1bf60ce24bd8fd8&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 3 Mixed; 3 of them in rollups
42 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-03-15.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Cargo alternative registry auth](https://github.com/rust-lang/rfcs/pull/3139)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Tracking Issue for JoinHandle::is_finished](https://github.com/rust-lang/rust/issues/90470)
* [disposition: merge] [Tracking issue for const extern fn and const unsafe extern fn](https://github.com/rust-lang/rust/issues/64926)
* [disposition: merge] [Always evaluate all cfg predicate in all() and any()](https://github.com/rust-lang/rust/pull/94295)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Packages as (optional) namespaces](https://github.com/rust-lang/rfcs/pull/3243)

## Upcoming Events

Rusty Events between 2022-03-16 - 2022-04-13 🦀

### Virtual

* 2022-03-16 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/284221983/)
* 2022-03-16 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydcfblb/)
* 2022-03-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Building a Randomizer**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcfbvb/)
* 2022-03-16 | Wien, AT | [Mob-Programming on Open Source Software](https://www.meetup.com/Mob-Programming-on-Open-Source-Software/)
    * [**The Rustic Mob**](https://www.meetup.com/Mob-Programming-on-Open-Source-Software/events/284548235/)
* 2022-03-17 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/lwgrssydcfbwb/)
* 2022-03-23 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/284379020/)
* 2022-03-24 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**Embedded Rust reaching out--Learn how Rust can interact with the outside world**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/284627448/)
* 2022-03-24 | Linz, AU | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 20th Edition**](https://www.meetup.com/Rust-Linz/events/284584338/)
* 2022-03-29 | Berlin, DE | [Berline](https://berline.rs/)
    * [**Rust Hack and Learn Tuesday**](https://berline.rs/2022/03/29/rust-hack-and-learn.html)
* 2022-03-29 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydcfbmc/)
* 2022-03-30 | México City, MX | [Rust MX](https://www.meetup.com/Rust-MX/)
    * [**Platica Marzo 2022 - Reescribir o no reescribir aplicaciones en Rust**](https://www.meetup.com/Rust-MX/events/284560362/)
* 2022-04-05 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydcgbhb/)
* 2022-04-05 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcgbhb/)
* 2022-04-05 | Denver, CO, US | [Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/)
    * [**Using Futures to wrap an unsafe USB API to play audio directly - with live stream**](https://www.meetup.com/Rust-Boulder-Denver/events/284371995/)
* 2022-04-06 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydcgbjb/)
* 2022-04-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcgbjb/)
* 2022-04-07 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #12**](https://www.meetup.com/rust-noris/events/zgfnssydcgbsb/)
* 2022-04-12 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcgbqb/)
* 2022-04-12 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly Meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydcgbqb/)
* 2022-04-13 | Boulder, CO | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcgbrb/)
* 2022-04-13 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydcgbrb/)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/283790509/)

### Europe
* 2022-03-17 | Edinburgh, UK | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**Rust Night March, Edinburgh**](https://www.meetup.com/rust-edi/events/284428178/)
* 2022-04-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/Rust-Berlin/)
    * [**Rust and Tell - an onsite event**](https://www.meetup.com/Rust-Berlin/events/284512764/) | [**Alt Link**](https://berline.rs/2022/04/12/rust-and-tell.html)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/283790509/)

### North America
* 2022-04-05 | Denver, CO, US | [Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/)
    * [**Using Futures to wrap an unsafe USB API to play audio directly - with live stream**](https://www.meetup.com/Rust-Boulder-Denver/events/284371995/)
* 2022-04-13 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcgbrb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs


**Juspay**

* [Freelance Rust Developer (Remote)](https://www.upwork.com/jobs/~0106e33943207a0fc6)

**Kollider**

* [Senior Frontend Engineer - Rust (Remote)](https://careers.kollider.xyz/senior-frontend-engineer/en)
* [Junior Backend Engineer - Rust (Remote)](https://careers.kollider.xyz/junior-backend-engineer/en)

**Tempus Ex**

 * [Several full-time Rust positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US, and Remote)](https://tempus-ex.com/careers)


*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> protip: the rust extern keyword has a --help flag
>
> ```text
> error[E0703]: invalid ABI: found `--help`
>  --> ext.rs:1:8
>   |
> 1 | extern "--help" {}  fn main() {}
>   |        ^^^^^^^^ invalid ABI
>   |
>   = help: valid ABIs: Rust, C, C-unwind, cdecl, stdcall, stdcall-unwind, fastcall, vectorcall, thiscall, thiscall-unwind, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
>
> error: aborting due to previous error
>
> For more information about this error, try `rustc --explain E0703`.
> ```

– [Aria the Cat (with some help from rustc) on twitter](https://twitter.com/Gankra_/status/1501307407292641280)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1188) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
