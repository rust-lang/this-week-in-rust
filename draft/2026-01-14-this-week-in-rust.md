Title: This Week in Rust 634
Number: 634
Date: 2026-01-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

* [What is maintenance, anyway?](https://blog.rust-lang.org/inside-rust/2026/01/12/what-is-maintenance-anyway/)
* [Infrastructure Team 2025 Q4 Recap and Q1 2026 Plan](https://blog.rust-lang.org/inside-rust/2026/01/13/infrastructure-team-q4-2025-recap-and-q1-2026-plan/)

### Foundation

### Newsletters

* [🦀 Rust Wrapped 2025](https://weeklyrust.substack.com/p/rust-wrapped-2025)

### Project/Tooling Updates

* [Bevy 0.18](https://bevy.org/news/bevy-0-18/)
* [Exponential growth continued — cargo-semver-checks 2025 Year in Review](https://predr.ag/blog/cargo-semver-checks-2025-year-in-review/)
* [This Week in Slatron v1.1: The intelligent TV station manager introduces AI DJ hosts w/ TTS support for DIY hackable TV stations, radio, and digital signage. ](https://justinwoodring.com/blog/this-week-in-slatron-v1-1/)
* [SeaORM 2.0 Migration Guide](https://www.sea-ql.org/blog/2026-01-12-sea-orm-2.0/)
* [BugStalker v0.4.0 - Modern debugger for Linux x86-64. Written in Rust for Rust programs.](https://godzie44.github.io/BugStalker/docs/blog/release_0_4/) now with VSCode support.
* [Announcing cadd: a library for painless checked arithmetic and conversions](https://users.rust-lang.org/t/announcing-cadd-a-library-for-painless-checked-arithmetic-and-conversions/137423)
* [READ_ONCE(), WRITE_ONCE(), but not for Rust](https://lwn.net/SubscriberLink/1053142/8ec93e58d5d3cc06/)
* [GuardianDB 0.14.0 - High-performance, local-first decentralized database built on Rust and Iroh](https://www.willsearch.com.br/guardiandb/)
* [A year of work on the ALPM project](https://devblog.archlinux.page/2026/a-year-of-work-on-the-alpm-project/)
* [GlueSQL v0.19 adds parameter binding and customizable query planners](https://github.com/gluesql/gluesql/releases/tag/v0.19.0)
* [diesel-guard: Your Diesel Migrations Might Be Ticking Time Bombs](https://dev.to/ayarotsky/your-diesel-migrations-might-be-ticking-time-bombs-30g7).
* [The Rapier physics engine 2025 review and 2026 goals](https://dimforge.com/blog/2026/01/09/the-year-2025-in-dimforge/)
* [Tako v0.5.0 → v0.7.1-2: from "nice router" to "mini platform"](https://rust-dd.com/post/tako-v0-5-0-v0-7-1-2-from-nice-router-to-mini-platform)

### Observations/Thoughts

* [The State of Rust Cryptography in 2026](https://kerkour.com/rust-cryptography-ecosystem-2026)
* [Garbage collection is contrarian](https://trynova.dev/blog/garbage-collection-is-contrarian)
* [Virtual Places and Borrow Checker Integration](https://bennolossin.github.io/blog/field-projections/virtual-places-and-borrowck.html)
* [video] [39c3 - Xous: A Pure-Rust Rethink of the Embedded Operating System](https://www.youtube.com/watch?v=BbWWGkyIBGM)
* [video] [Fast and Safe Image Decoding in Rust](https://www.youtube.com/watch?v=8ANzF7UwbZM)
* [video] [ere: Compiling Regular Expressions at Build-Time](https://www.youtube.com/watch?v=3SFx-emI5r4)
* [video] [Rust at Volvo Cars](https://www.youtube.com/watch?v=vBofCW8j70A)
* [audio] [Radar with Jeff Kao](https://corrode.dev/podcast/s05e08-radar/)

### Rust Walkthroughs
[ES] [Command Pattern in Rust: When intent doesn't need to be an object](https://codigolinea.com/patron-command-en-rust/) 

* [series] [Part 3: Model Architecture, Building an LLM from Scratch in Rust](https://www.tag1.com/how-to/part3-model-architecture-building-an-llm-from-scratch/)
* [The Impatient Programmer’s Guide to Bevy and Rust: Chapter 5 - Let There Be Pickups](https://aibodh.com/posts/bevy-rust-game-development-chapter-5/)
* [audio] [Netstack.FM episode 22 — Rust URL with Simon Sapin](https://netstack.fm/#episode-22)

### Research

### Miscellaneous
* [IBM Quantum: Rust is Real, but Quantum Advantage is Not (Yet)](https://filtra.io/rust/interviews/ibm-quantum-jan-26)

## Crate of the Week

This week's crate is [diesel-guard](https://github.com/ayarotsky/diesel-guard), a linter against dangerous Postgres migrations.

Thanks to [Alex Yarotsky](https://users.rust-lang.org/t/crate-of-the-week/2704/1520) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

<!-- If there are new CfT items this week, include:

  [Repo Name](Repo URL)
    * [<Feature name>](<Feature URL>)
        * [Testing steps](<Testing Steps URL>)

  - and make note in the item so the authors know to remove the `call-for-testing` label:
This RFC will appear in the **Call for Testing** section of the next issue (#) of This Week in Rust (TWiR).
You may remove the `call-for-testing` label.  Please feel free to leave the `call-for-testing` label in place if you would like this RFC to appear again in another issue of TWiR.

  - where `Repo Name` and `Repo URL` are one of:
[Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
[Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
[Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

  - and `Testing steps` points directly to the procedures the item wants users to exercise.

  - For all `Repo Names` with no new CfT items this week: use (removing the repos for which new
     CfT items did appear, of course)

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*
-->

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Diesel - diesel print-schema produces uncompileable schema with column named "table"](https://github.com/diesel-rs/diesel/issues/4928)
* [GuardianDB - Create Benchmarks](https://github.com/wmaslonek/guardian-db/issues/7)
* [GuardianDB - Create cohesive usage examples](https://github.com/wmaslonek/guardian-db/issues/5)
* [GuardianDB - Translate documentation to English](https://github.com/wmaslonek/guardian-db/issues/3)
* [rung - Add shell completions for bash/zsh/fish](https://github.com/auswm85/rung/issues/18)
* [rung - Add --quiet flag to suppress non-essential output](https://github.com/auswm85/rung/issues/19)
* [rung - Support NO_COLOR environment variable](https://github.com/auswm85/rung/issues/20)
* [rung - Add rung top / rung bottom navigation commands](https://github.com/auswm85/rung/issues/21)
* [rung - Add rung log command to show stack commits](https://github.com/auswm85/rung/issues/22)
* [rung - Add integration test for sync with merge conflicts](https://github.com/auswm85/rung/issues/23)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

539 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-01-06..2026-01-13

#### Compiler
* [fix `Expr::can_have_side_effects` for `[x; N]` style array literal and binary expressions](https://github.com/rust-lang/rust/pull/150385)

#### Library
* [add `AtomicPtr::null`](https://github.com/rust-lang/rust/pull/150736)
* [`Vec::push` in consts MVP](https://github.com/rust-lang/rust/pull/147893)
* [add allocator parameter to `HashMap`](https://github.com/rust-lang/rust/pull/148545)
* [implement `partial_sort_unstable` for slice](https://github.com/rust-lang/rust/pull/149318)
* [reflection MVP](https://github.com/rust-lang/rust/pull/146923)
* [stabilize `Peekable::next_if_map` (`#![feature(peekable_next_if_map)]`)](https://github.com/rust-lang/rust/pull/148941)
* [stabilize `slice::element_offset`](https://github.com/rust-lang/rust/pull/150777)

#### Cargo
* [`docs(unstable)`: expand docs for `-Zbuild-analysis`](https://github.com/rust-lang/cargo/pull/16476)
* [`feat(test)`: Make `CARGO_BIN_EXE_` available at runtime](https://github.com/rust-lang/cargo/pull/16421)
* [`fix(build-std)`: std link metadata propagate to user](https://github.com/rust-lang/cargo/pull/16496)
* [`fix(info)`: resolve underscore vs hyphen mismatch in schema lookup](https://github.com/rust-lang/cargo/pull/16455)
* [`fix(package)`: detect dirty files when run from workspace member](https://github.com/rust-lang/cargo/pull/16479)
* [add Clippy like lint groups](https://github.com/rust-lang/cargo/pull/16464)
* [add `--id` flag to `cargo report timings` and `cargo report rebuilds`](https://github.com/rust-lang/cargo/pull/16490)
* [display lockfile path in very verbose mode when blocking](https://github.com/rust-lang/cargo/pull/16491)
* [feat: in-memory only `Manifest`](https://github.com/rust-lang/cargo/pull/16409)
* [fix(timing)!: remove `--timings=<FMT>` optional format values](https://github.com/rust-lang/cargo/pull/16420)
* [fix: preserve `dep_name` for build script metadata](https://github.com/rust-lang/cargo/pull/16494)
* [fixed incorrect version comparision during build script dependency selection](https://github.com/rust-lang/cargo/pull/16486)
* [improve error message for missing dependencies](https://github.com/rust-lang/cargo/pull/16500)
* [isolate build script metadata progation between std and non-std crates](https://github.com/rust-lang/cargo/pull/16489)
* [refactor: new type for unit index](https://github.com/rust-lang/cargo/pull/16485)
* [test: add `-Zunstable-options` with custom targets](https://github.com/rust-lang/cargo/pull/16467)

#### Rustdoc
* [`rustdoc_json`: Remove one call to `std::mem::take` in `after_krate`](https://github.com/rust-lang/rust/pull/150867)

#### Clippy
* [A `return` in an iterator closure should not trigger `never_loop`](https://github.com/rust-lang/rust-clippy/pull/16364)
* [`strlen_on_c_strings`: mention the specific type (`CString or `CStr`)`](https://github.com/rust-lang/rust-clippy/pull/16391)
* [`float_point_arithmetic`: respect reduced applicability](https://github.com/rust-lang/rust-clippy/pull/16366)
* [`single_range_in_vec_init`: don't apply the suggestion automatically](https://github.com/rust-lang/rust-clippy/pull/16365)
* [`unnecessary_map_or`: respect reduced applicability](https://github.com/rust-lang/rust-clippy/pull/16387)
* [`useless_conversion`: respect reduced applicability](https://github.com/rust-lang/rust-clippy/pull/16372)
* [`missing_enforced_import_rename`: Do not enforce for underscores](https://github.com/rust-lang/rust-clippy/pull/16352)
* [`suspicious_to_owned`: improve lint messages](https://github.com/rust-lang/rust-clippy/pull/16376)
* [`transmuting_null`: Add checks for `without_provenance` and `without_provenance_mut`](https://github.com/rust-lang/rust-clippy/pull/16336)
* [add new `duration_suboptimal_units` lint](https://github.com/rust-lang/rust-clippy/pull/16250)
* [allow `expect` on `impl` for `derive_ord_xor_partial_ord`](https://github.com/rust-lang/rust-clippy/pull/16303)
* [clean-up `unnecessary_map_or` and `manual_is_variant_and`](https://github.com/rust-lang/rust-clippy/pull/16386)
* [do not ignore statements before a `break` when simplifying loop](https://github.com/rust-lang/rust-clippy/pull/16379)
* [do not show spans from external crates in `missing_trait_methods`](https://github.com/rust-lang/rust-clippy/pull/16380)
* [do not warn about large stack arrays without having a valid span](https://github.com/rust-lang/rust-clippy/pull/16347)
* [do not warn on arithmetic side effect for `String`+`String`](https://github.com/rust-lang/rust-clippy/pull/16358)
* [enhance `needless_collect` to cover vec `push`-alike](https://github.com/rust-lang/rust-clippy/pull/16305)
* [fix `LimitStack::pop_atr` in release builds](https://github.com/rust-lang/rust-clippy/pull/16371)
* [fix `clippy_utils::std_or_core(_)` marking `no_core` crates as `std`](https://github.com/rust-lang/rust-clippy/pull/16374)
* [fix `map_unwrap_or` fail to cover `Result::unwrap_or`](https://github.com/rust-lang/rust-clippy/pull/15718)
* [fix `significant_drop_tightening` suggests wrongly for non-method usage](https://github.com/rust-lang/rust-clippy/pull/16355)
* [fix `str_to_string` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/16276)
* [fix `unnecessary_to_owned` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/16354)
* [fix: restrict `match_bool` to 2 arms](https://github.com/rust-lang/rust-clippy/pull/16333)
* [improve `useless_conversion .into_iter()` suggestion for nested references](https://github.com/rust-lang/rust-clippy/pull/16238)
* [more fixes for handling of macros](https://github.com/rust-lang/rust-clippy/pull/16337)
* [overhaul `int_plus_one`](https://github.com/rust-lang/rust-clippy/pull/16373)

#### Rust-Analyzer
* [add inherit attributes for `extract_function` assist](https://github.com/rust-lang/rust-analyzer/pull/21442)
* [configure flycheck using workspace.discoverConfig](https://github.com/rust-lang/rust-analyzer/pull/18043)
* [allow rust paths in symbol search](https://github.com/rust-lang/rust-analyzer/pull/21415)
* [fix ignore flag for test attributes with values](https://github.com/rust-lang/rust-analyzer/pull/21436)
* [fix loses exists guard for `move_guard`](https://github.com/rust-lang/rust-analyzer/pull/21412)
* [fix not applicable on statement for `convert_to_guarded_return`](https://github.com/rust-lang/rust-analyzer/pull/20946)
* [fix not complete `mut` and `raw` in `&x.foo()`](https://github.com/rust-lang/rust-analyzer/pull/21451)
* [fix not disable string escape highlights](https://github.com/rust-lang/rust-analyzer/pull/21420)
* [disable `unused_variables` and `unused_mut` warnings](https://github.com/rust-lang/rust-analyzer/pull/21445)
* [fix crate root search in world symbols duplicating root entries](https://github.com/rust-lang/rust-analyzer/pull/21446)
* [fix lifetimes len diagnostics for fn pointers](https://github.com/rust-lang/rust-analyzer/pull/21432)
* [fixes for builtin derive expansions](https://github.com/rust-lang/rust-analyzer/pull/21421)
* [hide renamed imports from macros in symbol index](https://github.com/rust-lang/rust-analyzer/pull/21459)
* [lowering crash with supertrait predicates](https://github.com/rust-lang/rust-analyzer/pull/21364)
* [make `naked_asm!()` always return `!`](https://github.com/rust-lang/rust-analyzer/pull/21456)
* [properly lower `SelfOnly` predicates](https://github.com/rust-lang/rust-analyzer/pull/21399)
* [remove code made redundant by method resolution rewrite](https://github.com/rust-lang/rust-analyzer/pull/21434)
* [suggest traits other than ones in the environment crate](https://github.com/rust-lang/rust-analyzer/pull/21414)
* [sync cast checks to rustc again](https://github.com/rust-lang/rust-analyzer/pull/21462)
* [implement `Span::ByteRange` for proc-macro-srv](https://github.com/rust-lang/rust-analyzer/pull/21416)
* [migrate `generate_mut_trait_impl` assist to use SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21443)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Use either
* [Item title](Item URL)
  - or
* *No RFCs were approved this week.*
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
<!-- Either remove the group from the "No Items Entered Final Comment Period this week for" section
     and add the item(s) which entered Final comment period:
##### [Group](Group URL)
* [Item title](Item URL)
  - for `disposition-merge` `final-comment-period` items, or
* [disposition: postpone]
  - for `disposition-postpone` `final-comment-period` items, or
* [disposition: close]
  - for `disposition-close` `final-comment-period` items,
* [disposition: unspecified]
  - when `disposition` is unspecified or ensure the group is a part of the
     "No Items Entered Final Comment Period this week for" section
*No Items entered Final Comment Period this week for
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.
-->

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
<!-- Use either
* [Item title](Item URL)
  - for new items, or
* [updated] [Item title](Item URL)
  - for updated items, or
* *No New or Updated RFCs were created this week.*
-->

<!-- Sample commit message
Update CFT, FCP, MCP and RFC sections for TWiR-xxx
-->

## Upcoming Events

Rusty Events between 2026-01-14 - 2026-02-11 🦀

### Virtual
* 2026-01-07 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/4p6rxjc5)
* 2026-01-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312102790/)
* 2026-01-08 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Meet, swap, and learn!**](https://www.meetup.com/charlottesville-rust-meetup/events/312321120/)
* 2026-01-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312379275/)
* 2026-01-13 | Virtual | [libp2p Events](https://luma.com/libp2p)
    * [**rust-libp2p Open Maintainers Call**](https://luma.com/xov10pef)
* 2026-01-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254791/)
* 2026-01-13 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Rust code reading and open source contribution (UTC 16:00; English)**](https://www.meetup.com/code-mavens/events/312641560/)
* 2026-01-15 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**Janurary, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-15 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-18 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Rust code reading and open source contribution (UTC 16:00; English)**](https://www.meetup.com/code-mavens/events/312710291/)
* 2026-01-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/312489197/)
* 2026-01-21 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/e2ea7hxo)
* 2026-01-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Stack Safety**](https://www.meetup.com/vancouver-rust/events/310619449/)
* 2026-01-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254790/)
* 2026-01-28 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/9h9n0dau)
* 2026-01-29 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #12**](https://www.meetup.com/bevy-game-development/events/312681343/)
* 2026-01-29 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455921/)
* 2026-01-29 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #2 - Rusty processes, memory limits, and basic capsules**](https://www.meetup.com/charlottesville-rust-meetup/events/312326112/)
* 2026-02-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312187422/)

### Asia
* 2026-01-07 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust January 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)
* 2026-01-08 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/312645994/)
* 2026-01-17 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #12**](https://www.meetup.com/rustdelhi/events/312584516/)

### Europe
* 2026-01-07 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/312497150/)
* 2026-01-08 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-01-14 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)
* 2026-01-16 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/312662987/)
* 2026-01-20 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592260/)
* 2026-01-20 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #82**](https://www.meetup.com/rust-paris/events/312364675/)
* 2026-01-21 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/312749221/)
* 2026-01-26 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #17**: Emily Coaca - Entwicklung des Kernels Update für TockOS](https://rust-augsburg.github.io/meetup/Meetup_17.html)
* 2026-01-28 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - January 2026**](https://www.meetup.com/rust-dortmund/events/312485262/)
* 2026-02-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Beyond the Code: Designing Services That Stand the Test of Time**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)

### North America
* 2026-01-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Bringing the data lake to the browser with WASM**](https://www.meetup.com/utah-rust/events/312565472/)
* 2026-01-08 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyjccblb/)
* 2026-01-08 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust)
    * [**Toasty: An Easy-to-use Object-Relational Mapping for Rust**](https://www.meetup.com/pdxrust/events/312694009/)
* 2026-01-08 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust January Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/312675582/)
* 2026-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Central Cambridge Rust Lunch, Jan 10**](https://www.meetup.com/bostonrust/events/312483605/)
* 2026-01-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC Panel: AI Coding Agents in Rust: What Works, What Breaks**](https://www.meetup.com/rust-nyc/events/312632598/)
* 2026-01-14 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/312722481/)
* 2026-01-15 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**Janurary, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-17 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston Common Rust Lunch, Jan 17**](https://www.meetup.com/bostonrust/events/312483677/)
* 2026-01-17 | Herndon, VA, US | [NoVaLUG](https://mobilizon.us/@novalug)
    * [**Monthly Meeting - Make Brian Lunduke Angry, Program in Rust**](https://mobilizon.us/events/140c5c7c-01f3-4aaa-b218-58289c6b4449)
* 2026-01-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/310403081/)
* 2026-01-21 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312185794/)
* 2026-01-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Evening Boston Rust Meetup with Jiff, January 22**](https://www.meetup.com/bostonrust/events/312598080/)
* 2026-01-22 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312692728/)
* 2026-01-24 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Davis Square Rust Lunch, Jan 24**](https://www.meetup.com/bostonrust/events/312483715/)
* 2026-01-28 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust Los Angeles: Building Git-LFS Replacements in Rust**](https://www.meetup.com/rust-los-angeles/events/312267194/)
* 2026-01-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/308676002/)
* 2026-01-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Rust Programming 101**](https://www.meetup.com/music-city-rust-developers/events/312038621/)
* 2026-01-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Northeastern Rust Lunch, Jan 31**](https://www.meetup.com/bostonrust/events/312483767/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> I have written in dozens of computer languages, including specialized ones that were internal to Pixar (including one I designed). I spent decades writing C and C++. I wrote bit-slice microcode, coded for SIMD before many folks outside of Pixar had it.
> 
> I wrote the first malloc debugger that would stop your debugger at the source code line that was the problem. Unix workstation manufacturers had to do an unexpected release when this revealed all of the problems in their C libraries.
> 
> I am a better programmer in Rust for anything low-level or high-performance. It just keeps me from making an entire class of mistakes that were too easy to make in any language without garbage-collection.
> 
> Over the long term, anything that improves quality is going to win. There is a lot of belly-aching by folks who are too in love with what they've been using for decades, but it is mostly substance-free. Like people realizing that code marked "unsafe" is, surprise, unsafe. And that unsafe can be abused.

– [Bruce Perens on LinkedIn](https://www.linkedin.com/posts/bruce-perens_i-have-written-in-dozens-of-computer-languages-activity-7413127858266734592-iMc5)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1746) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
