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

This week's crate is [cargo-sweep](https://github.com/holmgr/cargo-sweep), a cargo subcommand to clean cargo's various temporaries. Thanks to [Viktor Holmgren](https://users.rust-lang.org/t/crate-of-the-week/2704/470) for the suggestion!

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

124 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-11-12..2018-11-19

* [remove "approx env bounds" if we already know from trait](https://github.com/rust-lang/rust/pull/56043)
* [resolve: implement uniform paths 2.0](https://github.com/rust-lang/rust/pull/56042)
* [chalk lowering rule: ProjectionEq-Normalize](https://github.com/rust-lang/rust/pull/52153)
* [miri backtrace improvements](https://github.com/rust-lang/rust/pull/55970)
* [fix emission of niche-filling discriminant values](https://github.com/rust-lang/rust/pull/55701)
* [avoid shared ref in UnsafeCell::get](https://github.com/rust-lang/rust/pull/56012)
* [CTFE: dynamically make sure we do not call non-const-fn](https://github.com/rust-lang/rust/pull/56007)
* [reattach all grandchildren when constructing specialization graph](https://github.com/rust-lang/rust/pull/54906)
* [ty: return impl Iterator from Predicate::walk_tys](https://github.com/rust-lang/rust/pull/55949)
* [a handful of hir tweaks](https://github.com/rust-lang/rust/pull/55930)
* [make miri value visitor useful for mutation](https://github.com/rust-lang/rust/pull/55916)
* [miri enum discriminant handling: Fix treatment of pointers, better error when it is undef](https://github.com/rust-lang/rust/pull/55894)
* [more precise spans for temps and their drops](https://github.com/rust-lang/rust/pull/55781)
* [reference count `crate_inherent_impls`s return value](https://github.com/rust-lang/rust/pull/55882)
* [unix RwLock: avoid racy access to write_locked](https://github.com/rust-lang/rust/pull/55865)
* [forward the ABI of the non-zero sized fields of an union if they have the same ABI](https://github.com/rust-lang/rust/pull/55834)
* [a few tweaks to iterations/collecting](https://github.com/rust-lang/rust/pull/55827)
* [move `static_assert!` into librustc_data_structures](https://github.com/rust-lang/rust/pull/55805)
* [wrap some query results in `Lrc`](https://github.com/rust-lang/rust/pull/55778)
* [avoid converting bytes to UTF-8 strings to print, just pass bytes to stdout/err](https://github.com/rust-lang/rust/pull/55754)
* [impl_stable_hash_for: support enums and tuple structs with generic parameters](https://github.com/rust-lang/rust/pull/55722)
* [std: Synchronize access to global env during `exec`](https://github.com/rust-lang/rust/pull/55939)
* [add mem::forget_unsized() for forgetting unsized values](https://github.com/rust-lang/rust/pull/55785)
* [core/char: speed up `to_digit()` for `radix <= 10`](https://github.com/rust-lang/rust/pull/55932)
* [add `VecDeque::resize_with`](https://github.com/rust-lang/rust/pull/56016)
* [add `FromIterator<A>` to `Box<[A]>`](https://github.com/rust-lang/rust/pull/55843)
* [speed up `String::from_utf16`](https://github.com/rust-lang/rust/pull/55530)
* [cargo: add `c` alias for `check`](https://github.com/rust-lang/cargo/pull/6218)
* [cargo: distinguish custom build invocations](https://github.com/rust-lang/cargo/pull/6331)
* [cargo: allow crate_type=bin examples to run](https://github.com/rust-lang/cargo/pull/6330)
* [rustdoc: properly calculate spans for intra-doc link resolution errors](https://github.com/rust-lang/rust/pull/55962)

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

> It’s like building stuff with LEGO. Sure, it could be a single type, but then you’d need a type for every possible combination of types, which would arguably be a whole lot worse.

– Daniel Keep [on rust-users](https://users.rust-lang.org/t/help-getting-started-with-converting-c-project/22370/8)

Thanks to llogiq for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
