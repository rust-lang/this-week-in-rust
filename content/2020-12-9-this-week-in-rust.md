Title: This Week in Rust 368
Number: 368
Date: 2020-12-9
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official
* [The Foundation Conversation](https://blog.rust-lang.org/2020/12/07/the-foundation-conversation.html)

### Newsletters
* [This Month in Rust GameDev #16 - November 2020](https://rust-gamedev.github.io/posts/newsletter-016/)
* [RiB Newsletter #18 - On to the Ribbles](https://www.reddit.com/r/rust/comments/k6cka7/rib_newsletter_18_on_to_the_ribbles/)

### Tooling
* [Rust Analyzer Changelog #54](https://rust-analyzer.github.io/thisweek/2020/12/07/changelog-54.html)
* [Knurling-rs Changelog #9](https://ferrous-systems.com/blog/knurling-changelog-9/)
* [IntelliJ Rust: Updates for 2020.3](https://blog.jetbrains.com/clion/2020/12/intellij-rust-updates-for-2020-3/)

### Observations/Thoughts
* [Monads and GATs in Nightly Rust](https://www.fpcomplete.com/blog/monads-gats-nightly-rust/)
* [Vanishing zeroes for geometric algebra in Rust](https://fanf.dreamwidth.org/134024.html)
* [On Generics and Associated Types](https://blog.thomasheartman.com/posts/on-generics-and-associated-types)
* [Adaptive Request Concurrency. Resilient observability at scale.](https://vector.dev/blog/adaptive-request-concurrency/)
* [Rust compression libraries](https://blog.logrocket.com/rust-compression-libraries/)
* [Rust makes cross compilation child's play](https://www.marcoieni.com/2020/12/rust-makes-cross-compilation-childs-play/)
* [Using the builder pattern to define test scenarios](https://jmmv.dev/2020/12/builder-pattern-for-tests.html)
* [Measuring Memory Usage in Rust](https://rust-analyzer.github.io/blog/2020/12/04/measuring-memory-usage-in-rust.html)
* [Saving time by switching users: Async support in Goose](https://www.tag1consulting.com/blog/saving-time-switching-users-async-support-goose)
* [Why Rust is meant to replace C](https://evrone.com/rust-vs-c)

### Rust Walkthroughs
* [Real-time video processing with Rust, FFmpeg and OpenCV](https://subvisual.com/blog/posts/real-time-video-processing-with-rust-ffmpeg-opencv/)
* [Merge k sorted arrays in Rust](https://dev.to/creativcoder/merge-k-sorted-arrays-in-rust-1b2f)
* [Make A Language - Part Thirteen: Whitespace & Events](https://arzg.github.io/lang/13/)
* [Unit-testing a console app (a text editor)](https://jmmv.dev/2020/12/unit-testing-a-console-app.html)
* [Rust and Async (on embedded devices)](https://blog.drogue.io/rust-and-async/)
* [Avoiding Duplicating Strings in Rust](https://www.fpcomplete.com/blog/avoiding-duplicating-strings-rust/)
* [OS in Rust: Custom target to build kernel for bare metal: Part-3](https://blog.knoldus.com/os-in-rust-custom-target-to-build-kernel-for-a-bare-metal-part-3/)
* [OS in Rust: Building kernel for custom target: Part-4](https://blog.knoldus.com/os-in-rust-building-kernel-for-custom-target-part-4/)
* [video] [Introduction to Rust Part 2](https://youtu.be/lLWchWTUFOQ)

### Project Updates
* [rust-gpu v0.2](https://github.com/EmbarkStudios/rust-gpu/releases/tag/v0.2)
* [Interior Mutability in Rust: Understanding The Cell Type](https://ibraheem.ca/posts/rust-interior-mutability-understanding-cell)

### Miscellaneous
* [Safe Interoperability between Rust and C++ with CXX](https://www.infoq.com/news/2020/12/cpp-rust-interop-cxx/)
* [Expanding Fuchsia's open source model](https://opensource.googleblog.com/2020/12/expanding-fuchsias-open-source-model.html)
* [Miri can now detect data races](https://www.reddit.com/r/rust/comments/k75tez/miri_can_now_detect_data_races/)

# Crate of the Week

This week's crate is [breadx](https://github.com/not-a-seagull/breadx), a X-windows protocol implementation in 100% safe and mutex-free Rust.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/851) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [Triox - Good First Issues](https://github.com/AaronErhardt/Triox/labels/good%20first%20issue)
* [libssh2 - Pull Request Needs Windows Reviewer](https://github.com/libssh2/libssh2/pull/517)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

279 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-11-30..2020-12-07

* [add wasm32 support to inline asm](https://github.com/rust-lang/rust/pull/78684)
* [improve attribute message error spans](https://github.com/rust-lang/rust/pull/79509)
* [chalk: always relate with Invariant to non-General inference vars](https://github.com/rust-lang/chalk/pull/659)
* [fix perf regression caused by match exhaustiveness split](https://github.com/rust-lang/rust/pull/79680)
* [pass around Symbols instead of Idents in doctree](https://github.com/rust-lang/rust/pull/79623)
* [tweak diagnostics on shadowing lifetimes/labels](https://github.com/rust-lang/rust/pull/79620)
* [avoid panic_bounds_check in `fmt::write`](https://github.com/rust-lang/rust/pull/78122)
* [fix incorrect `io::Take`'s limit resulting from `io::copy` specialization](https://github.com/rust-lang/rust/pull/79650)
* [`std::io`: use sendfile for UnixStream](https://github.com/rust-lang/rust/pull/79600)
* [cargo: slightly optimize `cargo vendor](https://github.com/rust-lang/cargo/pull/8937)
* [cargo: add "--workspace" to update command](https://github.com/rust-lang/cargo/pull/8725)
* [rustdoc: JSON backend experimental impl](https://github.com/rust-lang/rust/pull/79539)

## Rust Compiler Performance Triage

* [2020-12-08](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-08.md):
0 Regressions, 2 Improvements, 1 Mixed

Triage done by @simulacrum.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-12-08.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.


### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [RFC: Plan to make core and std's panic identical](https://github.com/rust-lang/rfcs/pull/3007)
* [RFC: Add `target_abi` configuration](https://github.com/rust-lang/rfcs/pull/2992)
* [added secret types rfc](https://github.com/rust-lang/rfcs/pull/2859)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [rustdoc: stabilise --default-theme command line option](https://github.com/rust-lang/rust/pull/79642)
* [disposition: merge] [Implement `From<char>` for u64 and u128.](https://github.com/rust-lang/rust/pull/79502)
* [disposition: merge] [Stabilize `unsafe_cell_get_mut`](https://github.com/rust-lang/rust/pull/79485)
* [disposition: merge] [Move `{f32,f64}::clamp` to core](https://github.com/rust-lang/rust/pull/79473)
* [disposition: merge] [Stabilize all stable methods of `Ipv4Addr`, `Ipv6Addr` and `IpAddr` as const](https://github.com/rust-lang/rust/pull/79342)
* [disposition: merge] [Acknowledge that `[CONST; N]` is stable](https://github.com/rust-lang/rust/pull/79270)
* [disposition: merge] [Deprecate atomic compare_and_swap method](https://github.com/rust-lang/rust/pull/79261)
* [disposition: merge] [Stabilize `core::slice::fill`](https://github.com/rust-lang/rust/pull/79213)
* [disposition: close] [Made matches! more useful by adding mapping support](https://github.com/rust-lang/rust/pull/79188)
* [disposition: merge] [passes: prohibit invalid attrs on generic params](https://github.com/rust-lang/rust/pull/79073)
* [disposition: merge] [stabilize deque_range](https://github.com/rust-lang/rust/pull/79022)
* [disposition: close] [Apply `unused_doc_comments` lint to inner items](https://github.com/rust-lang/rust/pull/78367)
* [disposition: merge] [Rename `overlapping_patterns` lint](https://github.com/rust-lang/rust/pull/78242)
* [disposition: merge] [Stabilize or_insert_with_key](https://github.com/rust-lang/rust/pull/78083)
* [disposition: close] [Add built-in implementations of `Default` for function definition and… ](https://github.com/rust-lang/rust/pull/77688)
* [disposition: merge] [Mark `-1` as an available niche for file descriptors](https://github.com/rust-lang/rust/pull/74699)
* [disposition: merge] [Stabilize the Wake trait](https://github.com/rust-lang/rust/pull/74304)
* [disposition: merge] [Tracking issue for map_ok and map_err method for `Poll<Option<Result<T, E>>>`](https://github.com/rust-lang/rust/issues/63514)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [December 10, Stuttgart, DE - Hack & Learn - Directions for 2021 - Rust Community Stuttgart](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/274892215/)
* [December 10, San Diego, CA, US - San Diego Rust December 2020 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/274757235/)
* [December 10, Washington, DC, US - How oso built a runtime reflection system for Rust—Rust DC](https://www.meetup.com/RustDC/events/274460587)
* [December 15, Russia - Russian Rust Online Meetup](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/274924961/)
* [December 16, Vancouver, BC, US - Are Results just Checked Exceptions? - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/npqfbsybcqbvb/)

### North America
* [December 10, Provo, UT, US - Mob Programming: Add `--tree -d` to `lsd`](https://www.meetup.com/utah-rust/events/273530244/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

* [Software Engineer, Systems at PathAI (Boston, MA, US)](https://www.pathai.com/careers/?gh_jid=4983568002)
* [Software Developer (Rust) at MeiliSearch (Remote)](https://www.welcometothejungle.com/fr/companies/meilisearch/jobs/software-developer-rust_paris)
* [Backend Engineer - Rust at Kraken (Remote NA, SA, EMEA)](https://jobs.lever.co/kraken/4019a818-4a7b-46ef-9225-c53c7a7f238c)
* [Backend Engineer, Kraken Futures - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Rust Engineer, Desktop GUI - Cryptowatch at Kraken (Remote)](https://jobs.lever.co/kraken/2442ee5c-56b6-4a73-a477-8cdda2b218d5)
* [Senior Backend Engineer - Rust at Kraken (Remote NA, SA, EMEA)](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105)
* [Senior Banking Engineer - Rust at Kraken (Remote)](https://jobs.lever.co/kraken/2863623f-13c9-4f50-992d-7c25736a60f9)
* [Software Engineer - Trading Technology (Rust) at Kraken (Remote NA, SA, EMEA](https://jobs.lever.co/kraken/4485f672-dc5f-4e49-a10b-2b0399e28a8d)
* [Rust for Embedded Environments at Ockam (Remote)](https://stackoverflow.com/jobs/294502/rust-for-embedded-environments-ockam)
* [Messaging protocol architect in Elixir (and Rust) at Ockam (Remote)](https://stackoverflow.com/jobs/400828/messaging-protocol-architect-in-elixir-and-rust-ockam)
* [Senior Software Engineer (Rust & C++) at NZXT (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Embedded Firmware Engineer in C & Rust at Astropad (Remote, US)](https://www.notion.so/Embedded-Firmware-Engineer-in-C-Rust-a9c741c539454ee7b8bbb969d8e90da2)


# Quote of the Week

> Writing rust for me is a gradual process of the compiler patiently guiding me towards the program I should have written in the first place, and at the end I take all the credit.

– [@felixwatts on Discord](https://discord.com/channels/442252698964721669/448238009733742612/783395725991084074)

Thanks to [Joshua Nelson](https://users.rust-lang.org/t/twir-quote-of-the-week/328/972) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
