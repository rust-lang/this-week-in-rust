Title: This Week in Rust 239
Number: 239
Date: 2018-06-19
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

* [MIR-based borrow check (NLL) status update](http://smallcultfollowing.com/babysteps/blog/2018/06/15/mir-based-borrow-check-nll-status-update/).
* [aturon.log: listening and trust, part 3](https://aturon.github.io/2018/06/18/listening-part-3/).
* [Integration tests](https://os.phil-opp.com/integration-tests/). Part of the series [Writing an OS in Rust](https://os.phil-opp.com/).
* [From ActiveRecord to Diesel](http://patshaughnessy.net/2018/6/9/from-activerecord-to-diesel).
* [A trick for test maintenance](https://matklad.github.io/2018/06/18/a-trick-for-test-maintenance.html).
* [Why Rust's async functions should use the outer return type approach](https://github.com/MajorBreakfast/rust-blog/blob/master/posts/2018-06-19-outer-return-type-approach.md).
* [Running Rust on the GPU with Accel](https://bheisler.github.io/post/rust-on-the-gpu-with-accel/).
* [This week in Rust and WebAssembly 3](https://rustwasm.github.io/2018/06/04/this-week-in-rust-wasm-003.html).
* [podcast] [Rusty Spike Podcast - episode 33](https://rusty-spike.blubrry.net/2018/06/13/episode-33-jun-13-2018/). Crossbeam channels, actix benchmarks (and how to use it), qt, servo, gfx-portability, debian, OS development, LibOS, and a new book.

# Crate of the Week

This week's crate is [clap-verbosity-flag](https://crates.io/crates/clap-verbosity-flag), a small crate to easily add a verbosity setting to Rust command line applications. Thanks to [Yoshuawuyts](https://users.rust-lang.org/u/yoshuawuyts) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help Rust Language Server get to 1.0](https://github.com/rust-lang-nursery/rls/issues/914).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

110 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-06-04..2018-06-11

* [Keyword unreservations (pure, sizeof, alignof, offsetof)](https://github.com/rust-lang/rust/pull/51196) (RFC [#2421](https://rust-lang.github.io/rfcs/2421-unreservations-2018.html))
* [parser: split `+=` into `+` and `=` where `+` is explicitly requested (such as generics)](https://github.com/rust-lang/rust/pull/51068)
* [enable fall through past $:lifetime matcher](https://github.com/rust-lang/rust/pull/51480)
* [deny `#[cfg]` and `#[cfg_attr]` on generic parameters](https://github.com/rust-lang/rust/pull/51283)
* [include parens to type parameter](https://github.com/rust-lang/rust/pull/50205)
* [use scope tree depths to speed up `nearest_common_ancestor`](https://github.com/rust-lang/rust/pull/51394)
* [avoid useless Vec clones in `pending_obligations`](https://github.com/rust-lang/rust/pull/51412)
* [NLL performance boost](https://github.com/rust-lang/rust/pull/51399)
* [NLL: report type moved from behind borrow of array/slice](https://github.com/rust-lang/rust/pull/51247)
* [deduplicate auto traits in trait objects](https://github.com/rust-lang/rust/pull/51276)
* [remove dependency on fmt_macros from typeck](https://github.com/rust-lang/rust/pull/51380)
* [re-enable trivial bounds](https://github.com/rust-lang/rust/pull/51042)
* [use spans pointing at the inside of a rustdoc attribute](https://github.com/rust-lang/rust/pull/51391)
* [suggest parentheses when a struct literal needs them](https://github.com/rust-lang/rust/pull/51360)
* [suggest not mutably borrowing a mutable reference](https://github.com/rust-lang/rust/pull/51242)
* [add deprecation lint for duplicated `macro_export`s](https://github.com/rust-lang/rust/pull/50143)
* [accept `..` in incorrect position to avoid further errors](https://github.com/rust-lang/rust/pull/51201)
* [fix the use of closures within `#[panic_implementation]`](https://github.com/rust-lang/rust/pull/51368)
* [refactor the const eval diagnostic API](https://github.com/rust-lang/rust/pull/51316)
* [check array indices in constant propagation](https://github.com/rust-lang/rust/pull/51308)
* [`ScalarPairs` are offset==0 field + other non-zst field](https://github.com/rust-lang/rust/pull/51307/files)
* [blocking Rayon queries](https://github.com/rust-lang/rust/pull/50699/files)
* [reexport `fmt::Alignment` into `std`](https://github.com/rust-lang/rust/pull/51333)
* [add `Future` and task system to the standard library](https://github.com/rust-lang/rust/pull/51263)
* [futures: add a few blanket impls to `std`](https://github.com/rust-lang/rust/pull/51442)
* [fix confusing error message for sub_instant](https://github.com/rust-lang/rust/pull/51255)
* [stabilize `Iterator::step_by`](https://github.com/rust-lang/rust/pull/51320)
* [stabilize `iterator_repeat_with`](https://github.com/rust-lang/rust/pull/51200)
* [stabilize `entry-or-default`](https://github.com/rust-lang/rust/pull/51079)
* [stabilize unit tests with non-`()` return type](https://github.com/rust-lang/rust/pull/51298)
* [cargo: allow rustc bootstrap to use unstable features even though it's using a beta-cargo](https://github.com/rust-lang/cargo/pull/5613)
* [fix crate-name option in rustdoc](https://github.com/rust-lang/rust/pull/51256)
* [rustbuild: allow enabling incremental via config.toml](https://github.com/rust-lang/rust/pull/51317)
* [rustbuild: do not require stage 2 compiler for rustdoc](https://github.com/rust-lang/rust/pull/51436)

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

* [disposition: merge] [Add `Option::replace` to the core library](https://github.com/rust-lang/rfcs/pull/2296).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for the to_bytes and from_bytes methods of integers](https://github.com/rust-lang/rust/issues/49792).
* [disposition: merge] [Tracking issue for RFC 554: `pattern_parentheses` feature](https://github.com/rust-lang/rust/issues/51087).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [Tracking issue for "macro naming and modularisation" (RFC #1561)](https://github.com/rust-lang/rust/issues/35896).
* [disposition: merge] [Tracking issue for promoting `!` to a type (RFC 1216)](https://github.com/rust-lang/rust/issues/35121).
* [disposition: merge] [Loosened rules involving statics mentioning other statics](https://github.com/rust-lang/rust/pull/51110).
* [disposition: merge] [Implement `PartialEq` between `&str` and `OsString`](https://github.com/rust-lang/rust/pull/51178).
* [disposition: merge] [Undeprecate `thread::sleep_ms`](https://github.com/rust-lang/rust/pull/51610).

## New RFCs

* [Clippy 1.0](https://github.com/rust-lang/rfcs/pull/2476).
* [Signing registry index commits](https://github.com/rust-lang/rfcs/pull/2474).
* [Add the `group_by` and `group_by_mut` methods to slice](https://github.com/rust-lang/rfcs/pull/2477).
* [Add `delete` and `delete_by` methods to `Iterator`](https://github.com/rust-lang/rfcs/pull/2475).
* [Accept semicolons as items](https://github.com/rust-lang/rfcs/pull/2479).
* [Update RFC 0430 to allow underscores between numbers in CamelCase names](https://github.com/rust-lang/rfcs/pull/2478).

# Upcoming Events

### Online

* [Jun 27. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jul  3. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jul  4. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jul  4. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Africa

* [Jul  3. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxkbfb/).

### Europe

* [Jun 21. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxjbcc/).
* [Jun 27. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/251675898/).
* [Jun 27. Cologne, DE - Rust Cologne at ThoughtWorks](https://www.meetup.com/RustCologne/events/vnwndpyxjbjb/).
* [Jun 27. Milan, IT - Perché non compila](https://www.meetup.com/rust-language-milano/events/251914721/)?

### North America

* [Jun 24. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbgc/).
* [Jun 25. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/kkjnpnyxjbhc/).
* [Jun 26. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxjbjc/).
* [Jun 26. New York City, US - Procedural Macros - parse JSX using nom](https://www.meetup.com/Rust-NYC/events/251490499/).
* [Jun 27. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbkc/).
* [Jun 28. San Francisco, US - Rust Bay Area - Zero Knowledge Proof Macros and Cernan (data pipelining)](https://www.meetup.com/Rust-Bay-Area/events/244156617/).
* [Jul  1. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkbcb/).
* [Jul  4. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxkbgb/).
* [Jul  4. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxkbgb/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).** Registration is now open.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at 1aim, Berlin](https://www.reddit.com/r/rust/comments/8qrcvv/rust_developer_roles_available_at_1aim_apply_now/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> explicit lifetimes no longer scare me the way they used to.

– Nicholas Nethercote on [his blog](https://blog.mozilla.org/nnethercote/2018/06/05/how-to-speed-up-the-rust-compiler-some-more-in-2018)

(selected by llogiq per one unanimous vote)

[Please submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
