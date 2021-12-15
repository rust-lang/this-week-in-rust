Title: This Week in Rust 421
Number: 421
Date: 2021-12-15
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

* [Launching the 2021 State of Rust Survey](https://blog.rust-lang.org/2021/12/08/survey-launch.html)

### Foundation

* [Member Spotlight: Automata](https://foundation.rust-lang.org/posts/2021-12-13-member-spotlight-automata/)

### Newsletters

* [WebAssembly Weekly](https://wasmweekly.news/issue-161/)
* [This Month in Rust GameDev #28 - November 2021](https://gamedev.rs/news/028/)

### Project/Tooling Updates

* [Rust Analyzer Changelog #107](https://rust-analyzer.github.io/thisweek/2021/12/13/changelog-107.html)
* [Cranelift Progress Report: A Look Back at 2021](https://bytecodealliance.org/articles/cranelift-progress-2021)
* [Rust for Linux](https://lore.kernel.org/lkml/20211206140313.5653-1-ojeda@kernel.org/)
* [Announcing rsadsb v0.4.0: View Airplanes in the sky with Rust](https://rsadsb.github.io/v0.4.0.html)
* [SixtyFPS (GUI crate): Changelog for 12th of December 2021](https://sixtyfps.io/thisweek/2021-12-13.html)
* [sysinfo: version 0.22 and FreeBSD support](https://blog.guillaume-gomez.fr/articles/2021-12-14+sysinfo%3A+version+0.22+and+FreeBSD+support)
* [Announcing the Grafana Plugin SDK for Rust](https://www.reddit.com/r/rust/comments/rbvmib/announcing_the_grafana_plugin_sdk_for_rust/)

### Observations/Thoughts

* [Rust as a platform for IoT](https://blog.ysndr.de/posts/essays/2021-12-12-rust-for-iot/)
* [https://madsravn.dk/posts/using-liquid-rust-with-serde](https://madsravn.dk/posts/using-liquid-rust-with-serde)
* [Rust Error Handling](https://www.unwoundstack.com/blog/rust-error-handling.html)
* [audio] [Refactoring to Rust with Lily Mara](https://rustacean-station.org/episode/049-lily-mara/)
* [video] [Talking about the Rust Programming Language with Luca Palmieri](https://www.youtube.com/watch?v=WaTEjSHFMbY)

### Rust Walkthroughs

* [A brutally effective hash function in Rust](https://nnethercote.github.io/2021/12/08/a-brutally-effective-hash-function-in-rust.html)
* [Less Painful Linear Types](https://aidancully.blogspot.com/2021/12/less-painful-linear-types.html)
* [A Rust Api pattern (Actix)](https://ctprods.cyprientaque.com/blog/a-rust-api-pattern-actix)
* [Authoring a SIMD enhanced Wasm library with Rust](https://nickb.dev/blog/authoring-a-simd-enhanced-wasm-library-with-rust)
* [Getting better insights into your Rust applications](https://21-lessons.com/getting-better-insights-into-your-rust-applications/)
* [Validating JSON input in Rust web services](https://vinted.engineering/2021/02/15/validating-json-input-in-rust-web-services/)
* [video] [Building a networked Web and Native app with Rust](https://www.youtube.com/watch?v=yIkUWT4QXCA)
* [series] [video] [rg3d - live game development #4](https://www.youtube.com/watch?v=FGi8evJFdnw)

### Miscellaneous

* [The DevX Initiative Sponsorship Program: Goals and Principles](https://medium.com/concordium/the-devx-initiative-sponsorship-program-goals-and-principles-e640063eeaa7)
* [Introducing the new Relay compiler](https://relay.dev/blog/2021/12/08/introducing-the-new-relay-compiler/)
* [DE] [Linux-Kernel: Rust-Entwicklung schreitet mit neuer Edition voran](https://www.heise.de/news/Linux-Kernel-Rust-Entwicklung-schreitet-mit-neuer-Edition-voran-6287775.html)

## Crate of the Week

This week's crate is [efg](https://crates.io/crates/efg), a proc macro to allow boolean expression like syntax for `#[cfg]`s.

Thanks to [farnbams](https://users.rust-lang.org/t/crate-of-the-week/2704/991) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

315 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-12-06..2021-12-13

* [asm: allow using r9 (ARM) and x18 (AArch64) if they are not reserved by the current target](https://github.com/rust-lang/rust/pull/91643)
* [suggest using a temporary variable to fix borrowck errors](https://github.com/rust-lang/rust/pull/83174)
* [tweak "call this function" suggestion to have smaller span](https://github.com/rust-lang/rust/pull/91503)
* [tweak assoc type obligation spans](https://github.com/rust-lang/rust/pull/91769)
* [better span for unexpected normalization failure in CTFE engine](https://github.com/rust-lang/rust/pull/91815)
* [give more help in the unaligned_references lint](https://github.com/rust-lang/rust/pull/91718)
* [suggest casting between `i`/`u32` and `char`](https://github.com/rust-lang/rust/pull/91245)
* [add a suggestion if `macro_rules` is misspelled](https://github.com/rust-lang/rust/pull/91337)
* [avoid cloning refcounted types during folding](https://github.com/rust-lang/rust/pull/91353)
* [deduplicate projection sub-obligations](https://github.com/rust-lang/rust/pull/90423)
* [do not ICE when suggesting elided lifetimes on non-existent spans](https://github.com/rust-lang/rust/pull/91764)
* [do not add `;` to expected tokens list when it's wrong](https://github.com/rust-lang/rust/pull/91531)
* [do not attempt to suggest help for overly malformed struct/function call](https://github.com/rust-lang/rust/pull/91634)
* [improve 'cannot contain emoji' error](https://github.com/rust-lang/rust/pull/91476)
* [add `spin_loop` hint for RISC-V architecture](https://github.com/rust-lang/rust/pull/91548)
* [override `Iterator::advance`(`_back`)`_by` for `array::IntoIter`](https://github.com/rust-lang/rust/pull/91512)
* [replace dominators algorithm with simple Lengauer-Tarjan](https://github.com/rust-lang/rust/pull/85013)
* [add `<*{const|mut} T>::{to|from}_bits`](https://github.com/rust-lang/rust/pull/91127)
* [add `array::IntoIter::`{`empty`, `from_raw_parts`}](https://github.com/rust-lang/rust/pull/91341)
* [add `rsplit_array` variants to slices and arrays](https://github.com/rust-lang/rust/pull/91515)
* [make `Option::cloned` `const`](https://github.com/rust-lang/rust/pull/90741)
* [make `(*mut T)::write_bytes` `const`](https://github.com/rust-lang/rust/pull/91824)
* [make `Borrow` and `BorrowMut` impls `const`](https://github.com/rust-lang/rust/pull/90270)
* [make `Unique`s methods `const`](https://github.com/rust-lang/rust/pull/91806)
* [make `intrinsics::write_bytes` `const`](https://github.com/rust-lang/rust/pull/90081)
* [implement `TryFrom<&'_ mut [T]>` for `[T; N]`](https://github.com/rust-lang/rust/pull/91086)
* [implement `core::future::join!`](https://github.com/rust-lang/rust/pull/91645)
* [implement concat_bytes!](https://github.com/rust-lang/rust/pull/87599)
* [provide the `ReadBuf` abstraction](https://github.com/rust-lang/rust/pull/81156)
* [stabilise `feature(const_generics_defaults)`](https://github.com/rust-lang/rust/pull/90207)
* [stabilize `ControlFlow::`{`is_break`, `is_continue`}](https://github.com/rust-lang/rust/pull/91091)
* [stabilize `const_cstr_unchecked`](https://github.com/rust-lang/rust/pull/91855)
* [cargo: improve I/O error message for fingerprint of build script](https://github.com/rust-lang/cargo/pull/10191)
* [rustdoc: show type layout for type aliases](https://github.com/rust-lang/rust/pull/91682)
* [clippy: add new lint to warn when `#[must_use]` attribute should be used on a method](https://github.com/rust-lang/rust-clippy/pull/8071)
* [clippy: fix FP on `question_mark` if returned object is not local](https://github.com/rust-lang/rust-clippy/pull/8080)
* [clippy: fix `blocks_in_if_conditions` false positive](https://github.com/rust-lang/rust-clippy/pull/8100)
* [clippy: fix bad suggestion on `option_if_let_else` when there is complex subpat](https://github.com/rust-lang/rust-clippy/pull/8086)
* [clippy: ignore associated types in traits when considering type complexity](https://github.com/rust-lang/rust-clippy/pull/8030)

### Rust Compiler Performance Triage

A week of mostly rather small changes with many regressions being hard to diagnose. The largest regression comes from the introduction of a new future (desugaring `.await` into a call to `IntoFuture::into_future`). This was deemed acceptable as it only seems to have a negative impact in stress test situations where async/await is used more than one would normally expect. Unfortunately this does mean regressions beat out improvements this week.

Triage done by **@rylev**.
Revision range: [1c0287..ecb65b](https://perf.rust-lang.org/?start=1c0287830e0fb3c4007afea2819ba03766da6e9c&end=ecb65b0e170fc5275870c9d0fba7267a57cf35e5&absolute=false&stat=instructions%3Au)

3 Regressions, 5 Improvements, 5 Mixed; 3 of them in rollups
31 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-12-07.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Static async fn in traits](https://github.com/rust-lang/rfcs/commit/69833de7b46a571c6110c9ffc0796aeabde335ff)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered final comment period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Relax priv-in-pub lint on generic bounds and where clauses of trait impls.](https://github.com/rust-lang/rust/pull/90586)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Alignment niches for references types.](https://github.com/rust-lang/rfcs/pull/3204)

## Upcoming Events

Rusty Events between 12/01-12/15 🦀

### Online

* [December 3, 2021 | New York City, NY, US | **Getting started in aerospace with Rust 🦀** | NewSpace NYC](https://www.meetup.com/NewSpace-NYC/events/282320805).
* [December 7, 2021 | Berlin, DE | **Rust Hack and Learn** | Berline.rs, a Berlin-local Rust community](https://berline.rs/2021/12/07/rust-hack-and-learn.html)
* [December 7, 2021 | Buffalo, NY, US | **First Tuesdays: Buffalo Rust User Group** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/281833990/)
* [December 7, 2021 | South Padre Island, TX, US | **Tuesdays - Book #24 - Rust for Rustaceans - Chapter 1** | Los Gatos Reading Group (South Padre Island)](https://www.meetup.com/the-south-padre-island-reading-group/events/281685234/)
* [December 8, 2021 | Los Angeles, CA, US | **Introduction to Photogrammetry with Geordon Worley [Virtual] Dec. 2021** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/281944671)
* [December 8, 2021 | Los Angeles, CA, US | **Rust Computer Vision Project with Geordon Worley** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/281944671/)
* [December 8, 2021 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282009864)
* [December 9, 2021 | Columbus, OH, US | **Second Thursdays: Monthly Meetup** | Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccqbmb/)
* [December 9, 2021 | London, UK | **Rust London x TrueLayer Takeover & Christmas Party** | Rust London User Group](https://www.meetup.com/Rust-London-User-Group/events/282343516/)
* [December 9, 2021 | Oslo, NO | **Rust Oslo @ Rebel w/Carol (Nichols || Goulding)**| Rust Oslo](https://www.meetup.com/Rust-Oslo/events/281877640/)
* [December 9, 2021 | San Diego, CA, US | **San Diego Rust December 2021 Tele-Meetup** | San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/282433355)
* [December 10, 2021 | Chicago, IL, US | **8LU Keynote by Eric Smith - Write Your Game in Rust!** | 8th Light University](https://www.meetup.com/8th-light-university/events/282381279)
* [December 11, 2021 | Nairobi, KE | **Rust Catchup 2021** | Rust Nairobi](https://www.meetup.com/Rust-Nairobi/events/282377713/)
* [December 13, 2021 | Eugene, OR, US | **Mondays - Book-Club - Rust Programming Language** | Functional Programming - Eugene](https://www.meetup.com/fp-eug/events/282533185)
* [December 14, 2021 | South Padre Island, TX, US | **Tuesdays - Book #24 - Rust for Rustaceans - Chapter 1 (session 2)** | Los Gatos Reading Group (South Padre Island)](https://www.meetup.com/the-south-padre-island-reading-group/events/282555459/)
* [December 14, 2021 | Seattle, WA, US | **Monthly Meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/281875277)

### North America

* [December 14, 2021 | Minneapolis, MN, US | **First Meetup** | Minneapolis Rust Meetup Group](https://www.meetup.com/minneapolis-rust-meetup-group/events/282526064/)
* [December 14, 2021 | New York, NY, US | **Presentation by Chelsea E. Manning: Rust on Arduino-style microcontrollers** | Rust NYC](https://www.meetup.com/Rust-NYC/events/282478738/)


### Europe

* [December 3, 2021 | Moscow, RU | **Rust Con** | RustCon.ru](https://rustcon.ru)
* [December 9, 2021 | London, UK | **Rust London x TrueLayer Takeover & Christmas Party** | Rust London User Group](https://www.meetup.com/Rust-London-User-Group/events/282343516/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is safer than you may think, because those who need async tend to know it themselves and
> don't ask "should I use async" question. In other words, asking itself is a signal that answer is
> no. MITM proxy case was a rare exception.

– [Seo Sanghyeon on rust-users](https://users.rust-lang.org/t/examples-of-high-performance-rust-multi-thread-network-app-w-o-async/68513/4)

Thanks to [Zeroexcuses](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1146) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
