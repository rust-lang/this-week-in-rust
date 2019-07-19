Title: This Week in Rust 296
Number: 296
Date: 2019-07-23
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

* [Whiteboard series on Rust Libp2p](https://www.youtube.com/watch?v=_9o6RTYG_xk)

# Crate of the Week

This week's crate is [overloadable](https://crates.io/crates/overloadable), a crate to provides you with the capabilities to overload your functions in a similar style to C# or C++, including support for meta attributes, type parameters and constraints, and visibility modifiers
Thanks to [Stevensonmt](https://users.rust-lang.org/t/crate-of-the-week/2704/585) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [std::future support merged to hyper master. There's now smaller tasks if you'd like to join in](https://github.com/hyperium/hyper/milestone/5).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

235 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-07-08..2019-07-15

* [Use const generics for array impls [part 1]](https://github.com/rust-lang/rust/pull/62435)
* [Update to LLVM 9](https://github.com/rust-lang/llvm-project/pull/19) (ongoing work)
* [Update LLVM: apply patch necessary for ThinLTO on RISC-V](https://github.com/rust-lang/rust/pull/62463)
* [Use visitor for `#[structural_match]` check](https://github.com/rust-lang/rust/pull/62339)
* [Normalize projections appearing in `impl Trait`](https://github.com/rust-lang/rust/pull/62221)
* [typeck: Merge opaque type inference logic](https://github.com/rust-lang/rust/pull/62090)
* [Fact generation for liveness calculations in Polonius](https://github.com/rust-lang/rust/pull/60266)
* [Add key and value methods to DebugMap](https://github.com/rust-lang/rust/pull/60458)
* [Add an AtomicCell abstraction](https://github.com/rust-lang/rust/pull/62577)
* [Add messages to `Option`'s and `Result`'s `must_use` annotation for `is_*`](https://github.com/rust-lang/rust/pull/62431)
* [Prevent Vec::drain_filter from double dropping on panic](https://github.com/rust-lang/rust/pull/61224)
* [Add `impl<T> FromIterator<T> for Arc/Rc<[T]>`](https://github.com/rust-lang/rust/pull/61953)
* [Add Iterator::partition_in_place() and is_partitioned()](https://github.com/rust-lang/rust/pull/62278)
* [Use `fold` in `Iterator::last` default implementation](https://github.com/rust-lang/rust/pull/62481)
* [rustc guide: Add humor appendix](https://github.com/rust-lang/rustc-guide/pull/350)

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

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Deprecate `try!` macro](https://github.com/rust-lang/rust/pull/62672).
* [disposition: merge] [Add joining slices of slices with a slice separator, not just a single item](https://github.com/rust-lang/rust/pull/62528).
* [disposition: merge] [Stablize Euclidean Modulo (feature euclidean_division)](https://github.com/rust-lang/rust/pull/61884).
* [disposition: merge] [Tracking issue for unstable `--json-rendered` flag](https://github.com/rust-lang/rust/issues/60987).
* [disposition: merge] [Tracking issue for stable `-C emit-artifact-notifications` rustc flag](https://github.com/rust-lang/rust/issues/60419).

## New RFCs

* [Target feature runtime](https://github.com/rust-lang/rfcs/pull/2725).
* [Allow Overloading || and &&](https://github.com/rust-lang/rfcs/pull/2722).

# Upcoming Events

### Africa

* [Jul 20. Nairobi, KE - Rust Nairobi - HACK & LEARN: Exercism Edition](https://www.meetup.com/Rust-Nairobi/events/259650701/).

### Asia Pacific

* [Jul 20. Chennai, IN - Rust Chennai - Monthly meetup - July](https://www.meetup.com/mad-rs/events/263158278).
* [Jul 23. Wellington, NZ - Rust Wellington - Talk: 5 Essential Traits](https://www.meetup.com/Rust-Wellington/events/262407494/).
* [Jun 25. Seoul, KR - Seoul Rust Meetup, Hapjeong](https://www.meetup.com/Rust-Seoul-Meetup/events/srxvzqyzkbfc/).
* [Aug 10. Singapore, SG - Rust Meetup](https://www.eventbrite.com/e/rust-meetup-tickets-65358532129).

### Europe

* [Jul 24. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzkbgc/).
* [Jul 24. London, GB - Rust London User Group - LDN Talks July 2019](https://www.meetup.com/Rust-London-User-Group/events/262999277/).
* [Jul 24. Milano, IT - Rust Language Milano - Rust Exercises](https://www.meetup.com/rust-language-milano/events/263140153).
* [Aug 07. Erlangen, DE - Rust Franken Meetup #1](https://www.meetup.com/Rust-NERF/events/263163435/).

### North America

* [Jul 24. Mexico City, MX - Rust MX - Reunión Julio: Rust y Big data](https://www.meetup.com/Rust-MX/events/262960131/).
* [Jul 24. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzkbdc/).
* [Jul 24. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzkbgc/).
* [Jul 24. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzkbgc/).
* [Jul 30. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzkbnc/).

### South America

* [Jul 27. Sao Paulo, BR - Rust SP - Encontro Julho 2019](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/262488375).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Graphics Engineer at Mozilla, Toronto, CA](https://careers.mozilla.org/position/gh/1728803/).
* [Backend Engineer at Bitski, San Francisco, US](https://angel.co/company/bitski/jobs/366874-backend-engineer).
* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is 5 languages stacked on top of each other, except that instead of ending up like 5 children under a trenchcoat, they end up like the power rangers.

– [reuvenpo on /r/rust](https://reddit.com/r/rust/comments/cb49lb/coworker_rust_doesnt_offer_anything_c_doesnt/etdddwt/)

Thanks to [Jelte Fennema](https://users.rust-lang.org/t/twir-quote-of-the-week/328/666) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
