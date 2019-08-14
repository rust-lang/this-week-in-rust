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

This week's crate is [topgrade](https://crates.io/crates/topgrade), a command-line program to upgrade all the things.

Thanks to [Dror Levin](https://users.rust-lang.org/t/crate-of-the-week/2704/598) for the suggestion!

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

270 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-08-05..2019-08-12

* [Sort the fat LTO modules to produce deterministic output](https://github.com/rust-lang/rust/pull/63352)
* [More explicit diagnostic when using a `vec![]` in a pattern](https://github.com/rust-lang/rust/pull/63399)
* [Give built-in macros stable addresses in the standard library](https://github.com/rust-lang/rust/pull/63056)
* [Remove gensym in `format_args`](https://github.com/rust-lang/rust/pull/63114)
* [Cleanup & Simplify stuff in lowering](https://github.com/rust-lang/rust/pull/63432)
* [Revert "Simplify MIR generation for logical ops"](https://github.com/rust-lang/rust/pull/63431)
* [CTFE: Simplify `ConstValue` by not checking for alignment](https://github.com/rust-lang/rust/pull/63079)
* [Miri: Use ldexp from cmath instead](https://github.com/rust-lang/miri/pull/898)
* [Fix generator size regressions due to optimization](https://github.com/rust-lang/rust/pull/63034)
* [Improve invalid_value lint message](https://github.com/rust-lang/rust/pull/63483)
* [Fix for "ambiguous associated type" issue with ATBs](https://github.com/rust-lang/rust/pull/61919)
* [Add implementations for converting boxed slices into boxed arrays](https://github.com/rust-lang/rust/pull/61515)
* [Add {`IoSlice`, `IoSliceMut`}`::advance`](https://github.com/rust-lang/rust/pull/62987)
* [Stabilize `duration_float`](https://github.com/rust-lang/rust/pull/62756)
* [Deprecate `try!` macro](https://github.com/rust-lang/rust/pull/62672)
* [Use internal iteration in the Sum and Product impls of `Result` and `Option`](https://github.com/rust-lang/rust/pull/62459)
* [Implement `DoubleEndedIterator` for `iter::`{`StepBy`, `Peekable`, `Take`}](https://github.com/rust-lang/rust/pull/61457)
* [Skip roundtrip on few structs on OpenBSD](https://github.com/rust-lang/libc/pull/1456)
* [cargo: Improve error message when using API command with non-remote registry](https://github.com/rust-lang/cargo/pull/7239)

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

> For me, acquiring a taste for rustfmt-style seems worthwhile to 'eliminate broad classes of debate', even if I didn't like some of the style when I first looked. I've resisted the temptation to even read about how to customise.
>
> Years ago, I was that person writing style guides etc. I now prefer this problem to be automated-away; freeing up time for malloc-memcpy-golf (most popular sport in the Rust community).

– [@dholroyd on rust-users](https://users.rust-lang.org/t/how-are-you-using-rustfmt-and-clippy/31082/8)

Thanks to [troiganto](https://users.rust-lang.org/t/twir-quote-of-the-week/328/680) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
