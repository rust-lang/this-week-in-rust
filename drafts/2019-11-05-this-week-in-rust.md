Title: This Week in Rust 311
Number: 311
Date: 2019-11-05
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

This week's crate is [displaydoc](https://github.com/yaahc/displaydoc), a procedural derive macro to implement `Display` by string-interpolating the doc comment.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/652) for the suggesion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [good first issue] [async-std: use once_cell instead of lazy_static](https://github.com/async-rs/async-std/issues/406).
* [good first issue] [async-std: Add Future::flatten](https://github.com/async-rs/async-std/issues/404).
* [good first issue] [async-std: Add stream::from_iter](https://github.com/async-rs/async-std/issues/400).
* [good first issue] [async-std: TCP smoke testing](https://github.com/async-rs/async-std/issues/407).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

217 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-10-28..2019-11-04

* [Allow foreign exceptions to unwind through Rust code and Rust panics to unwind through FFI](https://github.com/rust-lang/rust/pull/65646)
* [expand: Feature gate out-of-line modules in proc macro input](https://github.com/rust-lang/rust/pull/66078)
* [Lint ignored `#[inline]` on function prototypes](https://github.com/rust-lang/rust/pull/65294)
* [Improve the "try using a variant of the expected type" hint](https://github.com/rust-lang/rust/pull/65562)
* [Use heuristics to recover parsing of missing `;`](https://github.com/rust-lang/rust/pull/65640)
* [Point at local similarly named element and tweak references to variants](https://github.com/rust-lang/rust/pull/65421)
* [Custom lifetime error for `impl` item doesn't conform to `trait`](https://github.com/rust-lang/rust/pull/65068)
* [Add lint and tests for unnecessary parens around types](https://github.com/rust-lang/rust/pull/65112)
* [Correct handling of type flags with `ConstValue::Placeholder`](https://github.com/rust-lang/rust/pull/65643)
* [Use structured suggestion for unnecessary bounds in type aliases](https://github.com/rust-lang/rust/pull/65914)
* [save-analysis: Account for async desugaring in async fn return types](https://github.com/rust-lang/rust/pull/65936)
* [Switch CrateMetadata's source_map_import_info from RwLock to Once](https://github.com/rust-lang/rust/pull/65979)
* [Don't use `eval_always` for miri queries used from codegen](https://github.com/rust-lang/rust/pull/65927)
* [rustc: use IndexVec<DefIndex, T> instead of Vec<T>](https://github.com/rust-lang/rust/pull/65825)
* [Make `promote_consts` emit the errors when required promotion fails](https://github.com/rust-lang/rust/pull/65946)
* [Implement ordered/sorted iterators on `BinaryHeap`](https://github.com/rust-lang/rust/pull/65091)
* [Make `*`{`const`, `mut`} `T>::offset_from` const fn](https://github.com/rust-lang/rust/pull/63810)
* [Stabilize `float_to_from_bytes` feature](https://github.com/rust-lang/rust/pull/66002)
* [hashbrown: Introduce `ahash-compile-time-rng` feature](https://github.com/rust-lang/hashbrown/pull/125)
* [cargo: Add --filter-platform to `cargo metadata`](https://github.com/rust-lang/cargo/pull/7376)
* [cargo: Fix `cargo fix` not showing colors](https://github.com/rust-lang/cargo/pull/7550)
* [chalk: Remove delayed literals](https://github.com/rust-lang/chalk/pull/270)
* [chalk: Add TypeName::Error variant](https://github.com/rust-lang/chalk/pull/269)
* [chalk: Output multiple solutions](https://github.com/rust-lang/chalk/pull/263)
* [rustdoc: Stabilize `cfg(doctest)`](https://github.com/rust-lang/rust/pull/63803)

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

* [disposition: postpone] [Signing registry index commits](https://github.com/rust-lang/rfcs/pull/2474).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize --extern flag without a path](https://github.com/rust-lang/rust/pull/64882).
* [disposition: merge] [Stabilize cfg(doctest)](https://github.com/rust-lang/rust/pull/63803).
* [disposition: merge] [Fix accidental stabilization in feature-detection macros](https://github.com/rust-lang/rust/pull/64534).
* [disposition: merge] [Implement @argsfile to read arguments from command line](https://github.com/rust-lang/rust/issues/63576).
* [disposition: merge] [Tracking issue for todo! macro](https://github.com/rust-lang/rust/issues/59277).
* [disposition: merge] [Tracking issue for floats ↔ bytes conversions](https://github.com/rust-lang/rust/issues/60446).
* [disposition: merge] [Under what conditions can you implement Copy for a union](https://github.com/rust-lang/rust/issues/65748)?

## New RFCs

* [Propose implicit named arguments for formatting macros](https://github.com/rust-lang/rfcs/pull/2795).
* [Announcing the FFI-unwinding Project Group](https://github.com/rust-lang/rfcs/pull/2797).
* [Environment variable sandboxing](https://github.com/rust-lang/rfcs/pull/2794).
* [Add `[T]::as_ptr_range()`](https://github.com/rust-lang/rfcs/pull/2791).
* [Subslice-offset - Get the offset of references into a slice](https://github.com/rust-lang/rfcs/pull/2796).

# Upcoming Events

### Africa

* [Nov  6. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzpbjb/).

### Asia Pacific

* [Nov  4. Auckland, NZ - Rust AKL - Introduction to Rust (part 2 of 3)](https://www.meetup.com/rust-akl/events/259481269/).
* [Nov 13. TBD, MY - Rust Malaysia Meetup November 2019](https://docs.google.com/forms/d/e/1FAIpQLSfZM9XYmBXq9tjqRziR-O3vBmm4rt1Ltnc9bGcleVrLmZHrSg/viewform).

### Europe

* [Nov 2. Lviv, UA - Peer Lab Lviv #Rust: Introduction](https://t.me/peerlab_lviv_rust/135).
* [Nov 2. Kharkiv, UA - Peer Lab Kharkiv #Rust: Command-Line Applications in Rust](https://www.facebook.com/events/689432161466405/).
* **[Nov 9 & 10. Barcelona, ES - RustFest Barcelona 2019](https://barcelona.rustfest.eu/).**
* [Nov 12. Hamburg, DE - Rust Hack & Learn November 2019](https://www.meetup.com/Rust-Meetup-Hamburg/events/265899865/).
* [Nov 13. Wrocław, PL - Rust Wrocław Meetup #14](https://www.meetup.com/Rust-Wroclaw/events/265813648/).
* [Nov 13. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzpbrb/).
* [Nov 14. Zurich, CH - Rust Zurich - RustFest Decompression Zürich](https://www.meetup.com/Rust-Zurich/events/265593126/).
* [Nov 15. Barcelona, ES - Rust GTK/GStreamer Workshop at Linux Application Summit 2019](https://www.meetup.com/Barcelona-Free-Software/events/265596417/).

### North America

* [Nov  6. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzpbjb/).
* [Nov 13. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgryzpbrb/).
* [Nov 12. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdryzpbqb/).
* [Nov 13. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzpbrb/).
* [Nov 14. San Diego, CA, US - San Diego Rust November Meetup](https://www.meetup.com/San-Diego-Rust/events/265981542/).
* [Nov 14. Lehi, UT, US - Utah Rust - November 2019 Regular Meetup](https://www.meetup.com/utah-rust/events/265905259/).
* [Nov 14. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzpbsb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I did manage to get this compile in the end - does anyone else find that the process of asking the question well on a public forum organizes their thoughts well enough to solve the problem?

– [David Mason on rust-users](https://users.rust-lang.org/t/std-phantomdata-and-unused-fields-in-structs/34271/3)

Thanks to [Daniel H-M](https://users.rust-lang.org/t/twir-quote-of-the-week/328/725) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
