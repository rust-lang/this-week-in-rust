Title: This Week in Rust 345
Number: 345
Date: 2020-06-30
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
* [A practical guide to async in Rust](https://blog.logrocket.com/a-practical-guide-to-async-in-rust/)
* [Secure Rust Guidelines - ANSSI (National Cybersecurity Agency of France)](https://anssi-fr.github.io/rust-guide/)
* [Faster Rust development on AWS EC2 with VSCode](https://dev.to/rimutaka/faster-rust-development-on-aws-ec2-with-vscode-4hno)
* [Rust verification tools](https://alastairreid.github.io/rust-verification-tools/)
* [Examining ARM vs X86 Memory Models with Rust](https://www.nickwilcox.com/blog/arm_vs_x86_memory_model/)
* [Building a faster CouchDB View Server in Rust](https://www.garrensmith.com/blogs/fortuna-rs-couchdb-view-server)
* [Extremely Simple Rust Rocket Framework Tutorial](https://frogtok.com/extremely-simple-rust-rocket-framework-tutorial/)
* [Build a Smart Bookmarking Tool with Rust and Rocket](https://developers.facebook.com/blog/post/2020/06/03/build-smart-bookmarking-tool-rust-rocket/)
* [A Future is a Suspending Scheduler](https://nikhilism.com/post/2020/futures-suspending-scheduler/)
* [Implementing a Job queue with SQLx and Postgres](https://cetra3.github.io/blog/implementing-a-jobq-sqlx/)
* [Cross building Rust GStreamer plugins for the Raspberry Pi](https://www.collabora.com/news-and-blog/blog/2020/06/23/cross-building-rust-gstreamer-plugins-for-the-raspberry-pi/)
* [xi-editor retrospective](https://raphlinus.github.io/xi/2020/06/27/xi-retrospective.html)
* [rust-analyzer changelog #31](https://rust-analyzer.github.io/thisweek/2020/06/29/changelog-31.html)
* [IntelliJ Rust Changelog #125](https://intellij-rust.github.io/2020/06/29/changelog-125.html)
* [video] [Manipulating ports, virtual ports and pseudo terminals - Rust Wrocław Webinar](https://www.youtube.com/watch?v=_cYz03jS7tk&feature=youtu.be)
* [video] [Rust Stream: Iterators](https://www.youtube.com/watch?v=lQt0adYPdfQ&feature=youtu.be)

# Crate of the Week

This week's crate is [diskonaut](https://github.com/imsnif/diskonaut), a disk usage explorer.

Thanks to [Aram Drevekenin](https://users.rust-lang.org/t/crate-of-the-week/2704/781) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Gooseberry: Set the kb_dir somewhere more accessible to the user](https://github.com/out-of-cheese-error/gooseberry/issues/8)
* [Ruma: Add directory and profile query endpoints](https://github.com/ruma/ruma/issues/79)

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
* [impl PartialEq<Vec<B>> for &[A], &mut [A]](https://github.com/rust-lang/rust/pull/71660)
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

* [RFC: Deduplicate Cargo workspace information](https://github.com/rust-lang/rfcs/pull/2906)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Inline `const` expressions and patterns](https://github.com/rust-lang/rfcs/pull/2920)
* [Inline assembly](https://github.com/rust-lang/rfcs/pull/2873)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [impl `From<char>` for String](https://github.com/rust-lang/rust/pull/73466)
* [disposition: merge] [mv std libs to std/](https://github.com/rust-lang/rust/pull/73265)
* [disposition: merge] [Stabilize `transmute` in constants and statics but not const fn](https://github.com/rust-lang/rust/pull/72920)
* [disposition: merge] [added `.collect()` into String from `Box<str>`](https://github.com/rust-lang/rust/pull/72688)
* [disposition: merge] [Stabilize const_type_id feature](https://github.com/rust-lang/rust/pull/72488)

## New RFCs

* [Linking modifiers for native libraries](https://github.com/rust-lang/rfcs/pull/2951)
* [Hierarchic anonymous life-time](https://github.com/rust-lang/rfcs/pull/2949)
* [Portable packed SIMD vector type](https://github.com/rust-lang/rfcs/pull/2948)
* [crates.io token scopes](https://github.com/rust-lang/rfcs/pull/2947)

# Upcoming Events

### Online
* [June 30. Berlin, DE - Remote - Berlin Rust - Rust and Tell](https://berline.rs/)
* [July 1. Johannesburg, ZA - Remote - Monthly Joburg Rust Chat!](https://www.meetup.com/Johannesburg-Rust-Meetup/events/271286846/)
* [July 1. Dublin, IE - Remote - Rust Dublin - July Remote Meetup](https://www.meetup.com/Rust-Dublin/events/271417290/)
* [July 1. Indianapolis, IN, US - Indy Rust - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybckbcb/)
* [July 13. Seattle, WA, US - Seattle Rust Meetup - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybckbsb/)

### North America
* [June 30. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybcjbnc/)
* [July 8. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybckblb/)
* [July 9. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/)

### Asia Pacific
* [July 6. Auckland, NZ - Rust AKL](https://www.meetup.com/rust-akl/events/266876691/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Software Engineer - Backend at LogDNA, Remote, US](https://boards.greenhouse.io/logdna/jobs/4703358002)
* [Senior Software Engineer - Protocols in Rust at Ockam, Remote](https://www.ockam.io/team/Senior-Software-Engineer-Protocols-in-Rust/1d3e2e2d-e538-5847-874f-6bec1b63af97)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust's beauty lies in the countless decisions made by the development community that constantly make you feel like you can have ten cakes and eat all of them too.

– [Jake McGinty et al on the tonari blog](https://blog.tonari.no/why-we-love-rust)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/896) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hepkfq/this_week_in_rust_344/)</small>
