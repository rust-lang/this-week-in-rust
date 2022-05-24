Title: This Week in Rust 444
Number: 444
Date: 2022-05-25
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

### Newsletters

### Project/Tooling Updates

* [What’s New in IntelliJ Rust for 2022.1](https://blog.jetbrains.com/rust/2022/05/19/what-s-new-in-intellij-rust-for-2022-1)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W20](https://www.fornjot.app/blog/weekly-dev-log/2022-w20/)
* [Slint (UI crate) weekly update](https://slint-ui.com/thisweek/2022-05-23.html)
* [Apache Arrow has released version 8.0.0 of the DataFusion in-process SQL query engine](https://arrow.apache.org/blog/2022/05/16/datafusion-8.0.0/)

### Observations/Thoughts

* [Rust: A Critical Retrospective](https://www.bunniestudios.com/blog/?p=6375)

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [rustdoc-types](https://docs.rs/rustdoc-types), a crate with types to deserialize Rustdoc's JSON output.

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/crate-of-the-week/2704/1061) for the self-ish suggestion. 

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

363 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-05-16..2022-05-23

* [recover when resolution did not resolve lifetimes](https://github.com/rust-lang/rust/pull/97236)
* [add new lint to enforce whitespace after keywords](https://github.com/rust-lang/rust/pull/97179)
* [lint single-use lifetimes during AST resolution](https://github.com/rust-lang/rust/pull/96833)
* [fix misleading "cannot infer type for type parameter" error](https://github.com/rust-lang/rust/pull/97109)
* [improve `u32 as char` cast diagnostic](https://github.com/rust-lang/rust/pull/97169)
* [suggest dereferencing non-lval mutable reference on assignment](https://github.com/rust-lang/rust/pull/94639)
* [add a query for checking whether a function is an intrinsic](https://github.com/rust-lang/rust/pull/97012)
* [types with reachable constructors are reachable](https://github.com/rust-lang/rust/pull/97096)
* [miri: adjust diagnostics assertion so we don't ICE in setup](https://github.com/rust-lang/miri/pull/2141)
* [miri: initial work on Miri permissive-exposed-provenance](https://github.com/rust-lang/miri/pull/2059)
* [miri: make `allow_data_races_*` public and use it during `EnvVars::cleanup`](https://github.com/rust-lang/miri/pull/2142)
* [remove quadratic behaviour from `-Zunpretty=hir-tree`](https://github.com/rust-lang/rust/pull/97223)
* [clean up derived obligation creation](https://github.com/rust-lang/rust/pull/96892)
* [correctly deal with user type ascriptions in pat](https://github.com/rust-lang/rust/pull/96515)
* [rustc\_parse: move AST -> `TokenStream` conversion logic to `rustc_ast`](https://github.com/rust-lang/rust/pull/97251)
* [stabilize `Ipv6Addr::to_ipv4_mapped`](https://github.com/rust-lang/rust/pull/96906)
* [stabilize `array_from_fn`](https://github.com/rust-lang/rust/pull/94119)
* [add convenience byte offset/check align functions to pointers](https://github.com/rust-lang/rust/pull/95643)
* [add functions to un-poison `Mutex` and `RwLock`](https://github.com/rust-lang/rust/pull/96422)
* [improve codegen of `String::retain` method](https://github.com/rust-lang/rust/pull/96605)
* [change `NonNull::as_uninit_*` to take self by value (as opposed to reference), matching primitive pointers](https://github.com/rust-lang/rust/pull/96100)
* [remove unneeded null pointer asserts in `ptr2int` casts](https://github.com/rust-lang/rust/pull/97188)
* [make `ptr::invalid` not the same as a regular `int2ptr` cast](https://github.com/rust-lang/rust/pull/97219)
* [use pointers in `cell::{Ref,RefMut}` to avoid `noalias`](https://github.com/rust-lang/rust/pull/97027)
* [portable SIMD: add `Mask::cast`](https://github.com/rust-lang/portable-simd/pull/251)
* [backtrace: make Miri backtraces work with `#[global_allocator]`](https://github.com/rust-lang/backtrace-rs/pull/462)
* [hashbrown: add function for getting access to map `table: RawTable<(K, V), A>` field](https://github.com/rust-lang/hashbrown/pull/335)
* [cargo: add unstable `rustc-check-cfg` build script output](https://github.com/rust-lang/cargo/pull/10539)
* [cargo: restore proper error for crate not in local reg](https://github.com/rust-lang/cargo/pull/10683)
* [rustdoc: reduce `clean::Type` size](https://github.com/rust-lang/rust/pull/93963)
* [rustdoc: resolve some more doc links early 2](https://github.com/rust-lang/rust/pull/96713)
* [rustfmt: import_granularity: Don't normalize imports with comments](https://github.com/rust-lang/rustfmt/pull/5311)
* [clippy: fix `cmp_owned` on copy types](https://github.com/rust-lang/rust-clippy/pull/8807)
* [clippy: improve "unknown field" error messages](https://github.com/rust-lang/rust-clippy/pull/8823)
* [clippy: lint indirect usages in `disallowed_methods`](https://github.com/rust-lang/rust-clippy/pull/8852)
* [clippy: `dbg_macro` tolerates use of `dbg!` in test items](https://github.com/rust-lang/rust-clippy/pull/8838)
* [clippy: add suggestions to `rc_clone_in_vec_init`](https://github.com/rust-lang/rust-clippy/pull/8814)
* [rust-analyzer: fix inference when pattern matching a tuple field with a wildcard](https://github.com/rust-lang/rust-analyzer/pull/12355)
* [rust-analyzer: generate enum variant assist](https://github.com/rust-lang/rust-analyzer/pull/12334)
* [rust-analyzer: add "cargo clippy" task preset](https://github.com/rust-lang/rust-analyzer/pull/12326)
* [rust-analyzer: implement inlay hint tooltips](https://github.com/rust-lang/rust-analyzer/pull/12285)
* [rust-analyzer: improve docs generation assist](https://github.com/rust-lang/rust-analyzer/pull/12303)
* [rust-analyzer: add a "Add attribute" assist](https://github.com/rust-lang/rust-analyzer/pull/12296)
* [rust-analyzer: don't swallow build script errors](https://github.com/rust-lang/rust-analyzer/pull/12329)
* [rust-analyzer: fix broken async callback in join lines](https://github.com/rust-lang/rust-analyzer/pull/12342)
* [rustup: don't send logging to stdout](https://github.com/rust-lang/rustup/pull/2985)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-05-25 - 2022-06-22 🦀

### Virtual

* 2022-05-18 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydchbxb/)
* 2022-05-18 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydchbxb/)
* 2022-05-24 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/) & [Berline.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399980/)
* 2022-05-24 | Hyderbad, IN | [Rust Language Hyderbad](https://www.meetup.com/Rust-Hyderabad)
    * [**Rust Forum (half day event)**](https://www.meetup.com/Rust-Hyderabad/events/285837876/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways? (Livestream)**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-05-25 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydchbhc/)
* 2022-05-31 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydchbpc/)
* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)
* 2022-06-07 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydcjbkb/)
* 2022-06-07 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcjbkb/)
* 2022-06-08 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcjblb/)
* 2022-06-09 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydcjbmb/)
* 2022-06-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust June 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/285952122/)
* 2022-06-09 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydcjbmb/)
* 2022-06-14 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcjbsb/)
* 2022-06-15 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbtb/)
* 2022-06-15 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcjbtb/)

### North America

* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-05-25 | New York, NY, US | [Rust NYC](https://www.meetup.com/Rust-NYC/)
    * [**May Lightning Talks: State machines for sync, BonsaiDB, WASM Cloudflare Workers**](https://www.meetup.com/Rust-NYC/events/285925838/)
* 2022-06-08 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcjblb/)
* 2022-06-09 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcjbmb/)

### Europe

* 2022-05-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @AWS**](https://www.meetup.com/Stockholm-Rust/events/285701456/)
* 2022-05-19 & 05-20 | Berlin, DE | [Entwickler.de](https://entwickler.de/)
    * [**Rust Summit (paid)**](https://entwickler.de/rust-summit)
* 2022-05-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developer Meetup: Lightning Talks @ Fiberplane**](https://www.meetup.com/rust-amsterdam-group/events/285291653/)
* 2022-05-30 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks May 2022 *Quickwit Takeover* RSVP @Skills Matter**](https://www.meetup.com/Rust-London-User-Group/events/285740296/)
* 2022-05-31 | Rome, IT | [Rust Roma](https://www.meetup.com/Rust-Roma/)
    * [**When Rocket is fueled by Diesel #Aperitech**](https://www.meetup.com/Rust-Roma/events/285587293/)

### Oceania

* 2022-05-26 | Brisbane City, QL | [Rust Brisbane](https://www.meetup.com/Rust-Brisbane/)
    * [**May Meetup**](https://www.meetup.com/Rust-Brisbane/events/285665676/)

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

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is the difference in approaches of the two languages. In C++ if the code is vulnerable, the blame is on the programmer. In Rust if the code is vulnerable, Rust considers it a failure of the language, and takes responsibility to stop even “bad” programmers from writing vulnerable code. I can’t stress enough how awesome it is that I can be a careless fool, and still write perfectly robust highly multi-threaded code that never crashes.

– [kornel on lobste.rs](https://lobste.rs/s/wiavtb/rust_critical_retrospective#c_jkfhpb) (with a [caveat from ZiCog](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1244) that Rust does *not* guarantee freedom from all vulnerabilities!)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1243) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
