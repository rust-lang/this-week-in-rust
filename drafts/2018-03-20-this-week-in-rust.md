Title: This Week in Rust 226
Number: 226
Date: 2018-03-20
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

* [Making WebAssembly better for Rust & for all languages](https://hacks.mozilla.org/2018/03/making-webassembly-better-for-rust-for-all-languages/).
* [Building a fast Electron app with Rust](https://keminglabs.com/blog/building-a-fast-electron-app-with-rust/).
* [Statuses of unstable standard library features](https://internals.rust-lang.org/t/survey-of-unstable-standard-library-features/7075).
* [Building a DNS server in Rust](https://github.com/EmilHernvall/dnsguide).
* [Type-directed metaprogramming in Rust](http://willcrichton.net/notes/type-directed-metaprogramming-in-rust/).
* [Multicasting in Rust](https://bluejekyll.github.io/blog/rust/2018/03/18/multicasting-in-rust.html).
* Mutating Rust: [Pattern boldness](https://llogiq.github.io/2018/03/13/patterns.html) and [deciding if two types are equal](https://llogiq.github.io/2018/03/15/types.html).
* [kernel-roulette: An example Linux kernel module written in Rust](https://github.com/souvik1997/kernel-roulette).
* [CLI Working Group newsletter 1](https://internals.rust-lang.org/t/cli-wg-newsletter-1/7061).
* [The Embedded Working Group newsletter 1](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-1/7053).
* [This week in Rust docs 97](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-97).

# Crate of the Week

This week's crate is [noisy_float](https://crates.io/crates/noisy_float), a crate with surprisingly useful floating point types that would rather panic than be Not a Number. Thanks to [Ayose Cazorla](https://users.rust-lang.org/u/ayosec) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is more of a writing project than a programming project, but there are a bunch of things that need doing. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

145 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-03-12..2018-03-19

* [coherence diagnostic tweaks](https://github.com/rust-lang/rust/pull/49037)
* [two phase borrows rewrite](https://github.com/rust-lang/rust/pull/48770)
* [minimum viable chalkification](https://github.com/rust-lang/rust/pull/48985) (hooray!)
* [make CodeMap and FileMap thread-safe](https://github.com/rust-lang/rust/pull/48904)
* [introduce canonical queries, use for normalization and dropck-outlives](https://github.com/rust-lang/rust/pull/48411)
* [remove auto trait implementation section when empty](https://github.com/rust-lang/rust/pull/48898)
* [enable embedding LLVM bitcode for iOS](https://github.com/rust-lang/rust/pull/48896)
* [delegate debug implementation for `InternedString`](https://github.com/rust-lang/cargo/pull/5184)
* [check stability of macro invocations](https://github.com/rust-lang/rust/pull/48524)
* [make `assert` a built-in procedural macro](https://github.com/rust-lang/rust/pull/48813)
* [fix hygene issue when deriving Debug](https://github.com/rust-lang/rust/pull/48934)
* [add info message for `-Wall` command](https://github.com/rust-lang/rust/pull/48765) (welcome, C users)
* [move ascii::escape_default to libcore](https://github.com/rust-lang/rust/pull/48735)
* [fallible allocation](https://github.com/rust-lang/rust/pull/48648) (RFC [#2116](https://rust-lang.github.io/rfcs/2116-alloc-me-maybe.html))
* [add intrinsics for portable packed SIMD vector reductions](https://github.com/rust-lang/rust/pull/48983)
* [stabilize inclusive range (`..=`)](https://github.com/rust-lang/rust/pull/47813) (RFC [#1192](https://rust-lang.github.io/rfcs/1192-inclusive-ranges.html))
* [stabilize `!`](https://github.com/rust-lang/rust/pull/47630) (RFC [#1216](https://rust-lang.github.io/rfcs/1216-bang-type.html))
* [remove or hide deprecated unstable `SipHasher`{`13`, `24`}](https://github.com/rust-lang/rust/pull/49108)
* [cargo: faster resolver: clean code and the `backtrack_stack`](https://github.com/rust-lang/cargo/pull/5187),
  also [cache conflicting_activations](https://github.com/rust-lang/cargo/pull/5168)
* [rustbuild: faster submodule updating](https://github.com/rust-lang/rust/pull/49057)

## New Contributors

* Alan Du
* Alexandre Martin
* Alex Butler
* Boris-Chengbiao Zhou
* Dileep Bapat
* dragan.mladjenovic
* Eric Huss
* snf
* Yukio Siraichi

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2349: Standard library API for immovable types](https://github.com/rust-lang/rfcs/pull/2349).
* [RFC 2307: Add std::num::NonZeroU32 and friends, deprecate core::nonzero](https://github.com/rust-lang/rfcs/pull/2307).
* [RFC 2169: Add Euclidean modulo & division functionality for integers](https://github.com/rust-lang/rfcs/pull/2169).
* [RFC 2203: Constants in array repeat expressions](https://github.com/rust-lang/rfcs/pull/2203).
* [RFC 2342: Allow `if` and `match` in constants](https://github.com/rust-lang/rfcs/pull/2342).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [`Self` in type definitions allowing `enum List<T> { Nil, Cons(T, Box<Self>) }`](https://github.com/rust-lang/rfcs/pull/2300).
* [disposition: postpone] [Implement parent items with child traits](https://github.com/rust-lang/rfcs/pull/2303).
* [disposition: close] [Make the `as` keyword consider `Into` Trait implementations](https://github.com/rust-lang/rfcs/pull/2308).
* [disposition: close] [Quick `dbg!(expr)` macro](https://github.com/rust-lang/rfcs/pull/2173).

## New RFCs

* [Custom self types](https://github.com/rust-lang/rfcs/pull/2362).
* [Allow arbitrary enums to have explicit discriminants](https://github.com/rust-lang/rfcs/pull/2363).
* [Formalise reborrows](https://github.com/rust-lang/rfcs/pull/2364).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Mar 22. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 25. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbhc/).
* [Mar 21. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247387953/).
* [Mar 27. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Mar 27. Kitchener, CA - An Introduction To Rust & Writing a Crate (Kahan Sums)](https://www.meetup.com/Rust-KW/events/247661794/).
* [Mar 28. Milano, IT - Let's play with Procedural Macros in Rust](https://www.meetup.com/rust-language-milano/events/248725926/).
* [Mar 28. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Mar 28. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 29. Helsinki, FI - March Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/248805420/).
* [Apr  1. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbcb/).
* [Apr  3. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxgbfb/).
* [Apr  4. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxgbgb/).
* [Apr  4. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxgbgb/).
* [Apr  4. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/cpvshpyxgbgb/).
* [Apr  4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247388074/).
* [Apr  4. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr  5. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Imagine going back in time and telling the reporter “this bug will get fixed 16 years from now, and the code will be written in a systems programming language that doesn’t exist yet”.

— [Nicholas Nethercote](https://blog.mozilla.org/nnethercote/2018/03/09/a-new-preferences-parser-for-firefox/).

Thanks to [jleedev](https://users.rust-lang.org/t/twir-quote-of-the-week/328/501)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
