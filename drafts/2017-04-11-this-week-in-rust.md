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

[Launching a URL Shortener in Rust using Rocket](https://matthias-endler.de/2017/rust-url-shortener) - 

# Crate of the Week

This week's Crate of this Week is [fst](https://github.com/BurntSushi/fst), which contains Finite State Transducers and assorted algorithms that use them (e.g. fuzzy text search). Thanks to [Jules Kerssemakers](https://users.rust-lang.org/users/juleskers) for the suggestion!

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

114 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-27..2016-04-03

* [diverging types now check more correctly](https://github.com/rust-lang/rust/pull/40224)
* [`save-analysis` tracks associated types](https://github.com/rust-lang/rust/pull/40915)
* [`save-analysis` allows clients to get data directly](https://github.com/rust-lang/rust/pull/40751) (without writing a file)
* on-demand-ify [`custom_coerce_unsized_kind` and `inherent-impls`](https://github.com/rust-lang/rust/pull/40683) and the
  [typechecking of item bodies](https://github.com/rust-lang/rust/pull/40540)
* [more accurate macro expansion information](https://github.com/rust-lang/rust/pull/40597)
* [`borrowck` `&mut` suggestion refactoring](https://github.com/rust-lang/rust/pull/40841)
* [fix ICE on some nested macro definitions](https://github.com/rust-lang/rust/pull/40813)
* [fix build on MSP430 (16 bit)](https://github.com/rust-lang/rust/pull/40832)
* [keep `ast::NodeId` for range expressions during HIR lowering](https://github.com/rust-lang/rust/pull/40764)
* [improve `InternedString` usability](https://github.com/rust-lang/rust/pull/40606) (notably, it now implements `PartialEq` to anything string-y)
* [make `overlapping_inherent_impls` lint a hard error](https://github.com/rust-lang/rust/pull/40728)
* [specialize `Vec::from_iter` for `Vec::IntoIter`](https://github.com/rust-lang/rust/pull/40731)
* [`sort()` is now tested against random comparison functions](https://github.com/rust-lang/rust/pull/40947)
* [`core::cmp::Reverse`](https://github.com/rust-lang/rust/pull/40720) (also [implements `PartialOrd` fully](https://github.com/rust-lang/rust/pull/40929))
* [`impl `{`AsRawFd`, `IntoRawFd`}` for RawFd`](https://github.com/rust-lang/rust/pull/40842)
* [checked slicing for strings](https://github.com/rust-lang/rust/pull/40737)
* [no longer cache stdio handles on Windows](https://github.com/rust-lang/rust/pull/40516)
* [add Pijul support to Cargo](https://github.com/rust-lang/cargo/pull/3842)
* [rustdoc now uses `pulldown-cmark` instead of hoedown](https://github.com/rust-lang/rust/pull/40516) (yay!)
* [rustdoc accepts `#` at the start of a markdown file](https://github.com/rust-lang/rust/pull/40828)
* [rustdoc no longer documents reexported macros twice](https://github.com/rust-lang/rust/pull/40814)
* [`crates.io` now sorts versions following semver](https://github.com/rust-lang/crates.io/pull/665)
* [`crates.io` index corruption fixed](https://github.com/rust-lang/crates.io/pull/658)
* [the playpen now highlights spans in MIR comments](https://github.com/rust-lang/rust-playpen/pull/284)

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
