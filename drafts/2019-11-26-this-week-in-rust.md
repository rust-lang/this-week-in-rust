Title: This Week in Rust 314
Number: 314
Date: 2019-11-26
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

* [Tokio roadmap to 1.0](https://tokio.rs/blog/2019-11-tokio-0-2/).
* [Shipping a compiler every six weeks](https://www.pietroalbini.org/blog/shipping-a-compiler-every-six-weeks/).
* [`if` and `match` in constants on nightly rust](https://blog.rust-lang.org/inside-rust/2019/11/25/const-if-match.html).
* [Towards a unified theory of reactive UI](https://raphlinus.github.io/ui/druid/2019/11/22/reactive-ui.html).
* [Surveying error handling libraries](https://blog.yoshuawuyts.com/error-handling-survey/).
* [Porting librsvg to Rust: Refactoring the Length type](https://people.gnome.org/~federico/blog/refactoring-the-length-type.html).
* [Lessons learned by transpiling C to Rust](https://immunant.com/blog/2019/11/rust2020/).
* [How to panic in Rust](https://www.ralfj.de/blog/2019/11/25/how-to-panic-in-rust.html).
* [Neat Rust tricks: Passing Rust closures to C](https://blog.seantheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c).
* [Moving gnome-shell's styles to Rust](https://people.gnome.org/~federico/blog/moving-gnome-shell-styles-to-rust.html).
* [Async-awaitifying a Rust CLI App](https://zupzup.org/async-awaitify-rust-cli/).
* [Building a Rust driver for PineTime’s touch controller](https://medium.com/@ly.lee/building-a-rust-driver-for-pinetimes-touch-controller-cbc1a5d5d3e9).
* [RustFest Barcelona talk recordings are now available](https://blog.rustfest.eu/this-week-in-rustfest-barcelona-videos).
* [Cryptowatch is sponsoring development of Rust GUI library iced](https://blog.cryptowat.ch/2019/11/25/sponsoring-rust-gui-library-iced/).

### #Rust2020

Find all #Rust2020 posts at [Read Rust](https://readrust.net/rust-2020/).

# Crate of the Week

This week's crate is [rerast](https://github.com/google/rerast), a rule-based Rust code transformation tool.

Thanks to [Jan Riemer](https://users.rust-lang.org/t/crate-of-the-week/2704/674) for the suggestions!

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

260 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-11-18..2019-11-25

* [stabilize `!`](https://github.com/rust-lang/rust/pull/65355)
* [stabilize `cfg(doc)`](https://github.com/rust-lang/rust/pull/61351)
* [debuginfo: support for `std::collections::Hash*` in windows debuggers](https://github.com/rust-lang/rust/pull/66597)
* [make gdb pretty-printing more robust when printing uninitialized `Vec`](https://github.com/rust-lang/rust/pull/66576)
* [generate DWARF address ranges for faster lookups](https://github.com/rust-lang/rust/pull/66532)
* [fix cycle when debug-printing opaque types](https://github.com/rust-lang/rust/pull/66594)
* [resolve: give derive helpers highest priority during resolution](https://github.com/rust-lang/rust/pull/66529)
* [remove pretty printing of specific nodes in AST](https://github.com/rust-lang/rust/pull/66575)
* [point at type in `let` assignment on type errors](https://github.com/rust-lang/rust/pull/66539)
* [suggest calling async closure when needed](https://github.com/rust-lang/rust/pull/66239)
* [suggest `#[repr(C)]` instead of `#[repr(C, packed, ...)]`](https://github.com/rust-lang/rust/pull/66206)
* [add outlives suggestions for some lifetime errors](https://github.com/rust-lang/rust/pull/58281)
* [use a `SmallVec` for `Candidate::match_pairs`](https://github.com/rust-lang/rust/pull/66540)
* [miri: add `acos`, `asin`, and `atan` foreign functions](https://github.com/rust-lang/miri/pull/1067)
* [mir-opt: asking `?`s in a more optimized fashion](https://github.com/rust-lang/rust/pull/66282)
* [mir-opt: turn on the `ConstProp` pass by default](https://github.com/rust-lang/rust/pull/66074)
* [miri: support unwinding after a panic](https://github.com/rust-lang/miri/pull/693)
* [handle statics in MIR as const pointers](https://github.com/rust-lang/rust/pull/66587)
* [delay an `is_local_ever_initialized` call](https://github.com/rust-lang/rust/pull/66537)
* [reduce size of `hir::Expr` by boxing more of `hir::InlineAsm`](https://github.com/rust-lang/rust/pull/66515)
* [use proc-macro to derive HashStable everywhere](https://github.com/rust-lang/rust/pull/66279)
* [remove `compiler_builtins_lib` feature from libstd](https://github.com/rust-lang/rust/pull/66538)
* [std::error::Chain: remove `Copy`](https://github.com/rust-lang/rust/pull/66511)
* [use `drop_in_place` in `array::IntoIter::drop`](https://github.com/rust-lang/rust/pull/65821)
* [stabilize `Result::map_or_else`](https://github.com/rust-lang/rust/pull/66322)
* [libc: deprecate vfork](https://github.com/rust-lang/libc/pull/1574)
* [libc: add initial support for sparc-unknown-linux-gnu](https://github.com/rust-lang/libc/pull/1567)
* [cargo: extend documentation on security concerns of crate names in a registry](https://github.com/rust-lang/cargo/pull/7616)
* [cargo: turn the new lock file format on by default](https://github.com/rust-lang/cargo/pull/7579)
* [cargo: stabilize install-upgrade](https://github.com/rust-lang/cargo/pull/7560)
* [rustdoc: stabilize `edition` annotation](https://github.com/rust-lang/rust/pull/66238)
* [rustdoc: preserve whitespace inside one-backtick codeblocks](https://github.com/rust-lang/rust/pull/65613)
* [measureme: optimize FileSerializationSink by using parking_lot::Mutex and avoiding heap allocations in write_atomic](https://github.com/rust-lang/measureme/pull/88)

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

* [PhD, postdoc & intern positions in RustBelt and Iris projects at Max Planck](https://users.rust-lang.org/t/jobs-phd-postdoc-intern-positions-in-rustbelt-and-iris-projects-at-max-planck/35016).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I said it before, and I'll say it again: If one views Rust as a critique on C++, one should view it as a constructive critique.

– [llogiq on /r/rust](https://www.reddit.com/r/rust/comments/dyr8ps/rust_from_a_cc_point_of_view_viceversa/f835w7h)

Thanks to [Dmitry Kashitsyn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/741) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
