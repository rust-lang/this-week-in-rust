Title: This Week in Rust 543
Number: 543
Date: 2024-04-17
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

### Rust Nation UK
* [Tim McNamara - 4 levels of error handling](https://www.youtube.com/watch?v=hQWZGOuDYTE)
* [Mithun Hunsur - Ambient: A Rust and WebAssembly Runtime for Cross-Platform Multiplayer Games](https://www.youtube.com/watch?v=tSnKx2irwzE)
* [Alice Ryhl - What it takes to keep Tokio running](https://www.youtube.com/watch?v=Lc3aw_NIOt0)
* [Chris Biscardi - Bevy: A case study in ergonomic Rust](https://www.youtube.com/watch?v=CnoDOc6ML0Y&t=51s)
* [Pietro Albini - How Ferrocene qualified the Rust Compiler](https://www.youtube.com/watch?v=_ITnWoPvMKA)
* [Ben Wishovich - Full Stack Rust - Building Rust Websites with Leptos](https://www.youtube.com/watch?v=JJV5crU405s)
* [Natalie Serebryakova - Rustic Persistence: Automating PVC Lifecycles with Rust in Kubernetes](https://www.youtube.com/watch?v=n-ESPxF11tM)
* [Daniel McKenna - Creating a Text-To-Speech System in Rust](https://www.youtube.com/watch?v=HiqId_9pysM)
* [Konstantin Grechishchev - Java and Rust Integration](https://www.youtube.com/watch?v=fYaaBoKbDQs)
* [Heiko Seeberger - EventSourced – async_fn_in_trait in anger](https://www.youtube.com/watch?v=z40rgjZqrs4)
* [Tim Janus - Let's get interdisciplinary: Rust Design Patterns for Chemical Plants](https://www.youtube.com/watch?v=aK5lHOJxl98)
* [Marco Ieni - How Rust makes open-source easier](https://www.youtube.com/watch?v=HzTZoh7WaGo)

### Foundation

### Newsletters
* [New Meshes, New Examples, and Compute Shaders](https://thisweekinbevy.com/issue/2024-04-15-new-meshes-new-examples-and-compute-shaders)

### Project/Tooling Updates
* [futures-concurrency v7.6.0: Portable Concurrent Async Iteration](https://github.com/yoshuawuyts/futures-concurrency/releases/tag/v7.6.0)
* [Ratatui v0.26.2](https://ratatui.rs/highlights/v0262/)
* [Rust on Espressif chips](https://mabez.dev/blog/posts/esp-rust-12-04-2024/)
* `[ZH][EN]` [Announcing async-openai-wasm, and thoughts on wasmization and streams](https://ideas.reify.ing/en/blog/announcing-async-openai-wasm/): Async Rust library for interacting with OpenAI's APIs on WASM and how I did it.

- [Ratatui 0.26.2 is released! - a Rust library for cooking up terminal user interfaces](https://ratatui.rs/highlights/v0262/)
* [Introducing Dust DDS – A native Rust implementation of the Data Distribution Service (DDS) middleware](https://www.s2e-systems.com/2024/04/12/introducing_dust_dds/)
* [Announcing the first audited Rust implementation of CGGMP21, the state-of-the-art ECDSA threshold protocol](https://www.dfns.co/article/cggmp21-in-rust-at-last)
* [Nutype 0.4.2 - newtype with guarantees](https://github.com/greyblake/nutype/releases/tag/v0.4.2)
* [venndb 0.2.1 - any filters](https://github.com/plabayo/venndb/releases/tag/0.2.1)

### Observations/Thoughts
* [Climbing a (binary) Tree - Noise On The Net](https://noiseonthenet.space/noise/2024/04/climbing-a-binary-tree/)
* [Why is there no realloc that takes the number of bytes to copy?](https://shift.click/blog/missing-alloc-api/)
* [Some useful types for database-using Rust web apps](https://boinkor.net/2024/04/some-useful-types-for-database-using-rust-web-apps/)
* [My logging recipe for server side Rust](https://www.matildasmeds.com/posts/rust-logging-recipe/)

### Rust Walkthroughs

* [video] [developerlife.com - Rust testing deep dive with r3bl_terminal_async crate](https://www.youtube.com/watch?v=4iM9t5dgvU4)

### Research
* [Rust Digger: 7.53% of crates have both 'edition' and 'rust-version', 11.21% have neither](https://rust-digger.code-maven.com/news/msrv-stats)

### Miscellaneous
* [Iced Tutorial 0.12](https://leafheap.com/articles/iced-tutorial-version-0-12)
* [video] [Infinite Pong in the Bevy Game Engine - Let's Code!](https://www.youtube.com/watch?v=vwUz95Oo8IA)

* [video] [Marco Ieni - How Rust makes open-source easier (Rust Nation UK)](https://www.youtube.com/watch?v=HzTZoh7WaGo)
* [audio] [Release-plz with Marco Ieni](https://rustacean-station.org/episode/marco-ieni/)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

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

A quiet week, with slightly more improvements than regressions.
There were a few noise spikes, but other than that nothing too interesting.

Triage done by **@Kobzol**.
Revision
range: [86b603cd..ccfcd950b](https://perf.rust-lang.org/?start=86b603cd792b3f6172ba4f676d7b586c1af7630a&end=ccfcd950b333fed046275dd8d54fe736ca498aa7&absolute=false&stat=instructions%3Au)

**Summary**:

|         (instructions:u)          | mean  |     range      | count |
|:---------------------------------:|:-----:|:--------------:|:-----:|
|  Regressions ❌ <br /> (primary)   | 0.5%  |  [0.3%, 1.4%]  |   9   |
| Regressions ❌ <br /> (secondary)  | 0.4%  |  [0.2%, 1.1%]  |  20   |
|  Improvements ✅ <br /> (primary)  | -0.6% | [-2.5%, -0.2%] |  41   |
| Improvements ✅ <br /> (secondary) | -0.8% | [-1.4%, -0.2%] |   4   |
|         All ❌✅ (primary)          | -0.4% | [-2.5%, 1.4%]  |  50   |

1 Regressions, 3 Improvements, 6 Mixed; 5 of them in rollups
62 artifact comparisons made in total

[Full report here](https://github.com/Kobzol/rustc-perf/blob/28ee0f9b94c85d8591588b84a4048f46ab3fe0c2/triage/2024-04-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [Move the Crates.io Team under the Dev Tools team](https://github.com/rust-lang/rfcs/pull/3595)
* [disposition: merge] [Arbitrary self types v2](https://github.com/rust-lang/rfcs/pull/3519)
* [disposition: merge] [RFC: Syntax for embedding cargo-script manifests](https://github.com/rust-lang/rfcs/pull/3503)
* [disposition: merge] [rust-lang org GitHub access policy](https://github.com/rust-lang/rfcs/pull/2872)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Enforce closure args + return type are WF](https://github.com/rust-lang/rust/pull/123531)
* [disposition: merge] [Tracking Issue for `io_error_downcast`](https://github.com/rust-lang/rust/issues/99262)
* [disposition: merge] [More DefineOpaqueTypes::Yes](https://github.com/rust-lang/rust/pull/123794)
* [disposition: merge] [Tracking Issue for `std::path::absolute`](https://github.com/rust-lang/rust/issues/92750)
* [disposition: merge] [Tracking Issue for `utf8_chunks`](https://github.com/rust-lang/rust/issues/99543)
* [disposition: merge] [restrict promotion of `const fn` calls](https://github.com/rust-lang/rust/pull/121557)
* [disposition: merge] [Fix trait solver overflow with `non_local_definitions` lint](https://github.com/rust-lang/rust/pull/123594)
* [disposition: merge] [Use fulfillment in method probe, not evaluation ](https://github.com/rust-lang/rust/pull/122317)
* [disposition: merge] [rustdoc-search: single result for items with multiple paths](https://github.com/rust-lang/rust/pull/119912)
* [disposition: merge] [Ignore `-C strip` on MSVC](https://github.com/rust-lang/rust/pull/115120)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2024-04-17 - 2024-05-15 🦀

### Virtual

* 2024-04-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Reflections on RustNation UK 2024**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325512/)
* 2024-04-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Camigo (Peter Kehl)**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-04-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368799/)
* 2024-04-21 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Using AstroNvim for Rust development (in Hebrew)**](https://www.meetup.com/code-mavens/events/300265648/)
* 2024-04-23 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Show and Tell in April**](https://www.meetup.com/rust-trondheim/events/300469130/)
* 2024-04-24 | Virtual + In Person (Prague, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**#2: Making Safe Rust Safer (Pavel Šimerda)**](https://www.meetup.com/rust-czech-republic/events/300388563)
* 2024-04-25 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477692/)
* 2024-04-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcgbnc/)
* 2024-05-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 5 - Project Structure**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300325526/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)
* 2024-05-02 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368804/)
* 2024-05-07 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300100279/)
* 2024-05-09 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477697/)
* 2024-05-09 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/) 
    * [**Rust at Microsoft, Tel Aviv - Are we embedded yet?**](https://www.meetup.com/code-mavens/events/300144781/)
* 2024-05-09 | Virtual (Nuremberg/Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945257/)
* 2024-05-14| Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/298341699/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)

### Africa

* 2024-05-04 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdsoporgprofile)

### Asia

* 2024-04-20 | Kuala Lumpur, MY | [GoLang Malaysia](https://t.me/golangmalaysia)
    * [**Rust Talk & Workshop - Parallel Programming April 2024**](https://docs.google.com/forms/d/e/1FAIpQLSfeWzcnWic--G2Sj6uJFJNc_L2Iv7J27hIofZwhBYXu2CbUjQ/viewform) | [Event updates Telegram](https://t.me/+dF46Fly4A_BjOTJl) | [Event group chat](https://t.me/golangmalaysia)
* 2024-05-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2024-rustacean-meetup/)

### Europe

* 2024-04-17 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/)
    * [**Lær Rust med Conways Game of Life**](https://www.meetup.com/bergen-html-css-meetup-group/events/300031586/)
* 2024-04-17 | Lyon, FR | [Rust Lyon](https://www.meetup.com/rust-lyon/)
    * [**Rust Lyon Meetup #10**](https://www.meetup.com/rust-lyon/events/300268616/)
* 2024-04-17 | Ostrava, CZ | [TechMeetup Ostrava](https://www.meetup.com/techmeetupostrava/)
    * [**TechMeetup: RUST**](https://www.meetup.com/techmeetupostrava/events/299912212/)
* 2024-04-20 | Augsburg, DE | [Augsburger Linux-Infotag 2024](https://www.luga.de/static/LIT-2024/)
   * [**Augsburger Linux-Infotag 2024: Workshop Einstieg in Embedded Rust mit dem Raspberry Pico WH**](https://www.luga.de/static/LIT-2024/talks/einstieg_in_embedded_rust_mit_dem_raspberry_pico_wh/)
* 2024-04-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust'n'Tell - Rust for the Web**](https://www.meetup.com/rust-berlin/events/300047151/)
* 2024-04-23 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #67**](https://mobilizon.fr/events/4ba93021-c43a-4e4a-b3e5-09c1c0d0a957)
* 2024-04-24 | Virtual + In Person (Prague, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**#2: Making Safe Rust Safer (Pavel Šimerda)**](https://www.meetup.com/rust-czech-republic/events/300388563)
* 2024-04-25 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at MFT Energy**](https://www.meetup.com/rust-aarhus/events/299564517/)
* 2024-04-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - TBD**](https://www.meetup.com/rust-berlin/events/299288960/)
* 2024-04-25 | København/Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust meetup #46 sponsored by Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/300458178/)
* 2024-04-25 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna x Python User Group - April**](https://www.meetup.com/rust-vienna/events/300389154/)
* 2024-04-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Fullstack Rust - Workshop #2 (Register by 23 April)**](https://www.meetup.com/rust-basel/events/299933581/)
* 2024-04-27 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #2**](https://www.meetup.com/stockholm-rust/events/300369409)
* 2024-04-30 | Budapest, HU | [Budapest Rust Meetup Group](https://www.meetup.com/budapest-rust-meetup-group/)
    * [**Rust Meetup Budapest 2**](https://www.meetup.com/budapest-rust-meetup-group/events/300269044/)
* 2024-04-30 | Salzburg, AT | Rust Salzburg
    * [**Rust Salzburg meetup**]: 6:30pm - CCC Salzburg, 1. OG, ArgeKultur, Ulrike-Gschwandtner-Straße 5, 5020 Salzburg
* 2024-05-01 | Utrecht, NL | [NL-RSE Community](https://nl-rse.org/events/2024-05-01-meetup)
    * [**NL-RSE RUST meetup**](https://www.eventbrite.nl/e/nl-rse-rust-meetup-tickets-871056271757)
* 2024-05-06 | Delft, NL | [GOSIM](https://www.gosim.org/)
    * [**GOSIM Europe 2024**](https://europe2024.gosim.org/)
* 2024-05-07 & 2024-05-08 | Delft, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2024**](https://2024.rustnl.org/)
* 2024-05-09 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #2**](https://www.meetup.com/rust-gdansk/events/299766774/)
* 2024-05-14 | Virtual + In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid (Rescheduled)**](https://www.meetup.com/rust-munich/events/298507657/)

### North America

* 2024-04-18 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk: What Are Panics?**](https://www.meetup.com/deep-dish-rust/events/300204763/)
* 2024-04-18 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803586/)
* 2024-04-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299960315/)
* 2024-04-25 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers - Async Rust on Embedded**](https://www.meetup.com/music-city-rust-developers/events/299976876/)
* 2024-04-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Apr 26**](https://www.meetup.com/bostonrust/events/300116689/)
* 2024-05-04 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, May 4**](https://www.meetup.com/bostonrust/events/300116701/)
* 2024-05-12 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, May 12**](https://www.meetup.com/bostonrust/events/300116747/)

### Oceania

* 2024-04-17 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**WMaTIR 2024 Gala & Talks**](https://www.meetup.com/rust-sydney/events/299882966/)
* 2024-04-30 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Why Rust? Convince Me!**](https://www.meetup.com/rust-akl/events/300304958/)
* 2024-04-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**CRUG April Meetup: Generics and Traits**](https://www.meetup.com/rust-canberra/events/300023000/)

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
