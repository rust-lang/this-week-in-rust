Title: This Week in Rust 271
Number: 271
Date: 2019-01-29
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

This week's crate is [rust-analyzer](https://github.com/rust-analyzer/rust-analyzer), an experimental Rust compiler frontend for IDEs. Thanks to [llogiq](https://github.com/llogiq) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for Maintainers - criterion-plot](https://users.rust-lang.org/t/call-for-maintainers-criterion-plot/24413).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

215 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-01-14..2019-01-21

* [rustc: Remove platform intrinsics crate](https://github.com/rust-lang/rust/pull/57416)
* [implement new literal type `Err`](https://github.com/rust-lang/rust/pull/57651)
* [Better lifetime error message](https://github.com/rust-lang/rust/pull/56479)
* [Add span for malformed doc comment](https://github.com/rust-lang/rust/pull/57784)
* [Add "dereference boxed value" suggestion](https://github.com/rust-lang/rust/pull/57783)
* [Suggest correct cast for struct fields with shorthand syntax](https://github.com/rust-lang/rust/pull/57769)
* [Continue parsing after parent type args and suggest using angle brackets](https://github.com/rust-lang/rust/pull/57768)
* [Remove delay_span_bug from qualify_min_const_fn](https://github.com/rust-lang/rust/pull/57736)
* [Use structured suggestion to surround struct literal with parenthesis](https://github.com/rust-lang/rust/pull/57725)
* [Point at cause for expectation in return type type error](https://github.com/rust-lang/rust/pull/57723)
* [Fix suggestions given mulitple bad lifetimes](https://github.com/rust-lang/rust/pull/57720)
* [better error message for bad manifest with `cargo install`](https://github.com/rust-lang/cargo/pull/6560)
* [add applicability to remaining suggestions](https://github.com/rust-lang/rust/pull/57699)
* [use a faster early exit during region expansion](https://github.com/rust-lang/rust/pull/57697)
* [Tweak `expand_node`](https://github.com/rust-lang/rust/pull/57719)
* [simplify `TokenStream` some more](https://github.com/rust-lang/rust/pull/57486)
* [redo `hir::Stmt`](https://github.com/rust-lang/rust/pull/57689)
* [high priority resolutions for associated variants](https://github.com/rust-lang/rust/pull/57501)
* [provide suggestion for invalid boolean cast](https://github.com/rust-lang/rust/pull/57481)
* [two HIR tweaks](https://github.com/rust-lang/rust/pull/57658)
* [librustc_metadata: pass a default value when unwrapping a span](https://github.com/rust-lang/rust/pull/57650)
* [privacy: account for associated existential types](https://github.com/rust-lang/rust/pull/57649)
* [use structured macro and path resolve suggestions](https://github.com/rust-lang/rust/pull/57635)
* [unaccept `extern_in_paths`](https://github.com/rust-lang/rust/pull/57572)
* [querify `entry_fn`](https://github.com/rust-lang/rust/pull/57573)
* [querify local `plugin_registrar_fn` and `proc_macro_decls_static`](https://github.com/rust-lang/rust/pull/57570)
* [modify some parser diagnostics to continue evaluating beyond the parser](https://github.com/rust-lang/rust/pull/57540)
* [Fix poor worst case performance of set intersection](https://github.com/rust-lang/rust/pull/57043)
* [add `core::iter::once_with()`](https://github.com/rust-lang/rust/pull/57579)
* [Add `is_sorted` to `Iterator` and `[T]`](https://github.com/rust-lang/rust/pull/55045)
* [add a `debug_assert` to `Vec::set_len`](https://github.com/rust-lang/rust/pull/57589)
* [enhance `Pin` impl applicability for `PartialEq` and `PartialOrd`](https://github.com/rust-lang/rust/pull/57685)
* [Change bounds on `TryFrom` blanket impl to use `Into` instead of `From`](https://github.com/rust-lang/rust/pull/56796)
* [stabilize `FileExt::read_exact_at`/`write_all_at`](https://github.com/rust-lang/rust/pull/57625)
* [rustdoc: Add option to persist doc test executables](https://github.com/rust-lang/rust/pull/56189)
* [crates.io: Reject publishing of crates that depend on an alternative registry](https://github.com/rust-lang/crates.io/pull/1589)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2235: Implement Debug, Eq, PartialEq, and Hash for libc structs](https://github.com/rust-lang/rfcs/pull/2235).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [stabilize `std::task` and `std::future::Future`](https://github.com/rust-lang/rfcs/pull/2592).
* [disposition: e] [RFC for anonymous variant types, a minimal ad-hoc sum type](https://github.com/rust-lang/rfcs/pull/2587).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Deprecate the unstable Vec::resize_default](https://github.com/rust-lang/rust/pull/57656).
* [disposition: merge] [syntax: Remove warning for unnecessary path disambiguators](https://github.com/rust-lang/rust/pull/57565).
* [disposition: merge] [Automatically open an issue when a tool breaks](https://github.com/rust-lang/rust/pull/56951).
* [disposition: merge] [[WIP] Unsized rvalues: implement boxed closure impls.](https://github.com/rust-lang/rust/pull/55431).
* [disposition: merge] [Tracking issue for Range*::contains](https://github.com/rust-lang/rust/issues/32311).

## New RFCs

* [RFC for a formalized notion on where to enforce reference propertes in MIR](https://github.com/rust-lang/rfcs/pull/2631).
* [New proc-macro-attribute-recursion](https://github.com/rust-lang/rfcs/pull/2628).

# Upcoming Events

### Online

* [Jan 30. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Feb  6. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Feb  6. Sandown, ZA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/qbhxmqyzdbjb/).

### Europe

* [Jan 24. Hamburg, DE - Rust Hack & Learn Hamburg](https://www.meetup.com/Rust-Meetup-Hamburg/events/257153030/).
* [Jan 30. Toulouse, FR - Rust Toulouse](https://www.meetup.com/fr-FR/Toulouse-Rust-Meetup/events/257926837/).
* [Jan 31. Helsinki, FI - Helsinki Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/257863678/).
* [Jan 31. Torino, IT - Turin Rust meetup](https://www.meetup.com/Mozilla-Torino/events/sbtclqyzcbgc/).
* [Feb  3. Bruxelles, BE - Rust Dev Room @ FOSDEM](https://fosdem.org/2019/).
* [Feb  6. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzdbjb/).
* [Apr 26-29. Berlin, DE - OxidizeConf tickets are now available](https://oxidizeconf.com/).

### North America

* [Jan 28. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/).
* [Jan 30. Chicago, US - Chicago Rust Meetup - Property-Based Testing in Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/257469240/).
* [Jan 31. Phoenix, US - Phoenix Rust: Games](https://www.meetup.com/Desert-Rustaceans/events/257976456/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Engineer at slowtec Stuttgart, DE](https://www.reddit.com/r/rust/comments/ahahqj/job_rust_engineer_position_in_stuttgart_de/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Use usize for counting things that are in memory. Otherwise use the right size for whatever you are doing. Don’t use u32 to track the U.S. national debt, but it’s fine for counting the eggs in most recipes.

– David Roundy [on rust-users](https://users.rust-lang.org/t/how-i128-are-stored-in-a-32-bit-os-architecture/24321/6)

Thanks to [Cerberuser](https://users.rust-lang.org/t/twir-quote-of-the-week/328/605) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
