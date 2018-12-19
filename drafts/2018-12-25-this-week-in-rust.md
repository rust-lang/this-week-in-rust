Title: This Week in Rust 266
Number: 266
Date: 2018-12-25
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

* [How to Become a Rust Super-developer](https://hashnode.com/post/how-to-become-a-rust-super-developer-cjpv1ee7e000buhs2aqrdw2ym)

### #Rust2019

Find all #Rust2019 posts at [Read Rust](https://readrust.net/rust-2019/).

# Crate of the Week

This week's crate is [yaserde](https://github.com/media-io/yaserde), a specialized XML (de)serialization crate compatible with serde. Thanks to [Marc Antoine Arnaud](https://users.rust-lang.org/t/crate-of-the-week/2704/472) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [A call for Rust 2019 Roadmap blog posts](https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html).
* [Rust Latam CFP is now open, deadline is December 31st](https://cfp.rustlatam.org/events/rust-latam).
* [Tarpaulin: OSX support tracking issue](https://github.com/xd009642/tarpaulin/issues/152). Tarpaulin is a code coverage tool for Rust projects.
* [The imag project calls for contributors (2)](https://imag-pim.org/blog/2018/12/04/call-for-participation-2/).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

247 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-12-10..2018-12-17

* [x86: add the `adx` target feature to whitelist](https://github.com/rust-lang/rust/pull/56749)
* [bump minimum required LLVM version to 6.0](https://github.com/rust-lang/rust/pull/56642)
* [unconditionally emit the target-cpu LLVM attribute](https://github.com/rust-lang/rust/pull/56609)
* [account for `impl Trait` when suggesting lifetime](https://github.com/rust-lang/rust/pull/56755)
* [fixed issue with using `Self` ctor in typedefs](https://github.com/rust-lang/rust/pull/56850)
* [clearer error message for dead assign](https://github.com/rust-lang/rust/pull/56439)
* [emit error with span for empty asserts](https://github.com/rust-lang/rust/pull/56491)
* [fix span for invalid number of parameters in trait method](https://github.com/rust-lang/rust/pull/56641)
* [contexually dependent error message for E0424 when value is assigned to "self"](https://github.com/rust-lang/rust/pull/56572)
* [don't depend on `Allocation` sizes for pattern length](https://github.com/rust-lang/rust/pull/56540)
* [some cleanups around `AllocId` management](https://github.com/rust-lang/rust/pull/56461)
* [improve MIR match generation for ranges](https://github.com/rust-lang/rust/pull/56810)
* [rustc: add an unstable `simd_select_bitmask` intrinsic](https://github.com/rust-lang/rust/pull/56789)
* [allow ptr::hash to accept fat pointers](https://github.com/rust-lang/rust/pull/56751)
* [specialize: remove Boxes used by Children::insert](https://github.com/rust-lang/rust/pull/56744)
* [infer: remove Box from a returned Iterator](infer: remove Box from a returned Iterator)
* [`TokenStream` improvements](https://github.com/rust-lang/rust/pull/56737)
* [remove `tokenstream::Delimited`](https://github.com/rust-lang/rust/pull/56369)
* [overhaul `FileSearch` and `SearchPaths`](https://github.com/rust-lang/rust/pull/56090)
* [`SortedMap` upgrades](https://github.com/rust-lang/rust/pull/56039)
* [make `const unsafe fn` bodies `unsafe`](https://github.com/rust-lang/rust/pull/56706)
* [self-profiler: add column for percent of total time](https://github.com/rust-lang/rust/pull/56702)
* [`#[must_use]` on traits in stdlib](https://github.com/rust-lang/rust/pull/56677)
* [fix `BTreeMap` UB](https://github.com/rust-lang/rust/pull/56648)
* [std: activate compiler_builtins `mem` feature for no_std targets](https://github.com/rust-lang/rust/pull/56825)
* [add `checked_add` method to `Instant` time type](https://github.com/rust-lang/rust/pull/56490)
* [`VecDeque`: fix for stacked borrows](https://github.com/rust-lang/rust/pull/56161)
* [std: depend directly on crates.io crates](https://github.com/rust-lang/rust/pull/56092)
* [libtest: use deterministic HashMap, avoid spawning thread if there is no concurrency](https://github.com/rust-lang/rust/pull/56243)
* [greatly improve rustdoc rendering speed issues](https://github.com/rust-lang/rust/pull/56005)
* [rustdoc: fix local reexports of proc macros](https://github.com/rust-lang/rust/pull/56637)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Short-circuit Rc/Arc equality checking on equal pointers where T: Eq](https://github.com/rust-lang/rust/pull/56550).
* [disposition: merge] [Tracking issue for unsafe operations in const fn](https://github.com/rust-lang/rust/issues/55607).
* [disposition: merge] [Tracking issue for RFC 2539, "#[cfg_attr] expanding to multiple attributes"](https://github.com/rust-lang/rust/issues/54881).
* [disposition: merge] [`#[repr(packed(N))]` (tracking issue for RFC 1399)](https://github.com/rust-lang/rust/issues/33158).

## New RFCs

* [Add file-open-with RFC](https://github.com/rust-lang/rfcs/pull/2615).
* [eCrate name transfer](https://github.com/rust-lang/rfcs/pull/2614).

# Upcoming Events

### Online

* [Jan  2. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Europe

* [Dec 20. Cambridge, GB - The Last Cambridge Rust](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxqbbc/)?
* [Dec 20. Turin, IT - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/sbtclqyxqbkc/).
* [Dec 23. St. Petersburg, RU - St. Petersburg Rust Meetup](https://www.meetup.com/spbrust/events/gzjnmqyxqbfc).
* [Jan 10. Brno, CZ, Rust meetup at Masaryk University](https://rust-brno.github.io/)

### North America

* [Dec 20. Chicago, US - Rust for the Holidays](https://www.meetup.com/Chicago-Rust-Meetup/events/256778181).
* [Dec 23. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbfc/).
* [Dec 25. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxqbhc/).
* [Dec 26. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyxqbjc/).
* [Dec 26. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyxqbjc/).
* [Dec 30. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxqbnc/).
* [Jan  2. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/246726699/).
* [Jan  2. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyzcbdb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Sr. Software Engineer - Rust at Mersive, Denver, US](https://www.mersive.com/company/join-mersive-team/?gh_jid=4136286002)
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> `impl Drop for Mic {}`

– Nick Fitzgerald [rapping about Rust](http://fitzgeraldnick.com/2018/12/13/rust-raps.html)

Thanks to [mark-i-m](https://users.rust-lang.org/t/twir-quote-of-the-week/328/588) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
