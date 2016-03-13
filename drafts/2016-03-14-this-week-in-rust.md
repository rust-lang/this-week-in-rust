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


## Notable New Crates & Project Updates


# Summer of Code Projects

Hi students! Looking for an awesome summer project in Rust? Look no further! Chris Holcombe from Canonical is an experienced Google Summer of Code mentor and has a project to implement CephX protocol decoding. [Check it out here](https://wiki.ubuntu.com/GoogleSoC2016/Ideas#Decode_CephX_Protocol).

Servo is also accepting GoSC project submissions under the Mozilla banner. See if any of the [project ideas](https://wiki.mozilla.org/Community:SummerOfCode16#Servo) appeal to you and read the [advice for applications](https://wiki.mozilla.org/Community:SummerOfCode16#Application_Advice).

Servo also has [a project](https://teams.railsgirlssummerofcode.org/projects/104-servo) in Rails Girls Summer of Code. nom is [also participating](https://teams.railsgirlssummerofcode.org/projects/78-nom).

# Crate of the Week

The crate of this week is [LALRPOP](https://crates.io/crates/lalrpop), a LR(1) parser generator that compiles to Rust code. Thanks to [ogeon](https://users.rust-lang.org/users/ogeon) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

142 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-07..2016-03-14

## Notable changes

* [implement RFC 1192 inclusive ranges](https://github.com/rust-lang/rust/pull/30884)
* [SIMD intrinsics for fused multiply-add](https://github.com/rust-lang/rust/pull/32066)
* [AVX broadcast/conversion intrinsics](https://github.com/rust-lang/rust/pull/32140)
* [New i586-pc-windows-msvc target](https://github.com/rust-lang/rust/pull/32034)
* [Huge-String-Slice-Overflow-DoS averted](https://github.com/rust-lang/rust/pull/32064)
* [Forbid having overlapping fns in inherent impls for same type](https://github.com/rust-lang/rust/pull/31925)
* [Add Pass manager for MIR](https://github.com/rust-lang/rust/pull/31916)
* [fn item type shenanigans](https://github.com/rust-lang/rust/pull/31710)
* [Fix name resolution in lexical scopes](https://github.com/rust-lang/rust/pull/32141)
* [import resolution fixed](https://github.com/rust-lang/rust/pull/32097)
* [optimize some std::process functions (more speed, less unsafe)](https://github.com/rust-lang/rust/pull/31618)
* [mark associated types as live for the dead_code lint](https://github.com/rust-lang/rust/pull/32158)


## New Contributors


## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:



## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period


## New RFCs


# Upcoming Events

* [3/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [3/21. Rust Paris](http://www.meetup.com/Rust-Paris)
* [4/15. Frankfurt/Main Rust Lint Workshop](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/229564640/?eventId=229564640)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [PhD and postdoc positions](http://plv.mpi-sws.org/rustbelt/) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No QotW were selected for this week.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
