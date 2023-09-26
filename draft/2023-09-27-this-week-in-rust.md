Title: This Week in Rust 514
Number: 514
Date: 2023-09-27
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

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [async_fn_traits](https://docs.rs/async_fn_traits), a crate with async function traits to enable using higher ranked trait bounds in async functions.

Thanks to [kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1239) for the suggestion!

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

402 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-09-18..2023-09-25

* [*breaking change*: Validate crate name in `--extern`](https://github.com/rust-lang/rust/pull/116001)
* [add support for GNU/Hurd](https://github.com/rust-lang/rust/pull/115230)
* [enable ASAN/LSAN/TSAN for *-apple-ios-macabi](https://github.com/rust-lang/rust/pull/115644)
* [raise minimum supported Apple OS versions](https://github.com/rust-lang/rust/pull/104385)
* [`rustc_target/loongarch`: Fix passing of transparent unions with only one non-ZST member](https://github.com/rust-lang/rust/pull/115987)
* [`rustc_target/riscv`: Fix passing of transparent unions with only one non-ZST member](https://github.com/rust-lang/rust/pull/115499)
* [`rustc_hir_analysis`: add a helper to check function the signature mismatches](https://github.com/rust-lang/rust/pull/115897)
* [account for nested `impl Trait` in TAIT](https://github.com/rust-lang/rust/pull/116039)
* [add minimal std implementation for UEFI](https://github.com/rust-lang/rust/pull/105861)
* [add OwnedTargetMachine to manage llvm:TargetMachine](https://github.com/rust-lang/rust/pull/115911)
* [add Zba, Zbb, and Zbs as target features for riscv64-linux-android](https://github.com/rust-lang/rust/pull/116076)
* [add initial libstd support for Xous](https://github.com/rust-lang/rust/pull/104101)
* [adjust `ConstValue::Slice` to work for arbitrary slice types](https://github.com/rust-lang/rust/pull/115870)
* [adjust how closure/generator types are printed](https://github.com/rust-lang/rust/pull/115696)
* [allow `-Z treat-err-as-bug=0`](https://github.com/rust-lang/rust/pull/115690)
* [allow anyone to set llvm-fixed-upstream](https://github.com/rust-lang/rust/pull/115990)
* [allow higher-ranked fn sigs in `ValuePairs`](https://github.com/rust-lang/rust/pull/116073)
* [capture scrutinee of if let guards correctly](https://github.com/rust-lang/rust/pull/115999)
* [check that closure/generator's interior/capture types are sized](https://github.com/rust-lang/rust/pull/116081)
* [command: also print removed env vars](https://github.com/rust-lang/rust/pull/114379)
* [correctly deny late-bound lifetimes from parent in anon consts and TAITs](https://github.com/rust-lang/rust/pull/115486)
* [coverage: don't bother renumbering expressions on the Rust side](https://github.com/rust-lang/rust/pull/114399)
* [coverage: fix an unstable-sort inconsistency in coverage spans](https://github.com/rust-lang/rust/pull/115930)
* [coverage: remove debug code from the instrumentor](https://github.com/rust-lang/rust/pull/115962)
* [dependencies: reduce the amount of crates pulling in atty](https://github.com/rust-lang/rust/pull/115975)
* [detect cycle errors hidden by opaques during monomorphization](https://github.com/rust-lang/rust/pull/115801)
* [diagnostics: avoid mismatch between variance index and hir generic](https://github.com/rust-lang/rust/pull/116045)
* [do not create a DerefLen place for `Box<[T]>`](https://github.com/rust-lang/rust/pull/115794)
* [don't ICE when no bound vars found while doing closure hir type check](https://github.com/rust-lang/rust/pull/113396)
* [don't complain on a single non-exhaustive 1-ZST](https://github.com/rust-lang/rust/pull/115924)
* [don't modify libstd to dump rustc ICEs](https://github.com/rust-lang/rust/pull/115627)
* [don't resolve generic impls that may be shadowed by dyn built-in impls](https://github.com/rust-lang/rust/pull/114941)
* [enable -Zdrop-tracking-mir by default](https://github.com/rust-lang/rust/pull/107421)
* [enable effects for libcore](https://github.com/rust-lang/rust/pull/114776)
* [fall back to `_SC_NPROCESSORS_ONLN` if `sched_getaffinity` returns an empty mask](https://github.com/rust-lang/rust/pull/116038)
* [fall back to the unoptimized implementation in `read_binary_file` if `File::metadata` lies](https://github.com/rust-lang/rust/pull/115549)
* [fix `ui-fulldeps --stage=1` with `-Zignore-directory-in-diagnostics-source-blocks`](https://github.com/rust-lang/rust/pull/116009)
* [fix confusing let chain indentation in `rustc_resolve`](https://github.com/rust-lang/rust/pull/115983)
* [fix debug printing of tuple](https://github.com/rust-lang/rust/pull/116069)
* [give FutureIncompatibilityReason variants more explicit names](https://github.com/rust-lang/rust/pull/116049)
* [implement `Literal::byte_character`](https://github.com/rust-lang/rust/pull/112711)
* [implement `intercrate_ambiguity_causes` in the new solver](https://github.com/rust-lang/rust/pull/115996)
* [improve invalid UTF-8 lint by finding the expression initializer](https://github.com/rust-lang/rust/pull/115257)
* [interpret: more consistently use ImmTy in operators and casts](https://github.com/rust-lang/rust/pull/116010)
* [make unsized casts illegal](https://github.com/rust-lang/rust/pull/116056)
* [match on elem first while building move paths](https://github.com/rust-lang/rust/pull/115770)
* [more accurate suggestion for `self.` and `Self:`:](https://github.com/rust-lang/rust/pull/116086)
* [move `DepKind` to `rustc_query_system` and define it as `u16`](https://github.com/rust-lang/rust/pull/115920)
* [pass name of object file to LLVM so it can correctly emit `S_OBJNAME` in pdb files on Windows](https://github.com/rust-lang/rust/pull/115704)
* [point at cause of expectation of `break` value when possible](https://github.com/rust-lang/rust/pull/116071)
* [prevent promotion of const fn calls in inline consts](https://github.com/rust-lang/rust/pull/115936)
* [suggest desugaring to return-position `impl Future` when an `async fn` in trait fails an auto trait bound](https://github.com/rust-lang/rust/pull/115864)
* [tweak expected message to explain what it's actually signifying](https://github.com/rust-lang/rust/pull/116082)
* [miri: GC the Stacked Borrows allocation history](https://github.com/rust-lang/miri/pull/3083)
* [miri: deprecate -Zmiri-disable-abi-check](https://github.com/rust-lang/miri/pull/3071)
* [miri: implement `llvm.ctpop.v*` intrinsics](https://github.com/rust-lang/miri/pull/3072)
* [miri: issue discovered in TB: spurious reads are not (yet) possible in a concurrent setting](https://github.com/rust-lang/miri/pull/3054)
* [miri: move `llvm.x86.*` shims into `shims::x86` and implement `_addcarry_u32` and `_subborrow_u{32,64}`](https://github.com/rust-lang/miri/pull/3075)
* [open the FileEncoder file for reading and writing](https://github.com/rust-lang/rust/pull/116067)
* [simplify/Optimize FileEncoder](https://github.com/rust-lang/rust/pull/115542)
* [avoid overflow in `IoSlice::advance_slices`](https://github.com/rust-lang/rust/pull/116070)
* [call `panic_display` directly in `const_panic_fmt`](https://github.com/rust-lang/rust/pull/116007)
* [implement `cstr_count_bytes`](https://github.com/rust-lang/rust/pull/114443)
* [panic when encountering an illegal cpumask in `thread::available_parallelism`](https://github.com/rust-lang/rust/pull/115946)
* [add the `cfg_match!` macro](https://github.com/rust-lang/rust/pull/115416)
* [cargo: add some enhancements to `cargo clean`](https://github.com/rust-lang/cargo/pull/12638)
* [cargo: better suggestion for redundant mode in build and install commands](https://github.com/rust-lang/cargo/pull/12693)
* [cargo: buffer console status messages](https://github.com/rust-lang/cargo/pull/12727)
* [cargo: cargo add displays either feature list or summarized count](https://github.com/rust-lang/cargo/pull/12702)
* [cargo: doc: mention unstable flag `-Z asymmetric-token`](https://github.com/rust-lang/cargo/pull/12712)
* [cargo: fix spurious errors with networking tests](https://github.com/rust-lang/cargo/pull/12726)
* [cargo: fix: copy PDBs for EFI targets](https://github.com/rust-lang/cargo/pull/12688)
* [cargo: fix: use channel-specific link for registry auth error](https://github.com/rust-lang/cargo/pull/12709)
* [cargo: infra: add auto-trigger rules for new labels](https://github.com/rust-lang/cargo/pull/12713)
* [cargo: more specific registry index not found msg](https://github.com/rust-lang/cargo/pull/12732)
* [cargo: shortest path](https://github.com/rust-lang/cargo/pull/12678)
* [rustdoc-search: add support for type parameters](https://github.com/rust-lang/rust/pull/112725)
* [rustdoc: correctly render the return type of cross-crate async fns](https://github.com/rust-lang/rust/pull/116084)
* [rustdoc: custom code classes in docs warning](https://github.com/rust-lang/rust/pull/115947)
* [rustfmt: bugfix/comment duplication](https://github.com/rust-lang/rustfmt/pull/5913)
* [clippy: `redundant_guards`: lint if the pattern is on the left side](https://github.com/rust-lang/rust-clippy/pull/11522)
* [clippy: change defaults of `accept-comment-above-statement` and `accept-comment-above-attributes`](https://github.com/rust-lang/rust-clippy/pull/11170)
* [clippy: fix false positive with `needless_raw_string_hashes`](https://github.com/rust-lang/rust-clippy/pull/11518)
* [clippy: fix `cast_lossless` with macro call](https://github.com/rust-lang/rust-clippy/pull/11516)
* [clippy: fix mutably used async function argument in closure for `needless_pass_by_ref_mut`](https://github.com/rust-lang/rust-clippy/pull/11492)
* [clippy: fixed  caused by moving &mut reference inside of a closure](https://github.com/rust-lang/rust-clippy/pull/11551)
* [clippy: prevent ice when threshold is 0 and `enum` has no variants](https://github.com/rust-lang/rust-clippy/pull/11552)
* [clippy: remove most usage of `hir_ty_to_ty`](https://github.com/rust-lang/rust-clippy/pull/11544)
* [rust-analyzer: add `unused_variables` native diagnostic](https://github.com/rust-lang/rust-analyzer/pull/15659)
* [rust-analyzer: add option to show full function signatures in completion docs](https://github.com/rust-lang/rust-analyzer/pull/15582)
* [rust-analyzer: deunwrap `add_missing_match_arms`](https://github.com/rust-lang/rust-analyzer/pull/15594)
* [rust-analyzer: do not resolve inlayHint.textEdit for VSCode client](https://github.com/rust-lang/rust-analyzer/pull/15635)
* [rust-analyzer: bool to `enum` assist](https://github.com/rust-lang/rust-analyzer/pull/15484)
* [rust-analyzer: fix autoimport does nothing when importing trait that is as `_` imports](https://github.com/rust-lang/rust-analyzer/pull/15587)
* [rust-analyzer: fix inlining closures from local variables and functions](https://github.com/rust-lang/rust-analyzer/pull/15651)
* [rust-analyzer: give `unmerge_use` a label explaining what it will affect](https://github.com/rust-lang/rust-analyzer/pull/15621)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

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

Rusty Events between 2023-09-27 - 2023-10-25 🦀

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
* 2023-09-21 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295747006/)
* 2023-09-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust on the web! Get started with Leptos**](https://www.meetup.com/music-city-rust-developers/events/295587220/)
* 2023-09-26 | Mountain View, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn)
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

> The problem with Rust it appears,  
> that it leaves programmers in tears  
> if they have to go back  
> to languages that lack  
> in short they've got feature-arrears.

– [llogiq on /r/rust](https://www.reddit.com/r/rust/comments/16mv8bb/comment/k1buhp0/)

Thanks to [Frank Steffahn](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1468) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
