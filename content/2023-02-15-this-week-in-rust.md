Title: This Week in Rust 482
Number: 482
Date: 2023-02-15
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

### Newsletters
* [Rust Nigeria #14](https://rustnigeria.curated.co/issues/14#start)

### Project/Tooling Updates
* [Fornjot (code-first CAD in Rust) - Weekly Release - Really Shouldn't Be That Hard](https://www.fornjot.app/blog/weekly-release/2023-w07/)
* [New release – gtk-rs](https://gtk-rs.org/blog/2023/02/10/new-release.html)
* [rust-analyzer changelog #168](https://rust-analyzer.github.io/thisweek/2023/02/13/changelog-168.html)
* [IntelliJ Rust Changelog #188](https://intellij-rust.github.io/2023/02/13/changelog-188.html)
* [Meilisearch releases v1.0, the first completely stable version of its open-source search engine](https://github.com/meilisearch/meilisearch/releases/tag/v1.0.0/)
* [Zenoh-Flow a Rust-based data-flow programming framework build on Zenoh](https://zenoh.io/blog/2023-02-10-zenoh-flow/)
* [Nutype - the newtype with guarantees](https://github.com/greyblake/nutype)
* [pavex, a new Rust web framework - Progress report #1](https://www.lpalmieri.com/posts/pavex-progress-report-01/)

### Observations/Thoughts
* [How Rust went from a side project to the world’s most-loved programming language](https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/)
* [The bottom emoji breaks rust-analyzer](https://fasterthanli.me/articles/the-bottom-emoji-breaks-rust-analyzer)
* [Rust to WebAssembly the hard way](https://surma.dev/things/rust-to-webassembly/index.html)
* [From Erlang to Lunatic](https://mattpo.pe/posts/from-erlang-to-lunatic/)
* [STM32F4 Embedded Rust at the PAC: GPIO Control](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-pac-gpio-control)
* [A Case for Rust in Deep Learning](https://burn-rs.github.io/blog/a-case-for-rust-in-deep-learning)
* [TrustZone, trials and tribulations](https://tweedegolf.nl/en/blog/85/trustzone-trials-tribulations)
* [video] [Rust Releases! 1.67.0 & 1.67.1](https://www.youtube.com/watch?v=-UEyf1li1A4)
* [Everything you didn't need to know about `#[track_caller]`](https://hegdenu.net/posts/track-caller/)

### Rust Walkthroughs
* [video] [Build and deploy a Wasm smart contract with Rust-based language](https://www.youtube.com/watch?v=YnmBotet6_M)
* [video] [Write better parsers with nom_supreme](https://www.youtube.com/watch?v=Ph7xHhBfH0w)

### Miscellaneous
* [video] [From cargo to crates.io and back again](https://www.youtube.com/watch?v=zGS-HqcAvA4)
* [video] [Functional correctness with refinement types for Rust](https://m.youtube.com/live/_qUcEOQJByU?feature=share)
* [audio] [Rust ABI with Aurimas Blažulionis](https://rustacean-station.org/episode/aurimas-blazulionis/)
* [audio] [What's New in Rust 1.65, 1.66, and 1.67](https://rustacean-station.org/episode/rust-1.65-1.66-1.67/)
* [audio] [Open Source Security Podcast: A lesson in Rust from Carol Nichols](https://opensourcesecuritypodcast.libsyn.com/episode-362-a-lesson-in-rust-from-carol-nichols)

## Crate of the Week

This week's crate is [bkmr](https://github.com/sysid/bkmr), a fast CLI bookmark manager and launcher.

Thanks to [sysid](https://users.rust-lang.org/t/crate-of-the-week/2704/1159) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [warp - [Windows] locale and installer](https://gitlab.gnome.org/World/warp/-/issues/39)
* [man-in-the-middle-proxy - custom certificate](https://github.com/emanuele-em/man-in-the-middle-proxy/issues/5)
* [man-in-the-middle-proxy - Filter request by method](https://github.com/emanuele-em/man-in-the-middle-proxy/issues/6)
* [man-in-the-middle-proxy - Request duplication](https://github.com/emanuele-em/man-in-the-middle-proxy/issues/7)
* [man-in-the-middle-proxy - Delete single request](https://github.com/emanuele-em/man-in-the-middle-proxy/issues/8)
* [man-in-the-middle-proxy - modify request with custom fields and headers](https://github.com/emanuele-em/man-in-the-middle-proxy/issues/9)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

387 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-02-06..2023-02-13

* [add `-Z instrument-xray` flag](https://github.com/rust-lang/rust/pull/102963)
* [add missing normalization for union fields types](https://github.com/rust-lang/rust/pull/106938)
* [add only modified subcommand for compiletest](https://github.com/rust-lang/rust/pull/107657)
* [add parentheses properly for borrowing suggestion](https://github.com/rust-lang/rust/pull/105019)
* [allow multiple candidates with same response in new solver](https://github.com/rust-lang/rust/pull/107863)
* [allow wasi-libc to initialize its environment variables lazily](https://github.com/rust-lang/rust/pull/107866)
* [avoid exposing type parameters and implementation details sourced from macro expansions](https://github.com/rust-lang/rust/pull/107789)
* [disqualify `auto trait` built-in impl in new solver if explicit `impl` exists](https://github.com/rust-lang/rust/pull/107815)
* [do not bring trait alias supertraits into scope](https://github.com/rust-lang/rust/pull/107803)
* [do not eagerly recover for bad `impl Trait` types in macros](https://github.com/rust-lang/rust/pull/107813)
* [enable new rlib in non stable cases](https://github.com/rust-lang/rust/pull/105601)
* [fix implied outlives bounds logic for projections](https://github.com/rust-lang/rust/pull/101680)
* [fix suggestions rendering when the diff span is multiline](https://github.com/rust-lang/rust/pull/107671)
* [implement `deferred_projection_equality` for erica solver](https://github.com/rust-lang/rust/pull/107507)
* [introduce `-Zterminal-urls` to use OSC8 for error codes](https://github.com/rust-lang/rust/pull/107838)
* [make &mut !Unpin not dereferenceable, and `Box<!Unpin>` not noalias](https://github.com/rust-lang/rust/pull/106180)
* [make `derive_const` derive properly const-if-const impls](https://github.com/rust-lang/rust/pull/107777)
* [mark `'atomic_mut_ptr'` methods const](https://github.com/rust-lang/rust/pull/107706)
* [mir-Opt for copying enums with large discrepancies](https://github.com/rust-lang/rust/pull/85158)
* [optimize `TyKind::eq`](https://github.com/rust-lang/rust/pull/107717)
* [optimize `query_cache_hit` to reduce code size of the query hot path](https://github.com/rust-lang/rust/pull/107529)
* [reduce interning](https://github.com/rust-lang/rust/pull/107869)
* [simplify layout calculations in rawvec](https://github.com/rust-lang/rust/pull/107167)
* [suggest function call on pattern type mismatch](https://github.com/rust-lang/rust/pull/107098)
* [support `true` and `false` as boolean flag params](https://github.com/rust-lang/rust/pull/107043)
* [turn projections into copies in `CopyProp`](https://github.com/rust-lang/rust/pull/107662)
* [unused-lifetimes: don't warn about lifetimes originating from expanded code](https://github.com/rust-lang/rust/pull/107648)
* [implement `AsFd` and `AsRawFd` for `Rc`](https://github.com/rust-lang/rust/pull/107317)
* [implement cursors for BTreeMap](https://github.com/rust-lang/rust/pull/105641)
* [improve the `array::map` codegen](https://github.com/rust-lang/rust/pull/107634)
* [reverse Timsort scan direction](https://github.com/rust-lang/rust/pull/107191)
* [speedup heapsort by 1.5x by making it branchless](https://github.com/rust-lang/rust/pull/107894)
* [stabilize feature `cstr_from_bytes_until_nul`](https://github.com/rust-lang/rust/pull/107429)
* [stop at the first `NULL` argument when iterating `argv`](https://github.com/rust-lang/rust/pull/106001)
* [cargo: `-Zrustdoc-scrape-example` must fail with bad build script](https://github.com/rust-lang/cargo/pull/11694)
* [cargo: add '-C' flag for changing current dir before build](https://github.com/rust-lang/cargo/pull/10952)
* [cargo: re-export `cargo_new::NewProjectKind` as public](https://github.com/rust-lang/cargo/pull/11700)
* [clippy: make `arithmetic_side_effects` mind constant items](https://github.com/rust-lang/rust-clippy/pull/10310)
* [clippy: `cast_possible_truncation`: issue proper help message](https://github.com/rust-lang/rust-clippy/pull/10330)
* [clippy: `suspicious_to_owned`: use `span_suggestions` to suggest both intents](https://github.com/rust-lang/rust-clippy/pull/10295)
* [clippy: add `suspicious_command_arg_space` lint](https://github.com/rust-lang/rust-clippy/pull/10317)
* [clippy: `almost_swapped`: detect almost-swaps using `let` statements](https://github.com/rust-lang/rust-clippy/pull/10177)
* [clippy: negate suggestions when needed in `bool_assert_comparison`](https://github.com/rust-lang/rust-clippy/pull/10293)
* [rust-analyzer: add braces assist](https://github.com/rust-lang/rust-analyzer/pull/13991)
* [rust-analyzer: add postfix completion for `unsafe`](https://github.com/rust-lang/rust-analyzer/pull/14095)
* [rust-analyzer: add setting for limiting number of completions](https://github.com/rust-lang/rust-analyzer/pull/13986)
* [rust-analyzer: build `i686-pc-windows-msvc` binaries](https://github.com/rust-lang/rust-analyzer/pull/14127)
* [rust-analyzer: don't include `r#` prefix in filesystem changes](https://github.com/rust-lang/rust-analyzer/pull/14138)
* [rust-analyzer: don't insert a semicolon when typing = if parse errors are encountered](https://github.com/rust-lang/rust-analyzer/pull/14103)
* [rust-analyzer: fix bind pat inlay hints rendering for constant patterns](https://github.com/rust-lang/rust-analyzer/pull/14125)
* [rust-analyzer: fix completions after functions with no bodies](https://github.com/rust-lang/rust-analyzer/pull/14110)
* [rust-analyzer: fix parsing of nested tuple field accesses in a cursed way](https://github.com/rust-lang/rust-analyzer/pull/14084)
* [rust-analyzer: fix proc-macro-server incorrectly stripping delimiters](https://github.com/rust-lang/rust-analyzer/pull/14140)
* [rust-analyzer: insert spaces when inlining macros](https://github.com/rust-lang/rust-analyzer/pull/14114)
* [rust-analyzer: properly use location links for type hints of impl Future and its assoc type](https://github.com/rust-lang/rust-analyzer/pull/14099)
* [rust-analyzer: suppress extra indent after the end of field and function chains](https://github.com/rust-lang/rust-analyzer/pull/13975)
* [rust-analyzer: support `DidChangeWorkspaceFolders` notifications](https://github.com/rust-lang/rust-analyzer/pull/14098)

### Rust Compiler Performance Triage

Overall a good week for performance with 77 real world crates benchmarks showing an average of nearly 1% performance improvement. Unfortunately, the largest regressions are not yet fully understood and require additional investigation. Of particular interest were some large improvements in doc builds due to storing additional metadata. However, this change might cause some crates to compile slightly slower in incremental check builds, but this is still being investigated.

Triage done by **@rylev**.
Revision range: [e4dd9edb..9bb6e60](https://perf.rust-lang.org/?start=e4dd9edb76a34ecbca539967f9662b8c0cc9c7fb&end=9bb6e60d1f1360234aae90c97964c0fa5524f141&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.4%  | [0.4%, 11.0%]  | 13    |
| Regressions ❌ <br /> (secondary)  | 0.8%  | [0.2%, 1.6%]   | 4     |
| Improvements ✅ <br /> (primary)   | -1.4% | [-7.9%, -0.3%] | 64    |
| Improvements ✅ <br /> (secondary) | -2.1% | [-5.6%, -0.3%] | 73    |
| All ❌✅ (primary)                 | -0.9% | [-7.9%, 11.0%] | 77    |


3 Regressions, 4 Improvements, 9 Mixed; 4 of them in rollups
46 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-02-14.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Drop support for FreeBSD 10 and 11 from std](https://github.com/rust-lang/rust/issues/89058)
* [disposition: merge] [Stabilize cmpxchg16b_target_feature](https://github.com/rust-lang/rust/pull/106774)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-02-15 - 2023-03-15 🦀

### Virtual

* 2023-02-15 | Virtual | [MongoDB](https://www.mongodb.com/)
    * [**Write a Microservice With Rust and MongoDB**](https://www.mongodb.com/webinar/write-a-microservice-with-rust-and-mongodb)
* 2023-02-15 | Virtual (Redmond, WA, US; New York, NY, US; San Francisco, CA, US; São Paulo, BR) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/) and [Microsoft Rector New York](https://www.meetup.com/microsoft-reactor-new-york/) and [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-redmond/events/290224624/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224621/) | [**São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224623/)
* 2023-02-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Show & Tell: Rust state machines in a file processor**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfcdbtb/)
* 2023-02-16 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfcdbvb/)
* 2023-02-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsyfcdbcc/)
* 2023-02-23 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Tock, a Rust based Embedded Operating System**](https://www.meetup.com/charlottesville-rust-meetup/events/291248593/)
* 2023-02-23 | Virtual (Kassel, DE) | [Java User Group Hessen](https://www.meetup.com/java-user-group-hessen-jugh/)
    * [**Eine Einführung in Rust (Stefan Baumgartner)**](https://www.meetup.com/java-user-group-hessen-jugh/events/290346591/)
* 2023-02-23 | Virtual (México City, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust: ¿por qué es una opción adecuada para implantar Blockchain?**](https://www.meetup.com/rust-mx/events/291456677/)
* 2023-02-24 | Virtual (Tunis, TN) | [Rust Meetup Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number II**](https://www.meetup.com/rust-tunisia/events/291534817/)
* 2023-02-28 | Virtual (Berlin, DE) | [Open Tech School Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/290852327/)
* 2023-02-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust Nation - What we learnt**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291338734/)
* 2023-02-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcdblc/)
* 2023-02-28 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/291437669/)
* 2023-03-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfcfbcb/)
* 2023-03-02 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 30th Edition**](https://www.meetup.com/rust-linz/events/291483339/)
* 2023-03-07 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcfbkb/)
* 2023-03-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/) 
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcfblb/)
* 2023-03-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-03-14 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2023/03/14/rust-hack-and-learn.html)
* 2023-03-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Injecting Rust Hooks into a 1999 game binary (unsafe)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291354288/)

### Asia

* 2023-02-20 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**February Edition - Redis and BioCatch talking Rust!**](https://www.meetup.com/rust-tlv/events/291182881/)
* 2023-03-04 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
    * [**Fn vs FnMut vs FnOnce**](https://www.meetup.com/kansai-rust/events/291614614/)

### Europe

* 2023-02-15 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Nation Pre-Conference Reception with The Rust Foundation**](https://www.meetup.com/rust-london-user-group/events/290903823/)
* 2023-02-15 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust New Year's Resolution Bug Hunt**](https://www.meetup.com/rust-trondheim/events/290889889/)
* 2023-02-16, 2023-02-17 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation '23**](https://www.rustnationuk.com/)
* 2023-02-18 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Post-Conference Rust in Enterprise Brunch Hosted at Red Badger**](https://www.meetup.com/rust-london-user-group/events/291297886/)
* 2023-02-21 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #56**](https://www.meetup.com/rust-paris/events/291334081/)
* 2023-02-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Practical Cryptography - February Meetup (Registration opens 7 Feb 2023)**](https://www.meetup.com/de-DE/rust-zurich/events/290915075/)
* 2023-02-23 | Bordeaux, FR | [DedoTalk](https://www.meetup.com/dedotalk/)
    * [**#1 DedoTalk 🎙️ : Rust pour un développeur Python**](https://www.meetup.com/dedotalk/events/291199962/)
* 2023-02-23 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust metup #33**](https://www.meetup.com/copenhagen-rust-community/events/291288154/)
* 2023-02-23 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Meetup Revived with an Exciting Exploration of Ownership!**](https://www.meetup.com/rust-vienna/events/291465732/)
* 2023-02-28 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/291437669/)
* 2023-02-28 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Rust at RU**](https://www.meetup.com/rust-nederland/events/291489123/)
    * [**Student track: Rust at RU**](https://www.meetup.com/rust-nederland/events/291488539/)
* 2023-03-09 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Rust Meetup #7**](https://www.meetup.com/rust-basel/events/291228934/)
* 2023-03-09 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401965/)
    * [**Student track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401778/)
* 2023-03-15 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk around Embedded World Exhibition**](https://www.meetup.com/rust-noris/events/291623203/)

### North America

* 2023-02-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/rwvwzsyfcdbcc/)
* 2023-02-23 | Mountain View, CA, US | [Mountain View Rust Study Group](https://www.meetup.com/rust-study-group/)
    * [**Rust Study Group at Hacker Dojo**](https://www.meetup.com/rust-study-group/events/291623636/)
* 2023-03-01 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/291619816/)
* 2023-03-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Upcoming Event**](https://www.meetup.com/utah-rust/events/rrwbctyfcfbmb/)

### Oceania

* 2023-02-23 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**February Meetup**](https://www.meetup.com/rust-brisbane/events/291377036/)
* 2023-02-28 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/291278417/)
* 2023-03-01 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - We are back!**](https://www.meetup.com/rust-sydney/events/291265163/)

### South America

* 2023-02-22 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay/)
    * [**Hands on: Lifetimes**](https://www.meetup.com/rust-uruguay/events/291386143/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/10nmtew/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> All the pro C/C++ arguments seem to come down to "Good drivers don’t need seat belts because they don’t get in accidents"

– [otwkme on /r/rust](https://www.reddit.com/r/rust/comments/10rnymj/comment/j6x90x5)

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/113dwhu/this_week_in_rust_482/)</small>
