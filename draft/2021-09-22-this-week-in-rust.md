Title: This Week in Rust 409
Number: 409
Date: 2021-09-22
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

### Newsletters

### Project/Tooling Updates

* [Introducing SeaORM: An async & dynamic ORM for Rust](https://www.sea-ql.org/SeaORM/blog/2021-09-20-introducing-sea-orm)

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [qcell](https://github.com/uazu/qcell), with a type that works like a compile-time `RefCell`.

Thanks to [Soni L.](https://users.rust-lang.org/t/crate-of-the-week/2704/952) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

278 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-09-06..2021-09-13

* [fix ICE for functions with more than 65535 arguments](https://github.com/rust-lang/rust/pull/88733)
* [detect stricter constraints on gats where clauses in impls vs trait](https://github.com/rust-lang/rust/pull/88336)
* [ignore derived `Clone` and `Debug` implementations during dead code analysis](https://github.com/rust-lang/rust/pull/85200)
* [fix non-capturing closure return type coercion](https://github.com/rust-lang/rust/pull/88147)
* [suggest items be borrowed in `for i in items[x..]`](https://github.com/rust-lang/rust/pull/88578)
* [suggest wrapping expr in parentheses on invalid unary negation](https://github.com/rust-lang/rust/pull/88757)
* [improve error message when `_` is used for in/inout `asm` operands](https://github.com/rust-lang/rust/pull/88209)
* [emit suggestion when passing byte literal to `format!` macro](https://github.com/rust-lang/rust/pull/87441)
* [use smaller spans for some structured suggestions](https://github.com/rust-lang/rust/pull/87915)
* [use more correct span data in `for` loop desugaring](https://github.com/rust-lang/rust/pull/88214)
* [use `FxHashSet` instead of `Vec` for well formed tys](https://github.com/rust-lang/rust/pull/88771)
* [`mmap` the incremental data instead of reading it](https://github.com/rust-lang/rust/pull/83214)
* [`BTreeMap`/`BTreeSet::from_iter`: use bulk building to improve the performance](https://github.com/rust-lang/rust/pull/88448)
* [add `proc_macro::Span::`{`before`, `after`}](https://github.com/rust-lang/rust/pull/86165)
* [hashbrown: `insert_unique_unchecked` operation](https://github.com/rust-lang/hashbrown/pull/293)
* [clippy: add new lint `iter_not_returning_iterator`](https://github.com/rust-lang/rust-clippy/pull/7610)

### Rust Compiler Performance Triage

Fairly busy week, with some large improvements on several benchmarks. Several
larger rollups landed, in part due to recovery from a temporary CI outage,
and continued CI trouble since then. This is likely the cause for the
somewhat unusual presence of rollups in our results.

Triage done by **@simulacrum**.
Revision range: [69c4aa290..9f85cd6](https://perf.rust-lang.org/?start=69c4aa2901ffadf69deaf91b2f90604bcbc2eb36&end=9f85cd6f2ab2769c16e89dcdddb3e11d9736b351&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 4 Mixed; 2 of them in rollups

31 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-14.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Rust-lang crate ownership policy](https://github.com/rust-lang/rfcs/pull/3119)
* [Scrape code examples from examples/ directory for Rustdoc](https://github.com/rust-lang/rfcs/pull/3123)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: close] [Disable unused_must_use for statically known bools](https://github.com/rust-lang/rust/pull/88028)
* [disposition: merge] [Make #[derive(A, B, ...)] cfg-eval its input only for A, B, ... and stabilize feature(macro_attributes_in_derive_output)](https://github.com/rust-lang/rust/pull/87220)
* [disposition: merge] [Make `*const (), *mut ()` okay for FFI](https://github.com/rust-lang/rust/pull/84267)
* [disposition: merge] [Tracking issue Iterator map_while](https://github.com/rust-lang/rust/issues/68537)

### New RFCs

*No new RFCs were proposed this week.*

## Upcoming Events

### Online

* [September 15, 2021, Vancouver, BC, CA - Considering Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsyccmbtb/)
* [September 16, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [September 18, 2021, Tokyo, JP - Rust.Tokyo 2021](https://rust.tokyo/)
* [September 28, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccmblc/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Edition!

– [Niko and Daphne Matsakis on YouTube](https://www.youtube.com/watch?v=q0aNduqb2Ro)

Thanks to [mark-i-m](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1102) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
