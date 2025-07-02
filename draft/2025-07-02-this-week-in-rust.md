Title: This Week in Rust 606
Number: 606
Date: 2025-07-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Project/Tooling Updates
* [Asterinas: a new Linux-compatible kernel project](https://lwn.net/SubscriberLink/1022920/14dfdc76df0f1b96/)

### Observations/Thoughts

* [Leaktracer: A Rust allocator to trace memory allocations](https://blog.veeso.dev/blog/en/leaktracer-a-rust-allocator-to-trace-memory-allocations/)

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

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
Lots of changes this week with results dominated by the 1-5% improvements from
[#142941](https://github.com/rust-lang/rust/pull/142941) across lots of primary
benchmarks in the suite.

Triage done by **@simulacrum**.
Revision range: [42245d34..ad3b7257](https://perf.rust-lang.org/?start=42245d34d22ade32b3f276dcf74deb826841594c&end=ad3b7257615c28aaf8212a189ec032b8af75de51&absolute=false&stat=instructions%3Au)

3 Regressions, 6 Improvements, 5 Mixed; 4 of them in rollups
39 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025/2025-06-30.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### Tracking Issues & PRs
<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: -->
<!-- * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: -->
<!-- * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

## Upcoming Events

Rusty Events between 2025-07-02 - 2025-07-30 🦀

### Virtual
* 2025-06-25 | Virtual (Lima, PE)| [Perú Rust User Group](https://www.meetup.com/peru-rust-user-group/)
    * [**Interfaces y Costos en la nube con Rust**](https://www.meetup.com/peru-rust-user-group/events/308543965/)
* 2025-06-26 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/cgamfls6)
* 2025-06-26 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/304567869)
* 2025-06-29 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbmc)
* 2025-07-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031667)
* 2025-07-03 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820304)
* 2025-07-03 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #11**](https://www.meetup.com/bevy-game-development/events/308463394)
* 2025-07-05 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-07-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308298511)
* 2025-07-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305361452)
* 2025-07-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308298512)
* 2025-07-15 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/307560349)
* 2025-07-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/306757755)
* 2025-07-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307731031)
* 2025-07-17 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820305)
* 2025-07-20 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Readers Discord Discussion: Rust Atomics and Locks**](https://www.meetup.com/dallasrust/events/308383001)
* 2025-07-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/tgctrtyhckbdc)
* 2025-07-22 | Virtual (London, GB) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: Crates, Tips & Tricks Lightning Talks - Bring your ideas!**](https://www.meetup.com/women-in-rust/events/307560304)

### Asia
* 2025-06-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**June 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/june-2025-rustacean-meetup/)
* 2025-07-02 | Seoul, KR | [Seoul Rust (Programming Language) Meetup](https://www.meetup.com/rust-seoul-meetup/events/)
    * [**Seoul Rust Meetup**](https://www.meetup.com/rust-seoul-meetup/events/308408246)

### Europe
* 2025-06-25 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Lessons learnt from making a tiny game in nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)
* 2025-06-25 | Paris, FR | [Systematic Paris Region](https://systematic-paris-region.org/)
    * [**Rust Paris Conference 2025**](https://my.weezevent.com/rust-paris-2025)
* 2025-06-26 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**18th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/308399403)
* 2025-06-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #58**](https://www.meetup.com/copenhagen-rust-community/events/308161212)
* 2025-06-26 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/308416060)
* 2025-06-30 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/events/)
    * [**Meetup 2025/06: Drink-up zatvaranje sezone**](https://www.meetup.com/zagreb-rust-meetup/events/308477879)
* 2025-07-01 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #9**](https://www.meetup.com/rust-gdansk/events/308349712)
* 2025-07-01 | Paris, FR | [Stockly](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-1407389873999)
* 2025-07-02 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #12 @ kHaus**](https://www.meetup.com/rust-basel/events/307567391)
* 2025-07-02 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**The (un)holy Trinity of Flutter, Rust and C**](https://www.meetup.com/rust-rhein-main/events/308609465)
* 2025-07-02 | London, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and ACCU special - Vibe coding workshop**](https://www.meetup.com/oxford-rust-meetup-group/events/308435063/)
* 2025-07-02 | Posnan, PL | [Rust Poland](https://www.meetup.com/rust-poland-meetup/)
    * [**Rust Poland Meetup x Poznan**](https://www.meetup.com/rust-poland-meetup/events/308480357)
* 2025-07-05 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Ferris' Fika Forum #13**](https://www.meetup.com/stockholm-rust/events/308530949)
* 2025-07-08 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Garbage Collection for Rust: the Finalizer Frontier**](https://www.meetup.com/london-rust-project-group/events/308443710)
* 2025-07-09 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 07 2025**](https://lu.ma/hismn492)
* 2025-07-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtyhckbmb)
* 2025-07-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592246)
* 2025-07-15 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**TUI Power: Simulating & Visualising Sensor Data with Rust**](https://www.meetup.com/london-rust-project-group/events/308434768)

### North America
* 2025-06-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)
* 2025-06-26 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/308562608)
* 2025-06-26 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles/events/)
    * [**Rust in Web3 Developer Group**](https://www.meetup.com/rust-los-angeles/events/308401269)
* 2025-06-26 | Los Angeles (Chino Hills), CA, US | [Vara Network](https://lu.ma/events-by-vara-gear)
    * [**Rust in Web3**](https://lu.ma/ek8jx2r3)
* 2025-06-26 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Elixir y Rust**](https://www.meetup.com/rust-mx/events/308579237)
* 2025-06-26 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**Monthly Meetup: Making a CRUD API with Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 2025-06-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, June 28**](https://www.meetup.com/bostonrust/events/307936269)
* 2025-07-03 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**July Monthly Social**](https://www.meetup.com/rust-montreal/events/308532058)
* 2025-07-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Building Resilient and Observable Rust Services with steady_state**](https://www.meetup.com/stl-rust/events/306345853)
* 2025-07-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Alewife Rust Lunch, July 6**](https://www.meetup.com/bostonrust/events/307936287)
* 2025-07-07 | Denver, CO, US | [FOSS Rust Colorado](https://mobilizon.us/@foss_rust_colorado/events)
    * [**Embedded Rust Hack Night**](https://mobilizon.us/events/e9d6fd47-6120-4789-ad04-313d6a04f572)
* 2025-07-09 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans/events/)
    * [**Rust <> AI**](https://www.meetup.com/desert-rustaceans/events/308507249/)
* 2025-07-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307931266)
* 2025-07-17 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust on Bare Metal Series 1 : Introduction to Embedded Development**](https://www.meetup.com/music-city-rust-developers/events/304333113)
* 2025-07-17 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**July, 2025 Computer Programming Language Panel (Special Event)**](https://www.meetup.com/seattle-rust-user-group/events/307698855)
* 2025-07-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhckbfc)

### Oceania
* 2025-06-30 | Collingwood, VI, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/events/)
    * [**June 2025 Mini Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/308546374)
* 2025-07-01 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**July 2025 Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/308605782)

### South America
* 2025-07-12 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)

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
