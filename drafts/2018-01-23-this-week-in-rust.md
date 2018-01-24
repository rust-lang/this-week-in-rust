Title: This Week in Rust 218
Number: 218
Date: 2018-01-23
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

* [Welcome kodraus and withoutboats as full Libs Team members](https://internals.rust-lang.org/t/welcome-kodraus-and-withoutboats-as-full-libs-team-members/6582)!
* [Rust by Example is now part of official Rust docs (nightly)](https://doc.rust-lang.org/nightly/rust-by-example/).
* [Redox crash challenge](https://github.com/redox-os/redox/issues/1136).
* [Retooling the Rust Libs Team team for 2018](https://aturon.github.io/blog/2018/01/16/libs-mission/).
* [Introduction to lyon: 2D vector graphics rendering on the GPU in Rust](https://nical.github.io/posts/lyon-intro.html).
* [Brave new I/O: A new approach to I/O in embedded contexts](http://blog.japaric.io/brave-new-io/).
* [Oxidizing Source Maps with Rust and WebAssembly](https://hacks.mozilla.org/2018/01/oxidizing-source-maps-with-rust-and-webassembly/).
* [Why is Rust difficult](https://vorner.github.io/difficult.html)?
* [Using Capabilities to design safer, more expressive APIs](http://zsck.co/writing/capability-based-apis.html).
* [State machines in a Rust game](https://dev.to/mindflavor/lets-build-zork-using-rust-1opm).
* [Ref patterns, destructuring, and invisible borrows](https://medium.com/@robertgrosse/ref-patterns-destructuring-and-invisible-borrows-2f8ae6902656).
* [This week in Rust docs 89](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-89).
* [podcast] [The Changelog: Building a secure operating system with Rust](https://changelog.com/podcast/280).

## #Rust2018

Find all #Rust2018 posts at [Read Rust](http://readrust.net/rust2018/).

# Crate of the Week

This week's crate is [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat), a cargo subcommand to find out how much space crates/functions take up in an executable. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion!

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

158 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-08..2018-01-15

* [add a "-Z offline" flag to Cargo, altering it's dependency resolution behavior](https://github.com/rust-lang/cargo/pull/4770)
* [`auto trait Foo { .. }`](https://github.com/rust-lang/rust/pull/47416)
* [fix std breakage with custom libdir](https://github.com/rust-lang/rust/pull/46592)
* [syntax: rewrite parsing of impls](https://github.com/rust-lang/rust/pull/46455)
* [add error code for unstable feature errors](https://github.com/rust-lang/rust/pull/47413)
* [type error method suggestions use whitelisted identity-like conversions](https://github.com/rust-lang/rust/pull/46461)
* [avoid panicking when invalid argument is passed to `cfg(..)`](https://github.com/rust-lang/rust/pull/47372)
* [NLL: bad error message when converting anonymous lifetime to `'static`](https://github.com/rust-lang/rust/pull/47329)
* [NLL: resolve type and region variables in "NLL dropck"](https://github.com/rust-lang/rust/pull/47322)
* [treat `#[path]` files as mod.rs files](https://github.com/rust-lang/rust/pull/47298)
* [shorten names of some compiler generated artifacts](https://github.com/rust-lang/rust/pull/47269)
* [account for `pub` in `const` → `static` suggestion](https://github.com/rust-lang/rust/pull/47262)
* [remove deprecated unstable attribute `#[simd]`](https://github.com/rust-lang/rust/pull/47251)
* [rustc: tweak `#[target_feature]` syntax](https://github.com/rust-lang/rust/pull/47223)
* [macros: improve 1.0/2.0 interaction](https://github.com/rust-lang/rust/pull/46551)
* [make double ended searchers use dependent fingers](https://github.com/rust-lang/rust/pull/47208)
* [add iterator method specialisations to `Range*`](https://github.com/rust-lang/rust/pull/47180)
* [fix built-in indexing not being used where index type wasn't "obviously" `usize`](https://github.com/rust-lang/rust/pull/47167)
* [add `slice::`{`ExactChunks`, `ExactChunksMut`} iterators](https://github.com/rust-lang/rust/pull/47126)
* [better Debug impl for io::Error](https://github.com/rust-lang/rust/pull/47120)
* [fix nested imports not included in the save_analysis output](https://github.com/rust-lang/rust/pull/47081)
* [implement AsRef<Path> for Component](https://github.com/rust-lang/rust/pull/46985)
* [`BufRead`: only flush the internal buffer if seeking outside of it](https://github.com/rust-lang/rust/pull/46832)
* [pre-allocate in `fs::`{`read`, `read_string`}](https://github.com/rust-lang/rust/pull/47324)
* [implement `Write` for `Cursor<&mut Vec<T>>`](https://github.com/rust-lang/rust/pull/46830)
* [fix off-by-one error in `BufWriter`](https://github.com/rust-lang/rust/pull/47330)
* [deprecate `[T]::rotate` in favor of `[T]::rotate_`{`left`, `right`}](https://github.com/rust-lang/rust/pull/46777)
* [add `HashMap::remove_entry`](https://github.com/rust-lang/rust/pull/47259)

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
