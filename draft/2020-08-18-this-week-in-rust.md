Title: This Week in Rust 352
Number: 352
Date: 2020-08-18
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://audio.rustacean-station.org/file/rustacean-station/twir-2020-08-11.mp3)

# Updates from Rust Community

### Official
* [Laying the foundation for Rust's future](https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html)

### Tooling
* [Rust Analyzer Changelog #38](https://rust-analyzer.github.io/thisweek/2020/08/17/changelog-38.html)

### Newsletters
* [The Embedded Working Group Newsletter - 24](https://rust-embedded.github.io/blog/newsletter-24/)

### Observations/Thoughts
* [Why Rust is a great fit for embedded software](https://tweedegolf.nl/blog/39/why-rust-is-a-great-fit-for-embedded-software)
* [Why Rust's Unsafe Works](https://jam1.re/blog/why-rusts-unsafe-works)
* [Rust RwLock and Mutex Performance Oddities](https://fy.blackhats.net.au/blog/html/2018/10/19/rust_rwlock_and_mutex_performance_oddities.html)
* [The most exciting journey of 2020](https://medium.com/@knidarkness/the-most-exciting-journey-of-2020-6262d6c6f03)
* [The smooth resize test](https://raphlinus.github.io/rust/gui/2019/06/21/smooth-resize-test.html)
* ["Rust does not have a stable ABI"](https://people.gnome.org/~federico/blog/rust-stable-abi.html)
* [Rust Review](https://dev.to/yujiri8/rust-review-515p)
* [Rust vs C++: A JS/TS Developer's Perspective](https://www.reddit.com/r/rust/comments/icdape/rust_vs_c_a_jsts_developers_perspective/)
* [Comparative Unsafety](https://flak.tedunangst.com/post/comparative-unsafety)
* [Code Smell: Concrete Abstraction](https://matklad.github.io/2020/08/15/concrete-abstraction.html)
* [Against Glob Imports](https://drs.is/post/against-globs/)

### Learn Standard Rust
* [Lifetimes in Rust](https://hashrust.com/blog/lifetimes-in-rust/)
* [Learning Rust: The Compiler is your Friend](https://ferrous-systems.com/blog/the-compiler-is-your-friend/)
* [I am a Java, C#, C or C++ developer, time to do some Rust](https://fasterthanli.me/articles/i-am-a-java-csharp-c-or-cplusplus-dev-time-to-do-some-rust)
* [Frustrated? It's not you, it's Rust](https://fasterthanli.me/articles/frustrated-its-not-you-its-rust)
* [Async Unicorns love Rust](https://blog.kdubovikov.ml/articles/rust/async-unicorns-love-rust)
* [Who Builds the Builder?](https://matklad.github.io/2020/08/12/who-builds-the-builder.html)
* [Introduction to Rust](https://serokell.io/blog/rust-guide)
* [Rust for Beginners – Get Started with the Most Loved Programming Language](https://www.freecodecamp.org/news/rust-getting-started-with-the-most-loved-programming-language/)
* [First steps with WebAssembly in Rust](https://aralroca.com/blog/first-steps-webassembly-rust)
* [Variables and memory management](https://www.warambil.com/variables-and-memory-management-in-rust)
* [WebAssembly Without The Browser Part 1](https://alexene.dev/2020/08/17/webassembly-without-the-browser-part-1.html)
* [How I Learned to Stop Fighting the Borrow Checker and Love Dirty Structs](https://medium.com/adobetech/how-i-learned-to-stop-fighting-the-borrow-checker-and-learned-to-love-dirty-structs-b6c5fe91b1dd)
* [Software Development Languages: Rust](https://www.fosskers.ca/en/blog/rust-software-dev)
* [ES] [Trait Objects para hacer Dynamic Dispatch en Rust](https://emanuelpeg.blogspot.com/2020/08/trait-objects-para-hacer-dynamic.html#.XzSf1yK4C00.reddit)
* [ES] [Que es Ownership en Rust?](https://emanuelpeg.blogspot.com/2020/08/que-es-ownership-en-rust.html#.XzyW-R5KFe0.reddit)
* [ES] [Que es Ownership en Rust? Parte 2](https://emanuelpeg.blogspot.com/2020/08/que-es-ownership-en-rust-parte-2.html#.XzybavgXG4M.reddit)
* [PT] [Aprendendo Rust: 07 - Regiões da memória usadas pela Rust](https://dev.to/pehdepano/aprendendo-rust-07-regioes-da-memoria-usadas-pela-rust-208k)

### Learn More Rust
* [Linux Packages For Rust (2/3) - Building with GitHub Actions using Custom Actions and Docker Container Images](https://ebbflow.io/blog/vending-linux-2)
* [Temporarily opt-in to shared mutation](https://ryhl.io/blog/temporary-shared-mutation/)
* [RISC-V OS using Rust: Input Devices](https://blog.stephenmarz.com/2020/08/03/risc-v-os-using-rust-input-devices/)
* [Rusty Chains: A Basic Blockchain Implementation Written in Pure Rust](https://hackernoon.com/rusty-chains-a-basic-blockchain-implementation-written-in-pure-rust-gk2m3uri)
* [Error recovery with parser combinators (using nom)](https://www.eyalkalderon.com/nom-error-recovery/)
* [My first smart contract in Rust on Elrond VM](https://hiddentao.com/archives/2020/07/17/my-first-smart-contract-in-rust-on-elrond-vm)
* [Writing an Emacs module in Rust](https://dev.to/rfaulhaber/writing-an-emacs-module-in-rust-3pg5)
* [Exploring Azure CosmosDB with Rust - Part 2](https://dev.to/mindflavor/exploring-azure-cosmosdb-with-rust-part-2-32c0)
* [A whirlwind tour of the Mender client architecture using Rust](https://mender.io/blog/a-whirlwind-tour-of-the-mender-client-architecture-using-rust)

### Project Updates
* [Run Rust on your embedded device from VSCode in one click](https://ferrous-systems.com/blog/run-rust-on-your-embedded-device-from-vscode/)
* [Control Flow Guard for Clang/LLVM and Rust](https://msrc-blog.microsoft.com/2020/08/17/control-flow-guard-for-clang-llvm-and-rust/)
* [Cranelift can now compile rustc - giving nearly 7x faster compilations than LLVM!](https://www.reddit.com/r/rust/comments/iat25g/cranelift_can_now_compile_rustc_giving_nearly_7x/)
* [Nightly stdlib docs now document all keywords!](https://www.reddit.com/r/rust/comments/ia1vlc/nightly_stdlib_docs_now_document_all_keywords/)
* [Faster development with tiny bounties](https://opencollective.com/clap/updates/faster-development-with-tiny-bounties)

### Miscellaneous
* [RustFest goes Global](https://blog.rustfest.eu/rustfest-goes-global)
* [Oxide: The Essence of Rust](https://arxiv.org/abs/1903.00982)
* [Why I don't believe in pure functional programming anymore](https://dev.to/yujiri8/why-i-don-t-believe-in-pure-functional-programming-anymore-gin)
* [The merits of Rust for SSI and IAM software](https://dev.to/jolocomdev/engineering-safer-and-more-secure-solutions-for-digital-identity-and-access-management-with-rust-2e39)
* [Using Long Paths in Windows and Rust](https://gal.hagever.com/posts/windows-long-paths-in-rust/)
* [Stackage for Rust](https://www.snoyman.com/blog/2020/08/stackage-for-rust)
* [Why We're Bringing Astropad Cross-Platform with Rust](https://astropad.com/why-rust/)
* [Rewriting Pushpin's connection manager in Rust](https://blog.fanout.io/2020/08/11/rewriting-pushpins-connection-manager-in-rust/)
* [audio] [AreWePodcastYet - 06 Jonathan Turner](https://soundcloud.com/arewepodcastyet/awpy-06-jonathan-turner)

# Crate of the Week

This week's crate is [cargo-c](https://github.com/lu-zero/cargo-c), a cargo subcommand to build and install C-ABI compatibile dynamic and static libraries.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/799) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [RustFest Global 2020 CFP](https://cfp.rustfest.eu/events/2020)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

345 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-10..2020-08-17

* [use existing `infcx` when emitting trait impl diagnostic](https://github.com/rust-lang/rust/pull/75363)
* [detect JS-style `===` and `!==` and recover](https://github.com/rust-lang/rust/pull/75321)
* [detect likely `for foo of bar` JS syntax](https://github.com/rust-lang/rust/pull/75320)
* [move stack size check to `const_eval` machine](https://github.com/rust-lang/rust/pull/75338)
* [add `array` lang item and `[T; N]::map(f: FnMut(T) -> S)`](https://github.com/rust-lang/rust/pull/75212)
* [remove branch in optimized `is_ascii`](https://github.com/rust-lang/rust/pull/74562)
* [constified `str::from_utf8_unchecked`](https://github.com/rust-lang/rust/pull/75157)
* [hard way to respect `BTreeMap`'s minimum node length](https://github.com/rust-lang/rust/pull/75105)
* [BTreeMap: purge innocent use of `into_kv_mut`](https://github.com/rust-lang/rust/pull/75195)
* [hashbrown: implement `FusedIterator` and `size_hint` for `DrainFilter`](https://github.com/rust-lang/hashbrown/pull/188)
* [rustdoc: don't print "const" keyword on non-nightly build if `rustc_const_unstable` is used on the item](https://github.com/rust-lang/rust/pull/74936)

## Rust Compiler Performance Triage

* [2020-08-17](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-17.md).
  4 regressions, 3 improvements, 4 mixed bags.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Stabilize Range[Inclusive]::is_empty](https://github.com/rust-lang/rust/pull/75132)
* [disposition: merge][ stabilize ptr_offset_from](https://github.com/rust-lang/rust/pull/74238)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [August 19. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out Night](https://www.meetup.com/Vancouver-Rust/events/vcgsvrybclbzb/)
* [August 20. RustConf](https://rustconf.com/)

### North America
* [August 25. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybclbhc/)

### Asia Pacific
* [August 18. Seoul, KR - Rust Meetup - Rust last 6 months review (also available on Zoom)](https://www.meetup.com/Rust-Seoul-Meetup/events/qfkdvrybclbxb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Backend Engineer at Adobe (San Francisco)](https://adobe.wd5.myworkdayjobs.com/en-US/external_experienced/job/San-Francisco/Back-End-Engineer_84977)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> As Dave Herman always told me, “macros are for when you run out of language”. If you still have language left—and Rust gives you a lot of language—use the language first.

- [Patrick Walton on twitter](https://twitter.com/pcwalton/status/1294676975575896064)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/926) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]()</small>
