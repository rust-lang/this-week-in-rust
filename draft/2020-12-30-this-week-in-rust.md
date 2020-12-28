Title: This Week in Rust 371
Number: 371
Date: 2020-12-30
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

No official blog posts or newsletters this week.

### Project/Tooling Updates

### Observations/Thoughts

* [A look at tokio 1.0 API Changes](https://leshow.github.io/post/udp_tokio_1_0/)

### Rust Walkthroughs

### Miscellaneous

# Crate of the Week

This week's crate is [RustFFT](https://github.com/ejmahler/RustFFT), a *Fast* Fourier transformation library that lives up to the name.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/863) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [The CCC Rustaceans are looking for artwork for badges](https://users.rust-lang.org/t/rc3-assembly-ccc-congress/50283/3)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

292 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-12-14..2020-12-21

* [skip `dsymutil` by default for compiler bootstrap](https://github.com/rust-lang/rust/pull/80213)
* [implement if-let match guards](https://github.com/rust-lang/rust/pull/79051)
* [or\_patterns: implement `:pat` edition-specific behavior](https://github.com/rust-lang/rust/pull/80100)
* [improve and fix diagnostics of exhaustiveness checking](https://github.com/rust-lang/rust/pull/80104)
* [gracefully handle mistyping `->` as `=>` in function return type](https://github.com/rust-lang/rust/pull/77035)
* [handle desugaring in impl trait bound suggestion](https://github.com/rust-lang/rust/pull/80211)
* [enhance error message when misspelled label to value in break expression](https://github.com/rust-lang/rust/pull/80023)
* [always run intrinsics lowering pass](https://github.com/rust-lang/rust/pull/80040)
* [mir-opt: allow debuginfo to be generated for a constant or a Place](https://github.com/rust-lang/rust/pull/73210)
* [turn quadratic time on number of impl blocks into linear time](https://github.com/rust-lang/rust/pull/78317)
* [`MaybeUninit::copy`/`clone_from_slice`](https://github.com/rust-lang/rust/pull/79607)
* [stabilize `unsafe_cell_get_mut`](https://github.com/rust-lang/rust/pull/79485)
* [move {`f32`, `f64`}`::clamp` to `core`](https://github.com/rust-lang/rust/pull/79473)
* [stabilize all stable methods of `Ipv4Addr`, `Ipv6Addr` and `IpAddr` as const](https://github.com/rust-lang/rust/pull/79342)
* [stabilize `or_insert_with_key`](https://github.com/rust-lang/rust/pull/78083)
* [add fast futex-based thread parker for Windows](https://github.com/rust-lang/rust/pull/77618)
* [optimization for `bool`'s `PartialOrd` impl](https://github.com/rust-lang/rust/pull/80035)
* [fix overflow when converting ZST `Vec` to `VecDeque`](https://github.com/rust-lang/rust/pull/80003)
* [use pointer type in `AtomicPtr::swap` implementation](https://github.com/rust-lang/rust/pull/80236)
* [stdarch: move code out of constify macros](https://github.com/rust-lang/stdarch/pull/973)
* [stabilize cargo's new feature resolver](https://github.com/rust-lang/rfcs/pull/2957)

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

* [Stabilize Cargo's new feature resolver](https://github.com/rust-lang/rfcs/pull/2957)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Infallible promotion](https://github.com/rust-lang/rfcs/pull/3027)
* [disposition: postpone] [Opt-in Stable Trait VTables](https://github.com/rust-lang/rfcs/pull/2955)
* [disposition: merge] [RFC: Serve crates-io registry over HTTP as static files](https://github.com/rust-lang/rfcs/pull/2789)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition:merge] [Stabilize Arc::{increment,decrement}_strong_count](https://github.com/rust-lang/rust/pull/79285)
* [disposition:merge] [stabilize `#![feature(min_const_generics)]`](https://github.com/rust-lang/rust/pull/79135)
* [disposition:merge] [Add `impl Div<NonZeroU{0}> for u{0}` which cannot panic](https://github.com/rust-lang/rust/pull/79134)

## New RFCs

* [Primitive enum conversion reform](https://github.com/rust-lang/rfcs/pull/3040)
* [Rust 2021 Roadmap](https://github.com/rust-lang/rfcs/pull/3037)

# Upcoming Events

### Online
* [December 29, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcqbmc/)
* [January 5, Buffalo, NY, US - Buffalo Rust User Group](https://www.meetup.com/Buffalo-Rust-Meetup/events/274936687/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> It took me sometime to let go and embrace getting things working before optimizing. It was a major breakthrough on that journey when I realized that ALL my python variables are `Rc<RefCell<_>>` , so any chance I had to make a variable that was less complicated than that was already a big optimization. If 1/10 Rust variables had to be that complicated it would not feel good, but it would already be 90% better. So if 1/50 make the code ezere to read and maintain then do it!

– [Eh2406 on /r/rust](https://www.reddit.com/r/rust/comments/kdayix/i_need_some_advice_about_heap_usage_with_rust/gfvtcwx)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/977) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
