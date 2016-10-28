Title: This Week in Rust 154
Number: 154
Date: 2016-11-01
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## Blog Posts

## News & Project Updates

The community team hosts a hybrid hack event on November 19th/20th centered around the African/European time zones. If you are interested in participating, or supplying topics to work on, please head to the [novemb.rs](http://novemb.rs) website.

## Other Weeklies from Rust Community

# New Crates

# Crate of the Week

*No crate was selected for CotW.*

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Unused import warning should have identifier in first line](https://github.com/rust-lang/rust/issues/37376).
* [easy] [rust: Provide a better error message when the target sysroot is not installed](https://github.com/rust-lang/rust/issues/37131).
* [hard] [rust: Optimize emscripten targets with emcc](https://github.com/rust-lang/rust/issues/36899).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

126 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-10-17..2016-10-24

* [macros 1.1: future proofing and cleanup](https://github.com/rust-lang/rust/pull/37198).
* [Enable line number debuginfo in releases](https://github.com/rust-lang/rust/pull/37280).
* [Cargo: Use `CommandExt::exec` for `cargo run` on Unix](https://github.com/rust-lang/cargo/pull/2818).
* [libc: Add setresuid/setresgid for linux](https://github.com/rust-lang/libc/pull/434).
* [Implement `From<Cow<str>> for String` and `From<Cow<[T]>> for Vec<T>`](https://github.com/rust-lang/rust/pull/37326).
* [libc: Add support for Fuchsia](https://github.com/rust-lang/libc/pull/432).
* [Use a faster `deflate` setting](https://github.com/rust-lang/rust/pull/37298).
* [libc: Add UNIX 98 pty functions](https://github.com/rust-lang/libc/pull/431).
* [LLVM: Add triple for Fuchsia](https://github.com/rust-lang/rust/pull/37261).
* [macros: Future proof `#[no_link]`](https://github.com/rust-lang/rust/pull/37247).
* [ICH: Use 128-bit Blake2b hash instead of 64-bit SipHash for incr. comp. fingerprints](https://github.com/rust-lang/rust/pull/37233).
* [Expand `.zip()` specialization to `.map()` and `.cloned()`](https://github.com/rust-lang/rust/pull/37230).
* [Mark enums with non-zero discriminant as non-zero](https://github.com/rust-lang/rust/pull/37224).
* [impl `Debug` for `ReadDir`](https://github.com/rust-lang/rust/pull/37221).
* [macros: improve `$crate`](https://github.com/rust-lang/rust/pull/37213).
* [include LLVM version in `--version --verbose`](https://github.com/rust-lang/rust/pull/37200).
* [Lint against lowercase static mut](https://github.com/rust-lang/rust/pull/37162).
* [`#[may_dangle]` attribute](https://github.com/rust-lang/rust/pull/37117).
* [Optimize `Substs::super_fold_with`](https://github.com/rust-lang/rust/pull/37108).
* [Inline read_{un,}signed_leb128 and opaque::Decoder functions](https://github.com/rust-lang/rust/pull/37083).

## New Contributors

* Duncan
* loggerhead
* Ryan Senior
* Vangelis Katsikaros
* Артём Павлов [Artyom Pavlov]

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1721: Enable customizing the linkage of a platform's C runtime](https://github.com/rust-lang/rfcs/pull/1721).
* [RFC 1717: Use `#[link(kind)]` to fix imports from native libs on Windows](https://github.com/rust-lang/rfcs/pull/1717).
* [RFC 1682: Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).
* [RFC 1624: Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [A process for establishing the Rust roadmap](https://github.com/rust-lang/rfcs/pull/1728).
* [Windows subsystem support](https://github.com/rust-lang/rfcs/pull/1665).
* [Add Cortex-M targets to the compiler + binary releases of `core`](https://github.com/rust-lang/rfcs/pull/1645).

## New RFCs

* [Roadmap for 2017](https://github.com/rust-lang/rfcs/pull/1774).
* [A type representing an owned C-compatible wide string](https://github.com/rust-lang/rfcs/pull/1773).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [Customising Rustfmt](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).

[Issue 17](https://github.com/rust-lang-nursery/fmt-rfcs/issues/17) (comments) is ready for a PR, we'd love someone to help out with that, if you're interested ping someone in #rust-style.

*No FCP issues this week.*

New P-high issues:

* [Struct and union declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/30).
* [Enums and variants](https://github.com/rust-lang-nursery/fmt-rfcs/issues/31).
* [`type` aliases](https://github.com/rust-lang-nursery/fmt-rfcs/issues/32).

# Upcoming Events

* [10/26. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [10/26. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [10/27. Rust Belt Rust Conference - Pittsburgh](http://www.rust-belt-rust.com/).
* [10/31. Rust Paris](https://www.meetup.com/Rust-Paris/events/234528214/).
* [11/2. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [11/2. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [11/3. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [11/5. Servo / Rust Hackathon](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/234883249/?eventId=234883249).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
