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

* [Completing the transition to the new borrow checker](https://blog.rust-lang.org/2019/11/01/nll-hard-errors.html).
* [Rust support for Windows Runtime in the works by the author of C++ WinRT](https://kennykerr.ca/2019/11/05/rust/).
* [You probably didn't want `.into_iter().cloned()`](https://www.reddit.com/r/rust/comments/dp3s25/psa_you_probably_didnt_want_into_itercloned/).
* [Clippy is removing its plugin interface](https://blog.rust-lang.org/inside-rust/2019/11/04/Clippy-removes-plugin-interface.html).
* [Rust concurrency patterns: condvars and locks](https://medium.com/@polyglot_factotum/rust-concurrency-patterns-condvars-and-locks-e278f18db74f).
* [How to make your C codebase rusty: rewriting keyboard firmware keymap in Rust](https://about.houqp.me/posts/rusty-c/).
* [When writing a bump allocator, always bump downwards](https://fitzgeraldnick.com/2019/11/01/always-bump-downwards.html).
* [Adventures in motion control: initial motion system](http://adventures.michaelfbryan.com/posts/initial-motion-system/).
* [2019-10-24 compiler team triage meeting](https://blog.rust-lang.org/inside-rust/2019/10/30/compiler-team-meeting.html).

### #Rust2020

Find all #Rust2020 posts at [Read Rust](https://readrust.net/rust-2020/).

# Crate of the Week

This week's crate is [displaydoc](https://github.com/yaahc/displaydoc), a procedural derive macro to implement `Display` by string-interpolating the doc comment.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/652) for the suggesion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Announcing safety-dance: removing unnecessary unsafe code from popular crates](https://github.com/rust-secure-code/safety-dance).
* [RFC: make Cargo embed dependency versions in the compiled binary](https://github.com/rust-lang/rfcs/pull/2801).
* [good first issue] [cargo-sweep: Could cargo-sweep work without rustup](https://github.com/holmgr/cargo-sweep/issues/26)?
* [good first issue] [Rubble: Add a function for reading the device address to rubble-nrf52](https://github.com/jonas-schievink/rubble/issues/89).
* [good first issue] [Rubble: Don't give up when missing the initial transmit window](https://github.com/jonas-schievink/rubble/issues/77).
* [good first issue] [Rubble: LLCP updates are not applied when the event is missed](https://github.com/jonas-schievink/rubble/issues/74).
* [good first issue] [Rubble: Log buffer overflow on nrf52832](https://github.com/jonas-schievink/rubble/issues/69).
* [good first issue] [Rubble: Try out `scroll` or `zerocopy` for de/encoding of PDUs](https://github.com/jonas-schievink/rubble/issues/53).
* [good first issue] [Rubble: Only reply to LL_VERSION_IND once](https://github.com/jonas-schievink/rubble/issues/49).

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

* [disposition: merge] [Announcing the FFI-unwinding Project Group](https://github.com/rust-lang/rfcs/pull/2797).
* [disposition: postpone] [Signing registry index commits](https://github.com/rust-lang/rfcs/pull/2474).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize --extern flag without a path](https://github.com/rust-lang/rust/pull/64882).
* [disposition: merge] [Fully integrate derive helpers into name resolution](https://github.com/rust-lang/rust/pull/64694).
* [disposition: merge] [Make the semantics of Vec::truncate(N) consistent with slices](https://github.com/rust-lang/rust/pull/64432).
* [disposition: merge] [Use ptr::drop_in_place for VecDeque::truncate and VecDeque::clear](https://github.com/rust-lang/rust/pull/65933).

## New RFCs

* [Add method Result::into_ok](https://github.com/rust-lang/rfcs/pull/2799).
* [Make Cargo embed dependency versions in the compiled binary](https://github.com/rust-lang/rfcs/pull/2801).
* [Vec::recycle](https://github.com/rust-lang/rfcs/pull/2802).
* [Target tier policy](https://github.com/rust-lang/rfcs/pull/2803).
* [[T]::rejoin](https://github.com/rust-lang/rfcs/pull/2806).

# Upcoming Events

### Asia Pacific

* [Nov 13. Selangor, MY - Rust Malaysia Meetup November 2019](https://docs.google.com/forms/d/e/1FAIpQLSfZM9XYmBXq9tjqRziR-O3vBmm4rt1Ltnc9bGcleVrLmZHrSg/viewform).

### Europe

* **[Nov 9 & 10. Barcelona, ES - RustFest Barcelona 2019](https://barcelona.rustfest.eu/).**
* [Nov 12. Hamburg, DE - Rust Hack & Learn November 2019](https://www.meetup.com/Rust-Meetup-Hamburg/events/265899865/).
* [Nov 13. Wrocław, PL - Rust Wrocław Meetup #14](https://www.meetup.com/Rust-Wroclaw/events/265813648/).
* [Nov 13. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryzpbrb/).
* [Nov 14. Zurich, CH - Rust Zurich - RustFest Decompression Zürich](https://www.meetup.com/Rust-Zurich/events/265593126/).
* [Nov 14. Moscow, RU - Rust Moscow November 2019 Meetup](https://www.meetup.com/ru-RU/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/266184946/).
* [Nov 15. Barcelona, ES - Rust GTK/GStreamer Workshop at Linux Application Summit 2019](https://www.meetup.com/Barcelona-Free-Software/events/265596417/).
* [Nov 21. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/265961100).

### North America

* [Nov 12. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdryzpbqb/).
* [Nov 13. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgryzpbrb/).
* [Nov 13. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryzpbrb/).
* [Nov 14. San Diego, CA, US - San Diego Rust November Meetup](https://www.meetup.com/San-Diego-Rust/events/265981542/).
* [Nov 14. Lehi, UT, US - Utah Rust - November 2019 Regular Meetup](https://www.meetup.com/utah-rust/events/265905259/).
* [Nov 14. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgryzpbsb/).
* [Nov 14. Montreal, QC, CA - Montreal Rust Meetup - November 2019 RustMTL: November Common Traits & Causal Profiling](https://www.meetup.com/Rust-Montreal/events/prvrjryzpbqb/).
* [Nov 20. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/265998640/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Data Analysis Software Engineer at Swift Navigation, San Francisco, US](https://www.swiftnav.com/join-us/jobs-listing?gh_jid=4369805002).
* [Rust/Core Developer at Parity, Berlin, DE (Remote available)](https://www.parity.io/jobs/#undefined-rust-core-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I did manage to get this compile in the end - does anyone else find that the process of asking the question well on a public forum organizes their thoughts well enough to solve the problem?

– [David Mason on rust-users](https://users.rust-lang.org/t/std-phantomdata-and-unused-fields-in-structs/34271/3)

Thanks to [Daniel H-M](https://users.rust-lang.org/t/twir-quote-of-the-week/328/725) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
