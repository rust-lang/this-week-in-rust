Title: This Week in Rust 599
Number: 599
Date: 2025-05-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Announcing Google Summer of Code 2025 selected projects](https://blog.rust-lang.org/2025/05/08/gsoc-2025-selected-projects/)

### Foundation
* [10 Years of Stable Rust: An Infrastructure Story](https://rustfoundation.org/media/10-years-of-stable-rust-an-infrastructure-story/)

### Newsletters
* [This Month in Rust OSDev: April 2025 | Rust OSDev](https://rust-osdev.com/this-month/2025-04/)
* [The Embedded Rustacean Issue #45](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-45)

### Project/Tooling Updates
* [Avian Physics 0.3](https://joonaa.dev/blog/08/avian-0-3)
* [Two months in Servo: CSS nesting, Shadow DOM, Clipboard API, and more](https://servo.org/blog/2025/05/09/this-month-in-servo/)
* [Cot v0.3: Even Lazier](https://mackow.ski/blog/cot-v03-even-lazier/)
* [Streaming data analytics, Fluvio 0.17.3 release](https://www.fluvio.io/news/this-week-in-fluvio-0075)
* [CGP v0.4 is Here: Unlocking Easier Debugging, Extensible Presets, and More](https://contextgeneric.dev/blog/v0-4-0-release/)
* [Rama v0.2](https://github.com/plabayo/rama/discussions/544)

### Observations/Thoughts
* [Bad Type Patterns - The Duplicate duck](https://www.schneems.com/2025/05/07/bad-type-patterns-the-duplicate-duck/)
* [Rust nightly features you should watch out for](https://www.wakunguma.com/blog/interesting-rust-nightly-features)
* [Lock-Free Rust: How to Build a Rollercoaster While It’s on Fire](https://yeet.cx/blog/lock-free-rust/)
* [Simple & type-safe localization in Rust](https://aarol.dev/posts/rust-simple-i18n/)
* [From Rust to AVR assembly: Dissecting a minimal blinky program](https://n-eq.github.io/blog/2025/05/13/rust-avr-arduino-blink)
* [Tarpaulins Week Of Speed](https://xd009642.github.io/2025/05/08/Tarpaulins-Week-of-Speed.html)
* [Rustls Server-Side Performance](https://www.memorysafety.org/blog/rustls-server-perf/)
* [Is Rust the Future of Programming?](https://blog.jetbrains.com/rust/2025/05/13/is-rust-the-future-of-programming/)

### Rust Walkthroughs
* [Functional asynchronous Rust](https://willemvanhulle.tech/blog/func-async/)
* [The Power of Compile-Time ECS Architecture in Rust](https://minikin.me/blog/entity-component-systems-reimagined)
* [video] [Build with Naz : Spinner animation, lock contention, Ctrl+C handling for TUI and CLI](https://www.youtube.com/watch?v=iIMYzczF11c)

### Miscellaneous
* [April 2025 Rust Jobs Report](https://filtra.io/rust/jobs-report/apr-25)

## Crate of the Week

This week's crate is [brush](https://github.com/reubeno/brush/), a bash compatible shell implemented completely in Rust.

Thanks to [Josh Triplett](https://users.rust-lang.org/t/crate-of-the-week/2704/1434) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

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

* [rama - add ffi/rama-rhai: support ability to use services and layers written in rhai](https://github.com/plabayo/rama/issues/533)
* [rama - support akamai h2 passive fingerprint and expose in echo + fp services](https://github.com/plabayo/rama/issues/517)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* *No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

397 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-05-06..2025-05-13

#### Compiler

* [async drop fix for `async_drop_in_place<T>` layout for unspecified T](https://github.com/rust-lang/rust/pull/140902)
* [better error message for late/early lifetime param mismatch](https://github.com/rust-lang/rust/pull/140523)
* [perf: make the assertion in `Ident::new` debug-only](https://github.com/rust-lang/rust/pull/140880)
* [perf: merge typeck loop with static/const item eval loop](https://github.com/rust-lang/rust/pull/140854)

#### Library

* [implement (part of) ACP 429: add `DerefMut` to `Lazy[Cell/Lock]`](https://github.com/rust-lang/rust/pull/129334)
* [implement `VecDeque::truncate_front()`](https://github.com/rust-lang/rust/pull/140668)

#### Cargo

* [network: use Retry-After header for HTTP 429 responses](https://github.com/rust-lang/cargo/pull/15463)
* [rustc: Don't panic on unknown bins](https://github.com/rust-lang/cargo/pull/15497)
* [add glob pattern support for `known_hosts`](https://github.com/rust-lang/cargo/pull/15508)
* [add support for `-Zembed-metadata`](https://github.com/rust-lang/cargo/pull/15378)
* [fix tracking issue template link](https://github.com/rust-lang/cargo/pull/15494)
* [make cargo script ignore workspaces](https://github.com/rust-lang/cargo/pull/15496)

#### Rustdoc

* [rustdoc-json: remove newlines from attributes](https://github.com/rust-lang/rust/pull/140762)
* [ensure that temporary doctest folder is correctly removed even if doctests failed](https://github.com/rust-lang/rust/pull/140706)

#### Clippy

* [clippy: `item_name_repetitions`: exclude `enum` variants with identical path components](https://github.com/rust-lang/rust-clippy/pull/14619)
* [clippy: `return_and_then`: only lint returning expressions](https://github.com/rust-lang/rust-clippy/pull/14783)
* [clippy: `unwrap_used`, `expect_used`: accept macro result as receiver](https://github.com/rust-lang/rust-clippy/pull/14575)
* [clippy: add `allow_unused` config to `missing_docs_in_private_items`](https://github.com/rust-lang/rust-clippy/pull/14453)
* [clippy: add new `confusing_method_to_numeric_cast` lint](https://github.com/rust-lang/rust-clippy/pull/13979)
* [clippy: add new lint: `cloned_ref_to_slice_refs`](https://github.com/rust-lang/rust-clippy/pull/14284)
* [clippy: fix ICE in `missing_const_for_fn`](https://github.com/rust-lang/rust-clippy/pull/14776)
* [clippy: fix `integer_division` false negative for NonZero denominators](https://github.com/rust-lang/rust-clippy/pull/14664)
* [clippy: fix `manual_let_else` false negative when diverges on simple `enum` variant](https://github.com/rust-lang/rust-clippy/pull/14732)
* [clippy: fix `unnecessary_unwrap` emitted twice in closure](https://github.com/rust-lang/rust-clippy/pull/14770)
* [clippy: fix diagnostic paths printed by dogfood test](https://github.com/rust-lang/rust-clippy/pull/14746)
* [clippy: fix false negative for `unnecessary_unwrap`](https://github.com/rust-lang/rust-clippy/pull/14758)
* [clippy: make `let_with_type_underscore` help message into a suggestion](https://github.com/rust-lang/rust-clippy/pull/14749)
* [clippy: resolve through local re-exports in `lookup_path`](https://github.com/rust-lang/rust-clippy/pull/14772)

#### Rust-Analyzer

* [fix postfix snippets duplicating derefs](https://github.com/rust-lang/rust-analyzer/pull/19764)
* [resolve doc path from parent module if outer comments exist on module](https://github.com/rust-lang/rust-analyzer/pull/19507)
* [still complete parentheses & method call arguments if there are existing parentheses, but they are after a newline](https://github.com/rust-lang/rust-analyzer/pull/19763)

### Rust Compiler Performance Triage

Lot of changes this week. Overall result is positive, with one large win in type check.

Triage done by **@panstromek**.
Revision range: [62c5f58f..718ddf66](https://perf.rust-lang.org/?start=62c5f58f57670ce65e7fec34f8c4ba00c27da2d9&end=718ddf660e6a1802c39b4962cf7eaa4db57025ef&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.2%, 1.4%]    | 113   |
| Regressions ❌ <br /> (secondary)  | 0.5%  | [0.1%, 1.5%]    | 54    |
| Improvements ✅ <br /> (primary)   | -2.5% | [-22.5%, -0.3%] | 45    |
| Improvements ✅ <br /> (secondary) | -0.9% | [-2.3%, -0.2%]  | 10    |
| All ❌✅ (primary)                 | -0.3% | [-22.5%, 1.4%]  | 158   |

[Full report here](https://github.com/rust-lang/rustc-perf/blob/521ad9b18768d7c9890dbc6e6685e38b8d4c0164/triage/2025-05-12.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for `non_null_from_ref`](https://github.com/rust-lang/rust/issues/130823)
* [Add std::io::Seek instance for `std::io::Take`](https://github.com/rust-lang/rust/pull/138023)
* [aarch64-softfloat: forbid enabling the neon target feature](https://github.com/rust-lang/rust/pull/135160)
* [Stabilize the avx512 target features](https://github.com/rust-lang/rust/pull/138940)
* [make std::intrinsics functions actually be intrinsics](https://github.com/rust-lang/rust/pull/139916)
* [Error on recursive opaque ty in HIR typeck](https://github.com/rust-lang/rust/pull/139419)
* [Remove `i128` and `u128` from `improper_ctypes_definitions`](https://github.com/rust-lang/rust/pull/137306)
* [Guarantee behavior of transmuting `Option::<T>::None` subject to NPO](https://github.com/rust-lang/rust/pull/137323)
* [Temporary lifetime extension through tuple struct and tuple variant constructors](https://github.com/rust-lang/rust/pull/140593)
* [Stabilize `tcp_quickack`](https://github.com/rust-lang/rust/pull/129121)
* [Change the desugaring of `assert!` for better error output](https://github.com/rust-lang/rust/pull/122661)
* [Make well-formedness predicates no longer coinductive](https://github.com/rust-lang/rust/pull/140208)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Extended Standard Library (ESL)](https://github.com/rust-lang/rfcs/pull/3810)

## Upcoming Events

Rusty Events between 2025-05-14 - 2025-06-11 🦀

### Virtual
* 2025-05-15 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/eeqmobhz)
* 2025-05-15 | Virtual (Joint Meetup, Europe + Israel) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/) + [Rust Paris](https://www.meetup.com/de-DE/rust-paris/) + [London Rust Project Group](https://www.meetup.com/de-DE/london-rust-project-group/) + [Rust Zürisee](https://www.meetup.com/de-DE/rust-zurich/) + [Rust TLV](https://www.meetup.com/de-DE/rust-tlv/) + [Rust Nürnberg](https://www.meetup.com/de-DE/rust-noris/) + [Rust Munich](https://www.meetup.com/de-DE/rust-munich/) + [Rust Aarhus](https://www.meetup.com/de-de/rust-aarhus/) + [lunch.rs](http://lunch.rs/)
    * [**🦀 Celebrating 10 years of Rust 1.0 🦀**](https://www.meetup.com/rust-berlin/events/307293317)
* 2025-05-15 | Virtual (Zürich, CH) | [Rust Zürisee](https://www.meetup.com/rust-zurich)
    * [**🦀 Celebrating 10 years of Rust 1.0 (co-event with berline.rs) 🦀**](https://www.meetup.com/rust-zurich/events/307696718)
* 2025-05-18 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307796049)
* 2025-05-19 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Tauri: Cross-Platform desktop applications with Rust and web technologies**](https://www.meetup.com/rust-tlv/events/307178592)
* 2025-05-20 | Hybrid (EU/UK) | [Rust and C++ Dragons (former Cardiff)](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Talk and Connect - Fullstack - with Goetz Markgraf and Ben Wishovich**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/307836345)
* 2025-05-20 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Threading through lifetimes of borrowing - the Rust way**](https://www.meetup.com/women-in-rust/events/307522540)
* 2025-05-20 | Virtual (Tel Aviv, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Rust at Work a conversation with Ran Reichman Co-Founder & CEO of Flarion**](https://www.meetup.com/code-mavens/events/307635734/)
* 2025-05-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170826)
* 2025-05-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Linking**](https://www.meetup.com/vancouver-rust/events/307184332)
* 2025-05-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820302)
* 2025-05-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/8zabmc3w)
* 2025-05-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307668643)
* 2025-05-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361435)
* 2025-05-27 | Virtual (Tel Aviv, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversation with Eli Shalom & Igal Tabachnik of Eureka Labs**](https://www.meetup.com/code-mavens/events/307673680/)
* 2025-05-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820285)
* 2025-05-29 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/307730629)
* 2025-06-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307795210)
* 2025-06-03 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Why Rust? למה ראסט? -**](https://www.meetup.com/rust-tlv/events/307801358)
* 2025-06-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031665)
* 2025-06-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820301)
* 2025-06-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-06-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjblb)
* 2025-06-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305020417)
* 2025-06-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/307560326)

### Asia
* 2025-05-17 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi)
    * [**Rust Delhi Meetup #10**](https://www.meetup.com/rustdelhi/events/307657584)
* 2025-05-24 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2025-rustacean-meetup/)
* 2025-06-08 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust June 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europe
* 2025-05-13 - 2025-05-17 | Utrecht, NL | [Rust NL](https://rustweek.org/about)
    * [**RustWeek 2025**](https://rustweek.org)
* 2025-05-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045447)
* 2025-05-15 | Berlin, DE | [Rust Berlin](https://berline.rs/)
    * [**10 years anniversary of Rust 1.0**](https://www.c-base.org/calendar/#view=month&date=2025-05-15&event=92df14e3-c21c-477a-a150-84be085ed411)
* 2025-05-15 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust 10-year anniversary @ Appear**](https://www.meetup.com/rust-oslo/events/307427014)
* 2025-05-16 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Rust Week Hackathon**](https://www.meetup.com/rust-nederland/events/307107584)
* 2025-05-16 | Utrecht, NL | [Rust NL Meetup Group](https://www.meetup.com/rust-nederland/)
    * [**RustWeek Hackathon**](https://www.meetup.com/rust-nederland/events/307107584/)
* 2025-05-17 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Walking Tour around Utrecht - Saturday**](https://www.meetup.com/rust-nederland/events/307522004)
* 2025-05-20 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Talk and Connect - Fullstack - with Goetz Markgraf and Ben Wishovich**](https://www.meetup.com/rust-dortmund/events/307505881)
* 2025-05-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/307289572)
* 2025-05-20 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741635)
* 2025-05-22 | Augsburg, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Rust meetup #13:A Practical Guide to Telemetry in Rust**](https://rust-augsburg.github.io/meetup/Meetup_13.html)
* 2025-05-22 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #3 @zentroom**](https://www.meetup.com/rust-bern/events/307559606)
* 2025-05-22 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/307565141)
* 2025-05-22 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/307653223)
* 2025-05-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #11 @ Letsboot Basel**](https://www.meetup.com/rust-basel/events/307567083)
* 2025-05-27 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna - May at Bitcredit 🦀**](https://www.meetup.com/rust-vienna/events/307825439)
* 2025-05-29 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 2025-05-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #12**](https://www.meetup.com/stockholm-rust/events/307766469)
* 2025-06-04 | Ghent, BE | [Systems Programming Ghent](https://www.sysghent.be)
    * [**Grow smarter with embedded Rust**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 2025-06-04 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)
* 2025-06-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045448)

### North America
* 2025-05-15 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-15 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/307488039/)
* 2025-05-15 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Using Rust For Web Series 2 : Why you, Yes You. Should use Hyperscript!**](https://www.meetup.com/music-city-rust-developers/events/304333101)
* 2025-05-15 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**May, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658468)
* 2025-05-18 | Albuquerque, NM, US | [**Ideas and Coffee**](https://www.meetup.com/ideas-and-coffee/)
    * [**Intro Level Rust Get-together**](https://www.meetup.com/ideas-and-coffee/events/307645653/)
* 2025-05-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307337307)
* 2025-05-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Linking**](https://www.meetup.com/vancouver-rust/events/307184332)
* 2025-05-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/307720951)
* 2025-05-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)
* 2025-06-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)

### South America
* 2025-05-28 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 2025-05-31 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1knkfb6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> If a `Pin` drops in a room, and nobody around understands it, does it make an unsound? #rustlang

– [Josh Triplett on fedi](https://social.joshtriplett.org/notice/AtrAtfNxi0bcypcBDk)

Thanks to [Josh Triplett](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1684) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1ko2g0n/this_week_in_rust_599/)</small>
