Title: This Week in Rust 269
Number: 269
Date: 2019-01-15
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

* [Understanding Rust Lifetimes](https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa).

### #Rust2019

Find all #Rust2019 posts at [Read Rust](https://readrust.net/rust-2019/).

# Crate of the Week

This week's crate is [gfx-hal](https://crates.io/crates/gfx-hal), a hardware abstraction layer for gfx-rs. Thanks to [Vikrant Chaudhary](https://users.rust-lang.org/t/crate-of-the-week/2704/476) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [A call for #Rust2019 Roadmap blog posts](https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html).
* [A call for #RustWasm2019 Roadmap blog posts](https://rustwasm.github.io/2018/12/06/reflecting-on-rust-and-wasm-in-2018.html).
* [medium] [Mundane: Run tests with ASan and MSan](https://github.com/google/mundane/issues/15). Mundane is a Rust cryptography library backed by BoringSSL.
* [medium] [Mundane: Test BoringSSL refcounting](https://github.com/google/mundane/issues/14).
* [easy] [Mundane: CONTRIBUTING.md: Document that you need to pull from googlesource.com](https://github.com/google/mundane/issues/12).
* [Tetra: Black screen/shader issues on MacOS](https://github.com/17cupsofcoffee/tetra/issues/54). Tetra is a 2D game framework written in Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

166 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-12-31..2019-01-07

* [implement the Re-rebalance coherence RFC](https://github.com/rust-lang/rust/pull/56145)
* [forbid recursive `impl trait`](https://github.com/rust-lang/rust/pull/56074)
* [allow to dispatch fn traits depending on number of parameters](https://github.com/rust-lang/rust/pull/55986)
* [rustc: fix regression where jemalloc isn't used](https://github.com/rust-lang/rust/pull/57287)
* [syntax: fix regression in diagnostics for patterns in trait method parameters](https://github.com/rust-lang/rust/pull/57251)
* [resolve: simplify treatment of ambiguity errors](https://github.com/rust-lang/rust/pull/57199)
* [calculate privacy access only via query](https://github.com/rust-lang/rust/pull/57343)
* [privacy: fix regression in impl reachability](https://github.com/rust-lang/rust/pull/57344)
* [tweak unicode escape diagnostics](https://github.com/rust-lang/rust/pull/57210)
* [suggest using raw identifiers in 2018 edition when using keywords](https://github.com/rust-lang/rust/pull/57209)
* [do not complain about missing crate named as a keyword](https://github.com/rust-lang/rust/pull/57208)
* [use structured suggestions for nonexistent fields](https://github.com/rust-lang/rust/pull/57047)
* [use structured suggestion for method calls](https://github.com/rust-lang/rust/pull/57291)
* [add specific diagnostic when attempting to transmute between equal generic types](https://github.com/rust-lang/rust/pull/57044)
* [don't emit `Unevaluated` from `const_eval`](https://github.com/rust-lang/rust/pull/56723)
* [make `CompileController` thread-safe](https://github.com/rust-lang/rust/pull/57308)
* [NLL: user type annotations refactor, associated constant patterns and ref bindings](https://github.com/rust-lang/rust/pull/55937)
* [universes](https://github.com/rust-lang/rust/pull/55517)
* [rustdoc: force binary filename for compiled doctests](https://github.com/rust-lang/rust/pull/57338)
* [improve `Box<T>` → `Pin<Box<T>>` conversion](https://github.com/rust-lang/rust/pull/57313)
* [eliminate `Receiver::recv_timeout` panic](https://github.com/rust-lang/rust/pull/56827)
* [`VaList::copy` should not require a mutable ref](https://github.com/rust-lang/rust/pull/57311)
* [add duration constants](https://github.com/rust-lang/rust/pull/57375)
* [NLL: fix bug in associated constant type annotations](https://github.com/rust-lang/rust/pull/57304)
* [make sure feature gate errors are recoverable](https://github.com/rust-lang/rust/pull/57272)
* [cargo: fix error message when resolving dependencies](https://github.com/rust-lang/cargo/pull/6510)

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

* [disposition: merge] [Implement Debug, Eq, PartialEq, and Hash for libc structs](https://github.com/rust-lang/rfcs/pull/2235).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Summary issue for const-stabilizing `const_int_overflowing`](https://github.com/rust-lang/rust/issues/57237).
* [disposition: merge] [Const-stabilize `const_int_ops` + `const_ip`](https://github.com/rust-lang/rust/pull/57234).
* [disposition: merge] [Stabilize `let` bindings and destructuring in constants and const fn](https://github.com/rust-lang/rust/pull/57175).
* [disposition: merge] [Stablilize const_int_{rotate,wrapping,sign}](https://github.com/rust-lang/rust/pull/57105).
* [disposition: merge] [Stabilize `uniform_paths`](https://github.com/rust-lang/rust/pull/56759).
* [disposition: merge] [Stabilize the `integer_atomics` feature: Atomic{I,U}{8,16,32,64}](https://github.com/rust-lang/rust/issues/56753).
* [disposition: merge] [Stabilization proposal for #![feature(if_while_or_patterns)]](https://github.com/rust-lang/rust/issues/56212).
* [disposition: merge] [Tracking issue for RFC 2306, "Add core::convert::identity"](https://github.com/rust-lang/rust/issues/53500).
* [disposition: merge] [Tracking issue for write_all_at/read_exact_at convenience methods](https://github.com/rust-lang/rust/issues/51984).
* [disposition: merge] [Tracking issue for non-panicking pow](https://github.com/rust-lang/rust/issues/48320).
* [disposition: merge] [Tracking Issue for Result<Option> and Option<Result> Conversion](https://github.com/rust-lang/rust/issues/47338).

## New RFCs

* [Type Ascribed Coercions](https://github.com/rust-lang/rfcs/pull/2623).

# Upcoming Events

### Online

* [Jan 16. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jan 23. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Europe

* [Jan 10. Brno, CZ - Rust meetup at Masaryk University](https://rust-brno.github.io/).
* [Jan 14. Cologne, DE - Rust Cologne Meetup](https://www.meetup.com/RustCologne/events/vnwndpyzcbdb/).
* [Jan 15. Rome, IT - Rust Rome Meetup](https://www.meetup.com/Rust-Roma/events/257921654/).
* [Jan 22. Lyon, FR - TupperRust](https://tupperrust.github.io).
* [Jan 23. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzcbfc/).

### North America

* [Jan 10. Columbus, US - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dbcfrpyzcbnb/).
* [Jan 10. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209742/).
* [Jan 10. Arlington, US - Rust DC—Mid-month Rustful](https://www.meetup.com/RustDC/events/256380444).
* [Jan 13. Mountain view, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbrb/).
* [Jan 15. Los Angeles, US - Los Angeles Rust Meetup](https://www.meetup.com/Rust-Los-Angeles/events/257872752/).
* [Jan 20. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbbc/).
* [Jan 23. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzcbfc/).
* [Jan 23. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyzcbfc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Intern (Summer 2019) at Mozilla, San Francisco, US](https://careers.mozilla.org/position/gh/1480831/).
* [Kernel Engineer at System76, Denver, US](https://system76.com/careers#kernel-engineer).
* [Senior Software Engineer at Prevoty, Los Angeles, US](https://www.prevoty.com/about/careers?gh_jid=4032159002).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [volunteer] [UX Developer at Amethyst](https://community.amethyst-engine.org/t/position-available-showcase-team-ux-developers/321).
* [volunteer] [Team Artist at Amethyst](https://community.amethyst-engine.org/t/position-available-showcase-team-artists/319).
* [volunteer] [Core Developer at Amethyst](https://community.amethyst-engine.org/t/position-available-showcase-team-core-developers/320).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The name Rust suggests what it is: a thin layer on top of the metal.

– c3534l [on reddit](https://www.reddit.com/r/rust/comments/abm6hy/why_rust_is_successful_compared_with/ed1k1xl)

Thanks to [Cauê Baasch De Souza](https://users.rust-lang.org/t/twir-quote-of-the-week/328/593) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
