Title: This Week in Rust 405
Number: 405
Date: 2021-08-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

In the case of this newsletter, 404 is indeed found!

## Updates from Rust Community

### Project/Tooling Updates
* [rust-analyzer Changelog #91](https://rust-analyzer.github.io/thisweek/2021/08/23/changelog-91.html)
* [Apache Arrow Datafusion 5.0.0 release with major new features and performance improvements](https://arrow.apache.org/blog/2021/08/18/datafusion-5.0.0/)
* [Apache Arrow Ballista 0.5.0 release](https://arrow.apache.org/blog/2021/08/18/ballista-0.5.0/)
* [This week in Fluvio #3: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0003/)
* [SixtyFPS 0.1 Release](https://sixtyfps.io/blog/announcing-sixtyfps-0.1.html) and [SixtyFPS Weekly Report](https://sixtyfps.io/thisweek/2021-08-23.html)
* [nailgun: a DNS performance testing client](https://leshow.github.io/post/nailgun/)
* [nom 7.0 release: fast parser combinators, now without macros! And the new nom-bufreader](https://www.reddit.com/r/rust/comments/p9usvq/nom_70_release_fast_parser_combinators_now/)
* [Announcing egui 0.14](https://www.reddit.com/r/rust/comments/papqk1/announcing_egui_014/)
* [miette: a fancy new diagnostics definition and reporting crate for apps and libraries!](https://www.reddit.com/r/rust/comments/p9t3jh/miette_a_fancy_new_diagnostics_definition_and/)
* [Rudra: Rust Memory Safety & Undefined Behavior Detection](https://www.reddit.com/r/rust/comments/p8gcbe/rudra_rust_memory_safety_undefined_behavior/)
* [SnakeOS v0.1.0 released!](https://www.reddit.com/r/rust/comments/p86a24/snakeos_v010_released/)
* [Pueue v1.0.0 - Manage your shell commands. Reaching stable after 3.5 years](https://www.reddit.com/r/rust/comments/p84amc/release_pueue_v100_manage_your_shell_commands/)
* [Announcing Persy 1.0: a simple transactional storage](https://persy.rs/posts/persy-1.0.html)
* [This week in Datafuse #4](https://datafuselabs.github.io/weekly/2021-08-25-datafuse-weekly/)

### Observations/Thoughts
* [Overview of the Rust cryptography ecosystem](https://kerkour.com/blog/rust-cryptography-ecosystem)
* [Superpowers of Unsafe Rust](https://blog.knoldus.com/superpowers-of-unsafe-rust/)
* [Using KLEE on Rust-for-Linux (part 1)](https://project-oak.github.io/rust-verification-tools/2021/08/22/rust-on-linux-1.html)
* [Large Rust Workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)
* [Using Rust with Elixir for code reuse and performance](https://blog.doctave.com/2021/08/19/using-rust-with-elixir-for-code-reuse-and-performance.html)
* [Models of Generics and Metaprogramming: Go, Rust, Swift, D and More](https://thume.ca/2019/07/14/a-tour-of-metaprogramming-models-for-generics/)
* [Shaking Off the Rust 2: Ray Tracing in WebAssembly](https://clayto.com/2021/07/shaking-off-the-rust-2-ray-tracing-in-webassembly/)
* [Async Overloading](https://blog.yoshuawuyts.com/async-overloading/)
* [Scoped Arena Allocator](https://devblog.arcana.rs/scoped-arena-allocator)
* [Concurrency in Rust is indeed fearless](https://blog.knoldus.com/concurrency-in-rust-is-indeed-fearless/)

### Rust Walkthroughs
* [Rust vectors](https://saidvandeklundert.net/learn/2021-08-15-rust-vector/)
* [Daily Rust: Iterators](adventures.michaelfbryan.com/posts/daily/iterators/)
* [Getting Started with Systems Programming with Rust (Part 1)](https://www.bexxmodd.com/post/systems-programming-with-rust-1)
* [Getting Started with Systems Programming with Rust (Part 2)](https://www.bexxmodd.com/post/systems-programming-with-rust-2)
* [Optimizing Immutable Strings in Rust](https://dev.to/somedood/optimizing-immutable-strings-in-rust-2ahj)
* [Rust for Data Science: Tutorial 1](https://dev.to/davidedelpapa/rust-for-data-science-tutorial-1-4g5j)
* [Rust and WebAssembly Serverless functions in Vercel](https://dev.to/alabulei1/rust-and-webassembly-serverless-functions-in-vercel-47e4)
* [Pin, Unpin, and why Rust needs them](https://blog.adamchalmers.com/pin-unpin/)
* [Rust: How to Unwrap Multiple Required Options](https://blog.hendrikmaus.dev/rust-unwrap-options-in-a-function-that-returns-a-result-type/)
* [Hexagonal architecture in Rust #1](https://alexis-lozano.com/hexagonal-architecture-in-rust-1/)
* [Hexagonal architecture in Rust #2](https://alexis-lozano.com/hexagonal-architecture-in-rust-2/)
* [Daily Rust: Iterators](https://adventures.michaelfbryan.com/posts/daily/iterators/?utm_source=reddit&utm_medium=social&utm_campaign=daily-rust-basic-iterators)
* [Cross compiling Windows binaries from Linux](https://jake-shadle.github.io/xwin/)
* [Rust Trait Objects Demystified](https://desilva.io/posts/rust-trait-objects-demystified)
* [End-to-End Encryption through Kafka, with Rust](https://github.com/ockam-network/ockam/tree/develop/documentation/use-cases/end-to-end-encryption-through-kafka)
* [ZH] Formal Concept Analysis with Rust, [Part1](https://www.horsal.dev/formal-concept-analysis-with-rust-1-introduction), [Part2](https://www.horsal.dev/formal-concept-analysis-with-rust-2-basic-algorithm), [Part3](https://www.horsal.dev/formal-concept-analysis-with-rust-3-parallization)
* [video] [Building a small Finite State Machine in Rust](https://youtu.be/whN36JVUd6A)
* [video] [Rust Community Stuttgart - "Traits and trait objects - more than just interfaces"](https://www.youtube.com/watch?v=izXf9-CTAfc)

### Miscellaneous
* [An exhaustive list of all Rust resources regarding automated or semi-automated formalization efforts in any area, constructive mathematics, formal algorithms, and program verification.](https://github.com/newca12/awesome-rust-formalized-reasoning)
* [Aggregate streaming data in real-time with WebAssembly](https://www.infinyon.com/blog/2021/08/smartstream-aggregates/)
* [I re-implemented the legendary "Typing the technical interview" article using only Rust types!](https://www.reddit.com/r/rust/comments/pb97fa/i_reimplemented_the_legendary_typing_the/)
* [Chromium adds a Rust toolchain as a potential dependency](https://www.reddit.com/r/rust/comments/paxowq/chromium_adds_a_rust_toolchain_as_a_potential/)
* [New very promising Linux syscall for creating secret memory even the kernel can't read. I'll be working for creating a crate for using it out](https://www.reddit.com/r/rust/comments/p7qzdn/new_very_promising_linux_syscall_for_creating/)
* [picture] [Field Init Shorthand in Rust](https://www.reddit.com/r/rust/comments/pai1o8/media_field_init_shorthand_in_rust/)

## Crate of the Week

This week's crate is [kube-leader-election](https://github.com/hendrikmaus/kube-leader-election), a crate to implement leader election for Kubernetes workloads.

Thanks to [hendrikmaus](https://users.rust-lang.org/t/crate-of-the-week/2704/945) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rust-lang/cargo - "error: binary xxx already exists in destination" should print the full destination path](https://github.com/rust-lang/cargo/issues/9797)
* [ockam-network/ockam - Like ngrok, but using Ockam](https://github.com/ockam-network/ockam/issues/1762)
* [ockam-network/ockam - Forwarder in Rust](https://github.com/ockam-network/ockam/issues/1761)
* [ andreev-io/little-raft - New Contributors Wanted](https://github.com/andreev-io/little-raft/issues)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

293 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-08-16..2021-08-23

* [upgrade to LLVM 13](https://github.com/rust-lang/rust/pull/87570)
* [introduce `hir::ExprKind::Let` - Take 2](https://github.com/rust-lang/rust/pull/80357)
* [enable compiler consumers to obtain `mir::Body` with Polonius facts](https://github.com/rust-lang/rust/pull/86977)
* [force warn improvements](https://github.com/rust-lang/rust/pull/88134)
* [account for tabs when highlighting multiline code suggestions](https://github.com/rust-lang/rust/pull/87976)
* [use more accurate spans when proposing adding lifetime to item](https://github.com/rust-lang/rust/pull/87983)
* [add notes to macro-not-found diagnostics to point out how things with the same name were not a match](https://github.com/rust-lang/rust/pull/88232)
* [improve wording of the `drop_bounds` lint](https://github.com/rust-lang/rust/pull/86747)
* [improve NLL's "higher-ranked subtype error"s](https://github.com/rust-lang/rust/pull/86700)
* [suggest importing the right kind of macro](https://github.com/rust-lang/rust/pull/88229)
* [improve error reporting for closure return type mismatches](https://github.com/rust-lang/rust/pull/87661)
* [canonicalize consts before calling `try_unify_abstract_consts` query](https://github.com/rust-lang/rust/pull/88166)
* [reenable `RemoveZsts`](https://github.com/rust-lang/rust/pull/88176)
* [I/O safety](https://github.com/rust-lang/rust/pull/87329) (RFC [#3128](https://rust-lang.github.io/rfcs/3128-io-safety.html))
* [stabilize `arbitrary_enum_discriminant`](https://github.com/rust-lang/rust/pull/86860)
* [constified implementations of `Default`](https://github.com/rust-lang/rust/pull/86808)
* [optimize unnecessary check in `VecDeque::retain`](https://github.com/rust-lang/rust/pull/88075)
* [where available use `AtomicU`{`64`, `128`} instead of mutex for `Instant` backsliding protection](https://github.com/rust-lang/rust/pull/83093)
* [add fast path for `Path::cmp` that skips over long shared prefixes](https://github.com/rust-lang/rust/pull/86898)
* [cargo: fix panic with build-std of a proc-macro](https://github.com/rust-lang/cargo/pull/9834)
* [clippy: add new lints `negative_feature_names` and `redundant_feature_names`](https://github.com/rust-lang/rust-clippy/pull/7539)
* [clippy: move `branches_sharing_code` to nursery](https://github.com/rust-lang/rust-clippy/pull/7595)
* [clippy: remove stderr limit](https://github.com/rust-lang/rust-clippy/pull/7593)

### Rust Compiler Performance Triage

A few regressions but largely an improvement this week, mostly due to the
upgrade to LLVM 13.

Triage done by **@simulacrum**.
Revision range: [aa8f27b..33fdb79](https://perf.rust-lang.org/?start=aa8f27bf4d980023a8b245ceb25a490a18041eb2&end=33fdb797f59421c7bbecaa4588ed5d7a31a9494a&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 2 Mixed; 0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-08-24.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: let-expression](https://github.com/rust-lang/rfcs/pull/3159)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize std::os::unix::fs::chroot](https://github.com/rust-lang/rust/pull/88177)
* [disposition: merge] [Stabilize reserved prefixes](https://github.com/rust-lang/rust/issues/88140)
* [disposition: merge] [stabilize disjoint capture in closures (RFC 2229)](https://github.com/rust-lang/rust/issues/88126)
* [disposition: merge] [BTree: remove Ord bound from new](https://github.com/rust-lang/rust/pull/88040)
* [disposition: merge] [Update Windows Argument Parsing](https://github.com/rust-lang/rust/pull/87580)
* [disposition: merge] [Support #[track_caller] on closures and generators](https://github.com/rust-lang/rust/pull/87064)
* [disposition: merge] [Stabilize "force warn" option](https://github.com/rust-lang/rust/issues/86516)
* [disposition: merge] [Extend -Cdebuginfo with new options and named aliases](https://github.com/rust-lang/rust/pull/83947)
* [disposition: merge] [Allow writing of incomplete UTF-8 sequences to the Windows console via stdout/stderr](https://github.com/rust-lang/rust/pull/83342)
* [disposition: merge] [Tracking Issue for Iterator::intersperse](https://github.com/rust-lang/rust/issues/79524)
* [disposition: merge] [Provide an API to extract fields from Command builder](https://github.com/rust-lang/rust/issues/44434)

### New RFCs

* [RFC: cargo-run-deps](https://github.com/rust-lang/rfcs/pull/3168)
* [Proposal: Else clauses for for and while loops](https://github.com/rust-lang/rfcs/pull/3163)

## Upcoming Events

### Online

* [August 31, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/)
* [September 2, 2021, Zurich, CH - Exciting new Rustdoc features landing in 1.55.0 - Hybrid Meetup (Livestream!) - Rust Zurich](https://www.meetup.com/Rust-Zurich/events/280295950/)
* [September 2, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [September 8, 2021, Denver, CO, US - Rust Q&A - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/279407152/)

### North America

* [September 8, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccmblb/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Apple**

* [Senior Software Engineer - Apple Media Products](https://jobs.apple.com/en-us/details/200278498/senior-software-engineer-apple-media-products?team=SFTWR)

**Wingback**

* [Senior Backend Developer - rust 🦀 (Remote))](https://careers.wingback.com/o/senior-backend-developer-rust-remote)

**PolarFox Network**

* [Senior Rust Engineer (Remote)](https://polarsync.breezy.hr/p/0c1d3630d39d)

**Stealth Startup**

* [Senior Software Engineer (Raleigh, NC, US, Possible Remote US)](https://docs.google.com/document/d/1jOT6pDE3yNpUq3c9BvFJPy4XaqlIX7BiOqXiTU1Fpfk/edit?usp=sharing)

**Dusk Network**

* [Rust Developer (Fulltime) (Remote)](https://dusk.network/pages/rust-developer-vacancy)

**ChainSafe**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**Bitfury**

* [Rust Developer (Kyiv, Kiev, UA)](https://arbeitnow.com/view/rust-developer-bitfury-393648)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Parity Technologies**

* [Multiple Rust Engineering Positions Available](https://www.parity.io/jobs/)

**Subspace Labs**

* [Multiple Rust Engineering Positions Available](https://jobs.lever.co/subspacelabs)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Code doesn't deal with resources until it does. Similarly with everything else that forces you to reason about control flow - you don't care about thread management until you do, you don't care about action logs until you do, you don't care about performance until you do... and from the other side, code doesn't need to be exception-safe until it does. The trouble with this kind of "magic" language feature is that correctness becomes non-compositional: you can take two working pieces of code and put them together and get something that doesn't work.

– [Mickey Donaghy on Hacker News](https://news.ycombinator.com/item?id=26536896)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1096) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
