Title: This Week in Rust 263
Number: 263
Date: 2018-12-04
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

This week's crate is [cargo-call-stack](https://github.com/japaric/cargo-call-stack), a cargo subcommand for whole-program call stack analysis. Thanks to [Jorge Aparicio](https://mobile.twitter.com/japaricious/status/1069569802241486850) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust Latam CFP is now open, deadline is December 31st](https://cfp.rustlatam.org/events/rust-latam).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

254 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-11-26..2018-12-03

* [decouple proc_macro from the rest of the compiler](https://github.com/rust-lang/rust/pull/49219)
* [implement chalk unification routines](https://github.com/rust-lang/rust/pull/56214)
* [upgrade LLVM to trunk, still version 8](https://github.com/rust-lang/rust/pull/55835)
* [another LLVM Update](https://github.com/rust-lang/rust/pull/56313) and [Re-enable lldb](https://github.com/rust-lang/rust/pull/56298)
* [use sort_by_cached_key when the key function is not trivial/free](https://github.com/rust-lang/rust/pull/55821)
* [deduplicate literal → constant lowering](https://github.com/rust-lang/rust/pull/56312)
* [use `MaybeUninit` instead of `mem::uninitialized` for Windows Mutex](https://github.com/rust-lang/rust/pull/56275)
* [libcore: add VaList and variadic arg handling intrinsics](https://github.com/rust-lang/rust/pull/49878)
* [arena: speed up TypedArena::clear and improve common patterns](https://github.com/rust-lang/rust/pull/56378)
* [stabilize `macro_at_most_once_rep`](https://github.com/rust-lang/rust/pull/56245)
* [stabilize `dbg!(..)`](https://github.com/rust-lang/rust/pull/56395)
* [stabilize `self_in_typedefs`](https://github.com/rust-lang/rust/pull/56366)
* [stabilize `self_struct_ctor`](https://github.com/rust-lang/rust/pull/56365)
* [remove unsafe `unsafe` inner function](https://github.com/rust-lang/rust/pull/56236)
* [add `TryFrom<&[T]> for [T; $N] where T: Copy`](https://github.com/rust-lang/rust/pull/56216)
* [move VecDeque::resize_with out of the impl<T:Clone> block](https://github.com/rust-lang/rust/pull/56401)
* [use allow-dirty option in `cargo package` to skip vcs checks](https://github.com/rust-lang/cargo/pull/6280)
* [make `ParseIntError` and `IntErrorKind` fully public](https://github.com/rust-lang/rust/pull/55705)
* [use MaybeUninit in libcore](https://github.com/rust-lang/rust/pull/54668)
* [fix futures creating aliasing mutable and shared ref](https://github.com/rust-lang/rust/pull/56319)
* [add libstd Cargo feature `panic_immediate_abort`](https://github.com/rust-lang/rust/pull/55011)
* [cargo: ConflictStoreTrie: faster filtered search](https://github.com/rust-lang/cargo/pull/6366)
* [crates.io: email verification warning](https://github.com/rust-lang/crates.io/pull/1565)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Stabilise exhaustive integer pattern matching](https://github.com/rust-lang/rfcs/pull/2591).
* [disposition: merge] [Needle API (née Pattern API)](https://github.com/rust-lang/rfcs/pull/2500).
* [disposition: close] [Add RFC for officially adopting Ferris](https://github.com/rust-lang/rfcs/pull/2328).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [[Stabilization] Stablize using some arbitrary self types defined in std](https://github.com/rust-lang/rust/issues/55786).
* [disposition: merge] [Wxperiment: Support aliasing local crate root in extern prelude](https://github.com/rust-lang/rust/pull/55275).
* [disposition: merge] [Refiling "#[repr(simd)] struct(isize, isize) not allowed"](https://github.com/rust-lang/rust/issues/55078).
* [disposition: merge] [Tracking issue for inclusion of `derive` in lint `unused_attributes`](https://github.com/rust-lang/rust/issues/54651).
* [disposition: merge] [Tracking issue for RFC 2361, "Simpler alternative dbg!() macro"](https://github.com/rust-lang/rust/issues/54306).
* [disposition: merge] [Tracking issue for RFC 2302, Tuple struct construction with `Self(v1, v2, ..)`](https://github.com/rust-lang/rust/issues/51994).
* [disposition: merge] [tracking issue for `?` macro repetition](https://github.com/rust-lang/rust/issues/48075).

## New RFCs

* [Symbol Mangling v2](https://github.com/rust-lang/rfcs/pull/2603).
* [#[attribute]s galore](https://github.com/rust-lang/rfcs/pull/2602).
* [Multiple Attributes in an Attribute Container](https://github.com/rust-lang/rfcs/pull/2600).

# Upcoming Events

### Online

* [Dec  3. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Dec  5. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Dec 12. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Dec  5. Johannesburg, SA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/jdqplqyxqbhb/).

### Asia

* [Dec  6. Pune, IN - Rust workshop at Pune, India](https://reps.mozilla.org/e/rust-community-meetup-pune/).
* [Dec 12. Hangzhou, CN - Rust Hangzhou](https://www.meetup.com/Rust-Hangzhou/events/256338781/).

### Europe

* [Nov 29. Copenhagen, DK - Copenhagen Rust Group - Hack Night #11](http://cph.rs/).
* [Dec  3. Karlsruhe, DE - Rust 2018 Edition](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/256200841/?_xtd=gqFyqTE5MzgwNjQ5OKFwp2FuZHJvaWQ&from=ref).
* [Dec  5. Cologne, DE - Cologne meetup](https://rust.cologne/2018/12/05/rust-2018-eve.html).
* [Dec  6. Gouda, NL - Rust Gouda - Rust 2018 edition release party](https://www.meetup.com/Rust-Gouda/events/254877742/).
* [Dec 10. Vienna, AT - Metalab - Rust Workshop](https://metalab.at/wiki/Rust-Workshop).
* [Dec 11. Zurich, CH - Rust Zurich - Rust Embedded Edition 2018](https://www.meetup.com/Rust-Zurich/events/255279763/).
* [Dec 12. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxqbqb/).
* [Dec 15 & 16. Moscow, RU - RustRush 2018](https://rustrush.ru).

### North America

* [Dec  2. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbdb/).
* [Dec  5. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyxqbhb/).
* [Dec  5. Indianopolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxqbhb/).
* [Dec  6. Phoenix, US - Phoenix 2018 Edition Release Party](https://www.meetup.com/Desert-Rustaceans/events/256503618).
* [Dec  9. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbmb/).
* [Dec 10. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxqbnb/).
* [Dec 12. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyxqbqb/).
* [Dec 13. Arlington, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/256181658).
* [Dec 13. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxqbrb/).
* [Dec 13. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209738/).
* [Dec 13. San Diego, US - San Diego Rust December Meetup - Rust 2018 Overview + Memory Allocator](https://www.meetup.com/San-Diego-Rust/events/256264465/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Layer1 Capital, San Francisco, US](https://angel.co/layer1-capital/jobs/459718-rust-engineer).
* [Embedded operating system developer, Karlsruhe, DE](https://www.pse.kit.edu/karriere/joboffer.php?id=2093&language=en)
* [Student research assistant (embedded), Karlsruhe, DE](https://twitter.com/oli_obk/status/1064856324071178240)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The bug I did not have

– /u/pacman82's [reddit post](https://www.reddit.com/r/rust/comments/a1w75c/the_bug_i_did_not_have/) title

Thanks to [Felix](https://users.rust-lang.org/t/twir-quote-of-the-week/328/582) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
