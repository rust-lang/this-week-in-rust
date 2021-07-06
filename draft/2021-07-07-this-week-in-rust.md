Title: This Week in Rust 398
Number: 398
Date: 2021-07-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Project/Tooling Updates
* [ChainSafe's Rust implementation of Mina spec Community Spotlight (video)](https://youtu.be/NaxZR-LDc_g)
* [MoonZoon Dev News (5): Chat example, MoonZoon Cloud](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4)
* [Fluvio: The Programmable Data Platform](https://www.infinyon.com/blog/2021/06/introducing-fluvio/)

* [butido - a Linux Package Building Tool in Rust](https://beyermatthias.de/butido-a-linux-package-building-tool-in-rust)

### Observations/Thoughts
- [Walking through "The Java Tutorials" with Rust - 'What Is an Interface?' and specialization](https://rust-java-tutorials.netlify.app/blog/6-interfaces/)

- [Rust and Tinyverse](https://tinyverse.substack.com/p/rust-and-tinyverse)

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [css-inline](https://github.com/Stranger6667/css-inline), a crate to inline CSS into `style` tags.

Thanks to [Dmitry Dygalo](https://users.rust-lang.org/t/crate-of-the-week/2704/931) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

297 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-06-28..2021-07-05

* [make `ForceWarn` a lint level](https://github.com/rust-lang/rust/pull/86009)
* [change vtable memory representation to use `tcx` allocated allocations](https://github.com/rust-lang/rust/pull/86475)
* [support allocation failures when interpreting MIR](https://github.com/rust-lang/rust/pull/86255)
* [avoid byte to char position conversions in `is_multiline`](https://github.com/rust-lang/rust/pull/86778)
* [fix pretty print for `loop`](https://github.com/rust-lang/rust/pull/86358)
* [pretty-print macro matchers instead of using source code](https://github.com/rust-lang/rust/pull/86282)
* [fix ICE when main is declared in an extern block](https://github.com/rust-lang/rust/pull/86190)
* [ignore inference variables in certain queries](https://github.com/rust-lang/rust/pull/86866)
* [check the number of generic lifetime and const parameters of intrinsics](https://github.com/rust-lang/rust/pull/86148)
* [check node kind to avoid ICE in `check_expr_return()`](https://github.com/rust-lang/rust/pull/86728)
* [deny using default function in impl const Trait](https://github.com/rust-lang/rust/pull/86571)
* [fix garbled suggestion for missing lifetime specifier](https://github.com/rust-lang/rust/pull/86678)
* [fix misleading "impl Trait" error](https://github.com/rust-lang/rust/pull/86666)
* [alloc: `no_global_oom_handling`: disable `new()`s, `pin()`s, etc.](https://github.com/rust-lang/rust/pull/86810)
* [add linked list cursor end methods](https://github.com/rust-lang/rust/pull/86714)
* [stabilize `str::from_utf8_unchecked` as const](https://github.com/rust-lang/rust/pull/86213)
* [stabilize `string_drain_as_str`](https://github.com/rust-lang/rust/pull/86858)
* [stabilize `Bound::cloned()`](https://github.com/rust-lang/rust/pull/86797)
* [stabilize `Seek::rewind()`](https://github.com/rust-lang/rust/pull/86794)
* [when using `process::Command` on Windows, environment variable names must be case-preserving but case-insensitive](https://github.com/rust-lang/rust/pull/85270)
* [add `track_path::path` fn for usage in `proc_macro`s](https://github.com/rust-lang/rust/pull/84029)
* [libm: optimize `round` and `roundf`](https://github.com/rust-lang/libm/pull/253)
* [cargo: adjust error message with `offline` and `frozen`](https://github.com/rust-lang/cargo/pull/9644)
* [clippy: stabilize `cargo clippy --fix`](https://github.com/rust-lang/rust-clippy/pull/7405)
* [clippy: downgrade `nonstandard_macro_braces` to nursery](https://github.com/rust-lang/rust-clippy/pull/7424)
* [clippy: don't suggest `doc(hidden)` or unstable variants in wildcard lint](https://github.com/rust-lang/rust-clippy/pull/7407)
* [clippy: fix emitting in nested (`proc_`)`macro`s for `nonstandard_macro_braces` lint](https://github.com/rust-lang/rust-clippy/pull/7431)
* [clippy: fix `doc_markdown` false positive](https://github.com/rust-lang/rust-clippy/pull/7426)
* [clippy: new lint: `rc_mutex`](https://github.com/rust-lang/rust-clippy/pull/7316)
* [clippy: new lint: `strlen_on_c_strings`](https://github.com/rust-lang/rust-clippy/pull/7243)
* [clippy: new lint: `disallowed_script_idents`](https://github.com/rust-lang/rust-clippy/pull/7400)

### Rust Compiler Performance Triage

A fairly mixed week with improvements and regressions mostly balancing themselves out. The highlight of this week is we have now started to adopt a new performance triage process which will label PRs that introduce performance regressions with the `perf-regression` label. Authors and/or reviewers are expected to justify their performance regression either by a short summary of why the change is worth it despite the regression or by creating an issue to follow-up on the regression.

We hope this process will lead to better compiler performance in the long term.

Triage done by **@rylev**.
Revision range: [5a78340..9a27044](https://perf.rust-lang.org/?start=5a7834050f3a0ebcd117b4ddf0bc1e8459594309&end=9a27044f42ace9eb652781b53f598e25d4e7e918&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 2 Mixed
1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-07-06.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: let-else statements](https://github.com/rust-lang/rfcs/pull/3137)
* [disposition: merge] [RFC: I/O Safety](https://github.com/rust-lang/rfcs/pull/3128)
* [disposition: merge] [`#[derive(Default)]` on enums with a `#[default]` attribute](https://github.com/rust-lang/rfcs/pull/3107)
* [disposition: close] [New RFC: Collection Transmute](https://github.com/rust-lang/rfcs/pull/2756)
* [disposition: close] [RFC: Add delete and delete_by methods to Iterator](https://github.com/rust-lang/rfcs/pull/2475)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize bindings_after_at](https://github.com/rust-lang/rust/pull/85305)
* [disposition: merge] [Tracking Issue for std::io::Seek::rewind()](https://github.com/rust-lang/rust/issues/85149)
* [disposition: merge] [Stabilize `impl From<[(K, V); N]>` for HashMap (and friends)](https://github.com/rust-lang/rust/pull/84111)
* [disposition: merge] [Stabilize "RangeFrom" patterns in 1.55](https://github.com/rust-lang/rust/pull/83918)
* [disposition: merge] [Tracking Issue for feature(string_drain_as_str) - string::Drain::as_str()](https://github.com/rust-lang/rust/issues/76905)

### New RFCs

* [Candidate Target Policy](https://github.com/rust-lang/rfcs/pull/3145)

## Upcoming Events

### Online

* [July 6, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/jxfdjsycckbjb/)
* [July 7, 2021, Denver, CO, US - End-to-end Encrypted Messaging in Rust, with Ockam by Mrinal Wadhwa - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/277633525/)
* [July 13, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycckbrb/)
* [July 14, 2021, Malaysia - Rust Meetup July 2021 - Golang Malaysia, feat Rustlang, Erlang, Haskelllang and `.*-?(lang|script)\`](https://docs.google.com/forms/d/e/1FAIpQLSdoVbexvU3TZox1D9yLKPUggeTuih7TEDR6eaFQGTEgJtXZ5g/viewform)
* [July 14, 2021, Dublin, IE - Rust Dublin July Remote Meetup - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/278698763/)

### North America

* [July 14, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgrycckbsb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**ChainSafe Systems**
* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

**InfinyOn**

* [Senior Rust Engineer(Remote))](https://infinyon.zohorecruit.com/jobs/Careers/619885000000428015/Senior-Rust-Software-Engineer?source=CareerSite)


# Quote of the Week

> One thing I like about Rust is that it filters out lazy/sloppy thinkers. Even when I disagree with another Rust programmer, there is a certain level of respect that comes from knowing that they thought about the problem deeply enough to pass the borrow checker.

– [Zeroexcuses on rust-users](https://users.rust-lang.org/t/what-is-you-elevator-pitch-for-rust/61713/7?u=llogiq)

Thanks to [Jonah](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1070) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
