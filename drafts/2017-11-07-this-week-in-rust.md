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

* [AMA with Rust Language team](https://hashnode.com/ama/with-rust-language-team-cj99mv7s101yw4rwtk5zntk8k).
* [Accepted RFCs are now available to read in a book format](https://rust-lang.github.io/rfcs/).
* [Multithreading in Rust with MPSC (Multi-Producer, Single Consumer) channels](https://blog.softwaremill.com/multithreading-in-rust-with-mpsc-multi-producer-single-consumer-channels-db0fc91ae3fa).
* [Obtaining and using Microsoft's link.exe on Linux+Wine with Rust](https://gist.github.com/est31/7235ab253554d33046873dfb64e7ecdc).
* [Custom static assertions in Rust](https://nikolaivazquez.com/posts/programming/rust-static-assertions/).
* [When will the RLS be released](https://www.ncameron.org/blog/when-will-the-rls-be-released/)?
* [Portability concerns with Path](https://udoprog.github.io/rust/2017-11-05/portability-concerns-with-path.html).
* [Simple event hooks in Rust](https://mattgathu.github.io/simple-events-hook-rust/).
* [This week in Rust docs 80](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-80).
* [video playlist] [Talks from RustFest Zürich 2017](https://www.youtube.com/watch?v=jywiVWKm1TI&list=PL85XCvVPmGQj9mqbJizw-zi-EhcpS5jTP).
* [podcast] [Rusty Spike Podcast - Episode 6](https://rusty-spike.blubrry.net/2017/11/02/episode-6-nov-1-2017/). Cisco using Rust, StackOverflow’s survey, alternative registries, more NLL news, fun facts about Rust, and SpiderMonkey and Rust.

# Crate of the Week

This week's crate is [cargo-outdated](https://crates.io/crates/cargo-outdated), a cargo subcommand that shows outdated dependencies including latest compatible
and latest version. Thanks to [Colin Kiegel](https://users.rust-lang.org/u/colin_kiegel) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help us benchmark incremental compilation](https://internals.rust-lang.org/t/help-us-benchmark-incremental-compilation/6153)!
* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [MusicBrainz crate is looking for maintainers](https://www.reddit.com/r/rust/comments/7a2nq4/looking_for_potential_maintainer_to_musicbrainz/).
* [docs] [rust-ffi-guide: CString.as_ptr() problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/50).
* [docs] [rust-ffi-guide: Enum from integer is UB problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/51).
* [docs] [rust-ffi-guide: More examples of UB and FFI footguns](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/52).
* [docs] [rust-ffi-guide: Beware of allocators mismatch](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/53).
* [docs] [rust-ffi-guide: Rust enums and tagged unions](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/54).
* [docs] [rust-ffi-guide: Unwinding problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/49).
* [docs] [rust-ffi-guide: Linker errors and name mangling problem](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/48).

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

* David Wood
* Jonathan Behrens
* Lance John
* laurent
* matt rice
* Rolf Karp

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

* [Semantic build scripts for Cargo](https://github.com/rust-lang/rfcs/pull/2196).
* [Constants in array repeat expressions](https://github.com/rust-lang/rfcs/pull/2203).
* [The ConstDefault trait](https://github.com/rust-lang/rfcs/pull/2204).
* [`#[derive unfinished(..)]` and `#[unfinished]`](https://github.com/rust-lang/rfcs/pull/2205).

# Upcoming Events

* [Nov  9. Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/243672298/).
* [Nov  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/244164143/).
* [Nov  9. San Diego Rust November Meetup - Beginner's Training Session](https://www.meetup.com/San-Diego-Rust/events/244506375/).
* [Nov 13. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/244037662/).
* [Nov 15. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/244340757/).
* [Nov 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov 16. Cambridge Rust Meetup #5](https://www.meetup.com/Cambridge-Rust-Meetup/events/244114730/).
* [Nov 16. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Nov 19. Rust India Community Monthly Call](https://reps.mozilla.org/e/rust-india-monthly-call/).
* [Nov 21. Beginning Rust and Rust Hack Night @ Valtech Stockholm Sweden](https://www.meetup.com/ruststhlm/events/244792464/).
* [Nov 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust jobs from "Who's Hiring" thread on Hacker News](https://www.reddit.com/r/rust/comments/7adboi/17_rustrelated_job_openings_in_novembers_whos/).
* [Rust + Machine Learning at Etsy](https://www.reddit.com/r/rust/comments/7aoiod/job_etsy_rust_machine_learning/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> durka42: IMO the name "dangling" is scary enough :)
> Havvy gives durka42 a ptr::dangling::<Candy>().
> durka42 declines to unwrap() it

— [durka42 and Havvy](https://botbot.me/mozilla/rust-internals/2017-11-02/?msg=93047552&page=2) discussing [PR #45527](https://github.com/rust-lang/rust/pull/45527).

Thanks to [Centril](https://users.rust-lang.org/t/twir-quote-of-the-week/328/464) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
