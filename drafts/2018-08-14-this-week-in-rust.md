Title: This Week in Rust 247
Number: 247
Date: 2018-08-14
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

This week's crate is [warp](https://github.com/seanmonstar/warp), a fast, composable web framework. Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/428) for suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [How to land your first Rust pull request in TiKV](https://www.pingcap.com/blog/adding-built-in-functions-to-tikv/).
* [easy] [Maud: Update benchmarks](https://github.com/lfairy/maud/issues/143). Maud is an HTML template engine for Rust.
* [atom-language-rust: Help with PR reviews](https://users.rust-lang.org/t/twir-call-for-participation/4821/202).
* [intl_pluralrules: Seeking crate review](https://users.rust-lang.org/t/twir-call-for-participation/4821/203).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

165 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-23..2018-07-31

* [don't commit thread stack on Windows](https://github.com/rust-lang/rust/pull/52847)
* [implement a self profiler](https://github.com/rust-lang/rust/pull/51657)
* [update LLVM submodule to 7.0](https://github.com/rust-lang/rust/pull/52983)
* [use `BitVector` for global sets of `AttrId`](https://github.com/rust-lang/rust/pull/52799)
* [use suggestions for shell format arguments](https://github.com/rust-lang/rust/pull/52888)
* [async can begin expressions](https://github.com/rust-lang/rust/pull/52954)
* [resolve: modularize crate-local `#[macro_export] macro_rules`](https://github.com/rust-lang/rust/pull/52234)
* [resolve: record single-segment extern crate import resolutions](https://github.com/rust-lang/rust/pull/52930)
* [privacy: fix an ICE in `path_is_private_type`](https://github.com/rust-lang/rust/pull/53001)
* [reintroduce `Undef` and properly check constant value sizes](https://github.com/rust-lang/rust/pull/52712)
* [enable RISCV](https://github.com/rust-lang/rust/pull/52787)
* [aarch64 fix](https://github.com/rust-lang/llvm/pull/123)
* [NLL migration in the 2018 edition needs two-phase borrows too!](https://github.com/rust-lang/rust/pull/52975)
* [NLL mentions lifetimes that are not included in printed span(s)](https://github.com/rust-lang/rust/pull/52973)
* [NLL: dangly paths for box](https://github.com/rust-lang/rust/pull/52782)
* [NLL: disable some nice region errors in NLL mode](https://github.com/rust-lang/rust/pull/53115)
* [NLL: avoid computing liveness for locals that escape into statics](https://github.com/rust-lang/rust/pull/52991)
* [NLL: use smaller spans for errors involving closure captures](https://github.com/rust-lang/rust/pull/52959)
* [NLL: better Diagnostic When "Later" means "A Future Loop Iteration"](https://github.com/rust-lang/rust/pull/52948)
* [include lifetime in mutability suggestion in NLL messages](https://github.com/rust-lang/rust/pull/52883)
* [NLL: allow conflicting borrows of promoted length zero arrays](https://github.com/rust-lang/rust/pull/52834)
* [NLL: Don't make "fake" match variables mutable](https://github.com/rust-lang/rust/pull/52810)
* [fix NLL migration mode so that reports region errors when necessary](https://github.com/rust-lang/rust/pull/53045)
* [NLL: sort diagnostics by span](https://github.com/rust-lang/rust/pull/52904)
* [slices: fix ZST slice iterators making up pointers; debug_assert alignment in from_raw_parts](https://github.com/rust-lang/rust/pull/52206)
* [App-lint-cability](https://github.com/rust-lang/rust/pull/52968)
* [add more diagnostics to smooth edition transition](https://github.com/rust-lang/cargo/pull/5824)
* [fix memrchr in MIRI](https://github.com/rust-lang/rust/pull/52854)
* [`invalid_const_promotion`: check if we get the right signal](https://github.com/rust-lang/rust/pull/52823)
* [remove unstable and deprecated APIs](https://github.com/rust-lang/rust/pull/52732)
* [revert "Stabilize to_bytes and from_bytes for integers."](https://github.com/rust-lang/rust/pull/52850)
* [provide `{to,from}_{ne,le,be}_bytes` functions on integers](https://github.com/rust-lang/rust/pull/51919)
* [treat gc=No characters as numeric](https://github.com/rust-lang/rust/pull/51609)
* [implement inner deref for `Option` and `Result`](https://github.com/rust-lang/rust/pull/50267)
* [make `io::Read::read_to_end` consider `io::Take::limit`](https://github.com/rust-lang/rust/pull/52939)
* [use `SetLenOnDrop` in `Vec::truncate()`](https://github.com/rust-lang/rust/pull/52908)
* [Implement Unpin for FutureObj and LocalFutureObj](https://github.com/rust-lang/rust/pull/52870)
* [reexport tests without polluting namespaces](https://github.com/rust-lang/rust/pull/52890)
* [cargo: fix the edition build scripts are compiled with](https://github.com/rust-lang/cargo/pull/5861)
* [cargo: use listed dependency name for feature names](https://github.com/rust-lang/cargo/pull/5811)
* [cargo fully capture rustc and rustdoc output when `-Zcompile-progress` is passed](https://github.com/rust-lang/cargo/pull/5862)
* [cargo can silently fix some bad lockfiles (use `--locked` to disable)](https://github.com/rust-lang/cargo/pull/5831)
* [rustdoc: stabilize `--color` and `--error-format` options](https://github.com/rust-lang/rust/pull/53003)
* [rustdoc: make `everybody_loops` preserve item declarations](https://github.com/rust-lang/rust/pull/53002)
* [fix ICE when rustdoc encounters certain usages of HRTBs](https://github.com/rust-lang/rust/pull/52990)

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

* [disposition: merge] [Fix the Error trait](https://github.com/rust-lang/rfcs/pull/2504).
* [disposition: merge] [Add `is_sorted` to the standard library](https://github.com/rust-lang/rfcs/pull/2351).
* [disposition: merge] [Add `pub fn identity<T>(x: T) -> T { x }` to core::convert](https://github.com/rust-lang/rfcs/pull/2306).
* [disposition: merge] [if- and while-let-chains, take 2](https://github.com/rust-lang/rfcs/pull/2497).
* [disposition: merge] [Deprecate uninitialized in favor of a new MaybeUninit type](https://github.com/rust-lang/rfcs/pull/1892).
* [disposition: postpone] [Introduce panic_thin, a fmtless alternative to panic_fmt](https://github.com/rust-lang/rfcs/pull/2305).
* [disposition: close] [Add std::mem::zero](https://github.com/rust-lang/rfcs/pull/2291).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [(Modules) Tracking issue for the `mod.rs` changes](https://github.com/rust-lang/rust/issues/53125).
* [disposition: merge] [Allow to check if sync::Once is already initialized](https://github.com/rust-lang/rust/pull/53027).
* [disposition: merge] [Allow all literals in attributes (Tracking Issue for RFC #1559)](https://github.com/rust-lang/rust/issues/34981).
* [disposition: merge] [Tracking Issue for Iterator::find_map](https://github.com/rust-lang/rust/issues/49602).
* [disposition: close] [Define non-panicking UTF encoding methods on `char`](https://github.com/rust-lang/rust/pull/52580).

## New RFCs

* [Unify std::os::raw::c_void and libc::c_void via libcore](https://github.com/rust-lang/rfcs/pull/2521).
* [Generalized Type Ascription](https://github.com/rust-lang/rfcs/pull/2522).
* [#\[cfg(accessible(..) / version = ".." / nightly)\]](https://github.com/rust-lang/rfcs/pull/2523).

# Upcoming Events

### Online

* [Aug 22. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Aug 28. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Aug 29. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia

* [Aug 18. Chennai, IN - Rust Monthly Meetup](https://www.meetup.com/mad-rs/events/253751178/).

### Europe

* [Aug 16. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxlbvb/).
* [Aug 22. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/253062831/).

### North America

* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).**
* [Aug 19. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbzb/).
* [Aug 22. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxlblb/).
* [Aug 26. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxlbjc/).
* [Aug 27. Durham, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxlbkc/).
* [Aug 28. Chicago, US - Rust Meetup](https://www.meetup.com/Chicago-Rust-Meetup/events/253621611/).
* [Aug 28. Dallas, US - Rust Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxlblc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at Commure, Inc. (San Francisco, Boston, Montreal)](https://www.reddit.com/r/rust/comments/92e67g/commure_healthcare_software_startup_hiring_rust/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> We put in a lot of work to make upgrades painless; for example, we run a tool (called “crater”) before each Rust release that downloads every package on crates.io and attempts to build their code and run their tests.

– [Rust Blog: What is Rust 2018](https://blog.rust-lang.org/2018/07/27/what-is-rust-2018.html).

Thanks to [azriel91](https://users.rust-lang.org/u/azriel91) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
