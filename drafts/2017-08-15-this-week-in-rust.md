Title: This Week in Rust 195
Number: 195
Date: 2017-08-15
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

This week's crate is [exa](https://the.exa.website), a modern `ls` replacement (with a `tree` thrown in as well) written in Rust. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [PumpkinDB: Rust nightly after 2017-06-20 affects benchmarks negatively](https://github.com/PumpkinDB/PumpkinDB/issues/345). ([Discuss here](https://gitter.im/PumpkinDB/Lobby)).
* [easy] [gimli: Improve error handling in dwarfdump](https://github.com/gimli-rs/gimli/issues/231). gimli is a lazy, zero-copy parser for the DWARF debugging format.
* [wayland-window: Add control buttons](https://github.com/Smithay/wayland-window/issues/4).
* [wayland-window: Make borders prettier](https://github.com/Smithay/wayland-window/issues/19).
* [doc] [lyon: API guidelines: methods on collections that produce iterators follow the iter, iter_mut, into_iter conventions](https://github.com/nical/lyon/issues/86). Lyon is a GPU-based 2D graphics rendering engine in Rust.
* [doc] [lyon: API guidelines: ad-hoc conversions follow as_, to_, into_ conventions](https://github.com/nical/lyon/issues/85).
* [doc] [lyon: API guidelines: iterator type names should match the methods that produce them](https://github.com/nical/lyon/issues/87).
* [doc] [lyon: API guidelines: return error type from functions instead of empty struct](https://github.com/nical/lyon/issues/88).
* [medium] [lyon: Implement clipping line joins at the miter limit](https://github.com/nical/lyon/issues/35).
* [less easy] [bindgen: Emitting or deriving trait implementations](https://github.com/rust-lang-nursery/rust-bindgen/issues/886).
* [less easy] [bindgen: Emit a "manual" implementation of Debug when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/875).
* [less easy] [bindgen: "manually" implement Hash when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/877).
* [less easy] [bindgen: "manually" implement PartialEq when it cannot be derived](https://github.com/rust-lang-nursery/rust-bindgen/issues/879).
* [less easy] [bindgen: Derive `Eq` when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/880).
* [less easy] [bindgen: "manually" implement Eq when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/881).
* [less easy] [bindgen: Derive PartialOrd when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/882).
* [less easy] [bindgen: "manually" implement PartialOrd when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/883).
* [less easy] [bindgen: Derive Ord when possible](https://github.com/rust-lang-nursery/rust-bindgen/issues/884).
* [less easy] [bindgen: "manually" implement Ord when we cannot derive it](https://github.com/rust-lang-nursery/rust-bindgen/issues/885).
* [doc] [rust-ffi-guide: Make the book more consistent](https://github.com/Michael-F-Bryan/rust-ffi-guide/issues/8).
* [easy] [ggez: Input doesn't work on mac using tmux and iterm2](https://github.com/ggez/ggez/issues/30). ggez is a Rust library to create good games easily.
* [easy] [ggez: SDL controller input doesn't work](https://github.com/ggez/ggez/issues/35).
* [doc] [ggez: Finish full building-and-install docs for each platform](https://github.com/ggez/ggez/issues/118).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

128 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-08-07..2017-08-14

* [cleanup in preparation of no-LLVM build support](https://github.com/rust-lang/rust/pull/43842)
* [`#[must_use]` for functions](https://github.com/rust-lang/rust/pull/43728)
* [fix unused result lint triggering on functions returning `()`, `!` or empty enums](https://github.com/rust-lang/rust/pull/43813)
* [rustc can now be built without jemalloc](https://github.com/rust-lang/rust/pull/43589)
* [fixed the needless mut lint, found libcore bugs](https://github.com/rust-lang/rust/pull/43582)
* [fixed `#[thread_local]` statics check](https://github.com/rust-lang/rust/pull/43746)
* [fix `-Z hir-stats`](https://github.com/rust-lang/rust/pull/43824)
* [fix region hashing](https://github.com/rust-lang/rust/pull/43743)
* [nonlexical lifetimes region renumberer](https://github.com/rust-lang/rust/pull/43559) (one step closer to nonlexical lifetimes)
* [rearchitect lints to be emitted more eagerly](https://github.com/rust-lang/rust/pull/43522) (broke clippy)
* [`mem::unreachable`](https://github.com/rust-lang/rust/pull/43750) (the intrinsic, not the panic)
* [make `for_all_relevant_impls` O(1) again](https://github.com/rust-lang/rust/pull/43723)
* [add an overflow check to range's `Iter::next()` method](https://github.com/rust-lang/rust/pull/43595) (which turns out to make things faster)
* [optimize allocation paths in `RawVec`](https://github.com/rust-lang/rust/pull/43815)
* [improve error messages on duplicate type/method names](https://github.com/rust-lang/rust/pull/43737)
* [better labeling of mismatched return type](https://github.com/rust-lang/rust/pull/43484)
* [syntax hint for `extern C { .. }` errors](https://github.com/rust-lang/rust/pull/43720)
* [Validation now works correctly on blocks with multiple incoming edges](https://github.com/rust-lang/rust/pull/43748)
* [`break rust`](https://github.com/rust-lang/rust/pull/43745)
* [the case of the missing error codes](https://github.com/rust-lang/rust/pull/43709)

## New Contributors

* Eric Daniels
* Mario Idival
* Ryan Leckey
* scalexm
* Tobias Schaffner
* Tymoteusz Jankowski

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

* [disposition: merge] [Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [disposition: merge] [Unsafe pointer methods](https://github.com/rust-lang/rfcs/pull/1966).
* [disposition: merge] [Amend RFC 1242 to require an RFC for deprecation of crates from the nursery](https://github.com/rust-lang/rfcs/pull/1983).
* [disposition: close] [Match branch semicolon](https://github.com/rust-lang/rfcs/pull/1994).
* [disposition: merge] [Future-proofing enums/structs with `#[non_exhaustive]` attribute](https://github.com/rust-lang/rfcs/pull/2008).
* [disposition: merge] [Enable nested method calls](https://github.com/rust-lang/rfcs/pull/2025).
* [disposition: close] [Zero-Sized References](https://github.com/rust-lang/rfcs/pull/2040).
* [disposition: merge] [Evolving Rust through checkpoints](https://github.com/rust-lang/rfcs/pull/2052).
* [disposition: close] [Allow use of pipe operator in patterns](https://github.com/rust-lang/rfcs/pull/1882).
* [disposition: merge] [Generic associated types (associated type constructors)](https://github.com/rust-lang/rfcs/pull/1598).

## New RFCs

* [Non-lexical lifetimes](https://github.com/rust-lang/rfcs/pull/2094).
* [Infer `T: 'x` outlives requirements on structs](https://github.com/rust-lang/rfcs/pull/2093).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

The RFC style is now the default style in Rustfmt - try it out and let us know what you think!

Currently being discussed:

* [Define short](https://github.com/rust-lang-nursery/fmt-rfcs/issues/47)
* [Special casing some macros](https://github.com/rust-lang-nursery/fmt-rfcs/issues/86)

# Upcoming Events

* [Aug 10. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Aug 10. Columbus Rust Society - Monthly meetup](https://www.meetup.com/columbus-rs/events/czcwhlywlbnb/).
* [Aug 14. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/241535500/).
* [Aug 16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug 16. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* **[Aug 18-19. RustConf 2017](http://rustconf.com/)**.
* [Aug 23. GOTO Night Berlin: Modern low level - Rust in 2017](https://www.meetup.com/GOTO-Nights-Berlin/events/241544422/).
* [Aug 23. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/242277432/).
* [Aug 23. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Aug 23. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Aug 24. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Nah, it's not you, it's the borrow checker.
>> Honey, it's not you, it's &mut me.
>>> You can borrow me, and you can change me, but you can't own me.

— [/u/staticassert, /u/ybx, and /u/paholg on reddit](https://www.reddit.com/r/rust/comments/6s8vhg/how_do_i_do_if_let_somex_selfbla_selfdobla/dlazidp/).

Thanks to [Matt Ickstadt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/428) and [QuadDamaged](https://users.rust-lang.org/t/twir-quote-of-the-week/328/429) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
