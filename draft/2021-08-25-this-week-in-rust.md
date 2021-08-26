Title: This Week in Rust 405
Number: 405
Date: 2021-08-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

In the case of this newsletter, 404 is indeed found!

## Updates from Rust Community

### Project/Tooling Updates
* [Apache Arrow Datafusion 5.0.0 release with major new features and performance improvements](https://arrow.apache.org/blog/2021/08/18/datafusion-5.0.0/)
* [Apache Arrow Ballista 0.5.0 release](https://arrow.apache.org/blog/2021/08/18/ballista-0.5.0/)
- [This week in Fluvio #3: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0003/)

* [SixtyFPS 0.1 Release](https://sixtyfps.io/blog/announcing-sixtyfps-0.1.html) and [SixtyFPS Weekly Report](https://sixtyfps.io/thisweek/2021-08-23.html)

* [nailgun: a DNS performance testing client](https://leshow.github.io/post/nailgun/)

### Observations/Thoughts

* [Overview of the Rust cryptography ecosystem](https://kerkour.com/blog/rust-cryptography-ecosystem)
* [Superpowers of Unsafe Rust](https://blog.knoldus.com/superpowers-of-unsafe-rust/)

### Rust Walkthroughs

* [ZH] Formal Concept Analysis with Rust, [Part1](https://www.horsal.dev/formal-concept-analysis-with-rust-1-introduction), [Part2](https://www.horsal.dev/formal-concept-analysis-with-rust-2-basic-algorithm), [Part3](https://www.horsal.dev/formal-concept-analysis-with-rust-3-parallization)
* [Building a small Finite State Machine in Rust](https://youtu.be/whN36JVUd6A)
* [Rust vectors](https://saidvandeklundert.net/learn/2021-08-15-rust-vector/)
* [Daily Rust: Iterators](adventures.michaelfbryan.com/posts/daily/iterators/)
* [video] [Rust Community Stuttgart - "Traits and trait objects - more than just interfaces"](https://www.youtube.com/watch?v=izXf9-CTAfc)

### Research

### Miscellaneous
* [An exhaustive list of all Rust resources regarding automated or semi-automated formalization efforts in any area, constructive mathematics, formal algorithms, and program verification.](https://github.com/newca12/awesome-rust-formalized-reasoning)

- [nailgun: a DNS performance testing client](https://leshow.github.io/post/nailgun/)

- [Aggregate streaming data in real-time with WebAssembly](https://www.infinyon.com/blog/2021/08/smartstream-aggregates/)

## Crate of the Week

This week's crate is [kube-leader-election](https://github.com/hendrikmaus/kube-leader-election), a crate to implement leader election for Kubernetes workloads.

Thanks to [hendrikmaus](https://users.rust-lang.org/t/crate-of-the-week/2704/945) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Module of the Week

Continuing [Rust Module of the Week](https://motw.rs) this week is [std::fs Part 2: Dirs, Dirs, Dirs](https://motw.rs/blog/2021/08/08/stdfs-part-2-dirs-dirs-dirs/). Contribution and feedback welcome [here](https://github.com/slyons/rust-module-of-the-week).

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

293 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-08-16..2021-08-23

* [upgrade to LLVM 13](https://github.com/rust-lang/rust/pull/87570)
* [introduce `hir::ExprKind::Let` - Take 2](https://github.com/rust-lang/rust/pull/80357)
* [enable compiler consumers to obtain `mir::Body` with Polonius facts](https://github.com/rust-lang/rust/pull/86977)
* [force warn improvements](https://github.com/rust-lang/rust/pull/88134)
* [account for tabs when highlighting multiline code suggestions](https://github.com/rust-lang/rust/pull/87976)
* [use more accurate spans when proposing adding lifetime to item](https://github.com/rust-lang/rust/pull/87983)
* [add notes to macro-not-found diagnostics to point out how things with the same name were not a match](https://github.com/rust-lang/rust/pull/88232)
* [improve wording of the `drop_bounds` lint](https://github.com/rust-lang/rust/pull/86747)
* [improve NLL's "higher-ranked subtype error"s](https://github.com/rust-lang/rust/pull/86700)
* [suggest importing the right kind of macro](https://github.com/rust-lang/rust/pull/88229)
* [improve error reporting for closure return type mismatches](https://github.com/rust-lang/rust/pull/87661)
* [canonicalize consts before calling `try_unify_abstract_consts` query](https://github.com/rust-lang/rust/pull/88166)
* [reenable `RemoveZsts`](https://github.com/rust-lang/rust/pull/88176)
* [I/O safety](https://github.com/rust-lang/rust/pull/87329) (RFC [#3128](https://rust-lang.github.io/rfcs/3128-io-safety.html))
* [stabilize `arbitrary_enum_discriminant`](https://github.com/rust-lang/rust/pull/86860)
* [constified implementations of `Default`](https://github.com/rust-lang/rust/pull/86808)
* [optimize unnecessary check in `VecDeque::retain`](https://github.com/rust-lang/rust/pull/88075)
* [where available use `AtomicU`{`64`, `128`} instead of mutex for `Instant` backsliding protection](https://github.com/rust-lang/rust/pull/83093)
* [add fast path for `Path::cmp` that skips over long shared prefixes](https://github.com/rust-lang/rust/pull/86898)
* [cargo: fix panic with build-std of a proc-macro](https://github.com/rust-lang/cargo/pull/9834)
* [clippy: add new lints `negative_feature_names` and `redundant_feature_names`](https://github.com/rust-lang/rust-clippy/pull/7539)
* [clippy: move `branches_sharing_code` to nursery](https://github.com/rust-lang/rust-clippy/pull/7595)
* [clippy: remove stderr limit](https://github.com/rust-lang/rust-clippy/pull/7593)

### Rust Compiler Performance Triage

A fairly busy week which was to expected given that we've [adjusted our algorithm](https://github.com/rust-lang/rustc-perf/pull/956) for whether we label a change as a regression or not. Most regressions were relatively small, and only one has not yet been addressed in some way.

Triage done by **@rylev**.
Revision range: [6b2050..aa8f27](https://perf.rust-lang.org/?start=6b20506d17f4e5e5bf5bcad7e94add4d754b0ae3&end=aa8f27bf4d980023a8b245ceb25a490a18041eb2&absolute=false&stat=instructions%3Au)

59 comparisons made in total
3 Regressions, 2 Improvements, 2 Mixed; 0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-08-17.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize "force warn" option](https://github.com/rust-lang/rust/issues/86516)

### New RFCs

* [RFC: let-expression](https://github.com/rust-lang/rfcs/pull/3159)

## Upcoming Events

### Online

* [August 18, 2021, Denver, CO, US - Level up our Rust skills by building an ECS by Brooks Patton - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/278909353/)
* [August 18, 2021, Vancouver, BC, CA - Solving LeetCode Problems with Rust - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsycclbxb/)
* [August 19, 2021, Manchester, UK - Rust Manchester - Speeding Up the Snake: Extending Python with Rust](https://www.meetup.com/rust-manchester/events/279730616/)
* [August 19, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [August 31, 2021, Dallas, TX - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Wingback**

* [Senior Backend Developer - rust 🦀 (Remote))](https://careers.wingback.com/o/senior-backend-developer-rust-remote)

**PolarFox Network**

* [Senior Rust Engineer (Remote)](https://polarsync.breezy.hr/p/0c1d3630d39d)

**Stealth Startup**

* [Senior Software Engineer (Raleigh, NC, US, Possible Remote US)](https://docs.google.com/document/d/1jOT6pDE3yNpUq3c9BvFJPy4XaqlIX7BiOqXiTU1Fpfk/edit?usp=sharing)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)

**Parity Technologies**

* [Multiple Rust Engineering Positions Available](https://www.parity.io/jobs/)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Code doesn't deal with resources until it does. Similarly with everything else that forces you to reason about control flow - you don't care about thread management until you do, you don't care about action logs until you do, you don't care about performance until you do... and from the other side, code doesn't need to be exception-safe until it does. The trouble with this kind of "magic" language feature is that correctness becomes non-compositional: you can take two working pieces of code and put them together and get something that doesn't work.

– [Mickey Donaghy on Hacker News](https://news.ycombinator.com/item?id=26536896)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1096) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
