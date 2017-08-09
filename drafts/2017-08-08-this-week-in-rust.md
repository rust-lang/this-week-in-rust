Title: This Week in Rust 194
Number: 194
Date: 2017-08-08
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

This week's crate is [aesni](https://crates.io/crates/aesni), a crate providing a Rust AES (Rijndael) block ciphers
implementation using AES-NI. Thanks to [newpavlov](https://users.rust-lang.org/u/newpavlov) for the suggestion.

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

105 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-07-31..2017-07-07

* [rewrite large float math in Rust and use it for stable compile-time evaluation](https://github.com/rust-lang/rust/pull/43554)
* [borrowck: skip CFG construction when there is nothing to propagate](https://github.com/rust-lang/rust/pull/43547) (awesome memory savings)
* [cycle-free dependency graph](https://github.com/rust-lang/rust/pull/43590)
* [fix instability in import suggestions](https://github.com/rust-lang/rust/pull/43552) (alas, re-exports still aren't correctly filtered)
* [fix quadratic performance on `use` statements](https://github.com/rust-lang/rust/pull/43584)
* [save-analysis fixes](https://github.com/rust-lang/rust/pull/43533)
* [save subobligations in the projection cache](https://github.com/rust-lang/rust/pull/43546)
* [don't warn on unused `union` fields](https://github.com/rust-lang/rust/pull/43397)
* [error code & explanation for calling private methods from outside](https://github.com/rust-lang/rust/pull/43699)
* [improve error message for trying static dispatch on trait object](https://github.com/rust-lang/rust/pull/43600)
* [the case of the missing error codes](https://github.com/rust-lang/rust/pull/43709)
* [{`StdIn`, `StdOut`, `StdErr`}`.as_raw_fd()`](https://github.com/rust-lang/rust/pull/43459)
* [MIR Validate statement](https://github.com/rust-lang/rust/pull/43403) (hook for unsafe code guidelines validation via miri)
* [MIR don't build unused unwind cleanup blocks](https://github.com/rust-lang/rust/pull/43576)
* [MIR trans no longer ICEs on assignment errors](https://github.com/rust-lang/rust/pull/43568)
* [more parallelization between trans and LLVM](https://github.com/rust-lang/rust/pull/43506)
* [inline bitwise modification ops](https://github.com/rust-lang/rust/pull/43581)

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
