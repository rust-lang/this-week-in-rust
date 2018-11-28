Title: This Week in Rust 262
Number: 262
Date: 2018-11-27
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
- [Rust Latam CFP is now open, deadline is December 31st.](https://cfp.rustlatam.org/events/rust-latam) Also [ticket sales are open](https://rustlatam.org/#tickets).

 * [Rust Language Cheat Sheet (cheats.rs)](https://cheats.rs).

# Crate of the Week

This week's crate is [modulator](https://crates.io/crates/modulator), a crate of abstract modulators for use in audio synthesizers (and possibly elsewhere). Thanks to [Andrea Pessino](https://www.youtube.com/watch?v=n-txrCMvdms) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Mundane: Test that symbol version names are correct](https://github.com/google/mundane/issues/8).
* [Mundane: Linker bug compiling on Linux](https://github.com/google/mundane/issues/3).
* [Mundane: Pin to a particular version of BoringSSL](https://github.com/google/mundane/issues/10).
* [gcode: WebAssembly showcase website](https://github.com/Michael-F-Bryan/gcode-rs/issues/32).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

173 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-11-19..2018-11-26

* [lint if a private item has doctests](https://github.com/rust-lang/rust/pull/55367)
* [fix self profiler ICE on Windows](https://github.com/rust-lang/rust/pull/56170)
* [allow `#[must_use]` on traits](https://github.com/rust-lang/rust/pull/55663)
* [suggest correct syntax when writing type arg instead of assoc type](https://github.com/rust-lang/rust/pull/55808)
* [`match_ref_pats`: don't emit suggestions inside of a macro](https://github.com/rust-lang/rust-clippy/pull/3432)
* [fix stability hole with `static _](https://github.com/rust-lang/rust/pull/55983)
* [stabilize `macro_literal_matcher`](https://github.com/rust-lang/rust/pull/56072)
* [check arg/ret sizedness at `ExprKind::Path`](https://github.com/rust-lang/rust/pull/56045)
* [miri: accept extern types in structs if they are the only field](https://github.com/rust-lang/rust/pull/55672)
* [miri engine refactoring](https://github.com/rust-lang/rust/pull/55915)
* [allow assignments in const contexts](https://github.com/rust-lang/rust/pull/56070)
* [clean up and streamline snapshot data structures](https://github.com/rust-lang/rust/pull/55906)
* [remove clones made redundant by Intern `SourceId`](https://github.com/rust-lang/cargo/pull/6347)
* [cleanup from lexical MIR borrowck removal](https://github.com/rust-lang/rust/pull/55959)
* [stabilize `extern_crate_item_prelude`](https://github.com/rust-lang/rust/pull/56032)
* [generator fields are not necessarily initialized](https://github.com/rust-lang/rust/pull/56100)
* [stabilize the `int_to_from_bytes` feature](https://github.com/rust-lang/rust/pull/56207)
* [add `std::iter::unfold`](https://github.com/rust-lang/rust/pull/55869)
* [`read_c_str` should call the `AllocationExtra` hooks](https://github.com/rust-lang/rust/pull/56210)
* [implement `checked_add_duration` for `SystemTime`](https://github.com/rust-lang/rust/pull/55527)
* [return `&T` / `&mut T` in `ManuallyDrop` `Deref`(`Mut`) impl](https://github.com/rust-lang/rust/pull/55485)
* [debug: fix `VecDeque` pretty-printer](https://github.com/rust-lang/rust/pull/55961)
* [debug: fix `BTreeSet` and `BTreeMap` gdb pretty-printers](https://github.com/rust-lang/rust/pull/56144)
* [do not panic just because cargo failed](https://github.com/rust-lang/rust/pull/55867)
* [cargo: allow `crate_type=bin` examples to run](Allow crate_type=bin examples to run)

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

* [disposition: merge] [Stabilise exhaustive integer pattern matching](https://github.com/rust-lang/rfcs/pull/2591).
* [disposition: merge] [Needle API (née Pattern API)](https://github.com/rust-lang/rfcs/pull/2500).
* [disposition: close] [Add RFC for officially adopting Ferris](https://github.com/rust-lang/rfcs/pull/2328).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [[Stabilization] Pin APIs](https://github.com/rust-lang/rust/issues/55766).
* [disposition: merge] [Refiling "#[repr(simd)] struct(isize, isize) not allowed"](https://github.com/rust-lang/rust/issues/55078).
* [disposition: merge] [Tracking Issue: Duration::{as_nanos, as_micros, as_millis}](https://github.com/rust-lang/rust/issues/50202).
* [disposition: merge] [Tracking issue for `?` macro repetition](https://github.com/rust-lang/rust/issues/48075).
* [disposition: merge] [[Stabilization] Stablize using some arbitrary self types defined in std](https://github.com/rust-lang/rust/issues/55786).
* [disposition: merge] [[beta] resolve: Implement edition hygiene for imports and absolute paths](https://github.com/rust-lang/rust/pull/56053).

# Upcoming Events

### Online

* [Nov 28. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Dec  3. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Dec  5. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Africa

* [Dec  5. Johannesburg, SA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/jdqplqyxqbhb/).

### Asia

* [Nov 23. Delhi, IN- Rust workshop at Delhi, India]( https://reps.mozilla.org/e/rust-community-meetup-delhi/).
* [Dec  6. Pune, IN - Rust workshop at Pune, India](https://reps.mozilla.org/e/rust-community-meetup-pune/).

### Europe

* [Nov 24. St. Petersburg, RU - Rust Meetup](https://www.meetup.com/spbrust/events/bqctlqyxpbgc).
* **[Nov 24 & 25. Rome, IT - RustFest Rome 2018](https://rome.rustfest.eu).**
* [Nov 27. Sofia, BG - Rust Bulgaria @ Global Tech Summit](https://www.meetup.com/rust-bulgaria/events/256338832/).
* [Nov 27. Vienna, AT - Rust - November Meetup](https://www.meetup.com/Rust-Vienna/events/256401313).
* [Nov 28. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxpblc/).
* [Nov 29. Copenhagen, DK - Copenhagen Rust Group - Hack Night #11](http://cph.rs/).
* [Dec  3. Karlsruhe, DE - Rust 2018 Edition](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/256200841/?_xtd=gqFyqTE5MzgwNjQ5OKFwp2FuZHJvaWQ&from=ref).
* [Dec 15 & 16. Moscow, RU - RustRush 2018](https://rustrush.ru).

### North America

* [Nov 25. Mountain View, US - Rust Dev in Mountain View](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbhc/).
* [Nov 26. Durham, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxpbjc/).
* [Nov 27. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxpbkc/).
* [Nov 27. Chicago, US - Chicago Rust Meetup - So You Want To Write A Protocol Client In Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/255834903/).
* [Nov 28. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/xttphqyxpblc/).
* [Nov 28. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyxpblc/).
* [Dec  2. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbdb/).
* [Dec  5. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyxqbhb/).
* [Dec  5. Indianopolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxqbhb/).
* [Dec  6. Phoenix, US - Phoenix 2018 Edition Release Party](https://www.meetup.com/Desert-Rustaceans/events/256503618).

### South America

* [Nov 24. Sao Paulo, BR - Rust Meetup: Sao Paulo](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/255942981/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer for Qwant Maps, Paris, FR](https://www.welcometothejungle.co/companies/qwant-research/jobs/software-engineer-qwant-maps_paris).
* [Embedded operating system developer, Karlsruhe, DE](https://www.pse.kit.edu/karriere/joboffer.php?id=2093&language=en).
* [Student research assistant (embedded), Karlsruhe, DE](https://twitter.com/oli_obk/status/1064856324071178240).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> "I did not want to inflict memory management on my son" – @M_a_s_s_i

– Massimiliano Mantione [during his RustFest talk](https://twitter.com/RustFest/status/1058302698834087936)

Thanks to llogiq for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
