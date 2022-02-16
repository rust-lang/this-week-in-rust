Title: This Week in Rust 430
Number: 430
Date: 2022-02-16
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

### Newsletters

### Observations/Thoughts

* [My Rust Frontend Experiences](https://blog.urth.org/2022/02/08/my-rust-frontend-experiences/)

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [document-features](https://crates.io/crates/document-features), a small proc macro to parse your `Cargo.toml` and generate docs for your features.

Thanks to [Tobias Hunger](https://users.rust-lang.org/t/crate-of-the-week/2704/1017) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

301 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-01-31..2022-02-07

* [add new target armv7-unknown-linux-uclibceabi (softfloat)](https://github.com/rust-lang/rust/pull/92383)
* [continue work on associated const equality](https://github.com/rust-lang/rust/pull/93285)
* [clarify error on casting larger integers to char](https://github.com/rust-lang/rust/pull/91939)
* [fix suggestion to slice if scrutinee is a `Result` or `Option`](https://github.com/rust-lang/rust/pull/91343)
* [if an integer is entered with an upper-case base prefix (0Xbeef, 0O755, 0B1010), suggest to make it lowercase](https://github.com/rust-lang/rust/pull/93019)
* [add rustc lint, warning when iterating over hashmaps 2](https://github.com/rust-lang/rust/pull/92584)
* [borrowck: fix help on mutating `&self` in `async fn`s](https://github.com/rust-lang/rust/pull/93221)
* [perf: compress amount of hashed bytes for `isize` values in StableHasher](https://github.com/rust-lang/rust/pull/93432)
* [perf: use `fold_list` in `try_super_fold_with` for `SubstsRef`](https://github.com/rust-lang/rust/pull/93643)
* [codegen\_gcc: implement simd_neg](https://github.com/rust-lang/rustc_codegen_gcc/pull/125)
* [optimize `core::str::Chars::count`](https://github.com/rust-lang/rust/pull/90414)
* [impl `Arc::unwrap_or_clone`](https://github.com/rust-lang/rust/pull/91589)
* [carefully remove bounds checks from some chunk iterator functions](https://github.com/rust-lang/rust/pull/86988)
* [Impl arithmetic (assign-)ops traits for `Wrapping<_>` for rust 1.60.0](https://github.com/rust-lang/rust/pull/93208)
* [implement `RawWaker` and `Waker` getters for underlying pointers](https://github.com/rust-lang/rust/pull/91828)
* [rustdoc: collect traits in scope for foreign inherent impls](https://github.com/rust-lang/rust/pull/93539)
* [clippy: add `explicit_write` suggestions for `write!`s with format args](https://github.com/rust-lang/rust-clippy/pull/8365)
* [clippy: add lint `transmute_undefined_repr`](https://github.com/rust-lang/rust-clippy/pull/8398)
* [clippy: fix ICE in `ptr_arg`](https://github.com/rust-lang/rust-clippy/pull/8387)
* [clippy: split matches](https://github.com/rust-lang/rust-clippy/pull/8400)
* [clippy: `chars_next_cmp` fix unescaped suggestion](https://github.com/rust-lang/rust-clippy/pull/8376)
* [clippy: fix `explicit_counter_loop` suggesting `.into_iter()`, despite that triggering `into_iter_on_ref` in some cases](https://github.com/rust-lang/rust-clippy/pull/8382)
* [clippy: make `unwrap_used` also trigger on `.get().unwrap()`](https://github.com/rust-lang/rust-clippy/pull/8372)
* [clippy: warn if we find multiple clippy configs](https://github.com/rust-lang/rust-clippy/pull/8326)
* [rustfmt: fix doc of generic items formmating error](https://github.com/rust-lang/rustfmt/pull/5124)
* [rustfmt: fix import_granularity option when the use tree has an alias](https://github.com/rust-lang/rustfmt/pull/5209)
* [rustfmt: handle non-ascii character at boundary](https://github.com/rust-lang/rustfmt/pull/5089)

### Rust Compiler Performance Triage

A week with a number of correctness-related regressions, and a few small
cleanups yielding good performance results. Overall an improvement, particularly
for incremental benchmarks.

Triage done by **@simulacrum**.
Revision range: [1ea48517..775e480](https://perf.rust-lang.org/?start=1ea4851715893ee3f365a8ef09d47165e9a7864f&end=775e480722c7aba6ff4ff3ccec8c1f4639ae7889&absolute=false&stat=instructions%3Au)

4 Regressions, 2 Improvements, 5 Mixed; 1 of them in rollups
27 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-02-08.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered final comment period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: close] [Tracking issue for naked fns (RFC #1201)](https://github.com/rust-lang/rust/issues/32408)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Add a process_group method to UNIX CommandExt](https://github.com/rust-lang/rfcs/pull/3228)

## Upcoming Events

Rusty Events between 2/9/2022 - 3/9/2022 🦀

### Online

* [February 9, 2022 | Boulder, CO, US | **Monthly Meetup** | Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/283700330/)
* [February 9, 2022 | Los Angeles, CA, US | **Raphael Tessmer & Celeste, finding craters on a rusty planet** (Virtual) | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/283232930/)
* [February 9, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545292)
* [February 10, 2022 | San Diego, CA, US | **San Diego Rust February 2022 Tele-Meetup** | San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/283655575)
* [February 15, 2022 | Berlin, DE | **Rust Hack and Learn** | OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/events/283633079/)
* [February 15, 2022 | Indianapolis, IN, US | **Indy.rs - with Social Distancing** | Indy Rust](https://www.meetup.com/indyrs/events/283538948)
* [February 15, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/283351974/)
* [February 16, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out Night** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/283260386/)
* [February 17, 2022 | München, DE | **Rust - beyond "Hello World"**| Agile Softwareentwicklung München](https://www.meetup.com/maibornwolff-software-engineering-netzwerk/events/283379985)
* [February 17, 2022 | Nürnberg, DE | **Rust Nürnberg online #10**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/283545751/)
* [February 17, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545308)
* [February 17, 2022 | Würzburg, DE | **Meet and chat about Rust** | Rust Würzburg Meetup Group](https://www.meetup.com/rust-wurzburg-meetup-group/events/283609518)
* [February 22, 2022 | Dallas, TX, US | **Last Tuesday Meetup** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/283669162/)
* [February 22, 2022 | Dublin, IE | **Rust Dublin February Meetup** - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/283613610)
* [February 23, 2022 | México City, MX | **Platica Febrero 2022** | Rust MX](https://www.meetup.com/Rust-MX/events/283662630)
* [February 24, 2022 | Linz, AT | **Rust Meetup Linz - 19th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/283377693/)
* [March 1, 2022 | Buffalo, NY, US | **First Tuesdays: Buffalo Rust User Group** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/283638736)
* [March 2, 2022 | Berlin, DE | **Rust Hack and Learn** | OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/events/283633083/)
* [March 7, 2022 | Valence, FR | **Coding-dojo - Rust** | Ardèch’Drôm Dev](https://www.meetup.com/Ardech-Drom-Dev/events/283624590)
* [March 8, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/283221922/)

### North America

* [March 14, 2022 | Atlanta, GA, US | **_New_ Atlanta Rust Meetup** | Atlanta Rustaceans](https://twitter.com/atl_rustaceans/status/1489586471367589893)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> As the temporary human substitute for the temporarily unavailable automated representative of the governance process, I would like to thank the author for their work and everyone else who contributed.

– [Mara Bos (on behalf of RFCbot) on github](https://github.com/rust-lang/rust/issues/87096#issuecomment-1028792980)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1180) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
