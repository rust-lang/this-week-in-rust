Title: This Week in Rust 247
Number: 247
Date: 2018-08-14
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

This week's crate is [macro_railroad](https://github.com/lukaslueg/macro_railroad), a library to create neat syntax diagrams for `macro_rules!` declarative macros. Thanks to [kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/436) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [How to land your first Rust pull request in TiKV](https://www.pingcap.com/blog/adding-built-in-functions-to-tikv/).
* [easy] [Maud: Update benchmarks](https://github.com/lfairy/maud/issues/143). Maud is an HTML template engine for Rust.
* [atom-language-rust: Help with PR reviews](https://users.rust-lang.org/t/twir-call-for-participation/4821/202).
* [intl_pluralrules: Seeking crate review](https://users.rust-lang.org/t/twir-call-for-participation/4821/203).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

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

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for RFC 2093: Infer `T: 'x` outlives requirements on structs](https://github.com/rust-lang/rust/issues/44493).
* [disposition: merge] [Calculate capacity when collecting into Option and Result](https://github.com/rust-lang/rust/pull/52910).
* [disposition: close] [Undeprecate `thread::sleep_ms`](https://github.com/rust-lang/rust/pull/51610).

## New RFCs

* [Permit impl Trait in type aliases](https://github.com/rust-lang/rfcs/pull/2515).
* [Union initialization and Drop](https://github.com/rust-lang/rfcs/pull/2514).
* [Project-based Examples for Cargo Projects](https://github.com/rust-lang/rfcs/pull/2517).

# Upcoming Events

### Online

* [Aug 14. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Aug 15. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Aug 15. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Aug 22. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Europe

* [Aug 10. Frankfurt, DE - Rhein-Main Rust Meetup (with Special Guest)](https://www.meetup.com/Rust-Rhein-Main/events/253311151).
* [Aug 16. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxlbvb/).

### North America

* [Aug 12. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbqb/).
* [Aug 13. Seattle, US - Monthly Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxlbrb/).
* [Aug 15. Orange County, US - Rust Foreign Function Interface (FFI) Development](https://www.meetup.com/oc-rust/events/253565445/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).**
* [Aug 19. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbzb/).
* [Aug 22. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/253062831/).
* [Aug 22. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxlblb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. (San Francisco, Boston, Montreal)](https://www.reddit.com/r/rust/comments/92e67g/commure_healthcare_software_startup_hiring_rust/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Fearless concurrency includes fearless refactoring.

– [cuviper at rust-users](https://users.rust-lang.org/t/parallel-problems-to-showcase-rust-features/19365/6).

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/u/juleskers) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
