Title: This Week in Rust 461
Number: 461
Date: 2022-09-21
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

This was a fairly negative week for compiler performance, with regressions
overall up to 14% on some workloads (primarily incr-unchanged scenarios),
largely caused by [#101620](https://github.com/rust-lang/rust/pull/101620). We
are still chasing down either a revert or a fix for that regression, though a
partial mitigation in [#101862](https://github.com/rust-lang/rust/pull/101862)
has been applied. Hopefully the full fix or revert will be part of the next
triage report.

We also saw a number of other regressions land, though most were much smaller in magnitude.

Triage done by **@simulacrum**.
Revision range: [17cbdfd0..8fd6d03](https://perf.rust-lang.org/?start=17cbdfd07178349d0a3c
ecb8e7dde8f915666ced&end=8fd6d03e22fba2930ad377b87299de6a37076074&absolute=false&stat=instructions%3Au)

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-09-20.md) for more details.

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-09-21 - 2022-10-19 🦀

### Virtual

* 2022-09-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcmbsb/)
* 2022-09-14 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Introduction to Async in Rust**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/288154493/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)
* 2022-09-15 | Virtual (Columbus, OH, US) | [GDG Columbus](https://www.meetup.com/gdg-columbus/)
    * [**Past, Present, and Future of Internet Money! (Custom tokenomics, RUST and CosmWASM library...)**](https://www.meetup.com/gdg-columbus/events/287972746/)
* 2022-09-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcmbtb/)
* 2022-09-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful: Bencher—Catch Performance Regressions in CI—Everett Pompeii**](https://www.meetup.com/rustdc/events/287004599/)
* 2022-09-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out (Call for Participation)**](https://www.meetup.com/vancouver-rust/events/285933975/)
* 2022-09-22 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Rust based Bluetooth tools (BlueR) you can use today**](https://www.meetup.com/charlottesville-rust-meetup/events/288123436/)
* 2022-09-22 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Rust Iran Meetup #9 - Let's Write An Async Executor**](https://rust-meetup.ir/2022/09/22/9th-meetup.html)
* 2022-09-23 | Virtual (Tokyo, JP) | [Rust Tokyo](https://rust.tokyo)
    * [**Rust Tokyo 2022**](https://rust.tokyo/2022)
* 2022-09-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcmbkc/)
* 2022-09-28 | Virtual (London, UK) | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)
* 2022-10-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #18**](https://www.meetup.com/rust-noris/events/hlvbvsydcnbrb/)
* 2022-10-08 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-10-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-12 | Virtual (San Francisco, CA, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/)

### Europe

* 2022-09-15 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #52**](https://www.meetup.com/rust-paris/events/288136736/)
* 2022-09-27 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Rust at RU**](https://www.meetup.com/rust-nederland/events/288376119/)
    * [**Student track: Rust at RU**](https://www.meetup.com/rust-nederland/events/288440591/)
* 2022-09-28 | London, UK + Virtual | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)
* 2022-10-03 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Microsoft Reactor**](https://www.meetup.com/Stockholm-Rust/events/288453360/)
* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)

### North America

* 2022-09-14 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288287206/)
* 2022-09-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcmbbc/)
* 2022-09-22 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Game Prototyping with Rusty Engine with Nathan Stocks and Food!**](https://www.meetup.com/utah-rust/events/rvpgxsydcmbmc/)
* 2022-09-29 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Nuestra RustMX comunidad tiene página**](https://www.meetup.com/rust-mx/events/288388973/)

### Oceania

* 2022-09-14 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust-Sydney Lightning Talks**](https://www.meetup.com/rust-sydney/events/287979855/)
* 2022-09-20 | Phillip, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/288450836/)

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
