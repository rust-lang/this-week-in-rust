Title: This Week in Rust 646
Number: 646
Date: 2026-04-08
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

### Project/Tooling Updates

* [Processing 1M Chess Games in 15 Seconds with Rust](https://dev.to/chrnx/processing-1m-chess-games-in-15-seconds-with-rust-pe3)
* [Dumap v1.1: Cross-platform disk usage treemap visualization](https://github.com/jrobhoward/dumap/releases/tag/v1.1.0)
* [Proxelar 0.4.0: Intercept & Modify Traffic](https://micheletti.io/proxelar-040/)
* [amoxide: composable, context-aware shell aliases](https://d34dl0ck.me/amoxide-composable-context-aware-shell-aliases/index.html)
* [Ply 1.1: Building Polished UIs in Rust](https://plyx.iz.rs/blog/ply-1-1/)

### Observations/Thoughts

- [800 Rust terminal projects in 3 years](https://blog.orhun.dev/800-rust-projects/)

### Rust Walkthroughs
* [Learn Rust Basics By Building a Brainfuck Interpreter](https://blog.sheerluck.dev/posts/learn-rust-basics-by-building-a-brainfuck-interpreter/)

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

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

A shorter week than normal (probably due to later perf triage last week).
Overall fairly small changes scattered across various PRs, though the net
effect was slightly positive (-0.5% avg change). All changed ended up either
mixed or improvements this week.

Triage done by **@simulacrum**.
Revision range: [cf7da0b7..e73c56ab](https://perf.rust-lang.org/?start=cf7da0b7277cad05b79f91b60c290aa08a17a6f0&end=e73c56abd0baf3dbaafbdc3ce6072a416aade867&absolute=false&stat=instructions%3Au)

0 Regressions, 3 Improvements, 8 Mixed; 5 of them in rollups
26 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2026/2026-04-05.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [report the `varargs_without_pattern` lint in deps](https://github.com/rust-lang/rust/pull/154599)
* [Partially stabilize LoongArch target features](https://github.com/rust-lang/rust/pull/154510)
* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [cargo clean: Add target directory validation](https://github.com/rust-lang/cargo/pull/16712)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [Propose the concept of a crates.io username for identity](https://github.com/rust-lang/rfcs/pull/3946)
* [RFC: Inheriting of default-features in Cargo](https://github.com/rust-lang/rfcs/pull/3945)
* [Add Bitbucket Cloud OAuth login for crates.io](https://github.com/rust-lang/rfcs/pull/3944)
* [MIR move elimination](https://github.com/rust-lang/rfcs/pull/3943)

## Upcoming Events

Rusty Events between 2026-04-08 - 2026-05-06 🦀

### Virtual
* 2026-04-09 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455926/)
* 2026-04-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254784/)
* 2026-04-14 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506013/)
* 2026-04-14 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Web development using axum in Rust - part 3**](https://www.meetup.com/code-mavens/events/314072969/)
* 2026-04-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-15 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/jia7wtfv)
* 2026-04-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**April, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-19 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/313846195/)
* 2026-04-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/314007434/)
* 2026-04-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/26dvwb85)
* 2026-04-23 | Virtual (Amsterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development)
    * [**Bevy Meetup #13**](https://www.meetup.com/bevy-game-development/events/313842977/)
* 2026-04-23 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455927/)
* 2026-04-28 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254783/)
* 2026-04-28 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: From Protobuf to Production - A Guide to gRPC in Rust**](https://www.meetup.com/women-in-rust/events/313505777/)
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/8hi2xywi)
* 2026-05-01 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Hacker's Hike 0x1**](https://www.meetup.com/rust-noris/events/312788983/)
* 2026-05-02 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763928837)
* 2026-05-03 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314036479/)
* 2026-05-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjchbjb/)

### Asia
* 2026-04-11 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**April 2026/Pre-Conference Rustacean meetup**](https://hasgeek.com/rustbangalore/april-2026-pre-conference-rustacean-meetup/)
* 2026-04-17 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Rust India Workshop**](https://rustindia.org/schedule)
* 2026-04-18 | Bangalore, IN | [Rust India](https://rustindia.org/)
    * [**Rust India Conference**](https://rustindia.org/schedule)

### Europe
* 2026-04-08 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 04 2026**](https://luma.com/z8aoscr9)
* 2026-04-09 | Geneva, CH | [Rust Meetup Geneva](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-04-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust talks @ AutoStore – "Patterns for Event Driven Systems" and "Rust + WASM"**](https://www.meetup.com/rust-oslo/events/313806765/)
* 2026-04-21 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Native GUIs with Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813853/)
* 2026-04-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night and Birthday Party at MFT Energy**](https://www.meetup.com/rust-aarhus/events/313910468/)
* 2026-05-04 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a stock portfolio simulation in Rust with Leptos**](https://www.meetup.com/rust-rhein-main/events/314051688/)

### North America
* 2026-04-09 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/313987321/)
* 2026-04-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Utah Rust April Meetup**](https://www.meetup.com/utah-rust/events/314156736/)
* 2026-04-09 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal)
    * [**April Monthly Social**](https://www.meetup.com/rust-montreal/events/313872135/)
* 2026-04-09 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust)
    * [**Hexadecimal Floating Point? Why? How?**](https://www.meetup.com/pdxrust/events/314159131/)
* 2026-04-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust April Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721879/)
* 2026-04-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Brookline Rust Lunch, Apr 11**](https://www.meetup.com/bostonrust/events/313883710/)
* 2026-04-14 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Sharpening Your Rust Skills for Job Interviews**](https://www.meetup.com/charlottesville-rust-meetup/events/313262215/)
* 2026-04-15 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Nushell**](https://www.meetup.com/vancouver-rust/events/313471712/)
* 2026-04-16 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**April, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873177/)
* 2026-04-16 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313995065/)
* 2026-04-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Community Meetup**](https://www.meetup.com/music-city-rust-developers/events/314090462/)
* 2026-04-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, Apr 18**](https://www.meetup.com/bostonrust/events/313883701/)
* 2026-04-20 - 2026-04-22 | Portland, OR | [Tokio](https://tokio.rs/)
    * [**TokioConf 2026**](https://www.tokioconf.com/)
* 2026-04-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/313918531/)
* 2026-04-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/314000435/)
* 2026-04-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Formally Verified Rust & SAT Solvers**](https://www.meetup.com/rust-nyc/events/314167944/)
* 2026-04-23 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA April!**](https://www.meetup.com/rust-los-angeles/events/313542139/)
* 2026-04-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**South Station Rust Lunch, Apr 25**](https://www.meetup.com/bostonrust/events/313883704/)
* 2026-04-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228662/)

### Oceania
* 2026-04-09 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane Apr 2026**](https://www.meetup.com/rust-brisbane/events/313975190/)

### South America
* 2026-04-11 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://www.meetup.com/rust-argentina/events/313857456/)
* 2026-04-11 | Buenos Aires, AR | [Oxidar Org](https://luma.com/user/oxidar)
    * [**Oxidar.org Hackaton - Snakear - ¡Veni a hackear con Rust!**](https://luma.com/5f51ey45)
* 2026-04-17 | Rio de Janeiro, BR | [Meetups Rust RJ](https://luma.com/calendar/cal-z65k0aMSe7DaqKv)
    * [**Meetup Rust RJ**](https://luma.com/ce46pl7z)

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
