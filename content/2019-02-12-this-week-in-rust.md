Title: This Week in Rust 273
Number: 273
Date: 2019-02-12
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

* [Learning Rust in 2019](https://www.ragona.com/posts/learning_rust_2019).
* [A quick look at trait objects in Rust](https://tratt.net/laurie/blog/entries/a_quick_look_at_trait_objects_in_rust.html).
* [Allocations in Rust: An introduction to the memory model](https://speice.io/2019/02/understanding-allocations-in-rust.html).
* [Custom exit status codes with ? in main](https://www.joshmcguigan.com/blog/custom-exit-status-codes-rust/).
* [Rust on STM32: Blinking an LED](https://jonathanklimt.de/electrics/programming/rust-STM32F103-blink/).
* [Generators I: Toward a minimum viable product](https://boats.gitlab.io/blog/post/generators-i/).
* [Aturon retires from the Core Team (but not from Rust)](https://internals.rust-lang.org/t/aturon-retires-from-the-core-team-but-not-from-rust/9392).
* [Rewriting stackcollapse-xdebug in Rust](https://daniellockyer.com/rewriting-stackcollapse-xdebug/).
* [Are you still using 'println' in Rust for debugging](https://blog.knoldus.com/are-you-still-using-println-in-rust-for-debugging/)?
* [UCG+Miri All-Hands 2019 Recap](https://www.ralfj.de/blog/2019/02/12/all-hands-recap.html).

# Crate of the Week

This week's crate is [sysinfo](https://github.com/guillaumeGomez/sysinfo), a system handler to get information and interact with processes. Thanks to [GuillaumeGomez](https://users.rust-lang.org/t/crate-of-the-week/2704/483) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [raft: Convert `Storage::entries`'s `max_size` argument to `Option<u64>`](https://github.com/pingcap/raft-rs/issues/98).
* [TiKV: Convert trait objects to `dyn` syntax for Rust 2018](https://github.com/tikv/tikv/issues/4197).
* [TiKV: Remove all the `extern crate`s for Rust 2018](https://github.com/tikv/tikv/issues/4196).
* [TiKV: Add tcmalloc support to the tikv_alloc crate](https://github.com/tikv/tikv/issues/4191).
* [rand: Standard should be implemented for NonZero types](https://github.com/rust-random/rand/issues/727).
* [Tarpaulin: Test coveralls with other CI services](https://github.com/xd009642/tarpaulin/issues/213).
* [Inferno: Multiple good first issues](https://github.com/jonhoo/inferno/issues).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

236 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-02-04..2019-02-11

* [Initial addition of the Embedded Rust Book](https://github.com/rust-lang/rust/pull/56291).
* [Add const generics to the AST](https://github.com/rust-lang/rust/pull/58191).
* [Error on duplicate matcher bindings](https://github.com/rust-lang/rust/pull/57617).
* [libc: RFC 2235 - Implement PartialEq,Eq,Hash,Debug for all types](https://github.com/rust-lang/libc/pull/1217).
* [Lower constant patterns with ascribed types](https://github.com/rust-lang/rust/pull/58161).
* [Make `intern_lazy_const` actually intern its argument](https://github.com/rust-lang/rust/pull/58207).
* [Avoid committing to autoderef in object method probing](https://github.com/rust-lang/rust/pull/57885).
* [Add #[must_use] to core::task::Poll](https://github.com/rust-lang/rust/pull/58145).
* [Add #[must_use] message to Fn* traits](https://github.com/rust-lang/rust/pull/58262).
* [Avoid some bounds checks in binary_heap::{PeekMut,Hole}](https://github.com/rust-lang/rust/pull/58123).
* [Make -Zdump-mir dump shims](https://github.com/rust-lang/rust/pull/58103).
* [Cargo: Bail when trying to run "test --doc --no-run"](https://github.com/rust-lang/cargo/pull/6628).
* [Improve error message and docs for non-UTF-8 bytes in stdio on Windows](https://github.com/rust-lang/rust/pull/58136).
* [Move privacy checking later in the pipeline and make some passes run in parallel](https://github.com/rust-lang/rust/pull/58010).
* [Overhaul `syntax::fold::Folder`](https://github.com/rust-lang/rust/pull/58061).
* [Factor out error reporting from `smart_resolve_path_fragment` fn](https://github.com/rust-lang/rust/pull/58065).
* [Do not ICE in codegen when using a extern_type static](https://github.com/rust-lang/rust/pull/58192).
* [hir: add more HirId methods](https://github.com/rust-lang/rust/pull/58139).
* [Implement more detailed self profiling](https://github.com/rust-lang/rust/pull/58085).
* [Add a forever unstable opt-out of const qualification checks](https://github.com/rust-lang/rust/pull/56123).
* [Initial implementation of rustfixable unused_imports lint](https://github.com/rust-lang/rust/pull/56645).
* [Add a query type which is always marked as red if it runs](https://github.com/rust-lang/rust/pull/57770).
* [Don't try to clean predicates involving ReErased](https://github.com/rust-lang/rust/pull/57851).
* [Deduplicate mismatched delimiter errors](https://github.com/rust-lang/rust/pull/57944).
* [Add suggestion for duplicated import](https://github.com/rust-lang/rust/pull/57973).
* [Allow #[repr(align(x))] on enums](https://github.com/rust-lang/rust/pull/57998).

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

* [disposition: postpone] [Generic integers](https://github.com/rust-lang/rfcs/pull/2581).
* [disposition: postpone] [Accept semicolons as item-like](https://github.com/rust-lang/rfcs/pull/2479).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize TryFrom and TryInto with a convert::Infallible empty enum](https://github.com/rust-lang/rust/pull/58302).
* [disposition: merge] [Tracking issue for str::as_mut_ptr](https://github.com/rust-lang/rust/issues/58215).
* [disposition: merge] [Stabilize slice_sort_by_cached_key](https://github.com/rust-lang/rust/pull/58074).
* [disposition: merge] [deprecate before_exec in favor of unsafe pre_exec](https://github.com/rust-lang/rust/pull/58059).
* [disposition: merge] [Stabilize linker-plugin based LTO (aka cross-language LTO)](https://github.com/rust-lang/rust/pull/58057).
* [disposition: merge] [Tracking issue for std::iter::successors](https://github.com/rust-lang/rust/issues/58045).
* [disposition: merge] [Tracking issue for Option::copied](https://github.com/rust-lang/rust/issues/57126).
* [disposition: merge] [Tracking issue for std::ptr::hash](https://github.com/rust-lang/rust/issues/56286).
* [disposition: merge] [Rename `MaybeUninit` to `MaybeUninitialized`](https://github.com/rust-lang/rust/pull/56138).
* [disposition: merge] [Tracking issue for std::iter::from_fn](https://github.com/rust-lang/rust/issues/55977).

## New RFCs

* [Changing the overflow behavior for usize in release builds to panic](https://github.com/rust-lang/rfcs/pull/2635).
* [#[ffi_returns_twice]](https://github.com/rust-lang/rfcs/pull/2633).

# Upcoming Events

### Online

* [Feb 20. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Feb 25. Rust Community Content Subteam Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Feb 27. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia Pacific

* [Feb 16. Chennai, IN - Rust Chennai meetup](https://www.meetup.com/mad-rs/events/258822338/).

### Europe

* [Feb 18. Karlsruhe, DE - Karlsruhe Rust Hack and Learn](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/258728236/).
* [Feb 20. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzdbbc/).

### North America

* [Feb 14. Columbus, US - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dbcfrpyzdbsb/).
* [Feb 20. Sacramento, US - Sacramento Rust Inaugural Meetup](https://www.meetup.com/Rust-Sacramento/events/258393260/).
* [Feb 20. Chicago, US - Chicago Rust Meetup - Property-Based Testing in Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/257469240/).
* [Feb 20. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/hkllqqyzdbbc/).
* [Feb 21. San Diego, US - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/258775454/).
* [Feb 21. Arlington, US - Rust DC—Learn+Try: Custom Redis Datastructures](https://www.meetup.com/RustDC/events/257969733).
* [Feb 25. Durham, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/events/mfglwpyzdbhc/).
* [Feb 27. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzdbkc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Developer at Finhaven, Vancouver, CA](https://angel.co/finhaven/jobs/411238-software-developer).
* [Software Engineer at Discord, San Francisco, US](https://discordapp.com/jobs/4200751002).
* [Network Engineer at NearProtocol, San Francisco, US](https://nearprotocol.com/careers/?gh_jid=4205573002).
* [Navitia Software Engineer at Kisio Digital, Paris, FR](https://www.welcometothejungle.co/companies/kisio-digital/jobs/rust-c-developpeur-h-f_paris).
* [Rust web developer at Impero, Denmark/remote](https://impero.com/job/full-stack-web-developer-rust/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Once again, we have two quotes for the price of one:

> I love Rust because it reduces bugs by targeting it’s biggest source… me.

– [ObliviousJD on Twitter](https://twitter.com/ObliviousJD/status/1094456407376637952)

> Say the same thing about seatbelts in a car. If you don’t plan to have accidents, why do you need seatbelts?
>
> Car accidents, like mistakes in programming are a risk that has a likelihood that is non-zero. A seatbelt might be a little bit annoying when things go well, but much less so when they don’t. Rust is there to stop you in most cases when you try to accidentally shot yourself into the leg, unless you deliberately without knowing what you are doing while yelling “hold my beer” (unsafe). And contrary to popular belief even in unsafe blocks many of Rust’s safety guarantees hold, just not all.
>
> …
>
> Just like with the seatbelt, there will be always those that don’t wear one for their very subjective reasons (e.g. because of edge cases where a seatbelt could trap you in a burning car, or because it is not cool, or because they hate the feeling and think accidents only happen to people who can’t drive).

– [atoav on HN](https://news.ycombinator.com/item?id=19139949) comparing Rust's safety guarantees with seat-belts.

Thanks to [Kornel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/619) and [pitdicker](https://users.rust-lang.org/t/twir-quote-of-the-week/328/623) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/aq6zqa/this_week_in_rust_273/).</small>
