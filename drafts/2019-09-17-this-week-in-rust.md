Title: This Week in Rust 304
Number: 304
Date: 2019-09-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

* [Announcing awesome-rust-mentors](https://rustbeginners.github.io/awesome-rust-mentors/).

## News & Blog Posts

- [Adventures in Motion Control](http://adventures.michaelfbryan.com/posts/announcing-adventures-in-motion-control/)
* [Oreboot: Coreboot minus C, a talk by Google](https://osfc.io/uploads/talk/paper/23/Oreboot.pdf).

# Crate of the Week

This week's crate is [texture-synthesis](https://github.com/EmbarkStudios/texture-synthesis), a program to generate textures by choosing examples.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/621) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for help implementing an independent Rust frontend for GCC](https://users.rust-lang.org/t/call-for-help-implementing-an-independent-rust-frontend-for-gcc/32163).
* [Rusoto: Looking for maintainers](https://github.com/rusoto/rusoto/issues/1496).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

282 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-09-09..2019-09-16

* [rustc: Allow the cdylib crate type with wasm32-wasi](https://github.com/rust-lang/rust/pull/64188)
* [Hide diagnostics emitted during --cfg parsing](https://github.com/rust-lang/rust/pull/64467)
* [Improve hygiene of `alloc::format!`](https://github.com/rust-lang/rust/pull/64060)
* [Cleanup handling of hygiene for built-in macros](https://github.com/rust-lang/rust/pull/64469)
* [resolve: Tweak some "cannot find" wording for macros](https://github.com/rust-lang/rust/pull/64483)
* [Provide a span if main function is not present in crate](https://github.com/rust-lang/rust/pull/64290)
* [def_collector: Do not ICE on attributes on unnamed fields](https://github.com/rust-lang/rust/pull/64457)
* [Permit impls referencing errors to overlap](https://github.com/rust-lang/rust/pull/64474)
* [lowering: Extend temporary lifetimes around await](https://github.com/rust-lang/rust/pull/64292)
* [Shrink `ObligationCauseCode`](https://github.com/rust-lang/rust/pull/64302)
* [check_match: Refactor + improve non-exhaustive diagnostics for default binding modes](https://github.com/rust-lang/rust/pull/64271)
* [fn ptr is structural match](https://github.com/rust-lang/rust/pull/64431)
* [rustc_mir: Buffer -Zdump-mir output instead of pestering the kernel constantly](https://github.com/rust-lang/rust/pull/64344)
* [std: Add a `backtrace` module](https://github.com/rust-lang/rust/pull/64154)
* [Stabilize `Vec::new` and `String::new` as `const fn`s](https://github.com/rust-lang/rust/pull/64028)
* [Stabilise weak_ptr_eq](https://github.com/rust-lang/rust/pull/61797)
* [Make `abs`, `wrapping_abs`, `overflowing_abs` const functions](https://github.com/rust-lang/rust/pull/63786)
* [Use `try_fold` instead of manually carrying an accumulator](https://github.com/rust-lang/rust/pull/64473)
* [Improve `BTreeSet::Intersection::size_hint`](https://github.com/rust-lang/rust/pull/64383)
* [cargo: Don't build libstd as a `dylib`](https://github.com/rust-lang/cargo/pull/7353)

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

* [disposition: merge] [RFC for an operator to take a raw reference](https://github.com/rust-lang/rfcs/pull/2582).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `Vec::new` and `String::new` as `const fn`s](https://github.com/rust-lang/rust/pull/64028).
* [disposition: merge] [Stabilize `param_attrs` in Rust 1.39.0](https://github.com/rust-lang/rust/pull/64010).
* [disposition: merge] [Stabilise weak_ptr_eq](https://github.com/rust-lang/rust/pull/61797).

## New RFCs

* [Add methods for converting from `bool` to `Option<T>`](https://github.com/rust-lang/rfcs/pull/2757).

# Upcoming Events

### Asia Pacific

* [Sep 30. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/2110177005945081/).

### Europe

* [Sep 17. Wrocław, PL - Rust Wrocław Meetup #12](https://www.meetup.com/Rust-Wroclaw/events/264586907/)
* [Sep 18. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzmbxb/).
* [Sep 25. Milano, IT - Rust Language Milano - Virtual Filesystem with Rust](https://www.meetup.com/rust-language-milano/events/264311325).

### North America

* [Sep 18. Portland, OR, US - PDXRust - Hack Night (not the usual meetup!)](https://www.meetup.com/PDXRust/events/264332355/).
* [Sep 18. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzmbxb/).
* [Sep 20-21. Denver, CO, US - Colorado Gold Rust](https://www.cogoldrust.com/).
* [Sep 23. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzmbfc/).
* [Sep 24. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzmbgc/).
* [Sep 25. Chicago, IL, US - Chicago Rust Meetup - Wait, why does Rust have 4 string types](https://www.meetup.com/Chicago-Rust-Meetup/events/264559606).
* [Sep 25. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzmbhc/).
* [Sep 25. Mesa, AZ, US - Desert Rust - Rust: Web assembly](https://www.meetup.com/Desert-Rustaceans/events/wmmphryzmbhc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer at Laika, Hillsboro, OR, US](https://www.laika.com/careers/job-listing?jobid=1847970).
* [Senior Distributed Systems Engineer at Interchain Foundation, Toronto, CA](https://www.linkedin.com/jobs/cap/view/1464883134/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Well, let me tell you: unless your code is cooler than ICE, the compiler does not miss anything. Rust accompanies us at each step of our path, very gently pulling our hand when we are too close to falling onto a (safety) hole, and also very gently letting us fall all the way down the hole, as soon we spell the forbidden incantation: `unsafe`.

– [Daniel H-M on rust-users](https://users.rust-lang.org/t/looking-for-a-deeper-understanding-of-phatomdata/32477/4)

Thanks to [Cerberuser](https://users.rust-lang.org/t/twir-quote-of-the-week/328/700) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
