Title: This Week in Rust 601
Number: 601
Date: 2025-05-28
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
* [April Project Goals Update](https://blog.rust-lang.org/2025/05/26/april-project-goals-update/)
* [Demoting i686-pc-windows-gnu to Tier 2](https://blog.rust-lang.org/2025/05/26/demoting-i686-pc-windows-gnu/)

### Foundation
* [The Rust Foundation seeks input on its three-year strategy (2026-2028)](https://docs.google.com/forms/d/e/1FAIpQLSca3ziiYWrTti6Ti2ki3Sv9Okmhzc4wGMxQgudUOmQrmh3pVg/viewform?usp=dialog)

### Project/Tooling Updates
* [gitoxide May 2025](https://github.com/GitoxideLabs/gitoxide/discussions/2021#discussion-8357816)
* [The GCC compiler backend can now fully bootstrap the Rust compiler!](https://old.reddit.com/r/rust/comments/1ktph3c/media_the_gcc_compiler_backend_can_now_fully/)
* [Rust Coreutils 0.1 Released With Big Performance Gains](https://www.phoronix.com/news/Rust-Coreutils-0.1-Released)
* [Introducing Roto: A Compiled Scripting Language for Rust](https://blog.nlnetlabs.nl/introducing-roto-a-compiled-scripting-language-for-rust/)
* [alpine-rustx: Simple Rust cross-compilation using custom Docker images](https://github.com/tindzk/alpine-rustx)
* [Taskfinder 2.9.0](https://codeberg.org/kdwarn/taskfinder/src/commit/9d2779bfdd79826374bc5e77b85928c065b1094b/CHANGELOG.md#2-9-0-https-codeberg-org-kdwarn-taskfinder-compare-v2-8-0-v2-9-0-2025-05-22)
* [Yelken Second Alpha Release](https://bwqr.github.io/yelken-blog/second-alpha-release/)
* [First look at Blinksy](https://blog.mikey.nz/first-look-at-blinksy/)
* [malai 0.2.5 is here: Instantly share local TCP services (database/SSH) with others](https://www.malai.sh/hello-tcp/)

### Observations/Thoughts
* [iOS Deep-Linking with Bevy](https://rustunit.com/blog/2025/05-18-bevy-ios-deep-linking/)
* [Sguaba: hard-to-misuse rigid body transforms for engineers with other things to worry about than linear algebra](https://blog.helsing.ai/sguaba-hard-to-misuse-rigid-body-transforms-for-engineers-with-other-things-to-worry-about-than-aeaa45af9e0d)
* [Making the rav1d Video Decoder 1% Faster](https://ohadravid.github.io/posts/2025-05-rav1d-faster/)
* [Async from scratch 3: Pinned against the wall](https://natkr.com/2025-05-22-async-from-scratch-3/)
* [Fork Union: Beyond OpenMP in C++ and Rust?](https://ashvardanian.com/posts/beyond-openmp-in-cpp-rust/)
* [Programming language: Rust 2024 is the most comprehensive edition to date](https://www.heise.de/en/background/Programming-language-Rust-2024-is-the-most-comprehensive-edition-to-date-10393917.html)
* [Type-level Bounded Recursion in Rust](https://catgirl.ai/log/typelevel-bounded-recursion/)
* [A Tale of Testability and Sending Non-Send Types in Rust](https://geo-ant.github.io/blog/2025/rust-testability-and-non-send-types/)
* [video] [Hot-reloading Rust Game Dev: Coding Flappy Bird in Bevy From Scratch](https://www.youtube.com/watch?v=fo6FXxeP0Wg)
* [SIMD in zlib-rs (part 2): compare256](https://tweedegolf.nl/en/blog/155/simd-in-zlib-rs-part-2-compare256)

### Rust Walkthroughs
* [Secrets managers considered harmful. How to securely encrypt your sensitive data with envelope encryption and KMS in Rust](https://kerkour.com/rust-secrets-kms-envelope-encryption)
* [Rust, Memory performance & latency](https://developerlife.com/2025/05/19/rust-mem-latency/)

### Miscellaneous
* [How To Get A Rust Job Part II: Introducing Rust At Your Current Company](https://filtra.io/rust/career-help/how-to-get-a-rust-job-II)

## Crate of the Week

This week's crate is [boreal](https://github.com/vthib/boreal), a safe and performant [YARA](https://virustotal.github.io/yara/) rules evaluator.

Thanks to [Vincent Thiberville](https://users.rust-lang.org/t/crate-of-the-week/2704/1439) for the self-suggestion!

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

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hyperswitch - Add integrity check implementation in Adyen](https://github.com/juspay/hyperswitch/issues/8149)
* [Hyperswitch - Add integrity check implementation in Authorize.net](https://github.com/juspay/hyperswitch/issues/8150)
* [Hyperswitch - Add integrity check implementation in ACI](https://github.com/juspay/hyperswitch/issues/8151)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

[EuroRust 2025](https://www.papercall.io/eurorust-2025)| CFP closes on 2025-06-02 | Paris, France | 2025-10-09

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

433 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-05-20..2025-05-27

#### Compiler

* [don't rerun goals if none of their vars have changed](https://github.com/rust-lang/rust/pull/141500)
* [fold predicate fast path in canonicalizer and eager resolver](https://github.com/rust-lang/rust/pull/141442)

#### Library

* [add `std::os::unix::process::CommandExt::chroot` to safely chroot a child process](https://github.com/rust-lang/rust/pull/137759)
* [fix aliasing bug in UNIX process implementation](https://github.com/rust-lang/rust/pull/138896)
* [implement `ptr::try_cast_aligned` and `NonNull::try_cast_aligned`](https://github.com/rust-lang/rust/pull/141222)
* [implement `advance_by` via `try_fold` for `Sized` iterators](https://github.com/rust-lang/rust/pull/141086)

#### Cargo

* [toml: Remove workaround for rustc frontmatter support](https://github.com/rust-lang/cargo/pull/15570)
* [add `-Zfix-edition`](https://github.com/rust-lang/cargo/pull/15596)
* [add the future edition](https://github.com/rust-lang/cargo/pull/15595)
* [direct extraction for registry sources](https://github.com/rust-lang/cargo/pull/15514)
* [vendor files with .rej/.orig suffix](https://github.com/rust-lang/cargo/pull/15569)

#### Rustdoc

* [Unify type aliases rendering with other ADT](https://github.com/rust-lang/rust/pull/140863)
* [on mobile, make the sidebar full width and linewrap](https://github.com/rust-lang/rust/pull/139831)
* [speed up `TypeAliasPart::get`](https://github.com/rust-lang/rust/pull/141421)

#### Clippy

* [`manual_flatten`: fix with nested `Some` or `Ok` pattern](https://github.com/rust-lang/rust-clippy/pull/14846)
* [`needless_borrow`: do not contradict `dangerous_implicit_autorefs`](https://github.com/rust-lang/rust-clippy/pull/14810)
* [consider consts in patterns as refutable](https://github.com/rust-lang/rust-clippy/pull/14887)
* [fix `assign_op_pattern` false positive on unstable const trait](https://github.com/rust-lang/rust-clippy/pull/14886)
* [fix `manual_find` wrong suggestion when return type needs adjustment](https://github.com/rust-lang/rust-clippy/pull/14892)
* [fix `needless_for_each` wrong suggestion when closure has no braces](https://github.com/rust-lang/rust-clippy/pull/14735)
* [fix `manual_slice_size_computation` ICE and trigger in `const` context](https://github.com/rust-lang/rust-clippy/pull/14804)
* [make `trivial-copy-size-limit` consistently the size of the target pointer](https://github.com/rust-lang/rust-clippy/pull/13319)
* [various macro fixes for loop lints](https://github.com/rust-lang/rust-clippy/pull/14631)

#### Rust-Analyzer

* [change import prefix default to be by crate](https://github.com/rust-lang/rust-analyzer/pull/19819)
* [correctly set the span of the `proc_macro` crate's Group delimiters](https://github.com/rust-lang/rust-analyzer/pull/19839)
* [fix IDE resolution of item macros](https://github.com/rust-lang/rust-analyzer/pull/19862)
* [fix cache problems with lints level](https://github.com/rust-lang/rust-analyzer/pull/19824)
* [ide-assists, generate mut trait impl indent](https://github.com/rust-lang/rust-analyzer/pull/19792)
* [normalize when checking for uninhabited types for pattern exhaustiveness checking](https://github.com/rust-lang/rust-analyzer/pull/19851)
* [properly implement `might_be_inside_macro_call()` using semantic information instead of syntactical hacks](https://github.com/rust-lang/rust-analyzer/pull/19864)
* [ide-assists, `generate_new` indent loses](https://github.com/rust-lang/rust-analyzer/pull/19785)

### Rust Compiler Performance Triage

A week dominated by new sources of noise. By and large there were not that many real changes to compiler performance. Some highlights of real change to focus on are an improvement in rustdoc that had large wins in a few key benchmarks and an improvement in trait selection that comes from moving from an if/else chain to pattern matching.

Triage done by **@rylev**.
Revision range: [59372f2c..2805e1dc](https://perf.rust-lang.org/?start=59372f2c81ba74554d9a71b12a4ed7f29adb33a2&end=2805e1dc4c18ed4c84d161502c48da870c56f68a&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.1%, 7.5%]    | 73    |
| Regressions ❌ <br /> (secondary)  | 1.4%  | [0.1%, 6.8%]    | 34    |
| Improvements ✅ <br /> (primary)   | -4.0% | [-78.5%, -0.1%] | 41    |
| Improvements ✅ <br /> (secondary) | -6.2% | [-22.1%, -0.1%] | 28    |
| All ❌✅ (primary)                 | -1.0% | [-78.5%, 7.5%]  | 114   |


5 Regressions, 7 Improvements, 8 Mixed; 9 of them in rollups
45 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/02eafc9ca0dda4c5851fb38850166b8af55eda91/triage/2025-05-27.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Allow comparisons between `CStr`, `CString`, and `Cow<CStr>`.](https://github.com/rust-lang/rust/pull/137268)
* [Tracking Issue for `const_eq_ignore_ascii_case`](https://github.com/rust-lang/rust/issues/131719)
* [Stabilize feature result_flattening](https://github.com/rust-lang/rust/pull/141072)
* [Tracking Issue for `breakpoint` feature (`core::arch::breakpoint`)](https://github.com/rust-lang/rust/issues/133724)
* [Stabilize `sha512`. `sm3` and `sm4` for x86](https://github.com/rust-lang/rust/pull/140767)
* [Stabilize keylocker](https://github.com/rust-lang/rust/pull/140766)
* [terminology: allocated object → allocation](https://github.com/rust-lang/rust/pull/141224)
* [Tracking Issue for `keylocker_x86`](https://github.com/rust-lang/rust/issues/134813)
* [Tracking Issue for `sha512_sm_x86`](https://github.com/rust-lang/rust/issues/126624)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Use `gix` for `cargo package`](https://github.com/rust-lang/cargo/pull/15534)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Changes to target_feature attribute](https://github.com/rust-lang/rfcs/pull/3820)
* [new] [RFC: Promote aarch64-pc-windows-msvc to Tier 1](https://github.com/rust-lang/rfcs/pull/3817)

## Upcoming Events

Rusty Events between 2025-05-28 - 2025-06-25 🦀

### Virtual
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
* 2025-06-04 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs)
     * [**Scientific Computing in Rust 2025**](https://scientificcomputing.rs/2025)
* 2025-06-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820301)
* 2025-06-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-06-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307927093)
* 2025-06-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305020417)
* 2025-06-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/307560326)
* 2025-06-12 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Meet, swap, and learn!**](https://www.meetup.com/charlottesville-rust-meetup/events/307767236)
* 2025-06-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308074808)
* 2025-06-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170853)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820303)
* 2025-06-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbdc)
* 2025-06-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361436)
* 2025-06-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Building Efficient Web Scrapers: Rust vs. Python for Data Ingestion**](https://www.meetup.com/women-in-rust/events/306683025)

### Asia
* 2025-06-08 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust June 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europe
* 2025-05-28 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Surprise Topic**](https://www.meetup.com/rust-rhein-main/events/307836400)
* 2025-05-29 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 2025-05-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #12**](https://www.meetup.com/stockholm-rust/events/307766469)
* 2025-06-04 | Ghent, BE | [Systems Programming Ghent](https://www.sysghent.be/)
    * [**Grow smarter with embedded Rust**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 2025-06-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Risc V - the new challenger for cpus in AI and embedded systems.**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)
* 2025-06-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/308080874)
* 2025-06-10 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw)
    * [**Rust Warsaw Meetup #5**](https://www.meetup.com/rust-warsaw/events/307955051)
* 2025-06-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045448)
* 2025-06-17 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741641)
* 2025-06-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus meetup at Trifork**](https://www.meetup.com/rust-aarhus/events/308060489)
* 2025-06-19 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/308023524)
* 2025-06-20 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/308023512)
* 2025-06-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Code Night**](https://www.meetup.com/rust-manchester/events/307919158)
* 2025-06-25 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Lessons learnt from making a tiny game in nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)

### North America
* 2025-05-28 | Albuquerque, NM, US | [At Ideas and Coffee](https://www.meetup.com/ideas-and-coffee)
    * [**Intro Level Rust Get-together**](https://www.meetup.com/ideas-and-coffee/events/307645653)
* 2025-05-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/307720951)
* 2025-05-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)
* 2025-05-29 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/307498676)
* 2025-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/307936097)
* 2025-06-05 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/308091592)
* 2025-06-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)
* 2025-06-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston University Rust Lunch, June 8**](https://www.meetup.com/bostonrust/events/307936165)
* 2025-06-11 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> Security**](https://www.meetup.com/desert-rustaceans/events/308010023)
* 2025-06-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307595021)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Using Rust For Web Series 3 : Final Presentations and Community Social**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 2025-06-19 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**Monthly Meetup: Making CRUD with Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 2025-06-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Lechmere Rust Lunch, June 20**](https://www.meetup.com/bostonrust/events/307936242)
* 2025-06-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)

### Oceania
* 2025-06-16 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/307808896)
* 2025-06-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/307520854)

### South America
* 2025-05-31 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)
* 2025-06-04 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 2025-06-12 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Junio de WebAssembly!**](https://www.meetup.com/rust-argentina/events/307990465)

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

> This is basically the programming version of "learning Japanese as an English speaker is hard, therefore it is not a good language for babies to learn"

– [/u/Aaron1924 on /r/rust](https://www.reddit.com/r/programming/comments/1kqo2tc/comment/mt72ihj/) discussing whether Rust might be a good first language or not.

Thanks to [robin](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1688) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1kytv0p/this_week_in_rust_601_this_week_in_rust/)</small>
