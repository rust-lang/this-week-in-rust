Title: This Week in Rust 369
Number: 369
Date: 2020-12-16
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

### Project Updates

* [Sequoia PGP](https://www.sequoia-pgp.org/) released [version 1.0](https://sequoia-pgp.org/blog/2020/12/16/202012-1.0/)

### Miscellaneous

# Crate of the Week

This week's crate is [breadx](https://github.com/not-a-seagull/breadx), a X-windows protocol implementation in 100% safe and mutex-free Rust.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/851) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

279 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-11-30..2020-12-07

* [add wasm32 support to inline asm](https://github.com/rust-lang/rust/pull/78684)
* [improve attribute message error spans](https://github.com/rust-lang/rust/pull/79509)
* [chalk: always relate with Invariant to non-General inference vars](https://github.com/rust-lang/chalk/pull/659)
* [fix perf regression caused by match exhaustiveness split](https://github.com/rust-lang/rust/pull/79680)
* [pass around Symbols instead of Idents in doctree](https://github.com/rust-lang/rust/pull/79623)
* [tweak diagnostics on shadowing lifetimes/labels](https://github.com/rust-lang/rust/pull/79620)
* [avoid panic_bounds_check in `fmt::write`](https://github.com/rust-lang/rust/pull/78122)
* [fix incorrect `io::Take`'s limit resulting from `io::copy` specialization](https://github.com/rust-lang/rust/pull/79650)
* [`std::io`: use sendfile for UnixStream](https://github.com/rust-lang/rust/pull/79600)
* [cargo: slightly optimize `cargo vendor](https://github.com/rust-lang/cargo/pull/8937)
* [cargo: add "--workspace" to update command](https://github.com/rust-lang/cargo/pull/8725)
* [rustdoc: JSON backend experimental impl](https://github.com/rust-lang/rust/pull/79539)

## Rust Compiler Performance Triage

* [2020-12-08](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-08.md):
0 Regressions, 2 Improvements, 1 Mixed

Triage done by @simulacrum.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-08.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Plan to make core and std's panic identical](https://github.com/rust-lang/rfcs/pull/3007)
* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [added secret types rfc](https://github.com/rust-lang/rfcs/pull/2859)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [rustdoc: stabilise --default-theme command line option](https://github.com/rust-lang/rust/pull/79642)
* [disposition: merge] [Implement `From<char>` for u64 and u128.](https://github.com/rust-lang/rust/pull/79502)
* [disposition: merge] [Stabilize `unsafe_cell_get_mut`](https://github.com/rust-lang/rust/pull/79485)
* [disposition: merge] [Move `{f32,f64}::clamp` to core](https://github.com/rust-lang/rust/pull/79473)
* [disposition: merge] [Stabilize all stable methods of `Ipv4Addr`, `Ipv6Addr` and `IpAddr` as const](https://github.com/rust-lang/rust/pull/79342)
* [disposition: merge] [Acknowledge that `[CONST; N]` is stable](https://github.com/rust-lang/rust/pull/79270)
* [disposition: merge] [Deprecate atomic compare_and_swap method](https://github.com/rust-lang/rust/pull/79261)
* [disposition: merge] [Stabilize `core::slice::fill`](https://github.com/rust-lang/rust/pull/79213)
* [disposition: close] [Made matches! more useful by adding mapping support](https://github.com/rust-lang/rust/pull/79188)
* [disposition: merge] [passes: prohibit invalid attrs on generic params](https://github.com/rust-lang/rust/pull/79073)
* [disposition: merge] [stabilize deque_range](https://github.com/rust-lang/rust/pull/79022)
* [disposition: close] [Apply `unused_doc_comments` lint to inner items](https://github.com/rust-lang/rust/pull/78367)
* [disposition: merge] [Rename `overlapping_patterns` lint](https://github.com/rust-lang/rust/pull/78242)
* [disposition: merge] [Stabilize or_insert_with_key](https://github.com/rust-lang/rust/pull/78083)
* [disposition: close] [Add built-in implementations of `Default` for function definition and… ](https://github.com/rust-lang/rust/pull/77688)
* [disposition: merge] [Mark `-1` as an available niche for file descriptors](https://github.com/rust-lang/rust/pull/74699)
* [disposition: merge] [Stabilize the Wake trait](https://github.com/rust-lang/rust/pull/74304)
* [disposition: merge] [Tracking issue for map_ok and map_err method for `Poll<Option<Result<T, E>>>`](https://github.com/rust-lang/rust/issues/63514)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [December 10, Stuttgart, DE - Hack & Learn - Directions for 2021 - Rust Community Stuttgart](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/274892215/)
* [December 10, San Diego, CA, US - San Diego Rust December 2020 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/274757235/)
* [December 10, Washington, DC, US - How oso built a runtime reflection system for Rust—Rust DC](https://www.meetup.com/RustDC/events/274460587)
* [December 15, Russia - Russian Rust Online Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/274924961/)
* [December 16, Vancouver, BC, US - Are Results just Checked Exceptions? - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsybcqbvb/)

### North America
* [December 10, Provo, UT, US - Mob Programming: Add `--tree -d` to `lsd`](https://www.meetup.com/utah-rust/events/273530244/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Writing rust for me is a gradual process of the compiler patiently guiding me towards the program I should have written in the first place, and at the end I take all the credit.

– [@felixwatts on Discord](https://discord.com/channels/442252698964721669/448238009733742612/783395725991084074)

Thanks to [Joshua Nelson](https://users.rust-lang.org/t/twir-quote-of-the-week/328/972) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
