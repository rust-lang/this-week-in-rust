Title: This Week in Rust 302
Number: 302
Date: 2019-09-03
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

* [This Month in Rust GameDev #1 - August 2019](https://rust-gamedev.github.io/2019/09/02/newsletter-001.html)

# Crate of the Week

This week's crate is [include_flate](https://crates.io/crates/include_flate), a variant of `include_bytes!`/`include_str` with compile-time DEFLATE compression and runtime lazy decompression.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/606) for the suggestion!

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

221 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-08-19..2019-08-26

* [Stabilize `async_await` in Rust 1.39.0](https://github.com/rust-lang/rust/pull/63209)
* [When declaring a declarative macro in an item it's only accessible inside it](https://github.com/rust-lang/rust/pull/63624)
* [Improve diagnostics: break/continue in wrong context](https://github.com/rust-lang/rust/pull/63780)
* [Audit uses of `apply_mark` in built-in macros + Remove default macro transparencies](https://github.com/rust-lang/rust/pull/63823)
* [Ensure miri can do bit ops on pointer values](https://github.com/rust-lang/rust/pull/63839)
* [Use more optimal `Ord` implementation for integers](https://github.com/rust-lang/rust/pull/63767)
* [Fix bug in `iter::Chain::size_hint`](https://github.com/rust-lang/rust/pull/63691)
* [Implement `nth_back` for `ChunksExactMut`](https://github.com/rust-lang/rust/pull/63265)
* [Avoid unnecessary reservations in `std::io::Take::read_to_end`](https://github.com/rust-lang/rust/pull/63216)
* [cargo: Fix `error:`/`warning:` coloring inconsistency with rustc](https://github.com/rust-lang/cargo/pull/7294)
* [rustdoc: Support `impl Trait` in inlined documentation](https://github.com/rust-lang/rust/pull/61613)
* [rustup: Replace mem::uninitialized with MaybeUninit](https://github.com/rust-lang/rustup.rs/pull/1963)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [hint::bench_black_box](https://github.com/rust-lang/rfcs/pull/2360).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Constify LinkedList new function](https://github.com/rust-lang/rust/pull/63684).
* [disposition: merge] [Test that Wrapping arithmetic ops are implemented for all int types](https://github.com/rust-lang/rust/pull/63692).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Africa

* [Sep  4. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzmbgb/).

### Asia Pacific

* [Sep  2. Auckland, NZ - Rust AKL - Introduction to Rust (session 1 of 3)](https://www.meetup.com/rust-akl/events/259481026/).
* [Sep 11. Selangor, MY - Rust Malaysia Meetup September 2019](https://docs.google.com/forms/d/e/1FAIpQLScsqK0kH3o6ti12AEc9Fn4To-W0rXo9Q-frLmZ3JZUWc8yjjw/viewform).

### Europe

* [Sep  4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzmbgb/).
* [Sep  5. Hamburg, DE - Rust Hack & Learn September 2019](https://www.meetup.com/Rust-Meetup-Hamburg/events/264102479/).

### North America

* [Sep  4. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzmbgb/).
* [Sep  4. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzmbgb/).
* [Sep  5. Seattle, WA, US - Seattle Rust Meetup - Physical Computing Workshop](https://www.meetup.com/Seattle-Rust-Meetup/events/264245990/).
* [Sep 10. Detroit, MI, US - Detroit Rust - September Detroit Rust at the Altimetrik Collider](https://www.meetup.com/detroitrust/events/264251923/).
* [Sep 12. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzmbqb/).
* [Sep 12. San Diego, CA, US - San Diego Rust September Meetup](https://www.meetup.com/San-Diego-Rust/events/264062555/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer - Rust at IOHK, Remote](https://iohk.recruiterbox.com/jobs/fk03udp/).
* [Senior Software Engineer at ConsenSys R&D, Remote](https://consensys.net/open-roles/1792013/).
* [Rust/Core Developer at Parity, Berlin, DE](https://www.parity.io/jobs/#berlin-rust-core-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Just as Bruce Lee practiced Jeet Kune Do, the style of all styles, Rust is not bound to any one paradigm. Instead of trying to put it into an existing box, it's best to just feel it out. Rust isn't Haskell and it's not C. It has aspects in common with each and it has traits unique to itself.

– [Alexander Nye on rust-users](https://users.rust-lang.org/t/idiomatic-rust-favors-functional-or-imperative-style/31720/2)

Thanks to [Louis Cloete](https://users.rust-lang.org/t/twir-quote-of-the-week/328/685) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
