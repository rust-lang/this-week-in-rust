Title: This Week in Rust 304
Number: 304
Date: 2019-09-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* [Upcoming docs.rs changes](https://blog.rust-lang.org/2019/09/18/upcoming-docsrs-changes.html).
* [GitHub Actions for Rust](https://svartalf.info/posts/2019-09-16-github-actions-for-rust/).
* [Rustconf 2019 videos](https://www.youtube.com/playlist?list=PL85XCvVPmGQhDOUIZBe6u388GydeACbTt).
* [Improved C variadics in Rust and C2Rust](https://immunant.com/blog/2019/09/variadics/).
* [Oreboot: Coreboot minus C, a talk by Google](https://osfc.io/uploads/talk/paper/23/Oreboot.pdf).
* [Announcing awesome-rust-mentors](https://rustbeginners.github.io/awesome-rust-mentors/).
* [Adventures in motion control](http://adventures.michaelfbryan.com/posts/announcing-adventures-in-motion-control/).

# Crate of the Week

This week's crate is [texture-synthesis](https://github.com/EmbarkStudios/texture-synthesis), a program to generate textures by choosing examples.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/621) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rustup needs your help (testing a new feature)](https://www.reddit.com/r/rust/comments/d5hbdu/rustup_needs_your_help_testing_a_new_feature/).
* [Rustup needs your help (testing a new feature, part two)](https://www.reddit.com/r/rust/comments/d5kxr6/rustup_needs_your_help_testing_a_new_feature_part/).
* [Notify: Looking for maintainers](https://github.com/notify-rs/notify/issues/209). Notify is a cross-platform filesystem notification library for Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

282 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-09-09..2019-09-16

* [rustc: Allow the cdylib crate type with wasm32-wasi](https://github.com/rust-lang/rust/pull/64188)
* [Hide diagnostics emitted during --cfg parsing](https://github.com/rust-lang/rust/pull/64467)
* [Improve hygiene of `alloc::format!`](https://github.com/rust-lang/rust/pull/64060)
* [Cleanup handling of hygiene for built-in macros](https://github.com/rust-lang/rust/pull/64469)
* [resolve: Tweak some "cannot find" wording for macros](https://github.com/rust-lang/rust/pull/64483)
* [Provide a span if main function is not present in crate](https://github.com/rust-lang/rust/pull/64290)
* [def_collector: Do not ICE on attributes on unnamed fields](https://github.com/rust-lang/rust/pull/64457)
* [Permit impls referencing errors to overlap](https://github.com/rust-lang/rust/pull/64474)
* [lowering: Extend temporary lifetimes around await](https://github.com/rust-lang/rust/pull/64292)
* [Shrink `ObligationCauseCode`](https://github.com/rust-lang/rust/pull/64302)
* [check_match: Refactor + improve non-exhaustive diagnostics for default binding modes](https://github.com/rust-lang/rust/pull/64271)
* [fn ptr is structural match](https://github.com/rust-lang/rust/pull/64431)
* [rustc_mir: Buffer -Zdump-mir output instead of pestering the kernel constantly](https://github.com/rust-lang/rust/pull/64344)
* [std: Add a `backtrace` module](https://github.com/rust-lang/rust/pull/64154)
* [Stabilize `Vec::new` and `String::new` as `const fn`s](https://github.com/rust-lang/rust/pull/64028)
* [Stabilise weak_ptr_eq](https://github.com/rust-lang/rust/pull/61797)
* [Make `abs`, `wrapping_abs`, `overflowing_abs` const functions](https://github.com/rust-lang/rust/pull/63786)
* [Use `try_fold` instead of manually carrying an accumulator](https://github.com/rust-lang/rust/pull/64473)
* [Improve `BTreeSet::Intersection::size_hint`](https://github.com/rust-lang/rust/pull/64383)
* [cargo: Don't build libstd as a `dylib`](https://github.com/rust-lang/cargo/pull/7353)
* [rustup: Update to most recent viable nightly](https://github.com/rust-lang/rustup.rs/pull/1997)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2582: RFC for an operator to take a raw reference](https://github.com/rust-lang/rfcs/pull/2582).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [`#[cfg(accessible(..) / version(..))]`](https://github.com/rust-lang/rfcs/pull/2523).
* [disposition: postpone] [Cargo versioning](https://github.com/rust-lang/rfcs/pull/2182).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize `str::len`, `[T]::len` and `str::as_bytes` as const fn](https://github.com/rust-lang/rust/pull/63770).
* [disposition: merge] [Stabilize `param_attrs` in Rust 1.39.0](https://github.com/rust-lang/rust/pull/64010).
* [disposition: close] [Expose Linux syscall interface](https://github.com/rust-lang/rust/pull/63745).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online

* [Sep 25. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Oct  2. Johannesburg, ZA - Johannesburg Rust Meetup - futures (part 2)](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryznbdb/).

### Asia Pacific

* [Sep 21. Chennai, IN - Rust Chennai - Monthly meetup](https://www.meetup.com/mad-rs/events/264945694).
* [Sep 25. Hangzhou, CN - Rust in Blockchain Hangzhou - In Rust We Trust](https://www.meetup.com/Rust-in-Blockchain-Hangzhou/events/264778357/).
* [Sep 28. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/2110177005945081/).

### Europe

* [Sep 23. Oslo, NO - Rust Oslo - Hack & Learn](https://www.meetup.com/Rust-Oslo/events/264778400/).
* [Sep 24. London, GB - Rust London User Group - LDN Talks September 2019](https://www.meetup.com/Rust-London-User-Group/events/264890481/).
* [Sep 25. Milano, IT - Rust Language Milano - Virtual Filesystem with Rust](https://www.meetup.com/rust-language-milano/events/264311325).
* [Sep 25. Copenhagen, DK - Copenhagen Rust Hack Night #18](https://cph.rs/).
* [Sep 26. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/264748662).
* [Oct  1. Göteborg, SE - Rust Gbg — Golden October Rust 2019](https://www.meetup.com/rustgbg/events/264957575/).
* [Oct  2. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbdb/).
* [Oct  3. Toulouse, FR - Rust talks at DevFest Toulouse 2019](https://devfesttoulouse.fr/).
* [Oct  4. Toulouse, FR - Toulouse Rust Meetup - Future<Output = Rust>](https://www.meetup.com/Toulouse-Rust-Meetup/events/264780064).

### North America

* [Sep 20-21. Denver, CO, US - Colorado Gold Rust](https://www.cogoldrust.com/).
* [Sep 23. Durham, NC, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzmbfc/).
* [Sep 24. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzmbgc/).
* [Sep 25. Chicago, IL, US - Chicago Rust Meetup - Wait, why does Rust have 4 string types](https://www.meetup.com/Chicago-Rust-Meetup/events/264559606).
* [Sep 25. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryzmbhc/).
* [Sep 25. Mesa, AZ, US - Desert Rust - Rust: Web assembly](https://www.meetup.com/Desert-Rustaceans/events/wmmphryzmbhc/).
* [Sep 26. New York, NY - Local Native: A Decentralized Cross-platform App Developed with Rust](https://www.meetup.com/Rust-NYC/events/264849068/).
* [Oct  1. Toronto, ON, CA - Rust Toronto - Rust for the Web](https://www.meetup.com/Rust-Toronto/events/264727074/).
* [Oct  2. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyznbdb/).
* [Oct  2. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbdb/).
* [Oct  2. Boston, MA, US - Boston Rust Meetup at VMware](https://www.meetup.com/BostonRust/events/264555065/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [2D Graphics Programmer at Dungeonfog, Vienna, AT (Remote available)](https://www.dungeonfog.com/about/job-offers/).
* [Principal Software Engineer at Microsoft, Redmond, WA, US](https://twitter.com/ryan_levick/status/1171830191804551168).
* [Multiple Rust positions at Parity, Berlin, DE (Remote available)](https://www.parity.io/jobs/).
* [Software Engineer at 3DSignals, Kfar Saba, IL](https://3dsig.com/positions/software-engineer/).
* [Senior Engineer at Ditto at San Francisco, US (Remote available)](https://twitter.com/Adam_Fish/status/1173672751271268352).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Well, let me tell you: unless your code is cooler than ICE, the compiler does not miss anything. Rust accompanies us at each step of our path, very gently pulling our hand when we are too close to falling onto a (safety) hole, and also very gently letting us fall all the way down the hole, as soon we spell the forbidden incantation: `unsafe`.

– [Daniel H-M on rust-users](https://users.rust-lang.org/t/looking-for-a-deeper-understanding-of-phatomdata/32477/4)

Thanks to [Cerberuser](https://users.rust-lang.org/t/twir-quote-of-the-week/328/700) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/d6920w/this_week_in_rust_304/).</small>
