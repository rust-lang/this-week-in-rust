Title: This Week in Rust 676
Number: 676
Date: 2026-11-04
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters

* [Rust Trends Issue 86 - systemd Opens the Door to Rust](https://rust-trends.com/newsletter/systemd-opens-the-door-to-rust/)

### Project/Tooling Updates

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

### Rust Walkthroughs
* [Who Are You? versus Who Owns You? — What Rust Memory Safety and Wireless Addressing Have in Common](https://medium.com/@potto_94870/who-are-you-versus-who-owns-you-what-rust-memory-safety-and-wireless-addressing-have-in-common-b876a81468d5)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [tellus](https://crates.io/crates/tellus), a tokio-based actor framework that can be used without writing async code.

Thanks to [Sean Kwak](https://users.rust-lang.org/t/crate-of-the-week/2704/1708) for the partial-self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->
* [INVAR - Turn the HELLTEST hostile-input probes into committed regression tests](https://github.com/Ramesh-35/invar/issues/2

* [Plastor - Port the tracing step from Python to Rust](https://github.com/endlessGold/Plastor/issues/6)
* [Plastor - Read depth from tone bands and cross-contours (fix flat heads on real drawings)](https://github.com/endlessGold/Plastor/issues/2)
* [Plastor - CI: build and run the Blender-free checks on Linux, macOS and Windows](https://github.com/endlessGold/Plastor/issues/8)
* [Plastor - `mh score`: report watertight and Euler characteristic](https://github.com/endlessGold/Plastor/issues/10)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->
* [**FOSDEM'27 Rust Devroom](https://rust-fosdem.github.io/) | 2026-11-20 23:59 CET | Brussels, Belgium | 2027-01-31

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

597 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-10-27..2026-11-03

#### Compiler
* [GCI: check where-clauses of associated constants for well-formedness](https://github.com/rust-lang/rust/pull/165184)
* [MIR move elimination (1.5/6): Ensure ZSTs are always initialized](https://github.com/rust-lang/rust/pull/163359)
* [MIR move elimination (5/6): Storage reconstruction](https://github.com/rust-lang/rust/pull/163339)
* [check associated const binding types](https://github.com/rust-lang/rust/pull/161131)
* [destringlyfy lint infrastructure](https://github.com/rust-lang/rust/pull/164607)
* [expand an early return case in `coerce_unsized`](https://github.com/rust-lang/rust/pull/164341)
* [new solver opts for `diesel` (part 2)](https://github.com/rust-lang/rust/pull/164234)
* [new solver opts for diesel (part 3)](https://github.com/rust-lang/rust/pull/165160)
* [perf: emit field names as a concatenated string in `derive(Debug)` for structs](https://github.com/rust-lang/rust/pull/164726)

#### Library
* [add `MaybeUninit::from_ref`](https://github.com/rust-lang/rust/pull/165214)
* [add {`Receiver`, `Sender`, `SyncSender`}`::`{`is_empty,len`}](https://github.com/rust-lang/rust/pull/165248)
* [add `strict_` methods to `NonZero`](https://github.com/rust-lang/rust/pull/164848)
* [add `TrySendError::into_inner`](https://github.com/rust-lang/rust/pull/165103)
* [core: constify `resumable_breakpoint` and re-export it in `std`](https://github.com/rust-lang/rust/pull/164608)
* [ensure the display impl of uXX is small with `optimize_for_size`](https://github.com/rust-lang/rust/pull/165284)
* [implement `Duration::display_seconds`](https://github.com/rust-lang/rust/pull/165181)
* [implement `Fn` traits for safe functions of any ABI](https://github.com/rust-lang/rust/pull/160186)
* [implement `Vec::retain_unstable` ACP](https://github.com/rust-lang/rust/pull/164155)
* [implement `advance_back_by` on `Fuse`](https://github.com/rust-lang/rust/pull/165132)
* [make `IoSlice` and `IoSliceMut` methods unstably const](https://github.com/rust-lang/rust/pull/144090)
* [move lang item collection to the resolver](https://github.com/rust-lang/rust/pull/163525)
* [specialize vec `extend_with`](https://github.com/rust-lang/rust/pull/163757)
* [optimize large elements for slice rotating](https://github.com/rust-lang/rust/pull/162754)

#### Cargo
* [windows: Delay-load system DLLs that most commands don't use](https://github.com/rust-lang/cargo/pull/17623)
* [fix manifest schema generation: `TomlOptLevel` and `StringOrVec`](https://github.com/rust-lang/cargo/pull/17660)
* [warn about conflicting proc-macro manifest settings](https://github.com/rust-lang/cargo/pull/17641)

#### Rustdoc
* [add unstable option `--crate-list-heading` to customize the sidebar crate list](https://github.com/rust-lang/rust/pull/138143)
* [improve `ItemKind`](https://github.com/rust-lang/rust/pull/162916)
* [notable trait badge and popover enhancements](https://github.com/rust-lang/rust/pull/160370)

#### Rustfmt
* [have `cfg_select!` double-brace concise formatting respect `control_brace_style`](https://github.com/rust-lang/rustfmt/pull/7214)

#### Clippy
* [don't report missing safety doc for `#[target_feature(enable)]` attributed functions](https://github.com/rust-lang/rust-clippy/pull/17051)
* [fix ICE in `arbitrary_source_item_ordering`](https://github.com/rust-lang/rust-clippy/pull/17941)
* [fix `assert_is_empty` false positive with constant context](https://github.com/rust-lang/rust-clippy/pull/17950)
* [fix `borrowed_box` suggestion for trait objects to mention `.as_ref()` at call sites](https://github.com/rust-lang/rust-clippy/pull/17794)
* [fix `manual_inspect` suggestions for mutating closures](https://github.com/rust-lang/rust-clippy/pull/17741)
* [fix: signed abs rounding error in remainder identity check](https://github.com/rust-lang/rust-clippy/pull/17963)

#### Rust-Analyzer
* [fix LSP panic on unbound function when generics are present](https://github.com/rust-lang/rust-analyzer/pull/23602)
* [fix lowering of `yield` in `async gen`](https://github.com/rust-lang/rust-analyzer/pull/23586)
* [fix: suggest reference after dereferences](https://github.com/rust-lang/rust-analyzer/pull/23562)
* [panic `Expr not in lookup` due to projection types](https://github.com/rust-lang/rust-analyzer/pull/23495)
* [perf: provide `ThinMap`, a map that uses `ThinVec` when the length is small](https://github.com/rust-lang/rust-analyzer/pull/23228)

### Rust Compiler Performance Triage

This week a 0.4% increase in instructions is traded for a 33% reduction in `rmeta` size on some real benchmarks.
Even with that regression, this week's average is still very positive thanks to a refactoring around `AttrTokenStream`,
by using `Symbol`s instead of `String`s in the lint infrastructure,
and by avoiding an extra AST walk for lang item collection, among other things.
A lot of noise this week, which is mostly caused by two large rollups whose base PR was noise.

Triage done by **@JonathanBrouwer**.
Revision range: [019b59d8..4c46afa6](https://perf.rust-lang.org/?start=019b59d8101bb3e141f1b880684747b4217beeb6&end=4c46afa63688fb473618f9b3553d70b0c15a88eb&absolute=false&stat=instructions%3Au)

3 Regressions, 11 Improvements, 27 Mixed; 2 of them in rollups
195 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/87c69dbbcdc5f571ebe50dc0107c846b6a102422/triage/2026/2026-11-02.md)

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Add `required-targets` for workspace package selection](https://github.com/rust-lang/rfcs/pull/4013)
* [Update actions/deploy-pages action to v5.0.1](https://github.com/rust-lang/rfcs/pull/4026)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Properly feature gate doc(rust_logo)](https://github.com/rust-lang/rust/pull/164582)
* [core: Stabilize `abort_immediate` as `crash`](https://github.com/rust-lang/rust/pull/164731)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Add bootstrap tool to analyze compiler tests code coverage](https://github.com/rust-lang/compiler-team/issues/1047)
* [Add testing for lint machinery at runtime](https://github.com/rust-lang/compiler-team/issues/1004)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Crate list heading RFC](https://github.com/rust-lang/rfcs/pull/4027)

## Upcoming Events

Rusty Events between 2026-11-04 - 2026-12-02 🦀

### Virtual
* 2026-10-28 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Operating Systems Book Club: Paging Mechanisms**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316880405/)
* 2026-10-29 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Creating a hexadecimal editor in Rust**](https://luma.com/q8i3385k)
* 2026-11-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331277/)
* 2026-11-03 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Solving Real-World Planning Problems in Rust with SolverForge**](https://luma.com/rfbzk3ae)
* 2026-11-03 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773682/)
* 2026-11-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcpbgb/)
* 2026-11-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907780/)
* 2026-11-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254770/)
* 2026-11-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619619/)
* 2026-11-15 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Cursed Rust**](https://luma.com/omlooqv8)
* 2026-11-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563020/)
* 2026-11-17 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Miscompilation in Rust 1.97**](https://luma.com/2qfai4a8)
* 2026-11-17 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: The Harness Wore a Trenchcoat - Building a Rust MCP Server**](https://www.meetup.com/women-in-rust/events/315719313/)
* 2026-11-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcpbwb/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907781/)
* 2026-11-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254769/)

### Asia
* 2026-11-03 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust November 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**November 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europe
* 2026-10-28 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #72 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/316545936/)
* 2026-10-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 – Edition 019**](https://www.meetup.com/rust-berlin/events/317130457/)
* 2026-10-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 2026-11-01 - 2026-11-03 | Italy, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - The International Conference on Rust in Italy**](https://rustlab.it/schedule)
* 2026-11-02 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/317103436/)
* 2026-11-04 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Getting Started with the New Rust Driver for Oracle AI Database**](https://www.meetup.com/rust-dublin/events/316898325/)
* 2026-11-05 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Digital Model Trains & Rust: Bare-Metal DCC Command Station on ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)
* 2026-11-05 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Epitome of synaptic excellence**](https://www.meetup.com/oxford-rust-meetup-group/events/317158298/)
* 2026-11-12 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-11-12 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano/events/)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-language-milan/events/317094800/)
* 2026-11-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Everything Is Git, Protocols Are Just Adapters**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)
* 2026-11-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Talk Night at EIVA**](https://www.meetup.com/rust-aarhus/events/316938696/)
* 2026-11-19 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust: FFI with C++, and Intro to Async**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/317045463/)
* 2026-11-20 and 2026-11-21 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Embedded Rust mit Embassy - Workshop und Projekttag**](https://my.tuxedocomputers.com/event/embedded-rust-mit-embassy-workshop-und-projekttag-28/register)

### North America
* 2026-10-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316655403/)
* 2026-10-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, Oct 31**](https://www.meetup.com/bostonrust/events/316924288/)
* 2026-11-05 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**November Monthly Social**](https://www.meetup.com/rust-montreal/events/316919004/)
* 2026-11-05 | San Francisco, CA | [Rust LA: NVIDIA](https://luma.com/user/usr-eWgtha6CHSTma7v/)
    * [**NVIDIA x Lawrence Harvey | CUDA Rust**](https://luma.com/iqe7wng1)
* 2026-11-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**HTMX and Rust**](https://www.meetup.com/stl-rust/events/316896797/)
* 2026-11-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust November Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316886787/)
* 2026-11-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, Nov 14**](https://www.meetup.com/bostonrust/events/316924331/)
* 2026-11-17 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA November w/ Hadrian!**](https://www.meetup.com/rust-los-angeles/events/316859673/)
* 2026-11-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783989/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-18 | San Francisco, CA, US) | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Social Night**](https://luma.com/v0mxbgs8)
* 2026-11-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Winter Hill Rust Lunch, Nov 21**](https://www.meetup.com/bostonrust/events/316924366/)

### South America
* 2026-11-07 | Planalto Paulista, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP - Agora no Luma**](https://www.meetup.com/rust-sao-paulo-meetup/events/316812852/)

### Oceania:
* 2026-11-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/316907340/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Think about how your contributions may be received before you send them. This kind of thought, true thought, with real emotion and reasoning, is the only thing that you, a human, are guaranteed to be able to contribute.

– [Jubilee on rust-users](https://users.rust-lang.org/t/tips-for-your-first-pull-request/143356/3)

Thanks to [Jonas Fassbender](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1838) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
