Title: This Week in Rust 168
Number: 168
Date: 2017-02-07
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

* 🎈🎉 [Announcing Rust 1.15](https://blog.rust-lang.org/2017/02/02/Rust-1.15.html). 🎉🎈
* [Rust's 2017 roadmap](https://blog.rust-lang.org/2017/02/06/roadmap.html).
* [Announcing Diesel 0.10.0](https://github.com/diesel-rs/diesel/releases/tag/v0.10.0). Diesel now works on Rust stable.
* [Rocket v0.2](https://rocket.rs/news/2017-02-06-version-0.2/). Managed state & more.
* [Incremental Compilation is available on nightly and ready for public beta testing](https://internals.rust-lang.org/t/incremental-compilation-beta/4721).
* [Unsafe code and shared references](http://smallcultfollowing.com/babysteps/blog/2017/02/01/unsafe-code-and-shared-references/).
* [Communicating intent](https://github.com/jaheba/stuff/blob/master/communicating_intent.md). Understanding _newtype_ pattern, as well as the `From` and `Input` traits.
* [Writing Python extensions in Rust](https://kushaldas.in/posts/writing-python-extensions-in-rust.html).
* [Stupid tricks with Rust higher-order functions and "impl trait"](http://integer32.com/2017/02/02/stupid-tricks-with-higher-order-functions.html).
* [What Rust can do that other languages can't, in six short lines](http://robert.ocallahan.org/2017/02/what-rust-can-do-that-other-languages.html).
* [Benchmarking Paillier encryption in Rust, C, and more languages](https://medium.com/snips-ai/benchmarking-paillier-encryption-15631a0b5ad8).
* [Rust on Teensy part 2: Sending a message](https://branan.github.io/teensy/2017/01/28/uart.html). PJRC Teensy is a USB-based microcontroller development system.
* [This week in Rust docs 42](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-42).
* [This week in Servo 91](https://blog.servo.org/2017/02/06/twis-91/).
* [video] [Ferris makes Emulators 19](https://www.youtube.com/watch?v=bhokkyhFcKQ): Sweep and Mod 2.

# Crate of the Week

This week's crate of the week is [djangohashers](https://crates.io/crates/djangohashers), a Rust port of Django's password primitives. Thanks to [Ronaldo Ferreira](https://users.rust-lang.org/users/Racum) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [servo: Looking for something to work on](https://github.com/servo/servo/issues/15162).
* [easy] [clippy: Lint fns that take immutable refs and return mutables](https://github.com/Manishearth/rust-clippy/issues/1507).
* [kafka-rust: Parallel communication to brokers](https://github.com/spicavigo/kafka-rust/issues/63).
* [kafka-rust: Integration testing](https://github.com/spicavigo/kafka-rust/issues/138).
* [easy] [clippy: Lint for redundant cast](https://github.com/Manishearth/rust-clippy/issues/1497).
* [easy] [clippy: Exclude self-by-value trait methods implemented on Box<T> from boxed_local](https://github.com/Manishearth/rust-clippy/issues/1478).
* [easy] [clippy: Lint on method/struct fields sharing the same name](https://github.com/Manishearth/rust-clippy/issues/1481).
* [android-rs-glue: Add more arguments and use clap to parse the arguments](https://github.com/tomaka/android-rs-glue/issues/115).
* [tokei: Add package repositories](https://github.com/Aaronepower/tokei/issues/92).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

144 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-30..2017-02-06

* [Fix `IntoIter::as_mut_slice` signature](https://github.com/rust-lang/rust/pull/39466) (this made stable [1.15.1](https://github.com/rust-lang/rust-www/pull/712),
see [release notes](https://github.com/rust-lang/rust/pull/39517))
* [`Cell` for non-`Copy` types](https://github.com/rust-lang/rust/pull/39287) (implements RFC [#1651](https://github.com/rust-lang/rfcs/blob/master/text/1651-movecell.md))
* [implement `kind="static-nobundle"`](https://github.com/rust-lang/rust/pull/38426) (implements RFC [#1717](https://github.com/rust-lang/rfcs/blob/master/text/1717-dllimport.md))
* [`Option::get_or_insert`(`_with`)`()`](https://github.com/rust-lang/rust/pull/39289)
* [`Iterator::rfind(_)`](https://github.com/rust-lang/rust/pull/39399)
* [make `format!` use `String::with_capacity(_)`](https://github.com/rust-lang/rust/pull/39356)
* [branchless `.filter().count()`](https://github.com/rust-lang/rust/pull/39107)
* [specialize `ToString` for `String`, `Cow<str>`](https://github.com/rust-lang/rust/pull/39440)
* [slightly faster `slice::sort()`](https://github.com/rust-lang/rust/pull/39538) (remember that it got a massive speedup recently?)
* [`FileDesc::set_`{`nonblocking`, `cloexec`} use less syscalls](https://github.com/rust-lang/rust/pull/39514)
* [128-bit atomics](https://github.com/rust-lang/rust/pull/38959)
* [fix `TryFrom`/`TryInto` for 128-bit integers](https://github.com/rust-lang/rust/pull/39408)
* [better error message when adding two `&str`s](https://github.com/rust-lang/rust/pull/39116)
* [better error message on unknown derives](https://github.com/rust-lang/rust/pull/39444)
* [rustc no longer suggests things that don't exist](https://github.com/rust-lang/rust/pull/39443)
* [rustc notes lints by name that were activated by group](https://github.com/rust-lang/rust/pull/38103)
* [minimize dependency graph on incremental compilation](https://github.com/rust-lang/rust/pull/39424)
* [save-analysis more carefully generates paths](https://github.com/rust-lang/rust/pull/39453)
* [fix uninhabited `while let` patterns](https://github.com/rust-lang/rust/pull/39526)
* [warn about default diverging types](https://github.com/rust-lang/rust/pull/39009)
* [fix uninitialized variable in libbacktrace](https://github.com/rust-lang/rust/pull/39509)
* [disable FPO on `i686-pc-windows-gnu` targets](https://github.com/rust-lang/rust/pull/39379) (fixes backtraces)
* [rustdoc now works with non-feature crate attrs](https://github.com/rust-lang/rust/pull/38161)
* [cargo no longer leaks job](https://github.com/rust-lang/cargo/pull/3621)
* [cargo now works with new `rustdoc --test` output](https://github.com/rust-lang/cargo/pull/3616)
* [cargo now searches path dependencies of workspace members](https://github.com/rust-lang/cargo/pull/3562)
* [crates.io now supports gitlab CI badges](https://github.com/rust-lang/crates.io/pull/539)

## New Contributors

* bluecereal
* f001
* Freyskeyd
* Jean-Sébastien Pédron
* Jimmy Cuadra
* Sergey Pepyakin
* Trevor Spiteri

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1828: Rust bookshelf](https://github.com/rust-lang/rfcs/pull/1828). Create a "Rust Bookshelf" of learning resources for Rust.

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).
* [disposition: close] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: close] [Warn by default when encountering a statement which only consists of an equality comparison](https://github.com/rust-lang/rfcs/pull/1812).
* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).

## Closed RFCs

Following proposals were rejected by [the team](https://www.rust-lang.org/team.html) after their 'final comment period' elapsed.

* [Variant types and untagged enums](https://github.com/rust-lang/rfcs/pull/1450).

## New RFCs

* [Allow use of pipe operator in patterns](https://github.com/rust-lang/rfcs/pull/1882).
* [Move metadata into import libraries for MSVC](https://github.com/rust-lang/rfcs/pull/1883).
* [Add unstable sort to libcore](https://github.com/rust-lang/rfcs/pull/1884).
* [Add explicit proper tail calls to Rust via the `become` keyword](https://github.com/rust-lang/rfcs/pull/1888).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [match](https://github.com/rust-lang-nursery/fmt-rfcs/pull/56)
* [type aliases](https://github.com/rust-lang-nursery/fmt-rfcs/pull/55)
* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)

Ready for PR:

There's [a lot of them](https://github.com/rust-lang-nursery/fmt-rfcs/issues?q=is%3Aopen+is%3Aissue+label%3Aready-for-PR) right now, contributions here would be very welcome. If you want advice or help getting started, please ping nrc, or any other member of the style team, in #rust-style.

Issues in final comment period:

* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).

Other significant issues:

* [types](https://github.com/rust-lang-nursery/fmt-rfcs/issues/15)
* [closures](https://github.com/rust-lang-nursery/fmt-rfcs/issues/35)
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)
* [`extern` vs `extern "C"`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/52)

# Upcoming Events

* [2/8. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658909/).
* [2/9. Rust Bay Area: Security and Crypto with RustTLS and Macaroons](https://www.meetup.com/Rust-Bay-Area/events/237115024/).
* [2/8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/9. Rust Boulder/Denver - Redox OS with Denver Open Source OS](https://www.meetup.com/Rust-Boulder-Denver/events/237016107/).
* [2/9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/236907254/).
* [2/9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [2/13. Prague Rust Meetup](https://www.meetup.com/GDG-%C4%8CVUT-Prague/events/237352629/).
* [2/13. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/236811856/).
* [2/15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/16. Rust DC Learn + Try: clap.rs](https://www.meetup.com/RustDC/events/236719329/).
* [2/18. Rust NYC: Rust Hack & Learn](https://www.meetup.com/Rust-NYC/events/237386964/).
* [2/22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/23. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Full Stack Developer for Resin Supervisor (JavaScript + Rust)](https://resin.workable.com/jobs/399897).
* [Postdoc positions for RustBelt project](http://lists.seas.upenn.edu/pipermail/types-announce/2017/006485.html) at MPI-SWS.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Friends of the Forest

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. These people are
'friends of the forest'.

This week's friends of the forest are:

* From [jer]:

> I'd like to nominate [colin_kiegel] and [florob]. Both are co-organizers at the monthly [rust.cologne] meetup, keeping it running all the time.

[jer]: https://github.com/badboy
[colin_kiegel]: https://users.rust-lang.org/users/colin_kiegel/activity
[florob]: https://users.rust-lang.org/users/florob/activity
[rust.cologne]: http://rust.cologne/

* From [brson]:

> A few months ago [Yamakaky] pinged me and offered to help maintain [error-chain]. I eagerly dumped all maintenence duties on them, and since then they have made the code more elegant, dealt with all issue and pull request triage, and made the releases. If you've appreciated error-chain lately, appreciate @Yamakaky, Friend of the Forest. Thanks, @Yamakaky!

[brson]: https://github.com/brson
[Yamakaky]: https://github.com/Yamakaky
[error-chain]: https://github.com/brson/error-chain

# Quote of the Week

> Back in these days, we had none of this newfangled “stability” business. The compiler broke your code every two weeks. Of course, you wouldn’t know that because the compiler would usually crash before it could tell you that your code was broken! Sigils roamed the lands freely, and cargo was but a newborn child which was destined to eventually end the tyranny of Makefiles.

— [Manishearth in a blog post](https://manishearth.github.io/blog/2017/01/10/rust-tidbits-box-is-special/).

Thanks to [llogiq for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/344).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
