Title: This Week in Rust 300
Number: 300
Date: 2019-08-20
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

This week's crate is [async-std](https://crates.io/crates/async-std), a library with async variants of the standard library's IO etc.

Thanks to [mmmmib](https://users.rust-lang.org/t/crate-of-the-week/2704/602) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Survey from the Rust Game Development Working Group](https://users.rust-lang.org/t/survey-from-the-rust-game-development-working-group/31270).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

268 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-08-12..2019-08-19

* [Hash the remapped sysroot instead of the original](https://github.com/rust-lang/rust/pull/63505)
* [Make sure that all file loading happens via SourceMap](https://github.com/rust-lang/rust/pull/63525)
* [syntax: Account for CVarArgs being in the argument list](https://github.com/rust-lang/rust/pull/63459)
* [Remove redundant `ty` fields from `mir::Constant` and `hair::pattern::PatternRange`](https://github.com/rust-lang/rust/pull/63495)
* [resolve: Remove remaining special cases from built-in macros](https://github.com/rust-lang/rust/pull/63449)
* [resolve: Properly integrate derives and `macro_rules` scopes](https://github.com/rust-lang/rust/pull/63667)
* [Point at the right enclosing scope when using `await` in non-async fn](https://github.com/rust-lang/rust/pull/63509)
* [typeck: Prohibit RPIT types that inherit lifetimes](https://github.com/rust-lang/rust/pull/62849)
* [Handle elision in async fn correctly](https://github.com/rust-lang/rust/pull/63499)
* [When needing type annotations in local bindings, account for impl Trait and closures](https://github.com/rust-lang/rust/pull/63507)
* [Improved error message for break in async block](https://github.com/rust-lang/rust/pull/63659)
* [Suggest Rust 2018 on `<expr>.await` with no such field](https://github.com/rust-lang/rust/pull/63539)
* [Crank up invalid value lint](https://github.com/rust-lang/rust/pull/63657)
* [Refactor Miri ops (unary, binary) to have more types](https://github.com/rust-lang/rust/pull/63658)
* [Do not generate allocations for zero sized allocations](https://github.com/rust-lang/rust/pull/63635)
* [Feature gate 'yield $expr?' pre-expansion](https://github.com/rust-lang/rust/pull/63545)
* [Provide map_ok and map_err method for Poll<Option<Result<T, E>>>](https://github.com/rust-lang/rust/pull/63512)
* [Implement `Clone`, `Display` for `ascii::EscapeDefault`](https://github.com/rust-lang/rust/pull/63421)
* [Add APIs for uninitialized `Box`, `Rc`, and `Arc` (Plus `get_mut_unchecked`)](https://github.com/rust-lang/rust/pull/62451)
* [Reduce the genericity of closures in the iterator traits](https://github.com/rust-lang/rust/pull/62429)
* [Add custom `nth_back` for `Chain`](https://github.com/rust-lang/rust/pull/60492)
* [`cargo install`: Remove orphaned executables](https://github.com/rust-lang/cargo/pull/7246)

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

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Remove recommendation about idiomatic syntax for Arc::clone](https://github.com/rust-lang/rust/pull/63252).
* [disposition: merge] [Tracking issue for {HashMap, BTreeMap}::get_key_value stabilization](https://github.com/rust-lang/rust/issues/49347).
* [disposition: merge] [Tracking issue for Pin::{into_inner, into_inner_unchecked} (feature `pin_into_inner`)](https://github.com/rust-lang/rust/issues/60245).
* [disposition: merge] [Clarify `Box<T>` representation and its use in FFI](https://github.com/rust-lang/rust/pull/62514).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Asia Pacific

* [Aug 17. Taipei, TW - "Everything in Rust" at COSCUP 2019](https://coscup.org/2019/en/).
* [Aug 27. Seoul, KR - Seoul Rust Meetup, Hapjeong](https://www.meetup.com/Rust-Seoul-Meetup/events/nxkdfryzlbkc/).
* [Aug 20. Wellington, NZ - Rust Wellington - Talks: "A C++ Programmer Learns Rust!" + "Ergonomic Errors"](https://www.meetup.com/Rust-Wellington/events/262426843/).

### Europe

* [Aug 19. Berlin, DE - Rust Berlin - Rust for Decentralised Technology](https://www.meetup.com/Rust-Berlin/events/263390533).
* [Aug 21. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzlbcc/).
* [Aug 21. Berlin, DE - In Rust We Trust - VM on Blockchain](https://www.meetup.com/Rust-in-Blockchain-Berlin/events/263526816/).
* [Aug 26. Thessaloniki, GR - Rust + GNOME Workshop at GUADEC](https://wiki.gnome.org/GUADEC/2019/Hackingdays/RustGtkGstWorkshop).
* [Aug 27. Thessaloniki, GR - Rust + GNOME BoF at GUADEC](https://wiki.gnome.org/GUADEC/2019/Hackingdays/RustBoF).
* [Aug 29. Zurich, CH - Rust Zurich - August Community Meetup](https://www.meetup.com/Rust-Zurich/events/263756588/).

### North America

* [Aug 21. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzlbcc/).
* [Aug 21. Portland, OR, US - PDXRust - Hack Night (not the usual meetup!)](https://www.meetup.com/PDXRust/events/263076291/).
* [Aug 26. Durham, NC, US - Triangle Rustaceans - Build a syslog server with mio](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzlbjc/).
* [Aug 27. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzlbkc/).
* [Aug 27. Chicago, IL, US - Chicago Rust Meetup - Macros Rule! A Dive Into Rust's Syntax Extension Toolbox](https://www.meetup.com/Chicago-Rust-Meetup/events/263849534).
* [Aug 28. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzlblc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).
* [Security Engineer at Parity, Berlin, DE](https://www.parity.io/jobs/#berlin-security-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> C++ being memory safe is like saying riding a motorcycle is crash safe.
>
> It totally is, if you happen to have the knowledge and experience to realize this is only true if you remember to put on body-armor, a helmet, a full set of leathers including gloves and reinforced boots, and then remember to operate the motorcycle correctly afterwards. In C/C++ though, that armor is completely 100% optional.

– [cyrusm on /r/rust](https://www.reddit.com/r/rust/comments/cseulx/is_rust_a_new_paradigmclass_of_programing/exeyibc)

Thanks to [Dmitry Kashitsyn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/682) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
