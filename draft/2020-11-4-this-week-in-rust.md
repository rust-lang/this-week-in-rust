Title: This Week in Rust 363
Number: 363
Date: 2020-11-04
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

### Tooling
* [Blogpost on Cargo features in IntelliJ Rust](https://blog.jetbrains.com/clion/2020/10/intellij-rust-new-functionality-for-cargo-features/)

### Observations/Thoughts
* [Semantic FFI Bindings in Rust - Reactivating the Borrow Checker](https://blog.schichler.dev/semantic-ffi-bindings-in-rust-reactivating-the-borrow-checker-ckgxtoxo8057pwrs174dqhcsi)

### Learn Rust
* [DE] [The Rust Programming Language (translated in German)](https://rust-lang-de.github.io/rustbook-de/)

* [Continuous Deployment For Rust Applications (Zero To Production In Rust #5)](https://www.lpalmieri.com/posts/2020-11-01-zero-to-production-5-how-to-deploy-a-rust-application/)

### Project Updates
* [oso, an open-source policy engine for authorization written in Rust](https://github.com/osohq/oso), released [version 0.7.1 of their authorization library for Rust projects!](https://docs.rs/oso/0.7.1/oso/)

### Miscellaneous

# Crate of the Week

This week's crate is [tract](https://github.com/sonos/tract) from Sonos, a neural network inference library, written purely in Rust for models in ONNX, NNEF and TF formats.

Thanks to [Benjamin Minixhofer](https://users.rust-lang.org/t/crate-of-the-week/2704/837) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

374 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-10-26..2020-11-02

* [add cg_clif as optional codegen backend](https://github.com/rust-lang/rust/pull/77975) (Woohoo!)
* [rustc_span: improve bounds checks in byte_pos_to_line_and_col](https://github.com/rust-lang/rust/pull/78423)
* [adjust turbofish help message for const generics](https://github.com/rust-lang/rust/pull/78460)
* [avoid complex diagnostics in snippets which contain newlines](https://github.com/rust-lang/rust/pull/75020)
* [suggest calling await on method call and field access](https://github.com/rust-lang/rust/pull/78297)
* [fix control flow check for breaking with diverging values](https://github.com/rust-lang/rust/pull/77317)
* [uplift `temporary-cstring-as-ptr` lint from clippy into rustc](https://github.com/rust-lang/rust/pull/75671)
* [check object safety of generic constants](https://github.com/rust-lang/rust/pull/78365)
* [chalk: make max goal size for recursive solver configurable](https://github.com/rust-lang/chalk/pull/647)
* [coherence check perf: iterate over the smaller list](https://github.com/rust-lang/rust/pull/78323)
* [optimise align_offset for stride=1 further](https://github.com/rust-lang/rust/pull/75728)
* [inline `NonZeroN::from(n)`](https://github.com/rust-lang/rust/pull/78491)
* [inline Default::default() for atomics](https://github.com/rust-lang/rust/pull/78621)
* [inline some functions in core::str](https://github.com/rust-lang/rust/pull/78073)
* [prevent `String::retain` from creating non-utf8 strings when abusing panic](https://github.com/rust-lang/rust/pull/78499)
* [add `fetch_update` methods to `AtomicBool` and `AtomicPtr`](https://github.com/rust-lang/rust/pull/78637)
* [add `[T]::as_chunks`(`_mut`)](https://github.com/rust-lang/rust/pull/76635)
* [fix `Box::into_unique`](https://github.com/rust-lang/rust/pull/78446)
* [hashbrown: better branch likelyness on stable](https://github.com/rust-lang/hashbrown/pull/209)
* [futures: add `WeakShared`](https://github.com/rust-lang/futures-rs/pull/2169)
* [cargo: add a future-compatibility warning on allowed feature name characters](https://github.com/rust-lang/cargo/pull/8814)
* [cargo: new namespaced features implementation](https://github.com/rust-lang/cargo/pull/8799)

## Rust Compiler Performance Triage

* [2020-11-03](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-03.md):
0 Regressions, 5 Improvements, 0 mixed

A number of improvements on various benchmarks. The most notable news this week
in compiler performance is the progress on instruction metric collection on a
per-query level; see [measureme#143](https://github.com/rust-lang/measureme/pull/143) for the latest.

Otherwise, this week was an excellent one for performance (though mostly on
stress tests and auto-generated test cases rather than commonly seen code).

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-11-03.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Target extension](https://github.com/rust-lang/rfcs/pull/2048)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)
* [disposition: merge] [consider assignments of union field of ManuallyDrop type safe](https://github.com/rust-lang/rust/pull/78068)
* [disposition: merge] [repr(transparent) on generic type skips "exactly one non-zero-sized field" check](https://github.com/rust-lang/rust/issues/77841)
* [disposition: merge] [Rename/Deprecate LayoutErr in favor of LayoutError](https://github.com/rust-lang/rust/pull/77691)
* [disposition: merge] [Tracking Issue for raw_ref_macros](https://github.com/rust-lang/rust/issues/73394)
* [disposition: merge] [Add checking for no_mangle to unsafe_code lint](https://github.com/rust-lang/rust/pull/72209)

## New RFCs
* [Checking conditional compilation at compile time](https://github.com/rust-lang/rfcs/pull/3013)

# Upcoming Events

### Online
* [November 4. Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/274142374/)
* [November 4. Dublin, IE - Rust Dublin November - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/274202454/)
* [November 4. Indianapolis, IN, US - Indy.rs - with Social Distancing - Indy.rs](https://www.meetup.com/indyrs/events/jhfstrybcpbgb/)
* [November 7 & 8, Global, RustFest Global](https://rustfest.global/)
* [November 10, Seattle, WA, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybcpbnb/)
* [November 10, Saarbücken, Saarland, DE - Meetup: 5u16 (virtual) - Rust Saar](https://www.meetup.com/de-DE/Rust-Saar/events/273949461/)
* [November 12, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcpbqb/)
* [November 12, Washington, DC, US - Mid-month Rustful—How oso built a runtime reflection system for Rust - Rust DC](https://www.meetup.com/RustDC/events/273813659)
* [November 12, Lehi, UT, US - WASM, Rust, and the State of Async/Await - Utah Rust](https://www.meetup.com/utah-rust/events/273757338/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer (IoT/Robotics) at Wayve (London, UK)](https://boards.greenhouse.io/wayve/jobs/4881949002)
* [Software Engineer at ChainSafe Systems (Toronto, Remote)](https://www.notion.so/chainsafe/Blockchain-Developer-Rust-0d577a2636b84511a5d4efc69454585d)
* [Senior Software Engineer - Rust at Immunant (Remote US)](https://immunant.com/jobs)
* [Backend Engineer - Rust at Kraken (Remote NA, SA, EMEA)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Rust Engineer, Desktop GUI - Cryptowatch at Kraken (Remote)](https://jobs.lever.co/kraken/2442ee5c-56b6-4a73-a477-8cdda2b218d5)
* [Senior Backend Engineer - Rust at Kraken (Remote NA, SA, EMEA)](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105)
* [Senior Full Stack Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)
* [Software Engineer - Trading Technology (Rust) at Kraken (Remote NA, SA, EMEA)](https://jobs.lever.co/kraken/4485f672-dc5f-4e49-a10b-2b0399e28a8d)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Like other languages Rust does have footguns. The difference is that we keep ours locked up in the unsafe.

– [Ted Mielczarek on twitter](https://twitter.com/TedMielczarek/status/1322618223980892161)

Thanks to [Nikolai Vazquez](https://users.rust-lang.org/t/twir-quote-of-the-week/328/956) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/jk35ha/this_week_in_rust_362/)</small>
