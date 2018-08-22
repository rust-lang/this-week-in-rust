Title: This Week in Rust 248
Number: 248
Date: 2018-08-21
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

* [Announcing Rust 2018 Preview 2](https://internals.rust-lang.org/t/annoucning-rust-2018-preview-2/8218). <sub>[[discuss](https://www.reddit.com/r/rust/comments/97mpt0/annoucning_rust_2018_preview_2/)]</sub>
* 12 [Safe partial initialization in Rust](https://scottjmaddox.github.io/Safe-partial-initialization-in-Rust/). <sub>[[discuss](https://www.reddit.com/r/rust/comments/98nmge/safe_partial_initialization_in_rust/)]</sub>
* [Announcing the RLS 1.0 release candidate](https://www.ncameron.org/blog/rls-1-0-release-candidate/). <sub>[[discuss](https://www.reddit.com/r/rust/comments/98gqsg/announcing_the_rls_10_release_candidate/)]</sub>
* [How Rust’s standard library was vulnerable for years and nobody noticed](https://medium.com/@shnatsel/how-rusts-standard-library-was-vulnerable-for-years-and-nobody-noticed-aebf0503c3d6). <sub>[[discuss](https://www.reddit.com/r/rust/comments/988euh/how_rusts_standard_library_was_vulnerable_for/)]</sub>
* [With undefined behavior, anything is possible](https://raphlinus.github.io/programming/rust/2018/08/17/undefined-behavior.html). <sub>[[discuss](https://www.reddit.com/r/rust/comments/985id0/with_undefined_behavior_anything_is_possible/)]</sub>
* [Rust GraphQL webserver with Warp, Juniper, and MongoDB](http://alex.amiran.it/post/2018-08-16-rust-graphql-webserver-with-warp-juniper-and-mongodb.html). <sub>[[discuss](https://www.reddit.com/r/rust/comments/97zlav/rust_graphql_webserver_with_warp_juniper_mongodb/)]</sub>
* 82 [Thanks for asking](https://llogiq.github.io/2018/08/16/ask.html). An analysis of questions that are asked on r/rust subreddit. <sub>[[discuss](https://www.reddit.com/r/rust/comments/97ps8m/blog_thanks_for_asking/)]</sub>
* [podcast] [New Rustacean news: Rust 1.28](https://newrustacean.com/show_notes/news/rust_1_28/). <sub>[[discuss](https://www.reddit.com/r/rust/comments/98tkve/new_rustaceannews_rust_128/)]</sub>
* [Programming Servo: Anatomy of a fetch](https://medium.com/programming-servo/anatomy-of-a-fetch-8872a5c843cd). <sub>[[discuss](https://www.reddit.com/r/rust/comments/986p56/programming_servo_anatomy_of_a_fetch/)]</sub>
* [This week in Rust and WebAssembly 6](https://rustwasm.github.io/2018/08/14/this-week-in-rust-wasm-006.html). <sub>[[discuss](https://www.reddit.com/r/rust/comments/97ah4r/this_week_in_rust_and_webassembly_6/)]</sub>

# Crate of the Week

This week's crate is [macro_railroad](https://github.com/lukaslueg/macro_railroad), a library to create neat syntax diagrams for `macro_rules!` declarative macros. Thanks to [kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/436) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for help building a distributed filesystem in Rust](https://www.reddit.com/r/rust/comments/98d3zk/call_for_help_building_a_distributed_filesystem/).
* [easy] [rustc: mark applicability of diagnostic suggestions](https://github.com/rust-lang/rust/issues/50723).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

102 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-08-06..2018-08-13

* [whitelist wasm32 simd128 target feature](https://github.com/rust-lang/rust/pull/53179)
* [fix a few regressions from enabling macro modularization](https://github.com/rust-lang/rust/pull/53270)
* [resolve: support custom attributes when macro modularization is enabled](https://github.com/rust-lang/rust/pull/53053)
* [Place unions, pointer casts and pointer derefs behind extra feature gates](https://github.com/rust-lang/rust/pull/51990)
* [suggest float for integer literals where a float was expected](https://github.com/rust-lang/rust/pull/53283)
* [suggest missing comma in macro call](https://github.com/rust-lang/rust/pull/53183)
* [add help message for missing `IndexMut` impl](https://github.com/rust-lang/rust/pull/52788)
* [add errors for unknown, stable and duplicate feature attributes](https://github.com/rust-lang/rust/pull/52644)
* [suggest comma when writing `println!("{}" a);`](https://github.com/rust-lang/rust/pull/52397)
* [emit error for pattern arguments in trait methods](https://github.com/rust-lang/rust/pull/53051)
* [fix improper_ctypes lint for individual foreign items](https://github.com/rust-lang/rust/pull/53100)
* [NLL: use span of the closure args in free region errors](https://github.com/rust-lang/rust/pull/53088)
* [apply some fixes to cross-language LTO (especially when targeting MSVC)](https://github.com/rust-lang/rust/pull/53031)
* [Un-name globals with private linkage](https://github.com/rust-lang/rust/pull/51007)
* [avoid many allocations for `CString`s during codegen](https://github.com/rust-lang/rust/pull/53161)
* [change `assert!` to `debug_assert!` in `visit_with`](https://github.com/rust-lang/rust/pull/53025)
* [don't `collect()` when `size_hint` is useless](https://github.com/rust-lang/rust/pull/53019)
* [make IpvXAddr::new const fns and the well known addresses associated constants](https://github.com/rust-lang/rust/pull/52872)
* [change rustdoc style so fully qualified name does not overlap src link](https://github.com/rust-lang/rust/pull/53060)
* [crates.io: add crate size on the crate detail page](https://github.com/rust-lang/crates.io/pull/1436)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2504: Fix the Error trait](https://github.com/rust-lang/rfcs/pull/2504).
* [RFC 2351: Add `is_sorted` to the standard library](https://github.com/rust-lang/rfcs/pull/2351).
* [RFC 2229: Closures Capture Disjoint Fields](https://github.com/rust-lang/rfcs/pull/2229).
* [RFC 1892: Deprecate uninitialized in favor of a new MaybeUninit type](https://github.com/rust-lang/rfcs/pull/1892).
* [RFC 2306: Add `pub fn identity<T>(x: T) -> T { x }` to `core::convert`](https://github.com/rust-lang/rfcs/pull/2306).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [eRFC: if- and while-let-chains, take 2](https://api.github.com/repos/rust-lang/rfcs/issues/2497).
* [disposition: merge] [Unify std::os::raw::c_void and libc::c_void via libcore](https://github.com/rust-lang/rfcs/pull/2521).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add Error::source method per RFC 2504.](https://api.github.com/repos/rust-lang/rust/issues/53533).
* [disposition: merge] [Tracking issue for integer types conversion to and from byte arrays](https://api.github.com/repos/rust-lang/rust/issues/52963).
* [disposition: merge] [Tracking Issue for Iterator::find_map](https://api.github.com/repos/rust-lang/rust/issues/49602).
* [disposition: merge] [Allow all literals in attributes (Tracking Issue for RFC #1559)](https://api.github.com/repos/rust-lang/rust/issues/34981).
* [disposition: merge] [Tracking issue for RFC#1685: Deprecate anonymous parameters](https://github.com/rust-lang/rust/issues/41686).
* [disposition: close] [Define non-panicking UTF encoding methods on `char`](https://api.github.com/repos/rust-lang/rust/issues/52580).

## New RFCs

* [Support underscores as constant names](https://api.github.com/repos/rust-lang/rfcs/issues/2526).
* [RFC: Permit _ in type aliases](https://api.github.com/repos/rust-lang/rfcs/issues/2524).

# Upcoming Events

### Online

* [Aug 28. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Aug 29. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Aug 29. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Sep  5. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Aug 25. Nairobi, KE - Rustbridge Workshop at GirlsCode](https://www.meetup.com/Rust-Nairobi/events/253950971/).
* [Sep  4. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxmbgb/).

### Asia

* [Sep 2. Tel Aviv, IL - Cargo, Production and N00bing](https://www.meetup.com/Rust-TLV/events/253408497/).

### Europe

* [Sep  4. Brussels, BE - #3 futures/async/tokio && Gotham-rs](https://www.meetup.com/Belgium-Rust-user-group/events/249899651/).
* [Sep  5. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/253541000/).

### North America

* [Aug 26. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbjc/).
* [Aug 27. Durham, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxlbkc/).
* [Aug 28. Chicago, US - Rust Meetup](https://www.meetup.com/Chicago-Rust-Meetup/events/253621611/).
* [Aug 28. Dallas, US - Rust Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxlblc/).
* [Sep  2. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbdb/).
* [Sep  5. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxmbhb/).
* [Sep  5. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxmbhb/).
* [Sep  5. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxmbhb/).

### South America

* [Aug 23. Montevideo, UY - Rust meetup Montevideo](https://www.meetup.com/Rust-Uruguay/events/253617627/).
* [Sep  3. Montevideo, UY - Rust meetup - WebAssembly](https://www.meetup.com/Rust-Uruguay/events/253617627/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer - Blockchain at TenX, Sydney, AU](https://tenx.workable.com/jobs/689268).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Fearless concurrency includes fearless refactoring.

– [cuviper at rust-users](https://users.rust-lang.org/t/parallel-problems-to-showcase-rust-features/19365/6).

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/u/juleskers) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<sub>Discuss this issue on [r/rust]()</sub>
