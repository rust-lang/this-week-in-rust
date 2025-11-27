Title: This Week in Rust 627
Number: 627
Date: 2025-11-26
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
* [Switching to Rust's own mangling scheme on nightly | Rust Blog](https://blog.rust-lang.org/2025/11/20/switching-to-v0-mangling-on-nightly/)
* [Interview with Jan David Nose | Rust Blog](https://blog.rust-lang.org/2025/11/25/interview-with-jan-david-nose/)
* [This Development-cycle in Cargo: 1.92 | Inside Rust Blog](https://blog.rust-lang.org/inside-rust/2025/11/24/this-development-cycle-in-cargo-1.92/)

### Foundation
* [video] [Interview: Christian Legnitto, Maintainer: rust-gpu, rust-cuda](https://www.youtube.com/watch?v=monOq_uHHcg)

### Project/Tooling Updates
* [SeaORM 2.0: Nested ActiveModel and Cascade Operations](https://www.sea-ql.org/blog/2025-11-25-sea-orm-2.0/)
* [Symbolica 1.0: Symbolic mathematics in Rust](https://symbolica.io/posts/stable_release/)
* [APT Rust requirement raises questions](https://lwn.net/SubscriberLink/1046841/c7ac9fabff6244af/)

### Observations/Thoughts
* [Running real-time Rust](https://tweedegolf.nl/en/blog/198/running-real-time-rust)
* [How Cloudflare uses Rust to serve (and break) millions of websites at 50+ millions requests per second](https://kerkour.com/how-cloudflare-uses-rust)
* [audio] [Netstack.FM episode 15 — Pingora with Edward and Noah from Cloudflare](https://netstack.fm/#episode-15)
* [video] [Grind: Java Deserves Modern Tooling*](https://www.youtube.com/watch?v=-mOby4FPRXg)

### Rust Walkthroughs
* [Rust Unit Testing: File reading](https://jorgeortiz.dev/posts/rust_unit_testing_file_reading/)
* [A look at Rust from 2012](https://purplesyringa.moe/blog/a-look-at-rust-from-2012/)
* [Practical Performance Lessons from Apache DataFusion](https://greptime.com/blogs/2025-11-25-datafusion)
* [Describing binary data with Deku](https://codeconstruct.com.au/docs/deku-elf-parser/)
* [Making the case that Cargo features could be improved to alleviate Rust compile times](https://saghm.com/cargo-features-rust-compile-times/)
* [filtra.io | Toyota's "Tip Of The Spear" Is Choosing Rust](https://filtra.io/rust/interviews/woven-by-toyota-nov-25)

### Miscellaneous
* [Rust For Linux Kernel Co-Maintainer Formally Steps Down](https://www.phoronix.com/news/Alex-Gaynor-Rust-Maintainer)
* [JetBrains supports the open source Rust projects Ratatui and Biome](https://blog.jetbrains.com/blog/2025/11/18/open-source-in-focus-projects-we-re-proud-to-support/)

## Crate of the Week

This week's crate is [grapheme-utils](https://github.com/rustkins/grapheme-utils), a library of functions to ergonomically work with UTF graphemes.

Thanks to [rustkins](https://users.rust-lang.org/t/crate-of-the-week/2704/1495) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

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
*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->
* [**Rustikon 2026**](https://sessionize.com/rustikon-2026/)| CFP closes 2025-11-24 | Warsaw, Poland | 2025-03-19 - 2025-03-20 | [Event Website](https://www.rustikon.dev/)
* [**TokioConf 2026**](https://tokio.rs/blog/2025-09-26-announcing-tokio-conf-cfp)| CFP closes 2025-12-08 | Portland, Oregon, USA | 2026-04-20
* [**RustWeek 2026**](https://sessionize.com/rustweek-2026/)| CFP closes 2025-12-31 | Utrecht, The Netherlands | 2026-05-19 - 2026-05-20



If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

456 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-11-18..2025-11-25

 #### Compiler
* [allow unnormalized types in drop elaboration](https://github.com/rust-lang/rust/pull/148719)
* [avoid encoding non-constness or non-asyncness in metadata](https://github.com/rust-lang/rust/pull/149054)
* [fix MaybeUninit codegen using GVN](https://github.com/rust-lang/rust/pull/147827)
* [fix suggestion for the `cfg!` macro](https://github.com/rust-lang/rust/pull/148484)
* [handle cycles when checking impl candidates for `doc(hidden)`](https://github.com/rust-lang/rust/pull/149185)
* [inherent const impl](https://github.com/rust-lang/rust/pull/148434)
* [recommend using a HashMap if a HashSet's second generic parameter doesn't implement BuildHasher](https://github.com/rust-lang/rust/pull/147171)
* [reduce confusing `unreachable_code` lints](https://github.com/rust-lang/rust/pull/149044)
* [replace OffsetOf by an actual sum of calls to intrinsic](https://github.com/rust-lang/rust/pull/148151)
* [sess: default to v0 symbol mangling on nightly](https://github.com/rust-lang/rust/pull/89917)
* [turn moves into copies after copy propagation](https://github.com/rust-lang/rust/pull/147804)
* [warn against calls which mutate an interior mutable `const`-item](https://github.com/rust-lang/rust/pull/148407)
 #### Library
* [add `bit_width` for unsigned `NonZero<T>`](https://github.com/rust-lang/rust/pull/148797)
* [alloc: fix `Debug` implementation of `ExtractIf`](https://github.com/rust-lang/rust/pull/147035)
* [make SIMD intrinsics available in `const`-contexts](https://github.com/rust-lang/rust/pull/147521)
* [match `<OsString` as `Debug>::fmt` to that of str](https://github.com/rust-lang/rust/pull/148798)
* [see if this is the time we can remove `layout::size_align`](https://github.com/rust-lang/rust/pull/149109)
* [unwrap ret ty of `iter::ArrayChunks::into_remainder`](https://github.com/rust-lang/rust/pull/149127)
* [v0 mangling for std on nightly](https://github.com/rust-lang/rust/pull/149148)
* [hashbrown: add `HashTable` methods related to the raw bucket index](https://github.com/rust-lang/hashbrown/pull/657)
* [hashbrown: allow providing the key at insertion time for EntryRef](https://github.com/rust-lang/hashbrown/pull/579)
 #### Cargo
* [`docs(guide)`: When suggesting alt dev profile, link to related issue](https://github.com/rust-lang/cargo/pull/16275)
* [`feat(generate-lockfile)`: Add unstable --publish-time flag](https://github.com/rust-lang/cargo/pull/16265)
* [`feat(tree)`: Add more native completions](https://github.com/rust-lang/cargo/pull/16296)
* [`fix(bindeps)`: do not propagate artifact dependency to proc macro or build deps](https://github.com/rust-lang/cargo/pull/15788)
* [`fix(config-include)`: disallow glob and template syntax](https://github.com/rust-lang/cargo/pull/16285)
* [`fix(package)`: exclude target/package from backups](https://github.com/rust-lang/cargo/pull/16272)
* [`refactor(timings)`: separate data collection and presentation](https://github.com/rust-lang/cargo/pull/16282)
* [`test(config-include)`: include always relative to including config](https://github.com/rust-lang/cargo/pull/16286)
* [enable `CARGO_CFG_DEBUG_ASSERTIONS` in build scripts based on profile](https://github.com/rust-lang/cargo/pull/16160)
* [feat: emit a warning when both `package.publish` and `--index` are specified](https://github.com/rust-lang/cargo/pull/16268)
* [test: re-enable test since not flaky anymore](https://github.com/rust-lang/cargo/pull/16287)
 #### Rustdoc
* [rustdoc-json: add rlib path to ExternalCrate to enable robust crate resolution](https://github.com/rust-lang/rust/pull/149043)
* [rustdoc: make mergeable crate info more usable](https://github.com/rust-lang/rust/pull/148234)
 #### Clippy
* [`explicit_deref_methods`: don't lint in `impl Deref(Mut)`](https://github.com/rust-lang/rust-clippy/pull/16113)
* [add `large-error-ignored` config-knob](https://github.com/rust-lang/rust-clippy/pull/15697)
* [fix `useless_asref` suggests wrongly when used in ctor](https://github.com/rust-lang/rust-clippy/pull/16115)
* [fix wrongly unmangled macros for `transmute_ptr_to_ptr` and `transmute_bytes_to_str`](https://github.com/rust-lang/rust-clippy/pull/16105)
* [taking a raw pointer on a union field is a safe operation](https://github.com/rust-lang/rust-clippy/pull/16079)
 #### Rust-Analyzer
* [add `unsafe(…)` attribute completion](https://github.com/rust-lang/rust-analyzer/pull/21047)
* [add pretty number for `add_explicit_enum_discriminant`](https://github.com/rust-lang/rust-analyzer/pull/20559)
* [add semantic tokens for deprecated items](https://github.com/rust-lang/rust-analyzer/pull/21100)
* [add deprecated semantic token for extern crate shorthand](https://github.com/rust-lang/rust-analyzer/pull/21116)
* [add assist to convert char literal](https://github.com/rust-lang/rust-analyzer/pull/21093)
* [allow inferring array sizes](https://github.com/rust-lang/rust-analyzer/pull/21061)
* [basic support for declarative attribute/derive macros](https://github.com/rust-lang/rust-analyzer/pull/21121)
* [completion `= $0` after keyval cfg predicate](https://github.com/rust-lang/rust-analyzer/pull/21083)
* [derive ParamEnv from GenericPredicates](https://github.com/rust-lang/rust-analyzer/pull/21059)
* [don't suggest duplicate `const` completions `raw`](https://github.com/rust-lang/rust-analyzer/pull/20937)
* [enhance `remove_parentheses` assist to handle return expressions](https://github.com/rust-lang/rust-analyzer/pull/21090)
* [extract function panics on more than one usage of variable in macro](https://github.com/rust-lang/rust-analyzer/pull/21053)
* [fix hit `incorrect_case` on `no_mangle` static items](https://github.com/rust-lang/rust-analyzer/pull/21048)
* [fix not applicable on `and` for `replace_method_eager_lazy`](https://github.com/rust-lang/rust-analyzer/pull/20967)
* [fix not fill guarded match arm for `add_missing_match_arms`](https://github.com/rust-lang/rust-analyzer/pull/21111)
* [fix trailing newline in `tool_path`](https://github.com/rust-lang/rust-analyzer/pull/21088)
* [fix field completion in irrefutable patterns](https://github.com/rust-lang/rust-analyzer/pull/21065)
* [fix formatting request blocking on `crate_def_map` query](https://github.com/rust-lang/rust-analyzer/pull/21084)
* [fix parameter info with missing arguments](https://github.com/rust-lang/rust-analyzer/pull/21126)
* [fix some inference of patterns](https://github.com/rust-lang/rust-analyzer/pull/21060)
* [include all target types with paths outside package root](https://github.com/rust-lang/rust-analyzer/pull/21098)
* [infer range patterns correctly](https://github.com/rust-lang/rust-analyzer/pull/21113)
* [make dyn inlay hints configurable](https://github.com/rust-lang/rust-analyzer/pull/21068)
* [make postfix completion handle all references correctly](https://github.com/rust-lang/rust-analyzer/pull/21036)
* [move visibility diagnostics for fields to correct location](https://github.com/rust-lang/rust-analyzer/pull/21018)
* [never remove parens from prefix ops with valueless return/break/continue](https://github.com/rust-lang/rust-analyzer/pull/21092)
* [parse cargo config files with origins](https://github.com/rust-lang/rust-analyzer/pull/21015)
* [remove some deep normalizations from infer](https://github.com/rust-lang/rust-analyzer/pull/20980)
* [rewrite method resolution to follow rustc more closely](https://github.com/rust-lang/rust-analyzer/pull/20974)
* [show no error when parameters match macro names](https://github.com/rust-lang/rust-analyzer/pull/21074)
* [implement precedence for `print_hir`](https://github.com/rust-lang/rust-analyzer/pull/21057)
* [improve assist qualified to top when on first segment](https://github.com/rust-lang/rust-analyzer/pull/21042)
* [infer range pattern fully](https://github.com/rust-lang/rust-analyzer/pull/21026)
* [integrate postcard support into proc-macro server CLI](https://github.com/rust-lang/rust-analyzer/pull/20986)
* [optimize `SmolStr::clone` 4-5x speedup inline, 0.5x heap (slow down)](https://github.com/rust-lang/rust-analyzer/pull/21017)
* [perf: improve start up time](https://github.com/rust-lang/rust-analyzer/pull/21046)
* [perf: prime trait impls in cache priming](https://github.com/rust-lang/rust-analyzer/pull/21087)
* [perf: produce less progress reports](https://github.com/rust-lang/rust-analyzer/pull/21085)
* [perf: reduce allocations in `try_evaluate_obligations`](https://github.com/rust-lang/rust-analyzer/pull/21086)
* [print more macro information in `DefMap` dumps](https://github.com/rust-lang/rust-analyzer/pull/21094)
* [proc-macro-srv: reimplement token trees via immutable trees](https://github.com/rust-lang/rust-analyzer/pull/21097)
* [support multiple variant for `generate_from_impl_for_enum`](https://github.com/rust-lang/rust-analyzer/pull/21038)
* [use inferred type in “extract type as type alias” assist and display inferred type placeholder `_` inlay hints](https://github.com/rust-lang/rust-analyzer/pull/20125)

### Rust Compiler Performance Triage

Only a handful of performance-related changes landed this week. The largest one was changing the default name mangling scheme in nightly to the v0 version, which produces slightly larger symbol names, so it had a small negative effect on binary sizes and compilation time.

Triage done by **@kobzol**.
Revision range: [6159a440..b64df9d1](https://perf.rust-lang.org/?start=6159a44067ebce42b38f062cc7df267a1348e092&end=b64df9d1012f2482b54a4d959548cf8fc67e820c&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.9%  | [0.3%, 2.7%]   | 48    |
| Regressions ❌ <br /> (secondary)  | 0.9%  | [0.2%, 2.1%]   | 25    |
| Improvements ✅ <br /> (primary)   | -0.5% | [-6.8%, -0.1%] | 33    |
| Improvements ✅ <br /> (secondary) | -0.5% | [-1.4%, -0.1%] | 53    |
| All ❌✅ (primary)                 | 0.4%  | [-6.8%, 2.7%]  | 81    |

1 Regression, 2 Improvements, 5 Mixed; 1 of them in rollups
28 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/abaa823dbb9569ddf8d5c8a9fa4738106a4eb947/triage/2025/2025-11-25.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Use either
* [Item title](Item URL)
  - or
* *No RFCs were approved this week.*
-->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [Make closure capturing have consistent and correct behaviour around patterns](https://github.com/rust-lang/rust/pull/138961)
* [misc coercion cleanups and handle safety correctly](https://github.com/rust-lang/rust/pull/148602)
* [Implement `TryFrom<char>` for `usize`.](https://github.com/rust-lang/rust/pull/146792)
* [Contracts: primitive ownership assertions: `owned` and `block`](https://github.com/rust-lang/compiler-team/issues/942)
* [const validation: remove check for mutable refs in final value of const](https://github.com/rust-lang/rust/pull/148746)

*No Items entered Final Comment Period this week for
  [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or 
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Exhaustive traits. Traits that enable cross trait casting between trait objects.](https://github.com/rust-lang/rfcs/pull/3885)
* [CMSE calling conventions](https://github.com/rust-lang/rfcs/pull/3884)
* [`RUSTC_ALLOW_UNSTABLE_<feature>`: a `RUSTC_BOOTSTRAP` alternative](https://github.com/rust-lang/rfcs/pull/3882)
* [Target Stages, an improvement of the incremental system](https://github.com/rust-lang/rfcs/pull/3881)

## Upcoming Events

Rusty Events between 2025-11-26 - 2025-12-24 🦀

### Virtual
* 2025-11-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona) | [Silicon Girona](https://silicongirona.github.io)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/q5tjirkt)
* 2025-11-27 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Meet de Noviembre - Async Runtimes, parte 2!**](https://www.meetup.com/rust-argentina/events/312061828/)
* 2025-11-30 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Macros**](https://www.meetup.com/dallasrust/events/311109188/)
* 2025-12-02 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Advent of Code - Kick Off!**](https://www.meetup.com/women-in-rust/events/311068648/)
* 2025-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304242/)
* 2025-12-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/311886445/)
* 2025-12-04 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046643/)
* 2025-12-05 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103307/)
* 2025-12-06 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763878687)
* 2025-12-07 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/311103329/)
* 2025-12-09 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361537/)
* 2025-12-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/li5de4ts)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046672/)
* 2025-12-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002338/)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/6v2rorp3)
* 2025-12-18 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046644/)
* 2025-12-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361448/)

### Asia
* 2025-12-08 | Tokyo, JP | [Rust Global: Tokyo](https://rustfoundation.org/event/rust-global-tokyo/)
    * [**Rust Global: Tokyo**](https://rustfoundation.org/event/rust-global-tokyo/)
* 2025-12-20 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**December 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/december-2025-rustacean-meetup/)

### Europe
* 2025-11-26 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #5 @Source Engineers**](https://www.meetup.com/rust-bern/events/311792568/)
* 2025-11-27 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #16: Christian Meusel - Oxidizing Step by Step**](https://rust-augsburg.github.io/meetup/Meetup_16.html)
* 2025-11-27 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**19th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/311787736/)
* 2025-11-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin on location 🏳️‍🌈 - Edition 009**](https://www.meetup.com/rust-berlin/events/312169727/)
* 2025-11-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #63 sponsored by Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/312070502/)
* 2025-11-27 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Exotically Sized Types, and Rust in Space at Spire!**](https://www.meetup.com/rust-and-friends/events/311753411/)
* 2025-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague)
    * [**Rust Meetup Prague @ Barclays**](https://www.meetup.com/rust-prague/events/311846118/)
* 2025-12-03 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 12 2025**](https://luma.com/8ncu1p8l)
* 2025-12-03 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/311994790/)
* 2025-12-04 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna S2E2 - December | at metalab 🦀**](https://www.meetup.com/rust-vienna/events/311680386/)
* 2025-12-08 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - December 2025**](https://www.meetup.com/rust-dortmund/events/312165912/)
* 2025-12-08 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #81**](https://www.meetup.com/rust-paris/events/312004357/)
* 2025-12-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-16 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #3 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312037597)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)

### North America
* 2025-11-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/310457310/)
* 2025-11-26 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**Booze.rs**](https://www.meetup.com/desert-rustaceans/events/312000222/)
* 2025-11-27 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyhcpbkc/)
* 2025-11-29 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, Nov 29**](https://www.meetup.com/bostonrust/events/311917256/)
* 2025-12-02 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Talk December**](https://www.meetup.com/chicago-rust-meetup/events/311736848/)
* 2025-12-04 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Optimizando rendimiento de Python con Rust**](https://www.meetup.com/rust-mx/events/312052780/)
* 2025-12-04 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Actix Web Unleashed: Mastering State, Security, and Scalable Handlers in Rust**](https://www.meetup.com/stl-rust/events/311396006/)
* 2025-12-05 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC Unconf 2025: Our Biggest Event Yet!**](https://www.meetup.com/rust-nyc/events/311757146/)
* 2025-12-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Downtown Rust Lunch, Dec 6**](https://www.meetup.com/bostonrust/events/311917263/)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Competetive Robotics with Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312103517/)
* 2025-12-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Dec 13**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, Dec 20**](https://www.meetup.com/bostonrust/events/311917280/)
* 2025-12-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312076080/)

### Oceania
* 2025-12-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Meetup Dec 2025**](https://www.meetup.com/rust-brisbane/events/312027415/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ow6s90/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Also: a program written in Rust had a bug, and while it caused downtime, *there was no security issue and nobody's data was compromised* .

– [Josh Triplett on /r/rust](https://www.reddit.com/r/rust/comments/1p3dc7y/comment/nq4alwr/)

Thanks to [Michael Voelkl](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1732) for the suggestion!

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
