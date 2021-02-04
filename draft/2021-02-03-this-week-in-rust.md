Title: This Week in Rust 376
Number: 376
Date: 2021-02-03
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No official blog posts this week.

### Newsletters
* [RiB Newsletter #20 - Old fashioned chill](https://www.reddit.com/r/rust/comments/lc3tir/rib_newsletter_20_old_fashioned_chill/)

### Project/Tooling Updates
* [rust-analyzer Changelog #62](https://rust-analyzer.github.io/thisweek/2021/02/01/changelog-62.html)
* [IntelliJ Rust Changelog #140](https://intellij-rust.github.io/2021/02/01/changelog-140.html)
* [Llama Rust SDK preview 0.1.3](https://ericsink.com/entries/llama_rust_013.html)
* [Gfx-rs Release of v0.7](https://gfx-rs.github.io/2021/02/02/release-0.7.html)
* [Open-sourcing Datanymizer: in-flight template-driven data anonymization](https://evrone.com/datanymizer)
* [Announcing Step/Dir - Universal Stepper Motor Interface](https://flott-motion.org/news/announcing-step-dir/)
* [Announcing RampMaker - Stepper Motor Acceleration Ramp Generator](https://flott-motion.org/news/announcing-ramp-maker/)
* [Krustlet v0.6.0](https://github.com/deislabs/krustlet/releases/tag/v0.6.0)

### Observations/Thoughts
* [Is Rust a Functional Programming Language?](https://robert.kra.hn/posts/2021-02-03_is-rust-fp/)
* [Rust Collections Case Study: BTreeMap](https://cglab.ca/~abeinges/blah/rust-btree-case/)
* [Saving some allocations](https://vorner.github.io/2021/01/31/saving-some-allocations.html)
* [Bringing Stack Clash Protection to Clang/X86 - the Open Source Way](https://blog.llvm.org/posts/2021-01-05-stack-clash-protection/)
* [Announcing Krator: Build Kubernetes Operators with state machines.](https://deislabs.io/posts/introducing-krator/)

### Rust Walkthroughs

* [Returning Rust Iterators](https://depth-first.com/articles/2020/06/22/returning-rust-iterators/)
* [How to Read Rust Functions, Part 1](https://www.possiblerust.com/guide/how-to-read-rust-functions-part-1)
* [Parsing PDF Documents in Rust](https://adventures.michaelfbryan.com/posts/parsing-pdfs-in-rust/?utm_source=reddit&utm_medium=social&utm_campaign=parsing-pdf-documents)
* [Building and deploying Rust utilities](https://robert.kra.hn/posts/2021-02-01_cross-compile-rust/)
* [Learning to Fly: Let's create a simulation in Rust! (pt 2)](https://pwy.io/en/posts/learning-to-fly-pt2/)
* [Rust made my open source project 1000x faster](https://www.reddit.com/r/rust/comments/lazq0i/rust_made_my_open_source_project_1000x_faster/)
* [Introducing Drogue Device](https://blog.drogue.io/introducing-drogue-device/)
* [3 Things to Try When You Can't Make A Trait Object](https://www.possiblerust.com/pattern/3-things-to-try-when-you-can-t-make-a-trait-object)
* [Making concurrency fearless with Rust (for C++ developers)](https://radekvit.medium.com/making-concurrency-fearless-with-rust-for-c-developers-d5d8da50a452)
* [Exploring WebSocket with Rust and Tide](https://javierviola.com/post/exploring-websocket-with-rust-and-tide/)
* [Macros in Rust: A tutorial with examples](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/)
* [PL] [CrabbyBird #5 Obsługa kolizji](https://postacnormalna.pl/crabbybird-5-obsluga-kolizji/)


### Miscellaneous
* [RustBelt: Securing the Foundations of the Rust Programming Language](https://people.mpi-sws.org/~dreyer/papers/rustbelt/paper.pdf)
* [Polymorphisation: Improving Rust compilation times through intelligent monomorphisation](https://davidtw.co/media/masters_dissertation.pdf)
* [A Memory Safe TLS Module for the Apache HTTP Server](https://www.abetterinternet.org/post/memory-safe-tls-apache/)
* [Chats with James: 006 - Bryan Cantrill](https://jamesmunns.com/podcast/006-bryan/)
* [Long range networking with LoRa: an overview featuring a Rust modem driver](https://tweedegolf.nl/blog/51/long-range-networking-with-lora-an-overview)

# Crate of the Week

This week's crate is [fancy-regex](https://github.com/fancy-regex/fancy-regex) a regex implementation using regex for speed and backtracking for fancy features.

Thanks to [Benjamin Minixhofer](https://users.rust-lang.org/t/crate-of-the-week/2704/877) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [dotenv-linter has several good first issues](https://github.com/dotenv-linter/dotenv-linter/issues)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

323 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-01-25..2021-02-01

* [rustc: stabilize `-Zrun-dsymutil` as `-Csplit-debuginfo`](https://github.com/rust-lang/rust/pull/79570)
* [point only at generic arguments when they are unexpected](https://github.com/rust-lang/rust/pull/79591)
* [improve diagnostics for Precise Capture](https://github.com/rust-lang/rust/pull/81062)
* [account for existing `_` field pattern when suggesting `..`](https://github.com/rust-lang/rust/pull/81422)
* [tweak suggestion for missing field in patterns](https://github.com/rust-lang/rust/pull/81416)
* [visit only statements in always live locals](https://github.com/rust-lang/rust/pull/81440)
* [avoid memory allocation when removing dead blocks](https://github.com/rust-lang/rust/pull/81470)
* [make hitting the recursion limit in projection non-fatal](https://github.com/rust-lang/rust/pull/81055)
* [`clashing_extern_declarations`: use symbol interning to avoid string alloc](https://github.com/rust-lang/rust/pull/81453)
* [miri: add random failures to `compare_exchange_weak`](https://github.com/rust-lang/miri/pull/1686)
* [trying to `Vec::shrink_to` greater than capacity should be no-op](https://github.com/rust-lang/rust/pull/81335)
* [implement Rust 2021 panic](https://github.com/rust-lang/rust/pull/80851)
* [implement missing `AsMut<str>` for `str`](https://github.com/rust-lang/rust/pull/80279)
* [implement `io::Seek` for `io::Empty`](https://github.com/rust-lang/rust/pull/78044)
* [let `io::copy` reuse `BufWriter` buffers](https://github.com/rust-lang/rust/pull/78641)
* [add 'Box::downcast()` for `dyn Any + Send + Sync`](https://github.com/rust-lang/rust/pull/80945)
* [add `unwrap_unchecked()` methods for `Option` and `Result`](https://github.com/rust-lang/rust/pull/80876)
* [add `core::stream::Stream`](https://github.com/rust-lang/rust/pull/79023)
* [stabilize `core::slice::fill_with`](https://github.com/rust-lang/rust/pull/81048)
* [stabilize `unsigned_abs`](https://github.com/rust-lang/rust/pull/80959)
* [stabilize raw ref macros](https://github.com/rust-lang/rust/pull/80886)
* [stabilize by-value `[T; N]` iterator `core::array::IntoIter`](https://github.com/rust-lang/rust/pull/80470)
* [stabilise `cargo test -- --include-ignored`](https://github.com/rust-lang/rust/pull/80053)
* [stabilize `Arc::`{`increment`, `decrement`}`_strong_count`](https://github.com/rust-lang/rust/pull/79285)
* [stabilize `Seek::stream_position` (feature `seek_convenience`)](https://github.com/rust-lang/rust/pull/70904)
* [optimize decimal formatting of 128-bit integers](https://github.com/rust-lang/rust/pull/81484)
* [stabilize int_bits_const](https://github.com/rust-lang/rust/pull/81590)
* [hashbrown: reduce the amount of llvm IR instantiated](https://github.com/rust-lang/hashbrown/pull/205)
* [libtest: wait for test threads to exit after they report completion](https://github.com/rust-lang/rust/pull/81367)
* [cargo: impl warn for locked install without Cargo.lock](https://github.com/rust-lang/cargo/pull/9108)
* [rustdoc: improve docblock readability on small screen](https://github.com/rust-lang/rust/pull/81563)

## Rust Compiler Performance Triage

Another week dominated by rollups, most of which had relatively small changes
with unclear causes embedded. Overall no major changes in performance this week.

Triage done by **@simulacrum**.
Revision range: [1483e67addd37d9bd20ba3b4613b678ee9ad4d68..f6cb45ad01a4518f615926f39801996622f46179](https://perf.rust-lang.org/?start=1483e67addd37d9bd20ba3b4613b678ee9ad4d68&end=f6cb45ad01a4518f615926f39801996622f46179&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 1 Mixed

3 of them in rollups

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-02-02.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Allow "artifact dependencies" on bin, cdylib, and staticlib crates](https://github.com/rust-lang/rfcs/pull/3028)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Rust 2021 Roadmap](https://github.com/rust-lang/rfcs/pull/3037)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Remove requirement that forces symmetric and transitive PartialEq impls to exist](https://github.com/rust-lang/rust/pull/81198)
* [disposition: merge] [Stabilize `core::slice::fill_with`](https://github.com/rust-lang/rust/pull/81048)
* [disposition: merge] [Add Box::downcast() for dyn Any + Send + Sync](https://github.com/rust-lang/rust/pull/80945)
* [disposition: merge] [Stabilize by-value `[T; N]` iterator `core::array::IntoIter`](https://github.com/rust-lang/rust/pull/80470)
* [disposition: merge] [Implement missing `AsMut<str>` for `str`](https://github.com/rust-lang/rust/pull/80279)
* [disposition: merge] [stabilise `cargo test -- --include-ignored`](https://github.com/rust-lang/rust/pull/80053)
* [disposition: merge] [Stabilize `peekable_next_if`](https://github.com/rust-lang/rust/pull/80011)
* [disposition: merge] [rustc: Stabilize `-Zrun-dsymutil` as `-Csplit-debuginfo`](https://github.com/rust-lang/rust/pull/79570)
* [disposition: merge] [Stabilize Arc::{increment,decrement}_strong_count](https://github.com/rust-lang/rust/pull/79285)
* [disposition: merge] [expand/resolve: Turn `#[derive]` into a regular macro attribute](https://github.com/rust-lang/rust/pull/79078)
* [disposition: merge] [Implement io::Seek for io::Empty](https://github.com/rust-lang/rust/pull/78044)
* [disposition: merge] [Tracking Issue for `feature(int_bits_const): <integer>::BITS`](https://github.com/rust-lang/rust/issues/76904)
* [disposition: merge] [Tracking Issue for `fmt::Arguments::as_str()`](https://github.com/rust-lang/rust/issues/74442)


## New RFCs

* [Change visibility scoping rules for macro_rules macros](https://github.com/rust-lang/rfcs/pull/3067)

# Upcoming Events

### Online
* [Februar 2, Dublin, IE - Rust Dublin Remote February Meetup - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/275827557/)
* [February 2, Buffalo, NY, US - Buffalo Rust User Group - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/275593411/)
* [February 2, Denver, CO, US - ML in Rust, implementing logistic and linear regression from scratch - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/275352662/)
* [February 3, Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/275986420/)
* [February 4, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccdbgb/)
* [February 4, Budapest, HU - Rust meetup S03! - Rust Hungary Meetup](https://www.meetup.com/Rust-Hungary-Meetup/events/275579644/)
* [February 7, Indianapolis, IN, US - Monthly Meetup - Indy.rs](https://www.meetup.com/indyrs/events/246726699/)
* [February 9, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccdbmb/)
* [February 9, Saarbücken, Saarland, DE - Meetup: 8u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/275720207/)

### North America
* [February 10, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccdbnb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Software Engineer at Aleph Alpha (Heidelberg, Germany)](https://aleph-alpha.de/career/software-engineer-rust)
* [Distributed Systems (Rust) Developer at Signal (Remote US Timezone)](https://jobs.lever.co/signal/7aa1ff1f-bd43-4359-82c7-8703d8b842d9)
* [3D Driver Development Engineer - Rust tooling for GPUs at AMD (Boxborough, MA, USA)](https://jobs.amd.com/job/Boxborough-3D-Driver-Development-Engineer-Tools-82766-Mass/709592800/)
* [Senior Software Engineer (Rust & C++) at NZXT (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Backend Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105)
* [Core Backend - Developer Experience Engineer at Kraken (Remote)](https://jobs.lever.co/kraken/1c6b290f-e430-430d-9b40-a258d07686b0)
* [Rust API SDET at Kraken (Remote)](https://jobs.lever.co/kraken/5ec9958a-529c-4bae-89b3-0d1a104cbd81)
* [Software Engineer - Trading Technology (Rust) at Kraken (Remote)](https://jobs.lever.co/kraken/4485f672-dc5f-4e49-a10b-2b0399e28a8d)
* [Several Positions at Fluence Labs (Remote)](https://fluence.network/join.html)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

This time we had two very good quotes, I could not decide, so here are both:

> What I have been learning ... was not Rust in particular, but how to write sound software in general, and that in my opinion is the largest asset that the rust community tough me, through the language and tools that you developed.
>
> Under this prism, it was really easy for me to justify the step learning curve that Rust offers: I wanted to learn how to write sound software, writing sound software is really hard , and the Rust compiler is a really good teacher.
>
> \[...\]
>
> This ability to identify unsound code transcends Rust's language, and in my opinion is heavily under-represented in most cost-benefit analysis over learning Rust or not.

– [Jorge Leitao on rust-users](https://users.rust-lang.org/t/thank-you-for-the-teaching-on-how-to-write-sound-software/54714)

and

> Having a fast language is not enough (ASM), and having a language with strong type guarantees neither (Haskell), and having a language with ease of use and portability also neither (Python/Java). Combine all of them together, and you get the best of all these worlds.
>
> Rust is not the best option for any coding philosophy, it’s the option that is currently the best at combining all these philosophies.

– [/u/CalligrapherMinute77 on /r/rust](https://www.reddit.com/r/rust/comments/l7vvo9/writing_a_proposal_to_use_rust_at_work/gl9lfk8)

Thanks to [2e71828](https://users.rust-lang.org/t/twir-quote-of-the-week/328/996) and [Rusty Shell](https://users.rust-lang.org/t/twir-quote-of-the-week/328/998) for their respective suggestions.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
