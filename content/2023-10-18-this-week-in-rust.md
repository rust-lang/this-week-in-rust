Title: This Week in Rust 517
Number: 517
Date: 2023-10-18
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

### Foundation
* [Q3 2023 Recap from Rebecca Rumbul](https://foundation.rust-lang.org/news/q3-2023-recap-from-rebecca-rumbul/)

### Newsletters
* [This Month in Rust OSDev: September 2023](https://rust-osdev.com/this-month/2023-09/)

### Project/Tooling Updates
* [Announcing EtherCrab, a Rust implementation of the EtherCAT industrial automation protocol](https://wapl.es/announcing-ethercrab-the-rust-ethercat-controller/)
* [rust-analyzer changelog #203](https://rust-analyzer.github.io/thisweek/2023/10/16/changelog-203.html)

### Observations/Thoughts
* [Why async Rust?](https://without.boats/blog/why-async-rust/)
* [Compile Times and Code Graphs](https://blog.danhhz.com/compile-times-and-code-graphs)
* [Containerize Rust applications on Ubuntu & Alpine, with GitHub Actions](https://medium.com/@vapor.schitcrafter/containerise-rust-applications-on-ubuntu-alpine-with-github-actions-or-docker-builders-9378a02b98fd)

### Rust Walkthroughs
* [A type level contains operation for heterogeneous list using associated types](https://blog.weiznich.de/blog/eurorust-non-overlapping-contains-for-hlists/)
* [Using GraphQL in Rust](https://www.shuttle.rs/blog/2023/10/16/graphql-in-rust)
* [Writing parsers in Winnow](https://www.youtube.com/watch?v=QF3kMyzMC40)

### Research
* [Yuga: Automatically Detecting Lifetime Annotation Bugs in the Rust Language](https://arxiv.org/abs/2310.08507)
* [Fast Summary-based Whole-program Analysis to Identify Unsafe Memory Accesses in Rust](https://arxiv.org/abs/2310.10298)

### Miscellaneous
* [Eurorust reflections](https://smallcultfollowing.com/babysteps/blog/2023/10/14/eurorust-reflections/)
* [EuroRust 2023 Reflections: What's a Conference For?](https://lucumr.pocoo.org/2023/10/14/eurorust-whats-a-conference/)
* [audio] [RustShip: Graphite - Raster and Vector Graphics in Rust](https://ieni.dev/2023/10/%EF%B8%8F-graphite-raster-and-vector-graphics-in-rust-rustship-4/)

## Crate of the Week

This week's crate is [rinf](https://github.com/cunarist/rinf), a library to write Rust in Flutter.

Thanks to [Kim Dong-Hyun](https://users.rust-lang.org/t/crate-of-the-week/2704/1249) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch (Hacktoberfest) - [OpenNode] Currency Unit Conversion](https://github.com/juspay/hyperswitch/issues/2240)
* [Hyperswitch (Hacktoberfest) - [Stax] Currency Unit Conversion](https://github.com/juspay/hyperswitch/issues/2246)
* [Hyperswitch (Hacktoberfest) - [ACI] Currency Unit Conversion](https://github.com/juspay/hyperswitch/issues/2198)
* [Ockam - Make `ockam space show` (no args) interactive by asking the user to choose from a list of space names to show (tuify)](https://github.com/build-trust/ockam/issues/6472)
* [Ockam - Improve `ockam tcp-inlet delete --help` text (`clap` command)](https://github.com/build-trust/ockam/issues/6645)
* [Ockam - Enroll "email: '+' character not allowed"](https://github.com/build-trust/ockam/issues/6095)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

409 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-10-09..2023-10-16

* [`const_eval`: allow function pointer signatures containing &mut T in const contexts](https://github.com/rust-lang/rust/pull/116015)
* [`rustc_hir_pretty` cleanups](https://github.com/rust-lang/rust/pull/116625)
* [add `Config::hash_untracked_state` callback](https://github.com/rust-lang/rust/pull/116731)
* [add the V (vector) extension to the riscv64-linux-android target spec](https://github.com/rust-lang/rust/pull/116618)
* [also consider call and yield as MIR SSA](https://github.com/rust-lang/rust/pull/113915)
* [broaden the consequences of recursive TLS initialization](https://github.com/rust-lang/rust/pull/116172)
* [cleanup `rustc_features` some more](https://github.com/rust-lang/rust/pull/116550)
* [compute NLL loan scopes using the polonius model](https://github.com/rust-lang/rust/pull/113218)
* [const-eval: allow calling functions with targat features disabled at compile time in WASM](https://github.com/rust-lang/rust/pull/116576)
* [const-eval: make misalignment a hard error](https://github.com/rust-lang/rust/pull/115524)
* [coverage: separate initial span extraction from span processing](https://github.com/rust-lang/rust/pull/116409)
* [detect ruby-style closure in parser](https://github.com/rust-lang/rust/pull/116645)
* [do not check for impossible predicates in const-prop lint](https://github.com/rust-lang/rust/pull/116315)
* [don't UB on dangling ptr deref, instead check inbounds on projections](https://github.com/rust-lang/rust/pull/114330)
* [exhaustiveness: rework constructor splitting](https://github.com/rust-lang/rust/pull/116391)
* [explicitly handle auto trait leakage in coherence](https://github.com/rust-lang/rust/pull/116689)
* [fix exit status / wait status on non-Unix `cfg(unix)` platforms](https://github.com/rust-lang/rust/pull/115108)
* [fix overflow checking in range patterns](https://github.com/rust-lang/rust/pull/116623)
* [handle several `#[diagnostic::on_unimplemented]` attributes correctly](https://github.com/rust-lang/rust/pull/116642)
* [implement `-Clink-self-contained=-linker` opt out](https://github.com/rust-lang/rust/pull/116014)
* [improve check-cfg diagnostics](https://github.com/rust-lang/rust/pull/116666)
* [improve handling of assertion failures with very long conditions](https://github.com/rust-lang/rust/pull/116548)
* [in smir use `FxIndexMap` to store indexed ids](https://github.com/rust-lang/rust/pull/116560)
* [linker: also pass debuginfo compression flags](https://github.com/rust-lang/rust/pull/116702)
* [make "request changes" reviews apply `S-waiting-on-author`](https://github.com/rust-lang/rust/pull/116661)
* [on type error involving closure, avoid ICE](https://github.com/rust-lang/rust/pull/116676)
* [on type error of closure call argument, point at earlier calls that affected inference](https://github.com/rust-lang/rust/pull/116250)
* [opt-dist: disable unused features for tabled crate](https://github.com/rust-lang/rust/pull/116790)
* [pass rustc shim flags using environment variable](https://github.com/rust-lang/rust/pull/116448)
* [prevent more spurious unreachable pattern lints](https://github.com/rust-lang/rust/pull/116715)
* [prevent showing methods from blanket impls of not available foreign traits to show up in the search results](https://github.com/rust-lang/rust/pull/116597)
* [prevent spurious `unreachable pattern` lints](https://github.com/rust-lang/rust/pull/115937)
* [relate alias ty with variance](https://github.com/rust-lang/rust/pull/116219)
* [remove `DefiningAnchor::Bubble` from opaque wf check](https://github.com/rust-lang/rust/pull/116802)
* [show `enum` discriminant if a compatible repr is used](https://github.com/rust-lang/rust/pull/116600)
* [stabilize `async fn` and return-position `impl Trait` in trait](https://github.com/rust-lang/rust/pull/115822)
* [structurally normalize for closure](https://github.com/rust-lang/rust/pull/116436)
* [suggest adding `return` if the for semi which can coerce to the fn return type](https://github.com/rust-lang/rust/pull/115196)
* [suggest labeling block if `break` is in bare block](https://github.com/rust-lang/rust/pull/116366)
* [suggest trait bounds for used associated type on type param](https://github.com/rust-lang/rust/pull/116257)
* [support AIX in Rust standard library](https://github.com/rust-lang/rust/pull/109882)
* [use `PatKind::Error` when an ADT const value has violation](https://github.com/rust-lang/rust/pull/116522)
* [use env variable to control thread ids in `rustc_log`](https://github.com/rust-lang/rust/pull/116586)
* [add ability to get lines/filename for Span in smir](https://github.com/rust-lang/rust/pull/116630)
* [miri: implement `llvm.x86.sse41.*` intrinsics](https://github.com/rust-lang/miri/pull/3118)
* [miri: make NaN generation non-deterministic](https://github.com/rust-lang/rust/pull/116551)
* [copy 1-element arrays as scalars, not vectors](https://github.com/rust-lang/rust/pull/116510)
* [optimize `librustc_driver.so` with BOLT](https://github.com/rust-lang/rust/pull/116352)
* [optimize file read in `Config::verify`](https://github.com/rust-lang/rust/pull/116635)
* [optimize zipping over array iterators](https://github.com/rust-lang/rust/pull/115515)
* [stabilize `atomic_from_ptr`](https://github.com/rust-lang/rust/pull/115719)
* [stabilize `const_maybe_uninit_assume_init_read`](https://github.com/rust-lang/rust/pull/116233)
* [stabilize `{IpAddr, Ipv6Addr}::to_canonical`](https://github.com/rust-lang/rust/pull/115955)
* [impl Not, Bit{And,Or}{,Assign} for IP addresses](https://github.com/rust-lang/rust/pull/113747)
* [impl Default for ExitCode](https://github.com/rust-lang/rust/pull/114589)
* [add invariant to `Vec::pop` that len `<` cap if pop successful](https://github.com/rust-lang/rust/pull/114370)
* [implement `BufRead` for `VecDeque<u8>`](https://github.com/rust-lang/rust/pull/110604)
* [implement `OnceCell/Lock::try_insert()`](https://github.com/rust-lang/rust/pull/116540)
* [implement `slice::split_once` and `slice::rsplit_once`](https://github.com/rust-lang/rust/pull/112818)
* [add explicit-endian `String::from_utf16` variants](https://github.com/rust-lang/rust/pull/95967)
* [implement FusedIterator for DecodeUtf16 when the inner iterator does](https://github.com/rust-lang/rust/pull/110729)
* [implement `sys::args` for UEFI](https://github.com/rust-lang/rust/pull/116341)
* [inline `Bytes::next` and `Bytes::size_hint`](https://github.com/rust-lang/rust/pull/116775)
* [make `try_exists` return `Ok(true)` for Windows Unix Sockets](https://github.com/rust-lang/rust/pull/116683)
* [mark `new_in` as `const` for BTree collections](https://github.com/rust-lang/rust/pull/116559)
* [regex-automata/meta: revert broadening of reverse suffix optimization](https://github.com/rust-lang/regex/pull/1111)
* [regex-lite: tweak nest limit on stack overflow test](https://github.com/rust-lang/regex/pull/1106)
* [regex: loosen ASCII compatible rules + improve reverse suffix optimization](https://github.com/rust-lang/regex/pull/1105)
* [regex, regex-automata: fix compilation of doctests on 32-bit architectures](https://github.com/rust-lang/regex/pull/1107)
* [regex-lite: fix compilation of doctests on 32-bit architectures](https://github.com/rust-lang/regex/pull/1101)
* [regex: revert recent regex-syntax interval set optimizations](https://github.com/rust-lang/regex/pull/1102)
* [cargo: `fix(install)`: Suggest an alternative version on MSRV failure](https://github.com/rust-lang/cargo/pull/12798)
* [cargo: add detailed message when target folder path is invalid](https://github.com/rust-lang/cargo/pull/12820)
* [cargo: add package name and version to warning messages](https://github.com/rust-lang/cargo/pull/12799)
* [cargo: support `public` dependency configuration with workspace deps](https://github.com/rust-lang/cargo/pull/12817)
* [rustfmt: support let-chains](https://github.com/rust-lang/rustfmt/pull/5910)
* [rustdoc-search: add impl disambiguator to duplicate assoc items](https://github.com/rust-lang/rust/pull/109422)
* [rustdoc: hide `#[repr(transparent)]` if it isn't part of the public ABI](https://github.com/rust-lang/rust/pull/115439)
* [rustdoc: show crate name beside smaller logo](https://github.com/rust-lang/rust/pull/115948)
* [clippy: `get_first`: lint on non-primitive slices](https://github.com/rust-lang/rust-clippy/pull/11609)
* [clippy: `manual_is_ascii_check`: Also check for `is_ascii_hexdigt`](https://github.com/rust-lang/rust-clippy/pull/11659)
* [clippy: `unnecessary_lazy_eval`: reduce applicability if closure has return type annotation](https://github.com/rust-lang/rust-clippy/pull/11673)
* [clippy: fix ICE in internal author lint](https://github.com/rust-lang/rust-clippy/pull/11664)
* [rust-analyzer: add `replace_is_ok_with_if_let_ok` assist](https://github.com/rust-lang/rust-analyzer/pull/15752)
* [rust-analyzer: add `replace_is_some_with_if_let_some` assist](https://github.com/rust-lang/rust-analyzer/pull/15743)
* [rust-analyzer: add diagnostics messages for chars and byte literal errors](https://github.com/rust-lang/rust-analyzer/pull/15744)
* [rust-analyzer: make cursor select at `_tmp`](https://github.com/rust-lang/rust-analyzer/pull/15755)
* [rust-analyzer: string literals diagnose](https://github.com/rust-lang/rust-analyzer/pull/15746)

### Rust Compiler Performance Triage

Overall an interesting week performance wise, with small improvements to a vast
number of benchmarks seeming to outweigh an isolated set of (slightly) larger
regressions. It included a number of PRs regressed instruction counts but did
not matter for cycle times, plus one mysterious regression to `check_match` and
`mir_borrowck` from reworking constructor splitting (see report on PR 116391 for
details), and an awesome broad set of improvements from automatically inlining
small functions across crates (see report on PR 116505 for details).

Triage done by **@pnkfelix**.
Revision range: [84d44dd1..b9832e72](https://perf.rust-lang.org/?start=84d44dd1d8ec1e98fff94272ba4f96b2a1f044ca&end=b9832e72c9223f4e96049aa5911effd258b92591&absolute=false&stat=instructions%3Au)

4 Regressions, 1 Improvements, 4 Mixed; 3 of them in rollups
84 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/420012f0bb12281b5a3e897280d3f38b241a4735/triage/2023-10-18.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [2024 Edition](https://github.com/rust-lang/rfcs/pull/3501)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for result_option_inspect](https://github.com/rust-lang/rust/issues/91345)
* [disposition: merge] [Allow partially moved values in match](https://github.com/rust-lang/rust/pull/103208)
* [disposition: close] [`read_dir` has unexpected behavior for `""`](https://github.com/rust-lang/rust/issues/114149)
* [disposition: merge] [rustdoc: align stability badge to baseline instead of bottom](https://github.com/rust-lang/rust/pull/105666)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Superseding public/private dependencies](https://github.com/rust-lang/rfcs/pull/3516)
* [new] [add float semantics RFC](https://github.com/rust-lang/rfcs/pull/3514)
* [new] [Reserve `gen` keyword in 2024 edition and start an experiment implementation of `Iterator` generators](https://github.com/rust-lang/rfcs/pull/3513)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-10-18 - 2023-11-15 🦀

### Virtual

* 2023-10-18 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating System Primitives (Atomics & Locks Chapter 8)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296531173/)
* 2023-10-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Pinning**](https://www.meetup.com/vancouver-rust/events/295057159/)
* 2023-10-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcnbzb/)
* 2023-10-19 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)
* 2023-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679778/) | [**Mirror**](https://berline.rs/)
* 2023-10-24 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Month-end Rusting—Fun with 🍌 and 🔎!**](https://www.meetup.com/rustdc/events/296217448/)
* 2023-10-31 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcnbpc/)
* 2023-11-01 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**ECS with Bevy Game Engine**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583207/)
* 2023-11-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyfcpbcb)
* 2023-11-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296661148/)
* 2023-11-09 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732666/)
* 2023-11-15 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Building Our Own Locks (Atomics & Locks Chapter 9)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296582223/)
* 2023-11-15 | Virtual (Richmond, VA, US) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)

### Asia

* 2023-10-18 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Rust and the Age of High-Integrity Languages**](https://www.meetup.com/tokyo-rust-meetup/events/296551482)
* 2023-10-20 | Singapore, SG | [Rust Meetup Singapore](https://www.eventbrite.com/e/rust-meetup-google-developer-space-tickets-736345678747)
    * [**Rust meetup @ Google Developer Space**](https://www.eventbrite.com/e/rust-meetup-google-developer-space-tickets-736345678747)
* 2023-10-21 | Pune, IN | [Rust Pune](https://www.meetup.com/rust-pune)
    * [**Rust 101 and Decoding Fearless Concurrency**](https://www.meetup.com/rust-pune/events/296765951/)

### Europe

* 2023-10-19 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Amsterdam Meetup @ Terraform**](https://www.meetup.com/rust-amsterdam-group/events/296495570/)
* 2023-10-19 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #35**](https://www.meetup.com/rust-wroclaw/events/296507983/)
* 2023-10-20 | Saarbrücken, DE | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Rustlings Live Coding Session**](https://www.meetup.com/rust-saar/events/296800552/)
* 2023-10-24 | Bucharest, RO | [Rust Lang Bucharest Meetup](https://www.meetup.com/rust-lang-bucharest-meetup/)
    * [**Rust Bucharest Meetup #4**](https://www.meetup.com/rust-lang-bucharest-meetup/events/296789945/)
* 2023-10-25 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Biome, web development tooling with Rust**](https://www.meetup.com/rust-dublin/events/295179534/)
* 2023-10-25 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [Rust for the web - Paris meetup #61](https://mobilizon.fr/events/149c0367-66cb-42c6-aa0c-8495bf6d2a52)
* 2023-10-25 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup)
    * [Rust Meetup 2023/10: Lunatic](https://www.meetup.com/zagreb-rust-meetup/events/296765355/)
* 2023-10-26 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Leipzig Rust Meetup #3**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/296183126/)
* 2023-10-26 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust at TU Delft**](https://www.meetup.com/rust-nederland/events/296488286/)
* 2023-10-26 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #4 at SFEIR**](https://www.meetup.com/meetup-group-zgphbyet/events/296766699/)
* 2022-10-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Aira + Netlight**](https://www.meetup.com/Stockholm-Rust/events/296578336/)
* 2023-11-01 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Rust-Meetup November, Save the date**](https://www.meetup.com/rustcologne/events/296540949/)
* 2023-11-07 | Brussels, BE | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus - Rust and Talk beginners edition**](https://www.meetup.com/rust-aarhus/events/296223647/)
* 2023-11-09 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/)
    * [**11th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/296567395)
* 2023-11-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296083417/)

### North America

* 2023-10-18 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch**](https://www.meetup.com/bostonrust/events/296223807/)
* 2023-10-19 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/296369976/)
* 2023-10-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust Goes Where It Pleases Pt2 - Rust on the front end!**](https://www.meetup.com/music-city-rust-developers/events/296254420/)
* 2023-10-19 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - October Meetup**](https://www.meetup.com/seattle-rust-user-group/events/296110729)
* 2023-10-24 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/296422277)
* 2023-10-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/296495790)
* 2023-10-25 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/296657993/)
* 2023-11-04 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Let's make a Discord bot!**](https://www.meetup.com/boulder-rust-meetup/events/296437292/)
* 2023-11-15 | Richmond, VA, US + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2023 (Nov 13-16)**](https://lpc.events/event/17/sessions/170/)

### Oceania

* 2023-10-17 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/296324436/)
* 2023-10-9 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Demolition Day 🤯**](https://www.meetup.com/rust-sydney/events/296672158/)
* 2023-10-26 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**October Meetup**](https://www.meetup.com/rust-brisbane/events/296628243/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/163w6fl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> When your Rust build times get slower after adding some procedural macros:
>
> We call that the syn tax 🦀

– [janet on fosstodon.org](https://fosstodon.org/@janet/111223564960983226)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1472) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/17b86sn/this_week_in_rust_517/)</small>
