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

* [It's gfx-rs aLL the way down](https://users.rust-lang.org/t/its-gfx-rs-all-the-way-down/13339) - an interim update on the LL rewrite progress.

# Crate of the Week

Despite there being no votes, the crate of this week is [abrute](https://crates.io/crates/abrute), a crate to brute-force AES keys.
Thanks to [Daniel P. Clark](https://users.rust-lang.org/u/danielpclark) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help test out ThinLTO](https://internals.rust-lang.org/t/help-test-out-thinlto/6017)!
* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [Hacktoberfest and the impl period](http://blog.community.rs/2017/10/02/hacktoberfest-and-the-impl-period.html).
* [medium] [allocators-rs: Travis: Spurious test failures on Mac](https://github.com/ezrosent/allocators-rs/issues/36).
* [tarpaulin: Update outdated dependencies](https://github.com/xd009642/tarpaulin/issues/45).
* [Join us at FOSDEM 2018: devroom CFP](https://rust-fosdem.github.io)

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

* Andreas Jonson
* Barret Rennie
* Garrett Berg
* hinaria
* James Munns
* Kevin Hunter Kesling
* leavehouse
* Maik Klein
* mchlrhw
* Nikolai Vazquez
* Niv Kaminer
* Pirh
* Stephane Raux
* Suriyaa ✌️️

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

* [Handle borrows of unions in NLL](https://github.com/rust-lang/rfcs/pull/2168).

# Upcoming Events

* [Ocr 12. San Diego Rust October Meetup - Beginner's Study Session](https://www.meetup.com/San-Diego-Rust/events/243960032/).
* [Oct 12. Rust Washington DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/243672292/).
* [Oct 12. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/243389836/).
* [Oct 16. Rust London - Rust learning and hacking evening #4](https://www.meetup.com/Rust-London-User-Group/events/244021078/).
* [Oct 16. Rust Roma - Rust learning and hacking evening #2](https://www.meetup.com/it-IT/Rust-Roma/events/243948917/).
* [Oct 18. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/243663198/).
* [Oct 18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct 18. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct 19. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Oct 21. Mozilla Brasil - Mergulhando em Rust — O futuro da programação de sistemas #4](https://www.eventbrite.com.br/e/mergulhando-em-rust-o-futuro-da-programacao-de-sistemas-4-registration-38145874337).
* [Oct 23. Durham, NC - Triangle Rustaceans - with Rust Community Team member Ben Striegel](https://www.meetup.com/triangle-rustaceans/events/243586365/).
* [Oct 25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct 25. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct 26. Finland Rust-lang group October meetup](https://www.meetup.com/Finland-Rust-Meetup/events/243886850/).
* [Oct 26. Mozilla Community Dresden - Rust meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/243915635/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Multiple job openings mentioning Rust](https://www.reddit.com/r/rust/comments/743qal/15_job_openings_mentioning_rust_in_this_months/).
* [Rust Research Intern - University 2018 - Mozilla](https://careers.mozilla.org/position/gh/864822).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The long compile breaks give me time to focus on TV.

— [/u/staticassert on watching TV while programming in Rust](https://www.reddit.com/r/rust/comments/74tyqx/what_are_reasons_to_spend_time_on_rust/do1bzpx/).

Thanks to [/u/tomwhoiscontrary and /u/kixunil](https://www.reddit.com/r/rust/comments/74tyqx/what_are_reasons_to_spend_time_on_rust/do4n5xq/?context=3) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
