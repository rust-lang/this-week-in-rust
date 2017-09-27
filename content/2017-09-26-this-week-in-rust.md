Title: This Week in Rust 201
Number: 201
Date: 2017-09-26
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

* [Security advisory for crates.io, 2017-09-19](https://users.rust-lang.org/t/security-advisory-for-crates-io-2017-09-19/12960).
* [kennytm joins the Rust infrastructure team](https://internals.rust-lang.org/t/please-welcome-kennytm-to-the-infrastructure-team/5944).
* [How the RLS works](https://www.ncameron.org/blog/how-the-rls-works/).
* [An RFC for a Tokio revamp](https://tokio.rs/blog/tokio-reform/).
* [Rewriting a Java application in Rust](https://blog.rom1v.com/2017/09/gnirehtet-rewritten-in-rust/).
* [The impl period newsletter, week 1](https://internals.rust-lang.org/t/the-impl-period-newsletter-week-1/5971).
* [This week in Rust docs 74](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-74).
* [These weeks in dev-tools, issue 2](https://www.ncameron.org/blog/these-weeks-in-dev-tools-issue-2/).
* [This week in Redox 29](https://redox-os.org/news/this-week-in-redox-29/).

# Crate of the Week

This week's crate is [rustbreak](https://crates.io/crates/rustbreak), a crate providing simple single-file storage to e.g. persist settings.
Thank you, [Dieter Konrad](https://users.rust-lang.org/u/dkotrada) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [impl period opportunities: rustdoc](https://quietmisdreavus.net/code/2017/09/18/come-work-on-rustdoc/).
* [impl period opportunities: bindgen](http://fitzgeraldnick.com/2017/09/21/come-hack-on-bindgen-with-us.html).
* [Neon - a library for writing native Node.js modules is looking for contributors](http://calculist.org/blog/2017/09/25/neon-wants-your-help/).
* [imag - a personal information management suite needs help with 0.5.0 milestone](https://github.com/matthiasbeyer/imag/milestone/5).
* [Help with expanding UNIC’s components for the Unicode Character Database](https://github.com/behnam/rust-unic/issues/158).
* [rsmt2 - library to interact with SMT-LIB 2 compliant solvers is looking for contributors](https://github.com/kino-mc/rsmt2/issues).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

157 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-09-18..2017-09-25

* [incr.comp.: Add new DepGraph implementation](https://github.com/rust-lang/rust/pull/44772)
* [Use memoized hashes when hashing Fingerprint](https://github.com/rust-lang/cargo/pull/4521)
* [Don't visit the same unit multiple times](https://github.com/rust-lang/cargo/pull/4520) (fixes perf regression)
* [Make the fallback of generator resumption be unreachable instead of using return](https://github.com/rust-lang/rust/pull/44747)
* [Include unary operator to span for `ExprKind::Unary`](https://github.com/rust-lang/rust/pull/44746)
* [Make `-Z borrowck-mir` imply that `EndRegion`'s should be emitted](https://github.com/rust-lang/rust/pull/44717)
* [Move effect-checking to MIR](https://github.com/rust-lang/rust/pull/44700)
* [Implement underscore lifetimes](https://github.com/rust-lang/rust/pull/44691)
* [Add iterator method `.rfold(init, function)` the reverse of fold](https://github.com/rust-lang/rust/pull/44682)
* [rustbuild: with --no-fail-fast, report the specific commands that failed](https://github.com/rust-lang/rust/pull/44680)
* [Add clippy to `toolstate.toml`](https://github.com/rust-lang/rust/pull/44679) (one more step to a stable clippy)
* [Compress most of spans to 32 bits](https://github.com/rust-lang/rust/pull/44646) (memory savings + modest speedup)
* [Improve diagnostics when attempting to match tuple enum variant with struct pattern](https://github.com/rust-lang/rust/pull/44786)
* [Stabilized vec_splice and modified splice tracking issue](https://github.com/rust-lang/rust/pull/44640)
* [Record semantic types for all syntactic types in bodies](https://github.com/rust-lang/rust/pull/44633)
* [`--cap-lints allow` switches off `can_emit_warnings`](https://github.com/rust-lang/rust/pull/44627)
* [Forbid interpolated tokens in the HIR](https://github.com/rust-lang/rust/pull/44601)
* [cargo_compile: iterate packages once, not three times](https://github.com/rust-lang/cargo/pull/4494)
* [Add pub visibility for methods as well](https://github.com/rust-lang/rust/pull/44554)
* [Implement `Copy`/`Clone` for closures](https://github.com/rust-lang/rust/pull/44551)
* [Refactor translation unit partitioning/collection as a query](https://github.com/rust-lang/rust/pull/44529)
* [Correctly bubble up errors from libbacktrace](https://github.com/rust-lang/rust/pull/44525)
* [Add `Cow<str>` → `Box<Error>` impls](https://github.com/rust-lang/rust/pull/44466)
* [Add Duration::from_micros](https://github.com/rust-lang/rust/pull/44436)
* [Only consider yields coming after the expressions when computing generator interiors](https://github.com/rust-lang/rust/pull/44392)
* [Optimize drain_filter](https://github.com/rust-lang/rust/pull/44355)
* [Improve how rustdoc warnings are displayed](https://github.com/rust-lang/rust/pull/44350)
* [Require rlibs for dependent crates when linking static executables](https://github.com/rust-lang/rust/pull/44279)
* [don't suggest placing `use` statements into expanded code](https://github.com/rust-lang/rust/pull/44215)
* [add comparison operators to must-use lint](https://github.com/rust-lang/rust/pull/44103)
* [Allow writing metadata without llvm](https://github.com/rust-lang/rust/pull/44085)
* [only set non-ADT derive error once per attribute, not per trait](https://github.com/rust-lang/rust/pull/44055) (yay for more focused error messages)
* [Add deref suggestion](https://github.com/rust-lang/rust/pull/43870)

## New Contributors

* Basile Desloges
* Bob Sun
* James Tucker
* Lucas Morales
* Marcus Buffett
* P.Y. Laligand
* Romain Porte

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2045: target_feature](https://github.com/rust-lang/rfcs/pull/2045).
* [RFC 2011: generic_assert: Make the `assert!` macro recognize more expressions](https://github.com/rust-lang/rfcs/pull/2011).
* [RFC 1990: Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Non-lexical lifetimes](https://github.com/rust-lang/rfcs/pull/2094).
* [disposition: merge] [Support defining C-compatible variadic functions in Rust](https://github.com/rust-lang/rfcs/pull/2137).
* [disposition: merge] [Add support to Cargo for alternative registries](https://github.com/rust-lang/rfcs/pull/2141).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: postpone] [Infer function signatures from trait declaration into 'impl's](https://github.com/rust-lang/rfcs/pull/2063).
* [disposition: postpone] [Tuple-based variadic generics](https://github.com/rust-lang/rfcs/pull/1935).
* [disposition: postpone] [`'fn` lifetime ascription](https://github.com/rust-lang/rfcs/pull/1847). Add a `'fn` lifetime that is bound to the scope of the body of the current innermost function or closure.
* [disposition: postpone] [Default struct field values](https://github.com/rust-lang/rfcs/pull/1806).
* [disposition: postpone] [Introduce `Option::<&T>::borrowed`](https://github.com/rust-lang/rfcs/pull/1792).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

* [Sep 28. Mozilla Community Dresden - Rust Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/242610304/).
* [Sep 28. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/241234876/).
* **[Sep 30 - Oct 1. RustFest Zürich](http://zurich.rustfest.eu).**
* [Sep 30. Rust Bangalore IO and Error Handling Workshop](https://www.meetup.com/rustox/events/243364708/).
* [Sep 30. Rust Mexico #8: Taller Introductorio a Rust y Rocket](https://www.meetup.com/Rust-MX/events/243334902/).
* [Oct  2 - Oct 3. Impl Days at RustFest Zürich](https://github.com/RustFestEU/blog.rustfest.eu/issues/29).
* [Oct  4. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct  4. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct  4. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/243164851/).
* [Oct  4. Rust Cologne - Open Space](https://www.meetup.com/RustCologne/events/243156120/).
* [Oct  4. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/243084182/).
* [Oct  5. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Oct  7. Rust Bangalore SQL Data Binding Workshop](https://www.meetup.com/rustox/events/243387585/).
* [Oct 11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct 11. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct 12. Rust Washington DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/243672292/).
* [Oct 12. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/243389836/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust developers at Æternity](https://blog.aeternity.com/join-the-t%C3%A6m-rust-or-erlang-devs-wanted-31908daba788).
* [Rust web developer - remote position](https://www.reddit.com/r/rust/comments/717rk2/hiring_rust_web_developer_contractor_remote/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> A Box always holds exactly one thing, like a single large struct. A Vec holds zero to many things of exactly one type and can change over time. If you had to relate them, a Box is a Vec with one element that went to Neverland and forgot it could ever grow.

— [/u/zzyzzyxx on reddit](https://www.reddit.com/r/rust/comments/70szta/hey_rustaceans_got_an_easy_question_ask_here/dncs4wa/?context=3).

Thanks to [/u/l-arkham](https://www.reddit.com/r/rust/comments/70szta/hey_rustaceans_got_an_easy_question_ask_here/dncs4wa/) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
