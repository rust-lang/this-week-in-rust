Title: This Week in Rust 485
Number: 485
Date: 2023-03-08
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
* [RustConf 2023 is Coming—Submit a Talk Today!](https://foundation.rust-lang.org/news/rustconf-2023-is-coming-submit-a-talk-today/)
* [Member Spotlight: HighTec](https://foundation.rust-lang.org/news/member-spotlight-hightec/)
* [Best Practices for Integrating Rust and Qt in Embedded Systems](https://foundation.rust-lang.org/news/best-practices-for-integrating-rust-and-qt-in-embedded-systems/)

### Rust Nation 2023

* [Opening Address - Rebecca Rumbul](https://www.youtube.com/watch?v=y4_a3ALa_20)
* [The Rustacean Cycle: Learn, Teach, Deliver - Nell Shamrell-Harrington](https://www.youtube.com/watch?v=dTktT8y8niw)
* [Fast, Flexible Iteration with Rust and Rhai - Jonathan Strong](https://www.youtube.com/watch?v=u4Dd7dBxcEA)
* [iOS, Android and Web applications that share a single Rust core - Stuart Harris](https://www.youtube.com/watch?v=cWCZms92-1g)
* [Tricks of the Trait: Enabling Ergonomic Extractors - Rob Ede](https://www.youtube.com/watch?v=7DOYtnCXucw)
* [Let's Get Rusty In Here - Daniel Thompson-Yvetot](https://www.youtube.com/watch?v=4FymKv23J34)
* [Let’s write async rust from the ground up! - Conrad Ludgate](https://www.youtube.com/watch?v=7pU3gOVAeVQ)
* [What I learned by solving 50 Advent of Code challenges in Rust - Luciano Mammino](https://www.youtube.com/watch?v=udHjmno-tfA)
* [Spreading Rust to the rest of the company: Moving past the proof of concept - Tim McNamara](https://www.youtube.com/watch?v=4DLhTPsg8QQ)
* [A tale of binary translation - Amanieu D'Antras](https://www.youtube.com/watch?v=Avp55U2JFcQ)
* [Embracing Rust at fly.io: How Rust powers our networking layer - Senyo Simpson](https://www.youtube.com/watch?v=-O31eFqBmf4)
* [Closing the Supply Chain Security Loop with Rust and Pyrsia - Steven Chin](https://www.youtube.com/watch?v=ec8vvD1SG-s)
* [SurrealDB: from Golang to Rust — building the world’s fastest-growing db - Tobie Morgan Hitchcock](https://www.youtube.com/watch?v=Chl8IdMxr4Y)
* [Moving beyond `Arc<Mutex<T>>` - Katharina Fey](https://www.youtube.com/watch?v=Z-2siR9Ki84)
* [What does the Rust Foundation do? - Rust Foundation team](https://www.youtube.com/watch?v=EFtassAEK5A)
* [Rust on RISC-V, a case study - Jorge Prendes and James Wainwright](https://www.youtube.com/watch?v=t5q0M5VDlQM)
* [Rust in Rhymes II - Andre Bogus](https://www.youtube.com/watch?v=rwH57zNm-A8)
* [Living with Rust Long-Term - Jon Gjengset](https://www.youtube.com/watch?v=r35cBkPRNMI)

### Newsletters

### Project/Tooling Updates
* [Bevy 0.10](https://bevyengine.org/news/bevy-0-10/)
* [rust-analyzer hangelog #171](https://rust-analyzer.github.io/thisweek/2023/03/06/changelog-171.html)
* [Zellij 0.35.1 Stacked Panes and Swap Layouts](https://zellij.dev/news/stacked-panes-swap-layouts/)
* [Oxy is Cloudflare's Rust-based next generation proxy framework](https://blog.cloudflare.com/introducing-oxy/)
* [Fornjot (code-first CAD in Rust) - Weekly Release - Progressed Extremely Well](https://www.fornjot.app/blog/weekly-release/2023-w10/)
* [Databend 1.0 Release - The Future of Cloud Data Analytics](https://databend.rs/blog/databend-release-v1.0)
* [Introducing runst: Handle desktop notifications neatly on Linux!](https://blog.orhun.dev/introducing-runst/)
* [This Month in hyper: February 2023](https://seanmonstar.com/post/710694914534539264/this-month-in-hyper-february-2023)

### Observations/Thoughts
* [Safety and Soundness in Rust](https://jacko.io/safety_and_soundness.html)
* [Re-exporting an enum with a type alias is breaking, but not major](https://predr.ag/blog/re-exporting-enum-with-type-alias-breaking-change-not-major/)
* [Trait transformers (send bounds, part 3)](https://smallcultfollowing.com/babysteps/blog/2023/03/03/trait-transformers-send-bounds-part-3/)
* [Professional Rustacean, 3 months in](https://briankung.dev/2023/02/17/professional-rustacean-3-months-in/)
* [Rust coding style](https://tzemanovic.gitlab.io/posts/rust-coding-style/)
* [Fixing the Next 10,000 Aliasing Bugs](https://blog.polybdenum.com/2023/03/05/fixing-the-next-10-000-aliasing-bugs.html)
* [\Device\Afd, or, the Deal with the Devil that makes async Rust work on Windows](https://notgull.github.io/device-afd/)
* (audio) [Rustdoc with Joshua Nelson](https://rustacean-station.org/episode/joshuan-nelson/)
* (audio) [Asynchronix with Serge Barral](https://rustacean-station.org/episode/serge-barral/)
* (video) [strace feels like magic — let’s fix that (with Rust)](https://www.youtube.com/watch?v=engduNoI6DE)
* (video) [Let's make an htop-like in your browser (with Rust)](https://www.youtube.com/watch?v=c_5Jy_AVDaM)
* (video) [Build your entire tech stack in Rust](https://www.youtube.com/watch?v=luOgEhLE2sg)


### Rust Walkthroughs
* [Rust's BufRead, And When To Use It](https://www.brandons.me/blog/bufread-and-when-to-use-it)
* [Getting Started with Rust & GPT-3](https://www.shuttle.rs/blog/2023/03/01/getting-started-with-rust-and-gpt)
* [Build a Ray Tracer, pt. 2 - Enter The Matrix](https://www.superperfundo.dev/articles/ray-tracer-part2)
* [Creating and publishing a Python package written in Rust](https://antoniosbarotsis.github.io/posts/python_package_written_in_rust/)
* [The World's Smallest Hash Table](https://orlp.net/blog/worlds-smallest-hash-table/)
* [Refactoring in Rust: Introducing Traits](https://fettblog.eu/refactoring-rust-introducing-traits/)
* [Embedded Rust on ESP32C3 Board, a Hands-on Quickstart Guide](https://gitlab.com/cyril-marpaud/rust_esp_quickstart/)
* [video] [Matching Braces With a Stack, Beginner Tutorial](https://www.youtube.com/watch?v=i_ghB5AusDs)

### Research

### Miscellaneous
* [When Zig is safer and faster than Rust](https://zackoverflow.dev/writing/unsafe-rust-vs-zig/)

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

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-03-08 - 2023-04-05 🦀

### Virtual

* 2023-03-08 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcfblb/)
* 2023-03-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsyfcfbmb/)
* 2023-03-11 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-03-14 | Virtual (Italy) | [Hinto](https://www.eventbrite.it/o/hinto-28025248045)
    * [**Webinar online | Introduzione a Rust**](https://www.eventbrite.com/e/biglietti-webinar-online-introduzione-a-rust-558594419947?aff=ebdssbonlinesearch&keep_tld=1)
* 2023-03-14 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2023/03/14/rust-hack-and-learn.html)
* 2023-03-14 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/291809763/)
* 2023-03-14 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep1**](https://www.meetup.com/microsoft-reactor-redmond/events/291676352/)
* 2023-03-14 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 27u16**](https://www.meetup.com/rust-saar/events/292076386/)
* 2023-03-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Injecting Rust Hooks into a 1999 game binary (unsafe)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291354288/)
* 2023-03-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/wqchctyfcfbtb/)
* 2023-03-16 | Virtual (Raleigh, NC, US) | [Triangle BitDevs](https://www.meetup.com/triangle-bitdevs/)
    * [**Rust for Bitcoiners**](https://www.meetup.com/triangle-bitdevs/events/292032273/)
* 2023-03-16 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Introduction to WebAssembly (WASM) with Rust and WASMEdge**](https://www.meetup.com/microsoft-reactor-redmond/events/291681809/)
* 2023-03-16 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/291847774/)
* 2023-03-21 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep2**](https://www.meetup.com/microsoft-reactor-redmond/events/291676961/)
* 2023-03-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Rust+Tell Lightning Talks**](https://www.meetup.com/rustdc/events/vdhxgsyfcfbcc/)
* 2023-03-22 | Virtual (Richmond, VA, US) | [Rustaceans RVA](https://www.meetup.com/rustaceans-rva/)
    * [**Rustaceans RVA - March Meetup**](https://www.meetup.com/rustaceans-rva/events/291963911/)
* 2023-03-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcfblc/)
* 2023-03-28 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Crack code interview problems in Rust: S2 Ep3**](https://www.meetup.com/microsoft-reactor-redmond/events/291677113/)
* 2023-03-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Writing your own rust 'book' with mdBook**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/291892487/)
* 2023-04-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfcgbgb/)
* 2023-04-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/291967741/)
* 2023-04-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfcgbhb/)


### Europe
 
* 2023-03-09 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Rust Meetup #7**](https://www.meetup.com/rust-basel/events/291228934/)
* 2023-03-09 | Delft, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Regular track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401965/)
    * [**Student track: Embedded Rust**](https://www.meetup.com/rust-nederland/events/291401778/)
* 2023-03-09 | Lyon, FR | [Rust Lyon](https://www.meetup.com/fr-FR/rust-lyon/)
    * [**Rust Lyon Meetup #2**](https://www.meetup.com/fr-FR/rust-lyon/events/291727241/)
* 2023-03-15 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk around Embedded World Exhibition**](https://www.meetup.com/rust-noris/events/291623203/)
* 2023-03-16 | Paris, FR | [OCaml Users in Paris - OUPS](https://www.meetup.com/ocaml-paris/events/)
    * [**OUPS Mars 2023: Creusot a prophetic verifier for Rust -- Xavier Denis**](https://www.meetup.com/ocaml-paris/events/291637370/)
* 2023-03-17 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfcfbwb/)
* 2023-03-28 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
    * [**High performance concurrent data structures in Rust - March Meetup**](https://www.meetup.com/de-DE/rust-zurich/events/291449557/)
* 2023-03-29 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #57**](https://www.meetup.com/rust-paris/events/291963747/)

### North America

* 2023-03-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Trails, Triumphs, & Travails of Yet-Another-Database-Crate with PJ and Food!**](https://www.meetup.com/utah-rust/events/rrwbctyfcfbmb/)

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
