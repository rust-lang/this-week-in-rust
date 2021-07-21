Title: This Week in Rust 400
Number: 400
Date: 2021-07-21
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Issue 400!

We are so happy to have reached issue 400 of *This Week in Rust*! To mark this occasion, we would like to introduce you to your editors who put these issues together for you every week!

### Current Editors

**Nell Shamrell-Harrington**

Hello everyone! I'm Nell Shamrell-Harrington ([nellshamrell](https://github.com/nellshamrell/) on GitHub). I've served as lead editor of This Week in Rust for a little over a year now. Currently, I work as a Principal Engineer at Microsoft, prior to that I was on the Rust team at Mozilla. I also am a member of the Rust Foundation Board of Directors. My greatest joy in editing This Week in Rust is seeing how dedicated Rustaceans are to teaching and passing on what they have learned. We are a community where personal maturity and empathy are as important as technical excellence. When I'm not working, I'm often caring for and playing with my three pet bunnies - Lucy, Leia, and Noah!

**Andre Bogus**

Greetings, Rustaceans! I'm Andre 'llogiq' Bogus, and I've been editing TWiR since 2016. I currently work with [synth](https://getsynth.com), my third Job using Rust. I am one of the first clippy maintainers, a mod team member, a [Rust bard](https://twitter.com/llogiq) and I have several crates to my name. I'm always amused with the quotes you folks suggest, and like being on top of the merged PRs, so I know what's coming in the next Rust versions. Besides Rust, I like making music, biking, skateboarding and spending time with my wife, three kids and cat.

**Colton Donnelly**

Good morning to all of you fellow Rustaceans! I'm Colton Donnelly (usually under the screen name cdmistman), and I've been editing TWiR since May 2020. I'm currently a co-op working on the [Alan](https://alan-lang.org) programming language, which uses Rust in the runtime - this is the second time I've had an internship using Rust! I've really enjoyed reading all of your Rust blog posts and articles over the past year (and practicing my speed-reading while I'm at it), it's been awesome seeing how much knowledge y'all like to share. When I'm not coding, I'm usually playing games with friends or binge-watching shows.

### Past Editors

Thank you so much to all who have edited This Week in Rust over the years!

* [emberian](https://github.com/emberian)
* [brson](https://github.com/brson)
* [nasa42](https://github.com/nasa42)
* [Flavsditz](https://github.com/Flavsditz)
* [srikwit](https://github.com/srikwit)
* [mdinger](https://github.com/mdinger)
* [BurntSushi](https://github.com/BurntSushi)

### Thank YOU

And a special thank you to all who have contributed to This Week in Rust and every single one of our subscribers and readers! Here is to many more issues!

## Updates from Rust Community

No newsletters or papers this week.

### Official
* [Rust 2021 public testing period](https://blog.rust-lang.org/2021/07/21/Rust-2021-public-testing.html)

### Project/Tooling Updates
* [IntelliJ Rust Changelog #151](https://intellij-rust.github.io/2021/07/19/changelog-151.html)
* [rust-analyzer Changelog #86](https://rust-analyzer.github.io/thisweek/2021/07/19/changelog-86.html)
* [Announcing tokio-uring: io-uring support for Tokio](https://tokio.rs/blog/2021-07-tokio-uring)
* [Franzplot: a teaching software (re)written in Rust](https://gfx-rs.github.io/stories/franzplot.html)
* [wgpu: release of v0.9 and the future](https://gfx-rs.github.io/2021/07/16/release-0.9-future.html)
* [Allsorts: Font Shaping Engine 0.6 Release](https://yeslogic.com/blog/allsorts-rust-font-shaping-engine-0-6/)
* [This Week In TensorBase 12](https://tensorbase.io/thisweek/2021-07-21-tw_12/)
* [cargo-ui: Introducing cargo-ui, a GUI for cargo](https://sixtyfps.io/blog/introducing-cargo-ui.html)
* [Quickwit: A highly cost-efficient search engine in Rust](https://quickwit.io/blog/quickwit-first-release/)

### Observations/Thoughts
* [Compiling Rust is NP-hard](https://niedzejkob.p4.team/rust-np/)
* [How we improved the performance of our Rust app](https://www.poor.dev/blog/performance/)
* [Making Rust Float Parsing Fast: libcore Edition](https://www.reddit.com/r/rust/comments/omelz4/making_rust_float_parsing_fast_libcore_edition/)
* [Adventures in Rust and Load Balancers](https://bparli.medium.com/adventures-in-rust-and-load-balancers-73a0bc61a192)
* [Faster (and smaller) uploads in Discourse with Rust, WebAssembly and MozJPEG](https://blog.discourse.org/2021/07/faster-user-uploads-on-discourse-with-rust-webassembly-and-mozjpeg)

### Rust Walkthroughs
* [How to implement worker pools in Rust](https://kerkour.com/blog/rust-worker-pool/)
* [Run rust wasm in electron app](https://domtac.github.io/rust/webassembly/electron/2021/07/20/Run-rust-in-electron.html)
* [Host a Wasm module easily on Raspberry Pi Part 2](https://blog.knoldus.com/host-a-wasm-module-easily-on-raspberry-pi-part-2/)
* [Rust and the JVM](https://blog.frankel.ch/start-rust/7/)
* [Smart Pointers in Rust: What, why and how?](https://dev.to/rogertorres/smart-pointers-in-rust-what-why-and-how-oma)
* [Lazy async operations in Rust](https://joshchoo.com/writing/rust-lazy-async-operations)
* [05 - Basic CRUD with rust using tide - front-end with tera](https://javierviola.com/post/05-basic-crud-with-rust-using-tide-front-end-with-tera/)
* [Rust #5: Naming conventions](https://dev.to/cthutu/rust-5-naming-conventions-3cjf)
* [Leader election in rust the journey towards implementing nun-db leader election](https://mateusfreira.github.io/@mateusfreira-leader-election-rust-the-journey-towards-nun-db-leader-election-implementation/)
* [Novel way to Develop, Test, and Document C libraries from Rust](https://wasmer.io/posts/novel-way-to-develop--test-and-document-c-libraries-from-rust)
* [Rust + Tauri + Svelte Tutorial](https://jbarszczewski.com/rust-tauri-svelte-tutorial)
* [Static Integer Types](https://tratt.net/laurie/blog/entries/static_integer_types.html)
* [Serializing data faster](https://devblog.arcana.rs/serializing-data-faster)
* [series] [Basic CRUD api with Rust and Tide](https://dev.to/pepoviola/series/13592)
* [series] Implementing ICMP in Rust
  * [I. Implementing ICMP in Rust](https://dev.to/xphoniex/i-implementing-icmp-in-rust-296o)
  * [II. Implementing ICMP in Rust](https://dev.to/xphoniex/ii-implementing-icmp-in-rust-3bk5)
* [series] [video] [Explaining rust-analyzer](https://youtube.com/playlist?list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y)

### Miscellaneous
* [A GPIO driver in Rust](https://lwn.net/Articles/863459/)
* [Computer Scientist proves safety claims of the programming language Rust](https://www.eurekalert.org/pub_releases/2021-07/su-cs071521.php)
* [JetBrains The State of Developer Ecosystem 2021: Rust](https://www.reddit.com/r/rust/comments/olqarw/jetbrains_the_state_of_developer_ecosystem_2021/)

## Crate of the Week

This week's crate is [dylint](https://github.com/trailofbits/dylint), a tool for running Rust lints from dynamic libraries.

Thanks to [George Hahn](https://users.rust-lang.org/t/crate-of-the-week/2704/938) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Knurling-rs user survey](https://forms.office.com/r/aMfHG79N9K)
* [Rust.Tokyo 2021](https://www.papercall.io/rusttokyo2021)
* [Helix editor icon/logo brainstorm](https://github.com/helix-editor/helix/issues/283)
* [Ockam welcomes new contributors!](https://github.com/ockam-network/ockam/discussions/1081)
* [Forest - Cleanup net peers output](https://github.com/ChainSafe/forest/issues/1184)
* [Forest - FOREST_CONFIG_PATH env var](https://github.com/ChainSafe/forest/issues/1191)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from Rust Core

280 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-07-12..2021-07-19

* [handle non-integer const generic parameters in debuginfo type names](https://github.com/rust-lang/rust/pull/87082)
* [warn about useless assignments of variables/fields to themselves](https://github.com/rust-lang/rust/pull/87129)
* [suggest a path separator if a stray colon is found in a match arm](https://github.com/rust-lang/rust/pull/87101)
* [add diagnostics for mistyped inclusive range](https://github.com/rust-lang/rust/pull/87071)
* [various diagnostics clean ups/tweaks](https://github.com/rust-lang/rust/pull/87225)
* [compute a better `lint_node_id` during expansion](https://github.com/rust-lang/rust/pull/87146)
* [TAIT: infer all inference variables in opaque type substitutions via `InferCx`](https://github.com/rust-lang/rust/pull/87200)
* [remove refs from `Pat` slices](https://github.com/rust-lang/rust/pull/87140)
* [shrink the `CrateStore` dynamic interface](https://github.com/rust-lang/rust/pull/87117)
* [loop over all opaque types instead of looking at just the first one with the same DefId](https://github.com/rust-lang/rust/pull/87107)
* [cache expansion hash globally](https://github.com/rust-lang/rust/pull/87044)
* [perf: noise and variance](https://github.com/rust-lang/rustc-perf/pull/902)
* [some perf optimizations and logging](https://github.com/rust-lang/rust/pull/87203)
* [update Rust Float-Parsing to use the Eisel-Lemire algorithm](https://github.com/rust-lang/rust/pull/86761)
* [stabilize `[T; N]::map(_)`](https://github.com/rust-lang/rust/pull/87174)
* [split `MaybeUninit::write' into new feature gate and stabilize it](https://github.com/rust-lang/rust/pull/86344)
* [mark Option::insert as `must_use`](https://github.com/rust-lang/rust/pull/87196)
* [added `Arc::try_pin`](https://github.com/rust-lang/rust/pull/85579)
* [hashbrown: replace some custom unsafe code with `array::map`](https://github.com/rust-lang/hashbrown/pull/281)
* [hashbrown: optimize `find`](https://github.com/rust-lang/hashbrown/pull/279)
* [cargo: deduplicate compiler diagnostics](https://github.com/rust-lang/cargo/pull/9675)
* [cargo: add `d` as an alias for doc](https://github.com/rust-lang/cargo/pull/9680)
* [clippy: fix false positives and document `branches_sharing_code` lint](https://github.com/rust-lang/rust-clippy/pull/7462)
* [clippy: new lint: `self_named_constructor`](https://github.com/rust-lang/rust-clippy/pull/7403)
* [clippy: add `Arc` to `redundant_allocation`](https://github.com/rust-lang/rust-clippy/pull/7308)
* [clippy: fix ICE in `is_integer_const`](https://github.com/rust-lang/rust-clippy/pull/7473)

### Rust Compiler Performance Triage

Mostly quiet week; improvements outweighed regressions.

Triage done by **@simulacrum**.
Revision range: [9a27044f4..5aff6dd](https://perf.rust-lang.org/?start=9a27044f42ace9eb652781b53f598e25d4e7e918&end=5aff6dd07a562a2cba3c57fc3460a72acb6bef46&absolute=false&stat=instructions%3Au)

1 Regressions, 4 Improvements, 0 Mixed; 0 of them in rollups

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2021-07-13.md).

### Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: let-else statements](https://github.com/rust-lang/rfcs/pull/3137)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Stabilize Cargo's weak-dep-features and namespaced-features.](https://github.com/rust-lang/rfcs/pull/3143)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize arbitrary_enum_discriminant](https://github.com/rust-lang/rust/pull/86860)
* [disposition: close] [regression: infallible residual could not convert error](https://github.com/rust-lang/rust/issues/86831)
* [disposition: merge] [Allow reifying intrinsics to fn pointers.](https://github.com/rust-lang/rust/pull/86699)
* [disposition: merge] [Commit to not supporting IPv4-in-IPv6 addresses](https://github.com/rust-lang/rust/pull/86335)
* [disposition: merge] [Stabilize core::task::if_ready!](https://github.com/rust-lang/rust/pull/81050)
* [disposition: close] [Implement RFC 2500 Needle API (Part 1)](https://github.com/rust-lang/rust/pull/76901)

### New RFCs

* [Scoped threads in the standard library, take 2](https://github.com/rust-lang/rfcs/pull/3151)

## Upcoming Events

### Online

* [July 21, 2021, Vancouver, BC, CA - Rust Adoption at Huawei - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/zkqvjsycckbcc/)
* [July 22, 2021, Tokyo, JP - Rust LT Online#4 - Rust JP](https://www.youtube.com/watch?v=oK0iJz7XF3Y)
* [July 22, 2021, Berlin, DE - Rust Hack and Learn - Berline.rs](https://berline.rs/)
* [July 27, 2021, Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrycckbkc/)
* [August 3, 2021, Buffalo, NY, US - Buffalo Rust User Group, First Tuesdays - Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/jxfdjsycclbfb/)

### North America

* [July 27, 2021, Chicago, IL, US - Rust in production at Tempus - Chicago Rust Meetup](https://www.meetup.com/Chicago-Rust-Meetup/events/279131036)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**The Tor Project**

* [Software Developer, Rust](https://www.torproject.org/about/jobs/rust-dev/)

**Anixe**

* [Architect/Senior Rust Software Engineer (Wrocław, PL)](https://anixe.bamboohr.com/jobs/view.php?id=154)
* [Junior Rust Software Engineer (Wrocław, PL)](https://anixe.bamboohr.com/jobs/view.php?id=152)

**Snapview**

* [(Senior) Rust Backend Engineer (m/f/d) (Remote)](https://snapview.jobs.personio.de/job/381815)

**Luminovo**

* [(Senior) Software Engineer - Rust (m/f/d) (Munich, DE / Remote)](https://arbeitnow.com/view/senior-software-engineer-rust-mfd-luminovo-gmbh-55556)

**Clear**

* [Rust Engineer (Remote)](https://docs.google.com/document/d/1OuG5Ts_6s4eWO6CXGzcbklOweD7qGnOgADnSoPjEa10/edit)

**ChainSafe**

* [Rust Developer (Remote)](https://jobs.smartrecruiters.com/ChainSafeSystemsInc/743999739358248-rust-developer)

**PolarFox Network**

* [Senior Rust Engineer (Remote)](https://jro-ventures.breezy.hr/p/0c1d3630d39d)

**CNRS**

* [Rust Software Engineer in AI research applied to Robotics (Toulouse, FR)](https://emploi.cnrs.fr/Offres/CDD/UPR8001-ARTBIT-004/Default.aspx?lang=EN)

**The Mobility House GmbH**

* [Senior Software Engineer - Rust m/f/d (Remote / Germany)](https://germantechjobs.de/jobs/The-Mobility-House-GmbH-Senior-Software-Engineer-mfd)

**Immunant**

* [Systems Programmer/Rustacean (Optionally Remote)](https://immunant.com/jobs/)

**Wingback**

* [Senior Backend Developer - Rust 🦀 (Fully Remote)](https://careers.wingback.app/o/senior-backend-developer-rust-remote)

**Modeldrive**

* [Tech Lead (London / Remote)](https://www.modeldrive.com/jobs)
* [Senior Backend Engineer (London / Remote)](https://www.modeldrive.com/jobs)

**NZXT**

* [Senior Software Engineer for CAM (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=259)
* [Senior Software Engineer for Streaming Software (Remote)](https://nzxt.bamboohr.com/jobs/view.php?id=317)

**Kollider**

* [Junior Backend Engineer (Remote)](https://kollider.homerun.co/junior-backend-engineer/en)
* [Senior Backend Engineer (Remote)](https://kollider.homerun.co/senior-backend-engineer/en)
* [DevOps Engineer (Remote)](https://kollider.homerun.co/devops-engineer/en)

**Tempus Ex**

* [Several positions available (San Francisco, Atlanta, Austin, and Remote)](https://tempus-ex.com/careers?r=twir)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Tip: whenever you wonder if Pin could be the solution, it isn't

– [@SkiFire13 on the official Rust Discord](https://discord.com/channels/442252698964721669/448238009733742612/866312170890330122)

Thanks to [Kestrer](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1071) for the self-suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
