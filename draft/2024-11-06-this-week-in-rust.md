Title: This Week in Rust 572
Number: 572
Date: 2024-11-06
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
* [The Embedded Rustacean Issue #31](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-31)

### Project/Tooling Updates

* [Announcing Sycamore v0.9.0](https://sycamore.dev/post/announcing-v0-9-0)

### Observations/Thoughts

* [Reached the recursion limit... at build time?](https://blog.veeso.dev/blog/en/reached-the-recursion-limit-at-build-time/)

### Rust Walkthroughs
* [Difference Between iter() and into_iter() in Rust](https://crustc.com/difference-iter-and-into_iter/)

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

A week dominated by one large improvement and one large regression where luckily the improvement had a larger impact. The regression seems to have been caused by a newly introduced lint that might have performance issues. The improvement was in building rustc with protected visibility which reduces the number of dynamic relocations needed leading to some nice performance gains. Across a large swath of the perf suit, the compiler is on average 1% faster after this week compared to last week.

Triage done by **@rylev**.
Revision range: [c8a8c820..27e38f8f](https://perf.rust-lang.org/?start=c8a8c82035439cb2404b8f24ca0bc18209d534ca&end=27e38f8fc7efc57b75e9a763d7a0ee44822cd5f7&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.8%  | [0.1%, 2.0%]    | 80    |
| Regressions ❌ <br /> (secondary)  | 1.9%  | [0.2%, 3.4%]    | 45    |
| Improvements ✅ <br /> (primary)   | -1.9% | [-31.6%, -0.1%] | 148   |
| Improvements ✅ <br /> (secondary) | -5.1% | [-27.8%, -0.1%] | 180   |
| All ❌✅ (primary)                 | -1.0% | [-31.6%, 2.0%]  | 228   |


1 Regression, 1 Improvement, 5 Mixed; 3 of them in rollups
46 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/bd155e07def612eff4cb7fec391cf60f22674673/triage/2024-11-05.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [[RFC] Default field values](https://github.com/rust-lang/rfcs/pull/3681)
* [RFC: Give users control over feature unification](https://github.com/rust-lang/rfcs/pull/3692)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [Add support for `use Trait::func`](https://github.com/rust-lang/rfcs/pull/3591)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Stabilize Arm64EC inline assembly](https://github.com/rust-lang/rust/pull/131781)
* [disposition: merge] [Stabilize s390x inline assembly](https://github.com/rust-lang/rust/pull/131258)
* [disposition: merge] [rustdoc-search: simplify rules for generics and type params](https://github.com/rust-lang/rust/pull/127589)
* [disposition: merge] [Fix ICE when passing DefId-creating args to legacy_const_generics.](https://github.com/rust-lang/rust/pull/130443)
* [disposition: merge] [Tracking Issue for `const_option_ext`](https://github.com/rust-lang/rust/issues/91930)
* [disposition: merge] [Tracking Issue for const_unicode_case_lookup](https://github.com/rust-lang/rust/issues/101400)
* [disposition: merge] [Reject raw lifetime followed by `'`, like regular lifetimes do](https://github.com/rust-lang/rust/pull/132341)
* [disposition: merge] [Enforce that raw lifetimes must be valid raw identifiers](https://github.com/rust-lang/rust/pull/132363)
* [disposition: merge] [Stabilize WebAssembly `multivalue`, `reference-types`, and `tail-call` target features](https://github.com/rust-lang/rust/pull/131080)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Implement The Update Framework for Project Signing](https://github.com/rust-lang/rfcs/pull/3724)
* [new] [[RFC] Static Function Argument Unpacking](https://github.com/rust-lang/rfcs/pull/3723)
* [new] [[RFC] Explicit ABI in extern](https://github.com/rust-lang/rfcs/pull/3722)
* [new] [Add homogeneous_`try_blocks` RFC](https://github.com/rust-lang/rfcs/pull/3721)

## Upcoming Events

Rusty Events between 2024-11-06 - 2024-12-04 🦀

### Virtual
* 2024-11-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031651/)
* 2024-11-07 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633272/)
* 2024-11-08 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304099245/)
* 2024-11-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346985/)
* 2024-11-14 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898070/)
* 2024-11-14 | Virtual and In-Person (Lehi, UT, US) | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Green Thumb: Building a Bluetooth-Enabled Plant Waterer with Rust and Microbit**](https://www.meetup.com/utah-rust/events/304206130/)
* 2024-11-14 | Virtual and In-Person (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**November Meetup**](https://www.meetup.com/join-srug/events/304166747/)
* 2024-11-15 | Virtual (Jersey City, NJ, US) | [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcpbtb/)
* 2024-11-19 | Virtual (Los Angeles, CA, US) | [DevTalk LA](https://www.meetup.com/lajugstudygroup/)
    * [**Discussion - Topic: Rust for UI**](https://www.meetup.com/lajugstudygroup/events/302952703/)
* 2024-11-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346971/)
* 2024-11-20 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Embedded Rust Workshop**](https://www.meetup.com/vancouver-rust/events/304047664/)
* 2024-11-21 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633273/)
* 2024-11-21 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Trustworthy IoT with Rust--and passwords!**](https://www.meetup.com/charlottesville-rust-meetup/events/304216847/)
* 2024-11-21 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #7**](https://www.meetup.com/bevy-game-development/events/304078762/)
* 2024-11-25 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**ONLINE Talk, sponsored by Sonalake - Bratislava Rust Meetup**](https://www.meetup.com/bratislava-rust-meetup-group/events/304373224/)
* 2024-11-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcpbjc/)
* 2024-11-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 2024-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)

### Asia
* 2024-11-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**RustTechX Summit 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 2024-11-30 | Tokyo, JP | [Rust Tokyo](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)

### Europe
* 2024-11-06 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123398/)
* 2024-11-06 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/paris-rustaceans-74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1037795553437)
* 2024-11-09 - 2024-11-11 | Florence, IT | [Rust Lab](https://rustlab.it/)
    * [**Rust Lab 2024: The International Conference on Rust in Florence**](https://rustlab.it/schedule)
* 2024-11-12 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/events/)
    * [**Encrypted/distributed filesystems, wasm-bindgen**](https://www.meetup.com/rust-zurich/events/304162840)
* 2024-11-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/303915771/)
* 2024-11-14 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/304124737/)
* 2024-11-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Daten sichern mit ZFS (und Rust)**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425200/)
* 2024-11-21 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (pub)**](https://www.meetup.com/rust-and-friends/events/304110922/)
* 2024-11-21 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154277/)
* 2024-11-23 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust + HTMX - Workshop #3**](https://www.meetup.com/rust-basel/events/303714372/)
* 2024-11-27 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 2024-11-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 2024-11-28 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 2024-11-28 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421381/)

### North America
* 2024-11-07 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/)
    * [**Chicago Rust Meetup**](https://www.meetup.com/chicago-rust-meetup/events/304327595/)
* 2024-11-07 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/)
    * [**November Monthly Social**](https://www.meetup.com/rust-montreal/events/304248702/)
* 2024-11-07 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Game development with Rust and the Bevy engine**](https://www.meetup.com/stl-rust/events/302371464/)
* 2024-11-12 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcpbqb/)
* 2024-11-14 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/hackerdojo/events/304211045/)
* 2024-11-15 | Mexico City, DF, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Multi threading y Async en Rust parte 2 - Smart Pointes y Closures**](https://www.meetup.com/rust-mx/events/304150412/)
* 2024-11-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, Nov 15**](https://www.meetup.com/bostonrust/events/303708398/)
* 2024-11-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638252/)
* 2024-11-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, Nov 23**](https://www.meetup.com/bostonrust/events/303708407/)
* 2024-11-25 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcpbhc/)
* 2024-11-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)

### Oceania
* 2024-11-12 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/304029765/)

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
