Title: This Week in Rust 650
Number: 650
Date: 2026-05-06
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
* [Announcing Google Summer of Code 2026 selected projects](https://blog.rust-lang.org/2026/04/30/gsoc-2026-selected-projects/)

### Foundation

### Newsletters

* [Rust Trends Issue 77 - Rust Sharpens the Craft](https://rust-trends.com/newsletter/rust-sharpens-the-craft/)

### Project/Tooling Updates
* [Imgclip: A Cross-Platform CLI for Clipboard ↔ Image File Conversion](https://dev.to/alex_yan_6135f8195a1a3b01/imgclip-a-cross-platform-cli-for-clipboard-image-file-conversion-2i1l)

* [Connectors: Where AimDB Meets the Real World](https://aimdb.dev/blog/connectors-where-aimdb-meets-the-real-world)

* [rkik-nts 1.0.0: a high-level Rust Network Time Security (RFC 8915) client library](https://github.com/aguacero7/rkik-nts/releases/tag/v1.0.0)

* [unix-ancillary 0.2.2 — safe SCM_RIGHTS fd-passing for Rust](https://github.com/MohibShaikh/unix-ancillary/releases/tag/v0.2.2)

* [kache 0.2.0: zero-copy, content-addressed Rust build cache (RUSTC_WRAPPER)](https://github.com/kunobi-ninja/kache/releases/tag/v0.2.0)

### Observations/Thoughts
* [One week of view_types](https://scrabsha.dev/articles/one-week-of-view-types.html)
* [Async Rust never left the MVP state](https://tweedegolf.nl/en/blog/237/async-rust-never-left-the-mvp-state)
* [stable specialization in Rust](https://goldstein.lol/posts/stable-specialization/)
* [Your Clippy Config Should Be Stricter](https://emschwartz.me/your-clippy-config-should-be-stricter/)
* [Your Clippy Config Should Be Stricter-er](https://billylevin.dev/posts/clippy-config/)
* [The `Sync` bound nobody asked for](https://verrchu.github.io/blog/1-the-sync-bound-nobody-asked-for/)
* [Cross-platform Rust: Analyzing how WhatsApp, Signal and more are shipping Rust to billions of devices](https://kerkour.com/rust-cross-platform-apps)
* [audio] [Netstack.FM episode 37 — dial9: from black box to insight in Tokio](https://netstack.fm/#episode-37)

### Rust Walkthroughs
* [oops, cubic macro!](https://bal-e.org/blog/2026/oops-cubic-macro/)
* [video] [RustCurious lesson 7: Arrays and Slices](https://www.youtube.com/watch?v=JWfVqDEkQQw)
* [Writing Middlewares for Rust Lambda Functions](https://loige.co/writing-middlewares-for-rust-lambda-functions/)
* [Learn Error Handling in Rust By Building a TOML Config Parser](https://blog.sheerluck.dev/posts/learn-error-hanlding-in-rust/)

### Research

### Miscellaneous
* [Awesome SQLx Resources](https://github.com/szabgab/awesome-sqlx)

## Crate of the Week

This week's crate is [burn](https://github.com/tracel-ai/burn), a tensor and deep learning library.

Thanks to [Jonas](https://users.rust-lang.org/t/crate-of-the-week/2704/1604) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

<!-- If there are new CfT items this week, include:

  [Repo Name](Repo URL)
    * [<Feature name>](<Feature URL>)
        * [Testing steps](<Testing Steps URL>)

  - and make note in the item so the authors know to remove the `call-for-testing` label:
This RFC will appear in the **Call for Testing** section of the next issue (#) of This Week in Rust (TWiR).
You may remove the `call-for-testing` label.  Please feel free to leave the `call-for-testing` label in place if you would like this RFC to appear again in another issue of TWiR.

  - where `Repo Name` and `Repo URL` are one of:
[Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
[Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
[Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

  - and `Testing steps` points directly to the procedures the item wants users to exercise.

  - For all `Repo Names` with no new CfT items this week: use (removing the repos for which new
     CfT items did appear, of course)

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*
-->

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
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

504 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-04-28..2026-05-05

#### Compiler
* [canonicalize free regions from inputs as placeholders in root univ](https://github.com/rust-lang/rust/pull/155487)

#### Library
* [don't reload length in `String::push`](https://github.com/rust-lang/rust/pull/155847)

#### Cargo
* [`feat(lints)`: Add deny-by-default `text_direction_codepoint` lints](https://github.com/rust-lang/cargo/pull/16950)
* [`fix(compile)`: Where possible, hint about misplaced deps](https://github.com/rust-lang/cargo/pull/16940)
* [`fix(config): [env]` relative paths definition](https://github.com/rust-lang/cargo/pull/16957)
* [`fix(config)`: normalize included config paths](https://github.com/rust-lang/cargo/pull/16964)
* [remove curl dependency from crates-io crate](https://github.com/rust-lang/cargo/pull/16936)

#### Rustdoc
* [fix `doc_cfg` feature on reexports](https://github.com/rust-lang/rust/pull/156073)
* [preserve parent doc cfg for `macro_export` macros](https://github.com/rust-lang/rust/pull/155954)

#### Clippy
* [add a check for some followed by filter](https://github.com/rust-lang/rust-clippy/pull/15745)
* [fix `bad_bit_mask` ICE for overloaded bit ops](https://github.com/rust-lang/rust-clippy/pull/16937)
* [`needless_return_with_question_mark` trigger in async functions](https://github.com/rust-lang/rust-clippy/pull/16952)

#### Rust-Analyzer
* [`diagnostics`: add handler for E0130](https://github.com/rust-lang/rust-analyzer/pull/22197)
* [add AssocItemList `add_item` editor variant](https://github.com/rust-lang/rust-analyzer/pull/22245)
* [expand glob import on cyclic import fail](https://github.com/rust-lang/rust-analyzer/pull/22244)
* [add diagnostic for E0784](https://github.com/rust-lang/rust-analyzer/pull/22202)
* [allow renaming of elided lifetimes](https://github.com/rust-lang/rust-analyzer/pull/22178)
* [diagnose trait errors 🎉](https://github.com/rust-lang/rust-analyzer/pull/22186)
* [emit a diagnostic for `non_exhaustive struct` when constructed](https://github.com/rust-lang/rust-analyzer/pull/22193)
* [offer on if-expr with else-if for `convert_to_guarded_return`](https://github.com/rust-lang/rust-analyzer/pull/22199)
* [support if-else in value on postfix completions](https://github.com/rust-lang/rust-analyzer/pull/22222)
* [add missing exprs to visiting](https://github.com/rust-lang/rust-analyzer/pull/22214)
* [add missing solver lang items](https://github.com/rust-lang/rust-analyzer/pull/22274)
* [add semicolon after expr in stmt for `unwrap_branch`](https://github.com/rust-lang/rust-analyzer/pull/22217)
* [catch `#[rustc_reservation_impl = "reason"]`](https://github.com/rust-lang/rust-analyzer/pull/22282)
* [don't fetch diagnostics until proc-macros are loaded](https://github.com/rust-lang/rust-analyzer/pull/22272)
* [don't panic on `impl ?Sized` for `introduce_named_type_parameter`](https://github.com/rust-lang/rust-analyzer/pull/22265)
* [fix `unwrap_branch` in `match_arm`](https://github.com/rust-lang/rust-analyzer/pull/22247)
* [fix stack overflow on projection display](https://github.com/rust-lang/rust-analyzer/pull/22215)
* [handle empty expr in tuple expr](https://github.com/rust-lang/rust-analyzer/pull/22201)
* [improve `prettify_macro_expansion()`](https://github.com/rust-lang/rust-analyzer/pull/22058)
* [improve whitespaces for trait item complete](https://github.com/rust-lang/rust-analyzer/pull/22240)
* [infer the expected type as the return type for async blocks defined by async fns](https://github.com/rust-lang/rust-analyzer/pull/22275)
* [port array and ref exprs inference from rustc](https://github.com/rust-lang/rust-analyzer/pull/22271)
* [qualify .new path and no complete generic params](https://github.com/rust-lang/rust-analyzer/pull/22210)
* [remove usage of `references_error()` in upvar inference](https://github.com/rust-lang/rust-analyzer/pull/22276)
* [show the user's message for `#[must_use]`](https://github.com/rust-lang/rust-analyzer/pull/22253)
* [use `Pattern_White_Space` for whitespace handling](https://github.com/rust-lang/rust-analyzer/pull/22008)
* [various fixes for `lower_coroutine_body_with_moved_arguments()`](https://github.com/rust-lang/rust-analyzer/pull/22207)
* [wrap top level or patterns in parens in `convert_match_to_let_else`](https://github.com/rust-lang/rust-analyzer/pull/22229)
* [hir-ty: emit diagnostic for unused `#[must_use]` values](https://github.com/rust-lang/rust-analyzer/pull/22239)
* [ide-diagnostics: emit error for duplicate field in record expression](https://github.com/rust-lang/rust-analyzer/pull/22235)
* [ide-diagnostics: emit error for mismatched array pattern length](https://github.com/rust-lang/rust-analyzer/pull/22238)
* [migrate generate function to SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/22159)
* [perf: cache more things that are related to lang items (paren traits, children/sibling assoc types/functions) but are not lang items themselves](https://github.com/rust-lang/rust-analyzer/pull/22280)
* [perf: do not intern `AdtDef`](https://github.com/rust-lang/rust-analyzer/pull/22187)
* [perf: improve performance of integer-based symbols](https://github.com/rust-lang/rust-analyzer/pull/22267)
* [remove add predicate for Where syntax](https://github.com/rust-lang/rust-analyzer/pull/22246)
* [remove unused a method in `edit_in_place`](https://github.com/rust-lang/rust-analyzer/pull/22242)
* [replace insert use and insert use as alias with its editor variant](https://github.com/rust-lang/rust-analyzer/pull/22241)
* [use syntaxFactory in generic arg instead of vanilla make](https://github.com/rust-lang/rust-analyzer/pull/22243)

### Rust Compiler Performance Triage

This week's result is pretty much neutral. It looks negative in icount numbers, but that's spurious, wall time remained largely unchanged. Some big performance improvements landed in the new solver, which is not enabled by default, yet.

Triage done by **@panstromek**.
Revision range: [ca9a134e..1d72d7e8](https://perf.rust-lang.org/?start=ca9a134e0985765ded9cfdde4030a5df4db7e2bd&end=1d72d7e8136faaebad3a85eeed432e6ea1b2ffab&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ? <br /> (primary)    | 0.6%  | [0.2%, 1.2%]   | 106   |
| Regressions ? <br /> (secondary)  | 0.7%  | [0.2%, 2.4%]   | 67    |
| Improvements ? <br /> (primary)   | -0.6% | [-1.7%, -0.2%] | 66    |
| Improvements ? <br /> (secondary) | -0.6% | [-2.8%, -0.0%] | 60    |
| All ?? (primary)                 | 0.1%  | [-1.7%, 1.2%]  | 172   |


1 Regression, 2 Improvements, 9 Mixed; 5 of them in rollups
34 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/477a72d1755b1b8adb3c4b7eef2ed34e0c954de7/triage/2026/2026-05-05.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Use either
* [Item title](Item URL)
  - or
* *No RFCs were approved this week.*
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
<!-- Either remove the group from the "No Items Entered Final Comment Period this week for" section
     and add the item(s) which entered Final comment period:
##### [Group](Group URL)
* [Item title](Item URL)
  - for `disposition-merge` `final-comment-period` items, or
* [disposition: postpone]
  - for `disposition-postpone` `final-comment-period` items, or
* [disposition: close]
  - for `disposition-close` `final-comment-period` items,
* [disposition: unspecified]
  - when `disposition` is unspecified or ensure the group is a part of the
     "No Items Entered Final Comment Period this week for" section
*No Items entered Final Comment Period this week for
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.
-->

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
<!-- Use either
* [Item title](Item URL)
  - for new items, or
* [updated] [Item title](Item URL)
  - for updated items, or
* *No New or Updated RFCs were created this week.*
-->

<!-- Sample commit message
Update CFT, FCP, MCP and RFC sections for TWiR-xxx
-->

## Upcoming Events

Rusty Events between 2026-05-06 - 2026-06-03 🦀

### Virtual
* 2026-05-06 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Practical introduction to SIMD**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/314301861/)
* 2026-05-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/rd05z3vo)
* 2026-05-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/314323890/)
* 2026-05-07 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455928/)
* 2026-05-07 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345240/)
* 2026-05-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Learning Rust the Hard Way: Building a TUI Chess Game**](https://luma.com/u436v3d7)
* 2026-05-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254782/)
* 2026-05-12 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506068/)
* 2026-05-12 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Introduction to database access using Rust SQLx**](https://www.meetup.com/code-mavens/events/314642118/)
* 2026-05-17 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329043/)
* 2026-05-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjchbzb/)
* 2026-05-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Mouse Control with Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/548kbqhl)
* 2026-05-21 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455929/)
* 2026-05-21 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #4 - Capsule coding in QEMU!**](https://www.meetup.com/charlottesville-rust-meetup/events/314477948/)
* 2026-05-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254781/)
* 2026-05-26 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Seeing Into Your Code - A Practical Guide to Tracing in Rust**](https://www.meetup.com/women-in-rust/events/313506048/)
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/9v7hv2g1)
* 2026-06-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcjbfb/)

### Africa
* 2026-05-12 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup)
    * [**Rust by Example - Flow of Control**](https://www.meetup.com/johannesburg-rust-meetup/events/314614331/)

### Asia
* 2026-05-13 | Malaysia, MY | [Rust Meetup Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup May 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
* 2026-05-14 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/314649688/)
* 2026-05-16 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2026 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2026-rustacean-meetup/)

### Europe
* 2026-05-06 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in May: Rust for Starters, Part 2**](https://www.meetup.com/rustcologne/events/314552161/)
* 2026-05-06 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Rust Milan @ Python Milano: Python or Rust? Yes!**](https://www.meetup.com/rust-language-milan/events/314521855/)
* 2026-05-06 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Building LLMs from scratch**](https://www.meetup.com/oxford-rust-meetup-group/events/314456933/)
* 2026-05-07 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust May Talks: Aetherus + Bevy**](https://www.meetup.com/rust-and-friends/events/314300802/)
* 2026-05-13 | Girona, ES | [Rust Girona](https://luma.com/rust-girona)
    * [**Rust Girona Hack & Learn 05 2026**](https://luma.com/ooub1kt0)
* 2026-05-14 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-05-18 - 2026-05-23 | Amsterdam, NL | [RustWeek 2026](https://2026.rustweek.org/)
    * [**RustWeek 2026**](https://2026.rustweek.org/)
* 2026-05-18 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**RustWeek 2026**](https://www.meetup.com/rust-language-milan/events/314329200/)
* 2026-05-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/314129975/)
* 2026-05-19 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**RustWeek 2026 announcement**](https://www.meetup.com/rust-nederland/events/312861992/)
* 2026-05-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Cross-Building & Cross-Testing**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813902/)
* 2026-05-19 | London, UK | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**RustWeek lunch meetup**](https://www.meetup.com/women-in-rust/events/314313054/)
* 2026-05-21 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**RustWeek Hackathon**](https://www.meetup.com/rust-nederland/events/314301699/)
* 2026-05-22 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Bike tour around Utrecht**](https://www.meetup.com/rust-nederland/events/314523659/)
* 2026-05-26 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Agentic Programming - May**](https://www.meetup.com/rust-dortmund/events/314522781/)
* 2026-05-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/314452972/)
* 2026-05-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396588/)

### North America
* 2026-05-07 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Reversing the Great Firewall and Geospatial Rust**](https://www.meetup.com/rust-nyc/events/314567143/)
* 2026-05-07 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Open Project Night**](https://www.meetup.com/stl-rust/events/313807225/)
* 2026-05-09 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, May 9**](https://www.meetup.com/bostonrust/events/314480529/)
* 2026-05-14 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314469265/)
* 2026-05-14 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust)
    * [**From Radio Waves to Pixels - Real-Time Visualizations with Rust and WebAssembly**](https://www.meetup.com/pdxrust/events/314256732/)
* 2026-05-14 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust May Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721886/)
* 2026-05-16 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Lechmere Rust Lunch, May 16**](https://www.meetup.com/bostonrust/events/314480531/)
* 2026-05-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314154841/)
* 2026-05-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Mouse Control with Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | San Francisco, CA, US | [Bay Area Rust Meetup](https://luma.com/bayarearust)
    * [**Bay Area Rust Meetup**](https://luma.com/9j3q5ejl)
* 2026-05-21 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Community Meetup**](https://www.meetup.com/music-city-rust-developers/events/314359076/)
* 2026-05-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Allston Rust Lunch, May 23**](https://www.meetup.com/bostonrust/events/314480534/)
* 2026-05-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/314209662/)
* 2026-05-28 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539319/)
* 2026-05-28 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust in Embedded & Autonomous Systems at Parallel Systems in DTLA**](https://www.meetup.com/rust-los-angeles/events/314218564/)
* 2026-05-30 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Central Cambridge Rust Lunch, May 30**](https://www.meetup.com/bostonrust/events/314480537/)

### Oceania
* 2026-05-14 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne - May 2026**](https://www.meetup.com/rust-melbourne/events/314260890/)
* 2026-05-26 | Barton, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/314050576/)

### South America
* 2026-05-13 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Rust Uruguay meetup de Mayo**](https://www.meetup.com/rust-uruguay/events/314532884/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> From a business standpoint, we should have reasonable confidence that it’ll stick around and be healthy for more than 10 years. We’d also like a robust ecosystem of code and tools that we can rely on, and experts we can hire.

– [David Anderson on the tailscale blog](https://tailscale.com/blog/tailscale-rs-rust-tsnet-library-preview)

Thanks to [Ivan Fraixedes](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1764) for the suggestion!

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
