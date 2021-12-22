Title: This Week in Rust 422
Number: 422
Date: 2021-12-22
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
* [Follow-up on the moderation issue](https://blog.rust-lang.org/inside-rust/2021/12/17/follow-up-on-the-moderation-issue.html)

### Foundation
* [Take the State of Rust Survey](https://foundation.rust-lang.org/posts/2021-12-15-take-the-state-of-rust-survey/)
* [Member Spotlight: Spectral](https://foundation.rust-lang.org/posts/2021-12-20-member-spotlight-spectral/)

### Project/Tooling Updates
* [Zellij 0.23.0 released with new collaboration features](https://zellij.dev/news/multiplayer-sessions/)
* [SixtyFPS (GUI crate): Changelog for 19th of December 2021](https://sixtyfps.io/thisweek/2021-12-20.html)
* [You can now buy Black Hat Rust with PayPal, Apple Pay & Google Pay](https://kerkour.com/black-hat-rust-paypal-apple-pay-google-pay/)
* [This week in Databend #21: an elastic and reliable cloud warehouse](https://weekly.databend.rs/2021-12-22-databend-weekly/)
* [This week in Fluvio #17: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0017/)
* [Announcing Tokio Console 0.1](https://tokio.rs/blog/2021-12-announcing-tokio-console)
* [Rust Analyzer Changelog #108](https://rust-analyzer.github.io//thisweek/2021/12/20/changelog-108.html)
* [IntelliJ Rust Changelog #162](https://intellij-rust.github.io/2021/12/20/changelog-162.html)
* [SixtyFPS (GUI crate): 13th to 19th of December 2021](https://sixtyfps.io/thisweek/2021-12-20.html)
* [GCC Rust in 2021](https://thephilbert.io/2021/12/20/gcc-rust-in-2021/)

### Observations/Thoughts
* [Contexts and capabilities in Rust](https://tmandry.gitlab.io/blog/posts/2021-12-21-context-capabilities/)
* [Stop Whining about Rust Hype - A Pro-Rust Rant](https://thenewwazoo.github.io/whining.html)
* [Contexts and capabilities in Rust](https://tmandry.gitlab.io/blog/posts/2021-12-21-context-capabilities/)
* [Thread Safety in C++ and Rust](https://blog.reverberate.org/2021/12/18/thread-safety-cpp-rust.html)
* [Rust in Production: Astropad](https://serokell.io/blog/rust-in-production-astropad)
* [Unbuffered I/O Can Make Your Rust Programs Much Slower](https://era.co/blog/unbuffered-io-slows-rust-programs)
* [video] [Safe && Portable Data Structure Design (10 minute lightning talk)](https://www.youtube.com/watch?v=1UtklNrB8XA&t=1619s)

### Rust Walkthroughs
* [Cross-compiling Rust Lambdas on macOS without Docker](https://noserve.rs/rust-lambdas-macos/)
* [A Beginner's Guide to Parsing in Rust](https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/)
* [Building a CEDICT parser in Rust with Nom](https://briankung.dev/2021/12/07/building-a-cedict-parser-in-rust-with-nom/)
* [DE] [Ferris Talk #4: Asynchrone Programmierung in Rust](https://www.heise.de/hintergrund/Ferris-Talk-4-Asynchrone-Programmierung-in-Rust-6299096.html)
* [ZH] [「Pin 三部曲」第二部之 《Rust Pin 进阶](https://folyd.com/blog/rust-pin-advanced/)

### Miscellaneous
* [TypeVille Call for Papers (ends 9th of January 2022)](https://docs.google.com/forms/d/e/1FAIpQLSdzWAX_N7rkJlVza73hZuNDCZIzKtinGJv6OjcdfdOpJ5w6Ww/viewform)
* [Homegrown rendering with Rust](https://medium.com/embarkstudios/homegrown-rendering-with-rust-1e39068e56a7)
* [audio] [SE-Radio: Tim McNamara on Rust 2021 Edition](https://www.se-radio.net/2021/12/episode-490-tim-mcnamara-on-rust-2021-edition/)
* [video] [Web api benchmarking: Rust (Warp) vs Rust (actix-web)](https://www.youtube.com/watch?v=zC-r1jzVwh4)
* [PT] [Olá e seja bem vindo a outra edição de esta semana em: Rust! #420 (08/12/2021). 🌟](https://github.com/luisvonmuller/Esta-Semana-Em-Rust/blob/main/%23420.md)

## Crate of the Week

This week's crate is [kajiya](https://github.com/EmbarkStudios/kajiya), an experimental real-time global illumination renderer made with Rust and Vulkan.

llogiq is pretty pleased with himself for this suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [diesel-rs - Documentation improvements](https://github.com/diesel-rs/diesel/pull/2996)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

340 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-12-13..2021-12-20

* [add user seed to `-Z randomize-layout`](https://github.com/rust-lang/rust/pull/91932)
* [improve suggestion to change struct field to `&mut`](https://github.com/rust-lang/rust/pull/91516)
* [suggest to specify a target triple when lang item is missing](https://github.com/rust-lang/rust/pull/91820)
* [tweak errors coming from `for`-loop, `?` and `.await` desugaring](https://github.com/rust-lang/rust/pull/90939)
* [show the unused type for `unused_results` lint](https://github.com/rust-lang/rust/pull/91818)
* [recover on invalid operators `<>` and `<=>`](https://github.com/rust-lang/rust/pull/91597)
* [perf: manually implement `Hash` for `DefId`](https://github.com/rust-lang/rust/pull/91660)
* [enable `#[thread_local]` for all windows-msvc targets](https://github.com/rust-lang/rust/pull/92042)
* [add entry_ref API to HashMap](https://github.com/rust-lang/hashbrown/pull/301)
* [add `[T]::as_simd`(`_mut`)](https://github.com/rust-lang/rust/pull/91479)
* [add `BinaryHeap::try_reserve` and `BinaryHeap::try_reserve_exact`](https://github.com/rust-lang/rust/pull/91529)
* [add `io::Error::other`](https://github.com/rust-lang/rust/pull/91947)
* [avoid sorting in hash map stable hashing](https://github.com/rust-lang/rust/pull/91837)
* [constify (most) `Option` methods](https://github.com/rust-lang/rust/pull/91928)
* [constify `bool::then`{,`_some`}](https://github.com/rust-lang/rust/pull/91918)
* [make `MaybeUninit::zeroed` `const`](https://github.com/rust-lang/rust/pull/91851)
* [mark defaulted `PartialEq`/`PartialOrd` methods as const](https://github.com/rust-lang/rust/pull/91439)
* [optimize `vec::retain` performance](https://github.com/rust-lang/rust/pull/91527)
* [readd `track_caller` to `Result::from_residual`](https://github.com/rust-lang/rust/pull/91752)
* [stabilize `destructuring_assignment`](https://github.com/rust-lang/rust/pull/90521)
* [stabilize `iter::zip`](https://github.com/rust-lang/rust/pull/91881)
* [stabilize `asm!` and `global_asm!`](https://github.com/rust-lang/rust/pull/91728)
* [remove `P: Unpin` bound on `impl Stream for Pin`](https://github.com/rust-lang/rust/pull/92020)
* [futures: add `StreamExt::count` method](https://github.com/rust-lang/futures-rs/pull/2495)
* [futures: limit `FuturesUnordered` max value of `yield_every`](https://github.com/rust-lang/futures-rs/pull/2527)
* [cargo: detect filesystem loop during walking the projects](https://github.com/rust-lang/cargo/pull/10188)
* [cargo: display alias target on 'cargo help <alias>`](https://github.com/rust-lang/cargo/pull/10193)
* [rustdoc: fix source code page sidebar on mobile](https://github.com/rust-lang/rust/pull/91905)
* [clippy: add `unnecessary_to_owned` lint](https://github.com/rust-lang/rust-clippy/pull/7978)
* [clippy: don't emit `return_self_not_must_use` lint if `Self` already is marked as `#[must_use]`](https://github.com/rust-lang/rust-clippy/pull/8146)
* [clippy: ensure that `return_self_not_must_use` is not emitted if the method already has `#[must_use]`](https://github.com/rust-lang/rust-clippy/pull/8143)
* [clippy: fix `SAFETY` comment tag casing in `undocumented_unsafe_blocks`](https://github.com/rust-lang/rust-clippy/pull/8138)
* [rustfmt: prevent duplicate comma when formatting struct pattern with ".."](https://github.com/rust-lang/rustfmt/pull/5090)

### Rust Compiler Performance Triage

Unfortunately a change introduced in [rust-lang/rust#89836](https://github.com/rust-lang/rust/pull/89836) has made performance across different compiler artifacts much more variable by embedding compiler version information (including a git commit hash) in demangled symbol names. This means that even if two compiler artifacts are built from the same exact source code (with only the git commit changed), the compiler will have slightly different performance characteristics. This makes comparisons across pull requests virtually impossible. 

The compiler team is still deciding what to do to handle this, but in the mean time, performance testing is largely broken. This issue is currently being tracked in[rust-lang/rustc-perf#1126](https://github.com/rust-lang/rustc-perf/issues/1126).

Triage done by **@rylev**.
Revision range: [404c847..3d57c61](https://perf.rust-lang.org/?start=404c8471aba60c2d837fa728e7c729a0f52d5830&end=3d57c61a9e04dcd3df633f41142009d6dcad4399&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 23 Mixed; 9 of them in rollups
38 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-12-21.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Thread local Cell methods.](https://github.com/rust-lang/rfcs/pull/3184)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No new RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Add UnwindSafe to Once](https://github.com/rust-lang/rust/pull/90625)
* [disposition: merge] [Allow reverse iteration of lowercase'd/uppercase'd chars](https://github.com/rust-lang/rust/pull/88858)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No new RFCs were published this week.*

## Upcoming Events

Rusty Events between 12/22/2021 - 1/15/2022 🦀

### Online

* [December 23, 2021 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/ttjjqsyccqbfc/)
* [December 28, 2021 | Dallas, TX, US | **Dallas Rust - Last Tuesday** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccqblc/)
* [January 5, 2022 | Indianapolis, IN, US | **Indy.rs - with Social Distancing** | Indy Rust](https://www.meetup.com/indyrs/events/qwtdjsydccbhb/)
* [January 6, 2022 | Nürnberg, DE | **Rust Nürnberg online #8**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/282344613/)
* [January 8, 2022 | Various cities | **Rust GameDev Monthly Meetup** | Rust GameDev](https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com)
* [January 11, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydccbpb/)
* [January 12, 2022 | Boulder, CO, US | **Monthly Meetup** | Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydccbqb/)
* [January 12, 2022 | Los Angeles, CA, US | **Live Coding Session - Mob Programming a Rust Code Kata [Virtual] Jan. 2022** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/282580016/)
* [January 12, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/gjrtqsydccbqb/)


### North America
* [January 6, 2022 | Austin, TX, US | **Rust Lunch** | Rust ATX](https://www.meetup.com/rust-atx/events/282756864/)
* [January 12, 2022 | Atlanta, GA, US | **Grab a beer with fellow Rustaceans** | Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsydccbqb/)
* [January 13, 2022 | Columbus, OH, US | **Monthly Meeting** | Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrydccbrb/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Astropad**

* [Software Engineering Manager (Remote North America time zones)](https://astropad.com/software-engineering-manager/)

**Timescale**

* [Senior Rust/C/C++ Engineer, Database Toolkit (Remote (UTC-8 to UTC-5)](https://boards.greenhouse.io/timescale/jobs/5542785002)

**Tangam**

* [Rust Developer (Remote)](https://www.tangram.dev/jobs)

**Kraken**

* [Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Senior Rust Engineer - Banking (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)

**Parity Technologies**

* [Rust Core Engineer - Solidity Compiler (Solang) (Remote)](https://grnh.se/a5a5c0a33us)
* [Rust Core Engineer - Smart Contract Platform (Remote)](https://grnh.se/cb272e3b3us)
* [Multiple other Rust / Blockchain Engineering positions](https://www.parity.io/jobs)


*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Important crab-related diagnostics improvement shipping in nightly
> [@rustlang](https://twitter.com/rustlang)
>
> ```
> error: Ferris cannot be used as an identifier
> --> src/main.rs:2:9
>   |
> 2 |     let 🦀 = 123;
>   |         ^^ help: try using their name instead: `ferris`
> 3 |
> 4 |     for i in 0..🦀 {
>   |
> ```

– [Mara Bos on twitter](https://twitter.com/m_ou_se/status/1471077145258647554)

Thanks to [Julian Wollersberger](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1147) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
