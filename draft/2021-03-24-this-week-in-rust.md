Title: This Week in Rust 383
Number: 383
Date: 2021-03-24
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

[ZH] [Rust in Embedded World](https://zhuanlan.zhihu.com/p/352945545)

### Rust Walkthroughs

* [How to execute shellcodes from memory in Rust](https://kerkour.com/blog/rust-execute-from-memory/)
* [Beginner's Guide to Rust Pattern Matching](https://doma-dev.medium.com/pattern-matching-in-rust-and-other-imperative-languages-7cf1c6abf4a1)
* [video] [The four-part "Overview of the Rust Programming Language" for beginners is now complete](https://www.youtube.com/watch?v=gesNaLkUJeA&list=PLP2yfE2-FXdQmXLvrQ5QN64enbF_KCYQW)

### Papers and Research Projects

### Miscellaneous
* [Streaming using Bastion & Kafka - near consumer native correlation](https://vertexclique.com/streaming-with-bastion-and-kafka/)
* [Performance comparison: counting words in Python, Go, C++, C, AWK, Forth, and Rust](https://benhoyt.com/writings/count-words/)

# Crate of the Week

This week's crate is [egg](https://egraphs-good.github.io), a project using e-graphs to provide a new way to build program optimizers and synthesizers.

Thanks to [Daniel Nugent](https://users.rust-lang.org/t/crate-of-the-week/2704/891) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [delta-rs has many good first issues for those who want to learn Delta Lake or Rust](https://github.com/delta-io/delta-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
* [dotenv-linter has many good first issues](https://github.com/dotenv-linter/dotenv-linter/issues/390)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

389 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-03-15..2021-03-22

* [enable mutable noalias for LLVM >= 12](https://github.com/rust-lang/rust/pull/82834) (Fingers crossed)
* [allow registering tool lints with `register_tool`](https://github.com/rust-lang/rust/pull/83216)
* [more precise spans for HIR paths](https://github.com/rust-lang/rust/pull/83092)
* [`const_evaluatable_checked`: stop eagerly erroring in `is_const_evaluatable`](https://github.com/rust-lang/rust/pull/82707)
* [miri: improve error message of calling unsupported non-"C"/"system"-ABI foreign function](https://github.com/rust-lang/miri/pull/1745)
* [make source-based code coverage compatible with MIR inlining](https://github.com/rust-lang/rust/pull/83080)
* [stabilize `or_patterns` (RFC 2535, 2530, 2175)](https://github.com/rust-lang/rust/pull/79278)
* [stabilize `feature(osstring_ascii)`](https://github.com/rust-lang/rust/pull/80193)
* [stabilize `slice::IterMut::as_slice`](https://github.com/rust-lang/rust/pull/82771)
* [stabilize `assoc_char_funcs` and `assoc_char_consts`](https://github.com/rust-lang/rust/pull/82919)
* [implement `String::remove_matches`](https://github.com/rust-lang/rust/pull/71780)
* [add a check for ASCII characters in `to_upper` and `to_lower`](https://github.com/rust-lang/rust/pull/81358)
* [fix invalid slice access in `String::retain`](https://github.com/rust-lang/rust/pull/82554)
* [constify copy related functions](https://github.com/rust-lang/rust/pull/83091)
* [add `as_str` method for split whitespace str iterators](https://github.com/rust-lang/rust/pull/82570)
* [`Vec::dedup_by` optimization](https://github.com/rust-lang/rust/pull/82191)
* [fix overflowing length in `Vec<ZST>` to `VecDeque`](https://github.com/rust-lang/rust/pull/83244)
* [implement `TrustedLen` and `TrustedRandomAccess` for `Range<integer>`, `array::IntoIter`, `VecDequeue`'s iterators](https://github.com/rust-lang/rust/pull/81607)
* [cargo: allow cargo update to operate with the `--offline` flag](https://github.com/rust-lang/cargo/pull/9279)
* [cargo: refactor feature handling, and improve error messages](https://github.com/rust-lang/cargo/pull/9290)
* [rustdoc: reduce GC work during search](https://github.com/rust-lang/rust/pull/83077)
* [rustfmt: fix issue 'double spaces between struct field prefix and identity when using long attributes](https://github.com/rust-lang/rustfmt/pull/4747)

## Rust Compiler Performance Triage

Added two benchmarks over the past week to the perf suite - diesel and stm32f4,
which are intended to add to the level of tracking for rustdoc and, for both, a
focus on compiler trait machinery.

Performance results for this week are mixed, but overall largely positive.

Triage done by **@simulacrum**.
Revision range: [861872b..f24ce9b0](https://perf.rust-lang.org/?start=861872bc453bde79b83ff99d443d035225f10e87&end=f24ce9b0140d9be5a336954e878d0c1522966bb8&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 4 Mixed

0 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Declarative macro metavariable expressions](https://github.com/rust-lang/rfcs/pull/3086)
* [Change visibility scoping rules for macro_rules macros](https://github.com/rust-lang/rfcs/pull/3067)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [rustdoc: allow list syntax for #[doc(alias)] attributes](https://github.com/rust-lang/rust/pull/82846)
* [disposition: merge] [Deprecate `doc(include)`](https://github.com/rust-lang/rust/pull/82539)
* [disposition: merge] [Stabilize or_patterns (RFC 2535, 2530, 2175)](https://github.com/rust-lang/rust/pull/79278)
* [disposition: merge] [Add IEEE 754 compliant fmt/parse of -0, infinity, NaN](https://github.com/rust-lang/rust/pull/78618)
* [disposition: close] [`impl<A, B>` IntoIterator for (A, B) as Zip](https://github.com/rust-lang/rust/pull/78204)
* [disposition: merge] [tracking issue for `debug_non_exhaustive` feature](https://github.com/rust-lang/rust/issues/67364)
* [disposition: close] [[Edition vNext] Consider deprecating weird nesting of items](https://github.com/rust-lang/rust/issues/65516)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [March 18, Manchester, UK - Rust Manchester Opening Night - Rust Manchester](https://www.meetup.com/rust-manchester/events/276567843/)
* [March 18, Linz, AT - Rust Meetup Linz - 8th Edition - Rust Linz](https://www.meetup.com/Rust-Linz/events/276520435)
* [March 18, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccfbxb/)
* [March 23, Berlin, DE - Rust and Tell - 2021 Kickoff - Berline.rs](https://berline.rs/2021/03/23/rust-and-tell.html)
* [March 25. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/276796209/).
* [March 30, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccfbnc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is just to say,  
> I have rebased  
> the feature branch  
> opened against  
> master
> 
> and which  
> you might have been  
> already working  
> on fixing
> 
> Forgive me,  
> the diff was so trivial  
> so minor  
> so smol

– [Jubilee on rust-lang zulip](https://rust-lang.zulipchat.com/#narrow/stream/257879-project-portable-simd/topic/2021-03-08.20Meeting/near/231384678)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1020) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
