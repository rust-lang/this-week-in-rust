Title: This Week in Rust 206
Number: 206
Date: 2017-10-31
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

* [Fun facts about Rust's growing popularity](http://www.jonathanturner.org/2017/10/fun-facts-about-rust-growth.html).
* [Cargo alternative registries](https://boats.gitlab.io/blog/post/2017-10-28-alternative-registries/).
* [A game developer's experience with writing a plant simulation game in Rust](https://www.reddit.com/r/rust/comments/795dg4/i_spent_the_last_year_writing_a_plant/).
* [A brief history of Rust](http://slides.com/bstrie/triangle-rust-history).
* [The Expressive C++17 coding challenge… in Rust](http://words.steveklabnik.com/the-expressive-c-17-coding-challenge-in-rust).
* [Announcing bindgen 0.31.0](https://www.reddit.com/r/rust/comments/795n0k/announcing_bindgen_0310/).
* [Pleco: Creating a Chess engine with Rust](https://sfleischman105.github.io/2017/10/26/creating-a-chess-engine.html).
* [Swagger.io can now autogenerate Rust servers from OpenAPI specifications](https://www.metaswitch.com/the-switch/metaswitch-swagger-codegen-for-rust-accepted-upstream).
* [This week in Rust docs 79](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-79).
* [podcast] [Rusty Spike Podcast - Episode 5](https://rusty-spike.blubrry.net/2017/10/25/episode-5-oct-25-2017/). A new game written in Rust, improvements to WASM support, a JIT compiler in Rust, and Gstreamer support in Servo.s

# Crate of the Week

This week's crate is [cargo-outdated](https://crates.io/crates/cargo-outdated), a cargo subcommand that shows outdated dependencies including latest compatible
and latest version. Thanks to [Colin Kiegel](https://users.rust-lang.org/u/colin_kiegel) for the suggestion.

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

111 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-10-23..2017-10-30

* [ci: upgrade Android SDK/NDK and refactor to use sdkmanager/avdmanager](https://github.com/rust-lang/rust/pull/45580)
* [implement RFC 1861: Extern types](https://github.com/rust-lang/rust/pull/44295)
* [avoid unnecessary copies of arguments that are simple bindings](https://github.com/rust-lang/rust/pull/45380)
* [add short error message-format](https://github.com/rust-lang/rust/pull/44636)
* [add several lints into `unused` lint group](https://github.com/rust-lang/rust/pull/45424)
* [resolve types properly in const eval](https://github.com/rust-lang/rust/pull/45488)
* [create NormalizeTy query](https://github.com/rust-lang/rust/pull/44984)
* [`crate` shorthand visibility modifier](https://github.com/rust-lang/rust/pull/45401)
* [move Generics from MethodSig to TraitItem and ImplItem](https://github.com/rust-lang/rust/pull/44766)
* [add generics to LateContext](https://github.com/rust-lang/rust/pull/45611)
* [don't emit the same compiler diagnostic twice](https://github.com/rust-lang/rust/pull/45519)
* [improve diagnostics when list of tokens has incorrect separators](https://github.com/rust-lang/rust/pull/45503)
* [use 128 bit instead of Symbol for crate disambiguator](https://github.com/rust-lang/rust/pull/45476)
* [remove dependency tracking for variance computation](https://github.com/rust-lang/rust/pull/45473)
* [implement Hash for raw pointers to unsized types](https://github.com/rust-lang/rust/pull/45483)
* [visit attribute tokens in `DefCollector` and `BuildReducedGraphVisitor`](https://github.com/rust-lang/rust/pull/45464)
* [remove deprecated `collections` crate](https://github.com/rust-lang/rust/pull/45446)
* [fix 32- vs 64-bit platform instability in StableHasher](https://github.com/rust-lang/rust/pull/45522)
* [std: optimize thread park/unpark implementation](https://github.com/rust-lang/rust/pull/45524)
* [std: disable usage of mmap allocator in libbacktrace ](https://github.com/rust-lang/rust/pull/45523)
* [add current_pid function](https://github.com/rust-lang/rust/pull/45059)
* [cargo: add unit test checking to `cargo check`](https://github.com/rust-lang/cargo/pull/4592)
* [cargo: improving the error message for when a patched dependency does not resolve to anything](https://github.com/rust-lang/cargo/pull/4607)
* [cargo: alternative registries](https://github.com/rust-lang/cargo/pull/4506)
* [rustdoc: Show src button and function version on mobile version ](https://github.com/rust-lang/rust/pull/45502)

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
