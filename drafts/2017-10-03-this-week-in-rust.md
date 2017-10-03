Title: This Week in Rust 202
Number: 202
Date: 2017-10-03
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

[This year in the Amethyst game engine](https://www.amethyst.rs/2017/09/16/twia-10.html)

# Crate of the Week

Sadly, no one suggested a crate for the week.

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

100 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-09-25..2017-10-02

* [fix native main() signature on 64bit](https://github.com/rust-lang/rust/pull/44906)
* [add suggestions for misspelled method names](https://github.com/rust-lang/rust/pull/44297)
* [code suggestions for unused-mut, while-true, deprecated-attribute, and unused-parens lints](https://github.com/rust-lang/rust/pull/44942)
* [allow unused extern crate again](https://github.com/rust-lang/rust/pull/44825) (backs out until diagnostics are fixed)
* [friendlier error message for closure argument type mismatch](https://github.com/rust-lang/rust/pull/44735)
* [macros: fix bug in collecting trait and impl items with derives](https://github.com/rust-lang/rust/pull/44757)
* [apply attr proc macros before cfg processing](https://github.com/rust-lang/rust/pull/44528)
* [handle nested generics in `Generics::type_param`/`region_param`](https://github.com/rust-lang/rust/pull/44959)
* [encode region::Scope using fewer bytes](https://github.com/rust-lang/rust/pull/44809)
* [initial support for `..=` syntax](https://github.com/rust-lang/rust/pull/44709)
* [some fixes to mir-borrowck](https://github.com/rust-lang/rust/pull/44736)
* [allow replacing HashMap entries](https://github.com/rust-lang/rust/pull/44278)
* [`impl<T, U> TryFrom<T> for U where U: From<T>`](https://github.com/rust-lang/rust/pull/44174)
* [`impl<T> Try for Option<T>](https://github.com/rust-lang/rust/pull/42526) (this was a long time coming)
* [do not require semantic types for all syntactic types when there are errors](https://github.com/rust-lang/rust/pull/44945)
* [add more custom folding to `core::iter` adaptors](https://github.com/rust-lang/rust/pull/44856)
* [trustedRandomAccess specialisation for Iterator::cloned when Item: Copy](https://github.com/rust-lang/rust/pull/44790)
* [fix capacity comparison in `VecDeque::reserve`](https://github.com/rust-lang/rust/pull/44802)

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
