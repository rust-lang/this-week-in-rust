Title: This Week in Rust 674
Number: 674
Date: 2026-10-21
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

* [Announcing Rust 1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)
* [Announcing the GPU notification group and Linker Working Area](https://blog.rust-lang.org/inside-rust/2026/10/20/gpu-target-notification-group/)

### Foundation

### Newsletters

* [Rust Trends Issue 85 - If It Compiles, the Agent Got It Right](https://rust-trends.com/newsletter/if-it-compiles-the-agent-got-it-right/)

### Project/Tooling Updates

* [Introducing tsrs: a Port of TypeScript to Rust](https://blog.cloudflare.com/typescript-rust-port/)

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

* [What agents mean for Rust (and vice versa)](https://smallcultfollowing.com/babysteps/blog/2026/10/14/agentic-dev-and-rust/)
* [Black-Box HashDoS: Attacking Hash Tables Without Knowing the Hash Function](https://orlp.net/blog/black-box-hashdos/)
* [2026 is an exciting year for the Rust compiler (and me)](https://yoyo-code.com/2026-is-exciting-year-for-rust-compiler-and-me/)
* [What the crates.io download count really measures: 90 days of daily data from 69 crates](https://tensorust.com/articles/crates-io-download-count/)
* [Safe self-referential data structures in kernel Rust](https://lwn.net/SubscriberLink/1099224/54c55e87a0828522/)
* [Cross-crate forward declarations in Rust](https://lwn.net/SubscriberLink/1099596/7bfb3ee846a5f102/)
* [Why tellus message handlers are synchronous](https://heikoseeberger.de/2026-10-18-why-sync-handlers/)
* [Upstreaming Tyr: Rust, DRM, and the Linux Kernel](https://www.collabora.com/news-and-blog/blog/2026/10/21/upstreaming-tyr-rust-drm-and-the-linux-kernel/)

### Rust Walkthroughs

* [I forced a speech model onto the GPU for months. The CPU was faster.](https://altunenes.github.io/posts/reuse/)
* [Superoptimizing an FPU emulator](https://purplesyringa.moe/blog/superoptimizing-an-fpu-emulator/)
* [Draw and Animate Loading Spinner with GPUI](https://hlcfan.github.io/gpui-canvas-spinner.html)
* [series] [Encoding TCP in Rust's Type System, Part 1: A Sequence Number Is Not a `u32`](https://noahkawaguchi.com/blog/typenet-post-1/)
* [series] [Running 10 million Codex CLIs on one server - LFI processes](https://kapteko.com/running-10-million-codex-clis-on-one-server-lfi-processes/)

### Research

* [An Incremental Approach to JIT Construction](https://doi.org/10.1145/3830440.3832304)
* [video] [[Scheme'26] An Incremental Approach to JIT Construction](https://www.youtube.com/watch?v=8qwXgappTWQ)

### Miscellaneous

## Crate of the Week

<!-- COTW goes here -->

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

* [ScreenCue - Add a reproducible desktop bug-report issue form](https://github.com/CrioNIK/ScreenCue/issues/1)

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
* [sysknife - README says the CLI's approval gate is sysknife approve; the CLI approves in-process after its own prompt](https://github.com/lacs-project/sysknife/issues/560)
* [sysknife - --dry-run exits 0 for a plan that breaks --max-risk or that this distro cannot run](https://github.com/lacs-project/sysknife/issues/561)
* [sysknife - Re-running sysknife-setup while the daemon runs fails with ETXTBSY and can leave mismatched binaries](https://github.com/lacs-project/sysknife/issues/563)
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Manguebyte - Put ParamSmoother under the allocation guard](https://github.com/alessonqueirozdev-hub/manguebyte/issues/22)
* [Manguebyte - Treat a NaN delay as the shortest delay in DelayLine::read and Delay::set_time](https://github.com/alessonqueirozdev-hub/manguebyte/issues/23)
* [Manguebyte - Check security advisories in the cargo deny job](https://github.com/alessonqueirozdev-hub/manguebyte/issues/24)
* [Manguebyte - Let a plugin report its tail to hosts](https://github.com/alessonqueirozdev-hub/manguebyte/issues/25)
* [Manguebyte - Add audio widgets, starting with an on-screen keyboard](https://github.com/alessonqueirozdev-hub/manguebyte/issues/7)
* [krate - Fix two warnings on the wasm32 and Windows targets (easy)](https://github.com/incyashraj/krate/issues/32)
* [krate - Linux: follow the person's dark mode and text size (medium)](https://github.com/incyashraj/krate/issues/28)
* [krate - krate pack: add a --keep-debug flag (medium)](https://github.com/incyashraj/krate/issues/29)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

A week dominated by changes of small magnitude. However, a number of
improvements landed that results in an overall average improvement of 1.4%,
which is a quite nice to see.

Triage done by **@simulacrum**.
Revision range: [7211ea59..bd4c6cc4](https://perf.rust-lang.org/?start=7211ea594cbf137ef6118b1752b959dea7cb9088&end=bd4c6cc4aeef0eb9919c6f73c1e690677e9c6b38&absolute=false&stat=instructions%3Au)

6 Regressions, 9 Improvements, 8 Mixed; 0 of them in rollups
43 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-10-19.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Fix libs nominated triagebot.toml](https://github.com/rust-lang/rfcs/pull/4022)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [core: Add `assert_matches` and `debug_assert_matches` to the next prelude](https://github.com/rust-lang/rust/pull/164328)
* [Lower #[no_mangle] to #[symbol_name]/#[link_name]](https://github.com/rust-lang/rust/pull/159374)
* [stabilize String::into_chars](https://github.com/rust-lang/rust/pull/163816)
* [stabilize {Rc, Arc}::is_unique](https://github.com/rust-lang/rust/pull/163458)
* [vec: stabilise allocators for into_iter](https://github.com/rust-lang/rust/pull/163499)
* [Don't require `unsafe` for struct and array patterns against union fields](https://github.com/rust-lang/rust/pull/161771)
* [Add a future incompat warning for #[no_link]](https://github.com/rust-lang/rust/pull/164028)
* [raise ambiguity error on attribute macro that could be tool attribute](https://github.com/rust-lang/rust/pull/162597)
* [stabilize `-Znext-solver=globally`](https://github.com/rust-lang/rust/pull/164209)
* [Stabilize `--remap-path-scope` in rustdoc and the `documentation` scope](https://github.com/rust-lang/rust/pull/163895)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Add support for `annotate-snippets`'s `no-graphics` output format](https://github.com/rust-lang/compiler-team/issues/1045)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Deprecate `std::env::{vars, args}`](https://github.com/rust-lang/rfcs/pull/4021)
* [RFC: document a strict `Future` contract that forbids snoozing](https://github.com/rust-lang/rfcs/pull/4019)
* [RFC: extern "Rust_manyregs" ABI](https://github.com/rust-lang/rfcs/pull/4018)
* [RFC: add `poll_progress` to `AsyncIterator`](https://github.com/rust-lang/rfcs/pull/4020)

## Upcoming Events

Rusty Events between 2026-10-21 - 2026-11-18 🦀

### Virtual
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Paging Mechanisms**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316880405/)
* 2026-10-22 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Rust and the GPU from Native to Web: An Introduction to `wgpu`**](https://luma.com/k1978ath)
* 2026-10-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-26 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**No Python was harmed: teaching a tiny MCU to learn as it goes**](https://luma.com/1byfc495)
* 2026-10-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Reasoning with Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)
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

### Asia
* 2026-11-03 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust November 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**November 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europe
* 2026-10-22 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/316799366/)
* 2026-10-22 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - Rust & Python interoperability**](https://www.meetup.com/rust-community-toulouse/events/316880185/)
* 2026-10-27 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night: Rust meets AI**](https://www.meetup.com/rust-aarhus/events/316796329/)
* 2026-10-27 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #89**](https://www.meetup.com/rust-paris/events/316865643/)
* 2026-10-28 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #72 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/316545936/)
* 2026-10-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 2026-11-01 - 2026-11-03 | Italy, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - The International Conference on Rust in Italy**](https://rustlab.it/schedule)
* 2026-11-04 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Getting Started with the New Rust Driver for Oracle AI Database**](https://www.meetup.com/rust-dublin/events/316898325/)
* 2026-11-05 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Digital Model Trains & Rust: Bare-Metal DCC Command Station on ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)
* 2026-11-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Everything Is Git, Protocols Are Just Adapters**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)

### North America
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Embedded Meetup**](https://luma.com/ur4pm34i)
* 2026-10-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC w/Bryan Cantrill - Code Matters: Reflections on Rust in the LLM Age**](https://www.meetup.com/rust-nyc/events/316901518/)
* 2026-10-24 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Davis Square Rust Lunch, Oct 24**](https://www.meetup.com/bostonrust/events/316924143/)
* 2026-10-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316655403/)
* 2026-10-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, Oct 31**](https://www.meetup.com/bostonrust/events/316924288/)
* 2026-11-05 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**November Monthly Social**](https://www.meetup.com/rust-montreal/events/316919004/)
* 2026-11-05 | San Francisco, CA | Rust LA: NVIDIA!
    * [**NVIDIA x Lawrence Harvey | CUDA Rust**](https://www.meetup.com/rust-montreal/events/316919004/](https://luma.com/iqe7wng1)
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

### South America
* 2026-11-07 | Planalto Paulista, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP - Agora no Luma**](https://www.meetup.com/rust-sao-paulo-meetup/events/316812852/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

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
