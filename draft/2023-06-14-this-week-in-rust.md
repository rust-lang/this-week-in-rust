Title: This Week in Rust 499
Number: 499
Date: 2023-06-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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

This week's crate is [mailtutan](https://github.com/mailtutan/mailtutan), a simulating SMTP server.

Thanks to [Mohsen Alizadeh](https://users.rust-lang.org/t/crate-of-the-week/2704/1204) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
* [Photos.network - FOSS photo management - Evaluate rewrite in Rust](https://github.com/photos-network/core/pull/27/)


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

346 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-06-05..2023-06-12

* [expand: prevent infinite loop in macro containing only "///"](https://github.com/rust-lang/rust/pull/112345)
* [expand: change how `#![cfg(FALSE)]` behaves on crate root](https://github.com/rust-lang/rust/pull/110141)
* [add `task::Waker::noop`](https://github.com/rust-lang/rust/pull/96875)
* [add `windows_sys` type definitions for ARM32 manually](https://github.com/rust-lang/rust/pull/112527)
* [add deprecation warning to python versions `<3.6` in x.py](https://github.com/rust-lang/rust/pull/112483)
* [add help for trying to do C-like pointer arithmetics](https://github.com/rust-lang/rust/pull/112261)
* [add new Tier-3 targets: `loongarch64-unknown-none*`](https://github.com/rust-lang/rust/pull/112310)
* [added custom risc32-imac for esp-espidf target](https://github.com/rust-lang/rust/pull/111369)
* [adjust span labels for `hidden_glob_reexports`](https://github.com/rust-lang/rust/pull/112413)
* [avoid ICE on `#![doc(test(...)]` with literal parameter](https://github.com/rust-lang/rust/pull/112081)
* [avoid passing `--cpu-features` when empty](https://github.com/rust-lang/rust/pull/112179)
* [avoid unwind across `extern "C"` in `thread_local::fast_local`](https://github.com/rust-lang/rust/pull/112292)
* [diagnostics: do not suggest type name tweaks on type-inferred closure args](https://github.com/rust-lang/rust/pull/112325)
* [do `fix_*_builtin_expr` hacks on the writeback results](https://github.com/rust-lang/rust/pull/112410)
* [don't hold the active queries lock while calling `make_query`](https://github.com/rust-lang/rust/pull/112333)
* [don't mention already-set fields in `struct` constructor missing field error](https://github.com/rust-lang/rust/pull/112323)
* [don't suggest changing `&self` and `&mut self` in function signature to be mutable when taking `&mut self` in closure](https://github.com/rust-lang/rust/pull/112019)
* [dont compute `opt_suggest_box_span` span for TAIT](https://github.com/rust-lang/rust/pull/112513)
* [emit an error when return-type-notation is used with type/const params](https://github.com/rust-lang/rust/pull/111047)
* [ensure space is inserted after keyword in `unused_delims`](https://github.com/rust-lang/rust/pull/112316)
* [fall back to bidirectional normalizes-to if no subst-relate candidate in alias-relate goal](https://github.com/rust-lang/rust/pull/112076)
* [fix ICE for while loop with assignment condition with LHS place expr](https://github.com/rust-lang/rust/pull/112392)
* [fix suggestion for matching `struct` with `..` on both ends](https://github.com/rust-lang/rust/pull/112199)
* [force all native libraries to be statically linked when linking a static binary](https://github.com/rust-lang/rust/pull/111698)
* [implement stdout streaming in `render_tests::Renderer`](https://github.com/rust-lang/rust/pull/112541)
* [improved std support for ps vita target](https://github.com/rust-lang/rust/pull/111819)
* [increase the accuracy of effective visibilities calculation](https://github.com/rust-lang/rust/pull/112426)
* [make "consider importing" consistent for macros](https://github.com/rust-lang/rust/pull/112452)
* [make GDB Python Pretty Printers loadable after spawning GDB, avoiding required `rust-gdb`](https://github.com/rust-lang/rust/pull/111962)
* [merge method, type and const object safety checks](https://github.com/rust-lang/rust/pull/112318)
* [normalize in infcx instead of globally for `Option::as_deref` suggestion](https://github.com/rust-lang/rust/pull/112303)
* [prevent emitting `missing_docs` for `pub extern crate`](https://github.com/rust-lang/rust/pull/112343)
* [private-in-public lints implementation](https://github.com/rust-lang/rust/pull/111801)
* [remember names of `cfg`-ed out items to mention them in diagnostics](https://github.com/rust-lang/rust/pull/109005)
* [remove default visitor impl in region constraint generation](https://github.com/rust-lang/rust/pull/112358)
* [removed use of iteration through a HashMap/HashSet in `rustc_incremental` and replaced with IndexMap/IndexSet](https://github.com/rust-lang/rust/pull/110040)
* [resolve vars in result from `scrape_region_constraints`](https://github.com/rust-lang/rust/pull/112196)
* [respect `RUST_BACKTRACE` for delayed bugs](https://github.com/rust-lang/rust/pull/112359)
* [structurally resolve pointee in `check_pat_lit`](https://github.com/rust-lang/rust/pull/112428)
* [suggest using `ptr::null_mut` when user provided `ptr::null` to a function expecting `ptr::null_mut`](https://github.com/rust-lang/rust/pull/112302)
* [take MIR dataflow analyses by mutable reference](https://github.com/rust-lang/rust/pull/108293)
* [uplift `clippy::cmp_nan` lint](https://github.com/rust-lang/rust/pull/111818)
* [uplift `clippy::undropped_manually_drops` lint](https://github.com/rust-lang/rust/pull/111530)
* [use `load`+`store` instead of `memcpy` for small integer arrays](https://github.com/rust-lang/rust/pull/111999)
* [write to stdout if `-` is given as output file](https://github.com/rust-lang/rust/pull/111626)
* [use 128 bits for TypeId hash](https://github.com/rust-lang/rust/pull/109953)
* [support float-like tuple indices in `offset_of!()`](https://github.com/rust-lang/rust/pull/112216)
* [codegen\_gcc: regen intrinsics with latest LLVM version](https://github.com/rust-lang/rustc_codegen_gcc/pull/279)
* [cargo: initial support for single-file packages](https://github.com/rust-lang/cargo/pull/12245)
* [cargo: disable multiplexing for some versions of curl](https://github.com/rust-lang/cargo/pull/12234)
* [cargo: fetch nested git submodules](https://github.com/rust-lang/cargo/pull/12244)
* [cargo: test: loose overly matches for git cli output](https://github.com/rust-lang/cargo/pull/12241)
* [cargo: upgrade to `gix` v0.45 for multi-round pack negotiations](https://github.com/rust-lang/cargo/pull/12236)
* [rustdoc: Fix infinite loop when retrieving impls for type alias](https://github.com/rust-lang/rust/pull/112543)
* [rustdoc: List matching impls on type aliases](https://github.com/rust-lang/rust/pull/112429)
* [rustdoc: search for slices and arrays by type with `[]`](https://github.com/rust-lang/rust/pull/111958)
* [rustfmt: recover comments between attrs and generic param](https://github.com/rust-lang/rustfmt/pull/5780)
* [rustfmt: remove rustc-workspace-hack](https://github.com/rust-lang/rustfmt/pull/5776)
* clippy: new lints:
  [`excessive_nesting`](https://github.com/rust-lang/rust-clippy/pull/10672),
  [`needless_if`](https://github.com/rust-lang/rust-clippy/pull/10921),
  [`unnecessary_literal_unwrap`](https://github.com/rust-lang/rust-clippy/pull/10358),
  [redundant type annotations](https://github.com/rust-lang/rust-clippy/pull/10570),
  [`arc_with_non_send_or_sync`](https://github.com/rust-lang/rust-clippy/pull/10898),
  [`min_ident_chars`](https://github.com/rust-lang/rust-clippy/pull/10916),
  [`large_stack_frames`](https://github.com/rust-lang/rust-clippy/pull/10827)
* [clippy: `let_with_type_underscore`: Don't emit on locals from procedural macros](https://github.com/rust-lang/rust-clippy/pull/10865)
* [clippy: `missing_fields_in_debug`: don't ICE when self type is a generic param](https://github.com/rust-lang/rust-clippy/pull/10897)
* [clippy: `redundant_closure`: special case inclusive ranges](https://github.com/rust-lang/rust-clippy/pull/10905)
* [clippy: `suspicious_else_formatting`: Don't warn if there is a comment between else and curly bracket](https://github.com/rust-lang/rust-clippy/pull/10904)
* [clippy: `type_repetition_in_bounds`: Don't lint on derived code](https://github.com/rust-lang/rust-clippy/pull/10894)
* [clippy: `unnecessary_to_owned`: check that the adjusted type matches target](https://github.com/rust-lang/rust-clippy/pull/10913)
* [clippy: `useless_vec`: lint `vec!` invocations when a slice or an array would do](https://github.com/rust-lang/rust-clippy/pull/10901)
* [clippy: `useless_vec`: lint on `vec![_]` invocations that adjust to a slice](https://github.com/rust-lang/rust-clippy/pull/10933)
* [clippy: allow disabling module inception on private modules](https://github.com/rust-lang/rust-clippy/pull/10917)
* [clippy: consider autoderef through user-defined `Deref` in `eager_or_lazy`](https://github.com/rust-lang/rust-clippy/pull/10896)
* [clippy: don't lint `as_conversions` in proc macros](https://github.com/rust-lang/rust-clippy/pull/10911)
* [clippy: extend `explicit_iter_loop` and `explicit_into_iter_loop`](https://github.com/rust-lang/rust-clippy/pull/10416)
* [clippy: fix `diverging_sub_expression` not checking body of block](https://github.com/rust-lang/rust-clippy/pull/10785)
* [clippy: fix `useless_vec` suggestion in `for _ in vec![..]`](https://github.com/rust-lang/rust-clippy/pull/10909)
* [clippy: handle exponent without digits in `numeric_literal`](https://github.com/rust-lang/rust-clippy/pull/10914)
* [clippy: ignore more pointer types in `unnecessary_cast`](https://github.com/rust-lang/rust-clippy/pull/10910)
* [clippy: ignore more type aliases in `unnecessary_cast`](https://github.com/rust-lang/rust-clippy/pull/10927)
* [clippy: make `cast_possible_wrap` work correctly for 16 bit {u,i}size](https://github.com/rust-lang/rust-clippy/pull/10564)
* [fix rust-analyzer proc macro server](https://github.com/rust-lang/rust/pull/112339)
* [rust-analyzer: add span to group](https://github.com/rust-lang/rust-analyzer/pull/14960)
* [rust-analyzer: count query entries in memory usage command](https://github.com/rust-lang/rust-analyzer/pull/15020)
* [rust-analyzer: inline const as literal](https://github.com/rust-lang/rust-analyzer/pull/14925)
* [rust-analyzer: fix panic in displaying const trait objects](https://github.com/rust-lang/rust-analyzer/pull/15019)
* [rust-analyzer: fix panic in displaying unsized structs](https://github.com/rust-lang/rust-analyzer/pull/15022)
* [rust-analyzer: deduplicate fields and types in completion](https://github.com/rust-lang/rust-analyzer/pull/15026)
* [rust-analyzer: derive source scope from syntax node to be transformed](https://github.com/rust-lang/rust-analyzer/pull/14989)
* [rust-analyzer: exclude Markdown injection grammar from .vscodeignore](https://github.com/rust-lang/rust-analyzer/pull/15032)
* [rust-analyzer: fix ci never running on nightly when proc-macro things change](https://github.com/rust-lang/rust-analyzer/pull/14994)
* [rust-analyzer: fix panic in `format_args` expansion](https://github.com/rust-lang/rust-analyzer/pull/15006)
* [rust-analyzer: fix proc-macro slow test](https://github.com/rust-lang/rust-analyzer/pull/14995)
* [rust-analyzer: implemeted lifetime transformation fot assists](https://github.com/rust-lang/rust-analyzer/pull/14875)
* [rust-analyzer: infer return type for async function in `generate_function`](https://github.com/rust-lang/rust-analyzer/pull/15012)
* [rust-analyzer: lower const params with a bad id](https://github.com/rust-lang/rust-analyzer/pull/14932)
* [rust-analyzer: only generate trait bound for associated types in field types](https://github.com/rust-lang/rust-analyzer/pull/15000)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-06-14 - 2023-07-12 🦀

### Virtual

* 2023-06-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309294)
* 2023-06-07 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/293616568)
* 2023-06-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732649)
* 2023-06-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/293014938)
* 2023-06-14 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Building Spin Locks and Channels - Rust Atomics & Locks Bookclub Chapters 4 & 5**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/293882628/)
* 2023-06-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/293014897)
* 2023-06-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/293786806)
* 2023-06-19 | Virtual (San Francisco, CA, US) | [Data Science on AWS - San Francisco, Global](https://www.meetup.com/data-science-on-aws/)
    * [**Generative AI Parameter Efficient Fine Tuning (PEFT), RLHF + Polars: "Polars, lightning-fast DataFrame library for Rust and Python", presented by Suman Debnath**](https://www.meetup.com/data-science-on-aws/events/289912375)
* 2023-06-20 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/293485510)
* 2023-06-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/jkxsctyfcjbbc/)
* 2023-06-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763484)
* 2023-06-22 | Virtual (Karlsruhe, DE) | [Karlsruhe Functional Programmers Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA)**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293937801)
* 2023-06-25 | Virtual (Auckland, NZ) | [ResBaz Aotearoa 2023](https://resbaz.auckland.ac.nz/)
    * [**Research Computing With The Rust Programming Language - Tim McNamara**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-637648623197?aff=ebdssbdestsearch)
* 2023-06-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcjbkc/)
* 2023-06-28 | Virtual (Chicago, IL, US) | [Chicago Healthcare Cloud Technology Community](https://www.meetup.com/chicago-healthcare-tech-and-ai/)
    * [**Rust for Mission-Critical AI: A Journey into Healthcare's Safest Language**](https://www.meetup.com/chicago-healthcare-tech-and-ai/events/293278396)
* 2023-07-04 | Virtual (Berlin, DE) | [Berline.rs / OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfckbgb/)
* 2023-07-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfckbgb/)
* 2023-07-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/293309295)

### Asia

* 2023-06-10 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Workshop/Hack and Learn Malaysia June 2023**](https://forms.gle/2fvbCG77HXCkWLfe6) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)
* 2023-06-10 | Pune, IN | [Rust Pune](https://www.meetup.com/rust-pune)
    * [**#1 - Meet & Greet**](https://www.meetup.com/rust-pune/events/293936676/)

### Europe

* 2023-06-08 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus meetup #2 sponsored by BRØLSTÆRK**](https://www.meetup.com/rust-aarhus/events/292865970/)
* 2023-06-08 | Paris, FR | [Stockly.ai](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-630742055467)
* 2023-06-08 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**Unsafe, Miri, SIMD - June Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/293322792/)
* 2023-06-16 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfcjbvb/)
* 2023-06-22 | Vienna, AT | [Papers We Love Vienna](https://www.meetup.com/papers-we-love-vienna/)
    * [**June: Data and Ownership in Rust**](https://www.meetup.com/papers-we-love-vienna/events/293974147)
* 2023-06-28 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake**](https://www.meetup.com/bratislava-rust-meetup-group/events/293732916)
* 2023-07-03 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Rust in the Linux Kernel - July Meetup**](https://www.meetup.com/rust-zurich/events/293322905)

### North America

* 2023-06-07 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/293730065)
* 2023-06-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Rust 1.70.0, Module System Deep Dive & Pizza**](https://www.meetup.com/utah-rust/events/293849386/)
* 2023-06-08 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Weekly leetcode group**](https://www.meetup.com/thursday-go/events/293927644)
* 2023-06-10 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfcjbnb/)
* 2023-06-15 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294032616)
* 2023-06-17 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/293825860)
* 2023-06-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/293118809)
* 2023-06-24 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfcjbgc/)
* 2023-07-01 | San Jose, CA, US | [Rust Breakfast & Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/jnmgftyfckbcb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

Alas this week remains quoteless for lack of suggestions.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
