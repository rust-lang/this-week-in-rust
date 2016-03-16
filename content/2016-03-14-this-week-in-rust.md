Title: This Week in Rust 122
Number: 122
Date: 2016-03-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts

* [video] [Chrome vs Firefox vs Safari vs Servo WebRender](https://youtu.be/u0hYIRQRiws).
* [Machine learning in Rust](https://athemathmo.github.io/2016/03/07/rusty-machine.html).
* [Rust testing with Stainless](http://bettong.net/2016/03/09/rust-testing-with-stainless/). Stainless is a lightweight, flexible, unopinionated testing framework.
* [This week in Servo 54](http://blog.servo.org/2016/03/07/twis-54/). Servo is a web browser engine written in Rust.
* [This week in Redox 12](http://www.redox-os.org/news/this-week-in-redox-12/). Redox is an operating-system written in Rust.
* [These weeks in Amethyst 7](http://blog.amethyst.rs/2016/03/13/twia-7/). Amethyst is a data-oriented game engine written in Rust.

## Notable New Crates & Project Updates

* [Library stabilizations for the 1.9 release](https://internals.rust-lang.org/t/library-stabilizations-for-the-1-9-release/3236).
* [Multirust 0.8](https://users.rust-lang.org/t/multirust-0-8-with-cross-std-installation/4901) now enables installation of additional standard libraries for cross compilation.
* [regex crate can match regular expressions on arbitrary bytes](https://doc.rust-lang.org/regex/regex/bytes/index.html).
* [Pencil](https://fengsp.github.io/blog/2016/3/introducing-pencil/). A web application microframework for Rust.
* [preferences](https://github.com/AndyBarron/preferences-rs). Read and write user-specific application data in Rust.
* [Proteus](https://github.com/wireapp/proteus). Axolotl crypto protocol implementation in Rust.
* [libterm](https://github.com/Ticki/libterm). A pure Rust library for handling, manipulating and reading information about terminals. This provides a full-featured alternative to Termbox.
* [dryad](https://github.com/m4b/dryad). An almost-parallel, semi-functioning, dynamic linker experiment, written in Rust.

# Summer of Code Projects

Hi students! Looking for an awesome summer project in Rust? Look no further! Chris Holcombe from Canonical is an experienced Google Summer of Code mentor and has a project to implement CephX protocol decoding. [Check it out here](https://wiki.ubuntu.com/GoogleSoC2016/Ideas#Decode_CephX_Protocol).

Servo is also accepting GSoC project submissions under the Mozilla banner. See if any of the [project ideas](https://wiki.mozilla.org/Community:SummerOfCode16#Servo) appeal to you and read the [advice for applications](https://wiki.mozilla.org/Community:SummerOfCode16#Application_Advice).

Servo also has [a project](https://teams.railsgirlssummerofcode.org/projects/104-servo) in Rails Girls Summer of Code. nom is [also participating](https://teams.railsgirlssummerofcode.org/projects/78-nom).

# Crate of the Week

This week's Crate of the Week is [LALRPOP](https://crates.io/crates/lalrpop), a LR(1) parser generator that compiles to Rust code. Thanks to [ogeon](https://users.rust-lang.org/users/ogeon) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Easy] [Servo: Reduce duplication of the "android-18" property in the Android build](https://github.com/servo/servo/issues/8348).
* [Less easy] [Servo: Add support for :target pseudo-selector](https://github.com/servo/servo/issues/7720).
* [Hard] [`cargo add`: Infer crate name from path/git repo](https://github.com/killercup/cargo-edit/issues/14).
* [Easy] [`cargo add`: Target specifications](https://github.com/killercup/cargo-edit/issues/13).
* [Easy] [`cargo list`: More tests](https://github.com/killercup/cargo-edit/issues/16).
* [Easy/Mentored] [`multipart`: create sample projects](https://github.com/cybergeek94/multipart/issues/29)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

142 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-07..2016-03-14

## Notable changes

* [implement RFC 1192 inclusive ranges](https://github.com/rust-lang/rust/pull/30884).
* [SIMD intrinsics for fused multiply-add](https://github.com/rust-lang/rust/pull/32066).
* [AVX broadcast/conversion intrinsics](https://github.com/rust-lang/rust/pull/32140).
* [New i586-pc-windows-msvc target](https://github.com/rust-lang/rust/pull/32034).
* [Huge-String-Slice-Overflow-DoS averted](https://github.com/rust-lang/rust/pull/32064).
* [Forbid having overlapping fns in inherent impls for same type](https://github.com/rust-lang/rust/pull/31925).
* [Add Pass manager for MIR](https://github.com/rust-lang/rust/pull/31916).
* [fn item types are properly unique and have size zero](https://github.com/rust-lang/rust/pull/31710).
* [Add impl of FnOnce to AssertRecoverSafe](https://github.com/rust-lang/rust/pull/32102).
* [Fix name resolution in lexical scopes](https://github.com/rust-lang/rust/pull/32141).
* [import resolution fixed](https://github.com/rust-lang/rust/pull/32097).
* [optimize some std::process functions (more speed, less unsafe)](https://github.com/rust-lang/rust/pull/31618).
* [mark associated types as live for the dead_code lint](https://github.com/rust-lang/rust/pull/32158).


## New Contributors

* Andrew Cantino
* Andrey Cherkashin
* Craig M. Brandenburg
* Noah
* Tang Chenglong
* Tim Montague
* vagrant

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week!*.

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add support for naked functions](https://github.com/rust-lang/rfcs/pull/1201).
* [Expand the current pub/non-pub categorization of items with the ability to say "make this item visible solely to a (named) module tree"](https://github.com/rust-lang/rfcs/pull/1422).
* [Add octet-oriented interface to `std::net::Ipv6Addr`](https://github.com/rust-lang/rfcs/pull/1498).
* [Unix socket support in the standard library](https://github.com/rust-lang/rfcs/pull/1479).
* [Implement a method, `contains()`, for `Range`, `RangeFrom`, and `RangeTo`, checking if a number is in the range.](https://github.com/rust-lang/rfcs/pull/1434).
* [Add a `replace_slice` method to `Vec<T>` and `String` which removes a range of elements, and replaces it in place with a given sequence of values](https://github.com/rust-lang/rfcs/pull/1432).

## New RFCs

* [Saturating and checking integer wrapper types](https://github.com/rust-lang/rfcs/pull/1534).
* [Stabilize the `-C overflow-checks` command line argument](https://github.com/rust-lang/rfcs/pull/1535).
* [Revise type ascription operator to use type equality, not coercion](https://github.com/rust-lang/rfcs/pull/1539).
* [Add `TryFrom` and `TryInto` traits](https://github.com/rust-lang/rfcs/pull/1542).
* [Add more integer atomic types](https://github.com/rust-lang/rfcs/pull/1543).

# Upcoming Events

* [3/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [3/16. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [3/17. Rust London Meetup #4](http://www.meetup.com/Rust-London-User-Group/events/229413056/).
* [3/21. Rust Paris](http://www.meetup.com/Rust-Paris)
* [4/23. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [4/15. Frankfurt/Main Rust Lint Workshop](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/229564640/?eventId=229564640)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.
* [Rust developer](http://rust.jobboard.io/jobs/125594-rust-developer-at-the-blackbird) at The Blackbird.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is not what makes these projects awesome. These projects are what make Rust awesome.

— [Manish on twitter](https://twitter.com/ManishEarth/status/707222273871052800).

Thanks to [llogiq](https://users.rust-lang.org/users/llogiq) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
