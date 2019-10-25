Title: This Week in Rust 310
Number: 310
Date: 2019-10-29
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

This week's crate is [grubbnet](https://github.com/dooskington/grubbnet), a TCP client/server library for networked applications and games.

Thanks to [Dooskington](https://users.rust-lang.org/t/crate-of-the-week/2704/650) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Clippy: Passing mutable references](https://github.com/rust-lang/rust-clippy/issues/353).
* [rust-bindgen: Add --symbol-prefix flag](https://github.com/rust-lang/rust-bindgen/issues/1375).
* [good first issue] [Spirit: Write a tutorial for Spirit](https://github.com/vorner/spirit/issues/42). Spirit is a helper to make creating and configuring unix daemons easier.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

353 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-10-14..2019-10-21

* [Stabilize proc macros generating `macro_rules` items](https://github.com/rust-lang/rust/pull/64035)
* [Return `false` from `needs_drop` for all zero-sized arrays](https://github.com/rust-lang/rust/pull/65389)
* [Optimize `LexicalResolve::expansion`](https://github.com/rust-lang/rust/pull/65260)
* [Remove custom `PartialEq` impls for `LocalInternedString`](https://github.com/rust-lang/rust/pull/65426)
* [Optimize `BitIter`](https://github.com/rust-lang/rust/pull/65425)
* [Optimize dropck](https://github.com/rust-lang/rust/pull/64595)
* [More symbol cleanups](https://github.com/rust-lang/rust/pull/65545)
* [Avoid unnecessary arena allocations in `expand_pattern()`](https://github.com/rust-lang/rust/pull/65463)
* [Avoid unnecessary `TokenTree` to `TokenStream` conversions](https://github.com/rust-lang/rust/pull/65455)
* [expand: Simplify expansion of derives](https://github.com/rust-lang/rust/pull/65252)
* [Fix suggestion to constrain trait for method to be found](https://github.com/rust-lang/rust/pull/65242)
* [syntax: add parser recovery for intersection- / and-patterns `p1 @ p2`](https://github.com/rust-lang/rust/pull/65410)
* [Reducing spurious unused lifetime warnings](https://github.com/rust-lang/rust/pull/64603)
* [Bring attention to suggestions when the only difference is capitalization](https://github.com/rust-lang/rust/pull/65398)
* [Use structured suggestion for restricting bounds](https://github.com/rust-lang/rust/pull/65192)
* [Fix zero-size uninitialized boxes](https://github.com/rust-lang/rust/pull/65174)
* [Add check for overlapping ranges to unreachable patterns lint](https://github.com/rust-lang/rust/pull/64007)
* [Use more fine grained locks for the dep graph](https://github.com/rust-lang/rust/pull/63756)
* [Fix `canonicalize_const_var` leaking inference variables](https://github.com/rust-lang/rust/pull/65652)
* [mir-opt: Improve SimplifyLocals pass so it can remove unused consts](https://github.com/rust-lang/rust/pull/65624)
* [Improve error message for APIT with explicit generic arguments](https://github.com/rust-lang/rust/pull/65614)
* [Remove unreachable unit tuple compare binop codegen](https://github.com/rust-lang/rust/pull/65605)
* [Avoid ICE when `include!` is used by stdin crate](https://github.com/rust-lang/rust/pull/65603)
* [Implement `AsRef<[T]>` for `List<T>`](https://github.com/rust-lang/rust/pull/65444)
* [hashbrown: Remove most `#[inline]` annotations](https://github.com/rust-lang/hashbrown/pull/119)
* [Always inline `mem::`{`size_of`, `align_of`} in debug builds](https://github.com/rust-lang/rust/pull/65016)
* [Avoid realloc in `CString::new`](https://github.com/rust-lang/rust/pull/65551)
* [`BTreeSet` symmetric_difference & union optimized](https://github.com/rust-lang/rust/pull/65226)
* [cargo: Allow `--all-features` in root of virtual workspace](https://github.com/rust-lang/cargo/pull/7525)
* [rustup install: add `--profile` flag to override profile](https://github.com/rust-lang/rustup.rs/pull/2075)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for todo! macro](https://github.com/rust-lang/rust/issues/59277).
* [disposition: merge] [Tracking issue for floats ↔ bytes conversions](https://github.com/rust-lang/rust/issues/60446).
* [disposition: merge] [Stabilize RFC 2451, re-rebalance coherence](https://github.com/rust-lang/rust/issues/63599).
* [disposition: merge] [[rustdoc] stabilize cfg(doctest)](https://github.com/rust-lang/rust/pull/63803).
* [disposition: merge] [Stabilize nested self receivers in 1.40.0](https://github.com/rust-lang/rust/pull/64325).
* [disposition: merge] [Stabilize `Option::flatten`](https://github.com/rust-lang/rust/pull/64747).
* [disposition: merge] [Stabilize `const_constructor`](https://github.com/rust-lang/rust/pull/65188).

## New RFCs

* [Standard lazy types](https://github.com/rust-lang/rfcs/pull/2788).
* [Serve crates-io registry over HTTP as static files](https://github.com/rust-lang/rfcs/pull/2789).

# Upcoming Events

### Africa

* [Nov  6. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzpbjb/).

### Asia Pacific

* [Oct 26. Tokyo, JP - Rust.Tokyo 2019](https://rust.tokyo/).
* [Oct 26. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/495062051340992/).
* [Oct 29. Sydney, AU - Rust Sydney - Meetup 18](https://www.meetup.com/Rust-Sydney/events/265708002/).
* [Nov  4. Auckland, NZ - Rust AKL - Introduction to Rust (part 2 of 3)](https://www.meetup.com/rust-akl/events/259481269/).

### Europe

* [Oct 28. Zurich, CH - Rust Zurich - October Meetup: Claudia Saxer – 66 hours of Rust](https://www.meetup.com/Rust-Zurich/events/265507413/).
* [Oct 28. Gouda, NL - Rust Nederland - Rust - Talks & Demos](https://www.meetup.com/Rust-Nederland/events/265656966).
* [Oct 28. London, GB - Rust London User Group - LDN Talks October 2019](https://www.meetup.com/Rust-London-User-Group/events/265590044/).
* [Oct 30. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbnc/).
* [Oct 30. Copenhagen, DK - Copenhagen Rust Hack Night #19](https://cph.rs/).
* **[Nov 9 & 10. Barcelona, ES - RustFest Barcelona 2019](https://barcelona.rustfest.eu/).**
* [Nov 15. Barcelona, ES - Rust GTK/GStreamer Workshop at Linux Application Summit 2019](https://www.meetup.com/Barcelona-Free-Software/events/265596417/).

### North America

* [Oct 28. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyznblc/).
* [Oct 29. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyznbmc/).
* [Oct 29. Chicago, IL, US - Chicago Rust Meetup - Entity Component Systems: An Intro To The Specs Crate Using Roguelikes](https://www.meetup.com/Chicago-Rust-Meetup/events/265283294).
* [Oct 30. San Francisco, CA, US - Rust in Blockchain Workshop Day (SFBW)](https://www.meetup.com/Rust-in-Blockchain-San-Francisco/events/265362152/)
* [Oct 30. Santa Clara, CA, US - Rust Bay Area - [@ Intel Santa Clara] Security with Rust & SGX + Life of an Async fn](https://www.meetup.com/Rust-Bay-Area/events/265478102).
* [Oct 30. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbnc/).
* [Nov  6. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzpbjb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Provable, London, GB (Remote available)](https://www.reddit.com/r/rust/comments/d9l79d/official_rrust_whos_hiring_thread_for_jobseekers/f4r63ms/).
* [Rust Senior Developer at Token, Palo Alto, CA, US](https://drive.google.com/file/d/1Rnc8HQLfiy4mvzZP--1ww1vTAX5FCagD/view).
* [Software Manager at Georg Fischer Signet, El Monte, CA, US](https://www.indeed.com/m/viewjob?jk=e82dad5c02d490a2).
* [Associate Software Engineer at Georg Fischer Signet, El Monte, CA, US](https://www.indeed.com/m/viewjob?jk=6d5ae77b64b16f72).
* [Rust/Core Developer at Parity, Berlin, DE (Remote available)](https://www.parity.io/jobs/#berlin-rust-core-developer).
* [Rust Intern at RUDDER, Paris, FR](https://taleez.com/apply/74t9em)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust helped me grasp concepts I should have known when writing C++

– [Alexander Clarke on the Microsoft Security Response Center blog]()

Thanks to [mmmmib](https://users.rust-lang.org/t/twir-quote-of-the-week/328/712) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
