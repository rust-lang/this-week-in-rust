Title: This Week in Rust 510
Number: 510
Date: 2023-08-30
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
* [Announcing Rust 1.72.0](https://blog.rust-lang.org/2023/08/24/Rust-1.72.0.html)
* [Change in Guidance on Committing Lockfiles](https://blog.rust-lang.org/2023/08/29/committing-lockfiles.html)
* [Cargo changes how arrays in config are merged](https://blog.rust-lang.org/inside-rust/2023/08/24/cargo-config-merging.html)
* [Seeking help for initial Leadership Council initiatives](https://blog.rust-lang.org/inside-rust/2023/08/25/leadership-initiatives.html)
* [Leadership Council Membership Changes](https://blog.rust-lang.org/inside-rust/2023/08/29/leadership-council-membership-changes.html)

### Foundation

### Newsletters
* [This Week in Ars Militaris VIII](https://arsmilitaris.com/#this-week-in-ars-militaris-viii)

### Project/Tooling Updates
* [rust-analyzer changelog #196](https://rust-analyzer.github.io/thisweek/2023/08/28/changelog-196.html)
* [The First Stable Release of a Memory Safe sudo Implementation](https://www.memorysafety.org/blog/sudo-first-stable-release/)
* [We're open-sourcing the library that powers 1Password's ability to log in with a passkey](https://blog.1password.com/passkey-crates/)

- [ratatui 0.23.0 is released! (official successor of tui-rs)](https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0)

### Observations/Thoughts
* [The fastest WebSocket implementation](https://c410-f3r.github.io/thoughts/the-fastest-websocket-implementation/)
* [Rust Malware Staged on Crates.io](https://blog.phylum.io/rust-malware-staged-on-crates-io/)
* [ESP32 Standard Library Embedded Rust: SPI with the MAX7219 LED Dot Matrix](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-spi-with-the-max7219-led-dot-matrix)
* [A JVM in Rust part 5 - Executing instructions](https://andreabergia.com/blog/2023/08/a-jvm-in-rust-part-5-executing-instructions/)
* [Compiling Rust for .NET, using only tea and stubbornness!](https://fractalfir.github.io/generated_html/rustc_codegen_clr_v0_0_1.html)
* [Ad-hoc polymorphism erodes type-safety](https://cs-syd.eu/posts/2023-08-25-ad-hoc-polymorphism-erodes-type-safety)
* [How to speed up the Rust compiler in August 2023](https://nnethercote.github.io/2023/08/25/how-to-speed-up-the-rust-compiler-in-august-2023.html)
* [This isn't the way to speed up Rust compile times](https://xeiaso.net/blog/serde-precompiled-stupid)
* [Rust Cryptography Should be Written in Rust](https://briansmith.org/rust-cryptography-should-be-written-in-rust-01)
- [video][Rust 1.72.0](https://youtu.be/jVoEA7qmN8c)
- [Dependency injection in Axum handlers. A quick tour](https://tulipemoutarde.be/posts/2023-08-20-depencency-injection-rust-axum/)
* [Best Rust Web Frameworks to Use in 2023](https://www.shuttle.rs/blog/2023/08/23/rust-web-framework-comparison)
- [From tui-rs to Ratatui: 6 Months of Cooking Up Rust TUIs](https://blog.orhun.dev/ratatui-0-23-0/)

### Rust Walkthroughs
* [series] [Distributed Tracing in Rust, Episode 3: tracing basics](https://heikoseeberger.de/2023-08-28-dist-tracing-3/)
* [Use Rust in shell scripts](https://www.kurtlawrence.info/blog/gufdjkjkq7wphfhkvrumcvmqdr4r69)
* [A Simple CRUD API in Rust with Cloudflare Workers, Cloudflare KV, and the Rust Router](https://medium.com/@estebanrules/a-simple-crud-api-in-rust-with-cloudflare-workers-cloudflare-kv-and-the-rust-router-cbc1b9015e7b)

### Research

### Miscellaneous
* [video][Rust 1.72 Release Train](https://www.youtube.com/watch?v=EFdzH67G-lw)
* [Interview with Rust and operating system Developer Andy Python](https://blog.rust.careers/post/andy-python-interview/)

- [Leveraging Rust in our high-performance Java database](https://questdb.io/blog/leveraging-rust-in-our-high-performance-java-database/)

## Crate of the Week

This week's crate is [dprint](https://github.com/dprint/dprint), a fast code formatter that formats Markdown, TypeScript, JavaScript, JSON, TOML and many other types natively via Wasm plugins.

Thanks to [Martin Geisler](https://users.rust-lang.org/t/crate-of-the-week/2704/1232) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

366 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-08-21..2023-08-28

* [reassign sparc-unknown-none-elf to tier 3](https://github.com/rust-lang/rust/pull/115075)
* [wasi: round up the size for `aligned_alloc`](https://github.com/rust-lang/rust/pull/115254)
* [allow `MaybeUninit` in input and output of inline assembly](https://github.com/rust-lang/rust/pull/114790)
* [allow explicit `#[repr(Rust)]`](https://github.com/rust-lang/rust/pull/114201)
* [fix CFI: f32 and f64 are encoded incorrectly for cross-language CFI](https://github.com/rust-lang/rust/pull/115151)
* [add `suggestion` for some `#[deprecated]` items](https://github.com/rust-lang/rust/pull/113365)
* [add an (perma-)unstable option to disable vtable vptr](https://github.com/rust-lang/rust/pull/114974)
* [add comment to the `push_trailing` function](https://github.com/rust-lang/rust/pull/115190)
* [add note when matching on tuples/ADTs containing non-exhaustive types](https://github.com/rust-lang/rust/pull/114397)
* [add support for `ptr::write`s for the `invalid_reference_casting` lint](https://github.com/rust-lang/rust/pull/115100)
* [allow overwriting `ExpnId` for concurrent decoding](https://github.com/rust-lang/rust/pull/115081)
* [avoid duplicate `large_assignments` lints](https://github.com/rust-lang/rust/pull/114774)
* [contents of reachable statics is reachable](https://github.com/rust-lang/rust/pull/115114)
* [do not emit invalid suggestion in E0191 when spans overlap](https://github.com/rust-lang/rust/pull/115077)
* [do not forget to pass DWARF fragment information to LLVM](https://github.com/rust-lang/rust/pull/115139)
* [ensure that THIR unsafety check is done before stealing it](https://github.com/rust-lang/rust/pull/115012)
* [emit a proper diagnostic message for unstable lints passed from CLI](https://github.com/rust-lang/rust/pull/114959)
* [fix races conditions with `SyntaxContext` decoding](https://github.com/rust-lang/rust/pull/115082)
* [fix waiting on a query that panicked](https://github.com/rust-lang/rust/pull/115198)
* [improve note for the `invalid_reference_casting` lint](https://github.com/rust-lang/rust/pull/115102)
* [include compiler flags when you `break rust;`](https://github.com/rust-lang/rust/pull/115158)
* [load `include_bytes!` directly into an Lrc](https://github.com/rust-lang/rust/pull/115296)
* [make `Sharded` an `enum` and specialize it for the single thread case](https://github.com/rust-lang/rust/pull/114860)
* [make `rustc_on_unimplemented` std-agnostic for `alloc::rc`](https://github.com/rust-lang/rust/pull/115210)
* [more precisely detect cycle errors from `type_of` on opaque](https://github.com/rust-lang/rust/pull/115294)
* [point at type parameter that introduced unmet bound instead of full HIR node](https://github.com/rust-lang/rust/pull/115219)
* [record allocation spans inside `force_allocation`](https://github.com/rust-lang/rust/pull/115184)
* [suggest mutable borrow on read only for-loop that should be mutable](https://github.com/rust-lang/rust/pull/115147)
* [tweak output of `to_pretty_impl_header` involving only anon lifetimes](https://github.com/rust-lang/rust/pull/115322)
* [use the same DISubprogram for each instance of the same inlined function within a caller](https://github.com/rust-lang/rust/pull/114643)
* [walk through full path in `point_at_path_if_possible`](https://github.com/rust-lang/rust/pull/115221)
* [warn on elided lifetimes in associated constants (`ELIDED_LIFETIMES_IN_ASSOCIATED_CONSTANT`)](https://github.com/rust-lang/rust/pull/115011)
* [make RPITITs capture all in-scope lifetimes](https://github.com/rust-lang/rust/pull/114489)
* [add stable for Constant in smir](https://github.com/rust-lang/rust/pull/115202)
* [add `generics_of` to smir](https://github.com/rust-lang/rust/pull/115092)
* [add smir `predicates_of`](https://github.com/rust-lang/rust/pull/115084)
* [treat `StatementKind::Coverage` as completely opaque for SMIR purposes](https://github.com/rust-lang/rust/pull/115093)
* [do not convert copies of packed projections to moves](https://github.com/rust-lang/rust/pull/115138)
* [don't do intra-pass validation on MIR shims](https://github.com/rust-lang/rust/pull/115005)
* [MIR validation: reject in-place argument/return for packed fields](https://github.com/rust-lang/rust/pull/115164)
* [disable MIR SROA optimization by default](https://github.com/rust-lang/rust/pull/115140)
* [miri: automatically start and stop josh in rustc-pull/push](https://github.com/rust-lang/miri/pull/3036)
* [miri: fix some bad regex capture group references in test normalization](https://github.com/rust-lang/miri/pull/3037)
* [stop emitting non-power-of-two vectors in (non-portable-SIMD) codegen](https://github.com/rust-lang/rust/pull/115236)
* [resolve: stop creating `NameBinding`s on every use, create them once per definition instead](https://github.com/rust-lang/rust/pull/113408)
* [fix a `pthread_t` handle leak](https://github.com/rust-lang/rust/pull/114696)
* [when terminating during unwinding, show the reason why](https://github.com/rust-lang/rust/pull/115045)
* [avoid triple-backtrace due to panic-during-cleanup](https://github.com/rust-lang/rust/pull/115280)
* [add additional float constants](https://github.com/rust-lang/rust/pull/103836)
* [add ability to spawn Windows process with Proc Thread Attributes | Take 2](https://github.com/rust-lang/rust/pull/114848)
* [fix implementation of `Duration::checked_div`](https://github.com/rust-lang/rust/pull/114238)
* [hashbrown: allow serializing `HashMap`s that use a custom allocator](https://github.com/rust-lang/hashbrown/pull/449)
* [hashbrown: change `&` to `&mut` where applicable](https://github.com/rust-lang/hashbrown/pull/464)
* [hashbrown: simplify `Clone` by removing redundant guards](https://github.com/rust-lang/hashbrown/pull/458)
* [regex-automata: fix incorrect use of Aho-Corasick's "standard" semantics](https://github.com/rust-lang/regex/pull/1072)
* [cargo: **Very** preliminary MSRV resolver support](https://github.com/rust-lang/cargo/pull/12560)
* [cargo: Use a more compact relative-time format](https://github.com/rust-lang/cargo/pull/12542)
* [cargo: Improve TOML parse errors](https://github.com/rust-lang/cargo/pull/12556)
* [cargo: add support for `target.'cfg(..)'.linker`](https://github.com/rust-lang/cargo/pull/12535)
* [cargo: config: merge lists in precedence order](https://github.com/rust-lang/cargo/pull/12515)
* [cargo: create dedicated unstable flag for asymmetric-token](https://github.com/rust-lang/cargo/pull/12551)
* [cargo: set MSRV for internal packages](https://github.com/rust-lang/cargo/pull/12381)
* [cargo: improve deserialization errors of untagged enums](https://github.com/rust-lang/cargo/pull/12574)
* [cargo: improve resolver version mismatch warning](https://github.com/rust-lang/cargo/pull/12573)
* [cargo: stabilize `--keep-going`](https://github.com/rust-lang/cargo/pull/12568)
* [cargo: support dependencies from registries for artifact dependencies, take 2](https://github.com/rust-lang/cargo/pull/12421)
* [cargo: use AND search when having multiple terms](https://github.com/rust-lang/cargo/pull/12548)
* [rustdoc: add unstable `--no-html-source` flag](https://github.com/rust-lang/rust/pull/115135)
* [rustdoc: rename typedef to type alias](https://github.com/rust-lang/rust/pull/115078)
* [rustdoc: use unicode-aware checks for redundant explicit link fastpath](https://github.com/rust-lang/rust/pull/115070)
* [clippy: new lint: `implied_bounds_in_impls`](https://github.com/rust-lang/rust-clippy/pull/11362)
* [clippy: new lint: `reserve_after_initialization`](https://github.com/rust-lang/rust-clippy/pull/11373)
* [clippy: `arithmetic_side_effects`: detect division by zero for `Wrapping` and `Saturating`](https://github.com/rust-lang/rust-clippy/pull/11395)
* [clippy: `if_then_some_else_none`: look into local initializers for early returns](https://github.com/rust-lang/rust-clippy/pull/11401)
* [clippy: `iter_overeager_cloned`: detect `.cloned().all()` and `.cloned().any()`](https://github.com/rust-lang/rust-clippy/pull/11360)
* [clippy: `unnecessary_unwrap`: lint on `.as_ref().unwrap()`](https://github.com/rust-lang/rust-clippy/pull/11387)
* [clippy: allow trait alias DefIds in `implements_trait_with_env_from_iter`](https://github.com/rust-lang/rust-clippy/pull/11338)
* [clippy: fix `"derivable_impls`: attributes are ignored"](https://github.com/rust-lang/rust-clippy/pull/11404)
* [clippy: fix `tuple_array_conversions` lint on nightly](https://github.com/rust-lang/rust-clippy/pull/11379)
* [clippy: skip `float_cmp` check if lhs is a custom type](https://github.com/rust-lang/rust-clippy/pull/11385)
* [rust-analyzer: diagnostics for 'while let' loop with label in condition](https://github.com/rust-lang/rust-analyzer/pull/15517)
* [rust-analyzer: respect `#[allow(unused_braces)]`](https://github.com/rust-lang/rust-analyzer/pull/15527)

### Rust Compiler Performance Triage

A fairly quiet week, with improvements exceeding a small scattering of
regressions. Memory usage and artifact size held fairly steady across the week,
with no regressions or improvements.

Triage done by **@simulacrum**.
Revision range: [d4a881e..cedbe5c](https://perf.rust-lang.org/?start=d4a881e1433cd10e424843353e1f939f5a798f4e&end=cedbe5c715c1fa9359683c5f108bed2054ac258b&absolute=false&stat=instructions%3Au)

2 Regressions, 3 Improvements, 2 Mixed; 0 of them in rollups
108 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-08-29.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-08-30 - 2023-09-27 🦀

### Virtual

* 2023-08-23 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 32nd Edition**](https://www.meetup.com/rust-linz/events/294718621/)
* 2023-08-24 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295250677/)
* 2023-08-24 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Macros Procedurales y Metalenguajes en Rust**](https://www.meetup.com/rust-mx/events/295545343)
* 2023-09-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295207389/)
* 2023-09-05 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294049877)
* 2023-09-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcmbkb/)
* 2023-09-12 - 2023-09-15 | Virtual (Albuquerque, NM, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/gqdlgtyfcmbqb/)
* 2023-09-13 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/295011539)
* 2023-09-13 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-09-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732655)

### Asia

* 2023-09-06 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**RustTLV @ Final - September Edition**](https://www.meetup.com/rust-tlv/events/295441355/)

### Europe

* 2023-08-23 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks Aug 2023: Rust London x RNL (The next Frontier in App Development)**](https://www.meetup.com/rust-london-user-group/events/295338396/)
* 2023-08-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus Hack and Learn at Trifork**](https://www.meetup.com/rust-aarhus/events/293950871/)
* 2023-08-31 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #2**](https://www.meetup.com/rust-meetup-augsburg/events/294538503/)
* 2023-09-05 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-21 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Third Rust Bern Meetup**](https://www.meetup.com/rust-bern/events/295503351/)

### North America

* 2023-08-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295008514)
* 2023-08-24 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295107743/)
* 2023-08-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #39 sponsored by Fermyon**](https://www.meetup.com/copenhagen-rust-community/events/294806394)
* 2023-09-06 | Bellevue, WA, US | [The Linux Foundation](https://www.linuxfoundation.org/)
    * [**Rust Global**](https://events.linuxfoundation.org/rust-global/)
* 2023-09-12 - 2023-09-15 | Albuquerque, NM, US  + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/295545278)

### Oceania

* 2023-08-24 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**August Meetup**](https://www.meetup.com/rust-brisbane/events/295415680/)
* 2023-09-13 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**Rust Meetup 2: Lunch & Learn**](https://www.linkedin.com/events/7097356771584880640/)

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

> In \[other languages\], I could end up chasing silly bugs and waste time debugging and tracing to find that I made a typo or ran into a language quirk that gave me an unexpected nil pointer. That situation is almost non-existent in Rust, it's just me and the problem. Rust is honest and upfront about its quirks and will yell at you about it before you have a hard to find bug in production.

– [dannersy on Hacker News](https://news.ycombinator.com/item?id=37107992)

Thanks to [Kyle Strand](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1463) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>