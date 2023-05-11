Title: This Week in Rust 494
Number: 494
Date: 2023-05-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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

* [API token scopes for crates.io](https://blog.rust-lang.org/inside-rust/2023/05/09/api-token-scopes.html)
* [Updating Rust's Linux musl targets](https://blog.rust-lang.org/2023/05/09/Updating-musl-targets.html)

### Newsletters

* [Rust Magazine: Issue 3](https://rustmagazine.org/issue-3/)

### Observations/Thoughts

* [A guide to test parametrization in Rust](https://unterwaditzer.net/2023/rust-test-parametrization.html)
* [Iterator, Generator](https://without.boats/blog/iterator-generator/)
* [Breaking semver in Rust by adding a private type, or by adding an import](https://predr.ag/blog/breaking-semver-in-rust-by-adding-private-type-or-import/)
* [Ask not what the compiler can do for you](https://github.com/juspay/hyperswitch/wiki/Ask-not-what-the-compiler-can-do-for-you)

### Rust Walkthroughs

* [How We Built Our Own Time-Tracking Algorithm for a Rust app](https://michellelim.dev/writing/measure-time-spent-in-app/)
* [ESP32 Embedded Rust at the HAL: UART Serial Communication](https://apollolabsblog.hashnode.dev/esp32-embedded-rust-at-the-hal-uart-serial-communication?ref=twitter-share)
* [Build a non-binary tree that is thread safe using Rust](https://developerlife.com/2022/02/24/rust-non-binary-tree/)
* [Coherence in Rust (feat. rustc sources)](https://ohadravid.github.io/posts/2023-05-coherence-and-errors/)
* [video] [Beginner's guide to Rust's Result, "?" and Try/FromResidual](https://youtu.be/NSqN2r0h8DE)
* [video] [Crust of Rust: std::collections](https://youtu.be/EF3Z4jdD1EQ)
* [ES] [video] [Introducción a Rust en Español, Parte 2: Todo sobre variables, let, mut, scopes, tipos y más.](https://youtu.be/6icd5wwsvF8)

### Miscellaneous

* [How Rust-based search engine Meilisearch uses dynamic virtual address management to scale indexes](https://blog.meilisearch.com/dynamic-virtual-address-management/)
* [The Way of the Crab Podcast, Epside 1: Hello, World!](https://wayofthecrab.com/episodes/001/)
* [Hangman over QUIC (using Rust and Quinn)](https://ochagavia.nl/blog/hangman-over-quic/)

## Crate of the Week

This week's crate is [dlhn](https://crates.io/crates/dlhn), a serde-compatible serialization format geared for performance.

Thanks to [Shogo Otake](https://users.rust-lang.org/t/crate-of-the-week/2704/1193) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [velo - Implement simple bevy-markdown renderer](https://github.com/StaffEngineer/velo/issues/93)
* [ockam - `#[ockam::node]` macro doesn't handle returned errors](https://github.com/build-trust/ockam/issues/4662)
* [ockam - Make clap command asynchronously wait for the changes to take place](https://github.com/build-trust/ockam/issues/4885)

* [Hyperswitch - move connector config to a separate file](https://github.com/juspay/hyperswitch/issues/1109)
* [Hyperswitch - Implement `ReverseLookupInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/1115)
* [Hyperswitch - Implement `EventInterface` for `MockDb`](https://github.com/juspay/hyperswitch/issues/1116)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

**Calls for Proposals**

Open calls for submissions to conferences and meetups.

* [EuroRust Call for Speakers (By 2023-06-11)](https://www.papercall.io/eurorust-2023)

## Updates from the Rust Project

386 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-05-01..2023-05-08

* [implement `c"foo"` literals](https://github.com/rust-lang/rust/pull/108801) (RFC [#3348](https://rust-lang.github.io/rfcs/3348-c-str-literal.html))
* [output LLVM optimization remark kind in `-Cremark` output](https://github.com/rust-lang/rust/pull/111203)
* [STD support for PSVita](https://github.com/rust-lang/rust/pull/110638)
* [add `force` option for `--extern` flag](https://github.com/rust-lang/rust/pull/109421)
* [`rustc_middle`: Fix `opt_item_ident` for non-local def ids](https://github.com/rust-lang/rust/pull/111146)
* [add FreeBSD cpuset support to `std::thread::available_concurrency`](https://github.com/rust-lang/rust/pull/110830)
* [add deployment-target --print flag for Apple targets](https://github.com/rust-lang/rust/pull/105354)
* [add hint for `=<` as `<=`](https://github.com/rust-lang/rust/pull/111230)
* [avoid alignment mismatch between ABI and layout for unions](https://github.com/rust-lang/rust/pull/104872)
* [check arguments length in trivial diagnostic lint](https://github.com/rust-lang/rust/pull/111289)
* [check array type of repeat exprs is wf](https://github.com/rust-lang/rust/pull/111100)
* [constProp into `PlaceElem::Index`](https://github.com/rust-lang/rust/pull/110824)
* [constify slice flatten method](https://github.com/rust-lang/rust/pull/111127)
* [correctly recurse when expanding anon consts](https://github.com/rust-lang/rust/pull/111103)
* [debuginfo: split method declaration and definition](https://github.com/rust-lang/rust/pull/111167)
* [disable nrvo mir opt](https://github.com/rust-lang/rust/pull/111007)
* [do not recurse into const generic args when resolving self lifetime elision](https://github.com/rust-lang/rust/pull/110982)
* [don't bail out early when checking invalid `repr` attr](https://github.com/rust-lang/rust/pull/111062)
* [don't compute trait super bounds unless they're positive](https://github.com/rust-lang/rust/pull/111211)
* [don't validate constants in const propagation](https://github.com/rust-lang/rust/pull/109521)
* [emit `while_true` lint spanning the entire loop condition](https://github.com/rust-lang/rust/pull/111300)
* [enable `rust_2018_idioms` lint group for doctests](https://github.com/rust-lang/rust/pull/106621)
* [encode def span for foreign return-position `impl Trait` in trait](https://github.com/rust-lang/rust/pull/111039)
* [ensure test library issues json string line-by-line](https://github.com/rust-lang/rust/pull/109729)
* [expand the LLVM coverage of `--print target-cpus`](https://github.com/rust-lang/rust/pull/111274)
* [explicitly reject negative and reservation drop impls](https://github.com/rust-lang/rust/pull/110859)
* [fix elaboration with associated type bounds](https://github.com/rust-lang/rust/pull/110512)
* [fix lifetime suggestion for type aliases with objects in them](https://github.com/rust-lang/rust/pull/110827)
* [fix miscompilation when calling default methods on `Future`](https://github.com/rust-lang/rust/pull/111354)
* [fix some suggestions where a `Box<T>` is expected](https://github.com/rust-lang/rust/pull/111056)
* [fix spans in LLVM-generated inline asm errors](https://github.com/rust-lang/rust/pull/110985)
* [forbid escaping bound vars in combine](https://github.com/rust-lang/rust/pull/111246)
* [further normalize msvc-non-utf8-ouput](https://github.com/rust-lang/rust/pull/111262)
* [implement `tuple<->array` convertions via `From`](https://github.com/rust-lang/rust/pull/97594)
* [implement negative bounds for internal testing purposes](https://github.com/rust-lang/rust/pull/110791)
* [interpret: fail more gracefully on uninit unsized locals](https://github.com/rust-lang/rust/pull/110943)
* [introduce `AliasKind::Inherent` for inherent associated types](https://github.com/rust-lang/rust/pull/109410)
* [leave promoteds untainted by errors when borrowck fails](https://github.com/rust-lang/rust/pull/111038)
* [make PlaceMention a non-mutating use](https://github.com/rust-lang/rust/pull/110826)
* [make `(try_)subst_and_normalize_erasing_regions` take `EarlyBinder`](https://github.com/rust-lang/rust/pull/110297)
* [make `generics_of has_self` on RPITITs delegate to the opaque](https://github.com/rust-lang/rust/pull/111265)
* [make some simple queries no longer cache on disk](https://github.com/rust-lang/rust/pull/111028)
* [optimize builder sizes](https://github.com/rust-lang/rust/pull/110846)
* [prevent aborting guard from aborting the process in a forced unwind](https://github.com/rust-lang/rust/pull/104070)
* [reduce MIR dump file count for MIR-opt tests](https://github.com/rust-lang/rust/pull/110773)
* [refactor `core::char::EscapeDefault` and co. structures](https://github.com/rust-lang/rust/pull/105076)
* [reject borrows of projections in ConstProp](https://github.com/rust-lang/rust/pull/110954)
* [replace `tcx.mk_trait_ref` with `TraitRef::new`](https://github.com/rust-lang/rust/pull/110806)
* [suggest `struct` when we get colon in fileds in `enum`](https://github.com/rust-lang/rust/pull/111118)
* [support return-type bounds on associated methods from supertraits](https://github.com/rust-lang/rust/pull/111161)
* [uplift `clippy::clone_double_ref` as `suspicious_double_ref_op`](https://github.com/rust-lang/rust/pull/110955)
* [use fulfillment to check `Drop` impl compatibility](https://github.com/rust-lang/rust/pull/110577)
* [miri: avoid interpreting code that has lint errors](https://github.com/rust-lang/miri/pull/2869)
* [miri: clearer variable names in `data_race`](https://github.com/rust-lang/miri/pull/2876)
* [miri: simplify event selection in TB diagnostics](https://github.com/rust-lang/miri/pull/2867)
* [box AssertKind](https://github.com/rust-lang/rust/pull/111082)
* [stabilize `debugger_visualizer`](https://github.com/rust-lang/rust/pull/108668)
* [stabilize raw-dylib, `link_ordinal, import_name_type` and -Cdlltool](https://github.com/rust-lang/rust/pull/109677)
* [partial stabilisation of `c_unwind`](https://github.com/rust-lang/rust/pull/106075)
* [inline SocketAddr methods](https://github.com/rust-lang/rust/pull/111125)
* [add 64-bit `time_t` support on 32-bit glibc Linux to `set_times`](https://github.com/rust-lang/rust/pull/110093)
* [make sure the implementation of `TcpStream::as_raw_fd` is fully inlined](https://github.com/rust-lang/rust/pull/111057)
* [add `ascii::Char`](https://github.com/rust-lang/rust/pull/111009)
* [`assume` the runtime range of `align_offset`](https://github.com/rust-lang/rust/pull/111113)
* [`btree_map: Cursor{,Mut}::peek_prev` must agree](https://github.com/rust-lang/rust/pull/111238)
* [add `is_positive` method for signed non-zero integers](https://github.com/rust-lang/rust/pull/111186)
* [constify `[u8]::is_ascii` (unstably)](https://github.com/rust-lang/rust/pull/111222)
* [fix `checked_{add,sub}_duration` incorrectly returning `None` when `other` has more than `i64::MAX` seconds](https://github.com/rust-lang/rust/pull/103056)
* [correctly convert an NT path to a Win32 path in `read_link`](https://github.com/rust-lang/rust/pull/107978)
* [libtest: include test output in junit xml reports](https://github.com/rust-lang/rust/pull/110651)
* [hashbrown: mark `RawTable::data_start` NonNull](https://github.com/rust-lang/hashbrown/pull/387)
* [hashbrown: special case `clear()` on empty tables](https://github.com/rust-lang/hashbrown/pull/428)
* [cargo: metadata: add `workspace_default_members`](https://github.com/rust-lang/cargo/pull/11978)
* [cargo: do not try an exponential number of package names](https://github.com/rust-lang/cargo/pull/12083)
* [cargo: remove repeated definite articles](https://github.com/rust-lang/cargo/pull/12067)
* [cargo: support for shallow clones and fetches with `gitoxide`](https://github.com/rust-lang/cargo/pull/11840)
* [rustdoc-search: add slices and arrays to index](https://github.com/rust-lang/rust/pull/110780)
* [rustdoc: restructure type search engine to pick-and-use IDs](https://github.com/rust-lang/rust/pull/110371)
* [clippy: `imprecise_flops`: Globally ignore `#[no_std]` crates](https://github.com/rust-lang/rust-clippy/pull/10730)
* [clippy: `wildcard_imports` ignore `test.rs` files](https://github.com/rust-lang/rust-clippy/pull/10584)
* [clippy: add lint to detect construction of unit `struct` using `default`](https://github.com/rust-lang/rust-clippy/pull/10716)
* [clippy: ignore `borrow_deref_ref` warnings in code from procedural macros](https://github.com/rust-lang/rust-clippy/pull/10761)
* [clippy: ignore expressions from macros in `default_constructed_unit_structs`](https://github.com/rust-lang/rust-clippy/pull/10752)
* [clippy: initial `clippy::ref_patterns` implementation](https://github.com/rust-lang/rust-clippy/pull/10736)
* [rust-analyzer: add config for disabling hover memory layout data](https://github.com/rust-lang/rust-analyzer/pull/14758)
* [rust-analyzer: closure capture inlay hints](https://github.com/rust-lang/rust-analyzer/pull/14742)
* [rust-analyzer: creating rust dependencies tree explorer](https://github.com/rust-lang/rust-analyzer/pull/11557)
* [rust-analyzer: define problem matcher for panics in VS Code](https://github.com/rust-lang/rust-analyzer/pull/14749)
* [rust-analyzer: emit function bodies in expanding builtin derives](https://github.com/rust-lang/rust-analyzer/pull/14725)
* [rust-analyzer: fix pattern type mismatches for bindings, enable pattern type mismatch diagnostics again](https://github.com/rust-lang/rust-analyzer/pull/14732)
* [rust-analyzer: highlight closure captures when cursor is on pipe or move keyword](https://github.com/rust-lang/rust-analyzer/pull/14711)
* [rust-analyzer: fix body lowering not using block def maps](https://github.com/rust-lang/rust-analyzer/pull/14738)
* [rust-analyzer: fix some mir related bugs](https://github.com/rust-lang/rust-analyzer/pull/14705)
* [rust-analyzer: generate delegate methods filters out functions that already exist on the struct's impls](https://github.com/rust-lang/rust-analyzer/pull/14707)
* [rust-analyzer: ide: do not highlight escapes in raw strings](https://github.com/rust-lang/rust-analyzer/pull/14713)
* [rust-analyzer: ide: exclude sized in go-to actions in hover](https://github.com/rust-lang/rust-analyzer/pull/14714)
* [rust-analyzer: ignore impls with `#[rustc_reservation_impl]`](https://github.com/rust-lang/rust-analyzer/pull/14750)
* [rust-analyzer: lazy evaluate consts in `path_to_const`](https://github.com/rust-lang/rust-analyzer/pull/14727)
* [rust-analyzer: only pass unstable flags to cargo metadata from extra args config](https://github.com/rust-lang/rust-analyzer/pull/14712)
* [rust-analyzer: parse bare dyn types with leading lifetime](https://github.com/rust-lang/rust-analyzer/pull/14739)
* [rust-analyzer: show type alias layout](https://github.com/rust-lang/rust-analyzer/pull/14748)
* [rust-analyzer: sort rust dependencies in vscode tree view](https://github.com/rust-lang/rust-analyzer/pull/14745)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add a `[lints]` table to `Cargo.toml`](https://github.com/rust-lang/rfcs/pull/3389)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [New rustc and Cargo options to allow path sanitisation by default](https://github.com/rust-lang/rfcs/pull/3127)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Remove misleading target feature aliases](https://github.com/rust-lang/rust/pull/107707)
* [disposition: merge] [expand: Change how `#![cfg(FALSE)]` behaves on crate root](https://github.com/rust-lang/rust/pull/110141)
* [disposition: merge] [do not allow inference in `predicate_must_hold` (alternative approach)](https://github.com/rust-lang/rust/pull/110100)
* [disposition: merge] [Tracking Issue for slice::split_at in const context](https://github.com/rust-lang/rust/issues/101158)
* [disposition: merge] [Implement `AsHandle`/`AsSocket` for `Arc`/`Rc`/`Box` on Windows](https://github.com/rust-lang/rust/pull/108196)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Unix socket ancillary data v2](https://github.com/rust-lang/rfcs/pull/3430)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-05-10 - 2023-06-07 🦀

### Virtual

* 2023-05-10 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/292464903)
* 2023-05-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/gmkpctyfchbpb/)
* 2023-05-11 | Virtual (South Pasadena, CA, US) | [Pasadena Thursday Go / Rust](https://www.meetup.com/thursday-go/)
    * [**Weekly study group**](https://www.meetup.com/thursday-go/events/293338004)
* 2023-05-13 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-05-13 | Virtual + In person (Singapore, SG) | [Web3Dev.Community](https://www.meetup.com/web3devc/)
    * [**[Hybrid] You'll Never Rust Alone - Rust Study Group**](https://www.meetup.com/web3devc/events/zcgndtyfchbrb/)
* 2023-05-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/jkxsctyfchbvb/)
* 2023-05-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Atomics and Locks Book Club Chapter 2**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292847157/)
* 2023-05-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Spin and Kata Containers**](https://www.meetup.com/vancouver-rust/events/lqkkctyfchbwb/)
* 2023-05-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfchbxb/)
* 2023-05-20 | Virtual + In person (Singapore, SG) | [Web3Dev.Community](https://www.meetup.com/web3devc/)
    * [**[Hybrid] You'll Never Rust Alone - Rust Study Group**](https://www.meetup.com/web3devc/events/zcgndtyfchbbc/)
* 2023-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293302808)
* 2023-05-25 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Practical Monads**](https://www.meetup.com/charlottesville-rust-meetup/events/293384348)
* 2023-05-25 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Proyecto "Taller de Rust"**](https://www.meetup.com/rust-mx/events/293332410)
* 2023-05-25 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA) - various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293349464)
* 2023-05-25 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: Discussing Rust, Ballista, Ray SQL, DataFusion with Andy Grove**](https://www.meetup.com/data-ai-online/events/293432877)
* 2023-05-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/293014934)
* 2023-05-31 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396/?chapterContext=true&regToRsvp=true&isFromReg=true)
* 2023-06-06 | Virtual (Austin, TX, US) | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting - Run Rust Warp in WasmEdge -- Alan, Poon Yong Quan**](https://www.meetup.com/webassembly-and-wasmedge/events/293014949)
* 2023-06-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/293296995)
* 2023-06-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309294)

### Asia

* 2023-05-13 | Singapore, SG | [Web3Dev.Community](https://www.meetup.com/web3devc/)
    * [**[Hybrid] You'll Never Rust Alone - Rust Study Group**](https://www.meetup.com/web3devc/events/zcgndtyfchbrb/)
* 2023-05-18 | Seoul, KR | [Seoul Substrate Blockchain Meetup](https://www.meetup.com/seoul-substrate-blockchain-meetup/)
    * [***Seoul Substrate Meetup - 최신 cyprography - Rust*](https://www.meetup.com/seoul-substrate-blockchain-meetup/events/293016466)
* 2023-05-20 | Singapore, SG | [Web3Dev.Community](https://www.meetup.com/web3devc/)
    * [**[Hybrid] You'll Never Rust Alone - Rust Study Group**](https://www.meetup.com/web3devc/events/zcgndtyfchbbc/)
* 2023-05-25 | Amsterdam, NL | [Frontend Developer Meetup Amsterdam](https://www.meetup.com/frontend-developer-meetup-amsterdam/)
    * [**Svelte Frontend Meetup (signup required) - Building a Svelte-Rust app using Tauri**](https://www.meetup.com/frontend-developer-meetup-amsterdam/events/293272364)

### Europe

* 2023-05-10 | Amsterdam, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2023**](https://2023.rustnl.org/)
* 2023-05-19 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfchbzb/)
* 2023-05-23 | Paris, FR | [Kaïbee](https://www.meetup.com/kaibee/)
    * [**Atelier Axum & Rust**](https://www.meetup.com/kaibee/events/293169086)
* 2023-05-24 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #4**](https://www.meetup.com/fr-FR/rust-lyon/events/293322211)
* 2023-05-25 | Barcelona, ES | [C++ Programmer Meetup.](https://www.meetup.com/c-programmer-meetup/)
    * [**Rust for C++ Developers.**](https://www.meetup.com/c-programmer-meetup/events/292816507)
* 2023-05-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #36 at Adapt Agency!**](https://www.meetup.com/copenhagen-rust-community/events/293293863)
* 2023-05-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #59**](https://www.meetup.com/rust-paris/events/293191172)
* 2023-05-30 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**10th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/293363107)

### North America

* 2023-05-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Interesting Title and Food!**](https://www.meetup.com/utah-rust/events/rrwbctyfchbpb/)
* 2023-05-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfchbvb/)
* 2023-05-17 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Too many unit tests: A tale of macros and BigDecimals**](https://www.meetup.com/rust-nyc/events/293316694)

### Oceania

* 2023-05-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/292717772/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/12tehic/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Thanks to all for the very helpful responses. "The Book" says *The community is very welcoming and happy to answer students’ questions* "; I expected that to be just marketing, but I was wrong."

– [Daryl Lee on rust-users](https://users.rust-lang.org/t/w-why-is-it-called-unit/93521/8)

Thanks to [evann](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1413) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/13ehxq8/this_week_in_rust_494/)</small>
