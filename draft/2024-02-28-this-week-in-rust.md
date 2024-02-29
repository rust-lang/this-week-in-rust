Title: This Week in Rust 536
Number: 536
Date: 2024-02-28
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
* [Mastering `ManuallyDrop<T>` - A Guide to Explicit Resource Management in Rust](https://asyncmove.com/blog/2024/02/mastering-manuallydropt-a-guide-to-explicit-resource-management-in-rust/)

### Rust Walkthroughs

* [Practical guide to Error Handling in Rust](https://dev-state.com/posts/error_handling/)
* [Building an Async Runtime with mio](https://tweedegolf.nl/en/blog/114/building-an-async-runtime-with-mio)

### Research

### Miscellaneous

* [Sequential-storage: efficiently store data in flash](https://tweedegolf.nl/en/blog/115/sequential-storage-efficiently-store-data-in-flash)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

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

A rare week where regressions out powered improvements to make the compiler roughly half a percent slower on average on nearly 100 benchmarks. Some regressions have fixes in the pipeline, but some remain elusive or were introduced to address correctness issues.

Triage done by **@rylev**.
Revision range: [5af21304..71ffdf7f](https://perf.rust-lang.org/?start=5af2130440c198afefbe5b8099342057cf272ef4&end=71ffdf7ff7ac6df5f9f64de7e780b8345797e8a0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.0%  | [0.2%, 4.4%]   | 69    |
| Regressions ❌ <br /> (secondary)  | 1.4%  | [0.2%, 4.9%]   | 66    |
| Improvements ✅ <br /> (primary)   | -1.1% | [-3.3%, -0.2%] | 28    |
| Improvements ✅ <br /> (secondary) | -0.6% | [-1.5%, -0.2%] | 33    |
| All ❌✅ (primary)                 | 0.4%  | [-3.3%, 4.4%]  | 97    |


4 Regressions, 6 Improvements, 5 Mixed; 2 of them in rollups
58 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/0528b31d7dad7c98af395e29271591740e984e16/triage/2024-02-27.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Lint singleton gaps after exclusive ranges](https://github.com/rust-lang/rust/pull/118879)
* [disposition: merge] [Tracking Issue for slice::split_at_unchecked() and split_at_mut_unchecked()](https://github.com/rust-lang/rust/issues/76014)
* [disposition: merge] [Tracking Issue for generic `NonZero`](https://github.com/rust-lang/rust/issues/120257)
* [disposition: merge] [Made `INVALID_DOC_ATTRIBUTES` lint deny by default](https://github.com/rust-lang/rust/pull/111505)
* [disposition: merge] [Tracking Issue for ARM CRC32 intrinsics](https://github.com/rust-lang/rust/issues/117215)
* [disposition: merge] [use `confstr(_CS_DARWIN_USER_TEMP_DIR, ...)` as a `TMPDIR` fallback on Darwin](https://github.com/rust-lang/rust/pull/100824)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: RArrow Dereference for Pointer Ergonomics](https://github.com/rust-lang/rfcs/pull/3577)

## Upcoming Events

Rusty Events between 2024-02-28 - 2024-03-27 🦀

### Virtual

* 2024-02-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457901/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/02/29/rust-hack-and-learn.html)
* 2024-02-29 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Surfing the Rusty Wireless Waves with the ESP32-C3 Board**](https://www.meetup.com/charlottesville-rust-meetup/events/298372724/)
* 2024-03-06 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**An intro to `nom`, parsing made easy for Rustaceans**](https://www.meetup.com/rust-dublin/events/299358988/)
* 2024-03-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047891/)
* 2024-03-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)
* 2024-03-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341582/)
* 2024-03-12 | Hybrid (Virtual + In-person) Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Web Frontend Co-Learning (online)**](https://www.meetup.com/opentechschool-berlin/events/298406445/)
* 2024-03-14 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298457903/) | [**Mirror: Berline.rs page**](https://berline.rs/2024/03/14/rust-hack-and-learn.html)
* 2024-03-14 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945252/)
* 2024-03-21 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298631832/)
* 2024-03-26 | Virtual + In Person (Barcelona, ES) | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/) - [Stream](https://www.youtube.com/@bcnrust)

### Europe

* 2024-02-29 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Season start 2024**](https://www.meetup.com/rust-berlin/events/299190389/)
* 2024-02-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust Meetup #44 sponsored by Bang & Olufsen**](https://www.meetup.com/copenhagen-rust-community/events/299353844/)
* 2024-03-06 | Zürich, CH | [Rust Zürisee](https://www.meetup.com/rust-zurich/)
    * [**How to (partial) Migration - March Meetup**](https://www.meetup.com/rust-zurich/events/299380190/)
* 2024-03-12 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-03-13 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.com/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-830340830777)
* 2024-03-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/298533419/)
* 2024-03-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/299028814/)
* 2024-03-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Rust Interactive Session**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/299309224/)
* 2024-03-20 | Girona, ES | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Introduction to programming Microcontrollers with Rust**](https://www.meetup.com/rust-girona/events/299172343/)
* 2024-03-21 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #6**](https://www.meetup.com/de-DE/rust-meetup-augsburg/events/299354449/)
* 2024-03-21 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #6: Du RSS et de L'ECS !**](https://www.meetup.com/meetup-group-zgphbyet/events/299295547/)
* 2024-03-26 | Barcelona, ES + Virtual | [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**13th BcnRust Meetup**](https://www.meetup.com/es-ES/bcnrust/events/299223178/)
* 2024-03-26, 2024-03-28 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation 2024**](https://www.rustnationuk.com/)

### North America

* 2024-02-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/297380841/)
* 2024-02-28 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/299284926/)
* 2024-03-04 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Central Cambridge Rust Lunch**](https://www.meetup.com/bostonrust/events/299261953/)
* 2024-03-07 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299043793/)
* 2024-03-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Northeastern Rust Lunch**](https://www.meetup.com/bostonrust/events/299262009/)
* 2024-03-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186823/)
* 2024-03-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299220136/)
* 2024-03-27 | Hawthorne, CA, US | [Freeform](https://freeform.co/)
    * [**Rust in the Physical World 🦀 Tech Talk Event at Freeform**](https://freeformxrust.rsvpify.com/)

### Oceania

* 2024-02-29 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**February Meetup**](https://www.meetup.com/rust-brisbane/events/299304438/)
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
