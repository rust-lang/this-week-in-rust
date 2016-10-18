Title: This Week in Rust 153
Number: 153
Date: 2016-10-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## Blog Posts

## News & Project Updates

## Other Weeklies from Rust Community

# Crate of the Week

This week's Create of the Week is [xargo](https://github.com/japaric/xargo) - for effortless cross compilation of Rust programs to custom bare-metal targets like ARM Cortex-M. It recently reached version 0.2.0 and you can [read the announcement here](https://users.rust-lang.org/t/xargo-v0-2-0-effortless-cross-compilation-to-custom-bare-metal-targets/7679).

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Provide a better error message when the target sysroot is not installed](https://github.com/rust-lang/rust/issues/37131).
* [less easy] [servo: Implement HTMLTimeElement#dateTime](https://github.com/servo/servo/issues/12967).
* [hard] [rust: Optimize emscripten targets with emcc](https://github.com/rust-lang/rust/issues/36899).
* [hard] [rust: Tell emscripten to remove exception handling code when the panic runtime is used](https://github.com/rust-lang/rust/issues/36900).
* [easy] [imag: Iterator for `Iterator<Item = Result<T, Error>>` tracing (wanna learn how to implement iterators?](https://github.com/matthiasbeyer/imag/issues/813).
* [easy] [maud: Support "while" and "while let"](https://github.com/lfairy/maud/issues/51). Maud is an HTML template engine for Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

106 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-10-10..2016-10-17

* [Implement `read_offset` and `write_offset`](https://github.com/rust-lang/rust/pull/35704).
* [Add ThreadId for comparing threads](https://github.com/rust-lang/rust/pull/36341).
* [Cache conscious hashmap table](https://github.com/rust-lang/rust/pull/36692).
* [Add `method str::repeat(self, usize) -> String`](https://github.com/rust-lang/rust/pull/36699).
* [Add two functions to check type of given address](https://github.com/rust-lang/rust/pull/36707).
* [Add `Vec::dedup_by` and `Vec::dedup_by_key`](https://github.com/rust-lang/rust/pull/36743).
* [Add two functions to check type of `SockAddr`](https://github.com/rust-lang/rust/pull/36762).
* [Add `println!()` macro with out any arguments](https://github.com/rust-lang/rust/pull/36825).
* [stabilise `?`, attributes on stmts, deprecate Reflect](https://github.com/rust-lang/rust/pull/36995).
* [Error monitor should emit error to stderr instead of stdout](https://github.com/rust-lang/rust/pull/37066).
* [Make the AF_NETLINK constant available for Android](https://github.com/rust-lang/libc/pull/424).
* [Specialize `Vec::extend` to `Vec::extend_from_slice`](https://github.com/rust-lang/rust/pull/37094).
* [Lint against lowercase static mut](https://github.com/rust-lang/rust/pull/37162).

## New Contributors

* Danny Hua
* Fabian Frei
* Mikko Rantanen
* Nabeel Omer

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

* [Use `#[link(kind)]` to fix imports from native libs on Windows](https://github.com/rust-lang/rfcs/pull/1717).
* [Enable customizing the linkage of a platform's C runtime](https://github.com/rust-lang/rfcs/pull/1721).
* [Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).
* [Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).

## New RFCs

* [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

FCP issues:

* [Guiding principles](https://github.com/rust-lang-nursery/fmt-rfcs/issues/4).
* [Comments](https://github.com/rust-lang-nursery/fmt-rfcs/issues/17).

Other issues getting a lot of discussion:

* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24).
* [Boolean and arithmetic expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/18).
* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/issues/11).

_No PRs this week._

# Upcoming Events

* [10/19. Los Angeles Rust Meetup](https://www.meetup.com/Rust-Los-Angeles/events/234140744/).
* [10/19. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [10/19. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [10/20. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [10/27 and 10/28 Rust Belt Rust in Pittsburgh, US](http://www.rust-belt-rust.com/). Tickets still available.
* [10/26. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [10/26. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* **[Rust Belt Rust Conference - Pittsburgh](http://www.rust-belt-rust.com/)**.
* [10/31. Rust Paris](https://www.meetup.com/Rust-Paris/events/234528214/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Friends of the Forest

Our community likes to recognize people who have made outstanding contributions
to the Rust Project, its ecosystem, and its community. These people are
'friends of the forest'.

This week's friends of the forest are:

* From [AtheMathmo]:

> I'd like to nominate [bluss] for his work on scientific programming in Rust.
> [ndarray] is a monumental project but in addition to that he has worked
> (really) hard to share that knowledge among others and provided easy-to-use
> libraries like [matrixmultiply]. Without bluss' assistance rulinalg would be
> in a far worse state.

* From [nasa42]:

> I'd like to nominate [Yehuda Katz], the lord of package managers.

[AtheMathmo]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/9
[bluss]: https://github.com/bluss
[ndarray]: https://github.com/bluss/rust-ndarray
[matrixmultiply]: https://github.com/bluss/matrixmultiply
[nasa42]: https://www.reddit.com/r/rust/comments/576h2q/this_week_in_rust_151/d8pca3k
[Yehuda Katz]: https://github.com/wycats

[Submit your Friends-of-the-Forest nominations for next week][foft]!

[foft]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295

# Quote of the Week

> <dRk\> that gives a new array of errors, guess that's a good thing
> <misdreavus\> you passed one layer of tests, and hit the next layer :P
> <misdreavus\> rustc is like onions
> <dRk\> it makes you cry?


— From [#rust-beginners](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners).

Thanks to [Quiet Misdreavus](https://users.rust-lang.org/users/quietmisdreavus) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
