Title: This Week in Rust 365
Number: 365
Date: 2020-11-18
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

### Tooling

### Observations/Thoughts

### Rust Walkthroughs
* [video] [(Live Coding) Audio adventures in Rust: UI with Actix, WebView, and React](https://youtu.be/vmvq9jKBlGc)

[SQLite File Parser Pt. 2: The Header... continues](https://freemasen.com//blog/sqlite-parser-pt-2/index.html)

[PL] [CrabbyBird #3 Generowanie świata gry – cześć I](https://postacnormalna.pl/crabbybird-3-generowanie-swiata-gry-czesc-i/)

### Project Updates

* [xd(1): hex-dumping tool with a ♥♪ code page 437 twist ♫♥](https://www.azabani.com/2020/11/15/xd.html)

### Miscellaneous
* [OS in Rust: Custom target to build kernel for a bare metal: Part-3](https://blog.knoldus.com/os-in-rust-custom-target-to-build-kernel-for-a-bare-metal-part-3/)
* [Creating a Tetris Clone in Rust, with Bevy (Part 1)](https://corbamico.github.io/2020/11/12/tetris-1/)

# Crate of the Week

This week's crate is [postfix-macros](https://github.com/est31/postfix-macros), a clever hack to allow postfix macros in stable Rust.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/841) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

333 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-11-02..2020-11-09

* [Implement destructuring assignment for tuples](https://github.com/rust-lang/rust/pull/78748)
* [reverse binding order in matches to allow the subbinding of copyable fields in bindings after `@`](https://github.com/rust-lang/rust/pull/78638)
* [Fix unreachable sub-branch detection in or-patterns](https://github.com/rust-lang/rust/pull/78167)
* [Transform post order walk to an iterative approach](https://github.com/rust-lang/rust/pull/78607)
* [Compile rustc crates with the initial-exec TLS model](https://github.com/rust-lang/rust/pull/78201)
* [Make some `std::io` functions `const`](https://github.com/rust-lang/rust/pull/78811)
* [Stabilize `Poll::is_ready` and `is_pending` as const](https://github.com/rust-lang/rust/pull/76227)
* [Stabilize `hint::spin_loop`](https://github.com/rust-lang/rust/pull/76097)
* [Simplify the implementation of `Cell::get_mut`](https://github.com/rust-lang/rust/pull/78735)
* [futures: Add `StreamExt::cycle`](https://github.com/rust-lang/futures-rs/pull/2252)
* [futures: Add `TryStreamExt::try_buffered`](https://github.com/rust-lang/futures-rs/pull/2245)
* [cargo: Avoid some extra downloads with new feature resolver](https://github.com/rust-lang/cargo/pull/8823)

## Rust Compiler Performance Triage

* [2020-11-10](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-10.md):
1 Regression, 2 Improvements, 2 mixed

A mixed week with improvements still outweighing regressions. Perhaps the biggest highlight was the move to compiling rustc crates [with the initial-exec TLS model](https://github.com/rust-lang/rust/pull/78201) which results in fewer calls to `_tls_get_addr` and thus faster compile times.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-10.md) for more.

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
* [disposition: merge] [Stabilize clamp](https://github.com/rust-lang/rust/pull/77872)
* [disposition: merge] [[android] Add support for android's file descriptor ownership tagging to libstd.](https://github.com/rust-lang/rust/pull/74860)
* [disposition: merge] [Implement Error for &(impl Error)](https://github.com/rust-lang/rust/pull/75180)
* [disposition: merge] [Add checking for no_mangle to unsafe_code lint](https://github.com/rust-lang/rust/pull/72209)
* [disposition: merge] [Tracking issue for methods converting `bool` to `Option<T>`](https://github.com/rust-lang/rust/issues/64260)

## New RFCs
* [User/ardavis/checked cfg](https://github.com/rust-lang/rfcs/pull/3013)
* [add const-ub RFC](https://github.com/rust-lang/rfcs/pull/3016)
* [Adds `must_not_await_lint` RFC](https://github.com/rust-lang/rfcs/pull/3014)

# Upcoming Events

### Online
* [November 12, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcpbqb/)
* [November 12, Washington, DC, US - Mid-month Rustful—How oso built a runtime reflection system for Rust - Rust DC](https://www.meetup.com/RustDC/events/273813659)
* [November 12, Lehi, UT, US - WASM, Rust, and the State of Async/Await - Utah Rust](https://www.meetup.com/utah-rust/events/273757338/)
* [November 18, Vancouver, BC, CA - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsybcpbxb/)
* [November 24, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcpbgc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*
# Quote of the Week

> There are no bad programmers, only insufficiently advanced compilers

– [Esteban Kuber on twitter](https://twitter.com/ekuber/status/1319476290395664384)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/957) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/joxy7n/this_week_in_rust_363/)</small>
