Title: This Week in Rust 396
Number: 396
Date: 2021-06-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

No newsletters or research articles this week.

### Official

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [hypergraph](https://github.com/yamafaktory/hypergraph), graph data structure implementation where edges can join arbitrary numbers of vertices.

Thanks to [Davy Duperron](https://users.rust-lang.org/t/crate-of-the-week/2704/929) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

284 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-06-21..2021-06-28

* [fix type checking of return expressions outside of function bodies](https://github.com/rust-lang/rust/pull/86206)
* [add `future_prelude_collision` lint](https://github.com/rust-lang/rust/pull/85707)
* [do not emit alloca for ZST locals with multiple assignments](https://github.com/rust-lang/rust/pull/86166)
* [fix panic-safety in specialized `Zip::next_back`](https://github.com/rust-lang/rust/pull/86452)
* [add `io::Cursor::`{`remaining`, `remaining_slice`, `is_empty`}](https://github.com/rust-lang/rust/pull/86037)
* [make `fmt::Arguments::as_str` unstably const](https://github.com/rust-lang/rust/pull/86655)
* [cargo: unify weak and namespaced features](https://github.com/rust-lang/cargo/pull/9574)
* [rustdoc: properly render higher-ranked trait bounds](https://github.com/rust-lang/rust/pull/84814)
* [rustdoc: do not list impl when trait has doc(hidden)](https://github.com/rust-lang/rust/pull/86513)
* [rustdoc: render `<Self as X>::Y` type casts properly across crate bounds](https://github.com/rust-lang/rust/pull/86449)
* [rustdoc: staggered layout for module contents on mobile](https://github.com/rust-lang/rust/pull/85651)
* [clippy: add suspicious group](https://github.com/rust-lang/rust-clippy/pull/7350)

### Rust Compiler Performance Triage

A few small regressions on smaller benchmarks (e.g., helloworld), likely
centered around more IR being generated in a few cases.

Triage done by **@simulacrum**.
Revision range: [d192c80..3912083](https://perf.rust-lang.org/?start=d192c80d2284ba6b5146bb3da586354c3762c72b&end=3912083821c5072f700a75589c8af6a9d3e20a21&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 0 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-22.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [RFC: Add delete and delete_by methods to Iterator](https://github.com/rust-lang/rfcs/pull/2475)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Redefine ErrorKind::Other and stop using it in std.](https://github.com/rust-lang/rust/pull/85746)
* [disposition: merge] [When using process::Command on Windows, environment variable names must be case-preserving but case-insensitive](https://github.com/rust-lang/rust/pull/85270)
* [disposition: merge] [Tracking Issue for std::io::Seek::rewind()](https://github.com/rust-lang/rust/issues/85149)
* [disposition: merge] [Support forwarding caller location through trait object method call](https://github.com/rust-lang/rust/pull/81360)
* [disposition: merge] [Tracking issue for ops::Bound::cloned()](https://github.com/rust-lang/rust/issues/61356)

### New RFCs

* [Stabilize Cargo's weak-dep-features and namespaced-features.](https://github.com/rust-lang/rfcs/pull/3143)

## Upcoming Events

### Online

* [June 24, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [June 29, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccjbmc/)
* [July 6, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/jxfdjsycckbjb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> When a panic has a payload that's an object which needs Drops,  
> And the panic hits a catch_unwind for unexpected stops  
> Before if its Drop panicked we'd just crash to your desktops,  
> Now the payload gets forgotten, and you'd better grab some mops!

– [Josh Triplett on twitter](https://twitter.com/josh_triplett/status/1407776002973986819)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1069) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
