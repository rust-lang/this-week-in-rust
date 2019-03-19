Title: This Week in Rust 278
Number: 278
Date: 2019-03-19
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

This week's crate is [copyless](https://crates.io/crates/copyless), a crate to extend boxes and vecs to operate on values while avoiding `memcpy`s. Thanks to [Dzmitry Malyshau](https://users.rust-lang.org/t/crate-of-the-week/2704/503) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Classic Unix utilities make great beginner projects](https://www.reddit.com/r/rust/comments/b0i625/classic_unix_utilities_make_great_beginner/).
* [TiKV: Set compile-time env vars in build script instead of Makefile](https://github.com/tikv/tikv/issues/4051)
* [TiKV: Use breakpad + symbolic to generate and interpret minidump-format core dumps](https://github.com/tikv/tikv/issues/4202)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

205 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-03-11..2019-03-18

* [Optimize copying large ranges of undefmask blocks](https://github.com/rust-lang/rust/pull/58556)
* [Unregress using scalar unions in constants](https://github.com/rust-lang/rust/pull/59139)
* [Ignore higher-ranked object bound conditions created by WF](https://github.com/rust-lang/rust/pull/59132)
* [Visit impl Trait for `dead_code` lint](https://github.com/rust-lang/rust/pull/59129)
* [rustc: Fix ICE when trait alias has bare Self](https://github.com/rust-lang/rust/pull/59118)
* [Fix ICE in MIR pretty printing](https://github.com/rust-lang/rust/pull/59036)
* [resolve: Account for new importable entities](https://github.com/rust-lang/rust/pull/59047)
* [Remove restriction on `isize`/`usize` in `repr(simd)`](https://github.com/rust-lang/rust/pull/59201)
* [Add `Cmp::clamp` for ranges](https://github.com/rust-lang/rust/pull/58710)
* [Stabilize `Range*::contains`](https://github.com/rust-lang/rust/pull/59152)
* [Change `std::fs::copy` to use `copyfile` on MacOS and iOS](https://github.com/rust-lang/rust/pull/58901)
* [Stabilize `Option::copied`](https://github.com/rust-lang/rust/pull/59231)
* [cargo: Fix resolving yanked crates when using a local registry](https://github.com/rust-lang/cargo/pull/6742)
* [cargo: Stricter package change detection](https://github.com/rust-lang/cargo/pull/6740)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2592: stabilize `std::task` and `std::future::Future`](https://github.com/rust-lang/rfcs/pull/2592).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Stabilize the alloc crate](https://github.com/rust-lang/rfcs/pull/2480).
* [disposition: merge] [Transparent Unions and Enums](https://github.com/rust-lang/rfcs/pull/2645).
* [disposition: postpone] [Initial pipeline](https://github.com/rust-lang/rfcs/pull/2656).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add provided methods `Seek::{stream_len, stream_position}`](https://github.com/rust-lang/rust/pull/58422).
* [disposition: merge] [Tracking issue for Ref/RefMut::map_split](https://github.com/rust-lang/rust/issues/51476).

## New RFCs

* [Roadmap for 2019](https://github.com/rust-lang/rfcs/pull/2657).
* [Amend the template from RFC 1589](https://github.com/rust-lang/rfcs/pull/2658).

# Upcoming Events

### Online

* [Mar 27. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 27. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Asia Pacific

* [Mar 16. Chennai, IN - Rust Chennai monthly meetup](https://www.meetup.com/mad-rs/events/259616775).
* [Mar 20. Tokyo, JP - Tokyo Rust Game Hack](https://connpass.com/event/122171/).
* [Mar 25. Melbourne, AU - Rust Melbourne Meetup](https://www.meetup.com/Rust-Melbourne/events/259230502/).

### Europe

* [Mar 14. Brno, CZ - Rust Brno Meetup at Masaryk University](https://rust-brno.github.io/).
* [Mar 14. Göteborg, SE - Rust Gothenburg](https://www.meetup.com/rustgbg/events/259386306/).
* [Mar 17. St. Petersburg, RU - St. Petersburg Rust Meetup](https://www.meetup.com/spbrust/events/whmxrqyzfbwb/).
* [Mar 19. Nijmegen, NL - Rust Nijmegen: Rust for the (Inter)Net - API's, HTTP/3 and Tide](https://www.meetup.com/Rust-Nijmegen/events/258758167).
* [Mar 20. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzfbbc/).
* [Mar 21. Torino, IT - Turin Rust Meetup](https://www.meetup.com/Mozilla-Torino/events/ktqcpqyzfblc/).
* [Mar 26. Berlin, DE - Rust and Tell Berlin](https://www.meetup.com/Rust-Berlin/events/szgnqqyzfbjc/).
* [Apr 04. Wroclaw, PL - Rust Wroclaw Meetup](https://www.meetup.com/Rust-Wroclaw/events/259511136/).

### North America

* [Mar 14. Columbus, US - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dbcfrpyzfbsb/).
* [Mar 14. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/258703993/).
* [Mar 14. San Diego, US - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/259691778/).
* [Mar 19. Austin, US - March Meetup](https://www.meetup.com/rust-atx/events/259273075/).
* [Mar 20. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/gqbksqyzfbbc/).
* [Mar 20. Ciudad de México, MX - Study group RustMX](https://www.meetup.com/Rust-MX/events/259473311/).
* [Mar 25. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzfbhc/).
* [Mar 27. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzfbkc/).
* [Mar 27. Mesa, US - Phoenix Rust: Scientific Computing](https://www.meetup.com/Desert-Rustaceans/events/259615926/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at ANIXE, Wrocław, PL](https://anixe.bamboohr.co.uk/jobs/view.php?id=72).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Sadly, no quote was nominated this week.

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
