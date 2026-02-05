Title: This Week in Rust 637
Number: 637
Date: 2026-02-04
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
* [The Embedded Rustacean Issue #64](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-64)

* [Rust Trends Issue #74: When Meta and Anthropic Choose Rust](https://rust-trends.com/newsletter/when-meta-and-anthropic-choose-rust)

### Project/Tooling Updates

* [Compiling Rust to readable C with Eurydice](https://lwn.net/SubscriberLink/1055211/0c358474dee845ec/)
* [3DCF/doc2dataset v0.2.0](https://github.com/3DCF-Labs/doc2dataset/releases/tag/v0.2.0) – embeddable document-compression [crate](https://crates.io/crates/three-dcf-core) (3DCF encoder + JSONL export helpers) to turn PDFs/markdown/HTML/etc into token-efficient chunks inside your Rust tooling.
* [kinded v0.5.0 - proc-macro for generating data-free companion enum](https://github.com/greyblake/kinded/releases/tag/v0.5.0)
* [CGP v0.6.1 Release: Improving Ergonomics and Debugging](https://contextgeneric.dev/blog/v0-6-1-release/)

### Observations/Thoughts
* [Rust for Network Programming](https://dev.to/godofgeeks/rust-for-network-programming-1en5)

### Rust Walkthroughs

* [`post.explain_builders().build()`](https://hemomorphic.alexblood.net/posts/builders/)
* [Homebrew and One-Line Installers for My Rust CLI: Lessons Learned](https://ivaniscoding.github.io/posts/rustpackaging2/)
* [series] [The Impatient Programmer's Guide to Bevy and Rust: Chapter 7 - Let There Be Enemies](https://aibodh.com/posts/bevy-rust-game-development-chapter-7/)

### Research

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

### [Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen)

* [Tracking Issue for timing report SVG render backend](https://github.com/rust-lang/cargo/issues/16440)

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
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

* [Spindalis - QR factorisation](https://github.com/lignum-vitae/spindalis/issues/56)
* [Spindalis - Add a function and macro that can expand polynomials](https://github.com/lignum-vitae/spindalis/issues/36)
* [Goombay-rs - Add Gotoh algorithm](https://github.com/lignum-vitae/goombay-rs/issues/8)
* [Goombay-rs - Add Waterman-Smith-Beyer](https://github.com/lignum-vitae/goombay-rs/issues/7)
* [Goombay-rs - Add functions to LocalAlignmentModel](https://github.com/lignum-vitae/goombay-rs/issues/4)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.
* [**Oxidize Conference](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP open until 2026-03-23 | Berlin, Germany | 2026-09-14 - 2026-09-16
  
<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | CFP closes 2026-02-16 | Montreal, Quebec, Canada | 2026-09-08 - 2026-09-11

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

Overall a positive week for instruction counts (~1% improvement on
check/debug/opt/doc builds). Cycle counts and memory usage remain broadly
unchanged across the week though.

Triage done by **@simulacrum**.
Revision range: [ebf13cca..a60d12cb](https://perf.rust-lang.org/?start=ebf13cca58b551b83133d4895e123f7d1e795111&end=a60d12cbccfbeaf153f3cecb90454aa696ea4b3b&absolute=false&stat=instructions%3Au)

0 Regression, 6 Improvements, 3 Mixed; 3 of them in rollups
33 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-02-02.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:
* [Trait method impl restrictions (`final` methods)](https://github.com/rust-lang/rfcs/pull/3678)
* [RFC: `#[export_visibility = ...]` attribute](https://github.com/rust-lang/rfcs/pull/3834)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Stabilize `core::hint::cold_path`](https://github.com/rust-lang/rust/pull/151576)
* [Tracking Issue for minimal const `ControlFlow` methods (`min_const_control_flow`)](https://github.com/rust-lang/rust/issues/148738)
* [Tracking Issue for `new_range_api` (part of RFC 3550)](https://github.com/rust-lang/rust/issues/125687)
* [Stabilize `assert_matches`](https://github.com/rust-lang/rust/pull/137487)
* [resolve: Report more visibility-related early resolution ambiguities for imports](https://github.com/rust-lang/rust/pull/149596)
* [Add FCW for derive helper attributes that will conflict with built-in attributes](https://github.com/rust-lang/rust/pull/151152)
* [Constify `fmt::from_fn`](https://github.com/rust-lang/rust/pull/150300)
* [Feature-gate `mut ref` patterns in struct pattern field shorthand](https://github.com/rust-lang/rust/pull/151102)
* [Tracking Issue for raw-pointer-to-reference conversion methods](https://github.com/rust-lang/rust/issues/122034)
* [implement PartialEq\<Vec\<U\>\> for \[T; N\] and &\[T; N\]](https://github.com/rust-lang/rust/pull/149045)
* [thread::scope: document how join interacts with TLS destructors](https://github.com/rust-lang/rust/pull/149482)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Proposal for a dedicated test suite for the parallel frontend](https://github.com/rust-lang/compiler-team/issues/906)
* [Promote tier 3 riscv32 ESP-IDF targets to tier 2](https://github.com/rust-lang/compiler-team/issues/864)
* [Proposal for Adapt Stack Protector for Rust](https://github.com/rust-lang/compiler-team/issues/841)

#### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [docs(report): enhance man pages for `cargo report *`](https://github.com/rust-lang/cargo/pull/16430)

#### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Guarantee repr(C) union field offset](https://github.com/rust-lang/reference/pull/2128)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Natural Method Disambiguation](https://github.com/rust-lang/rfcs/pull/3913)
* [Add `derive(Deref)` RFC](https://github.com/rust-lang/rfcs/pull/3911)
* [Abi Descriptors](https://github.com/rust-lang/rfcs/pull/3910)
* [Cargo mTLS registry authentication](https://github.com/rust-lang/rfcs/pull/3907)
* [Let `Option` derive `#[must_use]`](https://github.com/rust-lang/rfcs/pull/3906)
* [Version-typed cfgs](https://github.com/rust-lang/rfcs/pull/3905)

## Upcoming Events

Rusty Events between 2026-02-04 - 2026-03-04 🦀

### Virtual
* 2026-02-04 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting started with Rust Part 1: Common Programming Concepts**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312946936/)
* 2026-02-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312187422/)
* 2026-02-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2026-02-09 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Rust code reading and open source contribution (UTC 18:00; English)**](https://www.meetup.com/code-mavens/events/312985189/)
* 2026-02-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254789/)
* 2026-02-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/312799368/)
* 2026-02-11 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting Started with Rust Part 2: Ownership and Structs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312947249/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/5bu9kas1)
* 2026-02-12 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455922/)
* 2026-02-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312385179/)
* 2026-02-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/312951859/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ir8s81ec)
* 2026-02-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**February, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254788/)
* 2026-02-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & learn: Rust Pattern Matching Unpacked**](https://www.meetup.com/women-in-rust/events/312799411/)
* 2026-02-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/fvcjjuv8)
* 2026-02-26 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455923/)
* 2026-03-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcfbgb/)

### Asia
* 2026-02-05 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/312799833/)
* 2026-02-11 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup February 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)
* 2026-02-21 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**February 2026 Rustacean meetup**](https://hasgeek.com/rustbangalore/february-2026-rustacean-meetup/)
* 2026-02-23 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust February 2026 at Nuvoton in Herzliya**](https://www.meetup.com/rust-tlv/events/312989544/)

### Europe
* 2026-02-04 | Darmstadt, HE, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a newsletter subscription service with axum**](https://www.meetup.com/rust-rhein-main/events/312798996/)
* 2026-02-04 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 02 2026**](https://luma.com/e0uay6q5)
* 2026-02-04 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in February: Speed up Your Python**](https://www.meetup.com/rustcologne/events/313111752/)
* 2026-02-04 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 1**](https://www.meetup.com/rust-munich/events/312844145/)
* 2026-02-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Paul Grenyer: Beyond the Code: Designing Services That Stand the Test of Time**](https://www.meetup.com/oxford-rust-meetup-group/events/311744940/)
* 2026-02-05 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/312679714/)
* 2026-02-11 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-18 - 2026-02-19 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)
* 2026-02-24 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #5 @ Zrch: Doom on Embedded**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/313109606)
* 2026-02-24 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester February Talk**](https://www.meetup.com/rust-manchester/events/313172595/)
* 2026-03-04 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn March 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Records, Shredded on Ice: A Primer on Parquet and Iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)

### North America
* 2026-02-05 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/313163092/)
* 2026-02-05 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust 101: What is Rust and how can I use it?**](https://www.meetup.com/music-city-rust-developers/events/313133786/)
* 2026-02-05 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**February Monthly Social**](https://www.meetup.com/rust-montreal/events/313068358/)
* 2026-02-05 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312859472/)
* 2026-02-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Rendering the Mandelbrot set in Rust**](https://www.meetup.com/stl-rust/events/312614666/)
* 2026-02-07 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Allston Rust Lunch, Feb 7**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-11 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust ATX at Cloudflare**](https://www.meetup.com/rust-atx/events/313147803/)
* 2026-02-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Full Stack Web Development in Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**February, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Community Meet and Greet**](https://www.meetup.com/music-city-rust-developers/events/312038658/)
* 2026-02-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312755776/)
* 2026-02-25 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust as a Glue Layer- Infrastructure for AI-Native Applications**](https://www.meetup.com/rust-los-angeles/events/313097225/)
* 2026-02-26 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228648/)

### Oceania
* 2026-02-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane Feb 2026**](https://www.meetup.com/rust-brisbane/events/313087789/)
* 2026-02-11 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney)
* 2026-02-11 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney)
    * [**Welcome 🦀 to 2026**](https://www.meetup.com/rust-sydney/events/313074935/)
* 2026-02-24 | Canberra, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/313199994/)

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
