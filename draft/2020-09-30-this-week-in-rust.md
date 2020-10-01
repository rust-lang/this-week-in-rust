Title: This Week in Rust 358
Number: 358
Date: 2020-09-30
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official
* [Inside] [Announcing the Portable SIMD Project Group](https://blog.rust-lang.org/inside-rust/2020/09/29/Portable-SIMD-PG.html)

### Tooling
* [Rust Analyzer Changelog #44](https://rust-analyzer.github.io/thisweek/2020/09/28/changelog-44.html)
* [Knurling-rs Changelog #2](https://ferrous-systems.com/blog/knurling-changelog-2/)

### Observations/Thoughts
* [Benchmarking vol. 2: Pitting Actix against Rocket v0.4 and v0.5-dev](https://matej.laitl.cz/bench-actix-rocket/)
* [Optimization - Making Rust Code Go Brrrr](https://aspenuwu.me/posts/rust-optimization.html)
* [Revisiting a 'smaller Rust'](https://without.boats/blog/revisiting-a-smaller-rust/)
* [A Future for Rust Debugging](https://nbaksalyar.github.io/2020/05/19/rust-debug.html)
* [Learning Rust the Open Source Way](https://bitsbybrad.com/2020-09-29-learning-rust/)
* [So you want to live-reload Rust](https://fasterthanli.me/articles/so-you-want-to-live-reload-rust)
* [Drop order in Rust: It's tricky](https://vojtechkral.github.io/blag/rust-drop-order/)

### Learn Simple Rust
* [What’s new in Rust 1.46.0 about const_fn?](https://blog.knoldus.com/whats-new-in-rust-1-46-0/)
* [Rust: expression vs statement](https://dev.to/nickymeuleman/rust-expression-vs-statement-2h7g)
* [To Box or not to Box -- My First Real Rust Refactor](https://medium.com/@KevinHoffman/to-box-or-not-to-box-my-first-real-rust-refactor-db467119c4c7)
* [Bevy #2: Space Shooter - The Player](https://dev.to/ethanyidong/bevy-2-space-shooter-the-player-3e7d)
* [Cucumber in Rust - Beginner's Tutorial](https://www.florianreinhard.de/2020-08-16/cucumber-in-rust-beginners-tutorial/)
* [Data Types in Rust](https://edfloreshz.blog/data-types-in-rust)
* [video] [Rust 101](https://www.youtube.com/watch?v=wNzXj4eddDo&feature=youtu.be)

### Learn More Rust
* [Are we observable yet? An introduction to Rust telemetry - Zero To Production #4](https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/)
* [A Fistful of States: More State Machine Patterns in Rust](https://deislabs.io/posts/a-fistful-of-states/)
* [OS in Rust: An executable that runs on bare metal: Part-1](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-1/)
* [OS in Rust: An executable that runs on bare metal: Part-2](https://blog.knoldus.com/os-in-rust-an-executable-that-runs-on-bare-metal-part-2/)
* [Build a Discord Bot with Rust and Serenity](https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/)
* [Porting EBU R128 audio loudness analysis from C to Rust – Porting Details](https://coaxion.net/blog/2020/09/porting-ebu-r128-audio-loudness-analysis-from-c-to-rust-porting-details/)
* [Building a runtime reflection system for Rust 🦀️ (Part 1)](https://www.osohq.com/post/rust-reflection-pt-1)
* [Rusty-PID: Porting the TSic sensor from C to Rust](https://nitschinger.at/Rusty-PID-Porting-the-TSic-sensor-from-C-to-Rust/)
* [Playing Codenames in Rust with word vectors](https://rolisz.ro/2020/09/26/playing-codenames-in-rust-with-word-vectors/)
* [Type-level Programming in Rust](https://willcrichton.net/notes/type-level-programming/)
* [Multi-threading with Wgpu and Rayon](https://sotrh.github.io/learn-wgpu/intermediate/tutorial13-threading/#threading-build-rs)
* [Flicker free fireworks (or how I accidentally rediscovered the regen buffer)](https://blog.darrien.dev/posts/flicker-free-fireworks/)
* [audio] [The Rustacean Station Podcast - WebAssembly on the Server with Krustlet](https://rustacean-station.org/episode/030-krustlet/)
* [video] [RIOT Summit 2020 - Safer & Simpler Embedded Programs with Rust on RIOT](https://www.youtube.com/watch?v=LvfCSnOM1Hs&feature=youtu.be)
* [video] [One Day Build - Anachro SPI pt 2](https://www.youtube.com/watch?time_continue=69&v=2vk9H9-O_pI&feature=emb_logo)
* [video] [Rusty USB Gadgets Make Barking BeagleBones](https://www.youtube.com/watch?v=NgdWeR1CvI8&feature=emb_logo)

### Project Updates
* [Krustlet v0.5.0 Release](https://github.com/deislabs/krustlet/releases/tag/v0.5.0)
* [oso, an open-source policy engine for authorization written in Rust](https://github.com/osohq/oso), added [a Rust library for implementing authorization for Rust projects!](https://docs.rs/oso/0.6.0-alpha/oso/)
* [Rust on the ESP32 & ESP8266 - Building an ecosystem](https://mabez.dev/blog/posts/esp-rust-ecosystem/)
* [What (not so) recently happened in Miri](https://www.ralfj.de/blog/2020/09/28/miri.html)
* [Progress report on rustc_codegen_cranelift (Sep 2020)](https://bjorn3.github.io/2020/09/28/progress-report-sep-2020.html)

### Miscellaneous
* [Supercharge your Electron apps with Rust](https://blog.logrocket.com/supercharge-your-electron-apps-with-rust/)
* [How to Make a 💡?](https://rust-analyzer.github.io/blog/2020/09/28/how-to-make-a-light-bulb.html)
* [Towards principled reactive UI](https://raphlinus.github.io/rust/druid/2020/09/25/principled-reactive-ui.html)
* [Building even faster interpreters in Rust](https://blog.cloudflare.com/building-even-faster-interpreters-in-rust/)
* [AMD Is Hiring To Work On New Radeon Driver Tooling Written In Rust](https://www.phoronix.com/scan.php?page=news_item&px=AMD-Hiring-Radeon-Rust)
* [Rust programming language exploit mitigations](https://rcvalle.blog/2020/09/16/rust-lang-exploit-mitigations/)
* [Custom Literals in Rust](https://www.5snb.club/posts/2020/09/25/custom-literals-in-rust.html)

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

* [Rust 2021: GUI](https://raphlinus.github.io/rust/druid/2020/09/28/rust-2021.html)
* [Rust 2021: maintain dominance on the web, easy funding, easier learning](https://www.reddit.com/r/rust/comments/j1ihax/rust_2021_maintain_dominance_on_the_web_easy/)
* [Rust 2021: Easier to integrate into existing codebases](https://www.reddit.com/r/rust/comments/j06zc9/rust_2021_easier_to_integrate_into_existing/)
* [Rust 2021: Continue](https://www.reddit.com/r/rust/comments/izk75l/rust_2021_continue/)
* [My Rust 2021 roadmap](https://raphlinus.github.io/rust/druid/2020/09/28/rust-2021.html)
* [Rust 2021 | Features and Trait Bounds in Macros](https://blog.carminecrystal.com/rust-2021-features-and-trait-bounds-in-macros.html)
* [Rust 2021: Quality, frictionless tooling](https://swatinem.de/blog/rust-2021/)
* [My Rust 2021 roadmap: crates, concision, and community](http://jamesmcm.github.io/blog/2020/09/24/rust2021/#en)
* [Rust 2021 Roadmap Wishlist](https://aldaronlau.com/rust-2021/)
* [Rust in 2021: We've done a lot, more to be done](https://popzxc.github.io/rust-2021)
* [Rust in 2021: Leveraging the Type System for Infallible Message Buffers](https://christian.amsuess.com/blog/website/2020-09-24_rust_2021/)
* [Rust 2021](https://blog.coderspirit.xyz/blog/2020/09/26/rust-2021/)
* [Rust 2021: Minor major improvements](https://blog.thomasheartman.com/posts/rust-2021)
* [Rust 2021](https://beyermatthias.de/blog/2020/09/28/rust-2021/)
* [My Rust 2021](https://blog-dry.com/entry/2020/09/29/135738)

# Crate of the Week

This week's crate is [fs-err](https://crates.io/crates/fs-err), a library to make filesystem errors usable.

Thanks to [Emerentius](https://users.rust-lang.org/t/crate-of-the-week/2704/821) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

* [time-rs: support #![no_std] targets without global allocation](https://github.com/time-rs/time/issues/249)
* [time-rs: OffsetDateTime::replace_time?](https://github.com/time-rs/time/issues/256)

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

370 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-09-21..2020-09-28

* [return values up to 128 bits in registers](https://github.com/rust-lang/rust/pull/76986)
* [add `asm!` support for MIPS](https://github.com/rust-lang/rust/pull/76839)
* [diagnostics: improve closure/generic parameter mismatch](https://github.com/rust-lang/rust/pull/76711)
* [avoiding unnecesary allocations at `rustc_errors`](https://github.com/rust-lang/rust/pull/76846)
* [add fast path for match checking](https://github.com/rust-lang/rust/pull/76918)
* [cache types during normalization](https://github.com/rust-lang/rust/pull/76928)
* [fix the performance regression of #76244](https://github.com/rust-lang/rust/pull/76913)
* [encode less metadata for proc-macro crates](https://github.com/rust-lang/rust/pull/76897)
* [invalidate local LLVM cache less often](https://github.com/rust-lang/rust/pull/77126)
* [introduce a new flag to enable experimental/unsound mir opts](https://github.com/rust-lang/rust/pull/76899)
* [MIR pass to remove unneeded drops on types not needing drop](https://github.com/rust-lang/rust/pull/76673)
* [add optimization to avoid load of address](https://github.com/rust-lang/rust/pull/76683)
* [miri: more informative deallocation error messages](https://github.com/rust-lang/rust/pull/77047)
* [miri: add API for capturing backtrace](https://github.com/rust-lang/miri/pull/1559)
* [`DroplessArena`: allocate objects from the end of memory chunk](https://github.com/rust-lang/rust/pull/77014)
* [unstably allow `assume` intrinsic in const contexts](https://github.com/rust-lang/rust/pull/76973)
* [add `array::from_ref`](https://github.com/rust-lang/rust/pull/77074)
* [add `#[track_caller]` to more panicking `Cell` functions](https://github.com/rust-lang/rust/pull/77055)
* [make some methods of `Pin` unstably const](https://github.com/rust-lang/rust/pull/76655)
* [revert `const_type_id` stabilization](https://github.com/rust-lang/rust/pull/77083)
* [revert adding `Atomic::from_mut`](https://github.com/rust-lang/rust/pull/76967)
* [add `cfg(target_has_atomic_equal_alignment)` and use it for `Atomic::from_mut`](https://github.com/rust-lang/rust/pull/76965)
* [make `[].as_`[`mut_`]`ptr_range()` (unstably) const](https://github.com/rust-lang/rust/pull/77097)
* [log: implement `Log` for `Box<Log>`](https://github.com/rust-lang/log/pull/414)

## Rust Compiler Performance Triage

* [2020-09-28](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-09-28.md):
0 Regressions, 1 Improvements, 3 Mixed


Most significant changes this week came in response to regressions discussed in
last week's triage report. Curious readers may be interested in
[#77058](https://github.com/rust-lang/rust/issues/77058), in which the removal
of a single field from a struct caused a 25% decrease in wall-times for one
seemingly unrelated benchmark, or
[#76986](https://github.com/rust-lang/rust/issues/76986), an ABI change that
should be a pretty clear win but seems to have mixed results.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-09-28.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Destructuring assignment](https://github.com/rust-lang/rfcs/pull/2909)
* [RFC: impl-only glob imports](https://github.com/rust-lang/rfcs/pull/2782)
* [Fetching cargo registry tokens from external processes](https://github.com/rust-lang/rfcs/pull/2730)
* [RFC: Permit _ in type aliases](https://github.com/rust-lang/rfcs/pull/2524)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Stabilize slice_ptr_range.](https://github.com/rust-lang/rust/pull/77111)
* [disposition: merge][Make RawFd implement the RawFd traits](https://github.com/rust-lang/rust/pull/76969)
* [disposition: merge][Fix Debug implementations of some of the HashMap and BTreeMap iterator types](https://github.com/rust-lang/rust/pull/75377)
* [disposition: merge][Tracking issue for slice_partition_at_index](https://github.com/rust-lang/rust/issues/55300)

## New RFCs

* [[RFC] A new stack-based vector](https://github.com/rust-lang/rfcs/pull/2990)
* [RFC: Add `target` configuration](https://github.com/rust-lang/rfcs/pull/2991)
* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [adds async stream rfc](https://github.com/rust-lang/rfcs/pull/2996)

# Upcoming Events

### Online
* [October 1. Berlin, DE - Berline.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbcb/)
* [October 7. Johannesburg, ZA - Johannesburg Rust Meetup - Monthly Joburg Rust Chat!](https://www.meetup.com/Johannesburg-Rust-Meetup/events/273455489/)
* [October 7. Dublin, IE - Rust Dublin - October Remote Meetup](https://www.meetup.com/Rust-Dublin/events/273014329/)
* [October 7. Indianapolis, IN, US - Indy.rs - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybcnbkb/)
* [October 8. Linz, AT - Rust Linz - Rust Meetup Linz](https://www.meetup.com/de-DE/Rust-Linz/events/271857253/)
* [October 8. San Diego, CA, US - San Diego Rust - San Diego Rust October 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/273486967/)
* [October 13. Saabrücken, DE - Rust-Saar Meetup - `4u16`](https://www.meetup.com/Rust-Saar/events/273252813/)
* [October 12 - 18. RustLab](https://www.rustlab.it/agenda)

### Asia Pacific
* [October 4. Auckland, NZ - Rust AKL - Rust meetup](https://www.meetup.com/rust-akl/events/266876708/)

### North America
* [October 8. Lehi, UT - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

* [3D Driver Development Engineer - Rust tooling for GPUs at AMD (Boxborough, MA, USA)](https://jobs.amd.com/job/Boxborough-3D-Driver-Development-Engineer-80489-Mass/677678000/)
* [Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Egineer, Kraken Futures - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Rust Engineer, Desktop GUI - Cryptowatch at Kraken (Remote)](https://jobs.lever.co/kraken/2442ee5c-56b6-4a73-a477-8cdda2b218d5)
* [Senior Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105)
* [Backend / Quant Developer at Kraken (Remote)](https://jobs.lever.co/kraken/9d9cc4b5-ef5f-40bd-b785-9acf9164aa74)
* [Senior Software Engineer (New IDE Platform) at JetBrains (Amsterdam, NL)](https://www.linkedin.com/jobs/view/2151145919/)

# Quote of the Week

> Rust has a curse (it has many, but this one is critical): inefficient code is generally visible. Experienced developers hate to notice that their code is inefficient. They will recoil at seeing `Arc<RefCell<T>>` , but won't bat an eye at using Python.

- [Esteban Kuber on rust-users](https://users.rust-lang.org/t/failed-to-contribute-due-to-difficulty-in-understanding-rust/49148/6)

Thanks to [Jon G Stødle](https://users.rust-lang.org/t/twir-quote-of-the-week/328/945) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/iu3ge0/this_week_in_rust_356/)</small>
