Title: This Week in Rust 336
Number: 336
Date: 2020-04-28
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

# Crate of the Week

This week's crate is [regex2fat](https://github.com/8051Enthusiast/regex2fat), a program to convert a regular expression into a decidedly nonstandard FAT32 file system.

Thanks to [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/757) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Compiler Explorer: Bytes support for Rust](https://github.com/mattgodbolt/compiler-explorer/issues/1925).
* [rlua is looking for maintainers](https://github.com/kyren/rlua/issues/172).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

408 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-04-13..2020-04-20

* [remove a stack frame from `.await` calls](https://github.com/rust-lang/rust/pull/70831)
* [improve async-await/generator obligation errors in some cases](https://github.com/rust-lang/rust/pull/70679)
* [make `needs_drop` less pessimistic on generators](https://github.com/rust-lang/rust/pull/70015)
* [provide better compiler output when using `?` on `Option` in fn returning `Result` and vice-versa](https://github.com/rust-lang/rust/pull/71141)
* [suggest `.into()` over `try_into()` when it would work](https://github.com/rust-lang/rust/pull/71051)
* [maintain chain of derived obligations](https://github.com/rust-lang/rust/pull/69793)
* [chalk: recursive solver](https://github.com/rust-lang/chalk/pull/372)
* [chalk: recursive solver: Exit early on ambiguity](https://github.com/rust-lang/chalk/pull/404)
* [chalk:  make it possible to cache the result of env elaboration](https://github.com/rust-lang/chalk/pull/403)
* [use query to determine whether function needs const checking](https://github.com/rust-lang/rust/pull/69642)
* [mir-opt: run `SimplifyLocals` to a fixedpoint and handle most rvalue](https://github.com/rust-lang/rust/pull/70755)
* [miri: add option to disable alignment check](https://github.com/rust-lang/miri/pull/1332)
* [miri: let machine hook dynamically decide about alignment checks](https://github.com/rust-lang/rust/pull/71101)
* [miri: expand frame hooks](https://github.com/rust-lang/rust/pull/71100)
* [miri: handle `std::sync::atomic::spin_loop_hint()`](https://github.com/rust-lang/miri/pull/1342)
* [ptr: introduce `len()` method on raw slices](https://github.com/rust-lang/rust/pull/71082)
* [miri: use pre-computed layouts some more](https://github.com/rust-lang/miri/pull/1349)
* [miri-unleashed: test that we detect heap allocations](https://github.com/rust-lang/rust/pull/71276)
* [deprecate the `asm!` macro in favor of `llvm_asm!`](https://github.com/rust-lang/rust/pull/71007)
* [backtrace: remove memmap dependency](https://github.com/rust-lang/backtrace-rs/pull/311)
* [cargo: several updates to token/index handling](https://github.com/rust-lang/cargo/pull/7973)
* [cargo: try to avoid panics on buggy (?) clocks](https://github.com/rust-lang/cargo/pull/8114)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2834: Cargo report future-incompat](https://github.com/rust-lang/rfcs/pull/2834).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Major change proposal process for compiler team](https://github.com/rust-lang/rfcs/pull/2904).
* [disposition: merge] [Project Groups](https://github.com/rust-lang/rfcs/pull/2856).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: close] [Tracking issue for Vec::remove_item](https://github.com/rust-lang/rust/issues/40062).
* [disposition: close] [Tracking issue for Box::into_raw_non_null](https://github.com/rust-lang/rust/issues/47336).
* [disposition: merge] [Stabilize the `#[alloc_error_handler]` attribute (for no_std + liballoc)](https://github.com/rust-lang/rust/issues/66740).
* [disposition: merge] [Make `handle_alloc_error` default to panic (for no_std + liballoc)](https://github.com/rust-lang/rust/issues/66741).
* [disposition: merge] [proc_macro: Stabilize `Span::resolved_at` and `Span::located_at`](https://github.com/rust-lang/rust/pull/69041).
* [disposition: close] [Return error for current_exe on nonexistent file](https://github.com/rust-lang/rust/pull/69557).
* [disposition: merge] [Implement BitOr and BitOrAssign for the NonZero integer types](https://github.com/rust-lang/rust/pull/69813).
* [disposition: merge] [Should enum discriminants have generics in scope](https://github.com/rust-lang/rust/issues/70453).
* [disposition: merge] [stabilize BTreeMap::remove_entry](https://github.com/rust-lang/rust/pull/70712).
* [disposition: merge] [Remove language-level UB for non-UTF-8 str](https://github.com/rust-lang/rust/issues/71033).
* [disposition: merge] [Stabilize UNICODE_VERSION (feature unicode_version)](https://github.com/rust-lang/rust/pull/71068).
* [disposition: merge] [Define UB in float-to-int casts to saturate](https://github.com/rust-lang/rust/pull/71269).

## New RFCs

* [Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation](https://github.com/rust-lang/rfcs/pull/2912).
* [Destructuring assignment](https://github.com/rust-lang/rfcs/pull/2909).

# Upcoming Events

### Online

* [Apr 24. Russia - Rust online meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/269992161/) ([translation](https://youtu.be/NCE4w42hb7o))
* [Apr 30. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcgbnc/).
* [Apr 30. Zurich, CH - Rust Zurich - Security in Rust: cargo-crev and cargo-audit](https://www.meetup.com/Rust-Zurich/events/270169298/).

### Asia Pacific

* [May  4. Auckland, NZ - Rust AKL - Rust Meetup](https://www.meetup.com/rust-akl/events/266876545/).

### North America

* [Apr 27. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybcgbkc/).
* [Apr 28. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmybcgblc/).
* [May  6. Portland, OR, US - PDXRust - NES Emulation in Rust](https://www.meetup.com/PDXRust/events/269165311/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> What's special about UB is that it attacks your ability to find bugs, like a disease that attacks the immune system. Undefined behavior can have arbitrary, non-local and even non-causal effects that undermine the deterministic nature of programs. That's intolerable, and that's why it's so important that safe Rust rules out undefined behavior even if there are still classes of bugs that it doesn't eliminate.

– [@trentj on rust-users](https://users.rust-lang.org/t/newbie-learning-how-to-deal-with-the-borrow-checker/40972/11)

Thanks to [Louis Cloete](https://users.rust-lang.org/t/twir-quote-of-the-week/328/854) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [nasa42](https://github.com/nasa42).*

<small>[Discuss on r/rust]().</small>
