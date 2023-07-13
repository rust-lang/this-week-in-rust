Title: This Week in Rust 503
Number: 503
Date: 2023-07-12
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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
* [Announcing regex 1.9](https://blog.rust-lang.org/2023/07/05/regex-1.9.html)

### Foundation

### Newsletters
* [This Month in Rust OSDev: June 2023](https://rust-osdev.com/this-month/2023-06/)

### Project/Tooling Updates
* [Regex engine internals as a library](https://blog.burntsushi.net/regex-internals/)
* [Bevy 0.11](https://bevyengine.org/news/bevy-0-11/)
* [rustc_codegen_gcc: Progress Report #24](https://blog.antoyo.xyz/rustc_codegen_gcc-progress-report-24)
* [rust-analyzer changelog #189](https://rust-analyzer.github.io/thisweek/2023/07/10/changelog-189.html)

### Observations/Thoughts
* [How to think about `async`/`await` in Rust](https://cliffle.com/blog/async-inversion/)
* [Back-end parallelism in the Rust compiler](https://nnethercote.github.io/2023/07/11/back-end-parallelism-in-the-rust-compiler.html)
* [bridging fuzzing and property testing](https://blog.yoshuawuyts.com/bridging-fuzzing-and-property-testing/)
* [Mastering Rust: 2 Years of Building Brilliance and Lessons Learned](https://opeolluwa.hashnode.dev/mastering-rust-2-years-of-building-brilliance-and-lessons-learned)
* [audio] [Shuttle with Ivan Cernja](https://rustacean-station.org/episode/ivan-cernja/)

### Rust Walkthroughs
* [video] [Build A Full Stack Chatbot in Rust (feat. Leptos & Rustformers)](https://www.youtube.com/watch?v=vAjle3c9Xqc)
* [video] [Render the Julia set in 3 dozen lines of Rust code](https://www.youtube.com/watch?v=g4vN2Z0JuZI)

### Research

### Miscellaneous
* [Docker Desktop 4.21: Support for new Wasm runtimes, Docker Init support for Rust, Docker Scout Dashboard enhancements, Builds view (Beta), and more](https://www.docker.com/blog/docker-desktop-4-21/)

## Crate of the Week

This week's crate is [dysk](https://dystroy.org/dysk/), a nice `df` like utility to display the fill level of your disks.

Thanks to [Denys Séguret](https://users.rust-lang.org/t/crate-of-the-week/2704/1211) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [RustQuant - autodiff module needs re-structure to avoid lifetimes.](https://github.com/avhz/RustQuant/issues/72)
* [Ockam - Add `sqlite` support as an alternative to `lmdb` 2](https://github.com/build-trust/ockam/issues/5321)
* [Ockam - Improve Secure Channel shutdown (make Encryptor and Decryptor shut down each other)](https://github.com/build-trust/ockam/issues/5322)
* [Ockam - Improve Secure Channel shutdown (make Encryptor or Decryptor remove itself from the Registry) #5323](https://github.com/build-trust/ockam/issues/5323)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

378 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-07-03..2023-07-10

* [add basic types to SMIR](https://github.com/rust-lang/rust/pull/113412)
* [add flag for enabling global cache usage for proof trees and printing proof trees on error](https://github.com/rust-lang/rust/pull/113296)
* [add simple markdown formatting to `rustc --explain` output](https://github.com/rust-lang/rust/pull/112697)
* [avoid calling `item_name` for RPITIT](https://github.com/rust-lang/rust/pull/113419)
* [avoid calling `report_forbidden_specialization` for RPITITs](https://github.com/rust-lang/rust/pull/113456)
* [don't ICE in `resolve_bound_vars` when associated return-type bounds are in bad positions](https://github.com/rust-lang/rust/pull/113426)
* [don't ICE for `dyn* Trait: Trait` (built-in object) goals during selection in new trait solver](https://github.com/rust-lang/rust/pull/113395)
* [don't require associated types with Self: Sized bounds in `dyn Trait` objects](https://github.com/rust-lang/rust/pull/112319)
* [effects/keyword generics MVP](https://github.com/rust-lang/rust/pull/113210)
* [fix the issue of wrong diagnosis for `extern pub fn`](https://github.com/rust-lang/rust/pull/113350)
* [implement `ConstEvaluatable` goals in new solver](https://github.com/rust-lang/rust/pull/113324)
* [llvm ffi: Expose `CallInst->setTailCallKind`](https://github.com/rust-lang/rust/pull/112791)
* [make RPITITs assume/require their parent method's predicates](https://github.com/rust-lang/rust/pull/113215)
* [prefer object candidates in new selection](https://github.com/rust-lang/rust/pull/113397)
* [replace RPITIT current impl with new strategy that lowers as a GAT](https://github.com/rust-lang/rust/pull/112988)
* [require TAITs to be mentioned in the signatures of functions that register hidden types for them](https://github.com/rust-lang/rust/pull/112652)
* [suggest importing for partial mod path matching in name resolving](https://github.com/rust-lang/rust/pull/112917)
* [miri: check that assignments do not self-overlap](https://github.com/rust-lang/rust/pull/113441)
* [miri: better error on missing `#[start]`](https://github.com/rust-lang/miri/pull/2963)
* [miri: restore test filtering by substring](https://github.com/rust-lang/miri/pull/2960)
* [miri: test and fix return place alias restrictions](https://github.com/rust-lang/miri/pull/2973)
* [perform `TokenStream` replacement in-place when possible in `expand_macro`](https://github.com/rust-lang/rust/pull/113270)
* [add `Read`, `Write` and `Seek` impls for `Arc<File>` where appropriate](https://github.com/rust-lang/rust/pull/94748)
* [additional `io::copy` specializations](https://github.com/rust-lang/rust/pull/113493)
* [regex: automata/nfa/backtrack: fix memory usage](https://github.com/rust-lang/regex/pull/1028)
* [rewrite the regex crate](https://github.com/rust-lang/regex/pull/978)
* [cargo: add profile strip to config docs](https://github.com/rust-lang/cargo/pull/12337)
* [rustfmt: handle `skip_macro_invocations` from config file](https://github.com/rust-lang/rustfmt/pull/5817)
* [clippy: `[significant_drop_tightening]` consider manual aliases of the `drop` function](https://github.com/rust-lang/rust-clippy/pull/11129)
* [clippy: `arc_with_non_send_sync`: reword and move to `suspicious`](https://github.com/rust-lang/rust-clippy/pull/11104)
* [clippy: `filter_next`: suggest making binding mutable if needed](https://github.com/rust-lang/rust-clippy/pull/11016)
* [clippy: `manual_range_patterns`: lint negative values](https://github.com/rust-lang/rust-clippy/pull/11096)
* [clippy: `unnecessary_literal_unwrap`: don't lint if binding initializer comes from expansion](https://github.com/rust-lang/rust-clippy/pull/11110)
* [clippy: add `needless_pass_by_ref_mut` lint](https://github.com/rust-lang/rust-clippy/pull/10900)
* [clippy: fix ICE in `needless_borrow`](https://github.com/rust-lang/rust-clippy/pull/11130)
* [clippy: fix regex lints for regex 1.9.0](https://github.com/rust-lang/rust-clippy/pull/11111)
* [clippy: new lint `manual_partial_ord_and_ord_impl`](https://github.com/rust-lang/rust-clippy/pull/10788)
* [clippy: new lint: `read_line_without_trim`](https://github.com/rust-lang/rust-clippy/pull/10970)
* [clippy: new lints `manual_is_infinite` and `manual_is_finite`](https://github.com/rust-lang/rust-clippy/pull/11049)
* [clippy: pass correct substs to `implements_trait` in `incorrect_impls`](https://github.com/rust-lang/rust-clippy/pull/11122)
* [rust-analyzer: assist: add `enum` to `glob_import_expand`](https://github.com/rust-lang/rust-analyzer/pull/15226)
* [rust-analyzer: assist: generate trait from impl](https://github.com/rust-lang/rust-analyzer/pull/15152)
* [rust-analyzer: disable remove unnecessary braces diagnotics for self imports](https://github.com/rust-lang/rust-analyzer/pull/15216)
* [rust-analyzer: feature: add a memory layout viewer](https://github.com/rust-lang/rust-analyzer/pull/15081)
* [rust-analyzer: fix `size_of_val` and support `min_align_of_val`](https://github.com/rust-lang/rust-analyzer/pull/15222)
* [rust-analyzer: fix missing terminator in pattern matching of consts](https://github.com/rust-lang/rust-analyzer/pull/15245)
* [rust-analyzer: fix: don't show `unresolved-field` diagnostic for missing names](https://github.com/rust-lang/rust-analyzer/pull/15223)
* [rust-analyzer: fix: indent after pressing enter on a blank line](https://github.com/rust-lang/rust-analyzer/pull/15227)
* [rust-analyzer: implement recursion in mir interpreter without recursion](https://github.com/rust-lang/rust-analyzer/pull/15228)
* [rust-analyzer: recover from missing associated items and generic const defaults](https://github.com/rust-lang/rust-analyzer/pull/15212)
* [rust-analyzer: stop inserting semicolon when extracting match arm](https://github.com/rust-lang/rust-analyzer/pull/15235)
* [rust-analyzer: support GATs in bounds for associated types](https://github.com/rust-lang/rust-analyzer/pull/15211)
* [rust-analyzer: support `read_via_copy` intrinsic](https://github.com/rust-lang/rust-analyzer/pull/15244)
* [rust-analyzer: support getrandom syscall](https://github.com/rust-lang/rust-analyzer/pull/15258)

### Rust Compiler Performance Triage

A very quiet week with nearly no changes in compiler performance. 

Triage done by **@rylev**.
Revision range: [52d8c49..1d4f5af](https://perf.rust-lang.org/?start=52d8c490a3aabe65cdd9f2d3aed95034dd5dbad7&end=1d4f5affbdee00c816f961c227c6b28a3e725ce6&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.1%  | [0.7%, 1.7%]   | 8     |
| Regressions ❌ <br /> (secondary)  | 2.1%  | [0.4%, 3.7%]   | 2     |
| Improvements ✅ <br /> (primary)   | -0.9% | [-1.2%, -0.5%] | 26    |
| Improvements ✅ <br /> (secondary) | -1.2% | [-1.8%, -0.2%] | 16    |
| All ❌✅ (primary)                 | -0.4% | [-1.2%, 1.7%]  | 34    |


4 Regressions, 2 Improvements, 2 Mixed; 1 of them in rollups
51 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-07-11.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Start working on a Rust specification](https://github.com/rust-lang/rfcs/pull/3355)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Implement RefUnwindSafe for Backtrace](https://github.com/rust-lang/rust/pull/100455)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [Add `bf16`, `f64f64` and `f80` types](https://github.com/rust-lang/rfcs/pull/3456)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-07-12 - 2023-08-09 🦀

### Virtual

* 2023-07-11 - 2023-07-13 | Virtual (Europe) | [Mainmatter](https://mainmatter.com/)
    * [**Web-based Services in Rust, 3-day Workshop with Stefan Baumgartner**](https://rust-web-services-workshop.mainmatter.com/)  
* 2023-07-13 - 2023-07-14 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs/)
    * [**Scientific Computing in Rust workshop**](https://scientificcomputing.rs/)
* 2023-07-13 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/294707594/)
* 2023-07-13 | Virtual (Edinburgh, UK) | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**Reasoning about Rust: an introduction to Rustdoc’s JSON format**](https://www.meetup.com/rust-edi/events/293820336/)
* 2023-07-13 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #27**](https://www.meetup.com/rust-noris/events/289732650)
* 2023-07-18 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/zdrzpsyfckbxb/)
* 2023-07-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763486)
* 2023-07-20 | Virtual (Tehran, IR) | [Iran Rust Meetup](https://rust-meetup.ir/)
    * [**Iran Rust Meetup #12 - Ownership and Memory management**](https://rust-meetup.ir/2023/07/20/12th-meetup.html)
* 2023-07-24 | Virtual (Toronto, CA) | [Programming Languages Virtual Meetup](https://www.meetup.com/programming-languages-toronto-meetup/)
    * [**Crafting Interpreters Chapter 18: Types of Values**](https://www.meetup.com/programming-languages-toronto-meetup/events/294616842)
* 2023-07-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfckbhc/)
* 2023-07-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfckbkc/)
* 2023-07-28 | Virtual (Tunis, TN) | [Rust Meetup Tunisia](https://www.meetup.com/rust-tunisia/)
    * [**Rust Meetup Tunisia - Volume I, Number IV**](https://www.meetup.com/rust-tunisia/events/294664236/)
* 2023-08-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfclbcb//)
* 2023-08-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfclblb/)

### Asia

* 2023-07-13 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**A "Bit" of Fun With Geometry in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/294571542)

### Europe

* 2023-07-13 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - beer garden Edition**](https://www.meetup.com/rust-berlin/events/294627419)
* 2023-07-13 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/mstlftyfckbrb/)
* 2023-07-21 | Nuremberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nuremberg Get Together #2**](https://www.meetup.com/rust-noris/events/293823522/)

### North America

* 2023-07-12 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/294373345)
* 2023-07-12 | Waterloo, ON, CA | [Rust KW](https://www.meetup.com/rust-kw/)
    * [**Overengineering FizzBuzz**](https://www.meetup.com/rust-kw/events/294355516/)
* 2023-07-13 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Writing Kubernetes Operators in Rust**](https://www.meetup.com/utah-rust/events/294604589/)
* 2023-07-13 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/294631273)
* 2023-07-13 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**July Meetup**](https://www.meetup.com/seattle-rust-user-group/events/294191599/)
* 2023-07-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfckbxb)

### Oceania

* 2023-07-18 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**July Meetup**](https://www.meetup.com/rust-canberra/events/294321350/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> It's all ducks and sunshine until something starts barking.

– [u/ZZaaaccc on r/rust](https://www.reddit.com/r/rust/comments/14rm1fs/comment/jqup09v)

Thanks to [Patrice Peterson](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1445) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
