Title: This Week in Rust 89
Date: 2015-07-27
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: WHO??

# From the Blogosphere

# Tips & Tricks

# In the News

# New Releases & Project Updates

# What's cooking on nightly?

XXX pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-07-20..2015-07-27

* [Rewrite the improper_ctypes
  lint](https://github.com/rust-lang/rust/pull/26583). Makes the lint
  a bit more accurate , and improves the quality of the diagnostic
  messages.
* [Resolve lifetime parameters for foreign functions](https://github.com/rust-lang/rust/pull/26588)
* [Allow and implement recursive static variables](https://github.com/rust-lang/rust/pull/26630)
* [Implement default type-parameter fallback](https://github.com/rust-lang/rust/pull/26870)
* [Create proper debug info for functions and function pointers](https://github.com/rust-lang/rust/pull/27025)
* [Fix negate_unsigned feature gate check](https://github.com/rust-lang/rust/pull/27026)
* [Add `IntoRaw{Fd,Handle,Socket}` traits](https://github.com/rust-lang/rust/pull/27064)
* [Get cargo working on i686-pc-windows-msvc](https://github.com/rust-lang/cargo/pull/1825)

# New Contributors

* Andy Caldwell
* Antti Keränen
* eternaleye
* Jason Schein
* Jonathan Hansford
* Kornel Lesiński
* Leif Arne Storset
* midinastasurazz
* mitaa
* Ticki

# Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 837: Allow macros in types](https://github.com/rust-lang/rfcs/pull/873)
* [RFC 1191: Add an HIR to the compiler](https://github.com/rust-lang/rfcs/pull/1191)
* [RFC 1193: Prevent lint changes being breaking](https://github.com/rust-lang/rfcs/pull/1193)

# Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-07-20..2015-07-27

* [I/O error handling design](https://github.com/rust-lang/rfcs/pull/770)
* [Expand the std::net module](https://github.com/rust-lang/rfcs/pull/1158)
* [Allow changing the default allocator](https://github.com/rust-lang/rfcs/pull/1183)
* [Stabilize the #[no_std] attribute](https://github.com/rust-lang/rfcs/pull/1184)
* [Multiple import
  renaming](https://github.com/rust-lang/rfcs/pull/1219). e.g. `use
  foo::{self as bar, baz as quux}`.

# New RFCs

* [Clarify (and improve) rules for projections and well-formedness](https://github.com/rust-lang/rfcs/pull/1214)
* [Promote `!` to a type](https://github.com/rust-lang/rfcs/pull/1216)
* [Checked integer conversions](https://github.com/rust-lang/rfcs/pull/1218)
* [Multiple import renaming](https://github.com/rust-lang/rfcs/pull/1219)
* [Update the RFC process with sub-teams, amongst other things](https://github.com/rust-lang/rfcs/pull/1224)

# Internals discussions

# Subteam reports

Every week the [Rust teams](http://www.rust-lang.org/team.html)
release a report on what is going on in their corner of the
project. Here are the highlights from [this week's
report](https://internals.rust-lang.org/t/subteam-reports-2015-07-24/2397/2).

* The compiler is being refactored to work on an HIR and an MIR.
* Work is proceeding on stabilizing the core library.
* Basic allocators will soon be available.
* MSVC integration is proceeding rapidly.

# Friend of the Tree

The Rust Team likes to occassionally recognize people who have made
outstanding contributions to The Rust Project, its ecosystem, and its
community. These people are 'friends of the tree'.

[This week's friend of the tree](https://internals.rust-lang.org/t/subteam-reports-2015-07-24/2397) was @tshepang:

Over the last year Tshepang has landed over 100 improvements to our documentation. Tshepang saw where documentation was not, and said "No. This will not do."

We should all endeavor to care about docs as much as Tshepang.

# Upcoming Events

* [7/29. Cologne, Germany](http://www.meetup.com/Rust-Cologne-Bonn/events/222915034/)
* [8/1. RustCamp](http://www.rustcamp.com).
* [8/5. Montreal](http://www.meetup.com/Montreal-Rust-Language-Meetup/events/224148410/)
* [8/10. Seattle](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)
* [8/11. San Diego](http://www.meetup.com/San-Diego-Rust/events/223766853/)
* [8/17. Paris](http://www.meetup.com/Rust-Paris)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*"Quote"*

Explanation and link.

Thanks to XXX for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
