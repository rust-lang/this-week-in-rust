Title: This Week in Rust 642
Number: 642
Date: 2026-03-11
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
* [mdterm v1.0.0 - A terminal-based Markdown browser](https://github.com/bahdotsh/mdterm/releases/tag/v1.0.0)
* [The Anatomy of a 500ns Parser: Porting libphonenumber to Rust](https://dev.to/vloldik/the-anatomy-of-a-500ns-parser-porting-libphonenumber-to-rust-3daa)
* [mini-agent: A Rust AI Agent Framework](https://dev.to/rajmandaliya/building-a-rust-ai-agent-framework-from-scratch-what-i-learned-3o23)
* [ClickHouse meets SeaORM: Arrow-powered data pipeline](https://www.sea-ql.org/blog/2026-03-08-sea-clickhouse/)
* [Rustaceans.AI](https://rustaceans.ai/)
* [Leptodon 1.0.0: UI toolkit for the Leptos WASM framework](https://www.openanalytics.eu/blog/2026/03/09/leptodon-1.0.0/)

* [Signing Rust Binaries Shouldn't Require Shell Scripts](https://d34dl0ck.me/cargo-codesign/index.html)

### Observations/Thoughts
* [Why SeaORM over JavaScript client database options?](https://opeolluwa.github.io/almonds/blog/why-not-js-database)
* [Rust is slowly but surely eating PostgreSQL: Deep dive into Neon, ParadeDB, PgDog and more](https://kerkour.com/rust-eating-postgres)
* [What Happens When You Constrain an Event-Driven System to Three Primitives](https://www.rodriguez.today/articles/emergent-event-driven-workflows)

* [My Rust dev setup in 2026](https://bitfieldconsulting.com/posts/rust-dev-tools)

* [audio] [Netstack.FM episode 30 — uReq with Martin Algesten](https://netstack.fm/#episode-30)

* [Weighing up Zngur and CXX for Rust/C++ Interop](https://www.kdab.com/weighing-up-zngur-and-cxx-for-rustc-interop/)

### Rust Walkthroughs
* [ZK snarks for rust developer part 1/8](https://rustarians.com/polynomials-in-zk-snarks/)
* [Get in Line (Part 2) - Vyukov's Queue and its specializations](https://abhikja.in/blog/2026-03-10-get-in-line-part-2/)

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
* [diesel-guard - REFRESH MATERIALIZED VIEW without CONCURRENTLY](https://github.com/ayarotsky/diesel-guard/issues/89)
* [diesel-guard - ADD CHECK CONSTRAINT without NOT VALID](https://github.com/ayarotsky/diesel-guard/issues/88)
* [diesel-guard - ADD FOREIGN KEY without NOT VALID](https://github.com/ayarotsky/diesel-guard/issues/87)
* [diesel-guard - no lock_timeout/statement_timeout before DDL](https://github.com/ayarotsky/diesel-guard/issues/97)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**Rust India Conference 2026**](https://hasgeek.com/rustbangalore/cfp-rust-india-conference-2026/) | CFP open until 2026-03-14 | Bangalore, IN | 2026-04-18
* [**Oxidize Conference**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP open until 2026-03-23 | Berlin, Germany | 2026-09-14 - 2026-09-16
* [**EuroRust**](https://sessionize.com/eurorust-2026/) | CFP open until 2026-04-27 | Barcelona, Spain | 2026-10-14 - 2026-10-17

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

* [Remove `ATTRIBUTE_ORDER`](https://github.com/rust-lang/rust/pull/153041)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Custom lint profiles](https://github.com/rust-lang/rfcs/pull/3926)

## Upcoming Events

Rusty Events between 2026-03-11 - 2026-04-08 🦀

### Virtual
* 2026-03-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/cgzfpzcp)
* 2026-03-12 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455924/)
* 2026-03-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/313490537/)
* 2026-03-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Embedded Rust**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-18 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Hybrid event with Rust Dortmund!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/313621933/)
* 2026-03-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/45qqc2eo)
* 2026-03-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**March, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-20 | Virtual | [Packt Publishing Limited](https://www.eventbrite.com/o/70306584013)
    * [**Rust Adoption, Safety, and Cloud with Francesco Ciulla**](https://www.eventbrite.com/e/rust-adoption-safety-and-cloud-with-francesco-ciulla-registration-1981847709850)
* 2026-03-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254785/)
* 2026-03-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/312799496/)
* 2026-03-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 03 2026**](https://luma.com/vq9w8q0w)
* 2026-03-26 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455925/)
* 2026-04-01 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/me4jwgxu)
* 2026-04-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcgbcb/)
* 2026-04-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345237/)
* 2026-04-04 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 2026-03-22 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust March 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/312862609/)

### Europe
* 2026-03-11 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/313426708/)
* 2026-03-11 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a Python compiler in Rust**](https://www.meetup.com/rust-rhein-main/events/313617138/)
* 2026-03-12 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2026 Rust Talks Bern #1 @bespinian**](https://www.meetup.com/rust-bern/events/313443248/)
* 2026-03-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-03-18 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Intro to Embedded Rust - March**](https://www.meetup.com/rust-dortmund/events/313338784/)
* 2026-03-19 - 2026-03-20 | Warsaw, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Rustikon Conference**](https://www.rustikon.dev/)
* 2026-03-23 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #18**: Ludwig Weinzierl - Bevy: Spielentwicklung mit Rust](https://rust-augsburg.github.io/meetup/Meetup_18.html)
* 2026-03-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night - Advent of Code**](https://www.meetup.com/rust-aarhus/events/313284304/)
* 2026-03-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester March Code Night**](https://www.meetup.com/rust-manchester/events/313495449/)
* 2026-03-24 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust projects - show and tell in March**](https://www.meetup.com/rust-trondheim/events/313537618/)
* 2026-03-26 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #84**](https://www.meetup.com/rust-paris/events/313646981/)
* 2026-03-27 | Paris, FR | [Rust in Paris](https://www.rustinparis.com/)
    * [**Rust in Paris**](https://www.rustinparis.com/)
* 2026-04-01 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Rust/ACCU meetup.**](https://www.meetup.com/oxford-rust-meetup-group/events/312664491/)

### North America
* 2026-03-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**An Interpreter for Computability theory, Written the Hard Way**](https://www.meetup.com/utah-rust/events/313506767/)
* 2026-03-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust March Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721867/)
* 2026-03-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, Mar 14**](https://www.meetup.com/bostonrust/events/313208587/)
* 2026-03-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/313404095/)
* 2026-03-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Embedded Rust**](https://www.meetup.com/vancouver-rust/events/313471716/)
* 2026-03-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**March, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274882/)
* 2026-03-19 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313569258/)
* 2026-03-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Applied Rust - Building Rust Applictions**](https://www.meetup.com/music-city-rust-developers/events/313576317/)
* 2026-03-19 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: Social Interoperability - Rust, C++ and The Greater Good**](https://www.meetup.com/rust-nyc/events/313639707/)
* 2026-03-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Porter Square Rust Lunch, Mar 21**](https://www.meetup.com/bostonrust/events/313208597/)
* 2026-03-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/313653030/)
* 2026-03-25 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Digital Asset Adoption Special**](https://www.meetup.com/rust-nyc/events/313713085/)
* 2026-03-26 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228658/)
* 2026-04-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**SIUE Cruft Crawler with LLM**](https://www.meetup.com/stl-rust/events/313482094/)

### Oceania
* 2026-03-12 | Brisbane City, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane Mar 2026**](https://www.meetup.com/rust-brisbane/events/313596218/)
* 2026-03-26 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**TBD March Meetup**](https://www.meetup.com/rust-melbourne/events/313471749/)

### South America
* 2026-03-21 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP (migrado pro Lumma)**](https://www.meetup.com/rust-sao-paulo-meetup/events/313446400/)

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
