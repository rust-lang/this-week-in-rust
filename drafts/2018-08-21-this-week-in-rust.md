Title: This Week in Rust 248
Number: 248
Date: 2018-08-21
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

* [Emacs org-babel support for rust](https://gitlab.com/ajyoon/ob-rust)

# Crate of the Week

This week's crate is [macro_railroad](https://github.com/lukaslueg/macro_railroad), a library to create neat syntax diagrams for `macro_rules!` declarative macros. Thanks to [kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/436) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rustc: mark applicability of diagnostic suggestions](https://github.com/rust-lang/rust/issues/50723).
* [WG-Net: Call For Example Web Projects](https://github.com/rust-lang-nursery/wg-net/issues/44).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

102 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-08-06..2018-08-13

* [whitelist wasm32 simd128 target feature](https://github.com/rust-lang/rust/pull/53179)
* [fix a few regressions from enabling macro modularization](https://github.com/rust-lang/rust/pull/53270)
* [resolve: support custom attributes when macro modularization is enabled](https://github.com/rust-lang/rust/pull/53053)
* [Place unions, pointer casts and pointer derefs behind extra feature gates](https://github.com/rust-lang/rust/pull/51990)
* [suggest float for integer literals where a float was expected](https://github.com/rust-lang/rust/pull/53283)
* [suggest missing comma in macro call](https://github.com/rust-lang/rust/pull/53183)
* [add help message for missing `IndexMut` impl](https://github.com/rust-lang/rust/pull/52788)
* [add errors for unknown, stable and duplicate feature attributes](https://github.com/rust-lang/rust/pull/52644)
* [suggest comma when writing `println!("{}" a);`](https://github.com/rust-lang/rust/pull/52397)
* [emit error for pattern arguments in trait methods](https://github.com/rust-lang/rust/pull/53051)
* [fix improper_ctypes lint for individual foreign items](https://github.com/rust-lang/rust/pull/53100)
* [NLL: use span of the closure args in free region errors](https://github.com/rust-lang/rust/pull/53088)
* [apply some fixes to cross-language LTO (especially when targeting MSVC)](https://github.com/rust-lang/rust/pull/53031)
* [Un-name globals with private linkage](https://github.com/rust-lang/rust/pull/51007)
* [avoid many allocations for `CString`s during codegen](https://github.com/rust-lang/rust/pull/53161)
* [change `assert!` to `debug_assert!` in `visit_with`](https://github.com/rust-lang/rust/pull/53025)
* [don't `collect()` when `size_hint` is useless](https://github.com/rust-lang/rust/pull/53019)
* [make IpvXAddr::new const fns and the well known addresses associated constants](https://github.com/rust-lang/rust/pull/52872)
* [change rustdoc style so fully qualified name does not overlap src link](https://github.com/rust-lang/rust/pull/53060)
* [crates.io: add crate size on the crate detail page](https://github.com/rust-lang/crates.io/pull/1436)

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

* [disposition: merge] [Fix the Error trait](https://github.com/rust-lang/rfcs/pull/2504).
* [disposition: merge] [Add `is_sorted` to the standard library](https://github.com/rust-lang/rfcs/pull/2351).
* [disposition: merge] [Add `pub fn identity<T>(x: T) -> T { x }` to core::convert](https://github.com/rust-lang/rfcs/pull/2306).
* [disposition: merge] [if- and while-let-chains, take 2](https://github.com/rust-lang/rfcs/pull/2497).
* [disposition: merge] [Deprecate uninitialized in favor of a new MaybeUninit type](https://github.com/rust-lang/rfcs/pull/1892).
* [disposition: postpone] [Introduce panic_thin, a fmtless alternative to panic_fmt](https://github.com/rust-lang/rfcs/pull/2305).
* [disposition: close] [Add std::mem::zero](https://github.com/rust-lang/rfcs/pull/2291).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [(Modules) Tracking issue for the `mod.rs` changes](https://github.com/rust-lang/rust/issues/53125).
* [disposition: merge] [Allow to check if sync::Once is already initialized](https://github.com/rust-lang/rust/pull/53027).
* [disposition: merge] [Allow all literals in attributes (Tracking Issue for RFC #1559)](https://github.com/rust-lang/rust/issues/34981).
* [disposition: merge] [Tracking Issue for Iterator::find_map](https://github.com/rust-lang/rust/issues/49602).
* [disposition: close] [Define non-panicking UTF encoding methods on `char`](https://github.com/rust-lang/rust/pull/52580).

## New RFCs

* [Unify std::os::raw::c_void and libc::c_void via libcore](https://github.com/rust-lang/rfcs/pull/2521).
* [Generalized Type Ascription](https://github.com/rust-lang/rfcs/pull/2522).
* [#\[cfg(accessible(..) / version = ".." / nightly)\]](https://github.com/rust-lang/rfcs/pull/2523).

# Upcoming Events

### Online

* [Aug 22. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Aug 28. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Aug 29. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia

* [Aug 18. Chennai, IN - Rust Monthly Meetup](https://www.meetup.com/mad-rs/events/253751178/).

### Europe

* [Aug 16. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxlbvb/).
* [Aug 22. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/253062831/).

### North America

* [Aug 16. Portland, US - Meetup w/ Zcash, ParityTech, Solana | Why Use Rust For Blockchain Development](https://www.meetup.com/Portland-Solana-Blockchain-Meetup/events/253180468/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).**
* [Aug 19. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbzb/).
* [Aug 22. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxlblb/).
* [Aug 26. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbjc/).
* [Aug 27. Durham, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxlbkc/).
* [Aug 28. Chicago, US - Rust Meetup](https://www.meetup.com/Chicago-Rust-Meetup/events/253621611/).
* [Aug 28. Dallas, US - Rust Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxlblc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust/Core Developer at Parity Technologies, Berlin](https://paritytech.io/jobs/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Fearless concurrency includes fearless refactoring.

– [cuviper at rust-users](https://users.rust-lang.org/t/parallel-problems-to-showcase-rust-features/19365/6).

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/u/juleskers) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*
