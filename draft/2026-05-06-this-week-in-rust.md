Title: This Week in Rust 650
Number: 650
Date: 2026-05-06
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

* [Rust Trends Issue 77 - Rust Sharpens the Craft](https://rust-trends.com/newsletter/rust-sharpens-the-craft/)

### Project/Tooling Updates
* [Imgclip: A Cross-Platform CLI for Clipboard ↔ Image File Conversion](https://dev.to/alex_yan_6135f8195a1a3b01/imgclip-a-cross-platform-cli-for-clipboard-image-file-conversion-2i1l)

* [Connectors: Where AimDB Meets the Real World](https://aimdb.dev/blog/connectors-where-aimdb-meets-the-real-world)

* [rkik-nts 1.0.0: a high-level Rust Network Time Security (RFC 8915) client library](https://github.com/aguacero7/rkik-nts/releases/tag/v1.0.0)

* [unix-ancillary 0.2.2 — safe SCM_RIGHTS fd-passing for Rust](https://github.com/MohibShaikh/unix-ancillary/releases/tag/v0.2.2)

* [kache 0.2.0: zero-copy, content-addressed Rust build cache (RUSTC_WRAPPER)](https://github.com/kunobi-ninja/kache/releases/tag/v0.2.0)

### Observations/Thoughts

* [The `Sync` bound nobody asked for](https://verrchu.github.io/blog/1-the-sync-bound-nobody-asked-for/)
* [Cross-platform Rust: Analyzing how WhatsApp, Signal and more are shipping Rust to billions of devices](https://kerkour.com/rust-cross-platform-apps)
* [audio] [Netstack.FM episode 37 — dial9: from black box to insight in Tokio](https://netstack.fm/#episode-37)
* [Your Clippy Config Should Be Stricter](https://emschwartz.me/your-clippy-config-should-be-stricter/)

### Rust Walkthroughs

* [video] [RustCurious lesson 7: Arrays and Slices](https://www.youtube.com/watch?v=JWfVqDEkQQw)
* [Writing Middlewares for Rust Lambda Functions](https://loige.co/writing-middlewares-for-rust-lambda-functions/)
* [Learn Error Handling in Rust By Building a TOML Config Parser](https://blog.sheerluck.dev/posts/learn-error-hanlding-in-rust/)

### Research

### Miscellaneous
* [Awesome SQLx Resources](https://github.com/szabgab/awesome-sqlx)

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

Rusty Events between 2026-05-06 - 2026-06-03 🦀

### Virtual
* 2026-04-29 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/8hi2xywi)
* 2026-05-01 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Hacker's Hike 0x1**](https://www.meetup.com/rust-noris/events/312788983/)
* 2026-05-02 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763928837)
* 2026-05-03 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/314036479/)
* 2026-05-05 | Virtual (Tel Aviv-yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens)
    * [**Rust code reading and open source contribution**](https://www.meetup.com/code-mavens/events/314538967/)
* 2026-05-06 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Practical introduction to SIMD**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/314301861/)
* 2026-05-06 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/rd05z3vo)
* 2026-05-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/314323890/)
* 2026-05-07 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455928/)
* 2026-05-07 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313345240/)
* 2026-05-09 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Learning Rust the Hard Way: Building a TUI Chess Game**](https://luma.com/u436v3d7)
* 2026-05-12 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254782/)
* 2026-05-12 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/313506068/)
* 2026-05-17 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/314329043/)
* 2026-05-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjchbzb/)
* 2026-05-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Mouse Control with Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/548kbqhl)
* 2026-05-21 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455929/)
* 2026-05-21 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Tock OS Part #4 - Capsule coding in QEMU!**](https://www.meetup.com/charlottesville-rust-meetup/events/314477948/)
* 2026-05-26 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254781/)
* 2026-05-26 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Seeing Into Your Code - A Practical Guide to Tracing in Rust**](https://www.meetup.com/women-in-rust/events/313506048/)
* 2026-05-27 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Weekly coding session**](https://luma.com/9v7hv2g1)

### Asia
* 2026-05-13 | Malaysia, MY | [Rust Meetup Malaysia](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
    * [**Rust Meetup May 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfMh6PA05ujl3lS59tJU3DcLHGVZ1zjzJhl49hXEHU7e6vsQA/viewform)
* 2026-05-16 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2026 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2026-rustacean-meetup/)

### Europe
* 2026-04-29 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #67**](https://www.meetup.com/copenhagen-rust-community/events/314279730/)
* 2026-04-29 | Paris, FR | [Paris Rustaceans](https://www.eventbrite.fr/o/74289178383)
    * [**Rust Meetup in Paris**](https://www.eventbrite.fr/e/rust-meetup-in-paris-tickets-1984135342220)
* 2026-04-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Berlin Talks: The next generation**](https://www.meetup.com/rust-berlin/events/314292918/)
* 2026-04-30 | Manchester, GB | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester April Talk**](https://www.meetup.com/rust-manchester/events/314229892/)
* 2026-05-02 | Augsburg, DE | [Rust Munich](https://rust-munich.de/) and [Rust Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Augsburger Linux-Infotag 2026: Gemeinschaftsstand Rust Augsburg und Rust München**](https://www.luga.de/static/LIT-2026/)
* 2026-05-04 | Amsterdam, NH, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/314268909/)
* 2026-05-04 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Writing a stock portfolio simulation in Rust with Leptos**](https://www.meetup.com/rust-rhein-main/events/314051688/)
* 2026-05-05 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia)
    * [**5. Rust Moravia Meetup (Ukaž testy!)**](https://www.meetup.com/rust-moravia/events/314218493/)
* 2026-05-06 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Rust Milan @ Python Milano: Python or Rust? Yes!**](https://www.meetup.com/rust-language-milan/events/314521855/)
* 2026-05-06 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Building LLMs from scratch**](https://www.meetup.com/oxford-rust-meetup-group/events/314456933/)
* 2026-05-07 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust May Talks: Aetherus + Bevy**](https://www.meetup.com/rust-and-friends/events/314300802/)
* 2026-05-13 | Girona, ES | [Rust Girona](https://luma.com/rust-girona)
    * [**Rust Girona Hack & Learn 05 2026**](https://luma.com/ooub1kt0)
* 2026-05-14 | Switzerland, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/events/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-05-18 - 2026-05-23 | Amsterdam, NL | [RustWeek 2026](https://2026.rustweek.org/)
    * [**RustWeek 2026**](https://2026.rustweek.org/)
* 2026-05-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/314129975/)
* 2026-05-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Cross-Building & Cross-Testing**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313813902/)
* 2026-05-19 | London, UK | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**RustWeek lunch meetup**](https://www.meetup.com/women-in-rust/events/314313054/)
* 2026-05-21 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**RustWeek Hackathon**](https://www.meetup.com/rust-nederland/events/314301699/)
* 2026-05-22 | Amsterdam, NL | [RustNL](https://www.meetup.com/rust-amsterdam)
    * [**Bike tour around Utrecht**](https://www.meetup.com/rust-nederland/events/314523659/)
* 2026-05-26 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund Meetup - Agentic Programming - May**](https://www.meetup.com/rust-dortmund/events/314522781/)
* 2026-05-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/314452972/)

### North America
* 2026-04-30 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228662/)
* 2026-04-30 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/314225247/)
* 2026-05-02 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, May 2**](https://www.meetup.com/bostonrust/events/314480527/)
* 2026-05-07 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Open Project Night**](https://www.meetup.com/stl-rust/events/313807225/)
* 2026-05-09 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, May 9**](https://www.meetup.com/bostonrust/events/314480529/)
* 2026-05-14 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust)
    * [**From Radio Waves to Pixels - Real-Time Visualizations with Rust and WebAssembly**](https://www.meetup.com/pdxrust/events/314256732/)
* 2026-05-14 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust May Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313721886/)
* 2026-05-16 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Lechmere Rust Lunch, May 16**](https://www.meetup.com/bostonrust/events/314480531/)
* 2026-05-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/314154841/)
* 2026-05-20 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Mouse Control with Rust**](https://www.meetup.com/vancouver-rust/events/313572925/)
* 2026-05-20 | San Francisco, CA, US | [Bay Area Rust Meetup](https://luma.com/bayarearust)
    * [**Bay Area Rust Meetup**](https://luma.com/9j3q5ejl)
* 2026-05-21 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**May, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/313873203/)
* 2026-05-21 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Community Meetup**](https://www.meetup.com/music-city-rust-developers/events/314359076/)
* 2026-05-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Allston Rust Lunch, May 23**](https://www.meetup.com/bostonrust/events/314480534/)
* 2026-05-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/314209662/)

### Oceania
* 2026-05-14 | Melbourne, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**Rust Melbourne - May 2026**](https://www.meetup.com/rust-melbourne/events/314260890/)
* 2026-05-26 | Barton, AC, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**May Meetup**](https://www.meetup.com/rust-canberra/events/314050576/)

### South America
* 2026-05-13 | Montevideo, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Rust Uruguay meetup de Mayo**](https://www.meetup.com/rust-uruguay/events/314532884/)

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
