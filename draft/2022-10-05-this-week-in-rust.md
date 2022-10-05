Title: This Week in Rust 463
Number: 463
Date: 2022-10-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official
* [Announcing the Rust Style Team](https://blog.rust-lang.org/inside-rust/2022/09/29/announcing-the-rust-style-team.html)

### Foundation
* [Rust Foundation Project Grants are open for applications](https://foundation.rust-lang.org/news/2022-10-03-project-grants-open-for-applications/)

### Newsletters

### Project/Tooling Updates
* [cargo careful: run your Rust code with extra careful debug checking](https://www.ralfj.de/blog/2022/09/26/cargo-careful.html)
* [Async UI: a Rust UI Library where Everything is a Future](https://wishawa.github.io/posts/async-ui-intro/)
* [rust-analyzer changelog #149](https://rust-analyzer.github.io/thisweek/2022/10/03/changelog-149.html)

### Observations/Thoughts
* [How (and why) nextest uses tokio, part 1](https://sunshowers.io/posts/nextest-and-tokio-1/)
* [in-place constructors](https://y86-dev.github.io/blog/safe-pinned-initialization/in-place.html)
* [Quirks of Rust’s token representation](https://nnethercote.github.io/2022/10/05/quirks-of-rusts-token-representation.html)
* [Brute forcing protected ZIP archives in Rust](https://agourlay.github.io/brute-forcing-protected-zip-rust/)
* [This week in Fluvio #47: The programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0047/)

### Rust Walkthroughs
* [How to call a C function from Rust (A simple FFI tutorial)](https://github.com/vanjacosic/rust-ffi-to-c)
* [Rewriting the Modern Web in Rust](https://implfuture.dev/blog/rewriting-the-modern-web-in-rust)
* [Implementing truly safe semaphores in rust](https://neosmart.net/blog/2022/implementing-truly-safe-semaphores-in-rust/)
* [Model an ALU in Rust](https://www.superperfundo.tech/articles/alu-model)
* [6 things you can do with the Cow 🐄 in Rust 🦀](https://dev.to/kgrech/6-things-you-can-do-with-the-cow-in-rust-4l55)
* [Platform Agnostic Drivers in Rust: MAX7219 Naive Code Refactoring](https://apollolabsblog.hashnode.dev/platform-agnostic-drivers-in-rust-max7219-naive-code-refactoring)
* [Last mile DynamoDB: Deno Deploy edition](https://artofserverless.com/dynamodb-deno-deploy/)

### Research

### Miscellaneous
* [The Initial Rust Infrastructure Has Been Merged Into Linux 6.1](https://www.phoronix.com/news/Rust-Is-Merged-Linux-6.1)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

A great week, with 170 primary benchmark scenarios seeing improvement. Every PR
flagged by perf provided at least some wins, and perhaps more impressive: No
rollup PR's were flagged by perf this week! Furthermore, cjgillot fixed an issue
where incremental compilation was being unnecessarily hindered by our span and
lint system. Great work everyone!

Triage done by **@pnkfelix**.
Revision range: [d9297d22..02cd79af](https://perf.rust-lang.org/?start=d9297d22ad9edc2b56f0dd8734c1187a0c88be69&end=02cd79afb8080fce8c8ce35533c54d8ecf8f390e&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-10-04.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [make const_err a hard error](https://github.com/rust-lang/rust/pull/102091)
* [disposition: merge] [Elaborate supertrait bounds when triggering unused_`must_use` on `impl Trait`](https://github.com/rust-lang/rust/pull/102287)
* [disposition: merge] [Stabilize proc_macro Span::source_text](https://github.com/rust-lang/rust/issues/101991)
* [disposition: merge] [`const`-stablilize `NonNull::as_ref`](https://github.com/rust-lang/rust/pull/102198)
* [disposition: merge] [Add documentation about the memory layout of `UnsafeCell<T>`](https://github.com/rust-lang/rust/pull/101717)
* [disposition: merge] [Handle projections as uncovered types during coherence check](https://github.com/rust-lang/rust/pull/100555)
* [disposition: merge] [Never panic in `thread::park` and `thread::park_timeout`](https://github.com/rust-lang/rust/pull/102412)
* [disposition: merge] [Stabilize `nonzero_bits`](https://github.com/rust-lang/rust/pull/101514)
* [disposition: merge] [`EscapeAscii` is not an `ExactSizeIterator`](https://github.com/rust-lang/rust/pull/99880)
* [disposition: merge] [Change default level of INVALID_HTML_TAGS to warning and stabilize it](https://github.com/rust-lang/rust/pull/101720)
* [disposition: merge] [Add `Box<[T; N]>: TryFrom<Vec<T>>`](https://github.com/rust-lang/rust/pull/101837)
* [disposition: merge] [add `no_compile` doctest attribute](https://github.com/rust-lang/rust/pull/96573)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-10-05 - 2022-11-02 🦀

### Virtual

* 2022-09-28 | Virtual (London, UK) | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-30 | Virtual (Minneapolis, MN, US) | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/minneapolis-rust-meetup/events/288601893/)
* 2022-10-04 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydcnbgb/)
* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)
* 2022-10-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #18**](https://www.meetup.com/rust-noris/events/hlvbvsydcnbrb/)
* 2022-10-08 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-10-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcnbpb/)
* 2022-10-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-12 | Virtual (Erlangen, DE) | [Rust Franken](https://www.meetup.com/rust-nerf/)
    * [**Rust Franken Meetup #4**](https://www.meetup.com/rust-nerf/events/288723552/)
* 2022-10-12 | Virtual (San Francisco, CA, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/)
* 2022-10-13 | Virtual (Berlin, DE) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-15 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 2 (CuteCopter): Reverse Engineering a tiny drone**](https://www.meetup.com/rust-noris/events/287347851/)
* 2022-10-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Impractical Rust: The HATETRIS World Record**](https://www.meetup.com/rustdc/events/vdhxgsydcnbxb/)
* 2022-10-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcnbzb/)
* 2022-10-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcnbbc/)
* 2022-10-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcnbhc/)

### Asia

* 2022-10-11 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Cost-Efficient Rust in Practice**](https://www.meetup.com/tokyo-rust-meetup/events/288597490/)

### Europe

* 2022-09-28 | London, UK + Virtual | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**Rust (Hybrid) Hack & Learn September 2022**](https://www.meetup.com/rust-london-user-group/events/288436078/)
* 2022-09-29 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/288266277/)
* 2022-09-29 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #29**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179100/)
* 2022-09-29 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Going full stack on Rust**](https://www.meetup.com/dutch-rust-meetup/events/286727064/)
* 2022-09-30 | Berlin, DE | [RustFi Hackathon](https://rustfi.keyrock.com/)
    * [**RustFi Hackathon 30 Sept - 2 Oct**](https://rustfi.keyrock.com/)
* 2022-10-02 | Florence, IT + Virtual | [RustLab](https://rustlab.it/)
    * [**RustLab Conference 2022 (Oct 2-4)**](https://rustlab.it/schedule/)
* 2022-10-03 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @Microsoft Reactor**](https://www.meetup.com/Stockholm-Rust/events/288453360/)
* 2022-10-04 | Helsinki, FI | [Finland Rust Meetup](https://www.meetup.com/Finland-Rust-Meetup/)
    * [**October meetup**](https://www.meetup.com/Finland-Rust-Meetup/events/288724370/)
* 2022-10-06 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #29**](https://www.meetup.com/rust-wroclaw/events/288667400/)
* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)
* 2022-10-13 | Berlin, DE + Virtual | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-18 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #53**](https://www.meetup.com/rust-paris/events/288735204/)

### North America

* 2022-09-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/288590318/)
* 2022-09-29 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Zola o como la comunidad de RustMX tiene página**](https://www.meetup.com/rust-mx/events/288388973/)
* 2022-10-13 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcnbrb/)
* 2022-10-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcnbxb/)
* 2022-10-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Anyhow ? Turbofish ::<> / HTTP calls and errors in Rust.**](https://www.meetup.com/rust-nyc/events/288756215/)
* 2022-10-20 | New York, NY, US | [Cloud Native New York](https://www.meetup.com/cloud-native-new-york/)
    * [**Cloud-native Search Engine for Log Management and Analytics.**](https://www.meetup.com/cloud-native-new-york/events/288818963/)
* 2022-10-25 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**Rust DHCP**](https://www.meetup.com/rust-toronto/events/288589539/)

### Oceania

* 2022-10-10 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust Lightning Talks**](https://www.meetup.com/rust-sydney/events/288746516/)
* 2022-10-20 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Tune Up Edition: software engineering management**](https://www.meetup.com/rust-wellington/events/288738684/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
