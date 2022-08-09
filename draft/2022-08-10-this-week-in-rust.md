Title: This Week in Rust 455
Number: 455
Date: 2022-08-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
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

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

A pretty quiet week for performance. Unfortunately, by far the biggest change was a regression introduced by increasing the minimum libc version for linux-gnu targets. The exact reason for why this happened in this case is unclear, and it's not easy to investigate. Luckily, the average regression introduced by this change was 0.4% which is fairly small, and many of the larger regressions were limited to doc builds.

Triage done by **@rylev**.
Revision range: [792bc5a0..cc4dd6fc](https://perf.rust-lang.org/?start=792bc5a0102d0973d42183a2b267850bb905236f&end=cc4dd6fc9f1a5c798df269933c7e442b79661a86&absolute=false&stat=instructions%3Au)

**Summary**:

|            | mean | max | count |
|:----------:|:----:|:---:|:-----:|
| Regressions ❌ <br /> (primary) | 0.5% | 1.4% | 146   |
| Regressions ❌ <br /> (secondary) | 0.8% | 1.6% | 78    |
| Improvements ✅ <br /> (primary) | N/A  | N/A | 0     |
| Improvements ✅ <br /> (secondary) | -2.0% | -4.0% | 9     |
| All ❌✅ (primary) | 0.5% | 1.4% | 146   |

1 Regressions, 2 Improvements, 2 Mixed; 1 of them in rollups
42 artifact comparisons made in total

Full report [here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-08-09.md)

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-08-10 - 2022-09-07 🦀

### Virtual

* 2022-08-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)
* 2022-08-05 | Virtual + Portland, OR, US | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydclbmb/)
* 2022-08-09 | Virtual (Myrtle Point, OR, US) | [#EveryoneCanContribute Cafe](https://www.meetup.com/everyonecancontribute-cafe/)
    * [**Summer Chill & Learn: including OpenTelemetry getting started with Rust**](https://www.meetup.com/everyonecancontribute-cafe/events/286609523/)
* 2022-08-10 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydclbnb/)
* 2022-08-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydclbpb/)
* 2022-08-12 | Virtual + Tokyo, JP | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://www.reddit.com/r/rust/comments/w7bktx/2022_tokyo_and_elsewhere_rust_game_hack_event_aug/)
* 2022-08-13 | Virtual | [Rust Gamedev](https://gamedev.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://www.google.com/url?q=https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2Eop9Blil9YUWeTq472NIi)
* 2022-08-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydclbvb/)
* 2022-08-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)
* 2022-08-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydclbxb/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-30 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydclbnc/)

### Asia

* 2022-08-12 | Tokyo, JP + Virtual | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://bombercrab-rust-game-hack.peatix.com/view)

### Europe

* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)

### North America

* 2022-08-05 | Portland, OR, US + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-06 | Portland, OR, US | [Rust Project Teams](https://www.rust-lang.org/governance)
    * [**RustConf 2022 PostConf Unconf**](https://www.eventbrite.com/e/rustconf-postconf-unconf-registration-373057423797) | [**Blog post**](https://blog.rust-lang.org/2022/06/28/rust-unconference.html)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)
* 2022-08-11 | Columbus, OH, US| [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydclbpb/)
* 2022-08-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydclbvb/)
* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)
* 2022-08-25 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Concurrencia & paralelismo con Rust**](https://www.meetup.com/rust-mx/events/287561814/)
* 2022-08-25 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Hello World Cargo Crates Using Github Actions with jojobyte and Food!**](https://www.meetup.com/utah-rust/events/kvrxqsydclbpb/)

### Oceania

* 2022-08-24 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-26 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**August 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/287468753/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
