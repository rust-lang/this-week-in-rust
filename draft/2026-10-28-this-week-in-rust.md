Title: This Week in Rust 675
Number: 675
Date: 2026-10-28
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
* [Variadic Generics Micro Survey results](https://blog.rust-lang.org/inside-rust/2026/10/27/variadic-generics-micro-survey-results/)
* [How to fund Rust contributors](https://blog.rust-lang.org/2026/10/26/how-to-fund-rust-contributors/)
### Foundation

### Newsletters

* [The Embedded Rustacean Issue #83](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-83)

### Project/Tooling Updates
* [Ring Buffers Project Summary - Redox - Your Next(Gen) OS](https://www.redox-os.org/news/nlnet-rb-summary/)
* [tar-codec: a fast and secure tar parser for the Rust ecosystem](https://astral.sh/blog/tar-codec)
<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts
* [Rust Without Shortcuts: What Safety Actually Costs](https://ieeexplore.ieee.org/abstract/document/11723159)
* [What stable Rust stands on](https://kivooeo.github.io/blog/what-stable-rust-stands-on/)
* [Where a Flood Goes](https://shieldify.ee/research/where-a-flood-goes/)

* [Discussing a rewrite of XArray](https://lwn.net/SubscriberLink/1100520/2e0f0241deef5804/)
* [Rust features in the future](https://lwn.net/SubscriberLink/1099606/0fe2b8d731fd167f/)
* [Guidelines for panics in kernel code](https://lwn.net/SubscriberLink/1100873/0934a9a322c6df4c/)
* [video] [P99 CONF 2026 | Building Ultra-Performant Message Streaming by Piotr Gankiewicz](https://www.youtube.com/watch?v=2j6BzKaODUk)
* [What distributed systems and end-to-end encrypted apps have in common? CRDTs and Rust!](https://kerkour.com/rust-crdt-love)

### Rust Walkthroughs
* [video] [Beyond Vibe Coding: Formal Verification for AI-Generated Python & Rust Software](https://www.youtube.com/watch?v=P9xCUmjkPUM)
* [The heat is on: handling units in Rust](https://bitfieldconsulting.com/posts/heat-is-on)
* [video] [Rust and the GPU from Native to Web: An Introduction to `wgpu` by Liam Rust](https://www.youtube.com/watch?v=LnxuQKoxvFM)
* [Finding panics with Call Graph Analyzer](https://tweedegolf.nl/en/blog/242/finding-panics-with-call-graph-analyzer/)

### Research

### Miscellaneous

* [uz] [Announcing Rust Uzbekistan & The Official Uzbek Translation of The Rust Book](https://doc.ferris.uz/)

## Crate of the Week

This week's crate is [offline-protocol](https://crates.io/crates/offline-protocol), a crate that supplies mesh-networking/peer discovery for a variety of networks, under AGPL/proprietary license.

Thanks to [Gokul](https://users.rust-lang.org/t/crate-of-the-week/2704/1702) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
* [sysknife - SetSshdOption reports success while an earlier sshd_config.d drop-in overrides the value it wrote](https://github.com/lacs-project/sysknife/issues/559)
* [sysknife - On Postgres the daemon anchors the audit chain but every reader reports the anchor unsupported and exits 2](https://github.com/lacs-project/sysknife/issues/558)
* [sysknife - ci-local.test.sh stays green when the #555 exit fix, the missing-tool default or the opt-out flag is reverted](https://github.com/lacs-project/sysknife/issues/591)
* [Tabularis - [Feat]: Import connections from pgAdmin 4](https://github.com/TabularisDB/tabularis/issues/1309)
* [Keepword - Say how many networks have observed a node that isn't located yet](https://git.keepword.net/keepword/keepword/issues/49)
* [Octomind - `octomind workflow docs` fails with "Is a directory" when `./docs/` exists](https://github.com/Muvon/octomind/issues/242)
* [Octomind - Workflow loop with `max_iterations = 0` passes validation and never runs](https://github.com/Muvon/octomind/issues/241)
* [Octomind - ACP: send tool kind, file locations and diffs with tool calls](https://github.com/Muvon/octomind/issues/166)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

609 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-10-20..2026-10-27

#### Compiler
* [account for missing turbofish in resolve errors](https://github.com/rust-lang/rust/pull/159689)
* [add various `#[inline]` annotations for simple functions on hot paths that don't get inlined](https://github.com/rust-lang/rust/pull/163845)
* [deduplicate DefId encodings in the query cache](https://github.com/rust-lang/rust/pull/164300)
* [`#[inline(always)] + #[target_feature(enable = "....")]` #2](https://github.com/rust-lang/rust/pull/162460)
* [MIR move elimination \[4/6\]: MoveElimination](https://github.com/rust-lang/rust/pull/163338)
* [`rustc_builtin_macros` cleanup, part 8](https://github.com/rust-lang/rust/pull/163485)
* [optimize Span operations, especially the hot `Span::to`](https://github.com/rust-lang/rust/pull/163971)
* [skip redundant monomorphization checks](https://github.com/rust-lang/rust/pull/164330)
* [use the span of the doctest code block fence for figuring out the line number](https://github.com/rust-lang/rust/pull/164849)

#### Library
* [const-stabilize {`BinaryHeap`, `BTreeMap`, `BTreeSet`, `LinkedList`, `VecDeque`)`::`{`len`, `is_empty`}](https://github.com/rust-lang/rust/pull/164643)
* [core: implement `char::is_xid_`{`start`, `continue`}](https://github.com/rust-lang/rust/pull/164474)
* [don't try to skip anything in `Skip::advance_by(0)`](https://github.com/rust-lang/rust/pull/164488)
* [implement `AsMut<T>` and `AsRef<T>` for `!`](https://github.com/rust-lang/rust/pull/161253)
* [improve bit permutation codegen](https://github.com/rust-lang/rust/pull/164611)
* [process: improve the output of `Debug` for `ExitStatus` on Unix](https://github.com/rust-lang/rust/pull/163510)
* [stabilize ExitCodeExt for Windows](https://github.com/rust-lang/rust/pull/164656)
* [stabilize allocators for `Vec::into_iter`](https://github.com/rust-lang/rust/pull/163499)
* [stabilize `String::into_chars`](https://github.com/rust-lang/rust/pull/163816)
* [stabilize `ptr_cast_slice`](https://github.com/rust-lang/rust/pull/162927)
* [stabilize `slice_split_once`](https://github.com/rust-lang/rust/pull/164641)
* [stabilize {Rc, `Arc}::is_unique`](https://github.com/rust-lang/rust/pull/163458)

#### Cargo
* [`timings`: peak memory as a column in unit table](https://github.com/rust-lang/cargo/pull/17631)
* [stop uplifting rlibs on nightly](https://github.com/rust-lang/cargo/pull/17621)
* [exclude abspath of path deps from `-Cmetadata` hash](https://github.com/rust-lang/cargo/pull/17624)
* [test: simplify file content assertions](https://github.com/rust-lang/cargo/pull/17617)
* [use build-rs for buildscripts](https://github.com/rust-lang/cargo/pull/17637)

#### Rustdoc
* [include `deprecated` attributes in `rustc_resolve` intra-doc link resolution](https://github.com/rust-lang/rust/pull/164733)
* [correctly handle inner `deprecated` attribute on modules](https://github.com/rust-lang/rust/pull/164594)
* [don't panic on a nested use group with no path prefix](https://github.com/rust-lang/rust/pull/164883)

#### Rustfmt
* [prevent more comment loss in `cfg_select!`](https://github.com/rust-lang/rust/pull/164815)
* [don't lose comments and attributes between `cfg_select!` predicates, outer/inner blocks, arm body and trailing commas](https://github.com/rust-lang/rustfmt/pull/7211)
* [fix `cfg_if!` error count leaks](https://github.com/rust-lang/rustfmt/pull/7195)

#### Clippy
* [`wildcard_imports`: no longer lint in automatically derives and proc macro expansions](https://github.com/rust-lang/rust-clippy/pull/17898)
* [add `ifs_as_logical_ops` lint](https://github.com/rust-lang/rust-clippy/pull/14904)
* [add lint for `map_or_else` to `map_or`](https://github.com/rust-lang/rust-clippy/pull/17847)
* [fix `missing_const_for_fn` false positives in `const impl`](https://github.com/rust-lang/rust-clippy/pull/17892)
* [fix `double_comparisons` suggestion dropping required parentheses](https://github.com/rust-lang/rust-clippy/pull/17907)
* [perf: optimise `unconditional_recursion` lint](https://github.com/rust-lang/rust-clippy/pull/17835)

#### Rust-Analyzer
* [automated sync of target feature implications](https://github.com/rust-lang/rust-analyzer/pull/23484)
* [fix NamedTempFile issues on windows](https://github.com/rust-lang/rust-analyzer/pull/23424)
* [complete 'main' 'lib' module in sub module](https://github.com/rust-lang/rust-analyzer/pull/23529)
* [compute ref-match on undotted self](https://github.com/rust-lang/rust-analyzer/pull/23489)
* [do not panic when inline call param usages in macro](https://github.com/rust-lang/rust-analyzer/pull/23231)
* [fix `fetch_semantic` logic bug introduced in #22272](https://github.com/rust-lang/rust-analyzer/pull/23534)
* [fix status message for clients not supporting `experimental/serverStatusNotification`](https://github.com/rust-lang/rust-analyzer/pull/23552)
* [use absolute path in `format_smolstr`](https://github.com/rust-lang/rust-analyzer/pull/23540)
* [search manifest ancestors for the lockfile when fetching cargo metadata](https://github.com/rust-lang/rust-analyzer/pull/23296)

### Rust Compiler Performance Triage

Another week of positive changes. Few unexpected improvements from cleanups and a handful of targeted optimizations.
Apart from instruction count improvements, we also merged a large reduction of query cache size in [#164300](https://github.com/rust-lang/rust/pull/164300).
Negative changes come mostly from unexpected `doc` regression, which is currently under investigation.

Triage done by **@panstromek**.
Revision range: [bd4c6cc4..019b59d8](https://perf.rust-lang.org/?start=bd4c6cc4aeef0eb9919c6f73c1e690677e9c6b38&end=019b59d8101bb3e141f1b880684747b4217beeb6&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.8%  | [0.2%, 6.8%]    | 25    |
| Regressions ❌ <br /> (secondary)  | 6.8%  | [0.0%, 189.1%]  | 34    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-12.6%, -0.1%] | 195   |
| Improvements ✅ <br /> (secondary) | -1.2% | [-10.6%, -0.1%] | 157   |
| All ❌✅ (primary)                 | -0.6% | [-12.6%, 6.8%]  | 220   |


3 Regressions, 6 Improvements, 3 Mixed
32 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/5080585bb0dc997cd8aeca17eff036e0c5ddf713/triage/2026/2026-10-26.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [2052-epochs.md: typo](https://github.com/rust-lang/rfcs/pull/4024)
* [3722-explicit-extern-abis.md: typo](https://github.com/rust-lang/rfcs/pull/4023)
* [Fix libs nominated triagebot.toml](https://github.com/rust-lang/rfcs/pull/4022)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Add `let_underscore_drop_guard` lint.](https://github.com/rust-lang/rust/pull/164414)
* [Deprecate internal function `vec::from_elem`](https://github.com/rust-lang/rust/pull/164653)
* [core: Add `assert_matches` and `debug_assert_matches` to the next prelude](https://github.com/rust-lang/rust/pull/164328)
* [clarify when `Vec::into_boxed_slice()` is in-place](https://github.com/rust-lang/rust/pull/164258)
* [Partially stabilize `ptr_alignment_type` as `alignment_type`](https://github.com/rust-lang/rust/pull/153261)
* [std: make `Instant` a signed `Duration` on all platforms](https://github.com/rust-lang/rust/pull/158368)
* [Lower #[no_mangle] to #[symbol_name]/#[link_name]](https://github.com/rust-lang/rust/pull/159374)
* [lang team review for new trait solver](https://github.com/rust-lang/rust/issues/164701)
* [Implement `Fn` traits for safe functions of any ABI](https://github.com/rust-lang/rust/pull/160186)
* [cmse: lint on unions crossing the secure boundary](https://github.com/rust-lang/rust/pull/147697)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Add `required-targets` for workspace package selection](https://github.com/rust-lang/rfcs/pull/4013)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Add support for `annotate-snippets`'s `no-graphics` output format](https://github.com/rust-lang/compiler-team/issues/1045)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*


## Upcoming Events

Rusty Events between 2026-10-28 - 2026-11-25 🦀

### Virtual
* 2026-10-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Operating Systems Book Club: Paging Mechanisms**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316880405/)
* 2026-10-29 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Creating a hexadecimal editor in Rust**](https://luma.com/q8i3385k)
* 2026-11-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331277/)
* 2026-11-03 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Solving Real-World Planning Problems in Rust with SolverForge**](https://luma.com/rfbzk3ae)
* 2026-11-03 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773682/)
* 2026-11-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcpbgb/)
* 2026-11-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907780/)
* 2026-11-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254770/)
* 2026-11-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619619/)
* 2026-11-15 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Cursed Rust**](https://luma.com/omlooqv8)
* 2026-11-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563020/)
* 2026-11-17 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Miscompilation in Rust 1.97**](https://luma.com/2qfai4a8)
* 2026-11-17 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: The Harness Wore a Trenchcoat - Building a Rust MCP Server**](https://www.meetup.com/women-in-rust/events/315719313/)
* 2026-11-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcpbwb/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907781/)
* 2026-11-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254769/)

### Asia
* 2026-11-03 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust November 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**November 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europe
* 2026-10-28 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #72 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/316545936/)
* 2026-10-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 – Edition 019**](https://www.meetup.com/rust-berlin/events/317130457/)
* 2026-10-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 2026-11-01 - 2026-11-03 | Italy, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - The International Conference on Rust in Italy**](https://rustlab.it/schedule)
* 2026-11-02 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/317103436/)
* 2026-11-04 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Getting Started with the New Rust Driver for Oracle AI Database**](https://www.meetup.com/rust-dublin/events/316898325/)
* 2026-11-05 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Digital Model Trains & Rust: Bare-Metal DCC Command Station on ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)
* 2026-11-05 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Epitome of synaptic excellence**](https://www.meetup.com/oxford-rust-meetup-group/events/317158298/)
* 2026-11-12 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-11-12 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano/events/)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-language-milan/events/317094800/)
* 2026-11-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Everything Is Git, Protocols Are Just Adapters**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)
* 2026-11-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Talk Night at EIVA**](https://www.meetup.com/rust-aarhus/events/316938696/)

### North America
* 2026-10-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316655403/)
* 2026-10-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, Oct 31**](https://www.meetup.com/bostonrust/events/316924288/)
* 2026-11-05 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**November Monthly Social**](https://www.meetup.com/rust-montreal/events/316919004/)
* 2026-11-05 | San Francisco, CA | [Rust LA: NVIDIA](https://luma.com/user/usr-eWgtha6CHSTma7v/)
    * [**NVIDIA x Lawrence Harvey | CUDA Rust**](https://luma.com/iqe7wng1)
* 2026-11-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**HTMX and Rust**](https://www.meetup.com/stl-rust/events/316896797/)
* 2026-11-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust November Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316886787/)
* 2026-11-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, Nov 14**](https://www.meetup.com/bostonrust/events/316924331/)
* 2026-11-17 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA November w/ Hadrian!**](https://www.meetup.com/rust-los-angeles/events/316859673/)
* 2026-11-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783989/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, Nov 21**](https://www.meetup.com/bostonrust/events/316924366/)

### South America
* 2026-11-07 | Planalto Paulista, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP - Agora no Luma**](https://www.meetup.com/rust-sao-paulo-meetup/events/316812852/)

### Oceania:
* 2026-11-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/316907340/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> we're at the 25 minute mark and still bikeshedding tabs vs spaces

– [Josh Triplett in a Library Team meeting](https://hackmd.io/RUHqJ5InQkCBRs5d0WpFBA)

Thanks to [Theemathas](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1836) for the suggestion!

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
