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

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [wgsl-bindgen](https://github.com/Swoorup/wgsl-bindgen), a binding generator for WGSL, the WebGPU shading language, to be used with [wgpu](https://github.com/gfx-rs/wgpu).

Thanks to [Artem Borisovskiy](https://users.rust-lang.org/t/crate-of-the-week/2704/1511) for the suggestion!

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

297 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-23..2025-12-30

#### Compiler
* [recursive delegation improvements](https://github.com/rust-lang/rust/pull/150347)
* [miri: fix ICE for particular data race situations](https://github.com/rust-lang/miri/pull/4796)
* [miri: show a warning when combing native-lib mode and many-seeds](https://github.com/rust-lang/miri/pull/4790)
* [miri: tree Borrows: improve protector end access child skipping](https://github.com/rust-lang/miri/pull/4766)

#### Library
* [add `MaybeDangling` to `core`](https://github.com/rust-lang/rust/pull/149775)
* [alloc: specialize `String::extend` for slices of str](https://github.com/rust-lang/rust/pull/149694)
* [implement `Duration::div_duration_{floor,ceil}`](https://github.com/rust-lang/rust/pull/149582)
* [implement flatten for `Option<&Option<T>>` and `Option<&mut Option<T>>`](https://github.com/rust-lang/rust/pull/108671)
* [optimized implementation for `uN::{gather,scatter}_bits`](https://github.com/rust-lang/rust/pull/149663)
* [rewrite `String::replace_range`](https://github.com/rust-lang/rust/pull/149447)
* [stabilize `lazy_get`](https://github.com/rust-lang/rust/pull/150016)

#### Cargo
* [`index`: Stabilize pubtime](https://github.com/rust-lang/cargo/pull/16372)
* [`report`: new command `cargo report sessions`](https://github.com/rust-lang/cargo/pull/16428)
* [`report`: support --manifest-path in `cargo report timings`](https://github.com/rust-lang/cargo/pull/16441)
* [`resolver`: List features when no close match](https://github.com/rust-lang/cargo/pull/16445)
* [`toml`: TOML 1.1 parse support](https://github.com/rust-lang/cargo/pull/16415)
* [`vendor`: recursively filter git files in subdirectories](https://github.com/rust-lang/cargo/pull/16439)
* [`vendor`: unpack from local-registry cache path](https://github.com/rust-lang/cargo/pull/16435)
* [`build-rs`: Reduce from 'build' to 'check' where possible](https://github.com/rust-lang/cargo/pull/16444)
* [experiment: render timing pipeline in SVG](https://github.com/rust-lang/cargo/pull/15091)
* [patch: Display where the patch was defined in patch-related error messages](https://github.com/rust-lang/cargo/pull/16407)

#### Rustdoc
* [if line number setting is disabled, do not make line numbers take space](https://github.com/rust-lang/rust/pull/150396)
* [fix copy code example with line numbers](https://github.com/rust-lang/rust/pull/150395)
* [fix duplicate Re-exports sections](https://github.com/rust-lang/rust/pull/150362)
* [fix incorrect type filter name in help popup](https://github.com/rust-lang/rust/pull/150360)

#### Clippy
* [fix `assertions_on_constants` false positive when there is non-constant value in the condition expr](https://github.com/rust-lang/rust-clippy/pull/16297)
* [fix `double_parens` false positive on macro repetition patterns](https://github.com/rust-lang/rust-clippy/pull/16301)
* [fix `obfuscated_if_else` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/16289)
* [fix `result_large_err` false negative on closures](https://github.com/rust-lang/rust-clippy/pull/16277)
* [preserve explicit lifetime information when removing `mut`](https://github.com/rust-lang/rust-clippy/pull/16273)
* [various fixes for handling of macros](https://github.com/rust-lang/rust-clippy/pull/16296)

#### Rust-Analyzer
* [add bidirectional messaging proc-macro-srv prototype](https://github.com/rust-lang/rust-analyzer/pull/21249)
* [add macro segment completion](https://github.com/rust-lang/rust-analyzer/pull/20741)
* [implement configuration to change sub command for test, bench and doctest](https://github.com/rust-lang/rust-analyzer/pull/21308)
* [provide a setting to disable showing rename conflicts](https://github.com/rust-lang/rust-analyzer/pull/20193)
* [stabilize type mismatch diagnostic 🎉](https://github.com/rust-lang/rust-analyzer/pull/21337)
* [indent for `convert_to_guarded_return`](https://github.com/rust-lang/rust-analyzer/pull/21330)
* [fix LSP configuration request handling](https://github.com/rust-lang/rust-analyzer/pull/21297)
* [fix parsing of `format_args!("...", keyword=...)`](https://github.com/rust-lang/rust-analyzer/pull/21351)
* [fix type inference when hovering on `_`](https://github.com/rust-lang/rust-analyzer/pull/21358)
* [reenable fixpoint variance](https://github.com/rust-lang/rust-analyzer/pull/21348)
* [do not really expand builtin derives, instead treat them specifically](https://github.com/rust-lang/rust-analyzer/pull/21200)
* [pre-allocate some buffers in parsing](https://github.com/rust-lang/rust-analyzer/pull/21353)
* [reduce channel lock contention for drop-threads](https://github.com/rust-lang/rust-analyzer/pull/21355)
* [prompt the user in VSCode to add the rust-anaylzer componenet to the toolchain file](https://github.com/rust-lang/rust-analyzer/pull/21359)

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

> what even is time?!?

– [Ralf Jung on his blog](https://www.ralfj.de/blog/2025/12/22/miri.html)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1743) for the suggestion!

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
