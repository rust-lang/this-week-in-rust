Title: This Week in Rust 277
Number: 277
Date: 2019-03-12
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

* [Tools team changes](https://internals.rust-lang.org/t/tools-team-changes/9569).
* [Raspberry Pi 3: Hardware debugging using JTAG](https://github.com/rust-embedded/rust-raspi3-OS-tutorials/tree/master/0B_hw_debug_JTAG).
* [From 46s to 5s - Optimizing a 350 line raytracer in Rust](https://blog.usejournal.com/from-48s-to-5s-optimizing-a-350-line-pathtracer-in-rust-191ab4a1a412).
* [Should Rust channels panic on send if nobody's listening](http://www.randomhacks.net/2019/03/08/should-rust-channels-panic-on-send/)?
* [In nightly Rust, 'await!' may never return (dropping futures)](http://www.randomhacks.net/2019/03/09/in-nightly-rust-await-may-never-return/).
* [The rise of wgpu](https://gfx-rs.github.io/2019/03/06/wgpu.html).
* [Making a case: Rust for Python developers](https://medium.com/@rajasekar3eg/making-a-case-rust-for-python-developers-1a114e2d89f4).
* [Intro to web programming in Rust for NodeJS developers](https://medium.com/@gruberbastian/intro-to-web-programming-in-rust-for-nodejs-developers-1a9c048c4de1).
* [Remote development and debugging of Rust with CLion](https://medium.com/nearprotocol/remote-development-and-debugging-of-rust-with-clion-39c38ced7cc1).
* [This week in Rust and WebAssembly 13](https://rustwasm.github.io/2019/03/07/this-week-in-rust-and-wasm-013.html).
* [The Embedded WG newsletter 16](https://rust-embedded.github.io/blog/newsletter-16/).

# Crate of the Week

This week's crate is [validator](https://github.com/Keats/validator), a crate offering simple validation for Rust structs. Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/500) for the suggestion!

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

173 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-03-04..2019-03-11

* [Make the rustc driver and interface demand driven](https://github.com/rust-lang/rust/pull/56732)
* [Make the lifetime parameters of tcx consistent](https://github.com/rust-lang/rust/pull/58926)
* [Don't promote function calls to nonpromotable things](https://github.com/rust-lang/rust/pull/58784)
* [HirIdification: almost there](https://github.com/rust-lang/rust/pull/58915)
* [Monomorphize generator field types for debuginfo](https://github.com/rust-lang/rust/pull/58906)
* [Always emit unclosed delimiter diagnostics](https://github.com/rust-lang/rust/pull/58903)
* [Mention `unwind(aborts)` in diagnostics for `#[unwind]`](https://github.com/rust-lang/rust/pull/58762)
* [Add const generics to ty (and transitive dependencies)](https://github.com/rust-lang/rust/pull/58583)
* [Make the Entry API of HashMap<K, V> Sync and Send](https://github.com/rust-lang/rust/pull/58369)
* [On return type `impl Trait` for block with no expr point at last semi](https://github.com/rust-lang/rust/pull/58204)
* [Create a derive macro for HashStable and allow proc macros in rustc](https://github.com/rust-lang/rust/pull/58013)
* [Move index updates off the web server](https://github.com/rust-lang/crates.io/pull/1588)
* [Make `Unique::as_ptr`, `NonNull::dangling` and `NonNull::cast` const](https://github.com/rust-lang/rust/pull/58750)
* [MaybeUninit: add read_initialized, add examples](https://github.com/rust-lang/rust/pull/58660)
* [Add `as_slice()` to `slice::IterMut` and `vec::Drain`](https://github.com/rust-lang/rust/pull/58924)
* [cargo: Emit warning on misspelled environment variables](https://github.com/rust-lang/cargo/pull/6694)
* [rustdoc: add option to calculate "documentation coverage"](https://github.com/rust-lang/rust/pull/58626)

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

> – Rust’s ownership is difficult.
> – Yes, ownership is difficult. For such a difficult thing, you (are going to) ever check by hand rather than having machine do it for you.

– @Cryolite [on twitter](https://twitter.com/Cryolite/status/1104325100881375232) (translated from Japanese)

Thanks to [Xidorn Quan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/629) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/b0m6vv/this_week_in_rust_277/).</small>
