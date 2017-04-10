Title: This Week in Rust 177
Number: 177
Date: 2017-04-11
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

This week's Crate of this Week is [rust-skeptic](https://github.com/brson/rust-skeptic), a cargo subcommand to doctest your README.md. Thanks to [staticassert](https://users.rust-lang.org/users/staticassert) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Want to join the Rust docs team](http://words.steveklabnik.com/want-to-join-the-rust-docs-team)?
* [Rust reference docs: Document all features](https://github.com/rust-lang-nursery/reference/issues/9).
* [Rusoto an AWS SDK for Rust is looking for maintainers](https://github.com/rusoto/rusoto).
* [liner: Make keyboard interrupts (e.g. SIGINT from Ctrl-c) work](https://github.com/MovingtoMars/liner/issues/4). Liner is a readline-like library in Rust.
* [liner: Tilde expansion](https://github.com/MovingtoMars/liner/issues/34).
* [liner: Password mode](https://github.com/MovingtoMars/liner/issues/25).
* [liner: Use right arrow key to select autocompletion](https://github.com/MovingtoMars/liner/issues/37).
* [Ion: Optional Descriptions for Functions](https://github.com/redox-os/ion/issues/232). Ion is a shell for UNIX platforms, and is the default shell in Redox.
* [Ion: Implement Mapfiles](https://github.com/redox-os/ion/issues/247).
* [parenchyma: CUDA maintainer](https://github.com/lychee-eng/parenchyma/issues/22). Parenchyma is an extensible HPC-Framework for CUDA, OpenCL and native CPU.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

132 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-04-03..2016-04-10

* [ABI layout computation is no longer tied to LLVM](https://github.com/rust-lang/rust/pull/40658) (yay!)
* [new `#[used]` attribute](https://github.com/rust-lang/rust/pull/39987)
* [the "visible parent map" is now immutable](https://github.com/rust-lang/rust/pull/41061) (potentially, but unlikely plugin-breaking)
* [undefined types (e.g. due to parsing errors) are now `TyError` instead of `TyInfer`](https://github.com/rust-lang/rust/pull/40887) (potentially plugin-breaking)
* [avoid dropflags creation for empty drops](https://github.com/rust-lang/rust/pull/41148)
* [the `overlapping_inherent_impls` lint is now a hard error](https://github.com/rust-lang/rust/pull/41052)
* [fix macros including `#[derive]`s](https://github.com/rust-lang/rust/pull/41050)
* [on Linux, use `poll instead of `select`](https://github.com/rust-lang/rust/pull/41039) (for more than 1K file descriptors)
* [improve `iterator::Rev::`{`find`, `rfind`} plumbing](https://github.com/rust-lang/rust/pull/41028) ([also on `slice::Iter`/`IterMut`](https://github.com/rust-lang/rust/pull/41154)
* [simplify `HashMap::Bucket` for awesome speedups](https://github.com/rust-lang/rust/pull/40561)
* [optimize `AtomicBool::fetch_nand(..)`](https://github.com/rust-lang/rust/pull/41143)
* [`RawFd` no longer implements `AsRawFd`/`IntoRawFd`](https://github.com/rust-lang/rust/pull/41035)
* [`Vec::place_back()` no longer requires `T: Clone`](https://github.com/rust-lang/rust/pull/40909)
* [new `[T]::`{`rsplit`, `rsplit_mut` methods}`(..)`](https://github.com/rust-lang/rust/pull/41065)
* [add safe wrapper for `atomic_compilerfence` intrinsics](https://github.com/rust-lang/rust/pull/41092)
* [on-demandify reachability](https://github.com/rust-lang/rust/pull/40873)
* [point out private fields inadvertently called as methods](https://github.com/rust-lang/rust/pull/41062)
* [better error message on missing item category](https://github.com/rust-lang/rust/pull/40815)
* [Suggest enum when variant is erroneously used as type](https://github.com/rust-lang/rust/pull/40775)
* [don't try to blame tuple fields for immutability](https://github.com/rust-lang/rust/pull/41108)
* [always show the end of multiline annotations](https://github.com/rust-lang/rust/pull/41136)
* [show last valid token on syntax errors](https://github.com/rust-lang/rust/pull/40811)
* [`save-analysis` tracks associated types](https://github.com/rust-lang/rust/pull/40915)
* [allow multiple output types again](https://github.com/rust-lang/rust/pull/41085) (regressed after 1.14)
* [rustdoc now uses pulldown-cmark instead of hoedown](https://github.com/rust-lang/rust/pull/41112) (also [assorted issues fixed](https://github.com/rust-lang/rust/pull/41111))
* [crates.io now shows links directly under crate name header](https://github.com/rust-lang/crates.io/pull/668)

## New Contributors

* Alan Stoate
* aStoate
* Donnie Bishop
* GAJaloyan
* Jörg Thalheim
* Malo Jaffré
* Micah Tigley
* Nick Sweeting
* Phil Ellison
* raph

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

* [disposition: postpone] [Polymorphic Numeric Constants](https://github.com/rust-lang/rfcs/pull/1945).
* [disposition: postpone] [Introduce `with` bounds for pi types](https://github.com/rust-lang/rfcs/pull/1932).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).

## New RFCs

* [Copy most of the static `ptr::` functions to methods on unsafe pointers themselves](https://github.com/rust-lang/rfcs/pull/1966).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24)
* [Closures](https://github.com/rust-lang-nursery/fmt-rfcs/issues/35)
* [Where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)


# Upcoming Events

* [Apr  5. Rust User Group Cologne - Crate Polishing](http://rust.cologne/2017/04/05/crate-polishing.html).
* [Apr  5. Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/238104881/).
* [Apr  5. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Apr  5. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Apr  5. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238613284/).
* [Apr  6. Rust DC Learn + Try: tokio](https://www.meetup.com/RustDC/events/238221152/).
* [Apr  6. Rust Detroit - Letting the type system catch errors for you](https://www.meetup.com/rust-detroit/events/238662757).
* [Apr  6. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr 10. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/238404173/).
* [Apr 11. Toronto Rust Meetup](https://www.meetup.com/Rust-Toronto/events/238780453/).
* [Apr 12. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Apr 12. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Apr 13. Rust Melbourne - Why your first FizzBuzz Rust implementation may not work](https://www.meetup.com/Rust-Melbourne/events/238108356/).
* [Apr 13. San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/238305909/).
* [Apr 13. Rust Meetup Hamburg - Hack & Learn Tokio Edition](https://www.meetup.com/Rust-Meetup-Hamburg/events/237984043/).
* [Apr 13. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/238502945/).
* [Apr 18. Mozilla Meetup Switzerland - Iron - Web development with Rust](https://www.meetup.com/en-US/Mozilla-Meetup-Switzerland/events/237870710/).
* [Apr 19. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Apr 19. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Apr 20. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr 20. Rust Utrecht - Use Rust: Mentored Workshop](https://www.meetup.com/Rust-Utrecht/events/238725437/).
* **[Apr 30. RustFest 2017 - Kyiv, Ukraine](http://2017.rustfest.eu/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I gave my company's Embedded C training course this morning.
> It's amazing how much more sense C makes when you explain it in Rust terms.

— [theJPster in #rust-embedded](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-embedded).

Thanks to [Oliver Schneider for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/379).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
