Title: This Week in Rust 545
Number: 545
Date: 2024-05-01
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

* [r3bl_trerminal_async v0.5.1 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v051-2024-04-28)

### Observations/Thoughts

* [Writing ergonomic async assertions in Rust](https://www.niklaslong.ch/deadline/)
* [Making an HTML parsing script a hundred times faster with Rayon](https://medium.com/@sam.van.overmeire/making-an-html-parsing-script-a-hundred-times-faster-with-rayon-5ed952c9011c)
* [Rust binaries stability](https://mirekdlugosz.com/blog/2024/rust-binaries-stability/)
* [audio] [Ratatui with Orhun Parmaksiz :: Rustacean Station](https://rustacean-station.org/episode/orhun-parmaksiz/)

### Rust Walkthroughs
* [Zed Decoded: Rope & SumTree](https://zed.dev/blog/zed-decoded-rope-sumtree)
* [Boosting Dev Experience with Serverless Rust in RustRover](https://blog.jetbrains.com/rust/2024/04/26/boosting-dev-experience-with-serverless-rust-in-rustrover/)

* [developerlife.com - Rust Polymorphism, dyn, impl, using existing traits, trait objects for testing and extensibility](https://developerlife.com/2024/04/28/rust-polymorphism-dyn-impl-trait-objects-for-testing-and-extensibiity/)

### Research

### Miscellaneous
- [Writing A Wasm Runtime In Rust](https://skanehira.github.io/writing-a-wasm-runtime-in-rust/)

## Crate of the Week

This week's crate is [efs](https://codeberg.org/RatCornu/efs), a no-std ext2 filesystem implementation with plans to add other file systems in the future.

Another week completely devoid of suggestions, but llogiq stays hopeful he won't have to dig for next week's crate all by himself.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

409 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-04-23..2024-04-30

* [abort a process when FD ownership is violated](https://github.com/rust-lang/rust/pull/124210)
* [add support for run-make-support unit tests to be run with bootstrap](https://github.com/rust-lang/rust/pull/124321)
* [ast: generalize item kind visiting](https://github.com/rust-lang/rust/pull/124382)
* [coverage: avoid hard-coded values when visiting logical ops](https://github.com/rust-lang/rust/pull/124508)
* [coverage: replace boolean options with a `CoverageLevel enum`](https://github.com/rust-lang/rust/pull/124507)
* [debuginfo: stabilize `-Z debug-macros`, `-Z collapse-macro-debuginfo` and `#[collapse_debuginfo]`](https://github.com/rust-lang/rust/pull/120845)
* [delegation: support renaming, and async, const, extern "ABI" and C-variadic functions](https://github.com/rust-lang/rust/pull/122500)
* [deny gen keyword in `edition_2024_compat` lints](https://github.com/rust-lang/rust/pull/123680)
* [deref patterns: lower deref patterns to MIR](https://github.com/rust-lang/rust/pull/122598)
* [detect borrow error involving sub-slices and suggest `split_at_mut`](https://github.com/rust-lang/rust/pull/124313)
* [disallow ambiguous attributes on expressions](https://github.com/rust-lang/rust/pull/124099)
* [do not ICE on invalid consts when walking mono-reachable blocks](https://github.com/rust-lang/rust/pull/124425)
* [don't ICE when `codegen_select_candidate` returns ambiguity in new solver](https://github.com/rust-lang/rust/pull/124374)
* [don't fatal when calling `expect_one_of` when recovering arg in `parse_seq`](https://github.com/rust-lang/rust/pull/124169)
* [enforce closure args + return type are WF](https://github.com/rust-lang/rust/pull/123531)
* [fix ICE on invalid const param types](https://github.com/rust-lang/rust/pull/124394)
* [fix ICE when ADT tail has type error](https://github.com/rust-lang/rust/pull/124057)
* [fix weak memory bug in TLS on Windows](https://github.com/rust-lang/rust/pull/124281)
* [improve diagnostic for unknown `--print` request](https://github.com/rust-lang/rust/pull/124333)
* [improve handling of expr→field errors](https://github.com/rust-lang/rust/pull/124200)
* [mark unions non-const-propagatable in `KnownPanicsLint` without calling layout](https://github.com/rust-lang/rust/pull/124504)
* [pretty-print parenthesis around binary in postfix match](https://github.com/rust-lang/rust/pull/124269)
* [provide more context and suggestions in borrowck errors involving closures](https://github.com/rust-lang/rust/pull/124136)
* [record certainty of `evaluate_added_goals_and_make_canonical_response` call in candidate](https://github.com/rust-lang/rust/pull/124444)
* [remove special-casing for `SimplifiedType` for next solver](https://github.com/rust-lang/rust/pull/124379)
* [rename `inhibit_union_abi_opt()` to `inhibits_union_abi_opt()`](https://github.com/rust-lang/rust/pull/124463)
* [renamed `DerivedObligation` to `WellFormedDeriveObligation`](https://github.com/rust-lang/rust/pull/124381)
* [require explicitly marking closures as coroutines](https://github.com/rust-lang/rust/pull/123792)
* [restrict promotion of `const fn` calls](https://github.com/rust-lang/rust/pull/121557)
* [set writable and `dead_on_unwind` attributes for sret arguments](https://github.com/rust-lang/rust/pull/121298)
* [strengthen tracking issue policy with consequences](https://github.com/rust-lang/rust/pull/124334)
* [suggest ref mut for pattern matching assignment](https://github.com/rust-lang/rust/pull/119650)
* [suggest using type args directly instead of equality constraint](https://github.com/rust-lang/rust/pull/122591)
* [use fulfillment in method probe, not evaluation](https://github.com/rust-lang/rust/pull/122317)
* [use probes more aggressively in new solver](https://github.com/rust-lang/rust/pull/124415)
* [weak lang items are not allowed to be `#[track_caller]`](https://github.com/rust-lang/rust/pull/124067)
* [miri: detect wrong vtables in wide pointers](https://github.com/rust-lang/rust/pull/124220)
* [miri: `unix_sigpipe`: don't inline DEFAULT, just use it from rustc](https://github.com/rust-lang/miri/pull/3510)
* [miri: add `-Zmiri-env-set` to set environment variables without modifying the host environment](https://github.com/rust-lang/miri/pull/3493)
* [miri env: split up Windows and Unix environment variable handling](https://github.com/rust-lang/miri/pull/3517)
* [miri: file descriptors: make write take &mut self](https://github.com/rust-lang/miri/pull/3524)
* [miri: implement LLVM x86 AVX2 intrinsics](https://github.com/rust-lang/miri/pull/3492)
* [miri: make miri-script a workspace root](https://github.com/rust-lang/miri/pull/3512)
* [miri: use the interpreted program's TZ variable in `localtime_r`](https://github.com/rust-lang/miri/pull/3523)
* [miri: windows: basic support for GetUserProfileDirectoryW](https://github.com/rust-lang/miri/pull/3502)
* [stabilise `inline_const`](https://github.com/rust-lang/rust/pull/104087)
* [stabilize `Utf8Chunks`](https://github.com/rust-lang/rust/pull/123909)
* [stabilize `non_null_convenience`](https://github.com/rust-lang/rust/pull/124498)
* [stabilize `std::path::absolute`](https://github.com/rust-lang/rust/pull/124335)
* [stabilize `io_error_downcast`](https://github.com/rust-lang/rust/pull/124076)
* [deLLVMize some intrinsics (use `u32` instead of `Self` in some integer intrinsics)](https://github.com/rust-lang/rust/pull/124003)
* [stop using LLVM `struct` types for alloca](https://github.com/rust-lang/rust/pull/122053)
* [`thread_local`: be excruciatingly explicit in dtor code](https://github.com/rust-lang/rust/pull/124387)
* [fix `offset_of!` returning a temporary](https://github.com/rust-lang/rust/pull/124484)
* [relax `A: Clone` bound for `rc::Weak::into_raw_and_alloc`](https://github.com/rust-lang/rust/pull/124432)
* [`PathBuf`: replace transmuting by accessor functions](https://github.com/rust-lang/rust/pull/124410)
* [codegen\_gcc: some fixes for aarch64](https://github.com/rust-lang/rustc_codegen_gcc/pull/504)
* [codegen\_gcc: some more fixes and workarounds for Aarch64](https://github.com/rust-lang/rustc_codegen_gcc/pull/508)
* [cargo: alias: Aliases without subcommands should not panic](https://github.com/rust-lang/cargo/pull/13819)
* [cargo: lints: Don't always inherit workspace lints](https://github.com/rust-lang/cargo/pull/13812)
* [cargo install: Don't respect MSRV for non-local installs](https://github.com/rust-lang/cargo/pull/13790)
* [cargo toml: Be more forceful with underscore/dash redundancy](https://github.com/rust-lang/cargo/pull/13798)
* [cargo toml: Don't double-warn when underscore is used in workspace dep](https://github.com/rust-lang/cargo/pull/13800)
* [cargo toml: Remove underscore field support in 2024](https://github.com/rust-lang/cargo/pull/13804)
* [cargo toml: Warn, rather than fail publish, if a target is excluded](https://github.com/rust-lang/cargo/pull/13713)
* [cargo toml: remove support for inheriting badges](https://github.com/rust-lang/cargo/pull/13788)
* [cargo: note where lint was set](https://github.com/rust-lang/cargo/pull/13801)
* [cargo: cleanup linting system](https://github.com/rust-lang/cargo/pull/13797)
* [cargo: fix target entry in .gitignore](https://github.com/rust-lang/cargo/pull/13817)
* [cargo: fix warning suppression for config.toml vs config compat symlinks](https://github.com/rust-lang/cargo/pull/13793)
* [bindgen: add dynamic loading of variable](https://github.com/rust-lang/rust-bindgen/pull/2812)
* [bindgen: remove which dependency](https://github.com/rust-lang/rust-bindgen/pull/2809)
* [bindgen: simplify Rust to Clang target conversion](https://github.com/rust-lang/rust-bindgen/pull/2808)
* [clippy: `single_match`(`_else`) may be machine applicable](https://github.com/rust-lang/rust-clippy/pull/12726)
* [clippy: `non_canonical_partial_ord_impl`: Fix emitting warnings which conflict with `needless_return`](https://github.com/rust-lang/rust-clippy/pull/12702)
* [clippy: `type_complexity`: Fix duplicate errors](https://github.com/rust-lang/rust-clippy/pull/12736)
* [clippy: check if closure as method arg has read access in `collection_is_never_read`](https://github.com/rust-lang/rust-clippy/pull/12694)
* [clippy: configurably allow `useless_vec` in tests](https://github.com/rust-lang/rust-clippy/pull/12725)
* [clippy: fix `large_stack_arrays` linting in `vec` macro](https://github.com/rust-lang/rust-clippy/pull/12624)
* [clippy: fix false positive in `cast_possible_truncation`](https://github.com/rust-lang/rust-clippy/pull/12722)
* [clippy: suppress `readonly_write_lock` for underscore-prefixed bindings](https://github.com/rust-lang/rust-clippy/pull/12734)
* [rust-analyzer: different error code of "no such field" error based on variant type](https://github.com/rust-lang/rust-analyzer/pull/17131)
* [rust-analyzer: don't retry position relient requests and version resolve data](https://github.com/rust-lang/rust-analyzer/pull/17157)
* [rust-analyzer: fix attributes on generic parameters colliding in item tree](https://github.com/rust-lang/rust-analyzer/pull/17151)
* [rust-analyzer: fix doc comment desugaring for proc-macros](https://github.com/rust-lang/rust-analyzer/pull/17153)
* [rust-analyzer: fix expression scopes not being calculated for inline consts](https://github.com/rust-lang/rust-analyzer/pull/17135)
* [rust-analyzer: fix source roots not always being created when necessary](https://github.com/rust-lang/rust-analyzer/pull/17145)
* [rust-analyzer: make `cargo run` always available for binaries](https://github.com/rust-lang/rust-analyzer/pull/16972)
* [rust-analyzer: manual: remove suggestion of rust-project.json example](https://github.com/rust-lang/rust-analyzer/pull/17144)
* [rust-analyzer: support hovering limits for adts](https://github.com/rust-lang/rust-analyzer/pull/17021)
* [rustfmt: fix wrong indentation on inner attribute](https://github.com/rust-lang/rustfmt/pull/6148)

### Rust Compiler Performance Triage

Several non-noise changes this week, with both improvements and regresions
coming as a result. Overall compiler performance is roughly neutral across the
week.

Triage done by **@simulacrum**.
Revision range: [a77f76e2..c65b2dc9](https://perf.rust-lang.org/?start=a77f76e26302e9a084fb321817675b1dfc1dcd63&end=c65b2dc935c27c0c8c3997c6e8d8894718a2cb1a&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 3 Mixed; 1 of them in rollups
51 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-04-29.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [experimental project goal program for 2024 H2](https://github.com/rust-lang/rfcs/pull/3614)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [Precise capturing](https://github.com/rust-lang/rfcs/pull/3617)
* [disposition: merge] [Unsafe Extern Blocks](https://github.com/rust-lang/rfcs/pull/3484)
* [disposition: merge] [MaybeDangling](https://github.com/rust-lang/rfcs/pull/3336)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Add `Option::take_if`](https://github.com/rust-lang/rust/issues/98934)
* [disposition: merge] [elaborate obligations in coherence](https://github.com/rust-lang/rust/pull/124532)
* [disposition: merge] [Allow coercing functions whose signature differs in opaque types in their defining scope into a shared function pointer type](https://github.com/rust-lang/rust/pull/124297)
* [disposition: merge] [Let's `#[expect]` some lints: `Stabilize lint_reasons` (RFC 2383)](https://github.com/rust-lang/rust/pull/120924)
* [disposition: merge] [Tracking Issue for ASCII trim functions on byte slices](https://github.com/rust-lang/rust/issues/94035)
* [disposition: merge] [Add `IntoIterator` for `Box<[T]>` + edition 2024-specific lints](https://github.com/rust-lang/rust/pull/124097)
* [disposition: merge] [Add `Box<[T; N]>: IntoIterator` without any method dispatch hacks](https://github.com/rust-lang/rust/pull/124108)
* [disposition: merge] [rustdoc-search: search for references](https://github.com/rust-lang/rust/pull/124148)
* [disposition: close] [Extra trait bound makes function body fail to typecheck](https://github.com/rust-lang/rust/issues/82219)
* [disposition: merge] [Make casts of pointers to trait objects stricter](https://github.com/rust-lang/rust/pull/120248)
* [disposition: merge] [Tracking Issue for split_at_checked](https://github.com/rust-lang/rust/issues/119128)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Precise capturing](https://github.com/rust-lang/rfcs/pull/3617)

## Upcoming Events

Rusty Events between 2024-05-01 - 2024-05-29 🦀

### Virtual

* 2024-05-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 5 - Project Structure**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325526/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)
* 2024-05-02 | Virtual (Aarhus, DK) | [Rust Aarhus Organizers](https://www.meetup.com/rust-aarhus-organizers/)
    * [**Rust Aarhus Organizers: Status**](https://www.meetup.com/rust-aarhus-organizers/events/300416935/)
* 2024-05-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368804/)
* 2024-05-02 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Women in Rust: Lunch & Learn! (Virtual)**](https://www.meetup.com/women-in-rust/events/300208946/)
* 2024-05-07 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)
* 2024-05-09 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477697/)
* 2024-05-09 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Rust at Microsoft, Tel Aviv - Are we embedded yet?**](https://www.meetup.com/code-mavens/events/300144781/)
* 2024-05-09 | Virtual (Nuremberg/Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945257/)
* 2024-05-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341699/)
* 2024-05-14 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/300437775/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-05-16 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346490/)
* 2024-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477699/)
* 2024-05-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/300533392/)

### Africa

* 2024-05-04 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Asia

* 2024-05-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2024-rustacean-meetup/)

### Europe

* 2024-05-01 | Köln/Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, May**](https://www.meetup.com/rustcologne/events/300610856/)
* 2024-05-01 | Utrecht, NL | [NL-RSE Community](https://nl-rse.org/events/2024-05-01-meetup)
    * [**NL-RSE RUST meetup**](https://www.eventbrite.nl/e/nl-rse-rust-meetup-tickets-871056271757)
* 2024-05-06 | Delft, NL | [GOSIM](https://www.gosim.org/)
    * [**GOSIM Europe 2024**](https://europe2024.gosim.org/)
* 2024-05-07 & 2024-05-08 | Delft, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2024**](https://2024.rustnl.org/)
* 2024-05-07 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**More Rust - Generics, constraints, safety.**](https://www.meetup.com/oxford-rust-meetup-group/events/300567559/)
* 2024-05-08 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/300573716/)
* 2024-05-09 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #2**](https://www.meetup.com/rust-gdansk/events/299766774/)
* 2024-05-14 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn May 2024**](https://www.meetup.com/rust-london-user-group/events/300715979/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-14 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (May 2024)**](https://www.meetup.com/rust-prague/events/300566374/)
* 2024-05-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/299694474/)
* 2024-05-16 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #7**](https://www.meetup.com/rust-meetup-augsburg/events/300174327/)
* 2024-05-16 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #68**](https://mobilizon.fr/events/14b51ccc-211f-400f-9615-707d9d871e78)
* 2024-05-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Save the date - Mai Meetup**](https://www.meetup.com/rust-zurich/events/300513957/)
* 2024-05-22 | Leiden, NL | [Future-proof Software Development by FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Coding Dojo Session**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 2024-05-23 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 2024-05-24 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discussions**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)

### North America

* 2024-05-04 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, May 4**](https://www.meetup.com/bostonrust/events/300116701/)
* 2024-05-08 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Social - Ann Arbor**](https://www.meetup.com/detroitrust/events/300763859/)
* 2024-05-09 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300020003/)
* 2024-05-12 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, May 12**](https://www.meetup.com/bostonrust/events/300116747/)
* 2024-05-14 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/300744140/)
* 2024-05-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 2024-05-20 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, May 20**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)
* 2024-05-25 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk Double Feature**](https://www.meetup.com/deep-dish-rust/events/300665520/)

### Oceania

* 2024-05-02 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**May Meetup**](https://www.meetup.com/rust-brisbane/events/300647409/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> "I'll never!"
> "No, never is in the 2024 Edition."
> "But never can't be this year, it's never!"
> "Well we're trying to make it happen now!"
> "But never isn't now?" "I mean technically, now never is the unit."
> "But how do you have an entire unit if it never happens?"

– [Jubilee on Zulip](https://rust-lang.zulipchat.com/#narrow/stream/268952-edition/topic/should.20have.20been.202025.20edition/near/435845944)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1565) for the suggestion! 

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
