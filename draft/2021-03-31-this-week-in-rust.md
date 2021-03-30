Title: This Week in Rust 384
Number: 384
Date: 2021-03-31
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs
* [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)

### Papers and Research Projects
* [Video] [Safer Rust: Program Verification with Creusot](https://www.youtube.com/watch?v=BPt987BRdDw)

### Miscellaneous

# Crate of the Week

This week's crate is [tide-acme](https://github.com/http-rs/tide-acme), a crate for automatic HTTPS certificaion using Let's Encrypt for Tide.

Thanks to [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/894) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [dotenv-linter has many good first issues](https://github.com/dotenv-linter/dotenv-linter/issues/390)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

327 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-03-22..2021-03-29

* [coverage bug fixes and optimization support](https://github.com/rust-lang/rust/pull/83307)
* [ban custom inner attributes in expressions and statements](https://github.com/rust-lang/rust/pull/83488)
* [`GenericParam` does not need to be a HIR owner](https://github.com/rust-lang/rust/pull/83424)
* [remove assignments to ZST places instead of marking ZST return place as unused](https://github.com/rust-lang/rust/pull/83177)
* [run analyses before thir-tree dumps](https://github.com/rust-lang/rust/pull/83050)
* [import small cold functions](https://github.com/rust-lang/rust/pull/82980)
* [implement `feature(const_generics_defaults)`](https://github.com/rust-lang/rust/pull/75384)
* [stabilize `debug_non_exhaustive`](https://github.com/rust-lang/rust/pull/83041)
* [simplify encoder and decoder](https://github.com/rust-lang/rust/pull/83273)
* [remove (lots of) dead code](https://github.com/rust-lang/rust/pull/83185)
* [use `TrustedRandomAccess` for in-place iterators where possible](https://github.com/rust-lang/rust/pull/79846)
* [instruct LLVM that `binary_search` returns a valid index](https://github.com/rust-lang/rust/pull/81354)
* [make `NonNull::as_ref` (and friends) return refs with unbound lifetimes](https://github.com/rust-lang/rust/pull/80771)
* [add function `core::iter::zip`](https://github.com/rust-lang/rust/pull/82917)
* [revert reverting of stabilizing `integer::BITS`](https://github.com/rust-lang/rust/pull/82565)
* [generalize and inline `slice::fill` specializations](https://github.com/rust-lang/rust/pull/83245)
* [add `Result::into_err` where the Ok variant is the never type](https://github.com/rust-lang/rust/pull/83421)
* [remove `Option::`{`unwrap_none`, `expect_none`}](https://github.com/rust-lang/rust/pull/83349)
* [futures: add `AsyncSeekExt::stream_position`](https://github.com/rust-lang/futures-rs/pull/2380)
* [cargo: default macOS targets to unpacked debuginfo](https://github.com/rust-lang/cargo/pull/9298)
* [rustdoc: sidebar trait items order](https://github.com/rust-lang/rust/pull/83051)
* [docs.rs: stop displaying and serving authorship information](https://github.com/rust-lang/docs.rs/pull/1322)

## Rust Compiler Performance Triage

An overall busy but decent week for performance. While there were some performance regressions they were mostly small, and they were outnumbered by performance gains. Perhaps the most interesting news is not a compiler performance improvement but rather the introduction of no-alias optimizations at the LLVM level. This slightly hurts optimized build time performance in some cases, but it should make some workloads run faster after compilation.

Triage done by **@rylev**.
Revision range: [f24ce9b0..9b6339e4](https://perf.rust-lang.org/?start=f24ce9b0140d9be5a336954e878d0c1522966bb8&end=9b6339e4b9747d473270baa42e77e1d2fff39bf4&absolute=false&stat=instructions%3Au)

2 Regressions, 5 Improvements, 3 Mixed

1 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Adds must_not_suspend_lint RFC](https://github.com/rust-lang/rfcs/pull/3014)
* [adds async stream rfc](https://github.com/rust-lang/rfcs/pull/2996)
* [Make the authors field optional](https://github.com/rust-lang/rfcs/pull/3052)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Declarative macro metavariable expressions](https://github.com/rust-lang/rfcs/pull/3086)
* [RFC: Hidden trait implementations](https://github.com/rust-lang/rfcs/pull/2529)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize bufreader_seek_relative](https://github.com/rust-lang/rust/pull/82992)
* [disposition: merge] [Add IEEE 754 compliant fmt/parse of -0, infinity, NaN](https://github.com/rust-lang/rust/pull/78618)
* [disposition: merge] [Implement indexing slices with pairs of `core::ops::Bound<usize>`](https://github.com/rust-lang/rust/pull/77704)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [March 25, Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/276796209/).
* [March 30, Munich, DE - Rust Remote #6 - Rust Munich](https://www.meetup.com/rust-munich/events/276424952)
* [March 30, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccfbnc/)
* [April 1, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccgbcb/)
* [April 6, Buffalo, NY, US - Buffalo Rust User Group - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/276717867/)

### Asia Pacific

* [March 31, Wellington, NZ - Coffee - Rust Wellington](https://www.meetup.com/Rust-Wellington/events/277104604/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Despite all the negative aspects, I must say that I do generally really like the poll-based approach that Rust is taking. Most of the problems encountered are encountered not because of mistakes, but because no other language really has pushed this principle this far. Programming language design is first and foremost an “artistic” activity, not a technical one, and anticipating the consequences of design choices is almost impossible.

– [tomaka on medium](https://tomaka.medium.com/a-look-back-at-asynchronous-rust-d54d63934a1c)

Thanks to [Michael Howell](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1028) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
