Title: This Week in Rust 303
Number: 303
Date: 2019-09-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

# Crate of the Week

This week's crate is [viu](https://github.com/atanunq/viu), a terminal image viewer.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/617) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

303 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-09-02..2019-09-09

* [Support both static and dynamic linking mode in testing for vxWorks](https://github.com/rust-lang/rust/pull/63789)
* [Point at variant on pattern field count mismatch](https://github.com/rust-lang/rust/pull/64161)
* [Use hygiene for AST passes](https://github.com/rust-lang/rust/pull/63919)
* [Account for doc comments coming from proc macros without spans](https://github.com/rust-lang/rust/pull/63930)
* [Reduce span to function name in unreachable calls](https://github.com/rust-lang/rust/pull/64229)
* [Move path parsing earlier](https://github.com/rust-lang/rust/pull/64120)
* [or-patterns: Uniformly use `PatKind::Or` in AST & Fix/Cleanup resolve](https://github.com/rust-lang/rust/pull/64111)
* [Allow checking of run-pass execution output in compiletest](https://github.com/rust-lang/rust/pull/63825)
* [Rust 2018: NLL migrate mode => hard error](https://github.com/rust-lang/rust/pull/63565)
* [Extend Polonius fact generation for (some) move tracking](https://github.com/rust-lang/rust/pull/62800)
* [polonius: Finalise initialisation calculations](https://github.com/rust-lang/polonius/pull/110)
* [libc: Remove WASI Core API](https://github.com/rust-lang/libc/pull/1461)
* [Use wasi crate for Core API](https://github.com/rust-lang/rust/pull/63676)
* [Use unicode-xid crate instead of libcore](https://github.com/rust-lang/rust/pull/62848)
* [Add `Result::cloned`{,`_err`} and `Result::copied`{,`_err`}](https://github.com/rust-lang/rust/pull/63166)
* [Stabilize `bind_by_move_pattern_guards` in Rust 1.39.0](https://github.com/rust-lang/rust/pull/63118)
* [Stabilize `checked_duration_since` for 1.38.0](https://github.com/rust-lang/rust/pull/62860)
* [Stabilize `pin_into_inner` in 1.39.0](https://github.com/rust-lang/rust/pull/63985)
* [`Rev::rposition` counts from the wrong end](https://github.com/rust-lang/rust/pull/63549)
* [Override `StepBy::{try_fold, try_rfold}`](https://github.com/rust-lang/rust/pull/64121)
* [Add Iterator comparison methods that take a comparison function](https://github.com/rust-lang/rust/pull/62205)
* [Add methods for converting `bool` to `Option<T>`](https://github.com/rust-lang/rust/pull/64255)
* [cargo: Rename `--all` to `--workspace`](https://github.com/rust-lang/cargo/pull/7241)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2360: hint::bench_black_box](https://github.com/rust-lang/rfcs/pull/2360).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for udp_peer_addr](https://github.com/rust-lang/rust/issues/59127).
* [disposition: merge] [Specify: int->float and f32->f64 round to nearest, overflow to infinity](https://github.com/rust-lang/rust/issues/62231).
* [disposition: merge] [Stabilize `bind_by_move_pattern_guards` in Rust 1.39.0](https://github.com/rust-lang/rust/pull/63118).
* [disposition: merge] [Make `abs`, `wrapping_abs`, `overflowing_abs` const functions](https://github.com/rust-lang/rust/pull/63786).
* [disposition: merge] [Stabilize `Vec::new` and `String::new` as `const fn`s](https://github.com/rust-lang/rust/pull/64028).

## New RFCs

* [Collection Transmute](https://github.com/rust-lang/rfcs/pull/2756).
* [Stabilize `#[unwind]`](https://github.com/rust-lang/rfcs/pull/2753).

# Upcoming Events

### Asia Pacific

* [Sep 11. Selangor, MY - Rust Malaysia Meetup September 2019](https://docs.google.com/forms/d/e/1FAIpQLScsqK0kH3o6ti12AEc9Fn4To-W0rXo9Q-frLmZ3JZUWc8yjjw/viewform).

### Europe

* [Sep 17. Wrocław, PL - Rust Wrocław Meetup #12](https://www.meetup.com/Rust-Wroclaw/events/264586907/)
* [Sep 18. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzmbxb/).

### North America

* [Sep 10. Detroit, MI, US - Detroit Rust - September Detroit Rust at the Altimetrik Collider](https://www.meetup.com/detroitrust/events/264251923/).
* [Sep 12. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzmbqb/).
* [Sep 12. San Diego, CA, US - San Diego Rust September Meetup](https://www.meetup.com/San-Diego-Rust/events/264062555/).
* [Sep 18. Portland, OR, US - PDXRust - Hack Night (not the usual meetup!)](https://www.meetup.com/PDXRust/events/264332355/).
* [Sep 18. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzmbxb/).
* [Sep 20-21. Denver, CO, US - Colorado Gold Rust](https://www.cogoldrust.com/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).
* [Senior Software Engineer at ConsenSys R&D, Remote](https://consensys.net/open-roles/1792013/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The Rust compiler is basically 30 years of trying to figure out how to teach a computer how to see the things we worry about as C developers.

– [James Munns (@bitshiftmask) on Twitter](https://mobile.twitter.com/bitshiftmask/status/1170043794387083268)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/699) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
