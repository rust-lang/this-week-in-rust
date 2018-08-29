Title: This Week in Rust 249
Number: 249
Date: 2018-08-28
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

* [Programming Servo: The Debug Way](https://medium.com/coding-neutrino-blog/programming-servo-the-debug-way-5db01f09b7f4). <small>[[discuss](https://www.reddit.com/r/rust/comments/9anveo/)]</small>
* [Programming Servo: the makings of a task-queue](https://medium.com/programming-servo/programming-servo-the-makings-of-a-task-queue-b4138cd246ca). <small>[[discuss](https://www.reddit.com/r/rust/comments/9axo53/programming_servo_the_makings_of_a_taskqueue/)]</small>

# Crate of the Week

This week's crate is [generational-arena](https://github.com/lukaslueg/macro_railroad), a safe arena allocator that allows deletion without suffering from the ABA problem by using generational indices. Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/447) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for help building a distributed filesystem in Rust](https://www.reddit.com/r/rust/comments/98d3zk/call_for_help_building_a_distributed_filesystem/).
* [easy] [rustc: mark applicability of diagnostic suggestions](https://github.com/rust-lang/rust/issues/50723).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

157 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-08-13..2018-08-20

* [wasm: remove --strip-debug argument to LLD](https://github.com/rust-lang/rust/pull/53434)
* [resolve: overhaul `#![feature(uniform_paths)]` error reporting](https://github.com/rust-lang/rust/pull/53427)
* [do not suggest conversion method that is already there](https://github.com/rust-lang/rust/pull/53406)
* [visit all attributes for feature collection](https://github.com/rust-lang/rust/pull/53397)
* [speed up NLL with HybridIdxSetBuf](https://github.com/rust-lang/rust/pull/53383)
* [`{to,from}_{ne,le,be}_bytes` for unsigned integer types](https://github.com/rust-lang/rust/pull/53358)
* [fix error for unsized packed struct field](https://github.com/rust-lang/rust/pull/53342)
* [resolve: crates only exist in the type namespace](https://github.com/rust-lang/rust/pull/53335)
* [`Self` in type definitions](https://github.com/rust-lang/rust/pull/53324)
* [`TokenStream::extend`](https://github.com/rust-lang/rust/pull/53304)
* [various changes to `rustc_on_unimplemented`](https://github.com/rust-lang/rust/pull/53295)
* [NLL: optimize reassignment immutable state](https://github.com/rust-lang/rust/pull/53258)
* [don't accept non-string literals for the format string in writeln](https://github.com/rust-lang/rust/pull/53256)
* [don't panic on std::env::vars() when env is null](https://github.com/rust-lang/rust/pull/53208)
* [implement Iterator::size_hint for Elaborator](https://github.com/rust-lang/rust/pull/52858)
* [non-naive implementation of `VecDeque.append`](https://github.com/rust-lang/rust/pull/52553)
* [implement Unsized Rvalues](https://github.com/rust-lang/rust/pull/51131)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2504: Fix the Error trait](https://github.com/rust-lang/rfcs/pull/2504).
* [RFC 2351: Add `is_sorted` to the standard library](https://github.com/rust-lang/rfcs/pull/2351).
* [RFC 2229: Closures Capture Disjoint Fields](https://github.com/rust-lang/rfcs/pull/2229).
* [RFC 1892: Deprecate uninitialized in favor of a new MaybeUninit type](https://github.com/rust-lang/rfcs/pull/1892).
* [RFC 2306: Add `pub fn identity<T>(x: T) -> T { x }` to `core::convert`](https://github.com/rust-lang/rfcs/pull/2306).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Unify std::os::raw::c_void and libc::c_void via libcore](https://github.com/rust-lang/rfcs/pull/2521).
* [disposition: merge] [Add lint warning for inner function marked as `#[test]`](https://github.com/rust-lang/rfcs/pull/2471).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add Error::source method per RFC 2504.](https://github.com/rust-lang/rust/pull/53533).
* [disposition: merge] [set cfg(rustdoc) when rustdoc is running on a crate](https://github.com/rust-lang/rust/pull/53076).
* [disposition: merge] [Tracking issue for RFC#1685: Deprecate anonymous parameters ](https://github.com/rust-lang/rust/issues/41686).
* [disposition: merge] [Tracking issue for lifetime elision for impl headers (feature impl_header_lifetime_elision)](https://github.com/rust-lang/rust/issues/15872).

## New RFCs

* [Keeping Secrets in Rust](https://github.com/rust-lang/rfcs/issues/2533).
* [RFC: Associated type defaults and Default groups](https://github.com/rust-lang/rfcs/pull/2532).
* [Amend RFC 2175 to support for loops and leading vert](https://github.com/rust-lang/rfcs/pull/2530).
* [RFC: Hidden trait implementations](https://github.com/rust-lang/rfcs/pull/2529).
* [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528).
* [Support underscores as constant names](https://github.com/rust-lang/rfcs/pull/2526).
* [RFC: Permit _ in type aliases](https://github.com/rust-lang/rfcs/pull/2524).

# Upcoming Events

### Online

* [Aug 29. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Aug 29. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Sep  5. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Sep  5. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Sep 11. Rust Community Content Subteam Meeting at channel #rust-community](irc://irc.mozilla.org/rust-community).

### Africa

* [Aug 25. Nairobi, KE - Rustbridge Workshop at GirlsCode](https://www.meetup.com/Rust-Nairobi/events/253950971/).
* [Sep  4. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxmbgb/).

### Asia

* [Sep  2. Tel Aviv, IL - Cargo, Production and N00bing](https://www.meetup.com/Rust-TLV/events/253408497/).

### Europe

* [Sep  4. Brussels, BE - #3 futures/async/tokio && Gotham-rs](https://www.meetup.com/Belgium-Rust-user-group/events/249899651/).
* [Sep  5. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/253541000/).
* [Sep  5. Cologne, DE - Rust Cologne](http://rust.cologne/2018/09/05/fun-traits.html).
* [Sep  7. Helsinki, FIN - Helsinki Rust informal meetup](https://www.meetup.com/Finland-Rust-Meetup/).

### North America

* [Sep  2. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbdb/).
* [Sep  5. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxmbhb/).
* [Sep  5. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxmbhb/).
* [Sep  5. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxmbhb/).
* [Sep  5. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/246726699/).
* [Sep  9. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View).
* [Sep 10. Seattle, US - Seattle Rust Meetup](http://www.meetup.com/Seattle-Rust-Meetup/).

### South America

* [Aug 23. Montevideo, UY - Rust meetup Montevideo](https://www.meetup.com/Rust-Uruguay/events/253617627/).
* [Sep  3. Montevideo, UY - Rust meetup - WebAssembly](https://www.meetup.com/Rust-Uruguay/events/253617627/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer - Blockchain at TenX, Sydney, AU](https://tenx.workable.com/jobs/689268).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*


> # Bastion of the Turbofish
>
> Beware travellers, lest you venture into waters callous and unforgiving, where hope must abandoned, ere it is cruelly torn from you. For here stands the bastion of the Turbofish: an impenetrable fortress holding unshaking against those who would dare suggest the supererogation of the Turbofish.
>
> Once I was young and foolish and had the impudence to imagine that I could shake free from the coils by which that creature had us tightly bound. I dared to suggest that there was a better way: a brighter future, in which Rustaceans both new and old could be rid of that vile beast. But alas! In my foolhardiness my ignorance was unveiled and my dreams were dashed unforgivingly against the rock of syntactic ambiguity.
>
> This humble program, small and insignificant though it might seem, demonstrates that to which we had previously cast a blind eye: an ambiguity in permitting generic arguments to be provided without the consent of the Great Turbofish. Should you be so naïve as to try to revolt against its mighty clutches, here shall its wrath be indomitably displayed. This program must pass for all eternity, fundamentally at odds with an impetuous rebellion against the Turbofish.
>
> My heart aches in sorrow, for I know I am defeated. Let this be a warning to all those who come after. Here stands the bastion of the Turbofish.

– [varkor on the rust github](https://github.com/rust-lang/rust/pull/53562).

Thanks to [Mazdak Farrokhzad](https://users.rust-lang.org/u/Centril) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>Discuss this issue on [r/rust]()</small>
