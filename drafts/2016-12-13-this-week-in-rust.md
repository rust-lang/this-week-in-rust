Title: This Week in Rust 160
Number: 160
Date: 2016-12-13
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

## Other Weeklies from Rust Community

# Crate of the Week

This week, sadly no crate was nominated. [Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704


# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [less easy] [unicode-reverse: Fuzz testing](https://github.com/mbrubeck/unicode-reverse/issues/2). unicode-reverse is a Unicode-aware in-place string reverse function in Rust.
* [easy] [tera: Use 64 bits for int/float](https://github.com/Keats/tera/issues/79). Tera is a template engine for Rust based on Jinja2/Django.
* [easy] [tera: Fix include whitespace](https://github.com/Keats/tera/issues/72).
* [easy] [tera: Adding tests (not unit test, the tester feature)](https://github.com/Keats/tera/issues/62).
* [hard] [tera: Add not to mean `!`](https://github.com/Keats/tera/issues/39).
* [hard] [tera: Add a magical variable that dumps the context](https://github.com/Keats/tera/issues/74).
* [less easy] [rayon: Parity with the `Iterator` trait](https://github.com/nikomatsakis/rayon/milestone/2). Rayon: A data parallelism library for Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

74 pull requests were [merged in the last week][merged]. This contains a good number of plugin-breaking changes.

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-12-05..2016-12-12

* [Use link(kind) annotation to fix native Windows imports](https://github.com/rust-lang/rust/pull/37973) ([RFC #1717](https://github.com/rust-lang/rfcs/blob/master/text/1717-dllimport.md))
* [Function arity errors now show the original definition](https://github.com/rust-lang/rust/pull/38121)
* [`HashMap`/-`Set` now allocate smarter on `extend(_)`, `from_iter(_)`](https://github.com/rust-lang/rust/pull/38017)
* [Faster `sort()`](https://github.com/rust-lang/rust/pull/38192)
* [More forwarded `ExactSizeIterator` / `is_empty()`](https://github.com/rust-lang/rust/pull/38149)
* [Stricter lifetimes for `LateLintPass`](https://github.com/rust-lang/rust/pull/38191) (Plugin-unbreaking 🙂)
* [`Iterator::nth(_)` no longer needs `self` to be `Sized`](https://github.com/rust-lang/rust/pull/38134)
* [Incremental compilation will now recompile items on visibility change](https://github.com/rust-lang/rust/pull/38272)
* [`-Z always_encode_mir`](https://github.com/rust-lang/rust/pull/38217)
* [dylib symbol handling improvements](https://github.com/rust-lang/rust/pull/38117)
* [`tidy` now checks the licenses of vendored dependencies](https://github.com/rust-lang/rust/pull/38291)
* [Rustbuild is now the default build system](https://github.com/rust-lang/rust/pull/37817) (1 year in the making)
* [Allow `--test` on procedural-macro crates](https://github.com/rust-lang/rust/pull/38107)
* [Cargo will pick up `build.rs` scripts by default](https://github.com/rust-lang/cargo/pull/3361) unless `package.build = false` explicitly, for now warns
* [`cargo test --all` for Workspaces](https://github.com/rust-lang/cargo/pull/3221)

## New Contributors

* Clar Charr
* Theodore DeRego
* Xidorn Quan

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1636: Require documentation for all new features](https://github.com/rust-lang/rfcs/pull/1636).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Allow intrinsics to be marked as _safe_, overriding the implicit `unsafe` from being in an extern block](https://github.com/rust-lang/rfcs/pull/1248).
* [Procedural macros](https://github.com/rust-lang/rfcs/pull/1566).

## New RFCs

* [Default struct field values](https://github.com/rust-lang/rfcs/pull/1806).
* [Alloca for Rust](https://github.com/rust-lang/rfcs/pull/1808). Add a builtin `fn core::mem::reserve<'a, T>(elements: usize) -> StackSlice<'a, T>` that reserves space for the given number of elements on the stack and returns a `StackSlice<'a, T>` to it which derefs to `&'a [T]`.

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [Customising Rustfmt (FCP)](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).
* [Conventions for Cargo.toml files](https://github.com/rust-lang-nursery/fmt-rfcs/pull/41).

Final comment period:

* [boolean and arithmetic expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/18).
* [struct and union declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/30).
* [type aliases](https://github.com/rust-lang-nursery/fmt-rfcs/issues/32).
* [match](https://github.com/rust-lang-nursery/fmt-rfcs/issues/34).
* [#[macro_use]](https://github.com/rust-lang-nursery/fmt-rfcs/issues/36).
* [To indent empty lines or not?](https://github.com/rust-lang-nursery/fmt-rfcs/issues/37).

Other notable issues:

* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).

# Upcoming Events

* [12/8. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/235498108/).
* [12/8. San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/236011811/).
* [12/12. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/235157890/).
* [12/14. South Florida Rust: Intro to Rust](https://www.meetup.com/South-Florida-Rust-Meetup/events/235596291/).
* [12/14. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [12/14. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [12/15. Rust Bay Area: Syn/Macros 1.1, Helix, and Binding C in OpenSSL](https://www.meetup.com/Rust-Bay-Area/events/235285192/).
* [12/17. South Florida Rust: Intro to Rust](https://www.meetup.com/South-Florida-Rust-Meetup/events/235596339/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

* [Mozilla Research Internship (US/INTL) - University 2017](https://careers.mozilla.org/position/gh/503816).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Such large. Very 128. Much bits.

— [@nagisa introducing 128-bit integers in Rust](https://github.com/rust-lang/rust/pull/37900/commits/760da30ce3cfe69a7fed38d528e7228365c60b87).

Thanks to [leodasvacas](https://users.rust-lang.org/users/leodasvacas) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
