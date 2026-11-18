Title: This Week in Rust 678
Number: 678
Date: 2026-11-18
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

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

### Rust Walkthroughs

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
* [AppBridge - [easy] Add CLI subcommand `appbridge delta` to inspect byte savings between two .appbridge bundles](https://github.com/Kolgrim33/appbridge/issues/1)
* [AppBridge - [medium] Support real Ed25519 asymmetric keypair generation and verification for .appbridge bundles](https://github.com/Kolgrim33/appbridge/issues/2)
<!-- * [ - ]() -->
* [presswork - Hardware matrix: run the IPP conformance suite against real printers](https://github.com/vindeckyy/presswork/issues/6)
* [presswork - Expand ipptool conformance matrix to remaining upstream .test files](https://github.com/vindeckyy/presswork/issues/7)

* [Praxi-Bot - Add shell completions and man pages for the Rust CLI (easy)](https://github.com/emreocell/Praxi-Bot/issues/235)
* [Praxi-Bot - Extend the deterministic natural-language schedule parser (medium)](https://github.com/emreocell/Praxi-Bot/issues/188)

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

A generally positive week, with improvements significantly outweighing
regressions for an average of -1.1% instructions on primary benchmarks.

Triage done by **@simulacrum**.
Revision range: [54d0b7ed..6ca2b346](https://perf.rust-lang.org/?start=54d0b7edad511e84181ad50ec8413ef10f93b367&end=6ca2b34602bf2f3591ffa7359c166b6409a44b0a&absolute=false&stat=instructions%3Au)

5 Regressions, 12 Improvements, 4 Mixed; 0 of them in rollups
147 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/main/triage/2026/2026-11-15.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Associated const underscore](https://github.com/rust-lang/rfcs/pull/3527)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [debugger: stabilize trim-paths unremap file loading](https://github.com/rust-lang/rust/pull/163910)
* [Stabilization of the `documentation` scope in `rustc`](https://github.com/rust-lang/rust/issues/163896)
* [Stabilize `char::is_case_ignorable`, `char::is_cased`, and the `titlecase` feature](https://github.com/rust-lang/rust/pull/165434)
* [std: use `_exit` for `process::exit`](https://github.com/rust-lang/rust/pull/165125)
* [Add lint against duplicate parameter names in fns without body](https://github.com/rust-lang/rust/pull/165366)
* [Stabilize unchecked `nonzero_ops`](https://github.com/rust-lang/rust/pull/164485)
* [Update `PartialEq`/`PartialOrd` guarantees for comparisons to match reality](https://github.com/rust-lang/rust/pull/164473)
* [Move lint validation to attribute parsing](https://github.com/rust-lang/rust/pull/164918)
* [parser: Feature-gate ambiguous attributes on expressions (take 2)](https://github.com/rust-lang/rust/pull/160235)
* [make conflicting_repr_hints a hard error](https://github.com/rust-lang/rust/pull/165238)
* [Make unsafe unknown diagnostic attributes an error again](https://github.com/rust-lang/rust/pull/165782)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Promote powerpc64-unknown-linux-gnuelfv2 to tier 2 with host tools](https://github.com/rust-lang/compiler-team/issues/1049)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [document that extern statics may be bigger than the declared size](https://github.com/rust-lang/reference/pull/2334)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Funding team charter](https://github.com/rust-lang/rfcs/pull/4029)

## Upcoming Events

Rusty Events between 2026-11-18 - 2026-12-16 🦀

### Virtual
* 2026-11-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619619/)
* 2026-11-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563020/)
* 2026-11-17 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Miscompilation in Rust 1.97**](https://luma.com/2qfai4a8)
* 2026-11-17 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: The Harness Wore a Trenchcoat - Building a Rust MCP Server**](https://www.meetup.com/women-in-rust/events/315719313/)
* 2026-11-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/317182776/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907781/)
* 2026-11-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254769/)
* 2026-11-26 | Virtual | [betterCode() Rust](https://rust.bettercode.eu/)
    * [**betterCode() Rust 2026**](https://rust.bettercode.eu/)
* 2026-12-01 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Cursed Rust**](https://luma.com/omlooqv8)
* 2026-12-01 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Christmas Coding Challenge - Kick Off!**](https://www.meetup.com/women-in-rust/events/316993134/)
* 2026-12-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcqbdb/)
* 2026-12-03 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907782/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)
* 2026-12-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331278/)
* 2026-12-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254767/)
* 2026-12-08 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/316993086/)

### Asia
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**November 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europe
* 2026-11-12 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Debugging in Rust**](https://www.meetup.com/rust-rhein-main/events/317175844/)
* 2026-11-12 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-11-12 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano/events/)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-language-milan/events/317094800/)
* 2026-11-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Everything Is Git, Protocols Are Just Adapters**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)
* 2026-11-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at EIVA**](https://www.meetup.com/rust-aarhus/events/316938696/)
* 2026-11-19 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust: FFI with C++, and Intro to Async**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/317045463/)
* 2026-11-20 - 2026-11-21 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Embedded Rust mit Embassy - Workshop und Projekttag**](https://my.tuxedocomputers.com/event/embedded-rust-mit-embassy-workshop-und-projekttag-28/register)
* 2026-11-24 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #17 @ ERNI**](https://www.meetup.com/rust-basel/events/317258865/)
* 2026-11-25 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ Reclamefolder**](https://www.meetup.com/rust-amsterdam-group/events/317381204/)
* 2026-11-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #73 sponsored by Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/317181792/)
* 2026-11-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**Rust London x Zed**](https://www.meetup.com/rust-london-user-group/events/317253958/)
* 2026-11-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/317222338/)
* 2026-11-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Lovable**](https://www.meetup.com/stockholm-rust/events/317161063/)
* 2026-12-01 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #90**](https://www.meetup.com/rust-paris/events/316865676/)
* 2026-12-02 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-rhein-main/events/317186779/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)

### North America
* 2026-11-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Rust Game Jam**](https://www.meetup.com/utah-rust/events/317178211/)
* 2026-11-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust November Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316886787/)
* 2026-11-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, Nov 14**](https://www.meetup.com/bostonrust/events/316924331/)
* 2026-11-17 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA November w/ Hadrian!**](https://www.meetup.com/rust-los-angeles/events/316859673/)
* 2026-11-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783989/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-18 | San Francisco, CA, US) | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Social Night**](https://luma.com/v0mxbgs8)
* 2026-11-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Winter Hill Rust Lunch, Nov 21**](https://www.meetup.com/bostonrust/events/316924366/)
* 2026-11-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**South Station Rust Lunch, Nov 28**](https://www.meetup.com/bostonrust/events/316924412/)
* 2026-12-04 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: The UnConf 2026.**](https://www.meetup.com/rust-nyc/events/317259308/)
* 2026-12-04 | New York, NY, US | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋Women in Rust x Rust NYC UnConf 2026**](https://www.meetup.com/women-in-rust/events/317383331/)

### Oceania
* 2026-11-12 | Brisbane City, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane • November 2026**](https://www.meetup.com/rust-brisbane/events/317296037/)
* 2026-11-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/316907340/)

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
