Title: This Week in Rust 579
Number: 579
Date: 2024-12-25
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
* [The Embedded Rustacean Issue #35](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-35)

### Project/Tooling Updates

* [musi lili retro game engine 0.1 released](https://crates.io/crates/musi_lili)
* [Nutype 0.5.1: better no_std support and bug fixes](https://github.com/greyblake/nutype/releases/tag/v0.5.1)
* [Ibis 0.2.0 - Federated Wiki with Shiny Redesign, based on Diesel, Actix and Leptos](https://ibis.wiki/article/Ibis_release_0.2.0_-_Federated_Wiki_with_Shiny_Redesign@ibis.wiki)

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

* [Rust university course exercises](https://kobzol.github.io/teaching/2024/12/18/rust-exercises.html)

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
* [CfT: Test out Rustup's `reqwest` backend with `rustls`](https://github.com/rust-lang/rustup/issues/3806)
  - [Testing steps](https://github.com/rust-lang/rustup/issues/3806#issue-2278962476)

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

* [Rama — provide constants for common (network) hosts to rama-net's Host](https://github.com/plabayo/rama/issues/363)
* [Rama — support vec/array impl for DnsResolver](https://github.com/plabayo/rama/issues/332)
* [Rama — support HAR exporter (http) layer in rama](https://github.com/plabayo/rama/issues/357)

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

We missed triage last week due to some process issues, so this triage includes two weeks of data. The overall result is positive, due to parser optimizations ([#133793](https://github.com/rust-lang/rust/pull/133793)), trait solving optimizations ([#134501](https://github.com/rust-lang/rust/pull/134501), [#132325](https://github.com/rust-lang/rust/pull/132325)) and bumping the cc crate ([#134505](https://github.com/rust-lang/rust/pull/134505)), which [improved the performance](https://github.com/rust-lang/cc-rs/pull/1279) of C/C++ dependencies of the compiler.

Triage done by **@kobzol**.
Revision range: [1b3fb316..0eca4dd3](https://perf.rust-lang.org/?start=1b3fb316751227d30b1523ed0e3f00d83956d4d0&end=0eca4dd3205a01dba4bd7b7c140ec370aff03440&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.3%, 0.8%]   | 3     |
| Regressions ❌ <br /> (secondary)  | 1.0%  | [1.0%, 1.0%]   | 1     |
| Improvements ✅ <br /> (primary)   | -1.8% | [-7.5%, -0.3%] | 254   |
| Improvements ✅ <br /> (secondary) | -1.3% | [-5.4%, -0.3%] | 224   |
| All ❌✅ (primary)                 | -1.8% | [-7.5%, 0.8%]  | 257   |

4 Regressions, 10 Improvements, 12 Mixed; 9 of them in rollups
90 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/457d83dc231ed684e9f09e96fdf41f45bed0fe67/triage/2024-12-23.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add support for `use Trait::func`](https://github.com/rust-lang/rfcs/pull/3591)
* [crates.io: Trusted Publishing Support](https://github.com/rust-lang/rfcs/pull/3691)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Specify the behavior of `file!`](https://github.com/rust-lang/rust/pull/134442)
* [disposition: merge] [Stabilize `feature(trait_upcasting)`](https://github.com/rust-lang/rust/pull/134367)
* [disposition: merge] [Stabilize `derive(CoercePointee)`](https://github.com/rust-lang/rust/pull/133820)
* [disposition: merge] [Stabilize `asm_goto feature gate`](https://github.com/rust-lang/rust/pull/133870)
* [disposition: merge] [Tracking Issue for get_many_mut](https://github.com/rust-lang/rust/issues/104642)
* [disposition: merge] [`--nocapture` doesn't follow common CLI conventions, making it a stumbling block to people debugging failures](https://github.com/rust-lang/rust/issues/133073)
* [disposition: merge] [Tracking Issue for `sub_ptr` (feature `ptr_sub_ptr`)](https://github.com/rust-lang/rust/issues/95892)
* [disposition: merge] [From iterator for more tuples](https://github.com/rust-lang/rust/pull/132431)
* [disposition: merge] [Tracking Issue for const_swap](https://github.com/rust-lang/rust/issues/83163)
* [disposition: merge] [Tracking issue for const `alloc::Layout`](https://github.com/rust-lang/rust/issues/67521)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Set CARGO_CHECK environment variable when type checking](https://github.com/rust-lang/rfcs/pull/3748)
* [new] [unsized params in traits](https://github.com/rust-lang/rfcs/pull/3745)
* [new] [Convert "reassignment of immutable local" and "mutable borrow of immutable local" from a hard error to a deny-by-default lint](https://github.com/rust-lang/rfcs/pull/3742)

## Upcoming Events

Rusty Events between 2024-12-25 - 2025-01-22 🦀

### Virtual
* 2024-12-19 | Virtual | [Scandio GmBH](https://www.eventbrite.de/o/scandio-gmbh-75623231843)
    * [**Einführung in Rust: Für eine nachhaltige Zukunft / Introduction to Rust: For a Sustainable Future**](https://www.eventbrite.com/e/einfuhrung-in-rust-fur-eine-nachhaltige-zukunft-tickets-1106203667949)
* 2024-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633276/)
* 2024-12-19 | Virtual (Buenos Aires, AR) | [Rust en Español](https://www.meetup.com/rust-argentina/events/)
    * [**Despedida de Año 🎉🎉**](https://www.meetup.com/rust-argentina/events/305095113)
* 2024-12-19 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 2024-12-20 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 2024-12-22 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller**](https://www.meetup.com/rust-tlv/events/304937982)
* 2024-12-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)
* 2024-12-26 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898145)
* 2025-01-02| Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633277/)
* 2025-01-04 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-01-06 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**logger.info(f"Don't Give your {secrets} away") by Tamar Galer (Virtual, English)**](https://www.meetup.com/code-mavens/events/305045436)
* 2025-01-07 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Microdosing Rust to your organization with Aviram Hassan (Virtual, English)**](https://www.meetup.com/code-mavens/events/304883841)
* 2025-01-08 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**BlockMesh Network implemented in Rust with Ohad Dahan (Virtual, English)**](https://www.meetup.com/code-mavens/events/304951805)
* 2025-01-09 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898167)
* 2025-01-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/302815031)
* 2025-01-14 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**An introduction to WASM in Rust with Márk Tolmács (Virtual, English)**](https://www.meetup.com/code-mavens/events/305064546)
* 2025-01-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)

### Asia
* 2025-01-12 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust January 2025 at Abra in Raanana**](https://www.meetup.com/rust-tlv/events/304898730/)

### Europe
* 2024-12-18 | Ghent, BE | [Systems Programming Ghent](https://sysghent.be)
    * [**Launch of new community for Rust and C++ developers**](https://sysghent.be)
* 2025-01-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305038426)
* 2025-01-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820279/)
* 2025-01-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154281)

### North America
* 2024-12-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Harvard Square Rust Lunch, Dec 22**](https://www.meetup.com/bostonrust/events/304951457)
* 2024-12-26 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)
* 2025-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Lechmere Rust Lunch, Jan 10**](https://www.meetup.com/bostonrust/events/304951467)

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
