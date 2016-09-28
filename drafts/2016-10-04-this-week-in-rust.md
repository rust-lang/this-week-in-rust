Title: This Week in Rust 150
Number: 150
Date: 2016-10-04
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



## New Crates & Project Updates



# Crate of the Week

Somewhat unsurprisingly, this week's crate of the week is [ripgrep](https://crates.io/crates/ripgrep). In case you've missed it, this is a grep/ag/pt/whatever search tool you use replacement that absolutely smokes the competition in most performance tests. Thanks to [DanielKeep](https://users.rust-lang.org/users/DanielKeep) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust Design Patterns](https://github.com/rust-unofficial/patterns) is looking for collaborators. Check out the [README](https://github.com/rust-unofficial/patterns#readme) for more information.
* [easy] [rust: incr.comp.: Issue warning if cache directory is on FS without hard-linking](https://github.com/rust-lang/rust/issues/36680).
* [tedious] [rust: Initial webassembly support via LLVM](https://github.com/rust-lang/rust/issues/36317).
* [easy] [rust: Bootstrap key logic is too strict](https://github.com/rust-lang/rust/issues/36548).
* [easy] [rust: rustc should emit an error when there's a bootstrap key mismatch](https://github.com/rust-lang/rust/issues/36544).
* [easy] [rust: Lint against using generic conversion traits when concrete methods are available](https://github.com/rust-lang/rust/issues/36443).
* [moderate] [rust: Create official .deb packages](https://github.com/rust-lang/rust/issues/28307).
* [easy] [rust-www: Better front-page example](https://github.com/rust-lang/rust-www/issues/180).
  The front page example on the website isn't so special. Make it shine.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

77 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-09-19..2016-09-26

* [`Self` can no longer be a type parameter](https://github.com/rust-lang/rust/pull/36649)
* [MIR: Trivial copy propagation](https://github.com/rust-lang/rust/pull/36388)
* [MIR: Constant propagation](https://github.com/rust-lang/rust/pull/36639)
* [Attribute invocation at crate root level allowed again](https://github.com/rust-lang/rust/pull/36618) (were inadvertently disallowed two weeks ago)
* [`TypedArena` now allocates lazily](https://github.com/rust-lang/rust/pull/36618), [loses `.with_capacity(_)`](https://github.com/rust-lang/rust/pull/36657) (the latter is a breaking change)
* [`syntax::codemap::Span`s can now be merged if adjacent](https://github.com/rust-lang/rust/pull/36585)
* [RBML is gone](https://github.com/rust-lang/rust/pull/36585) (epic PR)
* [`#[inline]`d functions are now only instantiated on use site](https://github.com/rust-lang/rust/pull/36524) (epic speedup)
* [Better `parent` info for `save-analysis](https://github.com/rust-lang/rust/pull/36487)
* [`trans::adt` is superceded by `rustc::ty::layout`](https://github.com/rust-lang/rust/pull/36151)
* [Rustc metadata diagnostics](https://github.com/rust-lang/rust/pull/36102)
* [`assert_ne!(..)` and `debug_assert_ne!(..)`](https://github.com/rust-lang/rust/pull/35074)
* [`2u64.pow(99)` now panics instead of silently overflowing](https://github.com/rust-lang/rust/pull/34942)
* [`String` no longer `impl`s `From<Vec<char>>` nor `From<&'a [char]>`](https://github.com/rust-lang/rust/pull/36685) (for now, until the regressions are sorted out)
* [ARM LLVM bug workaround: Setting discriminant via `memset`](https://github.com/rust-lang/rust/pull/36496)
* [Preparations for macros 2.0](https://github.com/rust-lang/rust/pull/36154)

## New Contributors

* aclarry
* Alexander von Gluck IV
* Andrew Lygin
* Ashley Williams
* Austin Hicks
* Eitan Adler
* Gianni Ciccarelli
* jacobpadkins
* James Duley
* Joe Neeman
* Niels Sascha Reedijk
* Vanja Cosic

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

* [Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).

## New RFCs

* [Custom attributes](https://github.com/rust-lang/rfcs/pull/1755).
* [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).

# Upcoming Events

* [9/28. Boston Rust Meetup](https://www.meetup.com/BostonRust/events/234241654/).
* 9/28. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 9/28. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org.
* [9/29. Rust Meetup Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/7).
* [9/29. Rust DC: Who is more foolish? Novice traps in Rust](https://www.meetup.com/RustDC/events/232445143/).
* [10/5. Open-Space Rust Meetup Cologne/Bonn](http://rustaceans.cologne/2016/10/05/open-space.html).
* 10/5. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* 10/5. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org.
* [10/6. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [10/10. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/233577254/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Friends of the Forest

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. These people are
'friends of the forest'.

This week's friends of the forest were nominated at [RustConf 2016]:

* [dtolnay] and [oli-obk] for taking over most of the maintenance of the [serde] stack
* [seanmonstar] for answering my too many questions
* [phildawes] for racer
* [mitsuhiko] for [redis]
* [serde] team ([dtolnay], [oli-obk], [erickt])
* [athemathmo] for [rusty_machine]
* [carllerche] for [mio]/[tokio]
* [killercup] for keeping [diesel] running
* [dikaiosune] - [rusty-dash]
* [nasa42] and [llogiq] - [This Week In Rust]
* [WindowsBunny] - being the fuzziest bunny +1 +1 (ed: the +1's are from multiple people)
* [eddyb] - for knowing everything about rust
* [chriskrycho] for [New Rustacean]
* [steveklabnik], Rust documentation superhero
* [carllerche], [eternaleye], [staticassert]
* [Matthias Beyer]
* [llogiq] and [manishearth]
* [illegalprime] for his work on [rust-websocket]
* [Mark-Simulacrum] for awesome work on the [compiler performance website]
* [sfackler] and [briansmith] for enhancing the crypto/security story for Rust.
  Their efforts have made running Rust in production code much more feasible.
  sfackler: [rust-openssl], [rust-security-framework], [schannel-rs],
  [rust-native-tls], briansmith: [ring], [webpki]

[RustConf]: http://rustconf.com/
[dtolnay]: https://github.com/dtolnay
[oli-obk]: https://github.com/oli-obk
[seanmonstar]: https://github.com/seanmonstar
[phildawes]: https://github.com/phildawes
[mitsuhiko]: https://github.com/mitsuhiko
[redis]: https://github.com/mitsuhiko/redis-rs
[erickt]: https://github.com/erickt
[serde]: https://github.com/serde-rs
[athemathmo]: https://github.com/AtheMathmo
[rusty_machine]: https://github.com/AtheMathmo/rusty-machine
[carllerche]: https://github.com/carllerche
[mio]: https://github.com/carllerche/mio
[tokio]: https://github.com/tokio-rs
[killercup]: https://github.com/killercup
[diesel]: http://diesel.rs/
[dikaiosune]: https://github.com/dikaiosune
[rusty-dash]: http://rusty-dash.com/
[nasa42]: https://github.com/nasa42
[llogiq]: https://github.com/llogiq
[This Week In Rust]: https://this-week-in-rust.org/
[WindowsBunny]: https://github.com/retep998
[eddyb]: https://github.com/eddyb
[chriskrycho]: https://github.com/chriskrycho
[New Rustacean]: http://www.newrustacean.com/
[steveklabnik]: https://github.com/steveklabnik
[eternaleye]: https://github.com/eternaleye
[staticassert]: https://github.com/insaneinside
[Matthias Beyer]: http://beyermatthias.de
[llogiq]: https://github.com/llogiq
[manishearth]: https://github.com/manishearth
[illegalprime]: https://github.com/illegalprime
[rust-websocket]: https://github.com/cyderize/rust-websocket
[Mark-Simulacrum]: https://github.com/Mark-Simulacrum
[compiler performance website]: http://perf.rust-lang.org/
[sfackler]: https://github.com/sfackler
[briansmith]: https://github.com/briansmith
[rust-openssl]: https://github.com/sfackler/rust-openssl
[rust-security-framework]: https://github.com/sfackler/rust-security-framework
[schannel-rs]: https://github.com/steffengy/schannel-rs
[rust-native-tls]: https://github.com/sfackler/rust-native-tls
[ring]: https://github.com/briansmith/ring
[webpki]: https://github.com/briansmith/webpki

[Submit your Friends-of-the-Forest nominations for next week][foft]!

[foft]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295

# Quote of the Week

> You can actually return Iterators without summoning one of the Great Old Ones now, which is pretty cool.

— [/u/K900_ on reddit](https://www.reddit.com/r/rust/comments/53uzzh/most_interesting_uses_of_impl_trait/d7wgp42).

Thanks to [Johan Sigfrids](https://users.rust-lang.org/users/johansigfrids) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
