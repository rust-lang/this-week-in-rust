Title: This Week in Rust 242
Number: 242
Date: 2018-07-10
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

* 🎈🎉 [Announcing Rust 1.27.1](https://blog.rust-lang.org/2018/07/10/Rust-1.27.1.html). 🎉🎈
* [Use of unsafe in actix-web codebase is down to <15 from 100+](https://www.reddit.com/r/rust/comments/8wlkbe/actixweb_has_removed_all_unsound_use_of_unsafe_in/).
* [Rust + actix-web in one of the biggest music festival Atlas Weekend](https://www.reddit.com/r/rust/comments/8xdsx5/rust_actixweb_in_the_on_of_the_biggest_music/).
* [A web application completely written in Rust](https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471).
* [Debian is starting to package Rust crates](https://www.reddit.com/r/rust/comments/8w9mfy/debian_is_starting_to_package_rust_crates/).
* [Relative paths in Rust 2018](https://internals.rust-lang.org/t/relative-paths-in-rust-2018/7883).
* [Exploring new communication channels](https://internals.rust-lang.org/t/exploring-new-communication-channels/7859). Some Rust teams are trying out the Discord chat platform for team discussions.
* [Security Advisory for rustdoc](https://blog.rust-lang.org/2018/07/06/security-advisory-for-rustdoc.html).
* [Programming for Redox OS](https://dev.to/legolord208/programming-for-redox-os-4124).
* [Programming Servo: an HTTP cache](https://medium.com/programming-servo/programming-servo-an-http-cache-edb52a7f267f).
* [podcast] [New Rustacean: Traits deep dive, part 3](https://newrustacean.com/show_notes/e025/index.html). Closure traits, impl trait, dyn trait, and object safety.

# Crate of the Week

This week's crate is [cargo-geiger](https://github.com/anderejd/cargo-geiger), which detects usage of unsafe Rust in your project and its dependencies.

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

158 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-02..2018-07-09

* [Stable release 1.27.1](https://github.com/rust-lang/rust/pull/52134).
* [Add `#[repr(transparent)]` to some libcore types](https://github.com/rust-lang/rust/pull/51395).
* [Stabilize rc_downcast](https://github.com/rust-lang/rust/pull/52103).
* [Add lint warning for inner function marked as `#[test]`](https://github.com/rust-lang/rust/pull/51450).
* [rust: add initial changes to support powerpc64le musl](https://github.com/rust-lang/rust/pull/51619).
* [Initialize LLVM's AMDGPU target machine, if available](https://github.com/rust-lang/rust/pull/51548).
* [Implement always-fallible TryFrom for usize/isize conversions that are infallible on some platforms](https://github.com/rust-lang/rust/pull/51564).
* [Haiku: several smaller fixes to build and run rust on Haiku](https://github.com/rust-lang/rust/pull/51757).
* [Add `read_exact_at` and `write_all_at` methods to `FileExt` on unix](https://github.com/rust-lang/rust/pull/51809).
* [Add the `alloc::prelude` module](https://github.com/rust-lang/rust/pull/52159).
* [Ship clippy in manifests](https://github.com/rust-lang/rust/pull/52131).
* [Don't suggest `let` bindings if they don't help with borrows](https://github.com/rust-lang/rust/pull/52106).
* [Get rid of `TyImplTraitExistential`](https://github.com/rust-lang/rust/pull/51979).
* [rename rustc's lld to rust-lld](https://github.com/rust-lang/rust/pull/51936).
* [Add outlives annotations to `BTreeMap`](https://github.com/rust-lang/rust/pull/51914).
* [Performance improvement of Vec's swap_remove](https://github.com/rust-lang/rust/pull/52166).
* [Add a punch card to weird expressions test](https://github.com/rust-lang/rust/pull/52073).
* [Add linux musl powerpc (32-bit) support](https://github.com/rust-lang/libc/pull/1031).
* [Implementation of tool lints](https://github.com/rust-lang/rust/pull/52018).
* [Enable Atomic*.{load,store} for ARMv6-M / MSP430](https://github.com/rust-lang/rust/pull/51953).
* [Make causal tracking lazy](https://github.com/rust-lang/rust/pull/51889).
* [Move self trait predicate to items](https://github.com/rust-lang/rust/pull/51895).
* [Mostly fix metadata_only backend and extract some code out of rustc_codegen_llvm](https://github.com/rust-lang/rust/pull/51590).
* [Deprecate `std::env::home_dir` and fix incorrect documentation](https://github.com/rust-lang/rust/pull/51656).

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

> actix-web has removed all unsound use of unsafe in its codebase. It’s down to less than 15 occurrences of unsafe from 100+.

– [u/_ar7 celebrating this commendable achievement](https://www.reddit.com/r/rust/comments/8wlkbe/actixweb_has_removed_all_unsound_use_of_unsafe_in/).

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/t/twir-quote-of-the-week/328/542) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
