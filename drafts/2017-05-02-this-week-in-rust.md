Title: This Week in Rust 180
Number: 180
Date: 2017-05-02
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

This week's crate of the week is [pq](https://crates.io/crates/pq), a crate to generically decode protobuf messages. Thanks to [sevagh](https://users.rust-lang.org/users/sevagh) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [What topics would you like to see covered in a video course about Rust](https://users.rust-lang.org/t/what-topics-would-you-like-to-see-covered-in-a-video-course-about-rust/10500)?
* [rust: Debian Rust packages](https://github.com/rust-lang/rust/issues/28307#issuecomment-295283017).
* [rdedup](https://github.com/dpc/rdedup) - a data deduplication with compression and public key encryption library, is [looking for contributors](https://users.rust-lang.org/t/twir-call-for-participation/4821/42) who are interested in crypto, command line, and backups.
* [PumpkinDB](https://github.com/PumpkinDB/PumpkinDB) has a list of [starter issues](https://github.com/PumpkinDB/PumpkinDB/issues?q=is%3Aissue+is%3Aopen+label%3AWhatCanIStartWith%3F) for [people interested in an event sourcing database engine](https://users.rust-lang.org/t/twir-call-for-participation/4821/43).
* [easy] [maud: Remove `error!` macro](https://github.com/lfairy/maud/issues/84). Maud is an HTML template engine for Rust.
* [easy] [maud: Document toggled classes](https://github.com/lfairy/maud/issues/85).
* [less easy] [rust-bindgen: Add in-worklist bits to the analysis runner](https://github.com/servo/rust-bindgen/issues/664).
* [easy] [flate2: Use distinct Flush types for `Compress::compress` vs `Decompress::decompress`](https://github.com/alexcrichton/flate2-rs/issues/79). flate2 implements FLATE, Gzip, and Zlib bindings for Rust.
* [easy] [flate2: Write usage examples](https://github.com/alexcrichton/flate2-rs/issues/76).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

98 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-04-17..2017-04-24

* [repr struct alignment](https://github.com/rust-lang/rust/pull/39999) (RFC [#1358](https://github.com/rust-lang/rfcs/blob/master/text/1358-repr-align.md)]
* [syntax: support parenthesis around trait bounds](https://github.com/rust-lang/rust/pull/41077)
* [:vis matcher for macro_rules](https://github.com/rust-lang/rust/pull/41012)
* [`traits::select(..)` filters out predicates from other traits](https://github.com/rust-lang/rust/pull/41486)
* [cache DTOR check constraints on abstract data types](https://github.com/rust-lang/rust/pull/41485)
* [performance audit Spring 2017](https://github.com/rust-lang/rust/pull/41469)
* [remove unstable deprecated items](https://github.com/rust-lang/rust/pull/41437)
* [don't panic if attribute macros don't resolve at crate root](https://github.com/rust-lang/rust/pull/41432)
* [hoedown makes a comeback!](https://github.com/rust-lang/rust/pull/41290)
* [re-enable hoedown by default](https://github.com/rust-lang/rust/pull/41431)
* [specialize `Vec::extend(IntoIter)`](https://github.com/rust-lang/rust/pull/41191)
* [specialize {`Path`, `OsStr`}`.clone_into()`](https://github.com/rust-lang/rust/pull/41390)
* [add functions to transmute floats to ints](https://github.com/rust-lang/rust/pull/39271)
* [don't clog register allocator with byvals](https://github.com/rust-lang/rust/pull/41378)
* [back out backtrace pruning logic](https://github.com/rust-lang/rust/pull/41364) (it was too eager)
* [convert calls to `visit_all_item_likes_in_crate(..)`](https://github.com/rust-lang/rust/pull/41360)
* [fix debug infinite loop](https://github.com/rust-lang/rust/pull/41342)
* [on-demand-ify `associated_item_def_ids`](https://github.com/rust-lang/rust/pull/41340)
* [on-demand-ify `monomorphic_const_eval`](https://github.com/rust-lang/rust/pull/41310)
* [polymorphic `const_eval(..)`](https://github.com/rust-lang/rust/pull/41408)
* [cargo: add `overflow_checks` to profiles](https://github.com/rust-lang/cargo/pull/3908)
* [cargo: CLI support for `--all-`{`bins`, `tests`, `examples`, `benches`}](https://github.com/rust-lang/cargo/pull/3901)
* [cargo: support `$RUSTC_WRAPPER`](https://github.com/rust-lang/cargo/pull/3887)

## New Contributors

* Dylan Maccora
* Maxwell Paul Brickner
* Nicolas Bigaouette

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1733: Trait alias](https://github.com/rust-lang/rfcs/pull/1733).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: close] [Check future-proofing of `macro_rules!` using FIRST sets](https://github.com/rust-lang/rfcs/pull/1746).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Proposal for default crate recommendation ranking](https://github.com/rust-lang/rfcs/pull/1824).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Improve the `assert_eq` failure message formatting to increase legibility](https://github.com/rust-lang/rfcs/pull/1866).
* [disposition: merge] [A portability lint](https://github.com/rust-lang/rfcs/pull/1868).
* [disposition: postpone] [Add `SafeDeref` and `SafeDerefMut`, equivalent to `Deref` and `DerefMut` but which are guaranteed to always return the same object](https://github.com/rust-lang/rfcs/pull/1873).
* [disposition: postpone] [Unsafe lifetime](https://github.com/rust-lang/rfcs/pull/1918). Add a new special lifetime, `'unsafe`, that implicitly satisfies any constraint, but may only be instantiated within an unsafe context.

## New RFCs

* [Make RangeInclusive just a two-field struct (amend 1192)](https://github.com/rust-lang/rfcs/pull/1980).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

PRs:

* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/pull/81)

Issues in final comment period:

* [Ordering of types of groups within a module](https://github.com/rust-lang-nursery/fmt-rfcs/issues/71)
* [Struct and tuple literals](https://github.com/rust-lang-nursery/fmt-rfcs/issues/64)
* [Array literals](https://github.com/rust-lang-nursery/fmt-rfcs/issues/63)
* [Where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)
* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)

Other interesting issues:

* [Function calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/64) and [chains of calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/66)
* [Combining opening and closing delims](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)

# Upcoming Events

* [Apr 27. Rust Mexico City Meetup #5](https://www.meetup.com/es-ES/Rust-MX/events/239279107/).
* [Apr 27. Rust Stockholm - Rust meetup @ Distil Networks](https://www.meetup.com/ruststhlm/events/238207716/).
* [Apr 27. Rust Meetup Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/28).
* **[Apr 30. RustFest 2017 - Kyiv, Ukraine](http://2017.rustfest.eu/).**
* [Apr 30. Kickstart to Rust Mozilla Gujarat User Group](https://reps.mozilla.org/e/kickstart-to-rust-mozilla-gujarat-user-group/).
* [May  2. Rust Workshop - RustBridge - Dortmund, Germany](https://www.meetup.com/Softwerkskammer-Ruhrgebiet/events/239380377/).
* [May  3. Intro to Rust for Java programmers - Code@LTH](https://www.facebook.com/events/1395576530485976/).
* [May  3. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238448447/).
* [May  3. Boston Rust: Rust 1.0 Anniversary Party and Hack Night](https://www.meetup.com/BostonRust/events/239319480/).
* [May  3. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May  3. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May  4. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [May  4. Rust Bay Area: Using Rust at Dropbox to make Magic Pocket](https://www.meetup.com/Rust-Bay-Area/events/239222217/).
* [May  8. Prague Rust Meetup #3](https://www.meetup.com/rust-prague/events/239129625/).
* [May  8. Seattle Rust - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/).
* [May 10. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 10. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 11. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlywhbpb/).
* [May 11. Rust DC - Building high performance REST APIs with Rust and Rocket](https://www.meetup.com/RustDC/events/239115583/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [RustJobs.rs](https://rustjobs.rs) - a website dedicated to Rust jobs. There are 13 open Rust positions plus a list of companies using Rust.
* [Rust Software Engineer at resin.io](https://resin.workable.com/j/ACF748D4A2).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> There are many ways in which Rust is like a version of C/C++ that mutated when Haskell was injected into its veins.

— [Lokathor on reddit](https://www.reddit.com/r/rust/comments/667ocp/why_are_some_people_comparing_rust_to_haskell/dggbked/).

Thanks to [Johan Sigfrids](https://users.rust-lang.org/t/twir-quote-of-the-week/328/392) and [liquidivy](https://www.reddit.com/r/rust/comments/667ocp/why_are_some_people_comparing_rust_to_haskell/dgge8vj/) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
