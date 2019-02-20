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

* [Moving from Ruby to Rust](https://deliveroo.engineering/2019/02/14/moving-from-ruby-to-rust.html).
* [Rust lifetime visualization ideas](https://blog.adamant-lang.org/2019/rust-lifetime-visualization-ideas/).
* [Generators II: The question mark problem](https://boats.gitlab.io/blog/post/generators-ii/).
* [A list of itch.io games written in Rust](https://itch.io/c/449652/rustlang-games).
* [Cross-compiling Rust code to Minix](https://iandouglasscott.com/2019/02/18/cross-compiling-rust-code-to-minix/).
* [One hundred Rust PRs later](https://phansch.net/2019/02/18/onehundred-rust-prs/).
* [This week in Rust and WebAssembly 10](https://rustwasm.github.io/2019/02/13/this-week-in-rust-and-wasm-010.html).

# Crate of the Week

This week's crate is [num-format](https://github.com/bcmyers/num-format), a crate to format numbers to international standards. Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/485) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [RustFest is searching for local teams to apply for the next events](https://blog.rustfest.eu/call-for-teams).
* [The CLI-WG wants to help other CLI maintainers to write good first issues for contributions](https://github.com/rust-lang-nursery/cli-wg/issues/120).
* [TiKV: Figure out how to add a second "release" profile for "dev+optimized" builds](https://github.com/tikv/tikv/issues/4189).
* [TiKV: Consolidate rocksdb imports into one module for engine abstraction](https://github.com/tikv/tikv/issues/4229).
* [TiKV: Avoid unnecessary clone of unstable raft log](https://github.com/tikv/tikv/issues/2373).

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

* [RFC 2570: Linked list cursors](https://github.com/rust-lang/rfcs/pull/2570).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [#[link(kind="raw-dylib")]](https://github.com/rust-lang/rfcs/pull/2627).
* [disposition: merge] [Associated type defaults](https://github.com/rust-lang/rfcs/pull/2532).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Relax some Ord bounds on BinaryHeap<T>](https://github.com/rust-lang/rust/pull/58421).
* [disposition: merge] [Relax some Hash bounds on HashMap<K, V, S> and HashSet<T, S>](https://github.com/rust-lang/rust/pull/58370).
* [disposition: merge] [Stabilize TryFrom and TryInto with a convert::Infallible empty enum](https://github.com/rust-lang/rust/pull/58302).
* [disposition: merge] [Clarify guarantees for `Box` allocation](https://github.com/rust-lang/rust/pull/58183).
* [disposition: merge] [dbg!() without parameters](https://github.com/rust-lang/rust/pull/57847).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online

* [Feb 27. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar  6. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Mar  6. Johannesburg, ZA - Rust on embedded](https://www.meetup.com/Johannesburg-Rust-Meetup/events/qbhxmqyzfbjb/).

### Europe

* [Feb 28. Copenhagen, DK - Copenhagen Rust Hack Night #0xC](https://cph.rs/).
* [Feb 28. Torino, IT - Rust Turin Meetup](https://www.meetup.com/Mozilla-Torino/events/258586428).
* [Mar  6. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzfbjb/).

### North America

* [Feb 21. San Diego, US - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/258775454/).
* [Feb 21. Arlington, US - Rust DC—Learn+Try: Custom Redis Datastructures](https://www.meetup.com/RustDC/events/257969733).
* [Feb 25. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzdbhc/).
* [Feb 26. Irvine, US - Orange County Rust](https://www.meetup.com/oc-rust/events/258331354/).
* [Feb 27. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzdbkc/).
* [Feb 27. Mesa, US - Phoenix Rust: Embedded Devices](https://www.meetup.com/Desert-Rustaceans/events/258596537).
* [Mar  6. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzfbjb/).
* [Mar  6. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyzfbjb/).
* [Mar  6. Vancouver, CN - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/hkllqqyzfbjb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Software Consultant at Knoldus, Noida, IN](https://www.knoldus.com/careers/rust-software-consultant.knol).
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
