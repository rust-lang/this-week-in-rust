Title: This Week in Rust 317
Number: 317
Date: 2019-12-17
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

- [WASM as a Platform for Abstraction](http://adventures.michaelfbryan.com/posts/wasm-as-a-platform-for-abstraction/)

# Crate of the Week

This week's crate is [bstr](https://github.com/BurntSushi/bstr), a string type for Rust that is not required to be valid UTF-8.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/603) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [We need your help before `rustup` 1.21.0 can be released](https://www.reddit.com/r/rust/comments/e7rer9/we_need_your_help_before_rustup_1210_can_be/).
* [smallvec: Implement Clone for IntoIter<A: Array> where A: Clone](https://github.com/servo/rust-smallvec/issues/178).
* [mundane: Document items behind feature flags](https://github.com/google/mundane/issues/22).
* [crates.io: carols10cents will be mentoring multiple issues for the month of November & December](https://github.com/rust-lang/crates.io/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3AE-mentor).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

223 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-12-09..2019-12-16

* [revert stabilization of never type](https://github.com/rust-lang/rust/pull/67224) (sorry – llogiq)
* [enable `loop` and `while` in constants behind a feature flag](https://github.com/rust-lang/rust/pull/67216)
* [make transparent enums more ordinary](https://github.com/rust-lang/rust/pull/67323)
* [ensure a hard error on generic ZST constants whose body causes an eval error](https://github.com/rust-lang/rust/pull/67134)
* [improve diagnostics and code for exhaustiveness of empty matches](https://github.com/rust-lang/rust/pull/67026)
* [do not ICE on unnamed future](https://github.com/rust-lang/rust/pull/67289)
* [remove the `DelimSpan` from `NamedMatch::MatchedSeq`](https://github.com/rust-lang/rust/pull/67250)
* [optimize `shallow_resolve_changed`](https://github.com/rust-lang/rust/pull/67079)
* [add ExactSizeIterator bound to return types](https://github.com/rust-lang/rust/pull/67125)
* [fix `-Z print-type-sizes`'s handling of zero-sized fields](https://github.com/rust-lang/rust/pull/67215)
* [track polonius in `-Z self-profile`](https://github.com/rust-lang/rust/pull/67193)
* [fix constant propagation for scalar pairs](https://github.com/rust-lang/rust/pull/67015)
* [fix `unused_parens` triggers on macro by example code](https://github.com/rust-lang/rust/pull/66983)
* [rustc: allow non-empty ParamEnv's in global trait select/eval caches](https://github.com/rust-lang/rust/pull/66821)
* [remove uniform array move MIR passes](https://github.com/rust-lang/rust/pull/66650)
* [chalk: remove depth getting passed around](https://github.com/rust-lang/chalk/pull/308)
* [chalk: when truncating a goal, don't truncate the environment](https://github.com/rust-lang/chalk/pull/294)
* [use first nonempty buffer in vectored I/O](https://github.com/rust-lang/futures-rs/pull/1998)
* [use deref target in Pin trait implementations](https://github.com/rust-lang/rust/pull/67039)
* [improve code generated for `starts_with('<literal char>')`](https://github.com/rust-lang/rust/pull/67249)
* [optimize `Ord` trait implementation for bool](https://github.com/rust-lang/rust/pull/66881)
* [inline some common methods on `OsStr`](https://github.com/rust-lang/rust/pull/67169)
* [`LinkedList`: drop remaining items when drop panics](https://github.com/rust-lang/rust/pull/67243)
* [`VecDeque`: drop remaining items on destructor panic](https://github.com/rust-lang/rust/pull/67235)
* [stabilize `Result::map_or`](https://github.com/rust-lang/rust/pull/66570)
* [add a separate path for messages with no format arguments](https://github.com/rust-lang/log/pull/366)
* [remove `NodeState::{Waiting,Done}`](https://github.com/rust-lang/rust/pull/66405)
* [match `VecDeque::extend` to `Vec::extend_desugared`](https://github.com/rust-lang/rust/pull/66341)
* [stabilize the `core::panic` module](https://github.com/rust-lang/rust/pull/66771)
* [`From<NonZero*>` impls for wider `NonZero` types](https://github.com/rust-lang/rust/pull/66277)
* [add str::strip_prefix and str::strip_suffix](https://github.com/rust-lang/rust/pull/66735)
* [cargo: emit error on `[target.'cfg(debug_assertions)'.dependencies]` and similar](https://github.com/rust-lang/cargo/pull/7660)
* [rustup: improve preinstalled rust message](https://github.com/rust-lang/rustup/pull/2155)
* [docs.rs: fix panic viewing source if crate failed to build](https://github.com/rust-lang/docs.rs/pull/519)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Propose implicit named arguments for formatting macros](https://github.com/rust-lang/rfcs/pull/2795).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Deprecate Error::description for real](https://github.com/rust-lang/rust/pull/66919).
* [disposition: merge] [Stabilize the `core::panic` module](https://github.com/rust-lang/rust/pull/66771).
* [disposition: merge] [stabilize Result::map_or](https://github.com/rust-lang/rust/pull/66570).
* [disposition: merge] [Make Layout::new const](https://github.com/rust-lang/rust/pull/66254).
* [disposition: merge] [Stabilize attribute macros on inline modules](https://github.com/rust-lang/rust/pull/64273).

## New RFCs

* [Demote Apple 32bit targets to Tier 3](https://github.com/rust-lang/rfcs/pull/2837).
* [Move `std::net` types into `core:.net`](https://github.com/rust-lang/rfcs/pull/2832).
* [Cargo report future-incompat](https://github.com/rust-lang/rfcs/pull/2834).
* [Announcing the safe-transmute project group](https://github.com/rust-lang/rfcs/pull/2835).
* [Introduce the ASM project group](https://github.com/rust-lang/rfcs/pull/2836).

# Upcoming Events

### Europe

* [Dec 16. Amsterdam, NL - Rust Nederland - Rust - Talks & Demos](https://www.meetup.com/Rust-Nederland/events/266888452/).
* [Dec 20. Stuttgart, DE - Meetup Stuttgart - Rust Hack and Learn](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/267063341/).

### North America

* [Dec 18. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzqbhc/).
* [Dec 23. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzqbfc/).
* [Dec 31. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzqbpc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Hey @rustlang folks, is there a comprehensive writeup/reference anywhere of how the formatting machinery (format!(), write!(), etc.) work? Specifically from an implementation perspective (wrt trait objects, recursion)?

– [James Munns](https://jamesmunns.com/blog/fmt-unreasonably-expensive/)

> It’s dark and ancient magic. I don’t think anyone knows it very well, never mind documentation

– [Nick R. Cameron](https://twitter.com/nick_r_cameron/status/1203753952329650176?ref_src=twsrc%5Etfw)

Thanks to [mmmmib](https://users.rust-lang.org/t/twir-quote-of-the-week/328/756) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
