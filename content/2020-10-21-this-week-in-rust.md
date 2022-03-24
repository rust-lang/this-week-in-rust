Title: This Week in Rust 361
Number: 361
Date: 2020-10-21
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official
* [Marking issues as regressions](https://blog.rust-lang.org/2020/10/20/regression-labels.html)
* [Inside] [Lang team Backlog Bonanza and Project Proposals](https://blog.rust-lang.org/inside-rust/2020/10/16/Backlog-Bonanza.html)

### Tooling
* [rust-analyzer Changelog #47](https://rust-analyzer.github.io/thisweek/2020/10/19/changelog-47.html)
* [IntelliJ Rust Changelog #133](https://intellij-rust.github.io/2020/10/19/changelog-133.html)

### Observations/Thoughts
* [Fearless concurrency: how Clojure, Rust, Pony, Erlang and Dart let you achieve that.](https://sites.google.com/a/athaydes.com/renato-athaydes/posts/fearlessconcurrencyhowclojurerustponyerlanganddartletyouachievethat)
* [Shock Result<>?: Rust faster than Python in one test of file parsing](http://www.coralbark.net/blog/technology/2020/10/shock-result-rust-faster-than-python-in-one-test-of-file-parsing/)
* [Building a Recipe Manager - Part 2 - Druid Experience Report](https://bheisler.github.io/post/recipe-manager-part-2-druid-experience-report/)
* [No, C++ still isn't cutting it.](https://da-data.blogspot.com/2020/10/no-c-still-isnt-cutting-it.html)
* [A pitfall of Rust's move/copy/drop semantics and zeroing data](https://benma.github.io/2020/10/16/rust-zeroize-move.html)
* [Proving that 1 + 1 = 10 in Rust](https://tavianator.com/2020/one_plus_one.html)
* [Study of std::io::Error](https://matklad.github.io/2020/10/15/study-of-std-io-error.html)
* [Fun With Rust's Traits](https://samwho.dev/blog/fun-with-rust-traits/)

### Learn Simple Rust
* [Arrays, vectors and slices in Rust](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/)
* [Building Your Own Error Type: Part 1](https://yaah.dev/building-your-own-error-type)
* [Lifetimes in Rust](https://blog.thoughtram.io/lifetimes-in-rust/)
* [Piece by Piece: Write Readable Rust Code](https://impl.dev/posts/write-readable-rust-code/)
* [Are out parameters idiomatic in Rust?](https://steveklabnik.com/writing/are-out-parameters-idiomatic-in-rust)
* [Non-Generic Inner Functions](https://www.possiblerust.com/pattern/non-generic-inner-functions)
* [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/)
* [Create Your Own PineTime Watch Face in Rust... And Publish on crates.io](https://lupyuen.github.io/pinetime-rust-mynewt/articles/watchface)
* [Either Types for Rust](https://dev.to/sirech/either-types-for-rust-46k4)
* [Rust syntax: What is the questionmark?](https://dev.to/nickymeuleman/rust-syntax-what-the-questionmark-2n58)
* [Private Methods on a Public Trait](https://jack.wrenn.fyi/blog/private-trait-methods/)
* [Learn Rust with Benford's Law](https://gliderkite.github.io/posts/learn-rust-with-benford/)
* [How to Write Hygienic Rust Macros](https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1)
* [video] [Rust Linz, October 2020 - Valentin Tolmer - How not to rely on inheritance](https://youtu.be/m6Gee5kNe7U)

### Learn More Rust
* [Building a runtime reflection system for Rust 🦀️ (Part 2)](https://www.osohq.com/post/runtime-reflection-pt-2)
* [Compile Rust for Raspberry Pi ARM](https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050)
* [Basic non-blocking IO using epoll in Rust](https://zupzup.org/epoll-with-rust/)
* [Debugging async generator errors in Rust](https://meltware.com/2020/10/21/rust-async-nonsense.html)
* [video] [(Live Coding) Audio adventures in Rust: Spotify integration](https://youtu.be/5q4NB9WdYIo)
* [video] [Rust Linz, October 2020 - Matthias Heiden - Writing a Kernel Driver with Rust](https://youtu.be/wREGR7QQHco)

### Project Updates
* [Announcing Tokio 0.3 and the path to 1.0](https://tokio.rs/blog/2020-10-tokio-0-3)
* [oso, an open-source policy engine for authorization written in Rust](https://github.com/osohq/oso), released [version 0.7.0 of their authorization library for Rust projects!](https://docs.rs/oso/0.7.0/oso/)
* ⚡️ [Dotenv-linter v2.2.0: find and fix problems in .env files](https://evrone.com/dotenv-linter-v220)
* [Version 0.3.0 of cargo-wipe has been released](https://github.com/mihai-dinculescu/cargo-wipe)

### Miscellaneous
* [A new look, tickets and what's to come](https://blog.rustfest.eu/a-new-look)
* [ICU4X Project Announcement](https://github.com/unicode-org/icu4x/wiki/ICU4X-Project-Announcement)
* [Kata Containers rewritten in Rust gets a major speed boost](https://www.zdnet.com/article/kata-containers-rewritten-in-rust-and-gets-a-major-speed-boost/)
* [Assorted thoughts on zig (and rust)](https://scattered-thoughts.net/writing/assorted-thoughts-on-zig-and-rust/)
* [Flask Creator Armin Ronacher Interview](https://evrone.com/armin-ronacher-interview)
* [A recipe for start using Rust actix-web and launch chrome 🚀](https://itnext.io/a-recipe-for-starting-actix-web-server-and-launch-chrome-b792987935a)
* [Sailfish OS 3.4 Released with Experimental Rust Support, Finally Eyeing 64-bit ARM](https://www.phoronix.com/scan.php?page=news_item&px=Sailfish-OS-3.4-Released)
* [Create Your Own PineTime Watch Face in Rust... And Publish on crates.io](https://lupyuen.github.io/pinetime-rust-mynewt/articles/watchface)
* [Getting started with Datalog & Rust for program analysis](https://hexgolems.com/2020/10/getting-started-with-ddlog/)
* [Three open source Sonos projects: efficient embedded development in Rust](https://tech-blog.sonos.com/posts/three-open-source-sonos-projects-in-rust/)

# Crate of the Week

This week's crate is [icu4x](https://github.com/unicode-org/icu4x), the Unicode Consortium's official crate for dealing with i18n in resource constrained environments.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/828) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [GitUI: Good First Issue](https://github.com/extrawurst/gitui/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
* [this-week-in-rust: Very light font can be difficult to read](https://github.com/rust-lang/this-week-in-rust/issues/708)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

398 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-10-12..2020-10-19

* [make set_span take `mut self`](https://github.com/rust-lang/rust/pull/78047)
* [resolve: further improvements to "try using the enum's variant" diagnostic](https://github.com/rust-lang/rust/pull/77855)
* [`min_const_generics` diagnostics improvements](https://github.com/rust-lang/rust/pull/77825)
* [make sure arenas don't allocate bigger than `HUGE_PAGE`](https://github.com/rust-lang/rust/pull/78058)
* [make `ObligationForest` more efficient](https://github.com/rust-lang/rust/pull/77908)
* [add `std::thread::available_concurrency`](https://github.com/rust-lang/rust/pull/74480)
* [remove `shrink_to_fit` from default `ToString::to_string` implementation](https://github.com/rust-lang/rust/pull/77997)
* [add `str::`{`Split`, `RSplit`, `SplitN`, `RSplitN`, `SplitTerminator`, `RSplitTerminator`, `SplitInclusive`}`::as_str` methods](https://github.com/rust-lang/rust/pull/75265)
* [liballoc: `VecDeque`: Add binary search functions](https://github.com/rust-lang/rust/pull/77751)
* [BTreeMap: fix gdb provider on `BTreeMap` with ZST keys or values](https://github.com/rust-lang/rust/pull/77788)
* [hashbrown: remove the need for unwrap when using `ProbeSeq`](https://github.com/rust-lang/hashbrown/pull/208)

## Rust Compiler Performance Triage

* [2020-10-21](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-21.md):
4 Regressions, 7 Improvements, 0 Mixed

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-21.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Promote aarch64-unknown-linux-gnu to a Tier-1 Rust target](https://github.com/rust-lang/rfcs/pull/2959)
* [YieldSafe auto trait](https://github.com/rust-lang/rfcs/pull/2890)
* [Access to traits' associated functions and constants from trait objects](https://github.com/rust-lang/rfcs/pull/2886)
* [Variadic tuples](https://github.com/rust-lang/rfcs/pull/2775)
* [RFC for a match based surface syntax to get pointer-to-field](https://github.com/rust-lang/rfcs/pull/2666)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)
* [disposition: merge] [Allow making `RUSTC_BOOTSTRAP` conditional on the crate name](https://github.com/rust-lang/rust/pull/77802)
* [disposition: merge] [stop promoting union field accesses in 'const'](https://github.com/rust-lang/rust/pull/77526)
* [disposition: merge] [passes: `check_attr` on more targets](https://github.com/rust-lang/rust/pull/77015)
* [disposition: merge] [Stabilize `Poll::is_ready` and `is_pending` as const](https://github.com/rust-lang/rust/pull/76227)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [October 22. Edinburgh, UK - Fluence: interface-types for server-side WebAssembly modules - Rust Edinburgh](https://www.meetup.com/rust-edi/events/273685985)
* [October 27. Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcnbkc/)
* [October 29. Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbmc/)

### Asia Pacific
* [November 1. Auckland, NZ - Rust meetup - Introduction to Rust - Rust AKL](https://www.meetup.com/rust-akl/events/266876718/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Back End Engineer at Hopin (Remote)](https://hopin.to/careers/902859)
* [Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Senior Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105)
* [Senior Full Stack Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)
* [Software Engineer - Trading Technology (Rust) at Kraken (Remote)](https://jobs.lever.co/kraken/4485f672-dc5f-4e49-a10b-2b0399e28a8d)
* [Rust Engineer, Desktop GUI - Cryptowatch at Kraken (Remote)](https://jobs.lever.co/kraken/2442ee5c-56b6-4a73-a477-8cdda2b218d5)
* [Backend Engineer, Kraken Futures - Rust at Kraken(Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Full remote Rust developer, long-term contract (French CDI) at Massa Labs (Remote)](https://massa.network/#jobs)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> And it's true that a lot of stuff requires a "sufficiently smart compiler" but really it's 2020, if your compiler isn't serving you breakfast in bed you need to be upping your expectations.

- [Jubilee on the Rust Zulip](https://rust-lang.zulipchat.com/#narrow/stream/257879-project-portable-simd/topic/The.20movemasquerade/near/212794818)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/949) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/jg7hkt/this_week_in_rust_361/)</small>
