Title: This Week in Rust 299
Number: 299
Date: 2019-08-13
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

* [Parsing Rust Strings into Slices](https://wduquette.github.io/parsing-strings-into-slices)

# Crate of the Week

This week's crate is [broot](https://github.com/Canop/broot), a program to show the gist of a directory tree.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/596) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Actix: Call to community and participation](https://github.com/actix/actix-web/issues/1019).
* [Kate editor: Support Rust LSP server auto-detect some useful root path based on location of Cargo.toml](https://phabricator.kde.org/D22963).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

249 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-07-29..2019-08-05

* [Avoid ICE when suggestion span is at Eof](https://github.com/rust-lang/rust/pull/62995)
* [On `format!()` arg count mismatch provide extra info](https://github.com/rust-lang/rust/pull/63121)
* [Syntax: Recover on `for ( $pat in $expr ) $block`](https://github.com/rust-lang/rust/pull/62928)
* [dead_code: Properly inspect fields in struct patterns with type relative paths](https://github.com/rust-lang/rust/pull/63227)
* [Collect file → edition mapping after AST expansion](https://github.com/rust-lang/rls/pull/1513)
* [Unsupport the `await!(future)` macro](https://github.com/rust-lang/rust/pull/62293)
* [Round generator sizes to a multiple of their alignment](https://github.com/rust-lang/rust/pull/63208)
* [miri: Fix determining size of an "extra function" allocation](https://github.com/rust-lang/rust/pull/63076)
* [miri: Add misssing 'roundf32' and 'roundf64' intrinsics](https://github.com/rust-lang/miri/pull/885)
* [Impl `Debug` for `Chars`](https://github.com/rust-lang/rust/pull/63000)
* [`const fn`-ify `std::any::type_name`](https://github.com/rust-lang/rust/pull/63123)
* [hashbrown: Replace FxHash with AHash as the default hasher](https://github.com/rust-lang/hashbrown/pull/97)
* [hashbrown: Experimentally expose RawTable under the "raw" feature](https://github.com/rust-lang/hashbrown/pull/108)
* [rustc: Stabilize options for pipelined compilation](https://github.com/rust-lang/rust/pull/62766)
* [cargo: Enable pipelined compilation by default](https://github.com/rust-lang/cargo/pull/7143)
* [cargo: Improve error message for unmatched prerelease dependencies](https://github.com/rust-lang/cargo/pull/7191)
* [rustdoc: Use doc comments from 'pub use' statements](https://github.com/rust-lang/rust/pull/63048)

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

*No new RFCs were proposed this week.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [[Stabilization] async/await MVP](https://github.com/rust-lang/rust/issues/62149).
* [disposition: merge] [Stabilize duration_float](https://github.com/rust-lang/rust/pull/62756).
* [disposition: merge] [Stabilize checked_duration_since for 1.38.0](https://github.com/rust-lang/rust/pull/62860).
* [disposition: merge] [Give built-in macros stable addresses in the standard library](https://github.com/rust-lang/rust/pull/63056).
* [disposition: merge] [Tracking issue for {HashMap, BTreeMap}::get_key_value stabilization](https://github.com/rust-lang/rust/issues/49347).

## New RFCs

* [Add a new unsafe trait TypeInfo to core::any, and implement it for all types](https://github.com/rust-lang/rfcs/pull/2738).

# Upcoming Events

### Asia Pacific

* [Aug 10. Singapore, SG - Rust Meetup](https://www.eventbrite.com/e/rust-meetup-tickets-65358532129).
* [Aug 17. Taipei, TW - "Everything in Rust" at COSCUP 2019](https://coscup.org/2019/en/).
* [Aug 15. Wellington, NZ - Rust Wellington - Coffee & cake](https://www.meetup.com/Rust-Wellington/events/hgrxbryzlbtb/).

### Europe

* [Aug 21. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzlbcc/).
* [Aug 19. Berlin, DE - Rust Berlin - Rust for Decentralised Technology](https://www.meetup.com/Rust-Berlin/events/263390533).
* [Aug 21. Berlin, DE - In Rust We Trust - VM on Blockchain](https://www.meetup.com/Rust-in-Blockchain-Berlin/events/263526816/).

### North America

* [Aug 13. Toronto, ON, CA - Rust Toronto - Toronto Rustaceans Tech and Talk](https://www.meetup.com/Rust-Toronto/events/263395708).
* [Aug 13. Denver, CO, US - Rust Boulder/Denver - Hack 'N Snack](https://www.meetup.com/Rust-Boulder-Denver/events/263156621/).
* [Aug 13. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdryzlbrb/).
* [Aug 21. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzlbcc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).
* [Senior Platform Engineer - Layout as Mozilla, Portland, US](https://careers.mozilla.org/position/gh/1787784/).
* [Senior Software Engineer at ConsenSys R&D, Remote](https://consensys.net/open-roles/1792013/).
* [Rust Developer at Finhaven, Vancouver, CA](https://finhaven.humi.ca/job-board/engineering/1306).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> If you want to block threads, get your own threads.

– [kornel on rust-users](https://users.rust-lang.org/t/how-to-implement-a-future-for-a-long-running-function-i-can-not-modify/30610/12)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/679) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
