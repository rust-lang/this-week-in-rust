Title: This Week in Rust 236
Number: 236
Date: 2018-05-29
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

* 🎈🎉 [Announcing Rust 1.26.1](https://blog.rust-lang.org/2018/05/29/Rust-1.26.1.html). 🎉🎈
* [Exploring Rust fat pointers](https://iandouglasscott.com/2018/05/28/exploring-rust-fat-pointers/).
* [Typed key pattern](https://matklad.github.io/2018/05/24/typed-key-pattern.html). A pattern for extracting values from a weakly typed map.
* [aturon.log: listening and trust, part 1](https://aturon.github.io/2018/05/25/listening-part-1/). Thoughts on controversies from past week.
* [Clippy is removing the `#![plugin(clippy)]` API in favour of `cargo clippy`](https://github.com/rust-lang-nursery/rust-clippy/pull/2783).
* [Fuzz testing in Rust with Cargo-fuzz](https://medium.com/@seasoned_sw/fuzz-testing-in-rust-with-cargo-fuzz-13b89feecc30).
* [Wicked fast web servers in Rust - An introduction to writing a simple web server using Thruster](https://medium.com/@MertzAlertz/wicked-fast-web-servers-in-rust-4947688426bc).
* [Bobbin SDK: Richer hardware abstractions for embedded systems programming](http://www.bobbin.io/blog/post/bobbin_sdk_richer_hardware/).
* [video] [RustFest Paris 2018 recordings](https://www.youtube.com/watch?v=23lRkdDXqY0&list=PL85XCvVPmGQgdqz9kz6qH3SI_hp7Zb4s1).
* [podcast] [Rusty Spike Podcast - episode 30](https://rusty-spike.blubrry.net/2018/05/24/episode-30-may-23-2018/). Compile times, Aaron’s new quest, books, logic programming, JetBrains, and RustConf.

### GSoC Projects

* [Nebulet - Booting up](http://lsneff.me/nebulet-booting-up/). Nebulet is a microkernel that executes WebAssembly. See also: [Why Nebulet](http://lsneff.me/why-nebulet/)?
* [Cargo-ebuild - GSoC week 1 report](https://gibix.github.io/2018/05/20/gsoc-week-1-report.html).

### RSoC Projects

* [Porting tokio-rs to redox](https://www.redox-os.org/news/rsoc-porting-tokio-1/).
* [Implementing a FAT32 Filesystem in Redox](https://www.redox-os.org/news/rsoc-fat32-1/).

# Crate of the Week

This week's crate is [syntect](https://crates.io/crates/syntect), a library for syntax highlighting using Sublime Text syntax definitions. Thanks to [kornel](https://users.rust-lang.org/u/kornel) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Have you ever complained that rustc is slow? We want to know more](https://github.com/rust-lang-nursery/rustc-perf/issues/232)!
* [rand: Add wasm_bindgen support](https://github.com/rust-lang-nursery/rand/issues/478).
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

## New Contributors

* Benjamin Lamowski
* Cory Sherman
* Jaro Fietz
* Joe ST
* Martin Carton
* Mateusz Mikuła
* Nick Babcock
* Takanori Ishibashi
* uuttff8

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2386: `#[used]` static variables](https://github.com/rust-lang/rfcs/pull/2386).
* [RFC 2294: Allow `if let` guards in `match` expressions](https://github.com/rust-lang/rfcs/pull/2294).
* [RFC 2421: Keyword unreservations (pure, sizeof, alignof, offsetof)](https://github.com/rust-lang/rfcs/pull/2421).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Reserve `throw` and `fail` as keywords in edition 2018](https://github.com/rust-lang/rfcs/pull/2441).
* [disposition: merge] [Unstable features accidentally usable on the Stable release chanel are still unstable](https://github.com/rust-lang/rfcs/pull/2405).
* [disposition: postpone] [Implied `#[derive(SuperTrait)]`](https://github.com/rust-lang/rfcs/pull/2385).
* [disposition: postpone] [DynSized without ?DynSized — Lint against use of `extern type` in `size_of_val`, and more](https://github.com/rust-lang/rfcs/pull/2310).

## New RFCs

* [Announce tracking issues in FCP](https://github.com/rust-lang/rfcs/pull/2449).
* [Lint for unused results of `const fn` functions](https://github.com/rust-lang/rfcs/pull/2450).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Jun  2. Nairobi, KE - Rust on the BBC micro:bit](https://www.meetup.com/Rust-Nairobi/events/250762823/).
* [Jun  2. Florianópolis, BR - 1º Encontro Rust Floripa](https://www.meetup.com/rustfloripa/events/xvglrpyxjbdb/).
* [Jun  3. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbbc/).
* [Jun  4. Karlsruhe, DE - Rust Hack & Learn Karlsruhe - Meetup & Talks](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/250646555/).
* [Jun  5. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Jun  5. Johannesburg, ZA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxjbhb/).
* [Jun  6. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jun  6. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun  6. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/cpvshpyxjbjb/).
* [Jun  6. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxjbjb/).
* [Jun 10. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxjbnb/).
* [Jun 11. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxjbpb/).
* [Jun 13. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/250904450/).
* [Jun 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Jun 13. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxjbrb/).
* [Jun 14. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxjbsb/).
* [Jun 14. San Diego, US - San Diego Rust June Meetup](https://www.meetup.com/San-Diego-Rust/events/251001684/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).** Registration is now open.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

* [Freelance Rust developer in Paris (remote work available)](https://twitter.com/JonathanBelolo/status/1000128978936623104).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
