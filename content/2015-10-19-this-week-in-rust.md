Title: This Week in Rust 101
Number: 101
Date: 2015-10-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [nasa42](https://github.com/nasa42), [brson](https://github.com/brson), and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [The little book of Rust macros](https://danielkeep.github.io/tlborm/).
* [Rust and IDEs](https://www.rust-lang.org/ides.html). The plan for better integration of Rust with IDEs.
* [This week in Redox 2](http://www.redox-os.org/news/this-week-in-redox-2/). Redox is an upcoming OS written in Rust.
* [This Week In Servo 37](http://blog.servo.org/2015/10/12/twis-37/).
* [Rust to the rescue (of Ruby)](https://medium.com/@fbzga/rust-to-the-rescue-of-ruby-2067f5e1dc25). Call Rust code from Ruby.
* [Hacking Servo for noobs](https://gist.github.com/paulrouget/2f00941e6e82aeecad23).
* [Building an SQL database with 10 Rust beginners](https://lukaskalbertodt.github.io/2015/10/09/building-an-sql-database-with-10-rust-beginners.html).
* [Using a Rust DLL from C#](http://www.loekvandenouweland.com/content/using-rust-code-from-csharp).
* [Creating a C API for a Rust library](http://www.joshmatthews.net/blog/2015/10/creating-a-c-api-for-a-rust-library/).
* [A simple link checker built with Rust](http://antonyh.co.uk/2015/10/a-simple-link-checker-built-with-rust/).

## Notable New Crates & Projects

* [Rust-Bio](https://github.com/rust-bio/rust-bio). A bio-informatics library for Rust.
* [Rust64](https://github.com/kondrak/rust64). A C64 emulator written in Rust.
* [Simplemad](https://github.com/bendykst/simple-mad.rs). A Rust interface for the MPEG audio decoding library libmad.
* [pine](https://github.com/softprops/pine). Process line output.
* [JSONlite](https://github.com/evuez/jsonlite.rs). A simple, self-contained, serverless, zero-configuration, json document store.
* [medio](https://github.com/softprops/medio). Rust bindings to Medium.com api.

# Updates from Rust Core

100 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-10-12..2015-10-19

See the [subteam report for 2015-10-16][subteam] for details.

[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-10-16/2801

## Notable changes

* [rust_trans: struct argument over ffi were passed incorrectly in some situations. Change cabi_x86_64 to better model the sysv abi](https://github.com/rust-lang/rust/pull/27017).
* [Move desugarings to lowering step](https://github.com/rust-lang/rust/pull/28857).
* [Implement conversion traits for primitive integer types](https://github.com/rust-lang/rust/pull/28921).
* [Remove the `push_unsafe!` and `pop_unsafe!` macros](https://github.com/rust-lang/rust/pull/28980).
* [Add `#[derive(Clone)]` to `std::fs::Metadata`](https://github.com/rust-lang/rust/pull/29021).
* [Add `into_inner` and `get_mut` to `Mutex` and `RwLock`](https://github.com/rust-lang/rust/pull/29031).
* [Reject `+` and `-` when parsing floats](https://github.com/rust-lang/rust/pull/29050).
* [Add `Shared` pointer and have `{Arc, Rc}` use it](https://github.com/rust-lang/rust/pull/29110).

## New Contributors

* billpmurphy
* David Ripton
* Fabiano Beselga
* kickinbahk
* Marcello Seri
* nxnfufunezn
* Robert Gardner

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [1257: Implement `.drain(range)` and `.drain()` respectively as appropriate on collections](https://github.com/rust-lang/rfcs/pull/1257).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [Promote the `libc` crate from the nursery](https://github.com/rust-lang/rfcs/pull/1291).
* [Enable the compiler to cache incremental workproducts](https://github.com/rust-lang/rfcs/pull/1298).
* [Add some additional utility methods to `OsString` and `OsStr`](https://github.com/rust-lang/rfcs/pull/1307).

## New RFCs

* [Changes to the compiler to support IDEs](https://github.com/rust-lang/rfcs/pull/1317).
* [Prevent unstable items from causing name resolution conflicts with downstream code](https://github.com/rust-lang/rfcs/pull/1321).
* [Amend `recover` with a `PanicSafe` bound](https://github.com/rust-lang/rfcs/pull/1323).
* [Add "panic-safe" or "total" alternatives to the existing panicking slicing syntax](https://github.com/rust-lang/rfcs/pull/1325).
* [Refine the unguarded-escape-hatch from RFC 1238](https://github.com/rust-lang/rfcs/pull/1327).
* [Allow a custom panic handler](https://github.com/rust-lang/rfcs/pull/1328).

# Upcoming Events

* [10/20. Rust Hack and Learn Hamburg](http://www.meetup.com/Rust-Meetup-Hamburg/events/225865482/).
* [10/28. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [10/28. Rust Amsterdam](http://www.meetup.com/Rust-Amsterdam/events/225117486/).
* [10/28. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/events/225614991/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [Glium](https://github.com/tomaka/glium) a safe Rust wrapper for OpenGL. Thanks to [DroidLogician](https://users.rust-lang.org/users/DroidLogician) for the suggestion. [Submit your suggestions for next week][submit_crate]!

OpenGL is a time-honored standard, which also means its API has seen enough growth to make it look like you might find Sleeping Beauty if you look deep enough. Also it was created when multi-core wasn't exactly on the radar, so many things are not thread safe. Caveat error! Glium pre-verifies whatever it can to make errors either impossible at compile time or panic before it can crash (so you at least get a helpful message instead of random garbage). It caches the context, manages your buffers using Rust's standard RAII idiom and by this brings some much-needed sanity to OpenGL programming.

# Quote of the Week

> S-s-s-s-stack alloc Queen, no C++ though
> Might drop that pointer, no nullable though
> Tell golang, "Yo, don't you got enough mem to slow?"
> Tell 'em Kangaroo Rust, I'll box your flow
>
> Advanced pattern matching - possible
> Don't go against Rusty - impossible
> Runtime will leave your CPU on popsicle
> Man these h*es couldn't be any less logical﻿

— [A tribute to #NickiMinaj and #rustlang by @boghison](https://twitter.com/boghison/status/653194698983669764).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
