Title: This Week in Rust 149
Number: 149
Date: 2016-09-27
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

* [Returning from Exceptions](http://os.phil-opp.com/returning-from-exceptions.html). How to return from exceptions correctly. Part of the series [Writing an OS in Rust](http://os.phil-opp.com/).
* [ripgrep is faster than {grep, ag, git grep, ucg, pt, sift}](http://blog.burntsushi.net/ripgrep/). ripgrep is a command line search tool that combines the usability of The Silver Searcher (an ack clone) with the raw speed of GNU grep.
* [Intersection Impls](http://smallcultfollowing.com/babysteps/blog/2016/09/24/intersection-impls/). A specialization example of adding a blanket impl that implements `Clone` for any `Copy` type, its shortcomings, and one proposed fix using intersection impls (also called lattice impls).
* [My adventures in Rust web development](https://medium.com/@tomaka/my-adventures-in-rust-webdev-850c67be6c40).
* [From tweet to Rust feature](https://llogiq.github.io/2016/09/14/feature.html). Journey of an idea from being a tweet to becoming a Rust feature.
* [How to count newlines really fast in Rust](https://llogiq.github.io/2016/09/24/newline.html).
* [Writing Cocoa apps in Rust](https://kylewlacy.github.io/posts/cocoa-apps-in-rust-eventually/).
* [Experiment: compare ZODB file-storage iteration with Python and Rust](http://jimfulton.info/site/2016/Sep/25/experiment-compare-zodb-file-storage-iteration-with-python-and-rust/).
* [Implementing Huffman coding in Rust](http://pramode.in/2016/09/26/huffman-coding-in-rust/).

### RustConf/RustFest Experiences

* [My Thoughts on RustConf 2016](http://jeenalee.com/2016/09/23/rust-conf.html) by Jeena Lee.
* [RustConf and Strange Loop 2016](http://hellomalisa.me/2016-09-20/rustconf-and-strange-loop.html) by Malisa.
* [Habitat at RustConf](https://blog.chef.io/2016/09/23/habitat-at-rustconf/) by Salim Alam.
* [RustFest was great!](https://blog.cyplo.net/posts/2016/09/rustfest-organization-was-the-best.html) by Cyryl Płotnicki.

## New Crates & Project Updates

* Rust Community Team [announces the Rust programming language video channel](https://users.rust-lang.org/t/announcing-the-rust-programming-language-video-channel-and-the-rustvideos-twitter-account/7370) on [YouTube](https://www.youtube.com/channel/UCaYhcUwRBNscFNUKTjgPFiA/playlists8) and [Twitter](https://twitter.com/rustvideos).
* Servo [maintains a fork of rust-bindgen](https://github.com/servo/rust-bindgen) which just got updated with a [major rewrite](https://github.com/servo/rust-bindgen/pull/37) that cleans up the codebase and paves the way for future improvements.
* [slog version 1.0 released](https://github.com/dpc/slog-rs). slog is a structured, composable logging library for Rust.
* [Cyano](https://github.com/ticki/cyano). An advanced Rust-to-JavaScript transpiler.
* [gimli](https://github.com/gimli-rs/gimli). A lazy, zero-copy parser for the DWARF debugging format.
* [Sarkara](https://github.com/quininer/sarkara). A Post-Quantum cryptography library written in Rust.
* [Native Windows GUI](https://github.com/gabdube/native-windows-gui). Native Window GUI (nwg for short) is a GUI library for Windows.
* [dataplotlib](https://github.com/coder543/dataplotlib). Plotting library that tries to make it easy to do scientific plots in Rust.
* [itertools version 0.5 released](http://bluss.github.io/rust/2016/09/26/itertools-0.5.0/). itertools provides extra functionality for iterators.
* [rouille](https://github.com/tomaka/rouille). Rust web server middleware.
* [Perlin](https://github.com/JDemler/perlin). A lazy, zero-allocation and data-agnostic Information Retrieval library.
* [This week in Rust Docs 23](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-23). Updates from the Rust documentation team.
* [This week in Servo 78](https://blog.servo.org/2016/09/19/twis-78/). Servo is a prototype web browser engine written in Rust.
* [This week in Tock OS 5](http://www.tockos.org/blog/2016/talking-tock-5/). Tock is a safe, multitasking operating system for low-power, low-memory microcontrollers.
* [These weeks in intermezzOS 3](https://intermezzos.github.io/blog/articles/twii3/). intermezzOS is a teaching operating system focused on introducing systems programming concepts to experienced developers.
* [This week in TiKV 2016-09-16](http://www.pingcap.com/tikv/2016/09/26/tikv-weekly/). TiKV is a distributed Key-Value database which is based on the design of Google Spanner and HBase.
* [This week in Ruma 2016-09-18](https://www.ruma.io/news/this-week-in-ruma-2016-09-18/). Ruma is a Matrix homeserver written in Rust.
* [This week in Ruma 2016-09-25](https://www.ruma.io/news/this-week-in-ruma-2016-09-25/).
* [What's coming up in imag 16](http://beyermatthias.de/blog/2016/09/23/what-s-coming-up-in-imag-16/). imag is a text based personal information management suite.

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
* [RBML is gone](https://github.com/rust-lang/rust/pull/36551) (epic PR)
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

# Quote of the Week

> You can actually return Iterators without summoning one of the Great Old Ones now, which is pretty cool.

— [/u/K900_ on reddit](https://www.reddit.com/r/rust/comments/53uzzh/most_interesting_uses_of_impl_trait/d7wgp42).

Thanks to [Johan Sigfrids](https://users.rust-lang.org/users/johansigfrids) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
