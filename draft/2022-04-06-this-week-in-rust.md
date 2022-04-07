Title: This Week in Rust 437
Number: 437
Date: 2022-04-06
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

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [bet](https://github.com/Canop/bet), a library of binary expression trees.

Thanks to [Denys Séguret](https://users.rust-lang.org/t/crate-of-the-week/2704/1050) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

311 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-03-28..2022-04-04

* [include a header in .rlink files](https://github.com/rust-lang/rust/pull/95589)
* [make lowering pull-based](https://github.com/rust-lang/rust/pull/90204)
* [make GATs object safe under generic_associated_types_extended feature](https://github.com/rust-lang/rust/pull/94911)
* [lazy type-alias-impl-trait take two](https://github.com/rust-lang/rust/pull/94081)
* [allow large Size again](https://github.com/rust-lang/rust/pull/95456)
* [reduce max hash in raw strings from `u16` to `u8`](https://github.com/rust-lang/rust/pull/95251)
* [a new matcher representation for use in `parse_tt`](https://github.com/rust-lang/rust/pull/95555)
* [yet more `parse_tt` improvements](https://github.com/rust-lang/rust/pull/95425)
* [simplify `MatcherPos` some more](https://github.com/rust-lang/rust/pull/95509)
* [reduce unnecessary escaping in `proc_macro::Literal::character`/`string`](https://github.com/rust-lang/rust/pull/95343)
* [remember mutability in `DefKind::Static`](https://github.com/rust-lang/rust/pull/95436)
* [ast\_lowering: stop wrapping `ident` matchers into groups](https://github.com/rust-lang/rust/pull/95571)
* [convert more `DefId`s to `LocalDefId`](https://github.com/rust-lang/rust/pull/95560)
* [miri: add `-Zmiri-strict-provenance`](https://github.com/rust-lang/miri/pull/2045)
* [add error message suggestion for missing `noreturn` in naked function](https://github.com/rust-lang/rust/pull/95544)
* [add note to the move size diagnostic](https://github.com/rust-lang/rust/pull/95478)
* [add suggestion to borrow `Fn` and `FnMut` params/opaque/closures instead of move](https://github.com/rust-lang/rust/pull/95257)
* [add the `generic_associated_types_extended` feature](https://github.com/rust-lang/rust/pull/94869)
* [better suggestions for `Fn`-family trait selection errors](https://github.com/rust-lang/rust/pull/95260)
* [don't ICE when opaque types get their hidden type constrained again](https://github.com/rust-lang/rust/pull/95471)
* [don't emit non-asm contents error for naked function composed of errors](https://github.com/rust-lang/rust/pull/95553)
* [fix double drop of allocator in `IntoIter  impl of Vec`](https://github.com/rust-lang/rust/pull/95298)
* [improve method name suggestions](https://github.com/rust-lang/rust/pull/95119)
* [specialize infinite-type "insert some indirection" suggestion for Option](https://github.com/rust-lang/rust/pull/91416)
* [suggest `i += 1` when we see `i++` or `++i`](https://github.com/rust-lang/rust/pull/88672)
* [suggest borrowing when trying to coerce unsized type into `dyn Trait`](https://github.com/rust-lang/rust/pull/95609)
* [suggest wrapping patterns in enum variants](https://github.com/rust-lang/rust/pull/95386)
* [suggest wrapping single-expr blocks in square brackets](https://github.com/rust-lang/rust/pull/95293)
* [codegen\_gcc: add intrinsic translation for x86 arch](https://github.com/rust-lang/rustc_codegen_gcc/pull/151)
* [codegen\_gcc: add missing vendor intrinsics](https://github.com/rust-lang/rustc_codegen_gcc/pull/150)
* [codegen\_gcc: add support for target builtins](https://github.com/rust-lang/rustc_codegen_gcc/pull/149)
* [codegen\_gcc: feature/packed struct](https://github.com/rust-lang/rustc_codegen_gcc/pull/148)
* [codegen\_gcc: feature/simd](https://github.com/rust-lang/rustc_codegen_gcc/pull/129)
* [stabilize native library modifier syntax and the `whole-archive` modifier specifically](https://github.com/rust-lang/rust/pull/93901)
* [stabilize `windows_process_extensions_raw_arg`](https://github.com/rust-lang/rust/pull/92942)
* [stabilize `Termination` and `ExitCode`](https://github.com/rust-lang/rust/pull/93840)
* [stabilize feature `vec_retain_mut` on `Vec` and `VecDeque`](https://github.com/rust-lang/rust/pull/95491)
* [stabilize `thread::is_finished`](https://github.com/rust-lang/rust/pull/95130)
* [stabilize `total_cmp`](https://github.com/rust-lang/rust/pull/95431)
* [add `SyncUnsafeCell`](https://github.com/rust-lang/rust/pull/95438)
* [strict Provenance MVP](https://github.com/rust-lang/rust/pull/95241)
* [implement provenance preserving methods on `NonNull`](https://github.com/rust-lang/rust/pull/95556)
* [async: give predictable name to binding generated from .await expressions](https://github.com/rust-lang/rust/pull/95011)
* [portable-simd: move comparisons to traits](https://github.com/rust-lang/portable-simd/pull/265)
* [rustdoc: fix invalid DOM generation](https://github.com/rust-lang/rust/pull/95568)
* [rustdoc: fix multiline attributes handling in doctests](https://github.com/rust-lang/rust/pull/95590)
* [rustdoc: fix rustdoc attribute display](https://github.com/rust-lang/rust/pull/95613)
* [rustdoc: only show associated consts from inherent impls in sidebar](https://github.com/rust-lang/rust/pull/95475)
* [rustfmt: preserve semicolon after macro call inside foreign mod](https://github.com/rust-lang/rustfmt/pull/5282)
* [rustfmt: fix struct field formatting with doc comments present](https://github.com/rust-lang/rustfmt/pull/5217)
* [clippy: add `crate_in_macro_def` lint](https://github.com/rust-lang/rust-clippy/pull/8576)
* [clippy: do not fire `panic` in a constant environment](https://github.com/rust-lang/rust-clippy/pull/8592)
* [clippy: don't lint `cast_ptr_alignment` when used for unaligned reads and writes](https://github.com/rust-lang/rust-clippy/pull/8632)
* [clippy: don't warn int-to-char transmutes in const contexts](https://github.com/rust-lang/rust-clippy/pull/8610)
* [clippy: fix ICE for `iter_overeager_cloned`](https://github.com/rust-lang/rust-clippy/pull/8602)
* [clippy: handle relative paths in module_files lints](https://github.com/rust-lang/rust-clippy/pull/8611)
* [clippy: provide suggestion context in map_unit_fn](https://github.com/rust-lang/rust-clippy/pull/8584)
* [clippy: rework `undocumented_unsafe_blocks`](https://github.com/rust-lang/rust-clippy/pull/8450)
* [clippy: add `empty_structs_with_brackets`](https://github.com/rust-lang/rust-clippy/pull/8594)
* [clippy: single_element_loop: handle arrays for Edition 2021](https://github.com/rust-lang/rust-clippy/pull/8616)
* [cargo-bisect: add support for git tags in `--start` and `--end`](https://github.com/rust-lang/cargo-bisect-rustc/pull/147)
* [rustc-perf: use a summary table for PR comments](https://github.com/rust-lang/rustc-perf/pull/1245)

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

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I've seen similar sentiments echoed before, elsewhere. The point it's making is the same one that's argued whenever people say you should learn LISP because it'll make you a better programmer.
>
> There's no such thing as a perfectly intuitive programming language because algorithmic thinking isn't something that comes to us intuitively. That's why the first language is always the hardest.
>
> It's helpful and mind-expanding to learn new paradigms and force yourself out of old cognitive ruts. Thus, from an "improving your ability to solve problems and function as a programmer" perspective, what makes Rust difficult is valuable because it's forcing you to learn to think about problems in new ways.
>
> That's the distinction between necessary complexity and complexity due to ill-considered design. (Similar to how, in video games, there's a difference between genuine difficulty and difficulty caused by something like a crappy control scheme.)

– [Stephan Sokolow on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1207) (in our quotes thread!)

Thanks to [Christopher Durham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1208) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
