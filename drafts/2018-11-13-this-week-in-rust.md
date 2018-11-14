Title: This Week in Rust 260
Number: 260
Date: 2018-11-13
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

# Crate of the Week

This week's crate is [parse_wiki_text](https://crates.io/crates/parse_wiki_text), a crate to parse MediaWiki entries into a tree of elements. Thanks to [Fredrik](https://users.rust-lang.org/t/crate-of-the-week/2704/467) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [swc transcompiles new-generation JavaScript to old-generation JavaScript and needs help](https://users.rust-lang.org/t/twir-call-for-participation/4821/214).
* [Tera: Consider remove error_chain dependency](https://github.com/Keats/tera/issues/297).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

131 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-10-29..2018-11-05

* [fix DWARF generation for enums](https://github.com/rust-lang/rust/pull/54004)
* [add libproc_macro to rust-src distribution](https://github.com/rust-lang/rust/pull/55280)
* [remove the `alloc_jemalloc` crate](https://github.com/rust-lang/rust/pull/55238)
* [add Retagging statements](https://github.com/rust-lang/rust/pull/55316)
* [implement object-safety and dynamic dispatch for arbitrary_self_types](https://github.com/rust-lang/rust/pull/54383)
* [universes refactor 3](https://github.com/rust-lang/rust/pull/55305)
* [correct alignment of atomic types and (re)add `Atomic`{`I`,`U`}`128`](https://github.com/rust-lang/rust/pull/55410)
* [rustc_target: pass contexts by reference, not value](https://github.com/rust-lang/rust/pull/55665)
* [take advantage of impl Iterator in (transitive/elaborate)_bounds](https://github.com/rust-lang/rust/pull/55473)
* [change a `flat_map` with 0/1-element vecs to a `filter_map`](https://github.com/rust-lang/rust/pull/55476)
* [improve a few cases of collecting to an `FxHash`(`Map`/`Set`)](https://github.com/rust-lang/rust/pull/55205)
* [crates.io: ensure only exact name matches are added to the index](https://github.com/rust-lang/crates.io/pull/1550)
* [use `SmallVec` within `MoveData`](https://github.com/rust-lang/rust/pull/55574)
* [tweak `MatcherPos::matches`](https://github.com/rust-lang/rust/pull/55558)
* [make `-Z ls` list the actual filename of external dependencies](https://github.com/rust-lang/rust/pull/55555)
* [syntax: improve a few allocations](https://github.com/rust-lang/rust/pull/55542)
* [pass suggestions as impl Iterator instead of Vec](https://github.com/rust-lang/rust/pull/55536)
* [fix `invalid_const_promotion` test on some archs](https://github.com/rust-lang/rust/pull/55575)
* [add `raw_entry` API to `HashMap`](https://github.com/rust-lang/rust/pull/54043)
* [cargo: configure tar to not set mtime](https://github.com/rust-lang/cargo/pull/6257)
* [rustdoc: hide default impls items](https://github.com/rust-lang/rust/pull/54162)
* [rustdoc: refactor: centralize all command-line argument parsing](https://github.com/rust-lang/rust/pull/55515)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2561: Meta-RFC: Future possibilities](https://github.com/rust-lang/rfcs/pull/2561).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Linked list cursors](https://github.com/rust-lang/rfcs/pull/2570).
* [disposition: close] [Create Editorconfig File as Part of Cargo Project](https://github.com/rust-lang/rfcs/pull/2549).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [add FromIterator<A> to Box<[A]>](https://github.com/rust-lang/rust/pull/55843).
* [disposition: merge] [Tracking issue for `literal` fragment specifier (RFC 1576)](https://github.com/rust-lang/rust/issues/35625).
* [disposition: close] [Tracking issue for FnBox()](https://github.com/rust-lang/rust/issues/28796).

## New RFCs

* [Custom DSTs](https://github.com/rust-lang/rfcs/pull/2594).
* [Enum variant types](https://github.com/rust-lang/rfcs/pull/2593).
* [Stabilize `std::task` and `std::future::Future`](https://github.com/rust-lang/rfcs/pull/2592).
* [Stabilise exhaustive integer pattern matching](https://github.com/rust-lang/rfcs/pull/2591).

# Upcoming Events

### Online

* [Nov 19. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov 21. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Nov 28. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Asia

* [Nov 17. Chennai, IN - Monthly Meetup - November](https://www.meetup.com/mad-rs/events/256339435/).

### Europe

* [Nov 15. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxpbtb/).
* [Nov 15. Madrid, ES - Madrid Rust Meetup: Rust in Deliveroo](https://www.meetup.com/MadRust/events/256141489).
* [Nov 17. Toulouse, FR - Capitole du Libre 2018 - Introduction à Rust](https://2018.capitoledulibre.org/programme/#introduction-a-rust-2).
* [Nov 18. Toulouse, FR - Capitole du Libre 2018 - Initiation à Rust](https://2018.capitoledulibre.org/programme/#initiation-a-rust).
* [Nov 20. Paris, FR - Rust Paris](http://www.meetup.com/Rust-Paris).
* [Nov 21. Oslo, NO - Hack & Learn](https://www.meetup.com/Rust-Oslo/events/255966088/).
* [Nov 21. Hamburg, DE - Rust Hack & Learn Nov 2018](https://www.meetup.com/Rust-Meetup-Hamburg/events/254969484/).
* [Nov 24. St. Petersburg, RU - Rust Meetup](https://www.meetup.com/spbrust/events/bqctlqyxpbgc).
* **[Nov 24 & 25. Rome, IT - RustFest Rome 2018](https://rome.rustfest.eu).**
* [Nov 27. Sofia, BG - Rust Bulgaria @ Global Tech Summit](https://www.meetup.com/rust-bulgaria/events/256338832/).
* [Nov 28. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxpblc/).
* [Nov 29. Copenhagen, DK - Copenhagen Rust Group - Hack Night #11](http://cph.rs/).

### North America

* [Nov 18. Mountain View, US - Rust Dev in Mountain View](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbxb/).
* [Nov 25. Mountain View, US - Rust Dev in Mountain View](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbhc/).
* [Nov 26. Durham, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxpbjc/).
* [Nov 27. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxpbkc/).
* [Nov 28. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/xttphqyxpblc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Backend Engineer at Kraken, Remote](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Senior Blockchain Protocol Engineer at crypto.com, Hong Kong](https://cryptocom.bamboohr.com/jobs/view.php?id=61).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Everything about Rust is ironic.

– @jessitron [on twitter](https://mobile.twitter.com/jessitron/status/1057080556863799298)

Thanks to [David Sullins](https://users.rust-lang.org/t/twir-quote-of-the-week/328/578) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
