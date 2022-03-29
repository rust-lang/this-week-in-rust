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

### Official

### Foundation

* [Member Spotlight: Wyliodrin](https://foundation.rust-lang.org/news/2022-03-29-member-spotlight-wyliodrin/)

### Project/Tooling Updates

* [Announcing fp-bindgen](https://fiberplane.dev/blog/announcing-fp-bindgen/)
* [Rust-Analyzer Changelog #122](https://rust-analyzer.github.io/thisweek/2022/03/28/changelog-122.html)
* [IntelliJ Rust Changelog #167](https://intellij-rust.github.io/2022/03/28/changelog-167.html)

### Observations/Thoughts

* [dyn*: can we make dyn sized?](https://smallcultfollowing.com/babysteps//blog/2022/03/29/dyn-can-we-make-dyn-sized/)
* [Oxide on My Wrist: Hubris on PineTime was the best worst idea](https://artemis.sh/2022/03/28/oxide-hubris-on-pinetime.html)
* [Self Modifying Code](https://matklad.github.io/2022/03/26/self-modifying-code.html)
* [Async destructors, async genericity and completion futures](https://sabrinajewson.org/blog/async-drop)
* [A thanks to the traits working group in 2021](https://jackh726.github.io/rust/2022/03/25/a-thanks.html)

### Rust Walkthroughs

* [Introducing "High Assurance Rust": a FREE systems software security book!](https://highassurance.rs/)
* [Advanced deserialization with Serde: Parsing Cloudformation templates](https://rtoch.com/posts/advanced-serde/)
* [My First Clippy Lint - Statistically Insignificant](https://jamesmcm.github.io/blog/2022/03/26/my-first-clippy-lint/)
* [Introducing "High Assurance Rust"](https://www.reddit.com/r/rust/comments/toq2wj/introducing_high_assurance_rust_a_free_systems/)
* [Rust for JavaScript developers: SQS batch error handling with AWS Lambda](https://dfrasca.hashnode.dev/rust-for-javascript-developers-sqs-batch-error-handling-with-aws-lambda)
* [video] [Introduction to WAGI by Rainer Stropek & Stefan Baumgartner](https://www.youtube.com/watch?v=9NDwHBjLlhQ)
* [video] [Rust for Linux by Miguel Ojeda and Wedson Almeida Filho](https://www.youtube.com/watch?v=fVEeqo40IyQ)

### Miscellaneous

* [audio] [Interview with Dustin (A/B Street) | Rust Game Dev](https://rustgamedev.com/episodes/interview-with-dustin-a-b-street-ZCVQAek1)
* [audio] [Rust Servers, Services, and Apps with Prabhu Eshwarla :: Rustacean ...](https://rustacean-station.org/episode/061-prabhu-eshwarla/)
* [videos] [RustFest LATAM 2022 [ESP]](https://www.youtube.com/playlist?list=PL85XCvVPmGQg4ks051r3VbgKGbV3FYUzo)
* [videos] [RustFest LATAM 2022 [ENG]](https://www.youtube.com/playlist?list=PL85XCvVPmGQiT5Ug60zPQ_t9j3dHyawcy)

## Crate of the Week

This week's crate is [heph](https://docs.rs/heph), an event-driven, non-blocking I/O, share-nothing actor framework.

Thanks to [Cole Lawrence](https://users.rust-lang.org/t/crate-of-the-week/2704/1045) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

287 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-03-14..2022-03-21

* [always evaluate all cfg predicate in all() and any()](https://github.com/rust-lang/rust/pull/94295)
* [stabilise `aarch64_target_feature`](https://github.com/rust-lang/rust/pull/90621)
* [implement `-Z oom=panic`](https://github.com/rust-lang/rust/pull/88098)
* [make negative coherence work when there's impl negative on super predicates](https://github.com/rust-lang/rust/pull/95039)
* [more robust fallback for `use` suggestion](https://github.com/rust-lang/rust/pull/94584)
* [suggest removing type ascription in bad parsing position](https://github.com/rust-lang/rust/pull/95104)
* [improve `unsafe` diagnostic](https://github.com/rust-lang/rust/pull/91133)
* [fix diagnostics for `#![feature(deprecated_suggestion)]`](https://github.com/rust-lang/rust/pull/94948)
* [miri: add a lot more information to SB fatal errors](https://github.com/rust-lang/miri/pull/1971)
* [miri: make backtraces work with #[global_allocator]](https://github.com/rust-lang/miri/pull/1975)
* [miri: implement SIMD float rounding functions](https://github.com/rust-lang/miri/pull/2028)
* [miri: implement SIMD square root and fused multiply-add](https://github.com/rust-lang/miri/pull/2031)
* [miri: implement SIMD bitmask intrinsics](https://github.com/rust-lang/miri/pull/2029)
* [add `#[inline]` to trivial `AsRef`/`AsMut` impls](https://github.com/rust-lang/rust/pull/94372)
* [`BTreeMap::entry`: avoid allocating if no insertion](https://github.com/rust-lang/rust/pull/92962)
* [implement `Write for Cursor<[u8; N]>`, plus `A: Allocator` cursor support](https://github.com/rust-lang/rust/pull/92663)
* [improve `expect` impl and handle `#[expect(unfulfilled_lint_expectations)]` (RFC 2383)](https://github.com/rust-lang/rust/pull/94670)
* [make `Weak::new` const](https://github.com/rust-lang/rust/pull/94991)
* [portable-simd: fix big-endian bitmasks smaller than a byte](https://github.com/rust-lang/portable-simd/pull/267)
* [libc: add support for Apple WatchOS](https://github.com/rust-lang/libc/pull/2717)
* [codegen\_gcc: fix ice in box alloc](https://github.com/rust-lang/rustc_codegen_gcc/pull/137)
* [codegen\_gcc: fix shift of unsigned integer by signed integer](https://github.com/rust-lang/rustc_codegen_gcc/pull/141)
* [codegen\_gcc: fix version of compiler_builtins to fix compilation failure](https://github.com/rust-lang/rustc_codegen_gcc/pull/139)
* [cargo: fix panic when artifact target is used for `[target.'cfg(<target>)'.dependencies`](https://github.com/rust-lang/cargo/pull/10433)
* [rustfmt: add `short_item_threshold` config option](https://github.com/rust-lang/rustfmt/pull/5228)
* [rustfmt: honor `#[rustfmt::skip::attributes(derive)]` attribute](https://github.com/rust-lang/rustfmt/pull/5271)
* [rustfmt: search for struct body span after any generic arguments](https://github.com/rust-lang/rustfmt/pull/5274)
* [clippy: add lint `cast_enum_constructor`](https://github.com/rust-lang/rust-clippy/pull/8562)
* [clippy: add `or_then_unwrap`](https://github.com/rust-lang/rust-clippy/pull/8561)
* [clippy: don't lint `ptr_arg` on `&mut Cow<_>`](https://github.com/rust-lang/rust-clippy/pull/8552)
* [clippy: don't lint `transmute_undefined_repr` when changing the type of generic params](https://github.com/rust-lang/rust-clippy/pull/8553)
* [clippy: fix `unncessary_to_owned` false positive](https://github.com/rust-lang/rust-clippy/pull/8509)
* [clippy: `unnecessary_lazy_eval` show suggestions on multiline lint](https://github.com/rust-lang/rust-clippy/pull/8543)
* [clippy: fix suggestion on `map_flatten` being cropped causing possible information loss](https://github.com/rust-lang/rust-clippy/pull/8520)
* [clippy: `match_same_arms` fix](https://github.com/rust-lang/rust-clippy/pull/8232)
* [clippy: more `transmute_undefined_repr` fixes](https://github.com/rust-lang/rust-clippy/pull/8547)
* [clippy: move `iter_with_drain` to nursery](https://github.com/rust-lang/rust-clippy/pull/8541)
* [clippy: move `try_err` to restriction](https://github.com/rust-lang/rust-clippy/pull/8544)

### Rust Compiler Performance Triage

Very quiet week for performance, with just one statistically significant change
landing in the last week. This change was a regression, though primarily in
stress tests, and was a result of a soundness fix.

Triage done by **@simulacrum**.
Revision range: [3ba1ebea..3ea4493](https://perf.rust-lang.org/?start=3ba1ebea122238d1a5c613deb1bf60ce24bd8fd8&end=3ea44938e21f0de8ae7d4f6399a8a30f97867c70&absolute=false&stat=instructions%3Au)

1 Regressions, 0 Improvements, 0 Mixed; 0 of them in rollups
37 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-03-22.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add vendor-specific suffixes to v0 mangling RFC 2603](https://github.com/rust-lang/rfcs/pull/3224)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Cargo authenticating users without sending secrets over the network](https://github.com/rust-lang/rfcs/pull/3231)
* [disposition: merge] [Add provide_any module to core](https://github.com/rust-lang/rfcs/pull/3192)
* [disposition: close] [Add the partial-closure-args RFC.](https://github.com/rust-lang/rfcs/pull/2956)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Lazy tait (Type Alias Impl Trait) take two](https://github.com/rust-lang/rust/pull/94081)
* [disposition: close] [Allow struct and enum to contain inner attrs](https://github.com/rust-lang/rust/pull/84414)
* [disposition: merge] [Fix constants not getting dropped if part of a diverging expression](https://github.com/rust-lang/rust/pull/94775)
* [disposition: merge] [impl From\<&\[T; N\]\> and From\<&mut \[T; N\]\> for Vec\<T\>](https://github.com/rust-lang/rust/pull/95098)
* [disposition: merge] [Docs: make Vec::from_raw_parts documentation less strict](https://github.com/rust-lang/rust/pull/95016)
* [disposition: merge] [Clarify that ManuallyDrop\<T\> has same layout as T](https://github.com/rust-lang/rust/pull/88375)
* [disposition: merge] [Allow comparing Vecs with different allocators using `==`](https://github.com/rust-lang/rust/pull/93755)
* [disposition: merge] [Stabilize Termination and ExitCode](https://github.com/rust-lang/rust/pull/93840)
* [disposition: merge] [Tracking Issue for cell_filter_map](https://github.com/rust-lang/rust/issues/81061)
* [disposition: merge] [Stabilize native library modifier syntax and the `whole-archive` modifier specifically](https://github.com/rust-lang/rust/pull/93901)
* [disposition: merge] [Tracking Issue for making ptr offset methods "const fn"](https://github.com/rust-lang/rust/issues/71499)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Refined trait implementations](https://github.com/rust-lang/rfcs/pull/3245)

## Upcoming Events

Rusty Events between 2022-03-23 - 2022-04-20 🦀

### Virtual

* 2022-03-23 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/284379020/)
* 2022-03-24 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**Embedded Rust reaching out--Learn how Rust can interact with the outside world**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/284627448/)
* 2022-03-24 | Linz, AU | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 20th Edition**](https://www.meetup.com/Rust-Linz/events/284584338/)
* 2022-03-29 | Berlin, DE | [Berline](https://berline.rs/)
    * [**Rust Hack and Learn Tuesday**](https://berline.rs/2022/03/29/rust-hack-and-learn.html)
* 2022-03-29 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydcfbmc/)
* 2022-03-30 | London, UK | [Rust LDN](https://www.meetup.com/Rust-London-User-Group/)
    * [**Aero Rust takover!**](https://skillsmatter.com/meetups/13826-ldn-talks-march-2022-aerorust-takeover)
* 2022-03-30 | México City, MX | [Rust MX](https://www.meetup.com/Rust-MX/)
    * [**Platica Marzo 2022 - Reescribir o no reescribir aplicaciones en Rust**](https://www.meetup.com/Rust-MX/events/284560362/)
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
* 2022-04-07 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #12**](https://www.meetup.com/rust-noris/events/zgfnssydcgbsb/)
* 2022-04-12 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcgbqb/)
* 2022-04-12 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly Meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydcgbqb/)
* 2022-04-13 | Boulder, CO | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcgbrb/)
* 2022-04-13 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydcgbrb/)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/283790509/)
* 2022-04-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydcgbzb/)
* 2022-04-20 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcgbbc/)

### Europe
* 2022-03-30 | London, UK | [Rust LDN](https://www.meetup.com/Rust-London-User-Group/)
    * [**Aero Rust takeover!**](https://skillsmatter.com/meetups/13826-ldn-talks-march-2022-aerorust-takeover)
* 2022-04-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/Rust-Berlin/)
    * [**Rust and Tell - an onsite event**](https://www.meetup.com/Rust-Berlin/events/284512764/) | [**Alt Link**](https://berline.rs/2022/04/12/rust-and-tell.html)
* 2022-04-13 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/283790509/)
* 2022-04-14 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/)
    * [**Cambridge Rust Reboot 2**](https://www.meetup.com/Cambridge-Rust-Meetup/events/284505361/)

### North America
* 2022-04-05 | Denver, CO, US | [Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/)
    * [**Using Futures to wrap an unsafe USB API to play audio directly - with live stream**](https://www.meetup.com/Rust-Boulder-Denver/events/284371995/)
* 2022-04-13 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcgbrb/)
* 2022-04-14 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcgbsb/)
* 2022-04-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcgbzb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> today I learned that `unsafe` is also a tool for people who are actively looking to implement bugs.

– [blonk on rust-users](https://users.rust-lang.org/t/difficulty-in-inventing-bugs/72963)

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1197) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
