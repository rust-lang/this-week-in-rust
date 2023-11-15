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

### Newsletters
* [This Week in Ars Militaris VIII](https://arsmilitaris.com/#this-week-in-ars-militaris-viii)

### Project/Tooling Updates
* [rust-analyzer changelog #196](https://rust-analyzer.github.io/thisweek/2023/08/28/changelog-196.html)
* [The First Stable Release of a Memory Safe sudo Implementation](https://www.memorysafety.org/blog/sudo-first-stable-release/)
* [We're open-sourcing the library that powers 1Password's ability to log in with a passkey](https://blog.1password.com/passkey-crates/)
* [ratatui 0.23.0 is released! (official successor of tui-rs)](https://github.com/ratatui-org/ratatui/releases/tag/v0.23.0)
* [Zellij 0.38.0: session-manager, plugin infra, and no more offensive session names](https://zellij.dev/news/session-manager-protobuffs/)

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
* [Dependency injection in Axum handlers. A quick tour](https://tulipemoutarde.be/posts/2023-08-20-depencency-injection-rust-axum/)
* [Best Rust Web Frameworks to Use in 2023](https://www.shuttle.rs/blog/2023/08/23/rust-web-framework-comparison)
* [From tui-rs to Ratatui: 6 Months of Cooking Up Rust TUIs](https://blog.orhun.dev/ratatui-0-23-0/)
* [video] [Rust 1.72.0](https://www.youtube.com/watch?v=jVoEA7qmN8c)
* [video] [Rust 1.72 Release Train](https://www.youtube.com/watch?v=EFdzH67G-lw)

### Rust Walkthroughs
* [series] [Distributed Tracing in Rust, Episode 3: tracing basics](https://heikoseeberger.de/2023-08-28-dist-tracing-3/)
* [Use Rust in shell scripts](https://www.kurtlawrence.info/blog/gufdjkjkq7wphfhkvrumcvmqdr4r69)
* [A Simple CRUD API in Rust with Cloudflare Workers, Cloudflare KV, and the Rust Router](https://medium.com/@estebanrules/a-simple-crud-api-in-rust-with-cloudflare-workers-cloudflare-kv-and-the-rust-router-cbc1b9015e7b)
* [video] [base64 crate: code walkthrough](https://www.youtube.com/watch?v=KGP16TITJdU)

### Miscellaneous
* [Interview with Rust and operating system Developer Andy Python](https://blog.rust.careers/post/andy-python-interview/)
* [Leveraging Rust in our high-performance Java database](https://questdb.io/blog/leveraging-rust-in-our-high-performance-java-database/)
* [Rust error message to fix a typo](https://rust.code-maven.com/error-message-to-fix-typo)
* [video] [The Builder Pattern and Typestate Programming - Stefan Baumgartner - Rust Linz January 2023](https://www.youtube.com/watch?v=k8kd22jNcps)
* [video] [CI with Rust and Gitlab Selfhosting - Stefan Schindler - Rust Linz July 2023](https://www.youtube.com/watch?v=TrPyUVundbM)

## Crate of the Week

This week's crate is [dprint](https://github.com/dprint/dprint), a fast code formatter that formats Markdown, TypeScript, JavaScript, JSON, TOML and many other types natively via Wasm plugins.

Thanks to [Martin Geisler](https://users.rust-lang.org/t/crate-of-the-week/2704/1232) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hyperswitch - add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - deserialization error exposes sensitive values in the logs](https://github.com/juspay/hyperswitch/issues/1943)
* [Hyperswitch - move redis key creation to a common module](https://github.com/juspay/hyperswitch/issues/917)
* [mdbook-i18n-helpers - Write tool which can convert translated files back to PO](https://github.com/google/mdbook-i18n-helpers/issues/30)
* [mdbook-i18n-helpers - Package a language selector](https://github.com/google/mdbook-i18n-helpers/issues/12)
* [mdbook-i18n-helpers - Add links between translations](https://github.com/google/mdbook-i18n-helpers/issues/35)
* [Comprehensive Rust - Link to correct line when editing a translation](https://github.com/google/comprehensive-rust/issues/1075)
* [Comprehensive Rust - Track the number of times the redirect pages are visited](https://github.com/google/comprehensive-rust/issues/488)
* [RustQuant - Jacobian and Hessian matrices support.](https://github.com/avhz/RustQuant/issues/87)
* [RustQuant - improve Graphviz plotting of autodiff computational graphs.](https://github.com/avhz/RustQuant/issues/74)
* [RustQuant - bond pricing implementation.](https://github.com/avhz/RustQuant/issues/88)
* [RustQuant - implement cap/floor pricers.](https://github.com/avhz/RustQuant/issues/21)
* [RustQuant - Implement Asian option pricers.](https://github.com/avhz/RustQuant/issues/3)
* [RustQuant - Implement American option pricers.](https://github.com/avhz/RustQuant/issues/4)
* [release-plz - add ability to mark Gitea/GitHub release as draft](https://github.com/MarcoIeni/release-plz/issues/676)
* [zerocopy - CI step "Set toolchain version" is flaky due to network timeouts](https://github.com/google/zerocopy/issues/295)
* [zerocopy - Implement traits for tuple types (and maybe other container types?)](https://github.com/google/zerocopy/issues/274)
* [zerocopy - Prevent panics statically](https://github.com/google/zerocopy/issues/202)
* [zerocopy - Add positive and negative trait impl tests for SIMD types](https://github.com/google/zerocopy/issues/130)
* [zerocopy - Inline many trait methods (in zerocopy and in derive-generated code)](https://github.com/google/zerocopy/issues/7)
* [datatest-stable - Fix quadratic performance with nextest](https://github.com/nextest-rs/datatest-stable/issues/8)
* [Ockam - Use a user-friendly name for the shared services to show it in the tray menu](https://github.com/build-trust/ockam/issues/5686)
* [Ockam - Rename the Port to Address and support such format](https://github.com/build-trust/ockam/issues/5685)
* [Ockam - Ockam CLI should gracefully handle invalid state when initializing](https://github.com/build-trust/ockam/issues/5633)
* [css-inline - Update `cssparser` & `selectors`](https://github.com/Stranger6667/css-inline/issues/214)
* [css-inline - Non-blocking stylesheet resolving](https://github.com/Stranger6667/css-inline/issues/246)
* [css-inline - Optionally remove all `class` attributes](https://github.com/Stranger6667/css-inline/issues/13)

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

* [Create a Testing sub-team](https://github.com/rust-lang/rfcs/pull/3455)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Stabilize `PATH` option for `--print KIND=PATH`](https://github.com/rust-lang/rust/pull/114183)
* [disposition: merge] [Add alignment to the NPO guarantee](https://github.com/rust-lang/rust/pull/114845)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Special-cased performance improvement for `Iterator::sum` on `Range<u*>` and `RangeInclusive<u*>`](https://github.com/rust-lang/rfcs/pull/3481)
* [new] [Cargo Check T-lang Policy](https://github.com/rust-lang/rfcs/pull/3477)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-08-30 - 2023-09-27 🦀

### Virtual

* 2023-09-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295207389/)
* 2023-09-05 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294049877)
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
* 2023-09-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057154/)
* 2023-09-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295666673/)
* 2023-09-21 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Real Time Multiplayer Game Server in Rust**](https://www.meetup.com/utah-rust/events/294972877/)
* 2023-09-21 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 33rd Edition**](https://www.meetup.com/rust-linz/events/295363887/)
* 2023-09-25 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**How we built the SurrealDB Python client in Rust.**](https://www.meetup.com/Rust-Dublin/events/294908596/)

### Asia

* 2023-09-06 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**RustTLV @ Final - September Edition**](https://www.meetup.com/rust-tlv/events/295441355/)

### Europe

* 2023-08-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #39 sponsored by Fermyon**](https://www.meetup.com/copenhagen-rust-community/events/294806394)
* 2023-08-31 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #2**](https://www.meetup.com/rust-meetup-augsburg/events/294538503/)
* 2023-09-05 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295109905/)
* 2023-09-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Logging and tracing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504245/)
* 2023-09-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus - Rust and Talk at Concordium**](https://www.meetup.com/rust-aarhus/events/294031975/)
* 2023-09-21 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Third Rust Bern Meetup**](https://www.meetup.com/rust-bern/events/295503351/)

### North America

* 2023-09-05 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/295589114/)
* 2023-09-06 | Bellevue, WA, US | [The Linux Foundation](https://www.linuxfoundation.org/)
    * [**Rust Global**](https://events.linuxfoundation.org/rust-global/)
* 2023-09-12 - 2023-09-15 | Albuquerque, NM, US  + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**A Panel Discussion on Thriving in a Rust-Driven Workplace**](https://www.meetup.com/rust-nyc/events/295639294)
* 2023-09-12 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/295744114/)
* 2023-09-14 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295484105)
* 2023-09-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/295545278)
* 2023-09-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust on the web! Get started with Leptos**](https://www.meetup.com/music-city-rust-developers/events/295587220/)
* 2023-09-26 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/295771515)
* 2023-09-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295466314)

### Oceania

* 2023-09-13 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**Rust Meetup 2: Lunch & Learn**](https://www.linkedin.com/events/7097356771584880640/)
* 2023-09-19 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/295602231/)
* 2023-09-26 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/295432237/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/14zmcpw/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> In \[other languages\], I could end up chasing silly bugs and waste time debugging and tracing to find that I made a typo or ran into a language quirk that gave me an unexpected nil pointer. That situation is almost non-existent in Rust, it's just me and the problem. Rust is honest and upfront about its quirks and will yell at you about it before you have a hard to find bug in production.

– [dannersy on Hacker News](https://news.ycombinator.com/item?id=37107992)

Thanks to [Kyle Strand](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1463) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/165w3ea/this_week_in_rust_510/)</small>
