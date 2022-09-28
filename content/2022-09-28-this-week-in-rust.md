Title: This Week in Rust 462
Number: 462
Date: 2022-09-28
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
* [Announcing Rust 1.64.0 | Rust Blog](https://blog.rust-lang.org/2022/09/22/Rust-1.64.0.html)

### Newsletters
* [This Month in Rust GameDev #37 - August 2022](https://gamedev.rs/news/037/)

### Project/Tooling Updates
* [rust-analyzer - changelog #148](https://rust-analyzer.github.io/thisweek/2022/09/26/changelog-148.html)
* [IntelliJ Rust Changelog #179](https://intellij-rust.github.io/2022/09/26/changelog-179.html)
* [Announcing `async-dns`](https://www.reddit.com/r/rust/comments/xld9jk/announcing_asyncdns/)
* [Fornjot - Weekly Release - 2022-W39](https://www.fornjot.app/blog/weekly-release/2022-w39/)
* [gitoxide - August: Useful rev-spec parsing and an abstraction for remotes](https://github.com/Byron/gitoxide/discussions/541)
* [Getting Started with Seaography - A GraphQL framework for SeaORM](https://www.sea-ql.org/blog/2022-09-27-getting-started-with-seaography/)

### Observations/Thoughts
* [Internship Projects 2022: Concrete Playback](https://model-checking.github.io/kani-verifier-blog/2022/09/22/internship-projects-2022-concrete-playback.html)
* [Why Volvo thinks you should have Rust in your car](https://medium.com/volvo-cars-engineering/why-volvo-thinks-you-should-have-rust-in-your-car-4320bd639e09)
* [Linux embracing Rust will boost robotics community](https://www.therobotreport.com/linux-embracing-rust-will-boost-robotics-community/)
* [Better Java logging, inspired by Clojure and Rust](https://mccue.dev/pages/9-25-22-better-java-logging)
* [Why Async Rust](https://blog.yoshuawuyts.com/why-async-rust/)
* [Apache APISIX loves Rust! (and me too)](https://blog.frankel.ch/rust-apisix/1/)
* [Safe pinned initialization](https://y86-dev.github.io/blog/safe-pinned-initialization/overview.html)
* [Enabling Rapid Pulumi Prototyping with Rust](https://www.pulumi.com/blog/pulumi-watch-mode-with-rust/)
* [STM32F4 Embedded Rust at the HAL: SPI with the MAX7219 LED Dot Matrix](https://apollolabsblog.hashnode.dev/stm32f4-embedded-rust-at-the-hal-spi-with-the-max7219-led-dot-matrix)
* [audio] [Rustacean Station: Ockam with Mrinal Wadhwa](https://rustacean-station.org/episode/mrinal-wadhwa/)

### Rust Walkthroughs
* [Building a Real-Time Web Cipher with Rust, Sycamore and Trunk](https://rsdlt.github.io/posts/rust-sycamore-trunk-wasm-iterators-vigenere-cipher/)
* [Dyn async traits, part 9: call-site selection](https://smallcultfollowing.com/babysteps/blog/2022/09/21/dyn-async-traits-part-9-callee-site-selection/)
* [Rust 2024...the year of everywhere?](https://smallcultfollowing.com/babysteps/blog/2022/09/22/rust-2024-the-year-of-everywhere/)
* [Building Nix flakes from Rust workspaces](https://www.tweag.io/blog/2022-09-22-rust-nix/)
* [Accessing Firebird With Diesel and Rust](https://betterprogramming.pub/diesel-firebird-en-a4d00e793e1d)
* [Multithreading in Rust](https://kerkour.com/multithreading-in-rust)
* [Flutter and Rust combined](https://blog.argonauths.eu/2022/09/26/flutter-and-rust-combined-creating-a-plugin-to-support-various-operating-systems/)

### Miscellaneous
* [DE] [CTO von MS Azure: Nehmt Rust für neue Projekte und erklärt C/C++ für überholt!](https://www.heise.de/news/CTO-von-MS-Azure-Nehmt-Rust-fuer-neue-Projekte-und-erklaert-C-C-fuer-ueberholt-7269887.html)
* [DE] [Rust Foundation erhält 460.000 US-Dollar und gründet ein Team für Security](https://www.heise.de/news/Rust-Foundation-erhaelt-460-000-US-Dollar-und-gruendet-ein-Team-fuer-Security-7264511.html)
* [DE] [Programmiersprache Rust 1.64 erweitert asynchrone Programmierung mit IntoFuture](https://www.heise.de/news/Programmiersprache-Rust-1-64-erweitert-asynchrone-Programmierung-mit-IntoFuture-7273465.html)
* [video] [Rust & Wasm (Safe and fast web development)](https://www.youtube.com/watch?v=P4LMfkFLRsI)
* [video] [Crust of Rust: Build Scripts and Foreign-Function Interfaces (FFI)](https://www.youtube.com/watch?v=pePqWoTnSmQ)
* [video] [Bevy Basics Reflect](https://www.youtube.com/watch?v=7JYyqVlEjKE)

## Crate of the Week

This week's crate is [serde-transcode](https://docs.rs/serde-transcode), a crate to efficiently convert between various serde-supporting formats

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1109) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [zerocopy - Test more conditions in GitHub actions](https://github.com/google/zerocopy/issues/12)
* [pw-sys - help with CI for one of diesel's dependencies](https://github.com/sgrif/pq-sys/issues/42)
* [Ockam - Improve CowStr Display](https://github.com/build-trust/ockam/issues/3531)
* [Ockam - https://github.com/build-trust/ockam/issues/3507](https://github.com/build-trust/ockam/issues/3507)
* [Ockam - Refactor NodeManager constructor](https://github.com/build-trust/ockam/issues/3527)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

347 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-09-12..2022-09-19

* [add armv5te-none-eabi and thumbv5te-none-eabi targets](https://github.com/rust-lang/rust/pull/101329)
* [compiler-builtins: enable floating point intrinsics for RISCV32 microcontrollers](https://github.com/rust-lang/compiler-builtins/pull/493)
* [rustc\_transmute: fix big-endian discriminants](https://github.com/rust-lang/rust/pull/102076)
* [allow `~const` bounds on non-const functions](https://github.com/rust-lang/rust/pull/102273)
* [allow specializing on const trait bounds](https://github.com/rust-lang/rust/pull/102224)
* [recover from struct nested in struct](https://github.com/rust-lang/rust/pull/102143)
* [recover some items that expect braces and don't take semicolons](https://github.com/rust-lang/rust/pull/102286)
* [make cycle errors recoverable](https://github.com/rust-lang/rust/pull/102037)
* [avoid panicking on missing fallback](https://github.com/rust-lang/rust/pull/101952)
* [require `#[const_trait]` on `Trait` for `impl const Trait`](https://github.com/rust-lang/rust/pull/100982)
* [resolve async fn signature even without body (e.g., in trait)](https://github.com/rust-lang/rust/pull/102161)
* [diagnostics: avoid syntactically invalid suggestion in if conditionals](https://github.com/rust-lang/rust/pull/102210)
* [add help for invalid inline argument](https://github.com/rust-lang/rust/pull/101904)
* [suggest `Default::default()` when binding isn't initialized](https://github.com/rust-lang/rust/pull/102184)
* [improve error for when query is unsupported by crate](https://github.com/rust-lang/rust/pull/101958)
* [improve the help message for an invalid calling convention](https://github.com/rust-lang/rust/pull/100488)
* [look at move place's type when suggesting mutable reborrow](https://github.com/rust-lang/rust/pull/101431)
* [note if mismatched types have a similar name](https://github.com/rust-lang/rust/pull/101664)
* [note the type when unable to drop values in compile time](https://github.com/rust-lang/rust/pull/102194)
* [miri: don't back up past the caller when looking for an FnEntry span](https://github.com/rust-lang/miri/pull/2537)
* [interpret: expose `generate_stacktrace` without full `InterpCx`](https://github.com/rust-lang/rust/pull/101985)
* [inline `SyntaxContext` in both encoded span representation](https://github.com/rust-lang/rust/pull/98840)
* [introduce `mir::Unevaluated`](https://github.com/rust-lang/rust/pull/102056)
* [only generate closure def id for async fns with body](https://github.com/rust-lang/rust/pull/102244)
* [use function pointers instead of macro-unrolled loops in `rustc_query_impl`](https://github.com/rust-lang/rust/pull/101785)
* [separate definitions and HIR owners in the type system](https://github.com/rust-lang/rust/pull/102040)
* [use `partition_point` instead of `binary_search` when looking up source lines](https://github.com/rust-lang/rust/pull/101999)
* [skip `Equate` relation in `handle_opaque_type`](https://github.com/rust-lang/rust/pull/102069)
* [calculate `ProjectionTy::trait_def_id` for return-position `impl Trait` in trait correctly](https://github.com/rust-lang/rust/pull/102152)
* [manually cleanup token stream when macro expansion aborts](https://github.com/rust-lang/rust/pull/100250)
* [neither require nor imply lifetime bounds on opaque type for well formedness](https://github.com/rust-lang/rust/pull/95474)
* [normalize closure signature after construction](https://github.com/rust-lang/rust/pull/101708)
* [normalize opaques with bound vars](https://github.com/rust-lang/rust/pull/100980)
* [split out `async_fn_in_trait` into a separate feature](https://github.com/rust-lang/rust/pull/100734)
* [support overriding initial rustc and cargo paths](https://github.com/rust-lang/rust/pull/102266)
* [use internal iteration in `Iterator` comparison methods](https://github.com/rust-lang/rust/pull/100845)
* [`alloc`: add unstable cfg features `no_rc` and `no_sync`](https://github.com/rust-lang/rust/pull/89891)
* [a fn pointer doesn't implement `Fn`/`FnMut`/`FnOnce` if its return type isn't sized](https://github.com/rust-lang/rust/pull/100096)
* [fix `ConstProp` handling of `written_only_inside_own_block_locals`](https://github.com/rust-lang/rust/pull/102045)
* [implied\_bounds: deal with inference vars](https://github.com/rust-lang/rust/pull/102016)
* [make `Condvar`, `Mutex`, `RwLock` const constructors work with the `unsupported` impl](https://github.com/rust-lang/rust/pull/98457)
* [make projection bounds with const bounds satisfy const](https://github.com/rust-lang/rust/pull/101989)
* [resolve: set effective visibilities for imports more precisely](https://github.com/rust-lang/rust/pull/102109)
* [add option to deduplicate extern blocks](https://github.com/rust-lang/rust-bindgen/pull/2258)
* [codegen: implement manuallydrop fields better](https://github.com/rust-lang/rust-bindgen/pull/2279)
* [optimize `array::IntoIter`](https://github.com/rust-lang/rust/pull/100214)
* [std: use `sync::RwLock` for internal statics](https://github.com/rust-lang/rust/pull/100581)
* [stabilize const `BTree{Map,Set}::new`](https://github.com/rust-lang/rust/pull/102197)
* [constify `Default` impl's for Arrays and Tuples](https://github.com/rust-lang/rust/pull/102200)
* [constify `cmp_min_max_by`](https://github.com/rust-lang/rust/pull/102245)
* [constify `slice.split_at_mut`(`_unchecked`)](https://github.com/rust-lang/rust/pull/101800)
* [add `const_closure`, constify `Try` trait](https://github.com/rust-lang/rust/pull/102186)
* [make `ManuallyDrop` satisfy `~const Destruct`](https://github.com/rust-lang/rust/pull/102204)
* [make `from_waker`, `waker` and `from_raw` unstably `const`](https://github.com/rust-lang/rust/pull/101798)
* [extend `const_convert` with `const` {`FromResidual`, `Try`} for `ControlFlow`](https://github.com/rust-lang/rust/pull/102144)
* [recover error strings on Unix `from_lossy_utf8`](https://github.com/rust-lang/rust/pull/99609)
* [cargo: add support for relative git submodule paths](https://github.com/rust-lang/cargo/pull/11106)
* [cargo: improve errors for TOML fields that support workspace inheritance](https://github.com/rust-lang/cargo/pull/11113)
* [cargo: report cmd aliasing failure with more contexts](https://github.com/rust-lang/cargo/pull/11087)
* [cargo: error trailing args rather than ignore](https://github.com/rust-lang/cargo/pull/11119)
* [cargo: forward non-UTF8 arguments to external subcommands](https://github.com/rust-lang/cargo/pull/11118)
* [cargo: make unknown features on `cargo add` more discoverable](https://github.com/rust-lang/cargo/pull/11098)
* [rustdoc: stabilize --diagnostic-width](https://github.com/rust-lang/rust/pull/102216)
* [bindgen: handle `no_return` attributes](https://github.com/rust-lang/rust-bindgen/pull/2259)
* [bindgen: remove file added by mistake](https://github.com/rust-lang/rust-bindgen/pull/2283)
* [clippy: add `matches!` checking to `nonstandard_macro_braces`](https://github.com/rust-lang/rust-clippy/pull/9471)
* [clippy: fix ICE in `needless_pass_by_value` with unsized `dyn Fn`](https://github.com/rust-lang/rust-clippy/pull/9531)
* [clippy: fix ICE in `unnecessary_to_owned`](https://github.com/rust-lang/rust-clippy/pull/9505)
* [clippy: fix panic when displaying the backtrace of failing integration tests](https://github.com/rust-lang/rust-clippy/pull/9533)
* [clippy: moved `derive_partial_eq_without_eq` to nursery](https://github.com/rust-lang/rust-clippy/pull/9535)
* [clippy: `never_loop`: fix FP with `let`..`else` statements](https://github.com/rust-lang/rust-clippy/pull/9496)
* [clippy: `nonstandard_macro_braces` do not modify macro arguments](https://github.com/rust-lang/rust-clippy/pull/9499)
* [clippy: new `uninlined_format_args` lint to inline explicit arguments](https://github.com/rust-lang/rust-clippy/pull/9233)
* [clippy: `uninit_vec`: fix false positive with `set_len(0)`](https://github.com/rust-lang/rust-clippy/pull/9519)
* [rust-analyzer: add assist to unwrap tuple declarations](https://github.com/rust-lang/rust-analyzer/pull/13248)
* [rust-analyzer: fix diagnostics not working in enum variant bodies](https://github.com/rust-lang/rust-analyzer/pull/13288)
* [rust-analyzer: fix operator highlighting tags applying too broadly](https://github.com/rust-lang/rust-analyzer/pull/13268)
* [rust-analyzer: properly set the enum variant body type from the repr attribute](https://github.com/rust-lang/rust-analyzer/pull/13269)
* [rust-analyzer: properly support IDE functionality in enum variants](https://github.com/rust-lang/rust-analyzer/pull/13285)
* [rust-analyzer: use the sysroot proc-macro server for analysis-stats](https://github.com/rust-lang/rust-analyzer/pull/13289)
* [rust-analyzer: display the value of enum variant on hover](https://github.com/rust-lang/rust-analyzer/pull/12966)
* [rust-analyzer: type inference for generators](https://github.com/rust-lang/rust-analyzer/pull/13209)

### Rust Compiler Performance Triage

Overall a fairly quiet week in terms of new changes; the majority of the
delta this week was due to reverting [#101620](https://github.com/rust-lang/rust/pull/101620),
which was a regression noted in last week's report.

Triage done by **@simulacrum**.
Revision range: [8fd6d03e2..d9297d22](https://perf.rust-lang.org/?start=8fd6d03e22fba2930ad377b87299de6a37076074&end=d9297d22ad9edc2b56f0dd8734c1187a0c88be69&absolute=false&stat=instructions%3Au)

2 Regressions, 7 Improvements, 3 Mixed; 3 of them in rollups
53 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-09-27.md)

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

* [Rust Style Team](https://github.com/rust-lang/rfcs/pull/3309)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Allow transmutes between the same types after erasing lifetimes](https://github.com/rust-lang/rust/pull/101520)
* [disposition: merge] [Add `AsFd` implementations for stdio lock types on WASI.](https://github.com/rust-lang/rust/pull/101768)
* [disposition: merge] [Tracking Issue for asm_sym](https://github.com/rust-lang/rust/issues/93333)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [updated] [Update RFC 2906 to match the implementation](https://github.com/rust-lang/rfcs/pull/3321)
* [new] [RFC: `Aligned` trait](https://github.com/rust-lang/rfcs/pull/3319)
* [new] [RFC: Field projection](https://github.com/rust-lang/rfcs/pull/3318)

## Upcoming Events

Rusty Events between 2022-09-28 - 2022-10-26 🦀

### Virtual

* 2022-09-28 | Virtual (London, UK) | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-30 | Virtual (Minneapolis, MN, US) | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/minneapolis-rust-meetup/events/288601893/)
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)
* 2022-10-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #18**](https://www.meetup.com/rust-noris/events/hlvbvsydcnbrb/)
* 2022-10-08 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-10-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcnbpb/)
* 2022-10-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-12 | Virtual (Erlangen, DE) | [Rust Franken](https://www.meetup.com/rust-nerf/)
    * [**Rust Franken Meetup #4**](https://www.meetup.com/rust-nerf/events/288723552/)
* 2022-10-12 | Virtual (San Francisco, CA, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/)
* 2022-10-13 | Virtual (Berlin, DE) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-15 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 2 (CuteCopter): Reverse Engineering a tiny drone**](https://www.meetup.com/rust-noris/events/287347851/)
* 2022-10-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsydcnbxb/)
* 2022-10-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcnbzb/)
* 2022-10-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcnbbc/)
* 2022-10-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcnbhc/)

### Asia

* 2022-10-11 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Cost-Efficient Rust in Practice**](https://www.meetup.com/tokyo-rust-meetup/events/288597490/)

### Europe

* 2022-09-28 | London, UK + Virtual | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-09-30 | Berlin, DE | [RustFi Hackathon](https://rustfi.keyrock.com/)
    * [**RustFi Hackathon 30 Sept - 2 Oct**](https://rustfi.keyrock.com/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)
* 2022-10-03 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Microsoft Reactor**](https://www.meetup.com/Stockholm-Rust/events/288453360/)
* 2022-10-04 | Helsinki, FI | [Finland Rust Meetup](https://www.meetup.com/Finland-Rust-Meetup/)
    * [**October meetup**](https://www.meetup.com/Finland-Rust-Meetup/events/288724370/)
* 2022-10-06 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #29**](https://www.meetup.com/rust-wroclaw/events/288667400/)
* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)
* 2022-10-13 | Berlin, DE + Virtual | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-18 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #53**](https://www.meetup.com/rust-paris/events/288735204/)

### North America

* 2022-09-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288590318/)
* 2022-09-29 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Zola o como la comunidad de RustMX tiene página**](https://www.meetup.com/rust-mx/events/288388973/)
* 2022-10-13 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcnbrb/)
* 2022-10-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcnbxb/)
* 2022-10-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Anyhow ? Turbofish ::<> / HTTP calls and errors in Rust.**](https://www.meetup.com/rust-nyc/events/288756215/)
* 2022-10-25 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**Rust DHCP**](https://www.meetup.com/rust-toronto/events/288589539/)

### Oceania

* 2022-10-10 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust Lightning Talks**](https://www.meetup.com/rust-sydney/events/288746516/)
* 2022-10-20 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Tune Up Edition: software engineering management**](https://www.meetup.com/rust-wellington/events/288738684/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/xldzbl/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Semver has its philosophy, but a pragmatic approach to versioning is:
>
> `<upgrades may break API> . <downgrades may break API> . <fine either way>`

– [Kornel on rust-users](https://users.rust-lang.org/t/semver-for-refactoring-change/81370/13)

Thanks to [Artem Borisovskiy](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1301) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/xqs1gg/this_week_in_rust_462/)</small>
