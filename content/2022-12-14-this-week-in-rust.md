Title: This Week in Rust 473
Number: 473
Date: 2022-12-14
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
* [Launching the 2022 State of Rust Survey](https://blog.rust-lang.org/2022/12/05/survey-launch.html)

### Foundation
* [Welcoming Shopify as Our Inaugural Gold Member](https://foundation.rust-lang.org/news/welcoming-shopify-as-our-inaugural-gold-member/)

### Newsletters
* [This Month in Rust OSDev: November 2022](https://rust-osdev.com/this-month/2022-11/)

### Project/Tooling Updates
* [rust-analyzer changelog #159](https://rust-analyzer.github.io/thisweek/2022/12/12/changelog-159.html)
* [Making Dioxus (almost) as fast as SolidJS](https://dioxuslabs.com/blog/templates-diffing/)
* [Announcing Ksunami v0.1.x](https://ivandemarino.me/2022/12/announcing-ksunami)
* [Fornjot (code-first CAD in Rust) - Weekly Release](https://www.fornjot.app/blog/weekly-release/2022-w50/)
* [Youki v0.0.4 Release](https://dev.to/yjdoc2/announcing-youki-040-ocn)

### Observations/Thoughts
* [Catch 22! Rust in Review](https://llogiq.github.io/2022/12/11/catch22.html)
* [A Look at dyn* Code Generation](https://blog.theincredibleholk.org/blog/2022/12/12/dyn-star-codegen/)
* [Shopify Embraces Rust for Systems Programming](https://shopify.engineering/shopify-rust-systems-programming)
* [Rust in 2023](https://www.ncameron.org/blog/rust-in-2023/)
* [Category Theory with Rust (pt2) - GATs example](https://www.kurtlawrence.info/blog/category-theory-with-rust-pt2)
* [Rust: state of GUI, from the perspective of KAS](https://kas-gui.github.io/blog/state-of-GUI-2022.html)
* [A call for blogs about Rust GUI in 2023](https://poignardazur.github.io/2022/12/11/rust-gui-blog-posts-2023/)
* [video] [Next Generation i18n with Rust Using ICU4X](https://www.youtube.com/watch?v=ZzsbN7HBd7E)
* [video] [Supercharging Zero-Copy Deserialization](https://www.youtube.com/watch?v=DM2DI3ZI_BQ)
* [video] [Let's write a TCP Port Scanner in Rust](https://www.youtube.com/watch?v=2nslVz2ju9I)
* [video] [Rust Is Easy (The COMPILER teaches you!)](https://www.youtube.com/watch?v=CJtvnepMVAU)
* [audio] [Presser with Gray Olson](https://rustacean-station.org/episode/gray-olson/)
* [audio] [Kernel Density Estimation with Seaton Ullberg](https://rustacean-station.org/episode/seaton-ullberg/)

### Rust Walkthroughs
* [Clean Code with Rust & Axum](https://www.propelauth.com/post/clean-code-with-rust-and-axum)
* [Data-driven performance optimization with Rust and Miri](https://medium.com/source-and-buggy/data-driven-performance-optimization-with-rust-and-miri-70cb6dde0d35)
* [Embedded Rust & Embassy: Analog Sensing with ADCs](https://apollolabsblog.hashnode.dev/embedded-rust-embassy-analog-sensing-with-adcs)
* [Sorting with SIMD](https://tweedegolf.nl/en/blog/79/sorting-with-simd)
* [Sending Emails from the Edge with Rust](https://mainmatter.com/blog/2022/12/09/sending-emails-from-the-edge-with-rust/)
* [Composing an observable Rust application](https://blog.logrocket.com/composing-underpinnings-observable-rust-application/)

### Miscellaneous

* [video] [Meetup: Rust in critical infrastructure, Amsterdam 30 Nov 2022](https://www.youtube.com/watch?v=DSdXAWCdhmk&list=PL8Q1w7Ff68DDmxLks3_nkZU8xAc6CPe1l)

## Crate of the Week

This week's crate is [lazy_format](https://docs.rs/lazy_format), a lazy version of format! for more efficient composed string formatting operations.

Thanks to [Nathan West](https://users.rust-lang.org/t/crate-of-the-week/2704/1133) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
* [impl-tools - more testing](https://crates.io/crates/impl-tools) [more context](https://users.rust-lang.org/t/twir-call-for-participation/4821/481)
* [time - more testing](https://github.com/time-rs/time/issues/428) [more context](https://mastodon.social/@jhpratt/109498400905572278)
* [Ockam - Refactor request_controller function to accept an optional Identity](https://github.com/build-trust/ockam/issues/3903)
* [Ockam - Add optional --identity argument to secure-channel create and modify implementation on ockam_api](https://github.com/build-trust/ockam/issues/3906)


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

320 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-12-05..2022-12-12

* [add LLVM KCFI support to the Rust compiler](https://github.com/rust-lang/rust/pull/105109)
* [add StableOrd trait](https://github.com/rust-lang/rust/pull/105175)
* [add help for `#![feature(impl_trait_in_fn_trait_return)]`](https://github.com/rust-lang/rust/pull/105408)
* [compute generator sizes with `-Zprint_type_sizes`](https://github.com/rust-lang/rust/pull/104019)
* [consider `parent_count` for const param defaults](https://github.com/rust-lang/rust/pull/105410)
* [detect long types in E0308 and write them to disk](https://github.com/rust-lang/rust/pull/104922)
* [detect spurious ; before assoc fn body](https://github.com/rust-lang/rust/pull/105369)
* [disable top down MIR inlining](https://github.com/rust-lang/rust/pull/105119)
* [don't ICE in ExprUseVisitor on FRU for non-existent struct](https://github.com/rust-lang/rust/pull/105267)
* [don't call `diagnostic_hir_wf_check` query if we have infer variables](https://github.com/rust-lang/rust/pull/105283)
* [don't internalize `__llvm_profile_counter_bias`](https://github.com/rust-lang/rust/pull/102900)
* [enable ThinLTO for rustc on `x86_64-apple-darwin`](https://github.com/rust-lang/rust/pull/103647)
* [enable ThinLTO for rustc on x64 msvc](https://github.com/rust-lang/rust/pull/103591)
* [enable profiler in dist-powerpc64le-linux](https://github.com/rust-lang/rust/pull/105389)
* [fix build on powerpc-unknown-freebsd](https://github.com/rust-lang/rust/pull/104572)
* [fix invalid codegen during debuginfo lowering](https://github.com/rust-lang/rust/pull/105482)
* [fix lint perf regressions](https://github.com/rust-lang/rust/pull/105485)
* [group some fields in a common struct so we only pass one reference instead of three](https://github.com/rust-lang/rust/pull/105357)
* [interpret: clobber return place when calling function](https://github.com/rust-lang/rust/pull/105207)
* [llvm-wrapper: adapt for LLVM API changes](https://github.com/rust-lang/rust/pull/105555)
* [llvm-wrapper: adapt for an LLVM API change](https://github.com/rust-lang/rust/pull/105298)
* [make `VecDeque::from_iter` `O(1)` from `vec(_deque)::IntoIter`](https://github.com/rust-lang/rust/pull/105453)
* [make integer-to-integer `From` impls `#[inline(always)]`](https://github.com/rust-lang/rust/pull/105271)
* [make pointer `sub` and `wrapping_sub` methods `#[inline(always)]`](https://github.com/rust-lang/rust/pull/105508)
* [make some trivial functions `#[inline(always)]`](https://github.com/rust-lang/rust/pull/105262)
* [mangle "main" as `"__main_void"` on wasm32-wasi](https://github.com/rust-lang/rust/pull/105468)
* [on E0195 point at where clause lifetime bounds](https://github.com/rust-lang/rust/pull/105005)
* [point at GAT `where` clause when an obligation is unsatisfied](https://github.com/rust-lang/rust/pull/105324)
* [point at LHS on binop type err if relevant](https://github.com/rust-lang/rust/pull/105192)
* [point at args in associated const fn pointers](https://github.com/rust-lang/rust/pull/105349)
* [re-enable removal of ZST writes to unions](https://github.com/rust-lang/rust/pull/105229)
* [recurse into nested impl-trait when computing variance](https://github.com/rust-lang/rust/pull/105254)
* [remove `token::Lit` from `ast::MetaItemLit`](https://github.com/rust-lang/rust/pull/105160)
* [remove {`Early`, `Late`}`LintPassObjects`](https://github.com/rust-lang/rust/pull/105291)
* [shrink `rustc_parse_format::Piece`](https://github.com/rust-lang/rust/pull/105363)
* [suggest parenthesis around `ExprWithBlock BinOp ExprWithBlock`](https://github.com/rust-lang/rust/pull/105223)
* [suggest removing struct field from destructive binding only in shorthand scenario](https://github.com/rust-lang/rust/pull/105174)
* [tweak "the following other types implement trait"](https://github.com/rust-lang/rust/pull/105338)
* [tweak `rustc_must_implement_one_of` diagnostic output](https://github.com/rust-lang/rust/pull/105506)
* [miri: allow configurable and platform-specific page sizes](https://github.com/rust-lang/miri/pull/2721)
* [miri: make unix path handling on Windows hosts (and vice versa) preserve absoluteness](https://github.com/rust-lang/miri/pull/2725)
* [cargo: allow Check targets needed for optional doc-scraping to fail without killing the build](https://github.com/rust-lang/cargo/pull/11450)
* [rustdoc: only hide lines starting with `#` in rust code blocks](https://github.com/rust-lang/rust/pull/105539)
* [rustdoc: prevent auto/blanket impl retrieval if there were compiler errors](https://github.com/rust-lang/rust/pull/105457)
* [clippy: `arithmetic-side-effects`: consider user-provided pairs](https://github.com/rust-lang/rust-clippy/pull/9840)
* [clippy: `uninlined_format_args: `ignore assert! and `debug_assert!` before 2021 edition](https://github.com/rust-lang/rust-clippy/pull/10055)
* [clippy: add 1.58 MSRV for `collapsible_str_replace`](https://github.com/rust-lang/rust-clippy/pull/10047)
* [clippy: add `suppress_restriction_lint_in_const` config](https://github.com/rust-lang/rust-clippy/pull/9920)
* [clippy: add lint `almost_complete_digit_range`](https://github.com/rust-lang/rust-clippy/pull/10043)
* [clippy: add semicolon-outside/inside-block lints](https://github.com/rust-lang/rust-clippy/pull/9826)
* [clippy: don't suggest keeping borrows in `identity_op`](https://github.com/rust-lang/rust-clippy/pull/10004)
* [clippy: fix `zero_ptr` suggestion for `no_std` crates](https://github.com/rust-lang/rust-clippy/pull/10023)
* [rust-analyzer: compute data layout of types](https://github.com/rust-lang/rust-analyzer/pull/13490)
* [rust-analyzer: add "Remove redundant parentheses" assist](https://github.com/rust-lang/rust-analyzer/pull/13733)
* [rust-analyzer: add fallback case in generated `PartialEq` impl](https://github.com/rust-lang/rust-analyzer/pull/13732)
* [rust-analyzer: allow unwrap block in let initializers](https://github.com/rust-lang/rust-analyzer/pull/13726)
* [rust-analyzer: breaking snippets on typed incomplete suggestions](https://github.com/rust-lang/rust-analyzer/pull/13715)
* [rust-analyzer: don't show duplicated adjustment hints for blocks, ifs and matches](https://github.com/rust-lang/rust-analyzer/pull/13749)
* [rust-analyzer: fix parsing of `_ = x` in closure body](https://github.com/rust-lang/rust-analyzer/pull/13762)
* [rust-analyzer: make `make_body` respect comments in `extract_function`](https://github.com/rust-lang/rust-analyzer/pull/13746)
* [rust-analyzer: normalize projection after discarding free `BoundVar`s in RPIT](https://github.com/rust-lang/rust-analyzer/pull/13750)
* [rust-analyzer: only shift `BoundVar`s that come from outside lowering context](https://github.com/rust-lang/rust-analyzer/pull/13742)
* [rust-analyzer: show type info on hover of enum variant fields](https://github.com/rust-lang/rust-analyzer/pull/13745)

### Rust Compiler Performance Triage

Fairly quiet week with the only excitement coming from a nice improvement implemented by @nnethercote which fixed a pesky performance regressions in the linting system. This produced a 0.6% performance improvement across a large amount of the real world crates we test against. 

Triage done by **@rylev**.
Revision range: [9db224fc..109cccbe](https://perf.rust-lang.org/?start=9db224fc908059986c179fc6ec433944e9cfce50&end=109cccbe4f345c0f0785ce860788580c3e2a29f5&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.2%  | [0.2%, 0.2%]   | 3     |
| Regressions ❌ <br /> (secondary)  | 1.7%  | [0.3%, 3.3%]   | 11    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-2.2%, -0.2%] | 129   |
| Improvements ✅ <br /> (secondary) | -1.2% | [-4.4%, -0.1%] | 97    |
| All ❌✅ (primary)                 | -0.7% | [-2.2%, 0.2%]  | 132   |


2 Regressions, 2 Improvements, 4 Mixed; 3 of them in rollups
41 artifact comparisons made in total

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-12-13.md) for details.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Support upcasting of `dyn Trait` values](https://github.com/rust-lang/rfcs/pull/3324)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Add `core::mem::offset_of!` RFC](https://github.com/rust-lang/rfcs/pull/3308)
* [disposition: close] [Cargo allow running binaries from development or build dependencies](https://github.com/rust-lang/rfcs/pull/3168)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Stabilize `#![feature(target_feature_11)]`](https://github.com/rust-lang/rust/pull/99767)
* [disposition: merge] [Stabilize default_alloc_error_handler](https://github.com/rust-lang/rust/pull/102318)
* [disposition: merge] [Stop promoting all the things](https://github.com/rust-lang/rust/pull/105085)
* [disposition: merge] [`Arc::ptr_eq` does not always return "true if the two Arcs point to the same allocation" as documented](https://github.com/rust-lang/rust/issues/103763)
* [disposition: merge] [Don't normalize in AstConv](https://github.com/rust-lang/rust/pull/101947)
* [disposition: merge] [Encode spans relative to the enclosing item -- enable by default](https://github.com/rust-lang/rust/pull/84762)
* [disposition: merge] [impl DispatchFromDyn for Cell and UnsafeCell](https://github.com/rust-lang/rust/pull/97373)
* [disposition: merge] [Tracking issue for the "efiapi" calling convention](https://github.com/rust-lang/rust/issues/65815)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Start working on a Rust specification](https://github.com/rust-lang/rfcs/pull/3355)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2022-12-14 - 2023-01-11 🦀

### Virtual

* 2022-12-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcqbsb/)
* 2022-12-24 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 28th Edition**](https://www.meetup.com/rust-linz/events/290196122/)
* 2022-12-14 | Virtual (México City, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust y Arduino**](https://www.meetup.com/rust-mx/events/289973784/)
* 2022-12-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcqbtb/)
* 2022-12-20 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/12/20/rust-hack-and-learn.html)
* 2022-12-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsydcqbbc/)
* 2022-12-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Show & Tell: Tableturf**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcqbcc/)
* 2022-12-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcqbkc/)
* 2023-01-03 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/mbmxvsyfccbfb/)
* 2023-01-03 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289581074/)
* 2023-01-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfccbfb/)
* 2023-01-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfccbgb/)
* 2023-01-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfccbgb/)
* 2023-01-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Part 2: Exploring USB with Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/290122605/)


### Asia

* 2022-12-29 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**December Edition - xtask, macros and low level features**](https://www.meetup.com/rust-tlv/events/290156141/)

### Europe

* 2022-12-14 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust Advent of Code hackathon**](https://www.meetup.com/rust-trondheim/events/290100114/)
* 2022-12-15 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsydcqbvb/)

### North America

* 2022-12-14 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/290161310/)
* 2022-12-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcqbbc/)
* 2022-12-27 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**Atx Rustaceans Meetup**](https://www.meetup.com/atx-rustaceans/events/290064553/)
* 2023-01-05 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Interesting Title and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsydcqbdc/)


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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/ymepy8/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> ... you can lead a horse to git but you cannot make it commit.

– [/u/kibwen on /r/rust](https://old.reddit.com/r/rust/comments/zjx2xx/blog_post_rust_in_2023/izz4g8d/)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1341) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/zmhhvi/this_week_in_rust_473/)</small>
