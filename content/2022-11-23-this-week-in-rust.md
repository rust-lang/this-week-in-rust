Title: This Week in Rust 470
Number: 470
Date: 2022-11-23
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
* [Async fn in trait MVP comes to nightly](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html)

### Foundation
* [Community Grantee Spotlight: Sebastian Thiel](https://foundation.rust-lang.org/news/community-grantee-spotlight-sebastian-thiel/)

### Project/Tooling Updates
* [rust-analyzer changelog #156](https://rust-analyzer.github.io/thisweek/2022/11/21/changelog-156.html)
* [IntelliJ Rust Changelog #183](https://intellij-rust.github.io/2022/11/21/changelog-183.html)
* [Fornjot (code-first CAD in Rust) - Weekly Release](https://www.fornjot.app/blog/weekly-release/2022-w47/)
* [This Week in Fyrox #3](https://fyrox.rs/blog/post/twif3/)
* [futures-concurrency Release v7.0.0](https://github.com/yoshuawuyts/futures-concurrency/releases/tag/v7.0.0)
* [Rust Search Extension v1.9.0 has been released](https://rust.extension.sh/changelog/#v1-9-0-2022-11-20)
* [Chinese] [RustSBI 0.3.0 Has Released](https://rustcc.cn/article?id=18318ed2-d6b3-461c-a599-fe140ef41713)
* [Chinese] [Video: Technologies and Applications in RustSBI 0.3.0](https://www.bilibili.com/video/BV1U841187xH)

### Observations/Thoughts
* [A Better Way to Borrow in Rust: Stack Tokens](https://lucumr.pocoo.org/2022/11/23/stack-tokens/)
* [Category Theory with Rust (pt1)](https://www.kurtlawrence.info/blog/category-theory-with-rust-pt1)
* [If a Tree Falls in a Forest, Does It Overflow the Stack?](https://matklad.github.io/2022/11/18/if-a-tree-falls-in-a-forest-does-it-overflow-the-stack.html)
* [Safely writing code that isn't thread-safe](http://cliffle.com/blog/not-thread-safe/)
* [Embedded Rust & Embassy: GPIO Button Controlled Blinking](https://apollolabsblog.hashnode.dev/embedded-rust-embassy-gpio-button-controlled-blinking)
* [video] [Panel: Rust in reality - EuroRust 2022](https://www.youtube.com/watch?v=ubXvKRbULuo)

### Rust Walkthroughs
* [Calling Rust from iOS](https://burgers.io/calling-rust-from-ios)
* [Rust, Lambda, and DynamoDB](https://betterprogramming.pub/rust-lambda-and-dynamodb-bea841d47cca)
* [Render Pipelines in wgpu and Rust](https://whoisryosuke.com/blog/2022/render-pipelines-in-wgpu-and-rust/)
* [(Re)writing an interpreter in Rust](https://www.dannyvankooten.com/blog/2022-12-rewriting-interpreter-rust/)

### Miscellaneous
* [The carcinization of Go programs](https://xeiaso.net/blog/carcinization-golang)
* [Flux: Refinement Types for Rust](https://liquid-rust.github.io/2022/11/14/introducing-flux/)
* [video] [Rust 🤝 WebAssembly with Alex Crichton](https://youtu.be/Uk1_3gH1w58)
* [video] [Getting Started with Rust: Understanding Rust Compile Errors](https://www.youtube.com/watch?v=hgZQJys2zpY)
* [video] [Can you use Character Controllers for non-platformer games?](https://www.youtube.com/watch?v=aKAwlCFaOms)

## Crate of the Week

This week's crate is [graph](https://crates.io/crates/graph), a collection of high-performance graph algorithms.

Thanks to [Knutwalker](https://users.rust-lang.org/t/crate-of-the-week/2704/1125) for the (partial self-) suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*There were no calls for participation submitted this week. If you would like to submit, please check the [guidelines](https://users.rust-lang.org/t/twir-call-for-participation/4821).*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

388 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-11-14..2022-11-21

* [deduce closure signature from a type alias `impl Trait`'s supertraits](https://github.com/rust-lang/rust/pull/104258)
* [pass 128-bit C-style enum enumerator values to LLVM](https://github.com/rust-lang/rust/pull/102717)
* [detect incorrect chaining of if and if let conditions and recover](https://github.com/rust-lang/rust/pull/103405)
* [diagnostics `icu4x` based list formatting](https://github.com/rust-lang/rust/pull/104047)
* [diagnostics: only show one suggestion for method → assoc fn](https://github.com/rust-lang/rust/pull/104580)
* [fix inconsistent rounding of 0.5 when formatted to 0 decimal places](https://github.com/rust-lang/rust/pull/102935)
* [fix non-associativity of `Instant` math on `aarch64-apple-darwin` targets](https://github.com/rust-lang/rust/pull/103594)
* [improve generating Custom entry (as in `main()`) function](https://github.com/rust-lang/rust/pull/104001)
* [improve spans for RPITIT object-safety errors](https://github.com/rust-lang/rust/pull/104593)
* [interpret: support for per-byte provenance](https://github.com/rust-lang/rust/pull/104054)
* [llvm-wrapper adapt for LLVM API change](https://github.com/rust-lang/rust/pull/104413)
* [nll: correctly deal with bivariance](https://github.com/rust-lang/rust/pull/104411)
* [only do parser recovery on retried macro matching](https://github.com/rust-lang/rust/pull/104335)
* [record `LocalDefId` in HIR nodes instead of a side table](https://github.com/rust-lang/rust/pull/104170)
* [shift no characters when using raw string literals](https://github.com/rust-lang/rust/pull/104193)
* [slightly improve error message for invalid identifier](https://github.com/rust-lang/rust/pull/104309)
* [support `#[track_caller]` on async fns](https://github.com/rust-lang/rust/pull/104219)
* [miri: make `align_offset` always work on no-provenance pointers](https://github.com/rust-lang/miri/pull/2683)
* [miri: stack borrows: weak protectors](https://github.com/rust-lang/miri/pull/2684)
* [add new MIR constant propagation based on dataflow analysis](https://github.com/rust-lang/rust/pull/101168)
* [merge basic blocks where possible when generating LLVM IR](https://github.com/rust-lang/rust/pull/103138)
* [minimal implementation of implicit deref patterns for Strings](https://github.com/rust-lang/rust/pull/98914)
* [shrink `ast::Expr` harder](https://github.com/rust-lang/rust/pull/101562)
* [use `token::Lit` in `ast::ExprKind::Lit`](https://github.com/rust-lang/rust/pull/102944)
* [perform simple scalar replacement of aggregates (SROA) MIR opt](https://github.com/rust-lang/rust/pull/102570)
* [make `pointer::byte_offset_from` more generic](https://github.com/rust-lang/rust/pull/103489)
* [fix `mod_inv` termination for the last iteration](https://github.com/rust-lang/rust/pull/103378)
* [improve accuracy of asinh and acosh](https://github.com/rust-lang/rust/pull/104553)
* [stabilize const char convert](https://github.com/rust-lang/rust/pull/102470)
* [`VecDeque::resize` should re-use the buffer in the passed-in element](https://github.com/rust-lang/rust/pull/104435)
* [`unchecked_`{`shl`, `shr`} should use `u32` as the RHS](https://github.com/rust-lang/rust/pull/103456)
* [constify `is_aligned` via `align_offset`](https://github.com/rust-lang/rust/pull/102795)
* [`x86_64` SSE2 fast-path for `str.contains(&str)` and short needles](https://github.com/rust-lang/rust/pull/103779)
* [remove HRTB from `slice::is_sorted_by`(`_key`)](https://github.com/rust-lang/rust/pull/102977)
* [portable-simd: scatter/gather for pointers](https://github.com/rust-lang/portable-simd/pull/315)
* [stdarch: fix undefined behavior in `movemask_epi8`](https://github.com/rust-lang/stdarch/pull/1354)
* [compiler-builtins: skip assembly implementations on the UEFI targets](https://github.com/rust-lang/compiler-builtins/pull/504)
* [compiler-builtins: use a stub stdlib.h when compiling for UEFI targets](https://github.com/rust-lang/compiler-builtins/pull/506)
* [cargo: fix cargo install --index when used with registry.default](https://github.com/rust-lang/cargo/pull/11302)
* [cargo: alternative registry authentication support](https://github.com/rust-lang/cargo/pull/10592) (RFC [#3139](https://rust-lang.github.io/rfcs/3139-cargo-alternative-registry-auth.html))
* [cargo: improve error message for cargo add/remove](https://github.com/rust-lang/cargo/pull/11375)
* [rustdoc: fix missing minification for static files](https://github.com/rust-lang/rust/pull/104404)
* [rustdoc: resolve doc links in external traits having local impls](https://github.com/rust-lang/rust/pull/104364)
* [clippy: `never_loop`: don't emit AlwaysBreaks if it targets a block](https://github.com/rust-lang/rust-clippy/pull/9858)
* [clippy: add new lint `misnamed-getters`](https://github.com/rust-lang/rust-clippy/pull/9770)
* [clippy: allow manual swap in const fn](https://github.com/rust-lang/rust-clippy/pull/9871)
* [clippy: allow return types for closures with lifetime binder](https://github.com/rust-lang/rust-clippy/pull/9849)
* [clippy: `arithmetic_side_effects`: detect overflowing associated constants of integers](https://github.com/rust-lang/rust-clippy/pull/9592)
* [clippy: extend `needless_borrowed_reference` to structs and tuples, ignore `_`](https://github.com/rust-lang/rust-clippy/pull/9855)
* [clippy: lint unchecked subtraction of a 'Duration' from an 'Instant'](https://github.com/rust-lang/rust-clippy/pull/9570)
* [clippy: fix `#[allow]` for `module_name_repetitions` & `single_component_path_imports`](https://github.com/rust-lang/rust-clippy/pull/9879)
* [clippy: preserve `ref` on `infallible_destructuring_match` suggestion](https://github.com/rust-lang/rust-clippy/pull/9850)
* [rust-analyzer: allow viewing the full compiler diagnostic in a readonly textview](https://github.com/rust-lang/rust-analyzer/pull/13633)
* [rust-analyzer: make "Remove dbg!()" assist work on selections](https://github.com/rust-lang/rust-analyzer/pull/13629)
* [rust-analyzer: remove `item_const` which had default value when implement missing members](https://github.com/rust-lang/rust-analyzer/pull/13642)
* [rust-analyzer: format expression parsing edge-cases](https://github.com/rust-lang/rust-analyzer/pull/13641)
* [rust-analyzer: include generic parameter in GAT completions](https://github.com/rust-lang/rust-analyzer/pull/13622)
* [rust-analyzer: resolve inference variable before applying adjustments](https://github.com/rust-lang/rust-analyzer/pull/13624)
* [rust-analyzer: strip comments and attributes off of all trait item completions](https://github.com/rust-lang/rust-analyzer/pull/13623)
* [rust-analyzer: support multiple targets for checkOnSave (in conjunction with cargo 1.64.0+)](https://github.com/rust-lang/rust-analyzer/pull/13290)

### Rust Compiler Performance Triage

A fairly quiet week with regressions unfortunately slightly outweighing improvements. There was not any particular change of much note. Many of the regressions were justifiable since they were for critical bug fixes.

Triage done by **@rylev**.
Revision range: [96ddd32c..a78c9bee](https://perf.rust-lang.org/?start=96ddd32c4bfb1d78f0cd03eb068b1710a8cebeef&end=a78c9bee4d9d51a3891bd8ecae1f28a93b83653b&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.2%, 3.0%]   | 76    |
| Regressions ❌ <br /> (secondary)  | 1.5%  | [0.3%, 8.4%]   | 69    |
| Improvements ✅ <br /> (primary)   | -0.7% | [-1.8%, -0.2%] | 18    |
| Improvements ✅ <br /> (secondary) | -1.4% | [-3.2%, -0.2%] | 35    |
| All ❌✅ (primary)                 | 0.4%  | [-1.8%, 3.0%]  | 94    |


7 Regressions, 4 Improvements, 6 Mixed; 5 of them in rollups
47 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-11-22.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Style evolution](https://github.com/rust-lang/rfcs/pull/3338)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Expand a style-guide principle: readability in plain text](https://github.com/rust-lang/rust/pull/104506)
* [disposition: merge] [Stabilize native library modifier `verbatim`](https://github.com/rust-lang/rust/pull/104360)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [feature iter_find_many](https://github.com/rust-lang/rfcs/pull/3350)
* [new] [RFC: UTF-8 characters and escape codes in (byte) string literals](https://github.com/rust-lang/rfcs/pull/3349)

## Upcoming Events

Rusty Events between 2022-11-23 - 2022-12-21 🦀

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/ymepy8/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> While working on these userspace Mesa changes today, I did not hit a single GPU kernel driver bug. Not. A. Single. Bug.
>
> This is thanks to Lina's phenomenal efforts. She took a gamble writing the kernel driver in Rust, knowing it would take longer to get to the first triangle but believing it would make for a more robust driver in the end. She was right.
>
> A few months of Lina's Rust development has produced a more stable driver than years of development in C on certain mainline Linux GPU kernel drivers.
>
> I think... I think I have Rust envy 🦀
>
> ....Or maybe just Lina envy 😊

– [Alyssa Rosenzweig tooting on Mastodon](https://social.treehouse.systems/@alyssa/109311654876060384)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1331) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/z38fop/this_week_in_rust_470/)</small>
