Title: This Week in Rust 282
Number: 282
Date: 2019-04-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

**Important Update**: The *This Week in Rust* privacy policy has changed due to our migration to GitHub pages for hosting. The current policy can be accessed [here](https://this-week-in-rust.org/pages/privacy-policy.html). The git-diff can be [viewed here](https://github.com/cmr/this-week-in-rust/pull/885/files).

# Updates from Rust Community

## News & Blog Posts
* [Web Development with Rust — 03/x: Create a REST API](https://dev.to/gruberb/web-development-with-rust-03-x-create-a-rest-api-3i82)
* [Building a pub/sub server with Sonr](https://hagsteel.com/posts/building-a-pub-sub-with-sonr-part-1/)

The [CFP for Colorado Gold Rust](https://cfp.cogoldrust.com/events/cogoldrust-2019) is open now!
Submit a talk today! The organizers are also looking for volunteers to help people draft talk proposals.
If you can help out [send them an email](mailto:coloradogoldrust@gmail.com) or DM them on Twitter at [@COGoldRust](https://twitter.com/cogoldrust)!

* [A Love Letter to Rust Macros](https://happens.lol/posts/a-love-letter-to-rust-macros/).

# Crate of the Week

This week's crate is [sendfd](https://github.com/Standard-Cognition/sendfd), a simple way to send file descriptors over UNIX sockets. Thanks to [Léo Gaspard](https://users.rust-lang.org/t/crate-of-the-week/2704/514) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Releasing nom 5.0](https://www.reddit.com/r/rust/comments/ba366j/call_for_help_releasing_nom_50/).
* [Veloren, the open-source voxel MMORPG, is looking for contributors](https://veloren.net/).
* [pulldown-cmark: Create a framework for detecting quadratic time regressions](https://github.com/raphlinus/pulldown-cmark/issues/257).
* [compact_arena: Make the crate `no_std`](https://github.com/llogiq/compact_arena/issues/1).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

241 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-04-08..2019-04-15

* [save-analysis: Pull associated type definition using `qpath_def`](https://github.com/rust-lang/rust/pull/59894)
* [Recover from missing semicolon based on the found token](https://github.com/rust-lang/rust/pull/59866)
* [Increase `Span` from 4 bytes to 8 bytes](https://github.com/rust-lang/rust/pull/59693)
* [proc_macro: Stop using LEB128 for RPC](https://github.com/rust-lang/rust/pull/59820)
* [Remove duplicated redundant spans](https://github.com/rust-lang/rust/pull/59896)
* [Mark variables captured by reference as mutable correctly](https://github.com/rust-lang/rust/pull/59708)
* [Suggest removing `?` to resolve type errors](https://github.com/rust-lang/rust/pull/59859)
* [Make duplicate matcher bindings a hard error](https://github.com/rust-lang/rust/pull/59858)
* [Improvement for comparision against fn](https://github.com/rust-lang/rust/pull/59798)
* [Suggest importing macros from the crate root](https://github.com/rust-lang/rust/pull/59784)
* [Function arguments should never get promoted](https://github.com/rust-lang/rust/pull/59724)
* [miri: Implement non-deterministc mode](https://github.com/rust-lang/miri/pull/683)
* [miri: Unsized locals and by-value dyn traits](https://github.com/rust-lang/rust/pull/59780)
* [std: Add {`read`, `write`}`_vectored` for more types](https://github.com/rust-lang/rust/pull/59852)
* [`MaybeUninit`: Remove deprecated functions](https://github.com/rust-lang/rust/pull/59912)
* [Stabilize the `alloc` crate](https://github.com/rust-lang/rust/pull/59675)
* [Improve warning in `cargo new` with parse error](https://github.com/rust-lang/cargo/pull/6839)
* [rustup.rs: Less copying during dist installation](https://github.com/rust-lang/rustup.rs/pull/1744)
* [rustup.rs: Shell completions for Cargo](https://github.com/rust-lang/rustup.rs/pull/1646)
* [Add --path option to 'rustup override set'](https://github.com/rust-lang/rustup.rs/pull/1524)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2480: Stabilize the alloc crate](https://github.com/rust-lang/rfcs/pull/2480).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Permit `-> _` return types for improved diagnostics](https://github.com/rust-lang/rust/issues/56132).

## New RFCs

* [Named custom cargo profiles](https://github.com/rust-lang/rfcs/pull/2678).
* [Add the Close trait](https://github.com/rust-lang/rfcs/pull/2677).

# Upcoming Events

### Asia Pacific

* [Apr 20. Beijing, CN - RustCon Asia](https://rustcon.asia/).
* [Apr 20. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/400895290642737/).
* [Apr 24. Tokyo, JP - Tokyo Rust Meetup](https://rust.connpass.com/event/125666/).

### Europe

* [Apr 11. Oslo, NO - Rust Oslo - Hack & Learn](https://www.meetup.com/Rust-Oslo/events/260244075/).
* [Apr 13. Kyiv, UA - PeerLab Kyiv #NativeDev: Rust 1.34 Release in Depth](https://www.meetup.com/PeerLab-Native-Developers/events/260050471/).
* [Apr 16. Rome, IT - Rust Roma - Rust learning and hacking evening #17](https://www.meetup.com/Rust-Roma/events/260430915/).
* [Apr 17. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzgbwb/).
* [Apr 17. Stuttgart, DE - Rust Workshop - Beginner to Intermediate](https://www.meetup.com/Rust-Community-Stuttgart/events/260337649/).
* [Apr 18. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/ktqcpqyzgbhc/).
* [Apr 25. Toulouse, FR - Mon premier service web en Rust](https://www.meetup.com/Toulouse-Rust-Meetup/events/260218832).
* [Apr 25. Paris, FR - Rust Paris meetup #44](https://www.meetup.com/Rust-Paris/events/260443108/).
* [Apr 25. Brno, CZ - Rust Brno meetup](https://rust-brno.github.io/).
* [Apr 26. Stuttgart, DE - Rust Meetup #2](https://gettogether.community/rust-stuttgart/)
* [Apr 26. Berlin, DE - Oxidize Berlin Conference](https://oxidizeconf.com/).
* [Apr 30. London, UK - Rust London User Group - LDN Talks](https://www.meetup.com/Rust-London-User-Group/events/260565918/).

### North America

* [Apr 11. Orem, US - Utah Rust - Meetup #11: Hack and learn](https://www.meetup.com/utah-rust/events/260015102/).
* [Apr 11. Arlington, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/259782531).
* [Apr 11. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzgbpb/).
* [Apr 11. San Diego, US - San Diego Rust April Meetup](https://www.meetup.com/San-Diego-Rust/events/260346466/).
* [Apr 18. Denver, US - Rust Boulder/Denver - Rust Meetup for April](https://www.meetup.com/Rust-Boulder-Denver/events/259124388/).
* [Apr 17. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/gqbksqyzgbwb/).
* [Apr 22. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzgbdc/).
* [Apr 23. Chicago, US - Chicago Rust Meetup - Discussion: Better Method Chaining in Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/260321118).
* [Apr 24. Sacramento, US - Hands-on Rust](https://www.meetup.com/Rust-Sacramento/events/260347016/).
* [Apr 24. Ann Arbor, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/vsncvqyzgbgc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Software Engineer, Backend - Rust at Kraken, Remote](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105).
* [Rust Systems Software Engineer at Cloudseal, Remote (US)](https://www.cloudseal.io/hiring/rust-systems-software-engineer-sp19).
* [Compilers (LLVM), distributed systems, & theorem proving engineers at Offscale.io, Remote/Sydney, AU](https://www.reddit.com/r/rust/comments/bb33yo/job_compilers_llvm_distributed_systems_theorem/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Confusion is a product type.

– [/u/casual-cryptarch on /r/rust](https://www.reddit.com/r/rust/comments/bbpqbc/how_would_you_articulate_this_difference_in_trait/eklggop/)

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
