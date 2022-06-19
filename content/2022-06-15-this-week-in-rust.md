Title: This Week in Rust 447
Number: 447
Date: 2022-06-15
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

* [Cloud Compute Program Update](https://foundation.rust-lang.org/news/2022-06-09-cloud-compute-program-update/)
* [Community Grants Program Awards Announcement](https://foundation.rust-lang.org/news/2022-06-14-community-grants-program-awards-announcement/)

### Newsletters

* [This Month in Rust OSDev: May 2022](https://rust-osdev.com/this-month/2022-05/)

### Project/Tooling Updates

* [Announcing error-stack](https://hash.dev/blog/announcing-error-stack)
* [Rust Explorer: Rust playground with the top 10k crates](https://www.rustexplorer.com/b/about)
* [Clap 3.2: Last Call Before 4.0](https://epage.github.io/blog/2022/06/clap-32-last-call-before-40/)
* [C2Rust is Back](https://immunant.com/blog/2022/06/back/)
* [rust-analyzer changelog #133](https://rust-analyzer.github.io/thisweek/2022/06/13/changelog-133.html)
* [Progress report on rustc_codegen_cranelift (June 2022)](https://bjorn3.github.io/2022/06/13/progress-report-june-2022.html)
* [rustc_codegen_gcc: Progress Report #12](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-12)
* [IntelliJ Rust Changelog #172](https://intellij-rust.github.io/2022/06/14/changelog-172.html)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W23](https://www.fornjot.app/blog/weekly-dev-log/2022-w23/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-06-13.html)
* [This week in Databend #46: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-06-15-databend-weekly/)
* [This week in Fluvio #36: The programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0036/)

### Observations/Thoughts

* [Into the Future with IntoFuture - Improving Rust Async Ergonomics](https://refaktory.net/blog/posts/into-the-future-with-intofuture-improving-rust-async-ergonomics)
* [The Rust borrow checker just got (a little bit) smarter](https://jackh726.github.io/rust/2022/06/10/nll-stabilization.html)
* [Async cancellation: a case study of pub-sub in mini-redis](https://smallcultfollowing.com/babysteps/blog/2022/06/13/async-cancellation-a-case-study-of-pub-sub-in-mini-redis/)
* [Local Async Executors and Why They Should be the Default](https://maciej.codes/2022-06-09-local-async.html)
* [Rust advocacy at a medium-sized startup](https://briankung.dev/2022/06/14/rust-advocacy-at-a-medium-sized-startup/)
* [Carcinisation of mirrord (or: why we use Rust)](https://metalbear.co/blog/carcinisation-of-mirrord-or-why-we-use-rust/)
* [Everything Is Broken: Shipping rust-minidump at Mozilla – Part 1](https://hacks.mozilla.org/2022/06/everything-is-broken-shipping-rust-minidump-at-mozilla/)
* [How to Make Rust Leak Memory (Also: How to Make It Stop)](https://fly.io/blog/rust-memory-leak/)
* [video] [Post-Crust of Rust Q&A](https://www.youtube.com/watch?v=aGrlEdAe6YE)
* [audio] [Rust Foundation with Rebecca Rumbul](https://rustacean-station.org/episode/rebecca-rumbul/)
* [audio] [This Week in Rust - Issue 444](https://rustacean-station.org/episode/twir-444/)

### Rust Walkthroughs

* [Caches In Rust](https://matklad.github.io/2022/06/11/caches-in-rust.html)
* [Flavors of enums with Rust bindgen](https://www.mdaverde.com/posts/rust-bindgen-enum/)
* [Implementing strace in Rust](https://jakobwaibel.com/2022/06/06/ptrace/)
* [Async Cancellation II: Time and Signals](https://blog.yoshuawuyts.com/async-cancellation-2/)
* [Tidy up your Rust imports with a prelude.rs](https://justinwoodring.com/blog/tidy-your-rust-imports-with-prelude/)
* [Hitchikers guide to Lunatic TCP Servers in 🦀](https://towardsdev.com/hitchikers-guide-to-lunatic-tcp-servers-in-ae44258750f1)
* [Building a web application with Rust and WebAssembly](https://kerkour.com/web-application-with-rust-and-webassembly)
* [Unwrap a value with the question mark operator in Rust](https://shane-o.dev/blog/unwrap-a-value-with-the-question-mark-operator-in-rust)
* [video] [Async I/O in Depth: State Machines, Event Loops and Non-Blocking I/O System Calls in Rust (Part 2)](https://www.youtube.com/watch?v=_3LpJ6I-tzc)
* [video] [Crust of Rust: Send, Sync, and their implementors](https://www.youtube.com/watch?v=yOezcP-XaIw)
* [video] [series] [Introduction to Yew.rs](https://www.youtube.com/playlist?list=PLrmY5pVcnuE_R5qJ0o30eGw77bWmnrUtL)

### Research

### Miscellaneous

* [Rusty Jam 2](https://gamedev.rs/blog/rustyjam-02/)
* [GitHub brings supply chain security features to the Rust community](https://github.blog/2022-06-06-github-brings-supply-chain-security-features-to-the-rust-community/)

## Crate of the Week

This week's crate is [dxf-rs](https://github.com/IxMilia/dxf-rs), a library to parse AutoCAD files.

Thanks to [cosj](https://users.rust-lang.org/t/crate-of-the-week/2704/1067) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [cdrs-tokio - Protocol v5 tests](https://github.com/krojew/cdrs-tokio/issues/96)
* [boa has several good first issues](https://github.com/boa-dev/boa/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

368 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-06-06..2022-06-13

* [add support for emitting functions with `coldcc` to LLVM](https://github.com/rust-lang/rust/pull/97512)
* [allow unstable items to be re-exported unstably without requiring the feature be enabled](https://github.com/rust-lang/rust/pull/97301)
* [don't suggest adding `let` in certain `if` conditions](https://github.com/rust-lang/rust/pull/97856)
* [fix ICEs from zsts within unsized types with non-zero offsets](https://github.com/rust-lang/rust/pull/97738)
* [fix precise field capture of univariant enums](https://github.com/rust-lang/rust/pull/97325)
* [never regard macro rules with `compile_error!` invocations as unused](https://github.com/rust-lang/rust/pull/97903)
* [use more targeted suggestion when confusing i8 with std::i8](https://github.com/rust-lang/rust/pull/97845)
* [`ValuePairs::PolyTraitRefs` should be called "trait"s in type error diagnostics](https://github.com/rust-lang/rust/pull/98012)
* [use precise spans for recursive const evaluation](https://github.com/rust-lang/rust/pull/97740)
* [suggest escaping `box` as identifier](https://github.com/rust-lang/rust/pull/97857)
* [suggest to swap a struct and a trait in trait impls](https://github.com/rust-lang/rust/pull/97812)
* [suggest using `iter()` or `into_iter()` for `Vec`](https://github.com/rust-lang/rust/pull/97871)
* [tidy up miscellaneous bounds suggestions](https://github.com/rust-lang/rust/pull/97778)
* [cleanup bound variable handling](https://github.com/rust-lang/rust/pull/97648)
* [do not introduce bindings for types and consts in higher-ranked trait bounds](https://github.com/rust-lang/rust/pull/97927)
* [folding revamp](https://github.com/rust-lang/rust/pull/97447)
* [make `Encodable` and `Encoder` infallible](https://github.com/rust-lang/rust/pull/94732)
* [re-use the type op instead of calling the `implied_outlives_bounds` query directly](https://github.com/rust-lang/rust/pull/97081)
* [remove redundant calls to `reserve` in `impl Write for VecDeque`](https://github.com/rust-lang/rust/pull/97922)
* [remove unnecessary `to_string` and `String::new`](https://github.com/rust-lang/rust/pull/98043)
* [miri: make scheduler preemptive](https://github.com/rust-lang/miri/pull/2208)
* [stabilize `$$` in Rust 1.63.0](https://github.com/rust-lang/rust/pull/95860)
* [stabilize the `bundle` native library modifier](https://github.com/rust-lang/rust/pull/95818)
* [stabilize `explicit_generic_args_with_impl_trait`](https://github.com/rust-lang/rust/pull/96868)
* [stabilize `const_intrinsic_copy`](https://github.com/rust-lang/rust/pull/97276)
* [stabilize scoped threads](https://github.com/rust-lang/rust/pull/97992)
* [allow `ptr_from_addr_cast` to fail](https://github.com/rust-lang/rust/pull/97763)
* [add the `Provider` api to `core::any`](https://github.com/rust-lang/rust/pull/91970)
* [`BTreeSet`: avoid intermediate sorting when collecting sorted iterators](https://github.com/rust-lang/rust/pull/97868)
* [`impl Read` and `Write` for `VecDeque<u8>`](https://github.com/rust-lang/rust/pull/95632)
* [change `Direction::{is_forward,is_backward}` functions into constants](https://github.com/rust-lang/rust/pull/97832)
* [fix infinite recursion in x86\_64 memcmp if SSE2 is not present](https://github.com/rust-lang/compiler-builtins/pull/471)
* [fix `Termination` impl panic on closed stderr](https://github.com/rust-lang/rust/pull/97970)
* [hashbrown: add `Extend<&'a (K, V)> for HashMap<K, V, S, A>`](https://github.com/rust-lang/hashbrown/pull/340)
* [hashbrown: editing `do_alloc` for reducing LLVM IR](https://github.com/rust-lang/hashbrown/pull/341)
* [codegen\_gcc: feature/more simd](https://github.com/rust-lang/rustc_codegen_gcc/pull/176)
* [clippy: fix `iter_overeager_cloned` false positive](https://github.com/rust-lang/rust-clippy/pull/8960)
* [clippy: fix some `#[expect]` lint interaction](https://github.com/rust-lang/rust-clippy/pull/8976)
* [clippy: fix `derive_partial_eq_without_eq`](https://github.com/rust-lang/rust-clippy/pull/8950)
* [clippy: check const context](https://github.com/rust-lang/rust-clippy/pull/8907)
* [rustfmt: dedup `imports_granularity = "Item"` (#4737)](https://github.com/rust-lang/rustfmt/pull/5380)
* [rust-analyzer: more precise proc-macro errors](https://github.com/rust-lang/rust-analyzer/pull/12514)
* [rust-analyzer: restart server automatically on settings changes](https://github.com/rust-lang/rust-analyzer/pull/12477)
* [rust-analyzer: add proc macro ABI for rustc 1.63](https://github.com/rust-lang/rust-analyzer/pull/12492)
* [rust-analyzer: on assoc item name hover, render trait decl docs](https://github.com/rust-lang/rust-analyzer/pull/12519)
* [rust-analyzer: add label to loop](https://github.com/rust-lang/rust-analyzer/pull/12481)
* [rust-analyzer: fix inline variable produce mismatched type](https://github.com/rust-lang/rust-analyzer/pull/12464)
* [rust-analyzer: don't respond to cancelled requests when retrying them](https://github.com/rust-lang/rust-analyzer/pull/12508)
* [rustup: Visual Studio: add the English language pack](https://github.com/rust-lang/rustup/pull/3006)
* [rustup: Visual Studio: let the user choose install method](https://github.com/rust-lang/rustup/pull/3008)
* [rustup: improve handling of Visual Studio errors](https://github.com/rust-lang/rustup/pull/3004)

### Rust Compiler Performance Triage

A mixed week. I suppose it is best to focus on the fact we made some big
improvements to a large number of primary benchmarks, at the cost of some
smaller regressions to a smaller number of primary benchmarks.

Triage done by **@pnkfelix**.
Revision range: [bb55bd44..edab34ab](https://perf.rust-lang.org/?start=bb55bd449e65e611da928560d948982d73e50027&end=edab34ab2abbafc16a78daedf71dbacd2eb0b7bf&absolute=false&stat=instructions%3Au)

**Summary**:

|            | mean | max | count |
|:----------:|:----:|:---:|:-----:|
| Regressions 😿 <br /> (primary) | 0.6% | 1.6% | 35    |
| Regressions 😿 <br /> (secondary) | 2.1% | 8.1% | 23    |
| Improvements 🎉 <br /> (primary) | -0.8% | -3.5% | 72    |
| Improvements 🎉 <br /> (secondary) | -0.8% | -2.9% | 62    |
| All 😿🎉 (primary) | -0.4% | -3.5% | 107   |


4 Regressions, 3 Improvements, 5 Mixed; 4 of them in rollups
47 artifact comparisons made in total

Full report [here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-06-14.md).

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

* [RFC: Add a process_group method to UNIX CommandExt](https://github.com/rust-lang/rfcs/pull/3228)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Document Rust's stance on /proc/self/mem](https://github.com/rust-lang/rust/pull/97837)
* [disposition: merge] [lub: don't bail out due to empty binders](https://github.com/rust-lang/rust/pull/97867)
* [disposition: merge] [Impl Termination for Infallible and then make the Result impls of Termination into a blanket](https://github.com/rust-lang/rust/pull/97803)
* [disposition: merge] [Partial stabilization of "nonzero_checked_ops".](https://github.com/rust-lang/rust/pull/97547)
* [disposition: close] [Tracking issue for RFC 2514, "Union initialization and Drop"](https://github.com/rust-lang/rust/issues/55149)
* [disposition: merge] [Stabilize `Path::try_exists()` and improve doc](https://github.com/rust-lang/rust/pull/97912)
* [disposition: merge] [`impl<T: AsRawFd> AsRawFd for {Arc,Box}<T>`](https://github.com/rust-lang/rust/pull/97437)
* [disposition: merge] [os str capacity documentation](https://github.com/rust-lang/rust/pull/97202)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Cargo and Rustup safe file discovery.](https://github.com/rust-lang/rfcs/pull/3279)
* [new] [Mention infer_static_outlives_requirements removal](https://github.com/rust-lang/rfcs/pull/3278)
* [updated] [Mention de-approval of `cfg(target = "...")`](https://github.com/rust-lang/rfcs/pull/3276)

## Upcoming Events

Rusty Events between 2022-06-15 - 2022-07-13 🦀

### Virtual

* 2022-06-15 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbtb/)
* 2022-06-15 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Nushell**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcjbtb/)
* 2022-06-17 | Virtual | [Rust Iran Meetup](https://rust-meetup.ir/)
    * [**Rust Iran Meetup #7 - Actix-Web Framework**](https://rust-meetup.ir/2022/06/17/seventh-meetup.html)
* 2022-06-20 | Köln, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Reboot**](https://www.meetup.com/rustcologne/events/286557151/)
* 2022-06-21 | Berlin, DE | [Berline.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/06/21/rust-hack-and-learn.html)
* 2022-06-21 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydcjbcc/)
* 2022-06-28 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydcjblc/)
* 2022-06-29 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcjbmc/)
* 2022-06-30 | Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 23th Edition**](https://www.meetup.com/Rust-Linz/events/286029968/)
* 2022-07-05 | Austin, TX, US | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting #10**](https://www.meetup.com/webassembly-and-wasmedge/events/zzdnrsydckbhb/)
* 2022-07-05 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/jbfnrsydckbhb/)
* 2022-07-05 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydckbhb/)
* 2022-07-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydckbjb/)
* 2022-07-07 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust, nalgebra, and Fourier Optics**](https://www.meetup.com/charlottesville-rust-meetup/events/285818136/)
* 2022-07-09 | Virtual | [Rust Game Dev](https://github.com/rust-gamedev/wg)
    * [**Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)

### North America

* 2022-06-15 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/286463726/)
* 2022-06-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcjbcc/)
* 2022-06-29 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - June 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)
* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)

### Europe

* 2022-06-16 | Bristol City, UK | [Rust Bristol](https://www.meetup.com/rust-bristol/)
    * [**Talks - Serverless WASM & Graphics in Rust**](https://www.meetup.com/rust-bristol/events/286391025/)
* 2022-06-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/Rust-Oslo/)
    * [**Async Rust and Embedded**](https://www.meetup.com/Rust-Oslo/events/286236751/)
* 2022-06-22 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/286305083/)
* 2022-06-23 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #26**](https://www.meetup.com/rust-wroclaw/events/286415834/)
* 2022-06-28 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks June 2022: Community Showcase**](https://www.meetup.com/rust-london-user-group/events/286489185/)

### Oceania

* 2022-06-17 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
    * [**June 2022 Meetup**](https://www.meetup.com/Rust-Melbourne/events/285962368/)
* 2022-06-23 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**June Meetup**](https://www.meetup.com/rust-brisbane/events/286385515/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

**Audiotool**

* [Rust / Audio DSP Developer](https://www.audiotool.com/jobs#rust-audio-dsp-developer)

**Kollider**

* [Junior Backend Engineer (Remote)](https://careers.kollider.xyz/junior-backend-engineer/en)

**Disney**

* [Senior Software Engineer (Seattle, WA, US, New York, NY, US, San Francisco Bay Area, CA, US, FT Remote)](https://jobs.disneycareers.com/job/seattle/senior-software-engineer/391/30162448528)

**Stockly**

* [Back-end developer (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly/jobs/back-end-developer-rust-grpc-postgresql_paris)

**Stellate**

* [Rust Engineer (Remote)](https://boards.greenhouse.io/stellate/jobs/4027858005)

**Enso**

* [Senior Rust IDE Developer (Remote)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-ide-developer.md)
* [Senior Cloud Rust Engineer (Remote)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-cloud-developer.md)

**Spire Global**

* [Software Engineers + Engineering Managers (Glasgow, UK, Luxembourg - Relocation Available)](https://spire.com/careers/job-openings/job/?gh_jid=4226230)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Because lower-level software has more operational constraints than higher-level software (e.g. it typically cannot tolerate a runtime or memory management via garbage collection), developing a memory safe language suitable for systems software is particularly challenging. The Rust language has met that challenge, however, and is an excellent candidate for replacing C in many systems applications.
>
> We plan to invest in the tools that allow systems engineers to move their software to Rust. This means investing in improving package management, compilers, and Foreign Function Interface (FFI) generators. In many cases this will include providing interfaces compatible with existing widely-used components to enable transition. With these tools, adoption of a memory safe alternative will scale much faster without replication of efforts.

– [The White House Open Source Software Mobilization Plan, multiple authors](https://8112310.fs1.hubspotusercontent-na1.net/hubfs/8112310/OpenSSF/White%20House%20OSS%20Mobilization%20Plan.pdf?utm_referrer=https%3A%2F%2Fopenssf.org%2Foss-security-mobilization-plan%2F) (PDF link)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1253) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/vdbleg/this_week_in_rust_447/)</small>
