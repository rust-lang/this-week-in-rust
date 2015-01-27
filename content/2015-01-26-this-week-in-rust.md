Title: This Week in Rust 67
Date: 2015-01-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

This Week in Rust is openly developed [on Github](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week the [mailing list was closed][ml], `std::io` was [renamed
`std::old_io`][oldio]. There were many pull requests merged, but not a
lot of churn.

[ml]: https://mail.mozilla.org/pipermail/rust-dev/2015-January/011558.html
[oldio]: http://discuss.rust-lang.org/t/psa-io-old-io/1403

# What's cooking on master?

143 pull requests were [merged in the last week][merged], and 6 [RFCs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-19..2015-01-25
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-01-19..2015-01-25

Flavio, Steve and Alex all made rollups. Thanks!

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* The `Show` and `String` formatting traits [have been renamed][fmt]
  to `Debug` and `Display` to more clearly reflect their related
  purposes. Automatically getting a string conversion to use with
  `format!{":?}")` is now written `#[derive(Debug)]`.
* Both the `#[start]` and `#[main]` attributes are [feature
  gated][gatemain] as a precaution. Use `#![feature(start)]` and
  `#![feature(main)]` to get them back.

[fmt]: https://github.com/rust-lang/rust/pull/21457
[gatemain]: https://github.com/rust-lang/rust/pull/21257

## Other Changes

* Abstract [OS-specific string types][osstr], `std::ff::{OsString,
  OsStr}`, provide strings in platform-specific encodings for easier
  interop with system APIs. [RFC][osstr-rfc].
* `extern crate` and `use` [no longer have to be written only at the
  top of a module][viewitems], but can be intermixed with other item
  definitions.
* Brian Leibig [added his LALR grammar][lalr], which parses almost all
  the Rust files that rustc can.
* The (oft-neglected) grammar from the manual was [extracted to its
  own file][grammar].
* The [`unconditional_recursion`][recur] lint detects basic
  infinite recursion scenarios that are probably not intended.

[osstr]: https://github.com/rust-lang/rust/pull/21488
[osstr-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md
[viewitems]: https://github.com/rust-lang/rust/pull/20179
[grammar]: https://github.com/rust-lang/rust/pull/19353
[recur]: https://github.com/rust-lang/rust/pull/20373
[lalr]: https://github.com/rust-lang/rust/pull/21452

## New Contributors

* Adam Roben
* Alexis
* Barosl LEE
* blackbeam
* Chris Thorn
* Daniel Griffen
* Daniel Raloff
* Eunji Jeong
* Flavio Percoco Premoli
* GuillaumeGomez
* Ignacio Corderi
* Jay True
* JP Sugarbroad
* KernelJ
* Kim Røen
* Logan Chien
* Luke Francl
* Michael Pankov
* Ryan Levick
* Sean Patrick Santos
* Steven Allen
* Tim Parenti
* Toby Scrace
* Tristan Storch
* visualfc
* Wangshan Lu
* Willson Mock

# Approved RFC's

* [242: Deref conversions][rfc-242]. [PR][rfc-242-pr]. Adds coercions
  from `&T` to `&U` when `T: Deref<U>`.
* [550: Macro future proofing][rfc-550]. Places limits on the grammar
  of macro matchers to avoid potential problems with adding more
  syntax to Rust. The implementation has already
  landed. [PR][rfc-550-pr].
* [565: `fmt::Show` and `fmt::String` guidelines][rfc-565]. Renames
  `Show` to `Debug`, `String` to `Display` and establishes conventions
  for when to use and implement each. [PR][rfc-565-pr].
* [Amendment to 517 for string handling][rfc-517]. [rfc-517-pr]. Defines
  the `OsString` and `OsStr` platform-specific string types.
* [587: Make return type of `Fn` traits an associated type][rfc-587]. Allows
  calls to be overloaded based on return type.

[rfc-242]: https://github.com/rust-lang/rfcs/blob/master/text/0241-deref-conversions.md
[rfc-242-pr]: https://github.com/rust-lang/rfcs/pull/241
[rfc-550]: https://github.com/rust-lang/rfcs/blob/master/text/0550-macro-future-proofing.md
[rfc-550-pr]: https://github.com/rust-lang/rfcs/pull/550
[rfc-565]: https://github.com/rust-lang/rfcs/blob/master/text/0565-show-string-guidelines.md
[rfc-565-pr]: https://github.com/rust-lang/rfcs/pull/565
[rfc-517]: https://github.com/rust-lang/rfcs/blob/master/text/0517-io-os-reform.md#string-handling
[rfc-517-pr]: https://github.com/rust-lang/rfcs/pull/575
[rfc-587]: https://github.com/rust-lang/rfcs/blob/master/text/0587-fn-return-should-be-an-associated-type.md
[rfc-587-pr]: https://github.com/rust-lang/rfcs/pull/587

# New RFC's

* [Introduce a default object lifetime bound][rfc-599-pr].
* [Replace `be` with `become`][rfc-601-pr].
* [Specify unwinding][rfc-638-pr].
* [Implement a `discriminant_value` intrinsic][rfc-639-pr].
* [Debug improvements][rfc-640-pr].
* [Disallow omitting the ABI in `extern` declarations][rfc-697-pr].
* [Syntax for `FullRange`][rfc-702-pr].
* [Add a new macro for 'unreachable' whose meaning differs in release
  builds][rfc-706-pr].
* [Truly unsized types][rfc-709-pr].
* [Reseeding `std::rand`][rfc-722-pr].
* [Unsafe enums][rfc-724-pr].
* [Amend macro future proofing RFC][rfc-733-pr].
* [Revert RFC to require impls to be near structs][rfc-735-pr].
* [Privacy-respecting FRU][rfc-736-pr].
* [Support variance for type parameters][rfc-738-pr].
* [Amend RFC 517 with material on `std::fs`][rfc-739-pr].
* [Integer guidelines RFC][rfc-741-pr].
* [Replace `ref` by `*` in patterns][rfc-742-pr].

[rfc-599-pr]: https://github.com/rust-lang/rfcs/pull/599
[rfc-601-pr]: https://github.com/rust-lang/rfcs/pull/601
[rfc-638-pr]: https://github.com/rust-lang/rfcs/pull/638
[rfc-639-pr]: https://github.com/rust-lang/rfcs/pull/639
[rfc-640-pr]: https://github.com/rust-lang/rfcs/pull/640
[rfc-697-pr]: https://github.com/rust-lang/rfcs/pull/697
[rfc-702-pr]: https://github.com/rust-lang/rfcs/pull/702
[rfc-706-pr]: https://github.com/rust-lang/rfcs/pull/706
[rfc-709-pr]: https://github.com/rust-lang/rfcs/pull/709
[rfc-722-pr]: https://github.com/rust-lang/rfcs/pull/722
[rfc-724-pr]: https://github.com/rust-lang/rfcs/pull/724
[rfc-733-pr]: https://github.com/rust-lang/rfcs/pull/733
[rfc-735-pr]: https://github.com/rust-lang/rfcs/pull/735
[rfc-736-pr]: https://github.com/rust-lang/rfcs/pull/736
[rfc-738-pr]: https://github.com/rust-lang/rfcs/pull/738
[rfc-739-pr]: https://github.com/rust-lang/rfcs/pull/739
[rfc-741-pr]: https://github.com/rust-lang/rfcs/pull/741
[rfc-742-pr]: https://github.com/rust-lang/rfcs/pull/742

# Community

What's [Built with Rust]?

[Built with Rust]: http://builtwithrust.com/

## Announcements

* [Weekly-meetings/2015-01-20][mtg]: goodbye view items; deref
  coercion; nounwind; discourse/ml changes; discriminant intrinsics;
  issue triage; fott. [/r/rust][mtg-r-rust].
* [PSA: io => old_io][oldio]. As part of the overhaul of `std::io` the
  existing `io` module is being renamed to move it out of the way.
* [Moving editor highlighting into their own repos][edit]. Your vim
  and emacs modes will need to be obtained elsewhere before long.
* [Issue categorization changes][iss]. Steve has been retriaging old
  issues.
* [Call for more comments on IO RFCs][iorfcs]. Now is your chance.

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-01-20.md
[mtg-r-rust]: https://www.reddit.com/r/rust/comments/2t405p/weekly_meeting_20150120_goodbye_view_items_deref/
[edit]: http://discuss.rust-lang.org/t/moving-editor-highlighting-into-their-own-repos/1395
[oldio]: http://discuss.rust-lang.org/t/psa-io-old-io/1403
[iss]: http://discuss.rust-lang.org/t/issue-categorization-changes/1417
[iorfcs]: http://discuss.rust-lang.org/t/call-for-more-comments-io-rfcs/1449

## Blog Posts

* [Functional reactive event handling][frp]. Explorations into
  functional reactive programming in Rust.
* [Mozilla's Servo Still On Track ofr 2015 Alpha
  Release][servo]. Phoronix picks up Jack's linux.conf.au
  talk. [/r/rust][servo-r-rust].
* [Writing Cross-Platform Games Using Rust and Piston][games]. It can
  be done.
* [Raft so far][raft-rs]. Hoverbear talks about his endeavors to
  implement the [Raft consensus algorithm][raft].
* [Graydon comments on Rust 1.0 alpha][graydon].

[frp]: http://blog.ebopp.de/blog/2015/01/26/frp-in-rust/
[servo]: http://www.phoronix.com/scan.php?page=news_item&px=Mozilla-Servo-Engine-LCA2015
[servo-r-rust]: https://www.reddit.com/r/rust/comments/2t0po8/mozillas_servo_still_on_track_for_2015_alpha/
[graydon]: http://graydon2.dreamwidth.org/195706.html
[games]: https://github.com/tedsta/getting-started-with-piston
[raft-rs]: http://www.hoverbear.org/2015/01/25/raft-so-far/
[raft]: https://raftconsensus.github.io/

## Discussions

* [Say goodbye to the mailing list][ml]. Reddit responds to the end of
  rust-dev.
* [A macro that is to `Result::or_else` what `try!` is to
  `Result::and_then`][try]. SimonSapin is trying.
* [Replace most of the collections API with ranges and
  iterators][ranges]. Gankro has a crazy far future idea.
* [Jai Demo: Data-oriented features: SOA, crazy
  'using'][jai]. Discussion on /r/rust about Jonathan Blow's latest.
* [How Rust applications and libraries fit into Debian][deb]. Making
  crates easy to repackage by Linux distributions is not yet solved.

[ranges]: http://discuss.rust-lang.org/t/crazy-replace-most-of-the-collections-api-with-ranges-and-iterators/1375
[jai]: https://www.reddit.com/r/rust/comments/2t6xqz/jai_demo_dataoriented_features_soa_crazy_using/
[ml]: https://www.reddit.com/r/rust/comments/2tdqgc/rustdev_say_goodbye_to_the_mailing_list/
[try]: http://discuss.rust-lang.org/t/a-macro-that-is-to-result-or-else-what-try-is-to-result-and-then/1416
[deb]: https://www.reddit.com/r/rust/comments/2tnql2/how_rust_applications_and_libraries_fit_into/

## Videos

* [Jim Blandy - Programming in Rust][blandy]. Mozilla's Jim Blandy
  talking about Rust for O'Reilly. Registration required.

[blandy]: http://post.oreilly.com/rd/9z1z7bquivj2k5agpg7tuouo569mv0bkk8e4jrajhv0

## New Projects

* [Built with Rust] - A website promoting projects built with Rust.
* [cql-ffi-safe] - Safe bindings to the DataStax C++ driver for Cassandra.
* [screenshot-rs] - Capturing the screen.
* [rust-atomicwrites] - Atomic file writing for POSIX systems.
* [winapi-rs] - Windows API bindings.
* [rust-beanstalkd] - A beanstalkd client that is compatible with
  IronMQ. It's API has been recently been redesigned.
* [mm] - A multimedia library that is like a cross between SDL and GDK.
* [seqloq] - Fast locks for read-heavy workloads.
* [r6.rs] - R6RS Scheme implementation in Rust.
* [oxischeme] - A Scheme implementation in Rust, with a working GC.
* [CoatCheck] - A library for storing values and referencing them by
  ticket.
* [emojicons] - An emoji parser.
* [maud] - A macro that takes markup and Rust tokens and emits HTML.
* [ocb.rs] - OCB-AES 'authenticryption'.
* [parser-combinators] - A parser-combinator library.

[Built with Rust]: http://builtwithrust.com/
[cql-ffi-safe]: https://github.com/tupshin/cql-ffi-safe
[screenshot-rs]: https://github.com/alexchandel/screenshot-rs
[rust-atomicwrites]: https://github.com/untitaker/rust-atomicwrites
[winapi-rs]: https://github.com/retep998/winapi-rs
[rust-beanstalkd]: https://github.com/schickling/rust-beanstalkd
[mm]: https://github.com/Daggerbot/mm
[seqloq]: https://github.com/kmcallister/seqloq
[r6.rs]: https://github.com/kimhyunkang/r6.rs
[oxischeme]: https://fitzgen.github.io/oxischeme/oxischeme/index.html
[CoatCheck]: https://www.reddit.com/r/rust/comments/2th6qf/library_coatcheck/
[emojicons]: https://github.com/sindriava/rust-emojicons
[ocb.rs]: https://github.com/kmcallister/ocb.rs
[parser-combinators]: https://github.com/Marwes/parser-combinators
[maud]: https://lambda.xyz/maud/doc/maud/

## Project Updates

* [This Week in Servo 20][twis].
* [dynamodule], kmc's experiment in dynamic OOP supports virtual
  destructors.
* [multirust] can install custom toolchains.

[twis]: http://blog.servo.org/2015/01/20/twis-20/
[dynamodule]: https://github.com/kmcallister/dynamodule/blob/561191a6d735901cb351bf6e6aa29c79f08ca43a/README.md#virtual-destructors
[multirust]: https://www.reddit.com/r/rust/comments/2tnmu2/multirust_can_also_manage_custom_local_toolchains/

## Upcoming Events

* [January 27 - Rust NY][ny].
* [January 28 - Amsterdam][amst].
* [February 1 - The Story of Rust][steve]. Steve Klabnik at FOSDEM.
* [February 1 - Servo and You][jdm]. Josh Matthews at FOSDEM.

[ny]: http://www.meetup.com/RustNY/events/219961968/
[amst]: http://www.meetup.com/Rust-Amsterdam/events/218908906/
[steve]: https://fosdem.org/2015/schedule/event/the_story_of_rust/
[jdm]: https://fosdem.org/2015/schedule/event/servo_the_parallel_web_browser_and_you!/
