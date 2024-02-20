Title: This Week in Rust 535
Number: 535
Date: 2024-02-21
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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

#### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [RFC: Checking conditional compilation at compile time](https://github.com/rust-lang/rfcs/pull/3013)
    * [Testing steps](https://github.com/rust-lang/rfcs/pull/3013#issuecomment-1936648479)

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

* [eRFC: Iterate on and stabilize libtest's programmatic output](https://github.com/rust-lang/rfcs/pull/3558)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [change equate for binders to not rely on subtyping](https://github.com/rust-lang/rust/pull/118247)
* [disposition: merge] [Implement RFC 3373: Avoid non-local definitions in functions](https://github.com/rust-lang/rust/pull/120393)
* [disposition: merge] [Tracking Issue for `waker_getters`](https://github.com/rust-lang/rust/issues/96992)
* [disposition: merge] [Stabilize the `#[diagnostic]` namespace and `#[diagnostic::on_unimplemented]` attribute](https://github.com/rust-lang/rust/pull/119888)
* [disposition: merge] [Tracking Issue for cfg-target-abi](https://github.com/rust-lang/rust/issues/80970)
* [disposition: merge] [make non-PartialEq-typed consts as patterns a hard error](https://github.com/rust-lang/rust/pull/120805)
* [disposition: close] [Allow ?-converting from `Result<T, E>` in functions returning `Option<Result<T, E>>`](https://github.com/rust-lang/rust/pull/99333)
* [disposition: merge] [Add Read Impl for &Stdin](https://github.com/rust-lang/rust/pull/99153)
* [disposition: merge] [Make `Barrier::new()` const ](https://github.com/rust-lang/rust/pull/119536)
* [disposition: close] [Implement `Future` for `Option<F>`](https://github.com/rust-lang/rust/pull/109691)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [`is` operator for pattern-matching and binding](https://github.com/rust-lang/rfcs/pull/3573)

## Upcoming Events

Rusty Events between 2024-02-21 - 2024-03-20 🦀

### Virtual

* 2024-02-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn**](https://www.meetup.com/rust-berlin/events/298457899/)
* 2024-02-15 | Virtual + In person (Praha, CZ) | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-19 | Virtual (Melbourne, VIC, AU)| [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 1**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 2024-02-20 | Virtual (Melbourne, VIC, AU) | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 2**](https://www.meetup.com/rust-melbourne/events/298877469/)
* 2024-02-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128189/)
* 2024-02-20 | Virtual | [Rust for Lunch](https://lunch.rs/about/)
    * [**Lunch**](https://lunch.rs/meetups/2024-02-20/)
* 2024-02-21 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 2 - Types**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/298991687/)
* 2024-02-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763497/)
* 2024-02-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298251150/)
* 2024-02-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/299068302/)
* 2024-02-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457901/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/02/29/rust-hack-and-learn.html)
* 2024-02-29 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Surfing the Rusty Wireless Waves with the ESP32-C3 Board**](https://www.meetup.com/charlottesville-rust-meetup/events/298372724/)
* 2024-03-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047891/)
* 2024-03-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)
* 2024-03-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341582/)
* 2024-03-12 | Hybrid (Virtual + In-person) Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)

### Asia

* 2024-02-17 | New Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #5**](https://www.meetup.com/rustdelhi/events/298864671/)

### Europe

* 2024-02-15 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust Hacknight #2: Compilers**](https://www.meetup.com/copenhagen-rust-community/events/298999792/)
* 2024-02-15 | Praha, CZ - Virtual + In-person | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/)
    * [**Introduction and Rust in production**](https://www.meetup.com/rust-czech-republic/events/298605120/)
* 2024-02-21 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #8**](https://www.meetup.com/fr-FR/rust-lyon/events/298775631/)
* 2024-02-22 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Partisia**](https://www.meetup.com/rust-aarhus/events/298689622/)
* 2024-02-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Season start 2024**](https://www.meetup.com/rust-berlin/events/299190389/)
* 2024-03-12 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)

### North America

* 2024-02-15 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, Feb 15**](https://www.meetup.com/bostonrust/events/297635043/)
* 2024-02-15 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631774/)
* 2024-02-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Mixer (Moved to Feb 20th)**](https://www.meetup.com/rust-nyc/events/298593474/)
* 2024-02-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/298603354/)
* 2024-02-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Evening Boston Rust Meetup at Microsoft, February 21**](https://www.meetup.com/bostonrust/events/299054786/)
* 2024-02-22 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043763/)
* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)
* 2024-03-07 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)

### Oceania

* 2024-02-19 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 1**](https://www.meetup.com/rust-melbourne/events/298877455/)
* 2024-02-20 | Melbourne, VIC, AU + Virtual | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**(Hybrid - in person & online) February 2024 Rust Melbourne Meetup - Day 2**](https://www.meetup.com/rust-melbourne/events/298877469/)
* 2024-02-27 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/297650401/)
* 2024-02-27 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 spire ⚡ & Quick**](https://www.meetup.com/rust-sydney/events/298892952/)
* 2024-03-05 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Introduction to Embedded Rust + The State of Rust UI**](https://www.meetup.com/rust-akl/events/299158887/)

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
