Title: This Week in Rust 346
Number: 346
Date: 2020-07-07
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

* [Boa release v0.9 and make use of Rust's measureme](https://boa-dev.github.io/2020/07/03/boa-release-09.html)

# Crate of the Week

This week's crate is [print_bytes](https://crates.io/crates/print_bytes), a library to print arbitrary bytes to a stream as losslessly as possible.

Thanks to [dylni](https://users.rust-lang.org/t/crate-of-the-week/2704/784) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

339 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-06-22..2020-06-29

* [move leak-check to during coherence, candidate eval](https://github.com/rust-lang/rust/pull/72493)
* [account for multiple impl/dyn Trait in return type when suggesting `'_`](https://github.com/rust-lang/rust/pull/73496)
* [tweak binop errors](https://github.com/rust-lang/rust/pull/73674)
* [adds a clearer message for when the async keyword is missing from a function](https://github.com/rust-lang/rust/pull/73672)
* [allow dynamic linking for iOS/tvOS targets](https://github.com/rust-lang/rust/pull/73516)
* [always capture tokens for `macro_rules!` arguments](https://github.com/rust-lang/rust/pull/73293)
* [change heuristic for determining range literal](https://github.com/rust-lang/rust/pull/73639)
* [check for assignments between non-conflicting generator saved locals](https://github.com/rust-lang/rust/pull/73244)
* [const prop: erase all block-only locals at the end of every block](https://github.com/rust-lang/rust/pull/73757)
* [emit line info for generator variants](https://github.com/rust-lang/rust/pull/73460)
* [explain move errors that occur due to method calls involving `self`](https://github.com/rust-lang/rust/pull/73708)
* [fix handling of reserved registers for ARM inline asm](https://github.com/rust-lang/rust/pull/73588)
* [improve compiler error message for wrong generic parameter order](https://github.com/rust-lang/rust/pull/72271)
* [point at the call span when overflow occurs during monomorphization](https://github.com/rust-lang/rust/pull/73601)
* [provide suggestions for some moved value errors](https://github.com/rust-lang/rust/pull/73534)
* [self contained linking option](https://github.com/rust-lang/rust/pull/72738)
* [perform obligation deduplication to avoid buggy `ExistentialMismatch`](https://github.com/rust-lang/rust/pull/73485)
* [show the values and computation that would overflow a const evaluation or propagation](https://github.com/rust-lang/rust/pull/73513)
* [stabilize `#![feature(const_if_match)]` and `#![feature(const_loop)]`](https://github.com/rust-lang/rust/pull/72437)
* [A way forward for pointer equality in const eval](https://github.com/rust-lang/rust/pull/73398)
* [the const propagator cannot trace references](https://github.com/rust-lang/rust/pull/73613)
* [warn if linking to a private item](https://github.com/rust-lang/rust/pull/72771)
* [`improper_ctypes_definitions` lint](https://github.com/rust-lang/rust/pull/72700)
* [add Windows system error codes that should map to io::ErrorKind::TimedOut](https://github.com/rust-lang/rust/pull/71756)
* [errors: use `-Z terminal-width` in JSON emitter](https://github.com/rust-lang/rust/pull/73763)
* [proc_macro: stop flattening groups with dummy spans](https://github.com/rust-lang/rust/pull/73102)
* [rustc_lint: only query `typeck_tables_of` when a lint needs it](https://github.com/rust-lang/rust/pull/73743)
* [rustdoc: fix doc aliases with crate filtering](https://github.com/rust-lang/rust/pull/73644)
* [chalk: .chalk file syntax writer](https://github.com/rust-lang/chalk/pull/430)
* [chalk: add method to get repr data of an ADT to ChalkDatabase](https://github.com/rust-lang/chalk/pull/523)
* [chalk: fix built-in `Fn` impls when generics are involved](https://github.com/rust-lang/chalk/pull/541)
* [chalk: fix coherence issue with associated types in generic bound](https://github.com/rust-lang/chalk/pull/538)
* [miri: implement rwlocks on Windows](https://github.com/rust-lang/miri/pull/1461)
* [miri: supply our own implementation of the CTFE pointer comparison intrinsics](https://github.com/rust-lang/miri/pull/1459)
* [shortcuts for min/max on ordinary BTreeMap/BTreeSet iterators](https://github.com/rust-lang/rust/pull/73627)
* [add `TryFrom<{int}>` for `NonZero{int}`](https://github.com/rust-lang/rust/pull/72717)
* [add a fast path for `std::thread::panicking`.](https://github.com/rust-lang/rust/pull/72617)
* [add `[T]::partition_point`](https://github.com/rust-lang/rust/pull/73577)
* [add unstable `core::mem::variant_count` intrinsic](https://github.com/rust-lang/rust/pull/73418)
* [added io forwarding methods to the stdio structs](https://github.com/rust-lang/rust/pull/72705)
* [stabilize `leading_trailing_ones`](https://github.com/rust-lang/rust/pull/73032)
* [`impl PartialEq<Vec<B>> for &[A], &mut [A]`](https://github.com/rust-lang/rust/pull/71660)
* [forward `Hash::write_iN` to `Hash::write_uN`](https://github.com/rust-lang/rust/pull/73800)
* [libc: add ancillary socket data accessor functions for solarish OSes](https://github.com/rust-lang/libc/pull/1792)
* [libc: FreeBSD: machine register structs](https://github.com/rust-lang/libc/pull/1791)
* [libc: add wexecv, wexecve, wexecvp, wexecvpe](https://github.com/rust-lang/libc/pull/1796)
* [cargo: add support for `workspace.metadata` table](https://github.com/rust-lang/cargo/pull/8323)
* [cargo: adding environment variable CARGO_PKG_LICENSE_FILE](https://github.com/rust-lang/cargo/pull/8387)
* [cargo: enable "--target-dir" in "cargo install"](https://github.com/rust-lang/cargo/pull/8391)
* [cargo: expose built cdylib artifacts in the Compilation structure](https://github.com/rust-lang/cargo/pull/8418)
* [cargo: improve support for non-`master` main branches ](https://github.com/rust-lang/cargo/pull/8364)
* [docs.rs: don't panic when a crate doesn't exist for target-redirect](https://github.com/rust-lang/docs.rs/pull/859)
* [docs.rs: improve executing tests](https://github.com/rust-lang/docs.rs/pull/861)
* [clippy: lint iterator.map(|x| x)](https://github.com/rust-lang/rust-clippy/pull/5694)
* [clippy: new lint: suggest `ptr::read` instead of `mem::replace(..., uninitialized())`](https://github.com/rust-lang/rust-clippy/pull/5695)
* [clippy: clippy-driver: pass all args to rustc if --rustc is present](https://github.com/rust-lang/rust-clippy/pull/5178)
* [clippy: cmp_owned: handle when PartialEq is not implemented symmetrically](https://github.com/rust-lang/rust-clippy/pull/5701)
* [rustfmt: do not reorder module declaration with #![macro_use]](https://github.com/rust-lang/rustfmt/pull/4284)
* [rustfmt: don't reformat with errors unless --force flag supplied](https://github.com/rust-lang/rustfmt/pull/4256)

## Rust Compiler Performance Triage

* [2020-06-30](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-30). Three regressions, two of them on rollups; two improvements, one on a rollup.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Deduplicate Cargo workspace information](https://github.com/rust-lang/rfcs/pull/2906)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Inline `const` expressions and patterns](https://github.com/rust-lang/rfcs/pull/2920)
* [Inline assembly](https://github.com/rust-lang/rfcs/pull/2873)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [impl `From<char>` for String](https://github.com/rust-lang/rust/pull/73466)
* [disposition: merge] [mv std libs to std/](https://github.com/rust-lang/rust/pull/73265)
* [disposition: merge] [Stabilize `transmute` in constants and statics but not const fn](https://github.com/rust-lang/rust/pull/72920)
* [disposition: merge] [added `.collect()` into String from `Box<str>`](https://github.com/rust-lang/rust/pull/72688)
* [disposition: merge] [Stabilize const_type_id feature](https://github.com/rust-lang/rust/pull/72488)

## New RFCs

* [Linking modifiers for native libraries](https://github.com/rust-lang/rfcs/pull/2951)
* [Hierarchic anonymous life-time](https://github.com/rust-lang/rfcs/pull/2949)
* [Portable packed SIMD vector type](https://github.com/rust-lang/rfcs/pull/2948)
* [crates.io token scopes](https://github.com/rust-lang/rfcs/pull/2947)

# Upcoming Events

### Online
* [June 30. Berlin, DE - Remote - Berlin Rust - Rust and Tell](https://berline.rs/)
* [July 1. Johannesburg, ZA - Remote - Monthly Joburg Rust Chat!](https://www.meetup.com/Johannesburg-Rust-Meetup/events/271286846/)
* [July 1. Dublin, IE - Remote - Rust Dublin - July Remote Meetup](https://www.meetup.com/Rust-Dublin/events/271417290/)
* [July 1. Indianapolis, IN, US - Indy Rust - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybckbcb/)
* [July 13. Seattle, WA, US - Seattle Rust Meetup - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybckbsb/)

### North America
* [June 30. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybcjbnc/)
* [July 8. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybckblb/)
* [July 9. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/)

### Asia Pacific
* [July 6. Auckland, NZ - Rust AKL](https://www.meetup.com/rust-akl/events/266876691/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> References are a sharp tool and there are roughly three different approaches to sharp tools.
>
> 1. Don't give programmers sharp tools. They may make mistakes and cut their fingers off. *This is the Java/Python/Perl/Ruby/PHP... approach.*
> 2. Give programmers all the sharp tools they want. They are professionals and if they cut their fingers off it's their own fault. *This is the C/C++ approach.*
> 3. Give programmers sharp tools, but put guards on them so they can't accidentally cut their fingers off. *This is Rust's approach.*
>
> Lifetime annotations are a safety guard on references. Rust's references have no sychronization and no reference counting -- that's what makes them sharp. References in category-1 languages (which typically *do* have synchronization and reference counting) are "blunted": they're not really *quite* as effective as category-2 and -3 references, but they don't cut you, and they still work; they might just slow you down a bit.
>
> So, frankly, I like lifetime annotations because they prevent me from cutting my fingers off.

– [trentj on rust-users](https://users.rust-lang.org/t/when-do-you-find-lifetime-annotations-helpful/44434/6)

Thanks to [Ivan Tham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/897) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/hisn3e/this_week_in_rust_345/)</small>
