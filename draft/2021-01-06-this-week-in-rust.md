Title: This Week in Rust 372
Number: 372
Date: 2020-01-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official

### Project/Tooling Updates
[napi 1.0 released](https://napi.rs)

* [Insta got a new website with docs](https://insta.rs/) and a [Visual Studio Code Extension](https://marketplace.visualstudio.com/items?itemName=mitsuhiko.insta)

### Observations/Thoughts
 * [Reflecting on developing a database (2020 edition)](https://alex-dukhno.github.io/2020-12-31-Reflecting-on-developing-a-database-(2020-edition)/)

* [bore(1) + nonymous: lessons learned writing a DNS query tool and `#![no_std]` DNS library](https://www.azabani.com/2021/01/03/nonymous-bore.html)

### Rust Walkthroughs

### Miscellaneous

# Crate of the Week

This week's crate is [autograd](https://github.com/raskr/rust-autograd), a library of differentiable operations and tensors for machine learning applications.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/864) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

275 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-12-21..2020-12-28

* [stabilize `min_const_generics`](https://github.com/rust-lang/rust/pull/79135) (Huzzah!)
* [utilize PGO for rustc linux dist builds](https://github.com/rust-lang/rust/pull/80262)
* [suggest fn ptr rather than fn item and suggest to use `Fn` trait bounds rather than the unique closure type](https://github.com/rust-lang/rust/pull/80284)
* [implement a compiler diagnostic for move async mistake](https://github.com/rust-lang/rust/pull/80160)
* [highlight edition-specific keywords correctly in code blocks, accounting for code block edition modifiers](https://github.com/rust-lang/rust/pull/80226)
* [prevent caching normalization results with a cycle](https://github.com/rust-lang/rust/pull/80246)
* [fix ICE when lookup method in trait for type that have bound vars](https://github.com/rust-lang/rust/pull/80170)
* [remove `DefPath` from `Visibility` and calculate it on demand](https://github.com/rust-lang/rust/pull/80099)
* [`rustc_query_system : reduce dependency graph memory usage](https://github.com/rust-lang/rust/pull/79589)
* [add `impl Div<NonZeroU'*`> for u`* which cannot panic](https://github.com/rust-lang/rust/pull/79134)
* [deprecate atomic `compare_and_swap` method](https://github.com/rust-lang/rust/pull/79261)
* [stabilize `core::slice::fill`](https://github.com/rust-lang/rust/pull/79213)
* [stabilize `deque_range`](https://github.com/rust-lang/rust/pull/79022)
* [use `clone_from` from `hashbrown::`{`HashMap`, `HashSet`}](https://github.com/rust-lang/rust/pull/80400)
* [futures: perf: pack the state and future of unfolds in the same memory](https://github.com/rust-lang/futures-rs/pull/2283)
* [cargo: stabilize `RUSTC_WORKSPACE_WRAPPER`](https://github.com/rust-lang/cargo/pull/8976)
* [rustdoc: stabilise `--default-theme` command line option](https://github.com/rust-lang/rust/pull/79642)

## Rust Compiler Performance Triage

* [2020-12-24](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-24.md):
3 Regressions, 5 Improvements
Major event this week is landing PGO for rustc (on x86_64-unknown-linux-gnu). We
expect other platforms to follow but further investigation will be needed,
especially for cross-compiled platforms. We expect to add LLVM PGO as well.

Triage done by @simulacrum.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-24.md) for more.

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

* [Stabilize slice::strip_prefix and slice::strip_suffix](https://github.com/rust-lang/rust/pull/77853)
* [Tracking issue for stable SIMD in Rust](https://github.com/rust-lang/rust/issues/48556)

## New RFCs

* [New trait: core::convert::IntoUnderlying](https://github.com/rust-lang/rfcs/pull/3046)
* [#[target_feature(..)] In Trait Methods](https://github.com/rust-lang/rfcs/pull/3042)

# Upcoming Events

### Online
* [January 5, Buffalo, NY, US - Buffalo Rust User Group](https://www.meetup.com/Buffalo-Rust-Meetup/events/274936687/)
* [January 6, Johannesburg, ZA - Monthly Joburg Rust Chat - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/275424876/)
* [January 6, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/jhfstrycccbjb/)
* [January 7, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrycccbkb/)
* [January 12, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrycccbqb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is a common theme in Rust’s design: To reduce breakage as code evolves, you’re only allowed to rely on features that have been intentionally declared by the author.

– [2e71828 on rust-users](https://users.rust-lang.org/t/why-explicit-const-fn-token-needed/53006/2)

Thanks to [Kornel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/980) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
