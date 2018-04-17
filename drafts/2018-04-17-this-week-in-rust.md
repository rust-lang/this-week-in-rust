Title: This Week in Rust 230
Number: 230
Date: 2018-04-17
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

* [rayascott](https://users.rust-lang.org/u/rayascott) created a [list of fearless rust bloggers](https://users.rust-lang.org/t/fearless-rust-bloggers/16770) from the this-week-in-rust history.
* [Armstrong Publications](armstrong-publications.com) released a [brand new Rust book](https://www.armstrong-publications.com/product/step-ahead-with-rust/) for intermediate to advanced programmers

### From Rust All Hands in Berlin

* [The Rust team All Hands in Berlin: a recap](https://blog.rust-lang.org/2018/04/06/all-hands.html).
* [Sound and ergonomic specialization for Rust](https://aturon.github.io/2018/04/05/sound-specialization/).
* [Cargo, Xargo, and Rustup](https://aturon.github.io/2018/04/06/rustup-xargo/).
* [Custom tasks in Cargo](https://aturon.github.io/2018/04/05/workflows/).

# Crate of the Week

This week's crate is [rain](https://github.com/substantic/rain), a framework for large-scale distributed computations. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help stabilize a subset of Macros 2.0](https://internals.rust-lang.org/t/help-stabilize-a-subset-of-macros-2-0/7252)!
* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is a project to write a short guide about how the rust compiler works, and it needs your help. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [good first issue] [distinst: Optimize Partition Moving Algorithm](https://github.com/pop-os/distinst/issues/51). distinst is a distribution installer backend written in Rust.
* [distinst: Reduce LUKS Device Detection Overhead](https://github.com/pop-os/distinst/issues/80).
* [distinst: Use Entire Disk as LUKS / LVM Partition](https://github.com/pop-os/distinst/issues/64).
* [easy] [tokei: Improve tokei's language test coverage](https://github.com/Aaronepower/tokei/issues/63).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

143 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-04-09..2018-04-16

* [implement Euclidean modulo](https://github.com/rust-lang/rust/pull/49389) (RFC #[2169](https://rust-lang.github.io/rfcs/2169-euclidean-modulo.html))
* [implement `#[repr(packed(n))]`](https://github.com/rust-lang/rust/pull/48528) (RFC #[1399](https://rust-lang.github.io/rfcs/1399-repr-pack.html))
* [fix unsoundness bug in functions input references](https://github.com/rust-lang/rust/pull/49885)
* [skip MIR encoding for cargo check](https://github.com/rust-lang/rust/pull/49433)
* [proc_macro: avoid cached `TokenStream` more often](https://github.com/rust-lang/rust/pull/49852)
* [proc_macro: Generalize `FromIterator` impl](https://github.com/rust-lang/rust/pull/49734)
* [hygiene 2.0: avoid comparing fields by name](https://github.com/rust-lang/rust/pull/49718)
* [fix derive(PartialOrd) and optimise final field operation](https://github.com/rust-lang/rust/pull/49881)
* [update `?` repetition disambiguation](https://github.com/rust-lang/rust/pull/49719)
* [suggest `!` for erroneous identifier `not`](https://github.com/rust-lang/rust/pull/49258)
* [fix incorrect span in `&mut` suggestion](https://github.com/rust-lang/rust/pull/49931)
* [don't recurse into allocations, use a global table instead](https://github.com/rust-lang/rust/pull/49833)
* [Fix ICE by disallowing `impl Trait` in unsupported position](https://github.com/rust-lang/rust/pull/49830)
* [chalkify: Implement lowering rule Implied-Bound-From-Trait](https://github.com/rust-lang/rust/pull/49435)
* [don't abort const eval due to long running evals, just warn](https://github.com/rust-lang/rust/pull/49947)
* [add `GlobalAlloc` trait + tweaks for initial stabilization](https://github.com/rust-lang/rust/pull/49669)
* [add `to_bytes` and `from_bytes` to primitive integers](https://github.com/rust-lang/rust/pull/49871)
* [correctly print fractional part of a second](https://github.com/rust-lang/cargo/pull/5357)
* [stabilize `Option::filter`](https://github.com/rust-lang/rust/pull/49575)
* [stabilize `fetch_nand`](https://github.com/rust-lang/rust/pull/49963)
* [stabilize `take_set_limit`](https://github.com/rust-lang/rust/pull/49681)
* [move `Range`*`::contains` to a single default impl on `RangeBounds`](https://github.com/rust-lang/rust/pull/49130)
* [replace manual iterator exhaust with `for_each(drop)`](https://github.com/rust-lang/rust/pull/48945)
* [merge the `std_unicode` crate into the `core` crate](https://github.com/rust-lang/rust/pull/49698)
* [core: Remove panics from some `Layout` methods](https://github.com/rust-lang/rust/pull/49884)
* [cargo: include package metadata in `cargo metadata`](https://github.com/rust-lang/cargo/pull/5360)
* [cargo: rustc cache](https://github.com/rust-lang/cargo/pull/5359)
* [rustdoc: add target features when extracting and running doctests](https://github.com/rust-lang/rust/pull/49864)
* [rustdoc: port the -C option from rustc](https://github.com/rust-lang/rust/pull/49956)

## New Contributors

* Chris Manchester
* Dan Callaghan
* Francis Gagné
* lloydmeta
* nabijaczleweli
* Valentine Valyaeff
* Wim Looman

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2338: Type alias enum variants](https://github.com/rust-lang/rfcs/pull/2338).
* [RFC 2102: Unnamed fields of struct and union type](https://github.com/rust-lang/rfcs/pull/2102).
* [RFC 2196: Semantic build scripts for Cargo](https://github.com/rust-lang/rfcs/pull/2196).
* [RFC 2295: Extend pattern API to OsStr](https://github.com/rust-lang/rfcs/pull/2295).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

*No RFCs are currently in final comment period.*

## New RFCs

* [Add futures to libcore](https://github.com/rust-lang/rfcs/pull/2395).
* [Async/await notation for ergonomic asynchronous IO](https://github.com/rust-lang/rfcs/pull/2394).
* [Reserve `try` for `try { .. }` block expressions](https://github.com/rust-lang/rfcs/pull/2388).
* [Delegation](https://github.com/rust-lang/rfcs/pull/2393). Syntax sugar for efficient code reuse via the composition pattern.
* [`#[used]` static variables](https://github.com/rust-lang/rfcs/pull/2386).
* [Introduce `#[do_not_recommend]` to control errors for trait impls](https://github.com/rust-lang/rfcs/pull/2397).
* [Implement a sandbox for environment variables and files](https://github.com/rust-lang/rfcs/pull/2391).
* [target_feature 1.1](https://github.com/rust-lang/rfcs/pull/2396).
* [Macros Derive PlopAhead and PlopBehind](https://github.com/rust-lang/rfcs/pull/2390).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Apr 12. San Diego, US - San Diego Rust April Meetup - UDP Challenge](https://www.meetup.com/San-Diego-Rust/events/249505098/).
* [Apr 12. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxgbqb/).
* [Apr 12. Arlington, US - Rust DC - Learn+Try: Rust in the Browser via WebAssembly](https://www.meetup.com/RustDC/events/248552247/).
* [Apr 15. Athens, GR - Rust Meetup](https://www.hackerspace.gr/wiki/5th_Rust_Meetup).
* [Apr 15. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbtb/).
* [Apr 17. Brussels, BE - Error, logs and more](https://www.meetup.com/Belgium-Rust-user-group/events/248297132/).
* [Apr 18. Paris, FR - Devoxx 2018 - Hands-on Labs - Rust introduction](http://cfp.devoxx.fr/2018/talk/QAL-4376/Atelier_Rust).
* [Apr 18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr 18. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247388143/).
* [Apr 18. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxgbxb/).
* [Apr 18. Wrocław, PL - Technocracy. 1st Rust Meetup in Wrocław](https://www.meetup.com/Technocracy/events/249259107/).
* [Apr 18. Milano, IT - Rust Exercises](https://www.meetup.com/rust-language-milano/events/249592365/).
* [Apr 19. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxgbzb/).
* [Apr 21. Chennai, IN - Monthly Meetup - April](https://www.meetup.com/mad-rs/events/249535481/).
* [Apr 22. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbdc/).
* [Apr 24. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Apr 24. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxgbgc/).
* [Apr 25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr 25. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Apr 26. NYC, US - Rust NYC (Security)](https://www.meetup.com/Rust-NYC/events/249849155/).
* [Apr 27. Darmstadt, DE - Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/249543182/).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Junior Blockchain Rust Developer at Witnet Foundation](https://angel.co/witnet-foundation-1/jobs/342268-junior-blockchain-rust-developer).
* [Senior Blockchain Rust Developer at Witnet Foundation](https://angel.co/witnet-foundation-1/jobs/342272-senior-blockchain-rust-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
