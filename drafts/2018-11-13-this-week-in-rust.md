Title: This Week in Rust 260
Number: 260
Date: 2018-11-13
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

This week's crate is [cargo-nono](https://github.com/hobofan/cargo-nono), a cargo subcommand to check a crate's dependencies for no-std compatibility. Thanks to [Hobofan](https://www.reddit.com/r/rust/comments/9wbv0v/cargo_nono_detect_possible_no_std_compatibility) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [swc transcompiles new-generation JavaScript to old-generation JavaScript and needs help](https://users.rust-lang.org/t/twir-call-for-participation/4821/214).
* [Tera: Consider remove error_chain dependency](https://github.com/Keats/tera/issues/297).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

140 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-11-05..2018-11-12

* [remove support for building against LLVM 4](https://github.com/rust-lang/rust/pull/55698)
* [use lld directly for Fuchsia target](https://github.com/rust-lang/rust/pull/55106)
* [support memcpy/memmove with differing src/dst alignment](https://github.com/rust-lang/rust/pull/55633)
* [treat "proc-macro" crate type the same as `proc-macro = true`](https://github.com/rust-lang/cargo/pull/6256)
* [custom diagnostic when trying to doc comment argument](https://github.com/rust-lang/rust/pull/55451)
* [enforce unused-must-use lint in macros](https://github.com/rust-lang/rust/pull/55569)
* [don't print opt fuel messages to stdout because it breaks Rustbuild](https://github.com/rust-lang/rust/pull/55495)
* [NLL: fix ICE with elided lifetimes](https://github.com/rust-lang/rust/pull/55822)
* [NLL: update box insensitivity test](https://github.com/rust-lang/rust/pull/55801)
* [NLL: missing errors for borrows of union fields](https://github.com/rust-lang/rust/pull/55696)
* [NLL: unions not reinitialized after assignment into field](https://github.com/rust-lang/rust/pull/55657)
* [consume optimization fuel from the MIR inliner](https://github.com/rust-lang/rust/pull/55739)
* [take supertraits into account when calculating associated types](https://github.com/rust-lang/rust/pull/55687)
* [typecheck patterns of all match arms first, so we get types for bindings](https://github.com/rust-lang/rust/pull/55819)
* [don't inline virtual calls (take 2)](https://github.com/rust-lang/rust/pull/55802)
* [use `SmallVec` to avoid allocations in `from_decimal_string`](https://github.com/rust-lang/rust/pull/55816)
* [un-`P` my `Lit`! Don't allocate it in vain](https://github.com/rust-lang/rust/pull/55777)
* [don't `Box` the `TyCtxt::associated_items`](https://github.com/rust-lang/rust/pull/55604)
* [make `MatcherPos::stack` a `SmallVec`](https://github.com/rust-lang/rust/pull/55525)
* [improve creation of 3 IndexVecs](https://github.com/rust-lang/rust/pull/55755)
* [implement rotate using funnel shift on LLVM >= 7](https://github.com/rust-lang/rust/pull/55650)
* [value visitors for miri](Value visitors for miri)
* [remove the `alloc_system` crate](https://github.com/rust-lang/rust/pull/55660)
* [std: improve codegen size of accessing TLS](https://github.com/rust-lang/rust/pull/55518)
* [std: enable usage of `thread_local!` through imports](https://github.com/rust-lang/rust/pull/55597)
* [choose predicates without inference variables over those with them](https://github.com/rust-lang/rust/pull/55453)
* [minor standard library constification](https://github.com/rust-lang/rust/pull/55278)
* [fix `Rc`/`Arc` allocation layout](https://github.com/rust-lang/rust/pull/55764)
* [fix undefined behavior in `Rc`/`Arc` allocation](https://github.com/rust-lang/rust/pull/54922)
* [cargo: avoid retaining all rustc output in memory](https://github.com/rust-lang/cargo/pull/6289)
* [cargo: timeout batch downloads, not each download](https://github.com/rust-lang/cargo/pull/6285)
* [cargo: small things to help with fuzz tests](https://github.com/rust-lang/cargo/pull/6274)
* [cargo: don't include build scripts in --out-dir](https://github.com/rust-lang/cargo/pull/6300)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2561: Meta-RFC: Future possibilities](https://github.com/rust-lang/rfcs/pull/2561).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Linked list cursors](https://github.com/rust-lang/rfcs/pull/2570).
* [disposition: close] [flat_map as an alias for and_then](https://github.com/rust-lang/rfcs/pull/2572).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Minor standard library constification](https://github.com/rust-lang/rust/pull/55278).

## New RFCs

* [An operator to take a raw reference](https://github.com/rust-lang/rfcs/pull/2582).
* [Structural Records](https://github.com/rust-lang/rfcs/pull/2584).
* [Unsafe blocks in unsafe fn](https://github.com/rust-lang/rfcs/pull/2585).
* [Anonymous variant types, a minimal ad-hoc sum type](https://github.com/rust-lang/rfcs/pull/2587).

# Upcoming Events

### Online

* [Nov 14. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov 19. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov 21. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Europe

* [Nov 14. Helsinki, FI - Helsinki Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/255855675/).
* [Nov 14. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxpbsb/).
* [Nov 15. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxpbtb/).
* [Nov 15. Madrid, ES - Rust + Deliveroo](https://www.meetup.com/MadRust/events/256141489).
* [Nov 17. Toulouse, FR - Capitole du Libre 2018 - Introduction à Rust](https://2018.capitoledulibre.org/programme/#introduction-a-rust-2).
* [Nov 17. Toulouse, FR - Capitole du Libre 2018 - Initiation à Rust](https://2018.capitoledulibre.org/programme/#initiation-a-rust).
* [Nov 21. Oslo, NO - Hack & Learn](https://www.meetup.com/Rust-Oslo/events/255966088/).
* [Nov 21. Hamburg, DE - Rust Hack & Learn Nov 2018](https://www.meetup.com/Rust-Meetup-Hamburg/events/254969484/).

### North America

* [Nov  8. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209655/).
* [Nov  8. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/254871472).
* [Nov  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxpblb/).
* [Nov  8. Boston, US - Rust/Scala meetup at SPLASH conf](https://www.meetup.com/BostonRust/events/255445951/).
* [Nov  8. Arlington, US - Rust DC—Mid-month Rustful](https://www.meetup.com/RustDC/events/254871472).
* [Nov 11. Mountain View, US - Rust Dev in Mountain View](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbpb/).
* [Nov 12. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxpbqb/).
* [Nov 13. Charlottesville, US - Rustlang Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/events/255551577/).
* [Nov 14. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/xttphqyxpbsb/).
* [Nov 18. Mountain View, US - Rust Dev in Mountain View](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbxb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Backend Engineer at Kraken, Remote](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Senior Blockchain Protocol Engineer at crypto.com, Hong Kong](https://cryptocom.bamboohr.com/jobs/view.php?id=61).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I’m also pretty sure that most languages would not go that far. The idea that the type plugged in has only one possible value, therefore it doesn’t need to be stored and methods on that don’t care about the `self` reference is pretty neat.

– Michael 'vorner' Vaner [on his blog](https://vorner.github.io/2018/11/11/truly-zero-cost.html)

Thanks to llogiq for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
