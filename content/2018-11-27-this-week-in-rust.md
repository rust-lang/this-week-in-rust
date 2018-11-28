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

* [Announcing Rust 2018 Beta release](https://internals.rust-lang.org/t/announcing-rust-2018-beta-release/8901).
* [Announcing Firecracker from Amazon: MicroVM for serverless computing](https://aws.amazon.com/blogs/opensource/firecracker-open-source-secure-fast-microvm-serverless/).
* [Rust survey 2018 results](https://blog.rust-lang.org/2018/11/27/Rust-survey-2018.html).
* [Getting started with nightly async/await support](https://jsdw.me/posts/rust-asyncawait-preview/).
* [Converting AsyncRead and AsyncWrite to Futures, Sinks, and Streams](https://jsdw.me/posts/rust-futures-tokio/).
* [Bootstrapping an embedded Rust development environment](https://josh.robsonchase.com/embedded-bootstrapping/).
* [Tide's evolving middleware approach](https://rust-lang-nursery.github.io/wg-net/2018/11/27/tide-middleware-evolution.html).
* [Rust traits and their (lack of) privacy](https://phaazon.net/blog/rust-traits-privacy).
* [Serverless HTTP](https://medium.com/@softprops/serverless-http-9a58f9b2df60).
* [Generic methods in Rust: How Exonum shifted from Iron to Actix-web](https://medium.com/meetbitfury/generic-methods-in-rust-how-exonum-shifted-from-iron-to-actix-web-7a2752171388).
* [Rust+GNOME Hackfest 4](http://antoyo.ml/rust-gnome-hackfest-thessaloniki).
* [Amethyst Foundation has been formed](https://www.amethyst.rs/blog/non-profit-announce/).
* [Rust language cheat sheet (cheats.rs)](https://cheats.rs).
* [Rust Latam CFP is now open, deadline is December 31st](https://cfp.rustlatam.org/events/rust-latam). Also [ticket sales are open](https://rustlatam.org/#tickets).
* [Videos from Rust Belt Rust 2018 are now available](https://www.youtube.com/playlist?list=PLgC1L0fKd7UlpVTHVfLYVtudVx8CzbSxW).
* [Videos from Rustfest 2018 Rome](https://media.ccc.de/c/rustfest18rome).

# Crate of the Week

This week's crate is [modulator](https://crates.io/crates/modulator), a crate of abstract modulators for use in audio synthesizers (and possibly elsewhere). Thanks to [Andrea Pessino](https://twitter.com/AndreaPessino/status/1065409923553517568) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust Latam CFP is now open, deadline is December 31st](https://cfp.rustlatam.org/events/rust-latam).

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
* [cargo: allow `crate_type=bin` examples to run](https://github.com/rust-lang/cargo/pull/6330)

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

* [disposition: merge] [[Stabilization] Stablize using some arbitrary self types defined in std](https://github.com/rust-lang/rust/issues/55786).
* [disposition: merge] [Wxperiment: Support aliasing local crate root in extern prelude](https://github.com/rust-lang/rust/pull/55275).
* [disposition: merge] [Refiling "#[repr(simd)] struct(isize, isize) not allowed"](https://github.com/rust-lang/rust/issues/55078).
* [disposition: merge] [Tracking issue for inclusion of `derive` in lint `unused_attributes`](https://github.com/rust-lang/rust/issues/54651).
* [disposition: merge] [Tracking issue for RFC 2361, "Simpler alternative dbg!() macro"](https://github.com/rust-lang/rust/issues/54306).
* [disposition: merge] [Tracking issue for RFC 2302, Tuple struct construction with `Self(v1, v2, ..)`](https://github.com/rust-lang/rust/issues/51994).
* [disposition: merge] [tracking issue for `?` macro repetition](https://github.com/rust-lang/rust/issues/48075).

## New RFCs

* [Symbol Mangling v2](https://github.com/rust-lang/rfcs/pull/2603).
* [#[attribute]s galore](https://github.com/rust-lang/rfcs/pull/2602).
* [Multiple Attributes in an Attribute Container](https://github.com/rust-lang/rfcs/pull/2600).

# Upcoming Events

### Online

* [Dec  3. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Dec  5. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Dec 12. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Dec  5. Johannesburg, SA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/jdqplqyxqbhb/).

### Asia

* [Dec  6. Pune, IN - Rust workshop at Pune, India](https://reps.mozilla.org/e/rust-community-meetup-pune/).
* [Dec 12. Hangzhou, CN - Rust Hangzhou](https://www.meetup.com/Rust-Hangzhou/events/256338781/).

### Europe

* [Nov 29. Copenhagen, DK - Copenhagen Rust Group - Hack Night #11](http://cph.rs/).
* [Dec  3. Karlsruhe, DE - Rust 2018 Edition](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/256200841/?_xtd=gqFyqTE5MzgwNjQ5OKFwp2FuZHJvaWQ&from=ref).
* [Dec  5. Cologne, DE - Cologne meetup](https://rust.cologne/2018/12/05/rust-2018-eve.html).
* [Dec  6. Gouda, NL - Rust Gouda - Rust 2018 edition release party](https://www.meetup.com/Rust-Gouda/events/254877742/).
* [Dec 10. Vienna, AT - Metalab - Rust Workshop](https://metalab.at/wiki/Rust-Workshop).
* [Dec 11. Zurich, CH - Rust Zurich - Rust Embedded Edition 2018](https://www.meetup.com/Rust-Zurich/events/255279763/).
* [Dec 12. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxqbqb/).
* [Dec 15 & 16. Moscow, RU - RustRush 2018](https://rustrush.ru).

### North America

* [Dec  2. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbdb/).
* [Dec  5. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyxqbhb/).
* [Dec  5. Indianopolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxqbhb/).
* [Dec  6. Phoenix, US - Phoenix 2018 Edition Release Party](https://www.meetup.com/Desert-Rustaceans/events/256503618).
* [Dec  9. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbmb/).
* [Dec 10. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxqbnb/).
* [Dec 12. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyxqbqb/).
* [Dec 13. Arlington, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/256181658).
* [Dec 13. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxqbrb/).
* [Dec 13. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209738/).
* [Dec 13. San Diego, US - San Diego Rust December Meetup - Rust 2018 Overview + Memory Allocator](https://www.meetup.com/San-Diego-Rust/events/256264465/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Layer1 Capital, San Francisco, US](https://angel.co/layer1-capital/jobs/459718-rust-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> "I did not want to inflict memory management on my son" – @M_a_s_s_i

– Massimiliano Mantione [during his RustFest talk](https://twitter.com/RustFest/status/1058302698834087936)

Thanks to llogiq for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/a16w4l/this_week_in_rust_262/).</small>
