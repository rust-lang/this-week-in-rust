Title: This Week in Rust 120
Number: 120
Date: 2016-02-29
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts


* [Comparing Rust and Java](https://llogiq.github.io/2016/02/28/java-rust.html).
* [Performance analysis](https://blog.ethcore.io/performance-analysis/) of Ethereum clients Parity (Rust) and Geth (Go).
* [Programming for Servo: Experience and knowledge gained](https://nikkisquared.github.io/2016/02/22/things-ive-learned.html).
* [Parallel iterators part 2: Producers](http://smallcultfollowing.com/babysteps/blog/2016/02/25/parallel-iterators-part-2-producers/).
* [video] [Ferris makes emulators: Episode 6](https://www.youtube.com/watch?v=TmNfPEb-ijo). Live stream of Ferris developing a N64 emulator in Rust (also on [Twitch](http://www.twitch.tv/ferrisstreamsstuff/profile)).
* [Query2 in timely dataflow](http://www.poumeyrol.fr/2016/02/22/Query2-in-timely/). Part #6 of a series about a BigData in Rust experiment.
* [opinion] [Rewrite Everything In Rust](http://robert.ocallahan.org/2016/02/rewrite-everything-in-rust.html).
* [Deciding to rewrite getaddrinfo in Rust](http://blog.dkhenry.com/2016/02/17/deciding-to-rewrite-getaddrinfo-in-rust/).
* [Testing Rust on iOS with Travis](http://sasheldon.com/blog/2016/02/22/testing-rust-on-ios-with-travis/).
* [Create GitHub releases with Rust using Hubcaps](https://fnordig.de/2016/02/23/create-releases-using-hubcaps-a-rust-library/).
* [More type-level shenanigans](https://llogiq.github.io/2016/02/23/moretypes.html).
* [Measuring Rust Runtime Performance: cargo bench vs. getrusage()](https://dikaiosune.github.io/rust-runtime-cargobench-vs-getrusage.html).
* [The highs and lows of Rust](https://www.jimmycuadra.com/posts/the-highs-and-lows-of-rust/).
* [This week in Amethyst 6](https://thisweekinamethyst.wordpress.com/2016/02/22/twia-6/).
* [This week in Servo 52](http://blog.servo.org/2016/02/22/twis-52/).

## Notable New Crates & Project Updates

* You can now output the generated MIR on [play.rust-lang.org](https://play.rust-lang.org/?gist=fee8ccf28bae2c89107d&version=nightly).
* [Serde 0.7 - Changelog](https://erickt.github.io/blog/2016/02/26/serde-0-dot-7/).
* [Mio: Proposal to unify sockets, timers, and channels](https://github.com/carllerche/mio/issues/360).
* [rust-everywhere](https://github.com/japaric/rust-everywhere). Use CI services to generate binary releases of your Rust program for Linux, Mac and Windows.
* [rust-bisect](https://github.com/kamalmarhubi/rust-bisect). Find the Rust nightly that that changed some behavior.
* [Polly](https://gitlab.com/Polly-lang/Polly). A truly logic-less templating language for Rust servers.
* [mrusty](https://github.com/anima-engine/mrusty). mruby safe bindings for Rust.
* [glitter](https://github.com/kylewlacy/glitter). A safe, low-level, zero-cost Rust OpenGL wrapper library.

# Crate of the Week

This week's Crate of the Week is [rotor](https://crates.io/crates/rotor), a [mio](https://crates.io/crates/mio)-based async-IO library providing an event loop, state machine combinators and futures.

Thanks to [LilianMoraru](https://users.rust-lang.org/users/LilianMoraru) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Easy] [Rust: repr(i64) picks 32-bit discriminant on 32-bit platform](https://github.com/rust-lang/rust/issues/26114).
* [Easy] [Rust: -C suggestions to use llc for details is annoying](https://github.com/rust-lang/rust/issues/30961).
* [Easy] [Rust: JSON errors with suggestions are incomplete](https://github.com/rust-lang/rust/issues/30701).
* [Easy] [Rust: All callable types could have better error messages in the "no method found" case to suggest that you may have forgotten to actually call them](https://github.com/rust-lang/rust/issues/29124).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821


# Updates from Rust Core

95 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-02-22..2016-02-29

## Notable changes

* [Make for loop desugaring for iterators more precise](https://github.com/rust-lang/rust/pull/31942).
* [Avoid excessive reallocations during item-bodies checking](https://github.com/rust-lang/rust/pull/31929).
* [more check_*_post methods for LintPasses](https://github.com/rust-lang/rust/pull/31926).
* [Make sure formatter errors are emitted by the default `Write::write_fmt`](https://github.com/rust-lang/rust/pull/31904).
* [rand: Fix `OsRng::fill_bytes` on Windows](https://github.com/rust-lang/rust/pull/31876).
* [std: Cap read/write limits on Windows networking](https://github.com/rust-lang/rust/pull/31858).
* [Resolve: fix scoping bug (fixes #31845) and improve unused import detection](https://github.com/rust-lang/rust/pull/31857).
* [Update libc to `16f1c19`](https://github.com/rust-lang/rust/pull/31832).
* [Add unstable `copy_from_slice`](https://github.com/rust-lang/rust/pull/31834).
* [mk: Move disable-jemalloc logic into makefiles](https://github.com/rust-lang/rust/pull/31846).
* [Resolve: Fix the visibility of extern crates](https://github.com/rust-lang/rust/pull/31362).
* [Implement `Clone` for `std::vec::IntoIter`](https://github.com/rust-lang/rust/pull/31704).
* [std: Use Android LFS `off64_t`, `ftruncate64`, and `lseek64`](https://github.com/rust-lang/rust/pull/31805).
* [Implement `compare_exchange` and `compare_exchange_weak`](https://github.com/rust-lang/rust/pull/30969).
* [Recognize `#[thread_local]` attr on extern static. Fixes #30795](https://github.com/rust-lang/rust/pull/30856)
* [`CStr::from_bytes_with_nul`](https://github.com/rust-lang/rust/pull/30614).

## New Contributors

* dileepb
* Katze
* Kevin Stock
* Michael Huynh
* Stephan Hügel
* tormol

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1210: Propose a design for _specialization_, which permits multiple `impl` blocks to apply to the same type/trait](https://github.com/rust-lang/rfcs/pull/1210).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

*No RFCs are currently in final comment period.*

## New RFCs

* [Add a new crate-type, rdylib](https://github.com/rust-lang/rfcs/pull/1510).
* [Stabilize implementing panics as aborts](https://github.com/rust-lang/rfcs/pull/1513).
* [Exclude macros from importing with `#[macro_use(not(...))]`](https://github.com/rust-lang/rfcs/pull/1517).
* [Add space-friendly arguments](https://github.com/rust-lang/rfcs/pull/1509). Add `-C link-arg` and `-C llvm-arg` which allows you to pass along argument with spaces.

# Upcoming Events

* [3/1. Rust Detroit presents Hadoop Next Gen: Using Cloudera Kudu and Mozilla Rust to Crunch Big Data](http://www.meetup.com/rust-detroit/events/224586618/).
* [3/2. Rust Amsterdam - Talks and Drinks](http://www.meetup.com/Rust-Amsterdam/events/227827508/).
* [3/2. Cologne: Live Rust-Coding](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/229013352/?eventId=229013352&chapter_analytics_code=UA-63812876-1).
* [3/3. Rust São Paulo Meetup](http://www.meetup.com/Rust-Sao-Paulo-Meetup/events/228868463/).
* [3/9. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [3/9. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [3/10. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [3/11. Darmstadt Rust Table of Regulars](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/228665878/).
* [3/14. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# GSoc Projects

Hi students! Looking for an awesome summer project in Rust? Look no further! Chris Holcombe from Canonical is an experienced GSoC mentor and has a project to implement CephX protocol decoding. [Check it out here](https://wiki.ubuntu.com/GoogleSoC2016/Ideas#Decode_CephX_Protocol).

Servo is also accepting project submissions under the Mozilla banner. See if any of the [project ideas](https://wiki.mozilla.org/Community:SummerOfCode16#Servo) appeal to you and read the [advice for applications](https://wiki.mozilla.org/Community:SummerOfCode16#Application_Advice).

# Quote of the Week

> Haskell: When you want to feel smarter than everyone else in the universe.
>
> Rust: When you want to feel like an idiot because even a mere compiler is smarter than you.

— [Manishearth on /r/rust](https://www.reddit.com/r/rust/comments/47ucw9/comparing_haskell_and_rust_which_to_choose_when/d0fsjvu).

Thanks to [Sinistersnare](https://users.rust-lang.org/users/sinistersnare) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
