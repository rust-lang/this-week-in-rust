Title: This Week in Rust 310
Number: 310
Date: 2019-10-29
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

* [A call for blogs 2020](https://blog.rust-lang.org/2019/10/29/A-call-for-blogs-2020.html).
* [Why async fn in traits are hard](https://smallcultfollowing.com/babysteps/blog/2019/10/26/async-fn-in-traits-are-hard/).
* [Spark implemented in Rust with promising results](https://medium.com/@rajasekar3eg/fastspark-a-new-fast-native-implementation-of-spark-from-scratch-368373a29a5c).
* [Nannou awarded Mozilla grant for foundational audio development in Rust](https://nannou.cc/posts/moss_grant_announce).
* [Rust and C++ on floating-point intensive code](https://www.reidatcheson.com/hpc/architecture/performance/rust/c++/2019/10/19/measure-cache.html).
* [docs.rs outage postmortem](https://blog.rust-lang.org/inside-rust/2019/10/24/docsrs-outage-postmortem.html).
* [Cost of rust-analyzer](https://rust-analyzer.github.io/2019/10/16/finance.html).
* [Building a widget for Druid (a WIP native Rust GUI toolkit)](https://pauljmiller.com/posts/druid-widget-tutorial.html).
* [Ferris Fencing - a Rust game built on a RISC-V VM](http://www.ferrisfencing.org/).
* [Technique for doing specialization on a stable compiler safely](https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md).
* [A closer look at Ownership in Rust](https://blog.thoughtram.io/ownership-in-rust/).
* [Programming Servo: the incredibly shrinking timer](https://medium.com/programming-servo/programming-servo-the-incredibly-shrinking-timer-7283ae2a2669).
* [Making good On momo's compile-time promise](https://llogiq.github.io/2019/10/28/momo-fast.html).
* [Adventures in motion control: initial motion system](http://adventures.michaelfbryan.com/posts/initial-motion-system/).
* [Please welcome pnkfelix as compiler team co-lead](https://blog.rust-lang.org/inside-rust/2019/10/24/pnkfelix-compiler-team-co-lead.html).

# Crate of the Week

Sadly, there was no nomination for crate of the week.

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

347 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-10-21..2019-10-28

* [Don't ICE for completely unexpandable `impl Trait` types](https://github.com/rust-lang/rust/pull/65777)
* [Eliminate `intersect_opt`](https://github.com/rust-lang/rust/pull/65648)
* [Derive `Rustc{En,De}codable` for `TokenStream`](https://github.com/rust-lang/rust/pull/65641)
* [Forbid non-`structural_match` types in const generics](https://github.com/rust-lang/rust/pull/65627)
* [Turn crate store into a resolver output](https://github.com/rust-lang/rust/pull/65625)
* [Simplify chalk-engine a bit](https://github.com/rust-lang/chalk/pull/264)
* [Use heuristics to suggest assignment](https://github.com/rust-lang/rust/pull/65566)
* [Point at associated type for some obligations](https://github.com/rust-lang/rust/pull/65288)
* [Lockless `LintStore`](https://github.com/rust-lang/rust/pull/65193)
* [Remove unnecessary trait bounds and derivations](https://github.com/rust-lang/rust/pull/65647)
* [Change untagged_unions to not allow union fields with drop](https://github.com/rust-lang/rust/pull/62330)
* [miri: Add `write_bytes` method to Memory doing bounds-checks and supporting iterators](https://github.com/rust-lang/rust/pull/65621)
* [Object safe for dispatch](https://github.com/rust-lang/rust/pull/57545)
* [Fix WASI sleep impl](https://github.com/rust-lang/rust/pull/65617)
* [Stabilize `const_constructor`](https://github.com/rust-lang/rust/pull/65188)
* [Stabilize `Option::flatten`](https://github.com/rust-lang/rust/pull/64747)
* [Stabilize `#[non_exhaustive]`](https://github.com/rust-lang/rust/pull/64639) (RFC 2008)
* [Make `is_power_of_two` a const function](https://github.com/rust-lang/rust/pull/65092)
* [Add by-value iterator for arrays](https://github.com/rust-lang/rust/pull/62959)
* [Add `Cow::`{`is_borrowed`, `is_owned`}`()`](https://github.com/rust-lang/rust/pull/65144)
* [Add `[_]::`{`as_ptr_range`, `as_mut_ptr_range`}`()`](https://github.com/rust-lang/rust/pull/65806)
* [Add {`String`, `Vec`}`::into_raw_parts()`](https://github.com/rust-lang/rust/pull/65705)
* [Add the `matches!( $expr, $pat ) -> bool` macro](https://github.com/rust-lang/rust/pull/65479)
* [Relax `ExactSizeIterator` bound on `write_bytes`](https://github.com/rust-lang/rust/pull/65704)
* [rustdoc: Forward `-Z` options to rustc](https://github.com/rust-lang/rust/pull/65314)

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

* [Embedded Software Engineer at Sense, Cambridge, MA, US](https://apply.workable.com/sense/j/ADDB5A7717/).
* [Rust Intern at RUDDER, Paris, FR](https://taleez.com/apply/74t9em).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> …man, starting to dig through the source code of a really large open source program is so weird. It’s like wandering around a giant cathedral that’s being constantly renovated and repaired and maintained over the course of years by a giant team of invisible crafters and architects, who mostly communicate via notes and designs pinned to the walls in various places.

– [icefoxen on their wiki](https://wiki.alopex.li/WhereRustcSpendsItsTime)

Thanks to [Ralf Jung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/717) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
