Title: This Week in Rust 415
Number: 415
Date: 2021-10-28
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

### Foundation

### Project/Tooling Updates

* [SixtyFPS (GUI crate): Changelog for 31th of October 2021](https://sixtyfps.io/thisweek/2021-11-01.html)
* [MirChecker: Detecting Bugs in Rust Programs via Static Analysis](https://mssun.me/research/ccs21mirchecker.html)

### Observations/Thoughts

### Rust Walkthroughs

* [I'm learning Rust (video) - Setup & fundamentals](https://youtu.be/K2oHkucybNs)
* [An Intro to the Rust Programming Language](https://acv.engineering/posts/an-intro-to-the-rust-programming-language/)

### Miscellaneous

* [Rust Editor Experience Survey](https://www.guidedtrack.com/programs/kfqw8vs/run)
* [Streaming the Reddit API using Fluvio's WASM ArrayMap](https://www.infinyon.com/blog/2021/10/smartstream-array-map-reddit/)
* [video] [IT] [Introduzione a Rust](https://video.linux.it/videos/watch/01b44259-e257-4bad-9685-8c91735aa08b)

## Crate of the Week

This week's crate is [cargo-crev](https://web.crev.dev/rust-reviews/), a tool to distribute security review of the crates we all depend on.

As there was no suggestion this week, llogiq is pretty thankful to himself for choosing this crate.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

353 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-10-04..2021-10-11

* [implement `split_array` and `split_array_mut`](https://github.com/rust-lang/rust/pull/83233)
* [normalize MIR with `RevealAll` before optimizations.](https://github.com/rust-lang/rust/pull/85254)
* [automatic exponential formatting of `f`{`32`, `64`} in `Debug`](https://github.com/rust-lang/rust/pull/86479)
* [reject octal zeros in `IPv4` addresses](https://github.com/rust-lang/rust/pull/86984)
* [add support for artifact size profiling](https://github.com/rust-lang/rust/pull/87404)
* [remove unnecessary condition in `Barrier::wait()`](https://github.com/rust-lang/rust/pull/87440)
* [make all proc-macro back-compat lints deny-by-default](https://github.com/rust-lang/rust/pull/88041)
* [stabilise `unix_process_wait_more`, extra `ExitStatusExt` methods](https://github.com/rust-lang/rust/pull/88300)
* [stabilize feature `saturating_div` for rust 1.58.0](https://github.com/rust-lang/rust/pull/88624)
* [remove unnecessary bound on `Zip' specialization impl](https://github.com/rust-lang/rust/pull/88789)
* [deduplicate `panic_fmt`](https://github.com/rust-lang/rust/pull/88860)
* [give better error for `macro_rules name`](https://github.com/rust-lang/rust/pull/89257)
* [stabilize `CString::from_vec_with_nul`(`_unchecked`)](https://github.com/rust-lang/rust/pull/89292)
* [point at overlapping impls when type annotations are needed](https://github.com/rust-lang/rust/pull/89427)
* [add rustc lint, warning when iterating over hashmaps](https://github.com/rust-lang/rust/pull/89558)
* [consider types appearing in const expressions to be invariant](https://github.com/rust-lang/rust/pull/89829)
* [use the "nice E0277 errors" for `!Send` `impl Future` from foreign crate](https://github.com/rust-lang/rust/pull/89889)
* [don't mark `for` loop iter expression as desugared](https://github.com/rust-lang/rust/pull/89895)
* [change `Duration::`(`try_`)`from_secs_`{`f32`, `f64`} underflow error](https://github.com/rust-lang/rust/pull/89944)
* [suggest a case insensitive match name regardless of levenshtein distance](https://github.com/rust-lang/rust/pull/89956)
* [make `From` impls of `NonZero` integer const](https://github.com/rust-lang/rust/pull/90077)
* [make more `From` impls `const` (libcore)](https://github.com/rust-lang/rust/pull/90009)
* [mark {`array`, `slice`}`::`{`from_ref`, `from_mut`} as `const fn`](https://github.com/rust-lang/rust/pull/90162)
* [avoid overflow in `VecDeque::with_capacity_in()`](https://github.com/rust-lang/rust/pull/90010)
* [fix MIRI UB in `Vec::swap_remove`](https://github.com/rust-lang/rust/pull/90099)
* [implement coherence checks for negative trait impls](https://github.com/rust-lang/rust/pull/90104)
* [make `RSplit<T, P>: Clone  not require `T: Clone`](https://github.com/rust-lang/rust/pull/90117)
* [codegen-gcc: disable strict aliasing](https://github.com/rust-lang/rustc_codegen_gcc/pull/104)
* [clippy: warn on structs with a trailing zero-sized array but no `repr` attribute](https://github.com/rust-lang/rust-clippy/pull/7838)
* [clippy: cover `Result` for `question_mark`](https://github.com/rust-lang/rust-clippy/pull/7840)
* [clippy: make `useless_format` recognize `format!("")`](https://github.com/rust-lang/rust-clippy/pull/7801)
* [clippy: avoid `eq_op` in test code](https://github.com/rust-lang/rust-clippy/pull/7811)
* [clippy: fix FP in `missing_safety_doc` lint](https://github.com/rust-lang/rust-clippy/pull/7849)
* [clippy: fix FP: no lint when cast is coming from `signum` method call for `cast_possible_truncation` lint](https://github.com/rust-lang/rust-clippy/pull/7850)
* [clippy: `missing_safety_doc`: handle 'implementation safety' headers as well](https://github.com/rust-lang/rust-clippy/pull/7856)
* [clippy: fix `match_str_case_mismatch` on uncased chars](https://github.com/rust-lang/rust-clippy/pull/7865)

### Rust Compiler Performance Triage

Multiple regressions this week, several of which were in rollups, without much
to balance them out on the improvements front.

Triage done by **@simulacrum**.
Revision range: [d45ed7..3c8f001d](https://perf.rust-lang.org/?start=d45ed7502ad225739270a368528725930f54b7b6&end=3c8f001d454b1b495f7472d8430ef8fdf10aac11&absolute=false&stat=instructions%3Au)

5 Regressions, 4 Improvements, 3 Mixed; 3 of them in rollups;
35 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-10-26.md)

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Thread local Cell methods.](https://github.com/rust-lang/rfcs/pull/3184)
* [disposition: merge] [Constrained Naked Functions](https://github.com/rust-lang/rfcs/pull/2972)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Update `std::env::temp_dir` to use GetTempPath2 on Windows when available.](https://github.com/rust-lang/rust/pull/89999)
* [disposition: merge] [Stabilize `const_raw_ptr_deref` for `*const T`](https://github.com/rust-lang/rust/pull/89551)
* [disposition: merge] [Clarification of default socket flags](https://github.com/rust-lang/rust/pull/88805)
* [disposition: merge] [use CLOCK_BOOTTIME in `Instant::now`](https://github.com/rust-lang/rust/pull/88714)
* [disposition: merge] [Implement `Termination` for `Result<Infallible, E>`](https://github.com/rust-lang/rust/pull/88601)
* [disposition: close] [Port clippy lint `redundant_field_names` to compiler](https://github.com/rust-lang/rust/pull/87512)
* [disposition: merge] [GATs: Decide whether to have defaults for `where Self: 'a`](https://github.com/rust-lang/rust/issues/87479)
* [disposition: merge] [Stabilize `File::options()`](https://github.com/rust-lang/rust/pull/85766)
* [disposition: merge] [Tracking Issue for relaxed struct unsizing rules](https://github.com/rust-lang/rust/issues/81793)
* [disposition: merge] [Tracking Issue for `option_result_unwrap_unchecked`](https://github.com/rust-lang/rust/issues/81383)
* [disposition: merge] [Tracking Issue for `destructuring_assignment`](https://github.com/rust-lang/rust/issues/71126)
* [disposition: close] [Tracking issue for `slice_concat_ext` stabilization](https://github.com/rust-lang/rust/issues/27747)

### New RFCs

* [Static async fn in traits](https://github.com/rust-lang/rfcs/pull/3185)

## Upcoming Events

### Online

* [October 27, 2021, London, UK - Rust London Ockam Takeover - Rust London User Group](https://skillsmatter.com/meetups/13606-rust-london-october2021#community)
* [October 27, 2021, Phoenix, AZ - Desert Rust Halloween - Desert Rust](https://www.meetup.com/Desert-Rustaceans/events/281215858/)
* [October 28, 2021, Copenhagen, DK - Hack Night #22 - Copenhagen Rust Group](https://cph.rs/)
* [November 2, 2021, Buffalo, NY, US - First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/281558952/)
* [November 9, 2021, Seattle, WA, US - Monthly Meetup - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksryccpbmb/)
* [November 10, 2021, Malaysia - Rust Meetup - Rust Malaysia](https://discord.gg/9Xj8H2EXTD)

### North America

* [November 10, 2021, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsyccpbnb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I think in general "force the user to think about the extra cases, and be explicit about ignoring them" is definitely idiomatic rust.

– [Daniel Wagner Hall on rust-internals](https://internals.rust-lang.org/t/pre-rfc-add-a-chunk-iterator-to-libcore/15101/16)

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1127) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
