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

This week's crate is [cargo-udeps](https://crates.io/crates/cargo-udeps), a cargo subcommand to find unused dependencies.

Thanks to [Christopher Durham](https://users.rust-lang.org/t/crate-of-the-week/2704/613) for the suggestion!

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

214 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-08-26..2019-09-02

* [rustc: Handle modules in "fat" LTO more robustly](https://github.com/rust-lang/rust/pull/63956)
* [Add default serialization for `Ident`s](https://github.com/rust-lang/rust/pull/63853)
* [Correctly suggest adding bounds to `impl Trait` argument](https://github.com/rust-lang/rust/pull/63811)
* [Strip code to the left and right in diagnostics for long lines](https://github.com/rust-lang/rust/pull/63402)
* [Do not complain about unused code when used in `impl` `Self` type](https://github.com/rust-lang/rust/pull/63317)
* [Simplify eager normalization of constants](https://github.com/rust-lang/rust/pull/63820)
* [miri: Stacked Borrows: Don't read from memory during retagging](https://github.com/rust-lang/miri/pull/931)
* [miri: detect too large dynamically sized objects](https://github.com/rust-lang/rust/pull/64014)
* [Small improvement for `Ord` implementation of integers](https://github.com/rust-lang/rust/pull/63992)
* [Improve Rustdoc's handling of procedural macros](https://github.com/rust-lang/rust/pull/62855)
* [crates.io: Show right-hand column for yanked versions to crate owners](https://github.com/rust-lang/crates.io/pull/1759)

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

> Threads are for working in parallel, async is for waiting in parallel.

– [ssokolow on /r/rust](https://reddit.com/r/rust/comments/cws788/is_await_only_useful_is_my_code_doesnt_do_much/eyfg4va/)

Thanks to [Philipp Oppermann](https://users.rust-lang.org/t/twir-quote-of-the-week/328/686) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
