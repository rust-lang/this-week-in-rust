Title: This Week in Rust 274
Number: 274
Date: 2019-02-19
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

This week's crate is [num-format](https://github.com/bcmyers/num-format), a crate to format numbers to international standards. Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/485) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [raft: Convert `Storage::entries`'s `max_size` argument to `Option<u64>`](https://github.com/pingcap/raft-rs/issues/98).
* [TiKV: Convert trait objects to `dyn` syntax for Rust 2018](https://github.com/tikv/tikv/issues/4197).
* [TiKV: Remove all the `extern crate`s for Rust 2018](https://github.com/tikv/tikv/issues/4196).
* [TiKV: Add tcmalloc support to the tikv_alloc crate](https://github.com/tikv/tikv/issues/4191).
* [rand: Standard should be implemented for NonZero types](https://github.com/rust-random/rand/issues/727).
* [Tarpaulin: Test coveralls with other CI services](https://github.com/xd009642/tarpaulin/issues/213).
* [Inferno: Multiple good first issues](https://github.com/jonhoo/inferno/issues).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

247 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-02-11..2019-02-18

* [Implement incremental "fat" LTO](https://github.com/rust-lang/rust/pull/58378)
* [Enable comparing fat pointers](https://github.com/rust-lang/rust/pull/58301)
* [`impl iter() for dyn Error`](https://github.com/rust-lang/rust/pull/58289)
* [Improve the error messages for missing stability attributes](https://github.com/rust-lang/rust/pull/58276)
* [Cut down on number formating code size](https://github.com/rust-lang/rust/pull/58272)
* [Reduce the size of `hir::Expr`](https://github.com/rust-lang/rust/pull/58258)
* [Make `saturating_add` and `saturating_sub` `const` functions](https://github.com/rust-lang/rust/pull/58246)
* [Stabilize `slice_sort_by_cached_key`](https://github.com/rust-lang/rust/pull/58074)
* [Stabilize `str::escape_*` methods with new return types](https://github.com/rust-lang/rust/pull/58051)
* [Stabilize the `time_checked_add` feature](https://github.com/rust-lang/rust/pull/58034)
* [Update the future/task API](https://github.com/rust-lang/rust/pull/57992)
* [Speed up the fast path for `assert_eq!` and `assert_ne!`](https://github.com/rust-lang/rust/pull/57815)
* [cargo: Stabilize Alternative Registries](https://github.com/rust-lang/cargo/pull/6654)

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

* [disposition: postpone] [Generic integers](https://github.com/rust-lang/rfcs/pull/2581).
* [disposition: postpone] [Accept semicolons as item-like](https://github.com/rust-lang/rfcs/pull/2479).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize TryFrom and TryInto with a convert::Infallible empty enum](https://github.com/rust-lang/rust/pull/58302).
* [disposition: merge] [Tracking issue for str::as_mut_ptr](https://github.com/rust-lang/rust/issues/58215).
* [disposition: merge] [Stabilize slice_sort_by_cached_key](https://github.com/rust-lang/rust/pull/58074).
* [disposition: merge] [deprecate before_exec in favor of unsafe pre_exec](https://github.com/rust-lang/rust/pull/58059).
* [disposition: merge] [Stabilize linker-plugin based LTO (aka cross-language LTO)](https://github.com/rust-lang/rust/pull/58057).
* [disposition: merge] [Tracking issue for std::iter::successors](https://github.com/rust-lang/rust/issues/58045).
* [disposition: merge] [Tracking issue for Option::copied](https://github.com/rust-lang/rust/issues/57126).
* [disposition: merge] [Tracking issue for std::ptr::hash](https://github.com/rust-lang/rust/issues/56286).
* [disposition: merge] [Rename `MaybeUninit` to `MaybeUninitialized`](https://github.com/rust-lang/rust/pull/56138).
* [disposition: merge] [Tracking issue for std::iter::from_fn](https://github.com/rust-lang/rust/issues/55977).

## New RFCs

* [Changing the overflow behavior for usize in release builds to panic](https://github.com/rust-lang/rfcs/pull/2635).
* [#[ffi_returns_twice]](https://github.com/rust-lang/rfcs/pull/2633).

# Upcoming Events

### Online

* [Feb 20. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Feb 25. Rust Community Content Subteam Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Feb 27. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia Pacific

* [Feb 16. Chennai, IN - Rust Chennai meetup](https://www.meetup.com/mad-rs/events/258822338/).

### Europe

* [Feb 18. Karlsruhe, DE - Karlsruhe Rust Hack and Learn](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/258728236/).
* [Feb 20. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzdbbc/).

### North America

* [Feb 14. Columbus, US - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dbcfrpyzdbsb/).
* [Feb 20. Sacramento, US - Sacramento Rust Inaugural Meetup](https://www.meetup.com/Rust-Sacramento/events/258393260/).
* [Feb 20. Chicago, US - Chicago Rust Meetup - Property-Based Testing in Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/257469240/).
* [Feb 20. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/hkllqqyzdbbc/).
* [Feb 21. San Diego, US - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/258775454/).
* [Feb 21. Arlington, US - Rust DC—Learn+Try: Custom Redis Datastructures](https://www.meetup.com/RustDC/events/257969733).
* [Feb 25. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzdbhc/).
* [Feb 27. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzdbkc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Developer at Finhaven, Vancouver, CA](https://angel.co/finhaven/jobs/411238-software-developer).
* [Software Engineer at Discord, San Francisco, US](https://discordapp.com/jobs/4200751002).
* [Network Engineer at NearProtocol, San Francisco, US](https://nearprotocol.com/careers/?gh_jid=4205573002).
* [Navitia Software Engineer at Kisio Digital, Paris, FR](https://www.welcometothejungle.co/companies/kisio-digital/jobs/rust-c-developpeur-h-f_paris).
* [Rust web developer at Impero, Denmark/remote](https://impero.com/job/full-stack-web-developer-rust/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> … the experience I had in 2019 was dramatically better than the first time I touched the language. After a month I’m feeling very comfortable, and looking forward to writing more.

Ryan Ragona, [Learning Rust in 2019](https://www.ragona.com/posts/learning_rust_2019)

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/t/twir-quote-of-the-week/328/624) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
