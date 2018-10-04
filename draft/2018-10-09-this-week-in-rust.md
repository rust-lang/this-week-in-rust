Title: This Week in Rust 255
Number: 255
Date: 2018-10-09
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

This week's crate is [Evcxr](https://github.com/google/evcxr), a Rust REPL and Rust Jupyter Kernel. Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/457) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [winapi: Take advantage of the new Class trait](https://github.com/retep998/winapi-rs/issues/681).
* [Crater now has a contributing guide and a few issues with mentoring instructions](https://github.com/rust-lang-nursery/crater/blob/master/CONTRIBUTING.md).
* [Quinn has some good first issues, listed here](https://github.com/djc/quinn/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

114 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-09-24..2018-10-01

* [Stabilize pattern_parentheses feature](https://github.com/rust-lang/rust/pull/54497).
* [Support an explicit annotation for marker traits](https://github.com/rust-lang/rust/pull/53693).
* [Better user experience when attempting to call associated functions with dot notation](https://github.com/rust-lang/rust/pull/54308).
* [Add "temporary value borrowed for too long" error](https://github.com/rust-lang/rust/pull/54164).
* [Remove `-Z disable_ast_check_for_mutation_in_guard`](https://github.com/rust-lang/rust/pull/54676).
* [Add a `-C default-linker-libraries` option](https://github.com/rust-lang/rust/pull/54675).
* [Implement the `dbg!(..)` macro](https://github.com/rust-lang/rust/pull/54317).
* [Add -Z emit-stack-sizes](https://github.com/rust-lang/rust/pull/51946).
* [`impl trait` in bindings (feature: impl-trait-existential-types](https://github.com/rust-lang/rust/pull/53542).
* [Make "await" a pseudo-edition keyword](https://github.com/rust-lang/rust/pull/54411).
* [Use full name to identify a macro in a `FileName`](https://github.com/rust-lang/rust/pull/54338).
* [Introduce the partition_dedup/by/by_key methods for slices](https://github.com/rust-lang/rust/pull/54058).
* [Rework how we handle outlives relationships](https://github.com/rust-lang/rust/pull/54453).
* [NLL: Get Polonius borrow check to work in simple cases](https://github.com/rust-lang/rust/pull/54468).
* [NLL: Be more permissive when checking access due to Match](https://github.com/rust-lang/rust/pull/53438).
* [NLL: Rework checking for borrows conflicting with drops](https://github.com/rust-lang/rust/pull/54509).
* [Don't lint non-extern-prelude extern crate's in Rust 2018](https://github.com/rust-lang/rust/pull/54650).
* [Deny the `overflowing_literals` lint for the 2018 edition](https://github.com/rust-lang/rust/pull/54507).
* [Rename slice::exact_chunks() to slice::chunks_exact()](https://github.com/rust-lang/rust/pull/54537).
* [Avoid loading constructor attributes in AdtDef decoding](https://github.com/rust-lang/rust/pull/54485).
* [Panic when using mem::uninitialized or mem::zeroed on an uninhabited type](https://github.com/rust-lang/rust/pull/54667).
* [Improvements to finding LLVM's FileCheck](https://github.com/rust-lang/rust/pull/54558).
* [In which we include attributes in unused `extern crate` suggestion spans](https://github.com/rust-lang/rust/pull/54488).
* [In which inferable outlives-requirements are linted](https://github.com/rust-lang/rust/pull/53013).
* [Add a per-tree error cache to the obligation forest](https://github.com/rust-lang/rust/pull/53255).

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

* [disposition: merge] [`#[cfg_attr]` expanding to multiple attributes](https://github.com/rust-lang/rfcs/pull/2539).
* [disposition: merge] [Or patterns, i.e `Foo(Bar(x) | Baz(x))`](https://github.com/rust-lang/rfcs/pull/2535).
* [disposition: merge] [Support underscores as constant names](https://github.com/rust-lang/rfcs/pull/2526).
* [disposition: merge] [The optimize attribute](https://github.com/rust-lang/rfcs/pull/2412).
* [disposition: close] [Prior doc comments](https://github.com/rust-lang/rfcs/pull/2374).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for RFC 2086: Allow Irrefutable Patterns in if-let and while-let statements](https://github.com/rust-lang/rust/issues/44495).

## New RFCs

* [Add bree-range-by](https://github.com/rust-lang/rfcs/pull/2553).

# Upcoming Events

### Online

* [Oct 9. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Oct 10. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Oct 17. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Asia

* [Oct 13. Bangalore, IN - Flat Buffers: What and How](https://www.meetup.com/rustox/events/254812229/).

### Europe

* [Oct 8. Rome, IT - Rust Rome Meetup](https://www.meetup.com/it-IT/Rust-Roma/events/255137175/).
* [Oct 9. Amsterdam, NL - Amsterdam Rust - Intro workshop & Hack night](https://www.meetup.com/Rust-Amsterdam/events/254791434/).
* [Oct 17. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/255020858/).
* [Oct 18. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxnbxb/).
* [Oct 25. Wroclaw, PL - Rust Wroclaw Meetup](https://www.meetup.com/Rust-Wroclaw/events/255053694/).

### North America

* [Oct 7. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnbkb/).
* [Oct 8. Seattle, US  - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxnblb/).
* [Oct 11. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxnbpb/).
* [Oct 11. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209633/).
* [Oct 14. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnbsb/).
* [Oct 17. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/dqldspyxnbwb/).
* **[Oct 19 & 20. Ann Arbor, US - Rust Belt Rust 2018](https://rust-belt-rust.com/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Compilers & distributed systems engineers in Australia](https://www.reddit.com/r/rust/comments/9kx94z/job_compilers_distributed_systems_engineers_in/).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
