Title: This Week in Rust 385
Number: 385
Date: 2021-04-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No papers/research projects this week.

### Official
* [Inside] [Core Team updates](https://blog.rust-lang.org/inside-rust/2021/04/03/core-team-updates.html)
* [Foundation] [Introducing Peixin Hou](https://foundation.rust-lang.org/posts/2021-04-08-introducing-peixin-hou/)
* [Foundation] [Introducing Florian Gilcher](https://foundation.rust-lang.org/posts/2021-04-08-introducing-florian-gilcher/)

### Newsletters
* [This Month in Rust OSDev (March 2021)](https://rust-osdev.com/this-month/2021-03/)
* [RiB Newsletter #22 - A few tweaks](https://www.reddit.com/r/rust/comments/mhmfu9/rib_newsletter_22_a_few_tweaks/)

### Project/Tooling Updates
* [GCC Rust Monthly Report #4 March 2021](https://thephilbert.io/2021/04/02/gcc-rust-monthly-report-4-march-2021/)
* [mrustc upgrade: rustc 1.39.0](https://www.reddit.com/r/rust/comments/mjxbaz/mrustc_upgrade_rustc_1390/)
* [rust-analyzer Changelog #71](https://rust-analyzer.github.io/thisweek/2021/04/05/changelog-71.html)
* [A new Left Recursive PEG Parser Generator for rust](https://www.mess.org/2021/03/26/Left-Recursive-PEG-Parser-Generator/)
* [Last Month in Flott (Motion Control Toolkit in Rust) - April 2021](https://flott-motion.org/news/last-month-in-flott-april-2021/)
* [Recent updates in IntelliJ Rust](https://blog.jetbrains.com/rust/2021/04/08/intellij-rust-updates-for-2021-1/)
* [Bevy 0.5](https://bevyengine.org/news/bevy-0-5/)

### Observations/Thoughts
* [Interfacing a low-level actor system to Rust async/await, part 1](https://uazu.github.io/blog/20210406.html)
* [A Tour of Safe Tracing GC Designs in Rust](https://manishearth.github.io/blog/2021/04/05/a-tour-of-safe-tracing-gc-designs-in-rust/)
* [How I Used Rust + Lunatic to Build a TelNet Chat Server with WebAssembly](https://www.hackernoon.com/how-i-used-rust-lunatic-to-build-a-telnet-chat-server-with-webassembly-rb3l33cg)
* [Eliminating Data Races in Firefox - A Technical Report](https://hacks.mozilla.org/2021/04/eliminating-data-races-in-firefox-a-technical-report/)
* [First-class IO](https://blog.sunfishcode.online/first-class-io/)
* [The modern packager's security nightmare](https://blogs.gentoo.org/mgorny/2021/02/19/the-modern-packagers-security-nightmare/)
* [Ordering Requests to Accelerate I/O](https://pkolaczk.github.io/disk-access-ordering/)
* [An essay of checked exceptions in Rust](https://users.rust-lang.org/t/an-essay-of-checked-exceptions-in-rust/57769)
* [Weird architectures weren't supported to begin with](https://blog.yossarian.net/2021/02/28/Weird-architectures-werent-supported-to-begin-with)
* [video] [I tried learning OpenGL in 7 days - using Rust](https://youtu.be/KEQIWqSq42k)

### Rust Walkthroughs
* [How we built our Python Client that's mostly Rust](https://www.fluvio.io/blog/2021/03/python-client/)
* [Hello world with KAS GUI](https://kas-gui.github.io/tutorials/hello.html)
* [How to create small Docker images for Rust](https://kerkour.com/blog/rust-small-docker-image/)
* [Oxidizing the Kubernetes Operator](https://www.pavel.cool/rust/rust-kubernetes-operators/)
* [Sending tuples from Node to Rust and back](https://www.fluvio.io/blog/2021/04/node-bindgen-tuples/)
* [Getting started with Kafka and Rust: Part 1](https://dev.to/abhirockzz/getting-started-with-kafka-and-rust-part-1-4hkb)
* [A Beginner's Guide to Handling Errors in Rust](https://dev.to/seanchen1991/a-beginner-s-guide-to-handling-errors-in-rust-40k2)
* [Using Seahorn](https://project-oak.github.io/rust-verification-tools/using-seahorn/)
* [Asynchronous streams in Rust (part 1) - Futures, buffering and mysterious compilation error messages](https://gendignoux.com/blog/2021/04/01/rust-async-streams-futures-part1.html)
* [series] [What would SQLite look like if written in Rust? — Part 3](https://medium.com/the-polyglot-programmer/what-would-sqlite-look-like-if-written-in-rust-part-3-edd2eefda473)
* [video] [Return a value from a function in Rust](https://www.youtube.com/watch?v=YNSg7g46Hso)
* [video] [Crust of Rust: Atomics and Memory Ordering](https://youtu.be/rMGWeSjctlY)
* [video] [Async/Await in Rust: Introduction](https://youtu.be/FNcXf-4CLH0)
* [video] [OpenVehicleDiag Rust live coding with Macchina's A0!!](https://youtu.be/zjAe-uvKMJ4)
* [video] [series] [Easy Rust - learn to program in Rust with simple English](https://youtube.com/playlist?list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk)

### Miscellaneous
* [best-of-ml-rust: A ranked list of awesome machine learning Rust libraries](https://github.com/e-tony/best-of-ml-rust)
* [Rust in the Android platform](https://security.googleblog.com/2021/04/rust-in-android-platform.html)
* [David Tolnay - thank you](https://www.reddit.com/r/rust/comments/mify2o/david_tolnay_thank_you/)
* [My "shiny future"](https://smallcultfollowing.com/babysteps/blog/2021/04/02/my-shiny-future/)

# Crate of the Week

This week's crate is [rs-pbrt](https://crates.io/crates/rs_pbrt), a counterpart to the PBRT book's (3rd edition) C++ code.

Thanks to [Jan Walter](https://users.rust-lang.org/t/crate-of-the-week/2704/900) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [dotenv-linter has several good first issues](https://github.com/dotenv-linter/dotenv-linter/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

313 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-03-29..2021-04-05

* [fix stack overflow detection on FreeBSD 11.1+](https://github.com/rust-lang/rust/pull/83771)
* [disallow the use of high byte registes as operands on `x86_64`](https://github.com/rust-lang/rust/pull/83853)
* [resolve/expand: cache intermediate results of `#[derive]` expansion](https://github.com/rust-lang/rust/pull/82907)
* [panic early when `TrustedLen` indicates a `length > usize::MAX`](https://github.com/rust-lang/rust/pull/83726)
* [suggest `box`/`pin`/`arc`ing receiver on method calls](https://github.com/rust-lang/rust/pull/83667)
* [run LLVM coverage instrumentation passes before optimization passes](https://github.com/rust-lang/rust/pull/83666)
* [simplify logical operations CFG](https://github.com/rust-lang/rust/pull/83663)
* [remove unneeded type resolving](https://github.com/rust-lang/rust/pull/83839)
* [unaligned_references: `align(N)` fields in `packed(N)` structs are fine](https://github.com/rust-lang/rust/pull/83605)
* [prevent very long compilation runtimes in `LateBoundRegionNameCollector`](https://github.com/rust-lang/rust/pull/83406)
* [reduce the impact of `Vec::reserve` calls that do not cause any allocation](https://github.com/rust-lang/rust/pull/83357)
* [BTree: no longer search arrays twice to check `Ord`](https://github.com/rust-lang/rust/pull/83267)
* [stream the dep-graph to a file instead of storing it in-memory](https://github.com/rust-lang/rust/pull/82780)
* [implement `SourceIterator` and `InPlaceIterable` for `ResultShunt`](https://github.com/rust-lang/rust/pull/81619)
* [optimize jumps in `PartialOrd::le`](https://github.com/rust-lang/rust/pull/83819)
* [`ffi::c_str` removed bound checks on `as_bytes`, `to_bytes`](https://github.com/rust-lang/rust/pull/83609)
* [added `as_slice` method to `BinaryHeap` collection](https://github.com/rust-lang/rust/pull/82331)
* [use `#[inline(always)]` on trivial `UnsafeCell` methods](https://github.com/rust-lang/rust/pull/83858)
* [add `#[inline]` to `IpAddr` methods](https://github.com/rust-lang/rust/pull/83831)
* [disallow octal format in Ipv4 string](https://github.com/rust-lang/rust/pull/83652)
* [constify methods of `std::net::SocketAddr`, `SocketAddrV4` and `SocketAddrV6`](https://github.com/rust-lang/rust/pull/82487)
* [constify some slice methods](https://github.com/rust-lang/rust/pull/83571)
* [stdsimd: add saturating abs/neg](https://github.com/rust-lang/stdsimd/pull/87)
* [hashbrown: make `RawTable::insert_no_grow` unsafe](https://github.com/rust-lang/hashbrown/pull/254)
* [cargo: add cargo config subcommand](https://github.com/rust-lang/cargo/pull/9302)
* [rustdoc: only look at blanket impls in `get_blanket_impls`](https://github.com/rust-lang/rust/pull/83681)
* [rustdoc: add unstable option to only emit shared/crate-specific files](https://github.com/rust-lang/rust/pull/83478)
* [rustdoc: don't enter an `infer_ctxt` in `get_blanket_impls` for impls that aren't blanket impls](https://github.com/rust-lang/rust/pull/82864)
* [rustdoc: highlight macros more efficiently](https://github.com/rust-lang/rust/pull/83793)
* [clippy: add `non_octal_unix_permissions` lint](https://github.com/rust-lang/rust-clippy/pull/7001)
* [clippy: don't lint `manual_map` in const functions](https://github.com/rust-lang/rust-clippy/pull/6976)
* [clippy: new Lint: `needless_for_each`](https://github.com/rust-lang/rust-clippy/pull/6706)
* [clippy: new Lint: `branches_sharing_code`](https://github.com/rust-lang/rust-clippy/pull/6463)
* [clippy: lint: `filter(Option::is_some).map(Option::unwrap)`](https://github.com/rust-lang/rust-clippy/pull/6342)
* [clippy: remove author requirement for `cargo_common_metadata`](https://github.com/rust-lang/rust-clippy/pull/7026)
* [Clippy going dark: adding a dark theme to Clippy's lint list](https://github.com/rust-lang/rust-clippy/pull/7030)
* [crates.io: topologically sort `db-dump.tar.gz`](https://github.com/rust-lang/crates.io/pull/3409)
* [parallelize tidy](https://github.com/rust-lang/rust/pull/82347)

## Rust Compiler Performance Triage

A pretty major week for [memory usage improvements] with an average of ~20% gains on memory usage for
release builds, and 5% on check builds, due to an update in the default allocator
used (to a more recent jemalloc). Wall time performance remained largely unchanged over this week.

Triage done by **@simulacrum**.
Revision range: [4896450e..d32238](https://perf.rust-lang.org/?start=4896450e7e0a522486b4d3a8d360ac4e1d2072a0&end=d32238532138485c80db4f2cd596372bce214e00&absolute=false&stat=instructions%3Au)

[memory usage improvements]: https://perf.rust-lang.org/?start=4896450e7e0a522486b4d3a8d360ac4e1d2072a0&end=d32238532138485c80db4f2cd596372bce214e00&absolute=false&stat=max-rss

1 Regressions, 4 Improvements, 0 Mixed

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Declarative macro metavariable expressions](https://github.com/rust-lang/rfcs/pull/3086)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [RFC: Structural Records](https://github.com/rust-lang/rfcs/pull/2584)
* [disposition: postpone] [Hygiene opt-out (escaping) for declarative macros 2.0](https://github.com/rust-lang/rfcs/pull/2498)
* [disposition: postpone] [RFC: Delegation](https://github.com/rust-lang/rfcs/pull/2393)
* [disposition: close] [RFC: `#[derive_no_bound(..)]` and `#[derive_field_bound(..)]`](https://github.com/rust-lang/rfcs/pull/2353)
* [disposition: postpone] [RFC: Eager Macro Expansion](https://github.com/rust-lang/rfcs/pull/2320)
* [disposition: merge] [try_trait_v2: A new design for the `?` desugaring](https://github.com/rust-lang/rfcs/pull/3058)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Remove `T: Debug` bound on UnsafeCell Debug impl](https://github.com/rust-lang/rust/pull/83707)
* [disposition: merge] [Turn old edition lint (anonymous-parameters) into warn-by-default on 2015](https://github.com/rust-lang/rust/pull/82918)
* [disposition: merge] [Stabilize `rustdoc::bare_urls` lint](https://github.com/rust-lang/rust/pull/81764)
* [disposition: merge] [Tracking issue: fNN::is_subnormal](https://github.com/rust-lang/rust/issues/79288)
* [disposition: merge] [Tracking Issue for feature(nonzero_leading_trailing_zeros)](https://github.com/rust-lang/rust/issues/79143)
* [disposition: merge] [Tracking Issue for `{BTreeMap,BTreeSet}::retain`](https://github.com/rust-lang/rust/issues/79025)
* [disposition: merge] [Tracking Issue for `#![feature(const_cell_into_inner)]`](https://github.com/rust-lang/rust/issues/78729)
* [disposition: merge] [Tracking Issue for `atomic_fetch_update`](https://github.com/rust-lang/rust/issues/78639)
* [disposition: merge] [Tracking Issue for feature: "option_insert"](https://github.com/rust-lang/rust/issues/78271)
* [disposition: merge] [Tracking Issue for `Duration` saturating operations](https://github.com/rust-lang/rust/issues/76416)
* [disposition: merge] [Tracking Issue for `Duration::{zero, is_zero} (#![feature(duration_zero)])`](https://github.com/rust-lang/rust/issues/73544)
* [disposition: close] [Tracking issue for FixedSizeArray trait](https://github.com/rust-lang/rust/issues/27778)

## New RFCs

* [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)

# Upcoming Events

### Online
* [April 7, Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/277133126/)
* [April 7, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/jhfstryccgbkb/)
* [April 12, Denver, CO, US - Building Delightful CLI Tools in Rust by Chuck Pierce - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/276801410/)
* [April 13, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccgbrb/)
* [April 13, Saarbrücken, Saarland, DE - **Rust Saar** 10u16](https://www.meetup.com/de-DE/Rust-Saar/events/276873622/)
* [April 20, Washington, DC, US - The Rust Borrow Checker—A Deep Dive - Rust DC](https://www.meetup.com/RustDC/events/ntvrgsyccgblb)

### North America

* [April 8, Columbus, OH, US - Monthly Meetup - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccgblb/)
* [April 14, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccgbsb/)

### Asia Pacific
* [April 19, Wellington, NZ - IGNITION: What is Rust and why should I care? Rust at work & at play - Rust Wellington](https://www.meetup.com/Rust-Wellington/events/277270667)

### Europe
* [April 21, Moscow, Russia - Monthly Meetup - Rust Moscow](https://www.meetup.com/ru-RU/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/277259838/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**IOTA Foundation**

* [IOTA Identity Software Engineer - Rust (Remote)](https://iota.bamboohr.com/jobs/view.php?id=143&source=other)

**Parity Technologies**

* [Blockchain Developer - Cross Chain Messaging (Remote)](https://grnh.se/9aec49883us)
* [Numerous other Rust engineering openings](https://www.parity.io/jobs/)

**Microsoft**

* [Azure IoT Senior Software Engineer (remote possible within U.S.)](https://careers.microsoft.com/us/en/job/960784/Senior-Software-Engineer)
    * Junior developers should also apply but relocation to Redmond is necessary in that case.

**Wallaroo**

* [Software Engineer (Remote)](https://wallaroo.breezy.hr/p/30939dc4e5c7-software-engineer)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Sadly there was no quote nominated for this week.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
