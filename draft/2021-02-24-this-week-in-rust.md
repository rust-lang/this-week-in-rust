Title: This Week in Rust 379
Number: 379
Date: 2021-02-24
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
* [MoonZoon - New Rust fullstack framework](https://moonzoon.rs)

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous
* [Application-wide panic handling](https://domwillia.ms/panik/)
* [Macros in Rust: A tutorial with examples](https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/)

[Benchmarking low-level I/O: C, C++, Rust, Golang, Java, Python](https://medium.com/star-gazers/benchmarking-low-level-i-o-c-c-rust-golang-java-python-9a0d505f85f7)

# Crate of the Week

This week's crate is [lever](https://crates.io/crates/lever), a library for writing transactional systems.

Thanks to [Mahmud Bulut](https://users.rust-lang.org/t/crate-of-the-week/2704/882) for the suggestion!

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

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-02-15..2021-02-22

* [suggest to create a new `const` item if the `fn` in the array is a `const fn`](https://github.com/rust-lang/rust/pull/81503)
* [fixing bad suggestion for `_` in `const` type when a function](https://github.com/rust-lang/rust/pull/81914)
* [simplify `eat_digits`](https://github.com/rust-lang/rust/pull/81427)
* [precompute ancestors when checking privacy](https://github.com/rust-lang/rust/pull/81574)
* [optimize counting digits in line numbers during error reporting](https://github.com/rust-lang/rust/pull/82248)
* [only store a `LocalDefId` in some HIR nodes](https://github.com/rust-lang/rust/pull/81611)
* [to digit simplification](https://github.com/rust-lang/rust/pull/82094)
* [reduce size of `InterpErrorInfo` to 8 bytes](https://github.com/rust-lang/rust/pull/82116)
* [pass large interpreter types by reference, not value](https://github.com/rust-lang/rust/pull/82124)
* [improve `assert_eq!` and `assert_ne!`](https://github.com/rust-lang/rust/pull/79100)
* [add `Mutex::unlock`](https://github.com/rust-lang/rust/pull/81873)
* [stabilize `Arguments::as_str`](https://github.com/rust-lang/rust/pull/82120)
* [futures: `FuturesUnordered`: do not poll the same future twice per iteration](https://github.com/rust-lang/futures-rs/pull/2333)
* [remove `unsafe impl Send for CompletedTest` & `TestResult`](https://github.com/rust-lang/rust/pull/82302)
* [test: print test name only once on timeout](https://github.com/rust-lang/rust/pull/82349)
* [cargo: propagate `lto=off` harder](https://github.com/rust-lang/cargo/pull/9182)

## Rust Compiler Performance Triage

Overall, a positive week for compiler performance with only one moderate regression. The change that introduced the regression leads to significantly improved [bootstrap speed](https://github.com/rust-lang/rust/pull/70951#issuecomment-766292996) of the compiler as well as easier maintainability.

Triage done by **@rylev**.
Revision range: [f1c47c..301ad8a](https://perf.rust-lang.org/?start=f1c47c79fe8438ed241630f885797eebef3a6cab&end=301ad8a4fa3ea56fb980443b7997c8f9d72dd717&absolute=false&stat=instructions%3Au)

1 Regression, 5 Improvements, 0 Mixed
0 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Adds `must_not_suspend_lint` RFC](https://github.com/rust-lang/rfcs/pull/3014)
* [RFC: Checking conditional compilation at compile time](https://github.com/rust-lang/rfcs/pull/3013)
* [RFC: add the Freeze trait to libcore/libstd](https://github.com/rust-lang/rfcs/pull/2944)
* [Generic Pointer to Field](https://github.com/rust-lang/rfcs/pull/2708)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Make char and u8 methods const](https://github.com/rust-lang/rust/pull/82078)
* [disposition: merge] [Demote x86_64-rumprun-netbsd target](https://github.com/rust-lang/rust/issues/81514)
* [disposition: merge] [`impl PartialEq<Punct> for char`; symmetry for #78636](https://github.com/rust-lang/rust/pull/80595)
* [disposition: merge] [Make rustdoc lints a tool lint instead of built-in](https://github.com/rust-lang/rust/pull/80527)
* [disposition: merge] [Stabilize `unsafe_op_in_unsafe_fn` lint](https://github.com/rust-lang/rust/pull/79208)
* [disposition: merge] [Add `NotSupported` to `std::io::ErrorKind`](https://github.com/rust-lang/rust/pull/78880)
* [disposition: merge] [[librustdoc] Only split lang string on `,`, ` `, and `\t`](https://github.com/rust-lang/rust/pull/78429)
* [disposition: merge] [Lint for unused borrows as part of `UNUSED_MUST_USE` ](https://github.com/rust-lang/rust/pull/76894)
* [disposition: merge] [Tracking Issue for str_split_once](https://github.com/rust-lang/rust/issues/74773)
* [disposition: merge] [Tracking Issue for ASCII methods on OsStr](https://github.com/rust-lang/rust/issues/70516)
* [disposition: close] [Tracking issue for `Option::expect_none(msg)` and `unwrap_none()`](https://github.com/rust-lang/rust/issues/62633)

## New RFCs

* [Add named path bases to cargo](https://github.com/rust-lang/rfcs/pull/3074)

# Upcoming Events

### Online
* [February 18, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccdbxb/)
* [February 23, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccdbfc/)
* [March 2, Dublin, IE - March Remote Meetup - Luca Palmieri - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/276334977/)
* [March 9, Saarbücken, Saarland, DE - Meetup: 9u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/276401469/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Finally, I feel it is necessary to debunk the “*fighting the borrow checker*” legend, a story depicting the Rust compiler as a boogeyman: in my experience, it happens mostly to beginners and the 1% trying to micro-optimize code or push the boundaries. Most experienced Rust developers know exactly how to model their code in a way that no time is wasted fighting the compiler on design issues, and can spot anti-patterns at a glance, just like most people know how to drive their car on the correct side of the road to avoid accidents, and notice those who don’t!

– [Simon Chemouil on the Kraken blog](https://blog.kraken.com/post/7964/oxidizing-kraken/)

Thanks to [scottmcm](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1004) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
