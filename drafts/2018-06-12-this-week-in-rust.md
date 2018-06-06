Title: This Week in Rust 238
Number: 238
Date: 2018-06-12
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

This week's crate, as decreed by llogiq, is [im](https://docs.rs/im), a library for immutable data structures in Rust.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [wasm-pack has several open good first issues available to new contributors](https://github.com/ashleygwilliams/wasm-pack/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

149 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-05-21..2018-05-28

* [fix building rustc on and for musl hosts](https://github.com/rust-lang/rust/pull/51063)
* [add polonius compare mode](https://github.com/rust-lang/rust/pull/51138)
* [make borrowck use polonius output](https://github.com/rust-lang/rust/pull/51133)
* [register outlives predicates from queries the right way around](https://github.com/rust-lang/rust/pull/51096)
* [resolve: make sure indeterminate and inconsistent macro resolutions always generate errors](https://github.com/rust-lang/rust/pull/51145)
* [typeck: do not pass the field check on field error](https://github.com/rust-lang/rust/pull/51146)
* [make GlobalCtxt thread-safe](https://github.com/rust-lang/rust/pull/50108)
* [stabilize short error format](https://github.com/rust-lang/rust/pull/49546)
* [suggest using `as_ref` on some borrow errors](https://github.com/rust-lang/rust/pull/51100)
* [merge unused-extern-crate and unnecessary-extern-crate lints](https://github.com/rust-lang/rust/pull/51015)
* [make anon params lint warn-by-default](https://github.com/rust-lang/rust/pull/48309)
* [do not promote union field accesses](https://github.com/rust-lang/rust/pull/51328)
* [make const decoding thread-safe](https://github.com/rust-lang/rust/pull/51060)
* [`impl Default for &mut str`](https://github.com/rust-lang/rust/pull/51306)
* [const fn integer operations](https://github.com/rust-lang/rust/pull/51299)
* [every match arm reads the match's borrowed input](https://github.com/rust-lang/rust/pull/50783)
* [also check `let` arms and nested patterns for mutable borrows](https://github.com/rust-lang/rust/pull/51274)
* [implement `#[panic_implementation]`](https://github.com/rust-lang/rust/pull/50338)
* [OOM handling changes](https://github.com/rust-lang/rust/pull/50880)
* [make the OOM hook return `()` rather than `!`](https://github.com/rust-lang/rust/pull/51264)
* [`std::fs::DirEntry.metadata`: use fstatat instead of lstat when possible](https://github.com/rust-lang/rust/pull/51050)
* [add missing Wrapping methods, use doc_comment!](https://github.com/rust-lang/rust/pull/50465)
* [optimize joining for slices](https://github.com/rust-lang/rust/pull/50340)
* [hash up to 8 bytes at once with `FxHasher`](https://github.com/rust-lang/rust/pull/51019)
* [two minor parsing tweaks](https://github.com/rust-lang/rust/pull/51240)
* [make `Layout`'s align a `NonZeroUsize`](https://github.com/rust-lang/rust/pull/51226)
* [make some std::intrinsics `const fn`s](https://github.com/rust-lang/rust/pull/51171)
* [simplify `HashMap` layout calculation by using `Layout`](https://github.com/rust-lang/rust/pull/51163)
* [optimize layout calculations in `HashMap`](https://github.com/rust-lang/rust/pull/51340)
* [fs: copy: use copy_file_range on Linux](https://github.com/rust-lang/rust/pull/50772)
* [add `From<bool>` for int types](https://github.com/rust-lang/rust/pull/50554)
* [add `as_nanos` function to `Duration`](https://github.com/rust-lang/rust/pull/50167)
* [`Arc` downcast](https://github.com/rust-lang/rust/pull/50836)
* [stabilize SliceIndex trait](https://github.com/rust-lang/rust/pull/51147)
* [stabilize SystemTime::UNIX_EPOCH](https://github.com/rust-lang/rust/pull/51144)
* [cargo: verify that src dir wasn't modified by `build.rs` when publishing](https://github.com/rust-lang/cargo/pull/5584)
* [cargo: fix a deadlock issue](https://github.com/rust-lang/cargo/pull/5570)
* [Rust Logo on a diet](https://github.com/rust-lang/rust-www/pull/915)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amend 1105: Unstable features accidentally usable on the Stable release chanel are still unstable](https://github.com/rust-lang/rfcs/pull/2405).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for the GlobalAlloc trait and related APIs](https://github.com/rust-lang/rust/issues/49668).
* [disposition: merge] [Stabilize GlobalAlloc and `#[global_allocator]`](https://github.com/rust-lang/rust/pull/51241).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [Tracking issue for "macro naming and modularisation" (RFC #1561)](https://github.com/rust-lang/rust/issues/35896).
* [disposition: merge] [Tracking issue for promoting `!` to a type (RFC 1216)](https://github.com/rust-lang/rust/issues/35121).
* [disposition: merge] [Exhaustive integer matching](https://github.com/rust-lang/rust/pull/50912).
* [disposition: merge] [Stabilize `std::path::Path::ancestors`](https://github.com/rust-lang/rust/pull/50894).
* [disposition: merge] [Add ability to apply custom derive to union types](https://github.com/rust-lang/rust/pull/50383).

## New RFCs

* [Re-Rebalancing Coherence](https://github.com/rust-lang/rfcs/pull/2451).
* [Allow non-ASCII identifiers](https://github.com/rust-lang/rfcs/pull/2457).
* [Unused const fn results](https://github.com/rust-lang/rfcs/pull/2450).

# Upcoming Events

* [Jun 10. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbnb/).
* [Jun 11. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxjbpb/).
* [Jun 12. Rome, IT - Rust learning and hacking evening #9](https://www.meetup.com/Rust-Roma/events/251298815/).
* [Jun 13. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/250904450/).
* [Jun 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun 13. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbrb/).
* [Jun 14. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxjbsb/).
* [Jun 14. San Diego, US - San Diego Rust June Meetup](https://www.meetup.com/San-Diego-Rust/events/251001684/).
* [Jun 14. Utah Valley, US - Utah Rust meetup](https://docs.google.com/document/d/1O8S7IEfDw-3jTN74CWCuKYl_UWxTLd6-epz7NOMDYRg).
* [Jun 17. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbwb/).
* [Jun 19. Denver, US - Rust Boulder/Denver - June Meetup in Boulder](https://www.meetup.com/Rust-Boulder-Denver/events/250076478/).
* [Jun 19. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jun 20. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jun 20. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun 21. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxjbcc/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).** Registration is now open.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Backend Engineer at ClanHQ, San Francisco (remote available)](https://twitter.com/JonathanBelolo/status/1000128978936623104).
* [Rust Developer at 1am, Berlin](https://twitter.com/__1aim/status/1002493500099833856).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> When picking up a lentil (Result) a pigeon (?) must consider two options. If the lentil is a good one (Ok), the pigeon simply puts it into the pot (evaluates to the wrapped value). However, if the lentil happens to be a bad one (Err), the pigeon eats it, digests it (from) and finally “returns” it. Also the silhouette of a pigeon kind of resembles a questionmark.

– [anatol1234](https://users.rust-lang.org/u/anatol1234) on [internals](https://internals.rust-lang.org/t/bikeshed-a-consise-verb-for-the-operator/7289/77)

Thanks to [Christopher Durham](https://users.rust-lang.org/u/cad97) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
