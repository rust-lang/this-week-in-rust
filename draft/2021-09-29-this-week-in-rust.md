Title: This Week in Rust 410
Number: 410
Date: 2021-09-29
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

* [Core team membership updates](https://blog.rust-lang.org/2021/09/27/Core-team-membership-updates.html)
* [Rust Foundation Member Spotlight: Open Source Security](https://foundation.rust-lang.org/posts/2021-09-21-member-spotlight-open-source-security-software/)

### Project/Tooling Updates

* [SixtyFPS (GUI crate) weekly update for 27th of September 2021](https://sixtyfps.io/thisweek/2021-09-27.html)
* [This week in Fluvio #6: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0006/)
* [This week in Databend #9: an elastic and reliable cloud warehouse](https://datafuselabs.github.io/weekly/2021-09-29-databend-weekly/)
* [IntelliJ Rust Changelog #156](https://intellij-rust.github.io/2021/09/27/changelog-156.html)
* [Rust Analyzer #96](https://rust-analyzer.github.io/thisweek/2021/09/27/changelog-96.html)

### Observations/Thoughts

- [Common Newbie Mistakes and Bad PRactices in Rust: Bad Habits](https://adventures.michaelfbryan.com/posts/rust-best-practices/bad-habits/)
- [Why Rust in medical imaging? A reflection on modern technologies for next generation systems](https://www.bmd-software.com/news/why-rust-in-medical-imaging-a-reflection-on-modern-technologies-for-next-generation-systems)
* [Why we built a custom Rust library for Capture](https://dropbox.tech/application/why-we-built-a-custom-rust-library-for-capture)
* [audio] [Rust for Rustaceans by Jon Gjengset](https://rustacean-station.org/episode/038-jon-gjengset/)
* [video] [Rust Talks at the Linux Plumbers Conference 2021](https://www.reddit.com/r/rust/comments/pxz7at/rustlinux_plumbers_conference_2021/)

### Rust Walkthroughs

* [Building a static site generator in 100 lines of Rust](https://kerkour.com/blog/rust-static-site-generator/)
* [STM32F3’s Magnetometer | Use-cases & Reading Extraction (Part-1)](https://blog.knoldus.com/stm32f3-magnetometer-use-cases-reading-extraction-part-1/)
* [Reproducible cross-compilation for Rust (with Docker)](https://kerkour.com/blog/rust-reproducible-cross-compilation-with-docker/)
* [Rust Brain Teasers](https://www.pragprog.com/titles/hwrustbrain/rust-brain-teasers/)
* [Writing embedded firmware using Rust](https://www.anyleaf.org/blog/writing-embedded-firmware-using-rust)
* [Using Rust Cloudflare Workers: Serverless hCaptcha](https://dev.to/askrodney/using-rust-cloudflare-workers-serverless-hcaptcha-358g)
* [Deploy an ultra-fast blog in minutes with Eleventy and AssemblyLift (WebAssembly + Lambda + API Gateway + Rust)](https://dev.to/akkoro/deploy-an-ultra-fast-blog-in-minutes-with-eleventy-and-assemblylift-webassembly-lambda-api-gateway-rust-568l)
* [PT] [Rust - Ownership ?](https://dev.to/higordiego/rust-ownership-1hka)
* [video] [Hacking on rustc - Negative literals in indexing expressions](https://www.youtube.com/watch?v=OGihuce8rl8)
* [video] [Getting started with Rust 🦀 2021: 6. Library API design overhaul, async and more](https://www.youtube.com/watch?v=J_yGWdgeGQM)

### Miscellaneous

* [GitHub Advisory Database now supports Rust](https://github.blog/2021-09-23-github-advisory-database-now-supports-rust/)
* [_Rust for the Polyglot Programmer_ - introducing a new guide to the language](https://www.chiark.greenend.org.uk/~ianmdlvl/rust-polyglot/index.html)
* [DE] [Modernes Rust im Jahr 2021 @ Gesellschaft für Informatik](https://rg-rhein-main.gi.de/veranstaltung/modernes-rust-im-jahr-2021)

## Crate of the Week

This week's crate is [miette](https://crates.io/crates/miette), a library for error handling that is beautiful both in code and output.

Thanks to [Kat Marchán](https://users.rust-lang.org/t/crate-of-the-week/2704/965) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

265 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-09-20..2021-09-27

* [suggest both of immutable and mutable trait implementations](https://github.com/rust-lang/rust/pull/89263)
* [give better error for `macro_rules! name!`](https://github.com/rust-lang/rust/pull/89221)
* [validate builtin attributes for macro args](https://github.com/rust-lang/rust/pull/88680)
* [implement `#[must_not_suspend]`](https://github.com/rust-lang/rust/pull/88865)
* [support `#[track_caller]` on closures and generators](https://github.com/rust-lang/rust/pull/87064)
* [make `#[track_caller]` actually do stuff in `Steal::borrow`](https://github.com/rust-lang/rust/pull/89237)
* [revise never type fallback algorithm](https://github.com/rust-lang/rust/pull/88804)
* [don't use projection cache or candidate cache in intercrate mode](https://github.com/rust-lang/rust/pull/89125)
* [don't normalize opaque types with escaping late-bound regions](https://github.com/rust-lang/rust/pull/89285)
* [disable visible path calculation for `PrettyPrinter` in `Ok` path of compiler](https://github.com/rust-lang/rust/pull/89120)
* [enable new pass manager with LLVM 13](https://github.com/rust-lang/rust/pull/88243)
* [simplify `scoped_thread`](https://github.com/rust-lang/rust/pull/89104)
* [stabilize `Iterator::map_while`](https://github.com/rust-lang/rust/pull/89086)
* [use ZST for `fmt` unsafety](https://github.com/rust-lang/rust/pull/89139)
* [rustfmt: trailing comma on match block goes missing when guard is on its own line](https://github.com/rust-lang/rustfmt/pull/4998)
* [rustfmt: simplify and speed up search for local path based deps with `cargo fmt --all`](https://github.com/rust-lang/rustfmt/pull/4997)
* [clippy: demote `float_cmp` to pedantic](https://github.com/rust-lang/rust-clippy/pull/7692)
* [clippy: new lint `if_then_panic`](https://github.com/rust-lang/rust-clippy/pull/7669)
* [clippy: stop `excessive_precision` from suggesting a float truncation that is not shorter](https://github.com/rust-lang/rust-clippy/pull/7722)
* [clippy: don't lint `suspicious_else_formatting` inside proc-macros](https://github.com/rust-lang/rust-clippy/pull/7707)

### Rust Compiler Performance Triage

The largest story for the week are the massive improvements that come from enabling the new pass manager in LLVM which leads to consistent 5% to 30% improvements across almost all test cases. The regressions were mostly minor with clear paths for addressing the ones that were not made with some specific trade off in mind.

Triage done by **@rylev**.
Revision range: [7743c9..83f147](https://perf.rust-lang.org/?start=7743c9fadd64886d537966ba224b9c20e6014a59&end=83f147b3baf21acfc367a6da1045d212cd3957e4&absolute=false&stat=instructions%3Au)

4 Regressions, 4 Improvements, 3 Mixed; 0 of them in rollups

43 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-28.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Supertrait item shadowing](https://github.com/rust-lang/rfcs/pull/2845)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)


* [disposition: merge] [Stabilize RFC 2345: Allow panicking in constants](https://github.com/rust-lang/rust/issues/89006)
* [disposition: merge] [Perform type inference in range pattern](https://github.com/rust-lang/rust/pull/88090)
* [disposition: close] [Disable unused_must_use for statically known bools](https://github.com/rust-lang/rust/pull/88028)
* [disposition: merge] [Decide whether asm! and/or global_asm! should be exported from the prelude.](https://github.com/rust-lang/rust/issues/87228)
* [disposition: merge] [Make #[derive(A, B, ...)] cfg-eval its input only for A, B, ... and stabilize feature(macro_attributes_in_derive_output)](https://github.com/rust-lang/rust/pull/87220)
* [disposition: merge] [Make `*const (), *mut ()` okay for FFI](https://github.com/rust-lang/rust/pull/84267)

### New RFCs

* [[Help wanted] First draft of patchfile RFC](https://github.com/rust-lang/rfcs/pull/3177)
* [Multiple artifact deps on the same crate with different names, for different targets](https://github.com/rust-lang/rfcs/pull/3176)

## Upcoming Events

### Online

* [October 2, 2021 - Rust Graphics meetup](https://github.com/gfx-rs/meetup)
* [October 5, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/280628523/)

### North America

* [October 13, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccnbrb/)

### Europe

* [September 22, 2021, Berlin, DE - Rust and Tell - an onsite event - Berline.rs](https://berline.rs/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Enso**

* [Senior Rust Developer (Remote)](https://github.com/enso-org/hiring/blob/main/people/senior-rust-developer.md)

**Stockly**

* [Back-end developer - Engine team (Rust, GRPC, PostgreSQL) (Remote)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)

**Timescale**

* [Senior Toolkit Engineer - Database (Remote)](https://boards.greenhouse.io/timescale/jobs/5542785002)

**ChainSafe**

* [Protocol Engineer for Filecoin Forest (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999769790643-protocol-engineer-forest-rust-)
* [Rust Engineer for Substrate (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999769885107-rust-engineer-substrate-)

**Kraken**

* [Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Senior Banking Engineer - Rust (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

This week we have two great quotes!

> The signature of your function is your contract with not only the compiler, but also users of your function.

– [Quine Dot on rust-users](https://users.rust-lang.org/t/why-rust-lifetime-elision-cannot-inference-the-proper-lifetime-annotations-on-functions/65106/3)

> Do you want to know what was harder than learning lifetimes? Learning the same lessons through twenty years of making preventable mistakes.

– [Zac Burns in his RustConf talk](https://www.youtube.com/watch?v=4_Jg-rLDy-Y&t=1658s)

Thanks to [Daniel H-M](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1116) and [Erik Zivkovic](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1117) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
