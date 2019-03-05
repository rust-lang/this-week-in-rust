Title: This Week in Rust 276
Number: 276
Date: 2019-03-05
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

This week's crate is [multi_try](https://github.com/JoshMcguigan/multi_try), a crate to simplify working with multiple results. Thanks to [Azriel Hoh](https://users.rust-lang.org/t/crate-of-the-week/2704/495) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [RustConf CfP is now open](https://cfp.rustconf.com/events/rustconf-2019).
* [RustFest is searching for local teams to apply for the next events](https://blog.rustfest.eu/call-for-teams).
* [Rust: Add LLVM atomic memcpy intrinsics, expose in core/std](https://github.com/rust-lang/rust/issues/58599).
* [TiKV: support ALLOW_INVALID_DATES in coprocessor](https://github.com/tikv/tikv/issues/4100).
* [TiKV: Use breakpad + symbolic to generate and interpret minidump-format core dumps](https://github.com/tikv/tikv/issues/4202).
* [TiKV: Make git dependency revisions explicit in Cargo.toml](https://github.com/tikv/tikv/issues/4283).
* [LSD: Looking for maintainers](https://github.com/Peltoche/lsd/issues/131).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

195 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-02-25..2019-03-04

* [Support defining C compatible variadic functions](https://github.com/rust-lang/rust/pull/57760)
* [Fix C-variadic function printing](https://github.com/rust-lang/rust/pull/58865)
* [Add support for using a jobserver with Rayon](https://github.com/rust-lang/rust/pull/56946)
* [Stabilize `unrestricted_attribute_tokens`](https://github.com/rust-lang/rust/pull/57367)
* [Include bounds from promoted constants in NLL](https://github.com/rust-lang/rust/pull/57202)
* [NLL: Type check operations with pointer types](https://github.com/rust-lang/rust/pull/58673)
* [NLL: Remove `LiveVar`](https://github.com/rust-lang/rust/pull/58505)
* [Self-Profiler: Make the profiler faster/more efficient](https://github.com/rust-lang/rust/pull/58425)
* [Normalize the type `Self` resolves to in an impl](https://github.com/rust-lang/rust/pull/58757)
* [Use internal iteration in all methods of `Filter` and `FilterMap`](https://github.com/rust-lang/rust/pull/58730)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2627: `#[link(kind="raw-dylib")]`](https://github.com/rust-lang/rfcs/pull/2627).
* [RFC 2532: Associated type defaults](https://github.com/rust-lang/rfcs/pull/2532).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*


### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Cosmetic changes to compiler comments and docs](https://github.com/rust-lang/rust/issues/58619).
* [disposition: merge] [Relax Read bounds on a bunch of BufReader<R> methods](https://github.com/rust-lang/rust/pull/58423).
* [disposition: merge] [Tracking issue for Option::xor](https://github.com/rust-lang/rust/issues/50512).
* [disposition: merge] [Tracking issue for RFC 2043: Add `align_offset` intrinsic (formerly: and `[T]::align_to` function)](https://github.com/rust-lang/rust/issues/44488).
* [disposition: merge] [Tracking issue for RefCell::{replace, swap}](https://github.com/rust-lang/rust/issues/43570).
* [disposition: merge] [Tracking issue for Vec::remove_item](https://github.com/rust-lang/rust/issues/40062).

## New RFCs

* [Add scoped threads to the standard library](https://github.com/rust-lang/rfcs/pull/2647).
* [Add more examples for | patterns in let in RFC 2175](https://github.com/rust-lang/rfcs/pull/2646).
* [Transparent Unions](https://github.com/rust-lang/rfcs/pull/2645).
* [Simplify visibility grammar](https://github.com/rust-lang/rfcs/pull/2640).

# Upcoming Events

### Online

* [Mar  6. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Mar 13. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Africa

* [Mar  6. Sandown, ZA - Johannesburg meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/qbhxmqyzfbjb/).

### Asia Pacific

* [Mar  6. Selangor, MY - Rust Malaysia Meetup Kuala Lumpur](https://www.facebook.com/events/1128655260646848/).

### Europe

* [Feb 28. Copenhagen, DK - Copenhagen Rust Hack Night #0xC](https://cph.rs/).
* [Feb 28. Torino, IT - Rust Turin Meetup](https://www.meetup.com/Mozilla-Torino/events/258586428).
* [Mar  3. St. Petersburg, RU - St. Petersburg Rust Meetup](https://www.meetup.com/spbrust/events/whmxrqyzfbfb).
* [Mar  6. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzfbjb/).
* [Mar 14. Brno, CZ - Rust Brno Meetup at Masaryk University](https://rust-brno.github.io/)

### North America

* [Mar  6. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzfbjb/).
* [Mar  6. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/cbcmbqyzfbjb/).
* [Mar  6. Vancouver, CN - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/hkllqqyzfbjb/).
* [Mar 11. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/nzfspqyzfbpb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer at TenX, Singapore](https://tenx.workable.com/jobs/689264).
* [Software Engineer - Blockchain at TenX, Sydney, AU](https://tenx.workable.com/jobs/689268).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

And again, we have two quotes for the week:

> > Can you eli5 why TryFrom and TryInto matters, and why it’s been stuck for so long ? (the RFC seems to be 3 years old)
>
> If you stabilise Try{From,Into}, you also want implementations of the types in std. So you want things like impl TryFrom<u8> for u16. But that requires an error type, and that was (I believe) the problem.
>
> u8 to u16 cannot fail, so you want the error type to be !. Except using ! as a type isn’t stable yet. So use a placeholder enum! But that means that once ! is stabilised, we’ve got this Infallible type kicking around that is redundant. So change it? But that would be breaking. So make the two isomorphic? Woah, woah, hold on there, this is starting to get crazy…
>
> *new person bursts into the room* “Hey, should ! automatically implement all traits, or not?”
>
> “Yes!” “No!” “Yes, and so should all variant-less enums!”
>
> Everyone in the room is shouting, and the curtains spontaneously catching fire. In the corner, the person who proposed Try{From,Into} sits, sobbing. It was supposed to all be so simple… but this damn ! thing is just ruining everything.
>
> … That’s not what happened, but it’s more entertaining than just saying “many people were unsure exactly what to do about the ! situation, which turned out to be more complicated than expected”.

– /u/Quxxy [on reddit](https://www.reddit.com/r/rust/comments/avbkts/this_week_in_rust_275/ehe2kre/?context=1)

> > What is the ! type?
>
> The never type 15 for computations that don’t resolve to a value. It’s named after its stabilization date.

– /u/LousyBeggar [on reddit](https://www.reddit.com/r/rust/comments/avbkts/this_week_in_rust_275/ehe50oj/)

Thanks to [runiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/625) and [StyMaar](https://users.rust-lang.org/t/twir-quote-of-the-week/328/626) for the suggestions!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
