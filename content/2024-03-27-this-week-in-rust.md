Title: This Week in Rust 540
Number: 540
Date: 2024-03-27
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
* [Announcing Rust 1.77.0](https://blog.rust-lang.org/2024/03/21/Rust-1.77.0.html)
* [2024 Edition Update](https://blog.rust-lang.org/inside-rust/2024/03/22/2024-edition-update.html)
* [This Development-cycle in Cargo: 1.78](https://blog.rust-lang.org/inside-rust/2024/03/26/this-development-cycle-in-cargo-1.78.html)

### Project/Tooling Updates
* [Unimock 0.6: Mutation patterns](https://audunhalland.github.io/blog/unimock-0-6/)
* [cargo-autoinherit: DRY up your workspace dependencies](https://mainmatter.com/blog/2024/03/18/cargo-autoinherit/)
* [rust-analyzer changelog #226](https://rust-analyzer.github.io/thisweek/2024/03/25/changelog-226.html)
* [reqwest v0.12](https://seanmonstar.com/blog/reqwest-v012/)
* [Fornjot v0.49.0 released - open source b-rep CAD kernel in Rust](https://www.fornjot.app/blog/release/0.49.0/)
* [Quickwit 0.8: Indexing and Search at Petabyte Scale](https://quickwit.io/blog/quickwit-0.8)

### Observations/Thoughts
* [Why choose async/await over threads?](https://notgull.net/why-not-threads/)
* [Rustls: Continuous Benchmarking Case Study](https://bencher.dev/learn/case-study/rustls/)
* [How I reduced (incremental) Rust compile times by up to 40%](https://www.coderemote.dev/blog/faster-rust-compiler-macro-expansion-caching/)
* [How I learned to stop worrying and love the global state](https://symbolica.io/posts/global_state/)
* [On Tech Debt: My Rust Library is now a CDO](https://lucumr.pocoo.org/2024/3/26/rust-cdo/)
* [How to use Rust on Debian (and Ubuntu, etc.)](https://diziet.dreamwidth.org/18122.html)
* [audio] [Hyper 1.0 with Sean McArthur](https://rustacean-station.org/episode/sean-mcarthur/)
* [video] [The magic of Rust's type system](https://www.youtube.com/watch?v=NDIU1GSBrVI)

### Rust Walkthroughs
* [Safe and Secure Coding in Rust: A Comparative Analysis of Rust and C/C++](https://luk6xff.github.io/other/safe_secure_rust_book)
* [It's a library AND a binary](https://blog.axo.dev/2024/03/its-a-lib-and-a-bin)
* [Everything you need to know about testing in Rust](https://www.shuttle.rs/blog/2024/03/21/testing-in-rust)
* [Building a Notification Service in Rust with AWS SNS](https://www.shuttle.rs/blog/2024/03/20/notification-service-rust)
* [Embedded Rust Bluetooth on ESP: BLE Server](https://apollolabsblog.hashnode.dev/embedded-rust-bluetooth-on-esp-ble-server)
* [series] [How Meilisearch Updates a Millions Vector Embeddings Database in Under a Minute](https://blog.kerollmops.com/how-meilisearch-updates-a-millions-vector-embeddings-database-in-under-a-minute)
* [video] [Extreme Clippy for a new Crate](https://youtu.be/dEkr5c5Kul8)

### Miscellaneous
* [Making Nix Usable With Rust](https://filtra.io/rust-flox-mar-24)
* [Rust Development Classes: learn Rust by doing](https://rust-classes.com/)
* [Restarting development of the Rust Digger: 20,000 new crates in 8 months](https://rust-digger.code-maven.com/news/restarting)

## Crate of the Week

This week's crate is [coffee\_break](https://github.com/radekvit/coffee_break), the premier crate for those who think Rust compile times are too fast.

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/crate-of-the-week/2704/1299) for the suggestion!

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

* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Braintree](https://github.com/juspay/hyperswitch/issues/4058)
* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Fiserv](https://github.com/juspay/hyperswitch/issues/4059)
* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Globepay](https://github.com/juspay/hyperswitch/issues/4060)
* [Rama — add Form support (IntroResponse + FromRequest)](https://github.com/plabayo/rama/issues/68)
* [Rama — Provide “and(matcher)” and “or(matcher)” methods to enum matchers](https://github.com/plabayo/rama/issues/93)
* [Rama — Provide support for boxed custom matchers in layer enums](https://github.com/plabayo/rama/issues/92)
* [Rama — add open-telemetry middleware and extended prometheus support](https://github.com/plabayo/rama/issues/23)
* [diesel - Review Diesel compile time error message](https://github.com/diesel-rs/diesel/discussions/3972)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [RustFest Zürich 2024](https://rustfest.ch/cfp/) | Closes 2024-03-31 | Zürich, Switzerland | Event date: 2024-06-19 - 2024-06-24
* [RustConf 2024](https://foundation.rust-lang.org/news/the-rustconf-2024-call-for-talk-proposals-is-open/) | Closes 2024-04-25 | Montreal, Canada | Event date: 2024-09-10
* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| Closes 2024-06-03 | Vienna, Austria & online | Event on 2024-10-10
* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/)| Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

444 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-03-19..2024-03-26

* [CFI: handle dyn with no principal](https://github.com/rust-lang/rust/pull/123003)
* [CFI: strip auto traits off Virtual calls](https://github.com/rust-lang/rust/pull/122879)
* [CFI: support `self_cell-like` recursion](https://github.com/rust-lang/rust/pull/122875)
* [CFI: support complex receivers](https://github.com/rust-lang/rust/pull/123005)
* [add a never type option to make diverging blocks `()`](https://github.com/rust-lang/rust/pull/122843)
* [add panic location to 'panicked while processing panic'](https://github.com/rust-lang/rust/pull/122930)
* [change `only_local` to a `enum` type](https://github.com/rust-lang/rust/pull/122695)
* [compiler: allow transmute of ZST arrays with generics](https://github.com/rust-lang/rust/pull/114009)
* [conditionally ignore fatal diagnostic in the SilentEmitter](https://github.com/rust-lang/rust/pull/122737)
* [delegation: fix ICE on `bound_vars` divergence](https://github.com/rust-lang/rust/pull/122881)
* [deref patterns: bare-bones feature gate and typechecking](https://github.com/rust-lang/rust/pull/122222)
* [implement macro-based `deref!()` syntax for deref patterns](https://github.com/rust-lang/rust/pull/122793)
* [do not use `?`-induced skewing of type inference in the compiler](https://github.com/rust-lang/rust/pull/122540)
* [don't ICE when encountering bound regions in generator interior type](https://github.com/rust-lang/rust/pull/122358)
* [don't emit an error about failing to produce a file with a specific name if user never gave an explicit name](https://github.com/rust-lang/rust/pull/122842)
* [don't trigger `unused_qualifications` on global paths](https://github.com/rust-lang/rust/pull/122435)
* [encode implied predicates for traits](https://github.com/rust-lang/rust/pull/122891)
* [ensure nested statics have a HIR node to prevent various queries from ICEing](https://github.com/rust-lang/rust/pull/122719)
* [ensure stack before parsing dot-or-call](https://github.com/rust-lang/rust/pull/122717)
* [experimental feature postfix match](https://github.com/rust-lang/rust/pull/121619)
* [extend format arg help for simple tuple index access expression](https://github.com/rust-lang/rust/pull/122556)
* [fix bad span for explicit lifetime suggestions](https://github.com/rust-lang/rust/pull/121587)
* [fix incorrect mutable suggestion information for binding in ref pattern](https://github.com/rust-lang/rust/pull/122677)
* [fix validation on substituted callee bodies in MIR inliner](https://github.com/rust-lang/rust/pull/122168)
* [fixed the `private-dependency` bug](https://github.com/rust-lang/rust/pull/122757)
* [gracefully handle `AnonConst` in `diagnostic_hir_wf_check()`](https://github.com/rust-lang/rust/pull/122370)
* [handle str literals written with `'` lexed as lifetime](https://github.com/rust-lang/rust/pull/122217)
* [ignore paths from expansion in `unused_qualifications`](https://github.com/rust-lang/rust/pull/122545)
* [implement `FusedIterator` for `gen` block](https://github.com/rust-lang/rust/pull/122829)
* [in `pretty_print_type()`, print async fn futures' paths instead of spans](https://github.com/rust-lang/rust/pull/122923)
* [inherit `RUSTC_BOOTSTRAP` when testing wasm](https://github.com/rust-lang/rust/pull/122795)
* [let codegen decide when to `mem::swap` with immediates](https://github.com/rust-lang/rust/pull/122582)
* [make `#[diagnostic::on_unimplemented]` format string parsing more robust](https://github.com/rust-lang/rust/pull/122402)
* [make `type_ascribe!` not a built-in](https://github.com/rust-lang/rust/pull/122806)
* [move `--sysroot` argument out of the argument file to fix miri issue](https://github.com/rust-lang/rust/pull/123030)
* [move more intrinsics to `rustc_intrinsic`](https://github.com/rust-lang/rust/pull/122037)
* [note that the caller chooses a type for type param](https://github.com/rust-lang/rust/pull/122195)
* [prevent opaque types being instantiated twice with different regions within the same function](https://github.com/rust-lang/rust/pull/116935)
* [print a backtrace in const eval if interrupted](https://github.com/rust-lang/rust/pull/111769)
* [provide structured suggestion for unconstrained generic constant](https://github.com/rust-lang/rust/pull/122802)
* [recursively evaluate the constants in everything that is 'mentioned'](https://github.com/rust-lang/rust/pull/122568)
* [replace closures with `_` when suggesting fully qualified path for method call](https://github.com/rust-lang/rust/pull/122799)
* [replace visibility test with reachability test in dead code detection](https://github.com/rust-lang/rust/pull/119552)
* [split an item bounds and an item's super predicates](https://github.com/rust-lang/rust/pull/121123)
* [split out `PredicatePolarity` from `ImplPolarity`](https://github.com/rust-lang/rust/pull/122839)
* [strip placeholders from hidden types before remapping generic parameter](https://github.com/rust-lang/rust/pull/122733)
* [suggest `RUST_MIN_STACK` workaround on overflow](https://github.com/rust-lang/rust/pull/122847)
* [suggest `_` for missing generic arguments in turbofish](https://github.com/rust-lang/rust/pull/122651)
* [track run-make-support lib in common inputs stamp](https://github.com/rust-lang/rust/pull/122962)
* [unbox and unwrap the contents of `StatementKind::Coverage`](https://github.com/rust-lang/rust/pull/122937)
* [uniquify `ReError` on input mode in canonicalizer](https://github.com/rust-lang/rust/pull/122907)
* [use MSVC-style escaping when passing a response/@ file to lld on windows](https://github.com/rust-lang/rust/pull/122596)
* [use `chunk_by` when building `ReverseSccGraph`](https://github.com/rust-lang/rust/pull/122970)
* [validate that we're only matching on unit `struct` for path pattern](https://github.com/rust-lang/rust/pull/122910)
* [miri: `phase_rustdoc`: add a heuristic to make us more certain that this is really rustdoc](https://github.com/rust-lang/miri/pull/3413)
* [miri: add support for missing SIMD float intrinsics](https://github.com/rust-lang/miri/pull/3396)
* [miri: allow `llvm.x86.sse2.pause` instrinsic to be called without SSE2](https://github.com/rust-lang/miri/pull/3393)
* [miri: many-seeds: propagate failure properly](https://github.com/rust-lang/miri/pull/3406)
* [miri: report retags as distinct from real memory accesses for data races](https://github.com/rust-lang/miri/pull/3385)
* [stop sorting via `DefId`s in region resolution](https://github.com/rust-lang/rust/pull/122824)
* [select `Vec::from_iter` impls in a const block to optimize compile times](https://github.com/rust-lang/rust/pull/122785)
* [stabilize `slice_split_at_unchecked`](https://github.com/rust-lang/rust/pull/120577)
* [import the 2021 prelude in the core crate](https://github.com/rust-lang/rust/pull/123042)
* [relax `SeqCst` ordering in standard library](https://github.com/rust-lang/rust/pull/122729)
* [`std::net`: adding acceptfilter feature for netbsd/freebsd](https://github.com/rust-lang/rust/pull/121881)
* [`std::thread`: refine `available_parallelism` for solaris/illumos](https://github.com/rust-lang/rust/pull/122992)
* [fix OOB pointer formed in `Vec::index`](https://github.com/rust-lang/rust/pull/122761)
* [add `NonNull::<[T]>::is_empty`](https://github.com/rust-lang/rust/pull/122800)
* [regex: add Cow guarantee to replace API](https://github.com/rust-lang/regex/pull/1178)
* [cargo alias: dont panic when resolving an empty alias](https://github.com/rust-lang/cargo/pull/13613)
* [cargo testsuite: Rename lints to `lints_table`](https://github.com/rust-lang/cargo/pull/13627)
* [cargo toml: Expose surce/spans for VirtualManifests](https://github.com/rust-lang/cargo/pull/13603)
* [cargo toml: Push diagnostic complexity on annotate-snippets](https://github.com/rust-lang/cargo/pull/13619)
* [cargo vendor: tiny not important refactors](https://github.com/rust-lang/cargo/pull/13610)
* [cargo: do not strip debuginfo by default for MSVC](https://github.com/rust-lang/cargo/pull/13630)
* [cargo: add a basic linting system](https://github.com/rust-lang/cargo/pull/13621)
* [cargo: report some dependency changes on any command](https://github.com/rust-lang/cargo/pull/13561)
* [cargo: fix debuginfo strip when using `--target`](https://github.com/rust-lang/cargo/pull/13618)
* [cargo: fix doc collision for lib/bin with a dash in the inferred name](https://github.com/rust-lang/cargo/pull/13640)
* [cargo: fix publish script due to crates.io CDN change](https://github.com/rust-lang/cargo/pull/13614)
* [cargo: do not borrow shell across registry query](https://github.com/rust-lang/cargo/pull/13647)
* [cargo: warn on -Zlints](https://github.com/rust-lang/cargo/pull/13632)
* [cargo: refactor: make lint names `snake_case`](https://github.com/rust-lang/cargo/pull/13635)
* [cargo: use `gitoxide` for `list_files_git`](https://github.com/rust-lang/cargo/pull/13592)
* [bindgen: allow custom derives on new-type alias](https://github.com/rust-lang/rust-bindgen/pull/2780)
* [bindgen: make `CargoCallbacks` more discoverable](https://github.com/rust-lang/rust-bindgen/pull/2778)
* [bindgen: move phantom fields to start of `struct` to avoid interfering with flexible array members](https://github.com/rust-lang/rust-bindgen/pull/2783)
* [clippy: `assigning_clones` should respect MSRV](https://github.com/rust-lang/rust-clippy/pull/12511)
* [clippy: `let_and_return`: avoid linting when code between last stmt and return expr is cfg'd out](https://github.com/rust-lang/rust-clippy/pull/12558)
* [clippy: `manual_assert`: do not add extra semicolon](https://github.com/rust-lang/rust-clippy/pull/12536)
* [clippy: `useless_asref`: do not lint `.as_ref().map(Arc::clone)`](https://github.com/rust-lang/rust-clippy/pull/12535)
* [clippy: add `missing_transmute_annotations` lint](https://github.com/rust-lang/rust-clippy/pull/12239)
* [clippy: add necessary parentheses to `manual_unwrap_or_default` lint output](https://github.com/rust-lang/rust-clippy/pull/12532)
* [clippy: change applicability of `assigning_clones` to `Unspecified`](https://github.com/rust-lang/rust-clippy/pull/12554)
* [clippy: correct version for `incompatible_msrv`](https://github.com/rust-lang/rust-clippy/pull/12522)
* [clippy: disable `cast_lossless` when casting to u128 from any (u)int type](https://github.com/rust-lang/rust-clippy/pull/12496)
* [clippy: do not warn on `.map(_::clone)` for `Arc`, `Rc`, and their weak variants](https://github.com/rust-lang/rust-clippy/pull/12529)
* [clippy: don't lint `mixed_attributes_style` when mixing docs and other attrs](https://github.com/rust-lang/rust-clippy/pull/12486)
* [clippy: enable `unused_qualifications` lint](https://github.com/rust-lang/rust-clippy/pull/12507)
* [clippy: fix infinite loop in `cast_sign_loss` when peeling unwrap method calls](https://github.com/rust-lang/rust-clippy/pull/12508)
* [clippy: fix `suspicious_else_formatting` false positive when else is included …](https://github.com/rust-lang/rust-clippy/pull/12549)
* [clippy: make `assigning_clones` MSRV check more precise](https://github.com/rust-lang/rust-clippy/pull/12516)
* [clippy: remove `unwrap` from `match_trait_method`](https://github.com/rust-lang/rust-clippy/pull/12540)
* [rust-analyzer: add fuel to match checking](https://github.com/rust-lang/rust-analyzer/pull/16879)
* [rust-analyzer: limit `struct` hover display nums](https://github.com/rust-lang/rust-analyzer/pull/16906)
* [rust-analyzer: don't assert paths being utf8 when filtering them in the watcher](https://github.com/rust-lang/rust-analyzer/pull/16918)
* [rust-analyzer: fix project discovery not checking whether the `Cargo.toml` actually exists](https://github.com/rust-lang/rust-analyzer/pull/16899)
* [rust-analyzer: fix projects depending on `rustc_private` hanging](https://github.com/rust-lang/rust-analyzer/pull/16911)
* [rust-analyzer: goto implementation to impls inside blocks](https://github.com/rust-lang/rust-analyzer/pull/16812)
* [rust-analyzer: handle `self::super` when lowering UseTree](https://github.com/rust-lang/rust-analyzer/pull/16919)
* [rust-analyzer: improve error recovery for match arms](https://github.com/rust-lang/rust-analyzer/pull/16885)
* [rust-analyzer: keep the span for `Attr::Literal` around](https://github.com/rust-lang/rust-analyzer/pull/16909)
* [rust-analyzer: prevent stack overflow in recursive const types](https://github.com/rust-lang/rust-analyzer/pull/16915)
* [rust-analyzer: rename `func_like` to `FuncLike`](https://github.com/rust-lang/rust-analyzer/pull/16927)
* [rust-analyzer: some file watching related vfs fixes](https://github.com/rust-lang/rust-analyzer/pull/16913)
* [rust-analyzer: handle panicking like rustc CTFE does](https://github.com/rust-lang/rust-analyzer/pull/16935)
* [rust-analyzer: have Derive Attribute share a token tree with it's proc macros](https://github.com/rust-lang/rust-analyzer/pull/16835)
* [rust-analyzer: resolve whether `$pat` is `$pat_param` or not via 🌟hygiene🌟](https://github.com/rust-lang/rust-analyzer/pull/16895)

### Rust Compiler Performance Triage

An overall fairly quiet week with the unfortunate one exception of large instruction count and binary size regressions caused by changes in const evaluation. This was largely balanced out (at least in instruction count) by a group of small improvements, but the compiler did end up 0.2% slower on average across 97 benchmarks.

Triage done by **@rylev**.
Revision range: [21d94a3..73476d](https://perf.rust-lang.org/?start=21d94a3d2c63cacf8eaf9d0ca770c0b450c558d4&end=73476d49904751f8d90ce904e16dfbc278083d2c&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.0%  | [0.2%, 3.2%]   | 56    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.1%, 1.9%]   | 38    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-1.5%, -0.2%] | 41    |
| Improvements ✅ <br /> (secondary) | -1.2% | [-5.2%, -0.4%] | 13    |
| All ❌✅ (primary)                 | 0.2%  | [-1.5%, 3.2%]  | 97    |


4 Regressions, 6 Improvements, 2 Mixed; 4 of them in rollups
63 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/a71e92ee7d976f7cb9bf51cc08b5680340ccd5d0/triage/2024-03-26.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [unsafe attributes](https://github.com/rust-lang/rfcs/pull/3325)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Eagerly instantiate closure/coroutine-like bounds with placeholders to deal with binders correctly](https://github.com/rust-lang/rust/pull/122267)
* [disposition: merge] [Tracking Issue for `Literal::byte_character`](https://github.com/rust-lang/rust/issues/115268)
* [disposition: merge] [Tracking Issue for `proc_macro_c_str_literals`](https://github.com/rust-lang/rust/issues/119750)
* [disposition: merge] [Tracking Issue for const_caller_location](https://github.com/rust-lang/rust/issues/76156)
* [disposition: merge] [Make inductive cycles always ambiguous](https://github.com/rust-lang/rust/pull/122791)
* [disposition: merge] [Add `REDUNDANT_LIFETIMES` lint to detect lifetimes which are semantically redundant](https://github.com/rust-lang/rust/pull/118391)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Move the Crates.io Team under the Dev Tools team](https://github.com/rust-lang/rfcs/pull/3595)
* [new] [RFC: Add realign_stack attribute to rustc](https://github.com/rust-lang/rfcs/pull/3594)
* [new, disposition: merge] [RFC: Reserve unprefixed guarded string literals in Edition 2024](https://github.com/rust-lang/rfcs/pull/3593)

## Upcoming Events

Rusty Events between 2024-03-27 - 2024-04-24 🦀

### Virtual

* 2024-03-28 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457904/)
* 2024-04-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/mrnrktygcgbdb/)
* 2024-04-03 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 4 - Error Handling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299507234/)
* 2024-04-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047892/)
* 2024-04-04 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368794/)
* 2024-04-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**BlueR: a Rust Based Tool for Robust and Safe Bluetooth Control**](https://www.meetup.com/dallasrust/events/298341660/)
* 2024-04-11 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477689/)
* 2024-04-11 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945256/)
* 2024-04-16 | Virtual (Washinigton, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346486/)
* 2024-04-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-04-18 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368799/)

### Africa

* 2024-04-05 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-03-30 | New Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Rust Delhi Meetup #6**](https://www.meetup.com/rustdelhi/events/299771772/)

### Europe

* 2024-03-27 & 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024 - Conference (Mar 26-28)**](https://www.rustnationuk.com/)
* 2024-03-28 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell**](https://www.meetup.com/rust-berlin/events/299288961/)
* 2024-04-10 | Manchester, UK | [Manchseter Rust Meetup](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester April 2024**](https://www.meetup.com/rust-manchester/events/299887934/)
* 2024-04-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Rust Meetup Reboot 3**](https://www.meetup.com/cambridge-rust-meetup/events/299730322/)
* 2024-04-10 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester April 2024**](https://www.meetup.com/rust-manchester/events/299887934/)
* 2024-04-10 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/299488225/)
* 2024-04-11 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #2 : Présentations**](https://www.meetup.com/bordeaux-rust/events/299628716/)
* 2024-04-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/299694473/)
* 2024-04-15 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/)
    * [**Rust Meetup 2024/04: Building cargo projects with NIX**](https://www.meetup.com/zagreb-rust-meetup/events/299905015/)
* 2024-04-16 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #5**](https://www.meetup.com/bratislava-rust-meetup-group/events/299302952/)
* 2024-04-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**winnow/nom**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/300024630/)
* 2024-04-16 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-04-17 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/)
    * [**Lær Rust med Conways Game of Life**](https://www.meetup.com/bergen-html-css-meetup-group/events/300031586/)

### North America

* 2024-03-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299220136/)
* 2024-03-27 | Hawthorne (Los Angeles), CA, US | [Freeform](https://freeform.co/)
    * [**Rust in the Physical World 🦀 Tech Talk Event at Freeform**](https://freeformxrust.rsvpify.com/)
* 2024-03-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch, Mar 31**](https://www.meetup.com/bostonrust/events/299262047/)
* 2024-04-04 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803577/)
* 2024-04-04 | Portland, OR, US | [PDXRust Meetup](https://www.meetup.com/pdxrust/)
    * [**Hack Night and First Post-Pandemic Meetup Restart**](https://www.meetup.com/pdxrust/events/300043905/)
* 2024-04-10 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Reproducible Developer Environments with Flox**](https://www.meetup.com/boulder-rust-meetup/events/300019409/)
* 2024-04-11 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509326/)
* 2024-04-11 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300019993/)
* 2024-04-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186907/)
* 2024-04-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group: Meet Servo and Robius Open Source Projects**](https://www.meetup.com/seattle-rust-user-group/events/299908469/)
* 2024-04-18 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803586/)
* 2024-04-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299960315/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1bpg8b8/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> "Top contributor" is not a place of glory, it *should* go to a bot because people should work at a sustainable pace and prioritize touching grass every once in a while. If a person ever works harder than bors, that's a problem!

– [Carol (Nichols || Goulding) on rust-internals](https://internals.rust-lang.org/t/exclude-bots-from-contributors/20516/5)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1552) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1bpicom/this_week_in_rust_540/)</small>
