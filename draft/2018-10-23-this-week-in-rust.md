Title: This Week in Rust 257
Number: 257
Date: 2018-10-23
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

* 🎈🎉 [Announcing Rust 1.29.2](https://blog.rust-lang.org/2018/10/12/Rust-1.29.2.html). 🎉🎈
* [Rust has a static garbage collector](https://words.steveklabnik.com/borrow-checking-escape-analysis-and-the-generational-hypothesis).
* [Serverless Rust with Cloudflare Workers](https://blog.cloudflare.com/cloudflare-workers-as-a-serverless-rust-platform/).
* [Notes on type layouts and ABIs in Rust](https://gankro.github.io/blah/rust-layouts-and-abis/).
* [Game dev from zero - part 1: Hello, Rust lang](https://hashnode.com/post/game-dev-from-zero-part-1-hello-rust-lang-cjn3brwto001jv7s2e533bdfc).
* [Fixing a Clippy crash](https://phansch.net/2018/10/10/fixing-a-clippy-crash/).
* [Handling configurations in a Rust app with envy](https://medium.com/@softprops/configuration-envy-a09584386705).
* [Routing and extraction in Tide: a first sketch](https://rust-lang-nursery.github.io/wg-net/2018/10/16/tide-routing.html).
* [Amethyst is growing; news on ongoing projects](https://www.amethyst.rs/blog/dev-news-10-2018/).

# Crate of the Week

This week's crate is [static-assertions](https://docs.rs/static_assertions), a crate that does what it says on the tin – allow you to write static assertions. Thanks to [llogiq](https://github.com/llogiq) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust Web Survey from Networking Services WG](https://docs.google.com/forms/d/e/1FAIpQLSf9KCUs-8G87pHB08lM8-iXcDSY_VttOI0PvkKseHaZseCGGA/viewform).
* [Alpine Linux is looking for help with porting Rust to non-x86 architectures](https://lists.alpinelinux.org/alpine-devel/6295.html).
* [Rav1e, an encoder for the new AV1 codec written in Rust, has some open issues for Hacktoberfest 2018](https://github.com/xiph/rav1e/projects/6).
* [Call for participation in the Rust devroom at FOSDEM (2-3 Feb 2019, Brussels, Belgium)](https://rust-fosdem.github.io/).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

115 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-10-15..2018-10-22

* [mir-inlining: don't inline virtual calls](https://github.com/rust-lang/rust/pull/55046)
* [reject partial init and reinit of uninitialized data](https://github.com/rust-lang/rust/pull/54941)
* [improve verify_llvm_ir config option](https://github.com/rust-lang/rust/pull/55031)
* [add missing lifetime fragment specifier to error message](https://github.com/rust-lang/rust/pull/55025)
* [rustc: fix (again) simd vectors by-val in ABI](https://github.com/rust-lang/rust/pull/55073)
* [resolve: scale back hard-coded extern prelude additions on 2015 edition](https://github.com/rust-lang/rust/pull/54671)
* [resolve: do not skip extern prelude during speculative resolution](https://github.com/rust-lang/rust/pull/55102)
* [allow explicit matches on ! without warning](https://github.com/rust-lang/rust/pull/55119)
* [deduplicate some code and compile-time values around vtables](https://github.com/rust-lang/rust/pull/55016)
* [NLL: propagate bounds from generators](https://github.com/rust-lang/rust/pull/55013)
* [NLL lacks various special case handling of closures](https://github.com/rust-lang/rust/pull/54976)
* [NLL: fix migrate mode issue by not buffering lints](https://github.com/rust-lang/rust/pull/55135)
* [NLL: change compare-mode=nll to use borrowck=migrate](https://github.com/rust-lang/rust/pull/55134)
* [NLL: use new region infer errors when explaining borrows](https://github.com/rust-lang/rust/pull/55069)
* [NLL type annotations in multisegment path](https://github.com/rust-lang/rust/pull/55093)
* [add filtering option to `rustc_on_unimplemented` and reword `Iterator` E0277 errors](https://github.com/rust-lang/rust/pull/54946    )
* [custom E0277 diagnostic for `Path`](https://github.com/rust-lang/rust/pull/54979)
* [`unused_patterns` lint](https://github.com/rust-lang/rust/pull/54820)
* [check the type of statics and constants for `Sized`ness](https://github.com/rust-lang/rust/pull/55004)
* [miri: layout should not affect CTFE checks](https://github.com/rust-lang/rust/pull/55142)
* [added graphviz visualization for obligation forests](https://github.com/rust-lang/rust/pull/54486)
* [replace CanonicalVar with DebruijnIndex](https://github.com/rust-lang/rust/pull/52984)
* [stabilize slice::chunks_exact(), chunks_exact_mut(), rchunks(), rchunks_mut(), rchunks_exact(), rchunks_exact_mut()](https://github.com/rust-lang/rust/pull/55178)
* [add a `copysign` function to f32 and f64](https://github.com/rust-lang/rust/pull/55169)
* [don't warn about parentheses on `match (return)`](https://github.com/rust-lang/rust/pull/55166)
* [handle underscore bounds in unexpected places](https://github.com/rust-lang/rust/pull/55162)
* [fix ICE and report a human readable error](https://github.com/rust-lang/rust/pull/55071)
* [add slice::rchunks(), rchunks_mut(), rchunks_exact() and rchunks_exact_mut()](https://github.com/rust-lang/rust/pull/54580)
* [unify multiple errors on single typo in match pattern](https://github.com/rust-lang/rust/pull/55156)
* [fix LLVMRustInlineAsmVerify return type mismatch](https://github.com/rust-lang/rust/pull/55128)
* [miri engine: hooks for basic stacked borrows](https://github.com/rust-lang/rust/pull/55125)
* [add support for 'cargo check --all-features'](https://github.com/rust-lang/rust.vim/pull/265)
* [cargo: add PackageError wrappers for activation errors](https://github.com/rust-lang/cargo/pull/6175)
* [rustdoc: use dyn keyword when rendering dynamic traits](https://github.com/rust-lang/rust/pull/55077)
* [rustdoc: don't prefer dynamic linking in doc tests](https://github.com/rust-lang/rust/pull/54939)
* [rustdoc: add lint for doc without codeblocks](https://github.com/rust-lang/rust/pull/54349)
* [detect if access to localStorage is forbidden by the user's browser](https://github.com/rust-lang/rust/pull/55080)
* [librustdoc: disable spellcheck for search field](https://github.com/rust-lang/rust/pull/55161)
* [crates.io: add a missing index on crates](https://github.com/rust-lang/crates.io/pull/1527)

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

* [disposition: merge] [Clippy 1.0](https://github.com/rust-lang/rfcs/pull/2476).
* [disposition: merge] [Allow non-ASCII identifiers](https://github.com/rust-lang/rfcs/pull/2457).
* [disposition: merge] [Formatting guidelines](https://github.com/rust-lang/rfcs/pull/2436).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Report const eval error inside the query](https://github.com/rust-lang/rust/pull/53821).
* [disposition: close] [Introduce the `Result::into_inner` method](https://github.com/rust-lang/rust/pull/54219).

## New RFCs

* [Meta-RFC: Future work](https://github.com/rust-lang/rfcs/pull/2561).
* [Attributes in formal function parameter position](https://github.com/rust-lang/rfcs/pull/2565).

# Upcoming Events

### Online

* [Oct 22. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Oct 24. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Oct 31. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Oct 27. Nairobi, KE - HACK & LEARN: Hacktoberfest Edition](https://www.meetup.com/Rust-Nairobi/events/255546089).

### Asia

* [Oct 27. St. Petersburg, RU - Неформальная встреча Rust-разработчиков](https://www.meetup.com/Rust-%D0%B2-%D0%9F%D0%B8%D1%82%D0%B5%D1%80%D0%B5/events/nhpkmpyxnbkc).

### Europe

* [Oct 18. Oslo, NO - Fuzzing and property-based testing in Rust](https://www.meetup.com/Rust-Oslo/events/254830021/).
* [Oct 18. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxnbxb/).
* [Oct 24. Zurich, CH - Rust Zurich - Atomics](https://www.meetup.com/Rust-Zurich/events/255279862/).
* [Oct 25. Wroclaw, PL - Rust Wroclaw Meetup](https://www.meetup.com/Rust-Wroclaw/events/255053694/).
* [Oct 30. Paris, FR - Rust Paris meetup #43](https://www.meetup.com/Rust-Paris/events/255604978).
* [Oct 31. Prague, CZ - Prague Containers Meetup - The way of Rust](https://www.meetup.com/Prague-Containers-Meetup/events/251325363/).
* [Oct 31. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxnbpc/).

### North America

* [Oct 18. Ann Arbor, US - Pre Rust Belt Impl Day](https://rust-belt-rust.com/).
* [Oct 19 & 20. Ann Arbor, US - Rust Belt Rust 2018](https://rust-belt-rust.com/).
* [Oct 20 & 21. Vancouver, CA - Vancouver Rust Hackathon](https://www.eventbrite.ca/e/vancouver-rust-hackathon-tickets-50012680273).
* [Oct 21. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnbcc/).
* [Oct 22. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxnbdc/).
* [Oct 23. Chicago, US - The Rust Ecosystem: What to Know After "Hello World"](https://www.meetup.com/Chicago-Rust-Meetup/events/255066746).
* [Oct 28. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnblc/).
* [Oct 30. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxnbnc/).
* [Oct 31. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/xttphqyxnbpc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at The Graph, Remote](https://thegraph.com/careers?job=3#section3).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Rust Developer at 1aim, Berlin](https://1aim.com/careers).
* [Rust Engineer at Commure, Inc. (San Francisco, Boston, Montreal)](https://www.reddit.com/r/rust/comments/92e67g/commure_healthcare_software_startup_hiring_rust/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Panic is “pulling over to the side of the road” whereas crash is “running into a telephone pole”.

– /u/zzzzYUPYUPphlumph [on /r/rust](https://www.reddit.com/r/rust/comments/9q3jqn/how_is_rust_safe_when_panics_can_happen_out_of/e86glzs/)

Thanks to [KillTheMule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/570) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
