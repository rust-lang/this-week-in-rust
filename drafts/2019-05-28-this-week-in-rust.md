Title: This Week in Rust 288
Number: 288
Date: 2019-05-28
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

This week's crate is [mockiato](https://github.com/myelin-ai/mockiato), a strict yet friendly mocking library for Rust 2018. Thanks to [Ruben Schmidmeister](https://users.rust-lang.org/t/crate-of-the-week/2704/550) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Running a Rust Belt RustBridge](https://users.rust-lang.org/t/rust-belt-rustbridges/28332).
* [Submit your experience for newly await syntax](https://internals.rust-lang.org/t/async-await-experience-reports/10200).
* [www.rust-lang.org: Frontend cleanup: Remove Skeleton](https://github.com/rust-lang/www.rust-lang.org/issues/780).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

286 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-05-20..2019-05-27

* [Turn turbo 🐟 🍨 into an error](https://github.com/rust-lang/rust/pull/61189)
* [Remove `ObsoleteInPlace`](https://github.com/rust-lang/rust/pull/60803)
* [Make place projections concrete](https://github.com/rust-lang/rust/pull/60441)
* [Simplify use of keyword symbols](https://github.com/rust-lang/rust/pull/60740)
* [Fix overflowing literal lint in loops](https://github.com/rust-lang/rust/pull/61098)
* [Use `Symbol` even more](https://github.com/rust-lang/rust/pull/60815)
* [Use `Symbol` more in lint APIs](https://github.com/rust-lang/rust/pull/60827)
* [Move gensym operations from `Symbol` to `Ident`](https://github.com/rust-lang/rust/pull/60903)
* [Avoid symbol interning in `file_metadata`](https://github.com/rust-lang/rust/pull/60973)
* [Avoid more symbol interning](https://github.com/rust-lang/rust/pull/61035)
* [Don't arena-allocate static symbols](https://github.com/rust-lang/rust/pull/61077)
* [rustc: Improve type size assertions](https://github.com/rust-lang/rust/pull/60959)
* [Allow null-pointer-optimized enums in FFI if their underlying representation is FFI safe](https://github.com/rust-lang/rust/pull/60300)
* [Preserve local scopes in generator MIR](https://github.com/rust-lang/rust/pull/60840)
* [Annotate each `reverse_bits` with `#[must_use]`](https://github.com/rust-lang/rust/pull/61134)
* [Vec: Avoid creating slices to the elements](https://github.com/rust-lang/rust/pull/61114)
* [Fix dangling reference in `Vec::append`](https://github.com/rust-lang/rust/pull/61082)
* [crates.io: Further address performance regression in search](https://github.com/rust-lang/crates.io/pull/1749)
* [rustbuild: Add clippy and fix commands to x.py](https://github.com/rust-lang/rust/pull/56595)

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

* [disposition: merge] [Named custom cargo profiles](https://github.com/rust-lang/rfcs/pull/2678).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize RefCell::try_borrow_unguarded](https://github.com/rust-lang/rust/pull/60850).
* [disposition: merge] [Tracking issue for reversing the bit pattern in an integer](https://github.com/rust-lang/rust/issues/48763).
* [disposition: merge] [BufReader/Writer extension methods: is_empty, buffer](https://github.com/rust-lang/rust/issues/45323).
* [disposition: merge] [Bors policy question: Auto-reassignment on r+](https://github.com/rust-lang/rust/issues/59489).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Africa

* [Jun  5. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/gpxrtqyzhbcb/).

### Asia

* [May 25. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/381254712479005/).
* [May 27. Tokyo, JP - Tokyo Rust Meetup - Rust LT #5](https://rust.connpass.com/event/129406/).

### Europe

* [May 23. Kharkiv, UA - PeerLab Kharkiv #Rust: Rust Success Stories](https://www.meetup.com/Native-Developers-in-UA/events/261508593/).
* [May 23. Paris, FR - Rust Paris meetup #45](https://www.meetup.com/Rust-Paris/events/260925527/).
* [May 26. St. Petersburg, RU - St. Petersburg Rust Meetup](https://www.meetup.com/spbrust/events/whmxrqyzhbjc/).
* [May 29. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzhbmc/).
* [May 28. Vilnius, LT - Rust Vilnius - Rust Safety and Distributed Consensus](https://www.meetup.com/Rust-in-Vilnius/events/260937510/).
* [Jun  6. Wroclaw, PL - Rust Wroclaw Meetup #11](https://www.meetup.com/Rust-Wroclaw/events/261283360/).

### North America

* [May 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzhbkc/).
* [May 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzhblc/).
* [May 29. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzhbmc/).
* [May 29. Chicago, IL, US - Chicago Rust Meetup - Unsafe Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/260918979).
* [Jun  5. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/kkzkxqyzjbhb/).
* [Jun  5. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzjbhb/).

### South America

* [Jun 1. Sao Paulo, BR - Rust SP - Encontro Junho 2019](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/261123153/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Software Engineer, Backend - Rust at Kraken, Berlin, DE or remote](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105).
* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I used to think of programs as execution flowing and think about what the CPU is doing. As I moved to rust I started thinking a lot more about memory: how the data was laid out in memory, and how ownership of different parts of memory is given to different parts of the program at run time.

[Oliver Gould on "The Open Source Show: All About Rust](https://youtu.be/FYGS2q1bljE?t=280)

Thanks to [PrototypeNM1](https://users.rust-lang.org/t/twir-quote-of-the-week/328/643) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
