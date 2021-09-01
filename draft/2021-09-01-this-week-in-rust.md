Title: This Week in Rust 406
Number: 406
Date: 2021-00-01
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
 * [partial-borrow](https://diziet.dreamwidth.org/9019.html): derive macro for multiple (maybe mut) references to subsets/views of a struct

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [kube-leader-election](https://github.com/hendrikmaus/kube-leader-election), a crate to implement leader election for Kubernetes workloads.

Thanks to [hendrikmaus](https://users.rust-lang.org/t/crate-of-the-week/2704/945) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

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

A few regressions but largely an improvement this week, mostly due to the
upgrade to LLVM 13.

Triage done by **@simulacrum**.
Revision range: [aa8f27b..33fdb79](https://perf.rust-lang.org/?start=aa8f27bf4d980023a8b245ceb25a490a18041eb2&end=33fdb797f59421c7bbecaa4588ed5d7a31a9494a&absolute=false&stat=instructions%3Au)

2 Regressions, 1 Improvements, 2 Mixed; 0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-08-24.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: let-expression](https://github.com/rust-lang/rfcs/pull/3159)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize std::os::unix::fs::chroot](https://github.com/rust-lang/rust/pull/88177)
* [disposition: merge] [Stabilize reserved prefixes](https://github.com/rust-lang/rust/issues/88140)
* [disposition: merge] [stabilize disjoint capture in closures (RFC 2229)](https://github.com/rust-lang/rust/issues/88126)
* [disposition: merge] [BTree: remove Ord bound from new](https://github.com/rust-lang/rust/pull/88040)
* [disposition: merge] [Update Windows Argument Parsing](https://github.com/rust-lang/rust/pull/87580)
* [disposition: merge] [Support #[track_caller] on closures and generators](https://github.com/rust-lang/rust/pull/87064)
* [disposition: merge] [Stabilize "force warn" option](https://github.com/rust-lang/rust/issues/86516)
* [disposition: merge] [Extend -Cdebuginfo with new options and named aliases](https://github.com/rust-lang/rust/pull/83947)
* [disposition: merge] [Allow writing of incomplete UTF-8 sequences to the Windows console via stdout/stderr](https://github.com/rust-lang/rust/pull/83342)
* [disposition: merge] [Tracking Issue for Iterator::intersperse](https://github.com/rust-lang/rust/issues/79524)
* [disposition: merge] [Provide an API to extract fields from Command builder](https://github.com/rust-lang/rust/issues/44434)

### New RFCs

* [RFC: cargo-run-deps](https://github.com/rust-lang/rfcs/pull/3168)
* [Proposal: Else clauses for for and while loops](https://github.com/rust-lang/rfcs/pull/3163)

## Upcoming Events

### Online

* [August 31, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/)
* [September 2, 2021, Zurich, CH - Exciting new Rustdoc features landing in 1.55.0 - Hybrid Meetup (Livestream!) - Rust Zurich](https://www.meetup.com/Rust-Zurich/events/280295950/)
* [September 2, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [September 8, 2021, Denver, CO, US - Rust Q&A - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/279407152/)

### North America

* [September 8, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccmblb/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Code doesn't deal with resources until it does. Similarly with everything else that forces you to reason about control flow - you don't care about thread management until you do, you don't care about action logs until you do, you don't care about performance until you do... and from the other side, code doesn't need to be exception-safe until it does. The trouble with this kind of "magic" language feature is that correctness becomes non-compositional: you can take two working pieces of code and put them together and get something that doesn't work.

– [Mickey Donaghy on Hacker News](https://news.ycombinator.com/item?id=26536896)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1096) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
