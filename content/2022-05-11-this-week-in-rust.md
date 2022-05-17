Title: This Week in Rust 442
Number: 442
Date: 2022-05-11
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

* [Security advisory: malicious crate rustdecimal](https://blog.rust-lang.org/2022/05/10/malicious-crate-rustdecimal.html)
* [CTCFT 2022-05-16 Agenda](https://blog.rust-lang.org/inside-rust/2022/05/10/CTCFT-may.html)

### Newsletters

* [Rust Nigeria Community Newsletter #5](https://rustnigeria.curated.co/issues/5)
* [This Month in Rust OSDev: April 2022](https://rust-osdev.com/this-month/2022-04/)

### Project/Tooling Updates

* [rust-analyzer changelog #128](https://rust-analyzer.github.io/thisweek/2022/05/09/changelog-128.html)
* [Fornjot 0.6](https://www.fornjot.app/blog/fornjot-0.6/)
* [Announcing the Kani Rust Verifier Project](https://model-checking.github.io/kani-verifier-blog/2022/05/04/announcing-the-kani-rust-verifier-project.html)
* [Slint (UI crate) weekly update -- Version 0.2.2 Release](https://slint-ui.com/thisweek/2022-05-09.html)
* [This week in Fluvio #32: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0032/)
* [Rocket's 2nd v0.5 Release Candidate - Rocket Web Framework](https://rocket.rs/v0.5-rc/news/2022-05-09-version-0.5-rc.2/)
* [rustc_codegen_gcc: Progress Report #11](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-11)
* [GCC Rust Monthly Report #16 April 2022](https://thephilbert.io/2022/05/04/gcc-rust-monthly-report-16-april-2022/)
* [RepliByte - An open-source tool to seed your dev database with real data](https://www.reddit.com/r/rust/comments/ukmnow/an_opensource_tool_to_seed_your_dev_database_with/)
* [Introducing Crane: Composable and Cacheable Builds with Cargo and Nix](https://ipetkov.dev/blog/introducing-crane/)
* [The run-up to v1.0 for Postcard](https://jamesmunns.com/blog/postcard-1-0-run/)

### Observations/Thoughts

* [Programming languages are platforms, not products](https://kerkour.com/programming-languages-are-platforms)
* [Introducing Rust in security research](https://tweedegolf.nl/en/blog/71/introducing-rust-in-security-research)
* [Xilem: an architecture for UI in Rust](https://raphlinus.github.io/rust/gui/2022/05/07/ui-architecture.html)
* [Modeling Finite State Machines with Rust | Ramnivas Laddad](https://www.ramnivas.com/blog/2022/05/09/fsm-model-rust)
* [Secure computation in Rust: Using Intel's SGX instructions with Teaclave and Fortanix](https://www.notamonadtutorial.com/secure-computation-in-rust-using-intels-sgx-instructions-with-teaclave-and-fortanix/)
* [Securing Crates](https://tl8.co/entry/securing-crates)
* [Over-Engineering A Fairly Simple Coding Challenge](https://ada-x64.github.io/over-engineering/)
* [Building Rust code for my OpenWrt Wi-Fi router](https://blog.dend.ro/building-rust-for-routers/)
* [Lisp interpreter in Rust](https://vishpat.github.io/lisp-rs/)
* [Demystifying Rust Embedded HAL Split and Constrain Methods](https://apollolabsblog.hashnode.dev/demystifying-rust-embedded-hal-split-and-constrain-methods)
* [An O(1) Generic Blog Post About Rust](https://peterkos.me/rust-const-generics/)

### Rust Walkthroughs

* [Building a crawler in Rust: Crawling a JSON API](https://kerkour.com/rust-crawler-json-api)
* [Rust-raspberrypi-OS-tutorials: Tutorial 17 - Kernel Symbols](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/17_kernel_symbols#readme)
* [Rust-raspberrypi-OS-tutorials: Tutorial 18 - Backtracing](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/18_backtrace#readme)
* [Parsing/Recursive Descent Parser](https://www.huy.rocks/everyday/05-08-2022-parsing-recursive-descent-parser)
* [Converting Integers to Floats Using Hyperfocus](https://blog.m-ou.se/floats/)
* [Easy Lossless Trees with Nom and Rowan](https://blog.kiranshila.com/blog/easy_cst.md)

### Research

* [Evolving Collaboration, Dependencies, and Use in the Rust Open Source Software Ecosystem](https://arxiv.org/abs/2205.03597)
* [Modeling Interconnected Social and Technical Risks in Open Source Software Ecosystems](https://arxiv.org/abs/2205.04268)

### Miscellaneous

* [Building a startup with Rust](https://www.shuttle.rs/blog/2021/10/08/building-a-startup-with-rust)
* [Decentralized cluster membership implementation in Rust](https://quickwit.io/blog/chitchat)
* [Black Hat Rust discount: Happy 2022](https://kerkour.com/black-hat-rust-discount-happy-2022)
* [DE] [Redox OS: Ein Betriebssystem, geschrieben in Rust](https://www.heise.de/news/Redox-OS-Ein-Betriebssystem-geschrieben-in-Rust-7071974.html)
* [video] [Let's Code Minesweeper with Rust and WASM](https://www.youtube.com/watch?v=0ywizYLPV00)
* [audio] [Rust Safety with Quentin Ochem and Florian Gilcher](https://rustacean-station.org/episode/067-quentin-ochem-florian-gilcher/)

## Crate of the Week

This week's crate is [enum\_dispatch](https://crates.io/crates/enum_dispatch), a proc-macro-attribute to replace dynamic dispatch with enum dispatch to gain performance.

Thanks to [David Mason](https://users.rust-lang.org/t/crate-of-the-week/2704/1059) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

377 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-05-02..2022-05-09

* [rustc + avr = ❤️❤️ back again](https://github.com/rust-lang/llvm-project/pull/139)
* [support tool lints with the `#[expect]` attribute](https://github.com/rust-lang/rust/pull/95542) (RFC [#2383](https://rust-lang.github.io/rfcs/2383-lint-reasons.html))
* [remove `#[rustc_deprecated]`](https://github.com/rust-lang/rust/pull/95960)
* [suggest fully qualified path with appropriate params](https://github.com/rust-lang/rust/pull/96772)
* [report that opaque types are not allowed in impls even in the presence of other errors](https://github.com/rust-lang/rust/pull/96673)
* [warn on unused `#[doc(hidden)]` attributes on trait impl items](https://github.com/rust-lang/rust/pull/96008)
* [use source callsite in `check_argument_types` suggestion](https://github.com/rust-lang/rust/pull/96589)
* [followups for method call error change](https://github.com/rust-lang/rust/pull/96155)
* [don't cache results of coinductive cycle](https://github.com/rust-lang/rust/pull/96458)
* [speed up `Token::{ident,lifetime}`](https://github.com/rust-lang/rust/pull/96683)
* [overhaul `MacArgs`](https://github.com/rust-lang/rust/pull/96546)
* [generalize "incoherent impls" impl for user defined types](https://github.com/rust-lang/rust/pull/96520)
* [optimize `promote_consts` by caching the results of `validate_local`](https://github.com/rust-lang/rust/pull/96815)
* [codegen\_gcc: implement more SIMD intrinsics](https://github.com/rust-lang/rustc_codegen_gcc/pull/172)
* [codegen\_gcc: use the provided pointee type in `<Builder as BuilderMethods>::load`](https://github.com/rust-lang/rustc_codegen_gcc/pull/170)
* [stabilize `bool::then_some`](https://github.com/rust-lang/rust/pull/96628)
* [add a dedicated length-prefixing method to `Hasher`](https://github.com/rust-lang/rust/pull/94598)
* [fix panic in `Path::strip_prefix`](https://github.com/rust-lang/rust/pull/93675)
* [make `sys::windows::os_str::Slice` `repr(transparent)`](https://github.com/rust-lang/rust/pull/96802)
* [futures: remove `Fuse`s from `select`, and only poll non-terminated streams](https://github.com/rust-lang/futures-rs/pull/2583)
* [hashbrown: remove third copy operation for `RustcOccupiedEntry::insert`](https://github.com/rust-lang/hashbrown/pull/329)
* [cargo: extend pkgid syntax with `@` support](https://github.com/rust-lang/cargo/pull/10582)
* [cargo: improve support of condition compilation checking](https://github.com/rust-lang/cargo/pull/10566)
* [cargo: when documenting private items in a binary, ignore warnings about links to private items](https://github.com/rust-lang/cargo/pull/10142)
* [rust-analyzer: remove handling of `#[rustc_deprecated]`](https://github.com/rust-lang/rust-analyzer/pull/11983)
* [rust-analyzer: lower values of char and byte literals](https://github.com/rust-lang/rust-analyzer/pull/12157)
* [rust-analyzer: sort items by trait definition assist](https://github.com/rust-lang/rust-analyzer/pull/12142)
* [rust-analyzer: allow auto importing starting segments of use items](https://github.com/rust-lang/rust-analyzer/pull/12188)
* [rust-analyzer: don't show assoc. type binding completions when invalid](https://github.com/rust-lang/rust-analyzer/pull/12199)
* [rust-analyzer: fix import insertion inserting after last comment in a file](https://github.com/rust-lang/rust-analyzer/pull/12197)
* [rust-analyzer: fix panic when a macro passes a float token to another macro](https://github.com/rust-lang/rust-analyzer/pull/12178)
* [rust-analyzer: fix snippets triggering where they shouldn't](https://github.com/rust-lang/rust-analyzer/pull/12175)
* [rust-analyzer: remap float parts as integers when parsed as indices](https://github.com/rust-lang/rust-analyzer/pull/12185)
* [rust-analyzer: resolve assoc. types of supertraits in the IDE layer](https://github.com/rust-lang/rust-analyzer/pull/12198)
* [rust-analyzer: try not to invalidate state when the proc macro preference didn't change](https://github.com/rust-lang/rust-analyzer/pull/12171)
* [rust-analyzer: fix macro expansion with float tokens](https://github.com/rust-lang/rust-analyzer/pull/12177)
* [rust-analyzer: split float literal tokens at `.` to fix parsing of tuple field accesses](https://github.com/rust-lang/rust-analyzer/pull/12149)
* [clippy: address `unnecessary_to_owned` false positive](https://github.com/rust-lang/rust-clippy/pull/8794)
* [clippy: create lint against unexpectedly late drop for temporaries in match scrutinee expressions](https://github.com/rust-lang/rust/pull/94206)
* [clippy: fix `cast_lossless` to avoid warning on `usize` to `f64` conversion](https://github.com/rust-lang/rust-clippy/pull/8778)
* [clippy: ignore type aliases in `init_numbered_fields`](https://github.com/rust-lang/rust-clippy/pull/8780)
* [clippy: lint `empty_lint_after_outer_attr` on argumentless macros](https://github.com/rust-lang/rust-clippy/pull/8790)
* [clippy: move `only_used_in_recursion` to nursery](https://github.com/rust-lang/rust-clippy/pull/8783)
* [clippy: optionally allow `expect` and `unwrap` in tests](https://github.com/rust-lang/rust-clippy/pull/8802)
* [clippy: support negative ints in `manual_range_contains`](https://github.com/rust-lang/rust-clippy/pull/8763)
* [clippy: `identity_op` false positive in front of if](https://github.com/rust-lang/rust-clippy/pull/8730)
* [rustfmt: fix `wrap_comments` breaking up type links](https://github.com/rust-lang/rustfmt/pull/5262)

### Rust Compiler Performance Triage

A good week: Several performance improvements, many around macro expansion. Only
one regression of note, and that PR author opened an issue to follow up on it.

Triage done by **@pnkfelix**.
Revision range: [468492c2..c51871c4](https://perf.rust-lang.org/?start=468492c2af3993f18b1fe98052200575c4a2e678&end=c51871c469f7ed3b35ae25d7e6e77bc73fbdd0e3&absolute=false&stat=instructions%3Au)


**Summary**:

|            | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:----------:|:------------------------------:|:--------------------------------:|:-------------------------------:|:---------------------------------:|:------------------------:|
| count      | 11                             | 37                               | 117                             | 65                                | 128                      |
| mean       | 0.7%                           | 0.7%                             | -1.2%                           | -1.6%                             | -1.1%                    |
| max        | 1.5%                           | 1.9%                             | -6.5%                           | -5.2%                             | -6.5%                    |


2 Regressions, 4 Improvements, 1 Mixed; 1 of them in rollups
59 artifact comparisons made in total

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-05-10.md) for more.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Make RwLockReadGuard covariant](https://github.com/rust-lang/rust/pull/96820)
* [disposition: merge] [Extend ptr::null and null_mut to all thin (including extern) types](https://github.com/rust-lang/rust/pull/94954)
* [disposition: merge] [Modify MIR building to drop repeat expressions with length zero](https://github.com/rust-lang/rust/pull/95953)
* [disposition: merge] [Tracking issue for explicit_generic_args_with_impl_trait](https://github.com/rust-lang/rust/issues/83701)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Precise Pre-release Deps](https://github.com/rust-lang/rfcs/pull/3263)
* [new] [Rolling co-lead roles for T-compiler](https://github.com/rust-lang/rfcs/pull/3262/files)
* [new] [RFC: extended_hrtbs](https://github.com/rust-lang/rfcs/pull/3261)
* [new] [Deprecating UnwindSafe](https://github.com/rust-lang/rfcs/pull/3260)

## Upcoming Events

Rusty Events between 2022-05-11 - 2022-06-08 🦀

### Virtual

* 2022-05-11 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydchbpb/)
* 2022-05-11 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/Xe61Zebj6tY53HR7A)
* 2022-05-12 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/tzjtssydchbqb/)
* 2022-05-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust May 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/285767149/)
* 2022-05-12 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydchbqb/)
* 2022-05-17 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydchbwb/)
* 2022-05-18 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydchbxb/)
* 2022-05-18 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydchbxb/)
* 2022-05-24 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399980/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-05-25 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydchbhc/)
* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)
* 2022-06-07 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcjbkb/)
* 2022-06-08 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcjblb/)

### North America

* 2022-05-11 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydchbpb/)
* 2022-05-12 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydchbqb/)
* 2022-05-17 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/285680468/)
* 2022-05-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydchbwb/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-06-08 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcjblb/)

### Europe

* 2022-05-17 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**On Site Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/285820373/)
* 2022-05-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @AWS**](https://www.meetup.com/Stockholm-Rust/events/285701456/)
* 2022-05-19 & 05-20 | Berlin, DE | [Entwickler.de](https://entwickler.de/)
    * [**Rust Summit (paid)**](https://entwickler.de/rust-summit)
* 2022-05-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developer Meetup: Lightning Talks @ Fiberplane**](https://www.meetup.com/rust-amsterdam-group/events/285291653/)

### Oceania

* 2022-05-26 | Brisbane City, QL | [Rust Brisbane](https://www.meetup.com/Rust-Brisbane/)
    * [**May Meetup**](https://www.meetup.com/Rust-Brisbane/events/285665676/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Element**

* [Software Engineer - VoIP (Go/Rust) (Remote)](https://apply.workable.com/elementio/j/5BD58AFB6F/)

**NXLog**

* [Rust Developer (Remote, Europe or worldwide)](https://application.nxlog.org/jobs/detail/rust-developer-39)

**Quickwit**

* [Senior Software Engineer, Rust & distributed systems (Remote, European/Asian time zones)](https://quickwit.io/jobs/distributed-software-engineer)

**Timescale**

* [Senior Rust Engineer - TimescaleDB Toolkit (Remote: UTC-5 to UTC-8)](https://www.timescale.com/careers/5920911002?gh_jid=5920911002)

**Enso**

* [Senior Rust IDE Developer (Remote Europe and US)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-ide-developer.md)
* [Senior Cloud Rust Engineer (Remote Europe and US)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-cloud-developer.md)

**Estuary**

* [Developer Evangelist (New York, NY, US, Columbus, OH, US, or Remote)](https://www.estuary.dev/about/#developerevangelist)
* [Senior Backend Engineer (New York, NY, US, Columbus, OH, US, or Remote)](https://www.estuary.dev/about/#backendengineer)

**Stockly**

* [Technical Engineer - Product Specialist (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-technical-engineer-rust-grpc-postgresql_paris)
* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)

**Kraken**

* [Engineering Manager - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/53def500-b146-40da-89a8-98adfd7e84e4)
* [Backend Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/53def500-b146-40da-89a8-98adfd7e84e4)

**Tempus Ex**

* [Several full-time Rust positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US, and Remote)](https://tempus-ex.com/careers)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> At Cloudflare we have big Rust projects/teams and onboard new developers regularly.
>
> There is a learning curve. Rust is rigid and unforgiving, and noobs need assistance when the compiler says “no” (although error messages and Clippy do a good job for common mistakes).
>
> However, the big upside is that noobs can contribute safely to Rust projects. Rust limits severity of the damage an inexperienced programmer can cause. Once they manage to get the code to compile, it already has lots of correctness guarantees. “Bad” Rust code may just clone more than strictly necessary, or write 10 lines of code for something that has a helper method in the stdlib, but it won’t corrupt memory or blindly run the happy path without checking for errors. Rust prefers to be locally explicit, so it’s also easy to review.

– [Kornel.Lesiński on lobste.rs](https://lobste.rs/s/ksj3ii/rust_is_hard_yes_does_it_matter)

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://this-week-in-rust.org/blog/2022/05/11/this-week-in-rust-442/)</small>
