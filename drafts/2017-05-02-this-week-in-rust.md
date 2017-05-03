Title: This Week in Rust 180
Number: 180
Date: 2017-05-02
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

* <img alt="balloon" class="emoji" title=":balloon:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/balloon.png?v=0"><img alt="tada" class="emoji" title=":tada:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/tada.png?v=0"> [Announcing Rust 1.17](https://blog.rust-lang.org/2017/04/27/Rust-1.17.html). <img alt="tada" class="emoji" title=":tada:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/tada.png?v=0"><img alt="balloon" class="emoji" title=":balloon:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/balloon.png?v=0">
* [Helix: Native Ruby extensions without fear](https://usehelix.com/).
* [bindgen now has a users guide](https://servo.github.io/rust-bindgen/).
* [Asynchronous Rust for fun & profit](http://xion.io/post/programming/rust-async-closer-look.html).
* [Rust your ARM microcontroller](http://blog.japaric.io/quickstart/). Build Rust applications for any ARM Cortex-M microcontroller.
* [Lessons learned redesigning and refactoring a Rust library](https://mgattozzi.com/refactor-rust).
* [Implementing Weld in Rust](http://dawn.cs.stanford.edu/blog/weld.html).
* [The RustConf 2017 call for proposals is now open](http://cfp.rustconf.com/events/rustconf-2017).
* [This week in Rust docs 54](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-54).
* [This week In Servo 100](https://blog.servo.org/2017/05/01/twis-100/).

# Crate of the Week

This week's crate of the week is [pq](https://crates.io/crates/pq), a crate to generically decode protobuf messages. Thanks to [sevagh](https://users.rust-lang.org/users/sevagh) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust team is looking for someone to take over the process of making Rust's release notes](https://internals.rust-lang.org/t/rust-release-milestone-predictions/4591/33).
* [rdedup](https://github.com/dpc/rdedup) - a data deduplication with compression and public key encryption library, is [looking for contributors](https://users.rust-lang.org/t/twir-call-for-participation/4821/42) who are interested in crypto, command line, and backups.
* [PumpkinDB](https://github.com/PumpkinDB/PumpkinDB) has a list of [starter issues](https://github.com/PumpkinDB/PumpkinDB/issues?q=is%3Aissue+is%3Aopen+label%3AWhatCanIStartWith%3F) for [people interested in an event sourcing database engine](https://users.rust-lang.org/t/twir-call-for-participation/4821/43).
* [easy] [tokei: AutoHotKey support](https://github.com/Aaronepower/tokei/issues/106). Tokei is a program that displays statistics about your code.
* [less easy] [tokei: Move CI to trust](https://github.com/Aaronepower/tokei/issues/120).
* [easy] [maud: Remove `error!` macro](https://github.com/lfairy/maud/issues/84). Maud is an HTML template engine for Rust.
* [less easy] [rust-bindgen: Add in-worklist bits to the analysis runner](https://github.com/servo/rust-bindgen/issues/664).
* [easy] [flate2: Use distinct Flush types for `Compress::compress` vs `Decompress::decompress`](https://github.com/alexcrichton/flate2-rs/issues/79). flate2 implements FLATE, Gzip, and Zlib bindings for Rust.
* [easy] [flate2: Write usage examples](https://github.com/alexcrichton/flate2-rs/issues/76).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

98 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-04-17..2017-04-24

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

* achernyak
* Artem Chernyak
* Eh2406
* gaurikholkar
* Henri Sivonen
* Jessica Hamilton
* Titus Barik

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1685: Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [RFC 1868: A portability lint](https://github.com/rust-lang/rfcs/pull/1868).
* [RFC 1824: Proposal for default crate recommendation ranking](https://github.com/rust-lang/rfcs/pull/1824).
* [RFC 1866: Improve the `assert_eq` failure message formatting to increase legibility](https://github.com/rust-lang/rfcs/pull/1866).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: close] [Generators](https://github.com/rust-lang/rfcs/pull/1832).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: postpone] [Add `SafeDeref` and `SafeDerefMut`, equivalent to `Deref` and `DerefMut` but which are guaranteed to always return the same object](https://github.com/rust-lang/rfcs/pull/1873).
* [disposition: close] [Allow the name (qualifier) of an enum variant to be elided in expressions and patterns whenever it can be inferred](https://github.com/rust-lang/rfcs/pull/1949).
* [disposition: merge] [Expand and stabilize `impl Trait`](https://github.com/rust-lang/rfcs/pull/1951).

## New RFCs

* [Amend RFC 1242 to require an RFC for deprecation of crates from the nursery](https://github.com/rust-lang/rfcs/pull/1983).
* [Tiered browser support policy for Rust's web content](https://github.com/rust-lang/rfcs/pull/1985).

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

* [May  3. Intro to Rust for Java programmers - Code@LTH](https://www.facebook.com/events/1395576530485976/).
* [May  3. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238448447/).
* [May  3. Boston Rust: Rust 1.0 Anniversary Party and Hack Night](https://www.meetup.com/BostonRust/events/239319480/).
* [May  3. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May  3. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May  4. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [May  4. Rust Bay Area: Using Rust at Dropbox to make Magic Pocket](https://www.meetup.com/Rust-Bay-Area/events/239222217/).
* [May  8. Prague Rust Meetup #3](https://www.meetup.com/rust-prague/events/239129625/).
* [May  8. Seattle Rust - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/).
* [May 10. Rust Rome - Rust Meetup #2 - Intro + Rocket.rs](https://www.meetup.com/it-IT/Rust-Roma/events/239513275/).
* [May 10. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 10. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 11. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlywhbpb/).
* [May 11. Rust DC - Building high performance REST APIs with Rust and Rocket](https://www.meetup.com/RustDC/events/239115583/).
* [May 15. Rust Sydney Meetup - Happy Birthday Rust](https://www.meetup.com/Rust-Sydney/events/239659974/)!
* [May 16. Tokyo Rust Meetup #7 - Rust Birthday Party](https://www.meetup.com/Tokyo-Rust-Meetup/events/239301821/)!
* [May 17. Rust LA May Meetup - Rust Birthday Party](https://www.meetup.com/Rust-Los-Angeles/events/239616841/)!
* [May 17. South Florida Rust - Rust Birthday Party](https://www.meetup.com/South-Florida-Rust-Meetup/events/239036595/)!
* [May 17. Rust Atlanta - Heterogeneous Collections in Rust at Tech Square Labs (Midtown)](https://www.meetup.com/Rust-ATL/events/239205124/).
* [May 17. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 17. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 18. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Indeed, it was that very event that lent the world to ruin...
>
> It had been decades since the last line of C code was erased, and the last C compiler (written, ironically (but unsurprisingly), in Rust) engraved onto a golden disc and launched toward that distant star around which the GooglePepsiMusk Sphere's construction could be faintly observed. To even utter that fell syllable would be met with swift retaliation from the paramilitary _Borrow Xekkers_ (the alphabet as well having been altered to better suit this shining memory-safe utopia). The whole world, everything, had all been rewritten in Rust, and at last the world knew universal peace and prosperity of boundless proportion.
>
> But something stirred... a prophecy ancient in origin. **The Second.0 Coming**. And when realized at last, the world was divided between the forward-thinking devotees of 𝕽𝖚𝖘𝖙 𝕿𝖜𝖔-𝕻𝖔𝖎𝖓𝖙-𝕺𝖍 and those vainly clinging to 𝕽𝖚𝖘𝖙 𝕺𝖓𝖊-𝕻𝖔𝖎𝖓𝖙-𝕱𝖔𝖚𝖗-𝕳𝖚𝖓𝖉𝖗𝖊𝖉-𝕬𝖓𝖉-𝕾𝖊𝖛𝖊𝖓𝖙𝖊𝖊𝖓.
>
> And thus did The Great Schism rend interstellar civilization in twain.
>
> There was but one ray of hope. As the fires of war rushed toward the newly-completed GooglePepsiMusk Sphere, its cadre of elite 10<sup>10x</sup> programmers set to work. Their task: to write the most advanced compound artificial intelligence ever known, one capable of averting the catastrophe consuming the universe: _AlexKryton.exe_<sup>[1]</sup> . Due to Rust's incredible productivity benefits, they completed their task in a mere 22 seconds--and thanks to the Rust compiler's incredible performance, spent only nine months waiting for it compile. Using the artificial black hole at the center of the GPMS, and with only moments to spare, they sent their AI back in time, to the era of Rust's birth: 2011.
>
> That's the whole story. And yet those perceptible among you may ask: "_which version of Rust did they use?!_" Alex alone knows...
>
> <sub>[1] Yeah, Windows won. Sorry.</sub>

— [/u/kibwen revealing the origin of a highly advanced AI from future, known today as "Alex Chrichton"](https://www.reddit.com/r/rust/comments/67x46l/announcing_rust_117/dgty1ay/).

Thanks to [/u/burkadurka](https://www.reddit.com/r/rust/comments/67x46l/announcing_rust_117/dguc8x0/) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
