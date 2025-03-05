Title: This Week in Rust 589
Number: 589
Date: 2025-03-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X (formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Foundation

### Newsletters
* [Rust Trends Issue #60](https://rust-trends.com/newsletter/from-dns-servers-to-json-speed-what-s-new-in-rust/)
* [The Embedded Rustacean Issue #40](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-40)

### Project/Tooling Updates

### Observations/Thoughts
* [Some things that make Rust lifetimes hard to learn](https://ntietz.com/blog/rust-lifetimes-learning/)
* [video] [Rust Under the Hood](https://www.youtube.com/watch?v=L8caNpK3Shs)
* [video] [9 Rules for Porting Rust to the Browser](https://www.youtube.com/watch?v=i6dahKSnuUg)

* [Cargo's missing stability guarantees](https://blog.weiznich.de/blog/cargo-instablity/)

### Rust Walkthroughs
* [The power of interning: making a time series database 2000x smaller in Rust](https://gendignoux.com/blog/2025/03/03/rust-interning-2000x.html)
* [Building a DNS Server in Rust: Part 1 of 2](https://rust-trends.com/posts/building-a-dns-server-in-rust/)

### Research

### Miscellaneous
* [video] [Creating a website on GitHub pages with mdBook](https://www.youtube.com/watch?v=x3vF9YiWBMQ)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer
and would like your RFC to appear in this list, add a `call-for-testing` label to your RFC along
with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

Let us know if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

A pretty quiet week, with minimal changes in performance (positive or negative).

Triage done by **@simulacrum**.
Revision range: [f5729cfe..daf59857](https://perf.rust-lang.org/?start=f5729cfed3c45e061e8a443677fc1d5ef9277df7&end=daf59857d6d2b87af4b846316bf1561a6083ed51&absolute=false&stat=instructions%3Au)

1 Regressions, 4 Improvements, 1 Mixed; 2 of them in rollups
29 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-03-03.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Denote `ControlFlow` as `#[must_use]`](https://github.com/rust-lang/rust/pull/137449)
* [Turn order dependent trait objects future incompat warning into a hard error](https://github.com/rust-lang/rust/pull/136968)
* [Stabilize `const_vec_string_slice`](https://github.com/rust-lang/rust/pull/137319)
* [add a "future" edition](https://github.com/rust-lang/rust/pull/137606)
* [Tracking Issue for const_sockaddr_setters](https://github.com/rust-lang/rust/issues/131714)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Deprecate the per-build-target `edition` field in `Cargo.toml`](https://github.com/rust-lang/rfcs/pull/3772)
* [RFC: Demote i686-pc-windows-gnu to Tier 2](https://github.com/rust-lang/rfcs/pull/3771)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [feat(package): add --exclude-lockfile flag](https://github.com/rust-lang/cargo/pull/15234)

#### Other Areas
* *No Items entered Final Comment Period this week for
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Local Default Bounds to assist `Forget` and other `?Trait`.](https://github.com/rust-lang/rfcs/pull/3783)
* [`Forget` marker trait](https://github.com/rust-lang/rfcs/pull/3782)
* [RFC: Crate changelog field](https://github.com/rust-lang/rfcs/pull/3779)

## Upcoming Events

Rusty Events between 2025-03-05 - 2025-04-02 🦀

### Virtual
* 2025-02-27 | Virtual (US) | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Intro to Rust**](https://www.eventbrite.com/e/intro-to-rust-tickets-1237517059839)
* 2025-02-27 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820295)
* 2025-02-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Parsing command line options with category theory and async**](https://www.meetup.com/charlottesville-rust-meetup/events/305948365)
* 2025-03-01 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-62876317658/)
* 2025-03-04 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Building Integration Libraries and parsing with Winnow in Rust with Kenny Flegal**](https://www.meetup.com/code-mavens/events/305793122/)
* 2025-03-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031659)
* 2025-03-06 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820281/)
* 2025-03-06 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #9**](https://www.meetup.com/bevy-game-development/events/306131557)
* 2025-03-06 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Ratatui - Terminal User Interfaces in Rust with Orhun Parmaksız**](https://www.meetup.com/code-mavens/events/305750365/)
* 2025-03-09 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Creating A Mock Blockchain in Rust with Sourav Mishra**](https://www.meetup.com/code-mavens/events/305587087/)
* 2025-03-09 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Becoming a Rust Champion: Leading Your Team to Success with Yoni Peleg**](https://www.meetup.com/rust-tlv/events/306396549)
* 2025-03-11 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/303522529)
* 2025-03-11 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/305716839)
* 2025-03-13 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820296)
* 2025-03-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170694)
* 2025-03-18 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**crum: Complex Numbers and Complex Matrices in Rust with Frans Slabber**](https://www.meetup.com/code-mavens/events/305823397/)
* 2025-03-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Bacon & Performance Benchmarking**](https://www.meetup.com/vancouver-rust/events/305470139)
* 2025-03-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361431)
* 2025-03-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: SKIing into Rust - crafting a simple interpreter**](https://www.meetup.com/women-in-rust/events/305988711)

### Asia
* 2025-03-01 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**Zig & Rust Meetup**](https://lu.ma/460w8v58)
* 2025-03-19 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust March 2025 at Jit in Tel Aviv**](https://www.meetup.com/rust-tlv/events/305697580)

### Europe
* 2025-02-26 | Darmstadt, DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**Rust Compiler Tuning**](https://www.meetup.com/rust-rhein-main/events/305990886/)
* 2025-02-26 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2025**](https://lu.ma/iwu6mlcj)
* 2025-02-26 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/events/)
    * [**Rust Vienna - February | at metalab 🦀**](https://www.meetup.com/rust-vienna/events/306274608)
* 2025-02-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #55 sponsored by Veo**](https://www.meetup.com/copenhagen-rust-community/events/306262465)
* 2025-02-27 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809675)
* 2025-02-27 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #75**](https://www.meetup.com/rust-paris/events/305791655)
* 2025-02-28 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Rust Web Workshop**](https://www.meetup.com/london-rust-project-group/events/306155563)
* 2025-03-01 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Technikmuseum Speyer**](https://www.meetup.com/rust-noris/events/305361732/)
* 2025-03-05 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**17th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/305887675)
* 2025-03-07 | Prague, CZ | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/events/)
    * [**Rust meetup in Braiins offices**](https://www.meetup.com/rust-czech-republic/events/306237623)
* 2025-03-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045445)
* 2025-03-13 | Biel, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #2 @ BFH Biel**](https://www.meetup.com/rust-bern/events/306169573) 
* 2025-03-14 | Paris, FR | [Rust in Paris](https://www.rustinparis.com/)
    * [**Rust in Paris 2025**](https://www.rustinparis.com/schedule)
* 2025-03-18 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #10 @ MDPI Basel**](https://www.meetup.com/rust-basel/events/306121044)
* 2025-03-18 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729673)
* 2025-03-27 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #12: Testing in Rust**](https://rust-augsburg.github.io/meetup/Meetup_12.html)

### North America
* 2025-02-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcdbjc)
* 2025-02-27 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Starting the meetup again**](https://www.meetup.com/rust-atl/events/305776081)
* 2025-02-27 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Spidering Wikipedia Politely In Async Rust**](https://www.meetup.com/pdxrust/events/306270454)
* 2025-03-02 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Beacon Hill Rust Lunch, Mar 2**](https://www.meetup.com/bostonrust/events/305805164)
* 2025-03-06 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**CRDTs in Rust**](https://www.meetup.com/stl-rust/events/305187783)
* 2025-03-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Davis Square Rust Lunch, Mar 10**](https://www.meetup.com/bostonrust/events/305805192)
* 2025-03-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638264)
* 2025-03-18 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/events/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/306368210)
* 2025-03-20 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**March, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658448)
* 2025-03-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcfbjc)

### Oceania
* 2025-02-27 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/events/)
    * [**Rust:7 Years Maintaining a Commercial Unicode Tool + Peace: Automation Framework**](https://www.meetup.com/rust-akl/events/306198434)
* 2025-03-04 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/events/)
    * [**How Orica is using Rust in their workplace**](https://www.meetup.com/perth-rust-meetup-group/events/306131753)
* 2025-03-11 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/306262384)

### South America:
* 2025-03-15 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na CloudWalk**](https://www.meetup.com/rust-sao-paulo-meetup/events/306034427)

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

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
