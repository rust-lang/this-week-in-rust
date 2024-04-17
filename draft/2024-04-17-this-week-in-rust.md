Title: This Week in Rust 543
Number: 543
Date: 2024-04-17
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

### Rust Nation UK
* [Tim McNamara - 4 levels of error handling](https://www.youtube.com/watch?v=hQWZGOuDYTE)
* [Mithun Hunsur - Ambient: A Rust and WebAssembly Runtime for Cross-Platform Multiplayer Games](https://www.youtube.com/watch?v=tSnKx2irwzE)
* [Alice Ryhl - What it takes to keep Tokio running](https://www.youtube.com/watch?v=Lc3aw_NIOt0)
* [Chris Biscardi - Bevy: A case study in ergonomic Rust](https://www.youtube.com/watch?v=CnoDOc6ML0Y&t=51s)
* [Pietro Albini - How Ferrocene qualified the Rust Compiler](https://www.youtube.com/watch?v=_ITnWoPvMKA)
* [Ben Wishovich - Full Stack Rust - Building Rust Websites with Leptos](https://www.youtube.com/watch?v=JJV5crU405s)
* [Natalie Serebryakova - Rustic Persistence: Automating PVC Lifecycles with Rust in Kubernetes](https://www.youtube.com/watch?v=n-ESPxF11tM)
* [Daniel McKenna - Creating a Text-To-Speech System in Rust](https://www.youtube.com/watch?v=HiqId_9pysM)
* [Konstantin Grechishchev - Java and Rust Integration](https://www.youtube.com/watch?v=fYaaBoKbDQs)
* [Heiko Seeberger - EventSourced – async_fn_in_trait in anger](https://www.youtube.com/watch?v=z40rgjZqrs4)
* [Tim Janus - Let's get interdisciplinary: Rust Design Patterns for Chemical Plants](https://www.youtube.com/watch?v=aK5lHOJxl98)
* [Marco Ieni - How Rust makes open-source easier](https://www.youtube.com/watch?v=HzTZoh7WaGo)

### Foundation

### Newsletters
* [New Meshes, New Examples, and Compute Shaders](https://thisweekinbevy.com/issue/2024-04-15-new-meshes-new-examples-and-compute-shaders)

### Project/Tooling Updates
* [futures-concurrency v7.6.0: Portable Concurrent Async Iteration](https://github.com/yoshuawuyts/futures-concurrency/releases/tag/v7.6.0)
* [Ratatui v0.26.2](https://ratatui.rs/highlights/v0262/)
* [Rust on Espressif chips](https://mabez.dev/blog/posts/esp-rust-12-04-2024/)
* [Introducing Dust DDS – A native Rust implementation of the Data Distribution Service (DDS) middleware](https://www.s2e-systems.com/2024/04/12/introducing_dust_dds/)
* [Announcing the first audited Rust implementation of CGGMP21, the state-of-the-art ECDSA threshold protocol](https://www.dfns.co/article/cggmp21-in-rust-at-last)
* [Nutype 0.4.2 - newtype with guarantees](https://github.com/greyblake/nutype/releases/tag/v0.4.2)
* [venndb 0.2.1 - any filters](https://github.com/plabayo/venndb/releases/tag/0.2.1)
* [ZH|EN] [Announcing async-openai-wasm, and thoughts on wasmization and streams](https://ideas.reify.ing/en/blog/announcing-async-openai-wasm/)

### Observations/Thoughts
* [Climbing a (binary) Tree - Noise On The Net](https://noiseonthenet.space/noise/2024/04/climbing-a-binary-tree/)
* [Why is there no realloc that takes the number of bytes to copy?](https://shift.click/blog/missing-alloc-api/)
* [Some useful types for database-using Rust web apps](https://boinkor.net/2024/04/some-useful-types-for-database-using-rust-web-apps/)
* [My logging recipe for server side Rust](https://www.matildasmeds.com/posts/rust-logging-recipe/)

### Rust Walkthroughs
* [Getting started with SurrealDB using Docker and a Rust client](https://rust.code-maven.com/surrealdb-with-docker)
* [video] [developerlife.com - Rust testing deep dive with r3bl_terminal_async crate](https://www.youtube.com/watch?v=4iM9t5dgvU4)

### Research
* [Rust Digger: 7.53% of crates have both 'edition' and 'rust-version', 11.21% have neither](https://rust-digger.code-maven.com/news/msrv-stats)

### Miscellaneous
* [Iced Tutorial 0.12](https://leafheap.com/articles/iced-tutorial-version-0-12)
* [video] [Infinite Pong in the Bevy Game Engine - Let's Code!](https://www.youtube.com/watch?v=vwUz95Oo8IA)
* [audio] [Release-plz with Marco Ieni](https://rustacean-station.org/episode/marco-ieni/)

## Crate of the Week

This week's crate is [venndb](https://crates.io/crates/venndb), an append-only memory DB whose tables can be build via a derive macro.

Thanks to [Glen De Cauwsemaecker](https://users.rust-lang.org/t/crate-of-the-week/2704/1303) for the self-suggestion!

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

* [mirrord - medschool generated malformed JSON](https://github.com/metalbear-co/mirrord/issues/1668)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [RustConf 2024](https://foundation.rust-lang.org/news/the-rustconf-2024-call-for-talk-proposals-is-open/) | Closes 2024-04-25 | Montreal, Canada | Event date: 2024-09-10
* [RustLab 2024](https://sessionize.com/rustlab-2024) | Closes 2024-05-01 | Florence, Italy | Event date: 2024-11-09 - 2024-11-11
* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| Closes 2024-06-03 | Vienna, Austria & online | Event date: 2024-10-10
* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/)| Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Closes 2024-07-22 | online | Event date: 2024-08-22

* [RustConf 2024](https://foundation.rust-lang.org/news/the-rustconf-2024-call-for-talk-proposals-is-open/) | Closes 2024-04-25 | Montreal, Canada | Event date: 2024-09-10

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

430 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-04-09..2024-04-16

* [add support for Arm64EC inline assembly (as unstable)](https://github.com/rust-lang/rust/pull/123507)
* [`statx` probe: `ENOSYS` might come from a faulty FUSE driver](https://github.com/rust-lang/rust/pull/123928)
* [account for trait/impl difference when suggesting changing argument from ref to mut ref](https://github.com/rust-lang/rust/pull/123523)
* [add `REDUNDANT_LIFETIMES` lint to detect lifetimes which are semantically redundant](https://github.com/rust-lang/rust/pull/118391)
* [add `unsafe` to two functions with safety invariants](https://github.com/rust-lang/rust/pull/123867)
* [add const generics support for pattern types](https://github.com/rust-lang/rust/pull/123689)
* [add support to intrinsics fallback body](https://github.com/rust-lang/rust/pull/123659)
* [async closure coroutine by move body MirPass refactoring](https://github.com/rust-lang/rust/pull/123668)
* [avoid a panic in `set_output_capture` in the default panic handler](https://github.com/rust-lang/rust/pull/122882)
* [be more specific when flagging imports as redundant due to the extern prelude](https://github.com/rust-lang/rust/pull/122954)
* [call `lower_const_param` instead of duplicating the code](https://github.com/rust-lang/rust/pull/123738)
* [call the panic hook for non-unwind panics in proc-macros](https://github.com/rust-lang/rust/pull/123825)
* [detect borrow checker errors where `.clone()` would be an appropriate user action](https://github.com/rust-lang/rust/pull/122603)
* [disable Ctrl-C handling on WASM](https://github.com/rust-lang/rust/pull/123788)
* [discard overflow obligations in `impl_may_apply`](https://github.com/rust-lang/rust/pull/123618)
* [do not add prolog for variadic naked functions](https://github.com/rust-lang/rust/pull/123249)
* [do not allocate for ZST ThinBox (attempt 2 using `const_allocate`)](https://github.com/rust-lang/rust/pull/123254)
* [don't delay a bug if we suggest adding a semicolon to the RHS of an assign operator](https://github.com/rust-lang/rust/pull/123736)
* [don't do coroutine-closure-specific upvar analysis if tainted by errors](https://github.com/rust-lang/rust/pull/123834)
* [don't even parse an intrinsic unless the feature gate is enabled](https://github.com/rust-lang/rust/pull/123603)
* [don't leak unnameable types in `-> _` recover](https://github.com/rust-lang/rust/pull/123931)
* [don't rely on upvars being assigned just because coroutine-closure kind is assigned](https://github.com/rust-lang/rust/pull/123662)
* [fix UB in LLVM FFI when passing zero or \>1 bundle](https://github.com/rust-lang/rust/pull/123941)
* [fix invalid silencing of parsing error](https://github.com/rust-lang/rust/pull/123223)
* [fix various bugs in `ty_kind_suggestion`](https://github.com/rust-lang/rust/pull/123924)
* [generic associated consts: Check regions earlier when comparing impl with trait item def](https://github.com/rust-lang/rust/pull/123898)
* [improve diagnostic by suggesting to remove visibility qualifier](https://github.com/rust-lang/rust/pull/123841)
* [just use `type_dependent_def_id` to figure out what the method is for an expr](https://github.com/rust-lang/rust/pull/123989)
* [linker flavors next steps: linker features](https://github.com/rust-lang/rust/pull/123656)
* [linker: avoid some allocations in search directory iteration](https://github.com/rust-lang/rust/pull/123827)
* [linker: remove laziness and caching from native search directory walks](https://github.com/rust-lang/rust/pull/123854)
* [make `PlaceRef` and `OperandValue::Ref` share a common `PlaceValue` type](https://github.com/rust-lang/rust/pull/123775)
* [make the computation of `coroutine_captures_by_ref_ty` more sophisticated](https://github.com/rust-lang/rust/pull/123660)
* [only assert for child/parent projection compatibility AFTER checking that theyre coming from the same place](https://github.com/rust-lang/rust/pull/123701)
* [only collect mono items from reachable blocks](https://github.com/rust-lang/rust/pull/123272)
* [openBSD fix long socket addresses](https://github.com/rust-lang/rust/pull/123779)
* [panic on overflow in `BorrowedCursor::advance`](https://github.com/rust-lang/rust/pull/123806)
* [propagate temporary lifetime extension into if and match](https://github.com/rust-lang/rust/pull/121346)
* [provide suggestion to dereference closure tail if appropriate](https://github.com/rust-lang/rust/pull/122213)
* [refactor `panic_unwind/seh.rs` pointer use](https://github.com/rust-lang/rust/pull/123490)
* [remove `From` impls for unstable types that break inference](https://github.com/rust-lang/rust/pull/123830)
* [rework ptr-to-ref conversion suggestion for method calls](https://github.com/rust-lang/rust/pull/123007)
* [set target-abi module flag for RISC-V targets](https://github.com/rust-lang/rust/pull/123612)
* [skip `unused_parens` report for `Paren(Path(..))` in macro](https://github.com/rust-lang/rust/pull/123314)
* [stop making any assumption about the projections applied to the upvars in the `ByMoveBody` pass](https://github.com/rust-lang/rust/pull/123658)
* [stop using `HirId` for fn-like parents since closures are not `OwnerNode`s](https://github.com/rust-lang/rust/pull/123804)
* [stop using `PolyTraitRef` for closure/coroutine predicates already instantiated w placeholders](https://github.com/rust-lang/rust/pull/123900)
* [store all args in the unsupported Command implementation](https://github.com/rust-lang/rust/pull/123633)
* [suppress `let else` suggestion for uninitialized refutable `let`s](https://github.com/rust-lang/rust/pull/123847)
* [tweak value suggestions in `borrowck` and `hir_analysis`](https://github.com/rust-lang/rust/pull/123704)
* [typeck: fix `?` suggestion span](https://github.com/rust-lang/rust/pull/123654)
* [use `fn` ptr signature instead of `{closure@..}` in infer error](https://github.com/rust-lang/rust/pull/123703)
* [use `suggest_impl_trait` in return type suggestion on type error](https://github.com/rust-lang/rust/pull/123761)
* [miri: `MIRI_REPLACE_LIBRS_IF_NOT_TEST`: also apply to crates.io crates](https://github.com/rust-lang/miri/pull/3457)
* [miri: add some basic support for GetFullPathNameW](https://github.com/rust-lang/miri/pull/3466)
* [miri: fix error display for './miri run --dep'](https://github.com/rust-lang/miri/pull/3465)
* [miri: handle Miri sysroot entirely outside the Miri driver](https://github.com/rust-lang/miri/pull/3411)
* [miri: make `split_simd_to_128bit_chunks` take only one operand](https://github.com/rust-lang/miri/pull/3462)
* [miri on Windows: run .CRT$XLB linker section on thread-end](https://github.com/rust-lang/rust/pull/123937)
* [miri: windows: add basic support for FormatMessageW](https://github.com/rust-lang/miri/pull/3464)
* [stabilize --json `unused-externs(-silent)`](https://github.com/rust-lang/rust/pull/115717)
* [stabilize `(const_)slice_ptr_len` and `(const_)slice_ptr_is_empty_nonnull`](https://github.com/rust-lang/rust/pull/123868)
* [stabilize `cstr_count_bytes`](https://github.com/rust-lang/rust/pull/123661)
* [implement `FromIterator` for `(impl Default + Extend, impl Default + Extend)`](https://github.com/rust-lang/rust/pull/107462)
* [re-enable `has_thread_local` for i686-msvc](https://github.com/rust-lang/rust/pull/123257)
* [`std::net`: TcpListener shrinks the backlog argument to 32 for Haiku](https://github.com/rust-lang/rust/pull/123857)
* [show `mode_t` as octal in `std::fs` Debug impls](https://github.com/rust-lang/rust/pull/122812)
* [add `A: 'static` bound for `Arc/Rc::pin_in`](https://github.com/rust-lang/rust/pull/120092)
* [`f16` and `f128` step 4: basic library support](https://github.com/rust-lang/rust/pull/122470)
* [add a `Debug` impl and some basic functions to `f16` and `f128`](https://github.com/rust-lang/rust/pull/123783)
* [specialize many implementations of `Read::read_buf_exact`](https://github.com/rust-lang/rust/pull/122393)
* [windows: set main thread name without re-encoding](https://github.com/rust-lang/rust/pull/123534)
* [cargo: make sure to also wrap the initial `-vV` invocation](https://github.com/rust-lang/cargo/pull/13659)
* [cargo resolve: Respect '--ignore-rust-version'](https://github.com/rust-lang/cargo/pull/13738)
* [cargo resolve: Fallback to 'rustc -V' for MSRV resolving](https://github.com/rust-lang/cargo/pull/13743)
* [cargo fix: dont apply same suggestion twice](https://github.com/rust-lang/cargo/pull/13728)
* [cargo package: Normalize paths in `Cargo.toml`](https://github.com/rust-lang/cargo/pull/13729)
* [cargo test: don't compress test registry crates](https://github.com/rust-lang/cargo/pull/13744)
* [rustdoc: correctly handle inlining of doc hidden foreign items](https://github.com/rust-lang/rust/pull/123459)
* [rustdoc: check redundant explicit links with correct itemid](https://github.com/rust-lang/rust/pull/123905)
* [rustdoc: point at span in `include_str!`-ed md file](https://github.com/rust-lang/rust/pull/123204)
* [rustdoc: reduce per-page HTML overhead](https://github.com/rust-lang/rust/pull/123706)
* [clippy: `module_name_repetition` Recognize common prepositions](https://github.com/rust-lang/rust-clippy/pull/12573)
* [clippy: fix: incorrect suggestions when `.then` and `.then_some` is used](https://github.com/rust-lang/rust-clippy/pull/12094)
* [clippy: pin `remark-lint-maximum-line-length` version](https://github.com/rust-lang/rust-clippy/pull/12668)
* [clippy: turn `duplicated_attributes` into a late lint](https://github.com/rust-lang/rust-clippy/pull/12646)
* [clippy: use `check_attributes` in doc lints](https://github.com/rust-lang/rust-clippy/pull/12635)
* [rust-analyzer: add static and const highlighting token types](https://github.com/rust-lang/rust-analyzer/pull/17074)
* [rust-analyzer: better inline preview for postfix completion](https://github.com/rust-lang/rust-analyzer/pull/17073)
* [rust-analyzer: wrap/Unwrap `cfg_attr`](https://github.com/rust-lang/rust-analyzer/pull/16813)
* [rust-analyzer: VFS should not confuse paths with source roots that have the same prefix](https://github.com/rust-lang/rust-analyzer/pull/17019)
* [rust-analyzer: fix `impl Trait<Self>` causing stack overflows](https://github.com/rust-lang/rust-analyzer/pull/16877)
* [rust-analyzer: fix inlay hint resolution being broken](https://github.com/rust-lang/rust-analyzer/pull/17063)
* [rust-analyzer: fix: support auto-closing for triple backticks](https://github.com/rust-lang/rust-analyzer/pull/17051)
* [rust-analyzer: run cargo test per workspace in the test explorer](https://github.com/rust-lang/rust-analyzer/pull/17056)

### Rust Compiler Performance Triage

A quiet week, with slightly more improvements than regressions.
There were a few noise spikes, but other than that nothing too interesting.

Triage done by **@Kobzol**.
Revision
range: [86b603cd..ccfcd950b](https://perf.rust-lang.org/?start=86b603cd792b3f6172ba4f676d7b586c1af7630a&end=ccfcd950b333fed046275dd8d54fe736ca498aa7&absolute=false&stat=instructions%3Au)

**Summary**:

|         (instructions:u)          | mean  |     range      | count |
|:---------------------------------:|:-----:|:--------------:|:-----:|
|  Regressions ❌ <br /> (primary)   | 0.5%  |  [0.3%, 1.4%]  |   9   |
| Regressions ❌ <br /> (secondary)  | 0.4%  |  [0.2%, 1.1%]  |  20   |
|  Improvements ✅ <br /> (primary)  | -0.6% | [-2.5%, -0.2%] |  41   |
| Improvements ✅ <br /> (secondary) | -0.8% | [-1.4%, -0.2%] |   4   |
|         All ❌✅ (primary)          | -0.4% | [-2.5%, 1.4%]  |  50   |

1 Regressions, 3 Improvements, 6 Mixed; 5 of them in rollups
62 artifact comparisons made in total

[Full report here](https://github.com/Kobzol/rustc-perf/blob/28ee0f9b94c85d8591588b84a4048f46ab3fe0c2/triage/2024-04-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [Move the Crates.io Team under the Dev Tools team](https://github.com/rust-lang/rfcs/pull/3595)
* [disposition: merge] [Arbitrary self types v2](https://github.com/rust-lang/rfcs/pull/3519)
* [disposition: merge] [RFC: Syntax for embedding cargo-script manifests](https://github.com/rust-lang/rfcs/pull/3503)
* [disposition: merge] [rust-lang org GitHub access policy](https://github.com/rust-lang/rfcs/pull/2872)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Enforce closure args + return type are WF](https://github.com/rust-lang/rust/pull/123531)
* [disposition: merge] [Tracking Issue for `io_error_downcast`](https://github.com/rust-lang/rust/issues/99262)
* [disposition: merge] [More DefineOpaqueTypes::Yes](https://github.com/rust-lang/rust/pull/123794)
* [disposition: merge] [Tracking Issue for `std::path::absolute`](https://github.com/rust-lang/rust/issues/92750)
* [disposition: merge] [Tracking Issue for `utf8_chunks`](https://github.com/rust-lang/rust/issues/99543)
* [disposition: merge] [restrict promotion of `const fn` calls](https://github.com/rust-lang/rust/pull/121557)
* [disposition: merge] [Fix trait solver overflow with `non_local_definitions` lint](https://github.com/rust-lang/rust/pull/123594)
* [disposition: merge] [Use fulfillment in method probe, not evaluation ](https://github.com/rust-lang/rust/pull/122317)
* [disposition: merge] [rustdoc-search: single result for items with multiple paths](https://github.com/rust-lang/rust/pull/119912)
* [disposition: merge] [Ignore `-C strip` on MSVC](https://github.com/rust-lang/rust/pull/115120)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2024-04-17 - 2024-05-15 🦀

### Virtual

* 2024-04-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Reflections on RustNation UK 2024**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325512/)
* 2024-04-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Camigo (Peter Kehl)**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-04-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368799/)
* 2024-04-21 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Using AstroNvim for Rust development (in Hebrew)**](https://www.meetup.com/code-mavens/events/300265648/)
* 2024-04-23 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Show and Tell in April**](https://www.meetup.com/rust-trondheim/events/300469130/)
* 2024-04-24 | Virtual + In Person (Prague, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**#2: Making Safe Rust Safer (Pavel Šimerda)**](https://www.meetup.com/rust-czech-republic/events/300388563)
* 2024-04-25 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477692/)
* 2024-04-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcgbnc/)
* 2024-05-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 5 - Project Structure**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325526/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)
* 2024-05-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368804/)
* 2024-05-07 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)
* 2024-05-09 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477697/)
* 2024-05-09 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Rust at Microsoft, Tel Aviv - Are we embedded yet?**](https://www.meetup.com/code-mavens/events/300144781/)
* 2024-05-09 | Virtual (Nuremberg/Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945257/)
* 2024-05-14| Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/298341699/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)

### Africa

* 2024-05-04 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Asia

* 2024-04-20 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Talk & Workshop - Parallel Programming April 2024**](https://docs.google.com/forms/d/e/1FAIpQLSfeWzcnWic--G2Sj6uJFJNc_L2Iv7J27hIofZwhBYXu2CbUjQ/viewform) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)
* 2024-05-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2024-rustacean-meetup/)

### Europe

* 2024-04-17 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/)
    * [**Lær Rust med Conways Game of Life**](https://www.meetup.com/bergen-html-css-meetup-group/events/300031586/)
* 2024-04-17 | Lyon, FR | [Rust Lyon](https://www.meetup.com/rust-lyon/)
    * [**Rust Lyon Meetup #10**](https://www.meetup.com/rust-lyon/events/300268616/)
* 2024-04-17 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/)
    * [**TechMeetup: RUST**](https://www.meetup.com/techmeetupostrava/events/299912212/)
* 2024-04-20 | Augsburg, DE | [Augsburger Linux-Infotag 2024](https://www.luga.de/static/LIT-2024/)
   * [**Augsburger Linux-Infotag 2024: Workshop Einstieg in Embedded Rust mit dem Raspberry Pico WH**](https://www.luga.de/static/LIT-2024/talks/einstieg_in_embedded_rust_mit_dem_raspberry_pico_wh/)
* 2024-04-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust'n'Tell - Rust for the Web**](https://www.meetup.com/rust-berlin/events/300047151/)
* 2024-04-23 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #67**](https://mobilizon.fr/events/4ba93021-c43a-4e4a-b3e5-09c1c0d0a957)
* 2024-04-24 | Virtual + In Person (Prague, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**#2: Making Safe Rust Safer (Pavel Šimerda)**](https://www.meetup.com/rust-czech-republic/events/300388563)
* 2024-04-25 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at MFT Energy**](https://www.meetup.com/rust-aarhus/events/299564517/)
* 2024-04-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - TBD**](https://www.meetup.com/rust-berlin/events/299288960/)
* 2024-04-25 | København/Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust meetup #46 sponsored by Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/300458178/)
* 2024-04-25 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna x Python User Group - April**](https://www.meetup.com/rust-vienna/events/300389154/)
* 2024-04-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Fullstack Rust - Workshop #2 (Register by 23 April)**](https://www.meetup.com/rust-basel/events/299933581/)
* 2024-04-27 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #2**](https://www.meetup.com/stockholm-rust/events/300369409)
* 2024-04-30 | Budapest, HU | [Budapest Rust Meetup Group](https://www.meetup.com/budapest-rust-meetup-group/)
    * [**Rust Meetup Budapest 2**](https://www.meetup.com/budapest-rust-meetup-group/events/300269044/)
* 2024-04-30 | Salzburg, AT | Rust Salzburg
    * [**Rust Salzburg meetup**]: 6:30pm - CCC Salzburg, 1. OG, ArgeKultur, Ulrike-Gschwandtner-Straße 5, 5020 Salzburg
* 2024-05-01 | Utrecht, NL | [NL-RSE Community](https://nl-rse.org/events/2024-05-01-meetup)
    * [**NL-RSE RUST meetup**](https://www.eventbrite.nl/e/nl-rse-rust-meetup-tickets-871056271757)
* 2024-05-06 | Delft, NL | [GOSIM](https://www.gosim.org/)
    * [**GOSIM Europe 2024**](https://europe2024.gosim.org/)
* 2024-05-07 & 2024-05-08 | Delft, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2024**](https://2024.rustnl.org/)
* 2024-05-09 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #2**](https://www.meetup.com/rust-gdansk/events/299766774/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)

### North America

* 2024-04-18 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk: What Are Panics?**](https://www.meetup.com/deep-dish-rust/events/300204763/)
* 2024-04-18 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803586/)
* 2024-04-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299960315/)
* 2024-04-25 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers - Async Rust on Embedded**](https://www.meetup.com/music-city-rust-developers/events/299976876/)
* 2024-04-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Apr 26**](https://www.meetup.com/bostonrust/events/300116689/)
* 2024-05-04 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, May 4**](https://www.meetup.com/bostonrust/events/300116701/)
* 2024-05-12 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, May 12**](https://www.meetup.com/bostonrust/events/300116747/)

### Oceania

* 2024-04-17 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**WMaTIR 2024 Gala & Talks**](https://www.meetup.com/rust-sydney/events/299882966/)
* 2024-04-30 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Why Rust? Convince Me!**](https://www.meetup.com/rust-akl/events/300304958/)
* 2024-04-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**CRUG April Meetup: Generics and Traits**](https://www.meetup.com/rust-canberra/events/300023000/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1bpg8b8/official_rrust_whos_hiring_thread_for_jobseekers)

# Quote of the Week

> There is absolutely no way I can imagine that `Option` is causing that error. That'd be like turning on the "Hide Taskbar" setting causing your GPU to catch fire.
>
> [...]
>
> If it's not any of those, consider an exorcist because your machine *might* be haunted.

– [Daniel Keep on rust-users](https://users.rust-lang.org/t/access-is-denied-os-error-5/109515/2)

Thanks to [Hayden Brown](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1561) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1c6o935/this_week_in_rust_543)</small>
