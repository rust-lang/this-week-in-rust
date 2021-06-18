Title: This Week in Rust 395
Number: 395
Date: 2021-06-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

**Highlight**

*Content warning: Suicide, Mental Health*

The following post mourns the death of somebody in the Rust community.
This is a very sensitive topic, and it's hard to truly do justice to the loss of human life.
The most important thing we can do is to raise awareness surrounding mental health.

If you or somebody you know is in need of a suicide prevention hotline,
please visit <https://www.opencounseling.com/suicide-hotlines> for emergency and suicide prevention hotline phone numbers in your country.

[Open Source and Mental Health](https://www.redox-os.org/news/open-source-mental-health/)

### Official
* [Inside] [1.53.0 pre-release testing](https://blog.rust-lang.org/inside-rust/2021/06/15/1.53.0-prelease.html)
* [Inside] [Please welcome Boxy, Léo Lanteri Thauvin and the8472 to compiler-contributors](https://blog.rust-lang.org/inside-rust/2021/06/15/boxyuwu-leseulartichaut-the8472-compiler-contributors.html)

### Newsletters
* [This Month in Rust GameDev #22 - May 2021](https://gamedev.rs/news/022/)

### Project/Tooling Updates
* [rust-analyzer Changelog #81](https://rust-analyzer.github.io/thisweek/2021/06/14/changelog-81.html)
* [AWS-SDK-Rust: CloudFormation, SageMaker, EC2, and SES](https://github.com/awslabs/aws-sdk-rust/releases/tag/v0.0.8-alpha)
* [Announcing Aya](https://confused.ai/posts/announcing-aya)
* [This Week In TensorBase 7](https://tensorbase.io/thisweek/2021-06-16-tw_7/)
* [Introducing cap-std, a capability-based version of the Rust standard library](https://blog.sunfishcode.online/introducing-cap-std/)
* [Wasmer 2.0, it's a big deal!](https://wasmer.io/posts/wasmer-2.0)
* [New crate announcement, `iterate`](https://www.reddit.com/r/rust/comments/nwdyip/new_create_announcement_iterate/)
* [ripgrep 13.0.0 release](https://github.com/BurntSushi/ripgrep/releases/tag/13.0.0)

### Observations/Thoughts
* [A few thoughts on Fuchsia security](https://blog.cr0.org/2021/06/a-few-thoughts-on-fuchsia-security.html)
* [On using Rust to implement shell history search/sync (interview with Ellie Huxtable)](https://console.dev/interviews/atuin-ellie-huxtable/)
* [Visualizing memory management in Rust](https://deepu.tech/memory-management-in-rust/)
* [audio] [Using Rust with Flutter](https://youtu.be/iQer8hUu0Es)

### Rust Walkthroughs
* [Embedded-Rust: Build & Flash binary to STM32F3DISCOVERY](https://blog.knoldus.com/embedded-rust-build-flash-binary-to-stm32f3discovery/)
* [Learning to Fly: Let's simulate evolution in Rust! (pt 4)](https://pwy.io/en/posts/learning-to-fly-pt4/)
* [Baby's first Rust with extra steps (XPC, launchd, and FFI)!](https://dstancu.net/macos/rust/ffi/xpc/ncurses/2021/06/15/rust-xpc-ffi.html)
* [Speeding Up the WebCola Graph Viz Library with Rust + WebAssembly](https://cprimozic.net/blog/speeding-up-webcola-with-webassembly/)
* [From NAND to Raytracer: Raytracing on the Hack computer (feat. Rust)](https://blog.alexqua.ch/posts/from-nand-to-raytracer/)
* [Cacao: Building macOS (and iOS) Apps in Rust](https://rymc.io/blog/2021/cacao-rs-macos-ios-rust/)
* [Build an API in Rust (Part 1)](https://dev.to/naruhodo/build-an-api-in-rust-part-1-5c43)
* [Learn to Build a Parser in Rust for Fun and Profit](https://dev.to/pancy/learn-to-build-a-parser-in-rust-for-fun-and-profit-2id5)
* [Beginner's Guide to Running Rust on AWS Lambda](https://dev.to/nicholaschiasson/beginner-s-guide-to-running-rust-on-aws-lambda-277n)
* [A Command Line Key-Value Data Store using the Rust Programming Language.](https://dev.to/sirneij/a-command-line-key-value-data-store-using-the-rust-programming-language-33b6)
* [How to use casbin authorization in your rust web-app (part 3)](https://dev.to/smrpn/how-to-use-casbin-authorization-in-your-rust-web-app-part-3-4g2f)
* [Speeding up incoming message parsing by 3- to 10-times by switching from serde to the nom library](https://medium.com/tezedge/speeding-up-incoming-message-parsing-by-3-to-10-times-by-switching-from-serde-to-the-nom-library-a74b04391bb4)
* [ZH] [series] [Build front-end web apps with Yew and WebAssembly in Rust -part 3: Assets & Refactor (Rust 和 Wasm 的融合，使用 yew 构建 web 前端（3）- 资源文件及小重构)](https://blog.budshome.com/budshome/rust-he-wasm-de-rong-he-,shi-yong-yew-gou-jian-web-qian-duan-(3)--zi-yuan-wen-jian-ji-xiao-zhong-gou)
* [ZH] [series] [Build front-end web apps with Yew and WebAssembly in Rust -part 4: GraphQL data & Parsing (Rust 和 Wasm 的融合，使用 yew 构建 web 前端（4）- 获取 GraphQL 数据并解析)](https://blog.budshome.com/budshome/rust-he-wasm-de-rong-he-,shi-yong-yew-gou-jian-web-qian-duan-(4)--huo-qu-graphql-shu-ju-bing-jie-xi)
* [video] [Building a Web Application with Rust - Part 1 - Configuration](https://youtu.be/Wmq3etdwCbM)
* [video] [Building a Web Application with Rust - Part 2 - Database Setup](https://youtu.be/c9qSGUHgE1c)
* [video] [Lock-Free to Wait-Free Simulation in Rust (part 2)](https://youtu.be/tNzCj8691LE)


### Research

### Miscellaneous
* [Meet the Rustaceans: A series of interviews with Facebook Rust engineers](https://developers.facebook.com/blog/post/2021/06/03/meet-rustaceans-neil-mitchell/)
* [Pop!_OS uses a lot of Rust](https://www.reddit.com/r/rust/comments/nyd1qw/pop_os_uses_a_lot_of_rust/)
* [video] [Keynote: Bryan Cantrill - Hardware/Software Co-design: The Coming Golden Age](https://youtu.be/nY07zWzhyn4)

## Crate of the Week

This week has two crates: [nativeshell](https://github.com/nativeshell/nativeshell) gets you a Flutter app in Rust, while [static-rc](https://github.com/matthieu-m/static-rc) is a compile-time reference-counted smart pointer.

Thanks to [Zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/922) for both nominations

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [boa-dev/boa has some good first issues](https://github.com/boa-dev/boa/labels/good%20first%20issue)
* [softprops/shiplift - We have too many unwrap() calls in the codebase](https://github.com/softprops/shiplift/issues/301)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

289 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-06-07..2021-06-14

* [fix force-warns to allow dashes](https://github.com/rust-lang/rust/pull/86117)
* [suggest a trailing comma if a 1-tuple is expected and a parenthesized expression is found](https://github.com/rust-lang/rust/pull/86116)
* [do not suggest to add type annotations for unnameable types](https://github.com/rust-lang/rust/pull/86215)
* [`to_digit` simplification (less jumps)](https://github.com/rust-lang/rust/pull/85630)
* [multiple improvements to `RwLock`s](https://github.com/rust-lang/rust/pull/84687)
* [add `Ipv6Addr::is_unicast`](https://github.com/rust-lang/rust/pull/85791)
* [stabilize `wasm simd intrinsics`](https://github.com/rust-lang/rust/pull/86204)
* [stabilize `maybe_uninit_ref`](https://github.com/rust-lang/rust/pull/86273)
* [stabilize `simd_x86_bittest`](https://github.com/rust-lang/rust/pull/86233)
* [cargo: implement warning for ignored trailing arguments](https://github.com/rust-lang/cargo/pull/9561)
* [clippy: fix `while_let_on_iterator` suggestion in a closure](https://github.com/rust-lang/rust-clippy/pull/7262)
* [clippy: remove requirement of fully qualified path for `disallowed_method`/`type`](https://github.com/rust-lang/rust-clippy/pull/7345)
* [clippy: fix false positive on `semicolon_if_nothing_returned`](https://github.com/rust-lang/rust-clippy/pull/7326)
* [clippy: fix false positive in `default_numeric_fallback` with external macro expansion](https://github.com/rust-lang/rust-clippy/pull/7325)
* [clippy: `Vec` `extend` to `append`](https://github.com/rust-lang/rust-clippy/pull/7270)
* [BPF target support](https://github.com/rust-lang/rust/pull/79608)
* [support for force-warns](https://github.com/rust-lang/rust/pull/85788)
* [improve debugging experience for enums on windows-msvc](https://github.com/rust-lang/rust/pull/85292)
* [parser: ensure that all nonterminals have tokens after parsing](https://github.com/rust-lang/rust/pull/84995)
* [don't suggest unsized indirection in where-clauses](https://github.com/rust-lang/rust/pull/85979)
* [rustc: allow safe `#[target_feature]` on wasm](https://github.com/rust-lang/rust/pull/84988)
* [always go through the `expn_that_defined` query](https://github.com/rust-lang/rust/pull/86002)
* [perf: miscellaneous inlining improvements](https://github.com/rust-lang/rust/pull/85892)
* [perf: only compute the trait map once](https://github.com/rust-lang/rust/pull/85905)
* [stabilize `vecdeque_binary_search`](https://github.com/rust-lang/rust/pull/83362)
* [update standard library for `IntoIterator` implementation of arrays](https://github.com/rust-lang/rust/pull/85930)
* [clippy: don't warn about `cfg!(..)` as a constant in assertions](https://github.com/rust-lang/rust-clippy/pull/7319)
* [clippy: fix `needless_collect` with binding shadowing](https://github.com/rust-lang/rust-clippy/pull/7289)
* [clippy: add lint `manual_str_repeat`](https://github.com/rust-lang/rust-clippy/pull/7265)

### Rust Compiler Performance Triage


Some good improvements, and a few regressions. No large changes.

Triage done by **@simulacrum**.
Revision range: [1160cf..a50d721](https://perf.rust-lang.org/?start=1160cf864f2a0014e3442367e1b96496bfbeadf4&end=a50d72158e08e02cfc051b863017bdbd2c45b637&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 1 Mixed; 1 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-06-08.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: close] [Add the boxed!() macro to "de-magic" box syntax](https://github.com/rust-lang/rfcs/pull/3057)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize span_open() and span_close().](https://github.com/rust-lang/rust/pull/86136)
* [disposition: merge] [Redefine ErrorKind::Other and stop using it in std.](https://github.com/rust-lang/rust/pull/85746)
* [disposition: merge] [When using process::Command on Windows, environment variable names must be case-preserving but case-insensitive](https://github.com/rust-lang/rust/pull/85270)
* [disposition: merge] [Ignore derived Clone and Debug implementations during dead code analysis](https://github.com/rust-lang/rust/pull/85200)
* [disposition: merge] [Support forwarding caller location through trait object method call](https://github.com/rust-lang/rust/pull/81360)
* [disposition: merge] [Tracking issue for ops::Bound::cloned()](https://github.com/rust-lang/rust/issues/61356)

### New RFCs

* [RFC: fallible-allocation](https://github.com/rust-lang/rfcs/pull/3140)
* [Cargo alternative registry auth](https://github.com/rust-lang/rfcs/pull/3139)

## Upcoming Events

### Online

* [June 17, 2021, Denver, CO, US - Learning Rust as a Python/Javascript developer by Juhis - Rust Denver](https://www.meetup.com/Rust-Boulder-Denver/events/277575285/)
* [June 18, 2021, Online - Learn Rust Fundamentals | Rust 101 - KubeDaily](https://www.youtube.com/watch?v=DIxjk0HTx5U)
* [June 29. 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwryccjbmc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Tweede golf**

* [Lead Developer Embedded Rust (Nijmegen, NL)](https://tweedegolf.nl/vacatures/2/lead-developer-embedded-rust)

**Paige**

* [Senior Software Engineer, Visualization (Remote, Europe)](https://boards.greenhouse.io/paige/jobs/5210311002)

**ChainSafe Systems**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**Gnosis**

* [Ethereum Core Developer (Remote)](https://arbeitnow.com/view/ethereum-core-developer-mwd-gnosis-126355)

**Braiins**

* [Rust Developer (Prague, CZ)](https://braiins.com/careers/rust-developer)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*
ChainSafe Systems 

# Quote of the Week

> If manually managing memory is like wielding a gun, the borrow checker is an automatic safety that prevents you from pulling the trigger when you're roughly pointing it at yourself. But it's coarse-grained and errs on the side of caution; it simulates your foot as as the rectangular box that would contain it, not as a detailed 3D mesh. If you *really* think you can aim it between your toes and avoid hitting yourself (for example, "the value returned by this function must remain alive for no more than 15 successive invocations of this function"), unsafe will let you try, but the borrow checker's built-in rules isn't granular enough to help you, though it will still stop you if you accidentally put your hand in front without declaring it.

– [infogulch on Hacker News](https://news.ycombinator.com/item?id=27468885)

Thanks to [StyMaar](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1056) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/o1l0xo/this_week_in_rust_395/)</small>
