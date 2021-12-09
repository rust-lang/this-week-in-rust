Title: This Week in Rust 420
Number: 420
Date: 2021-12-08
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
* [Announcing Rust 1.57.0](https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html)

### Foundation
* [Love for Rust at re:Invent](https://foundation.rust-lang.org/posts/2021-12-06-love-for-rust-at-reinvent/)

### Project/Tooling Updates
* [Updates in IntelliJ Rust for 2021.3](https://blog.jetbrains.com/rust/2021/12/06/updates-in-intellij-rust-for-2021-3/)
* [Rust Analyzer Changelog #106](https://rust-analyzer.github.io/thisweek/2021/12/06/changelog-106.html)
* [cloud-hypervisor v20.0](https://github.com/cloud-hypervisor/cloud-hypervisor/releases/tag/v20.0)
* [SixtyFPS (GUI crate): Changelog for 5th of December 2021](https://sixtyfps.io/thisweek/2021-12-06.html)
* [This Month in Rust OSDev (November 2021)](https://rust-osdev.com/this-month/2021-11/)
* [GCC Rust Monthly Report #12 November 2021](https://thephilbert.io/2021/12/06/gcc-rust-monthly-report-12-november-2021/)
* [rustc_codegen_gcc: Progress Report #6](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-6)
* [Arti 0.0.2 is released: API groundwork, refactoring, config, and optimism](https://forum.torproject.net/t/arti-0-0-2-is-released-api-groundwork-refactoring-config-and-optimism/939)
* [Rust takes a major step forward as Linux's second official language](https://www.zdnet.com/article/rust-takes-a-major-step-forward-as-linuxs-second-official-language/)
* [Updated Rust Code For Linux Kernel Patches Posted](https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-v2)
* [xmake v2.6.1 released, Switch to Lua5.4 runtime, Support Rust and Cplusplus mixed compilation](https://github.com/xmake-io/xmake/wiki/xmake-v2.6.1-released,-Switch-to-Lua5.4-runtime,-Support-Rust-and-Cplusplus-mixed-compilation)
* [Announcing Sycamore v0.7.0: Client-side hydration + Builder API](https://sycamore-rs.netlify.app/news/announcing-v0.7.0)
* [Nushell 0.41](https://www.nushell.sh/blog/2021-12-07-nushell_0_41.html)
* [DE] [WebAssembly: Wasmer 2.1 ermöglicht das Ausführen von Rust-Projekten im Browser](https://www.heise.de/news/WebAssembly-Wasmer-2-1-ermoeglicht-das-Ausfuehren-von-Rust-Projekten-im-Browser-6283513.html)
* [DE] [Programmiersprache: Rust 1.57 – Don't Panic!](https://www.heise.de/news/Programmiersprache-Rust-1-57-Don-t-Panic-6283665.html)

### Observations/Thoughts
* [Uninit read/write](https://blog.yoshuawuyts.com/uninit-read-write/)
* [Portable and interoperable async Rust](https://www.ncameron.org/blog/portable-and-interoperable-async-rust/)
* [Speedrunning GUI development in Rust](https://aaronerhardt.github.io/blog/posts/gui_speedrun/)
* [My Cryptography Final Project](https://codegito.xyz/2021/12/05/cryptography-final-project/)
* [Case study: Rust programming language community](https://zulip.com/case-studies/rust/)
* [Linear Types Can Help](https://aidancully.blogspot.com/2021/12/linear-types-can-help.html)
* [video] [On Hubris and Humility: developing an OS for robustness in Rust](https://talks.osfc.io/osfc2021/talk/JTWYEH/)

### Rust Walkthroughs
* [How to encrypt a file in Rust (Using streaming AEAD encryption)](https://kerkour.com/rust-file-encryption)
* [Getting Started with Rust on a Raspberry Pi Pico (Part 3)](https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry-a88)
* [A better cargo-readme - Issue 0: Humble Beginning](https://scrabsha.github.io/abcr-issue-0/)
* [How to Overriding (Patch) Cargo Dependencies](https://edger.substack.com/p/how-to-overriding-patch-cargo-dependencies)
* [Higher Kinded Types in Rust](https://hugopeters.me/posts/14/)
* [Node to Rust: Day 1 - From nvm to rustup](https://vino.dev/blog/node-to-rust-day-1-rustup/)
* [Creating a Web Page with Actix-Web (Rust)](https://dev.to/michaelin007/creating-a-web-page-with-actix-web-rust--2agd)
* [video] [rg3d - live game development #2](https://www.youtube.com/watch?v=TQaCyC_tGko)
* [video] [1Password Developer Fireside Chat: Ownership & Mutability Patterns in Rust](https://www.youtube.com/watch?v=hJ9IO-nYpjs)
* [video] [series] [Advent of Code in Rust by Lucille Blumire](https://www.youtube.com/channel/UCevTibyeBGT1ybiGzROnsfw)
* [video] [series] [Solving Advent of Code 2021 in Rust by Yoshua Wuyts](https://www.youtube.com/playlist?list=PL2F_NKy2ueKOpAVPl-c3szUXuwB7K9sDq)
* [video] [series] [Writing a Programming Language (in Rust) 9: Implementing Value References](https://www.youtube.com/watch?v=pgD7m02-XnI)

### Miscellaneous
* [Live Coder Jon Gjengset Gets into the Nitty-Gritty of Rust](https://nostarch.com/blog/jon-gjengset-talks-rust)
* [AWS SDK for Rust (Developer Preview)](https://aws.amazon.com/about-aws/whats-new/2021/12/aws-sdk-rust-developer-preview/)
* [BR-pt] [Esta Semana em Rust #419](https://github.com/luisvonmuller/Esta-Semana-Em-Rust/blob/main/%23419.md)

## Crate of the Week

This week's crate is [tap](https://crates.io/crates/tap), a library with extension traits to provide suffix-position pipeline behavior.

Thanks to [David Mason](https://users.rust-lang.org/t/crate-of-the-week/2704/988) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

286 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-11-29..2021-12-06

* [Optimize `rustc_lexer`](https://github.com/rust-lang/rust/pull/91393)
* [Add support for LLVM coverage mapping format versions 5 and 6](https://github.com/rust-lang/rust/pull/91207)
* [Add support for riscv64gc-unknown-freebsd](https://github.com/rust-lang/rust/pull/91284)
* [Fix ICE in `check_must_not_suspend_ty()`](https://github.com/rust-lang/rust/pull/91367)
* [Fix ICE when `yield`ing in function returning `impl Trait`](https://github.com/rust-lang/rust/pull/91488)
* [Don't suggest types whose inner type is erroneous](https://github.com/rust-lang/rust/pull/91450)
* [Only show notable traits if both types are the same](https://github.com/rust-lang/rust/pull/91366)
* [Improve diagnostic for missing half of binary operator in `if` condition](https://github.com/rust-lang/rust/pull/91435)
* [Improve error message for `E0659` if the source is not available](https://github.com/rust-lang/rust/pull/91298)
* [Improve error message for incorrect field accesses through raw pointers](https://github.com/rust-lang/rust/pull/91364)
* [Add `Option::inspect` and `Result::{inspect, inspect_err}`](https://github.com/rust-lang/rust/pull/91346)
* [Add a `try_reduce` method to the `Iterator` trait](https://github.com/rust-lang/rust/pull/87054)
* [Add slice `take` methods](https://github.com/rust-lang/rust/pull/88502)
* [Make `array::`{`try_from_fn`, `try_map`} and `Iterator::try_find` generic over `Try`](https://github.com/rust-lang/rust/pull/91286)
* [Introduce `RawVec::reserve_for_push`](https://github.com/rust-lang/rust/pull/91352)
* [Implement `VecDeque::retain_mut`](https://github.com/rust-lang/rust/pull/91215)
* [libc: Define `max_align_t` for wasi](https://github.com/rust-lang/libc/pull/2577)
* [portable-simd: Generic `core::ops` for `Simd<T, _>`](https://github.com/rust-lang/portable-simd/pull/195)
* [cargo: Stabilize `future-incompat-report`](https://github.com/rust-lang/cargo/pull/10165)
* [cargo: Support abbreviating `--release` as `-r`](https://github.com/rust-lang/cargo/pull/10133)
* [clippy: Consider `NonNull` as a pointer type](https://github.com/rust-lang/rust-clippy/pull/8074)
* [clippy: Escape backslash in `single_char_pattern.rs`](https://github.com/rust-lang/rust-clippy/pull/8067)
* [clippy: Fix `any()` not taking reference in `search_is_some` lint](https://github.com/rust-lang/rust-clippy/pull/7463)
* [clippy: Fix some false negatives for `single_char_pattern`](https://github.com/rust-lang/rust-clippy/pull/8077)
* [clippy: Parenthesize blocks in `needless_bool` suggestion](https://github.com/rust-lang/rust-clippy/pull/8066)
* [clippy: Upgrade `map_flatten` to complexity](https://github.com/rust-lang/rust-clippy/pull/8054)
* [rustfmt: Determine when new comment lines are needed for itemized blocks](https://github.com/rust-lang/rustfmt/pull/5097)

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

**DeepSource**

* [Software Engineer, Static Analysis - Rust (Bangalore, India)](https://careers.deepsource.io/o/software-engineer-static-analysis-rust)

**Clear**

* [Full Stack Developer (Remote)](https://docs.google.com/document/d/1OuG5Ts_6s4eWO6CXGzcbklOweD7qGnOgADnSoPjEa10/edit)

**Pixy**

* [Rust Developer Position](https://www.bigeyestudios.com/job-board)

**Bionaut Labs**

* [Embedded Software Engineer (Senior) (Los Angeles, CA, US)](https://www.indeed.com/viewjob?jk=775772a2587b4a1e)
* [Embedded Software Engineer (Junior) (Los Angeles, CA, US)](https://www.indeed.com/viewjob?jk=030a274941d1f7ac)

**Metawork**

* [Infrastructure Engineer (Remote US)](https://jobs.ashbyhq.com/metawork/51d61a87-d4b8-4ed2-a49b-abb4b2247b30)
* [Platform Engineer (Remote US)](https://jobs.ashbyhq.com/metawork/90575f85-de36-461e-a540-fbee126ad186)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> v2 of the patch-series "to add support for Rust as a second language to the Linux kernel" was posted to LKML \[..\]
> 
> There have been several improvements to the overall Rust support since RFC and v2 described in the linked mail.

– [Thorsten Leemhuis on twitter](https://twitter.com/kernellogger/status/1467874273582886921?s=20)

llogiq unanimously suggested and voted that this be our quote for this week.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
