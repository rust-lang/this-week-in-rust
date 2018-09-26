Title: This Week in Rust 253
Number: 253
Date: 2018-09-25
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

* 🎈🎉 [Announcing Rust 1.29.1](https://blog.rust-lang.org/2018/09/25/Rust-1.29.1.html). 🎉🎈
* [Citybound: Introducing live builds](http://aeplay.co/citybound-devblog/introducing-live-builds).
* [The biggest wasm-pack release yet](https://rustwasm.github.io/2018/09/24/the-biggest-wasm-pack-release-yet.html).
* [Office Hours with Niko Matsakis: Debugging with GDB](http://smallcultfollowing.com/babysteps/blog/2018/09/21/office-hours-0-debugging-with-gdb/).
* [Office Hours with Niko Matsakis: Cyclic services](http://smallcultfollowing.com/babysteps/blog/2018/09/24/office-hours-1-cyclic-services/).
* [Rust on iOS: How to](https://github.com/mtak-/rust-on-mobile/blob/master/examples/ios/example01.md).
* [A Rust FFI adventure in unsafety](https://travisf.net/capstone-rs-unsafety-adventure).
* [How to integrate Rust with C#](https://blog.getseq.net/rust-at-datalust-how-we-integrate-rust-with-csharp/).
* [Rewriting a bash script in Rust](https://wiredforge.com/blog/misc/getpid/index.html).
* [Creating a robot for Eurobot in Rust: The context](https://blog.florencepaul.com/creating-a-robot-for-eurobot-part-1-context).
* [Incinerator: The ABA problem and concurrent reclamation](https://bzim.gitlab.io/blog/posts/incinerator-the-aba-problem-and-concurrent-reclamation.html).

# Crate of the Week

This week's crate is [packed_simd](https://github.com/rust-lang-nursery/packed_simd), a crate with portable SIMD vector types. Thanks to [Gabriel Majeri](https://users.rust-lang.org/t/crate-of-the-week/2704/456) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust 2018 RC1 is available - please help test](https://internals.rust-lang.org/t/rust-2018-rc1-is-available/8435).
* [Rust office hours with Niko Matsakis](http://smallcultfollowing.com/babysteps/blog/2018/09/12/rust-office-hours/).
* [rust: Panic in `Receiver::recv()`](https://github.com/rust-lang/rust/issues/39364).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

154 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-09-17..2018-09-24

* [switch linker for `aarch64-pc-windows-msvc` from LLD to MSVC](https://github.com/rust-lang/rust/pull/54290)
* [remove (more) CAS API from Atomic* types where not natively supported](https://github.com/rust-lang/rust/pull/54280)
* [parser: tweak function parameter parsing to avoid rollback on succesfull path](https://github.com/rust-lang/rust/pull/54415)
* [improve handling of type bounds in `bit_set.rs`](https://github.com/rust-lang/rust/pull/54370)
* [use `HybridBitSet` in `SparseBitMatrix`](https://github.com/rust-lang/rust/pull/54318)
* [merge `bitvec.rs` and `indexed_set.rs`](https://github.com/rust-lang/rust/pull/54286)
* [split `Liveness::users` into three](https://github.com/rust-lang/rust/pull/54211)
* [compress `Liveness` data some more](https://github.com/rust-lang/rust/pull/54420)
* [NLL: deduplicate errors for incorrect move in loop](https://github.com/rust-lang/rust/pull/53995)
* [NLL: rework checking for borrows conflicting with drops](https://github.com/rust-lang/rust/pull/54509)
* [report when borrow could cause `&mut` aliasing during Drop](https://github.com/rust-lang/rust/pull/54310)
* [move `std::os::raw::c_void` into libcore and re-export in libstd](https://github.com/rust-lang/rust/pull/53910) and
  [Re-export `core::ffi::c_void` if it exists](https://github.com/rust-lang/libc/pull/1082) (RFC [#2521](https://github.com/rust-lang/rfcs/pull/2521))
* [make `rustc::middle::region::Scope`'s fields public](https://github.com/rust-lang/rust/pull/54260)
* [miri: correctly compute expected alignment for field](https://github.com/rust-lang/rust/pull/54298)
* [extend MIR inlining to all operand variants](https://github.com/rust-lang/rust/pull/54416)
* [std: check for overflow in `str::repeat`](https://github.com/rust-lang/rust/pull/54399)
* [switch wasm math symbols to their original names](https://github.com/rust-lang/rust/pull/54257)
* [update to a new pinning API](https://github.com/rust-lang/rust/pull/53877)
* [implement `[T]::copy_within`](https://github.com/rust-lang/rust/pull/53652)
* [implement `MaybeUninit`](https://github.com/rust-lang/rust/pull/53508)
* [`Duration` div mul extras](https://github.com/rust-lang/rust/pull/52813)
* [cargo: fix missing messages when --message-format=json is deeply nested](https://github.com/rust-lang/cargo/pull/6081)
* [cargo: fix incomplete conflict set backjump](https://github.com/rust-lang/cargo/pull/5988)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2438: Deny the `overflowing_literals` lint for the 2018 edition](https://github.com/rust-lang/rfcs/pull/2438).
* [RFC 2383: Lint Reasons](https://github.com/rust-lang/rfcs/pull/2383).
* [RFC 2437: Rustfmt stability](https://github.com/rust-lang/rfcs/pull/2437).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [The optimize attribute](https://github.com/rust-lang/rfcs/pull/2412).
* [disposition: merge] [Or patterns, i.e `Foo(Bar(x) | Baz(x))`](https://github.com/rust-lang/rfcs/pull/2535).
* [disposition: merge] [`#[cfg_attr]` expanding to multiple attributes](https://github.com/rust-lang/rfcs/pull/2539).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Impl From<NonZero<T>> for T](https://github.com/rust-lang/rust/pull/54240).
* [disposition: merge] [Tracking issue for eRFC 2497, "if- and while-let-chains, take 2", edition changes](https://github.com/rust-lang/rust/issues/53668).
* [disposition: merge] [Tracking issue for RFC 2296, `Option::replace`](https://github.com/rust-lang/rust/issues/51998).
* [disposition: close] [Tracking issue for channel selection](https://github.com/rust-lang/rust/issues/27800).

## New RFCs

* [Create Editorconfig File as Part of Cargo Project](https://github.com/rust-lang/rfcs/pull/2549).
* [Associated type lifetime elision](https://github.com/rust-lang/rfcs/pull/2548).

# Upcoming Events

### Online

* [Oct 3. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Oct 9. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Oct 10. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Oct 10. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Oct 2. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxnbdb/).

### Asia

* [Sep 30. Bangalore, IN - Diesel - A safe, extensible ORM and Query Builder](https://www.meetup.com/rustox/events/250769067/).
* [Oct 3. Kuala Lumpur, MY - Rust Lang Meetup - Project X](https://www.facebook.com/events/190938831689130/).

### Europe

* [Oct 1. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/BcnRust/events/254655075/).
* [Oct 3. Vilnius, LT - Vilnius Rust Meetup #3 - Network Simulation and WebAssembly](https://www.meetup.com/Rust-in-Vilnius/events/254403141/).
* [Oct 3. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/xkdlvpyxnbfb/).

### North America

* [Sep 30. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxmbnc/).
* [Oct 3. Indianopolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxnbfb/).
* [Oct 3. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxnbfb/).
* [Oct 3. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/dqldspyxnbfb/).
* [Oct 7. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnbkb/).
* [Oct 8. Seattle, US  - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxnblb/).
* [Oc 11. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxnbpb/).
* **[Oct 19 & 20. Ann Arbor, US - Rust Belt Rust 2018](https://rust-belt-rust.com/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Software Engineer at VMRay, Bochum, DE](https://careers.vmray.com/apply-software-engineer-rust/).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [RustBelt is looking for postdocs and PhD students](https://plv.mpi-sws.org/rustbelt/#positions).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust beginners worrying about lifetimes is like kids worrying about quicksand. Both turn out to be a non-issue in life.

– [frequentlywrong on r/rust](https://www.reddit.com/r/rust/comments/9i3xng/anyone_else_not_using_rust_until_nll/e6gsy90/).

Thanks to [pyfisch](https://users.rust-lang.org/t/twir-quote-of-the-week/328/562) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
