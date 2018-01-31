Title: This Week in Rust 219
Number: 219
Date: 2018-01-30
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

## #Rust2018

Find all #Rust2018 posts at [Read Rust](http://readrust.net/rust2018/).

# Crate of the Week

This week's crate is [rust-semverver](https://github.com/rust-lang-nursery/rust-semverver), an as-of-yet buggy, but already useful semantic versioning (semver) checking tool. Thanks to [Philipp Hansch](https://users.rust-lang.org/u/phansch) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [Help Rayon prepare for 1.0](https://users.rust-lang.org/t/rayon-1-0-on-feb-14/14950).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

135 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-22..2018-01-29

* [stabilized `#[repr(align(x))]` attribute](https://github.com/rust-lang/rust/pull/47006) (RFC [#1358](https://rust-lang.github.io/rfcs/1358-repr-align))
* [implement intra-rustdoc links](https://github.com/rust-lang/rust/pull/47046) (RFC [#1946](https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links))
* [fix type inhabitedness check for arrays](https://github.com/rust-lang/rust/pull/47600)
* [fix ICE on const eval of union field](https://github.com/rust-lang/rust/pull/47794)
* [fix ICE when return type includes unconstrained anon region](https://github.com/rust-lang/rust/pull/47668)
* [fix ICE when use trees have multiple empty nested groups](https://github.com/rust-lang/rust/pull/47705)
* [fix ICE with `use self;`](https://github.com/rust-lang/rust/pull/47633)
* [fix never-type rvalue ICE](https://github.com/rust-lang/rust/pull/47746)
* [track recursion limit when expanding existential impl trait](https://github.com/rust-lang/rust/pull/47529)
* [rustc: SIMD types use pointers in Rust's ABI](https://github.com/rust-lang/rust/pull/47743)
* [add CGU size heuristic for partitioning](https://github.com/rust-lang/rust/pull/47415)
* [first round of LLVM 6.0.0 compatibility](https://github.com/rust-lang/rust/pull/47710)
* [let LLVM 5 add DW_OP_deref to indirect args itself](https://github.com/rust-lang/rust/pull/47688)
* [LLVM5: update DW_OP_plus to DW_OP_plus_uconst](https://github.com/rust-lang/rust/pull/47610)
* [rustc: load the `rustc_trans` crate at runtime](https://github.com/rust-lang/rust/pull/47671)
* [rustc: add `-C lto=`{`thin`, `fat`} option](https://github.com/rust-lang/rust/pull/47521)
* [properly pass down immutability info for thread-locals](https://github.com/rust-lang/rust/pull/47425)
* [simplify irrefutable slice patterns](https://github.com/rust-lang/rust/pull/47374)
* [make use of the implemented red/green algorithm for variance](https://github.com/rust-lang/rust/pull/47696)
* [do not capture stderr in the compiler. Instead just panic silently for fatal errors](https://github.com/rust-lang/rust/pull/47634)
* [fix spans in unused import lint for nested groups](https://github.com/rust-lang/rust/pull/47726)
* [add `-Z teach` flag to provide extended diagnostic help](https://github.com/rust-lang/rust/pull/47652)
* [on missing method do not suggest private traits](https://github.com/rust-lang/rust/pull/47534)
* [immovable generators](https://github.com/rust-lang/rust/pull/45337)
* [NLL test for mutating &mut references](https://github.com/rust-lang/rust/pull/47609)
* [make the constructors of Duration const fns](https://github.com/rust-lang/rust/pull/47300)
* [make core::ops::Place an unsafe trait](https://github.com/rust-lang/rust/pull/47299)
* [make UnsafeCell::into_inner safe](https://github.com/rust-lang/rust/pull/47204)
* [expose float `from_bits` and `to_bits` in libcore](https://github.com/rust-lang/rust/pull/46931)
* [use the slice length to hint the optimizer about iter.position result](https://github.com/rust-lang/rust/pull/47772)
* [add rustc-args option to test runner](https://github.com/rust-lang/rust/pull/47558)
* [rustdoc: show when traits are auto traits](https://github.com/rust-lang/rust/pull/47672)

## New Contributors

* Adam C. Foltzer
* andjo403
* Dominik Winecki
* Gauri
* Marcel Hellwig
* Mark Mansi
* Matthew Walinga
* Petr Sumbera
* Pieter Penninckx
* Pulkit Goyal

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

* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [`Self` in type definitions allowing `enum List<T> { Nil, Cons(T, Box<Self>) }`](https://github.com/rust-lang/rfcs/pull/2300).
* [`?` repetition in macro rules](https://github.com/rust-lang/rfcs/pull/2298).
* [Tuple struct construction with `Self(v1, v2, ..)`](https://github.com/rust-lang/rfcs/pull/2302).
* [Add `Option::replace` to the core library](https://github.com/rust-lang/rfcs/pull/2296).
* [Add `pub fn identity<T>(x: T) -> T { x }` to core::convert](https://github.com/rust-lang/rfcs/pull/2306).
* [Introduce panic_thin, a fmtless alternative to panic_fmt](https://github.com/rust-lang/rfcs/pull/2305).
* [Implement parent items with child traits](https://github.com/rust-lang/rfcs/pull/2303).
* [Add std::num::NonZeroU32 and friends, deprecate core::nonzero](https://github.com/rust-lang/rfcs/pull/2307).
* [Inherent traits](https://github.com/rust-lang/rfcs/pull/2309).
* [Extend pattern API to OsStr](https://github.com/rust-lang/rfcs/pull/2295).
* [Make the `as` keyword consider `Into` Trait implementations](https://github.com/rust-lang/rfcs/pull/2308).

# Upcoming Events

* [Jan 25. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jan 29. Rust London User Group - LDN Talks: January 2018](https://www.meetup.com/Rust-London-User-Group/events/246637221/).
* [Jan 30. Rust Zurich - Embedded Rust](https://www.meetup.com/Rust-Zurich/events/246675630/).
* [Jan 30. Mexico City - Rust MX - Rust Meetup #9](https://www.meetup.com/Rust-MX/events/246913439/).
* [Jan 31. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jan 31. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb  2 - Feb 4. Multiple locations - Rust Roadshow Brazil 2018](https://mozillabr.org/2017/12/anunciando-o-rust-roadshow-brasil-2018-para-mobilizadores-de-todo-o-brasil/).
* [Feb  6. Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/).
* [Feb  7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Feb  7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Feb  7. Indianapolis - Indy.rs - February 2018](https://www.meetup.com/indyrs/events/246726699/).
* [Feb  7. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxdbkb/).
* [Feb  8. Helsinki - Finland Rust-lang Group](https://www.meetup.com/Finland-Rust-Meetup/events/246866694/).
* [Feb  8. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxdblb/).
* [Feb  8. San Diego Rust February Meetup](https://www.meetup.com/San-Diego-Rust/events/246906809/).
* [Feb  8. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Feb  9. Rust Table of Regulars Darmstadt / Germany](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/246744631)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

# Quote of the Week

> Rust is difficult because most programmers abuse shared mutable state and Rust makes you sacrifice your first-born to be able to to do it.

— [u/_errata_ on reddit](https://www.reddit.com/r/rust/comments/7rza1q/why_is_rust_difficult/dt11dqx/).

Thanks to [u/kixunil for the suggestion](https://www.reddit.com/r/rust/comments/7rza1q/why_is_rust_difficult/dt22fol/)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
