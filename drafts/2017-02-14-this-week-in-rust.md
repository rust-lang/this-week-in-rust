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


# Crate of the Week

This week's crate of the week is [derive_builder](https://crates.io/crates/derive_builder), a Rust port of Django's password primitives. Thanks to [Willi Kappler](https://users.rust-lang.org/users/willi_kappler) for the suggestion!

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

153 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-06..2017-02-13

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

Issues in final comment period:

* [`extern` vs `extern "C"`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/52)
* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [types](https://github.com/rust-lang-nursery/fmt-rfcs/issues/15)
* [closures](https://github.com/rust-lang-nursery/fmt-rfcs/issues/35)

Other significant issues:

* [ranges](https://github.com/rust-lang-nursery/fmt-rfcs/issues/60)
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)


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
* [2/24. Crate Polishing Workshop, Darmstadt/Germany](https://www.meetup.com/Rust-Rhein-Main/events/237509289/).
* [3/31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html)

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
