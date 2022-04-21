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
* [Rust Compiler April 2022 Steering Cycle](https://blog.rust-lang.org/inside-rust/2022/04/15/apr-steering-cycle.html)
* [Please welcome Thom and Chris to Library Contributors](https://blog.rust-lang.org/inside-rust/2022/04/18/libs-contributors.html)
* [Imposter Syndrome](https://blog.rust-lang.org/inside-rust/2022/04/19/imposter-syndrome.html)

### Foundation
* [Member Spotlight: KDAB](https://foundation.rust-lang.org/news/member-spotlight-kdab/)

### Project/Tooling Updates
* [MnemOS Initial Release Announcement](https://jamesmunns.com/blog/mnemos-initial-release/)
* [Bevy 0.7](https://bevyengine.org/news/bevy-0-7/)
* [rust-analyzer changelog #125](https://rust-analyzer.github.io/thisweek/2022/04/18/changelog-125.html)
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W15 (Pre-Vacation Edition)](https://www.fornjot.app/blog/weekly-dev-log/2022-w15/)
* [Rewriting sysctl(8) in Rust (systeroid)](https://blog.orhun.dev/rewriting-sysctl-in-rust/)
* [Postage v0.5.0](https://implaustin.hashnode.dev/whats-new-in-postage-v050)
* [Slint (GUI crate) weekly update](https://slint-ui.com/thisweek/2022-04-18.html)
* [This week in Fluvio #29: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0029/)
* [This week in Databend #38: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-04-20-databend-weekly/)

### Observations/Thoughts
* [Why You Should Be Worried About the Future of C/C++ in Embedded: A Case for Rust](https://apollolabsblog.hashnode.dev/why-you-should-be-worried-about-the-future-of-cc-in-embedded-a-case-for-rust)
* [Coherence and crate-level where-clauses](https://smallcultfollowing.com/babysteps/blog/2022/04/17/coherence-and-crate-level-where-clauses/)
* [How Async Functions in Traits could Work in Rustc](https://blog.theincredibleholk.org/blog/2022/04/18/how-async-functions-in-traits-could-work-in-rustc/)
* [Choosing the Right Integers](https://www.thecodedmessage.com/posts/programming-integers/)
* [Rustaceans at the border](https://lwn.net/SubscriberLink/889924/2f35f6746c3dd9b1/)
* [Building a new terminal - using macOS Metal APIs for UI rendering (interview with Warp)](https://console.dev/interviews/warp-zach-lloyd/)
* [35 Rust Learning Resources Every Beginner Should Know in 2022](https://apollolabsblog.hashnode.dev/35-rust-learning-resources-every-beginner-should-know-in-2022)
* [We asked 5 people why they like Embedded Rust](https://tweedegolf.nl/en/blog/70/we-asked-5-people-why-they-like-embedded-rust)
* [A Performance Evaluation on Rust Asynchronous Frameworks](https://zenoh.io/blog/2022-04-14-rust-async-eval/)
* [Compile time evaluation in Nim, Zig, Rust and C++](https://castillodel.github.io/compile-time-evaluation/)
* [Including two versions of a Rust crate in a single project](https://reltech.substack.com/p/including-two-versions-of-a-rust)
* [video] [2022-04-20 Design Meeting: Felienne Hermans, Psychology of Programming](https://www.youtube.com/watch?v=nrkVIq0ccII)
* [audio] [Armin Ronacher on experimental deserialization with Deser](https://rustacean-station.org/episode/064-armin-ronacher/)

### Rust Walkthroughs
* [LoRaWAN Applications in Rust](https://tweedegolf.nl/en/blog/69/lorawan-applications-in-rust)
* [Building a crawler in Rust: Synchronization (Atomic Types and Barriers)](https://kerkour.com/rust-crawler-synchronization-atomic-types-barrier)
* [Hostname based router with axum in Rust](https://kerkour.com/rust-axum-hostname-router)
* [Command line argument parsing in Rust using Clap](https://blog.logrocket.com/command-line-argument-parsing-rust-using-clap/)
* [Full-text search for DynamoDB using Lambda, EFS, Tantivy and Rust](https://jakejscott.com/full-text-search-for-dynamodb-using-lambda-efs-tantivy-and-rust)
* [Building and Deploying a URL shortener with Rust in 10 minutes or less](https://www.shuttle.rs/blog/2022/03/13/url-shortener)
* [ Rust on Nails: A full stack architecture for Rust web applications](https://cloak.software/blog/rust-on-nails/)
* [An Intro to the Sauron web Framework](https://blog.chainsafe.io/an-intro-to-the-sauron-web-framework-a6093a2ac9eb)
* [video] [I'm in ur address space](https://fasterthanli.me/videos/im-in-ur-address-space)
* [video] [A Mir Formality Walkthrough #1](https://www.youtube.com/watch?v=7ZQ-9yZztKA)

### Miscellaneous
* [Cancellation: RustFest Global EMEA Edition](https://rustfest.world/news/emea-edition-cancellation)

## Crate of the Week

This week's crate is [ttrpc](https://github.com/containerd/ttrpc-rust), a GRPC-like protocol implementation for memory-constrained environments.

Thanks to [George Hahn](https://users.rust-lang.org/t/crate-of-the-week/2704/1057) for the suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [RustConf 2022 CFP is Open!](https://cfp.rustconf.com/events/rustconf-2022)
* [Rust Foundation Community Grants Program 2022 is accepting applications](https://foundation.rust-lang.org/grants/)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

388 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-04-11..2022-04-18

(note: We now also track rust-analyzer, which recently joined the rust-lang org)

* [rustc\_metadata: do not encode unnecessary module children](https://github.com/rust-lang/rust/pull/95899)
* [miri: add size assertions for some core types](https://github.com/rust-lang/miri/pull/2070)
* [miri: implement `strerror_r`](https://github.com/rust-lang/miri/pull/2067)
* [use `Span::find_ancestor_inside` to get right span in `CastCheck`](https://github.com/rust-lang/rust/pull/95920)
* [when checking pointee metadata, canonicalize the `Sized` check](https://github.com/rust-lang/rust/pull/95315)
* [better method call ERROR messages](https://github.com/rust-lang/rust/pull/92364)
* [consider lifetimes when comparing types for equality in MIR validator](https://github.com/rust-lang/rust/pull/96004)
* [fix a bad error message for `relative paths are not supported in visibilities` error](https://github.com/rust-lang/rust/pull/95936)
* [improve diagnostics for unterminated nested block comment](https://github.com/rust-lang/rust/pull/95859)
* [strict provenance lint diagnostics improvements](https://github.com/rust-lang/rust/pull/96112)
* [`let_chains`: forbid `let` inside parentheses](https://github.com/rust-lang/rust/pull/95008)
* [make `unaligned_references` lint deny-by-default and make it show up in future-breakage report](https://github.com/rust-lang/rust/pull/95372)
* [prevent opaque types from appearing in impl headers](https://github.com/rust-lang/rust/pull/95973)
* [fix suggestions in case of `T:` bounds](https://github.com/rust-lang/rust/pull/95970)
* [fix miscompilation of inline assembly with outputs in cases where we emit an invoke instead of call instruction](https://github.com/rust-lang/rust/pull/95864)
* [remove function parameters only used in recursion](https://github.com/rust-lang/rust/pull/96027)
* [optimize `<SourceFile as Decodable>::decode`](https://github.com/rust-lang/rust/pull/95981)
* [simplify const params diagnostic on stable](https://github.com/rust-lang/rust/pull/95820)
* [stabilize `derive_default_enum`](https://github.com/rust-lang/rust/pull/94457)
* [stabilize `const_extern_fn` for "Rust" and "C"](https://github.com/rust-lang/rust/pull/95346)
* [move `CStr` to libcore, and `CString` to liballoc](https://github.com/rust-lang/rust/pull/94079)
* [replace `ReentrantMutex` by a futex based one on Linux](https://github.com/rust-lang/rust/pull/95727)
* [replace `RwLock` by a futex based one on Linux](https://github.com/rust-lang/rust/pull/95801)
* [implement SIMD gather/scatter via vector `getelementptr`](https://github.com/rust-lang/rust/pull/95961)
* [faster parsing for lower numbers for radix up to 16 (cont.)](https://github.com/rust-lang/rust/pull/95399)
* [implement `core::ptr::Unique` on top of `NonNull`](https://github.com/rust-lang/rust/pull/96010)
* [implement tuples using recursion](https://github.com/rust-lang/rust/pull/95914)
* [optimize `RcInnerPtr::inc_strong()`/`inc_weak()` instruction count](https://github.com/rust-lang/rust/pull/95224)
* [speed up `Vec::clear()`](https://github.com/rust-lang/rust/pull/96002)
* [`impl const Default for Box<[T]>` and `Box<str>`](https://github.com/rust-lang/rust/pull/95947)
* [futures: `TryFlattenUnordered`](https://github.com/rust-lang/futures-rs/pull/2577)
* [codegen\_gcc: add feature for future libgccjit 12 release](https://github.com/rust-lang/rustc_codegen_gcc/pull/158)
* [cargo: add support for `rustc --check-cfg` well known names and values](https://github.com/rust-lang/cargo/pull/10486)
* [cargo: import `cargo-add` into cargo](https://github.com/rust-lang/cargo/pull/10472)
* [cargo: completion support for `cargo-add`](https://github.com/rust-lang/cargo/pull/10577)
* [rustdoc: discriminate required and provided associated constants and types](https://github.com/rust-lang/rust/pull/95316)
* [rustfmt: fix exponential execution time by memoizing `format_expr`](https://github.com/rust-lang/rustfmt/pull/5139)
* [rustfmt: preserve attributes for `imports_granularity=Item`](https://github.com/rust-lang/rustfmt/pull/5314)
* [clippy: add `await_holding_invalid_type` lint](https://github.com/rust-lang/rust-clippy/pull/8707)
* [clippy: add `usize` cast to `manual_bits` suggestion](https://github.com/rust-lang/rust-clippy/pull/8677)
* [clippy: check for loops/closures in `local_used_after_expr`](https://github.com/rust-lang/rust-clippy/pull/8676)
* [clippy: do not trigger `rest_pat_in_fully_bound_structs` on `#[non_exhaustive]` structs](https://github.com/rust-lang/rust-clippy/pull/8690)
* [clippy: don't lint `let_unit_value` when needed for type inferenece](https://github.com/rust-lang/rust-clippy/pull/8563)
* [clippy: don't lint `manual_non_exhaustive` when the enum variant is used](https://github.com/rust-lang/rust-clippy/pull/8645)
* [clippy: fix ICE in `undocumented_unsafe_blocks`](https://github.com/rust-lang/rust-clippy/pull/8686)
* [clippy: introduce `needless_option_take` lint](https://github.com/rust-lang/rust-clippy/pull/8665)
* [clippy: new lint `format_add_strings`](https://github.com/rust-lang/rust-clippy/pull/8626)
* [clippy: only lint `mut_from_ref` when unsafe code is used](https://github.com/rust-lang/rust-clippy/pull/8647)
* [clippy: prevent infinite (exponential) recursion in `only_used_in_recursion`](https://github.com/rust-lang/rust-clippy/pull/8691)
* [clippy: `pub_use` restriction](https://github.com/rust-lang/rust-clippy/pull/8670)
* [clippy: adding condition for `map_clone` message](https://github.com/rust-lang/rust-clippy/pull/8688)
* [clippy: `assertions_on_constants`: ignore indirect `cfg!`](https://github.com/rust-lang/rust-clippy/pull/8614)
* [clippy: fix `unnecessary_to_owned` about msrv](https://github.com/rust-lang/rust-clippy/pull/8692)
* [rustup: don't prepend `$CARGO_HOME/bin` unnecessarily](https://github.com/rust-lang/rustup/pull/2978)
* [rust-analyzer: derive completions take existing derives into count](https://github.com/rust-lang/rust-analyzer/pull/12024)
* [rust-analyzer: add trailing `;` when typing `=` in assignment](https://github.com/rust-lang/rust-analyzer/pull/11971)
* [rust-analyzer: attempt to format `expand_macro` output with rustfmt if possible](https://github.com/rust-lang/rust-analyzer/pull/12014)
* [rust-analyzer: switch to LSP inlay hints](https://github.com/rust-lang/rust-analyzer/pull/11935)
* [rust-analyzer: allow customizing the command for running build scripts](https://github.com/rust-lang/rust-analyzer/pull/11956)
* [rust-analyzer: deprioritize already-imported names in `use` items](https://github.com/rust-lang/rust-analyzer/pull/11961)
* [rust-analyzer: fix proc-macro change check being inverted](https://github.com/rust-lang/rust-analyzer/pull/12008)
* [rust-analyzer: show `impl Trait` in argument positon in completion details](https://github.com/rust-lang/rust-analyzer/pull/12011)
* [rust-analyzer: enable ADT keyword completions in block expressions](https://github.com/rust-lang/rust-analyzer/pull/11993)
* [rust-analyzer: resolve `uN::method` even when `use std::uN;` is present](https://github.com/rust-lang/rust-analyzer/pull/11992)

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

* [Add provide_any module to core](https://github.com/rust-lang/rfcs/pull/3192)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Add target configuration](https://github.com/rust-lang/rfcs/pull/3239)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Stabilize let_chains in Rust 1.62.0](https://github.com/rust-lang/rust/pull/94927)
* [disposition: merge] [Implement Default for AssertUnwindSafe](https://github.com/rust-lang/rust/pull/95949)
* [disposition: merge] [Tracking Issue for const offset_from \(const_ptr_offset_from\)](https://github.com/rust-lang/rust/issues/92980)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Create a types team](https://github.com/rust-lang/rfcs/pull/3254)

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

**Element**

* [Software Engineer - VoIP (Go/Rust) (Remote)](https://apply.workable.com/elementio/j/5BD58AFB6F/)

**GMTO Organization**

* [Data Scientist (Pasadena, CA, US)](https://giantmagellan.org/employment/data-scientist/)

**NXLog**

* [Rust Developer (Remote, Europe or worldwide)](https://application.nxlog.org/jobs/detail/rust-developer-39)

**KidsLoop**

* [Junior Audio/ Video DevOps Engineer (Seoul, KR)](https://kidsloop.bamboohr.com/jobs/view.php?id=399)

**Timescale**

* [Senior Rust Engineer - TimescaleDB Toolkit (Remote: UTC-5 to UTC-8)](https://www.timescale.com/careers/5920911002?gh_jid=5920911002)

**Stockly**

* [Back-end developer (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-rust-grpc-postgresql_paris)
* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)

**Kollider**

* [Senior Frontend Engineer - Rust (Remote)](https://careers.kollider.xyz/senior-frontend-engineer/en)
* [Junior Backend Engineer - Rust (Remote)](https://careers.kollider.xyz/junior-backend-engineer/en)
* [Senior Full Stack Developer - Rust (Remote)](https://careers.kollider.xyz/senior-full-stack-developer/en)

**Tempus Ex**

* [Several full-time Rust positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US, and Remote)](https://tempus-ex.com/careers)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Alas, this week went by without any memorable quote.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/u8b9nc/this_week_in_rust_439/)</small>
