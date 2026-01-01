Title: This Week in Rust 632
Number: 632
Date: 2025-12-31
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
* [reqwest v0.13 - rustls by default](https://seanmonstar.com/blog/reqwest-v013-rustls-default/)
* [rama 0.3.0-alpha.4 is released — modular service framework to move and transform network packets](https://github.com/plabayo/rama/releases/tag/rama-0.3.0-alpha.4)
* [Ratatui 0.30.0 is released! - a Rust library for cooking up terminal user interfaces](https://ratatui.rs/highlights/v030/)

### Observations/Thoughts
* [Four Years of Rust: An Odyssey of Failures, Achievements, and Hard Lessons](https://medium.com/@adefemiadeoye/four-years-of-rust-an-odyssey-of-failures-achievements-and-hard-lessons-0da41298a152)
* [Simple Bidirectional Type Inference](https://ettolrach.com/blog/bidirectional_inference.html)
* [serde's borrowing can be treacherous](https://yossarian.net/til/post/serde-s-borrowing-can-be-treacherous/)
* [Garbage collection in Rust got a little better](https://claytonwramsey.com/blog/dumpster2/)
* [audio] [Netstack.FM episode 20 — Netstack.FM New Year Special, 2025 Wrap-Up](https://netstack.fm/#episode-20)

### Rust Walkthroughs
* [Why is calling my asm function from Rust slower than calling it from C?](https://ohadravid.github.io/posts/2025-12-rav1d-faster-asm/)
* [Rust Errors Without Dependencies](https://vincents.dev/blog/rust-errors-without-dependencies/)
* [video] [Building your first APP using the new Hotaru Web Framework!](https://www.youtube.com/watch?v=8pV-o04GuKk)


### Research

### Miscellaneous
* [audio] [2025 Holiday Special - Rust in Production Podcast](https://corrode.dev/podcast/s05e07-holiday/)
* [Investigating and fixing a nasty clone bug](https://kobzol.github.io/rust/2025/12/30/investigating-and-fixing-a-nasty-clone-bug.html)

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

<!-- If there are new CfT items this week, include:

  [Repo Name](Repo URL)
    * [<Feature name>](<Feature URL>)
        * [Testing steps](<Testing Steps URL>)

  - and make note in the item so the authors know to remove the `call-for-testing` label:
This RFC will appear in the **Call for Testing** section of the next issue (#) of This Week in Rust (TWiR).
You may remove the `call-for-testing` label.  Please feel free to leave the `call-for-testing` label in place if you would like this RFC to appear again in another issue of TWiR.

  - where `Repo Name` and `Repo URL` are one of:
[Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
[Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
[Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

  - and `Testing steps` points directly to the procedures the item wants users to exercise.

  - For all `Repo Names` with no new CfT items this week: use (removing the repos for which new
     CfT items did appear, of course)

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*
-->

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

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

* [Spindalis - Create an AST parser](https://github.com/lignum-vitae/spindalis/issues/27)
* [Spindalis - Add procedural macro for definite integral](https://github.com/lignum-vitae/spindalis/issues/39)
* [Spindalis - Add a function and macro that can expand polynomials](https://github.com/lignum-vitae/spindalis/issues/36)
* [Spindalis - Add display trait to functions in spindalis core](https://github.com/lignum-vitae/spindalis/issues/43)

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


Not a lot of changes this week. Overall result is positive, largely thanks to https://github.com/rust-lang/rust/pull/142881, which makes computing an expensive data structure for JumpThreading MIR optimization lazy.

Triage done by **@panstromek**.
Revision range: [e1212ea7..112a2742](https://perf.rust-lang.org/?start=e1212ea79b38d51954625291c04d2797c4bb8ec5&end=112a274275d77ebc2b892f056a1e2fad141f4f08&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.5%  | [0.1%, 1.7%]   | 11    |
| Regressions ❌ <br /> (secondary)  | 0.2%  | [0.1%, 0.5%]   | 6     |
| Improvements ✅ <br /> (primary)   | -0.5% | [-1.3%, -0.1%] | 74    |
| Improvements ✅ <br /> (secondary) | -0.6% | [-1.8%, -0.2%] | 71    |
| All ❌✅ (primary)                 | -0.4% | [-1.3%, 1.7%]  | 85    |


2 Regressions, 0 Improvements, 3 Mixed; 1 of them in rollups
37 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/19d2f05e6e3c86fe2496deb4d8ed585375602d78/triage/2025/2025-12-29.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Use either
* [Item title](Item URL)
  - or
* *No RFCs were approved this week.*
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
<!-- Either remove the group from the "No Items Entered Final Comment Period this week for" section
     and add the item(s) which entered Final comment period:
##### [Group](Group URL)
* [Item title](Item URL)
  - for `disposition-merge` `final-comment-period` items, or
* [disposition: postpone]
  - for `disposition-postpone` `final-comment-period` items, or
* [disposition: close]
  - for `disposition-close` `final-comment-period` items,
* [disposition: unspecified]
  - when `disposition` is unspecified or ensure the group is a part of the
     "No Items Entered Final Comment Period this week for" section
*No Items entered Final Comment Period this week for
  [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.
-->

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
<!-- Use either
* [Item title](Item URL)
  - for new items, or
* [updated] [Item title](Item URL)
  - for updated items, or
* *No New or Updated RFCs were created this week.*
-->

<!-- Sample commit message
Update CFT, FCP, MCP and RFC sections for TWiR-xxx
-->

## Upcoming Events

Rusty Events between 2025-12-31 - 2026-01-28 🦀

### Virtual
* 2025-12-30 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Live Open Source Rust project contribution**](https://www.meetup.com/code-mavens/events/312554028/)
* 2026-01-03 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763888717)
* 2026-01-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312102790/)
* 2026-01-08 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Meet, swap, and learn!**](https://www.meetup.com/charlottesville-rust-meetup/events/312321120/)
* 2026-01-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312379275/)
* 2026-01-13 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254791/)
* 2026-01-13 | Virtual | [libp2p Events](https://luma.com/libp2p)
    * [**rust-libp2p Open Maintainers Call**](https://luma.com/xov10pef)
* 2026-01-15 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/305646023/)
* 2026-01-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/312489197/)
* 2026-01-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Stack Safety**](https://www.meetup.com/vancouver-rust/events/310619449/)

### Asia
* 2026-01-07 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust January 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)

### Europe
* 2026-01-07 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Meetup @ Instruqt**](https://www.meetup.com/rust-amsterdam-group/events/312497150/)
* 2026-01-07 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)
* 2026-01-08 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-01-14 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/csvcvtyjccbsb/)
* 2026-01-20 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592260/)
* 2026-01-20 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #82**](https://www.meetup.com/rust-paris/events/312364675/)

### North America
* 2025-12-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Lechmere Rust Lunch, Dec 27**](https://www.meetup.com/bostonrust/events/312483556/)
* 2026-01-03 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Allston Rust Lunch, Jan 3**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-01-08 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/jqvvttyjccblb/)
* 2026-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Central Cambridge Rust Lunch, Jan 10**](https://www.meetup.com/bostonrust/events/312483605/)
* 2026-01-15 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**Janurary, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311814876/)
* 2026-01-17 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston Common Rust Lunch, Jan 17**](https://www.meetup.com/bostonrust/events/312483677/)
* 2026-01-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/310403081/)
* 2026-01-21 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312185794/)

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
