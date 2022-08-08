Title: This Week in Rust 455
Number: 455
Date: 2022-08-10
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

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [fang](https://github.com/ayrat555/fang) an async background processing crate.

Thanks to [Ayrat Badykov](https://users.rust-lang.org/t/crate-of-the-week/2704/1096) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

330 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-08-01..2022-08-08

* [add support for link-flavor rust-lld for iOS, tvOS and watchOS](https://github.com/rust-lang/rust/pull/98771)
* [enable function merging when opt is for size](https://github.com/rust-lang/rust/pull/100035)
* [recover `require`, `include` instead of `use` in item](https://github.com/rust-lang/rust/pull/100167)
* [recover from C++ style `enum struct`](https://github.com/rust-lang/rust/pull/99786)
* [provide suggestion on missing `let` in binding statement](https://github.com/rust-lang/rust/pull/100111)
* [improve diagnostics for `const a: = expr;`](https://github.com/rust-lang/rust/pull/100168)
* [revive suggestions for boxed trait objects instead of impl Trait](https://github.com/rust-lang/rust/pull/100019)
* [suggest a positional formatting argument instead of a captured argument](https://github.com/rust-lang/rust/pull/100058)
* [suggest adding/removing `ref` for binding patterns](https://github.com/rust-lang/rust/pull/99835)
* [warn about dead tuple struct fields](https://github.com/rust-lang/rust/pull/95977)
* [do not exclusively suggest `;` when `,` is also a choice](https://github.com/rust-lang/rust/pull/98796)
* [avoid pointing out `return` span if it has nothing to do with type error](https://github.com/rust-lang/rust/pull/100130)
* [always create elided lifetimes, even if inferred](https://github.com/rust-lang/rust/pull/99953)
* [enable unused_parens for match arms](https://github.com/rust-lang/rust/pull/100093)
* [prevent ICE for `doc_alias` on match arm, statement, expression](https://github.com/rust-lang/rust/pull/100029)
* [detect type mismatch due to loop that might never iterate](https://github.com/rust-lang/rust/pull/100094)
* [miri: add `mkstemp` shim for unix](https://github.com/rust-lang/miri/pull/2346)
* [miri: add shim for realpath on unix](https://github.com/rust-lang/miri/pull/2457)
* [miri: add support for env::home_dir](https://github.com/rust-lang/miri/pull/2467)
* [miri: also forward --manifest-path to 'cargo metadata'](https://github.com/rust-lang/miri/pull/2474)
* [miri: fix an ICE in nanosleep()](https://github.com/rust-lang/miri/pull/2466)
* [miri: implement some missing float functions](https://github.com/rust-lang/miri/pull/2469)
* [avoid invalidating the CFG in `MirPatch`](https://github.com/rust-lang/rust/pull/100087)
* [remove `fn backtrace` and replace with usages of provider API](https://github.com/rust-lang/rust/pull/99431)
* [add back `Send` and `Sync` impls on `ChunksMut` iterators](https://github.com/rust-lang/rust/pull/100023)
* [optimize `pointer::as_aligned_to`](https://github.com/rust-lang/rust/pull/100169)
* [portable SIMD: fix interleave/deinterleave for vectors with only one lane](https://github.com/rust-lang/portable-simd/pull/299)
* [codegen\_gcc: support symbol visibility](https://github.com/rust-lang/rustc_codegen_gcc/pull/203)
* [cargo: improve error message for `no such subcommand`](https://github.com/rust-lang/cargo/pull/10924)
* [rustdoc: avoid inlining modules with duplicate names](https://github.com/rust-lang/rust/pull/99738)
* [rustdoc: do not mark the contents of a skipped module as inlined](https://github.com/rust-lang/rust/pull/100207)
* [rust-analyzer: add a setting to disable comment continuation in VSCode](https://github.com/rust-lang/rust-analyzer/pull/12934)
* [rust-analyzer: add fixups for incomplete in proc-macros](https://github.com/rust-lang/rust-analyzer/pull/12937)
* [rust-analyzer: add more constructors and entry-APIs for la-arena](https://github.com/rust-lang/rust-analyzer/pull/12931)
* [rust-analyzer: add syntax fixup for while loops](https://github.com/rust-lang/rust-analyzer/pull/12880)
* [rust-analyzer: corrected order of printing op and `=`](https://github.com/rust-lang/rust-analyzer/pull/12974)
* [rust-analyzer: don't switch workspace on vfs file changes from libraries](https://github.com/rust-lang/rust-analyzer/pull/12947)
* [rust-analyzer: error Diagnostics appear in the wrong place](https://github.com/rust-lang/rust-analyzer/pull/12939)
* [rust-analyzer: fix `test_rainbow_highlighting` gate](https://github.com/rust-lang/rust-analyzer/pull/12959)
* [rust-analyzer: generate rust type from json](https://github.com/rust-lang/rust-analyzer/pull/12905)
* [rust-analyzer: more methods and traits for `la_arena::ArenaMap`](https://github.com/rust-lang/rust-analyzer/pull/12956)
* [rust-analyzer: parse range patterns in struct and slice without trailing comma](https://github.com/rust-lang/rust-analyzer/pull/12962)
* [rust-analyzer: run stable `fmt` & `cargo` through `rustup`](https://github.com/rust-lang/rust-analyzer/pull/12953)
* [rust-analyzer: use an empty expander for ignored non-attribute proc-macros](https://github.com/rust-lang/rust-analyzer/pull/12933)
* [rust-analyzer: handle operators like their trait functions in the IDE layer](https://github.com/rust-lang/rust-analyzer/pull/12948)
* [rust-analyzer: support associated values in "Generate Enum Variant" assist](https://github.com/rust-lang/rust-analyzer/pull/12837)
* [rust-analyzer: fix incorrect token pick rankings](https://github.com/rust-lang/rust-analyzer/pull/12949)
* [rust-analyzer: make `concat!` work with char](https://github.com/rust-lang/rust-analyzer/pull/12942)
* [clippy: add `elapsed_instant` lint](https://github.com/rust-lang/rust-clippy/pull/9264)
* [clippy: fix ICE when reading literals with weird proc-macro spans](https://github.com/rust-lang/rust-clippy/pull/9303)
* [clippy: fix `cast_abs_to_unsigned` with code in parens](https://github.com/rust-lang/rust-clippy/pull/9266)
* [clippy: fix suggestions for `async` closures in `redundant_closure_call`](https://github.com/rust-lang/rust-clippy/pull/9053)
* [clippy: more proc-macro detection](https://github.com/rust-lang/rust-clippy/pull/8694)
* [clippy: move `significant_drop_in_scrutinee` into `nursery`](https://github.com/rust-lang/rust-clippy/pull/9302)
* [clippy: rename `logic_bug` to `overly_complex_bool_expr`](https://github.com/rust-lang/rust-clippy/pull/9306)
* [clippy: `explicit_auto_deref` changes](https://github.com/rust-lang/rust-clippy/pull/9126)
* [clippy: add paren before '?' when suggesting deref for `clone_on_copy`](https://github.com/rust-lang/rust-clippy/pull/9282)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-08-10 - 2022-09-07 🦀

### Virtual

* 2022-08-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)
* 2022-08-05 | Virtual + Portland, OR, US | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydclbmb/)
* 2022-08-09 | Virtual (Myrtle Point, OR, US) | [#EveryoneCanContribute Cafe](https://www.meetup.com/everyonecancontribute-cafe/)
    * [**Summer Chill & Learn: including OpenTelemetry getting started with Rust**](https://www.meetup.com/everyonecancontribute-cafe/events/286609523/)
* 2022-08-10 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydclbnb/)
* 2022-08-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydclbpb/)
* 2022-08-12 | Virtual + Tokyo, JP | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://www.reddit.com/r/rust/comments/w7bktx/2022_tokyo_and_elsewhere_rust_game_hack_event_aug/)
* 2022-08-13 | Virtual | [Rust Gamedev](https://gamedev.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://www.google.com/url?q=https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2Eop9Blil9YUWeTq472NIi)
* 2022-08-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydclbvb/)
* 2022-08-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)
* 2022-08-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydclbxb/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-30 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydclbnc/)

### Asia

* 2022-08-12 | Tokyo, JP + Virtual | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://bombercrab-rust-game-hack.peatix.com/view)

### Europe

* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)

### North America

* 2022-08-05 | Portland, OR, US + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-06 | Portland, OR, US | [Rust Project Teams](https://www.rust-lang.org/governance)
    * [**RustConf 2022 PostConf Unconf**](https://www.eventbrite.com/e/rustconf-postconf-unconf-registration-373057423797) | [**Blog post**](https://blog.rust-lang.org/2022/06/28/rust-unconference.html)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)
* 2022-08-11 | Columbus, OH, US| [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydclbpb/)
* 2022-08-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydclbvb/)
* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)
* 2022-08-25 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Concurrencia & paralelismo con Rust**](https://www.meetup.com/rust-mx/events/287561814/)
* 2022-08-25 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Hello World Cargo Crates Using Github Actions with jojobyte and Food!**](https://www.meetup.com/utah-rust/events/kvrxqsydclbpb/)

### Oceania

* 2022-08-24 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-26 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**August 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/287468753/)

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Don't come empty-handed to a project saying "this could be rewritten in Rust". It's obnoxious and gives the rust community a bad name.  
> Do start the project on your own, adding Rust to the build system and converting some significant functions, and then ask the project's community for comments.

– [moltonel on /r/rust](https://www.reddit.com/r/rust/comments/wfriz3/comment/iiw49bw/)

Thanks to [zjp-CN](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1277) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
