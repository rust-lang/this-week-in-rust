Title: This Week in Rust 164
Number: 164
Date: 2017-01-10
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

This week's Crate of the Week is [trust](https://github.com/japaric/trust), a Travis CI and AppVeyor template to test your Rust crate on 5 architectures and publish binary releases of it for Linux, macOS and Windows. Thanks to Vikrant for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [Diesel: SQLite Getting started/Skeleton app](https://github.com/diesel-rs/diesel/issues/376).
* [easy] [Diesel: Refactorings using macros in type position](https://github.com/diesel-rs/diesel/issues/521).
* [easy] [Diesel: Deny missing docs](https://github.com/diesel-rs/diesel/issues/563).
* [android-rs-glue: Add more arguments and use clap to parse the arguments](https://github.com/tomaka/android-rs-glue/issues/115).
* [tokei: Add package repositories](https://github.com/Aaronepower/tokei/issues/92).
* [RustCrypto/hashes: Missing hash functions](https://github.com/RustCrypto/hashes/issues/1).
* [RustCrypto/block-ciphers: Missing block ciphers](https://github.com/RustCrypto/block-ciphers/issues/1).
* [Funding Redox OS development](https://www.reddit.com/r/rust/comments/5klu34/funding_redox_os_development/).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

112 pull requests were [merged in the last week][merged]. This contains a good number of plugin-breaking changes.

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-02..2017-01-09

* [`mem::transmute::<U, T>(_)` alignment fixed](https://github.com/rust-lang/rust/pull/38670)
* [`writeln!()`](https://github.com/rust-lang/rust/pull/38469)
* [more specific errors with `str` slices](https://github.com/rust-lang/rust/pull/38066)
* [`Impl From<`{`Ipv4Addr`, `Ipv6Addr`}`> for IpAddr`](https://github.com/rust-lang/rust/pull/38327)
* [placement-in for `Vec`: `Vec.place_back() <- _`](https://github.com/rust-lang/rust/pull/38551)
* [`BinaryHeap::peek_mut().pop()`](https://github.com/rust-lang/rust/pull/38733)
* [missing feature gate for {`std`, `core`}`::`{`i128`, `u128`}](https://github.com/rust-lang/rust/pull/38861)
* [stability check only public items](https://github.com/rust-lang/rust/pull/38689) (fixes ICE)
* [fix stack overflow when promoting MIR terminators](https://github.com/rust-lang/rust/pull/38833)
* [associated types fixed in `Copy` implementation](https://github.com/rust-lang/rust/pull/38152)
* [stabilize Macros 1.1](https://github.com/rust-lang/rust/pull/38783)
* [improved diagnostics for Macros 1.1](https://github.com/rust-lang/rust/pull/38792)
* [fix handling of empty types in patterns](https://github.com/rust-lang/rust/pull/38069)
* [fix regression with doubly exported macro rules](https://github.com/rust-lang/rust/pull/38793)
* [`const_eval` no longer builds fake HIR patterns](https://github.com/rust-lang/rust/pull/38766)
* [negation of unsigned ints no longer typecheck](https://github.com/rust-lang/rust/pull/38776)
* [smaller `std::unicode` tables](https://github.com/rust-lang/rust/pull/38781)
* [debuginfo for unsized struct members fixed](https://github.com/rust-lang/rust/pull/38543)
* [rustc no longer leaks private scope information in errors](https://github.com/rust-lang/rust/pull/38552)
* [separate `ty::Table` per `Body`](https://github.com/rust-lang/rust/pull/38813)
* [don't check global paths for `unused_qualifications`](https://github.com/rust-lang/rust/pull/38817)
* [closures get drop glue even without reference](https://github.com/rust-lang/rust/pull/38822) (went missing in trans)
* [on Windows, distinct processes no longer get overlapped handles](https://github.com/rust-lang/rust/pull/38835)
* [`cargo` now builds all workspace members with `build --all`](https://github.com/rust-lang/cargo/pull/3511)
* [`rustdoc` no longer ICEs with unstable features](https://github.com/rust-lang/rust/pull/38773)
* [dead foreign items are no longer warned against when allowing `dead_code`](https://github.com/rust-lang/rust/pull/38791)

## New Contributors

* Adam Langley
* Christoph Schulz
* Curtis McEnroe
* E. Dunham
* Ian Kerins
* kellerkindt
* Luc Street
* Martijn Vermaat
* Philip Craig

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [crates.io] [What categories should be available on crates.io](https://github.com/rust-lang/crates.io/pull/488)?
* [Roadmap for 2017](https://github.com/rust-lang/rfcs/pull/1774).
* [`core::mem::replace_with` for temporarily moving out of ownership](https://github.com/rust-lang/rfcs/pull/1736).
* [Add a 'thread lifetime, which denotes a thread-bounded region](https://github.com/rust-lang/rfcs/pull/1705).
* [Allow `Self` to appear in the where clause of trait impls](https://github.com/rust-lang/rfcs/pull/1647).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).
* [Add Rvalue-static-promotion](https://github.com/rust-lang/rfcs/pull/1414).

## New RFCs

* [Reject crates.io uploads which declare a feature named `no_std`](https://github.com/rust-lang/rfcs/pull/1841).
* [Generators](https://github.com/rust-lang/rfcs/pull/1832).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

Ready for PR:

There's [a lot of them](https://github.com/rust-lang-nursery/fmt-rfcs/issues?q=is%3Aopen+is%3Aissue+label%3Aready-for-PR) right now, contributions here would be very welcome. If you want advice or help getting started, please ping nrc, or any other member of the style team, in #rust-style.

Issues in final comment period:

* [Conventions for Cargo.toml files (FCP)](https://github.com/rust-lang-nursery/fmt-rfcs/pull/41).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).

# Upcoming Events


* [1/5. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/236137922/).
* [1/5. Rust DC Hack Session—Part 3](https://www.meetup.com/RustDC/events/236141535/).
* [1/9. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/236209293/)
* [1/11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/11 Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/12. Rust Utrecht](https://www.meetup.com/Rust-Utrecht/events/235444678/).
* [1/12. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I would argue that if the Rust project would have just one mission statement, it wouldn't be "create a safe systems programming language". It would be "move towards a world where safe systems programming is the norm".

— [GolDDranks in reply to steveklabnik](https://news.ycombinator.com/item?id=13277096).

Thanks to [matematikaadit](https://users.rust-lang.org/users/matematikaadit) for the [suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/338).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
