Title: This Week in Rust 240
Number: 240
Date: 2018-06-26
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

[Generic associated types in iterators](https://boiethios.gitlab.io/blog/2018-06-21_GATs_iterators.html)

# Crate of the Week

This week's crate is [mutagen](https://github.com/llogiq/mutagen), a mutation testing framework for Rust. Thanks to llogiq for the suggestion.

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

95 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-06-18..2018-06-25

* [async/await](https://github.com/rust-lang/rust/pull/51580)
* [rename OOM to allocation error](https://github.com/rust-lang/rust/pull/51543)
* [prohibit `global_allocator` in submodules](https://github.com/rust-lang/rust/pull/51335)
* [run some stuff in parallel](https://github.com/rust-lang/rust/pull/51383)
* [reduce number of allocations done by NLL](https://github.com/rust-lang/rust/pull/51617)
* [NLL: walk the MIR only once for the "unused mut" lint](https://github.com/rust-lang/rust/pull/51660)
* [NLL diagnostics: revise `fn check_access_permissions`](https://github.com/rust-lang/rust/pull/51275)
* [improve memoization and refactor NLL type check](https://github.com/rust-lang/rust/pull/51460)
* [add existential type definitions](https://github.com/rust-lang/rust/pull/51414)
* [make more `libsyntax` methods public](https://github.com/rust-lang/rust/pull/51664)
* [add ability to apply custom derive to union types](https://github.com/rust-lang/rust/pull/50383)
* [The Great Generics Generalisation: HIR Edition](https://github.com/rust-lang/rust/pull/48149)
* [fix processing mod with multi-level path on Windows](https://github.com/rust-lang/rust/pull/51278)
* [disable probestack when GCOV profiling is being used](https://github.com/rust-lang/rust/pull/51666)
* [support future deprecation for rustc_deprecated](https://github.com/rust-lang/rust/pull/51681)
* [fix an ICE when matching over const slices](https://github.com/rust-lang/rust/pull/51733)
* [yet another "old borrowck" bug around match default bindings](https://github.com/rust-lang/rust/pull/51686)
* [fix erroneous error note when using field after move](https://github.com/rust-lang/rust/pull/51688)
* [three diagnostics upgrades](https://github.com/rust-lang/rust/pull/51750)
* [various changes to existing diagnostics](https://github.com/rust-lang/rust/pull/51463)
* [don't suggest incorrect syntax](https://github.com/rust-lang/rust/pull/51670)
* [use fstatat64 where available](https://github.com/rust-lang/rust/pull/51785)
* [specialize `StepBy<Range(Inclusive)>`](https://github.com/rust-lang/rust/pull/51601)
* [`impl Hash for !`](https://github.com/rust-lang/rust/pull/51404)
* [stabilize std::path::Path::ancestors](https://github.com/rust-lang/rust/pull/50894)
* [replace tempdir by tempfile](https://github.com/rust-lang/rust/pull/50698)
* [rustdoc: greatly improve tables display](https://github.com/rust-lang/rust/pull/51482)
* [rustbuild: use quiet tests by default](https://github.com/rust-lang/rust/pull/51367)
* [ship LLVM tools with the toolchain](https://github.com/rust-lang/rust/pull/50336)

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
* [Jul  6. Darmstadt, DE - Rhein-Main Rust Meetup](https://www.meetup.com/Rust-Rhein-Main/events/251928672).

### North America

* [Jun 24. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbgc/).
* [Jun 25. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/kkjnpnyxjbhc/).
* [Jun 26. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxjbjc/).
* [Jun 26. New York City, US - Procedural Macros - parse JSX using nom](https://www.meetup.com/Rust-NYC/events/251490499/).
* [Jun 27. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbkc/).
* [Jun 28. San Francisco, US - Rust Bay Area - [Mozilla] GUI in Rust and Chalk](https://www.meetup.com/Rust-Bay-Area/events/251073767/).
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

> I’m hesitating in cc’ing [the crate author] because I’d rather this be an educational conversation, and not a unsafety witchhunt.

– vitalyd on [rust-users](https://users.rust-lang.org/t/how-not-to-use-unsafe-code/18170/13)

[Please submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
