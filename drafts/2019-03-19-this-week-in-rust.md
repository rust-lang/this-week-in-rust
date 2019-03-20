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
* [Web Programming in Rust - 02/x: Deployments](https://dev.to/gruberb/web-programming-in-rust-02x-deploy-your-first-app-1k05)

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
* [gfx-rs call for participation](https://users.rust-lang.org/t/gfx-rs-call-for-participation/26410)

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

* [disposition: e] [Initial pipeline rfc](https://github.com/rust-lang/rfcs/pull/2656).
* [disposition: merge] [Transparent Unions and Enums](https://github.com/rust-lang/rfcs/pull/2645).
* [disposition: merge] [Stabilize the alloc crate](https://github.com/rust-lang/rfcs/pull/2480).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add provided methods `Seek::{stream_len, stream_position}`](https://github.com/rust-lang/rust/pull/58422).
* [disposition: merge] [Tracking issue for {f32, f64}::copysign](https://github.com/rust-lang/rust/issues/58046).

## New RFCs

* [`std` Aware Cargo](https://github.com/rust-lang/rfcs/pull/2663).

# Upcoming Events

### Online

* [Mar 25. Rust Community Content Subteam Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Mar 27. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 27. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Mar 30. Nairobi, KE - Rust Nairobi Meetup: Projects Showcase](https://www.meetup.com/Rust-Nairobi/events/259650701/).

### Asia Pacific

* [Mar 24. Auckland, NZ - Rust AKL Meetup, Auckland](https://www.meetup.com/rust-akl/events/259480499/).
* [Mar 25. Melbourne, AU - Rust Melbourne Meetup](https://www.meetup.com/Rust-Melbourne/events/259230502/).
* [Mar 25. 東京, JP - Tokyo Rust Meetup](https://connpass.com/event/122171/).

### Europe

* [Mar 19. Nijmegen, NL - Rust Nijmegen: Rust for the (Inter)Net - API's, HTTP/3 and Tide](https://www.meetup.com/Rust-Nijmegen/events/258758167).
* [Mar 20. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzfbbc/).
* [Mar 21. Kharkiv, UA -  PeerLab Kharkiv #Rust](https://dou.ua/calendar/25951/).
* [Mar 21. Grenoble, FR - Rust Meetup - FLOSS Grenoble](https://www.meetup.com/FLOSS-Grenoble/events/259118704/).
* [Mar 21. Torino, IT - Turin Rust Meetup](https://www.meetup.com/Mozilla-Torino/events/ktqcpqyzfblc/).
* [Mar 26. Berlin, DE - Rust and Tell Berlin](https://www.meetup.com/Rust-Berlin/events/szgnqqyzfbjc/).
* [Mar 28. Copenhagen, DK - Copenhagen Rust Hack Night #14](https://cph.rs/).
* [Mar 28. Toulouse, FR - Rust Toulouse meetup](https://www.meetup.com/fr-FR/Toulouse-Rust-Meetup/events/259589986/).
* [Apr  3. Sandown, ZA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/gpxrtqyzgbfb/).
* [Apr  3. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzgbfb/).
* [Apr 04. Wroclaw, PL - Rust Wroclaw Meetup](https://www.meetup.com/Rust-Wroclaw/events/259511136/).

### North America

* [Mar 20. Vancouver, CN - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/gqbksqyzfbbc/).
* [Mar 20. McClellan Park, US - Sacramento Rust Meetup](https://www.meetup.com/Rust-Sacramento/events/259172607/).
* [Mar 20. Ciudad de México, MX - Study group RustMX](https://www.meetup.com/Rust-MX/events/259473311/).
* [Mar 25. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzfbhc/).
* [Mar 27. Chicago, US - Chicago Rust Meetup - Futures in Haskell and Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/259343384).
* [Mar 27. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzfbkc/).
* [Mar 27. Mesa, US - Phoenix Rust: Scientific Computing](https://www.meetup.com/Desert-Rustaceans/events/259615926/).
* [Apr  3. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/246726699/).
* [Apr  3. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/lgtvsqyzgbfb/).
* [Apr  3. Vancouver, CN - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/gqbksqyzgbfb/).

### South America

* [Mar 29. Montevideo, UY - Rust Latam @ Montevideo, Uruguay](None).

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
