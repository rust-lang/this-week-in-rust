Title: This Week in Rust 423
Number: 423
Date: 2021-12-29
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

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [kajiya](https://github.com/EmbarkStudios/kajiya), an experimental real-time global illumination renderer made with Rust and Vulkan.

llogiq is pretty pleased with himself for this suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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
  
**Parity Technologies**

* [Rust Core Engineer - Solidity Compiler (Solang)](https://grnh.se/a5a5c0a33us)
* [Core Runtime Engineer - Substrate](https://grnh.se/dddd76283us)
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
