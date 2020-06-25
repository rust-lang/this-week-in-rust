Title: This Week in Rust 344
Number: 344
Date: 2020-06-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/019-twir-344/)

# Updates from Rust Community

## News & Blog Posts

* [Announcing Rust 1.44.1](https://blog.rust-lang.org/2020/06/18/Rust.1.44.1.html)
* [Writing Non-Trivial Macros in Rust](http://adventures.michaelfbryan.com/posts/non-trivial-macros/)
* [Diving into Rust with a CLI](https://kbknapp.dev/rust-cli/)
* [Rust for Data-Intensive Computation](https://materialize.io/rust-for-data-intensive-computation/)
* [3K, 60fps, 130ms: achieving it with Rust](https://blog.tonari.no/why-we-love-rust?ref=twtr)
* [Rust concurrency: the archetype of a message-passing bug.](https://medium.com/@polyglot_factotum/rust-concurrency-the-archetype-of-a-message-passing-bug-817b60efd8f8?source=friends_link&sk=ad32b77d42eda3dd28a26671282271df)
* [How to Design For Panic Resilience in Rust](https://towardsdatascience.com/how-to-design-for-panic-resilience-in-rust-55d5fd2478b9)
* [GitHub Action for binary crates installation](https://svartalf.info/posts/2020-04-10-github-action-for-binary-crates-installation/)
* [Managing Rust bloat with Github Actions](https://tomforb.es/managing-rust-bloat-with-github-actions/)
* [A multiplayer board game in Rust and WebAssembly](http://www.mattkeeter.com/projects/pont/)
* [Im bad at unsafe {}](https://djugei.github.io/bad-at-unsafe/)
* [SIMD library plans](https://vorner.github.io/2020/05/08/simd-library-plans.html)
* [Tips for Faster Rust Compile Times](https://endler.dev/2020/rust-compile-times/)
* [Rust Analyzer Changelog #30](https://rust-analyzer.github.io/thisweek/2020/06/22/changelog-30.html)
* [Building A Blockchain in Rust & Substrate: A Step-by-Step Guide for Developers](https://hackernoon.com/building-a-blockchain-in-rust-and-substrate-a-step-by-step-guide-for-developers-kc223ybp)
* [Dart and Rust: the async story](https://dev.to/sunshine-chain/rust-and-dart-the-async-story-3adk)
* [Decode a certificate](https://dev.to/wayofthepie/decode-a-certificate-5903)
* [Four Years of Rust At OneSignal](https://onesignal.com/blog/4-years-of-rust-at-onesignal/)
* [How Rust Lets Us Monitor 30k API calls/min](https://blog.bearer.sh/how-rust-lets-us-monitor-30k-api-calls-min/)
* [How to use C++ polymorphism in Rust](https://medium.com/swlh/how-to-use-c-polymorphism-in-rust-76e1d1a88ed1)
* [Implementing a Type-safe printf in Rust](http://willcrichton.net/notes/type-safe-printf/)
* [Introduction to Rust for Node Developers](https://dev.to/tindleaj/introduction-to-rust-for-node-developers-3j05)
* [The programming language that wants to rescue the world from dangerous code](https://www.protocol.com/rust-programming-safety-security)
* [Property-based testing in Rust with Proptest](https://blog.logrocket.com/property-based-testing-in-rust-with-proptest/)
* [Rust at CNCF](https://www.cncf.io/blog/2020/06/22/rust-at-cncf/)
* [Rust's Huge Compilation Units](https://pingcap.com/blog/rust-huge-compilation-units)
* [RustHorn: CHC-Based Verification for Rust Programs](https://link.springer.com/chapter/10.1007%2F978-3-030-44914-8_18)
* [Shipping Linux binaries that don't break with Rust](https://saarw.github.io/dev/2020/06/18/shipping-linux-binaries-that-dont-break-with-rust.html)
* [Some examples of Rust Lifetimes in a struct](https://dev.to/frankmeza/some-examples-of-rust-lifetimes-in-a-struct-3m53)
* [Static PIE and ASLR for the x86_64-unknown-linux-musl Target](https://harald.hoyer.xyz/rust-static-pie/)
* [Three bytes to an integer](https://dev.to/wayofthepie/three-bytes-to-an-integer-13g5)
* [Using Rust to Delete Gitignored Cruft](https://www.forrestthewoods.com/blog/using-rust-to-delete-gitignored-cruft/)
* [Tour of Rust - Chapter 8 - Smart Pointers](https://tourofrust.com/chapter_8_en.html)
* [Thread-local Storage - Part 13 of Making our own executable packer](https://fasterthanli.me/blog/2020/thread-local-storage/)
* [RISC-V OS using Rust - Chapter 11](http://osblog.stephenmarz.com/ch11.html)
* [Zero To Production #2: Learn By Building An Email Newsletter](https://www.lpalmieri.com/posts/2020-06-21-zero-to-production-2-learn-by-building-an-email-newsletter/)
* [video] [Crust of Rust: Smart Pointers and Interior Mutability](https://www.youtube.com/watch?v=8O0Nt9qY_vo)
* [video] [CS 196 at Illinois](https://www.youtube.com/channel/UCRA18QWPzB7FYVyg0WFKC6g/videos)
* [video] [Ask Me Anything with Felix Klock](https://www.youtube.com/watch?v=jGgQmnPH0dQ&feature=youtu.be&t=28792)
* [video] [Rust Stream: The Guard Pattern and Interior Mutability](https://www.youtube.com/watch?v=lmEKIvLh9D4&feature=youtu.be)

# Crate of the Week

This week's crate is [diskonaut](https://github.com/imsnif/diskonaut), a disk usage explorer.

Thanks to [Aram Drevekenin](https://users.rust-lang.org/t/crate-of-the-week/2704/781) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

* [GitUI is looking for contributors](https://github.com/extrawurst/gitui/issues)
* [Ruma: /account/whoami should use UserId](https://github.com/ruma/ruma/issues/54)

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

325 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-15..2020-06-22

* [add `asm!()` support for hexagon](https://github.com/rust-lang/rust/pull/73214)
* [enable LLVM zlib](https://github.com/rust-lang/rust/pull/72696)
* [add methods to go from a nul-terminated `Vec<u8>` to a `CString`](https://github.com/rust-lang/rust/pull/73139)
* [allow multiple `asm!` options groups and report an error on duplicate options](https://github.com/rust-lang/rust/pull/73227)
* [diagnose use of incompatible sanitizers](https://github.com/rust-lang/rust/pull/73347)
* [disallow loading crates with non-ascii identifier name](https://github.com/rust-lang/rust/pull/73305)
* [export `#[inline]` fns with extern indicators](https://github.com/rust-lang/rust/pull/73034)
* [fix up autoderef when reborrowing](https://github.com/rust-lang/rust/pull/72280)
* [further tweak lifetime errors involving `dyn Trait` and `impl Trait` in return position](https://github.com/rust-lang/rust/pull/72804)
* [implement crate-level-only lints checking.](https://github.com/rust-lang/rust/pull/73300)
* [implement new gdb/lldb pretty-printers](https://github.com/rust-lang/rust/pull/72357)
* [improve diagnostics for `let x += 1`](https://github.com/rust-lang/rust/pull/71976)
* [make `need_type_info_err` more conservative](https://github.com/rust-lang/rust/pull/73027)
* [make all uses of ty::Error delay a span bug](https://github.com/rust-lang/rust/pull/70551)
* [make new type param suggestion more targetted](https://github.com/rust-lang/rust/pull/73320)
* [make novel structural match violations not a `bug`](https://github.com/rust-lang/rust/pull/73446)
* [only display other method receiver candidates if they actually apply](https://github.com/rust-lang/rust/pull/73382)
* [prefer accessible paths in 'use' suggestions](https://github.com/rust-lang/rust/pull/72623)
* [prevent attacker from manipulating FPU tag word used in SGX enclave](https://github.com/rust-lang/rust/pull/73471)
* [projection bound validation](https://github.com/rust-lang/rust/pull/72788)
* [report error when casting an C-like enum implementing Drop](https://github.com/rust-lang/rust/pull/72331)
* [specialization is unsound](https://github.com/rust-lang/rust/pull/71420)
* [use min_specialization in the remaining rustc crates](https://github.com/rust-lang/rust/pull/72707)
* [add specialization of `ToString for char`](https://github.com/rust-lang/rust/pull/73465)
* [suggest `?Sized` when applicable for ADTs](https://github.com/rust-lang/rust/pull/73261)
* [support sanitizers on aarch64-unknown-linux-gnu](https://github.com/rust-lang/rust/pull/73058)
* [test that bounds checks are elided when slice len is checked up-front](https://github.com/rust-lang/rust/pull/73362)
* [try to suggest dereferences on trait selection failed](https://github.com/rust-lang/rust/pull/72456)
* [use track caller for bug! macro](https://github.com/rust-lang/rust/pull/73373)
* [forbid mutable references in all constant contexts except for const-fns](https://github.com/rust-lang/rust/pull/72934)
* [linker: MSVC supports linking static libraries as a whole archive](https://github.com/rust-lang/rust/pull/72785)
* [linker: never pass `-no-pie` to non-gnu linkers](https://github.com/rust-lang/rust/pull/73384)
* [lint: normalize projections using opaque types](https://github.com/rust-lang/rust/pull/73287)
* [add a lint to catch clashing `extern` fn declarations.](https://github.com/rust-lang/rust/pull/70946)
* [memory access sanity checks: abort instead of panic](https://github.com/rust-lang/rust/pull/73054)
* [pretty/mir: const value enums with no variants](https://github.com/rust-lang/rust/pull/73442)
* [store `ObligationCause` on the heap](https://github.com/rust-lang/rust/pull/72962)
* [chalk: add closures](https://github.com/rust-lang/chalk/pull/519)
* [chalk: ignore auto traits order](https://github.com/rust-lang/chalk/pull/531)
* [fix asinh of negative values](https://github.com/rust-lang/rust/pull/72486)
* [stabilize Option::zip](https://github.com/rust-lang/rust/pull/72938)
* [stabilize vec::Drain::as_slice](https://github.com/rust-lang/rust/pull/72584)
* [use `Ipv4Addr::from<[u8; 4]>` when possible](https://github.com/rust-lang/rust/pull/73389)
* [core/time: Add Duration methods for zero](https://github.com/rust-lang/rust/pull/72790)
* [deprecate wrapping_offset_from](https://github.com/rust-lang/rust/pull/73580)
* [`impl PartialEq<Vec<B>>` for `&[A]`, `&mut [A]`](https://github.com/rust-lang/rust/pull/71660)
* [hashbrown: avoid creating small tables with a capacity of 1](https://github.com/rust-lang/hashbrown/pull/162)
* [stdarch: add AVX 512f gather, scatter and compare intrinsics](https://github.com/rust-lang/stdarch/pull/866)
* [cargo: adding environment variable CARGO_PKG_LICENSE](https://github.com/rust-lang/cargo/pull/8325)
* [cargo: cut down on data fetch from git dependencies](https://github.com/rust-lang/cargo/pull/8363)
* [cargo: fix doctests not running with --target=HOST](https://github.com/rust-lang/cargo/pull/8358)
* [cargo: fix order-dependent feature resolution.](https://github.com/rust-lang/cargo/pull/8395)
* [cargo: fix overzealous `clean -p` for reserved names](https://github.com/rust-lang/cargo/pull/8398)
* [cargo: support linker with -Zdoctest-xcompile.](https://github.com/rust-lang/cargo/pull/8359)
* [rustfmt: avoid using Symbol::intern](https://github.com/rust-lang/rustfmt/pull/4268)
* [rustfmt: ensure idempotency on empty match blocks](https://github.com/rust-lang/rustfmt/pull/4271)


## Rust Compiler Performance Triage

* [2020-06-23](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-23). Lots of improvements this week, and no regressions, which is good. But we regularly see significant performance effects on rollups, which is a concern.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [impl `From<char>` for String](https://github.com/rust-lang/rust/pull/73466)
* [disposition: merge] [stabilize leading_trailing_ones](https://github.com/rust-lang/rust/pull/73032)
* [disposition: merge] [Add `TryFrom<{int}>` for `NonZero{int}`](https://github.com/rust-lang/rust/pull/72717)
* [disposition: merge] [Stabilize `#[track_caller]`](https://github.com/rust-lang/rust/pull/72445)
* [disposition: merge] [add Windows system error codes that should map to `io::ErrorKind::TimedOut`](https://github.com/rust-lang/rust/pull/71756)
* [disposition: merge] [Tracking issue for RFC 2344, "Allow `loop` in constant evaluation"](https://github.com/rust-lang/rust/issues/52000)
* [disposition: merge] [Tracking issue for `Option::deref`, `Result::deref`, `Result::deref_ok`, and `Result::deref_err`](https://github.com/rust-lang/rust/issues/50264)

## New RFCs

* [RFC: 'C unwind' ABI](https://github.com/rust-lang/rfcs/pull/2945)

# Upcoming Events

### Online
* [June 24. Wroclaw, PL - Remote - Rust Wroclaw Meetup #22](https://www.meetup.com/Rust-Wroclaw/events/271319037/)
* [June 25. Edinburgh, UK - Remote - Pirrigator - Growing Tomatoes Free From Memory Errors and Race Conditions](https://www.meetup.com/rust-edi/events/271129693/)
* [June 25. Berlin, DE - Remote - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcjbhc/)
* [July 1. Johannesburg, ZA - Remote - Monthly Joburg Rust Chat!](https://www.meetup.com/Johannesburg-Rust-Meetup/events/271286846/)

### North America
* [June 30. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybcjbnc/)
* [July 1. Indianapolis, IN, US - Indy Rust - Indy.rs](https://www.meetup.com/indyrs/events/dtqwprybckbcb/)

### Asia Pacific
* [July 6. Auckland, NZ - Rust AKL](https://www.meetup.com/rust-akl/events/266876691/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs


* [Senior Back-End Rust Developer at Jewish Interactive (Ji)](https://www.jewishinteractive.org/senior-developer/?utm_campaign=Available%20Jobs%20-%202020&utm_content=132060591&utm_medium=social&utm_source=twitter&hss_channel=tw-449734543)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust's beauty lies in the countless decisions made by the development community that constantly make you feel like you can have ten cakes and eat all of them too.

– [Jake McGinty et al on the tonari blog](https://blog.tonari.no/why-we-love-rust)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/896) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hepkfq/this_week_in_rust_344/)</small>
