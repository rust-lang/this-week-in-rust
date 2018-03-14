Title: This Week in Rust 226
Number: 226
Date: 2018-03-20
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

This week's crate is [cursive](https://crates.io/crates/cursive), a library for easy text-user interface applications. Thanks to [Wangshan Lu](https://users.rust-lang.org/u/WiSaGaN) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [good first issue] [tera: Add loop controls](https://github.com/Keats/tera/issues/267). Tera is a template engine for Rust based on Jinja2/Django.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

124 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-03-05..2018-03-12

* [replace all const evaluation with miri](https://github.com/rust-lang/rust/pull/46882) (epic PR)
* [replace internal iterator structures with `impl Trait`](https://github.com/rust-lang/rust/pull/48699)
* [NLL: Make causal tracking lazy](https://github.com/rust-lang/rust/pull/48682)
* [turn feature-gate table into a query so it is covered by dependency tracking](https://github.com/rust-lang/rust/pull/48208)
* [Warn about ignored generic bounds in `for`](https://github.com/rust-lang/rust/pull/48326)
* [show used type variable when issuing a "can't use type parameters from outer function" error message](https://github.com/rust-lang/rust/pull/47574)
* [suggest type for overflowing bin/hex-literals](https://github.com/rust-lang/rust/pull/48432)
* [add functionality for gating feature flags on epochs; rejigger epoch lints](https://github.com/rust-lang/rust/pull/48801)
* [optimize `str::repeat`](https://github.com/rust-lang/rust/pull/48657)
* [add functions for reversing the bit pattern in an integer](https://github.com/rust-lang/rust/pull/48573)
* [implement `FromStr` for `PathBuf`](https://github.com/rust-lang/rust/pull/48292)
* [stabilize FusedIterator](https://github.com/rust-lang/rust/pull/47463)

## New Contributors

* 1011X
* Kurtis Nusbaum
* Maxim Nazarenko
* Peter Lyons
* Songbird0

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2341: Allow locals and destructuring in const fn](https://github.com/rust-lang/rfcs/pull/2341).
* [Update the disambiguation handling in RFC 1946 (intra-rustdoc-links) to match impl concerns](https://github.com/rust-lang/rfcs/pull/2285).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Standard library API for immovable types](https://github.com/rust-lang/rfcs/pull/2349).
* [disposition: merge] [Add Euclidean modulo & division functionality for integers](https://github.com/rust-lang/rfcs/pull/2169).
* [disposition: merge] [Constants in array repeat expressions](https://github.com/rust-lang/rfcs/pull/2203).
* [disposition: merge] [`Self` in type definitions allowing `enum List<T> { Nil, Cons(T, Box<Self>) }`](https://github.com/rust-lang/rfcs/pull/2300).
* [disposition: merge] [Add std::num::NonZeroU32 and friends, deprecate core::nonzero](https://github.com/rust-lang/rfcs/pull/2307).
* [disposition: merge] [Allow `if` and `match` in constants](https://github.com/rust-lang/rfcs/pull/2342).
* [disposition: close] [Make Cargo aware of standard library dependencies](https://github.com/rust-lang/rfcs/pull/1133).
* [disposition: close] [Quick `dbg!(expr)` macro](https://github.com/rust-lang/rfcs/pull/2173).

## New RFCs

* [Simpler alternative `dbg!()` macro](https://github.com/rust-lang/rfcs/pull/2361).
* [Finalize syntax for slice patterns with subslices](https://github.com/rust-lang/rfcs/pull/2359).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Mar 15. Hamburg, DE - Rust Hack & Learn with a Sprinkle of Web Assembly](https://www.meetup.com/Rust-Meetup-Hamburg/events/248310938/).
* [Mar 15. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/fmwshpyxfbtb/).
* [Mar 16. Frankfurt, DE - Rust Table of Regulars](https://www.meetup.com/Rust-Rhein-Main/events/248326240).
* [Mar 16. Pune, IN - Rust Hacks at Cummins](https://reps.mozilla.org/e/rust-at-cummins/).
* [Mar 17. Chennai, IN - Monthly Meetup - March](https://www.meetup.com/mad-rs/events/248475319/).
* [Mar 18. Bangalore, IN - Rust for newbies (Part 5 of 12)](https://www.meetup.com/rustox/events/247982987/).
* [Mar 18. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbxb/).
* [Mar 19. London, GB - LDN Talks: March 2018](https://www.meetup.com/Rust-London-User-Group/events/247681377/).
* [Mar 19. Karlsruhe, DE -`Hack and Meet](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/248201379/).
* [Mar 21. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247387953/).
* [Mar 21. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxfbcc/).
* [Mar 21. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Mar 22. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 25. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbhc/).
* [Mar 27. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Mar 27. Kitchener, CA - An Introduction To Rust & Writing a Crate (Kahan Sums)](https://www.meetup.com/Rust-KW/events/247661794/).
* [Mar 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Mar 28. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Librsvg and Gnome-class accepting interns](https://people.gnome.org/~federico/blog/interns-summer-2018.html).
* [Senior Computing Engineer at Marginal Unit](https://news.ycombinator.com/item?id=16493235).
* [Senior Data Engineer at Marginal Unit](https://news.ycombinator.com/item?id=16493216).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> _Captain's log, day 21_
>
> We have sailed on Reddit and Twitter for three weeks now, searching far and wide, yet the only thing we found was a barren landscape, with no end in sight. The supplies are shrinking, the men are growing impatient and hungry, and I fear we will have a mutiny soon. But I am stubborn and optimistic, and urge them to hold on and keep waiting until we find a quote of the week.

— [u/SelfDistinction on reddit](https://www.reddit.com/r/rust/comments/82nzc1/this_week_in_rust_224/dvbhaub/).

Thanks to [u/nasa42 for the suggestion](https://www.reddit.com/r/rust/comments/82nzc1/this_week_in_rust_224/dvbhyce/)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
