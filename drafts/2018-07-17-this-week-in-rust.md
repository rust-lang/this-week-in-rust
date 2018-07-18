Title: This Week in Rust 243
Number: 243
Date: 2018-07-17
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

This week's crate is [paste](https://github.com/jkcclemens/paste), a self-hosted pastebin made of a lot of Rust and a little Ruby. Thanks to [Kyle Clemens](https://users.rust-lang.org/u/jkcclemens) for both crate and suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rustberry: Test Rustberry 0.1 on Raspberry Pi](https://www.reddit.com/r/rust/comments/8x1ayd/calling_all_raspberry_pi_owners_rustberry_010_has/).
* [medium/hard] [rustc-guide: Codegen: LLVM IR, Monomorphization, Codegen Units, Partitioning, Symbol Linkage and Visibility](https://github.com/rust-lang-nursery/rustc-guide/issues/89).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

172 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-09..2018-07-16

* [add the `amdgpu-kernel` ABI](https://github.com/rust-lang/rust/pull/52032)
* [infinite loop detection for const evaluation](https://github.com/rust-lang/rust/pull/51702)
* [chalk lowering rule: WellFormed-TraitRef](https://github.com/rust-lang/rust/pull/50250)
* [fix ICE when using a pointer cast as array size](https://github.com/rust-lang/rust/pull/52314)
* [implement `#[alloc_error_handler]`](https://github.com/rust-lang/rust/pull/52191)
* [improve error message shown for unsafe operations](https://github.com/rust-lang/rust/pull/52207)
* [remove most of `PartialEq` and `Hash` impls from AST and HIR structures](https://github.com/rust-lang/rust/pull/51829)
* [deny bare trait objects in the rest of rust](https://github.com/rust-lang/rust/pull/52302)
* [reach the body of functions returning `impl Trait` but don't treat it as public](https://github.com/rust-lang/rust/pull/52348)
* [NLL: suggest `ref mut` and `&mut self`](https://github.com/rust-lang/rust/pull/52242)
* [resolve: functions introducing procedural macros reserve a slot in the macro namespace as well](https://github.com/rust-lang/rust/pull/52383)
* [proc_macro: fix crate root detection](https://github.com/rust-lang/rust/pull/52328)
* [correct suggestion for println](https://github.com/rust-lang/rust/pull/51614)
* [do not attempt to recompile codegen backend(s) with --keep-stage](https://github.com/rust-lang/rust/pull/52360)
* [openbsd fix](https://github.com/rust-lang/libc/pull/1040)
* [rustc: stabilize the `proc_macro` feature](https://github.com/rust-lang/rust/pull/52081)
* [ensure StorageDead is created even if variable initialization fails](https://github.com/rust-lang/rust/pull/52046)
* [rustc_codegen_llvm: replace the first argument early in FnType::new_vtable](https://github.com/rust-lang/rust/pull/52089)
* [change RangeInclusive to a three-field struct](https://github.com/rust-lang/rust/pull/51622)
* [add ExactChunks::remainder and ExactChunks::into_remainder](https://github.com/rust-lang/rust/pull/51339)
* [implement `Option::replace` in the core library](https://github.com/rust-lang/rust/pull/52003)
* [add `#[repr(transparent)]` to `Atomic*` types](https://github.com/rust-lang/rust/pull/52149)
* [remove sync::Once::call_once 'static bound](https://github.com/rust-lang/rust/pull/52239)
* [improve Debug display for a few types](https://github.com/rust-lang/cargo/pull/5712)
* [cargo: most sorts can be unstable](https://github.com/rust-lang/cargo/pull/5732)
* [implement default-run option to set default binary for cargo run](https://github.com/rust-lang/cargo/pull/5710)
* [rustdoc: don't panic when the cross-re-export handler sees a proc-macro](https://github.com/rust-lang/rust/pull/52361)

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

* [disposition: merge] [Deprecation of `str::slice_unchecked(_mut)`](https://github.com/rust-lang/rust/pull/51807).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [impl Clone for `Box<CStr>`, `Box<OsStr>`, `Box<Path>`](https://github.com/rust-lang/rust/pull/51912).

## New RFCs

* [Minimum Supported Rust Version](https://github.com/rust-lang/rfcs/pull/2495).

# Upcoming Events

### Asia

* [Jul 19. Tokyo, JP - Rust Meetup #9 in Shibuya](https://www.meetup.com/Tokyo-Rust-Meetup/events/252145423/).

### Europe

* [Jul 19. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxkbzb/).
* [Jul 20. Barcelona, ES - BcnRust 1st meetup with Ashley Williams & Steve Klabnik](https://www.meetup.com/es-ES/BcnRust/events/251237895/).
* [Jul 24. Rome, IT - Rust learning and hacking evening #10](https://www.meetup.com/Rust-Roma/events/252627092/).

### North America

* [Jul 12. Utah Valley, Utah, US - Utah Rust - Monthly Meeting](https://www.meetup.com/utahrust/events/251816575/).
* [Jul 18. Orange County, US - Crash Course for Traits and Associated Types](https://www.meetup.com/oc-rust/events/252639183/).
* [Jul 18. Standford, US - Rust Bay Area - [@ Stanford] Munching Macros and Facebook's Mononoke](https://www.meetup.com/Rust-Bay-Area/events/251862242/).
* [Jul 24. Denver, US - Rust Boulder/Denver - Rust Denver July Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/252275279/).
* [Jul 25. Chicago, US - Rust Meetup July 2018](https://www.meetup.com/Chicago-Rust-Meetup/events/251961097/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer at Distil Networks, San Francisco](https://www.distilnetworks.com/job/?id=c2a5db5c-12ce-40f2-949c-48510acf7fa1).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> References are not pointers, but temporary locks on data

– [Kornel](https://users.rust-lang.org/u/kornel) [on rust-users](https://users.rust-lang.org/t/cannot-move-out-of-borrowed-content-take-2/18700/7).

Thanks to [Squirrel](https://users.rust-lang.org/u/gilescope) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
