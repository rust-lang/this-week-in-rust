Title: This Week in Rust 562
Number: 562
Date: 2024-08-28
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

### Project/Tooling Updates
* [Standards for use of unsafe Rust in the kernel](https://lwn.net/Articles/982868/)

- [git-cliff 2.5.0 is released!](https://git-cliff.org/blog/2.5.0)

* [Bon builder generator 2.0 release 🎉](https://elastio.github.io/bon/blog/bon-builder-generator-v2-release)

* [Meilisearch 1.10 - Federated search, language settings, and improved AI search](https://blog.meilisearch.com/meilisearch-1-10/)

### Observations/Thoughts
* [Rust vs C++: A Real-World Perspective (interview with Tyler Weaver)](https://corrode.dev/blog/cpp-rust-interop/)
* [A Piece of UNIX History in Rust](https://howdytx.technology/unix-history-the-dc-calculator/)

### Rust Walkthroughs
* [video] [Command line tools: Implementing wc in Rust](https://www.youtube.com/watch?v=eQuSA9kSLAY)

- [video] [Explore Linux TTY, process, signals w/ Rust - Part 1/3 (background info)](https://www.youtube.com/watch?v=bolScvh4x7I)

### Research
* [On the Impact of Memory Safety on Fast Network I/O](https://ieeexplore.ieee.org/abstract/document/10635971)

### Miscellaneous

* [Crafting a Blockchain in Go and Rust: A Comparative Journey — Private keys, Public Keys and Signatures [Part 1]](https://hashblog.thepolyglotprogrammer.com/crafting-a-blockchain-in-go-and-rust-a-comparative-journey-private-keys-public-keys-and-signatures-part-1)
* [video] [A Multiplatform Mobile Navigation SDK with Rust at the Core](https://www.youtube.com/watch?v=WL0jY51PQR8)


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

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [crates.io: Remove dev-dependencies from the index](https://github.com/rust-lang/rfcs/pull/3674)
* [disposition: merge] [Mergeable rustdoc cross-crate info](https://github.com/rust-lang/rfcs/pull/3662)
* [disposition: merge] [Guard Patterns](https://github.com/rust-lang/rfcs/pull/3637)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Proposal: stabilize `const_refs_to_static`](https://github.com/rust-lang/rust/issues/128183)
* [disposition: merge] [Check WF of source type's signature on fn pointer cast](https://github.com/rust-lang/rust/pull/129021)
* [disposition: merge] [rustdoc: add header map to the table of contents](https://github.com/rust-lang/rust/pull/120736)
* [disposition: merge] [doc: Make block of inline Deref methods foldable](https://github.com/rust-lang/rust/pull/127474)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [`#[derive(Default)]` on enum variants with fields](https://github.com/rust-lang/rfcs/pull/3683)
* [new] [[RFC] Default field values](https://github.com/rust-lang/rfcs/pull/3681)
* [new] [Simplify lightweight clones, including into closures and async blocks](https://github.com/rust-lang/rfcs/pull/3680)

## Upcoming Events

Rusty Events between 2024-08-28 - 2024-09-25 🦀

### Virtual
* 2024-08-28 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633267/)
* 2024-08-29 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
* 2024-09-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007365/)
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - Typestate Pattern in Rust: With a Strict Builder Example**](https://www.meetup.com/indyrs/events/300328029/)
* 2024-09-05 | Virtual | [LambdaClass](https://lu.ma/user/usr-dkk9KnFvsvZEb7k)
    * [**Meetup Rust Septiembre [Spanish]**](https://lu.ma/uh1qpox0)
* 2024-09-05 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897957/)
* 2024-09-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820268/)
* 2024-09-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346981/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-12 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633268/)
* 2024-09-12 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #6**](https://www.meetup.com/bevy-game-development/events/302841892/)
* 2024-09-16 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302827971/)
* 2024-09-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346969/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-18 - 2024-09-20 | Hybrid - Virtual and In-Person (Vienna, AT) | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024**](https://lpc.events/event/18/sessions/186/)
* 2024-09-19 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897973/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585670/)

### Africa
* 2024-09-06 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587/)

### Asia
* 2024-09-09 | Ramat Gan, IL | [Coralogix](https://coralogix.com/)
    * [**Rust as Scale**](https://coralogix.com/rust-coralogix-meetup/)
* 2024-09-14 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**September 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/september-2024-rustacean-meetup/)

### Europe
* 2024-08-28 | Frankfurt (Main), DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Frankfurt WebAssembly**](https://www.meetup.com/rust-rhein-main/events/302738445/)
* 2024-08-29 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - The social Beergarden.**](https://www.meetup.com/rust-berlin/events/299421378/)
* 2024-08-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #50 sponsored by Adapt Agency**](https://www.meetup.com/copenhagen-rust-community/events/303040544/)
* 2024-09-04 | Oxford, UK | [Oxfrod Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**More Rust - Generics, constraints, safety.**](https://www.meetup.com/oxford-rust-meetup-group/events/302848276/)
* 2024-09-11 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302833564/)
* 2024-09-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425049/)
* 2024-09-17 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Making AI-models perform tasks, in Rust!**](https://www.meetup.com/rust-trondheim/events/302957040/)
* 2024-09-18 | Moravia, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/)
    * [**Rust Moravia Meetup (September 2024)**](https://www.meetup.com/rust-moravia/events/301360936)
* 2024-09-18 | Vienna, AT + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2024 (Sep 18-20)**](https://lpc.events/event/18/sessions/186/)
* 2024-09-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #6**](https://www.meetup.com/bratislava-rust-meetup-group/events/302916594/)

### North America
* 2024-08-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygclblc/)
* 2024-08-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : placeholder**](https://www.meetup.com/music-city-rust-developers/events/301420110/)
* 2024-08-29 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night/Happy Hour**](https://www.meetup.com/deep-dish-rust/events/302940777/)
* 2024-08-31 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Central Cambridge Rust Lunch, Aug 31**](https://www.meetup.com/bostonrust/events/302498706/)
* 2024-09-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302723843/)
* 2024-09-05 | Portland, OR, US | [PDX Rust](https://www.meetup.com/pdxrust/)
    * [**PDX Rust September!**](https://www.meetup.com/pdxrust/events/302588479/)
* 2024-09-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/hdzdmtygcmbhb/)
* 2024-09-07 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976293/)
* 2024-09-08 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/302498734/)
    * [**Northeastern Rust Lunch, Sep 8**](https://www.meetup.com/bostonrust/events/302498706/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA) | [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)
* 2024-09-11 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Boulder Elixir Meetup**](hhttps://www.meetup.com/boulder-elixir/events/302991078/)
* 2024-09-16 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Somerville Union Square Rust Lunch, Sep 16**](https://www.meetup.com/bostonrust/events/302498750/)
* 2024-09-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638248/)
* 2024-09-18 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Cells**](https://www.meetup.com/vancouver-rust/events/298631736/)
* 2024-09-19 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**September Meetup**](https://www.meetup.com/join-srug/events/303067835/)
* 2024-09-21 | Longview, TX, US | [Longview Code and Coffee](https://www.meetup.com/longview-code-and-coffee/)
    * [**Longview Code and Coffee**](https://www.meetup.com/longview-code-and-coffee/events/301976355/)
* 2024-09-24 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/zfcbntygcmbgc/)
* 2024-09-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/302274449/)

# Oceania
* 2024-08-28 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Stuff ⚡ And Crabs 🦀**](https://www.meetup.com/rust-sydney/events/302951173/)


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
