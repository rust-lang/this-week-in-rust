Title: This Week in Rust 162
Number: 162
Date: 2016-12-27
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

* [This Year in Gfx-rs - 2016](https://gfx-rs.github.io/2016/12/21/this-year.html).

## 24 Days of Rust

24 days of Rust is a series of articles introducing Rust language features, useful libraries, and cool projects built with Rust. Last week's articles are:

## Other Weeklies from Rust Community

# Crate of the Week

This week's Crate of the Week is [raster](https://github.com/kosinix/raster), an image processing library.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [servo: Stylo: implement -moz-orient](https://github.com/servo/servo/issues/14198).
* [easy] [servo: Allow passing --nocapture argument to test-unit mach command](https://github.com/servo/servo/issues/14595).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

109 pull requests were [merged in the last week][merged]. This contains a good number of plugin-breaking changes.

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-12-19..2016-12-26

* The 1.14 release brought some PRs to prepare and execute the rollout.
* [missing use statement leading to circular import path fixed](https://github.com/rust-lang/rust/pull/38539) (this crashed `rustc`)
* [`registry.register_custom_derive(..)`](https://github.com/rust-lang/rust/pull/38533) allows undeprecated legacy custom derives
* [signature info for `save-analysis`](https://github.com/rust-lang/rust/pull/38529)
* [disable field reordering](https://github.com/rust-lang/rust/pull/38523) (rolled back for now, will be phased in again gently
* [redox memalign](https://github.com/rust-lang/libc/pull/478)
* [`pub(restricted)` checking now uses `DefId`s instead of `NodeId`s](https://github.com/rust-lang/rust/pull/38490) (potentially plugin-breaking)
* [local closure variables debuginfo fixed](https://github.com/rust-lang/rust/pull/38483)
* [cross compilation to redox now possible](https://github.com/rust-lang/rust/pull/38401)
* [backwards incompatible import errors are now warnings](https://github.com/rust-lang/rust/pull/38271)
* [`where < ident >` no longer parses](https://github.com/rust-lang/rust/pull/38268) (future proofing)
* [unmangled spans for field/tup access nodes](https://github.com/rust-lang/rust/pull/38194)
* [`#[proc_macro_derive]` functions need to be `pub`lic](https://github.com/rust-lang/rust/pull/38140)
* [`impl From<[u16; 8]> for Ipv6Addr`](https://github.com/rust-lang/rust/pull/38131)
* [all `std` structs are now `fmt::Debug`gable](https://github.com/rust-lang/rust/pull/38006)
* [Abstract `std::slice::`{`binary_search`, `contains`}`(..)` over `Borrow`](https://github.com/rust-lang/rust/pull/37761)
* [The playpen now works with `error-format=json`](https://github.com/rust-lang/rust-playpen/pull/267)

## New Contributors

* Christophe Biocca
* Jeremy Fitzhardinge
* Jeremy Soller
* Jon Gjengset
* Kalita Alexey
* Michael Zapata

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1566: Procedural macros](https://github.com/rust-lang/rfcs/pull/1566).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Roadmap for 2017](https://github.com/rust-lang/rfcs/pull/1774).
* [`core::mem::replace_with` for temporarily moving out of ownership](https://github.com/rust-lang/rfcs/pull/1736).
* [Add a 'thread lifetime, which denotes a thread-bounded region](https://github.com/rust-lang/rfcs/pull/1705).
* [Allow `Self` to appear in the where clause of trait impls](https://github.com/rust-lang/rfcs/pull/1647).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).

## New RFCs

* [Stackless coroutines](https://github.com/rust-lang/rfcs/pull/1823). Add language-level support for stackless coroutines (also known as semicoroutines or generators).
* [Proposal for default crate recommendation ranking](https://github.com/rust-lang/rfcs/pull/1824).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [Conventions for Cargo.toml files (FCP)](https://github.com/rust-lang-nursery/fmt-rfcs/pull/41).

Ready for PR:

There's [a lot of them](https://github.com/rust-lang-nursery/fmt-rfcs/issues?q=is%3Aopen+is%3Aissue+label%3Aready-for-PR) right now, contributions here would be very welcome. If you want advice or help getting started, please ping nrc, or any other member of the style team, in #rust-style.

Issues in final comment period:

* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).

# Upcoming Events

* [12/21. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [12/21. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [12/28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [12/28. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [12/29. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

* [Mozilla Research Internship (US/INTL) - University 2017](https://careers.mozilla.org/position/gh/503816).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> fmtq: I'm ridiculously good at the borrow checker though
> fmtq: in Rust.
> bstrie: once you've mastered borrow checkers, you may move on to borrow chess

— in #rust-offtopic.

Thanks to [Havvy](https://users.rust-lang.org/users/havvy) for the [suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/334).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
