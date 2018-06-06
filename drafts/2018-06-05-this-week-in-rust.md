Title: This Week in Rust 237
Number: 237
Date: 2018-06-05
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

* 🎈🎉 [Announcing Rust 1.26.2](https://blog.rust-lang.org/2018/06/05/Rust-1.26.2.html). 🎉🎈
* [How to speed up the Rust compiler some more in 2018](https://blog.mozilla.org/nnethercote/2018/06/05/how-to-speed-up-the-rust-compiler-some-more-in-2018/).
* [The future of Clippy (the Rust linter)](https://manishearth.github.io/blog/2018/06/05/the-future-of-clippy-the-rust-linter/).
* [Async methods I: generic associated types](https://boats.gitlab.io/blog/post/async-methods-i/).
* [Async methods II: object safety](https://boats.gitlab.io/blog/post/async-methods-ii/).
* [Avoiding the pyramid of doom](https://www.reddit.com/r/rust/comments/8n4o5q/avoiding_the_pyramid_of_doom/).
* [The Rust way of OS development](https://phil-opp.github.io/talk-konstanz-may-2018/).
* [The secret life of Cows](https://deterministic.space/secret-life-of-cows.html).
* [The state of Gotham](https://gotham.rs/blog/2018/05/31/the-state-of-gotham.html).
* [Optimising path tracing with SIMD](https://bitshifter.github.io/blog/2018/06/04/simd-path-tracing/).
* [Writing a simple web service in Rust with actix-web](https://danielwelch.github.io/rust-web-service.html).
* [Newtype index pattern](https://matklad.github.io/2018/06/03/newtype-index-pattern.html).
* [The Rust language and special cases](https://blog.infinitenegativeutility.com/2018/6/the-rust-language-and-special-cases).
* [aturon.log: listening and trust, part 2](https://aturon.github.io/2018/06/02/listening-part-2/).
* [Introducing feL4: Rust programs for the seL4 microkernel](https://www.reddit.com/r/rust/comments/8oqppz/introducing_fel4_rust_programs_for_the_sel4/).
* [hyper v0.12 is released](http://seanmonstar.com/post/174480374517/hyper-v012).
* [This week in Rust docs 107](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-107).
* [This week in Rust and WebAssembly 3](https://rustwasm.github.io/2018/06/04/this-week-in-rust-wasm-003.html).
* [podcast] [New Rustacean: Crates you should know: wasm](https://newrustacean.com/show_notes/cysk/wasm/index.html). wasm intro, wasm-bindgen, and wasm-pack.
* [podcast] [Rusty Spike Podcast - episode 31](https://rusty-spike.blubrry.net/2018/05/31/episode-31-may-30-2018/). 1.26.1 release, RustFest videos, Rust reach, RustRush 2018, and a bit of behind the scenes look at running Rust.

# Crate of the Week

This week's crate is [syntect](https://crates.io/crates/syntect), a library for syntax highlighting using Sublime Text syntax definitions. Thanks to [kornel](https://users.rust-lang.org/u/kornel) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [wasm-pack has several open good first issues available to new contributors](https://github.com/ashleygwilliams/wasm-pack/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

141 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-05-21..2018-05-28

* [stable point release (1.26.1)](https://github.com/rust-lang/rust/pull/51045)
* [infer outlives requirements](https://github.com/rust-lang/rust/pull/50070) (RFC [#2093](https://github.com/rust-lang/rfcs/pull/2093))
* [don't ICE if crate has no valid crate types left](https://github.com/rust-lang/rust/pull/51035)
* [`NaN > NaN` is now false again](https://github.com/rust-lang/rust/pull/50812) (Breaking change, duh!)
* [add suggestion applicabilities to librustc and libsyntax](https://github.com/rust-lang/rust/pull/50724)
* [add -Z no-parallel-llvm flag](https://github.com/rust-lang/rust/pull/50972)
* [WebAssembly: fix fast-isel lowering illegal argument and return types](https://github.com/rust-lang/llvm/pull/117)
* [remove unused lowering field and method](https://github.com/rust-lang/rust/pull/51034)
* [quick refactoring around Substs & friends](https://github.com/rust-lang/rust/pull/50801)
* [operate on `HirId` instead of `NodeId` in `hir::Pat::each_binding`, and consequences of that](https://github.com/rust-lang/rust/pull/50929)
* [use `Ident`s for fields in HIR](https://github.com/rust-lang/rust/pull/51072)
* [remove extra calls to kill_loans_out_of_scope_at_location](https://github.com/rust-lang/rust/pull/50891)
* [fix behaviour of divergence in while loop conditions](https://github.com/rust-lang/rust/pull/51049)
* [fail typecheck if we encounter a bogus break](https://github.com/rust-lang/rust/pull/51070)
* [generate "invalidates" facts when -Znll-facts is passed](https://github.com/rust-lang/rust/pull/50798)
* [NLL facts invalidate followup](https://github.com/rust-lang/rust/pull/50998)
* [use `AllFacts` from polonius-engine](https://github.com/rust-lang/rust/pull/51047)
* [enforce stability of const fn in promoteds](https://github.com/rust-lang/rust/pull/50909)
* [stabilize suggestion applicability field in json output](https://github.com/rust-lang/rust/pull/50486)
* [shrink `LiveNode`](https://github.com/rust-lang/rust/pull/50981)
* [right-size the `VecDeque` in `coerce_unsized`](https://github.com/rust-lang/rust/pull/50963)
* [optimize seen Predicate filtering](https://github.com/rust-lang/rust/pull/50932)
* [inline `try_get`](https://github.com/rust-lang/rust/pull/50931)
* [make `&Slice` a thin pointer](https://github.com/rust-lang/rust/pull/50612)
* [find the largest niche when computing layouts](https://github.com/rust-lang/rust/pull/50860)
* ["crate-ify" paths that begin with a renamed crate](https://github.com/rust-lang/rust/pull/51010)
* [rustc: fix another double-lint issue with `crate::`](https://github.com/rust-lang/rust/pull/50982)
* [rustc: correctly pretty-print macro delimiters](https://github.com/rust-lang/rust/pull/50971)
* [rename `TokenStream::empty` to `TokenStream::new`](https://github.com/rust-lang/rust/pull/51073)
* [underline multiple suggested replacements in the same line](https://github.com/rust-lang/rust/pull/50987)
* [tweak `main` type arguments and where clause spans](https://github.com/rust-lang/rust/pull/50986)
* [fix span for type-only arguments](https://github.com/rust-lang/rust/pull/50979)
* [`CheckLoopVisitor`: also visit closure arguments](https://github.com/rust-lang/rust/pull/50849)
* [add lint for multiple associated types](https://github.com/rust-lang/rust/pull/50682)
* [`impl Trait` diagnostic/test cleanups](https://github.com/rust-lang/rust/pull/50943)
* [prohibit turbofish in `impl Trait` methods](https://github.com/rust-lang/rust/pull/51051)
* [fix naming conventions for new lints](https://github.com/rust-lang/rust/pull/50879)
* [MIRI API refactor](https://github.com/rust-lang/rust/pull/50967)
* [use different datastructure for MIRI relocations](https://github.com/rust-lang/rust/pull/50866)
* [misc changes related to Miri allocations](https://github.com/rust-lang/rust/pull/50520)
* [allow let bindings and destructuring in constants and const fn](https://github.com/rust-lang/rust/pull/49172)
* [allow `Size` to be any valid `u64`](https://github.com/rust-lang/rust/pull/50916)
* [implement the chalk-engine traits](https://github.com/rust-lang/rust/pull/50937)
* [fun testcase: What does an expression look like, that consists only of special characters?](https://github.com/rust-lang/rust/pull/51059)
* [escape combining characters in `char::Debug`](https://github.com/rust-lang/rust/pull/49283)
* [improve `Debug` impl of `time::Duration`](https://github.com/rust-lang/rust/pull/50364)
* [add SIMD math intrinsics and gather/scatter](https://github.com/rust-lang/rust/pull/50521)
* [`Unpin` changes](https://github.com/rust-lang/rust/pull/50984)
* [make `[T]::len` and `str::len` const fn](https://github.com/rust-lang/rust/pull/50863)
* [std: ensure OOM is classified as `nounwind`](https://github.com/rust-lang/rust/pull/51041)
* [stabilize `from_ref`](https://github.com/rust-lang/rust/pull/50945)
* [stabilize `ops::RangeBounds`](https://github.com/rust-lang/rust/pull/51033)
* [stabilize `Formatter` alignment](https://github.com/rust-lang/rust/pull/51078)
* [remove the unstable Float trait](https://github.com/rust-lang/rust/pull/50933)
* [add the 2018 edition of the book to doc.rust-lang.org](https://github.com/rust-lang/rust/pull/50952)
* [support `--target` argument in `cargo rustdoc`](https://github.com/rust-lang/cargo/pull/5587)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Amend 1105: Unstable features accidentally usable on the Stable release chanel are still unstable](https://github.com/rust-lang/rfcs/pull/2405).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for the GlobalAlloc trait and related APIs](https://github.com/rust-lang/rust/issues/49668).
* [disposition: merge] [Stabilize GlobalAlloc and `#[global_allocator]`](https://github.com/rust-lang/rust/pull/51241).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: merge] [Tracking issue for "macro naming and modularisation" (RFC #1561)](https://github.com/rust-lang/rust/issues/35896).
* [disposition: merge] [Tracking issue for promoting `!` to a type (RFC 1216)](https://github.com/rust-lang/rust/issues/35121).
* [disposition: merge] [Exhaustive integer matching](https://github.com/rust-lang/rust/pull/50912).
* [disposition: merge] [Stabilize `std::path::Path::ancestors`](https://github.com/rust-lang/rust/pull/50894).
* [disposition: merge] [Add ability to apply custom derive to union types](https://github.com/rust-lang/rust/pull/50383).

## New RFCs

* [Re-Rebalancing Coherence](https://github.com/rust-lang/rfcs/pull/2451).
* [Allow non-ASCII identifiers](https://github.com/rust-lang/rfcs/pull/2457).
* [Unused const fn results](https://github.com/rust-lang/rfcs/pull/2450).

# Upcoming Events

* [Jun 10. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbnb/).
* [Jun 11. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxjbpb/).
* [Jun 12. Rome, IT - Rust learning and hacking evening #9](https://www.meetup.com/Rust-Roma/events/251298815/).
* [Jun 13. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/250904450/).
* [Jun 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun 13. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbrb/).
* [Jun 14. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxjbsb/).
* [Jun 14. San Diego, US - San Diego Rust June Meetup](https://www.meetup.com/San-Diego-Rust/events/251001684/).
* [Jun 14. Utah Valley, US - Utah Rust meetup](https://docs.google.com/document/d/1O8S7IEfDw-3jTN74CWCuKYl_UWxTLd6-epz7NOMDYRg).
* [Jun 17. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbwb/).
* [Jun 19. Denver, US - Rust Boulder/Denver - June Meetup in Boulder](https://www.meetup.com/Rust-Boulder-Denver/events/250076478/).
* [Jun 19. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jun 20. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jun 20. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun 21. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxjbcc/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).** Registration is now open.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Backend Engineer at ClanHQ, San Francisco (remote available)](https://twitter.com/JonathanBelolo/status/1000128978936623104).
* [Rust Developer at 1am, Berlin](https://twitter.com/__1aim/status/1002493500099833856).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> We’re able to replicate the structure of cows, but not their lifetimes, in C++, which makes Cows dangerous.

— [Henri Sivonen](https://twitter.com/hsivonen) closing his talk about integrating Rust and C++ at [RustFest](https://paris.rustfest.eu/).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
