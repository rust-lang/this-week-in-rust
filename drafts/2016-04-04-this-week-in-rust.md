Title: This Week in Rust 125
Number: 125
Date: 2016-04-04
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

# Crate of the Week

This week's Crate of the Week is [rustful](https://crates.io/crates/rustful), a simple, modular REST-like HTTP framework. Thanks to [Austin B](https://users.rust-lang.org/users/DroidLogician) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [medium] [buildengine5: Test `net::test::client_server_send` fails](https://github.com/Ameliorate/buildengine5/issues/10).
* [less easy] [Vulkano: Add a memory pool](https://github.com/tomaka/vulkano/issues/14).
* [easy] [Servo: Implement `HTMLScriptElement.type` and similar attributes](https://github.com/servo/servo/issues/10227).
* [easy] [Servo: Stop re-exporting webrender_traits WebGL types from canvas_traits](https://github.com/servo/servo/issues/10211).
* [easy] [Servo: Using viewport percentage lengths (e.g. vh, vw) causes unnecessary style recalc on resize](https://github.com/servo/servo/issues/10104).
* [easy] [Servo: Enable CCACHE for AppVeyor Windows builds](https://github.com/servo/servo/issues/9874).
* [easy] [`cargo add`: Target specifications](https://github.com/killercup/cargo-edit/issues/13).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

65 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-28..2016-04-04

## Notable changes

* [specialization of `str::to_string()`](https://github.com/rust-lang/rust/pull/32586)
* [weed out unneeded dependencies within rustc](https://github.com/rust-lang/rust/pull/32571)
* [lldb breakpoints get source file names](https://github.com/rust-lang/rust/pull/32522)
* [Parser recovery is behind debug flag until kinks worked out](https://github.com/rust-lang/rust/pull/32494)
* [Parsing after EOF is now ICE](https://github.com/rust-lang/rust/pull/32479) (better than possible infinite loops)
* [melt the ICE on lowering impossible range](https://github.com/rust-lang/rust/pull/32267)
* [`const_eval` and `check_match` now live in their own crate](https://github.com/rust-lang/rust/pull/32259)
* [MIR traversals, orbit bootstraps](https://github.com/rust-lang/rust/pull/32210)
* [private fields/methods no longer interfere with selection](https://github.com/rust-lang/rust/pull/31938)
* [RefCell/RefMut coercible to unsized](https://github.com/rust-lang/rust/pull/32652)
* [Arc now `compare_exchange`s instead of `compare_and_swap`](https://github.com/rust-lang/rust/pull/32643) (should be faster on ARM)
* [`HashMap`/`HashSet` and their iterators are now covariant](https://github.com/rust-lang/rust/pull/32635)
* [`BTree`/`HashMap::values_mut()`](https://github.com/rust-lang/rust/pull/32633)

## New Contributors

* Alejandro Wainzinger
* Andrew Horton
* Cyryl Płotnicki-Chudyk
* David Henningsson
* ituxbag
* Kevin Brothaler
* nicholasf
* Novotnik, Petr

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1201: Add support for naked functions](https://github.com/rust-lang/rfcs/pull/1201).
* [RFC 1422: Expand the current pub/non-pub categorization of items with the ability to say "make this item visible solely to a (named) module tree"](https://github.com/rust-lang/rfcs/pull/1422).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Add a contains method to VecDeque and LinkedList](https://github.com/rust-lang/rfcs/pull/1552).
* [Add libutil to scope of libc crate on Linux](https://github.com/rust-lang/rfcs/pull/1529).
* [Stabilize implementing panics as aborts](https://github.com/rust-lang/rfcs/pull/1513).
* [Add a standard allocator interface and support for user-defined allocators](https://github.com/rust-lang/rfcs/pull/1398).
* [Remove some kinds of doc comments](https://github.com/rust-lang/rfcs/pull/1373).
* [Unsafe expressions](https://github.com/rust-lang/rfcs/pull/1346).
* [Prevent unstable items from causing name resolution conflicts with downstream code](https://github.com/rust-lang/rfcs/pull/1321).
* [Amend RFC 1228 with operator fixity and precedence](https://github.com/rust-lang/rfcs/pull/1319).

## New RFCs

* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).

# Upcoming Events

* [3/31. Tokyo Rust Meetup #4](http://www.meetup.com/Tokyo-Rust-Meetup/events/229260081/).
* [4/6. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [4/6. Germany/Cologne - Hack and Learn](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/229919455/).
* [4/6. Rust São Paulo Meetup](http://www.meetup.com/Rust-Sao-Paulo-Meetup/events/229377422/).
* [4/11. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
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

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
