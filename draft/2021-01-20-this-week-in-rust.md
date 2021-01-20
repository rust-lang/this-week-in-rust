Title: This Week in Rust 374
Number: 374
Date: 2021-01-20
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

### Miscellaneous

# Crate of the Week

This week's crate is [dotenv-linter](https://github.com/dotenv-linter/dotenv-linter), a lightning fast linter for `.env` files.

Thanks to [Grachev Mikhail](https://users.rust-lang.org/t/crate-of-the-week/2704/869) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ZcashFoundation/zebra - Create test to catch duplicate dependencies](https://github.com/ZcashFoundation/zebra/issues/1582)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

391 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-01-11..2021-01-18

* [use correct ABI for wasm32 by default](https://github.com/rust-lang/rust/pull/79998)
* [improve diagnostics when closure doesn't meet trait bound](https://github.com/rust-lang/rust/pull/80635)
* [enhance type inference errors involving the `?` operator](https://github.com/rust-lang/rust/pull/80517)
* [explain method-call move errors in loops](https://github.com/rust-lang/rust/pull/80324)
* [make CTFE able to check for undefined behavior](https://github.com/rust-lang/rust/pull/78407)
* [split a func into cold/hot parts, reducing rustc binary size](https://github.com/rust-lang/rust/pull/80042)
* [suggest `_` and `..` if a pattern has too few fields](https://github.com/rust-lang/rust/pull/80017)
* [suggest `async {}` for `async || {}`](https://github.com/rust-lang/rust/pull/76580)
* [do not suggest invalid code in pattern with loop](https://github.com/rust-lang/rust/pull/80941)
* [add allow-by-default lint on implicit ABI in extern function pointers and items](https://github.com/rust-lang/rust/pull/76219)
* [reintroduce `hir::ExprKind::If`](https://github.com/rust-lang/rust/pull/79328)
* [remove redundant `def_id` lookups](https://github.com/rust-lang/rust/pull/80232)
* [serialize incr comp structures to file via fixed-size buffer](https://github.com/rust-lang/rust/pull/80463)
* [turn type inhabitedness into a query to fix `exhaustive_patterns` perf](https://github.com/rust-lang/rust/pull/79670)
* [box `Item::Attributes`](https://github.com/rust-lang/rust/pull/80802)
* [resolve: simplify collection of traits in scope](https://github.com/rust-lang/rust/pull/80765)
* [use `Once` instead of `Mutex` to manage capture resolution](https://github.com/rust-lang/rust/pull/80736)
* [consistently avoid constructing optimized MIR when not doing codegen](https://github.com/rust-lang/rust/pull/80718)
* [add benchmark and fast path for `BufReader::read_exact`](https://github.com/rust-lang/rust/pull/80201)
* [add `MaybeUninit` method `array_assume_init`](https://github.com/rust-lang/rust/pull/80600)
* [change `BinaryHeap::append` rebuild heuristic](https://github.com/rust-lang/rust/pull/77435)
* [implement `ptr::write` without dedicated intrinsic](https://github.com/rust-lang/rust/pull/80290)
* [introduce {`Ref`, `RefMut`}`::try_map' for optional projections in `RefCell`](https://github.com/rust-lang/rust/pull/78455)
* [re-stabilize `Weak::as_ptr` and friends for unsized T](https://github.com/rust-lang/rust/pull/80764)
* [add `Iterator::intersperse_with`](https://github.com/rust-lang/rust/pull/80567)
* [`TrustedRandomAaccess` specialization composes incorrectly for nested `iter::Zips`](https://github.com/rust-lang/rust/pull/80670)
* [remove unreachable panics from `VecDeque::`{`front`/`back`}(`_mut`)]](https://github.com/rust-lang/rust/pull/80834)
* [add `NonZeroU`n`::is_power_of_two`](https://github.com/rust-lang/rust/pull/81107)
* [stabilize `split_inclusive`](https://github.com/rust-lang/rust/pull/77858)
* [stabilize the `poll_map` feature](https://github.com/rust-lang/rust/pull/80968)
* [add `as_rchunks` (and friends) to slices](https://github.com/rust-lang/rust/pull/78818)
* [hashbrown: hide allocator details and default to `Global`](https://github.com/rust-lang/hashbrown/pull/227)
* [hashbrown: export `AllocError` as well as `Allocator`](https://github.com/rust-lang/hashbrown/pull/223)
* [regex: implement `regex::Replacer` for `String`, `&String`, `Cow<'a, str>`, `&Cow<'a, str>`](https://github.com/rust-lang/regex/pull/728)
* [futures: fix type-inference in `sink::unfold()` by specifying more of its types](https://github.com/rust-lang/futures-rs/pull/2311)
* [clippy: roadmap for 2021](https://github.com/rust-lang/rust-clippy/pull/6462)

## Rust Compiler Performance Triage

* [2020-01-12](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-01-12.md):
1 Regressions, 2 Improvements, 3 Mixed
Overall, a positive albeit quiet week. The largest change came from the incremental compilation working group which delivered large gains in performance caused by [changes](https://github.com/rust-lang/rust/issues/76896) in how inlining is handled in debug mode. Unfortunately, these changes may be reversed due to concerns

Triage done by @rylev.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-01-12.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Serve crates-io registry over HTTP as static files](https://github.com/rust-lang/rfcs/pull/2789)
* [Infallible promotion](https://github.com/rust-lang/rfcs/pull/3027)
* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [RFC: Plan to make core and std's panic identical](https://github.com/rust-lang/rfcs/pull/3007)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Allow "artifact dependencies" on bin, cdylib, and staticlib crates](https://github.com/rust-lang/rfcs/pull/3028)
* [RFC: Pointer metadata & VTable](https://github.com/rust-lang/rfcs/pull/2580)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking Issue for `panic_any`](https://github.com/rust-lang/rust/issues/78500)
* [Tracking issue for stable SIMD in Rust](https://github.com/rust-lang/rust/issues/48556)

## New RFCs

* [Add the boxed!() macro to "de-magic" box syntax](https://github.com/rust-lang/rfcs/pull/3057)
* [try_trait_v2: A new design for the ? desugaring](https://github.com/rust-lang/rfcs/pull/3058)
* [Add language support for C-compatible bit-fields](https://github.com/rust-lang/rfcs/pull/3064)

# Upcoming Events

### Online
* [January 14, San Diego, CA, US - San Diego Rust January 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/275547915/)
* [January 20, Vancouver, BC, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/npqfbsycccbbc/)
* [January 21, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrycccbcc/)
* [January 26, Dallas, TX, US - Last Tuesay - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrycccbjc/)

### North America
* [January 14, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrycccbsb/)
* [January 14, Provo, UT, US - The Blue Pill: Rust on Microcontrollers (Jan / Third Round) - Utah Rust](https://www.meetup.com/utah-rust/events/268567961/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Why do I use the letter ‘o’ for my generic closure param name? [...] I recently realized that since Rust uses pipes to enclose a param block, using ‘o’ makes the block look like a TIE fighter. I am not a terribly serious person.

– [Tim Keating on medium](https://mrtact.medium.com/polishing-rust-30eeac3c4bf3)

Thanks to [Edoardo Morandi](https://users.rust-lang.org/t/twir-quote-of-the-week/328/990) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
