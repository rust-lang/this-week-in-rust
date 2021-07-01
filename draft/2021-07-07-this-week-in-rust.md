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

We only have partial results this week (more details in full report). From the results we have collected, we have one small regression and several improvements.
Also, there was a broad [max-rss regression](https://perf.rust-lang.org/compare.html?start=29cd70d40722930e66a8b726fe58a7bd1d64a22b&end=6b354a13820a444f834a33365ae4a8d97d7d27ce&stat=max-rss) from 11 days ago.
and narrower [max-rss regression](https://perf.rust-lang.org/compare.html?start=406d4a9cc3b9601cf98a07c6c83e0227d64f5d48&end=4573a4a879a8e1f773944a8859e4dcd136138af8&stat=max-rss) from 9 days ago.

Triage done by **@pnkfelix**.
Revision range: [406d4a9..5a78340](https://perf.rust-lang.org/?start=406d4a9cc3b9601cf98a07c6c83e0227d64f5d48&end=5a7834050f3a0ebcd117b4ddf0bc1e8459594309&absolute=false&stat=instructions%3Au)
Revision range: [7c3872e..7ede6e2](https://perf.rust-lang.org/?start=7c3872e6bfd555d2ad753ac1f871db3bd7f2a547&end=7ede6e2a2359c1bb9032baffa4fdafe5633749e3&absolute=false&stat=instructions%3Au)


1 Regressions, 5 Improvements, 0 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-30.md).

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
