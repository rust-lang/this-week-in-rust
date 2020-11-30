Title: This Week in Rust 367
Number: 367
Date: 2020-12-2
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
* [Inside] [What the Error Handling Project Group is Working On](https://blog.rust-lang.org/inside-rust/2020/11/23/What-the-error-handling-project-group-is-working-on.html)

### Newsletters

### Tooling

### Observations/Thoughts

### Rust Walkthroughs
* [OS in Rust: Building kernel for custom target: Part-4](https://blog.knoldus.com/os-in-rust-building-kernel-for-custom-target-part-4/)

### Project Updates

### Miscellaneous

# Crate of the Week

This week's crate is [cargo-intraconv](https://crates.io/crates/cargo-intraconv), a cargo subcommand to convert links in rust documentation to the newly stable intra-doc-links format.

Thanks to [Alexis Bourget](https://users.rust-lang.org/t/crate-of-the-week/2704/849) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

345 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-11-16..2020-11-23

* [never inline naked functions](https://github.com/rust-lang/rust/pull/79192)
* [fix exhaustiveness in case a byte string literal is used at slice type](https://github.com/rust-lang/rust/pull/79072)
* [Arena: use specialization to avoid copying data](https://github.com/rust-lang/rust/pull/78569)
* [add column number support to Backtrace](https://github.com/rust-lang/rust/pull/79002)
* [add support for custom allocators in `Vec`](https://github.com/rust-lang/rust/pull/78461)
* [change `slice::to_vec` to not use `extend_from_slice`](https://github.com/rust-lang/rust/pull/79186)
* [tighten the bounds on atomic Ordering in `std::sys::unix::weak::Weak`](https://github.com/rust-lang/rust/pull/79039)
* [Add `#[cold]` attribute to `std::process::abort` and `alloc::alloc::handle_alloc_error`](https://github.com/rust-lang/rust/pull/79172)
* [impl `Default` for `PhantomPinned`](https://github.com/rust-lang/rust/pull/77893)
* [add `trailing_zeros` and `leading_zeros` to non zero types](https://github.com/rust-lang/rust/pull/79114)
* [add `f`{`32`, `64`}`::is_subnormal`](https://github.com/rust-lang/rust/pull/76941)
* [add `core::slice::fill_with`](https://github.com/rust-lang/rust/pull/79222)
* [implement `Index` and `IndexMut` for arrays](https://github.com/rust-lang/rust/pull/74989)
* [make `as`{`_mut`,}`_slice` on `array::IntoIter` public](https://github.com/rust-lang/rust/pull/79194)
* [stabilize `refcell_take`](https://github.com/rust-lang/rust/pull/78608)
* [stabilize `clamp`](https://github.com/rust-lang/rust/pull/77872)
* [stabilise `then`](https://github.com/rust-lang/rust/pull/79299)
* [stabilize `IpAddr::is_ipv4` and `is_ipv6` as const](https://github.com/rust-lang/rust/pull/76226)
* [stabilize `alloc::Layout` const functions](https://github.com/rust-lang/rust/pull/78305)
* [futures: stream: unzip operator](https://github.com/rust-lang/futures-rs/pull/2263)
* [cargo: allow resolver="1" to explicitly use the old resolver behavior](https://github.com/rust-lang/cargo/pull/8857)
* [rustdoc: give a better error when rustdoc tests fail](https://github.com/rust-lang/rust/pull/78752)
* [semverver: speed compilation by using .rmeta over .rlib files](https://github.com/rust-lang/rust-semverver/pull/138)
* [measureme: hardware performance counter support (via `rdpmc`)](https://github.com/rust-lang/measureme/pull/143)

## Rust Compiler Performance Triage

* [2020-11-24](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-24.md):
1 Regression, 2 Improvements, 2 mixed

This week saw landing of [#79237](https://github.com/rust-lang/rust/pull/79237) which by itself provides no wins but opens the door to support for split debuginfo on macOS. This'll eventually show huge wins as we can likely avoid re-collecting debuginfo while retaining support for lldb and Rust backtraces. [#79361](https://github.com/rust-lang/rust/issues/79361) tracks the stabilization of the rustc flag, but the precise rollout to stable users is not yet 100% clear.

Triage done by @jyn514 and @simulacrum.

4 regressions, 4 improvements, 2 mixed results.
5 of them in rollups.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-24.md) for more.

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

*No Tracking Issues or PRs are currently in the final comment period.*

## New RFCs

* [Add a new syntax to declare that a trait must always be object-safe](https://github.com/rust-lang/rfcs/pull/3022)

# Upcoming Events

### Online
* [November 26, Edinburgh, UK - Rust in the Polymesh Project - Rust Edinburgh](https://www.meetup.com/rust-edi/events/273101770)
* [November 26, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcpbjc/)
* [November 26, Tel Aviv-Yafo, IL - Rust Machine Learning On-line Meetup - ODSC Tel Aviv Data Science](https://www.meetup.com/Tel-Aviv-Data-Science-ODSC/events/274650041/)
* [December 1, Buffalo, NY, US - Buffalo Rust User Group - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/274623141/)
* [December 1, Munich, DE - Rust Remote #4 (CEST) - Rust Munich Meetup](https://www.meetup.com/rust-munich/events/273529335)
* [December 2, Johannesburg, ZA - Monthly Joburg Rust Chat - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/274734310/)
* [December 2, Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy Rust](https://www.meetup.com/indyrs/events/jhfstrybcqbdb/)
* [December 8, Saarbücken, Saarland, DE - Meetup: 6u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/274592167)

### North America
* [December 9, Atlanta, GA, US - Grab a beer with fellow Rustaceans - Rust Atlanta](https://www.meetup.com/Rust-ATL/events/qxqdgrybcqbmb/)

### Asia Pacific
* [December 7, Auckland, NZ - Rust AKL - Show and Tell + Introduction to RUst II](https://www.meetup.com/rust-akl/events/266876724/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I know noting about the compiler internals but it looks to me as if 90% of the time is spent pretty-printing LayoutError.

– [Vadzim Dambrouski on github](https://github.com/rust-lang/rust/issues/75992#issuecomment-716622473)

Thanks to [mmmmib](https://users.rust-lang.org/t/twir-quote-of-the-week/328/968) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/joxy7n/this_week_in_rust_363/)</small>
