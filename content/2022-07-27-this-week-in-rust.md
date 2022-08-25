Title: This Week in Rust 453
Number: 453
Date: 2022-07-27
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
* [Announcing the Keyword Generics Initiative](https://blog.rust-lang.org/inside-rust/2022/07/27/keyword-generics.html)

### Foundation
* [What has the Foundation been up to?](https://foundation.rust-lang.org/news/2022-07-27-what-has-the-foundation-been-up-to/)

### Project/Tooling Updates
* [Aya: your tRusty eBPF companion](https://deepfence.io/aya-your-trusty-ebpf-companion/)
* [This week in Fluvio #39: The programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0039/)
* [This week in Fluvio #40: The programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0040/)
* [Announcing Relm4 v0.5 beta](https://relm4.org/blog/posts/announcing_relm4_v0.5_beta)
* [rust-analyzer changelog #139](https://rust-analyzer.github.io/thisweek/2022/07/25/changelog-139.html)
* [IntelliJ Rust Changelog #175](https://intellij-rust.github.io/2022/07/25/changelog-175.html)
* [Fornjot (code-first CAD in Rust) - Weekly Release - 2022-W30](https://www.fornjot.app/blog/weekly-release/2022-w30/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-07-25.html)
* [HexoSynth 2022 - Devlog #6 - Workflow and Oscilloscope](https://m8geil.de/posts/hexosynth-6/)
* [This week in Databend #52: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-07-27-databend-weekly/)

### Observations/Thoughts
* [How To Put 30 Languages Into 1.1MB](https://laurmaedje.github.io/posts/hypher/)
* [Does Rust need proc-macros 2.0?](https://aaronerhardt.github.io/blog/posts/proc_macro_v2/)
* [The Ferrocene Language Specification is here!](https://ferrous-systems.com/blog/the-ferrocene-language-specification-is-here/)
* [Fixing the Next Thousand Deadlocks: Why Buffered Streams Are Broken and How To Make Them Safer](https://blog.polybdenum.com/2022/07/24/fixing-the-next-thousand-deadlocks-why-buffered-streams-are-broken-and-how-to-make-them-safer.html)
* [A performance retrospective using Rust (part 2)](https://agourlay.github.io/rust-performance-retrospective-part2/)
* [Introduction to WebAssembly and Rust](https://bojanstipic.rs/blog/01-introduction-to-webassembly-and-rust/)
* [A look at serde-json](https://owengage.com/writing/2022-07-22-a-look-at-serde-json/)
* [Impl Trait Parameters And Turbofish - Veetaha Website](https://veetaha.github.io/impl-trait-parameters-and-turbofish.html)
* [Console #115 - Interview with Jonathan of BonsaiDb](https://console.substack.com/p/console-115)
* [audio] [Asynchronous Rust with Tyler Mandry](https://rustacean-station.org/episode/tyler-mandry/)
* [audio] [Tauri with Daniel Thompson-Yvetot](https://rustacean-station.org/episode/daniel-thompson/)

### Rust Walkthroughs
* [Writing Linux Kernel Modules in Rust](https://lore.kernel.org/rust-for-linux/CAN6UTaywU8NfeNVi2ZDfwgjo3LCKiQRbD9PUWKsRp33Gxo+4gg@mail.gmail.com/)
* [STM32F4 Embedded Rust at the HAL: Timer Ultrasonic Distance Measurement](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-hal-timer-ultrasonic-distance-measurement)
* [Move data out of closure](https://dev.to/antonov_mike/move-data-out-of-closure-4jck)

## Crate of the Week

This week's crate is [cargo-semver-checks](https://crates.io/crates/cargo-semver-checks), a CI-friendly tool to check your library's API.

Thanks to [Predrag Gruevski](https://users.rust-lang.org/t/crate-of-the-week/2704/1087) and [Matthias Beyer](https://users.rust-lang.org/t/crate-of-the-week/2704/1088) for the (self and other) nominations.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ockam - Make output node information and format consistent across `ockam node create | show | list` commands](https://github.com/build-trust/ockam/issues/3042)
* [ockam - Combine `ockam identity print` and `ockam identity export` into one command `ockam identity show`](https://github.com/pest-parser/pest/discussions/606)
* [mirrord - Add e2e tests for a bash script](https://github.com/metalbear-co/mirrord/issues/200)
* [mirrord - Add e2e sanity tests for Rails](https://github.com/metalbear-co/mirrord/issues/199)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

397 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-07-18..2022-07-25

* [rustc\_expand: switch `FxHashMap` to `FxIndexMap` where iteration is used](https://github.com/rust-lang/rust/pull/99320)
* [proc\_macro/bridge: stop using a remote object handle for proc\_macro `Ident` and `Literal`](https://github.com/rust-lang/rust/pull/98189)
* [allow to disable thinLTO buffer to support lto-embed-bitcode lld feature](https://github.com/rust-lang/rust/pull/98162)
* [allow `trait A: ~const B`](https://github.com/rust-lang/rust/pull/93429)
* [do not allow typeck children items to constrain outer RPITs](https://github.com/rust-lang/rust/pull/99345)
* [do not resolve associated const when there is no provided value](https://github.com/rust-lang/rust/pull/99449)
* [don't suggest unnameable generic arguments](https://github.com/rust-lang/rust/pull/99580)
* [improve suggestions for `NonZeroT` <- `T` coercion error](https://github.com/rust-lang/rust/pull/99438)
* [improve suggestions for returning binding](https://github.com/rust-lang/rust/pull/99539)
* [add E0790 as more specific variant of E0283](https://github.com/rust-lang/rust/pull/98028)
* [miri: add a scheme for always using the default toolchain, running clippy and fmt before running any other command](https://github.com/rust-lang/miri/pull/2381)
* [miri: add a flag to print a diagnostic when an outdated value is returned from an atomic load](https://github.com/rust-lang/miri/pull/2424)
* [support vec zero-alloc optimization for tuples and byte arrays](https://github.com/rust-lang/rust/pull/97581)
* [add `PhantomData` marker for dropck to `BTreeMap`](https://github.com/rust-lang/rust/pull/99413)
* [add `[f32]::sort_floats` and `[f64]::sort_floats`](https://github.com/rust-lang/rust/pull/93397)
* [protect `std::io::Take::limit` from overflow in `read`](https://github.com/rust-lang/rust/pull/95040)
* [fix `Skip::next` for non-fused inner iterators](https://github.com/rust-lang/rust/pull/99434)
* [stabilize `core::task::ready!`](https://github.com/rust-lang/rust/pull/99419)
* [add `IntoFuture::into_future` desugaring](https://github.com/rust-lang/reference/pull/1233)
* [futures: inline `WakerRef` functions](https://github.com/rust-lang/futures-rs/pull/2626)
* [cargo-miri: support nextest](https://github.com/rust-lang/miri/pull/2398)
* [cargo-miri: reorder `--target` to after the user-defined commands](https://github.com/rust-lang/miri/pull/2402)
* [cargo: stabilize workspace Inheritance](https://github.com/rust-lang/cargo/pull/10859)
* [docs.rs: add canonical URL for rustdoc pages](https://github.com/rust-lang/docs.rs/pull/1773)
* [clippy: add `arithmetic` lint](https://github.com/rust-lang/rust-clippy/pull/9130)
* [clippy: add `assertions_on_result_states` lint](https://github.com/rust-lang/rust-clippy/pull/9225)
* [clippy: check `assign_op_pattern` for conflicting borrows](https://github.com/rust-lang/rust-clippy/pull/9214)
* [clippy: check for `todo!` on every expression in `SpanlessEq`](https://github.com/rust-lang/rust-clippy/pull/9207)
* [clippy: check macro statements in `non_copy_const`](https://github.com/rust-lang/rust-clippy/pull/9246)
* [clippy: don't lint `std_instead_of_core` on `std::env`](https://github.com/rust-lang/rust-clippy/pull/9243)
* [clippy: fix ICE in `miri_to_const`](https://github.com/rust-lang/rust-clippy/pull/9241)
* [clippy: fix ICE in `question_mark`](https://github.com/rust-lang/rust-clippy/pull/9244)
* [clippy: fix `useless_format` spans for `format!("{foo}")`](https://github.com/rust-lang/rust-clippy/pull/9237)
* [clippy: fix suggestion causing error for `needless_borrow` function in field](https://github.com/rust-lang/rust-clippy/pull/9210)
* [rust-analyzer: find original as node before compute ref match](https://github.com/rust-lang/rust-analyzer/pull/12800)
* [rust-analyzer: fix missing fields check on destructuring assignment](https://github.com/rust-lang/rust-analyzer/pull/12863)
* [rust-analyzer: fix: autocomplete for struct fields includes receiver](https://github.com/rust-lang/rust-analyzer/pull/12861)
* [rust-analyzer: fix: don't add braces to 'if' completion in match guard position](https://github.com/rust-lang/rust-analyzer/pull/12851)
* [rust-analyzer: fix error tooltip message for VSCode status bar item](https://github.com/rust-lang/rust-analyzer/pull/12850)
* [rust-analyzer: fix restart server duplicating language clients](https://github.com/rust-lang/rust-analyzer/pull/12847)
* [rust-analyzer: fix: work around Code bug with empty diagnostics](https://github.com/rust-lang/rust-analyzer/pull/12809)
* [rust-analyzer: don't replace default members' body](https://github.com/rust-lang/rust-analyzer/pull/12832)
* [rustc-perf: enable measuring stable compiler builds](https://github.com/rust-lang/rustc-perf/pull/1341)

### Rust Compiler Performance Triage


Overall it was a mostly good week, with some very significant wins among the
secondary benchmarks. Rollups continue to complicate triage process.

Triage done by **@pnkfelix**.
Revision range: [8bd12e8c..50166d5e](https://perf.rust-lang.org/?start=8bd12e8cca3f28f302b9cc0f1f47bb64bd1f98fd&end=50166d5e5e82ca795306824decbe4ffabcc23d3d&absolute=false&stat=instructions%3Au)


**Summary**:

|            | mean | max | count |
|:----------:|:----:|:---:|:-----:|
| Regressions 😿 <br /> (primary) | N/A  | N/A | 0     |
| Regressions 😿 <br /> (secondary) | 2.2% | 3.2% | 6     |
| Improvements 🎉 <br /> (primary) | -1.8% | -21.2% | 199   |
| Improvements 🎉 <br /> (secondary) | -2.6% | -9.0% | 124   |
| All 😿🎉 (primary) | -1.8% | -21.2% | 199   |


5 Regressions, 4 Improvements, 4 Mixed; 4 of them in rollups
61 artifact comparisons made in total  

Full report [here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-07-27.md)

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

* [RFC: resolve crates.io source replacement ambiguity](https://github.com/rust-lang/rfcs/pull/3289)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Tracking Issue for ptr_const_cast](https://github.com/rust-lang/rust/issues/92675)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Improve C types for cross-language LLVM CFI support](https://github.com/rust-lang/rfcs/pull/3296)
* [new] [RFC: Postfix match](https://github.com/rust-lang/rfcs/pull/3295)

## Upcoming Events

Rusty Events between 2022-07-27 - 2022-08-24 🦀

### Virtual

* 2022-07-27 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Using Rust to read the Little Schemer**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121637/)
* 2022-07-28 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 24th Edition**](https://www.meetup.com/Rust-Linz/events/287238072/)
* 2022-07-29 | Virtual (Minneapolis, MN, US) | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286993342/)
* 2022-07-31 | Virtual (Seattle, WA, US) | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)
* 2022-08-02 | Virtual (Berlin, DE) | [Berline.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/08/02/rust-hack-and-learn.html)
* 2022-08-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**August Meetup: Using Rust on AWS Lambda**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydclbdb/)
* 2022-08-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)
* 2022-08-05 | Virtual + Portland, OR, US | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydclbmb/)
* 2022-08-10 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydclbnb/)
* 2022-08-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydclbpb/)
* 2022-08-12 | Virtual + Tokyo, JP | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://www.reddit.com/r/rust/comments/w7bktx/2022_tokyo_and_elsewhere_rust_game_hack_event_aug/)
* 2022-08-13 | Virtual | [Rust Gamedev](https://gamedev.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://www.google.com/url?q=https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2Eop9Blil9YUWeTq472NIi)
* 2022-08-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydclbvb/)
* 2022-08-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)
* 2022-08-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydclbxb/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)

### Asia

* 2022-08-12 | Tokyo, JP + Virtual | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://bombercrab-rust-game-hack.peatix.com/view)

### Europe

* 2022-07-27 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/287019877/)
* 2022-07-28 | Copenhagen, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #27**](https://www.meetup.com/copenhagen-rust-meetup-group/events/287057762/)
* 2022-07-28 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Rust & Relax @ Brygg w/Jon Gjengset**](https://www.meetup.com/rust-oslo/events/287422914/)
* 2022-08-02 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust London Code Dojo: Rust with Front-End Web Assembly**](https://www.meetup.com/rust-london-user-group/events/287271789/)

### North America

* 2022-07-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/287009519/)
* 2022-08-05 | Portland, OR, US + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-06 | Portland, OR, US | [Rust Project Teams](https://www.rust-lang.org/governance)
    * [**RustConf 2022 PostConf Unconf**](https://www.eventbrite.com/e/rustconf-postconf-unconf-registration-373057423797) | [**Blog post**](https://blog.rust-lang.org/2022/06/28/rust-unconference.html)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)
* 2022-08-11 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydclbpb/)
* 2022-08-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydclbvb/)
* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)

### Oceania

* 2022-07-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**July Meetup: Reflections on using Rust for Data Analysis (Libor Spacek)**](https://www.meetup.com/rust-brisbane/events/286889804/)
* 2022-08-24 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)

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

> # JuSt Be CaReFuL
>
> If there’s one lesson from decades of software engineering, it is the failure of “just be careful” as a strategy. C/C++ programmers still experience memory corruption constantly, no matter how careful they are. Java programmers still frequently see NullPointerExceptions, no matter how careful they are. And so on. One of the reasons that Rust is so successful is that it adds automated checks to prevent many common mistakes.

– [Robert Grosse on their blog](https://blog.polybdenum.com/2022/07/24/fixing-the-next-thousand-deadlocks-why-buffered-streams-are-broken-and-how-to-make-them-safer.html)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1274) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/w9y6jp/this_week_in_rust_453/)</small>
