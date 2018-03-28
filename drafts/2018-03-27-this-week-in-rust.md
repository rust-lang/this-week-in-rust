Title: This Week in Rust 227
Number: 227
Date: 2018-03-27
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

* [Async/Await VI: 6 weeks of great progress](https://boats.gitlab.io/blog/post/2018-03-20-async-vi/).
* [Python idioms in Rust](http://benjamincongdon.me/blog/2018/03/23/Python-Idioms-in-Rust/).
* [Dora: Implementing a JIT-compiler with Rust](https://dinfuehr.github.io/blog/dora-implementing-a-jit-compiler-with-rust/).
* [Atomics and memory ordering](https://vorner.github.io/2018/03/25/Atomics.html).
* [Talks I'd love to see at RustConf](https://jvns.ca/blog/2018/03/24/rustconf-talks/).
* [Designing a fast CLI join tool with rust](https://milancio42.github.io/blog/2018-03-22-rjoin/)
* [Building a Restful CRUD API with Rust](https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8).
* [Touring a fast, safe, and complete(ish) web service in Rust](https://brandur.org/rust-web)
* [Putting bors on a performance improvement plan](https://aturon.github.io/2018/03/19/bors/).
* [Mutation based testing & code coverage](https://llogiq.github.io/2018/03/25/cover.html).
* [Redox OS 0.3.5 is released with a new network stack](https://github.com/redox-os/redox/releases/tag/0.3.5).
* [Announcing the Ecosystem Working Group](https://users.rust-lang.org/t/announcing-the-ecosystem-working-group/16324).
* [Rust 2018 All Hands is under way in Berlin](https://internals.rust-lang.org/t/rust-2018-all-hands/7141).
* [This Week in Rust Docs 98](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-98).
* [podcast] [Rusty Spike Podcast - episode 24](https://rusty-spike.blubrry.net/2018/03/22/episode-24-mar-21-2018/). Latin America, async/await, electron, and behind the scenes on Rust 2018.

# Crate of the Week

This week's crate is [fui](https://crates.io/crates/fui), a crate to add both a command-line interface and text forms to your program. Thanks to [musicmatze](https://users.rust-lang.org/u/musicmatze) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* If you’re interested in language design, please help us polish up [the new Delegation RFC draft](https://internals.rust-lang.org/t/new-rfc-for-delegation-anyone-interested-in-contributing/6644/8).
* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is a project to write a short guide about how the rust compiler works, and it needs your help. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

178 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-03-19..2018-03-26

* [prepare the 1.25.0 stable release](https://github.com/rust-lang/rust/pull/49340)
* [rustc: add a `#[wasm_custom_section]` attribute](https://github.com/rust-lang/rust/pull/48883)
* [add basic PGO support](https://github.com/rust-lang/rust/pull/48346) (Hooray!)
* [improve lint for type alias bounds](https://github.com/rust-lang/rust/pull/48909)
* [stabilize the copy_closures and clone_closures features](https://github.com/rust-lang/rust/pull/49299)
* [stabilize impl Trait](https://github.com/rust-lang/rust/pull/49255) (Huzzah!)
* [detect illegal hidden lifetimes in `impl Trait`](https://github.com/rust-lang/rust/pull/49041)
* [NLL should identify and respect the lifetime annotations that the user wrote](https://github.com/rust-lang/rust/pull/48482)
* [fix DefKey lookup for proc-macro crates](https://github.com/rust-lang/rust/pull/49273)
* [always print `aborting due to n previous error(s)`](https://github.com/rust-lang/rust/pull/49046)
* [better diagnostics for '..' pattern fragment not in the last position](https://github.com/rust-lang/rust/pull/49268)
* [support elision in impl headers](https://github.com/rust-lang/rust/pull/49251) (`'_`)
* [fix type_dependent_defs ICE on method calls](https://github.com/rust-lang/rust/pull/49244)
* [pass attributes to hir::TyParam](https://github.com/rust-lang/rust/pull/49242)
* [produce nice array lengths on a best effort basis](https://github.com/rust-lang/rust/pull/49262)
* [remove slow HashSet during miri stack frame creation](https://github.com/rust-lang/rust/pull/49274)
* [refactor the `BorrowckErrors` trait to take `fn(self)`](https://github.com/rust-lang/rust/pull/48902)
* [don't check interpret_interner when accessing a static to fix miri mutable statics](https://github.com/rust-lang/rust/pull/49216)
* [implement "Implemented-From-Env" Chalk lowering rule](https://github.com/rust-lang/rust/pull/49211)
* [implement Raw Identifiers](https://github.com/rust-lang/rust/pull/48942) (RFC #[2151](https://rust-lang.github.io/rfcs/2151-raw-identifiers.html))
* [fix the conversion between bit representations and i128 representations](https://github.com/rust-lang/rust/pull/49210)
* [encode/decode extern statics in metadata and incremental cache](https://github.com/rust-lang/rust/pull/49200)
* [extend stable hasher to support `CanonicalTy`](https://github.com/rust-lang/rust/pull/49091)
* [`Pin`, `Unpin`, `PinBox`](https://github.com/rust-lang/rust/pull/49058) (immovable types for generators)
* [make resuming generators unsafe instead of the creation of immovable generators](https://github.com/rust-lang/rust/pull/49194)
* [add hexadecimal formatting of integers with fmt::Debug](https://github.com/rust-lang/rust/pull/48978)
* [reduce the diagnostic spam when multiple fields are missing in pattern](https://github.com/rust-lang/rust/pull/49160)
* [add a -Z flag for LLVM align attributes on arguments](https://github.com/rust-lang/rust/pull/49122)
* [deprecate the AsciiExt trait in favor of inherent methods](https://github.com/rust-lang/rust/pull/49109)
* [stabilize termination_trait, split out termination_trait_test](https://github.com/rust-lang/rust/pull/49162)
* [stabilise FromUtf8Error::as_bytes](https://github.com/rust-lang/rust/pull/49121)
* [stabilize 128-bit integers](https://github.com/rust-lang/rust/pull/49101)
* [replace `convert::Infallible` with `!`](https://github.com/rust-lang/rust/pull/49038)
* [add `simd_select` intrinsic](https://github.com/rust-lang/rust/pull/49141)
* [add `BufReader::buffer`](https://github.com/rust-lang/rust/pull/49139)
* [suggest removing `&`s](https://github.com/rust-lang/rust/pull/48834)
* [implement Integer methods for Wrapping](https://github.com/rust-lang/rust/pull/48810)
* [stabilize slice patterns without `..`](https://github.com/rust-lang/rust/pull/48516)
* [implement `get_key_value` for {`HashMap`, `BTreeMap`}](https://github.com/rust-lang/rust/pull/49346)
* [fix vector fmin/fmax non-fast/fast intrinsics NaN handling](https://github.com/rust-lang/rust/pull/49231)
* [add 12 num::NonZero* types for primitive integers, deprecate core::nonzero](https://github.com/rust-lang/rust/pull/48265)
* [cargo: faster resolver: use a inverse-index to not activate the causes of conflict](https://github.com/rust-lang/cargo/pull/5213)
* [fix ordering of auto-generated trait bounds in rustdoc output](https://github.com/rust-lang/rust/pull/49196)

## New Contributors

* Daniel Kolsoi
* lukaslueg
* Lymia Aluysia
* Maxwell Borden
* Maxwell Powlison
* memoryleak47
* Mrowqa
* Sean Silva
* Tyler Mandry

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2300: `Self` in type definitions allowing `enum List<T> { Nil, Cons(T, Box<Self>) }`](https://github.com/rust-lang/rfcs/pull/2300).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Type alias enum variants](https://github.com/rust-lang/rfcs/pull/2338).
* [disposition: postpone] [Formalise reborrows](https://github.com/rust-lang/rfcs/pull/2364).
* [disposition: postpone] [Minimal target feature unsafe](https://github.com/rust-lang/rfcs/pull/2212).
* [disposition: postpone] [Implement parent items with child traits](https://github.com/rust-lang/rfcs/pull/2303).

## New RFCs

* [Portable packed SIMD vector types](https://github.com/rust-lang/rfcs/pull/2366).
* [Make cargo install extensible](https://github.com/rust-lang/rfcs/pull/2376).
* [Inherent trait implementation](https://github.com/rust-lang/rfcs/pull/2375).
* [Prior/outer doc comments](https://github.com/rust-lang/rfcs/pull/2374)
* [Selfexhausting iter adapter](https://github.com/rust-lang/rfcs/pull/2370).
* [Non-selfexhausting Drain](https://github.com/rust-lang/rfcs/pull/2369).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Mar 29. Helsinki, FI - March Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/248805420/).
* [Mar 31. Minsk, BY - Informal Rust meetup](https://users.rust-lang.org/t/informal-rust-meetup-in-minsk-2018-march-31st/16314).
* [Mar 31. Saint Petersburg, RU - March Rust Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9F%D0%B8%D1%82%D0%B5%D1%80%D0%B5/events/248834955/).
* [Apr  1. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbcb/).
* [Apr  3. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxgbfb/).
* [Apr  4. Cologne, DE - April 2018 Open Space](https://www.meetup.com/RustCologne/events/247804338/).
* [Apr  4. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxgbgb/).
* [Apr  4. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxgbgb/).
* [Apr  4. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/cpvshpyxgbgb/).
* [Apr  4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247388074/).
* [Apr  4. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr  5. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr  8. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgblb/).
* [Apr  9. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxgbmb/).
* [Apr 10. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Apr 11. Munich, DE - Fun with Rust and Numerical Methods](https://www.meetup.com/rust-munich/events/248055969/).
* [Apr 11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr 11. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Apr 11. Denver, US - April Meetup in Boulder](https://www.meetup.com/Rust-Boulder-Denver/events/248792627/).
* [Apr 12. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxgbqb/).
* [Apr 12. Arlington, US - Rust DC - Learn+Try: Rust in the Browser via WebAssembly](https://www.meetup.com/RustDC/events/248552247/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Systems Engineer at Distil Networks, Stockholm](https://www.distilnetworks.com/job/?id=3d69e0a4-3f6f-40b1-a610-7a8a4f4bbf24).
* [Rust Developmer at Asquera GmbH, Berlin](http://asquera.de/blog/2018-02-16/open-position/).
* [Sr. Software Developer at Nymi, Toronto](https://nymi.com/careers/sr-software).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> If Rust is martial arts teacher, Perl is a pub brawler. If you survive either, you’re likely to be good at defending yourself, though both can be painful at times.

— [Michal 'vorner' Vaner](https://vorner.github.io/2018/03/11/Should-you-learn-rust.html).

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/502)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
