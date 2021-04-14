Title: This Week in Rust 385
Number: 385
Date: 2021-04-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No papers/research projects this week.

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts
* [podcast] [Building with Rust: Tim McNamara on Rust in Action](https://anchor.fm/building-with-rust/episodes/Building-with-Rust-Tim-McNamara-on-Rust-in-Action-eugoal/a-a1ptlh) [[transcript]](https://github.com/seanchen1991/building-with-rust/blob/main/transcripts/002.md)

### Rust Walkthroughs
* [Introducing easy-cast](https://kas-gui.github.io/blog/easy-cast.html)
* [ZH] [series] [Build GraphQL services based on Async Rust using tide + async-graphql + mongodb (基于 Async Rust 构建 GraphQL 服务，使用 tide + async-graphql + mongodb) - Part 1](https://blog.budshome.com/budshome/gou-jian-rust-yi-bu-graphql-fu-wu-:ji-yu-tide-+-async-graphql-+-mongodb(1)--qi-bu-ji-crate-xuan-ze)

### Miscellaneous

# Crate of the Week

This week's crate is [dipa](https://docs.rs/dipa), a crate to derive delta-encoding for Rust data structures.

Despite a lack of nominations, llogiq is very pleased with his choice.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

329 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-04-05..2021-04-12

* [allow specifying alignment for functions](https://github.com/rust-lang/rust/pull/81234)
* [fix handling of `--output-format json` flag](https://github.com/rust-lang/rust/pull/82497)
* [don't tell users to use a nightly flag on the stable channel](https://github.com/rust-lang/rust/pull/84055)
* [improve trait/impl method discrepancy errors](https://github.com/rust-lang/rust/pull/84014)
* [account for `ExprKind::Block` when suggesting .into() and deref](https://github.com/rust-lang/rust/pull/83952)
* [let `#[allow(unstable_name_collisions)]` work for things other than function](https://github.com/rust-lang/rust/pull/81922)
* [add `bad_asm_style` to `HardwiredLints`](https://github.com/rust-lang/rust/pull/84068)
* [improve debuginfo for closures and async functions on Windows MSVC](https://github.com/rust-lang/rust/pull/83941)
* [use `AnonConst` for `asm!` constants](https://github.com/rust-lang/rust/pull/83916)
* [add `FromIterator` and `IntoIterator` impls for `ThinVec`](https://github.com/rust-lang/rust/pull/83821)
* [add `strong_count` mutation methods to `Rc`](https://github.com/rust-lang/rust/pull/83476)
* [use a `SmallVec` in `impl_or_trait_item`](https://github.com/rust-lang/rust/pull/83932)
* [use `FromStr` trait for number option parsing](https://github.com/rust-lang/rust/pull/82483)
* [reduce threads spawned by ui-tests](https://github.com/rust-lang/rust/pull/81942)
* [core: disable `ptr::swap_nonoverlapping_one`'s block optimization on SPIR-V](https://github.com/rust-lang/rust/pull/83019)
* [stabilize `cmp_min_max_by`](https://github.com/rust-lang/rust/pull/81047)
* [stabilize `peekable_peek_mut`](https://github.com/rust-lang/rust/pull/81938)
* [stabilize `duration_saturating_ops`](https://github.com/rust-lang/rust/pull/84090)
* [stabilize `atomic_fetch_update` methods on `AtomicBool` and `AtomicPtr`](https://github.com/rust-lang/rust/pull/84085)
* [fix `HashMap`/`HashSet` LLDB pretty-printer after hashbrown](https://github.com/rust-lang/rust/pull/83920)
* [futures: move `try_fold`, `try_for_each`, and `try_for_each_concurrent` to `StreamExt`](https://github.com/rust-lang/futures-rs/pull/2342)
* [futures: add `stream::Peekable::`{`next_if`, `next_if_eq`}](https://github.com/rust-lang/futures-rs/pull/2379)
* [fix perf regression in `rustdoc::bare_urls`](https://github.com/rust-lang/rust/pull/84034)
* [rustdoc: cleanup handling of associated items for intra-doc links](https://github.com/rust-lang/rust/pull/83849)
* [rustdoc: sort search index items for compression](https://github.com/rust-lang/rust/pull/83835)
* [rustdoc: store intra-doc links in Cache instead of on items directly](https://github.com/rust-lang/rust/pull/83833)
* [rustdoc: use `ThinVec` in a few places](https://github.com/rust-lang/rust/pull/83828)
* [rustdoc: merge idents when generating source content](https://github.com/rust-lang/rust/pull/83992)
* [clippy: fix false positive in `single_component_path_imports` lint](https://github.com/rust-lang/rust-clippy/pull/6905)
* [clippy: fix `explicit_into_iter_loop`](https://github.com/rust-lang/rust-clippy/pull/6982)
* [clippy: consider mutability on `useless_vec` suggestions](https://github.com/rust-lang/rust-clippy/pull/7036)
* [clippy: fix `missing_panics_doc` not detecting `assert_eq!` and `assert_ne!`](https://github.com/rust-lang/rust-clippy/pull/7029)

## Rust Compiler Performance Triage

A pretty major week for [memory usage improvements] with an average of ~20% gains on memory usage for
release builds, and 5% on check builds, due to an update in the default allocator
used (to a more recent jemalloc). Wall time performance remained largely unchanged over this week.

Triage done by **@simulacrum**.
Revision range: [4896450e..d32238](https://perf.rust-lang.org/?start=4896450e7e0a522486b4d3a8d360ac4e1d2072a0&end=d32238532138485c80db4f2cd596372bce214e00&absolute=false&stat=instructions%3Au)

[memory usage improvements]: https://perf.rust-lang.org/?start=4896450e7e0a522486b4d3a8d360ac4e1d2072a0&end=d32238532138485c80db4f2cd596372bce214e00&absolute=false&stat=max-rss

1 Regressions, 4 Improvements, 0 Mixed

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Declarative macro metavariable expressions](https://github.com/rust-lang/rfcs/pull/3086)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [RFC: Structural Records](https://github.com/rust-lang/rfcs/pull/2584)
* [disposition: postpone] [Hygiene opt-out (escaping) for declarative macros 2.0](https://github.com/rust-lang/rfcs/pull/2498)
* [disposition: postpone] [RFC: Delegation](https://github.com/rust-lang/rfcs/pull/2393)
* [disposition: close] [RFC: `#[derive_no_bound(..)]` and `#[derive_field_bound(..)]`](https://github.com/rust-lang/rfcs/pull/2353)
* [disposition: postpone] [RFC: Eager Macro Expansion](https://github.com/rust-lang/rfcs/pull/2320)
* [disposition: merge] [try_trait_v2: A new design for the `?` desugaring](https://github.com/rust-lang/rfcs/pull/3058)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Remove `T: Debug` bound on UnsafeCell Debug impl](https://github.com/rust-lang/rust/pull/83707)
* [disposition: merge] [Turn old edition lint (anonymous-parameters) into warn-by-default on 2015](https://github.com/rust-lang/rust/pull/82918)
* [disposition: merge] [Stabilize `rustdoc::bare_urls` lint](https://github.com/rust-lang/rust/pull/81764)
* [disposition: merge] [Tracking issue: fNN::is_subnormal](https://github.com/rust-lang/rust/issues/79288)
* [disposition: merge] [Tracking Issue for feature(nonzero_leading_trailing_zeros)](https://github.com/rust-lang/rust/issues/79143)
* [disposition: merge] [Tracking Issue for `{BTreeMap,BTreeSet}::retain`](https://github.com/rust-lang/rust/issues/79025)
* [disposition: merge] [Tracking Issue for `#![feature(const_cell_into_inner)]`](https://github.com/rust-lang/rust/issues/78729)
* [disposition: merge] [Tracking Issue for `atomic_fetch_update`](https://github.com/rust-lang/rust/issues/78639)
* [disposition: merge] [Tracking Issue for feature: "option_insert"](https://github.com/rust-lang/rust/issues/78271)
* [disposition: merge] [Tracking Issue for `Duration` saturating operations](https://github.com/rust-lang/rust/issues/76416)
* [disposition: merge] [Tracking Issue for `Duration::{zero, is_zero} (#![feature(duration_zero)])`](https://github.com/rust-lang/rust/issues/73544)
* [disposition: close] [Tracking issue for FixedSizeArray trait](https://github.com/rust-lang/rust/issues/27778)

## New RFCs

* [RFC: Reserved prefixes in the 2021 edition](https://github.com/rust-lang/rfcs/pull/3101)

# Upcoming Events

### Online
* [April 7, Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/277133126/)
* [April 7, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/jhfstryccgbkb/)
* [April 12, Denver, CO, US - Building Delightful CLI Tools in Rust by Chuck Pierce - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/276801410/)
* [April 13, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccgbrb/)
* [April 13, Saarbrücken, Saarland, DE - **Rust Saar** 10u16](https://www.meetup.com/de-DE/Rust-Saar/events/276873622/)
* [April 20, Washington, DC, US - The Rust Borrow Checker—A Deep Dive - Rust DC](https://www.meetup.com/RustDC/events/ntvrgsyccgblb)

### North America

* [April 8, Columbus, OH, US - Monthly Meetup - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccgblb/)
* [April 14, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgryccgbsb/)

### Asia Pacific
* [April 19, Wellington, NZ - IGNITION: What is Rust and why should I care? Rust at work & at play - Rust Wellington](https://www.meetup.com/Rust-Wellington/events/277270667)

### Europe
* [April 21, Moscow, Russia - Monthly Meetup - Rust Moscow](https://www.meetup.com/ru-RU/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/277259838/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> What I actually value on a daily basis in \[rust is\]
> I can call code written by other people without unpleasant surprises.
>
> ```
> async fn verify_signature(token: &Jwt) -> Result<Claims, VerificationError>
> ```
>
> Looking at a code snippet:
> 
> * I know my JWT token won't be mutated, just accessed ( `&` );
> * I know the function will probably perform some kind of I/O ( `async` );
> * I know that the function might fail ( `Result` );
> * I know its failure modes ( `VerificationError` ).

– [Luca Palmieri on Twitter](https://twitter.com/algo_luca/status/1380928103019597827)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1031) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
