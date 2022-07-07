Title: This Week in Rust 450
Number: 450
Date: 2022-07-06
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
* [Announcing Rust 1.62.0](https://blog.rust-lang.org/2022/06/30/Rust-1.62.0.html)
* [RLS Deprecation](https://blog.rust-lang.org/2022/07/01/RLS-deprecation.html)

### Project/Tooling Updates
* [rust-analyzer changelog #136](https://rust-analyzer.github.io/thisweek/2022/07/04/changelog-136.html)
* [GCC-Rust Feedback Sought - Possibly Aiming For Upstream In GCC 13](https://www.phoronix.com/scan.php?page=news_item&px=GCC-Rust-Upstream-Talk-Thread)
* [Announcing lettre 0.10 - lettre](https://lettre.rs/post/lettre-0-10/)
* [The last two years in Miri](https://www.ralfj.de/blog/2022/07/02/miri.html)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W26](https://www.fornjot.app/blog/weekly-dev-log/2022-w26/)
* [HexoSynth (modular synthesizer) - Devlog 3 - Chunk of GUI Work](https://m8geil.de/posts/hexosynth-3/)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-07-04.html)
* [This week in Databend #49: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-07-06-databend-weekly/)

### Observations/Thoughts
* [C++ & Rust: (Interior) Mutability, Moving and Ownership](https://www.tangramvision.com/blog/c-rust-interior-mutability-moving-and-ownership)
* [More than you've ever wanted to know about errors in Rust](https://www.shuttle.rs/blog/2022/06/30/error-handling)
* [Minimalist Guide to Axum](https://tech.marksblogg.com/axum-rust-web-framework.html)
* [Insights of porting Hugging Face Rust Tokenizers to WASM](https://blog.mithrilsecurity.io/porting-tokenizers-to-wasm/)
* [audio] [New Rustacean with Chris Krycho](https://rustacean-station.org/episode/chris-krycho/)
* [video] [The Future of Programming Languages](https://www.youtube.com/watch?v=oMpqj_nMsg0)

### Rust Walkthroughs
* [mirrord internals - hooking libc functions in Rust and fixing bugs](https://metalbear.co/blog/mirrord-internals-hooking-libc-functions-in-rust-and-fixing-bugs/)
* [Pathfinding in Rust: A tutorial with examples](https://blog.logrocket.com/pathfinding-rust-tutorial-examples/)
* [Plantuml encoding in Rust using TDD](https://maksugr.com/posts/plantuml-encoding-in-rust-using-tdd)
* [A const builder pattern in Rust](https://wapl.es/rust/2022/07/03/const-builder-pattern.html)
* [STM32F4 Embedded Rust at the HAL: Button Controlled Blinking by Timer Polling](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-hal-button-controlled-blinking-by-timer-polling)
* [video] [Simple error handling in Rust](https://www.youtube.com/watch?v=g6WUHcyjsfc)
* [video] [Making a Rust crate compile faster](https://www.youtube.com/watch?v=pMiqRM5ooNw)
* [video] [Rust for the impatient](https://www.youtube.com/watch?v=br3GIIQeefY)
* [video] [Cph.rs Hack Night - Simon Rasmussen on WASM and WAT the fp-bindgen](https://www.youtube.com/watch?v=vJqhqRmSb68)
* [video] [Cph.rs Hack Night - David Pedersen on Axum](https://www.youtube.com/watch?v=ETdmhh7OQpA)

### Miscellaneous
* [DE] [Rust-Code im Linux-Kernel: Merge steht laut Linus Torvalds ab Linux 5.20 bevor](https://www.heise.de/news/Rust-Code-im-Linux-Kernel-Merge-steht-laut-Linus-Torvalds-ab-Linux-5-20-bevor-7154453.html)
* [DE] [Ferris Talk #10: Constant Fun mit Rust – const fn](https://www.heise.de/hintergrund/Ferris-Talk-10-Constant-Fun-mit-Rust-const-fn-7162074.html)
* [DE] [Programmiersprache Rust 1.62 kann Kernel für x86-64 bauen](https://www.heise.de/news/Programmiersprache-Rust-1-62-kann-Kernel-fuer-x86-64-bauen-7159459.html)
* [video] [Rust Coming Soon To A Linux Kernel Near You!!](https://www.youtube.com/watch?v=b1lIU8Hp6XU)

## Crate of the Week

This week's crate is [coprosize](https://crates.io/crates/coprosize), a (you guessed it) program aiding the study of dinosaur dung.

Thanks to [piotr](https://users.rust-lang.org/t/crate-of-the-week/2704/1075) for the self-nomination.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

415 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-06-27..2022-07-04

* [proc\_macro/bridge: stop using a remote object handle for proc\_macro `Punct` and `Group`](https://github.com/rust-lang/rust/pull/98188)
* [diagnostics: structs with new slug syntax](https://github.com/rust-lang/rustc-dev-guide/pull/1377)
* [clean up arg mismatch diagnostic, generalize tuple wrap suggestion](https://github.com/rust-lang/rust/pull/98607)
* [improve some inference diagnostics](https://github.com/rust-lang/rust/pull/98497)
* [some borrowck diagnostic fixes](https://github.com/rust-lang/rust/pull/98603)
* [make TAIT behave exactly like RPIT](https://github.com/rust-lang/rust/pull/96727)
* [fix FFI-unwind unsoundness with mixed panic mode](https://github.com/rust-lang/rust/pull/97235)
* [fix ICE for associated constant generics](https://github.com/rust-lang/rust/pull/98609)
* [fix glob import ICE in rustdoc JSON format](https://github.com/rust-lang/rust/pull/98611)
* [fix rust-call ICE in mir-inliner](https://github.com/rust-lang/rust/pull/98823)
* [fix box with custom allocator in miri](https://github.com/rust-lang/rust/pull/98554)
* [miri: allow non-ZST allocations to be adjacent](https://github.com/rust-lang/miri/pull/2279)
* [miri: enable permissive provenance by default](https://github.com/rust-lang/miri/pull/2275)
* [miri: optimizing Stacked Borrows (part 1?): cache locations of tags in a Borrow Stack](https://github.com/rust-lang/miri/pull/1935)
* [miri: add `./miri clippy`](https://github.com/rust-lang/miri/pull/2288)
* [miri: allocation tracking: also print size, alignment, kind of the allocation](https://github.com/rust-lang/miri/pull/2295)
* [miri: stacked borrows: add option for recursive field retagging](https://github.com/rust-lang/miri/pull/2287)
* [miri: tweak `int2ptr` diagnostics](https://github.com/rust-lang/miri/pull/2280)
* [avoid some `&str` to `String` conversions with `MultiSpan::push_span_label`](https://github.com/rust-lang/rust/pull/98668)
* [avoid unnecessary work in `finalize_resolutions_in`](https://github.com/rust-lang/rust/pull/98569)
* [don't use match-destructuring for derived ops on structs](https://github.com/rust-lang/rust/pull/98446)
* [enable MIR inlining](https://github.com/rust-lang/rust/pull/91743)
* [improve some deriving code and add a test](https://github.com/rust-lang/rust/pull/98376)
* [optimize non-consuming operators](https://github.com/rust-lang/rust/pull/98337)
* [`impl<T: AsRawFd> AsRawFd for` {`Arc`, `Box`}`<T>`](https://github.com/rust-lang/rust/pull/97437)
* [add `fetch_not` method on `AtomicBool`](https://github.com/rust-lang/rust/pull/98479)
* [optimize `Vec::insert` for the case where `index == len`](https://github.com/rust-lang/rust/pull/98755)
* [optimise vectored write](https://github.com/rust-lang/rust/pull/98324)
* [fix data race in `thread::scope`](https://github.com/rust-lang/rust/pull/98503)
* [cargo: don't panic with `--offline`](https://github.com/rust-lang/cargo/pull/10817)
* [rustfmt: allow `#[ignore]` tests to run in rustfmt's test suite](https://github.com/rust-lang/rustfmt/pull/5397)
* [rustfmt: config_type: add `unstable_variant` attribute](https://github.com/rust-lang/rustfmt/pull/5379)
* [clippy: add `Operators` lint pass](https://github.com/rust-lang/rust-clippy/pull/8921)
* [clippy: add `invalid_utf8_in_unchecked`](https://github.com/rust-lang/rust-clippy/pull/9105)
* [clippy: add lint `explicit_auto_deref` take 2](https://github.com/rust-lang/rust-clippy/pull/8355)
* [clippy: don't lint `while_let_loop` when significant drop order would change](https://github.com/rust-lang/rust-clippy/pull/8666)
* [clippy: fix ICE in `dereference.rs`](https://github.com/rust-lang/rust-clippy/pull/9093)
* [clippy: fix `#[expect]` for most clippy lints](https://github.com/rust-lang/rust-clippy/pull/9046)
* [clippy: fix direct `#[allow]` attributes in `let_unit_value`](https://github.com/rust-lang/rust-clippy/pull/9082)
* [clippy: fix false-positive in `equatable_if_let`](https://github.com/rust-lang/rust-clippy/pull/9074)
* [clippy: `new_without_default`: ignore const generics/lifetime params on `fn new`](https://github.com/rust-lang/rust-clippy/pull/9115)
* [clippy: `trivially_copy_pass_by_ref` fixes](https://github.com/rust-lang/rust-clippy/pull/8639)
* [let rust-analyzer ship on stable, non-preview](https://github.com/rust-lang/rust/pull/98640)
* [rust-analyzer: complete raw identifier with "r#" prefix](https://github.com/rust-lang/rust-analyzer/pull/12636)
* [rust-analyzer: show witnesses of non-exhaustiveness in `missing-match-arm` diagnostic](https://github.com/rust-lang/rust-analyzer/pull/12634)
* [rust-analyzer: implement destructuring assignment](https://github.com/rust-lang/rust-analyzer/pull/12428)
* [rust-analyzer: fix regressions on assignment expressions](https://github.com/rust-lang/rust-analyzer/pull/12680)
* [rust-analyzer: fix attribute macros on assoc items being discarded with disabled proc macros](https://github.com/rust-lang/rust-analyzer/pull/12670)
* [rust-analyzer: fix: extract Function produces duplicate fn names](https://github.com/rust-lang/rust-analyzer/pull/12662)
* [rust-analyzer: fix: complete enum variants as patterns in pattern path](https://github.com/rust-lang/rust-analyzer/pull/12627)
* [rust-analyzer: report proc macro errors in expressions correctly as well](https://github.com/rust-lang/rust-analyzer/pull/12648)

### Rust Compiler Performance Triage

Overall the week is a small improvement on average, with some benchmarks
(particularly in the primary category) showing significant improvements due to
the enablement of MIR inlining in
[#91743](https://github.com/rust-lang/rust/pull/91743). Inlining promises to
improve the quality of our generated LLVM IR and make other optimizations more
worthwhile, so it's great to see these early results already being quite
impactful.

Triage done by **@simulacrum**.
Revision range: [baf382e6..880646c](https://perf.rust-lang.org/?start=baf382e63c023259fa1f9042f8f479f183ca6ed3&end=880646ca9c6dc21e04efe2f1940369a45b71ff2d&absolute=false&stat=instructions%3Au)

3 Regressions, 6 Improvements, 6 Mixed; 4 of them in rollups
46 artifact comparisons made in total

[See full report for details](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-07-05.md)

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

Every week [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Make outlives::{components,verify} agree](https://github.com/rust-lang/rust/pull/97406)
* [disposition: merge] [Enforce that layout size fits in isize in Layout](https://github.com/rust-lang/rust/pull/95295)
* [disposition: merge] [Tracking Issue for feature(core_c_str) and feature(alloc_c_string)](https://github.com/rust-lang/rust/issues/98314)
* [disposition: merge] [Remove restrictions on compare-exchange memory ordering.](https://github.com/rust-lang/rust/pull/98383)
* [disposition: merge] [Partially stabilize const_slice_from_raw_parts](https://github.com/rust-lang/rust/pull/97522)
* [disposition: merge] [Implement network primitives with ideal Rust layout, not C system layout](https://github.com/rust-lang/rust/pull/78802)
* [disposition: merge] [Tracking Issue for `feature future_poll_fn`](https://github.com/rust-lang/rust/issues/72302)
* [disposition: merge] [Stabilize const-weak-new](https://github.com/rust-lang/rust/pull/95965)
* [disposition: merge] [stabilise mixed_integer_ops](https://github.com/rust-lang/rust/pull/98345)
* [disposition: merge] [Implement `fmt::Write` for `OsString`](https://github.com/rust-lang/rust/pull/97915)
* [disposition: merge] [Tracking Issue for process_set_process_group](https://github.com/rust-lang/rust/issues/93857)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Don't allow unwinding from Drop impls](https://github.com/rust-lang/rfcs/pull/3288)
* [new] [RFC: Add native code coverage support in Cargo](https://github.com/rust-lang/rfcs/pull/3287)

## Upcoming Events

Rusty Events between 2022-07-06 - 2022-08-03 🦀

### Virtual

* 2022-07-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydckbjb/)
* 2022-07-07 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust, nalgebra, and Fourier Optics**](https://www.meetup.com/charlottesville-rust-meetup/events/285818136/)
* 2022-07-09 | Virtual | [Rust Game Dev](https://github.com/rust-gamedev/wg)
    * [**Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-07-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Rust Meetup - "Lifetimes" and Social Hour**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286994065/)
* 2022-07-12 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydckbqb/)
* 2022-07-12 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/286935763/)
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
* 2022-07-26 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydckbjc/)
* 2022-07-29 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286993342/)
* 2022-07-31 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)
* 2022-08-02 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydclbdb/)
* 2022-08-03 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)

### Europe

* 2022-07-06 | Paris, FR | [Stockly](https://www.welcometothejungle.com/fr)
    * [**Rust Meetup in Paris**](https://www.eventbrite.com/e/rust-meetup-in-paris-hosted-by-stockly-tickets-358592809747)
* 2022-07-12 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/286935763/)

### North America

* 2022-07-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Rust Meetup - "Lifetimes" and Social Hour**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286994065/)
* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)
* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)

### Oceania

* 2022-07-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**July Meetup**](https://www.meetup.com/rust-brisbane/events/286889804/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


# Jobs

<!--

Rust Jobs:

TWiR has removed the jobs posting section. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/voglel/official_rrust_whos_hiring_thread_for_jobseekers/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> TIL: #cargo has build-in aliases for some commands
>
> so next time try out 
>
> cargo r  
> cargo b  
> cargo t

– [@5422m4n on twitter](https://twitter.com/5422m4n/status/1542345726310629376)

llogiq is pretty pleased with his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/vt66bs/this_week_in_rust_450/)</small>
