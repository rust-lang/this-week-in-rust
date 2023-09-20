Title: This Week in Rust 513
Number: 513
Date: 2023-09-20
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
* [Announcing Rust 1.72.1](https://blog.rust-lang.org/2023/09/19/Rust-1.72.1.html)

### Foundation
* [Announcing the Rust Foundation’s Associate Membership with OpenSSF](https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-associate-membership-with-openssf/)

### Newsletters

### Project/Tooling Updates
* [This month in Servo: upcoming events, new browser UI, and more!](https://servo.org/blog/2023/09/15/upcoming-events-and-new-browser-ui/)
* [Pagefind v1.0.0 — Stable static search at scale](https://github.com/CloudCannon/pagefind/releases/tag/v1.0.0)
* [Open sourcing the Grafbase Engine](https://grafbase.com/blog/open-sourcing-the-grafbase-engine)
* [Announcing Arroyo 0.6.0](https://www.arroyo.dev/blog/arroyo-0-6-0)
* [rust-analyzer changelog #199](https://rust-analyzer.github.io/thisweek/2023/09/18/changelog-199.html)
* [rumqttd 0.18.0](https://github.com/bytebeamio/rumqtt/releases/tag/rumqttd-0.18.0)

### Observations/Thoughts
* [Stability without stressing the !@#! out](https://smallcultfollowing.com/babysteps/blog/2023/09/18/stability-without-stressing-the-out/)
* [The State of Async Rust](https://corrode.dev/blog/async/)
* [NFS > FUSE: Why We Built our own NFS Server in Rust](https://about.xethub.com/blog/nfs-fuse-why-we-built-nfs-server-rust)
* [Breaking Tradition: Why Rust Might Be Your Best First Language](https://medium.com/@otukof/breaking-tradition-why-rust-might-be-your-best-first-language-d10afc482ac1)
* [The Embedded Rust ESP Development Ecosystem](https://apollolabsblog.hashnode.dev/the-embedded-rust-esp-development-ecosystem)
* [Sifting through crates.io for malware with OSSF Package Analysis](http://www.williballenthin.com/post/sifting-through-crates.io-for-malware-with-ossf-package-analysis/)
* [Choosing a more optimal `String` type](https://swatinem.de/blog/optimized-strings/)
* [Changing the rules of Rust](https://without.boats/blog/changing-the-rules-of-rust/)
* [Follow up to "Changing the rules of Rust"](https://without.boats/blog/follow-up-to-changing-the-rules-of-rust/)
* [When Zig Outshines Rust - Memory Efficient Enum Arrays](https://alic.dev/blog/dense-enums)
* [Three years of Bevy](https://trent.kiwi/bevy-three-years)
* [Should I Rust or should I go?](https://kerkour.com/should-i-rust-or-should-i-go)
* [audio] [What's New in Rust 1.68 and 1.69](https://rustacean-station.org/episode/rust-1.68-1.69/)
* [audio] [Pitching Rust to decision-makers, with Joel Marcey](https://rustacean-station.org/episode/joel-marcey-pitching-rust/)


### Rust Walkthroughs
* [🤗 Calling Hugging Face models from Rust](https://peterprototypes.com/blog/huggingface-from-rust/)
* [Rust Cross-Compilation With GitHub Actions](https://reemus.dev/tldr/rust-cross-compilation-github-actions)
* [tuify your clap CLI apps and make them more interactive](https://developerlife.com/2023/09/17/tuify-clap/)
* [Enhancing ClickHouse's Geospatial Support](https://tech.marksblogg.com/clickhouse-gis-rust.html)
* [video] [All Rust string types explained](https://www.youtube.com/watch?v=CpvzeyzgQdw)

### Research
* [A Grounded Conceptual Model for Ownership Types in Rust](https://blog.brownplt.org/2023/09/17/rust-ownership.html)
* [Debugging Trait Errors as Logic Programs](https://arxiv.org/abs/2309.05137)
* [REVIS: An Error Visualization Tool for Rust](https://arxiv.org/abs/2309.06640)

### Miscellaneous
* [JetBrains, You're scaring me. The Rust plugin deprecation situation.](https://chillfish8.ghost.io/jetbrains-youre-scaring-me/)

## Crate of the Week

This week's crate is [RustQuant](https://github.com/avhz/RustQuant), a crate for quantitative finance.

Thanks to [avhz](https://users.rust-lang.org/t/crate-of-the-week/2704/1238) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [r3bl_rs_utils - [tuify] Use nice ANSI symbols instead of ">" to decorate what row is currently selected](https://github.com/r3bl-org/r3bl_rs_utils/issues/133)
* [r3bl_rs_utils - [all] Use nu shell scripts (not just or fish) and add Github Actions to build & test on mac & linux](https://github.com/r3bl-org/r3bl_rs_utils/issues/120)
* [r3bl_rs_utils - [tuify] Use offscreen buffer from r3bl_tui to make repaints smooth](https://github.com/r3bl-org/r3bl_rs_utils/issues/118)
* [Ockam - make building of `ockam_app` create behind a feature flag](https://github.com/build-trust/ockam/issues/5977)
* [Ockam - Use the Terminal to print out RPC response instead of printlns](https://github.com/build-trust/ockam/issues/5904)
* [Hyperswitch - add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - separate payments_session from payments core](https://github.com/juspay/hyperswitch/issues/888)
* [Hyperswitch - move redis key creation to a common module](https://github.com/juspay/hyperswitch/issues/917)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

342 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-09-11..2023-09-18

* [`#[diagnostic::on_unimplemented]` without filters](https://github.com/rust-lang/rust/pull/114452)
* [`repr(transparent)`: it's fine if the one non-1-ZST field is a ZST](https://github.com/rust-lang/rust/pull/115334)
* [accept additional user-defined syntax classes in fenced code blocks](https://github.com/rust-lang/rust/pull/110800)
* [add `explicit_predicates_of` to SMIR](https://github.com/rust-lang/rust/pull/115793)
* [add `i686-pc-windows-gnullvm` triple](https://github.com/rust-lang/rust/pull/115687)
* [add diagnostic for raw identifiers in format string](https://github.com/rust-lang/rust/pull/115611)
* [add source type for invalid bool casts](https://github.com/rust-lang/rust/pull/115765)
* [cache `reachable_set` on disk](https://github.com/rust-lang/rust/pull/115740)
* [canonicalize effect vars in new solver](https://github.com/rust-lang/rust/pull/115850)
* [change `unsafe_op_in_unsafe_fn` to be `warn`-by-default from edition 2024](https://github.com/rust-lang/rust/pull/112038)
* [closure field capturing: don't depend on alignment of packed fields](https://github.com/rust-lang/rust/pull/115315)
* [consistently pass `ty::Const` through valtrees](https://github.com/rust-lang/rust/pull/115804)
* [coverage: simplify internal representation of debug types](https://github.com/rust-lang/rust/pull/115867)
* [disabled socketpair for Vita](https://github.com/rust-lang/rust/pull/115816)
* [enable varargs support for AAPCS calling convention](https://github.com/rust-lang/rust/pull/115860)
* [extend rustc -Zls](https://github.com/rust-lang/rust/pull/115735)
* [fallback effects even if types also fallback](https://github.com/rust-lang/rust/pull/115859)
* [fix `std::primitive` doc: homogenous → homogeneous](https://github.com/rust-lang/rust/pull/115329)
* [fix the error message for `#![feature(no_coverage)]`](https://github.com/rust-lang/rust/pull/115832)
* [fix: return early when has tainted in mir pass](https://github.com/rust-lang/rust/pull/115815)
* [improve Span in smir](https://github.com/rust-lang/rust/pull/115772)
* [improve `PadAdapter::write_char`](https://github.com/rust-lang/rust/pull/115782)
* [improve invalid let expression handling](https://github.com/rust-lang/rust/pull/115677)
* [inspect: closer to proof trees for coherence](https://github.com/rust-lang/rust/pull/115838)
* [llvm-wrapper: adapt for LLVM API changes](https://github.com/rust-lang/rust/pull/115871)
* [make `.rmeta` file in `dep-info` have correct name (`lib` prefix)](https://github.com/rust-lang/rust/pull/114750)
* [make `ty::Const` debug printing less verbose](https://github.com/rust-lang/rust/pull/115884)
* [make `useless_ptr_null_checks` smarter about some std functions](https://github.com/rust-lang/rust/pull/114494)
* [move `required_consts` check to general post-mono-check function](https://github.com/rust-lang/rust/pull/115748)
* [only suggest turbofish in patterns if we may recover](https://github.com/rust-lang/rust/pull/115785)
* [properly consider binder vars in `HasTypeFlagsVisitor`](https://github.com/rust-lang/rust/pull/115834)
* [read from non-scalar constants and statics in dataflow const-prop](https://github.com/rust-lang/rust/pull/115705)
* [remove `verbose_generic_activity_with_arg`](https://github.com/rust-lang/rust/pull/115736)
* [remove assert that checks type equality](https://github.com/rust-lang/rust/pull/115215)
* [resolve: mark binding is determined after all macros had been expanded](https://github.com/rust-lang/rust/pull/115269)
* [rework `no_coverage` to `coverage(off)`](https://github.com/rust-lang/rust/pull/114656)
* [small wins for formatting-related code](https://github.com/rust-lang/rust/pull/108043)
* [some ConstValue refactoring](https://github.com/rust-lang/rust/pull/115764)
* [some inspect improvements](https://github.com/rust-lang/rust/pull/115751)
* [treat host effect params as erased in codegen](https://github.com/rust-lang/rust/pull/115817)
* [turn custom code classes in docs into warning](https://github.com/rust-lang/rust/pull/115914)
* [visit `ExprField` for lint levels](https://github.com/rust-lang/rust/pull/115825)
* [store a index per dep node kind](https://github.com/rust-lang/rust/pull/115733)
* [stabilize the `Saturating` type](https://github.com/rust-lang/rust/pull/115477)
* [stabilize `const_transmute_copy`](https://github.com/rust-lang/rust/pull/115520)
* [make `Debug` impl for `ascii::Char` match that of `char`](https://github.com/rust-lang/rust/pull/115434)
* [add `minmax{,_by,_by_key}` functions to `core::cmp`](https://github.com/rust-lang/rust/pull/109409)
* [specialize count for range iterators](https://github.com/rust-lang/rust/pull/112229)
* [impl `Step` for IP addresses](https://github.com/rust-lang/rust/pull/113748)
* [add implementation for `thread::sleep_until`](https://github.com/rust-lang/rust/pull/113753)
* [cargo: cli: Add '-n' to dry-run](https://github.com/rust-lang/cargo/pull/12660)
* [cargo: pkgid: Allow incomplete versions when unambigious](https://github.com/rust-lang/cargo/pull/12614)
* [cargo: doc: differentiate defaults for split-debuginfo](https://github.com/rust-lang/cargo/pull/12680)
* [cargo: stabilize credential-process and registry-auth](https://github.com/rust-lang/cargo/pull/12649)
* [cargo: emit a warning for `credential-alias` shadowing](https://github.com/rust-lang/cargo/pull/12671)
* [cargo: generalise suggestion on abiguous spec](https://github.com/rust-lang/cargo/pull/12685)
* [cargo: limit cargo add feature print](https://github.com/rust-lang/cargo/pull/12662)
* [cargo: prerelease candidates error message](https://github.com/rust-lang/cargo/pull/12659)
* [cargo: consolidate clap/shell styles](https://github.com/rust-lang/cargo/pull/12655)
* [cargo: use `RegistryOrIndex enum` to replace two booleans](https://github.com/rust-lang/cargo/pull/12677)
* [rustfmt: Style help like cargo nightly](https://github.com/rust-lang/rustfmt/pull/5908)
* [clippy: ignore `#[doc(hidden)]` functions in clippy doc lints](https://github.com/rust-lang/rust/pull/115851)
* [clippy: reuse rustdoc's doc comment handling in Clippy](https://github.com/rust-lang/rust/pull/115689)
* [clippy: `extra_unused_type_parameters`: Fix edge case FP for parameters in where bounds](https://github.com/rust-lang/rust-clippy/pull/11484)
* [clippy: `filter_map_bool_then`: include multiple derefs from adjustments](https://github.com/rust-lang/rust-clippy/pull/11515)
* [clippy: `len_without_is_empty`: follow type alias to find inherent `is_empty` method](https://github.com/rust-lang/rust-clippy/pull/11452)
* [clippy: `used_underscore_bindings`: respect lint levels on the binding definition](https://github.com/rust-lang/rust-clippy/pull/11523)
* [clippy: `useless_conversion`: don't lint if type parameter has unsatisfiable bounds for `.into_iter()` receiver](https://github.com/rust-lang/rust-clippy/pull/11301)
* [clippy: fix FP of `let_unit_value` on async fn args](https://github.com/rust-lang/rust-clippy/pull/11509)
* [clippy: fix ICE by `u64::try_from(<u128>)`](https://github.com/rust-lang/rust-clippy/pull/11517)
* [clippy: trigger `transmute_null_to_fn` on chain of casts](https://github.com/rust-lang/rust-clippy/pull/11507)
* [clippy: fix `filter_map_bool_then` with a bool reference](https://github.com/rust-lang/rust-clippy/pull/11506)
* [clippy: ignore closures for some type lints](https://github.com/rust-lang/rust-clippy/pull/11504)
* [clippy: ignore span's parents in `collect_ast_format_args`/`find_format_args`](https://github.com/rust-lang/rust-clippy/pull/11473)
* [clippy: add `redundant_as_str` lint](https://github.com/rust-lang/rust-clippy/pull/11526)
* [clippy: add extra `byref` checking for the guard's local](https://github.com/rust-lang/rust-clippy/pull/11468)
* [clippy: new `unnecessary_map_on_constructor` lint](https://github.com/rust-lang/rust-clippy/pull/11413)
* [clippy: new lint: `path_ends_with_ext`](https://github.com/rust-lang/rust-clippy/pull/11483)
* [clippy: split `needless_borrow` into two lints](https://github.com/rust-lang/rust-clippy/pull/11511)
* [rust-analyzer: field shorthand overwritten in promote local to const assist](https://github.com/rust-lang/rust-analyzer/pull/15597)
* [rust-analyzer: don't skip closure captures after let-else](https://github.com/rust-lang/rust-analyzer/pull/15625)
* [rust-analyzer: fix lens location `"above_whole_item"` breaking lenses](https://github.com/rust-lang/rust-analyzer/pull/15606)
* [rust-analyzer: temporarily skip decl check in derive expansions](https://github.com/rust-lang/rust-analyzer/pull/15601)
* [rust-analyzer: prefer stable paths over unstable ones in import path calculation](https://github.com/rust-lang/rust-analyzer/pull/15611)

### Rust Compiler Performance Triage

A pretty quiet week, with relatively few statistically significant changes, though some good improvements to a number of benchmarks, particularly in cycle counts rather than instructions.

Triage done by **@simulacrum**.
Revision range: [7e0261e7ea..af78bae](https://perf.rust-lang.org/?start=7e0261e7ea2085bdc0bc3d0fd6776bf343473858&end=af78bae565e85b9c5698ee909af0652674eca6d4&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 2 Mixed; 2 of them in rollups

56 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-09-19.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Unicode and escape codes in literals](https://github.com/rust-lang/rfcs/pull/3349)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [stabilize combining +bundle and +whole-archive link modifiers](https://github.com/rust-lang/rust/pull/113301)
* [disposition: merge] [Stabilize `impl_trait_projections`](https://github.com/rust-lang/rust/pull/115659)
* [disposition: merge] [Tracking Issue for `option_as_slice`](https://github.com/rust-lang/rust/issues/108545)
* [disposition: merge] [Amend style guide section for formatting where clauses in type aliases](https://github.com/rust-lang/rust/pull/114901)
* [disposition: merge] [Add allow-by-default lint for unit bindings](https://github.com/rust-lang/rust/pull/112380)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Remove implicit features in a new edition](https://github.com/rust-lang/rfcs/pull/3491)
* [new] [RFC: const functions in traits](https://github.com/rust-lang/rfcs/pull/3490)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-09-20 - 2023-10-18 🦀

### Virtual

* 2023-09-20 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**SurrealDB for Rustaceans**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/295826608/)
* 2023-09-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Nightly Night: Generators**](https://www.meetup.com/vancouver-rust/events/295057154/)
* 2023-09-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295666673/)
* 2023-09-21 | Virtual (Cologne, DE) | [Cologne AWS User Group #AWSUGCGN](https://www.meetup.com/aws-cologne/)
    * [**AWS User Group Cologne - September Edition: Stefan Willenbrock: Developer Preview: Discovering Rust on AWS**](https://www.meetup.com/aws-cologne/events/294594401/)
* 2023-09-21 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 33rd Edition**](https://www.meetup.com/rust-linz/events/295363887/)
* 2023-09-21 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/295828383/)
* 2023-09-25 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**How we built the SurrealDB Python client in Rust.**](https://www.meetup.com/Rust-Dublin/events/294908596/)
* 2023-09-26 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679767/) | [**Mirror**](https://berline.rs/)
* 2023-09-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/295942051/)
* 2023-09-26 | Virtual (Melbourne, VIC, AU) | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - online & in person) September 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/296048213/)
* 2023-10-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295919493/)
* 2023-10-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-04 | Virtual (Various) | [Ferrous Systems](https://www.eventbrite.com/o/ferrous-systems-gmbh-68735392123)
    * [**A Decade of Rust with Ferrous Systems**](https://www.eventbrite.com/e/a-decade-of-rust-with-ferrous-systems-tickets-680492891557?aff=ebdssbdestsearch)
* 2023-10-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296135640/)
* 2023-10-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup: Mentorship (First Saturday)**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763617907?aff=erelpanelorg)
* 2023-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/) | [**Mirror**](https://berline.rs/)
* 2023-10-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtyfcnbnb/)
* 2023-10-11| Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcnbpb/)
* 2023-10-11 - 2023-10-13 | Virtual (Brussels, BE) | [EuroRust](https://eurorust.eu)
    * [**EuroRust 2023**](https://eurorust.eu)
* 2023-10-12 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732662/)
* 2023-10-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057159/)
 
### Asia

* 2023-09-25 | Singapore, SG | [Metacamp - Web3 Blockchain Community](https://www.meetup.com/singapore-web3-blockchain-meetup/)
    * [**Introduction to Rust**](https://www.meetup.com/singapore-web3-blockchain-meetup/events/296156132/)
* 2023-09-26 | Singapore, SG | [Rust Singapore](https://www.meetup.com/rust-singapore/)
    * [**SG Rustaceans! Updated - Singapore First Rust Meetup!**](https://www.meetup.com/rust-singapore/events/295505646/)
* 2023-10-03 | Taipei, TW | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/)
    * [**WebAssembly Meetup (Wasm Empowering AI) in Taipei**](https://www.meetup.com/wasm-rust-meetup/events/295672575/)

### Europe

* 2023-09-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus - Rust and Talk at Concordium**](https://www.meetup.com/rust-aarhus/events/294031975/)
* 2023-09-21 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2023 🦀**](https://www.meetup.com/rust-bern/events/295503351/)
* 2023-09-28 | Berlin, DE | [React Berlin](https://www.meetup.com/react-berlin-meetup/)
    * [**React Berlin September Meetup: Creating Videos with React & Remotion & More: Integrating Rust with React Native – Gheorghe Pinzaru**](https://www.meetup.com/react-berlin-meetup/events/295382108/)
* 2023-09-28 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/)
    * [**Primer evento Post COVID: ¡Cervezas MadRust!**](https://www.meetup.com/madrust/events/296063394/)
* 2023-09-28 | Paris, FR | [Paris Scala User Group (PSUG)](https://www.meetup.com/paris-scala-user-group-psug/events/296025234/)
    * [**PSUG #114 Comparons Scala et Rust**](https://www.meetup.com/paris-scala-user-group-psug/events/296025234/)
* 2023-09-30 | Saint Petersburg, RU | [Rust Saint Petersburg meetups](https://t.me/ruRust_spb)
    * [**Rust Community Meetup: A tale about how I tried to make my Blitz Basic - Vitaly; How to use nix to build projects on Rust – Danil; Getting to know tower middleware. General overview – Mikhail**](https://rurust-saint-petersburg-m.timepad.ru/event/2561864/) 
* 2023-10-10 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679773/)
* 2023-10-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295955356/)
* 2023-10-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**SIMD in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504251/)

### North America

* 2023-09-21 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**A Cargo Preview w/Ed Page, A Cargo Team Member**](https://www.meetup.com/utah-rust/events/294972877/)
* 2023-09-21 | Mountain View, CA, US| [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295747006/)
* 2023-09-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust on the web! Get started with Leptos**](https://www.meetup.com/music-city-rust-developers/events/295587220/)
* 2023-09-26 | Mountain View, CA, US| [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn)
    * [**Rust: snacks & learn**](https://www.meetup.com/rust-breakfast-learn/events/295579189/)
* 2023-09-26 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/295771515)
* 2023-09-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295466314)
* 2023-09-28 | Boulder, CO, US | [Solid State Depot - The Boulder Makerspace](https://www.meetup.com/solidstatedepot/)
    * [**Rust and ROS for Robotics + Happy Hour**](https://www.meetup.com/solidstatedepot/events/295466122/)
* 2023-10-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**First Meetup - Demo Day and Office Hours**](https://www.meetup.com/boulder-rust-meetup/events/296193722/)
* 2023-10-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**The Actor Model: Fearless Concurrency, Made Easy w/Chris Mena**](https://www.meetup.com/utah-rust/events/295771376/)
* 2023-10-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcnbwb/)

### Oceania

* 2023-09-26 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/295432237/)
* 2023-09-26 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - online & in person) September 2023 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/296048213/)
* 2023-09-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**September Meetup**](https://www.meetup.com/rust-brisbane/events/295946587/)

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

> This is the first programming language I've learned that makes it so easy to make test cases! It's actually a pleasure to implement them.

– [0xMB on rust-users](https://users.rust-lang.org/t/published-first-library-looking-for-feedback/99856/7)

Thanks to [Moy2010](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1467) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
