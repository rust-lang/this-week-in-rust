Title: This Week in Rust 449
Number: 449
Date: 2022-06-29
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
* [Call for testing: Cargo sparse-registry](https://blog.rust-lang.org/2022/06/22/sparse-registry-testing.html)
* [Announcing The RustConf PostConf UnConf](https://blog.rust-lang.org/2022/06/28/rust-unconference.html)
* [1.62.0 pre-release testing](https://blog.rust-lang.org/inside-rust/2022/06/28/1.62.0-prerelease.html)

### Foundation
* [Making a Career Move with Rust: A Developer’s Approach](https://foundation.rust-lang.org/news/2022-06-21-guest-blog-series-dotan-nahum/)

### Project/Tooling Updates
* [rust-analyzer changelog #135](https://rust-analyzer.github.io/thisweek/2022/06/27/changelog-135.html)
* [IntelliJ Rust Changelog #173](https://intellij-rust.github.io/2022/06/27/changelog-173.html)
* [Cross v0.2.2 Released](https://www.reddit.com/r/rust/comments/vk2xfc/cross_v022_released/)
* [Release-plz: release Rust packages from CI](https://www.marcoieni.com/2022/06/release-plz-release-rust-packages-from-ci/)
* [fmtools: Reinventing Rust formatting syntax](https://casualhacks.net/blog/2022-06-24/reinventing-rust-fmt/)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W25](https://www.fornjot.app/blog/weekly-dev-log/2022-w25/)
* [This week in Databend #48: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-06-29-databend-weekly/)
* [This week in Fluvio #37: The programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0037/)
* [Fetch-Data: New crate to download hash-verified files if necessary](https://docs.rs/fetch-data/latest/fetch_data/)

### Observations/Thoughts
* [Many modes: a GATs pattern](https://smallcultfollowing.com/babysteps/blog/2022/06/27/many-modes-a-gats-pattern/)
* [Complexity](https://www.ncameron.org/blog/complexity/)
* [Fuzzing rust-minidump for Embarrassment and Crashes – Part 2](https://hacks.mozilla.org/2022/06/fuzzing-rust-minidump-for-embarrassment-and-crashes/)
* [PrettySize 0.3 release and a weakness in rust’s type system](https://neosmart.net/blog/2022/prettysize-0-3-release-and-a-weakness-in-rusts-type-system/)
* [Obscure Rust: reborrowing is a half-baked feature](https://haibane-tenshi.github.io/rust-reborrowing/)
* [An Unfortunate Experience with Rust](https://blog.polybdenum.com/2022/06/25/an-unfortunate-experience-with-rust.html)
* [(Ab)using Rust traits to write silly things](https://ihatereality.space/08-abusing-rust-traits-to-write-silly-things/)
* [video] [Why Do Programmers Love Rust?](https://www.youtube.com/watch?v=vEuG2YoJZJw)
* [video] [Day in the Life of Open Source Maintenance](https://www.youtube.com/watch?v=lkwkBry1xcE)
* [video] [RTIC: Real time concurrency on ARM Cortex-M](https://www.youtube.com/watch?v=mzFTjn9eftU)
* [audio] [What's New in Rust 1.60 and 1.61](https://rustacean-station.org/episode/rust-1.60-1.61/)
* [audio] [Zig with Andrew Kelley](https://rustacean-station.org/episode/andrew-kelley/)
* [audio] [This Week in Rust - Issue 446](https://rustacean-station.org/episode/twir-446/)

### Rust Walkthroughs
* [Rust Default Values for Maintainability](https://cj.rs/blog/rust-default-values-for-maintainability/)
* [rust-phf: the perfect hash function](https://simplabs.com/blog/2022/06/23/the-perfect-hash-function/)
* [Generative metatag images in Rust](https://www.shuttle.rs/blog/2022/06/23/generative-metatag-images)
* [Play Ping-Pong with Lunatic 🦀 UDP](https://missmissm.medium.com/play-ping-pong-with-lunatic-udp-ef557a22a604)
* [Markdown Blog in Rust with Tide](https://nyxtom.dev/2022/06/26/tide)
* [Integrating a Svelte app with Rust using WebAssembly](https://blog.logrocket.com/integrating-svelte-app-rust-webassembly/)
* [STM32F4 Embedded Rust at the HAL: GPIO Button Controlled Blinking](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-hal-gpio-button-controlled-blinking)
* [Open Source Walk-Through with Rust & SeaORM](https://www.nahua.dev/blog/open-source-walk-through-with-rust-seaorm/)
* [video] [Rc and RefCell Smart Pointers](https://www.youtube.com/watch?v=KYJ95TxEC18)
* [video] [Microdosing Rust: Why & How to Get Started with AVR?](https://www.youtube.com/watch?v=3o_lzQMLU5Q)

### Miscellaneous
* [Deadlock-free Mutexes and Directed Acyclic Graphs](https://bertptrs.nl/2022/06/23/deadlock-free-mutexes-and-directed-acyclic-graphs.html)
* [Call for applications: Knurling-rs Summer of Code 2022 🦀](https://ferrous-systems.com/blog/knurling-summer-of-code/)
* [Memory Safety for the World’s Largest Software Project](https://www.memorysafety.org/blog/memory-safety-in-linux-kernel/)
* [Linus Torvalds: Rust For The Kernel Could Possibly Be Merged For Linux 5.20](https://www.phoronix.com/scan.php?page=news_item&px=Rust-For-Linux-5.20-Possible)

## Crate of the Week

This week's crate is [cap-std](https://github.com/bytecodealliance/cap-std) a std-replacement that introduces capabilities to facilitate defense-in-depth sandboxing.

Thanks to [Kinrany](https://users.rust-lang.org/t/crate-of-the-week/2704/1073) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [lockdiff - Add some automated tests](https://github.com/your-tools/lockdiff/issues/1)
* [pest - Transferring maintenance & keeping pest alive](https://github.com/pest-parser/pest/discussions/606)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

388 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-06-20..2022-06-27

* [LLVM on AVR: fix expanding `MOVW` for overlapping registers](https://github.com/rust-lang/llvm-project/pull/141)
* [work around llvm 12's memory ordering restrictions](https://github.com/rust-lang/rust/pull/98385)
* [proc_macro/bridge: cache static spans in proc_macro's client thread-local state](https://github.com/rust-lang/rust/pull/98187)
* [remove (transitive) reliance on sorting by `DefId` in pretty-printer](https://github.com/rust-lang/rust/pull/96955)
* [avoid an ICE and instead let the compiler report a useful error](https://github.com/rust-lang/rust/pull/98329)
* [greatly improve error reporting for futures and generators in `note_obligation_cause_code`](https://github.com/rust-lang/rust/pull/98259)
* [improve memory ordering diagnostics](https://github.com/rust-lang/rust/pull/97389)
* [improve suggestion for calling fn-like expr on type mismatch](https://github.com/rust-lang/rust/pull/98280)
* [diagnostics: consider parameter count when suggesting smart pointers](https://github.com/rust-lang/rust/pull/98509)
* [mitigate MMIO stale data vulnerability](https://github.com/rust-lang/rust/pull/98126)
* [suggest defining variable as mutable on `&mut _` type mismatch in pats](https://github.com/rust-lang/rust/pull/98431)
* [use correct substs in enum discriminant cast](https://github.com/rust-lang/rust/pull/98429)
* [improve `derive(Debug)`](https://github.com/rust-lang/rust/pull/98190)
* [miri: `freebsd-target-support`](https://github.com/rust-lang/miri/pull/2221)
* [miri: handle wildcard pointers in Stacked Borrows](https://github.com/rust-lang/miri/pull/2196)
* [miri: require local annotations for local diagnostics](https://github.com/rust-lang/miri/pull/2250)
* [miri: add `-Zmiri-report-progress` to regularly print a stacktrace of what we are executing](https://github.com/rust-lang/miri/pull/2272)
* [miri: fix ICE when const refers to extern static](https://github.com/rust-lang/miri/pull/2241)
* [miri: make sure a thread is joined](https://github.com/rust-lang/miri/pull/2276)
* [expand expressions where possible](https://github.com/rust-lang/rust/pull/98148) (RFC [#2011](https://rust-lang.github.io/rfcs/2011-generic-assert.html))
* [partial stabilization of "nonzero_checked_ops"](https://github.com/rust-lang/rust/pull/97547)
* [stabilize `NonZero*` checked operations constness](https://github.com/rust-lang/rust/pull/97908)
* [add `Iterator::next_chunk`](https://github.com/rust-lang/rust/pull/93700)
* [leak `pthread_`{`mutex`, `rwlock`}`_t` if it's dropped while locked](https://github.com/rust-lang/rust/pull/98194)
* [make `RwLockReadGuard` covariant](https://github.com/rust-lang/rust/pull/96820)
* [windows: Iterative `remove_dir_all`](https://github.com/rust-lang/rust/pull/96412)
* [portable-simd: change `Simd::splat` to not generate a loop](https://github.com/rust-lang/portable-simd/pull/284)
* [cargo: fetch GitHub commits by long hash more efficiently](https://github.com/rust-lang/cargo/pull/10079)
* [cargo: stabilize config-cli](https://github.com/rust-lang/cargo/pull/10755)
* [compiletest: strip debuginfo by default for mode=ui](https://github.com/rust-lang/rust/pull/98140)
* [clippy: add `manual_rem_euclid` lint](https://github.com/rust-lang/rust-clippy/pull/9031)
* [clippy: add `cargo dev deprecate`](https://github.com/rust-lang/rust-clippy/pull/8871)
* [clippy: check for `--force-warn` in Clippy's driver run condition](https://github.com/rust-lang/rust-clippy/pull/9036)
* [clippy: fix `extra_unused_lifetimes` false positive](https://github.com/rust-lang/rust-clippy/pull/9037)
* [clippy: fix `let_undescore_lock` false-positive when binding without locking](https://github.com/rust-lang/rust-clippy/pull/8990)
* [clippy: lint `single_match` on `Option` matches](https://github.com/rust-lang/rust-clippy/pull/8985)
* [clippy: suggest `pointer::cast` when possible in `transmute_ptr_to_ref`](https://github.com/rust-lang/rust-clippy/pull/8939)
* [clippy: add `manual_find` lint for function return case](https://github.com/rust-lang/rust-clippy/pull/8649)
* [clippy: add `vec.capacity()` to `slow_vec_initialization` detection](https://github.com/rust-lang/rust-clippy/pull/8953)
* [clippy: confirm using chain in `collapsible_span_lint_calls`](https://github.com/rust-lang/rust-clippy/pull/9028)
* [clippy: `enum_variant_names` should ignore when all prefixes are `_`](https://github.com/rust-lang/rust-clippy/pull/9032)
* [clippy: new lint `manual_retain`](https://github.com/rust-lang/rust-clippy/pull/8972)
* [clippy: put parentheses around `neg_multiply` suggestion if needed](https://github.com/rust-lang/rust-clippy/pull/9026)
* [rust-analyzer: correct `target_feature` completion](https://github.com/rust-lang/rust-analyzer/pull/12635)
* [rust-analyzer: fix completions for locals not working properly inside macro calls](https://github.com/rust-lang/rust-analyzer/pull/12643)
* [rust-analyzer: improve proc macro errors a bit](https://github.com/rust-lang/rust-analyzer/pull/12629)
* [rust-analyzer: completes non exhaustive variant within the defining crate](https://github.com/rust-lang/rust-analyzer/pull/12625)
* [rust-analyzer: deduplicate cfg completions](https://github.com/rust-lang/rust-analyzer/pull/12642)
* [rust-analyzer: fix `doc_links` link type - Determine link type at start](https://github.com/rust-lang/rust-analyzer/pull/12605)

### Rust Compiler Performance Triage

Overall, a positive week for compiler performance with regressions mainly being relegated to smaller issues. This can be see by the fact that 95 test cases in real world crates were improvements while regressions only happened in 22 test cases. The largest improvement was by @nnethercote where the compilation of `#[derive(Debug)]` was improved. This led to an average of 1% improvement in compile times across 124 real world crate test cases.

Triage done by **@rylev**.
Revision range: [abace0..baf382](https://perf.rust-lang.org/?start=abace0a1f17986d89aedf610819deab2b4afee56&end=baf382e63c023259fa1f9042f8f479f183ca6ed3&absolute=false&stat=instructions%3Au)

**Summary**:

|            | mean | max | count |
|:----------:|:----:|:---:|:-----:|
| Regressions 😿 <br /> (primary) | 0.6% | 2.4% | 22    |
| Regressions 😿 <br /> (secondary) | 0.5% | 1.0% | 35    |
| Improvements 🎉 <br /> (primary) | -1.1% | -5.2% | 95    |
| Improvements 🎉 <br /> (secondary) | -2.3% | -10.3% | 35    |
| All 😿🎉 (primary) | -0.8% | -5.2% | 117   |

3 Regressions, 3 Improvements, 4 Mixed; 5 of them in rollups
34 artifact comparisons made in total

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [RFC: Serve crates-io registry over HTTP as static files](https://github.com/rust-lang/rfcs/pull/2789)
    * [Testing steps](https://github.com/rust-lang/rfcs/pull/2789#issuecomment-1166155551)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Remove restrictions on compare-exchange memory ordering.](https://github.com/rust-lang/rust/pull/98383)
* [disposition: merge] [ptr::copy and ptr::swap are doing untyped copies](https://github.com/rust-lang/rust/pull/97712)
* [disposition: merge] [Tracking issue for `IntoFuture`](https://github.com/rust-lang/rust/issues/67644)
* [disposition: merge] [Implement `FusedIterator` for `std::net::[Into]Incoming`](https://github.com/rust-lang/rust/pull/97300)
* [disposition: merge] [Tracking Issue for core_ffi_c](https://github.com/rust-lang/rust/issues/94501)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC 3283: Backward compatible default features](https://github.com/rust-lang/rfcs/pull/3283)

## Upcoming Events

Rusty Events between 2022-06-29 - 2022-07-27 🦀

### Virtual

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
* 2022-07-13 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/rFzwUjh5YT1pVci6A)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)
* 2022-07-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydckbcc/)
* 2022-07-31 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)

### Asia

* 2022-07-04 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**Rust Interop, Rewrites and fun**](https://www.meetup.com/rust-tlv/events/286610368/)

### Europe

* 2022-06-29 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Get started with Rust and stories from the frontlines**](https://www.meetup.com/rust-nederland/events/286582960/)
* 2022-06-30 | Copenhagen, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #26**](https://cph.rs/)
* 2022-07-06 | Paris, FR | [Stockly](https://www.welcometothejungle.com/fr)
    * [**Rust Meetup in Paris**](https://www.eventbrite.com/e/rust-meetup-in-paris-hosted-by-stockly-tickets-358592809747)

### North America

* 2022-06-29 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/286596997/)
* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)
* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

We have made some changes to our jobs section, please see [this GitHub issue](https://github.com/rust-lang/this-week-in-rust/issues/3412) for an explanation and the [most current Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/utav5l/official_rrust_whos_hiring_thread_for_jobseekers/) for job postings.

# Quote of the Week

> JG: mem::replace / mem::swap ===
> [Indiana Jones swapping the artifact for a bag of sand in a temple](https://c.tenor.com/eqLNYv0A9TQAAAAC/swap-indiana-jones.gif)
>
> CV: except rustc would tell Indy that's a type mismatch
>
> JG: Yes, that would be the boulder, I assume.
>
> Older compilers were more aggressive in error reporting.

[Jake Goulding and Cuviper on the Rust Zulip](https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Clone.20implementation.20for.20Box)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1262) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/vnzd1l/this_week_in_rust_449/)</small>
