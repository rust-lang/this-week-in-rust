Title: This Week in Rust 456
Number: 456
Date: 2022-08-17
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

This week's crate is [cargo-pgo](https://github.com/Kobzol/cargo-pgo), a cargo subcommand to compile your code with profile-guided optimization and [BOLT](https://github.com/llvm/llvm-project/tree/main/bolt#readme) for good measure.

Thanks to [Jakub Beránek](https://users.rust-lang.org/t/crate-of-the-week/2704/1098) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

410 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-08-08..2022-08-15

* [make `\[rust\] use-lld=true` work on windows](https://github.com/rust-lang/rust/pull/100464)
* [debuginfo: generalize C++-like encoding for enums](https://github.com/rust-lang/rust/pull/98393)
* [recover from mutable variable declaration where `mut` is placed before `let`](https://github.com/rust-lang/rust/pull/100253)
* [suggest a missing semicolon before an array](https://github.com/rust-lang/rust/pull/100334)
* [suggest adding an appropriate missing pattern excluding comments](https://github.com/rust-lang/rust/pull/100305)
* [suggest const and static for global variable](https://github.com/rust-lang/rust/pull/100396)
* [suggest removing `let` if `const let` or `let const` is used](https://github.com/rust-lang/rust/pull/100115)
* [suggest removing a semicolon after impl/trait items](https://github.com/rust-lang/rust/pull/100446)
* [suggest the path separator when a dot is used on a trait](https://github.com/rust-lang/rust/pull/100367)
* [adjust span of fn argument declaration](https://github.com/rust-lang/rust/pull/100458)
* [point to generic or arg if it's the self type of unsatisfied projection predicate](https://github.com/rust-lang/rust/pull/100483)
* [do not manually craft a span pointing inside a multibyte character](https://github.com/rust-lang/rust/pull/100226)
* [argument type error improvements](https://github.com/rust-lang/rust/pull/100479)
* [set tainted errors bit before emitting coerce suggestions](https://github.com/rust-lang/rust/pull/100261)
* [iterate `generics_def_id_map` in reverse order to fix P-critical issue](https://github.com/rust-lang/rust/pull/100340)
* [miri: atomics must be mutable](https://github.com/rust-lang/miri/pull/2464)
* [make `TypeError` impl `Copy`](https://github.com/rust-lang/rust/pull/100510)
* [determine `match_has_guard` from candidates instead of looking up thir table again](https://github.com/rust-lang/rust/pull/99110)
* [optimize thread ID generation](https://github.com/rust-lang/rust/pull/100022)
* [simplify visitors](https://github.com/rust-lang/rust/pull/100392)
* [simplify `format_args` builtin macro implementation](https://github.com/rust-lang/rust/pull/100277)
* [stabilize `backtrace`](https://github.com/rust-lang/rust/pull/99573)
* [stabilize `ptr_const_cast`](https://github.com/rust-lang/rust/pull/100184)
* [replace pointer casting in `hashmap_random_keys` with safe code](https://github.com/rust-lang/rust/pull/100298)
* [add `Iterator::array_chunks` (take N+1)](https://github.com/rust-lang/rust/pull/100026)
* [optimize `next` and `nth` implementations of `Skip`](https://github.com/rust-lang/rust/pull/96350)
* [compiler-builtins: remove `c32()`  from `x86_64` `memcmp`](https://github.com/rust-lang/compiler-builtins/pull/488)
* [cargo: only override published resolver when the workspace is different](https://github.com/rust-lang/cargo/pull/10961)
* [cargo: use `std::thread::scope` to replace crossbeam](https://github.com/rust-lang/cargo/pull/10977)
* [rustdoc: don't document impossible to call default trait items on impls](https://github.com/rust-lang/rust/pull/100221)
* [rustdoc: avoid ICE in rustdoc when using `Fn` bounds](https://github.com/rust-lang/rust/pull/100205)
* [rustdoc: improve crate selection on rustdoc search results page](https://github.com/rust-lang/rust/pull/100374)
* [rustdoc: don't render impl blocks with doc comment if they only contain private items by default](https://github.com/rust-lang/rust/pull/100323)
* [rustdoc: fix handling of stripped enum variant in JSON output format](https://github.com/rust-lang/rust/pull/100582)
* [rustdoc: use a more compact encoding for implementors/trait.*.js](https://github.com/rust-lang/rust/pull/100150)
* [clippy: add lint recommending using `std::iter::once` and `std::iter::empty`](https://github.com/rust-lang/rust-clippy/pull/9187)
* [clippy: add `partialeq_to_none` lint](https://github.com/rust-lang/rust-clippy/pull/9288)
* [clippy: extend `if_then_some_else_none` to also suggest `bool::then_some`](https://github.com/rust-lang/rust-clippy/pull/9289)
* [clippy: fix `if_let_mutex` not checking mutexes behind refs](https://github.com/rust-lang/rust-clippy/pull/9318)
* [clippy: fixes `trait_duplication_in_bounds` false positives](https://github.com/rust-lang/rust-clippy/pull/9167)
* [clippy: skip `unnecessary_to_owned` when `t != t.to_string()`](https://github.com/rust-lang/rust-clippy/pull/9329)
* [clippy: use `check_proc_macro` for `missing_const_for_fn`](https://github.com/rust-lang/rust-clippy/pull/9308)
* [rust-analyzer: do not unconditionally succeed RUSTC_WRAPPER checks when run by build scripts](https://github.com/rust-lang/rust-analyzer/pull/13010)
* [rust-analyzer: fix pattern field completions not working for unions](https://github.com/rust-lang/rust-analyzer/pull/12986)
* [rust-analyzer: move VSCode diagnostics workaround into client code](https://github.com/rust-lang/rust-analyzer/pull/13016)
* [rust-analyzer: pad empty diagnostic messages in relatedInformation as well](https://github.com/rust-lang/rust-analyzer/pull/13017)
* [rust-analyzer: recover from missing ellipsis in record literals for path expressions](https://github.com/rust-lang/rust-analyzer/pull/12987)
* [rust-analyzer: remove imports that are also in edition 2021's prelude](https://github.com/rust-lang/rust-analyzer/pull/12981)
* [rust-analyzer: fix incorrect type mismatch with `cfg_if!` and other macros in expression position](https://github.com/rust-lang/rust-analyzer/pull/13027)
* [rust-analyzer: infer byte string pattern as `&\[u8\]` when matched against slices](https://github.com/rust-lang/rust-analyzer/pull/12992)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-08-17 - 2022-09-14 🦀

### Virtual

* 2022-08-10 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydclbnb/)
* 2022-08-10 | Virtual (Brasília, BR) | [Turing Community](https://www.meetup.com/turing-community/) + [Turing Community LatAm](https://www.meetup.com/turing-community/members/?op=leaders)
    * [**Coding Dojo #2: Rust**](https://www.meetup.com/turing-community/events/287559034/)
* 2022-08-10 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**cargo expand --bin writing_proc_macros**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287450224/)
* 2022-08-10 | Virtual (Hoboken, NJ, US) | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/kpgpssydclbnb/)
* 2022-08-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydclbpb/)
* 2022-08-12 | Virtual + Tokyo, JP | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://www.reddit.com/r/rust/comments/w7bktx/2022_tokyo_and_elsewhere_rust_game_hack_event_aug/)
* 2022-08-13 | Virtual | [Rust Gamedev](https://gamedev.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://www.google.com/url?q=https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2Eop9Blil9YUWeTq472NIi)
* 2022-08-13 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287058154/)
* 2022-08-16 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/287286736/)
* 2022-08-16 | Virtual (Granada, ES) | [Google Developer Group Granada](https://www.meetup.com/granadagdg/)
    * [**¡Aprendamos Rust!**](https://www.meetup.com/granadagdg/events/287702154/)
* 2022-08-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydclbvb/)
* 2022-08-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**C++ Concepts vs Rust Traits**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287450947/)
* 2022-08-17 | Virtual (Hoboken, NJ, US) | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/287468144/)
* 2022-08-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)
* 2022-08-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydclbxb/)
* 2022-08-18 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #8**](https://rust-meetup.ir/2022/08/18/8th-meetup.html)
* 2022-08-18 | Virtual (Wiesbaden, DE) | [Frontend RheinMain](https://www.meetup.com/frontend_rm/)
    * [**Rust for curious developers**](https://www.meetup.com/frontend_rm/events/287713743/)
* 2022-08-20 | Virtual | [Rust Edu](https://rust-edu.org/workshop)
    * [**Rust Education Workshop 2022 (Submission deadline 2022-08-16)**](https://rust-edu.org/workshop)
* 2022-08-20 | Virtual (Hoboken, NJ, US) | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**Share Your Coding Project(s)!**](https://www.meetup.com/neighborhood-math-club/events/kbjcssydclbbc/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-27 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059968/)
* 2022-08-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/287286751/)
* 2022-08-30 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydclbnc/)
* 2022-09-01 | Virtual | [Google Open Source Live](https://www.meetup.com/google-open-source/)
    * [**Rust Day on Google Open Source Live**](https://www.meetup.com/google-open-source/events/287435626/)
* 2022-09-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together**](https://www.meetup.com/rust-noris/events/287092397/)
* 2022-09-03 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059974/)
* 2022-09-03 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 1: Tokio my-redis Tutorial**](https://www.meetup.com/rust-noris/events/287346970/)
* 2022-09-06 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/286481325/)
* 2022-09-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcmbjb/)
* 2022-09-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121715/)
* 2022-09-10 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059979/)

### Asia

* 2022-08-12 | Tokyo, JP + Virtual | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://bombercrab-rust-game-hack.peatix.com/view)

### Europe

* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)

### North America

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

> TL;DR: my claim is that Rust is attempting to **raise the abstraction** in the programming language and ultimately to join **computer science** and **software engineering** into one single discipline, an ambition that has been around since these disciplines were created.

– [Linus Walley on his blog](https://people.kernel.org/linusw/rust-in-perspective)

Thanks to [Julian Wollersberger](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1278) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
