Title: This Week in Rust 657
Number: 657
Date: 2026-06-24
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

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

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [rustc_lint: Allow scoped `non_ascii_idents` lint levels](https://github.com/rust-lang/rust/pull/157497)
* [Stabilize `#[my_macro] mod foo;` (part of `proc_macro_hygiene`)](https://github.com/rust-lang/rust/pull/157857)
* [Implement `IntoIterator` for `[&[mut]] Box<[T; N], A>`](https://github.com/rust-lang/rust/pull/134021)
* [Tracking Issue for `string_from_utf8_lossy_owned`](https://github.com/rust-lang/rust/issues/129436)
* [Infer all anonymous lifetimes in assoc consts as `'static`](https://github.com/rust-lang/rust/pull/156508)
* [consider subtyping when checking if an infer var is sized](https://github.com/rust-lang/rust/pull/157820)
* [remove `box_patterns`](https://github.com/rust-lang/rust/pull/156749)
* [enable eager `param_env` norm in new solver](https://github.com/rust-lang/rust/pull/156976)
* [Lint against iterator functions that panic when `N` is zero](https://github.com/rust-lang/rust/pull/153563)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [Start a t-project-structure/t-comprehensibility](https://github.com/rust-lang/leadership-council/issues/298)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-06-24 - 2026-07-22 🦀

### Virtual
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/ekws5nr4)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455931/)
* 2026-06-21 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329044/)
* 2026-06-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254779/)
* 2026-06-23 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: What the heck are monads - and how do we fake them in Rust**](https://www.meetup.com/women-in-rust/events/313767883/)
* 2026-07-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/315210366/)
* 2026-07-02 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455932/)
* 2026-07-02 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Learning Game Development the Hard Way with Rust and Bevy**](https://www.meetup.com/charlottesville-rust-meetup/events/315211402/)
* 2026-07-02 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345243/)
* 2026-07-05 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314095287/)
* 2026-07-07 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315060981/)
* 2026-07-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254778/)
* 2026-07-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Jiff**](https://www.meetup.com/vancouver-rust/events/314233743/)
    
### Europe
* 2026-06-18 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/314965238/)
* 2026-06-18 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends comes to Glasgow! (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/315093492/)
* 2026-06-18 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends comes to Glasgow! (evening pub)**](https://www.meetup.com/rust-and-friends/events/315093500/)
* 2026-06-18 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**21st BcnRust Meetup**](https://www.meetup.com/bcnrust/events/315094938/)
* 2026-06-19 | Dresden, DE | [Rust Dresden](https://github.com/rust-dresden)
    * [**Second Meetup**](https://pretix.eu/rust-dresden/on-location-2)
* 2026-06-23 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #86**](https://www.meetup.com/rust-paris/events/315040676/)
* 2026-06-23 | Warsaw, PL | [Rust Warsaw](https://luma.com/rust.in.warsaw)
    * [**Rust Warsaw Meetup: June 2026**](https://luma.com/djs7ntfx)
* 2026-06-24 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester June Talks**](https://www.meetup.com/rust-manchester/events/315200163/)
* 2026-06-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314396600/)
* 2026-06-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #69**](https://www.meetup.com/copenhagen-rust-community/events/315214426/)
* 2026-06-25 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse/)
    * [**Rust Toulouse Meetup - Bevy & ESP32**](https://www.meetup.com/rust-community-toulouse/events/314947457/)
* 2026-07-02 | Edinburgh, GB | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Bevy, Bits, & Cats (Rust July Talks)**](https://www.meetup.com/rust-and-friends/events/314941098/)
* 2026-07-02 | Enschede, OV, NL | [Baseflow Tech Meetups](https://www.meetup.com/dutch-rust-meetup/events/)
    * [**AI Summit**](https://www.meetup.com/baseflow-tech-meetups/events/315099547/)
* 2026-07-08 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin/events/)
    * [**Join us live and INPERSON for Rust 261**](https://www.meetup.com/rust-dublin/events/315150327/)
* 2026-07-09 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)


### North America
* 2026-06-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/314000478/)
* 2026-06-18 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/314236370/)
* 2026-06-18 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Community Meetup**](https://www.meetup.com/music-city-rust-developers/events/315213927/)
* 2026-06-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Northeastern Rust Lunch, June 20**](https://www.meetup.com/bostonrust/events/315225854/)
* 2026-06-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/315105633/)
* 2026-06-24 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust-Based Constraint Solvers in 2D Sketching with Zoo Technologies**](https://www.meetup.com/rust-los-angeles/events/314386080/)
* 2026-06-25 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/313539326/)
* 2026-06-26 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC's Big Summer Social**](https://www.meetup.com/rust-nyc/events/315014582/)
* 2026-06-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Somerville Union Square Rust Lunch, June 27**](https://www.meetup.com/bostonrust/events/315225857/)
* 2026-07-02 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Git is easy?**](https://www.meetup.com/stl-rust/events/315103359/)
* 2026-07-04 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston University Rust Lunch, July 4**](https://www.meetup.com/bostonrust/events/315225861/)
* 2026-07-09 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/events/)
    * [**Utah Rust July Meetup**](https://www.meetup.com/utah-rust/events/314696647/)
* 2026-07-11 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, July 11**](https://www.meetup.com/bostonrust/events/315225865/)

### Oceania
* 2026-06-25 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne June 2026**](https://www.meetup.com/rust-melbourne/events/315039461/)

### South America
* 2026-06-18 | Florianópolis, BR | [Rust SC](https://luma.com/rust-sc)
    * [**Rust Floripa**](https://luma.com/acinctdf)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
