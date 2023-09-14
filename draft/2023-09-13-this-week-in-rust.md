Title: This Week in Rust 512
Number: 512
Date: 2023-09-13
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

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage


An interesting week. We saw a massive improvement to instruction-counts across
over a hundred benchmarks, thanks to #110050 an improved encoding scheme for the
dependency graphs that underlie incremental-compilation. However, these
instruction-count improvements did not translate to direct cycle time
improvements. We also saw an improvement to our artifact sizes due to #115306.
Beyond that, we had a scattering of small regressions to instruction-counts that
were justified because they were associated with bug fixes.

Triage done by **@pnkfelix**.
Revision range: [15e52b05..7e0261e7](https://perf.rust-lang.org/?start=15e52b05ca8f63e0da27c808680388717e5b997e&end=7e0261e7ea2085bdc0bc3d0fd6776bf343473858&absolute=false&stat=instructions%3Au)

3 Regressions, 2 Improvements, 5 Mixed; 2 of them in rollups
84 artifact comparisons made in total

[Full report  - pending](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-09-13.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Allow cfg-attributes in where clauses](https://github.com/rust-lang/rfcs/pull/3399)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Cargo Check T-lang Policy](https://github.com/rust-lang/rfcs/pull/3477)
* [disposition: merge] [[RFC2603] Extend `<const>` to include `str` and structural constants.](https://github.com/rust-lang/rfcs/pull/3161)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Raise minimum supported Apple OS versions](https://github.com/rust-lang/rust/pull/104385)
* [disposition: merge] [Stabilize const_transmute_copy](https://github.com/rust-lang/rust/pull/115520)
* [disposition: merge] [Don't resolve generic impls that may be shadowed by dyn built-in impls](https://github.com/rust-lang/rust/pull/114941)
* [disposition: merge] [closure field capturing: don't depend on alignment of packed fields](https://github.com/rust-lang/rust/pull/115315)
* [disposition: merge] [Accept additional user-defined syntax classes in fenced code blocks](https://github.com/rust-lang/rust/pull/110800)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Unify crate categories and keywords as tags](https://github.com/rust-lang/rfcs/pull/3488)
* [new] [RFC: Cargo feature visibility (private/public)](https://github.com/rust-lang/rfcs/pull/3487)
* [new] [RFC: Cargo feature deprecation](https://github.com/rust-lang/rfcs/pull/3486)
* [new] [RFC: Cargo feature descriptions](https://github.com/rust-lang/rfcs/pull/3485)
* [new] [Unsafe Extern Blocks](https://github.com/rust-lang/rfcs/pull/3484)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-09-13 - 2023-10-11 🦀

### Virtual

* 2023-09-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294049877)
* 2023-09-06 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/294343596/)
* 2023-09-07 | Virtual (Ann Arbor, MI, US) | [Michigan Python](https://www.meetup.com/michigan-python/)
    * [**Online MI Python: Improving Python Speed with a Bit of Rust**](https://www.meetup.com/michigan-python/events/294951180/)
* 2023-09-12 - 2023-09-15 | Virtual (Albuquerque, NM, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295635473/)
* 2023-09-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/gqdlgtyfcmbqb/)
* 2023-09-13 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/295011539)
* 2023-09-13 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-09-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732655)
* 2023-09-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—NeuronBench by Greg Hale**](https://www.meetup.com/rustdc/events/295778065)
* 2023-09-20 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**SurrealDB for Rustaceans**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/295826608/)
* 2023-09-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/295057154/)
* 2023-09-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295666673/)
* 2023-09-21 | Virtual (Cologne, DE) | [Cologne AWS User Group #AWSUGCGN](https://www.meetup.com/aws-cologne/)
    * [**AWS User Group Cologne - September Edition: Stefan Willenbrock: Developer Preview: Discovering Rust on AWS**](https://www.meetup.com/aws-cologne/events/294594401/)
* 2023-09-21 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Real Time Multiplayer Game Server in Rust**](https://www.meetup.com/utah-rust/events/294972877/)
* 2023-09-21 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 33rd Edition**](https://www.meetup.com/rust-linz/events/295363887/)
* 2023-09-21 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/295828383/)
* 2023-09-25 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**How we built the SurrealDB Python client in Rust.**](https://www.meetup.com/Rust-Dublin/events/294908596/)
* 2023-09-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcmbjc/)
* 2023-10-03 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295919493/)
* 2023-10-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcnbgb/)

### Asia

* 2023-10-03 | Taipei, TW | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/)
    * [**WebAssembly Meetup (Wasm Empowering AI) in Taipei**](https://www.meetup.com/wasm-rust-meetup/events/295672575/)

### Europe

* 2023-09-12 | Berlin, DE | [Berlin AWS User Group](https://www.meetup.com/berlinawsug/)
    * [**Berlin AWS Group Meetup - September 2023: Luca Zonta // Sustainable Serverless Computing with Rust**](https://www.meetup.com/berlinawsug/events/295565048/)
* 2023-09-12 | Zurich, CH| [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**The Lightning Rust Quiz - September Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/295804450/)
* 2023-09-13 | Cologne, DE | [Rust User Group Cologne](https://rust.cologne/2023/09/13/rare-rust.html)
    * [**Rare Rust**](https://www.meetup.com/rustcologne/events/295869748/) | [**Group Detail Page**](https://rust.cologne/2023/09/13/rare-rust.html)
* 2023-09-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/295109905/)
* 2023-09-15 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart/)
    * [**OnsiteMeeting**](https://www.meetup.com/rust-community-stuttgart/events/295639296/)
* 2023-09-15 | Tiel, NL | [Rust, Getting Started](https://www.meetup.com/rust-getting-started/)
    * [**Rust Workshop - 2**](https://www.meetup.com/rust-getting-started/events/295880062/)
* 2023-09-16 | Brussels, BE | [HSBXL](https://hsbxl.be/events/software-freedom-day/2023-09-16/)
    * [**Software Freedom Day 2023**](https://www.meetup.com/brussels-hackerspace/events/295912633/)
* 2023-09-19 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Logging and tracing in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504245/)
* 2023-09-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus - Rust and Talk at Concordium**](https://www.meetup.com/rust-aarhus/events/294031975/)
* 2023-09-21 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2023 🦀**](https://www.meetup.com/rust-bern/events/295503351/)
* 2023-09-26 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679767/)
* 2023-09-28 | Berlin, DE | [React Berlin](https://www.meetup.com/react-berlin-meetup/)
    * [**React Berlin September Meetup: Creating Videos with React & Remotion & More: Integrating Rust with React Native – Gheorghe Pinzaru**](https://www.meetup.com/react-berlin-meetup/events/295382108/)

### North America

* 2023-09-06 | Bellevue, WA, US | [The Linux Foundation](https://www.linuxfoundation.org/)
    * [**Rust Global**](https://events.linuxfoundation.org/rust-global/)
* 2023-09-07 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/295614871/) | [**Mountain View Rust Meetup Mirror**](https://www.meetup.com/mv-rust-meetup/events/295746992/)
* 2023-09-07 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Weekly Pasadena Python study group**](https://www.meetup.com/thursday-go/events/295818856/)
* 2023-09-09 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295572737/)
* 2023-09-12 - 2023-09-15 | Albuquerque, NM, US  + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**A Panel Discussion on Thriving in a Rust-Driven Workplace**](https://www.meetup.com/rust-nyc/events/295639294)
* 2023-09-12 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/295744114/)
* 2023-09-14 | Seattle, WA, US | [Seattle Rust User Group Meetup](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group - August Meetup**](https://www.meetup.com/seattle-rust-user-group/events/295484105)
* 2023-09-16 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295579189/)
* 2023-09-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/295545278)
* 2023-09-21 | Mountain View, CA, US| [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295747006/)
* 2023-09-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Rust on the web! Get started with Leptos**](https://www.meetup.com/music-city-rust-developers/events/295587220/)
* 2023-09-23 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/295869150/)
* 2023-09-26 | Pasadena, CA, US | [Pasadena Thursday Go/Rust](https://www.meetup.com/thursday-go/)
    * [**Monthly Rust group**](https://www.meetup.com/thursday-go/events/295771515)
* 2023-09-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295466314)
* 2023-09-30 | Mountain View, CA, US | [Rust Breakfast and Learn](https://www.meetup.com/rust-breakfast-learn/)
    * [**Rust: breakfast & learn**](https://www.meetup.com/rust-breakfast-learn/events/fktvgtyfcmbnc/)

### Oceania

* 2023-09-13 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**Rust Meetup 2: Lunch & Learn**](https://www.linkedin.com/events/7097356771584880640/) | [**Ticket Link**](https://www.tickettailor.com/events/perthrustusergroup/984771)
* 2023-09-19 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/295602231/)
* 2023-09-26 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**September Meetup**](https://www.meetup.com/rust-canberra/events/295432237/)

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
