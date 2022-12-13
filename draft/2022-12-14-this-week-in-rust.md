Title: This Week in Rust 473
Number: 473
Date: 2022-12-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

Fairly quiet week with the only excitement coming from a nice improvement implemented by @nnethercote which fixed a pesky performance regressions in the linting system. This produced a 0.6% performance improvement across a large amount of the real world crates we test against. 

Triage done by **@rylev**.
Revision range: [9db224fc..109cccbe](https://perf.rust-lang.org/?start=9db224fc908059986c179fc6ec433944e9cfce50&end=109cccbe4f345c0f0785ce860788580c3e2a29f5&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.2%  | [0.2%, 0.2%]   | 3     |
| Regressions ❌ <br /> (secondary)  | 1.7%  | [0.3%, 3.3%]   | 11    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-2.2%, -0.2%] | 129   |
| Improvements ✅ <br /> (secondary) | -1.2% | [-4.4%, -0.1%] | 97    |
| All ❌✅ (primary)                 | -0.7% | [-2.2%, 0.2%]  | 132   |


2 Regressions, 2 Improvements, 4 Mixed; 3 of them in rollups
41 artifact comparisons made in total

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-12-13.md) for details.

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-12-14 - 2023-01-11 🦀

### Virtual

* 2022-12-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/287027660/)
* 2022-12-07 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcqbkb/)
* 2022-12-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #20**](https://www.meetup.com/rust-noris/events/hlvbvsydcqblb/)
* 2022-12-08 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: The Genesis of Delta Rust with QP Hou**](https://www.meetup.com/data-ai-online/events/289672886/)
* 2022-12-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-12-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcqbrb/)
* 2022-12-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/289352426/)
* 2022-12-13 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 25u16**](https://www.meetup.com/rust-saar/events/289663288/)
* 2022-12-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcqbsb/)
* 2022-12-24 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 28th Edition**](https://www.meetup.com/rust-linz/events/290196122/)
* 2022-12-14 | Virtual (México City, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust y Arduino**](https://www.meetup.com/rust-mx/events/289973784/)
* 2022-12-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcqbtb/)
* 2022-12-20 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/12/20/rust-hack-and-learn.html)
* 2022-12-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsydcqbbc/)
* 2022-12-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Show & Tell: Tableturf**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcqbcc/)
* 2022-12-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcqbkc/)
* 2023-01-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfccbfb/)
* 2023-01-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfccbgb/)
* 2023-01-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfccbgb/)

### Asia

* 2022-12-29 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**December Edition - xtask, macros and low level features**](https://www.meetup.com/rust-tlv/events/290156141/)

### Europe

* 2022-12-07 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Advent of Rust 1.65**](https://www.meetup.com/rustcologne/events/290084307/)
* 2022-12-07 | Zurich, CH | [Rust Zurich](https://www.meetup.com/Rust-Zurich/)
    * [**Next generation i18n with rust (icu4x) and zero-copy deserialization**](https://www.meetup.com/rust-zurich/events/289518586/)
* 2022-12-14 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust Advent of Code hackathon**](https://www.meetup.com/rust-trondheim/events/290100114/)
* 2022-12-15 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsydcqbvb/)

### North America

* 2022-12-08 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcqblb/)
* 2022-12-14 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/290161310/)
* 2022-12-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcqbbc/)
* 2022-12-27 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**Atx Rustaceans Meetup**](https://www.meetup.com/atx-rustaceans/events/290064553/)

### Oceania

* 2022-12-09 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**December 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/290037796/)

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

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
