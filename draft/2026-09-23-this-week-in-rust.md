Title: This Week in Rust 670
Number: 670
Date: 2026-09-23
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

* [Be alert: targeted attacks on prominent Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)
* [GitHub Actions leaking secrets when Miri output is cached](https://blog.rust-lang.org/2026/09/21/github-actions-leaking-secrets-when-miri-output-is-cached/)
* [Maintainer spotlight: Alejandra González (@blyxyas)](https://blog.rust-lang.org/inside-rust/2026/09/21/maintainer-spotlight-alejandra-gonzalez-blyxyas/)
* [Announcing a Maintainer in Residence: Scott Schafer for the Cargo team](https://blog.rust-lang.org/2026/09/22/announcing-a-maintainer-in-residence-scott-schafer-for-the-cargo-team/)

### Foundation
* [Guest Post: Rust Is Tier-1 Language at Microsoft](https://rustfoundation.org/media/guest-post-rust-is-tier-1-language-at-microsoft/)

### Newsletters

### Project/Tooling Updates

* [Fearless SIMD v1.0 is here](https://linebender.org/blog/fearless-simd-1-0/)
* [Syncing Rust GCC backend or how to test Murphy's law](https://blog.guillaume-gomez.fr/articles/2026-09-22+Syncing+Rust+GCC+backend+or+how+to+test+Murphy%27s+law)
* [Benchmarking Wild vs Mold ](https://davidlattimore.github.io/posts/2026/09/18/benchmarking-wild-vs-mold.html)

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

* [Arguing about arguments](https://steveklabnik.com/writing/arguing-about-arguments/)
* [Nine Rules for Vibe Validation of Vibe-Coded (Rust) Algorithms](https://levelup.gitconnected.com/nine-rules-for-vibe-validation-of-vibe-coded-algorithms-20db019f5583)

### Rust Walkthroughs
* [video] [Understanding Rust Ownership by Building a Zero-Copy Log Line Parser](https://www.youtube.com/watch?v=ZxxUqoUTgnA)

* [Finding Bugs](https://matklad.github.io/2026/09/19/finding-bugs.html)
* [Why datadiff matches arrays by key instead of computing tree edit distance](https://dev.to/dimanovikov/why-datadiff-matches-arrays-by-key-instead-of-computing-tree-edit-distance-2ehp)
* [video] [RustCurious lesson 10: Three Ways to Fix Any Borrowing Error](https://www.youtube.com/watch?v=dLx8usb759E)

* [Solving for faster SHA-1 collision detection](https://sam.dev/blog/faster-sha1-collision-detection)

* [Small and secure Docker images for Rust: Alpine vs Debian vs Scratch](https://kerkour.com/rust-docker)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [fastlogging-rs](https://github.com/brmmm3/fastlogging-rs), a fast logger which supports 8 different programming languages.

Thanks to [brmmm3](https://users.rust-lang.org/t/crate-of-the-week/2704/1673) for the self-suggestion!

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

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [sysknife - A successful automatic rollback renders to the operator as unknown](https://github.com/lacs-project/sysknife/issues/482)
* [sysknife - sysknife-setup --uninstall deletes .mcp.json whole, taking every other MCP server with it](https://github.com/lacs-project/sysknife/issues/480)
* [sysknife - audit export publishes request_hash, an unsalted hash over unredacted params, with no statement of its sensitivity](https://github.com/lacs-project/sysknife/issues/268)
* [Apache Iggy - Python SDK: expose consumer shutdown and offset drain timeout](https://github.com/apache/iggy/issues/4165)
* [Apache Iggy - Python SDK: expose client disconnect and shutdown lifecycle methods](https://github.com/apache/iggy/issues/4163)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

618 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-09-15..2026-09-22

#### Compiler
* [A couple polonius constraints perf improvements](https://github.com/rust-lang/rust/pull/163027)
* [AST lowering cleanups](https://github.com/rust-lang/rust/pull/162747)
* [an assortment of polonius tweaks](https://github.com/rust-lang/rust/pull/162922)
* [check `tainted_by_error` in LateLint](https://github.com/rust-lang/rust/pull/147876)
* [even more cleanups for `rustc_builtin_macros`](https://github.com/rust-lang/rust/pull/162925)
* [perf: keep the first macro syntax-context mapping inline](https://github.com/rust-lang/rust/pull/162712)
* [suggest fully qualified path on method name collision](https://github.com/rust-lang/rust/pull/153662)

#### Library
* [add `Dir::try_clone`](https://github.com/rust-lang/rust/pull/163007)
* [add case mapping fast paths for Latin-1](https://github.com/rust-lang/rust/pull/162750)
* [complex conjugate, negation and default](https://github.com/rust-lang/rust/pull/162865)
* [constify comparison traits on sliced types](https://github.com/rust-lang/rust/pull/147790)
* [implement const Iterator for Range](https://github.com/rust-lang/rust/pull/156216)
* [stabilize `CommandExt::show_window`](https://github.com/rust-lang/rust/pull/162856)
* [stabilize `feature(trim_prefix_suffix)` ({`str`,`[T]`, `Path`}`::trim_prefix` and {`str`, `[T]`}`::trim_suffix`)](https://github.com/rust-lang/rust/pull/160544)
* [stabilize `windows_process_extensions_main_thread_handle`](https://github.com/rust-lang/rust/pull/160108)

#### Cargo
* [`build-rs`: make `unstable` compile](https://github.com/rust-lang/cargo/pull/17489)
* [account for (uplift) hardlinks when calculating clean file size](https://github.com/rust-lang/cargo/pull/17485)
* [fix: return correct package specs when resolving workspace deps](https://github.com/rust-lang/cargo/pull/17469)
* [remove -Zasymmetric-token / cargo:paseto](https://github.com/rust-lang/cargo/pull/17486)
* [report the number of errors with `build.warnings='deny'`](https://github.com/rust-lang/cargo/pull/17479)

#### Rustdoc
* [Correctly handle `dyn` trait methods linking for jump to def feature](https://github.com/rust-lang/rust/pull/163036)
* [Correctly handle intra-doc links on inlined same item with different names](https://github.com/rust-lang/rust/pull/162669)

#### Clippy
* [add `must_use_without_reason` lint](https://github.com/rust-lang/rust-clippy/pull/16592)
* [fix `const_trait_impl` related infinite loop in `needless_borrows_for_generic_args`](https://github.com/rust-lang/rust-clippy/pull/17731)
* [generalize `extend_with_drain` to `VecDeque` and `BinaryHeap`](https://github.com/rust-lang/rust-clippy/pull/16778)
* [lint `suboptimal_flops` for `mul_add`, `custom_abs` and `radians` in const context](https://github.com/rust-lang/rust-clippy/pull/17631)
* [lint nested `format_args!` for uninlined args](https://github.com/rust-lang/rust-clippy/pull/16885)

#### Rust-Analyzer
* [prioritise required items in trait autocomplete](https://github.com/rust-lang/rust-analyzer/pull/23407)
* [support completions inside `cfg!()`](https://github.com/rust-lang/rust-analyzer/pull/23384)
* [support hover on cfg predicate](https://github.com/rust-lang/rust-analyzer/pull/23390)
* [complete cfg value in string](https://github.com/rust-lang/rust-analyzer/pull/23395)
* [correct order deprecated const in builtin ty](https://github.com/rust-lang/rust-analyzer/pull/23405)
* [not complete attr args when before exists args](https://github.com/rust-lang/rust-analyzer/pull/23399)
* [watch include roots recursively once, not every directory](https://github.com/rust-lang/rust-analyzer/pull/23375)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [Types FCP v2: Supertrait item shadowing stabilization](https://github.com/rust-lang/rust/issues/162130)
* [declare C and C-unwind as mutually ABI-compatible](https://github.com/rust-lang/rust/pull/161904)
* [Distinguish `repr(C)` ZSTs from others in ABI compatibility rules](https://github.com/rust-lang/rust/pull/157973)
* [Implement Default for NumBuffer](https://github.com/rust-lang/rust/pull/162536)
* [rustc: Stabilize the WebAssembly `wide-arithmetic` feature](https://github.com/rust-lang/rust/pull/160877)
* [Allow elided ('static) lifetimes in `thread_local!`](https://github.com/rust-lang/rust/pull/159564)
* [Stabilize `mem::conjure_zst`](https://github.com/rust-lang/rust/pull/161710)
* [Prevent mutating the global environment pointer in `CommandExt::exec` and opt to use execve and resolve path manually](https://github.com/rust-lang/rust/pull/157144)
* [Stabilize `debug_closure_helpers`](https://github.com/rust-lang/rust/pull/146099)
* [Additional NonZero conversions](https://github.com/rust-lang/rust/pull/129036)
* [Stabilize `funnel_shifts` (including `const`)](https://github.com/rust-lang/rust/pull/161015)
* [Stabilize `Result::into_{ok,err}`](https://github.com/rust-lang/rust/pull/161712)
* [windows: stabilise inherit_handles](https://github.com/rust-lang/rust/pull/161163)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [OUT_DIR is also set when running the program](https://github.com/rust-lang/cargo/issues/17456)
* [feat(config): Add build.profile, install.profile](https://github.com/rust-lang/cargo/pull/17215)
* [fix(git)!: Default to net.git-fetch-with-cli if git is present](https://github.com/rust-lang/cargo/pull/17329)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Create a new tier 3 target: `wasm32-webp2`](https://github.com/rust-lang/compiler-team/issues/1037)
* [Test wasm in CI with threads & unwinding](https://github.com/rust-lang/compiler-team/issues/1039)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Formulate a `trusted-contributors` marker team](https://github.com/rust-lang/leadership-council/issues/325)
* [Participation in Outreachy Dec 2026 (dedication of funds)](https://github.com/rust-lang/leadership-council/issues/320)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-09-23 - 2026-10-21 🦀

### Virtual
* 2026-09-24 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315907979/)
* 2026-09-24 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Rust Cells — Interior Mutability from Rust Core to Tock OS**](https://www.meetup.com/charlottesville-rust-meetup/events/316460694/)
* 2026-09-29 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/315691730/)
* 2026-09-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Segmentation and Introduction to Paging**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316486941/)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yqxvguts)
* 2026-10-04 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316134009/)
* 2026-10-06 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773044/)
* 2026-10-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcnbkb/)
* 2026-10-08 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/315907995/)
* 2026-10-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619617/)
* 2026-10-10 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto)
    * [**[BEZPŁATNIE] Programowanie w języku Rust**](https://www.meetup.com/stacja-it-trojmiasto/events/316381946/)
* 2026-10-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254772/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-18 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563013/)
* 2026-10-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/fhvsztyjcnbbc/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)

### Asia
* 2026-09-23 | Maharashtra, IN | [Rust Pune](https://hasgeek.com/rustpune)
    * [**Exploring standard traits in Rust**](https://hasgeek.com/rustpune/exploring-standard-traits-in-rust/)

### Europe
* 2026-09-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at SkyTEM**](https://www.meetup.com/rust-aarhus/events/316236528/)
* 2026-09-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ BlockTech**](https://www.meetup.com/rust-amsterdam-group/events/316162802/)
* 2026-09-24 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**AI Agentic Coding**](https://www.meetup.com/rust-rhein-main/events/316328297/)
* 2026-09-24 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**Rust London, Lloyds Banking Group, with Luca Palmieri & Mainmatter**](https://www.meetup.com/rust-london-user-group/events/316560409/)
* 2026-09-25 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/316610395/)
* 2026-09-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #30**](https://www.meetup.com/stockholm-rust/events/316570423/)
* 2026-09-28 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #21: Maximilian Grauvogl & Marcel Fink - From Bits to Bugs: A Rust generator for SUIT Manifests and Structure-Aware Parser Fuzzing**](https://rust-augsburg.github.io/meetup/Meetup_21.html)
* 2026-09-29 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester September Code Night**](https://www.meetup.com/rust-manchester/events/316200964/)
* 2026-09-29 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Why Rust is not really OOP?**](https://www.meetup.com/rust-language-milan/events/316654592/)
* 2026-09-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-09-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/316661690/)
* 2026-10-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)
* 2026-10-08 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-10 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22nd bcnrust session**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-20 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)

### North America
* 2026-09-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyjcmbfc/)
* 2026-09-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316404827/)
* 2026-09-24 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539333/)
* 2026-09-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Rust Lunch, Sep 26**](https://www.meetup.com/bostonrust/events/316378817/)
* 2026-10-01 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Building a Minimal, Rootless Container in Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 2026-10-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Oct 3**](https://www.meetup.com/bostonrust/events/316378820/)
* 2026-10-08 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust October Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316319730/)
* 2026-10-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, Oct 10**](https://www.meetup.com/bostonrust/events/316378823/)
* 2026-10-14 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust w/ Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)
* 2026-10-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783988/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)

### Oceania
* 2026-09-29 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/316398052/)

### South America
* 2026-10-08 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**WebApp Ergonomics y Secretos Distribuidos.**](https://www.meetup.com/rust-argentina/events/316664266/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> operational pedantics

– [Clar Fon on rust zulip](https://rust-lang.zulipchat.com/#narrow/channel/136281-t-opsem/topic/Looping.20opsem.20into.20libs.20changes/near/626044816)

Thanks to [Jules Bertholet](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1803) for the suggestion!

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
