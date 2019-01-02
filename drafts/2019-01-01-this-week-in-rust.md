Title: This Week in Rust 267
Number: 267
Date: 2019-01-01
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

* [This year in gfx-rs: 2018](https://gfx-rs.github.io/2018/12/27/this-year.html).

### #Rust2019

Find all #Rust2019 posts at [Read Rust](https://readrust.net/rust-2019/).

# Crate of the Week

This week's crate is [Dose Response](https://tryjumping.itch.io/dose-response), an online-playable roguelike game with a probably bleak outcome. Thanks to [Vikrant Chaudhary](https://users.rust-lang.org/t/crate-of-the-week/2704/473) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [A call for Rust 2019 Roadmap blog posts](https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html).
* [PEACE: Implement loading functions from static linked libraries](https://github.com/playXE/PEACE/issues/1). PEACE is a simple JIT library.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

150 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-12-24..2018-12-31

* [add `-Z instrument-mcount`](https://github.com/rust-lang/rust/pull/57220)
* [parallel query tweaks](https://github.com/rust-lang/rust/pull/56983)
* [move jemalloc from rustc_driver to rustc](https://github.com/rust-lang/rust/pull/56986)
* [uninline some debugging code and use unlikely! macro](https://github.com/rust-lang/rust/pull/57035)
* [resolve: fix an ICE in import validation](https://github.com/rust-lang/rust/pull/57160)
* [resolve: fix another ICE in import validation](https://github.com/rust-lang/rust/pull/57181)
* [AST/HIR: introduce `ExprKind::Err` for better error recovery in the front-end](https://github.com/rust-lang/rust/pull/56999)
* [fix new unused patch warning](https://github.com/rust-lang/cargo/pull/6494)
* [suggest `.as_ref()?` instead of `?` in certain circumstances](https://github.com/rust-lang/rust-clippy/pull/3561)
* [suggest `.as_ref()` when appropriate for `Option` and `Result`](https://github.com/rust-lang/rust/pull/57158)
* [tweaks to format string diagnostics](https://github.com/rust-lang/rust/pull/57140)
* [various changes to string format diagnostics](https://github.com/rust-lang/rust/pull/57069)
* [point to cause of `fn` expected return type](https://github.com/rust-lang/rust/pull/57020)
* [codegen: make zero-sized arrays affect alignment](https://github.com/rust-lang/rust-bindgen/pull/1477)
* [make Alloc::check_bounds_ptr private; you should use Memory::check_bounds_ptr instead](https://github.com/rust-lang/rust/pull/57129)
* [add Unpin to std prelude, not just core](https://github.com/rust-lang/rust/pull/57137)
* [remove the private generic NonZero<T> wrapper type](https://github.com/rust-lang/rust/pull/57133)
* [stabilize Duration::{as_millis, as_micros, as_nanos}](https://github.com/rust-lang/rust/pull/57124)
* [make the getter for NonZero types into a const fn](https://github.com/rust-lang/rust/pull/57167)
* [make tm struct members public](https://github.com/rust-lang/libc/pull/1183)
* [testsuite: require failing commands to check output](https://github.com/rust-lang/cargo/pull/6497)

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

*No RFCs are currently in this section.*

## New RFCs

*There are currently no new RFCs*

# Upcoming Events

### Online

* [Jan  2. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jan  9. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Jan 14. Rust Community Content Subteam Meeting on Discord](ttps://discordapp.com/channels/442252698964721669/443773747350994945).
* [Jan 16. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia

* [Jan 9. Kuala Lumpur, MY - Rust Malaysia Meetup Kuala Lumpur](https://www.facebook.com/events/1128655260646848/).

### Europe

* [Dec 27 - 30. Leipzig, DE - Rust assembly at 35c3](https://users.rust-lang.org/t/35c3-rust-assembly-at-ccc-leipzig/22288).
* [Jan  8. Rapperswil-Jona, CH - Rust Zürichsee meetup at Coredump - Looking for a speaker](https://www.meetup.com/Rust-Zurich/events/253608548/).
* [Jan  9. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzcbmb/).
* [Jan 10. Brno, CZ - Rust meetup at Masaryk University](https://rust-brno.github.io/).
* [Jan 14. Cologne, DE - Rust Cologne Meetup](https://www.meetup.com/RustCologne/events/vnwndpyzcbdb/).

### North America

* [Dec 30. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbnc/).
* [Jan  2. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/246726699/).
* [Jan  2. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyzcbdb/).
* [Jan  6. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbjb/).
* [Jan  9. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyzcbmb/).
* [Jan  9. Boulder, US - Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [Jan 10. Columbus, US - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dbcfrpyzcbnb/).
* [Jan 10. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209742/).
* [Jan 10. San Diego, US - San Diego Rust](http://meetu.ps/c/2vF0G/4DXV4/a).
* [Jan 13. Mountain view, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbrb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at XAIN, Berlin, DE](https://www.linkedin.com/jobs/cap/view/948142464/?pathWildcard=948142464&trk=mcm).
* [Networking Engineer at MaidSafe, Ayr, GB (Remote)](https://maidsafe.net/careers/#networking_engineer).
* [Senior Backend/Blockchain Developer with Rust at BitFinex, Remote](https://bitfinex.recruitee.com/o/senior-backendblockchain-developer-with-rust-remote).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Sr. Software Engineer - Rust at Mersive, Denver, US](https://www.mersive.com/company/join-mersive-team/?gh_jid=4136286002).
* [Embedded operating system developer, Karlsruhe, DE](https://www.pse.kit.edu/karriere/joboffer.php?id=2093&language=en).
* [Student research assistant (embedded), Karlsruhe, DE](https://twitter.com/oli_obk/status/1064856324071178240).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> In theory it would be entirely reasonable to guess that most Rust projects would need to use a significant amount of unsafe code to escape the
> limitations of the borrow checker. However, in practice it turns out (shockingly!) that the overwhelming majority of programs can be implemented
> perfectly well using only safe Rust.

– PM_ME_UR_MONADS [on reddit](https://www.reddit.com/r/rust/comments/a7kkw9/looking_for_someone_to_change_my_view_on_this/ec3r38n/)

Thanks to [oberien](https://users.rust-lang.org/t/twir-quote-of-the-week/328/590) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
