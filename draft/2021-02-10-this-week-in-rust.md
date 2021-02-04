Title: This Week in Rust 377
Number: 377
Date: 2021-02-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

# Crate of the Week

This week's crate is [fancy-regex](https://github.com/fancy-regex/fancy-regex) a regex implementation using regex for speed and backtracking for fancy features.

Thanks to [Benjamin Minixhofer](https://users.rust-lang.org/t/crate-of-the-week/2704/877) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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

* [RFC: Pointer metadata & VTable](https://github.com/rust-lang/rfcs/pull/2580)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Allow leading | anywhere we allow or-patterns](https://github.com/rust-lang/rust/issues/81415)
* [disposition: merge] [libtest: allow multiple filters](https://github.com/rust-lang/rust/pull/81356)
* [disposition: merge] [Stabilize remaining integer methods as `const fn`](https://github.com/rust-lang/rust/pull/80962)
* [disposition: merge] [Add an impl of Error on `Arc<impl Error>`.](https://github.com/rust-lang/rust/pull/80553)
* [disposition: merge] [expand/resolve: Turn `#[derive]` into a regular macro attribute](https://github.com/rust-lang/rust/pull/79078)
* [disposition: merge] [Tracking Issue for `partition_point`](https://github.com/rust-lang/rust/issues/73831)

## New RFCs

* [consolidated usage of feature-name header field](https://github.com/rust-lang/rfcs/pull/3071)
* [Use more common 'tests' module name over 'test' in examples](https://github.com/rust-lang/rfcs/pull/3070)

# Upcoming Events

### Online
* [February 4, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccdbgb/)
* [February 4, Budapest, HU - Rust meetup S03! - Rust Hungary Meetup](https://www.meetup.com/Rust-Hungary-Meetup/events/275579644/)
* [February 7, Indianapolis, IN, US - Monthly Meetup - Indy.rs](https://www.meetup.com/indyrs/events/246726699/)
* [February 9, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccdbmb/)
* [February 9, Saarbücken, Saarland, DE - Meetup: 8u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/275720207/)

### North America
* [February 10, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccdbnb/)
* [February 11, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccdbpb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

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
