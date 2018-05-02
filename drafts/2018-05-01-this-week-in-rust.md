Title: This Week in Rust 232
Number: 232
Date: 2018-05-01
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

* [regex 1.0 is released](https://github.com/rust-lang/regex/releases/tag/1.0.0).
* [nperf: A sampling CPU profiler for Linux written in Rust from Nokia](https://github.com/nokia/nperf).
* [How to speed up the Rust compiler in 2018](https://blog.mozilla.org/nnethercote/2018/04/30/how-to-speed-up-the-rust-compiler-in-2018/).
* [Borrowing in async code](https://aturon.github.io/2018/04/24/async-borrowing/).
* [An alias-based formulation of the borrow checker](http://smallcultfollowing.com/babysteps/blog/2018/04/27/an-alias-based-formulation-of-the-borrow-checker/).
* [Writing an OS in Rust: Unit testing](https://os.phil-opp.com/unit-testing/).
* [How does dynamic dispatch work in WebAssembly](http://fitzgeraldnick.com/2018/04/26/how-does-dynamic-dispatch-work-in-wasm.html)?
* [Rust pattern: Precise closure capture clauses](http://smallcultfollowing.com/babysteps/blog/2018/04/24/rust-pattern-precise-closure-capture-clauses/).
* [Embedded Rust experiments - Flipping some bits high on STM32VLDISCOVERY board](https://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html).
* [Introducing Sentry for Rust](https://blog.sentry.io/2018/05/01/sentry-for-rust).
* [Installing Rust offline](https://hatsunearu.github.io/2018/04/29/rust-offline/).
* [This week in Rust docs 103](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-103).
* [This week in Rust and WebAssembly 001](https://rustwasm.github.io/2018/04/21/this-week-in-rust-wasm-001.html).
* [The Embedded Working Group newsletter 4](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-4/7414).
* [podcast] [Rusty Spike Podcast - episode 27](https://rusty-spike.blubrry.net/2018/04/26/episode-27-apr-25-2018/). Game development, GNOME, Facebook, and match-making.
* [podcast] [New Rustacean: e023 – Traits deep dive, part 1](https://newrustacean.com/show_notes/e023/).

# Crate of the Week

This week's crate is [human-panic](https://crates.io/crates/human-panic), a crate to make Rust's error handling usable to end users. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Clippy](https://github.com/rust-lang-nursery/rust-clippy) has a lot of [good first issues](https://github.com/rust-lang-nursery/rust-clippy/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) to get started.
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

132 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-04-16..2018-04-23

* [stabilize x86/x86_64 SIMD](https://github.com/rust-lang/rust/pull/49664) (WOOT!)
* [wasm: increase default stack size to 1MB](https://github.com/rust-lang/rust/pull/50083)
* [std: minimize size of panicking on wasm](https://github.com/rust-lang/rust/pull/49488)
* [remove 'proc' from the reserved keywords list](https://github.com/rust-lang/rust/pull/49699)
* [proc_macro: stay on the "use the cache" path more](https://github.com/rust-lang/rust/pull/50069)
* [work around LLVM debuginfo problem in librustc_driver](https://github.com/rust-lang/rust/pull/49904)
* [avoid allocating when parsing \u{...} literals](https://github.com/rust-lang/rust/pull/50052)
* [parser: do not override syntactic context for dummy spans](https://github.com/rust-lang/rust/pull/50152)
* [lazily evaluate EvalErrorKind::*.into() calls](https://github.com/rust-lang/rust/pull/50051)
* [change the hashcounts in raw `Lit` variants from `usize` to `u16`](https://github.com/rust-lang/rust/pull/49993)
* [remove HIR inlining](https://github.com/rust-lang/rust/pull/49991)
* [properly handle ranges of signed enums using both extremums](https://github.com/rust-lang/rust/pull/49981)
* [update Rhs on ShlAssign to default to Self](https://github.com/rust-lang/rust/pull/49630)
* [add inherent methods in libcore for `[T]`, `[u8]`, `str`, `f32`, and `f64`](https://github.com/rust-lang/rust/pull/49896)
* [implement size_hint for some iterators](https://github.com/rust-lang/cargo/pull/5272)
* [atomic: remove 'Atomic*' from Debug output](https://github.com/rust-lang/rust/pull/48553)
* [replace {`Alloc`, `GlobalAlloc`}`::oom` with a lang item](https://github.com/rust-lang/rust/pull/50144)
* [stabilize a bunch of minor api additions](https://github.com/rust-lang/rust/pull/50017)
* [rustdoc: UI tests for rustdoc](https://github.com/rust-lang/rust/pull/49542)
* [rustdoc: add doc search aliases](https://github.com/rust-lang/rust/pull/49757)
* [cargo: add new metadata fields](https://github.com/rust-lang/cargo/pull/5386)

## New Contributors

* Aaron Aaeng
* Irina Popa
* James Sanderson
* Pazzaz
* Philipp Hansch
* Ralf Biedert
* z4v1er

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2318: Custom test frameworks](https://github.com/rust-lang/rfcs/pull/2318).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Reserve `try` for `try { .. }` block expressions](https://github.com/rust-lang/rfcs/pull/2388).
* [disposition: merge] [Async/await notation for ergonomic asynchronous IO](https://github.com/rust-lang/rfcs/pull/2394).
* [disposition: merge] [Unreserve `proc`](https://github.com/rust-lang/rfcs/pull/2420).

## New RFCs

* [Add Async trait and task system to libcore](https://github.com/rust-lang/rfcs/pull/2418).
* [Unreserve `proc`](https://github.com/rust-lang/rfcs/pull/2420).
* [Keyword unreservations (pure, sizeof, alignof, offsetof)](https://github.com/rust-lang/rfcs/pull/2421).
* [`throw` expressions](https://github.com/rust-lang/rfcs/pull/2426).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [May  3. Utrecht, NL - Rust Workshop](https://www.meetup.com/Rust-Utrecht/events/248995086/).
* [May  6. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbjb/).
* [May  7. Sydney, AU - Rust Sydney - Meetup 13](https://www.meetup.com/Rust-Sydney/events/249764935/).
* [May  8. Helsinki, FI - Finland Rust-lang Group - May Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/250129359/).
* [May  8. São Paulo, BR - Encontro de Comunidades - Guru-SP e RustLangBR na TOTVS](https://www.meetup.com/Guru-SP-Grupo-de-Usuarios-Ruby-de-Sao-Paulo/events/249463627/).
* [May  8. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [May  9. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May  9. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [May 10. Redwood City, US - Bay Area - Where "Self-Driving" Database Meets a "Rusty" Distributed Key-Value Store](https://www.meetup.com/Bay-Area-NewSQL-Database-Meetup/events/249676562/).
* [May 10. Arlington, US - Rust DC - Learn+Try: parsing with nom](https://www.meetup.com/RustDC/events/249883820).
* [May 10. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/lcsdqpyxhbnb/).
* [May 10. San Diego, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/249783590/).
* [May 13. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbrb/).
* [May 14. Seattle, US - Seattle Rust Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxhbsb/).
* [May 16. Denver, US - Rust Boulder/Denver - Rust Denver May Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/249098925/).
* [May 16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/249497881/).
* [May 16. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxhbvb/).
* [May 17. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxhbwb/).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> last time i talked to the infra team they made a bot to replace kennytm. i fear if I ask them to write a rust based unikernel with a custom os to host the docs they’ll actually do it

— [@killercup on Twitter](https://twitter.com/killercup/status/988894247075155968).

Thanks to [skade](https://users.rust-lang.org/t/twir-quote-of-the-week/328/516) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
