Title: This Week in Rust 321
Number: 321
Date: 2020-01-14
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

This week's crate is [cxx](https://github.com/dtolnay/cxx), a library to build a C++ FFI safely by taking care of both sides.

Thanks to [Ehsan M. Kermani](https://users.rust-lang.org/t/crate-of-the-week/2704/702) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [good first issue] [bandwhich: Better error handling when accessing network interface](https://github.com/imsnif/bandwhich/issues/73).
* [bandwhich: In the UI, calculate bandwidth for the past N seconds rather than just 1](https://github.com/imsnif/bandwhich/issues/14).
* [bandwhich: Pause display with space](https://github.com/imsnif/bandwhich/issues/74).
* [Register your interest in Rust training with Carol Nichols and Jake Goulding in May 2020](https://docs.google.com/forms/d/e/1FAIpQLSdIJy7JbftA27qJlxEgR9Q5o1MB3kXqnH3bthTJbg7KS-P0YQ/viewform).
* [Rust LATAM Mexico 2020 CFP](https://cfp.rustlatam.org/events/rust-latam-mexico-2020).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

311 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-01-06..2020-01-13

* [prepare for LLVM 10 upgrade](https://github.com/rust-lang/rust/pull/67900)
* [allow specifying LLVM args in target specifications](https://github.com/rust-lang/rust/pull/68059)
* [don't require `allow_internal_unstable` unless `staged_api` is enabled](https://github.com/rust-lang/rust/pull/68114)
* [more reductions in error handling diversity](https://github.com/rust-lang/rust/pull/67770)
* [introduce `X..`, `..X`, and `..=X` range patterns](https://github.com/rust-lang/rust/pull/67258)
* [ban `...X` pats, harden tests, and improve diagnostics](https://github.com/rust-lang/rust/pull/68120)
* [add suggestions when encountering chained comparisons](https://github.com/rust-lang/rust/pull/68108)
* [handle multiple error fix suggestions carefully](https://github.com/rust-lang/rust/pull/67880)
* [simplify `into_key_slice_mut`](https://github.com/rust-lang/rust/pull/67725)
* [`Option::{expect,unwrap}` and `Result::{expect, expect_err, unwrap, unwrap_err}` have `#[track_caller]`](https://github.com/rust-lang/rust/pull/67887)
* [add `HashSet::get_or_insert_owned`](https://github.com/rust-lang/rust/pull/67358)
* [make `Layout::new` const](https://github.com/rust-lang/rust/pull/66254)
* [constify more of `alloc::Layout`](https://github.com/rust-lang/rust/pull/67494)
* [futures mpsc: split bounded and unbounded implementations](https://github.com/rust-lang/futures-rs/pull/1326)
* [distinguish between private items and hidden items in rustdoc](https://github.com/rust-lang/rust/pull/67875)
* [rustbuild: add `llvm-skip-rebuild` flag to `x.py`](https://github.com/rust-lang/rust/pull/68074)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2795: Propose implicit named arguments for formatting macros](https://github.com/rust-lang/rfcs/pull/2795).
* [RFC 2835: Announcing the Safe-Transmute Project Group](https://github.com/rust-lang/rfcs/pull/2835).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Deprecate stdlib modules dedicated to numeric constants and move those constants to associated consts](https://github.com/rust-lang/rfcs/pull/2700).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `#![feature(slice_patterns)]` in 1.42.0](https://github.com/rust-lang/rust/pull/67712).
* [disposition: merge] [Stabilize Condvar::wait_while and wait_timeout_while (previously wait_until, wait_timeout_until)](https://github.com/rust-lang/rust/pull/67076).

## New RFCs

* [Add llvm_asm! and deprecate asm!](https://github.com/rust-lang/rfcs/pull/2843).
* [Supertrait item shadowing](https://github.com/rust-lang/rfcs/pull/2845).

# Upcoming Events

### Europe

* [Jan 10. Darmstadt, DE - Rust Rhein-Main - 1st 2020 Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/267158461/).
* [Jan 16. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/267292439).
* [Jan 16. Helsinki, FI - Finland Rust-lang Group - January meetup](https://www.meetup.com/Finland-Rust-Meetup/events/267607507/).
* [Jan 17. Stuttgart, DE - Rust Hack and Learn](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/267764516).
* [Jan 22. Wrocław, PL - Rust Wrocław Meetup #16](https://www.meetup.com/Rust-Wroclaw/events/267514337/).
* [Jan 23. Paris, FR - Rust Paris meetup #49](https://www.meetup.com/Rust-Paris/events/267250053/).
* [Jan 23. Warsaw, PL - Rust Warsaw 3](https://www.meetup.com/Rust-Warsaw/events/267525144/).
* [Feb  2. Brussels, BE - Rust devroom @ FOSDEM](https://fosdem.org/2020/schedule/track/rust/).
* [Jan 22. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgrybccbdc/).
* [Jan 22. Hamburg, DE - Rust Hack & Learn January 2020](https://www.meetup.com/Rust-Meetup-Hamburg/events/267692684/).


### North America

* [Jan 14. Seattle, WA, US - Seattle Rust Meetup - Physical Computing Workshop](https://www.meetup.com/Seattle-Rust-Meetup/events/267538087/).
* [Jan 20. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Jan 22. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscrybccbdc/).
* [Jan 22. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/qgvxlrybccbdc/).

### South America

* [Jan 18. Sao Paulo, BR - Rust SP - Encontro Janeiro 2020](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/266858154/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> @ZiCog: Does anyone have a 'no holds barred, unsafe or not' solution to the problem in Rust that can match C?
>
> @kornel: Pipe the C version through c2rust :slight_smile:
>
> @ZiCog: Yay! Rust now beats both Clang and GCC!

– [ZiCog and Kornel on rust-users](https://users.rust-lang.org/t/clippy-driving-me-to-insanity-insisting-on-iterators/36796/19)

Thanks to [Jan Riemer](https://users.rust-lang.org/t/twir-quote-of-the-week/328/769) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
