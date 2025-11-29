Title: This Week in Rust 169
Number: 169
Date: 2017-02-14
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

* 🎈🎉 [Announcing Rust 1.15.1](https://blog.rust-lang.org/2017/02/09/Rust-1.15.1.html). 🎉🎈
* [rustc compiler team design sprint -- summary](http://smallcultfollowing.com/babysteps/blog/2017/02/12/compiler-design-sprint-summary/).
* [Specialization, coherence, and API evolution](https://aturon.github.io/blog/2017/02/06/specialization-and-coherence/).
* [Safer microcontrollers almost here](http://dylanmckay.io/blog/rust/avr/llvm/2017/02/09/safer-microcontrollers-almost-here.html). Rust on AVR microcontrollers.
* [Optimizing Rc memory usage in Rust](https://medium.com/@robertgrosse/optimizing-rc-memory-usage-in-rust-6652de9e119e).
* [Content aware image resizing in Rust](https://mht.technology/post/content-aware-resize/).
* [How to sanitize your Rust code](https://users.rust-lang.org/t/howto-sanitize-your-rust-code/9378).
* [Safe Elixir and Erlang NIFs in Rust with Rustlr](http://hansihe.com/2017/02/05/rustler-safe-erlang-elixir-nifs-in-rust.html).
* [Buffers in Rust](http://rust.fastmail.com.user.fm/2017/02/rusty-buffers/).
* [What Elm and Rust teach us about the Future](https://dev.to/martincerny/what-elm-and-rust-teach-us-about-the-future).
* [Chrono 0.3 released, and the future](https://users.rust-lang.org/t/chrono-0-3-released-and-the-future/9340).
* [This week in Rust docs 43](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-43).
* [This week in Servo 92](https://blog.servo.org/2017/02/13/twis-92/).

# Crate of the Week

This week's crate of the week is [derive_builder](https://crates.io/crates/derive_builder), automatically implement the builder pattern for arbitrary structs. Now with macro 1.1 support (custom derive since Rust 1.15). Thanks to [Willi Kappler](https://users.rust-lang.org/users/willi_kappler) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: If you are on nightly, help test Rust's new release infrastructure](https://internals.rust-lang.org/t/new-nightlies-coming-soon-help-out-and-test/4789).
* [easy] [crates.io: Document applying categories/adding new categories](https://github.com/rust-lang/crates.io/issues/544).
* [easy] [crates.io: Crate listings duplicated on category subpages](https://github.com/rust-lang/crates.io/issues/524).
* [easy] [crates.io: Update the site to include the email address help@crates.io somewhere](https://github.com/rust-lang/crates.io/issues/522).
* [easy] [crates.io: Dependent crates link not shown for crates without reverse dependencies](https://github.com/rust-lang/crates.io/issues/379).
* [medium] [crates.io: Exact match not being obvious](https://github.com/rust-lang/crates.io/issues/493).
* [medium] [crates.io: Be able to search within a keyword or category](https://github.com/rust-lang/crates.io/issues/491).
* [easy] [servo: Set origin header in http_network_or_cache_fetch](https://github.com/servo/servo/issues/14787).
* [easy/hard] [clippy: Lint unwrap in drop impls](https://github.com/Manishearth/rust-clippy/issues/1523).
* [easy] [clippy:  `to_mut` false positive](https://github.com/Manishearth/rust-clippy/issues/1530).
* [easy] [clippy: `len_without_is_empty` requires `allow` in the wrong place](https://github.com/Manishearth/rust-clippy/issues/1532).
* [easy/medium] [clippy: Lint for Iterator + Copy](https://github.com/Manishearth/rust-clippy/issues/1534).
* [easy] [clippy: calls to `std::mem::drop` with a `Copy` type](https://github.com/Manishearth/rust-clippy/issues/1537).


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

153 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-06..2017-02-13

* [Delete the makefile build system](https://github.com/rust-lang/rust/pull/39431).
* [ICE when mutably accessing immutable enum fixed](https://github.com/rust-lang/rust/pull/39602)
* [early-bind return type-only lifetimes](https://github.com/rust-lang/rust/pull/38897)
* [structured `repr` representation](https://github.com/rust-lang/rust/pull/39595)
* [simplified MIR conditional branching](https://github.com/rust-lang/rust/pull/39456)
* [stabilized static lifetimes in statics](https://github.com/rust-lang/rust/pull/39265) ([RFC #1623](https://github.com/rust-lang/rfcs/blob/master/text/1623-static.md))
* [`impl From<(I: Into<IpAddr>, u16)> for `{`SocketAddr`, `IpAddr`}](https://github.com/rust-lang/rust/pull/39372)
* [`impl Default for PathBuf`](https://github.com/rust-lang/rust/pull/38764)
* [specialize `PartialOrd<A> for [A] where A: Ord`](https://github.com/rust-lang/rust/pull/39642)
* [leak, address, memory & thread sanitizer support](https://github.com/rust-lang/rust/pull/38699)
* [compile rust data structures to Android](https://github.com/rust-lang/rust/pull/39724)
* [`cargo` now assumes `build.rs` is a build script](https://github.com/rust-lang/cargo/pull/3664)
* [`cargo` can now require features for `bin`s](https://github.com/rust-lang/cargo/pull/3667)

## New Contributors

* Aaron Power
* Alexander Battisti
* bjorn3
* Charlie Fan
* Gheorghe Anghelescu
* Giang Nguyen
* Henning Kowalk
* Ingvar Stepanyan
* Jan Zerebecki
* Jordi Polo
* Mario
* Rob Speer
* Shawn Walker-Salas

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1558: Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: close] [Add a `Transmute<T>` trait for representing types that can be transmuted to `T`](https://github.com/rust-lang/rfcs/pull/1891).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: close] [Warn by default when encountering a statement which only consists of an equality comparison](https://github.com/rust-lang/rfcs/pull/1812).
* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).

## Closed RFCs

Following proposals were rejected by [the team](https://www.rust-lang.org/team.html) after their 'final comment period' elapsed.

*No RFCs were closed this week!*

## New RFCs

* [Amend RFC 1105 to specify how dependency versions relate to semver](https://github.com/rust-lang/rfcs/pull/1890).
* [Deprecate uninitialized for uninhabited types](https://github.com/rust-lang/rfcs/pull/1892).
* [Introduce _pattern synonyms_ - used to create new patterns we can pattern match against from real patterns](https://github.com/rust-lang/rfcs/pull/1895).
* [Unions 1.2](https://github.com/rust-lang/rfcs/pull/1897).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [match](https://github.com/rust-lang-nursery/fmt-rfcs/pull/56)
* [type aliases](https://github.com/rust-lang-nursery/fmt-rfcs/pull/55)
* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)

Issues in final comment period:

* [`extern` vs `extern "C"`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/52)
* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [types](https://github.com/rust-lang-nursery/fmt-rfcs/issues/15)

Other significant issues:

* [ranges](https://github.com/rust-lang-nursery/fmt-rfcs/issues/60)
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

# Upcoming Events

* [Feb 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 16. Rust DC Learn + Try: clap.rs](https://www.meetup.com/RustDC/events/236719329/).
* [Feb 16. Rome Rust User Group Meetup](https://www.meetup.com/it-IT/Rust-Roma/events/237551678/).
* [Feb 18. Rust NYC: Rust Hack & Learn](https://www.meetup.com/Rust-NYC/events/237386964/).
* [Feb 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb 22. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658946/).
* [Feb 23. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb 23. Rust Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/24).
* [Feb 24. Crate Polishing Workshop, Darmstadt/Germany](https://www.meetup.com/Rust-Rhein-Main/events/237509289/).
* [Mar  1. South Florida Rust - Intro to Ownership and Borrowing](https://www.meetup.com/South-Florida-Rust-Meetup/events/237559303/).
* [Mar  1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Full Stack Developer for Resin Supervisor (JavaScript + Rust)](https://resin.workable.com/jobs/399897).
* [Postdoc positions for RustBelt project](http://lists.seas.upenn.edu/pipermail/types-announce/2017/006485.html) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
