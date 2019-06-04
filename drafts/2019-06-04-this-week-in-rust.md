Title: This Week in Rust 289
Number: 289
Date: 2019-06-04
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

This week's crate is [emu](https://github.com/calebwin/emu), a Rust-based language for programming GPUs. Thanks to [Caleb Winston](https://users.rust-lang.org/t/crate-of-the-week/2704/561) for the suggestion!

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

283 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-05-27..2019-06-03

* [Introduce Rust symbol mangling scheme](https://github.com/rust-lang/rust/pull/57967)
* [Remove `GlobalArenas` and use `Arena` instead](https://github.com/rust-lang/rust/pull/61389)
* [Short circuit `Send` and `Sync` impls for `TokenTree`](https://github.com/rust-lang/rust/pull/60967)
* [Explicitly suggest `type_ascription` feature](https://github.com/rust-lang/rust/pull/61374)
* [Recover gracefully from argument with missing type or param name](https://github.com/rust-lang/rust/pull/61331)
* [When encountering move error on an `Option`, suggest using `as_ref`](https://github.com/rust-lang/rust/pull/61147)
* [Reword malformed attribute input diagnostics](https://github.com/rust-lang/rust/pull/61140)
* [Apply `#[must_use]` lint to components of tuples](https://github.com/rust-lang/rust/pull/61100)
* [miri: Tag static/const allocations](https://github.com/rust-lang/miri/pull/748)
* [Update LLVM to include fmin/fmax optimisations](https://github.com/rust-lang/rust/pull/61384)
* [Stabilize `reverse_bits` feature](https://github.com/rust-lang/rust/pull/61364)
* [Stabilize `iter_nth_back` feature](https://github.com/rust-lang/rust/pull/61363)
* [Stabilize `RefCell::try_borrow_unguarded`](https://github.com/rust-lang/rust/pull/60850)
* [`Weak::into_raw`](https://github.com/rust-lang/rust/pull/60766)
* [Implement `iter::Sum` and `iter::Product` for `Option`](https://github.com/rust-lang/rust/pull/58975)
* [Add `Step::sub_usize`](https://github.com/rust-lang/rust/pull/60542)
* [`BufReader`: In Seek impl, remove extra discard_buffer call](https://github.com/rust-lang/rust/pull/61157)
* [Do not print panic message on doctest failures](https://github.com/rust-lang/rust/pull/60549)
* [cargo: Test the Resolver against the varisat Library](https://github.com/rust-lang/cargo/pull/6980)

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

* [disposition: merge] [Tracking issue for RFC 1789: Conversions from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rust/issues/43038).
* [disposition: merge] [Tracking issue for reversing the bit pattern in an integer](https://github.com/rust-lang/rust/issues/48763).
* [disposition: merge] [Stabilize rustdoc theme options](https://github.com/rust-lang/rust/pull/54733).
* [disposition: merge] [Bors policy question: Auto-reassignment on r+](https://github.com/rust-lang/rust/issues/59489).
* [disposition: merge] [Stabilize RefCell::try_borrow_unguarded](https://github.com/rust-lang/rust/pull/60850).
* [disposition: merge] [Stabilize `std::arch::wasm32::unreachable`](https://github.com/rust-lang/rust/issues/61119).
* [disposition: merge] [Add std::mem::take as suggested in #61129](https://github.com/rust-lang/rust/pull/61130).

## New RFCs

* [Add generalized arity tuples](https://github.com/rust-lang/rfcs/pull/2702).

# Upcoming Events

### Africa

* [Jun  5. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/gpxrtqyzhbcb/).

### Asia Pacific

* [Jun 10. Auckland, NZ - Rust AKL - WASM - the past, present and future](https://www.meetup.com/rust-akl/events/259480660/).
* [Jun 12. Melbourne, AU - Rust Melbourne meetup](https://www.meetup.com/Rust-Melbourne/events/261628621/).

### Europe

* [Jun  5. Clermont-Ferrand, FR - Clermont'ech: Rust Workshop](https://www.clermontech.org/workshops/workshop-3-rust.html).
* [Jun  6. Wroclaw, PL - Rust Wroclaw Meetup #11](https://www.meetup.com/Rust-Wroclaw/events/261283360/).
* [Jun 12. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzjbqb/).

### North America

* [Jun  5. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/kkzkxqyzjbhb/).
* [Jun  5. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzjbhb/).
* [Jun 11. Detroit, MI, US - Detroit Rust - June Detroit Rust at Bamboo](https://www.meetup.com/rust-detroit/events/244855856/).
* [May 14. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/nzfspqyzjbpb/).
* [Jun 13. San Diego, CA, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/261595821/).
* [Jun 13. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/261239650).
* [Jun 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzjbrb/).
* [Jun 12. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzjbqb/).
* [Jun 18. Denver, CO, US - Rust Boulder/Denver - Rust Meetup for June](https://www.meetup.com/Rust-Boulder-Denver/events/259124426/).

### South America

* [Jun 1. Sao Paulo, BR - Rust SP - Encontro Junho 2019](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/261123153/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [DevOps Storage Engineer at Comcast, Pennsylvania, US](https://career8.successfactors.com/sfcareer/jobreqcareer?jobId=198894).
* [Rust Developer at enhance, London, UK or remote](https://enhance.com/#jobs).
* [Rust Developer at Kaspersky Lab, Moscow, RU](https://careers.kaspersky.com/job/Developer-%D0%BD%D0%B0-Rust-(QA-Team%2C-KasperskyOS)/561880800/?locale=en_EU).
* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> apparently I wrote Building Git to explain a complex problem to rust devs who could then help me build it in rust

[/dev/horse @ jsconf eu (mountain_ghosts) on twitter](https://twitter.com/mountain_ghosts/status/1134739348593827841)

Thanks to [Dos Moonen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/656) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
