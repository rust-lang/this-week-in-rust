Title: This Week in Rust 678
Number: 678
Date: 2026-11-18
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
* [Enabling the parallel frontend on nightly](https://blog.rust-lang.org/2026/11/16/enabling-parallel-frontend-on-nightly/)

### Foundation
* [How Policy Is Shaping Development - Tune in to the Next JetBrains | Rust Foundation Livestream](https://rustfoundation.org/media/how-policy-is-shaping-development-tune-in-to-the-next-jetbrains-rust-foundation-livestream/)

### Newsletters
* [Rust Trends Issue 87 - Rust 1.100 Ships the Never Type](https://rust-trends.com/newsletter/rust-1-100-ships-the-never-type/)

### Project/Tooling Updates
* [Slint 1.19 Released](https://slint.dev/blog/slint-1.19-released)
* [Jiff 1.0](https://burntsushi.net/jiff/)
* [Rust-Native Cross-Language ML Compiler Foundation](https://tvm.apache.org/2026/11/16/rust-native-cross-language-ml-compiler-foundation)

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts
* [How Rust Changed Me As A Developer](https://blog.startifact.com/posts/how-rust-changed-me/)
* [OpenCode Browse rewrote its browser tool in Rust: 97% less memory](https://bryarworld.com/tutorials/opencode-browse-rust-rewrite/)
* [Mutex poisoning is not for everyone](https://bertptrs.nl/2026/11/16/mutex-poisoning-is-not-for-everyone.html)
* [Rust in the Linux kernel: lines of code, unsafe and calls into C for every release since v6.1](https://rust-trends.com/linux-rust/)
* [How to Bypass Rust's Orphan Rule](https://edvong.com/bypass-orphan-rule/)
* [Most Rust projects don't bundle CA roots](https://blog.legoktm.com/2026/11/15/most-rust-projects-dont-bundle-ca-roots.html)
* [Modern cryptography is fast, so you have no excuse for continuing to use C and Assembly](https://kerkour.com/rust-fast-cryptography)

### Rust Walkthroughs
* [Foreign Function Interface (FFI) in Rust, Part 1: Unsafe Foundations](https://tasarma.github.io/posts/ffi-part1/)
* [sccache: Stop compiling the same Rust code again and again](https://korntewin-b.medium.com/sccache-stop-compiling-the-same-rust-code-again-and-again-6ab2b584b3a7)
* [series] [Encoding TCP in Rust's Type System, Part 2: Placeholder Values Are Not Our Friends](https://noahkawaguchi.com/blog/typenet-post-2/)
* [series] [15 million web servers on one VM - smoltcp, DPDK, AF_XDP](https://kapteko.com/15-million-webservers/)
* [video] [Miscompilation in Rust 1.97 with Yoni Peleg](https://www.youtube.com/watch?v=-dNsUB7QRk4)

### Research
* [Rust in Peace](https://calif.io/research/rust-in-peace)

## Crate of the Week

This week's crate is [xdgen](https://github.com/pop-os/xdgen), a crate for translation of AppStream metainfo files via fluent.

Thanks to [Sosthène Guédon](https://users.rust-lang.org/t/crate-of-the-week/2704/1720) for the suggestion!

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
<!-- * [ - ]() -->
* [AppBridge - [easy] Add CLI subcommand `appbridge delta` to inspect byte savings between two .appbridge bundles](https://github.com/Kolgrim33/appbridge/issues/1)
* [AppBridge - [medium] Support real Ed25519 asymmetric keypair generation and verification for .appbridge bundles](https://github.com/Kolgrim33/appbridge/issues/2)
* [presswork - Hardware matrix: run the IPP conformance suite against real printers](https://github.com/vindeckyy/presswork/issues/6)
* [presswork - Expand ipptool conformance matrix to remaining upstream .test files](https://github.com/vindeckyy/presswork/issues/7)
* [rightcommit - Add a minimal Local CI workflow example](https://github.com/rightcommit/rightcommit/issues/11)
* [rightcommit - Add a regression test for Local CI Makefile precedence](https://github.com/rightcommit/rightcommit/issues/13)
* [Rustidian - Make the AppImage open its demo vault on first launch](https://github.com/MasihMoafi/rustidian/issues/4)

<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->
*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

616 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-11-10..2026-11-17

#### Compiler
* [avoid hoisting uninitialized reads in EarlyOtherwiseBranch](https://github.com/rust-lang/rust/pull/164864)
* [do not suggest `as_deref` for a slice pattern on `Err`](https://github.com/rust-lang/rust/pull/165406)
* [improve index inference diagnostics](https://github.com/rust-lang/rust/pull/165979)
* [normalize the simd field type before reading its length](https://github.com/rust-lang/rust/pull/166032)
* [reflection: reflect attributes from adts, fields, and variants](https://github.com/rust-lang/rust/pull/165810)
* [some attribute related cleanups](https://github.com/rust-lang/rust/pull/165872)
* [support nested functions with `#[offload_kernel]`](https://github.com/rust-lang/rust/pull/163077)
* [use 2 parallel frontend threads by default on the nightly and dev channel](https://github.com/rust-lang/rust/pull/162848)

#### Library
* [add `allocator()` to `vec_deque::IntoIter` and `vec_deque::Drain`](https://github.com/rust-lang/rust/pull/165902)
* [alloc: unstably add `Vec::repeat` and `Vec::repeat_in`](https://github.com/rust-lang/rust/pull/164043)
* [don't overflow when rounding up huge thread stack sizes on unix](https://github.com/rust-lang/rust/pull/165746)
* [implement `core::ops::FnAddr`](https://github.com/rust-lang/rust/pull/165412)
* [stabilize `int_from_ascii`](https://github.com/rust-lang/rust/pull/163696)

#### Cargo
* [`config`: Add a generated schema file](https://github.com/rust-lang/cargo/pull/17708)
* [`config`: Iterate on config.toml's schema](https://github.com/rust-lang/cargo/pull/17714)
* [`trim-paths`: sanitize absolute source paths in fingerprint](https://github.com/rust-lang/cargo/pull/17556)
* [add support for `annotate-snippets`'s `no-graphics` output format](https://github.com/rust-lang/cargo/pull/17735)
* [compile: switch dev profile to debug=lines-and-tables-only](https://github.com/rust-lang/cargo/pull/17518)
* [reworked output mtime tracking](https://github.com/rust-lang/cargo/pull/17732)
* [test: add unordered to `pub_priv::verify_mix_cargo_feature_z`](https://github.com/rust-lang/cargo/pull/17737)
* [unified invoked.timestamp location for compilation and build script units](https://github.com/rust-lang/cargo/pull/17718)

#### Rustdoc
* [prevent inlining of foreign items through glob reexports named `_`](https://github.com/rust-lang/rust/pull/165922)
* [correctly filter deref impls on type aliases](https://github.com/rust-lang/rust/pull/166045)

#### Rustfmt
* [consistently use "Diff Check" throughout](https://github.com/rust-lang/rustfmt/pull/7251)
* [don't split link destinations when `wrap_comments` is enabled](https://github.com/rust-lang/rustfmt/pull/7197)
* [fix idempotency issue with raw strings](https://github.com/rust-lang/rustfmt/pull/6696)
* [don't leave trailing whitespace before a skipped impl item](https://github.com/rust-lang/rustfmt/pull/6979)

#### Clippy
* [`missing_errors_doc`: exempt never-like error types](https://github.com/rust-lang/rust-clippy/pull/18060)
* [extend `assert_is_empty` to cover `VecDeque`](https://github.com/rust-lang/rust-clippy/pull/17946)
* [fix `single_match`/`single_match_else` suggestion lose semicolon](https://github.com/rust-lang/rust-clippy/pull/18017)
* [fix incorrect suggestions from `unnecessary_literal_unwrap`](https://github.com/rust-lang/rust-clippy/pull/18049)

#### Rust-Analyzer
* [`.for` add ref when only ref impls into-iter](https://github.com/rust-lang/rust-analyzer/pull/23654)
* [complete `fn unsafe` after `const`](https://github.com/rust-lang/rust-analyzer/pull/23647)
* [do not complete types when trait in type anchor](https://github.com/rust-lang/rust-analyzer/pull/23649)
* [large u128 values should not const eval to negative floats](https://github.com/rust-lang/rust-analyzer/pull/23638)
* [replace `builtin #` with forced keywords](https://github.com/rust-lang/rust-analyzer/pull/23665)

### Rust Compiler Performance Triage

A generally positive week, with improvements significantly outweighing
regressions for an average of -1.1% instructions on primary benchmarks.

Triage done by **@simulacrum**.
Revision range: [54d0b7ed..6ca2b346](https://perf.rust-lang.org/?start=54d0b7edad511e84181ad50ec8413ef10f93b367&end=6ca2b34602bf2f3591ffa7359c166b6409a44b0a&absolute=false&stat=instructions%3Au)

5 Regressions, 12 Improvements, 4 Mixed; 0 of them in rollups
147 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-11-15.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Associated const underscore](https://github.com/rust-lang/rfcs/pull/3527)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [debugger: stabilize trim-paths unremap file loading](https://github.com/rust-lang/rust/pull/163910)
* [Stabilization of the `documentation` scope in `rustc`](https://github.com/rust-lang/rust/issues/163896)
* [Stabilize `char::is_case_ignorable`, `char::is_cased`, and the `titlecase` feature](https://github.com/rust-lang/rust/pull/165434)
* [std: use `_exit` for `process::exit`](https://github.com/rust-lang/rust/pull/165125)
* [Add lint against duplicate parameter names in fns without body](https://github.com/rust-lang/rust/pull/165366)
* [Stabilize unchecked `nonzero_ops`](https://github.com/rust-lang/rust/pull/164485)
* [Update `PartialEq`/`PartialOrd` guarantees for comparisons to match reality](https://github.com/rust-lang/rust/pull/164473)
* [Move lint validation to attribute parsing](https://github.com/rust-lang/rust/pull/164918)
* [parser: Feature-gate ambiguous attributes on expressions (take 2)](https://github.com/rust-lang/rust/pull/160235)
* [make conflicting_repr_hints a hard error](https://github.com/rust-lang/rust/pull/165238)
* [Make unsafe unknown diagnostic attributes an error again](https://github.com/rust-lang/rust/pull/165782)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Promote powerpc64-unknown-linux-gnuelfv2 to tier 2 with host tools](https://github.com/rust-lang/compiler-team/issues/1049)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [document that extern statics may be bigger than the declared size](https://github.com/rust-lang/reference/pull/2334)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Funding team charter](https://github.com/rust-lang/rfcs/pull/4029)

## Upcoming Events

Rusty Events between 2026-11-18 - 2026-12-16 🦀

### Virtual
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907781/)
* 2026-11-22 | Virtual (Bengaluru, India) | [Embedded Rust Discord](https://discord.com/invite/pvYY69PvyS)
    * [**Silicon Sundays 5: Making a cute robot visor with `no_std` Rust**](https://discord.gg/t9Cb2gjjq7?event=1563494405086904360)
* 2026-11-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254769/)
* 2026-11-26 | Virtual | [betterCode() Rust](https://rust.bettercode.eu/)
    * [**betterCode() Rust 2026**](https://rust.bettercode.eu/)
* 2026-11-28 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Getting Started with Game Development in Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/317399210/)
* 2026-12-01 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Cursed Rust**](https://luma.com/omlooqv8)
* 2026-12-01 | Virtual | [ERNI](https://www.eventbrite.es/o/erni-30130841744)
    * [**Memory-safe by design: How Rust strengthens security in medtech**](https://www.eventbrite.es/e/memory-safe-by-design-how-rust-strengthens-security-in-medtech-tickets-2003651953931)
* 2026-12-01 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Christmas Coding Challenge - Kick Off!**](https://www.meetup.com/women-in-rust/events/316993134/)
* 2026-12-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/317404144/)
* 2026-12-03 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907782/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)
* 2026-12-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Entities, Components, and Systems: How Bevy Games Work**](https://www.meetup.com/charlottesville-rust-meetup/events/317399385/)
* 2026-12-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331278/)
* 2026-12-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254767/)
* 2026-12-08 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/316993086/)
* 2026-12-12 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Building a Game with Rust and Bevy: Putting It All Together**](https://www.meetup.com/charlottesville-rust-meetup/events/317399435/)
* 2026-12-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/316916636/)
* 2026-12-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/315210235/)

### Asia
* 2026-12-12 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**December 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/december-2026-rustacean-meetup/)

### Europe
* 2026-11-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at EIVA**](https://www.meetup.com/rust-aarhus/events/316938696/)
* 2026-11-19 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust: FFI with C++, and Intro to Async**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/317045463/)
* 2026-11-20 - 2026-11-21 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Embedded Rust mit Embassy - Workshop und Projekttag**](https://my.tuxedocomputers.com/event/embedded-rust-mit-embassy-workshop-und-projekttag-28/register)
* 2026-11-24 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #17 @ ERNI**](https://www.meetup.com/rust-basel/events/317258865/)
* 2026-11-25 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ Reclamefolder**](https://www.meetup.com/rust-amsterdam-group/events/317381204/)
* 2026-11-25 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**On-site Rust Ratatui Workshop in Dortmund**](https://www.meetup.com/rust-dortmund/events/317393392/)
* 2026-11-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #73 sponsored by Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/317181792/)
* 2026-11-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**Rust London x Zed**](https://www.meetup.com/rust-london-user-group/events/317253958/)
* 2026-11-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/317222338/)
* 2026-11-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Lovable**](https://www.meetup.com/stockholm-rust/events/317161063/)
* 2026-11-28 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #32**](https://www.meetup.com/stockholm-rust/events/317458938/)
* 2026-12-01 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #90**](https://www.meetup.com/rust-paris/events/316865676/)
* 2026-12-02 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-rhein-main/events/317186779/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)

### North America
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-18 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Social Night**](https://luma.com/v0mxbgs8)
* 2026-11-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Winter Hill Rust Lunch, Nov 21**](https://www.meetup.com/bostonrust/events/316924366/)
* 2026-11-21 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/317401121/)
* 2026-11-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**South Station Rust Lunch, Nov 28**](https://www.meetup.com/bostonrust/events/316924412/)
* 2026-12-03 | Montréal, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**December Monthly Social**](https://www.meetup.com/rust-montreal/events/317415310/)
* 2026-12-04 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: The UnConf 2026.**](https://www.meetup.com/rust-nyc/events/317259308/)
* 2026-12-04 | New York, NY, US | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋Women in Rust x Rust NYC UnConf 2026**](https://www.meetup.com/women-in-rust/events/317383331/)
* 2026-12-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783990/)
* 2026-12-16 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/315210235/)

### Oceania
* 2026-11-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/316907340/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1xzlbqp/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> **Presenter**: One of the obvious dangers of AI, it may one day rewrite all code in some cryptic programming language no human is capable of understanding, just because AI understands it much better and finds so efficient to work with ...
>
> **Voice from the hall**: "Do you mean Rust?"

– Unknown people overheard during a meeting

Thanks to [Bourumir](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1848) for the suggestion!

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
* [alex-semenyuk](https://github.com/alex-semenyuk)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
