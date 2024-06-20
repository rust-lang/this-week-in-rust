Title: This Week in Rust 552
Number: 552
Date: 2024-06-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

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

This week's crate is [yazi](https://yazi-rs.github.io), a blazing fast terminal file manager based on async I/O.

Despite a lamentable lack of suggestions, llogiq is content with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
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

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

470 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-06-11..2024-06-18

* [promote `arm64ec-pc-windows-msvc` to tier 2](https://github.com/rust-lang/rust/pull/126039)
* [edition 2024: Make `!` fall back to `!`](https://github.com/rust-lang/rust/pull/123508)
* [`E0229`: Suggest Moving Type Constraints to Type Parameter Declaration](https://github.com/rust-lang/rust/pull/126054)
* [`UniqueRc`: support allocators and `T: ?Sized`](https://github.com/rust-lang/rust/pull/126285)
* [account for existing bindings when suggesting `pin!()`](https://github.com/rust-lang/rust/pull/125684)
* [add a new concat metavar expr](https://github.com/rust-lang/rust/pull/118958)
* [avoid ICEs after reporting errors on erroneous patterns](https://github.com/rust-lang/rust/pull/126320)
* [build `libcxx-version` only when it doesn't exist](https://github.com/rust-lang/rust/pull/126472)
* [change method resolution to constrain hidden types instead of rejecting method candidates](https://github.com/rust-lang/rust/pull/123962)
* [check that alias-relate terms are WF if reporting an error in alias-relate](https://github.com/rust-lang/rust/pull/126404)
* [consistently use subtyping in method resolution](https://github.com/rust-lang/rust/pull/126128)
* [const validation: fix ICE on dangling ZST reference](https://github.com/rust-lang/rust/pull/126426)
* [const-eval: make lint scope computation consistent](https://github.com/rust-lang/rust/pull/126388)
* [coverage: arrange span extraction/refinement as a series of passes](https://github.com/rust-lang/rust/pull/126535)
* [coverage: replace the old span refiner with a single function](https://github.com/rust-lang/rust/pull/126294)
* [delegation: fix ICE on late diagnostics](https://github.com/rust-lang/rust/pull/126234)
* [delegation: fix ICE on recursive delegation](https://github.com/rust-lang/rust/pull/126236)
* [delegation: fix hygiene for `self`](https://github.com/rust-lang/rust/pull/126497)
* [detect pub structs never constructed even though they impl pub trait with assoc constants](https://github.com/rust-lang/rust/pull/126276)
* [do not ICE in privacy when type inference fails](https://github.com/rust-lang/rust/pull/126584)
* [do not define opaque types when selecting impls](https://github.com/rust-lang/rust/pull/126258)
* [ensure self-contained linker is only enabled on dev/nightly](https://github.com/rust-lang/rust/pull/126282)
* [expand list of trait implementers in E0277 when calling rustc with --verbose](https://github.com/rust-lang/rust/pull/126055)
* [for E0277 suggest adding `Result` return type for function when using QuestionMark `?` in the body](https://github.com/rust-lang/rust/pull/126187)
* [harmonize using root or leaf obligation in trait error reporting](https://github.com/rust-lang/rust/pull/126142)
* [honor `collapse_debuginfo` for statics](https://github.com/rust-lang/rust/pull/126365)
* [implement lint for obligations broken by never type fallback change](https://github.com/rust-lang/rust/pull/125289)
* [improve escaping of byte, byte str, and c str proc-macro literals](https://github.com/rust-lang/rust/pull/123769)
* [interpret: dyn trait metadata check: equate traits in a proper way](https://github.com/rust-lang/rust/pull/126232)
* [interpret: ensure we check bool/char for validity when they are used in a cast](https://github.com/rust-lang/rust/pull/126265)
* [make `ObligationEmittingRelation`s emit `Goal` rather than `Obligation`](https://github.com/rust-lang/rust/pull/126130)
* [make `storage-live.rs` robust against rustc internal changes](https://github.com/rust-lang/rust/pull/126286)
* [make suggestion to change `Fn` to `FnMut` work with methods as well](https://github.com/rust-lang/rust/pull/126226)
* [mark undetermined if target binding in current ns is not got](https://github.com/rust-lang/rust/pull/126568)
* [move `MatchAgainstFreshVars` to old solver](https://github.com/rust-lang/rust/pull/126353)
* [no uninitalized report in a pre-returned match arm](https://github.com/rust-lang/rust/pull/126295)
* [only compute `specializes` query if (min)specialization is enabled in the crate of the specializing impl](https://github.com/rust-lang/rust/pull/126139)
* [only compute vtable information during codegen](https://github.com/rust-lang/rust/pull/126505)
* [place explicit lifetime bound after generic param](https://github.com/rust-lang/rust/pull/124884)
* [point out failing never obligation for `DEPENDENCY_ON_UNIT_NEVER_TYPE_FALLBACK`](https://github.com/rust-lang/rust/pull/126367)
* [print `token::Interpolated` with token stream pretty printing](https://github.com/rust-lang/rust/pull/125174)
* [provide correct parent for nested anon const](https://github.com/rust-lang/rust/pull/126228)
* [resolve elided lifetimes in assoc const to static if no other lifetimes are in scope](https://github.com/rust-lang/rust/pull/125258)
* [safe transmute: support `Single` enums](https://github.com/rust-lang/rust/pull/126358)
* [walk into alias-eq nested goals even if normalization fails](https://github.com/rust-lang/rust/pull/125688)
* [simplify provider api to improve llvm ir](https://github.com/rust-lang/rust/pull/126242)
* [smir: merge identical Constant and ConstOperand types](https://github.com/rust-lang/rust/pull/126410)
* [spell out other trait diagnostic](https://github.com/rust-lang/rust/pull/126127)
* [spruce up the diagnostics of some early lints](https://github.com/rust-lang/rust/pull/125913)
* [tait must be constrained if in sig](https://github.com/rust-lang/rust/pull/113169)
* [unify guarantees about the default allocator](https://github.com/rust-lang/rust/pull/126266)
* [unify intrinsics body handling in StableMIR](https://github.com/rust-lang/rust/pull/126361)
* [use `Variance` glob imported variants everywhere](https://github.com/rust-lang/rust/pull/126354)
* [use a consistent way to filter out bounds instead of splitting it into three places](https://github.com/rust-lang/rust/pull/126471)
* [MIR Shl/Shr: the offset can be computed with `rem_euclid`](https://github.com/rust-lang/rust/pull/126469)
* [miri: cargo miri: add support for '--many-seeds'](https://github.com/rust-lang/miri/pull/3672)
* [miri: implement LLVM x86 SSE4.2 intrinsics](https://github.com/rust-lang/miri/pull/3622)
* [miri: show proper UB when making a too large allocation request](https://github.com/rust-lang/miri/pull/3682)
* [miri: tell people how to set miri flags](https://github.com/rust-lang/miri/pull/3683)
* [`rustc_span`: Optimize more hygiene operations using `Span::map_ctxt`](https://github.com/rust-lang/rust/pull/126543)
* [split core's PanicInfo and std's PanicInfo](https://github.com/rust-lang/rust/pull/115974)
* [remove superfluous UbChecks from `SliceIndex` methods](https://github.com/rust-lang/rust/pull/126299)
* [`std::unix::fs::link` using direct linkat call for Solaris](https://github.com/rust-lang/rust/pull/126351)
* [simplify `[T; N]::try_map` signature](https://github.com/rust-lang/rust/pull/126249)
* [follow up to splitting core's PanicInfo and std's PanicInfo](https://github.com/rust-lang/rust/pull/126322)
* [make `PathBuf` less Ok with adding UTF-16 then `into_string`](https://github.com/rust-lang/rust/pull/126305)
* [add `Option::is_none_or`](https://github.com/rust-lang/rust/pull/126328)
* [add `f16` and `f128` const eval for binary and unary operationations](https://github.com/rust-lang/rust/pull/126429)
* [add `f16` and `f128` inline ASM support for `x86` and `x86-64`](https://github.com/rust-lang/rust/pull/126417)
* [make `ptr::rotate` smaller when using `optimize_for_size`](https://github.com/rust-lang/rust/pull/125720)
* [hashbrown: improve Set Difference `size_hint` lower bound](https://github.com/rust-lang/hashbrown/pull/530)
* [hashbrown: make equivalent default feature](https://github.com/rust-lang/hashbrown/pull/532)
* [hashbrown: optimize Set `is_disjoint`](https://github.com/rust-lang/hashbrown/pull/531)
* [cargo fix: Address problems with implicit → explicit feature migration](https://github.com/rust-lang/cargo/pull/14018)
* [cargo: add assert redactions](https://github.com/rust-lang/cargo/pull/14054)
* [cargo: add local registry overlays](https://github.com/rust-lang/cargo/pull/13926)
* [cargo: change verification order during packaging](https://github.com/rust-lang/cargo/pull/14074)
* [cargo test: redact conditional compile-fail warning](https://github.com/rust-lang/cargo/pull/14064)
* [cargo: use `std::fs::absolute` instead of reimplementing it](https://github.com/rust-lang/cargo/pull/14075)
* [clippy: add MSRV for `manual_pattern_char_comparison`](https://github.com/rust-lang/rust-clippy/pull/12937)
* [clippy: add `field_scoped_visibility_modifiers` lint](https://github.com/rust-lang/rust-clippy/pull/12893)
* [clippy: add lint `manual_inspect`](https://github.com/rust-lang/rust-clippy/pull/12287)
* [clippy: add lint to check manual pattern char comparison](https://github.com/rust-lang/rust-clippy/pull/12849)
* [clippy: avoid emitting `assigning_clones` when cloned data borrows from the place to clone into](https://github.com/rust-lang/rust-clippy/pull/12756)
* [clippy: don't lint `indexing_slicing` lints on proc macros](https://github.com/rust-lang/rust-clippy/pull/12912)
* [clippy: fix ICE in `upper_case_acronyms`](https://github.com/rust-lang/rust-clippy/pull/12903)
* [clippy: handle single chars with `to_string()` for `single_char_add_str`](https://github.com/rust-lang/rust-clippy/pull/12915)
* [clippy: let `qualify_min_const_fn` deal with drop terminators](https://github.com/rust-lang/rust-clippy/pull/12681)
* [clippy: lint `manual_unwrap_or` for it let cases](https://github.com/rust-lang/rust-clippy/pull/12906)
* [clippy: normalize type aliases when checking significant drops](https://github.com/rust-lang/rust-clippy/pull/12904)
* [clippy: rework `octal_escapes`](https://github.com/rust-lang/rust-clippy/pull/12945)
* [rust-analyzer: allow choosing logical cores for num threads config](https://github.com/rust-lang/rust-analyzer/pull/17374)
* [rust-analyzer: allow rust-project.json to include arbitrary shell commands for runnables](https://github.com/rust-lang/rust-analyzer/pull/16840)
* [rust-analyzer: show type bounds from containers when hovering on functions](https://github.com/rust-lang/rust-analyzer/pull/17364)
* [rust-analyzer: fix and cleanup VSCode task building](https://github.com/rust-lang/rust-analyzer/pull/17440)
* [rust-analyzer: add a breaker to avoid infinite loops from source root cycles](https://github.com/rust-lang/rust-analyzer/pull/17412)
* [rust-analyzer: avoid doubling cargo args in runnables](https://github.com/rust-lang/rust-analyzer/pull/17407)
* [rust-analyzer: fix `HirDisplay` stackoverflow for parameter Self defaults](https://github.com/rust-lang/rust-analyzer/pull/17394)
* [rust-analyzer: fix pat fragment parsers choking on `<eoi>`](https://github.com/rust-lang/rust-analyzer/pull/17442)
* [rust-analyzer: properly prime all crate def maps in `parallel_prime_caches`](https://github.com/rust-lang/rust-analyzer/pull/17439)

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

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2024-06-19 - 2024-07-17 🦀

### Virtual

* 2024-06-12 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 8 - Asynchronous Programming**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-06-13 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 2024-06-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 2024-06-16 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Workshop: Web development in Rust using Rocket (English)**](https://www.meetup.com/code-mavens/events/301294669/)
* 2024-06-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346963/)
* 2024-06-19 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477705/)
* 2024-06-25 | Virtual (Dallas, TX, US)| [Dallas Rust User Group](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)
* 2024-06-25 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Using the Liquid template system in Rust (English)**](https://www.meetup.com/code-mavens/events/301487547/)
* 2024-06-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 2024-07-02 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 2024-07-03 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Build Web Apps with Rust and Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 2024-07-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328025/)
* 2024-07-04 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488820/)
* 2024-07-06 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-07-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346976/)
* 2024-07-10 | Virtual | [Centre for eResearch](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Research Computing With The Rust Programming Language**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)

### Asia
* 2024-06-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**June 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/june-2024-rustacean-meetup/)
* 2024-06-30 | Kyoto, JP | [Kyoto Rust](https://www.meetup.com/kyoto-rust/)
    * [**Rust Talk: Cross Platform Apps**](https://www.meetup.com/kyoto-rust/events/301499550/)
    
### Europe

* 2024-06-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/301012491/)
* 2024-06-18 | Frankfurt/Main, DE | [Rust Frankfurt Meetup](https://www.meetup.com/rust-frankfurt)
    * [**Rust Frankfurt is Back!**](https://www.meetup.com/rust-frankfurt/events/301441434/)
* 2024-06-19 - 2024-06-24 | Zürich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 2024-06-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)
* 2024-06-25 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #3**](https://www.meetup.com/rust-gdansk/events/301014697/)
* 2024-06-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288965/)
* 2024-06-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)

### North America

* 2024-06-12 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Detroit Rust Meet - Ann Arbor**](https://www.meetup.com/detroitrust/events/301387848/)
* 2024-06-13 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Crafting an Interpreter in Rust, pt. 1**](https://www.meetup.com/spokane-rust/events/300020010/)
* 2024-06-14 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Summer BBQ for Spokane's Local Tech User Groups at Saranac Pub Rooftop!**](https://www.meetup.com/spokane-rust/events/301569401/)
* 2024-06-17 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/301411625/)
* 2024-06-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)
* 2024-06-19 | Hybrid -  Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)
* 2024-06-24 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch, June 24**](https://www.meetup.com/bostonrust/events/301549722/)
* 2024-06-26 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 2024-06-27 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)
* 2024-07-05 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch, July 5**](https://www.meetup.com/bostonrust/events/301549737/)

### Oceania

* 2024-06-14 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**June 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/301311680/)
* 2024-06-20 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Full Stack Rust + Writing a compiler for fun and (no) profit**](https://www.meetup.com/rust-akl/events/301193761/)
* 2024-06-25 | Canberra, ACt, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/300749371/)

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

> If there’s a backdoor attack lurking in the crates ecosystem, then it’s lurking pretty deep at present. The popular crates that we all rely on day to day generally appear to be what they say they are.

– [Adam Harvey on his blog](https://lawngno.me/blog/2024/06/10/divine-provenance.html)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1575) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
