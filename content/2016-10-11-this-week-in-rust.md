Title: This Week in Rust 151
Number: 151
Date: 2016-10-11
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

* [Making terminal applications in Rust with Termion](https://ticki.github.io/blog/making-terminal-applications-in-rust-with-termion/).
* [Optimising string processing in Rust](https://lise-henry.github.io/articles/optimising_strings.html).
* [Bare minimum crates.io mirror plus one](http://integer32.com/2016/10/08/bare-minimum-crates-io-mirror-plus-one.html). Setting up & using download-API mirror of crates.io and registry index mirror of [crates.io-index](https://github.com/rust-lang/crates.io-index).
* [The arduous journey of porting C to Rust](https://bawk.space/2016/10/06/c-to-rust.html).
* [IntelliJ/Rust are fast becoming buddies](https://xrl.github.io/rust/2016/10/09/intellij-for-rust.html).
* [Using `mem::replace` to keep owned values in changed enums](https://github.com/rust-unofficial/patterns/blob/master/idioms/mem-replace.md).
* [How to teach X to people who've never learned X](http://jeenalee.com/2016/10/10/how-to-teach-and-how-to-learn.html).
* [Using Rust for webdev as a hobby programmer](http://neikos.me/Using_Rust_for_Webdev_as_a_Hobby_Programmer.html).
* [Rust and Automake](https://www.figuiere.net/hub/blog/?2016/10/07/862-rust-and-automake).
* [Type-Safe Unions in C++ and Rust](http://genbattle.bitbucket.org/blog/2016/10/07/Type-Safe-Unions-in-C-and-Rust/).
* [Servo: Wrapping up Google Summer of Code 2016](https://blog.servo.org/2016/09/28/gsoc-summary/).
* [videos] [RustConf 2016 - YouTube Playlist](https://www.youtube.com/playlist?list=PLE7tQUdRKcybLShxegjn0xyTTDJeYwEkI).
* [discussion] [Tricky corner cases in design of Rust programming language](https://www.reddit.com/r/rust/comments/55sio3/traps_during_the_design_of_rust_the_language/).

## News & Project Updates

* [GDB 7.12 is released with Rust debugging support](https://www.mail-archive.com/info-gnu@gnu.org/msg02192.html).
* [CodeLLDB: a LLDB front end for Visual Studio Code - now supports Rust debugging](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
* [Diesel 0.8.0 is released with diesel_codegen rewritten to use the Macros 1.1 framework](https://github.com/diesel-rs/diesel/commit/04bf2fcbc7f756d577e85afe00c8a133f9b84d4b).
* [rustup is now available in `Community` repo of Arch Linux](https://www.archlinux.org/packages/community/x86_64/rustup/).
* [Macros (and syntax extensions and compiler plugins) - where are we at](https://users.rust-lang.org/t/macros-and-syntax-extensions-and-compiler-plugins-where-are-we-at/7600)?
* [imag pre-1.0.0 release strategy](http://beyermatthias.de/blog/2016/10/04/imag-pre-1-0-0-release-strategy/).

## Other Weeklies from Rust Community

* [This week in Servo 80](https://blog.servo.org/2016/10/03/twis-80/). Servo is a prototype web browser engine written in Rust.
* [This week in Rust docs 25](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-25). Updates from the Rust documentation team.
* [This week in Ruma 2016-10-09](https://www.ruma.io/news/this-week-in-ruma-2016-10-09/). Ruma is a Matrix homeserver written in Rust.
* [This week in Tock OS 6](http://www.tockos.org/blog/2016/talking-tock-6/). Tock is a safe, multitasking operating system for low-power, low-memory microcontrollers.
* [This week in Ruru 2](http://this-week-in-ruru.org/2016/10/04/this-weeks-in-ruru-2/). Ruru lets you write native Ruby extensions in Rust.
* [What's coming up in imag (17)](http://beyermatthias.de/blog/2016/10/07/what-s-coming-up-in-imag-17/). imag is a text based personal information management suite.

## New Crates

* [SvgBobRus](https://github.com/ivanceras/svgbobrus). Convert your ASCII diagram scribbles into happy little SVG.
* [Curryrs](https://github.com/mgattozzi/curryrs). A library for providing easy to use bindings between Rust and Haskell code.
* [NetBricks](https://github.com/NetSys/NetBricks). A new network function framework based on Rust.
* [F3](https://users.rust-lang.org/t/f3-a-crate-to-play-with-the-stm32f3discovery/7541). A crate to play with STM32F3DISCOVERY development board with a Cortex-M4 microcontroller.
* [derivative](https://github.com/mcarton/rust-derivative): A set of attribute-enhanced `#[derive]` for built-in traits using Macro 1.1.

# Crate of the Week

*No crate was selected for CotW.*

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: tidy script no longer checks lang features](https://github.com/rust-lang/rust/issues/37013).
* [easy] [Servo: Stylo: Implement font-kerning](https://github.com/servo/servo/issues/13667).
* [hard] [rust: Optimize emscripten targets with emcc](https://github.com/rust-lang/rust/issues/36899).
* [hard] [rust: Tell emscripten to remove exception handling code when the panic runtime is used](https://github.com/rust-lang/rust/issues/36900).
* [moderate] [super: Coloring errors in the console](https://github.com/SUPERAndroidAnalyzer/super/issues/41).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

135 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-10-03..2016-10-10

* [std: Stabilize and deprecate APIs for 1.13](https://github.com/rust-lang/rust/pull/36815).
* [Add ThreadId for comparing threads](https://github.com/rust-lang/rust/pull/36341).
* [Add support for per-target rustflags in `.cargo/config`](https://github.com/rust-lang/cargo/pull/3157).
* [Speed up `plug_leaks`](https://github.com/rust-lang/rust/pull/36917).
* [Cargo: Add `--message-format` flag](https://github.com/rust-lang/cargo/pull/3000).
* [Add Thumbs target definitions to the compiler](https://github.com/rust-lang/rust/pull/36874).
* [rustc: Rename `rustc_macro` to `proc_macro`](https://github.com/rust-lang/rust/pull/36945).
* [Clarify HashMap's capacity handling](https://github.com/rust-lang/rust/pull/36766).
* [Enforce the shadowing restrictions from RFC 1560 for today's macros](https://github.com/rust-lang/rust/pull/36767).
* [Restrict where in the tree platform-specific cfgs may be mentioned](https://github.com/rust-lang/rust/pull/36807).
* [Improve error message and snippet for "did you mean `x`"](https://github.com/rust-lang/rust/pull/36798).
* [Cargo: Warn about path overrides that won't work](https://github.com/rust-lang/cargo/pull/3136).
* [Refactoring/bugfixing around definitions for struct/variant constructors](https://github.com/rust-lang/rust/pull/36814).
* [std: Correct stability attributes for some implementations](https://github.com/rust-lang/rust/pull/36902).

## New Contributors

* Anthony Ramine
* Christopher
* Eric Roshan-Eisner
* Florian Diebold
* KillTheMule
* Mathieu Borderé
* Nick Stevens
* p512
* Razican
* Stephen M. Coakley
* 石博文

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

* [Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).
* [Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).

## New RFCs

* [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [Abort by default v2](https://github.com/rust-lang/rfcs/pull/1765). Specify abort-by-default in `Cargo.toml` when the user does `cargo new --bin`, as well as various other refinements to the panick strategy system.

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

* [10/13. Rust Orange County Inaugural Meetup](https://www.meetup.com/Rust-Los-Angeles/events/234277000/).
* [10/13. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/233996456/).
* [10/15. South Florida Rust Meetup](http://www.meetup.com/South-Florida-Rust-Meetup/events/234791780/).
* [10/18. London Rust User Group Meetup #9](https://www.meetup.com/Rust-London-User-Group/events/233034964/).
* [10/19. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [10/19. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [10/20. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [10/27 and 10/28 Rust Belt Rust in Pittsburgh, US](http://www.rust-belt-rust.com/). Tickets still available.

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

* From [jimmycuadra]:

> [dtolnay] for outstanding work on [Serde], taking it from a great library
> into an outstanding library, improving documentation significantly, being on
> top of the macros 1.1 transition, and even developing a new high level
> library for making custom derive under macros 1.1 easier to work with.
>
> [sgrif] for outstanding work on [Diesel], an ORM that will change the game
> for ORMs, and for being incredibly helpful and friendly with early adopters.

* From [Mark_Simulacrum]:

> njn (on IRC) or [nnethercote1] on GitHub for outstanding work on compiler perf.
> They've [removed an allocation during HashSet creation][HashSet], made [TypedArena
> lazily allocate the first chunk][TypedArena], and [more]. They also have helped with
> [adding a benchmarking script][benchmarking] to compare two different compiler versions
> against the benchmarks in [http://perf.rust-lang.org], which helps future work
> in this area.

* From [nmatsakis]:

> I nominate [TimNN] for Friend of the Forest for his repeated and invaluable
> work minimizing and bisecting ([example]). Keep up the good work!

* From [dzamlo]:

> I'd like to nominate [BurntSushi] for Friend of the Forest. I think the
> multiples crates he contributed are both important and high quality. In
> addition to his code contributions to the ecosystem, he also did some good
> and informative write up about some of them.

* From [pmarcelll]:

> I'd like to nominate [GuillaumeGomez] for the "Rust documentation superhero"
> title as well.

[jimmycuadra]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/4?u=erickt
[dtolnay]: https://github.com/dtolnay
[Serde]: https://serde.rs/
[sgrif]: https://github.com/sgrif
[Diesel]: http://diesel.rs/
[Mark_Simulacrum]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/5?u=erickt
[nnethercote1]: https://github.com/nnethercote
[HashSet]: https://github.com/rust-lang/rust/pull/36734
[TypedArena]: https://github.com/rust-lang/rust/pull/36592
[more]: https://github.com/rust-lang/rust/pulls?utf8=%E2%9C%93&q=is%3Apr%20author%3Annethercote
[benchmarking]: https://github.com/rust-lang-nursery/rustc-benchmarks/pull/17
[http://perf.rust-lang.org]: http://perf.rust-lang.org
[nmatsakis]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/6?u=erickt
[TimNN]: https://github.com/TimNN
[example]: https://github.com/rust-lang/rust/issues/36954#issuecomment-251361101
[dzamlo]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295/7?u=erickt
[BurntSushi]: https://github.com/burntsushi
[pmarcelll]: https://www.reddit.com/r/rust/comments/55utit/this_week_in_rust_150/d8dwvcy
[GuillaumeGomez]: https://github.com/GuillaumeGomez

[Submit your Friends-of-the-Forest nominations for next week][foft]!

[foft]: https://users.rust-lang.org/t/twir-friends-of-the-forest/7295

# Quote of the Week

> < Celti> I just had a recruiter contact me for a Rust job requiring 3+ years of professional experience with it.

— From [#rust](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust).

Thanks to [bluss](https://users.rust-lang.org/users/bluss) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
