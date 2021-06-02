Title: This Week in Rust 394
Number: 394
Date: 2021-06-09
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

No official blog posts or research papers this week.

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [rust-codegen-gcc](https://github.com/antoyo/rustc_codegen_gcc), a drop-in replacement for the LLVM-based rust compiler backend targetting GCC.

Thanks to [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/920) for the nomination

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [Backroll-rs is looking for contributors](https://www.reddit.com/r/rust/comments/npnl1p/help_wanted_with_backrollrs_new_networking_library/)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

255 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-05-24..2021-05-31

* [post-monomorphization errors traces MVP](https://github.com/rust-lang/rust/pull/85633)
* [make closures inherit their parent's "safety context"](https://github.com/rust-lang/rust/pull/85607)
* [fix low-memory issue and lower tier platforms with no sysinfo](https://github.com/rust-lang/rustup/pull/2779)
* [fix bootstrap using host exe suffix for cargo](https://github.com/rust-lang/rust/pull/85590)
* [const-eval: disallow unwinding across functions that !fn_can_unwind()](https://github.com/rust-lang/rust/pull/85546)
* [deal with const_evaluatable_checked in ConstEquate](https://github.com/rust-lang/rust/pull/85481)
* [disallow shadowing const parameters](https://github.com/rust-lang/rust/pull/85478)
* [optimize proc macro bridge](https://github.com/rust-lang/rust/pull/85390)
* [fix incorrect suggestions for E0605](https://github.com/rust-lang/rust/pull/84968)
* [stabilize member constraints](https://github.com/rust-lang/rust/pull/84701)
* [E0599 suggestions and elision of generic argument if no canditate is found](https://github.com/rust-lang/rust/pull/84221)
* [a bit more polish on const eval errors](https://github.com/rust-lang/rust/pull/85767)
* [merge CrateDisambiguator into StableCrateId](https://github.com/rust-lang/rust/pull/85804)
* [do not try to build LLVM with Zlib on Windows](https://github.com/rust-lang/rust/pull/85762)
* [use u64 for the GroupWord on WebAssembly](https://github.com/rust-lang/hashbrown/pull/271)
* [don't hash `thir_body`](https://github.com/rust-lang/rust/pull/85729)
* [emit a hard error when a panic occurs during const-eval](https://github.com/rust-lang/rust/pull/85704)
* [don't sort a Vec before computing its DepTrackingHash](https://github.com/rust-lang/rust/pull/85702)
* [demote `ControlFlow::`{`from`, `into`}`_try` to `pub(crate)`](https://github.com/rust-lang/rust/pull/85645)
* [remove `Ipv6Addr::is_unicast_link_local_strict`](https://github.com/rust-lang/rust/pull/85819)
* [make `Step` trait safe to implement](https://github.com/rust-lang/rust/pull/83772)
* [fix unsoundness of `Debug` implementation for `linked_list::IterMut`](https://github.com/rust-lang/rust/pull/85814)
* [`Weak`'s type parameter may dangle on `drop`](https://github.com/rust-lang/rust/pull/85535)
* [add `TrustedRandomAccess` specialization for `Vec::extend()`](https://github.com/rust-lang/rust/pull/83770)
* [enable `Vec`'s calloc optimization for `Option<NonZero>`](https://github.com/rust-lang/rust/pull/85737)
* [prevent double `drop` in `Vec::dedup_by` if a destructor panics](https://github.com/rust-lang/rust/pull/85625)
* [fix pointer provenance in `<[T]>::copy_within`](https://github.com/rust-lang/rust/pull/85610)
* [add `String::extend_from_within`](https://github.com/rust-lang/rust/pull/85801)
* [add `inline` attr to `CString::into_inner` so it can optimize out `NonNull` checks](https://github.com/rust-lang/rust/pull/85719)
* [hashbrown: guard against allocations exceeding `isize::MAX`](https://github.com/rust-lang/hashbrown/pull/268)
* [futures: allow no limit for buffered stream combinators](https://github.com/rust-lang/futures-rs/pull/2429)
* [cargo: `cargo tree -e no-proc-macro` to hide procedural macro dependencies](https://github.com/rust-lang/cargo/pull/9488)
* [rustup: bring back `x86_64-sun-solaris` target to rustup](https://github.com/rust-lang/rust/pull/85252)
* [clippy: add `avoid_breaking_exported_api` config option](https://github.com/rust-lang/rust-clippy/pull/7187)
* [clippy: add lint `suspicious_splitn`](https://github.com/rust-lang/rust-clippy/pull/7292)
* [clippy: move `semicolon_if_nothing_returned` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/7268)
* [clippy: improve message for `not_unsafe_ptr_arg_deref` lint](https://github.com/rust-lang/rust-clippy/pull/7294)
* [clippy: fix ICE in `too_many_lines`](https://github.com/rust-lang/rust-clippy/pull/7287)
* [clippy: fix `allow` on some statement lints](https://github.com/rust-lang/rust-clippy/pull/7282)
* [clippy: fix `missing_docs_in_private_items` false negative](https://github.com/rust-lang/rust-clippy/pull/7281)
* [clippy: add the ability to invalidate caches to force metadata collection](https://github.com/rust-lang/rust-clippy/pull/7256)

### Rust Compiler Performance Triage

Busy week, with several reverted PRs due to performance regressions, but overall a positive week.

Triage done by **@simulacrum**.
Revision range: [cdbe288..1160cf8](https://perf.rust-lang.org/?start=cdbe2888979bb8797b05f0d58a6f6e60753983d2&end=1160cf864f2a0014e3442367e1b96496bfbeadf4&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 5 Mixed

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-01.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: 2021 Edition](https://github.com/rust-lang/rfcs/pull/3085)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Supertrait item shadowing](https://github.com/rust-lang/rfcs/pull/2845)
* [disposition: merge] [Type-changing struct update syntax](https://github.com/rust-lang/rfcs/pull/2528)
* [disposition: merge] [RFC: Introduce concat_bytes!() to join [u8] and byte str analogous to concat! for str](https://github.com/rust-lang/rfcs/pull/2509)
* [disposition: merge] [RFC: Overconstraining and omitting unsafe in impls of unsafe trait methods](https://github.com/rust-lang/rfcs/pull/2316)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Re-add support for parsing (and pretty-printing) inner-attributes in match body](https://github.com/rust-lang/rust/pull/85193)

### New RFCs

* [Switch from travis to github actions.](https://github.com/rust-lang/rfcs/pull/3136)

## Upcoming Events

### Online

* [June 8, 2021, Seattle, WA, US - Monthly meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccjblb/)
* [June 10, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [June 16, 2021, Vancouver, BC, US - Rust in Mozilla's Data Platform - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/fqpkjsyccjbvb/)
* [June 17, 2021, Denver, CO, US - Learning Rust as a Python/Javascript developer by Juhis - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/277575285/)

### North America

* [June 9, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccjbmb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Ok, you wanted it. Let's go full meta:

> I recently graduated with my Ph.D., after having worked on 5 different versions of my simulator, written in 4 different languages. The last version, written in pure, safe rust, worked correctly in part because of rust's strong guarantees about what 'safety' means, which I was able to leverage to turn what would normally be runtime errors into compile time errors. That let me catch errors that would normally be days or weeks of debugging into relatively simple corrections. \[...\] So, once again, thank you to everyone!

– [Cem Karan on rust-internals](https://internals.rust-lang.org/t/ot-thank-you-to-everyone-that-has-made-rust-possible/14777)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1053) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
