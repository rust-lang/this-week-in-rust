Title: This Week in Rust 436
Number: 436
Date: 2022-03-30
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Foundation

* [Member Spotlight: Wyliodrin](https://foundation.rust-lang.org/news/2022-03-29-member-spotlight-wyliodrin/)

### Project/Tooling Updates

* [Helix editor 22.03 released](https://helix-editor.com/news/release-22-03-highlights/)
* [Announcing fp-bindgen](https://fiberplane.dev/blog/announcing-fp-bindgen/)
* [Rust-Analyzer Changelog #122](https://rust-analyzer.github.io/thisweek/2022/03/28/changelog-122.html)
* [IntelliJ Rust Changelog #167](https://intellij-rust.github.io/2022/03/28/changelog-167.html)
* [BonsaiDb v0.4.0: Now available without async](https://bonsaidb.io/blog/bonsaidb-v0-4-0/)
* [Slint (GUI crate) weekly update](https://slint-ui.com/thisweek/2022-03-28.html)
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W12](https://www.fornjot.app/blog/weekly-dev-log/2022-w12/)
* [This week in Databend #35: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-03-30-databend-weekly/)
* [This week in Fluvio #26: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0026/)

### Observations/Thoughts

* [dyn*: can we make dyn sized?](https://smallcultfollowing.com/babysteps/blog/2022/03/29/dyn-can-we-make-dyn-sized/)
* [Oxide on My Wrist: Hubris on PineTime was the best worst idea](https://artemis.sh/2022/03/28/oxide-hubris-on-pinetime.html)
* [Self Modifying Code](https://matklad.github.io/2022/03/26/self-modifying-code.html)
* [Async destructors, async genericity and completion futures](https://sabrinajewson.org/blog/async-drop)
* [A thanks to the traits working group in 2021](https://jackh726.github.io/rust/2022/03/25/a-thanks.html)
* [When not to use Rust?](https://kerkour.com/why-not-rust)
* [Yet Another GitHub Profile Generator](https://blog.urth.org/2022/03/28/yet-another-github-profile-generator/)

### Rust Walkthroughs

* [Cross-compilation in Rust](https://kerkour.com/rust-cross-compilation)
* [Introducing "High Assurance Rust": a FREE systems software security book!](https://highassurance.rs/)
* [Advanced deserialization with Serde: Parsing Cloudformation templates](https://rtoch.com/posts/advanced-serde/)
* [Tiny and Fast Docker image for Rust Application](https://azzamsa.com/n/rust-docker/)
* [My First Clippy Lint - Statistically Insignificant](https://jamesmcm.github.io/blog/2022/03/26/my-first-clippy-lint/)
* [Introducing "High Assurance Rust"](https://www.reddit.com/r/rust/comments/toq2wj/introducing_high_assurance_rust_a_free_systems/)
* [Rust for JavaScript developers: SQS batch error handling with AWS Lambda](https://dfrasca.hashnode.dev/rust-for-javascript-developers-sqs-batch-error-handling-with-aws-lambda)
* [video] [Introduction to WAGI by Rainer Stropek & Stefan Baumgartner](https://www.youtube.com/watch?v=9NDwHBjLlhQ)
* [video] [Rust for Linux by Miguel Ojeda and Wedson Almeida Filho](https://www.youtube.com/watch?v=fVEeqo40IyQ)

### Miscellaneous

* [audio] [Interview with Dustin (A/B Street) | Rust Game Dev](https://rustgamedev.com/episodes/interview-with-dustin-a-b-street-ZCVQAek1)
* [audio] [Rust Servers, Services, and Apps with Prabhu Eshwarla](https://rustacean-station.org/episode/061-prabhu-eshwarla/)
* [videos] [RustFest LATAM 2022 [ESP]](https://www.youtube.com/playlist?list=PL85XCvVPmGQg4ks051r3VbgKGbV3FYUzo)
* [videos] [RustFest LATAM 2022 [ENG]](https://www.youtube.com/playlist?list=PL85XCvVPmGQiT5Ug60zPQ_t9j3dHyawcy)

## Crate of the Week

This week's crate is [lapce](https://lapce.dev), a lightning-fast powerful code editor written in Rust.

llogiq is pretty pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Speakers: RustFest Global - EMEA Edition (2022-04-23)](https://rustfest.world/news/twirf-latam-emea-announcement)
* [tauri-apps/tao - [bug] Global shortcuts are never triggered on Linux](https://github.com/tauri-apps/tao/issues/307)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

278 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-03-21..2022-03-28

* [proc-macro: stop wrapping `ident` matchers into groups](https://github.com/rust-lang/rust/pull/92472)
* [debuginfo: fix debuginfo for `Box<T>` where `T` is unsized](https://github.com/rust-lang/rust/pull/95270)
* [better errors when a Copy impl on a Struct is not self-consistent](https://github.com/rust-lang/rust/pull/94249)
* [provide suggestion for missing `>` in a type parameter list](https://github.com/rust-lang/rust/pull/94495)
* [suggest constraining param for unary ops when missing trait impl](https://github.com/rust-lang/rust/pull/95197)
* [suggest replacing a field when using the same type](https://github.com/rust-lang/rust/pull/95396)
* [tell users that `||` operators are not currently supported in let chain expressions](https://github.com/rust-lang/rust/pull/95314)
* [diagnostics: correct generic bounds with doubled colon](https://github.com/rust-lang/rust/pull/95318)
* [diagnostics: do not give `Option::as_ref` suggestion for complex match](https://github.com/rust-lang/rust/pull/95127)
* [diagnostics: do not suggest `fn foo({ <body> }`](https://github.com/rust-lang/rust/pull/95220)
* [diagnostics: suggest missing comma in bad FRU syntax](https://github.com/rust-lang/rust/pull/94939)
* [fix function pointers of different param counts unifying](https://github.com/rust-lang/chalk/pull/759)
* [change Thir to lazily create constants](https://github.com/rust-lang/rust/pull/94876)
* [fix perf issue for auto trait selection](https://github.com/rust-lang/rust/pull/95333)
* [overlap inherent impls](https://github.com/rust-lang/rust/pull/95082)
* [more macro expansion optimizations](https://github.com/rust-lang/rust/pull/95259)
* [ignore doc comments in a declarative macro matcher](https://github.com/rust-lang/rust/pull/95390)
* [allow comparing `Vec`s with different allocators using `==`](https://github.com/rust-lang/rust/pull/93755)
* [stabilize `const_ptr_offset`](https://github.com/rust-lang/rust/pull/93957)
* [impl `From<&[T; N]>` and `From<&mut [T; N]>` for `Vec<T>`](https://github.com/rust-lang/rust/pull/95098)
* [resolve: do not build expensive suggestions if they are not actually used](https://github.com/rust-lang/rust/pull/95255)
* [codegen\_gcc: add `--release-sysroot` flag to `build.sh`](https://github.com/rust-lang/rustc_codegen_gcc/pull/143)
* [codegen\_gcc: don't pass `--target` in `cargo.sh`](https://github.com/rust-lang/rustc_codegen_gcc/pull/147)
* [clippy: check if `lhs < rhs` in modulos in `identity_op`](https://github.com/rust-lang/rust-clippy/pull/8519)
* [clippy: `map_identity` checks for needless `map_err`](https://github.com/rust-lang/rust-clippy/pull/8487)
* [clippy: `unnecessary_join` lint](https://github.com/rust-lang/rust-clippy/pull/8579)

### Rust Compiler Performance Triage

A mixed week: some minor regressions, but things overall improved for instruction counts.

Max RSS has gone up slightly over the past
[month](https://perf.rust-lang.org/?start=2022-03-01&end=2022-03-30&kind=percentfromfirst&stat=max-rss),
on the order of 0.5% regression according to benchmark summary. pnkfelix is
following up on that with rustc-perf team on
[zulip](https://rust-lang.zulipchat.com/#narrow/stream/247081-t-compiler.2Fperformance/topic/max-rss.20over.202022-03/near/277194155)

Triage done by **@pnkfelix**.
Revision range: [3ea44938..3e751467](https://perf.rust-lang.org/?start=3ea44938e21f0de8ae7d4f6399a8a30f97867c70&end=3e7514670db841a7f0d7656f3b13b1c8b2c11599&absolute=false&stat=instructions%3Au)

4 Regressions, 5 Improvements, 4 Mixed; 3 of them in rollups
63 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-03-30.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [New Rust attribute to support embedding debugger visualizers](https://github.com/rust-lang/rfcs/pull/3191)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Tracking Issue for RFC 3107: derive_default_enum](https://github.com/rust-lang/rust/issues/87517)
* [disposition: merge] [Tracking Issue for scoped threads](https://github.com/rust-lang/rust/issues/93203)
* [disposition: merge] [Tracking Issue for windows_process_extensions_raw_arg](https://github.com/rust-lang/rust/issues/92939)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Interrupt calling conventions](https://github.com/rust-lang/rfcs/pull/3246)

## Upcoming Events

Rusty Events between 2022-03-30 - 2022-04-27 🦀

### Virtual

* 2022-03-30 | London, UK | [Rust LDN](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks Mar 2022 - AeroRust Takeover**](https://www.meetup.com/Rust-London-User-Group/events/284675763/) | [**RSVP**](https://skillsmatter.com/meetups/13826-ldn-talks-march-2022-aerorust-takeover)
* 2022-03-30 | México City, MX | [Rust MX](https://www.meetup.com/Rust-MX/)
    * [**Platica Marzo 2022 - Reescribir o no reescribir aplicaciones en Rust**](https://www.meetup.com/Rust-MX/events/284560362/)
* 2022-03-30 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcfbnc/)
* 2022-03-31 | Köln, DE | [Shop Apotheke Europe](https://www.meetup.com/shop-apotheke-europe/)
    * [**Remote Technology Summit with Kent C. Dodds, Kyle Simpson and Debbie O'Brien -  Web assembly with Rust, Daniel Nehrig, Expert, Software Engineering, SHOP APOTHEKE EUROPE**](https://www.meetup.com/shop-apotheke-europe/events/284819348/)
* 2022-04-04 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Mob Programming: Rome Tools**](https://www.meetup.com/RustPhilly/events/bgqgtsydcgbgb/)
* 2022-04-05 | Austin, TX, US | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge*)
    * [**Monthly WasmEdge Community Meeting #7 - WasmEdge Rust SDK presentation by Sam**](https://www.meetup.com/webassembly-and-wasmedge/events/zzdnrsydcgbhb/)
* 2022-04-05 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydcgbhb/)
* 2022-04-05 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcgbhb/)
* 2022-04-05 | Denver, CO, US | [Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/)
    * [**Using Futures to wrap an unsafe USB API to play audio directly - with live stream**](https://www.meetup.com/Rust-Boulder-Denver/events/284371995/)
* 2022-04-06 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydcgbjb/)
* 2022-04-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcgbjb/)
* 2022-04-06 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qnxdtsydcgbjb/)
* 2022-04-07 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #12**](https://www.meetup.com/rust-noris/events/zgfnssydcgbsb/)
* 2022-04-11 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Mob Programming: Rome Tools**](https://www.meetup.com/RustPhilly/events/bgqgtsydcgbpb/)
* 2022-04-11 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly Meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydcgbqb/)
* 2022-04-12 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcgbqb/)
* 2022-04-12 | Saarbrücken, DE | [Rust-Saar](https://www.meetup.com/Rust-Saar/)
    * [**Meetup: 20u16**](https://www.meetup.com/Rust-Saar/events/284753673/)
* 2022-04-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcgbrb/)
* 2022-04-13 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydcgbrb/)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/283790509/)
* 2022-04-14 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydcgbsb/)
* 2022-04-18 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Mob Programming: Rome Tools**](https://www.meetup.com/RustPhilly/events/bgqgtsydcgbxb/)
* 2022-04-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydcgbzb/)
* 2022-04-20 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust April 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/284705301/)
* 2022-04-20 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcgbbc/)
* 2022-04-23 | Various - EMEA | [Rustfest](https://rustfest.world/)
    * [**Rust EMEA Conference**](https://rustfest.world/news/twirf-latam-emea-announcement)
* 2022-04-27 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcgbkc/)

### Europe
* 2022-03-30 | London, UK | [Rust LDN](https://www.meetup.com/Rust-London-User-Group/)
    * [**Aero Rust takeover!**](https://skillsmatter.com/meetups/13826-ldn-talks-march-2022-aerorust-takeover)
* 2022-04-06 | Amsterdam, NL | [Rust Developers Amsterdam Group, Part of Rust Amsterdam Network](https://www.meetup.com/rust-amsterdam-group/) | [Alt link](https://www.meetup.com/pro/rust-amsterdam-network/)
    * [**Rust Developer Meetup: Serverless Rust and Rust on a Pi**](https://www.meetup.com/rust-amsterdam-group/events/284647946/)
* 2022-04-06 | Bristol, UK | [Rust Bristol](https://www.meetup.com/rust-bristol/)
    * [**Rust Bristol 🦀 Kickoff (1/2) - Intro & Embedded**](https://www.meetup.com/rust-bristol/events/284703797/)
* 2022-04-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/Rust-Berlin/)
    * [**Rust and Tell - an onsite event**](https://www.meetup.com/Rust-Berlin/events/284512764/) | [**Alt Link**](https://berline.rs/2022/04/12/rust-and-tell.html)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
* 2022-04-13 | Paris, FR | [Stockly](https://www.welcometothejungle.com/fr/companies/stockly-1)
    * [**Rust Meetup in Paris - hosted by Stockly, Qovery & Meilisearch**](https://www.eventbrite.com/e/rust-meetup-in-paris-hosted-by-stockly-qovery-meilisearch-tickets-277690869867)
* 2022-04-14 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/)
    * [**Cambridge Rust Reboot 2**](https://www.meetup.com/Cambridge-Rust-Meetup/events/284505361/)
* 2022-04-19 | Bristol, UK | [Rust Bristol](https://www.meetup.com/rust-bristol/)
    * [**Rust Bristol 🦀 Kickoff (2/2)**](https://www.meetup.com/rust-bristol/events/284704573/)

### North America
* 2022-04-05 | Denver, CO, US | [Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/)
    * [**Using Futures to wrap an unsafe USB API to play audio directly - with live stream**](https://www.meetup.com/Rust-Boulder-Denver/events/284371995/)
* 2022-04-13 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcgbrb/)
* 2022-04-14 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcgbsb/)
* 2022-04-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcgbzb/)

### Oceania
* 2022-04-21 | Melbourne, AUS | [Rust Melbourne](https://www.meetup.com/Rust-Melbourne/)
    * [**Rust Melbourne is back!**](https://www.meetup.com/Rust-Melbourne/events/284327357/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**PlayStation Global**

* [Senior Software Engineer (Aliso Viejo, CA, US)](https://boards.greenhouse.io/sonyinteractiveentertainmentglobal/jobs/4241782004)

**harrison.ai**

* [Software Engineer (Sydney, AU / Remote, AU)](https://jobs.lever.co/harrison/3fa6cc4e-acd4-4e82-9ba0-224524cb4c6a)

**Parity**

* [Parachains Engineer - Common Good (Remote)](https://boards.greenhouse.io/parity/jobs/4794657003)

**Enso**

* [☁ ️Senior Cloud Rust Engineer (Remote EU and US)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-cloud-developer.md)

**Nuclia**

* [Rust developer (Remote)](https://nuclia.com/careers/rust-developer/)

**Stockly**

* [Back-end developer - TechOps (Rust, gRPC, PostgreSQL) (Paris, FR, US)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-rust-grpc-postgresql_paris?q=77fd1862155dc1bff74b1adc38a04e92&o=421986&e=companies_jobs)
* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR, US)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris?q=77fd1862155dc1bff74b1adc38a04e92&o=625243&e=companies_jobs)

**Quickwit**

* [Senior Software Engineer, Rust & distributed systems - Rust (Remote)](https://quickwit.io/jobs/distributed-software-engineer)
* [Software search engineering intern - Rust (Remote)](https://quickwit.io/jobs/oss-software-search-engineering-intern)

**Tempus Ex**

* [Several full-time Rust positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US, and Remote)](https://tempus-ex.com/careers)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> All that to say that Rust does precisely this great job at decoupling some of these notions that have been, historically, quite tangled for a while; and for those used to that environment with everything muddied, it can be a bit hard to take a step back and rethink these distinctions that Rust makes.

– [Daniel H-M on rust-users](https://users.rust-lang.org/t/a-better-term-than-thread-safe/73199/4)

Thanks to [H2CO3](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1200) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/tspsnu/this_week_in_rust_436/)</small>
