Title: This Week in Rust 475
Number: 475
Date: 2022-12-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

This week's crate is [scraper](https://crates.io/crates/scraper), a crate for HTML parsing and querying with CSS selectors.

Thanks to [Carlo Federico Vescovo](https://users.rust-lang.org/t/crate-of-the-week/2704/1140) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

344 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-12-19..2022-12-26

* [make LLD build forward-compatible with LLVM 16](https://github.com/rust-lang/rust/pull/106000)
* [add `-Zno-jump-tables`](https://github.com/rust-lang/rust/pull/105812)
* [allow `&..` to be parsed as let initializer](https://github.com/rust-lang/rust/pull/105701)
* [add `implied_bounds_entailment` lint](https://github.com/rust-lang/rust/pull/105575)
* [allow building std with cranelift](https://github.com/rust-lang/rust/pull/106051)
* [correct branch-protection ModFlagBehavior for Aarch64 on LLVM-15](https://github.com/rust-lang/rust/pull/105932)
* [ensure param-env is const before calling `eval_to_valtree`](https://github.com/rust-lang/rust/pull/105847)
* [fix arch flag on i686-apple-darwin](https://github.com/rust-lang/rust/pull/105904)
* [give a more helpful error for "`trimmed_def_paths` constructed"](https://github.com/rust-lang/rust/pull/106057)
* [give opaque types a better coherence error](https://github.com/rust-lang/rust/pull/106010)
* [mark `proc_macro_decls_static` as always used](https://github.com/rust-lang/rust/pull/105978)
* [re-enable `Fn` trait call notation error for non-tuple argument](https://github.com/rust-lang/rust/pull/105966)
* [suggest associated const on possible capitalization mistake](https://github.com/rust-lang/rust/pull/105843)
* [suggest remove last method call when type coerce with expected type](https://github.com/rust-lang/rust/pull/105872)
* [miri: data race spans](https://github.com/rust-lang/miri/pull/2646)
* [switch `#[track_caller]` back to a no-op unless feature gate is enabled](https://github.com/rust-lang/rust/pull/104741)
* [make `VecDeque::new` const](https://github.com/rust-lang/rust/pull/105127)
* [implement `From<bool>` for f32, f64](https://github.com/rust-lang/rust/pull/100390)
* [add `ptr::from_`{`ref`, `mut`}](https://github.com/rust-lang/rust/pull/104977)
* [abort immediately on bad `mem::zeroed/uninit`](https://github.com/rust-lang/rust/pull/105997)
* [cargo: fix: deduplicate dependencies by artifact target](https://github.com/rust-lang/cargo/pull/11478)
* [cargo: support vendoring with different revs from same git repo](https://github.com/rust-lang/cargo/pull/10690)
* [add readable rustdoc display for tvOS and watchOS](https://github.com/rust-lang/rust/pull/105933)
* [clippy: add `permissions_set_readonly_false` lint](https://github.com/rust-lang/rust-clippy/pull/10063)
* [clippy: add `size_of_ref` lint](https://github.com/rust-lang/rust-clippy/pull/10098)
* [clippy: avoid `match_wildcard_for_single_variants` on guarded wild matches](https://github.com/rust-lang/rust-clippy/pull/10056)
* [clippy: fix false positives in `needless_return` when using yeet](https://github.com/rust-lang/rust-clippy/pull/10109)
* [clippy: fix `manual_filter` false positive](https://github.com/rust-lang/rust-clippy/pull/10091)
* [clippy: fix incorrect suggestion in `suboptimal_flops`](https://github.com/rust-lang/rust-clippy/pull/10113)
* [clippy: improve `needless_borrow`, `redundant_clone`](https://github.com/rust-lang/rust-clippy/pull/9701)
* [rust-analyzer: add xtask for publishing release notes in Markdown on GitHub Releases from a changelog in AsciiDoc](https://github.com/rust-lang/rust-analyzer/pull/13771)
* [rust-analyzer: complete enum variants without parens when snippets are disabled](https://github.com/rust-lang/rust-analyzer/pull/13805)
* [rust-analyzer: add an option to hide adjustment hints outside of `unsafe` blocks and functions](https://github.com/rust-lang/rust-analyzer/pull/13817)
* [rust-analyzer: fix binding mode hints always adding parentheses to or-patterns](https://github.com/rust-lang/rust-analyzer/pull/13820)
* [rust-analyzer: completion: remove bound insert of type in trait](https://github.com/rust-lang/rust-analyzer/pull/13831)
* [rust-analyzer: calculate the `TargetDataLayout` correctly for the selected target](https://github.com/rust-lang/rust-analyzer/pull/13814)
* [rust-analyzer: correctly check for parentheses redundancy in `remove_parentheses` assist](https://github.com/rust-lang/rust-analyzer/pull/13764)
* [rust-analyzer: don't let mbe expr fragments match let exprs and inline consts](https://github.com/rust-lang/rust-analyzer/pull/13800)
* [rust-analyzer: handle lifetime variables in `CallableSig` query](https://github.com/rust-lang/rust-analyzer/pull/13840)
* [rust-analyzer: skip adjustment hints if the adjustment is identity (`T` → `T`)](https://github.com/rust-lang/rust-analyzer/pull/13806)
* [rust-analyzer: implement location link for type inlay hints](https://github.com/rust-lang/rust-analyzer/pull/13699)
* [rust-analyzer: inline all format arguments where possible](https://github.com/rust-lang/rust-analyzer/pull/13835)
* [docs.rs: URL-encode canonical URLs when they can include UTF8 characters](https://github.com/rust-lang/docs.rs/pull/1968)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-12-28 - 2023-01-25 🦀

### Virtual

* 2022-12-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Show & Tell: Tableturf**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcqbcc/)
* 2022-12-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcqbkc/)
* 2023-01-03 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/mbmxvsyfccbfb/)
* 2023-01-03 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289581074/)
* 2023-01-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfccbfb/)
* 2023-01-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfccbgb/)
* 2023-01-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfccbgb/)
* 2023-01-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Part 2: Exploring USB with Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/290122605/)
* 2023-01-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfccbnb/)
* 2023-01-11 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/) 
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/290277662/)
* 2023-01-12 | Virtual (San Francisco, CA, US; Stockholm, SE; New York, NY US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) | [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/)
    * [**Crack code interview problems in Rust - Ep. 1**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290071417/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290071415/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290071420/)
* 2023-01-16 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/)
    * [**Primeros pasos con Rust - Qué es y Configuración el entorno de desarrollo**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224512/) | [**São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224516/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224515/)
* 2023-01-17 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289581080/)
 * 2023-01-17 | Virtual (San Francisco, CA, US; São Paulo, BR, New York, NY, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
    * [**Primeros pasos con Rust - Creación del primer programa de Rust**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224517/) | [***São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224521/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
* 2023-01-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/289015967/)
 * 2023-01-18 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224523/) | [**Sao Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224522/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224525/)
* 2023-01-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfccbxb/)

### Asia

* 2022-12-29 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**December Edition - xtask, macros and low level features**](https://www.meetup.com/rust-tlv/events/290156141/)

### Europe

* 2022-12-29 | Freiburg, DE | [Arso Collective](https://arso.xyz/)
    * [**Rust Café Freiburg**](https://tacker.fr/node/10951)


### North America

* 2022-12-27 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**Atx Rustaceans Meetup**](https://www.meetup.com/atx-rustaceans/events/290064553/)
* 2023-01-05 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Interesting Title and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsydcqbdc/)

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

> Rust does best when we're ambitious

– [Niko Matsakis quoted by Yoshua Wuyts on his blog](https://blog.yoshuawuyts.com/rust-2023/#ambition)

llogiq is inordinately pleased with [his suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1343) and thanks Yoshua for clearing the quote!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
