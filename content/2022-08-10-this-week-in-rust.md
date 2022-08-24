Title: This Week in Rust 455
Number: 455
Date: 2022-08-10
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
* [Non-lexical lifetimes (NLL) fully stable](https://blog.rust-lang.org/2022/08/05/nll-by-default.html)
* [Rust Compiler Midyear Report for 2022](https://blog.rust-lang.org/inside-rust/2022/08/08/compiler-team-2022-midyear-report.html)
* [Please welcome Dan to Library Contributors](https://blog.rust-lang.org/inside-rust/2022/08/10/libs-contributors.html)

### Foundation
* [Trademark Policy: Review and Survey](https://foundation.rust-lang.org/news/2022-08-09-trademark-policy-review-and-survey/)

### Newsletters
* [This Month in Rust OSDev: July 2022](https://rust-osdev.com/this-month/2022-07/)

### Project/Tooling Updates
* [Announcing: MiniRust](https://www.ralfj.de/blog/2022/08/08/minirust.html)
* [Achieving A Completely Open Source Implementation of Apple Code Signing and Notarization](https://gregoryszorc.com/blog/2022/08/08/achieving-a-completely-open-source-implementation-of-apple-code-signing-and-notarization/)
* [Announcing Cargo WAPM](https://adventures.michaelfbryan.com/posts/announcing-cargo-wapm/)
* [argmin 0.6.0 and argmin-math 0.1.0 released](http://argmin-rs.org/blog/version-v0-6-0/)
* [rust-analyzer changelog #141](https://rust-analyzer.github.io/thisweek/2022/08/08/changelog-141.html)
* [Fyrox 0.27 Feature Highlights](https://fyrox.rs/blog/post/feature-highlights-0-27/)
* [rustc_codegen_gcc: Progress Report #14](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-14)
* [IntelliJ Rust: Updates For the 2022.2 Release Cycle](https://blog.jetbrains.com/rust/2022/08/03/intellij-rust-updates-for-the-2022-2-release-cycle/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - 2022-W32](https://www.fornjot.app/blog/weekly-release/2022-w32/)
* [Fang, async background processing for Rust](https://pxp9.github.io/rust/async-processing/)
* [HexoSynth 2022 - Devlog #8 - A Visual DSP Programming Language](https://m8geil.de/posts/hexosynth-8/)
* [This week in Databend #54: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-08-10-databend-weekly/)

### Observations/Thoughts
* [Using unwrap() in Rust is Okay](https://blog.burntsushi.net/unwrap/)
* [A performance retrospective using Rust (part 3)](https://agourlay.github.io/rust-performance-retrospective-part3/)
* [The State Of Rust In 2022](https://deprogrammaticaipsum.com/the-state-of-rust-in-2022/)
* [Blocking Sockets and Async](https://www.thecodedmessage.com/posts/blocking-sockets/)
* [How does Rust async work?](https://aidancully.blogspot.com/2022/08/how-does-rust-async-work.html)
* [Basic Operator Overloading with Traits](https://rsdlt.onrender.com/posts/welcome-blog-rust-technology-development-programming-language/)

### Rust Walkthroughs
* [Understanding Rust generics and how to use them](https://blog.logrocket.com/understanding-rust-generics-how-use/)
* [Hot Reloading Rust — for Fun and Faster Feedback Cycles](https://robert.kra.hn/posts/hot-reloading-rust/)
* [The Magic of zerocopy (compared with scroll)](https://swatinem.de/blog/magic-zerocopy/)
* [Not a Yoking Matter (Zero-Copy #1)](https://manishearth.github.io/blog/2022/08/03/zero-copy-1-not-a-yoking-matter/)
* [Zero-Copy All The Things! (Zero-Copy #2)](https://manishearth.github.io/blog/2022/08/03/zero-copy-2-zero-copy-all-the-things/)
* [So Zero It's ... Negative? (Zero-Copy #3)](https://manishearth.github.io/blog/2022/08/03/zero-copy-3-so-zero-its-dot-dot-dot-negative/)
* [What is server middleware](https://www.shuttle.rs/blog/2022/08/04/middleware)
* [Rust and WebAssembly without a Bundler](https://tung.github.io/posts/rust-and-webassembly-without-a-bundler/)
* [What are Rust's HTTP extensions?](https://blog.adamchalmers.com/what-are-extensions/)
* [Mapping into the serde data model](https://owengage.com/writing/2022-08-05-mapping-into-the-serde-data-model/)
* [Generate Enum Variant with associated values in Rust Analyzer](https://dorianlistens.com/2022/08/rust-analyzer-generate-enum-variant-with-associated-values/)
* [7 ways to pass a string between 🦀 Rust and C](https://dev.to/kgrech/7-ways-to-pass-a-string-between-rust-and-c-4ieb)
* [STM32F4 Embedded Rust at the HAL: I2C Temperature & Pressure Sensing with BMP180](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-hal-i2c-temperature-pressure-sensing-with-bmp180)

### Miscellaneous
* [EuroRust CFP](https://www.papercall.io/eurorust-2022)
* [Bevy Jam #2](https://itch.io/jam/bevy-jam-2)
* [Rust Arcade: Designing a Custom Arcade Machine](https://www.youtube.com/watch?v=1hALxNZimzc)

## Crate of the Week

This week's crate is [fang](https://github.com/ayrat555/fang) an async background processing crate.

Thanks to [Ayrat Badykov](https://users.rust-lang.org/t/crate-of-the-week/2704/1096) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [ctest2 - CTest uses syntex_syntax which doesn't support raw identifiers](https://github.com/JohnTitor/ctest2/issues/6)
* [ctest2 - Fails to parse `_` as a const identifier](https://github.com/JohnTitor/ctest2/issues/38)
* [ockam - Change `ockam tcp-listener create` command for --at NODE and ADDRESS](https://github.com/build-trust/ockam/issues/3149)
* [ockam - Implement the ockam secure-channel list command](https://github.com/build-trust/ockam/issues/3188)
* [ockam - Make services information displayed with ockam node create | show command more dynamic](https://github.com/build-trust/ockam/issues/3177)
* [mirrord - Remove unused dependencies from mirrord-layer](https://github.com/metalbear-co/mirrord/issues/220)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

330 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-08-01..2022-08-08

* [add support for link-flavor rust-lld for iOS, tvOS and watchOS](https://github.com/rust-lang/rust/pull/98771)
* [enable function merging when opt is for size](https://github.com/rust-lang/rust/pull/100035)
* [recover `require`, `include` instead of `use` in item](https://github.com/rust-lang/rust/pull/100167)
* [recover from C++ style `enum struct`](https://github.com/rust-lang/rust/pull/99786)
* [provide suggestion on missing `let` in binding statement](https://github.com/rust-lang/rust/pull/100111)
* [improve diagnostics for `const a: = expr;`](https://github.com/rust-lang/rust/pull/100168)
* [revive suggestions for boxed trait objects instead of impl Trait](https://github.com/rust-lang/rust/pull/100019)
* [suggest a positional formatting argument instead of a captured argument](https://github.com/rust-lang/rust/pull/100058)
* [suggest adding/removing `ref` for binding patterns](https://github.com/rust-lang/rust/pull/99835)
* [warn about dead tuple struct fields](https://github.com/rust-lang/rust/pull/95977)
* [do not exclusively suggest `;` when `,` is also a choice](https://github.com/rust-lang/rust/pull/98796)
* [avoid pointing out `return` span if it has nothing to do with type error](https://github.com/rust-lang/rust/pull/100130)
* [always create elided lifetimes, even if inferred](https://github.com/rust-lang/rust/pull/99953)
* [enable unused_parens for match arms](https://github.com/rust-lang/rust/pull/100093)
* [prevent ICE for `doc_alias` on match arm, statement, expression](https://github.com/rust-lang/rust/pull/100029)
* [detect type mismatch due to loop that might never iterate](https://github.com/rust-lang/rust/pull/100094)
* [miri: add `mkstemp` shim for unix](https://github.com/rust-lang/miri/pull/2346)
* [miri: add shim for realpath on unix](https://github.com/rust-lang/miri/pull/2457)
* [miri: add support for env::home_dir](https://github.com/rust-lang/miri/pull/2467)
* [miri: also forward --manifest-path to 'cargo metadata'](https://github.com/rust-lang/miri/pull/2474)
* [miri: fix an ICE in nanosleep()](https://github.com/rust-lang/miri/pull/2466)
* [miri: implement some missing float functions](https://github.com/rust-lang/miri/pull/2469)
* [avoid invalidating the CFG in `MirPatch`](https://github.com/rust-lang/rust/pull/100087)
* [remove `fn backtrace` and replace with usages of provider API](https://github.com/rust-lang/rust/pull/99431)
* [add back `Send` and `Sync` impls on `ChunksMut` iterators](https://github.com/rust-lang/rust/pull/100023)
* [optimize `pointer::as_aligned_to`](https://github.com/rust-lang/rust/pull/100169)
* [portable SIMD: fix interleave/deinterleave for vectors with only one lane](https://github.com/rust-lang/portable-simd/pull/299)
* [codegen\_gcc: support symbol visibility](https://github.com/rust-lang/rustc_codegen_gcc/pull/203)
* [cargo: improve error message for `no such subcommand`](https://github.com/rust-lang/cargo/pull/10924)
* [rustdoc: avoid inlining modules with duplicate names](https://github.com/rust-lang/rust/pull/99738)
* [rustdoc: do not mark the contents of a skipped module as inlined](https://github.com/rust-lang/rust/pull/100207)
* [rust-analyzer: add a setting to disable comment continuation in VSCode](https://github.com/rust-lang/rust-analyzer/pull/12934)
* [rust-analyzer: add fixups for incomplete in proc-macros](https://github.com/rust-lang/rust-analyzer/pull/12937)
* [rust-analyzer: add more constructors and entry-APIs for la-arena](https://github.com/rust-lang/rust-analyzer/pull/12931)
* [rust-analyzer: add syntax fixup for while loops](https://github.com/rust-lang/rust-analyzer/pull/12880)
* [rust-analyzer: corrected order of printing op and `=`](https://github.com/rust-lang/rust-analyzer/pull/12974)
* [rust-analyzer: don't switch workspace on vfs file changes from libraries](https://github.com/rust-lang/rust-analyzer/pull/12947)
* [rust-analyzer: error Diagnostics appear in the wrong place](https://github.com/rust-lang/rust-analyzer/pull/12939)
* [rust-analyzer: fix `test_rainbow_highlighting` gate](https://github.com/rust-lang/rust-analyzer/pull/12959)
* [rust-analyzer: generate rust type from json](https://github.com/rust-lang/rust-analyzer/pull/12905)
* [rust-analyzer: more methods and traits for `la_arena::ArenaMap`](https://github.com/rust-lang/rust-analyzer/pull/12956)
* [rust-analyzer: parse range patterns in struct and slice without trailing comma](https://github.com/rust-lang/rust-analyzer/pull/12962)
* [rust-analyzer: run stable `fmt` & `cargo` through `rustup`](https://github.com/rust-lang/rust-analyzer/pull/12953)
* [rust-analyzer: use an empty expander for ignored non-attribute proc-macros](https://github.com/rust-lang/rust-analyzer/pull/12933)
* [rust-analyzer: handle operators like their trait functions in the IDE layer](https://github.com/rust-lang/rust-analyzer/pull/12948)
* [rust-analyzer: support associated values in "Generate Enum Variant" assist](https://github.com/rust-lang/rust-analyzer/pull/12837)
* [rust-analyzer: fix incorrect token pick rankings](https://github.com/rust-lang/rust-analyzer/pull/12949)
* [rust-analyzer: make `concat!` work with char](https://github.com/rust-lang/rust-analyzer/pull/12942)
* [clippy: add `elapsed_instant` lint](https://github.com/rust-lang/rust-clippy/pull/9264)
* [clippy: fix ICE when reading literals with weird proc-macro spans](https://github.com/rust-lang/rust-clippy/pull/9303)
* [clippy: fix `cast_abs_to_unsigned` with code in parens](https://github.com/rust-lang/rust-clippy/pull/9266)
* [clippy: fix suggestions for `async` closures in `redundant_closure_call`](https://github.com/rust-lang/rust-clippy/pull/9053)
* [clippy: more proc-macro detection](https://github.com/rust-lang/rust-clippy/pull/8694)
* [clippy: move `significant_drop_in_scrutinee` into `nursery`](https://github.com/rust-lang/rust-clippy/pull/9302)
* [clippy: rename `logic_bug` to `overly_complex_bool_expr`](https://github.com/rust-lang/rust-clippy/pull/9306)
* [clippy: `explicit_auto_deref` changes](https://github.com/rust-lang/rust-clippy/pull/9126)
* [clippy: add paren before '?' when suggesting deref for `clone_on_copy`](https://github.com/rust-lang/rust-clippy/pull/9282)

### Rust Compiler Performance Triage

A pretty quiet week for performance. Unfortunately, by far the biggest change was a regression introduced by increasing the minimum libc version for linux-gnu targets. The exact reason for why this happened in this case is unclear, and it's not easy to investigate. Luckily, the average regression introduced by this change was 0.4% which is fairly small, and many of the larger regressions were limited to doc builds.

Triage done by **@rylev**.
Revision range: [792bc5a0..cc4dd6fc](https://perf.rust-lang.org/?start=792bc5a0102d0973d42183a2b267850bb905236f&end=cc4dd6fc9f1a5c798df269933c7e442b79661a86&absolute=false&stat=instructions%3Au)

**Summary**:

|            | mean | max | count |
|:----------:|:----:|:---:|:-----:|
| Regressions ❌ <br /> (primary) | 0.5% | 1.4% | 146   |
| Regressions ❌ <br /> (secondary) | 0.8% | 1.6% | 78    |
| Improvements ✅ <br /> (primary) | N/A  | N/A | 0     |
| Improvements ✅ <br /> (secondary) | -2.0% | -4.0% | 9     |
| All ❌✅ (primary) | 0.5% | 1.4% | 146   |

1 Regressions, 2 Improvements, 2 Mixed; 1 of them in rollups
42 artifact comparisons made in total

Full report [here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-08-09.md)

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

* [disposition: merge] [[RFC] Support `.comment` section like GCC/Clang (`!llvm.ident`)](https://github.com/rust-lang/rust/pull/97550)
* [disposition: merge] [Tracking Issue for "unsafe blocks in unsafe fn" (RFC #2585)](https://github.com/rust-lang/rust/issues/71668)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Distinguish closures from callables](https://github.com/rust-lang/rfcs/pull/3300)

## Upcoming Events

Rusty Events between 2022-08-10 - 2022-09-07 🦀

### Virtual

* 2022-08-10 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydclbnb/)
* 2022-08-10 | Virtual (Brasília, BR) | [Turing Community](https://www.meetup.com/turing-community/) + [Turing Community LatAm](https://www.meetup.com/turing-community/members/?op=leaders)
    * [**Coding Dojo #2: Rust**](https://www.meetup.com/turing-community/events/287559034/)
* 2022-08-10 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**cargo expand --bin writing_proc_macros**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287450224/)
* 2022-08-10 | Virtual (Hoboken, NJ, US) | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydclbnb/)
* 2022-08-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydclbpb/)
* 2022-08-12 | Virtual + Tokyo, JP | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://www.reddit.com/r/rust/comments/w7bktx/2022_tokyo_and_elsewhere_rust_game_hack_event_aug/)
* 2022-08-13 | Virtual | [Rust Gamedev](https://gamedev.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://www.google.com/url?q=https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2Eop9Blil9YUWeTq472NIi)
* 2022-08-13 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287058154/)
* 2022-08-16 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/287286736/)
* 2022-08-16 | Virtual (Granada, ES) | [Google Developer Group Granada](https://www.meetup.com/granadagdg/)
    * [**¡Aprendamos Rust!**](https://www.meetup.com/granadagdg/events/287702154/)
* 2022-08-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydclbvb/)
* 2022-08-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**C++ Concepts vs Rust Traits**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287450947/)
* 2022-08-17 | Virtual (Hoboken, NJ, US) | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/287468144/)
* 2022-08-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)
* 2022-08-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydclbxb/)
* 2022-08-18 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #8**](https://rust-meetup.ir/2022/08/18/8th-meetup.html)
* 2022-08-18 | Virtual (Wiesbaden, DE) | [Frontend RheinMain](https://www.meetup.com/frontend_rm/)
    * [**Rust for curious developers**](https://www.meetup.com/frontend_rm/events/287713743/)
* 2022-08-20 | Virtual | [Rust Edu](https://rust-edu.org/workshop)
    * [**Rust Education Workshop 2022 (Submission deadline 2022-08-16)**](https://rust-edu.org/workshop)
* 2022-08-20 | Virtual (Hoboken, NJ, US) | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**Share Your Coding Project(s)!**](https://www.meetup.com/neighborhood-math-club/events/kbjcssydclbbc/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-27 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059968/)
* 2022-08-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/287286751/)
* 2022-08-30 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydclbnc/)
* 2022-09-01 | Virtual | [Google Open Source Live](https://www.meetup.com/google-open-source/)
    * [**Rust Day on Google Open Source Live**](https://www.meetup.com/google-open-source/events/287435626/)
* 2022-09-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together**](https://www.meetup.com/rust-noris/events/287092397/)
* 2022-09-03 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059974/)
* 2022-09-03 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 1: Tokio my-redis Tutorial**](https://www.meetup.com/rust-noris/events/287346970/)
* 2022-09-06 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/286481325/)
* 2022-09-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcmbjb/)
* 2022-09-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121715/)
* 2022-09-10 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059979/)

### Asia

* 2022-08-12 | Tokyo, JP + Virtual | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://bombercrab-rust-game-hack.peatix.com/view)

### Europe

* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)

### North America

* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)
* 2022-08-11 | Columbus, OH, US| [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydclbpb/)
* 2022-08-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydclbvb/)
* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)
* 2022-08-25 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Concurrencia & paralelismo con Rust**](https://www.meetup.com/rust-mx/events/287561814/)
* 2022-08-25 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Hello World Cargo Crates Using Github Actions with jojobyte and Food!**](https://www.meetup.com/utah-rust/events/kvrxqsydclbpb/)

### Oceania

* 2022-08-24 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-26 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**August 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/287468753/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/voglel/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Don't come empty-handed to a project saying "this could be rewritten in Rust". It's obnoxious and gives the rust community a bad name.  
> Do start the project on your own, adding Rust to the build system and converting some significant functions, and then ask the project's community for comments.

– [moltonel on /r/rust](https://www.reddit.com/r/rust/comments/wfriz3/comment/iiw49bw/)

Thanks to [zjp-CN](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1277) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/wlhfs1/this_week_in_rust_455/)</small>
