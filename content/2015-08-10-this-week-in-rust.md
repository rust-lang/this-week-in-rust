Title: This Week in Rust 91
Date: 2015-08-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# From the Blogosphere

* 🎈🎉 [Announcing Rust 1.2](http://blog.rust-lang.org/2015/08/06/Rust-1.2.html). 🎉🎈
* [Writing Unsafe and Low-Level Code in Rust](http://smallcultfollowing.com/rust-int-variations/imem-umem/guide-unsafe.html).
* [Creating a PHP Extension to Rust](http://hermanradtke.com/2015/08/03/creating-a-php-extension-to-rust.html).
* [Objective-C from Rust: objc_msgSend](http://sasheldon.com/blog/2015/08/02/objective-c-from-rust-objc_msgsend/).
* [A Simple Web App in Rust, Part 4 -- CLI Option Parsing](https://joelmccracken.github.io/entries/a-simple-web-app-in-rust-pt-4--cli-option-parsing/).
* [video] [The Rust Programming Language](https://youtu.be/d1uraoHM8Gg) from Google TechTalks.

# New Releases & Project Updates

* Check out the new unofficial [German-language Rust forums](http://forum.rustplatz.de/).
* [rust-cpp](https://github.com/mystor/rust-cpp). Embed C++ directly inside your rust code.
* [rustty](https://github.com/cpjreynolds/rustty). A terminal UI library.
* [coalesce-rs](https://github.com/arcnmx/coalesce-rs). Combine disjoint types that share common traits.
* [mdBook](https://github.com/azerupi/mdBook). Create a book from markdown files.
* [Serde 0.5.0 adds support for bincode](https://erickt.github.io/blog/2015/08/07/serde-0-dot-5-0-many-many-changes/).
* [Cargo Crusader 0.1](https://users.rust-lang.org/t/cargo-crusader-0-1-test-the-downstream-impact-of-rust-crate-changes-before-publishing/2373/5). A tool for validating changes to APIs prior to publishing.

# New Contributors

* Ivan Jager
* Jan Likar
* Marko Lalic
* Matt Friedman
* Mike Marcacci
* Ruby
* Tim Neumann

# Subteam reports

Every week [The Rust Team](http://www.rust-lang.org/team.html) release
a report on what is going on in their corner of the project. Here are
the highlights from [this week's report](https://internals.rust-lang.org/t/subteam-reports-2015-08-07/2473).

## Libs team

[Full report](https://github.com/rust-lang/subteams/blob/master/libs/reports/2015-08-07.md)

First off, we had a great RustCamp last weekend! As many of us were
involved with the event, it was a bit of a slow week.

Decisions from last week:

- [RFC PR #1184](https://github.com/rust-lang/rfcs/pull/1184):
  RFC: Stabilize the #![no_std] attribute
  - Merged
- [RFC PR #1183](https://github.com/rust-lang/rfcs/pull/1183):
  RFC: Allow changing the default allocator
  - Merged
- [RFC PR #770](https://github.com/rust-lang/rfcs/pull/770):
  io error handling design
  - Closed
- [RFC PR #980](https://github.com/rust-lang/rfcs/pull/980):
  read_exact
  - Merged
- [RFC PR #1194](https://github.com/rust-lang/rfcs/pull/1194):
  RFC: Add item recovery collection APIs
  - To be merged, pending last bits of bikeshedding
- [PR #26818](https://github.com/rust-lang/rust/pull/26818):
  Stabilize duration (with renamings)
  - Merge, need to decide about 1.3 backport

We'd like to call attention to the following two policy RFCs:

- [PR #1242](https://github.com/rust-lang/rfcs/pull/1242):
  RFC: policy for rust-lang crates
- [PR #1224](https://github.com/rust-lang/rfcs/pull/1224):
  Update the RFC process with sub-teams, amongst other things.

as well as an RFC relating to `catch_panic` and exception safety in Rust:

- [PR #1236](https://github.com/rust-lang/rfcs/pull/1236):
  RFC: Stabilize catch_panic

This week's RFCs going into (or staying in) **final comment period**:

- [FCP PR #1195](https://github.com/rust-lang/rfcs/pull/1195):
  ordered query API
- [FCP PR #1192](https://github.com/rust-lang/rfcs/pull/1192):
  RFC for inclusive ranges with ...

## Lang team

[Full report](https://github.com/rust-lang/subteams/blob/master/lang/reports/2015-08-07.md)

The following RFCs are being promoted to **final comment period**:

- [PR #886](https://github.com/rust-lang/rfcs/pull/886): Permit
  `#[must_use]` attributes on functions as well as types. This allows
  for functions whose return value should not be ignored even if the
  type of that value is unexceptional (e.g., the `ok()` method of
  `Result`).
- [PR #890](https://github.com/rust-lang/rfcs/pull/890): Custom
  preludes.  This RFC proposes an extension that allows crates to
  define their own preludes. This can be used to have common names
  available throughout a crate without forcing them to be explicitly
  imported everywhere. This is particularly useful when combined with
  the convention of having external crates define a `pub mod prelude`
  that downstream crates can import into their own local
  preludes. While clearly convenient, there were some concerns raised
  that this will give rise to multiple dialects of Rust.
- [PR #953](https://github.com/rust-lang/rfcs/pull/953): This defines
  traits to support `+=` and other operators, closing a gap in our
  operator overloading support. The traits take the LHS via an `&mut`
  reference to permit in-place updates, take the RHS by value, and do
  not require that the `Add` trait also be implemented.
- [PR #1135](https://github.com/rust-lang/rfcs/pull/1135): This PR
  permits raw fat pointers (e.g., `*[i32]` or `*Trait`) to be
  compared, just like raw thin pointer (e.g., `*i32`). The semantics
  are to compare both the pointer itself *and* any accompanying data
  (e.g., the length of the slice).
- [PR #1189](https://github.com/rust-lang/rfcs/pull/1189): This PR
  simply corrects typos. 

The following two RFCs have been accepted:

- [PR #1214](https://github.com/rust-lang/rfcs/pull/1214): Clarify
  (and improve) rules for projections and well-formedness.
- [PR #1219](https://github.com/rust-lang/rfcs/pull/1219): Allow
  aliasing imports when importing as a group.
  
In addition, I would like to call attention to the following RFC:

- [PR #1238](https://github.com/rust-lang/rfcs/pull/1238):
  Nonparametric dropck. This RFC simplifies the dropck rules to close
  some soundness holes and make room for specialization. The change is
  expected to cause little to no breakage in practice, e.g., a crater
  run found no affected crates, but it nonetheless affects a core
  component of the language.
  
## Compiler team

[Full report](https://github.com/rust-lang/subteams/blob/master/compiler/reports/2015-08-07.md)

@arielb1 opened
[PR #27551](https://github.com/rust-lang/rust/pull/27551), which
changes how structs and enums are represented in the compiler,
replacing various hashtables with a single `AdtDef` struct. This is a
reimplementation of a similar PR by @aatch. In addition to cleaner
code, it results in a small performance boost (approximately 5%).

There has been some progress towards removing drop flags. @pnkfelix
landed his "nonzeroing move hints" branch
([PR #26173](https://github.com/rust-lang/rust/pull/26173)). Unfortunately,
some critical bugs were found shortly thereafter. The fix
([PR #27413](https://github.com/rust-lang/rust/pull/27413)) is not yet
ready.

# Upcoming Events

* [8/10. Seattle](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [8/11. San Diego](http://www.meetup.com/San-Diego-Rust/events/223766853/).
* [8/18. Sydney](http://www.meetup.com/Rust-Sydney/).
* [8/19. Los Angeles](http://www.meetup.com/Rust-Los-Angeles/events/224231575/).
* [8/20. Berlin](http://www.meetup.com/Rust-Berlin/events/224141638/).
* [8/26. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [8/31. Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

There are some jobs writing Rust! This week's listings:

* Student Research Assistant in Karlsruhe, Germany for embedded development on ARM stm32. Contact [Oliver Schneider][oli_obk]

[oli_obk]: mailto:oliver.schneider@kit.edu

# Quote of the Week

    <bluss> I've tried using unchecked indexing in non-trivial code now a couple of times. It never makes a big difference
    <bluss> Profiling shows like 1-2% improvement if that
    <bluss> so it's the tightest loops you should worry about, not much more

@bluss knows a few things about micro-optimization.

Thanks to @bluss for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
