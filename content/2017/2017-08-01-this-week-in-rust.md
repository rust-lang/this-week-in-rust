Title: This Week in Rust 193
Number: 193
Date: 2017-08-01
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

* The [Rust Herald](https://herald.community.rs) has been in testing for a while and after fixing some bugs, is now ready for general use. The Herald is meant for small news snippets: your new crate, a meetup announcement, or some short thoughts on a subject. All posts will be shared through RSS and Twitter.
* [Announcing the “impl period”, Sep 18 - Dec 17](https://internals.rust-lang.org/t/announcing-the-impl-period-sep-18-dec-17/5676).
* [Revisiting Rust’s modules](https://aturon.github.io/blog/2017/07/26/revisiting-rusts-modules/). ([Discuss here](https://internals.rust-lang.org/t/revisiting-rusts-modules/5628)).
* [Announcing the `http` crate](https://users.rust-lang.org/t/announcing-the-http-crate/12123).
* [IntelliJ Rust plugin is now officially supported by JetBrains](https://intellij-rust.github.io/2017/07/31/changelog-47.html).
* [All contain-rs crates are now in maintenance-mode](https://users.rust-lang.org/t/all-contain-rs-crates-are-now-in-maintenance-mode/12056).
* [Implementing a bot for Slack in Rust, Rocket and Anterofit - Part 1](https://abishov.com/2017/07/27/hexocat-bot-part-1.html).
* [Grokking Diesel ORM](https://medium.com/@Buys/grokking-diesel-652cb8886a63).
* [Parsing NES ROM Headers with nom](https://bheisler.github.io/post/nes-rom-parser-with-nom/).
* [cortex-m-rtfm v2: simpler, less overhead and more device support](http://blog.japaric.io/rtfm-v2/).
* [Writing GStreamer applications in Rust](https://coaxion.net/blog/2017/07/writing-gstreamer-applications-in-rust/).
* [Rust: Not so great for codec implementing](https://codecs.multimedia.cx/?p=1246).
* [This week in Rust docs 66](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-66).
* [This week in Redox 27](https://redox-os.org/news/this-week-in-redox-27/).

# Crate of the Week

This week's crate is [tarpaulin](https://crates.io/crates/cargo-tarpaulin), a crate to collect test coverage of your Rust code. Thanks to [Colin Kiegel](https://users.rust-lang.org/u/colin_kiegel)
for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Proposals is open for Rust Belt Rust 2017 until 7 August](http://cfp.rust-belt-rust.com/).
* [Get in the swing with the libz blitz contest: Win free tickets to RustFest Zürich](http://blog.rustfest.eu/libz-blitz).
* [less easy] [bindgen: Emitting or deriving trait implementations](https://github.com/rust-lang-nursery/rust-bindgen/issues/886).
* [less easy] [bindgen: Emit a "manual" implementation of Debug when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/875).
* [less easy] [bindgen: Derive Hash when we can](https://github.com/rust-lang-nursery/rust-bindgen/issues/876).
* [less easy] [bindgen: "manually" implement Hash when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/877).
* [less easy] [bindgen: "manually" implement PartialEq when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/879).
* [less easy] [bindgen: Derive `Eq` when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/880).
* [less easy] [bindgen: "manually" implement Eq when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/881).
* [less easy] [bindgen: Derive PartialOrd when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/882).
* [less easy] [bindgen: "manually" implement PartialOrd when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/883).
* [less easy] [bindgen: Derive Ord when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/884).
* [less easy] [bindgen: "manually" implement Ord when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/885).
* [doc] [same-file: Note why `Handle` methods in `win` won't panic](https://github.com/BurntSushi/same-file/issues/8). `same-file` is a cross platform Rust library for checking whether two file paths are the same file.
* [easy] [same-file: Add `html_root` attribute](https://github.com/BurntSushi/same-file/issues/12).
* [doc] [same-file: Note that `dev` and `ino` methods are unix only](https://github.com/BurntSushi/same-file/issues/14).
* [doc] [same-file: Note why `Handle` methods in `unix` won't panic](https://github.com/BurntSushi/same-file/issues/7).
* [doc] [same-file: Clarify Approach to comparing files in docs](https://github.com/BurntSushi/same-file/issues/2).
* [walkdir: Implement `Clone` for `WalkDir`](https://github.com/BurntSushi/walkdir/issues/54). Walkdir is a Rust library for walking directories recursively.
* [doc] [rust-ffi-guide: Make the book more consistent](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/8).
* [easy] [ggez: Input doesn't work on mac using tmux and iterm2](https://github.com/ggez/ggez/issues/30). ggez is a Rust library to create good games easily.
* [easy] [ggez: SDL controller input doesn't work](https://github.com/ggez/ggez/issues/35).
* [doc] [ggez: Finish full building-and-install docs for each platform](https://github.com/ggez/ggez/issues/118).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

146 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-07-24..2017-07-31

* [1.20 stabilizations](https://github.com/rust-lang/rust/pull/43373)
* [Better diagnostics and recovery for `mut ref` in patterns](https://github.com/rust-lang/rust/pull/43489)
* [Point at path segment on module not found](https://github.com/rust-lang/rust/pull/43447)
* [proc macros keep more span information](https://github.com/rust-lang/rust/pull/43230)
* [simplify syntax path parsing](https://github.com/rust-lang/rust/pull/43438)
* [no more `-Z` in stable `--help`](https://github.com/rust-lang/rust/pull/43556) (the option was removed from stable in 1.19.0)
* [fix `-Z verbose` region printing](https://github.com/rust-lang/rust/pull/43458)
* [save subobligations in projection cache](https://github.com/rust-lang/rust/pull/43546)
* [Improve checking of conflicting packed and align representation hints on structs and unions](https://github.com/rust-lang/rust/pull/43443)
* [ARM hard-float supports homogeneous aggregates](https://github.com/rust-lang/rust/pull/43518)
* [rustdoc: link `[src]` of associated `fn`s in `impl`s](https://github.com/rust-lang/rust/pull/43509)
* [rustdoc: print associated types in traits "implementors" section](https://github.com/rust-lang/rust/pull/43515)
* [rustdoc: add unions to whitelist of sidebar types](https://github.com/rust-lang/rust/pull/43446)
* [cargo now applies `--all` on virtual workspaces](https://github.com/rust-lang/cargo/pull/4335)


## New Contributors

* Daiki Mizukami
* Danek Duvall
* Isaac van Bakel
* Joshua Liebow-Feeser
* MaulingMonkey
* Richard Dodd

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2027: Tweak object safety rules to allow static dispatch](https://github.com/rust-lang/rfcs/pull/2027).
* [RFC 2057: Add replace and swap functions to RefCell](https://github.com/rust-lang/rfcs/pull/2057).
* ["Guide-level" and "Reference-level" explanations to replace how we teach and detailed design sections](https://github.com/rust-lang/rfcs/pull/2059).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Generic associated types (associated type constructors)](https://github.com/rust-lang/rfcs/pull/1598).
* [disposition: merge] [Unsafe pointer methods](https://github.com/rust-lang/rfcs/pull/1966).
* [disposition: merge] [Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [disposition: postpone] [Eager expansion of macros](https://github.com/rust-lang/rfcs/pull/1628).
* [disposition: merge] [Future-proofing enums/structs with `#[non_exhaustive]` attribute](https://github.com/rust-lang/rfcs/pull/2008).
* [disposition: merge] [Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).

## New RFCs

* [Automatically usable external crates](https://github.com/rust-lang/rfcs/pull/2088).
* [Implied bounds](https://github.com/rust-lang/rfcs/pull/2089).
* [Semantic inlining (another try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [Allow Irrefutable Patterns in if-let and while-let statements](https://github.com/rust-lang/rfcs/pull/2086).
* [Warning on tautology in if or while statements](https://github.com/rust-lang/rfcs/pull/2087).
* [Add `std::net::MacAddr48` to `std::net`](https://github.com/rust-lang/rfcs/pull/2082).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

Currently being discussed:

* [Define short](https://github.com/rust-lang-nursery/fmt-rfcs/issues/47)
* [Special casing some macros](https://github.com/rust-lang-nursery/fmt-rfcs/issues/86)


# Upcoming Events

* [Aug  7. Rust Belt Rust 2017 CFP deadline](http://cfp.rust-belt-rust.com/).
* [Aug  9. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug  9. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Aug  9. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywlbmb/).
* [Aug 10. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Aug 10. Columbus Rust Society - Monthly meetup](https://www.meetup.com/columbus-rs/events/czcwhlywlbnb/).
* [Aug 14. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/241535500/).
* [Aug 16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug 16. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* **[Aug 18-19. RustConf 2017](http://rustconf.com/)**.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust developer at SparkTG India](https://twitter.com/by1x/status/887653738252451840).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
