Title: This Week in Rust 313
Number: 313
Date: 2019-11-19
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

* [crates.io: carols10cents will be mentoring multiple issues for the month of November & December](https://github.com/rust-lang/crates.io/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3AE-mentor).
* [simdeez: Create SIMD sin/cos/log etc functions as in agner fog's vector libraries](https://github.com/jackmott/simdeez/issues/17).
* [Spirit: Tokio 0.2 and hyper 0.13 support](https://github.com/vorner/spirit/issues/45).
* [Spirit: Support for slog](https://github.com/vorner/spirit/issues/46).

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

* [disposition: merge] [Stabilize `!` in Rust 1.41.0](https://github.com/rust-lang/rust/pull/65355).
* [disposition: merge] [Stabilize Result::map_or_else](https://github.com/rust-lang/rust/pull/66322).

## New RFCs

* [Generic parameters in derive](https://github.com/rust-lang/rfcs/pull/2811).
* [Adding is_zero() to core::time::Duration](https://github.com/rust-lang/rfcs/pull/2814).

# Upcoming Events

### Europe

* [Nov 14. Zurich, CH - Rust Zurich - RustFest Decompression Zürich](https://www.meetup.com/Rust-Zurich/events/265593126/).
* [Nov 14. Moscow, RU - Rust Moscow November 2019 Meetup](https://www.meetup.com/ru-RU/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/266184946/).
* [Nov 14. Zagreb, HR - impl Zagreb for Rust - Rust Meetup 201911: Proceduralni makroi](https://www.meetup.com/Zagreb-Rust-Meetup/events/266226748).
* [Nov 15. Barcelona, ES - Rust GTK/GStreamer Workshop at Linux Application Summit 2019](https://www.meetup.com/Barcelona-Free-Software/events/265596417/).
* [Nov 19-21, Rome, Italy - Weekly Rust course at "La Sapienza" University](https://lugsapienza.altervista.org/corsorust-nov2019).
* [Nov 21. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/265961100).
* [Nov 27. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzpbkc/).

### North America

* [Nov 14. San Diego, CA, US - San Diego Rust November Meetup](https://www.meetup.com/San-Diego-Rust/events/265981542/).
* [Nov 14. Lehi, UT, US - Utah Rust - November 2019 Regular Meetup](https://www.meetup.com/utah-rust/events/265905259/).
* [Nov 14. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzpbsb/).
* [Nov 14. Montreal, QC, CA - Montreal Rust Meetup - November 2019 RustMTL: November Common Traits & Causal Profiling](https://www.meetup.com/Rust-Montreal/events/prvrjryzpbqb/).
* [Nov 14. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/265769078).
* [Nov 20. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/265998640/).
* [Nov 20. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzpbkc/).
* [Nov 20. Chicago, IL, US - Chicago Rust Meetup - Constructing a Repl(like) from scratch](https://www.meetup.com/Chicago-Rust-Meetup/events/266237535/).
* [Nov 25. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzpbhc/).
* [Nov 26. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzpbjc/).
* [Nov 27. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzpbkc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I said it before, and I'll say it again: If one views Rust as a critique on C++, one should view it as a constructive critique.

– [llogiq on /r/rust](https://www.reddit.com/r/rust/comments/dyr8ps/rust_from_a_cc_point_of_view_viceversa/f835w7h)

Thanks to [Dmitry Kashitsyn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/741) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
