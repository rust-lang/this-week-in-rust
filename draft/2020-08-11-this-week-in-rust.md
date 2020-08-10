Title: This Week in Rust 351
Number: 351
Date: 2020-08-11
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/026-twir-350/)

# Updates from Rust Community

### Official


### Tooling


### Newsletters


### Observations/Thoughts


### Learn Standard Rust


### Learn More Rust
*[video][Using Linux libc in Rust - with the file-locker Crate](https://youtu.be/UgNrDb6hQQ0)


### Project Updates


### Miscellaneous


# Crate of the Week

This week's crate is [partial-io](https://lib.rs/crates/partial-io), a set of helpers to test partial, interrupted and would-block I/O operations.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/796) for the suggestion!

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

326 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-07-27..2020-08-03

* [suppress debuginfo on naked function arguments](https://github.com/rust-lang/rust/pull/74105)
* [normalize all opaque types when converting `ParamEnv` to `Reveal::All`](https://github.com/rust-lang/rust/pull/65989)
* [ensure stack when type checking and building MIR for large if expressions](https://github.com/rust-lang/rust/pull/74708)
* [replace a recursive algorithm with an iterative one](https://github.com/rust-lang/rust/pull/74983)
* [fix `#[track_caller]` shims for trait objects](https://github.com/rust-lang/rust/pull/74784)
* [make closures and generators `must_use` types](https://github.com/rust-lang/rust/pull/74869)
* [`BTreeMap::drain_filter` should not touch the root during iteration](https://github.com/rust-lang/rust/pull/74762)
* [add `str::`(`r`)`split_once`](https://github.com/rust-lang/rust/pull/74707)
* [add `Vec::spare_capacity_mut`](https://github.com/rust-lang/rust/pull/75015)
* [add `slice::array_chunks`](https://github.com/rust-lang/rust/pull/74373)
* [stabilize `const_type_id`](https://github.com/rust-lang/rust/pull/72488)
* [stabilize `Vec::leak` as a method](https://github.com/rust-lang/rust/pull/74605)
* [stabilize `Result::as_deref` and `as_deref_mut`](https://github.com/rust-lang/rust/pull/74948)
* [make `Option::unwrap` unstably const](https://github.com/rust-lang/rust/pull/74956)
* [make `mem::size_of_val` and `mem::align_of_val` unstably const](https://github.com/rust-lang/rust/pull/74930)
* [backtrace-rs: include source column numbers, where available](https://github.com/rust-lang/backtrace-rs/pull/367)

## Rust Compiler Performance Triage

* [2020-08-03](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-03.md).
  8 regressions, 2 improvements, 1 of them on rollups. 1 outstanding nag from last week.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: 'C unwind' ABI](https://github.com/rust-lang/rfcs/pull/2945)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

*No Tracking Issues or PRs are currently in the final comment period.*

## New RFCs

* [Procedural vtables and wide ptr metadata](https://github.com/rust-lang/rfcs/pull/2967)
* [Edition 2021 and beyond](https://github.com/rust-lang/rfcs/pull/2966)

# Upcoming Events

### Online
* [August 5. Johannesburg, ZA - Johannesburg Rust Meetup - Monthly Joburg Rust Chat](https://www.meetup.com/Johannesburg-Rust-Meetup/events/271875886/)
* [August 5. Dublin, IE - Rust Dublin - August Remote Meetup](https://www.meetup.com/Rust-Dublin/events/272162980/)
* [August 5. Buffalo, NY, US - Buffalo Rust Meetup - Rust User Group](https://www.meetup.com/Buffalo-Rust-Meetup/events/271511557/)
* [August 5. Indianapolis, IN, US - Indy Rust - Indy.rs with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybclbhb/)
* [August 6. Linz, AT - Rust Meetup Linz - Kick Off](https://www.meetup.com/de-DE/Rust-Linz/events/271857182/)
* [August 6. Berlin, DE - Berline.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybclbjb/)
* [August 11. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybclbpb/)
* [August 11. Saarbrücken, DE - Rust-Saar Meetup `3u16`](https://www.meetup.com/Rust-Saar/events/272044845/)
* [August 13. San Diego, CA, US - San Diego Rust - August 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/272060817/)

### North America
* [August 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybclbrb/)

### Asia Pacific

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> *Empowering* is the perfect word to describe Rust in 2020. What used to be a rough adventure with many pitfalls has turned into something beautiful, something that can lift your spirit. At least, that’s what it did for me.

- [Mathias Lafeldt on his blog](https://sharpend.io/giving-rust-another-shot-in-2020/)

Thanks to [Henrik Tougaard](https://users.rust-lang.org/t/twir-quote-of-the-week/328/915) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/i094wo/this_week_in_rust_349/)</small>
