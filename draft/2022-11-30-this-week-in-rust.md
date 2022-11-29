Title: This Week in Rust 471
Number: 471
Date: 2022-11-30
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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [code-radio-cli](https://github.com/JasonWei512/code-radio-cli), a command line interface for listening to freeCodeCamp's Code Radio music stream.

Thanks to [魏杰](https://users.rust-lang.org/t/crate-of-the-week/2704/1126) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

389 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-11-21..2022-11-28

* [add powerpc64-ibm-aix as Tier-3 target](https://github.com/rust-lang/rust/pull/102293)
* [stabilize native library modifier `verbatim`](https://github.com/rust-lang/rust/pull/104360)
* [use clang for the UEFI targets](https://github.com/rust-lang/rust/pull/104622)
* [optimize field ordering by grouping m×2ⁿ-sized fields with equivalently aligned ones](https://github.com/rust-lang/rust/pull/102750)
* [allow opaque types in trait impl headers and rely on coherence to reject unsound cases](https://github.com/rust-lang/rust/pull/103488)
* [allow power10-vector feature in PowerPC](https://github.com/rust-lang/rust/pull/104704)
* [branch `Clause` from `Predicate`](https://github.com/rust-lang/rust/pull/104846)
* [enable `fuzzy_provenance_casts` lint in liballoc and libstd](https://github.com/rust-lang/rust/pull/104647)
* [enable profiler in dist-riscv64-linux](https://github.com/rust-lang/rust/pull/104431)
* [lint: do not warn unused parens around higher-ranked function pointers](https://github.com/rust-lang/rust/pull/104796)
* [llvm-wrapper adapt for LLVM API change](https://github.com/rust-lang/rust/pull/104880)
* [make `deref_into_dyn_supertrait` lint the impl and not the usage](https://github.com/rust-lang/rust/pull/104742)
* [pass `InferCtxt` to `DropRangeVisitor` so we can resolve vars](https://github.com/rust-lang/rust/pull/104753)
* [pretty-print generators with their `generator_kind`](https://github.com/rust-lang/rust/pull/104931)
* [privacy: fix more (potential) issues with effective visibilities](https://github.com/rust-lang/rust/pull/104602)
* [properly handle `Pin<&mut dyn* Trait>` receiver in codegen](https://github.com/rust-lang/rust/pull/104594)
* [resolve: don't use constructor def ids in the map for field names](https://github.com/rust-lang/rust/pull/104747)
* [separate lifetime ident from lifetime resolution in HIR](https://github.com/rust-lang/rust/pull/104048)
* [stricter alignment enforcement for ScalarPair and Vector](https://github.com/rust-lang/rust/pull/105006)
* [suggest `.clone()` or `ref binding` on E0382](https://github.com/rust-lang/rust/pull/103908)
* [miri: fix handling of spurious accesses during retag](https://github.com/rust-lang/miri/pull/2694)
* [miri: make `no_std` work on Windows](https://github.com/rust-lang/miri/pull/2696)
* [miri: track local frames incrementally during execution](https://github.com/rust-lang/miri/pull/2647)
* [miri: use `.wasm` extension when building for wasm in cargo-miri](https://github.com/rust-lang/miri/pull/2685)
* [use an IndexVec to cache queries with index-like key](https://github.com/rust-lang/rust/pull/103808)
* [fix perf regression by correctly matching keywords](https://github.com/rust-lang/rust/pull/104410)
* [`rustc_metadata`: switch module children decoding to an iterator](https://github.com/rust-lang/rust/pull/104730)
* [codegen\_gcc: fix `simd_bitmask`](https://github.com/rust-lang/rustc_codegen_gcc/pull/240)
* [codegen\_gcc: fix the argument order for some AVX-512 intrinsics](https://github.com/rust-lang/rustc_codegen_gcc/pull/241)
* [don't build `compiler_builtins` with `-C panic=abort`](https://github.com/rust-lang/rust/pull/103786)
* [manually implement PartialEq for `Option<T>` and specialize non-nullable types](https://github.com/rust-lang/rust/pull/103556)
* [stop peeling the last iteration of the loop in `Vec::resize_with`](https://github.com/rust-lang/rust/pull/104818)
* [constify remaining `Layout` methods](https://github.com/rust-lang/rust/pull/102207)
* [mark `sys_common::once::generic::Once::new` const-stable](https://github.com/rust-lang/rust/pull/103193)
* [add slice methods for indexing via an array of indices](https://github.com/rust-lang/rust/pull/83608)
* [futures: `stream::size_hint` for mpsc channels](https://github.com/rust-lang/futures-rs/pull/2660)
* [futures: custom `Debug` implementations for `mpsc`](https://github.com/rust-lang/futures-rs/pull/2667)
* [futures: remove `Debug` constraint for `oneshot` types](https://github.com/rust-lang/futures-rs/pull/2666)
* [portable SIMD: avoid a scalar loop in `Simd::from_slice`](https://github.com/rust-lang/portable-simd/pull/318)
* [regex: speed up replacen loop](https://github.com/rust-lang/regex/pull/930)
* [`rustc_codegen_ssa`: write `.dwp` in a streaming fashion](https://github.com/rust-lang/rust/pull/104797)
* [cargo: add error message for `cargo fix` on an empty repo](https://github.com/rust-lang/cargo/pull/11400)
* [cargo: add suggestions when `cargo add` multiple packages](https://github.com/rust-lang/cargo/pull/11186)
* [cargo: store the sparse+ prefix in the SourceId for sparse registries](https://github.com/rust-lang/cargo/pull/11387)
* [rustdoc: improve popover focus handling JS](https://github.com/rust-lang/rust/pull/104946)
* [bindgen: add `--wrap-unsafe-ops` option](https://github.com/rust-lang/rust-bindgen/pull/2354)
* [bindgen: add `ParseCallbacks::process_comment`](https://github.com/rust-lang/rust-bindgen/pull/2347)
* [bindgen: deprecate Rust targets lower or equal than `1.30`](https://github.com/rust-lang/rust-bindgen/pull/2356)
* [bindgen: escape method fragments that happen to be rust keywords](https://github.com/rust-lang/rust-bindgen/pull/2359)
* [bindgen: remove traits that have a single implementation](https://github.com/rust-lang/rust-bindgen/pull/2363)
* [clippy: add `clippy_utils::msrv::Msrv` to keep track of the current MSRV](https://github.com/rust-lang/rust-clippy/pull/9924)
* [clippy: add allow-mixed-uninlined-format-args config](https://github.com/rust-lang/rust-clippy/pull/9865)
* [clippy: fix `unnecessary_to_owned` false positive](https://github.com/rust-lang/rust-clippy/pull/9796)
* [clippy: fix `redundant_closure_for_method_calls` suggestion](https://github.com/rust-lang/rust-clippy/pull/9745)
* [clippy: fix `unnecessary_lazy_eval` when type has significant drop](https://github.com/rust-lang/rust-clippy/pull/9750)
* [clippy: lint unnecessary safety comments](https://github.com/rust-lang/rust-clippy/pull/9851)
* [clippy: re-enable `uninlined_format_args` on multiline `format!`](https://github.com/rust-lang/rust-clippy/pull/9945)
* [clippy: remove blank lines when `needless_return` returns no value](https://github.com/rust-lang/rust-clippy/pull/9967)
* [rust-analyzer: add `deriveHelper` to `semanticTokenTypes` section of package.json](https://github.com/rust-lang/rust-analyzer/pull/13670)
* [rust-analyzer: add assist to generate trait impl's](https://github.com/rust-lang/rust-analyzer/pull/13592)
* [rust-analyzer: adds hover hint to ".." in record pattern](https://github.com/rust-lang/rust-analyzer/pull/13638)
* [rust-analyzer: check tail expressions more precisely in `extract_function`](https://github.com/rust-lang/rust-analyzer/pull/13681)
* [rust-analyzer: filter unnecessary completions after colon](https://github.com/rust-lang/rust-analyzer/pull/13611)
* [rust-analyzer: handle empty `checkOnSave/target` values](https://github.com/rust-lang/rust-analyzer/pull/13661)
* [rust-analyzer: handle sysroot config in detached-files workspaces](https://github.com/rust-lang/rust-analyzer/pull/13667)
* [rust-analyzer: tuple to named struct inside macros](https://github.com/rust-lang/rust-analyzer/pull/13647)
* [rust-analyzer: hir-expand: fix `compile_error!` expansion not unquoting strings](https://github.com/rust-lang/rust-analyzer/pull/13652)
* [rust-analyzer: improve goto declaration](https://github.com/rust-lang/rust-analyzer/pull/13671)
* [rust-analyzer: properly implement Drop for JodGroupChild](https://github.com/rust-lang/rust-analyzer/pull/13669)
* [rust-analyzer: suppress "Implement default members" inside contained items](https://github.com/rust-lang/rust-analyzer/pull/13576)
* [cargo-bisect-rustc: --start without --end defaults `end` to be today](https://github.com/rust-lang/cargo-bisect-rustc/pull/240)
* [cc-rs: refine CUDA support](https://github.com/rust-lang/cc-rs/pull/712)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-11-30 - 2022-12-28 🦀

### Virtual

* 2022-11-24 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 27th Edition**](https://www.meetup.com/rust-linz/events/289251460/)
* 2022-11-28 | Virtual | [Rust Formal Methods Interest Group](https://www.eventbrite.com/o/rust-formal-methods-interest-group-34404947509)
    * [**MiniRust with Ralf Jung**](https://www.eventbrite.com/e/minirust-with-ralf-jung-tickets-460741328717)
* 2022-11-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcpbmc/)
* 2022-11-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Common crates and their usage**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289645553/)
* 2022-11-30 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)
* 2022-12-01 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Exploring USB with Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/289563986/)
* 2022-12-01 | Virtual (Lehi, UT, US) | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Beginner Projects and Shop Talk with Food!**](https://www.meetup.com/utah-rust/events/289899804/)
* 2022-12-01 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Getting Started with Rust: Understanding Rust Compile Errors – Part 2**](https://www.meetup.com/microsoft-reactor-redmond/events/289830539/) 
* 2022-12-06 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/12/06/rust-hack-and-learn.html)
* 2022-12-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcqbjb/)
* 2022-12-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/287027660/)
* 2022-12-07 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcqbkb/)
* 2022-12-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #20**](https://www.meetup.com/rust-noris/events/hlvbvsydcqblb/)
* 2022-12-08 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: The Genesis of Delta Rust with QP Hou**](https://www.meetup.com/data-ai-online/events/289672886/)
* 2022-12-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-12-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/289352426/)
* 2022-12-13 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 25u16**](https://www.meetup.com/rust-saar/events/289663288/)
* 2022-12-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcqbsb/)
* 2022-12-20 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/12/20/rust-hack-and-learn.html)
* 2022-12-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsydcqbbc/)
* 2022-12-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcqbcc/)

### Europe

* 2022-11-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Initial Meet and Greet Rust meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/289028178/)
* 2022-11-24 | København, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #31**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179132/)
* 2022-11-28 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust London Code Dojo: Rust with Front-End Web Assembly**](https://www.meetup.com/rust-london-user-group/events/289631916/)
* 2022-11-30 | Amsterdam, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust in Critical Infrastructure**](https://www.meetup.com/rust-nederland/events/289204146/)
* 2022-11-30 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet)
    * [**Rust Lille #1**](https://www.meetup.com/meetup-group-zgphbyet/events/289620614/)
* 2022-11-30 | Milan, IT | [Rust Language Milano](https://www.meetup.com/rust-language-milano/)
    * [**Welcome GAT!!**](https://www.meetup.com/rust-language-milano/events/289767176/)
* 2022-11-30 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #54**](https://www.meetup.com/rust-paris/events/289833645/)
* 2022-11-30 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)
* 2022-12-01 | Edinburgh, UK | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**December Talks + Rust Book Raffle**](https://www.meetup.com/rust-edi/events/289582990/)
* 2022-12-01 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #30**](https://www.meetup.com/rust-wroclaw/events/289884642/)
* 2022-12-07 | Zurich, CH | [Rust Zurich](https://www.meetup.com/Rust-Zurich/)
    * [**Next generation i18n with rust (icu4x) and zero-copy deserialization**](https://www.meetup.com/rust-zurich/events/289518586/)
* 2022-12-12 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Rust Meetup - Subject TBA**](https://www.meetup.com/dutch-rust-meetup/events/289021643/)

### North America

* 2022-11-29 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**Atx Rustaceans Meetup**](https://www.meetup.com/atx-rustaceans/events/289594614/)
* 2022-12-01 | Lehi, UT, US + Virtual | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Beginner Projects and Shop Talk with Food!**](https://www.meetup.com/utah-rust/events/289899804/)
* 2022-12-08 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcqblb/)
* 2022-12-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcqbbc/)

### Oceania

* 2022-11-24 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**November Meetup**](https://www.meetup.com/rust-brisbane/events/289539610/)
* 2022-12-08 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**December 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/289745823/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> After many years of writing bugs, and then discovering Rust, I learned to appreciate explicitness in code, and hope you eventually will too.

– [Artem Borisovskiy on rust-users](https://users.rust-lang.org/t/how-to-add-a-crate/84602/11)

Thanks to [Árpád Goretity](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1334) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
