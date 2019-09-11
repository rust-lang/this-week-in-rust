Title: This Week in Rust 303
Number: 303
Date: 2019-09-10
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

* [How Rust optimizes async/await II: Program analysis](https://tmandry.gitlab.io/blog/posts/optimizing-await-2/).
* [Rust on the ESP32](https://mabez.dev/blog/posts/esp32-rust/).
* [Linux.Fe2O3: a Rust virus](https://www.guitmz.com/linux-fe2o3-rust-virus/).
* [Improvement to the compile time of a crate](http://antoyo.ml/compilation-time-dependencies).
* [hyper 0.13 alpha supports async/await](https://seanmonstar.com/post/187493499882/hyper-alpha-supports-asyncawait).
* [Writing an OS in Rust: Updates in August 2019](https://os.phil-opp.com/status-update/2019-09-09/).
* [Rust in large organizations - meeting notes](https://users.rust-lang.org/t/rust-in-large-organizations-meeting/32059).
* [Futures concurrency](https://blog.yoshuawuyts.com/futures-concurrency/).

# Crate of the Week

This week's crate is [viu](https://github.com/atanunq/viu), a terminal image viewer.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/617) for the suggestion!

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

303 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-09-02..2019-09-09

* [Support both static and dynamic linking mode in testing for vxWorks](https://github.com/rust-lang/rust/pull/63789)
* [Point at variant on pattern field count mismatch](https://github.com/rust-lang/rust/pull/64161)
* [Use hygiene for AST passes](https://github.com/rust-lang/rust/pull/63919)
* [Account for doc comments coming from proc macros without spans](https://github.com/rust-lang/rust/pull/63930)
* [Reduce span to function name in unreachable calls](https://github.com/rust-lang/rust/pull/64229)
* [Move path parsing earlier](https://github.com/rust-lang/rust/pull/64120)
* [or-patterns: Uniformly use `PatKind::Or` in AST & Fix/Cleanup resolve](https://github.com/rust-lang/rust/pull/64111)
* [Allow checking of run-pass execution output in compiletest](https://github.com/rust-lang/rust/pull/63825)
* [Rust 2018: NLL migrate mode => hard error](https://github.com/rust-lang/rust/pull/63565)
* [Extend Polonius fact generation for (some) move tracking](https://github.com/rust-lang/rust/pull/62800)
* [polonius: Finalise initialisation calculations](https://github.com/rust-lang/polonius/pull/110)
* [libc: Remove WASI Core API](https://github.com/rust-lang/libc/pull/1461)
* [Use wasi crate for Core API](https://github.com/rust-lang/rust/pull/63676)
* [Use unicode-xid crate instead of libcore](https://github.com/rust-lang/rust/pull/62848)
* [Add `Result::cloned`{,`_err`} and `Result::copied`{,`_err`}](https://github.com/rust-lang/rust/pull/63166)
* [Stabilize `bind_by_move_pattern_guards` in Rust 1.39.0](https://github.com/rust-lang/rust/pull/63118)
* [Stabilize `checked_duration_since` for 1.38.0](https://github.com/rust-lang/rust/pull/62860)
* [Stabilize `pin_into_inner` in 1.39.0](https://github.com/rust-lang/rust/pull/63985)
* [`Rev::rposition` counts from the wrong end](https://github.com/rust-lang/rust/pull/63549)
* [Override `StepBy::{try_fold, try_rfold}`](https://github.com/rust-lang/rust/pull/64121)
* [Add Iterator comparison methods that take a comparison function](https://github.com/rust-lang/rust/pull/62205)
* [Add methods for converting `bool` to `Option<T>`](https://github.com/rust-lang/rust/pull/64255)
* [cargo: Rename `--all` to `--workspace`](https://github.com/rust-lang/cargo/pull/7241)

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

> The Rust compiler is basically 30 years of trying to figure out how to teach a computer how to see the things we worry about as C developers.

– [James Munns (@bitshiftmask) on Twitter](https://mobile.twitter.com/bitshiftmask/status/1170043794387083268)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/699) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
