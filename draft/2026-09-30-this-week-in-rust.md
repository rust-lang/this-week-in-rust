Title: This Week in Rust 671
Number: 671
Date: 2026-09-30
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

* [Scientific Computing in Rust #22 (September 2026)](https://scientificcomputing.rs/monthly/2026-09)
* [The Embedded Rustacean Issue #81](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-81)

### Project/Tooling Updates

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

* [How do you stop being a Rust novice?](https://www.jochen.fyi/posts/how-do-you-stop-being-a-rust-novice)
* [We Have Named Arguments at Home](https://corrode.dev/blog/named-arguments-at-home/): a reply to the blog post *Arguing about arguments* mentioned in the last issue)
* [Upstream Rust maintenance report (August-September 2026)](https://kobzol.github.io/rust/2026/09/30/stf-august-september-2026.html)
* [Rust in the kernel? What about Rust without the kernel!](https://kerkour.com/rust-kernel)
* [Compiling the kernel with gccrs](https://lwn.net/SubscriberLink/1095553/7f34252658f8b8d1/)
* [Listening to the radio with Rust](https://lwn.net/SubscriberLink/1095721/e1d863e5fd827753/)
* [Native support for Rust on the GPU](https://lwn.net/SubscriberLink/1095731/a5ecc9da2388b8ec/)

### Rust Walkthroughs
* [ES] [Domain–Flow–Effects (DFE): an architecture designed for Rust](https://codigolinea.com/domain-flow-effects-dfe-arquitectura-rust/) 

* [Rust Reborrowing, Aliasing, and Mutable References](https://developerlife.com/2026/09/25/rust-reborrowing/)
* [A very condensed introduction of the basics of Rust](https://liw.fi/distilled-rust/)
* [video] [Making Our GPUI App Interactive with State and Events](https://youtu.be/bs8bpAZ10SM)

### Research

### Miscellaneous

* [DE][Rust & Linux Community Event – November 20–21, 2026 @ TUXEDO, Augsburg – Help us choose the workshop topic](https://cryptpad.fr/form/#/2/form/view/ppm1DazKFfZxfLB8Fw6-7W0vBC1kFV9KgkHQWqY7UU0/)

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

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- or if none - *No Calls for participation were submitted this week.* -->
* [unsynced - strace frontend fails on pwritev2 with offset -1 (current file offset)](https://github.com/zaydmulani09/unsynced/issues/1)
* [unsynced - Model hard links (link/linkat) instead of warning](https://github.com/zaydmulani09/unsynced/issues/2)
* [unsynced - Add an ext4 data=writeback persistence profile](https://github.com/zaydmulani09/unsynced/issues/3)
* [MemoryWhale - Cover friendly errors for incomplete `mw` arguments](https://github.com/wuisabel-gif/MemWhale/issues/253)
* [MemoryWhale - Lock down `mw --help` output](https://github.com/wuisabel-gif/MemWhale/issues/254)
* [dataprof - Remote Parquet refusal messages should say to download the file when the server ignores Range](https://github.com/AndreaBozzo/dataprof/issues/840)
* [dataprof - `ScoreBounds::dimension_scores` docs still list estimated key counts as unbounded](https://github.com/AndreaBozzo/dataprof/issues/827)
* [dataprof - The progress `finished` event under-counts rows when a row cap stops the incremental engine](https://github.com/AndreaBozzo/dataprof/issues/824)

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
* [Stop using dlltool for generating import libraries on MinGW](https://github.com/rust-lang/rust/pull/157712)
* [Stabilize `ptr::try_cast_aligned`](https://github.com/rust-lang/rust/pull/154170)
* [disposition: close] [1.99 beta crater regression: overflow evaluating the requirement](https://github.com/rust-lang/rust/issues/161916)
* [implement FCW for `rustc_allowed_through_unstable_modules` items](https://github.com/rust-lang/rust/pull/163161)
* [fix `VisibleForLeakCheck` in `RegionOutlives` fast path](https://github.com/rust-lang/rust/pull/163267)
* [Stabilize `debug_closure_helpers`](https://github.com/rust-lang/rust/pull/146099)
* [ Support type-relative assoc item paths in generic param defaults & const param types](https://github.com/rust-lang/rust/pull/161998)
* [FCW for `#[panic_handler]` on `unsafe fn`.](https://github.com/rust-lang/rust/pull/162974)
* [Stabilize `optimize` attribute](https://github.com/rust-lang/rust/pull/157273)
* [Allow unary operand types to be inferred later](https://github.com/rust-lang/rust/pull/159744)
* [Feat - `#[inline(always)] + #[target_feature(enable = "....")]` #2](https://github.com/rust-lang/rust/pull/162460)
* [Tracking Issue for `CStr::display`](https://github.com/rust-lang/rust/issues/139984)
* [Syntactically reject leading parenthesized precise capturing lists in bare trait object types (`(use<…>)+`)](https://github.com/rust-lang/rust/pull/162652)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [`f16b` type](https://github.com/rust-lang/rfcs/pull/3983)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [feat(trim-paths): stabilize `profile.trim-paths`](https://github.com/rust-lang/cargo/pull/17488)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Create new Tier 3 Target for QTEE: `aarch64-unknown-qtee`](https://github.com/rust-lang/compiler-team/issues/1038)

*No Items entered Final Comment Period this week for
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Make trait methods callable in const contexts, take III](https://github.com/rust-lang/rfcs/pull/4008)

## Upcoming Events

Rusty Events between 2026-09-30 - 2026-10-28 🦀

### Virtual
* 2026-09-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Segmentation and Introduction to Paging**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316486941/)
* 2026-10-01 | Virtual | [Rust Foundation & JetBrains](https://rustfoundation.org/event/livestream-smarter-coding-agents-for-rust-with-symposium/)
    * [**Livestream: Smarter Coding Agents for Rust with Symposium**](https://info.jetbrains.com/rustrover-livestream-october01-2026.html#form)
* 2026-10-02 | Virtual | [Rust Girona](https://luma.com/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/yqxvguts)
* 2026-10-03 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #14**](https://www.meetup.com/bevy-game-development/events/316736369/)
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
* 2026-10-10 | Hybrid (Kuala Lumpur, Malaysia) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup October 2026**](https://forms.gle/721DxqrPeHXY6omP9)
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
* 2026-10-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Reasoning with Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)

### Asia
* 2026-10-09 | Hybrid (Kuala Lumpur, MY) | [Rust Malaysia Meetup](https://discord.gg/Uz88bnZA3B)
    * [**Rust Meetup August 2026**](https://forms.gle/721DxqrPeHXY6omP9)

### Europe
* 2026-09-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #16 @ ERNI**](https://www.meetup.com/rust-basel/events/315986893/)
* 2026-09-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/316661690/)
* 2026-10-01 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Berlin on location 🏳️‍🌈 – Edition 018**](https://www.meetup.com/rust-berlin/events/316763107/)
* 2026-10-01 | Oxford, GB | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Embedded Rust for Duffers**](https://www.meetup.com/oxford-rust-meetup-group/events/316708765/)
* 2026-10-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2026 / 3**](https://www.meetup.com/rust-munich/events/316244709/)
* 2026-10-08 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/316564477/)
* 2026-10-08 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22nd bcnrust session**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-20 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)

### North America
* 2026-10-01 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Building a Minimal, Rootless Container in Rust**](https://www.meetup.com/stl-rust/events/316410027/)
* 2026-10-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Oct 3**](https://www.meetup.com/bostonrust/events/316378820/)
* 2026-10-08 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Lightning Talks N' Chill**](https://www.meetup.com/utah-rust/events/316708351/)
* 2026-10-08 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Zero Knowledge Proofs & GPUs in Rendering**](https://www.meetup.com/rust-nyc/events/316698929/)
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
* 2026-10-21 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Embedded Meetup**](https://luma.com/ur4pm34i)
* 2026-10-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316655403/)

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
