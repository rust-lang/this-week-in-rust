Title: This Week in Rust 148
Number: 148
Date: 2016-09-20
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

* [a RustConf travelogue](http://zackmdavis.net/blog/2016/09/rustconf-2016-travelogue/)
* [GFX Programming Model](http://gfx-rs.github.io/2016/09/14/programming-model.html). A deep dive into what makes gfx-rs complex and awesome.

## New Crates & Project Updates

* [releasetag](https://crates.io/crates/releasetag) release-tags to be extracted from core-files postmortem (upgrade 0.6.0 for rust 1.12/1.13)

# Crate of the Week

This week's crate of the week is (the in best TWiR-tradition shamelessly self-promoted) [mysql-proxy](https://crates.io/crates/mysql-proxy), a flexible, lightweight and scalable proxy for MySQL databases. Thanks to [andygrove](https://users.rust-lang.org/users/andygrove) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [hard] [rust: Support Apple app store bitcode](https://github.com/rust-lang/rust/issues/35968).
* [hard] [rust: Missed opportunities to eliminate bounds checks](https://github.com/rust-lang/rust/issues/35981).
* [easy] [tempdir: make directory removal robust on windows](https://github.com/rust-lang-nursery/tempdir/issues/15). This bug lets you publish a replacement for the unreliable `std::fs::remove_dir_all` fn.
* [moderate] [rust: Create official .deb packages](https://github.com/rust-lang/rust/issues/28307).
* [easy] [rust-www: Better front-page example](https://github.com/rust-lang/rust-www/issues/180).
  The front page example on the website isn't so special. Make it shine.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

98 pull requests were [merged in the last two weeks][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-09-12..2016-09-19

* [Macro invocations now fold/visit in the same order](https://github.com/rust-lang/rust/pull/36555) (makes it easier to reason about them)
* [Optimized parser's last token handling](https://github.com/rust-lang/rust/pull/36527) (who'd have thought it could be optimized further?)
* [Some dependency graph improvements](https://github.com/rust-lang/rust/pull/35960)
* [Rustbuild now supports python3](https://github.com/rust-lang/rust/pull/36509)
* [LLVM updated](https://github.com/rust-lang/rust/pull/36508)
* [Don't lose padding for constant closures and tuples](https://github.com/rust-lang/rust/pull/36406) (fixes [#36401](https://github.com/rust-lang/rust/issues/36401) segfault)
* [Improved move checker accuracy and error reports](https://github.com/rust-lang/rust/pull/36353)
* [Better error message when shadowing type with generics](https://github.com/rust-lang/rust/pull/36338)
* [Improve Macro-1.1 errors labelling](https://github.com/rust-lang/rust/pull/36308)
* [`SyntaxExtension::MacroRulesTT` is no more](https://github.com/rust-lang/rust/pull/36444)
* [`#[derive(Clone, Eq)]` produces less code](https://github.com/rust-lang/rust/pull/36384) (Yay! faster builds!)
* [Default stack size upped to 16MiB](https://github.com/rust-lang/rust/pull/36505) (temporary measure against stack overflows)
* [Change in invoking drop glue for boxed dynamically-sized values](https://github.com/rust-lang/rust/pull/36459) (fixes LLVM assertion failure)
* [`private_in_public` error demoted to warning](https://github.com/rust-lang/rust/pull/36270) (until remaining regressions are fixed, also in beta)
* [MIR optimization: Remove reborrows for references](https://github.com/rust-lang/rust/pull/36504) (and already pass dependencies seem to become subtle...)
* [Better parent info for `-Z save-analysis`](https://github.com/rust-lang/rust/pull/36487)
* [Avoid loading/parsing unused modules](https://github.com/rust-lang/rust/pull/36482) (e.g. `#[cfg(any())] mod foo`)
* [Fix closure-as-trait-object dropping](https://github.com/rust-lang/rust/pull/36468)
* [`Duration::checked_`{`add`, `sub`, `mul`, `div`}](https://github.com/rust-lang/rust/pull/36463)
* [`ty::TraitObject`'s projection bounds are now stably sorted](https://github.com/rust-lang/rust/pull/36425) (also unifies/removes diverse hashing implementations)
* [De-specialized `Zip` data](https://github.com/rust-lang/rust/pull/36490) (some ongoing optimization work)
* [`Iterator::sum()` and `product()` no longer check for overflow in release mode](https://github.com/rust-lang/rust/pull/36372)
* [Fix poor performance in `Vec::`{`extend_from_slice`,`extend_with_element`}`()`](https://github.com/rust-lang/rust/pull/36355)
* [`std::str::replacen(..)`](https://github.com/rust-lang/rust/pull/36347)
* [Zero the first byte of `CString`s on drop](https://github.com/rust-lang/rust/pull/36264)
* [`likely(_)`/`unlikely(_)` intrinsics added](https://github.com/rust-lang/rust/pull/36181) (help the CPU with branch prediction)
* [`std::io::Take::into_inner()`](https://github.com/rust-lang/rust/pull/36019)
* [`std::alloc::`{`Rc`, `Arc`}`::ptr_eq(..)`](https://github.com/rust-lang/rust/pull/35992)
* [`compiler-rt` is dead, long live `compiler-builtins`!](https://github.com/rust-lang/rust/pull/35021)
* [dist tarball now contains version info](https://github.com/rust-lang/rust/pull/36213)
* [sublime-rust now works with the new error format](https://github.com/rust-lang/sublime-rust/pull/87)

## New Contributors

* Cobrand
* Jake Goldsborough
* John Firebaugh
* Justin LeFebvre
* Kylo Ginsberg
* Nicholas Nethercote
* orbea
* Richard Janis Goldschmidt
* Ulrich Weigand

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1620: regex 1.0](https://github.com/rust-lang/rfcs/pull/1620).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [`mem::discriminant()`](https://github.com/rust-lang/rfcs/pull/1696). Add a function that extracts the discriminant from an enum variant as a comparable, hashable, printable, but (for now) opaque and unorderable type.
* [Add a compiler flag that emits crate dependencies on a best-effort basis](https://github.com/rust-lang/rfcs/pull/1622).

## New RFCs

* [Check future-proofing of `macro_rules!` using FIRST sets](https://github.com/rust-lang/rfcs/pull/1746).

# Upcoming Events

* **[9/17. Rustfest Europe Conference](http://www.rustfest.eu/)**.
* [9/19. Paris - Rust Paris](https://www.meetup.com/Rust-Paris/events/230111512/).
* [9/20. Rust NYC Meetup](https://www.meetup.com/Rust-NYC/events/233756447/).
* [9/21. Rust Boulder/Denver Monthly Meeting](https://www.meetup.com/Rust-Boulder-Denver/events/233463725/).
* 9/21. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [9/22. RustPH Mentors Meeting](http://www.rustph.tech/).
* 9/22. Rust release triage at #rust-triage on irc.mozilla.org.
* [9/26. São Paulo Meetup](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/233713814/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

* [Rust engineer at MaidSafe](http://maidsafe.net/careers.html#rust_engineer).
* [Rust developer at ANIXE](http://anixe.pl/rust_dev/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
