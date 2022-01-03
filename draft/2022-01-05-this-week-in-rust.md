Title: This Week in Rust 424
Number: 424
Date: 2022-01-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Project/Tooling Updates
* [This Week in Glean: Glean in 2021](https://fnordig.de/2021/12/17/glean-in-2021/)

### Research

### Observations/Thoughts
* [Why is my Rust build so slow?](https://fasterthanli.me/articles/why-is-my-rust-build-so-slow)

### Rust Walkthroughs
* [Truly headless draw.io exports](https://fasterthanli.me/series/dont-shell-out/part-1)
* [From Inkscape to poppler](https://fasterthanli.me/series/dont-shell-out/part-2)
* [A static poppler build: the easy way](https://fasterthanli.me/series/dont-shell-out/part-3)
* [Building poppler for Windows](https://fasterthanli.me/series/dont-shell-out/part-4)
* [Porting poppler to meson](https://fasterthanli.me/series/dont-shell-out/part-5)
* [Productionizing out poppler build](https://fasterthanli.me/series/dont-shell-out/part-6)
* [The rest of the fucking owl](https://fasterthanli.me/series/dont-shell-out/part-7)
* [One funny way to bundle assets](https://fasterthanli.me/series/dont-shell-out/part-8)

### Miscellaneous

## Crate of the Week

This week's crate is [zoxide](https://github.com/ajeetdsouza/zoxide), a smarter `cd` command.

Thanks to [Ajeet D'Souza](https://users.rust-lang.org/t/crate-of-the-week/2704/993) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

188 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-12-20..2021-12-27

* [miri: provide slightly better notes when tracking a pointer tag](https://github.com/rust-lang/miri/pull/1945)
* [backport LLVM changes to disable deferred inlining](https://github.com/rust-lang/rust/pull/92110)
* [fix duplicate derive clone suggestion](https://github.com/rust-lang/rust/pull/91544)
* [perf: change `Backtrace::enabled` atomic from `SeqCst` to `Relaxed`](https://github.com/rust-lang/rust/pull/92139)
* [perf: eliminate `ObligationCauseData`](https://github.com/rust-lang/rust/pull/91844)
* [fix bad caching of `~const Drop` bounds](https://github.com/rust-lang/rust/pull/92149)
* [constify `core::intrinsics::black_box` and `core::hint::black_box`](https://github.com/rust-lang/rust/pull/92226)
* [make `PTR::as_ref` and similar methods `const`](https://github.com/rust-lang/rust/pull/91823)
* [RawVec: don't recompute capacity after allocating](https://github.com/rust-lang/rust/pull/92220)
* [allow reverse iteration of lowercase'd/uppercase'd chars](https://github.com/rust-lang/rust/pull/88858)
* [stabilise `entry_insert`](https://github.com/rust-lang/rust/pull/90345)
* [suggest adding `#[cfg(test)]` to a test module](https://github.com/rust-lang/rust/pull/91770)
* [cargo: make levenshtein distance case insensitive.](https://github.com/rust-lang/cargo/pull/10224)
* [clippy: add suggestion for `neg_multiply` lint](https://github.com/rust-lang/rust-clippy/pull/8144)
* [clippy: fix `iter_skip_next` false positives](https://github.com/rust-lang/rust-clippy/pull/8133)
* [clippy: improve `unwrap_or_else_default` when handling `unwrap_or_else(XXX::new)`](https://github.com/rust-lang/rust-clippy/pull/8163)
* [clippy: fix `shadow_reuse` false negative for if let bindings](https://github.com/rust-lang/rust-clippy/pull/8165)
* [clippy: fix an ICE on unwrapping a None](https://github.com/rust-lang/rust-clippy/pull/8167)
* [clippy: new lint: `init-numbered-fields`](https://github.com/rust-lang/rust-clippy/pull/8170)
* [rustfmt: fix static async closure qualifier order](https://github.com/rust-lang/rustfmt/pull/5150)
* [rustfmt: retain qualified path when rewriting struct literal expressions](https://github.com/rust-lang/rustfmt/pull/5152)

### Rust Compiler Performance Triage

Relatively quiet week, mostly rustdoc improvements.

Triage done by **@simulacrum**.
Revision range: [3d57c61a..e91ad5fc62](https://perf.rust-lang.org/?start=3d57c61a9e04dcd3df633f41142009d6dcad4399&end=e91ad5fc62bdee4a29c18baa5fad2ca42fc91bf4&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 6 Mixed; 0 of them in rollups

26 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-12-28.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amend RFC 2996 to replace `Stream` with  `AsyncIterator`](https://github.com/rust-lang/rfcs/pull/3208)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No new RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Clarify the guarantees that ThreadId does and doesn't make.](https://github.com/rust-lang/rust/pull/84083)
* [disposition: merge] [Remove effect of `#[no_link]` attribute on name resolution](https://github.com/rust-lang/rust/pull/92034)
* [disposition: merge] [Tracking Issue for Stdin::lines, Stdin::split forwarder methods](https://github.com/rust-lang/rust/issues/87096)
* [disposition: merge] [Tracking issue for Result::cloned, Result::cloned_err, Result::copied, Result::copied_err](https://github.com/rust-lang/rust/issues/63168)
* [disposition: merge] [Remove unnecessary bounds for some Hash{Map,Set} methods](https://github.com/rust-lang/rust/pull/91593)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No new RFCs were published this week.*

## Upcoming Events

Rusty Events between 12/29/2021 - 1/31/2022 🦀

### Online

* [January 5, 2022 | Indianapolis, IN, US | **Indy.rs - with Social Distancing** | Indy Rust](https://www.meetup.com/indyrs/events/qwtdjsydccbhb/)
* [January 6, 2022 | Cardiff, UK | **Rust Book Study Session - Automated Tests & Building a Command Line Program** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/282667031/)
* [January 6, 2022 | Nürnberg, DE | **Rust Nürnberg online #8**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/282344613/)
* [January 8, 2022 | Various cities | **Rust GameDev Monthly Meetup** | Rust GameDev](https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com)
* [January 11, 2022 | Dallas, TX, US | **Second Tuesday Meetup**| Dallas Rust](https://www.meetup.com/Dallas-Rust/events/vqtjcsydccbpb/)
* [January 11, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydccbpb/)
* [January 12, 2022 | Boulder, CO, US | **Monthly Meetup** | Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydccbqb/)
* [January 12, 2022 | Los Angeles, CA, US | **Live Coding Session - Mob Programming a Rust Code Kata [Virtual] Jan. 2022** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/282580016/)
* [January 12, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/gjrtqsydccbqb/)
* [January 13, 2022 | Warsaw, PL | **Rust Warsaw #4** | Rust Warsaw](https://www.meetup.com/pl-PL/Rust-Warsaw/events/282879405/)
* [January 18, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/vdhxgsydccbxb/)
* [January 19, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out night** |Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydccbzb)
* [January 25, 2022 | Dallas, TX, US | **Last Tuesday Meetup** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrydccbhc/)

### North America

* [January 6, 2022 | Austin, TX, US | **Rust Lunch** | Rust ATX](https://www.meetup.com/rust-atx/events/282756864/)
* [January 12, 2022 | Atlanta, GA, US | **Grab a beer with fellow Rustaceans** | Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsydccbqb/)
* [January 13, 2022 | Columbus, OH, US | **Monthly Meeting** | Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrydccbrb/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> One reason we keep certain things as hard errors rather than lints: it establishes a baseline
> that you can safely assume about other people's code, since it can't be turned off. And as a
> result, that baseline can become part of people's mental model of Rust itself, rather than
> something that might or might not be true in any given codebase.
>
> We have to take care to not use that lightly, because that places work on all users of Rust to
> maintain code to that baseline. But there are cases where we do. We don't allow using one integer
> type where another was expected. We don't allow certain operations outside an unsafe block. ...
>
> I think the standard we should apply is asking whether something is part of the baseline that
> people should be able to assume about all Rust code, and if that's worth the tradeoff of requiring
> that baseline of all Rust users.

– [Josh Triplett on rust-internals](https://internals.rust-lang.org/t/lack-of-mut-in-bindings-as-a-deny-by-default-lint/15818/8)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1153) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
