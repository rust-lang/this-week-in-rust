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

* [Redox Summer of Code](https://www.redox-os.org/rsoc/).
* [Writing An NES Emulator with Rust and WebAssembly](https://medium.com/@bokuweb17/writing-an-nes-emulator-with-rust-and-webassembly-d64de101c49d).
* [Down a rusty rabbit hole](https://manishearth.github.io/blog/2018/04/12/down-a-rusty-rabbit-hole/). Discovering a series of quirks of the Rust compiler/language.
* [The case for deadlines in timeout-centric APIs](https://gist.github.com/alkis/9510a840f1965185ab0a02cb59761dd8).
* [Rust pattern: Rooting an Rc handle](http://smallcultfollowing.com/babysteps/blog/2018/04/16/rust-pattern-rooting-an-rc-handle/).
* [Conway's Game of Life: A tutorial on implementing a game in Rust and WebAssembly](https://rust-lang-nursery.github.io/rust-wasm/game-of-life/introduction.html).
* [Safe intrusive collections with pinning](https://www.ralfj.de/blog/2018/04/10/safe-intrusive-collections-with-pinning.html).
* [Rust memory safety revolution: Why, what and how for complete beginners](https://anixe.pl/content/news/rust_memory_safety_revolution).
* [A useful feature few Rust programmers know about](http://rickyhan.com/jekyll/update/2018/04/16/the-best-kept-secret-rust-feature.html). The `{:#?}` pretty-printer flag.
* [Instance identity in C++ and Rust](https://jrvanwhy.github.io/instance-identity/).
* [A shifty riddle: Why is `std::ops::Shl::shl` not equal to `<<`](https://llogiq.github.io/2018/04/11/shift.html)?
* [Fearless Rust bloggers](https://users.rust-lang.org/t/fearless-rust-bloggers/16770). A list of blogs from past This Week in Rust issues.
* [This week in Rust docs 101](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-101).
* [The Embedded Working Group newsletter 3](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-3/7319).
* [CLI Working Group newsletter #2](https://internals.rust-lang.org/t/cli-wg-newsletter-2/7309).
* [IntelliJ Rust changelog 72](https://intellij-rust.github.io/2018/04/16/changelog-72.html). Now with support for macro expansion.
* [podcast] [Rusty Spike Podcast - episode 25](https://rusty-spike.blubrry.net/2018/04/12/episode-25-apr-11-2018/). Rust Reach, the Rust all hands, webassembly.studio, more wasm tools, async/await, and GStreamer.

### From Rust All Hands in Berlin

* [Rust all-hands (dev-tools stuff)](https://www.ncameron.org/blog/rust-all-hands-dev-tools-stuff/).

# Crate of the Week

This week's crate is [rain](https://github.com/substantic/rain), a framework for large-scale distributed computations. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Clippy](https://github.com/rust-lang-nursery/rust-clippy) has a lot of [good first issues](https://github.com/rust-lang-nursery/rust-clippy/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22). If you are looking for something specific to get started with, here is one: [Split up our UI-tests into smaller parts](https://github.com/rust-lang-nursery/rust-clippy/issues/2038).
* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is a project to write a short guide about how the rust compiler works, and it needs your help. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [Help stabilize a subset of Macros 2.0](https://internals.rust-lang.org/t/help-stabilize-a-subset-of-macros-2-0/7252)!
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

* Alec Mocatta
* Chris Coulson
* Fabio B
* Hero
* Joshua Barretto
* Nikita Popov
* Steven Malis

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Unapprove placement RFCs](https://github.com/rust-lang/rfcs/pull/2387): [1228: Place left arrow syntax (`place <- expr`)](https://github.com/rust-lang/rfcs/blob/master/text/1228-placement-left-arrow.md) and [RFC 809: `box` and placement `in`](https://github.com/rust-lang/rfcs/blob/master/text/0809-box-and-in-for-stdlib.md).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Cargo publish with internal path dependencies](https://github.com/rust-lang/rfcs/pull/2224).

## New RFCs

* [Zero page optimization](https://github.com/rust-lang/rfcs/pull/2400).
* [`mut (x, y, ..)` and `mut [x, y, ..]` pattern shorthand](https://github.com/rust-lang/rfcs/pull/2401).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Apr 19. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxgbzb/).
* [Apr 21. Chennai, IN - Monthly Meetup - April](https://www.meetup.com/mad-rs/events/249535481/).
* [Apr 22. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbdc/).
* [Apr 24. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Apr 24. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxgbgc/).
* [Apr 25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr 25. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Apr 26. New York City, US - Rust NYC (Security)](https://www.meetup.com/Rust-NYC/events/249849155/).
* [Apr 27. Darmstadt, DE - Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/249543182/).
* [Apr 29. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbmc/).
* [May  1. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxhbcb/).
* [May  2. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/249134945/).
* [May  2. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May  2. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxhbdb/).
* [May  2. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxhbdb/).
* [May  2. Indianapolis, US - Indy.rs - Actix Actor Framework](https://www.meetup.com/indyrs/events/cpvshpyxhbdb/).
* [May  3. Utrecht, NL - Rust Workshop](https://www.meetup.com/Rust-Utrecht/events/248995086/).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Real Time Embedded Software Engineer at Callen-Lenz, UK](https://callenlenz.com/contact/careers/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is one of those friends that take some time to get along with, but that you'll finally want to engage with for a long term relationship.

— [Sylvain Wallez](https://bluxte.net/musings/2018/04/10/go-good-bad-ugly/).

Thanks to [u/rushmorem](https://www.reddit.com/r/rust/comments/8bjio2/xpost_from_rprogramming_go_the_good_the_bad_and/dx7u0lu/) and [saethlin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/514) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
