Title: This Week in Rust 204
Number: 204
Date: 2017-10-17
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

* <img alt="balloon" class="emoji" title=":balloon:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/balloon.png?v=0"><img alt="tada" class="emoji" title=":tada:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/tada.png?v=0"> [Announcing Rust 1.21](https://blog.rust-lang.org/2017/10/12/Rust-1.21.html). <img alt="tada" class="emoji" title=":tada:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/tada.png?v=0"><img alt="balloon" class="emoji" title=":balloon:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/balloon.png?v=0">
* [IntelliJ Rust 0.2.0 released](https://users.rust-lang.org/t/intellij-rust-0-2-0-released/13419).
* [str vs String](http://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html).
* [A little story about the `yes` Unix command](https://matthias-endler.de/2017/yes/).
* [Squeezing Rust into production for real time audio analysis](http://www.tzaeru.com/squeezing-rust-into-production-part-2/).
* [The case for writing a kernel in Rust](https://www.cs.virginia.edu/~bjc8c/papers/levy17rustkernel.pdf).
* [Exploring lock-free Rust](https://morestina.net/blog/742/exploring-lock-free-rust-1-locks).
* [It's gfx-rs aLL the way down](https://users.rust-lang.org/t/its-gfx-rs-all-the-way-down/13339) - an interim update on the LL rewrite progress.
* [Multi-platform Rust and Emscripten specific functions](https://www.worthe-it.co.za/programming/2017/10/10/multiplatform-rust-and-emscripten-specific-functions.html).
* [The impl period newsletter #2](https://internals.rust-lang.org/t/the-impl-period-newsletter-2/6034).
* [This week in Rust docs 77](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-77).
* [This week in Redox 30](https://www.redox-os.org/news/this-week-in-redox-30/).
* [podcast] [Rusty Spike Podcast - Episode 3](http://castbox.fm/episode/Episode-3-%E2%80%93-Oct-11%2C-2017-id1065347-id53530474?country=gb).
* [video] [Videos from RustFest Zürich 2017](http://www.video.ethz.ch/events/2017/rust.html).

# Crate of the Week

Despite there being no votes, the crate of this week is [abrute](https://crates.io/crates/abrute), a crate to brute-force AES keys.
Thanks to [Daniel P. Clark](https://users.rust-lang.org/u/danielpclark) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Join us at FOSDEM 2018: devroom CFP](https://rust-fosdem.github.io).
* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [Libz blitz: Out-of-band crate evaluation for 2017-10-20: mime](https://internals.rust-lang.org/t/out-of-band-crate-evaluation-for-2017-10-20-mime/5997).
* [easy] [allocators-rs: Cargo.toml: Add missing license](https://github.com/ezrosent/allocators-rs/issues/109).
* [medium] [allocators-rs: alloc-fmt: Skip early frames of backtrace](https://github.com/ezrosent/allocators-rs/issues/107).
* [medium] [allocators-rs: alloc-fmt: Allow disabling backtrace with feature](https://github.com/ezrosent/allocators-rs/issues/108).
* [doc] [num_cpus: libz blitz evaluation tracking issue](https://github.com/seanmonstar/num_cpus/issues/55).
* [doc] [num_cpus: Improve `get_physical()` documentation](https://github.com/seanmonstar/num_cpus/issues/59).
* [doc] [num_cpus: Expand crate level documentation](https://github.com/seanmonstar/num_cpus/issues/56).
* [doc] [num_cpus: Add example to `get()` docs](https://github.com/seanmonstar/num_cpus/issues/57).
* [doc] [num_cpus: Add example to `get_physical()` docs](https://github.com/seanmonstar/num_cpus/issues/58).
* [doc] [num_cpus: Improve `get()` documentation](https://github.com/seanmonstar/num_cpus/issues/60).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

117 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-10-02..2017-10-09

* [add -Zmutable-noalias flag](https://github.com/rust-lang/rust/pull/45012)
* [improvements to `proc_macro::Span` API](https://github.com/rust-lang/rust/pull/43604)
* [rustc: Don't create empty codegen units](https://github.com/rust-lang/rust/pull/45035)
* [rustc: Implement ThinLTO](https://github.com/rust-lang/rust/pull/44841)
* [store a new Region value every time we create a new region variable](https://github.com/rust-lang/rust/pull/44878)
* [rustc_trans: do not set NoCapture for anonymous lifetime &T arguments](https://github.com/rust-lang/rust/pull/45033)
* [faster compile times for release builds with llvm fix](https://github.com/rust-lang/rust/pull/45054)
* [don't panic in the coordinator thread, bubble up the failure](https://github.com/rust-lang/rust/pull/45111)
* [incr.comp.: Fix infinite recursion in Debug implementation of DepNode](https://github.com/rust-lang/rust/pull/45018)
* [incr.comp.: Switch to red/green change tracking, remove legacy system](https://github.com/rust-lang/rust/pull/44901)
* [add notes to report_conflicting_borrow MIR borrowck](https://github.com/rust-lang/rust/pull/44882)
* [MIR-borrowck: gather and signal any move errors](https://github.com/rust-lang/rust/pull/45016)
* [overlapping borrows can point to different lvalues](https://github.com/rust-lang/rust/pull/44999)
* [fix logic error in #44269's `prune_cache_value_obligations`](https://github.com/rust-lang/rust/pull/45065)
* [make non_snake_case lint allow extern no-mangle functions](https://github.com/rust-lang/rust/pull/44966)
* [allow atomic operations up to 32 bits](https://github.com/rust-lang/rust/pull/44978)
* [add read_to_end implementation to `&[u8]`'s Read impl](https://github.com/rust-lang/rust/pull/45083)
* [made `fs::copy` return the length of the main stream](https://github.com/rust-lang/rust/pull/44895)
* [implement `and_modify` on `Entry`](https://github.com/rust-lang/rust/pull/44734)
* [show callback function type args in rustdoc](https://github.com/rust-lang/rust/pull/44892)
* [cargo doc no longer crashes on impl trait](https://github.com/rust-lang/rust/pull/44860)
* [remove root from cargo lock](https://github.com/rust-lang/cargo/pull/4571)

## New Contributors

* Alexander Kuleshov
* Bráulio Bezerra
* Cameron Steffen
* Christopher Vittal
* Hoàng Đức Hiếu
* Jean Lourenço
* Jimmy Brisson
* JLockerman
* Joe Rattazzi
* Joshua Lockerman
* k0pernicus
* Matt
* Michael Hewson

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [or-patterns in if / while let expressions](https://github.com/rust-lang/rfcs/pull/2175).
* [Quick `dbg!(expr)` macro](https://github.com/rust-lang/rfcs/pull/2173).
* [Partial turbofish](https://github.com/rust-lang/rfcs/pull/2176).
* [Add Euclidean modulo & division functionality for integers](https://github.com/rust-lang/rfcs/pull/2169).

# Upcoming Events

* [Oct 19. Cambridge Rust Meetup #4](https://www.meetup.com/Cambridge-Rust-Meetup/events/244085314/).
* [Oct 19. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Oct 21. Mozilla Brasil - Mergulhando em Rust — O futuro da programação de sistemas #4](https://www.eventbrite.com.br/e/mergulhando-em-rust-o-futuro-da-programacao-de-sistemas-4-registration-38145874337).
* [Oct 23. Durham, NC - Triangle Rustaceans - with Rust Community Team member Ben Striegel](https://www.meetup.com/triangle-rustaceans/events/243586365/).
* [Oct 25. Rust Language Milano - Secondo incontro](https://www.meetup.com/rust-language-milano/events/244050676/).
* [Oct 25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct 25. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct 26. Finland Rust-lang group October meetup](https://www.meetup.com/Finland-Rust-Meetup/events/243886850/).
* [Oct 26. Mozilla Community Dresden - Rust meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/243915635/).
* [Oct 27. Rust Bangalore - Rust XML JSON and Serialization Workshop](https://www.meetup.com/rustox/events/243387629/).
* [Oct 31. Rust Zürich - Calling Rust from C and Java - October Community Meetup](https://www.meetup.com/Rust-Zurich/events/243147356/).
* [Nov  1. Boston Rust - Presentation and Hack Night](https://www.meetup.com/BostonRust/events/244260833/).
* [Nov  1. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/243942704/).
* [Nov  1. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywpbcb/).
* [Nov  1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov  1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov  2. Rust Bay Area - Zero Knowledge Proof Macros and Cernan (data pipelining)](https://www.meetup.com/Rust-Bay-Area/events/244156617/).
* [Nov  2. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Nov  4. Rust Bangalore - Rust Concurrency Workshop](https://www.meetup.com/rustox/events/240879563/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
