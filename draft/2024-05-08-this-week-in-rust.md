Title: This Week in Rust 546
Number: 546
Date: 2024-05-08
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

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
* [Announcing Rust 1.78.0](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html)
* [Announcing Rustup 1.27.1](https://blog.rust-lang.org/2024/05/06/Rustup-1.27.1.html)
* [Automatic checking of cfgs at compile-time](https://blog.rust-lang.org/2024/05/06/check-cfg.html)
* [Announcing Google Summer of Code 2024 selected projects](https://blog.rust-lang.org/2024/05/01/gsoc-2024-selected-projects.html)
* [Rust participates in OSPP 2024](https://blog.rust-lang.org/2024/05/07/OSPP-2024.html)
* [This Development-cycle in Cargo: 1.79](https://blog.rust-lang.org/inside-rust/2024/05/07/this-development-cycle-in-cargo-1.79.html)
* [Rust Project Goals Submission Period](https://blog.rust-lang.org/inside-rust/2024/05/07/announcing-project-goals.html)

### Foundation
* [$1M Microsoft Donation to Fund Key Rust Foundation & Project Priorities](https://foundation.rust-lang.org/news/1m-microsoft-donation-to-fund-key-rust-foundation-project-priorities/)

### Newsletters
* [This Month in Rust OSDev: April 2024](https://rust-osdev.com/this-month/2024-04/)
* [This Month in Rust GameDev #50 - April 2024](https://gamedev.rs/news/050/)

### Project/Tooling Updates

* [Zed Decoded: Linux when? - Zed Blog](https://zed.dev/blog/zed-decoded-linux-when)
* [image v0.25: performance improvements, production-ready WebP](https://www.reddit.com/r/rust/comments/1cj94va/image_v025_performance_improvements/)
* [Changelog #232](https://rust-analyzer.github.io/thisweek/2024/05/06/changelog-232.html)
* [rustc_codegen_gcc: Progress Report #32](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-32)
* [Introducing graphql-lint and gqlint](https://grafbase.com/changelog/graphql-lint)
* [Meilisearch releases v1.8](https://blog.meilisearch.com/meilisearch-1-8/) - ([Rust SDK](https://github.com/meilisearch/meilisearch-rust))
* [r3bl_terminal_async v0.5.2 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v052-2020-05-06)

### Observations/Thoughts
* [Pair Your Compilers At The ABI Café - Faultlore](https://faultlore.com/blah/abi-puns/)
* [Unwind considered harmful?](https://smallcultfollowing.com/babysteps/blog/2024/05/02/unwind-considered-harmful/)
* [Async Rust Complexity](https://v5.chriskrycho.com/journal/async-rust-complexity/)
* [Download Accelerator - Async Rust Edition](https://ochagavia.nl/blog/download-accelerator-async-rust-edition/)
* [video] [David Lattimore - A Linker in the Wild](https://www.youtube.com/watch?v=WSHt3-gwVxc)
* [audio] [curl - Daniel Stenberg, Open Source Maintainer and Public Speaker](https://corrode.dev/podcast/s02e01-curl/)

### Rust Walkthroughs
* [How to rewrite a C++ codebase successfully](https://gaultier.github.io/blog/how_to_rewrite_a_cpp_codebase_successfully.html)
* [Building a Redis / Kafka Data Sink in Rust](https://www.sea-ql.org/blog/2024-05-05-redis-kafka-data-sink/)
* [ZH | EN] [Writing a GPT Plugin in Rust, and Lost Gems](https://ideas.reify.ing/en/blog/gpt-plugin-rust-and-lost-gems/)
* [What is in a Rust Allocator?](https://blog.sulami.xyz/posts/what-is-in-a-rust-allocator/)
* [How hard can generating 1024-bit primes really be?](https://glitchcomet.com/articles/1024-bit-primes/)
* [STM32F4 Embedded Rust at the PAC: System Clock Configuration](https://blog.theembeddedrustacean.com/stm32f4-embedded-rust-at-the-pac-system-clock-configuration)
* [video] [Make a port scanner in #rustlang with Tokio and learn async Rust](https://www.youtube.com/watch?v=J3C6sNK2wnk)

### Research
* [VERT: Verified Equivalent Rust Transpilation with Few-Shot Learning](https://arxiv.org/abs/2404.18852)

### Miscellaneous

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)
  * [Testing steps](https://github.com/rust-lang/cargo/issues/13873)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* [CfT: Test out Rustup's `reqwest` backend with `rustls`](https://github.com/rust-lang/rustup/issues/3806)
  * [Testing steps](https://github.com/rust-lang/rustup/issues/3806#issue-2278962476)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Merge RFC 3325: Unsafe attributes](https://github.com/rust-lang/rfcs/pull/3325)
* [Merge RFC 3593: Reserve unprefixed guarded strings](https://github.com/rust-lang/rfcs/pull/3593)
* [Merge RFC 3606: Shorter temp lifetimes in tail exprs](https://github.com/rust-lang/rfcs/pull/3606)
* [Merge RFC 3519: Arbitrary self types v2](https://github.com/rust-lang/rfcs/pull/3519)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [RFC: New range types for Edition 2024](https://github.com/rust-lang/rfcs/pull/3550)
* [disposition: merge] [RFC: cargo-script](https://github.com/rust-lang/rfcs/pull/3502)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [[ptr] Document maximum allocation size](https://github.com/rust-lang/rust/pull/116675)
* [disposition: merge] [Stabilize `min_exhaustive_patterns`](https://github.com/rust-lang/rust/pull/122792)
* [disposition: merge] [Fix #124275: Implemented Default for Arc<\str>](https://github.com/rust-lang/rust/pull/124367)
* [disposition: merge] [elaborate obligations in coherence](https://github.com/rust-lang/rust/pull/124532)
* [disposition: merge] [Tracking Issue for `AtomicBool::fetch_not`](https://github.com/rust-lang/rust/issues/98485)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Function body blocks](https://github.com/rust-lang/rfcs/pull/3629)
* [new] [`async T` and `gen T` types](https://github.com/rust-lang/rfcs/pull/3628)
* [new] [Match ergonomics 2024](https://github.com/rust-lang/rfcs/pull/3627)
* [new] [Extend format_args implicit arguments to allow field access](https://github.com/rust-lang/rfcs/pull/3626)
* [new] [Supertrait item shadowing v2](https://github.com/rust-lang/rfcs/pull/3624)
* [new] [RFC: #[derive(SmartPointer)]](https://github.com/rust-lang/rfcs/pull/3621)

## Upcoming Events

Rusty Events between 2024-05-08 - 2024-06-05 🦀

### Virtual

* 2024-05-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 5 - Project Structure**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325526/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)
* 2024-05-02 | Virtual (Aarhus, DK) | [Rust Aarhus Organizers](https://www.meetup.com/rust-aarhus-organizers/)
    * [**Rust Aarhus Organizers: Status**](https://www.meetup.com/rust-aarhus-organizers/events/300416935/)
* 2024-05-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368804/)
* 2024-05-02 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Women in Rust: Lunch & Learn! (Virtual)**](https://www.meetup.com/women-in-rust/events/300208946/)
* 2024-05-07 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)
* 2024-05-09 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477697/)
* 2024-05-09 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Rust at Microsoft, Tel Aviv - Are we embedded yet?**](https://www.meetup.com/code-mavens/events/300144781/)
* 2024-05-09 | Virtual (Nuremberg/Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945257/)
* 2024-05-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341699/)
* 2024-05-14 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/300437775/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-05-16 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346490/)
* 2024-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477699/)
* 2024-05-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/300533392/)

### Africa

* 2024-05-04 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Asia

* 2024-05-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2024-rustacean-meetup/)

### Europe

* 2024-05-01 | Köln/Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, May**](https://www.meetup.com/rustcologne/events/300610856/)
* 2024-05-01 | Utrecht, NL | [NL-RSE Community](https://nl-rse.org/events/2024-05-01-meetup)
    * [**NL-RSE RUST meetup**](https://www.eventbrite.nl/e/nl-rse-rust-meetup-tickets-871056271757)
* 2024-05-06 | Delft, NL | [GOSIM](https://www.gosim.org/)
    * [**GOSIM Europe 2024**](https://europe2024.gosim.org/)
* 2024-05-07 & 2024-05-08 | Delft, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2024**](https://2024.rustnl.org/)
* 2024-05-07 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**More Rust - Generics, constraints, safety.**](https://www.meetup.com/oxford-rust-meetup-group/events/300567559/)
* 2024-05-08 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/300573716/)
* 2024-05-09 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #2**](https://www.meetup.com/rust-gdansk/events/299766774/)
* 2024-05-14 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn May 2024**](https://www.meetup.com/rust-london-user-group/events/300715979/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-14 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Prague (May 2024)**](https://www.meetup.com/rust-prague/events/300566374/)
* 2024-05-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/299694474/)
* 2024-05-16 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #7**](https://www.meetup.com/rust-meetup-augsburg/events/300174327/)
* 2024-05-16 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #68**](https://mobilizon.fr/events/14b51ccc-211f-400f-9615-707d9d871e78)
* 2024-05-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Save the date - Mai Meetup**](https://www.meetup.com/rust-zurich/events/300513957/)
* 2024-05-22 | Leiden, NL | [Future-proof Software Development by FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Coding Dojo Session**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 2024-05-23 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 2024-05-24 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discussions**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)

### North America

* 2024-05-04 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, May 4**](https://www.meetup.com/bostonrust/events/300116701/)
* 2024-05-08 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Social - Ann Arbor**](https://www.meetup.com/detroitrust/events/300763859/)
* 2024-05-09 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300020003/)
* 2024-05-12 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, May 12**](https://www.meetup.com/bostonrust/events/300116747/)
* 2024-05-14 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/300744140/)
* 2024-05-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 2024-05-20 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, May 20**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)
* 2024-05-25 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk Double Feature**](https://www.meetup.com/deep-dish-rust/events/300665520/)

### Oceania

* 2024-05-02 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**May Meetup**](https://www.meetup.com/rust-brisbane/events/300647409/)

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
