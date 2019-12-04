Title: This Week in Rust 315
Number: 315
Date: 2019-12-03
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

- [How to RiiR](http://adventures.michaelfbryan.com/posts/how-to-riir/)

### #Rust2020

Find all #Rust2020 posts at [Read Rust](https://readrust.net/rust-2020/).

# Crate of the Week

This week's crate is [joinery](https://docs.rs/joinery), a library for generic string joining. 

Thanks to [Nathan West](https://users.rust-lang.org/t/crate-of-the-week/2704/677) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [seeking input] [Time v0.2 pre-release feedback](https://github.com/time-rs/time/issues/190).
* [good first issue] [what: Shared os behaviour](https://github.com/imsnif/what/issues/17).
* [what: Listen on all interfaces](https://github.com/imsnif/what/issues/16).
* [crates.io: carols10cents will be mentoring multiple issues for the month of November & December](https://github.com/rust-lang/crates.io/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3AE-mentor).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

264 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-11-25..2019-12-02

* [stabilize nested self receivers in 1.41.0](https://github.com/rust-lang/rust/pull/64325)
* [add memoization for const function evaluations](https://github.com/rust-lang/rust/pull/66294)
* [add crc and crypto to target feature whitelist on arm](https://github.com/rust-lang/rust/pull/66918)
* [conditional compilation for sanitizers](https://github.com/rust-lang/rust/pull/66245)
* [add support for sanitizer recover and tracking origins of uninitialized memory](https://github.com/rust-lang/rust/pull/66522)
* [async fn resume after completion](https://github.com/rust-lang/rust/pull/66321)
* [use structured suggestion when requiring `Copy` constraint in type param](https://github.com/rust-lang/rust/pull/66567)
* [various tweaks to diagnostic output](https://github.com/rust-lang/rust/pull/66754)
* [add version mismatch help message for unimplemented trait](https://github.com/rust-lang/rust/pull/66561)
* [AST address-of](https://github.com/rust-lang/rust/pull/66671)
* [initial implementation of or-pattern usefulness checking](https://github.com/rust-lang/rust/pull/66612)
* [polonius: illegal subset relation errors using placeholder loans](https://github.com/rust-lang/polonius/pull/137)
* [fix opaque types resulting from projections in function signature](https://github.com/rust-lang/rust/pull/66178)
* [simplify memory categorization](https://github.com/rust-lang/rust/pull/66246)
* [remove interior mutability in mir predecessors cache](https://github.com/rust-lang/rust/pull/64736)
* [rustc: move debug info from LocalDecl and UpvarDecl into a dedicated VarDebugInfo](https://github.com/rust-lang/rust/pull/56231)
* [create promoted MIR fragments for `const` and `static`s](https://github.com/rust-lang/rust/pull/66642)
* [alloc: add new_zeroed() versions like new_uninit()](https://github.com/rust-lang/rust/pull/66128)
* [impl TrustedLen for vec::Drain](https://github.com/rust-lang/rust/pull/66759)
* [atomic as_mut_ptr](https://github.com/rust-lang/rust/pull/66705)
* [implement Debug for MaybeUninit](https://github.com/rust-lang/rust/pull/65013)
* [libc: add support for shared memory operations for solaris/illumos](https://github.com/rust-lang/libc/pull/1584)
* [cargo: stabilize profile-overrides](https://github.com/rust-lang/cargo/pull/7591)
* [rustup: add toolchain install --allow-downgrade option](https://github.com/rust-lang/rustup/pull/2126)
* [docs.rs: match library properly if multiple crate-types are in use](https://github.com/rust-lang/docs.rs/pull/499)

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

* [disposition: merge] [Stabilize `std::{rc,sync}::Weak::{weak_count, strong_count}`](https://github.com/rust-lang/rust/pull/65778).
* [disposition: merge] [Tracking issue for extra floating-point logarithm constants](https://github.com/rust-lang/rust/issues/50540).
* [disposition: merge] [Implement Debug for MaybeUninit](https://github.com/rust-lang/rust/pull/65013).
* [disposition: close] [Fixes soundness bug 18510 by aborting on unwind from safe extern "C" functions only](https://github.com/rust-lang/rust/pull/64315).

## New RFCs

* [Box error alias](https://github.com/rust-lang/rfcs/pull/2820).

# Upcoming Events

### Online

* [Dec  1. "Advent of Code" livestream in German](https://github.com/scy/advent-of-code).

### Africa

* [Dec  4. Johannesburg, ZA - Johannesburg Rust Meetup - Time for Some Highbrow FP in Rust](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpdtkryzqbgb/).

### Asia Pacific

* [Dec  2. Auckland, NZ - Rust AKL - Introduction to Rust (part 3)](https://www.meetup.com/rust-akl/events/259481456/).

### Europe

* [Nov 26+28, Rome, Italy - Weekly Rust course at "La Sapienza" University: 2nd lesson](https://lugsapienza.altervista.org/corsorust-nov2019).
* [Nov 27. Copenhagen, DK - Copenhagen Rust Hack Night #20](https://cph.rs/).
* [Nov 30. Kharkiv, UA - Peer Lab Kharkiv #Rust: Algorithmic problems solving](https://www.facebook.com/events/571415073420154/).
* [Dec  2. Karlsruhe, DE - Rust Hack & Learn Karlsruhe - Rust Meet-up](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/266554688/).
* [Dec  4. Wroclaw, PL - Rust Wroclaw Meetup #15](https://www.meetup.com/Rust-Wroclaw/events/266756721/).
* [Dec  4. Cologne, DE - Rust Cologne - Advent of .await](https://www.meetup.com/RustCologne/events/tnrnbryzqbgb/).
* [Dec 11. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzqbpb/).
* [Dec 12. Kyiv, UA - Rails Reactor - Rust Ukraine Meetup](https://www.facebook.com/events/1173477969528421/).
* [Dec 12. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/266750624).

### North America

* [Dec  3. San Francisco, CA, US - Rust Bay Area - [@ Cloudflare] Declarative UIs in Rust and Real-world production CLIs](https://www.meetup.com/Rust-Bay-Area/events/266571982).
* [Dec  4. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzqbgb/).
* [Dec  4. Portland, OR, US - PDXRust - macros_rule!](https://www.meetup.com/PDXRust/events/264733991/).
* [Dec 10. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdryzqbnb/).
* [Dec 11. Mesa, AZ, US - Desert Rust - Rust: Crates and Organization](https://www.meetup.com/Desert-Rustaceans/events/wmmphryzpbkc/).
* [Dec 11. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzqbpb/).
* [Dec 12. San Diego, CA, US - San Diego Rust December Meetup](https://www.meetup.com/San-Diego-Rust/events/266502136/).
* [Dec 12. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzqbqb/).
* [Dec 12. Lehi, UT, US - Utah Rust - December 2019 Regular Meetup](https://www.meetup.com/utah-rust/events/265905262/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> > Heard recently creative coding experience which rust gives. What about unconscious coding experience - do whatever you can to make your code compile as late as you can, then go sleep and find your code correct and working in the morning
>
> Woah, I know people say the Rust compiler is slow but I never had a Rust program that took all night to compile☺

– [Maxim Vorobjov and ZiCog in our Quote of the Week Thread](https://users.rust-lang.org/t/twir-quote-of-the-week/328/749)

Thanks to [both of them and mmmmib](https://users.rust-lang.org/t/twir-quote-of-the-week/328/752) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
