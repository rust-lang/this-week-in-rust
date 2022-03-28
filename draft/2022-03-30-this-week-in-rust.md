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

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

* [Introducing "High Assurance Rust": a FREE systems software security book!](https://highassurance.rs/)

### Miscellaneous

## Crate of the Week

This week's crate is [lapce](https://lapce.dev), a lightning-fast powerful code editor written in Rust.

llogiq is pretty pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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

> All that to say that Rust does precisely this great job at decoupling some of these notions that have been, historically, quite tangled for a while; and for those used to that environment with everything muddied, it can be a bit hard to take a step back and rethink these distinctions that Rust makes.

– [Daniel H-M on rust-users](https://users.rust-lang.org/t/a-better-term-than-thread-safe/73199/4)

Thanks to [H2CO3](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1200) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
