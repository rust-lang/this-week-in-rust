Title: This Week in Rust 467
Number: 467
Date: 2022-11-02
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

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

Noise continues to make triaging a bit tedious. We've become good at identifying noise, but we may need to invest in trying to reduce it or automate some of the triaging needed to identify it. In terms of performance, this week ending up being positive albeit with improvements only outweighing regressions by a little. Some of the largest improvements were in reverts of previous regressions as well.

Triage done by **@rylev**.
Revision range: [629a414d..822f8](https://perf.rust-lang.org/?start=629a414d7ba4caa3ca28b0a46c478e2ecb4c0059&end=822f8c22f540b12f296d844ad5bf39aaa47bfeb4&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.7%  | [0.2%, 7.9%]   | 28    |
| Regressions ❌ <br /> (secondary)  | 1.7%  | [0.2%, 7.0%]   | 97    |
| Improvements ✅ <br /> (primary)   | -1.2% | [-4.6%, -0.2%] | 73    |
| Improvements ✅ <br /> (secondary) | -1.3% | [-2.6%, -0.3%] | 61    |
| All ❌✅ (primary)                 | -0.4% | [-4.6%, 7.9%]  | 101   |

13 Regressions, 9 Improvements, 5 Mixed; 9 of them in rollups
41 artifact comparisons made in total

See [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-11-02.md) for details.

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-11-02 - 2022-11-30 🦀

### Virtual

* 2022-10-26 | Virtual (Redmond, WA, US / New York, NY, US / Toronto, CA / Stockholm, SE / London, UK) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Your First Rust Project: Rust Basics**](https://www.meetup.com/microsoft-reactor-redmond/events/288475815/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475839/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475818/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/288475819/) | [**London Mirror**](https://www.meetup.com/microsoft-reactor-london/events/288475821/)
* 2022-10-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Using Applicative Functors to parse command line options**](https://www.meetup.com/charlottesville-rust-meetup/events/288867237/)
* 2022-10-27 | Virtual (Karlsruhe, DE) | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA) (various topics, from C++ to Rust...)**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/288972651/)
* 2022-10-27 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 26th Edition**](https://www.meetup.com/rust-linz/events/289212637/)
* 2022-10-29 | Virtual (Ludwigslust, DE) | [Ludwigslust Rust Meetup](https://www.meetup.com/ludwigslust-rust-meetup/)
    * [**Von Nullen und Einsen | Rust Meetup Ludwigslust #1**](https://www.meetup.com/ludwigslust-rust-meetup/events/289168194/)
* 2022-11-01 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/events/289066646/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/289066646/)
* 2022-11-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcpbcb/)
* 2022-11-02 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust and C++ Cardiff Virtual Meet**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289052285/)
* 2022-11-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcpbdb/)
* 2022-11-02 | Virtual (Redmond, WA, US / San Francisco, SF, US / New York, NY, US / Toronto, CA / London, UK) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Getting Started with Rust: From Java Dev to Rust Developer**](https://www.meetup.com/microsoft-reactor-redmond/events/288475833/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475838/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475839/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475836/) | [**London Mirror**](https://www.meetup.com/microsoft-reactor-london/events/288475832/) 
* 2022-11-02 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/289092511/)
* 2022-11-08 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289211414/)
* 2022-11-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcpblb/)
* 2022-11-08 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/289352420/)
* 2022-11-08 | Virtual (Stockholm, SE) | [Func Prog Sweden](https://www.meetup.com/func-prog-sweden/)
    * [**Tenth Func Prog Sweden MeetUp 2022 – Online (with "Ready for Rust" by Erik Dörnenburg)**](https://www.meetup.com/func-prog-sweden/events/288323896/)
* 2022-11-09 | Virtual (Malaysia, MY) | [Rust Malaysia](https://forms.gle/zWXcMDAnnibiL4ni9)
    * [**Rust Meetup November 2022 - a couple of lightning talks**](https://discord.gg/9Xj8H2EXTD)
* 2022-11-10 | Virtual (Budapest, HU) | [HWSW free!](https://www.meetup.com/hwswfree/)
    * [**RUST! RUST! RUST! meetup (online formában!)**](https://www.meetup.com/hwswfree/events/289044458/)
* 2022-11-12 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-11-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc//)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/289015883/)
* 2022-11-15 | Virtual (Nairobi, KE / New York, NY, US)| [Data Umbrella Africa](https://www.meetup.com/data-umbrella-africa2/)
    * [**Online: Introduction to Rust Programming**](https://www.meetup.com/data-umbrella-africa2/events/289308825/) | [**New York Mirror**](https://www.meetup.com/data-umbrella/events/289308172/)
* 2022-11-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcpbvb/)
* 2022-11-17 | Virtual (Amsterdam, NL) | [ITGilde Tech-Talks](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups)
    * [**Introduction “Rust” an ITGilde Tech Talk delivered by Pascal van Dam**](https://www.meetup.com/itgilde-cooperatie-amsterdam-unix-linux-meetups/events/289167373/)
* 2022-11-21 | Virtual (Paris, FR) | [Meetup Paris - École Supérieure de Génie Informatique (ESGI)](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique)
    * [**Découverte de WebAssembly**](https://www.meetup.com/meetup-paris-ecole-superieur-du-genie-informatique/events/289112753/)

### Asia

* 2022-11-08 | Bangkok, TH | [Tech@Agoda](https://www.meetup.com/techatagoda/)
    * [**Rustacean Bangkok 5.0.0**](https://www.meetup.com/techatagoda/events/289329464/)

### Europe

* 2022-10-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks October 2022: Host by Amazon Prime Video**](https://www.meetup.com/rust-london-user-group/events/289023932/)
* 2022-10-26 | Bristol, UK | [Rust and C++ Cardiff/Rust Bristol](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Programming Veloren & Rust for a living**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289204085/)
* 2022-10-27 | København, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #30**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179125/)
* 2022-11-16 | Paris, FR | [Stockly](https://www.eventbrite.fr/o/stockly-42274765293)
    * [**Rust Meetup in Paris - hosted by Stockly**](https://www.eventbrite.fr/e/rust-meetup-in-paris-hosted-by-stockly-tickets-444152621447?aff=ebdssbdestsearch)
* 2022-11-23 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Initial Meet and Greet Rust meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/289028178/)    
### North America

* 2022-10-27 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Bevy Crash Course with Nathan and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsydcnbkc/)
* 2022-11-10 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcpbnb/)

### Oceania

* 2022-11-09 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**RustAU Sydney - Last physical for 2022 !**](https://www.meetup.com/rust-sydney/events/289061840/)
* 2022-11-22 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/288615873/)

### South America

* 2022-11-05 | São Paulo, SP, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/)
    * [**Rust-SP meetup Outubro 2022**](https://www.meetup.com/rust-sao-paulo-meetup/events/289176073/)

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
