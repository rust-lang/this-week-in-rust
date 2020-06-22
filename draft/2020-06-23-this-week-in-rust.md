Title: This Week in Rust 344
Number: 344
Date: 2020-06-23
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*]()

# Updates from Rust Community

## News & Blog Posts

* [Zero To Production #2: Learn By Building An Email Newsletter](https://www.lpalmieri.com/posts/2020-06-21-zero-to-production-2-learn-by-building-an-email-newsletter/)

# Crate of the Week

This week's crate is [safer_ffi](https://github.com/getditto/safer_ffi), a library to help write safe FFI code.

Thanks to [Vlad Frolov](https://users.rust-lang.org/t/crate-of-the-week/2704/780) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

354 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-08..2020-06-15

* [enable AVR as a Tier 3 target upstream](https://github.com/rust-lang/rust/pull/69478)
* [enable LVI hardening for x86_64-fortanix-unknown-sgx](https://github.com/rust-lang/rust/pull/72655)
* [add `-Z span-debug` to allow for easier debugging of proc macros](https://github.com/rust-lang/rust/pull/72799)
* [add methods to go from a null-terminated `Vec<u8>` to a `CString`](https://github.com/rust-lang/rust/pull/73139)
* [check for live drops in constants after drop elaboration](https://github.com/rust-lang/rust/pull/71824)
* [display information about captured variable in `FnMut` error](https://github.com/rust-lang/rust/pull/72598)
* [don't create impl candidates when obligation contains errors](https://github.com/rust-lang/rust/pull/73005)
* [enforce unwind invariants](https://github.com/rust-lang/rust/pull/73133)
* [explain move errors that occur due to method calls involving `self`](https://github.com/rust-lang/rust/pull/72389)
* [fix `#[thread_local]` statics as `asm!` sym operands](https://github.com/rust-lang/rust/pull/73033)
* [fix trait alias inherent impl resolution](https://github.com/rust-lang/rust/pull/72556)
* [free `default()` forwarding to `Default::default()`](https://github.com/rust-lang/rust/pull/73001)
* [handle assembler warnings properly](https://github.com/rust-lang/rust/pull/73169)
* [on recursive ADT, provide indirection structured suggestion](https://github.com/rust-lang/rust/pull/72740)
* [provide suggestion to convert numeric op LHS rather than unwrapping RHS](https://github.com/rust-lang/rust/pull/73195)
* [querify whether a type has structural equality](https://github.com/rust-lang/rust/pull/73066)
* [relate existential associated types with variance Invariant](https://github.com/rust-lang/rust/pull/71896)
* [suggest including unused asm arguments in a comment to avoid error](https://github.com/rust-lang/rust/pull/73230)
* [support proc macros in intra doc link resolution](https://github.com/rust-lang/rust/pull/73183)
* [track span of function in method calls, and use this in `#[track_caller]`](https://github.com/rust-lang/rust/pull/73182)
* [use `min_specialization` in the remaining rustc crates](https://github.com/rust-lang/rust/pull/72707)
* [use shorthand linker strip arguments in order to support MacOS](https://github.com/rust-lang/rust/pull/73138)
* [expand: more precise locations for expansion-time lints](https://github.com/rust-lang/rust/pull/73178)
* [extend network support for HermitCore](https://github.com/rust-lang/rust/pull/73331)
* [fix caller_location intrinsic for Miri](https://github.com/rust-lang/rust/pull/73277)
* [improper ctypes: normalize return types and transparent structs](https://github.com/rust-lang/rust/pull/72890)
* [normalize adt fields during structural match checking](https://github.com/rust-lang/rust/pull/72897)
* [resolve: do not suggest imports from the same module in which we are resolving](https://github.com/rust-lang/rust/pull/72789)
* [structural_match: non-structural-match ty closures](https://github.com/rust-lang/rust/pull/73353)
* [chalk: add FnOnce trait, and provide impl for Function type](https://github.com/rust-lang/chalk/pull/494)
* [chalk: model function ABI in the Rust IR](https://github.com/rust-lang/chalk/pull/481)
* [chalk: recursive solver factoring and privacy](https://github.com/rust-lang/chalk/pull/513)
* [chalk: refactor ProgramClauseData to remove Implies variant](https://github.com/rust-lang/chalk/pull/514)
* [chalk: add `Unsize` trait implementation](https://github.com/rust-lang/chalk/pull/427)
* [miri: avoid tracking current location three times](https://github.com/rust-lang/rust/pull/72879)
* [remove `RawVec::reserve_in_place`](https://github.com/rust-lang/rust/pull/72417)
* [stabilize `Option::zip`](https://github.com/rust-lang/rust/pull/72938)
* [stabilize `vec::Drain::as_slice`](https://github.com/rust-lang/rust/pull/72584)
* [impl `AsRef<[T]>` for `vec::IntoIter<T>`](https://github.com/rust-lang/rust/pull/72583)
* [std: enable atomic.fence emission on wasm32](https://github.com/rust-lang/rust/pull/73036)
* [stdarch: fix x86 extract_epi{8,16} functions](https://github.com/rust-lang/stdarch/pull/868)
* [implement new gdb/lldb pretty-printers](https://github.com/rust-lang/rust/pull/72357)
* [cargo: add environment variables to identify the binary and crate name](https://github.com/rust-lang/cargo/pull/8270)
* [cargo: allow passing a registry index url directly to `cargo install`](https://github.com/rust-lang/cargo/pull/8344)
* [cargo: fix doctests not running with `--target=HOST`](https://github.com/rust-lang/cargo/pull/8358)
* [cargo: support `{prefix}` and `{lowerprefix}` markers in `config.json` `dl` key](https://github.com/rust-lang/cargo/pull/8267)
* [crates.io: allow configuring the application's domain name](https://github.com/rust-lang/crates.io/pull/2543)
* [crates.io: modifiers/highlight-syntax: Disable aggressive whitespace stripping](https://github.com/rust-lang/crates.io/pull/2564)
* [doc: make impl block collapsible if it has an associated constant](https://github.com/rust-lang/rust/pull/71842)
* [docs.rs: add compression for uploaded documentation](https://github.com/rust-lang/docs.rs/pull/780)
* [docs.rs: limit the size of served files](https://github.com/rust-lang/docs.rs/pull/834)
* [clippy: macro use suggestion](https://github.com/rust-lang/rust-clippy/pull/5279)
* [clippy: let_and_return: avoid "does not live long enough" errors](https://github.com/rust-lang/rust-clippy/pull/5680)
* [rustfmt: pick up comments between visibility modifier and item name](https://github.com/rust-lang/rustfmt/pull/4239)

## Rust Compiler Performance Triage

* [2020-06-16](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-16)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [add Windows system error codes that should map to `io::ErrorKind::TimedOut`](https://github.com/rust-lang/rust/pull/71756)
* [disposition: merge] [impl `PartialEq<Vec<B>> for &[A], &mut [A]`](https://github.com/rust-lang/rust/pull/71660)

## New RFCs

[RFC: add the Freeze trait to libcore/libstd](https://github.com/rust-lang/rfcs/pull/2944)

# Upcoming Events

### Online
* [June 18. Zurich, CH - Remote - Embedded Rust Update: probe.rs](https://www.meetup.com/Rust-Zurich/events/271020472/)
* [June 18. Turin, IT - Remote - Rust Turin Study Group](https://community.mozilla.org/events/gruppo-di-studio-di-rust-2/)
* [June 25. Edinburgh, UK - Remote - Pirrigator - Growing Tomatoes Free From Memory Errors and Race Conditions](https://www.meetup.com/rust-edi/events/271129693/)
* [June 25. Berlin, DE - Remote - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcjbhc/)

### North America
* [June 17. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcjbwb/)
* [June 18. Durham, NC - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcjbdc/)
* [June 30. Dallas, TX - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybcjbnc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> It feels like being part of a village that learns to love the dragon it battles.

– [turbinerneiter on Hacker News](https://news.ycombinator.com/item?id=23437950)

Thanks to [blonk](https://users.rust-lang.org/t/twir-quote-of-the-week/328/892) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hactqu/this_week_in_rust_343/)</small>
