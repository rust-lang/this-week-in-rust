Title: This Week in Rust 416
Number: 416
Date: 2021-11-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Foundation

* [Launching the Rust Foundation Quarterly AMAs: Join the Conversation](https://foundation.rust-lang.org/posts/2021-11-04-rust-foundation-ama-launch/)

### Project/Tooling Updates

* [SixtyFPS (GUI crate): Changelog for 7th of November 2021](https://sixtyfps.io/thisweek/2021-11-08.html)
* [Rust Analyzer Changelog #102](https://rust-analyzer.github.io/thisweek/2021/11/08/changelog-102.html)
* [Intellij Rust Changelog #159](https://intellij-rust.github.io/2021/11/08/changelog-159.html)
* [This week in Fluvio #12: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0012/)
* [This week in Databend #15: an elastic and reliable cloud warehouse](https://datafuselabs.github.io/weekly/2021-11-10-databend-weekly/)
* [Knurling toolset v0.3.0 has been released!](https://ferrous-systems.com/blog/defmt-3/)

### Newsletter

* [This Month in Rust OSDev (October 2021)](https://rust-osdev.com/this-month/2021-10/)
* [This Month in Rust GameDev #27](https://gamedev.rs/news/027/)

### Observations/Thoughts

* [Benchmarking symmetric encryption (AEAD) in Rust](https://kerkour.com/rust-symmetric-encryption-aead-benchmark/)
* [View types for Rust](https://smallcultfollowing.com/babysteps//blog/2021/11/05/view-types/)
* [What Memory Model Should the Rust Language Use?](https://paulmck.livejournal.com/66175.html)
* [C++ Move Semantics Considered Harmful (Rust is better)](https://www.thecodedmessage.com/posts/cpp-move/)
* [It's Time to Get Hyped About Const Generics in Rust](https://nora.codes/post/its-time-to-get-hyped-about-const-generics-in-rust/)
* [Automating Ember releases with Rust](https://simplabs.com/blog/2021/11/09/automating-ember-learning-releases-with-rust/)
* [Breakout: optimizing rust+WASM](https://pci.github.io/blog/2021/10/advent-of-code-rust-wasm-and-js/)
* [How we extended Helm lifecycle with Rust](https://hub.qovery.com/guides/engineering/how_we_extended_helm_lifecycle_with_rust/)

### Rust Walkthroughs

* [Run Your Rust Games in a Browser: Hands-on Rust Bonus Content](https://hands-on-rust.com/2021/11/06/run-your-rust-games-in-a-browser-hands-on-rust-bonus-content/)
* [Building My First Command Line Interface (CLI) with Rust](https://devtails.medium.com/building-my-first-command-line-interface-cli-with-rust-b6beb9c284e0)
* [Making My Website Part 2: The Webserver](https://www.rotoclone.zone/blog/posts/webserver)
* [Debugging Rust application inside linux container](https://blog.erebe.dev/blog/debug-rust-aplication-inside-container/index.html)
* [Plugins in Rust: Reducing the Pain with Dependencies](https://nullderef.com/blog/plugin-abi-stable/)
* [Discover Hidden Secrets in Git Repos with Rust](https://blog.jonaylor.xyz/discover-hidden-secrets-in-git-repos-with-rust)
* [Prepare your Rust API docs for Github Pages](https://dev.to/deciduously/prepare-your-rust-api-docs-for-github-pages-2n5i)
* [Using KI18n with Rust and Qml](https://dev.to/ayush1325/using-ki18n-with-rust-and-qml-ja7)
* [Self Referential Structs in Rust](https://dev.to/arunanshub/self-referential-structs-in-rust-33cm)
* [series] [video] [Rust Book Club #5: Structs!](https://youtu.be/mBi_FsPKd9w)
* [video] [Rust Programming - Full 59-Second Course for Beginners](https://youtu.be/rzZt0fEzUXQ)
* [video] [Causing problems with Rust traits (then fixing them)](https://youtu.be/sNyEgAGkDN8)
* [audio] [Hyper with Sean McArthur](https://rustacean-station.org/episode/045-sean-arthur/)
* [series][video] [Writing a Programming Language (in Rust) 6: Function calls (Part 2)](https://www.youtube.com/watch?v=Qm1IM8SEvi8)
* [series][video] [Writing a Programming Language (in Rust) Extra 6.1: Fixing Environment Lookups](https://www.youtube.com/watch?v=sLmervrw8Ow)

### Miscellaneous

* [What does `&mut &[T]` mean?](https://ihatereality.space/04-what-mutref-to-slice-ref-means/)
* [Write Rust lints without forking Clippy](https://blog.trailofbits.com/2021/11/09/write-rust-lints-without-forking-clippy/)

## Crate of the Week

This week's crate is [chumsky](https://github.com/zesterer/chumsky), a friendly parser combinator crate.

Thanks to [Jan Riemer](https://users.rust-lang.org/t/crate-of-the-week/2704/981) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ockam - Renumber WebSocket transport type from 2 to 3](https://github.com/ockam-network/ockam/issues/2194)
* [ockam - Use Zeroize for temporary sensitive data](https://github.com/ockam-network/ockam/issues/2051)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

296 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-11-01..2021-11-08

* [improve error when an `.rlib` can't be parsed](https://github.com/rust-lang/rust/pull/88368)
* [implementation of GATs outlives lint](https://github.com/rust-lang/rust/pull/89970)
* [add beginner friendly lifetime elision hint to E0623](https://github.com/rust-lang/rust/pull/90179)
* [add `JoinHandle::is_running`](https://github.com/rust-lang/rust/pull/90439)
* [suggest `extern crate alloc` when using undeclared module `alloc`](https://github.com/rust-lang/rust/pull/90507)
* [suggest dereference of `Box` when inner type is expected](https://github.com/rust-lang/rust/pull/90627)
* [stabilize `relaxed_struct_unsize`](https://github.com/rust-lang/rust/pull/90417)
* [optimize bidi character detection.](https://github.com/rust-lang/rust/pull/90559)
* [implement `RefUnwindSafe` for `Rc<T>`](https://github.com/rust-lang/rust/pull/87467)
* [make `std::thread::available_concurrency` support process-limited number of CPUs](https://github.com/rust-lang/rust/pull/89310)
* [hashbrown: implement From\<array\> on HashSet and HashMap](https://github.com/rust-lang/hashbrown/pull/298)
* [cargo: fix debug panic on download with redirect body.](https://github.com/rust-lang/cargo/pull/10048)
* [clippy: add `cargo dev lint` to manually run clippy on a file](https://github.com/rust-lang/rust-clippy/pull/7917)
* [clippy: add suggestion to missing backticks error](https://github.com/rust-lang/rust-clippy/pull/7904)
* [clippy: advise to put a `::` prefix inside the ticks](https://github.com/rust-lang/rust-clippy/pull/7916)
* [clippy: fix panics while parsing format string that uses named arg twice](https://github.com/rust-lang/rust-clippy/pull/7906)
* [clippy: fix ICE in `undocumented_unsafe_blocks`](https://github.com/rust-lang/rust-clippy/pull/7945)
* [clippy: fix false negative in `match_overlapping_arms`](https://github.com/rust-lang/rust-clippy/pull/7909)
* [clippy: fix `manual_assert` and `match_wild_err_arm` for `#![no_std]` and Rust 2021](https://github.com/rust-lang/rust-clippy/pull/7851)
* [clippy: move `non_ascii_literal` to restriction](https://github.com/rust-lang/rust-clippy/pull/7907)
* [clippy: prevent `clippy::needless_lifetimes` false positive in async function definition](https://github.com/rust-lang/rust-clippy/pull/7901)
* [clippy: unseparated literal suffix](https://github.com/rust-lang/rust-clippy/pull/7726)
* [clippy: use .cargo/config.toml instead of .cargo/config](https://github.com/rust-lang/rust-clippy/pull/7918)
* [clippy: avoid linting `possible_truncation` on bit-reducing operations](https://github.com/rust-lang/rust-clippy/pull/7819)
* [rustfmt: put empty trait braces on same line if possible](https://github.com/rust-lang/rustfmt/pull/5060)
* [rustfmt: dedupe and simplify type alias formatting](https://github.com/rust-lang/rustfmt/pull/5068)
* [rustfmt: dedupe associated item visitation](https://github.com/rust-lang/rustfmt/pull/5069)
* [rustfmt: handle external mods imported via external → inline load hierarchy](https://github.com/rust-lang/rustfmt/pull/5064)

### Rust Compiler Performance Triage

Largely a positive week despite taking a significant performance hit from turning on incremental compilation verification for a subsection of the total queries that the compiler does in order to more quickly catch bugs in incremental compilation. Luckily optimizations in bidi detection brought large performance improvements.

Triage done by **@rylev**.
Revision range: [6384dc..eee8b](https://perf.rust-lang.org/?start=6384dca100f3cedfa031a9204586f94f8612eae5&end=eee8b9c7bafade55981d155dae71657f1cc55a22&absolute=false&stat=instructions%3Au)

2 Regressions, 4 Improvements, 4 Mixed; 1 of them in rollups
45 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-11-09.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Cargo --crate-type CLI Argument](https://github.com/rust-lang/rfcs/pull/3180)
* [disposition: merge] [Static async fn in traits](https://github.com/rust-lang/rfcs/pull/3185)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize format args capture](https://github.com/rust-lang/rust/pull/90473)
* [disposition: merge] [Stabilize -Z symbol-mangling-version=v0 as -C symbol-mangling-version=v0](https://github.com/rust-lang/rust/pull/90128)
* [disposition: merge] [Stabilize -Z strip as -C strip](https://github.com/rust-lang/rust/pull/90058)

### New RFCs

* [Add provide_any module to core](https://github.com/rust-lang/rfcs/pull/3192)

## Upcoming Events

Rusty Events between 11/10-11/24 🦀

### Online

* [November 10, 2021, Boulder, CO, US - Monthly Meetup - Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/281765483)
* [November 10, 2021, Malaysia - Rust Meetup - Rust Malaysia](https://discord.gg/9Xj8H2EXTD)
* [November 10, 2021, Los Angeles, CA, US - Rust Los Angeles: Live Coding Session - Mob Programming a Rust Code Kata](https://www.meetup.com/Rust-Los-Angeles/events/281944639/)
* [November 11, 2021 - Rust For Linux: Writing Safe Abstractions & Drivers - The Linux Foundation](https://linuxfoundation.org/webinars/rust-for-linux-writing-abstractions-and-drivers/)
* [November 16, 2021 - The Rust Foundation Ask Me Anything (AMA) - Rust Foundation](https://zoom.us/webinar/register/WN_BNOwcv_TS7aMpGNGEaTgEQ)
* [November 17, 2021, Vancouver, BC, CA - Borrowing and Lifetimes through Metaphors - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccpbwb/)
* [November 17, 2021, Houston, TX, US - A Functional Introduction to Rust - Houston Functional Programming User Group](https://www.meetup.com/houston-functional-programming-users-group/events/281526282)
* [November 17, 2021, Los Angeles, CA, US - Live Coding Session: Mob Programming a Rust Code Kata - Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/281944639)
* [November 23, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)

### North America

* [November 10, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccpbnb/)
* [November 10, 2021, Mesa, AZ, US - Booze.rs - Desert Rust](https://www.meetup.com/Desert-Rustaceans/events/281729697)

### Europe

* [November 11, 2021, Belgrade, RS - First! - Belgrade Rust Meetup Group](https://www.meetup.com/belgrade-rust-meetup-group/events/281523208/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**CoScreen**

* [Senior Desktop Application Developer (C++/Rust) (Remote)](https://jobs.lever.co/unusual/59a25c98-5d46-4ce5-8376-758e239bb356)

**Polar Sync**

* [Principal/Senior Software Engineer - Rust/C++ (Remote)](https://polarsync.breezy.hr/p/0c1d3630d39d)

**Tangram**

* [Senior Rust Developer](https://www.tangram.dev/jobs)

**Toposware**

* [Distributed Systems Engineer (Blockchain) (Remote)](https://toposware.bamboohr.com/jobs/view.php?id=23&source=toposware)

**Kraken**

* [Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Senior Rust Engineer - Banking (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> And even if you could fix all of rustc's soundness holes, or otherwise prevent user code from
> exploiting them, a soundness bug in any third-party library can also make it possible for
> malicious crates to trigger arbitrary behavior from safe code.
>
> [...]
>
> This is why we need to emphasize that while Rust's static analyses are very good at limiting
> accidental vulnerabilties in non-malicious code, they are not a sandbox system that can place
> meaningful limits on malicious code.

– [Matt Brubeck on rust-users](https://users.rust-lang.org/t/regarding-the-security-safety-of-libraries-on-crates-io/66294/24)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1132) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcy](https://github.com/kolharsam), [marriannegoldin](https://github.com/marriannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/qras8f/this_week_in_rust_416/)</small>
