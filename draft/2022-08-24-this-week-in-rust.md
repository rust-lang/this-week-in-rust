Title: This Week in Rust 457
Number: 457
Date: 2022-08-24
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
* [rust-analyzer changelog #143](https://rust-analyzer.github.io/thisweek/2022/08/22/changelog-143.html)
* [What's new in axum 0.6.0-rc.1](https://tokio.rs/blog/2022-08-whats-new-in-axum-0-6-0-rc1)

* [This week in Databend #56: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-08-24-databend-weekly/)

### Observations/Thoughts
* [State Machines II](https://blog.yoshuawuyts.com/state-machines-2/)
* [Come contribute to Salsa 2022!](https://smallcultfollowing.com/babysteps/blog/2022/08/18/come-contribute-to-salsa-2022/)

### Rust Walkthroughs
* [Writing a container in Rust](https://litchipi.github.io/series/container_in_rust)
* [Experimentally compiling PHP code to Rust - Ryan Chandler](https://ryangjchandler.co.uk/posts/experimentally-compiling-php-code-to-rust)
* [video] [Get under the hood of Rust Language with Assembly!!](https://www.youtube.com/watch?v=lRV_5IBUTes)
* [video] [Scoped threads in Rust 1.63](https://www.youtube.com/watch?v=VsIicvwf_Yc)
* [video] [1Password Developer Fireside Chat: Demystifying Atomics](https://www.youtube.com/watch?v=qhWbuQ8rv5k)

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

Overall some really impressive wins this week. Note in particular
PR [#100209](https://github.com/rust-lang/rust/pull/100209), "Lazily
decode SourceFile from metadata" (which improved 75 primary benchmark
scenarios and 158 secondary scenarios) and
PR [#98655](https://github.com/rust-lang/rust/pull/98655) "Don't derive
`PartialEq::ne`", which improved 65 primary scenarios and 27 secondary
scenarios). There were a few cases that pnkfelix explicitly decided not
to mark as triaged; see report for more details there.
Also pnkfelix wonders if there is a recent slight-upward trend on max-rss
for the past week, see the [summary graph](https://perf.rust-lang.org/?start=&end=&kind=percentfromfirst&stat=max-rss)

Triage done by **@pnkfelix**.
Revision range: [14a459bf..4a24f08b](https://perf.rust-lang.org/?start=14a459bf37bc19476d43e0045d078121c12d3fef&end=4a24f08ba43166cfee86d868b3fe8612aec6faca&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u) | mean | range | count |
|:----------------:|:----:|:-----:|:-----:|
| Regressions ❌ <br /> (primary) | 0.6% | [0.4%, 0.8%] | 27    |
| Regressions ❌ <br /> (secondary) | 0.4% | [0.2%, 0.6%] | 9     |
| Improvements ✅ <br /> (primary) | -1.7% | [-20.1%, -0.3%] | 91    |
| Improvements ✅ <br /> (secondary) | -3.6% | [-18.7%, -0.3%] | 160   |
| All ❌✅ (primary) | -1.2% | [-20.1%, 0.8%] | 118   |


3 Regressions, 4 Improvements, 4 Mixed; 3 of them in rollups
43 artifact comparisons made in total

[Full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-08-24.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Register wf obligation before normalizing in wfcheck](https://github.com/rust-lang/rust/pull/100046)
* [disposition: merge] [Partially stabilize `bound_as_ref` by stabilizing `Bound::as_ref`](https://github.com/rust-lang/rust/pull/99736)
* [disposition: merge] [Document NonZeroXxx layout guarantees](https://github.com/rust-lang/rust/pull/94786)
* [disposition: merge] [Strengthen invalid_value lint to forbid uninit primitives, adjust docs to say that's UB](https://github.com/rust-lang/rust/pull/98919)
* [disposition: merge] [Make forward compatibility lint deprecated_cfg_attr_crate_type_name deny by default](https://github.com/rust-lang/rust/pull/99784)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Statics in patterns](https://github.com/rust-lang/rfcs/pull/3305)

## Upcoming Events

Rusty Events between 2022-08-24 - 2022-09-21 🦀

### Virtual

* 2022-08-24 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Tech Talk Live Appointment: Customize GitHub Workflow with Serverless Functions - How to use Rust and JavaScript to automate GitHub processes**](https://www.meetup.com/wasm-rust-meetup/events/287876999/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-25 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch: together with the C++ UG KA; various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/287642940/)
* 2022-08-27 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059968/)
* 2022-08-30 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/287286751/)
* 2022-08-30 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydclbnc/)
* 2022-09-01 | Virtual (PST Timezone) | [Conf42](https://www.conf42.com/)
    * [**Conf42: Rustlang 2022**](https://www.conf42.com/rustlang2022)
* 2022-09-01 | Virtual | [Google Open Source Live](https://www.meetup.com/google-open-source/)
    * [**Rust Day on Google Open Source Live**](https://www.meetup.com/google-open-source/events/287435626/)
* 2022-09-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together**](https://www.meetup.com/rust-noris/events/287092397/)
* 2022-09-03 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059974/)
* 2022-09-03 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 1: Tokio my-redis Tutorial**](https://www.meetup.com/rust-noris/events/287346970/)
* 2022-09-04 | Virtual (Seattle, WA, US) | [Seattle Rust Meetup](https://www.meetup.com/seattle-rust-meetup/)
    * [**September Meetup**](https://www.meetup.com/seattle-rust-meetup/events/287726278/)
* 2022-09-06 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/286481325/)
* 2022-09-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcmbjb/)
* 2022-09-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121715/)
* 2022-09-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-09-10 | Virtual (Bangalore, IN) | [Polkadot India](https://www.meetup.com/polkadot-india/)
    * [**Substrate Saturday - Bootcamp Series 2: Fundamentals of Rust & Substrate**](https://www.meetup.com/polkadot-india/events/287059979/)
* 2022-09-12 | Virtual + Dublin, IE | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022**](https://lpc.events/event/16/sessions/150/)
* 2022-09-13 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcmbrb/)
* 2022-09-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/286485815/)
* 2022-09-14 | Virtual (Malaysia)| [Golang Malaysia](https://docs.google.com/forms/d/e/1FAIpQLScKGolWclIOR1OBCzTOitVU5Am5lSYxSlVhK71DGsc-fa-Yhg/viewform)
    * [**Rust Meetup September 2022**](https://discord.gg/9Xj8H2EXTD)
* 2022-09-15 | Virtual (Columbus, OH, US) | [GDG Columbus](https://www.meetup.com/gdg-columbus/)
    * [**Past, Present, and Future of Internet Money! (Custom tokenomics, RUST and CosmWASM library...)**](https://www.meetup.com/gdg-columbus/events/287972746/)
* 2022-09-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/287004599/)
* 2022-09-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out (Call for Participation)**](https://www.meetup.com/vancouver-rust/events/285933975/)


### Europe
* 2022-08-25 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**CPH Hack Night #28**](https://www.meetup.com/copenhagen-rust-meetup-group/events/287635498/)
* 2022-08-25 | Stockholm, SE | [StockholmCpp](https://www.meetup.com/stockholmcpp/)
    * [**0x21: Learning from Rust, Typical C++**](https://www.meetup.com/stockholmcpp/events/286854212/)
* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)
* 2022-09-12 | Dublin, IE + Virtual | [Linux Plumbers Conference](https://lpc.events)
    * [**Rust Microconference in LPC 2022**](https://lpc.events/event/16/sessions/150/)

### North America

* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)
* 2022-08-25 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Concurrencia & paralelismo con Rust**](https://www.meetup.com/rust-mx/events/287561814/)
* 2022-08-25 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Using Github Actions to Deploy Cargo Crates with Jordan and Food!**](https://www.meetup.com/utah-rust/events/kvrxqsydclbpb/)
* 2022-08-31 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/) 
    * [**August Meetup: Rewriting a high performance Vector Database in Rust.**](https://www.meetup.com/rust-nyc/events/287821884/)

### Oceania

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
