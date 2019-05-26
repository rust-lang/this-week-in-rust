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
* [How to sequence in Rust](https://dev.to/kdrakon/how-to-sequence-in-rust-4nhl)
* [Rust's Bindgen + Fuse in 2019](https://dev.to/kdrakon/rust-s-bindgen-fuse-in-2019-2e8l)
* [Creating a 'paged' Vec in Rust](https://dev.to/kdrakon/creating-a-paged-vec-in-rust-193l)

# Crate of the Week

This week we have two crates: [memory-profiles](https://github.com/nokia/memory-profiler), does what it says on the box. [momo](https://github.com/llogiq/momo) is a procedural macro that outlines generic conversions to reduce monomorphized code. Thanks to [ehsanmok](https://users.rust-lang.org/t/crate-of-the-week/2704/549) and llogiq for the suggestion!

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

240 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-05-13..2019-05-20

* [Move token tree related lexer state to a separate struct](https://github.com/rust-lang/rust/pull/60763)
* [Stop using gensyms in HIR lowering](https://github.com/rust-lang/rust/pull/60960)
* [Fix more escaping ReScopes](https://github.com/rust-lang/rust/pull/60765)
* [Perform constant propagation into terminators](https://github.com/rust-lang/rust/pull/60745)
* [Do some simple constant propagation in the `ConstProp` pass](https://github.com/rust-lang/rust/pull/60597)
* [Test interaction of unions with non-zero/niche-filling optimization](https://github.com/rust-lang/rust/pull/60590)
* [Forego caching for all participants in cycles, apart from root node](https://github.com/rust-lang/rust/pull/60444)
* [Mark `core::alloc::Layout::from_size_align_unchecked` const](https://github.com/rust-lang/rust/pull/60370)
* [Remove the unstable and deprecated `mpsc_select`](https://github.com/rust-lang/rust/pull/60921)
* [Stabilize core parts of `MaybeUninit`](https://github.com/rust-lang/rust/pull/60445)
* [Stabilize `vecdeque_rotate`](https://github.com/rust-lang/rust/pull/60678)
* [Add entry-like methods to `HashSet`](https://github.com/rust-lang/rust/pull/60894)
* [Add implementations of `last` in terms of `next_back` on a bunch of `DoubleEndedIterators`](https://github.com/rust-lang/rust/pull/60130)
* [Fix display of const generics in rustdoc](https://github.com/rust-lang/rust/pull/60760)
* [rustup: Avoid blocking on `CloseHandle`](https://github.com/rust-lang/rustup.rs/pull/1850)
* [rustc-guide: Add documentation about profile-guided optimization](https://github.com/rust-lang/rustc-guide/pull/318)
* [lint: convert `incoherent_fundamental_impls` into hard error](https://github.com/rust-lang/rust/pull/49799)
* [clippy: Prevent symbocalypse](https://github.com/rust-lang/rust-clippy/pull/4110)
* [crates.io: Fix performance regression on crate search](https://github.com/rust-lang/crates.io/pull/1746)

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

> Just the presence of well integrated Algebraic Data Types (ADTs) makes an incredible amount of difference. They are used to represent errors in a meaningful and easy to understand way (`Result<T>`), are used to show that a function may or may not return a meaningful value without needing a garbage value (`Option<T>`), and the optional case can even be used to wrap a null pointer scenario in a safe way (Option<Ref<T>> being the closest to a literal translation I think).
>
> That’s just one small feature that permeates the language. Whatever the opposite of a death-of-a-thousand-cuts is, Rust has it.

[tomcatfish on the orange website](https://news.ycombinator.com/item?id=19922344)

Thanks to [PrototypeNM1](https://users.rust-lang.org/t/twir-quote-of-the-week/328/643) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
