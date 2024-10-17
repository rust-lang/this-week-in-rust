Title: This Week in Rust 569
Number: 569
Date: 2024-10-16
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
* [Announcing the Rust Foundation’s 2024 Fellows](https://foundation.rust-lang.org/news/announcing-the-rust-foundation-s-2024-fellows/)

### Newsletters
* [This Week In Bevy - Animation Events, Curves, and no_std](https://thisweekinbevy.com/issue/2024-10-14-animation-events-curves-and-nostd)

### Project/Tooling Updates
* [On Rust in enterprise kernels](https://lwn.net/SubscriberLink/993337/68c7e5af573545e6/)
* [FFI type mismatches in Rust for Linux](https://lwn.net/SubscriberLink/993163/3c2b06af07814cd2/)
* [Zapping pointers out of thin air](https://lwn.net/SubscriberLink/993484/385b176aa8939e7b/)
* [Using LKMM atomics in Rust](https://lwn.net/SubscriberLink/993785/cbe8cf5846d6864a/)

* "pigg" (the Raspberry Pi GPIO GUI) 0.4.0 released with TCP connection support [Release](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.4.0), [Discussion](https://github.com/andrewdavidmackenzie/pigg/discussions/503)

### Observations/Thoughts
* [Why `Pin` is a part of trait signatures (and why that's a problem)](https://blog.yoshuawuyts.com/why-pin/)
* [The `Overwrite` trait and `Pin`](https://smallcultfollowing.com/babysteps/blog/2024/10/14/overwrite-and-pin)
* [Improve an algorithm performance step by step](https://blog.mapotofu.org/blogs/rabitq-bench/)
* [Replacing nginx with axum](https://felix-knorr.net/posts/2024-10-13-replacing-nginx-with-axum.html)
* [An experiment in async Rust](https://ochagavia.nl/blog/an-experiment-in-async-rust/)
* [Designing A Fast Concurrent Hash Table](https://ibraheem.ca/posts/designing-papaya/)
* [Rethinking Builders… with Lazy Generics](https://geo-ant.github.io/blog/2024/rust-rethinking-builders-lazy-generics/)
* [IPC in Rust](https://pranitha.rs/posts/rust-ipc-ping-pong/)
* [Serde Trait - Part 3: Deserialization](https://voelklmichael.github.io/Blog/serde-trait-part3.html)
* [Memory for Nothing: Why Vec is (probably) a bad idea](https://pwy.io/posts/memory-for-nothing/)
* [Upgrade the Logging in your Rust Tests](https://tylerjw.dev/posts/20241012-rust-logging-in-tests/)
* [Nine Rules for Running Rust on Embedded Systems](https://medium.com/towards-data-science/nine-rules-for-running-rust-on-embedded-systems-b0c247ee877e)
* [Why Rust is taking the data engineering world by storm](https://kerkour.com/rust-data-engineering)

### Rust Walkthroughs

* [video] [Build with Naz : Rust memory address and size](https://www.youtube.com/watch?v=ivqIty5EOf8)
### Research

### Miscellaneous

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
* *No calls for testing were issued this week.*

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

No major changes this week.

Triage done by **@simulacrum**.
Revision range: [e6c46db4..5ceb623a](https://perf.rust-lang.org/?start=e6c46db4e9fd11e3183c397a59d946731034ede6&end=5ceb623a4abd66e91e7959d25caaf0523f1a7f7c&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-10-14.md)

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
* [disposition: merge] [stabilize Strict Provenance and Exposed Provenance APIs](https://github.com/rust-lang/rust/pull/130350)
* [disposition: merge] [make unsupported_calling_conventions a hard error](https://github.com/rust-lang/rust/pull/129935)
* [disposition: merge] [Decide on name for `derive(SmartPtr)`](https://github.com/rust-lang/rust/issues/129104)
* [disposition: merge] [Stabilize `shorter_tail_lifetime`](https://github.com/rust-lang/rust/issues/131445)
* [disposition: merge] [Finish stabilization of `result_ffi_guarantees`](https://github.com/rust-lang/rust/pull/130628)
* [disposition: merge] [Implement edition 2024 match ergonomics restrictions](https://github.com/rust-lang/rust/pull/131381)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: No (opsem) Magic Boxes](https://github.com/rust-lang/rfcs/pull/3712)
* [new] [`#[inline(required)]`](https://github.com/rust-lang/rfcs/pull/3711)
* [new] [RFC: introduce the flavor syntactic design pattern](https://github.com/rust-lang/rfcs/pull/3710)

## Upcoming Events

Rusty Events between 2024-10-16 - 2024-11-13 🦀

### Virtual
* 2024-10-10 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 2 of 4 - Navigating Rust Web Frameworks: Axum, Actix, and Rocket**](https://www.meetup.com/women-in-rust/events/303213792/)
* 2024-10-10 | Virtual (Barcelona, ES) | [BcnRust](https://bcnrust.github.io) + [Codurance](https://www.codurance.com/) + [Heavy Duty Builders](https://heavyduty.builders/)
    * [**15th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/303443195/)
* 2024-10-10 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633270/)
* 2024-10-10 | Virtual (Girona, ES) | [Rust Girona](https://www.meetup.com/rust-girona/)
    * [**Leveraging Rust to Improve Your Programming Fundamentals & De Rust A Solana**](https://www.meetup.com/rust-girona/events/303484509/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-14 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/302828025/)
* 2024-10-14 | Virtual | [Rust for Lunch](https://lunch.rs/)
    * [**Rust for Lunch - Getting started with embedded Rust (Speaker: Sandro Stikić)**](https://lunch.rs/meetups/2024-07-09/) | [Online meeting link](https://lecture.senfcall.de/hay-gmh-wox-mru)
* 2024-10-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346970/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 3 of 4 - Hackathon Ideation Lab**](https://www.meetup.com/women-in-rust/events/303213817/)
* 2024-10-17| Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898023)
* 2024-10-22 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcnbdc/)
* 2024-10-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Part 4 of 4 - Hackathon Showcase: Final Projects and Presentations**](https://www.meetup.com/women-in-rust/events/303213835/)
* 2024-10-24 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633271/)
* 2024-10-26 | Virtual (Gdansk, PL) | [Stacja IT Trójmiasto](https://www.meetup.com/stacja-it-trojmiasto/)
    * [**Rust – budowanie narzędzi działających w linii komend**](https://www.meetup.com/stacja-it-trojmiasto/events/303550643/)
* 2024-10-29 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585671/)
* 2022-10-31 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820274/)
* 2024-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633272/)

### Africa
* 2024-11-02 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia
* 2024-10-09 | Subang Jaya / Kuala Lumpur, Selangor, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup - Traits and How to Read Trait (October 2024)**](https://docs.google.com/forms/d/e/1FAIpQLScNS5IWmnzTTJAOw-RIxdj4_BWbxB5NVmAVO30XHr_viMbLqQ/viewform)
* 2024-10-17 - 2024-10-18 | Beijing, CN | [Global Open-Source Innovation Meetup (GOSIM)](https://www.gosim.org/)
    * [**GOSIM 2024**](https://china2024.gosim.org/)
* 2024-10-19 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**October 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/october-2024-rustacean-meetup/)

### Europe
* 2024-10-09 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcnbmb/)
* 2024-10-10 - 2024-10-11 | Virtual and In-Person (Vienna, AT) | [Euro Rust](eurorust)
    * [**Euro Rust 2024**](https://eurorust.eu/)
* 2024-10-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 2024-10-17 | Darmstadr, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Code Together**](https://www.meetup.com/rust-rhein-main/events/303240000/)
* 2024-10-15 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/303606799/)
* 2024-10-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)
* 2024-10-15 | Munich, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/303273953/)
* 2024-10-16 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks October - Leptos and Crux**](https://www.meetup.com/rust-manchester/events/303658240/)
* 2024-10-17 | Barcelona, ES | [BcnRust](https://bcnrust.github.io)
    * [**16th BcnRust Meetup**](https://www.meetup.com/bcnrust/events/303792888/)
* 2024-10-17 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #3**](https://www.meetup.com/rust-bern/events/303617330/)
* 2024-10-22 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**New Rust Warsaw Meetup #2**](https://www.meetup.com/rust-warsaw/events/303619536/)
* 2024-10-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Ferris' Fika Forum #6**](https://www.meetup.com/stockholm-rust/events/303918943/)
* 2024-10-28 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust Meetup #71**](https://www.meetup.com/rust-paris/events/303663366/)
* 2024-10-29 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/303479865)
* 2024-10-30 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn October 2024**](https://www.meetup.com/rust-meetup-hamburg/events/303373054/)
* 2024-10-31 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/300820289/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)

### North America
* 2024-10-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638250/)
* 2024-10-16 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631737/)
* 2024-10-17 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**October Meetup**](https://www.meetup.com/join-srug/events/303545170/)
* 2024-10-19 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Oct 19**](https://www.meetup.com/bostonrust/events/303708335/)
* 2024-10-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcnbfc/)
* 2024-10-27 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Kendall Rust Lunch, Oct 27**](https://www.meetup.com/bostonrust/events/303708359/)
* 2024-11-04 | Brookline, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Coolidge Corner Brookline Rust Lunch, Nov 4**](https://www.meetup.com/bostonrust/events/303708387/)

### Oceania
* 2024-10-29 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/303128131/)

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
