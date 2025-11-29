Title: This Week in Rust 186
Number: 186
Date: 2017-06-13
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

* 🎈🎉 [Announcing Rust 1.18](https://blog.rust-lang.org/2017/06/08/Rust-1.18.html). 🎉🎈
* [Rustup 1.4.0 released](https://users.rust-lang.org/t/rustup-1-4-0-released/11268).
* [How MutexGuard was Sync when it should not have been](https://www.ralfj.de/blog/2017/06/09/mutexguard-sync.html).
* [Exploring Rust's standard library: system calls and errors](https://people.gnome.org/~federico/blog/rust-libstd-syscalls-and-errors.html).
* [Exploring MIR Semantics through miri](https://www.ralfj.de/blog/2017/06/06/MIR-semantics.html).
* [Docs.rs now supports building non-default features](https://github.com/onur/docs.rs/pull/131).
* [video] ["Rust: Hack without fear!" as the opening talk at C++Now 2017](https://youtu.be/lO1z-7cuRYI).
* [This week in Redox 21](https://www.redox-os.org/news/this-week-in-redox-21/).
* [This week in Rust docs 60 ](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-60).

# Crate of the Week

This week's crate is [structopt](https://crates.io/crates/structopt), a crate that lets your auto-derive your command-line options from a struct to parse them into. Thanks to [m4b](https://users.rust-lang.org/u/m4b) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [stdx: Set up testing of every stdx crate](https://github.com/brson/stdx/issues/36). stdx is a collection of curated Rust libraries.
* [stdx: Create stdx-check tool](https://github.com/brson/stdx/issues/35).
* [stdx: Add num_cpus, threadpool to stdx](https://github.com/brson/stdx/issues/33).
* [easy] [lyon: Make the basic_shapes tessellation routines provide vertex normals](https://github.com/nical/lyon/issues/13). Lyon is a library for GPU-based 2D graphics rendering in Rust.
* [easy] [lyon: Implement rounded rectangle stroke tessellation](https://github.com/nical/lyon/issues/38).
* [easy] [lyon: Implement circle stroke tessellation](https://github.com/nical/lyon/issues/40).
* [easy] [lyon: Add fuzz testing for the fill tesselator](https://github.com/nical/lyon/issues/11).
* [easy] [lyon: Lyon should conform to the rust API guidelines](https://github.com/nical/lyon/issues/44).
* [easy] [lyon: Implement computing the bounding rect for LineSegment and Triangle](https://github.com/nical/lyon/issues/65).
* [medium] [lyon: Implement clipping line joins at the miter limit](https://github.com/nical/lyon/issues/35).
* [medium] [lyon: Implement round line caps](https://github.com/nical/lyon/issues/33).
* [medium] [lyon: Implement round line joins](https://github.com/nical/lyon/issues/5).
* [medium] [lyon: Implement bevel line joins](https://github.com/nical/lyon/issues/34).
* [lyon: Make the source code easier to understand and discover](https://github.com/nical/lyon/issues/36).
* [perceptia: Add support for offscreen mode with VNC or Spice](https://github.com/perceptia/perceptia/issues/22). Perceptia is a dynamic window manager with support for Wayland.
* [perceptia: Add support for libinput](https://github.com/perceptia/perceptia/issues/18).
* [perceptia: Add configuration for panel](https://github.com/perceptia/perceptia/issues/14).
* [easy] [perceptia: Generate better name for screenshot file](https://github.com/perceptia/perceptia/issues/9).
* [easy] [perceptia: Add more options for setting background](https://github.com/perceptia/perceptia/issues/5).
* [rustup: Create and publish a snap of rustup](https://github.com/rust-lang-nursery/rustup.rs/issues/1144).
* [rust: Create and publish a snap of Rust](https://github.com/rust-lang/rust/issues/42349).
* [rust: Update docker images to share scripts when possible](https://github.com/rust-lang/rust/issues/42201).
* [corrode-but-in-rust: let-defined lambdas aren't combined](https://github.com/tcr/corrode-but-in-rust/issues/65).
* [easy] [rust-url: Modify docs to put error conditions into `Errors` sections](https://github.com/servo/rust-url/issues/314).
* [log: Remove env_logger from this repository](https://github.com/rust-lang-nursery/log/issues/145).
* [log: Support construction of `Record`s and `Metadata`](https://github.com/rust-lang-nursery/log/issues/116).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

115 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-06-06..2017-06-13

* [epic macro expansion speedup](https://github.com/rust-lang/rust/pull/42533)
* [`unimplemented!(..)` now allows message](https://github.com/rust-lang/rust/pull/42155)
* [better suggestions for missing trait impls](https://github.com/rust-lang/rust/pull/42383)
* [disentangle `InferCtxt", "MemCategorizationContext", `ExprUseVisitor`](https://github.com/rust-lang/rust/pull/42563) (plugin-breaking, but clippy already fixed)
* [convert `StdIo` from `File`, `ChildStdout`, `ChildStderr`](https://github.com/rust-lang/rust/pull/42133)
* [disallow "String"_ literals](https://github.com/rust-lang/rust/pull/41990) (No clue why this compiled in the first place?)
* [speed up `mem::swap(..)`](https://github.com/rust-lang/rust/pull/40454)
* [improve codegen for `.next_power_of_two` (and others)](https://github.com/rust-lang/rust/pull/42556)
* [our tests need more jokes!](https://github.com/rust-lang/rust/pull/42247) (look at the change list for some laughs)
* [don't panic, `rust_eh_parsonality()`!](https://github.com/rust-lang/rust/pull/42487)
* [don't store zero-sized pair fields](https://github.com/rust-lang/rust/pull/42486)
* [`#[inline]` `io::Error::from`](https://github.com/rust-lang/rust/pull/42426)
* [`use foo::self;` is now only one error](https://github.com/rust-lang/rust/pull/42580)
* [everything outlives the `'empty` lifetime](https://github.com/rust-lang/rust/pull/42482)
* [doctests skip files without three backticks](https://github.com/rust-lang/rust/pull/42437)
* [speed up cargo](https://github.com/rust-lang/cargo/pull/4118)

## New Contributors

* Arthur Arnold
* Campbell Barton
* Fuqiao Xue
* gentoo90
* Inokentiy Babushkin
* Michael Killough
* Nick Whitney

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2005: Match ergonomics using default binding modes](https://github.com/rust-lang/rfcs/pull/2005).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: merge] [Conversions from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rfcs/pull/1789).
* [disposition: merge] [Tiered browser support policy for Rust's web content](https://github.com/rust-lang/rfcs/pull/1985).
* [disposition: merge] [Prepare global allocators for stabilization](https://github.com/rust-lang/rfcs/pull/1974).

## New RFCs

* [Privacy for enum variants and trait items](https://github.com/rust-lang/rfcs/pull/2028).
* [Tweak object safety rules to allow static dispatch](https://github.com/rust-lang/rfcs/pull/2027).
* [Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [[macro_use] on the same line as crate](https://github.com/rust-lang-nursery/fmt-rfcs/issues/83)
* [trait bounds](https://github.com/rust-lang-nursery/fmt-rfcs/issues/80)
* [Specify rules for breaking long `where` conditions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/75)
* [Single-line `where`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/74)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [paths](https://github.com/rust-lang-nursery/fmt-rfcs/issues/69)
* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)
* [control flow](https://github.com/rust-lang-nursery/fmt-rfcs/issues/62)

# Upcoming Events

* [Jun 15. Rust DC Learn + Try: Embedded Rust](https://www.meetup.com/RustDC/events/239115658/).
* [Jun 15. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jun 16. RainOfRust Camp Nashik - III](https://reps.mozilla.org/e/rainofrust-camp-nashik-iii/).
* [Jun 17. Rust Bangalore - Rust 'core' Workshop on Types, Memory and Interfacing with other languages](https://www.meetup.com/rustox/events/240262219/).
* [Jun 17. RainOfRust Camp Gandhinagar, Gujarat](https://reps.mozilla.org/e/rainofrust-camp-gandhinagar-gujarat/).
* [Jun 17. Rust Activate - Ciudad de México](https://reps.mozilla.org/e/rust-activate/).
* [Jun 17. Rust Day Mexico City 2017](https://2017.rustday.mx/).
* [Jun 18. #RainOfRust Workshop in Pune](https://reps.mozilla.org/e/rainofrust-workshop-in-pune/).
* [Jun 19. First Belgian Rust meetup](https://users.rust-lang.org/t/first-belgian-rust-meetup/11172).
* [Jun 21. Rust Meetup Dresden](https://www.meetup.com/Mozilla-Community-Dresden/events/240188745/).
* [Jun 21. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jun 21. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun 24. RainOfRust Camp Ahmedabad, Gujarat](https://reps.mozilla.org/e/rainofrust-camp-ahmedabad-gujarat/).
* [Jun 27. Let's Rust - Hyderabad](https://reps.mozilla.org/e/let-s-rust/).
* [Jun 27. RainOfRust Camp Patan, Gujarat](https://reps.mozilla.org/e/rainofrust-camp-patan-gujarat/).
* [Jun 28. Boston Rust - Tutorial Bug-fixing Hackathon](https://www.meetup.com/BostonRust/events/240244837/).
* [Jun 28. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/240365553/).
* [Jun 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jun 28. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jun 29. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jul  4. Rust Utrecht - Rust Workshop](https://www.meetup.com/Rust-Utrecht/events/240660834/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Open Source Rust Engineer at Buoyant](https://rustjobs.rs/jobs/24/buoyant-open-source-rust-engineer).
* [Rust Developer at 1aim](https://rustjobs.rs/jobs/22/1aim-gmbh-rust-developer).
* [Rust Developer at Anixe](https://rustjobs.rs/jobs/21/anixe-rust-developer).
* [Rust Legend at Between Lines](https://rustjobs.rs/jobs/20/between-lines-ltd-rust-legend).
* Student Research Assistant for developing Clippy in Karlsruhe (contact oliver.schneider \at kit.edu).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
