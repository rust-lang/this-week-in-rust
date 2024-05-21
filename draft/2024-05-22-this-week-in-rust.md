Title: This Week in Rust 548
Number: 548
Date: 2024-05-22
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

## Call for Testing
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

* [MaybeDangling](https://github.com/rust-lang/rfcs/pull/3336)
* [RFC: New range types for Edition 2024](https://github.com/rust-lang/rfcs/pull/3550)
* [Merge RFC 3484: Unsafe extern blocks](https://github.com/rust-lang/rfcs/pull/3484)
* [RFC: cargo-script](https://github.com/rust-lang/rfcs/pull/3502)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [change method resolution to constrain hidden types instead of rejecting method candidates](https://github.com/rust-lang/rust/pull/123962)
* [disposition: merge] [Tracking Issue for asm_const](https://github.com/rust-lang/rust/issues/93332)
* [disposition: merge] [rustdoc: Add support for --remap-path-prefix](https://github.com/rust-lang/rust/pull/107099)
* [disposition: merge] [Make `std::env::{set_var, remove_var}` unsafe in edition 2024](https://github.com/rust-lang/rust/pull/124636)
* [disposition: merge] [Tracking Issue for `slice_flatten`](https://github.com/rust-lang/rust/issues/95629)
* [disposition: merge] [Edition 2024: Make `!` fall back to `!`](https://github.com/rust-lang/rust/pull/123508)
* [disposition: merge] [Stabilize div_duration](https://github.com/rust-lang/rust/pull/124667)
* [disposition: merge] [Panic if `PathBuf::set`_extension would add a path separator](https://github.com/rust-lang/rust/pull/125070)
* [disposition: merge] [Support C23's Variadics Without a Named Parameter](https://github.com/rust-lang/rust/pull/124048)
* [disposition: merge] [Stabilize `LazyCell` and `LazyLock`](https://github.com/rust-lang/rust/pull/121377)
* [disposition: merge] [Turn remaining non-structural-const-in-pattern lints into hard errors](https://github.com/rust-lang/rust/pull/124661)
* [disposition: merge] [Edition 2024: don't special-case diverging blocks](https://github.com/rust-lang/rust/pull/123590)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team RFCs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [document guarantee about evaluation of associated consts and const blocks](https://github.com/rust-lang/reference/pull/1497)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [[RFC] Add `#[export_ordinal(n)]` attribute](https://github.com/rust-lang/rfcs/pull/3641)
* [new] [[RFC] Add `#[diagnostic::blocking]` attribute](https://github.com/rust-lang/rfcs/pull/3639)
* [new] [Guard Patterns](https://github.com/rust-lang/rfcs/pull/3637)

## Upcoming Events

Rusty Events between 2024-05-22 - 2024-06-19 🦀

### Virtual

* 2024-05-15 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events/)
    * [**#RustSemineri - 7**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-7-0a97e784)
* 2024-05-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 6 - Testing**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300819214/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**NativeLink**](https://www.meetup.com/vancouver-rust/events/298542331/)
* 2024-05-16 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events)
    * [**#RustSemineri - 8**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-8-ddfe6b15)
* 2024-05-16 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-17 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Rust at Full Speed: Harnessing Concurrency with Confidence**](https://www.eventbrite.com/e/rust-at-full-speed-harnessing-concurrency-with-confidence-tickets-884842296127)
* 2024-05-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346490/)
* 2024-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477699/)
* 2024-05-23 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Web development in Rust using Rocket (Hebrew)**](https://www.meetup.com/code-mavens/events/300974367/)
* 2024-05-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/300533392/)
* 2024-05-28 & 2024-05-28 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [**Remote Workshop: Telemetry for Rust APIs – you can't fix what you can't see (fee)**](https://ti.to/mainmatter/rust-telemetry-may-2024)
* 2024-05-30 | Virtual + In Person (Barcelona, ES) | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 2024-06-04 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)
* 2024-06-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047896/)
* 2024-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477702/)
* 2024-06-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341709/)

### Africa

* 2024-06-01 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-05-22 | Singapore, SG | [SG Rust Meetup](https://www.meetup.com/rust-singapore/)
    * [**SG Rustaceans! Updated - SG Rust Meetup at CraftsforGreen Whole Studio**](https://www.meetup.com/rust-singapore/events/300988123/)

### Europe

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
* 2024-05-23 | Łodz, PL | [Mobica](https://www.linkedin.com/posts/mobica_rust-programming-embeddedsoftware-activity-7193232853717946369-CK68/)
    * [**Zapisz się na warsztat Rust / Embedded w Łodzi! / What's all the fuss about Rust?**](https://www.interankiety.pl/f/b4D7G7xO)
* 2024-05-23 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/300923207/)
* 2024-05-24 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discussions**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-25 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #3 [Embedded lab edition]**](https://www.meetup.com/stockholm-rust/events/301014982/)
* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)
* 2024-05-30 | Barcelona, ES | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288963/)
* 2024-05-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 sponsored by Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 2024-05-30 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)
* 2024-06-05 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn June 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 2025-06-06 | Vilnius, LT | [Rust Vilnius](https://www.meetup.com/rust-in-vilnius/)
    * [**Enjoy our second Rust and ZIG event**](https://www.meetup.com/rust-in-vilnius/events/301012097/)

### North America

* 2024-05-16 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775539/)
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
* 2024-05-30 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775547/)
* 2024-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/300116786/)
* 2024-06-08 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Porter Square Rust Lunch, Jun 8**](https://www.meetup.com/bostonrust/events/300116799/)

### Oceania

* 2024-05-28 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**a demo 🤯 & a lightning ⚡show ✨**](https://www.meetup.com/rust-sydney/events/300854266/)

### South America

* 2024-06-06 | Buenos Aires, AR | [Rust en Español | Rust Argentina](https://www.meetup.com/rust-argentina/)
    * [**Juntada de Junio**](https://www.meetup.com/rust-argentina/events/299740249)

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
