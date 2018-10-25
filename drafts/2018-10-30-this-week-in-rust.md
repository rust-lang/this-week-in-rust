Title: This Week in Rust 258
Number: 258
Date: 2018-10-30
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
* [Jim Blandy interview on mastering moves and borrows](https://corecursive.com/016-moves-and-borrowing-in-rust-with-jim-blandy/).

# Crate of the Week

This week's crate is [static-assertions](https://docs.rs/static_assertions), a crate that does what it says on the tin – allow you to write static assertions. Thanks to [llogiq](https://github.com/llogiq) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [toml: External enum serialization](https://github.com/alexcrichton/toml-rs/pull/267).

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

* [RFC 2476: Clippy 1.0](https://github.com/rust-lang/rfcs/pull/2476).
* [RFC 2514: Union initialization and Drop](https://github.com/rust-lang/rfcs/pull/2514).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Re-Rebalancing Coherence](https://github.com/rust-lang/rfcs/pull/2451).
* [disposition: merge] [Allow non-ASCII identifiers](https://github.com/rust-lang/rfcs/pull/2457).
* [disposition: merge] [Meta-RFC: Future possibilities](https://github.com/rust-lang/rfcs/pull/2561).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Report const eval error inside the query](https://github.com/rust-lang/rust/pull/53821).
* [disposition: merge] [Unchecked thread spawning](https://github.com/rust-lang/rust/pull/55043).
* [disposition: merge] [Implement FromStr for PathBuf](https://github.com/rust-lang/rust/pull/55148).
* [disposition: close] [Regression from stable: pointer to usize conversion no longer compiles](https://github.com/rust-lang/rust/issues/54709).

## New RFCs

* [SIMD vectors in FFI](https://github.com/rust-lang/rfcs/pull/2574).
* [flat_map as an alias for and_then](https://github.com/rust-lang/rfcs/pull/2572).
* [Linked list cursors](https://github.com/rust-lang/rfcs/pull/2570).

# Upcoming Events

### Online

* [Oct 31. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov  5. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov  7. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Africa

* [Oct 27. Nairobi, KE - HACK & LEARN: Hacktoberfest Edition](https://www.meetup.com/Rust-Nairobi/events/255546089).
* [Nov  6. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxpbjb/).

### Europe

* [Oct 27. St. Petersburg, RU - Неформальная встреча Rust-разработчиков](https://www.meetup.com/Rust-%D0%B2-%D0%9F%D0%B8%D1%82%D0%B5%D1%80%D0%B5/events/nhpkmpyxnbkc).
* [Oct 30. Paris, FR - Rust Paris meetup #43](https://www.meetup.com/Rust-Paris/events/255604978).
* [Oct 31. Prague, CZ - Prague Containers Meetup - The way of Rust](https://www.meetup.com/Prague-Containers-Meetup/events/251325363/).
* [Oct 31. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxnbpc/).
* [Oct 31. Milan, IT - Rust Language Milano - Rust Exercises](https://www.meetup.com/rust-language-milano/events/255737296/).
* [Nov  7. Stuttgart, DE - Rust in der Industrie & Automatisierung](https://www.meetup.com/slowtec/events/255390000/).
* [Nov  7. Cologne, DE - Rust Cologne](https://www.meetup.com/RustCologne/events/vnwndpyxpbkb/).

### North America

* [Oct 28. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnblc/).
* [Oct 30. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxnbnc/).
* [Oct 31. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/xttphqyxnbpc/).
* [Nov  4. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbgb/).
* [Nov  7. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxpbkb/).
* [Nov  7. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxpbkb/).
* [Nov  8. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209655/).
* [Nov  8. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/254871472).
* [Nov  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxpblb/).
* [Nov  8. Boston, US - Rust/Scala meetup at SPLASH conf](https://www.meetup.com/BostonRust/events/255445951/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at The Graph, Remote](https://thegraph.com/careers?job=3#section3).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Panic is “pulling over to the side of the road” whereas crash is “running into a telephone pole”.

– /u/zzzzYUPYUPphlumph [on /r/rust](https://www.reddit.com/r/rust/comments/9q3jqn/how_is_rust_safe_when_panics_can_happen_out_of/e86glzs/)

Thanks to [KillTheMule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/570) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
