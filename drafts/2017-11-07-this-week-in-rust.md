Title: This Week in Rust 207
Number: 207
Date: 2017-11-07
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

# Crate of the Week

This week's crate is [futures-await](https://crates.io/crates/futures-await), a crate to simplify writing futures-based async code. Thanks to [LilianMoraru](https://users.rust-lang.org/u/LilianMoraru) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [Call for help: Cargo "airplane" mode](https://internals.rust-lang.org/t/call-for-help-cargo-airplane-mode/6134). ([GitHub issue](https://github.com/rust-lang/cargo/issues/4686))
* [stdsimd: Implement all x86 vendor intrinsics](https://github.com/rust-lang-nursery/stdsimd/issues/40).
* [stdsimd: Implement all ARM NEON intrinsics](https://github.com/rust-lang-nursery/stdsimd/issues/148).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

125 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-10-30..2017-11-06

* [enable non-lexical lifetimes in the MIR borrow checker](https://github.com/rust-lang/rust/pull/45538)
* [fix MIR inlining panic in generic function](https://github.com/rust-lang/rust/pull/45723)
* [optimize some span operations](https://github.com/rust-lang/rust/pull/45602)
* [display spans correctly when there are zero-width or wide characters](https://github.com/rust-lang/rust/pull/45711)
* [typeck: suggest use of match_default_bindings feature](https://github.com/rust-lang/rust/pull/45409)
* [typeck: use subtyping on the LHS of binops](https://github.com/rust-lang/rust/pull/45435)
* [suggest renaming import if names clash](https://github.com/rust-lang/rust/pull/45660)
* [future-proofing enums/structs with `#[non_exhaustive]` attribute](https://github.com/rust-lang/rust/pull/45394) (RFC [#2008](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html))
* [implement auto trait syntax](https://github.com/rust-lang/rust/pull/45247)
* [incr.comp.: Implement compiler diagnostic persistence](https://github.com/rust-lang/rust/pull/45472)
* [incr.comp.: Fix two problems with HIR hashing](https://github.com/rust-lang/rust/pull/45551)
* save-analysis: [emit crate disambiguators](https://github.com/rust-lang/rust/pull/45468),
                 [fixes](https://github.com/rust-lang/rust/pull/45709)),
                 [more fixes](https://github.com/rust-lang/rust/pull/45798),
                 [union support](https://github.com/rust-lang/rust/pull/45647)
* [`BufReader::is_empty(..)`](https://github.com/rust-lang/rust/pull/45369)
* [bring back `slice::ref_slice` as `slice::from_ref`](https://github.com/rust-lang/rust/pull/45306)
* [remove the `T: Sync` requirement for `RwLock<T>: Send`](https://github.com/rust-lang/rust/pull/45267)
* [implement `TryFrom<&[T]>` for `&[T; N]`](https://github.com/rust-lang/rust/pull/44764)
* [copy all `AsciiExt` methods to the primitive types directly in order to deprecate it later](https://github.com/rust-lang/rust/pull/44042)
* [impl `From<T>` for `AtomicT`](https://github.com/rust-lang/rust/pull/45610)
* [next_power_of_two now panics on overflow](https://github.com/rust-lang/rust/pull/45754)
* [cargo: support uninstallation of multiple packages](https://github.com/rust-lang/cargo/pull/4561)
* [rustdoc: improve sidebar rendering and add methods list](https://github.com/rust-lang/rust/pull/45187)

## New Contributors

* Dustin Speckhals
* Igor Matuszewski
* John Paul Adrian Glaubitz
* Josh Leeb-du Toit
* Laurent Arnoud
* Nadav Zingerman
* Paul Liétar
* Thayne McCombs
* Virgil Palanciuc

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

* [Formally define repr(X) on enums with payloads](https://github.com/rust-lang/rfcs/pull/2195).
* [TryClone trait](https://github.com/rust-lang/rfcs/pull/2189).

# Upcoming Events

* [Nov  2. Live AMA with Rust Core Team members](https://hashnode.com/ama/with-rust-language-team-cj99mv7s101yw4rwtk5zntk8k).
* [Nov  2. Rust Bay Area - Zero Knowledge Proof Macros and Cernan (data pipelining)](https://www.meetup.com/Rust-Bay-Area/events/244156617/).
* [Nov  2. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Nov  4. Rust Bangalore - Rust Concurrency Workshop](https://www.meetup.com/rustox/events/240879563/).
* [Nov  5. DevFest Ahmedabad 2017: Dive Into Rust](http://devfest.gdgahmedabad.com/).
* [Nov  8. Rust Roma - Rust learning and hacking evening #3](https://www.meetup.com/Rust-Roma/events/244508431/).
* [Nov  8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov  8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov  9. Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/243672298/).
* [Nov  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/244164143/).
* [Nov  9. San Diego Rust November Meetup - Beginner's Training Session](https://www.meetup.com/San-Diego-Rust/events/244506375/).
* [Now 10. Rust Meetup Stuttgart (Germany)](https://blog.shackspace.de/?p=5723)
* [Nov 13. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/244037662/).
* [Nov 15. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/244340757/).
* [Nov 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov 16. Cambridge Rust Meetup #5](https://www.meetup.com/Cambridge-Rust-Meetup/events/244114730/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer at Fortanix, Mountain View, CA, US](https://www.fortanix.com/company/careers/#engineer)
* [Distributed Application Developer at Sphere Identity, Auckland, NZ](https://nz.linkedin.com/jobs/view/distributed-application-developers---blockchain-at-sphere-identity-ltd-442432632).
* [Full-time Rust position at Commure, San Francisco, US](https://news.ycombinator.com/item?id=15387799).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I feel like I'm doing something wrong because I'm programming faster and nothing has gone wrong yet

— [@0x424c41434b on Rust](https://twitter.com/0x424c41434b/status/923369121844043776).

Thanks to [@llogiq](https://twitter.com/llogiq/status/923431261523374081) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
