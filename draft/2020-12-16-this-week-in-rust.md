Title: This Week in Rust 369
Number: 369
Date: 2020-12-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official
* [Launching the Lock Poisoning Survey](https://blog.rust-lang.org/2020/12/11/lock-poisoning-survey.html)
* [Next steps for the Foundation Conversation](https://blog.rust-lang.org/2020/12/14/Next-steps-for-the-foundation-conversation.html)
* [Rust Survey 2020 Results](https://blog.rust-lang.org/2020/12/16/rust-survey-2020.html)
* [Inside] [Changes to Rust Compiler Team](https://blog.rust-lang.org/inside-rust/2020/12/14/changes-to-compiler-team.html)

### Newsletters
* [This Month in Rust OSDev (November 2020)](https://rust-osdev.com/this-month/2020-11/)

### Tooling
* [IntelliJ Rust Changelog #137](https://intellij-rust.github.io/2020/12/14/changelog-137.html)
* [Rust Analyzer Changelog #55](https://rust-analyzer.github.io/thisweek/2020/12/14/changelog-55.html)
* [Knurling-rs Changelog #10](https://ferrous-systems.com/blog/knurling-changelog-10/)
* [These Months in explaine.rs](https://jrvidal.github.io/explaine.rs/blog/these-months-in-explainers.html)
* [Cargo2nix 0.9.0 release notes](https://github.com/cargo2nix/cargo2nix/releases/tag/v0.9.0)

### Observations/Thoughts
* [FFI-Safe Polymorphism: Thin Trait Objects](https://adventures.michaelfbryan.com/posts/ffi-safe-polymorphism-in-rust/)
* [Adding BPF target support to the Rust compiler](https://confused.ai/posts/rust-bpf-target)
* [Why Rust Has a Bright Future in the Cloud](https://www.qovery.com/blog/why-rust-has-a-bright-future-in-the-cloud)
* [Closures: Magic Functions](https://rustyyato.github.io/rust/syntactic/sugar/2019/01/17/Closures-Magic-Functions.html)
* [Notes on cross-compiling Rust](https://john-millikin.com/notes-on-cross-compiling-rust)
* [Polymorphism in Rust: Enums vs Traits](https://www.mattkennedy.io/blog/rust_polymorphism/)
* [Beyond R and Python: Rust for Science](https://scie.nz/rust/)
* [Pointers Are Complicated II, or: We need better language specs](https://www.ralfj.de/blog/2020/12/14/provenance.html)
* [Serde tricks: The lesson learnt from refactoring rspotify](https://0x709394.me/Serde-tricks)
* [Notes On Lock Poisoning](https://matklad.github.io/2020/12/12/notes-on-lock-poisoning.html)
* [I tried to make the fastest sorting algorithm I could think of... in Rust](https://danielvz.cl/blog/fast-rust.html)
* [Still Rusting - One Year Later](https://deislabs.io/posts/still-rusting-one-year-later/)
* [video] [The Unsafe Chronicles: Exhibit A: Aliasing Boxes](https://youtu.be/EY7Wi9fV5bk)

* [6 Principles for Building Robust Flexible Shared Data Apps with Rust & MongoDB](https://developer.mongodb.com/article/six-principles-building-robust-flexible-shared-data-applications)

### Rust Walkthroughs
* [Make A Language - Part Fourteen: Comments](https://arzg.github.io/lang/14/)
* [Make A Language - Part Fifteen: Markers](https://arzg.github.io/lang/15/)
* [Rust, Python and Fish](https://benjamin.computer/posts/2020-12-12-rust-python.html)
* [Chess game in Rust using Bevy](https://caballerocoll.com/blog/bevy-chess-tutorial/)
* [Rust on the BBC micro:bit](https://blog.drogue.io/rust-and-microbit/)
* [Developing smart contracts with ink!](https://dev.to/yangwao/developing-smart-contracts-with-ink-4g72)
* [Learn SixtyFPS: Memory Game Tutorial (Rust)](https://sixtyfps.io/blog/memory-game-tutorial.html)
* [Aiming for correctness with types](https://fasterthanli.me/articles/aiming-for-correctness-with-types)
* [Zero to Production in Rust #6: Using Types To Guarantee Domain Invariants](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
* [FFI-Safe Polymorphism: Thin Trait Objects](https://adventures.michaelfbryan.com/posts/ffi-safe-polymorphism-in-rust/)
* [Distributing Rust Analyzer with Nix and Cargo2nix](https://github.com/cargo2nix/cargo2nix/tree/master/examples/4-independent-packaging)
* [PL] [CrabbyBird #4 Generowanie świata gry – cześć II](https://postacnormalna.pl/crabbybird-4-generowanie-swiata-gry-czesc-ii/)
* [video] [How oso built a runtime reflection library for Rust](https://youtu.be/J7Aosp1Uauo)
* [video] [Implementing Rust's Vec From Scratch](https://youtu.be/3OL95gZgPWA)
* [RU] [video] [Rust: Not as hard as you think / Russian Rust Online Meetup](https://www.youtube.com/watch?v=yCrc5BwZrtw)

### Project Updates
* [These Months in explaine.rs](https://jrvidal.github.io/explaine.rs/blog/these-months-in-explainers.html)
* [Sequoia PGP](https://www.sequoia-pgp.org/) released [version 1.0](https://sequoia-pgp.org/blog/2020/12/16/202012-1.0/)

### Miscellaneous
* [Signal Group Calls are powered by Rust](https://www.reddit.com/r/rust/comments/kdo06l/signal_group_calls_are_powered_by_rust/)
* [Rust's Option in One Figure](https://www.reddit.com/r/rust/comments/kdfb9k/rusts_option_in_one_figure/)
* [Authors of "Programming Rust 2nd Edition" have a sense of humor](https://www.reddit.com/r/rust/comments/kcou9c/authors_of_programming_rust_2nd_edition_have_a/)
* [Rotating the compiler team leads](https://smallcultfollowing.com/babysteps/blog/2020/12/11/rotating-the-compiler-team-leads/)
* [Debug Rust on PineCone BL602 with VSCode and GDB](https://lupyuen.github.io/articles/debug)

# Crate of the Week

This week's crate is [thermite](https://github.com/raygon-renderer/thermite), a SIMD struct-of-arrays-algorithms library.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/857) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

300 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-12-07..2020-12-14

* [fixes to Rust coverage](https://github.com/rust-lang/rust/pull/79818)
* [properly re-use def path hash in incremental mode](https://github.com/rust-lang/rust/pull/79721)
* [add some `core::cmp::Ordering` helpers](https://github.com/rust-lang/rust/pull/79656)
* [constify some `MaybeUninit` methods](https://github.com/rust-lang/rust/pull/79621)
* [Windows TLS: `ManuallyDrop` instead of `mem::forget`](https://github.com/rust-lang/rust/pull/79893)
* [use `is_write_vectored` to optimize the `write_vectored` implementation for `BufWriter`](https://github.com/rust-lang/rust/pull/78768)
* [enforce no-move rule of `ReentrantMutex` using `Pin` and fix UB in stdio](https://github.com/rust-lang/rust/pull/77801)
* [hashbrown: enable specialization with aHash](https://github.com/rust-lang/hashbrown/pull/207)
* [future: `SinkExt::feed`](https://github.com/rust-lang/futures-rs/pull/2155)
* [futures-util: migrate from pin-project to pin-project-lite](https://github.com/rust-lang/futures-rs/pull/2273)
* [cargo: check if rerun-if-changed points to a directory](https://github.com/rust-lang/cargo/pull/8973)
* [cargo: workaround fs issue in `cargo publish`](https://github.com/rust-lang/cargo/pull/8950)
* [clippy: add MSRV to more lints](https://github.com/rust-lang/rust-clippy/pull/6424)
* [rustfmt: don't force a newline after an empty where clause](https://github.com/rust-lang/rustfmt/pull/4557)

## Rust Compiler Performance Triage

* [2020-12-15](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-15.md):
6 Regressions, 1 Improvements, 2 Mixed

This week was fairly quite with lots of small regressions. Most of the regressions were either for fixes to changes that yielded large performance wins in previous weeks or small performance losses where there is already a plan for how to gain those losses back.

Triage done by @rylev.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-15.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Plan to make core and std's panic identical](https://github.com/rust-lang/rfcs/pull/3007)
* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [added secret types rfc](https://github.com/rust-lang/rfcs/pull/2859)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [rustdoc: stabilise --default-theme command line option](https://github.com/rust-lang/rust/pull/79642)
* [disposition: merge] [Implement `From<char>` for u64 and u128.](https://github.com/rust-lang/rust/pull/79502)
* [disposition: merge] [Stabilize `unsafe_cell_get_mut`](https://github.com/rust-lang/rust/pull/79485)
* [disposition: merge] [Move `{f32,f64}::clamp` to core](https://github.com/rust-lang/rust/pull/79473)
* [disposition: merge] [Stabilize all stable methods of `Ipv4Addr`, `Ipv6Addr` and `IpAddr` as const](https://github.com/rust-lang/rust/pull/79342)
* [disposition: merge] [Acknowledge that `[CONST; N]` is stable](https://github.com/rust-lang/rust/pull/79270)
* [disposition: merge] [Deprecate atomic compare_and_swap method](https://github.com/rust-lang/rust/pull/79261)
* [disposition: merge] [Stabilize `core::slice::fill`](https://github.com/rust-lang/rust/pull/79213)
* [disposition: close] [Made matches! more useful by adding mapping support](https://github.com/rust-lang/rust/pull/79188)
* [disposition: merge] [passes: prohibit invalid attrs on generic params](https://github.com/rust-lang/rust/pull/79073)
* [disposition: merge] [stabilize deque_range](https://github.com/rust-lang/rust/pull/79022)
* [disposition: close] [Apply `unused_doc_comments` lint to inner items](https://github.com/rust-lang/rust/pull/78367)
* [disposition: merge] [Rename `overlapping_patterns` lint](https://github.com/rust-lang/rust/pull/78242)
* [disposition: merge] [Stabilize or_insert_with_key](https://github.com/rust-lang/rust/pull/78083)
* [disposition: close] [Add built-in implementations of `Default` for function definition and… ](https://github.com/rust-lang/rust/pull/77688)
* [disposition: merge] [Mark `-1` as an available niche for file descriptors](https://github.com/rust-lang/rust/pull/74699)
* [disposition: merge] [Stabilize the Wake trait](https://github.com/rust-lang/rust/pull/74304)
* [disposition: merge] [Tracking issue for map_ok and map_err method for `Poll<Option<Result<T, E>>>`](https://github.com/rust-lang/rust/issues/63514)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [December 10, Stuttgart, DE - Hack & Learn - Directions for 2021 - Rust Community Stuttgart](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/274892215/)
* [December 10, San Diego, CA, US - San Diego Rust December 2020 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/274757235/)
* [December 10, Washington, DC, US - How oso built a runtime reflection system for Rust—Rust DC](https://www.meetup.com/RustDC/events/274460587)
* [December 15, Russia - Russian Rust Online Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/274924961/)
* [December 16, Vancouver, BC, US - Are Results just Checked Exceptions? - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsybcqbvb/)

### North America
* [December 10, Provo, UT, US - Mob Programming: Add `--tree -d` to `lsd`](https://www.meetup.com/utah-rust/events/273530244/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Engineering is not about "not doing mistakes". Engineering is about designing systems that ensure fewer mistakes occur.
>
> Rust is such a system.

– [amos on his blog](https://fasterthanli.me/articles/aiming-for-correctness-with-types)

Thanks to [Joshua Nelson](https://users.rust-lang.org/t/twir-quote-of-the-week/328/972) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
