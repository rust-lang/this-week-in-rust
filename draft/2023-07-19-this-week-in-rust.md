Title: This Week in Rust 504
Number: 504
Date: 2023-07-19
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
* [Announcing Rust 1.71.0](https://blog.rust-lang.org/2023/07/13/Rust-1.71.0.html)
* [Rustc Trait System Refactor Initiative Update](https://blog.rust-lang.org/inside-rust/2023/07/17/trait-system-refactor-initiative.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [rust-analyzer changelog #190](https://rust-analyzer.github.io/thisweek/2023/07/17/changelog-190.html)
* [Generating terminal user interfaces with Ratatui + ChatGPT](https://blog.orhun.dev/ratatui-0-22-0/)
* [Kani 0.32.0 has been released!](https://www.reddit.com/r/KaniRustVerifier/comments/14xytrg/kani_0320_has_been_released/)
* [Bevy XPBD 0.2.0: Spatial queries, Bevy 0.11 support, and a lot more](https://www.reddit.com/r/rust_gamedev/comments/14zr5i5/bevy_xpbd_020_spatial_queries_bevy_011_support/)
* [bwrap : A fast, lightweight, embedded environment-friendly Rust library for wrapping text](https://www.reddit.com/r/rust/comments/151usd5/bwrap_a_fast_lightweight_embedded/)

### Observations/Thoughts
* [Mutex without lock, Queue without push: cancel safety in lilos](https://cliffle.com/blog/lilos-cancel-safety/)
* [Why Rust is a great fit for embedded software - 2023 update](https://tweedegolf.nl/en/blog/96/why-rust-is-a-great-fit-for-embedded-software-2023-update)
* [{n} times faster than C, where n = 128](https://ipthomas.com/blog/2023/07/n-times-faster-than-c-where-n-128/)
* [I have written a JVM in Rust](https://andreabergia.com/blog/2023/07/i-have-written-a-jvm-in-rust/)
* [On Maximizing Your Rust Code's Performance](https://jbecker.dev/research/on-writing-performant-rust)
* [How `rustdoc` achieves a genius design](https://blog.goose.love/posts/rustdoc/)
* [ESP32 Standard Library Embedded Rust: GPIO Control](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-gpio-control)

### Rust Walkthroughs
* [Writing an AI Chatbot in Rust and Solid.js](https://thetechtrailblazer.blog/2023/07/18/writing-an-ai-chatbot-in-rust-and-solid-js/)

### Research
* [RustSmith: Random Differential Compiler Testing for Rust](https://www.doc.ic.ac.uk/~afd/homepages/papers/pdfs/2023/ISSTA-tool.pdf)

### Miscellaneous
* [video] [How Functions Function](https://www.youtube.com/watch?v=SqT5YglW3qU)
* [video] [Rust's iterators are more interesting than they look](https://www.youtube.com/watch?v=dad1NQdjd0I)
* [audio] [Bootstrapping Rust with Albert Larsan](https://rustacean-station.org/episode/albert-larsan/)

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

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

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

Rusty Events between 2023-07-19 - 2023-08-16 🦀

### Virtual

* 2023-07-11 - 2023-07-13 | Virtual (Europe) | [Mainmatter](https://mainmatter.com/)
    * [**Web-based Services in Rust, 3-day Workshop with Stefan Baumgartner**](https://rust-web-services-workshop.mainmatter.com/)  
* 2023-07-13 - 2023-07-14 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs/)
    * [**Scientific Computing in Rust workshop**](https://scientificcomputing.rs/)
* 2023-07-13 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/294707594/)
* 2023-07-13 | Virtual (Edinburgh, UK) | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**Reasoning about Rust: an introduction to Rustdoc’s JSON format**](https://www.meetup.com/rust-edi/events/293820336/)
* 2023-07-13 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #27**](https://www.meetup.com/rust-noris/events/289732650)
* 2023-07-15 | Virtual (Chandigarh, IN) | [Rust Chandigarh](https://hasgeek.com/rustchandigarh)
    * [**Rust Chandigarh Meetup #1**](https://hasgeek.com/rustchandigarh/july-2023-rust-meetup/)
* 2023-07-18 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfckbxb/)
* 2023-07-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763486)
* 2023-07-20 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #12 - Ownership and Memory management**](https://rust-meetup.ir/2023/07/20/12th-meetup.html)
* 2023-07-24 | Virtual (Toronto, CA) | [Programming Languages Virtual Meetup](https://www.meetup.com/programming-languages-toronto-meetup/)
    * [**Crafting Interpreters Chapter 18: Types of Values**](https://www.meetup.com/programming-languages-toronto-meetup/events/294616842)
* 2023-07-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfckbhc/)
* 2023-07-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfckbkc/)
* 2023-07-28 | Virtual (Tunis, TN) | [Rust Meetup Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number IV**](https://www.meetup.com/rust-tunisia/events/294664236/)
* 2023-08-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfclbcb//)
* 2023-08-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfclblb/)

### Asia

* 2023-07-13 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**A "Bit" of Fun With Geometry in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/294571542)

### Europe

* 2023-07-13 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - beer garden Edition**](https://www.meetup.com/rust-berlin/events/294627419)
* 2023-07-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/mstlftyfckbrb/)
* 2023-07-21 | Nuremberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together #2**](https://www.meetup.com/rust-noris/events/293823522/)

### North America

* 2023-07-12 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/294373345)
* 2023-07-12 | Waterloo, ON, CA | [Rust KW](https://www.meetup.com/rust-kw/)
    * [**Overengineering FizzBuzz**](https://www.meetup.com/rust-kw/events/294355516/)
* 2023-07-13 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Writing Kubernetes Operators in Rust**](https://www.meetup.com/utah-rust/events/294604589/)
* 2023-07-13 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294631273)
* 2023-07-13 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**July Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294191599/)
* 2023-07-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfckbxb)

### Oceania

* 2023-07-18 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**July Meetup**](https://www.meetup.com/rust-canberra/events/294321350/)

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
