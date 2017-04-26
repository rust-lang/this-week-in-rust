Title: This Week in Rust 179
Number: 179
Date: 2017-04-25
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
* [What topics would you like to see covered in a video course about Rust?](https://users.rust-lang.org/t/what-topics-would-you-like-to-see-covered-in-a-video-course-about-rust/10500)

# Crate of the Week

This week's crate of the week is [pq](https://crates.io/crates/pq), a crate to generically decode protobuf messages. Thanks to [sevagh](https://users.rust-lang.org/users/sevagh) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rust: Error message when VS 2015 build tools exist but not the SDK needs to be better](https://github.com/rust-lang/rust/issues/41151).
* [easy] [lazy_static: Include `homepage` in Cargo.toml](https://github.com/rust-lang-nursery/lazy-static.rs/issues/67). lazy_static is a small macro for defining lazy evaluated static variables in Rust.
* [easy] [lazy_static: Include `categories` in Cargo.toml](https://github.com/rust-lang-nursery/lazy-static.rs/issues/66).
* [easy] [lazy_static: Publish CI badges for all Tier 1 platforms](https://github.com/rust-lang-nursery/lazy-static.rs/issues/65).
* [easy] [flate2: Wire up rustdoc hyperlinks](https://github.com/alexcrichton/flate2-rs/issues/81). flate2 implements FLATE, Gzip, and Zlib bindings for Rust.
* [easy] [flate2: Use distinct Flush types for `Compress::compress` vs `Decompress::decompress`](https://github.com/alexcrichton/flate2-rs/issues/79).
* [easy] [flate2: Document error conditions in "Errors" sections](https://github.com/alexcrichton/flate2-rs/issues/78).
* [easy] [flate2: Write usage examples](https://github.com/alexcrichton/flate2-rs/issues/76).
* [easy] [flate2: Rename internal types to match the public types](https://github.com/alexcrichton/flate2-rs/issues/75).
* [liner: Make keyboard interrupts (e.g. SIGINT from Ctrl-c) work](https://github.com/MovingtoMars/liner/issues/4). Liner is a readline-like library in Rust.
* [liner: Tilde expansion](https://github.com/MovingtoMars/liner/issues/34).
* [liner: Password mode](https://github.com/MovingtoMars/liner/issues/25).
* [liner: Use right arrow key to select autocompletion](https://github.com/MovingtoMars/liner/issues/37).
* [Ion: Optional Descriptions for Functions](https://github.com/redox-os/ion/issues/232). Ion is a shell for UNIX platforms, and is the default shell in Redox.
* [Ion: Implement Mapfiles](https://github.com/redox-os/ion/issues/247).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

98 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-04-17..2016-04-24

* [repr struct alignment](https://github.com/rust-lang/rust/pull/39999) (RFC [#1358](https://github.com/rust-lang/rfcs/blob/master/text/1358-repr-align.md)]
* [syntax: support parenthesis around trait bounds](https://github.com/rust-lang/rust/pull/41077)
* [:vis matcher for macro_rules](https://github.com/rust-lang/rust/pull/41012)
* [`traits::select(..)` filters out predicates from other traits](https://github.com/rust-lang/rust/pull/41486)
* [cache DTOR check constraints on abstract data types](https://github.com/rust-lang/rust/pull/41485)
* [performance audit Spring 2017](https://github.com/rust-lang/rust/pull/41469)
* [remove unstable deprecated items](https://github.com/rust-lang/rust/pull/41437)
* [don't panic if attribute macros don't resolve at crate root](https://github.com/rust-lang/rust/pull/41432)
* [hoedown makes a comeback!](https://github.com/rust-lang/rust/pull/41290)
* [re-enable hoedown by default](https://github.com/rust-lang/rust/pull/41431)
* [specialize `Vec::extend(IntoIter)`](https://github.com/rust-lang/rust/pull/41191)
* [specialize {`Path`, `OsStr`}`.clone_into()`](https://github.com/rust-lang/rust/pull/41390)
* [add functions to transmute floats to ints](https://github.com/rust-lang/rust/pull/39271)
* [don't clog register allocator with byvals](https://github.com/rust-lang/rust/pull/41378)
* [back out backtrace pruning logic](https://github.com/rust-lang/rust/pull/41364) (it was too eager)
* [convert calls to `visit_all_item_likes_in_crate(..)`](https://github.com/rust-lang/rust/pull/41360)
* [fix debug infinite loop](https://github.com/rust-lang/rust/pull/41342)
* [on-demand-ify `associated_item_def_ids`](https://github.com/rust-lang/rust/pull/41340)
* [on-demand-ify `monomorphic_const_eval`](https://github.com/rust-lang/rust/pull/41310)
* [polymorphic `const_eval(..)`](https://github.com/rust-lang/rust/pull/41408)
* [cargo: add `overflow_checks` to profiles](https://github.com/rust-lang/cargo/pull/3908)
* [cargo: CLI support for `--all-`{`bins`, `tests`, `examples`, `benches`}](https://github.com/rust-lang/cargo/pull/3901)
* [cargo: support `$RUSTC_WRAPPER`](https://github.com/rust-lang/cargo/pull/3887)

## New Contributors

* Aaron Hill
* alexey zabelin
* nate
* Nathaniel Ringo
* Scott McMurray
* Suchith J N

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: postpone] [Amend RFC 1105 to specify how dependency versions relate to semver](https://github.com/rust-lang/rfcs/pull/1890).
* [disposition: merge] [A portability lint](https://github.com/rust-lang/rfcs/pull/1868).
* [disposition: merge] [Improve the `assert_eq` failure message formatting to increase legibility](https://github.com/rust-lang/rfcs/pull/1866).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [Proposal for default crate recommendation ranking](https://github.com/rust-lang/rfcs/pull/1824).
* [disposition: postpone] [Stackless coroutines](https://github.com/rust-lang/rfcs/pull/1823). Add language-level support for stackless coroutines (also known as semicoroutines or generators).
* [disposition: close] [Create a separate libc_types crate for basic C types](https://github.com/rust-lang/rfcs/pull/1783).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Traits should be aliased the same way types can be aliased with the `type` keyword](https://github.com/rust-lang/rfcs/pull/1733).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).

## New RFCs

* [Introduce a public/private distinction to crate dependencies](https://github.com/rust-lang/rfcs/pull/1977).
* [Allow the usage of `use` inside `impl` blocks and `match` blocks](https://github.com/rust-lang/rfcs/pull/1976).
* [Prepare global allocators for stabilization](https://github.com/rust-lang/rfcs/pull/1974).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [Add text from the structs/unions RFC to the guide](https://github.com/rust-lang-nursery/fmt-rfcs/pull/78).
* [Ordering of types of groups within a module](https://github.com/rust-lang-nursery/fmt-rfcs/issues/71).
* [Convention about empty lines](https://github.com/rust-lang-nursery/fmt-rfcs/issues/57).
* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24)
* [Closures](https://github.com/rust-lang-nursery/fmt-rfcs/issues/35)
* [Where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)

# Upcoming Events

* [Apr 20. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr 20. Rust Utrecht - Use Rust: Mentored Workshop](https://www.meetup.com/Rust-Utrecht/events/238725437/).
* [Apr 26. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Apr 26. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Apr 27. Rust Stockholm - Rust meetup @ Distil Networks](https://www.meetup.com/ruststhlm/events/238207716/).
* [Apr 27. Rust Meetup Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/28).
* **[Apr 30. RustFest 2017 - Kyiv, Ukraine](http://2017.rustfest.eu/).**
* [May  3. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May  3. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May  4. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [May  4. Rust Bay Area: Using Rust at Dropbox to make Magic Pocket](https://www.meetup.com/Rust-Bay-Area/events/239222217/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [RustJobs.rs](https://rustjobs.rs) - a website dedicated to rust jobs. There are 13 open rust positions plus a list of companies using rust.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust doesn't end unsafety, it just builds a strong, high-visibility fence around it, with warning signs on the one gate to get inside. As opposed to C's approach, which was to have a sign on the periphery reading "lol good luck".

— [Quxxy on reddit](https://www.reddit.com/r/rust/comments/65t0eq/the_end_of_unsafety_the_past_present_and_future/dgd3h8o/).

Thanks to [msiemens](https://users.rust-lang.org/t/twir-quote-of-the-week/328/391) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
