Title: This Week in Rust 210
Number: 210
Date: 2017-11-28
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

* [Nightly Rust compiler and Cargo now run on Redox](https://www.redox-os.org/news/this-week-in-redox-32/).
* [Announcing Failure](https://boats.gitlab.io/blog/post/2017-11-16-announcing-failure/).
* [Speed up your Python using Rust](https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/).
* [Evolving Rust with Milksnake](https://blog.sentry.io/2017/11/14/evolving-our-rust-with-milksnake).
* [Crates.io ecosystem not ready for embedded Rust](https://www.tockos.org/blog/2017/crates-are-not-safe/).
* [This week in Rust docs 82](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-82).
* Notes from Rust+GNOME Hackfest in Berlin - by [Federico](https://people.gnome.org/~federico/blog/rust-gnome-hackfest-berlin.html), [Guillaume](https://blog.guillaume-gomez.fr/articles/2017-11-18+Rust%2BGNOME+Hackfest+in+Berlin), and [Antoyo](http://antoyo.ml/rust-gnome-hackfest-november).
* [podcast] [Rusty Spike Podcast - episode 8](https://rusty-spike.blubrry.net/2017/11/16/episode-8-nov-15-2017/). Firefox Quantum, Lin’s Quantum post, fearless concurrency, incremental typecheck, better wasm support, and Cargo on Redox.
* [podcast] [New Rustacean News: Rust 1.21 and 1.22](http://www.newrustacean.com/show_notes/news/rust_1_21_1_22/index.html). Quality of life improvements, Failure, wasm, and rustdoc fun – or, a bunch of highlights from the new releases *and* the community since 1.20.

# Crate of the Week

This week's crate is [Ammonia](https://crates.io/crates/ammonia), a crate for sanitizing HTML to prevent cross-site scripting (XSS), layout breaking and clickjacking.
Thanks to [Jules Kerssemakers](https://users.rust-lang.org/u/juleskers) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [Molten - "a style-preserving TOML parser" has some easy and accessible issues for beginners](https://github.com/LeopoldArkham/Molten/issues).
* [easy] [mdbook: Select default theme](https://github.com/rust-lang-nursery/mdBook/issues/95).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

110 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-11-13..2017-11-20

* [implement `impl Trait` in argument position](https://github.com/rust-lang/rust/pull/45918) (RFC [#1951](https://rust-lang.github.io/rfcs/1951-expand-impl-trait.html))
* [trait object debug](https://github.com/rust-lang/rust/pull/45897)
* [refactor type memory layouts and ABIs, to be more general and easier to optimize](https://github.com/rust-lang/rust/pull/45225)
* [enable TrapUnreachable in LLVM](https://github.com/rust-lang/rust/pull/45920)
* [incr.comp.: Implement query result cache and use it to cache type checking tables](https://github.com/rust-lang/rust/pull/46004)
* [incr.comp. Collect stats about duplicated edge reads from queries](https://github.com/rust-lang/rust/pull/46068)
* [set short-message feature unstable](https://github.com/rust-lang/rust/pull/46005)
* [std: Add a new wasm32-unknown-unknown target](https://github.com/rust-lang/rust/pull/45905)
* [always add an unreachable branch on matches to give more info to llvm](https://github.com/rust-lang/rust/pull/45821)
* [MIR: hardcode pass list internally and remove premature pluggability](https://github.com/rust-lang/rust/pull/45916)
* [handle closures correctly in MIR inlining](https://github.com/rust-lang/rust/pull/45913)
* [normalize inlined function in MIR inliner](https://github.com/rust-lang/rust/pull/45909)
* [fix MIR borrowck EndRegion not found](https://github.com/rust-lang/rust/pull/45922)
* [add `StorageDead` handling](https://github.com/rust-lang/rust/pull/45936)
* [ignore borrowck for static lvalues and allow assignment to static muts](https://github.com/rust-lang/rust/pull/46032)
* [integrate MIR type-checker with NLL inference](https://github.com/rust-lang/rust/pull/45825)
* [MIR-borrowck: don't ICE for cannot move from array error](https://github.com/rust-lang/rust/pull/45967)
* [rustc_trans: atomically write .rmeta outputs to avoid races](https://github.com/rust-lang/rust/pull/45899)
* [simplify higher-ranked LUB/GLB](https://github.com/rust-lang/rust/pull/45853)
* [short-circuiting internal iteration with Iterator::try_fold & try_rfold](https://github.com/rust-lang/rust/pull/45595)
* [rustc_driver: expose a way to override query providers in CompileController](https://github.com/rust-lang/rust/pull/45944)
* [cargo: Add support for publish to optionally take the index that can be used](https://github.com/rust-lang/cargo/pull/4568)
* [start shipping the Cargo book](https://github.com/rust-lang/rust/pull/45692)
* [support `extern type` in rustdoc](https://github.com/rust-lang/rust/pull/46000)
* [make rustdoc not include self-by-value methods from Deref target](https://github.com/rust-lang/rust/pull/45645)
* [rustdoc: Fix primitive types not showing up](https://github.com/rust-lang/rust/pull/46066)

## New Contributors

* Alexey Orlenko
* Benjamin Hoffmeyer
* Chris Vittal
* Collin Anderson
* Dan Gohman
* Jeff Crocker
* Laurentiu Nicola
* loomaclin
* Martin Lindhe
* Michael Lamparski
* Ramana Venkata
* Ritiek Malhotra
* Robert T Baldwin

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

* [Guard Clause Flow Typing](https://github.com/rust-lang/rfcs/pull/2221).

# Upcoming Events

* [Nov 24. Monkey Tech Days Toulouse, France - Explore Languages (Go Vs Rust) - MKTD#5](https://www.meetup.com/Monkey-Tech-Days/events/237545492/).
* [Nov 25. Rust Bangalore - Rust Concurrency (part 2 of 2)](https://www.meetup.com/rustox/events/244782966/).
* [Nov 27. Triangle Rustaceans Durham, NC - Algebraic Data Types in Practice and Theory](https://www.meetup.com/triangle-rustaceans/events/kkjnpnywpbkc).
* [Nov 29. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 29. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov 29. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywpbmc).
* [Nov 30. Rust Munich: Rust Machine Learning with Juice](https://www.meetup.com/rust-munich/events/244580709/).
* [Nov 30. Rust Detroit - Introducing Tock OS 1.0](https://www.meetup.com/rust-detroit/events/244855856/).
* [Nov 30. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Dec  6. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Dec  6. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Dec  6. Rust Cologne: impl Glühwein](https://www.meetup.com/RustCologne/events/244487721/).
* [Dec  6. Rust Atlanta: Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmywqbjb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Contract opportunity @ Mozilla: Distributed compilation cache written in Rust](https://users.rust-lang.org/t/contract-opportunity-mozilla-distributed-compilation-cache-written-in-rust/13898).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust's abstraction layers feel both transparent and productive. It's like being on a glass-bottomed boat, you see the sharks, but they can't get you.
> It's like a teaching language that you can also use in production. Rust helped me understand C.
> Also Rust people are amazing.

— [@gibfahn on Twitter](https://twitter.com/gibfahn/status/931187143686393863).

Thanks to [@sebasmagri](https://twitter.com/sebasmagri/status/931246295439650816) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
