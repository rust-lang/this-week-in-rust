Title: This Week in Rust 552
Number: 552
Date: 2024-06-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Announcing Rust 1.79.0](https://blog.rust-lang.org/2024/06/13/Rust-1.79.0.html)
* [This Development-cycle in Cargo: 1.80](https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html)

### Foundation
* [Announcing the Safety-Critical Rust Consortium](https://foundation.rust-lang.org/news/announcing-the-safety-critical-rust-consortium/)

### Newsletters

### Project/Tooling Updates
* [rust-analyzer changelog #238](https://rust-analyzer.github.io/thisweek/2024/06/17/changelog-238.html)
* [Enhancing Code Completion for Rust in Cody](https://sourcegraph.com/blog/enhancing-code-completion-for-rust-in-cody)
* [Apache Datafusion-Comet- Rust-Based Apache Spark accelerator - 1.5x performance improvements in apache spark TPC-H queries, and other commodity hardware](https://github.com/apache/datafusion-comet)
* [Fluvio Distributed Streaming Engine Release 0.11.9](https://www.fluvio.io/news/this-week-in-fluvio-0062/)

### Observations/Thoughts
* [Path Generics in Rust: A Sketch Proposal for Simplicity and Generality](https://cfallin.org/blog/2024/06/12/rust-path-generics/)
* [Dioxus Labs + “High-level Rust”](https://dioxus.notion.site/Dioxus-Labs-High-level-Rust-5fe1f1c9c8334815ad488410d948f05e)
* [Making Your First Real-World Rust Project a Success](https://corrode.dev/blog/successful-rust-business-adoption-checklist/)
* [Future's liveness problem](https://skepfyr.me/blog/futures-liveness-problem/)
* [Optimizing Rust code with Flamegraph and DHAT – a practical example with Dust DDS](https://www.s2e-systems.com/2024/06/13/optimizing_rust_code/)
* [video] [Compiler-Driven Development in Rust](https://www.youtube.com/watch?v=Kdpfhj3VM04)
* [audio] [What's New in Rust 1.72 and 1.73](https://rustacean-station.org/episode/rust-1.72-1.73/)
* [audio] [Matic with Eric Seppanen](https://corrode.dev/podcast/s02e04-matic/)
* [video] [Rust 1.79.0: Top 10 Most Interesting Things](https://youtu.be/u5WD5Ta09vs)

### Rust Walkthroughs
* [Rust Ownership Explained: Merging Linked Lists](https://www.howtocodeit.com/articles/rust-ownership-explained-linked-lists)
* [Zero to Performance Hero: How to Benchmark and Profile Your eBPF Code in Rust](https://www.infoq.com/articles/benchmark-profile-ebpf-code/)
* [IPC in Rust - a Ping Pong Comparison](https://3tilley.github.io/posts/simple-ipc-ping-pong/)
* [I ported h2spec to Rust (also: codegen!)](https://fasterthanli.me/videos/h2spec-to-rust)
* [Parsing Python ASTs 20x Faster with Rust](https://www.gauge.sh/blog/parsing-python-asts-20x-faster-with-rust)
* [Making a const version of Rust's array::from_fn - How hard can it be?](https://gendignoux.com/blog/2024/06/17/const-array-from-fn.html)
* [Using Tauri to build a cross-platform security app](https://www.firezone.dev/blog/using-tauri)
* [Generate and package Rust client SDKs with Buf](https://buf.build/blog/bsr-generated-sdks-for-rust)
* [series] [Master Hexagonal Architecture in Rust (part 2): Separation of Concerns, Rust-Style](https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust#separation-of-concerns-rust-style)

### Research
* [SquirrelFS: using the Rust compiler to check file-system crash consistency](https://arxiv.org/abs/2406.09649)
* [Trusting code in the wild: Exploring contributor reputation measures to review dependencies in the Rust ecosystem](https://arxiv.org/abs/2406.10317)

### Miscellaneous

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No calls for testing were issued this week.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.
<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
* [Dusk - Archival Nodes APIs Infrastructure RFP](https://docs.dusk.network/grants/rfps/archival_node)
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [Rust Ukraine 2024](https://docs.google.com/forms/d/e/1FAIpQLSc9S_95oaCsFyrULF4iBQOIiTcMlOpG07izgquYLBCKFAYTKQ/viewform) | Closes 2024-07-06 | Online + Ukraine, Kyiv | Event date: 2024-07-27
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Closes 2024-07-22 | online | Event date: 2024-08-22

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

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

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [UnsafePinned: allow aliasing of pinned mutable references](https://github.com/rust-lang/rfcs/pull/3467)
* [disposition: postpone] [RFC: make Cargo embed dependency versions in the compiled binary](https://github.com/rust-lang/rfcs/pull/2801)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [RFC: Return Type Notation](https://github.com/rust-lang/rfcs/pull/3654)
* [disposition: merge] [RFC: Unblock Cargo feature metadata](https://github.com/rust-lang/rfcs/pull/3416)
* [disposition: postpone] [Allow requiring "at least one feature"](https://github.com/rust-lang/rfcs/pull/3347)
* [disposition: postpone] [Add a general mechanism of setting RUSTFLAGS in Cargo for the root crate only](https://github.com/rust-lang/rfcs/pull/3310)
* [disposition: close] [Allow specifying dependencies for individual artifacts](https://github.com/rust-lang/rfcs/pull/2887)
* [disposition: postpone] [RFC: make Cargo embed dependency versions in the compiled binary](https://github.com/rust-lang/rfcs/pull/2801)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team RFCs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [repr(type) for type aliases](https://github.com/rust-lang/rfcs/pull/3659)
* [new] [Proposed 2024h2 flagship goal: Rust for Linux](https://github.com/rust-lang/rfcs/pull/3658)
* [new] [Async project goal](https://github.com/rust-lang/rfcs/pull/3657)

## Upcoming Events

Rusty Events between 2024-06-19 - 2024-07-17 🦀

### Virtual

* 2024-06-12 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 8 - Asynchronous Programming**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-06-13 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 2024-06-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 2024-06-16 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Workshop: Web development in Rust using Rocket (English)**](https://www.meetup.com/code-mavens/events/301294669/)
* 2024-06-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346963/)
* 2024-06-19 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477705/)
* 2024-06-25 | Virtual (Dallas, TX, US)| [Dallas Rust User Group](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)
* 2024-06-25 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Using the Liquid template system in Rust (English)**](https://www.meetup.com/code-mavens/events/301487547/)
* 2024-06-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 2024-07-02 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 2024-07-03 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Build Web Apps with Rust and Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 2024-07-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328025/)
* 2024-07-04 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488820/)
* 2024-07-06 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-07-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346976/)
* 2024-07-10 | Virtual | [Centre for eResearch](https://www.eventbrite.co.nz/o/centre-for-eresearch-75893560993)
    * [**Research Computing With The Rust Programming Language**](https://www.eventbrite.com/e/research-computing-with-the-rust-programming-language-tickets-908002037537?aff=ebdssbdestsearch&keep_tld=1)
* 2024-07-11 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Reading JSON files in Rust (English)**](https://www.meetup.com/code-mavens/events/301636580/)
* 2024-07-16 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Web development in Rust using Rocket - part 2 (English)**](https://www.meetup.com/code-mavens/events/301736709/)

### Asia
* 2024-06-22 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**June 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/june-2024-rustacean-meetup/)
* 2024-06-23 | Tel Aviv, Israel | [Rust in Israel](https://www.meetup.com/rust-in-israel/)
    * [**Rust at Microsoft Tel Aviv in June 2024**](https://www.meetup.com/rust-in-israel/events/301670916/)
* 2024-06-30 | Kyoto, JP | [Kyoto Rust](https://www.meetup.com/kyoto-rust/)
    * [**Rust Talk: Cross Platform Apps**](https://www.meetup.com/kyoto-rust/events/301499550/)

### Europe

* 2024-06-18 | Frankfurt/Main, DE | [Rust Frankfurt Meetup](https://www.meetup.com/rust-frankfurt)
    * [**Rust Frankfurt is Back!**](https://www.meetup.com/rust-frankfurt/events/301441434/)
* 2024-06-19 - 2024-06-24 | Zürich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 2024-06-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)
* 2024-06-25 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #3**](https://www.meetup.com/rust-gdansk/events/301014697/)
* 2024-06-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288965/)
* 2024-06-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)
* 2024-07-10 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup - July**](https://www.meetup.com/reading-rust-workshop/events/301359031/)

### North America

* 2024-06-12 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Detroit Rust Meet - Ann Arbor**](https://www.meetup.com/detroitrust/events/301387848/)
* 2024-06-13 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Crafting an Interpreter in Rust, pt. 1**](https://www.meetup.com/spokane-rust/events/300020010/)
* 2024-06-14 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Summer BBQ for Spokane's Local Tech User Groups at Saranac Pub Rooftop!**](https://www.meetup.com/spokane-rust/events/301569401/)
* 2024-06-17 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/301411625/)
* 2024-06-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)
* 2024-06-19 | Hybrid -  Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)
* 2024-06-24 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch, June 24**](https://www.meetup.com/bostonrust/events/301549722/)
* 2024-06-26 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 2024-06-27 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)
* 2024-07-05 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston University Rust Lunch, July 5**](https://www.meetup.com/bostonrust/events/301549737/)

### Oceania

* 2024-06-14 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**June 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/301311680/)
* 2024-06-20 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Full Stack Rust + Writing a compiler for fun and (no) profit**](https://www.meetup.com/rust-akl/events/301193761/)
* 2024-06-25 | Canberra, ACt, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/300749371/)

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
