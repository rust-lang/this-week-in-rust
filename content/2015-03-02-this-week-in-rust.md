Title: This Week in Rust 72
Date: 2015-03-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

135 pull requests were [merged in the last week][merged], and 1 [RFC PR][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-23..2015-03-02
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-02-23..2015-03-02

Now you can follow breaking changes *[as they happen][BitRust]*!

[BitRust]: http://bitrust.octarineparrot.com/

## Breaking Changes

* [Partial implementation for UFCS trait-less associated paths][ufcs].
* [Basic implementation of the string pattern API][pat].
* [Deprecate std::sync::TaskPool][taskpool].

[ufcs]: https://github.com/rust-lang/rust/pull/22172
[pat]: https://github.com/rust-lang/rust/pull/22466
[taskpool]: https://github.com/rust-lang/rust/pull/22783

## Other Changes

* [Add support for default trait implementations][def].
* [Allow methods that require `Self:Sized` to be object safe][safe].
* [Permit `T::Item` based on bounds that appear in where clauses][where].

[def]: https://github.com/rust-lang/rust/pull/21689
[safe]: https://github.com/rust-lang/rust/pull/22301
[where]: https://github.com/rust-lang/rust/pull/22512

## New Contributors

* defuz
* FuGangqiang
* JP-Ellis
* lummax
* Michał Krasnoborski
* nwin
* Raphael Nestler
* Ryan Prichard
* Scott Olson

## Approved RFCs

Mysteriously, during the week of February 23 to March 1 there were no
RFCs approved to The Rust Language.

## New RFCs

* [Disallow parenthesized types][paren].
* [Proposed EOF reinstatement][eof].
* [Reserve more numeric types][num].
* [Move `std::thread_local::*` into `std::thread`][thread].
* [Remove `as_mut_vec` from `String`][string].
* [Const functions and inherent methods][const].
* [Improve CString construction methods][cstring].
* [Entry API v3][entry].
* [Named and destructable self][self].

[paren]: https://github.com/rust-lang/rfcs/pull/901
[eof]: https://github.com/rust-lang/rfcs/pull/903
[num]: https://github.com/rust-lang/rfcs/pull/907
[thread]: https://github.com/rust-lang/rfcs/pull/909
[string]: https://github.com/rust-lang/rfcs/pull/910
[const]: https://github.com/rust-lang/rfcs/pull/911
[cstring]: https://github.com/rust-lang/rfcs/pull/912
[entry]: https://github.com/rust-lang/rfcs/pull/921
[self]: https://github.com/rust-lang/rfcs/pull/922

# Quote of the Week

*"I must kindly ask that you please not go around telling people to disregard the rules of our community. Violations of Rule #6 will absolutely not be tolerated."*

[kibwen is serious][serious] about upholding community standards.

[serious]: https://www.reddit.com/r/rust/comments/2xl9fa/meta_definitely_offtopic_what_does_the_bee_rule/cp169jw

# Notable Links

* [Introduction to Systems Programming with Rust][sys]. A video by Mozilla's Lars Bergstrom.
* [Experience Report: Developing the Servo Web Browser Engine using Rust][servo]. By the busy Lars
  Bergstrom, et. al.
* [A tutorial on creating a drop-in replacement for rustc][replace].
* [What do C/C++ systems programmers think of Rust?][what]
* [What's your killer Rust feature][killer]? Rust is super.
* [What significant language/runtime features were removed][removed]? A trip down memory lane.
* [Experiences migrating a go project to Rust][go].
* [Weekly-meetings/2015-02-24][mtg]. `should_fail`; irc; error codes; type ascription; triage.
* [A list of Rust IRC channels][irc]. There are a great many.
* [An alternative introduction to Rust][alt].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-02-24.md
[irc]: http://users.rust-lang.org/t/a-list-of-rust-irc-channels/472/2
[replace]: https://github.com/nick29581/stupid-stats/blob/master/README.md
[what]: https://www.reddit.com/r/rust/comments/2wuxbe/quora_what_do_cc_systems_programmers_think_of_rust/
[killer]: https://www.reddit.com/r/rust/comments/2x0h17/whats_your_killer_rust_feature/
[removed]: https://www.reddit.com/r/rust/comments/2x2pon/what_significant_languageruntime_features_were/
[go]: http://learncamlirust.blogspot.de/
[sys]: https://vimeo.com/120512790
[alt]: http://words.steveklabnik.com/a-new-introduction-to-rust
[servo]: http://kmcallister.github.io/papers/2015-servo-experience-report-draft1.pdf

# Project Updates

* [Raft: A First Prototype][raft]. Hoverbear's implementation of the
  Raft consensus algorithm is working.
* [This Week in Servo 25][twis].
* [Rust DT]. An Eclipse-based Rust IDE that is making good progress.
* [Rust Share]. Share to play.rust-lang.org Straight from Sublime Text.
* [Kuchiki]. A vaporware HTML/XML tree manipulation library.
* [sketchy]. Probabalistic data structures.
* [ld9]. An OS X to Plan 9 cross-linker!
* [nom]. A byte oriented, zero copy parser combinator library with streaming support
* [There are Gentoo packages for Rust][gentoo].
* [literator]. Macros for container initialization.
* [open]. The 'open anything' library.
* [yup-oauth2]. An OAuthV2 library.
* [external_mixin]. A macro for generating Rust code using other languages.
* [clap]. Getopts-like argument parser.

[Rust Share]: https://github.com/GravityScore/Rust-Share
[Kuchiki]: http://users.rust-lang.org/t/kuchiki-a-vaporware-html-xml-tree-manipulation-library/435/6
[raft]: http://www.hoverbear.org/2015/02/24/raft-update-3/
[sketchy]: https://github.com/codahale/sketchy
[ld9]: https://github.com/alexchandel/ld9
[twis]: http://blog.servo.org/2015/02/24/twis-25/
[nom]: https://www.reddit.com/r/rust/comments/2x3mg0/nom_a_byte_oriented_zero_copy_parser_combinator/
[Rust DT]: http://users.rust-lang.org/t/rustdt-0-1-0-released-a-new-eclipse-rust-ide/460
[gentoo]: http://packages.gentoo.org/package/dev-lang/rust
[literator]: https://github.com/kmcallister/literator
[open]: https://crates.io/crates/open
[yup-oauth2]: https://crates.io/crates/yup-oauth2
[external_mixin]: https://www.reddit.com/r/rust/comments/2xch94/rust_mixin_python_mixin_macro_rules_with_realer/
[clap]: https://github.com/kbknapp/clap-rs

# Upcoming Events

* [3/9 Seattle][seattle].
* [3/10 San Diego][sd].
* [3/16 Paris][paris].
* [3/17 San Francisco][sf].

[seattle]: https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg
[sd]: http://sandiego.rs
[paris]: http://www.meetup.com/Rust-Paris
[sf]: http://www.meetup.com/Rust-Bay-Area/events/220627544/

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com
