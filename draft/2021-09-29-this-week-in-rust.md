Title: This Week in Rust 410
Number: 410
Date: 2021-09-29
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Project/Tooling Updates

* [SixtyFPS (GUI crate) weekly update for 27th of September 2021](https://sixtyfps.io/thisweek/2021-09-27.html)
* [This week in Fluvio #6: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0006/)

### Observations/Thoughts

- [Common Newbie Mistakes and Bad PRactices in Rust: Bad Habits](https://adventures.michaelfbryan.com/posts/rust-best-practices/bad-habits/)

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [flowistry](https://github.com/willcrichton/flowistry), a VS code extension to visualize data flow in Rust code.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/963) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

256 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-09-13..2021-09-20

* [ARMv6K Nintendo 3DS Tier 3 target added](https://github.com/rust-lang/rust/pull/88529)
* [add initial support for m68k](https://github.com/rust-lang/rust/pull/88321)
* [disable the evaluation cache when in intercrate mode](https://github.com/rust-lang/rust/pull/88994)
* [use a separate interner type for `UniqueTypeId`](https://github.com/rust-lang/rust/pull/87867)
* [accept `m!{ .. }.method()` and `m!{ .. }?` statements](https://github.com/rust-lang/rust/pull/88690)
* [recover from `Foo(a: 1, b: 2)`](https://github.com/rust-lang/rust/pull/88729)
* [emit clearer diagnostics for parens around for loop heads](https://github.com/rust-lang/rust/pull/86422)
* [make diagnostics clearer for `?` operators](https://github.com/rust-lang/rust/pull/86382)
* [improve error message for type mismatch in generator arguments](https://github.com/rust-lang/rust/pull/88911)
* [improve error message for missing trait in trait impl](https://github.com/rust-lang/rust/pull/88894)
* [add a separate error for `dyn Trait` in `const fn`](https://github.com/rust-lang/rust/pull/89021)
* [suggest replacing an inexisting field for an unmentioned field](https://github.com/rust-lang/rust/pull/87960)
* [suggest better place to add call parentheses for method expressions wrapped in parentheses](https://github.com/rust-lang/rust/pull/89055)
* [suggest removing bad parens in `(recv.method)()`](https://github.com/rust-lang/rust/pull/88841)
* [suggest removing `#![feature]` for library features that have been stabilized](https://github.com/rust-lang/rust/pull/89012)
* [don't lint about missing code examples in derived traits](https://github.com/rust-lang/rust/pull/88735)
* [point at argument instead of call for their obligations](https://github.com/rust-lang/rust/pull/88719)
* [reuse existing shared `Lrc` for `MatchImpl` parent](https://github.com/rust-lang/rust/pull/89000)
* [fast reject for `NeedsNonConstDrop`](https://github.com/rust-lang/rust/pull/88965)
* [simplify lazy `DefPathHash` decoding by using an on-disk hash table](https://github.com/rust-lang/rust/pull/82183)
* [avoid codegen for `Result::into_ok` in `lang_start`](https://github.com/rust-lang/rust/pull/88988)
* [use `<[T; N]>::map` in `Sharded` instead of `SmallVec` and unsafe code](https://github.com/rust-lang/rust/pull/89069)
* [introduce a fast path that avoids the `debug_tuple` abstraction when deriving `Debug` for unit-like enum variants](https://github.com/rust-lang/rust/pull/88832)
* [make `UnsafeCell::get_mut` const](https://github.com/rust-lang/rust/pull/88722)
* [`const` `drop`](https://github.com/rust-lang/rust/pull/88558)
* [don't inline `OnceCell` initialization closures](https://github.com/rust-lang/rust/pull/89031)
* [allow `panic!("{}", computed_str)` in `const fn`](https://github.com/rust-lang/rust/pull/88954)
* [fix potential race in `AtomicU64` time monotonizer](https://github.com/rust-lang/rust/pull/89017)
* [futures: fix Unusable `Sink` implementation on `Scan`](https://github.com/rust-lang/futures-rs/pull/2499)
* [clippy: change `while_let_on_iterator` suggestion to use `by_ref()`](https://github.com/rust-lang/rust-clippy/pull/7690)
* [clippy: improve accuracy of `mut_key`](https://github.com/rust-lang/rust-clippy/pull/7640)
* [clippy: new lint: `same_name_method`](https://github.com/rust-lang/rust-clippy/pull/7653)

### Rust Compiler Performance Triage

A nice week: more improvements than regressions.

Triage done by **@pnkfelix**.
Revision range: [9f85cd6f2..7743c9f](https://perf.rust-lang.org/?start=9f85cd6f2ab2769c16e89dcdddb3e11d9736b351&end=7743c9fadd64886d537966ba224b9c20e6014a59&absolute=false&stat=instructions%3Au)

2 Regressions, 4 Improvements, 8 Mixed; ??? of them in rollups

44 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-09-21.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Supertrait item shadowing](https://github.com/rust-lang/rfcs/pull/2845)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)


* [disposition: merge] [Stabilize RFC 2345: Allow panicking in constants](https://github.com/rust-lang/rust/issues/89006)
* [disposition: merge] [Perform type inference in range pattern](https://github.com/rust-lang/rust/pull/88090)
* [disposition: close] [Disable unused_must_use for statically known bools](https://github.com/rust-lang/rust/pull/88028)
* [disposition: merge] [Decide whether asm! and/or global_asm! should be exported from the prelude.](https://github.com/rust-lang/rust/issues/87228)
* [disposition: merge] [Make #[derive(A, B, ...)] cfg-eval its input only for A, B, ... and stabilize feature(macro_attributes_in_derive_output)](https://github.com/rust-lang/rust/pull/87220)
* [disposition: merge] [Make `*const (), *mut ()` okay for FFI](https://github.com/rust-lang/rust/pull/84267)

### New RFCs

* [[Help wanted] First draft of patchfile RFC](https://github.com/rust-lang/rfcs/pull/3177)
* [Multiple artifact deps on the same crate with different names, for different targets](https://github.com/rust-lang/rfcs/pull/3176)

## Upcoming Events

### Online

* [September 28, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccmblc/)
* [October 2, 2021 - Rust Graphics meetup](https://github.com/gfx-rs/meetup)
* [October 5, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/280628523/)

### North America

* [October 13, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccnbrb/)

### Europe

* [September 22, 2021, Berlin, DE - Rust and Tell - an onsite event - Berline.rs](https://berline.rs/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Timescale**

* [Senior Toolkit Engineer - Database (Remote)](https://boards.greenhouse.io/timescale/jobs/5542785002)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> the strains of the project have hurt a lot of people over the years and I think maybe the only path to recovery involves getting some distance from it.

– [Graydon Hoare on twitter](https://twitter.com/graydon_pub/status/1437521319722029056)

Thanks to [mmmmib](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1107) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
