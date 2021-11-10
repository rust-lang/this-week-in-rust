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

### Official

### Foundation

### Project/Tooling Updates

### Research Papers

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [chumsky](https://github.com/zesterer/chumsky), a friendly parser combinator crate.

Thanks to [Jan Riemer](https://users.rust-lang.org/t/crate-of-the-week/2704/981) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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
* [hashbrown: implement From<array> on HashSet and HashMap](https://github.com/rust-lang/hashbrown/pull/298)
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

The only significant regressions were 1. two PRs that slowed down doc
generation, and 2. some slowdown from the new lints to flag occurrences of
Unicode bidirectional control characters. The doc generation regression is being
investigated.

Triage done by **@pnkfelix**.
Revision range: [3c8f00..6384dc](https://perf.rust-lang.org/?start=3c8f001d454b1b495f7472d8430ef8fdf10aac11&end=6384dca100f3cedfa031a9204586f94f8612eae5&absolute=false&stat=instructions%3Au)

6 Regressions, 3 Improvements, 1 Mixed; 4 of them in rollups
39 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-11-02.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Multiple artifact deps on the same crate with different names, for different targets](https://github.com/rust-lang/rfcs/pull/3176)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Static async fn in traits](https://github.com/rust-lang/rfcs/pull/3185)
* [disposition: merge] [Constrained Naked Functions](https://github.com/rust-lang/rfcs/pull/2972)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize format args capture](https://github.com/rust-lang/rust/pull/90473)
* [disposition: merge] [Stabilize -Z symbol-mangling-version=v0 as -C symbol-mangling-version=v0](https://github.com/rust-lang/rust/pull/90128)
* [disposition: merge] [Stabilize -Z strip as -C strip](https://github.com/rust-lang/rust/pull/90058)
* [disposition: merge] [Stabilize `const_raw_ptr_deref` for `*const T`](https://github.com/rust-lang/rust/pull/89551)
* [disposition: merge] [Clarification of default socket flags](https://github.com/rust-lang/rust/pull/88805)
* [disposition: merge] [use CLOCK_BOOTTIME in `Instant::now`](https://github.com/rust-lang/rust/pull/88714)
* [disposition: merge] [GATs: Decide whether to have defaults for `where Self: 'a`](https://github.com/rust-lang/rust/issues/87479)

### New RFCs

* [take on bool](https://github.com/rust-lang/rfcs/pull/3189)
* [New Cargo and Rust options to support embedding Natvis into a PDB](https://github.com/rust-lang/rfcs/pull/3191)

## Upcoming Events

### Online

* [November 3, 2021, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/281258179)
* [November 9, 2021, San Diego, CA, US - San Diego Rust November 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/281801412)
* [November 9, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [November 9, 2021, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccpbmb/)
* [November 10, 2021, Malaysia - Rust Meetup - Rust Malaysia](https://discord.gg/9Xj8H2EXTD)
* [November 11, 2021 - Rust For Linux: Writing Safe Abstractions & Drivers - The Linux Foundation](https://linuxfoundation.org/webinars/rust-for-linux-writing-abstractions-and-drivers/)
* [November 17, 2021, Vancouver, BC, CA - Borrowing and Lifetimes through Metaphors - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccpbwb/)

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

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
