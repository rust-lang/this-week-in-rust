Title: This Week in Rust 472
Number: 472
Date: 2022-12-07
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Foundation

### Newsletters

### Project/Tooling Updates
* [What's new in Seaography 0.3.0](https://www.sea-ql.org/blog/2022-12-02-whats-new-in-seaography-0.3.0/)
* [Fornjot (code-first CAD in Rust) - Weekly Release](https://www.fornjot.app/blog/weekly-release/2022-w49/)

* [Helix editor 22.12 released](https://helix-editor.com/news/release-22-12-highlights/)

### Observations/Thoughts

* [What Every Rust Developer Should Know About Macro Support in IDEs](https://blog.jetbrains.com/rust/2022/12/05/what-every-rust-developer-should-know-about-macro-support-in-ides/)

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [zeroize](https://crates.io/crates/zeroize), a crate to securely clear secrets from memory either manually or on drop, with both methods for your types being but one `#[derive]` away.

Thanks to [Tally](https://users.rust-lang.org/t/crate-of-the-week/2704/1132) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

339 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-11-28..2022-12-05

* [remove drop order twist of && and || and make them associative](https://github.com/rust-lang/rust/pull/103293)
* [`rustc_ast_lowering: `stop lowering imports into multiple items](https://github.com/rust-lang/rust/pull/104963)
* [avoid Invalid code suggested when encountering unsatisfied trait bounds in derive macro code](https://github.com/rust-lang/rust/pull/104895)
* [don't add a note for implementing a trait if its inner type is erroneous](https://github.com/rust-lang/rust/pull/105181)
* [don't elide type information when printing E0308 with `-Zverbose`](https://github.com/rust-lang/rust/pull/105188)
* [don't suggest associated function call for associated const](https://github.com/rust-lang/rust/pull/104856)
* [restore `use` suggestion for `dyn` method call requiring `Sized`](https://github.com/rust-lang/rust/pull/105164)
* [some initial normalization method changes](https://github.com/rust-lang/rust/pull/104905)
* [suggest to use . instead of `:: ` when accessing a method of an object](https://github.com/rust-lang/rust/pull/101975)
* [implement `should_continue` in chalk-recursive](https://github.com/rust-lang/chalk/pull/774)
* [miri: refactor scheduler](https://github.com/rust-lang/miri/pull/2699)
* [rewrite LLVM's archive writer in Rust](https://github.com/rust-lang/rust/pull/97485)
* [cheaper `dump_mir`](https://github.com/rust-lang/rust/pull/105121)
* [allow to feed a value in another query's cache](https://github.com/rust-lang/rust/pull/104940)
* [attribute cleanups](https://github.com/rust-lang/rust/pull/104861)
* [also cache the stable hash of interned Predicates](https://github.com/rust-lang/rust/pull/94487)
* [merge generics and where predicates and prevent duplicates in where predicates](https://github.com/rust-lang/rust/pull/105183)
* [adjust inlining attributes around `panic_immediate_abort`](https://github.com/rust-lang/rust/pull/104999)
* [ensure query backtraces work for `DefId`s created after ast lowering](https://github.com/rust-lang/rust/pull/105133)
* [add `type_ascribe!` macro as placeholder syntax for type ascription](https://github.com/rust-lang/rust/pull/104614)
* [stabilize `nonzero_bits`](https://github.com/rust-lang/rust/pull/101514)
* [make `VecDeque::new_in` unstably const](https://github.com/rust-lang/rust/pull/105126)
* [send `VecDeque::from_iter` via `Vec::from_iter`](https://github.com/rust-lang/rust/pull/105046)
* [add `PathBuf::as_mut_os_string` and `Path::as_mut_os_str`](https://github.com/rust-lang/rust/pull/105002)
* [implement TcpStream shutdown for wasm32-wasi](https://github.com/rust-lang/rust/pull/104811)
* [create a hacky fail-fast mode that stops tests at the first failure](https://github.com/rust-lang/rust/pull/105153)
* [cargo: aware of compression ratio for unpack size limit](https://github.com/rust-lang/cargo/pull/11337)
* [cargo: improve file found in multiple build targets warning](https://github.com/rust-lang/cargo/pull/11299)
* [rustdoc-Json: don't inline foreign traits](https://github.com/rust-lang/rust/pull/105182)
* [clippy: `comparison_to_empty`: peel derefs in suggestions](https://github.com/rust-lang/rust-clippy/pull/9962)
* [clippy: `manual_let_else: `keep macro call on suggestion blocks](https://github.com/rust-lang/rust-clippy/pull/9943)
* [clippy: don't cross contexts while building the suggestion for `redundant_closure_call`](https://github.com/rust-lang/rust-clippy/pull/9987)
* [clippy: don't lint `explicit_auto_deref` when the initial type is neither a reference, nor a receiver](https://github.com/rust-lang/rust-clippy/pull/9997)
* [clippy: don't lint `from_over_into` for opaque types](https://github.com/rust-lang/rust-clippy/pull/9982)
* [clippy: don't lint `implicit_clone` when the type doesn't implement clone](https://github.com/rust-lang/rust-clippy/pull/10022)
* [clippy: don't lint `manual_assert` in `else if`](https://github.com/rust-lang/rust-clippy/pull/10013)
* [clippy: don't lint `string_lit_as_bytes` in match scrutinees](https://github.com/rust-lang/rust-clippy/pull/10012)
* [clippy: don't lint `unnecessary_cast` in mixed macro context](https://github.com/rust-lang/rust-clippy/pull/9980)
* [clippy: don't lint `unnecessary_operation` in mixed macro contexts](https://github.com/rust-lang/rust-clippy/pull/9981)
* [clippy: don't suggest removing `mut` from references in `redundant_static_lifetimes`](https://github.com/rust-lang/rust-clippy/pull/10006)
* [clippy: fix ICE in `unnecessary_to_owned`](https://github.com/rust-lang/rust-clippy/pull/10027)
* [clippy: fix ICE in `result_large_err` with uninhabited enums](https://github.com/rust-lang/rust-clippy/pull/10007)
* [clippy: fix ICE in `unused_rounding`](https://github.com/rust-lang/rust-clippy/pull/10001)
* [clippy: fix `manual_let_else` producing a wrong suggestion with or-patterns](https://github.com/rust-lang/rust-clippy/pull/9966)
* [clippy: fix `unnecessary_cast` suggestion when taking a reference](https://github.com/rust-lang/rust-clippy/pull/9996)
* [clippy: move `index_refutable_slice` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/9975)
* [clippy: move `unnecessary_unsafety_doc` to `pedantic`](https://github.com/rust-lang/rust-clippy/pull/9989)
* [rust-analyzer: add `move_const_to_impl` assist](https://github.com/rust-lang/rust-analyzer/pull/13707)
* [rust-analyzer: implement vararg parameter type inference](https://github.com/rust-lang/rust-analyzer/pull/13690)
* [rust-analyzer: fix signature help not showing up when cursor is between `))` or `>>`](https://github.com/rust-lang/rust-analyzer/pull/13696)

### Rust Compiler Performance Triage

A mixed bag of a week. 2 of the 3 regressions are connected to changes to the
query system that underlies incremental compilation. The third regression is
still being investigated. For all three, the impact on cycle counts that are
within our noise tolerance levels There were several small-ish improvements,
though PR #104963 is worth calling out: a well-factored change to
how we lower the AST into HIR that had a broad positive impact. One final
note: the summary-opt max-rss seems to gone up by 1.5% over the past month
([perf](https://perf.rust-lang.org/?start=2022-11-06&end=2022-12-06&kind=percentfromfirst&stat=max-rss)),
via a gradual climb; just something to keep our eyes on going forward.


Triage done by **@pnkfelix**.
Revision range: [8a09420a..9db224fc](https://perf.rust-lang.org/?start=8a09420ac48658cad726e0a6997687ceac4151e3&end=9db224fc908059986c179fc6ec433944e9cfce50&absolute=false&stat=instructions%3Au)

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-12-06.md) for details.

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-12-07 - 2023-01-04 🦀

### Virtual

* 2022-11-30 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Common crates and their usage**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289645553/)
* 2022-11-30 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)
* 2022-12-01 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Exploring USB with Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/289563986/)
* 2022-12-01 | Virtual (Lehi, UT, US) | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Beginner Projects and Shop Talk with Food!**](https://www.meetup.com/utah-rust/events/289899804/)
* 2022-12-01 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Getting Started with Rust: Understanding Rust Compile Errors – Part 2**](https://www.meetup.com/microsoft-reactor-redmond/events/289830539/) 
* 2022-12-06 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/12/06/rust-hack-and-learn.html)
* 2022-12-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcqbjb/)
* 2022-12-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/287027660/)
* 2022-12-07 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcqbkb/)
* 2022-12-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #20**](https://www.meetup.com/rust-noris/events/hlvbvsydcqblb/)
* 2022-12-08 | Virtual (San Francisco, CA, US) | [Data + AI Online Meetup](https://www.meetup.com/data-ai-online/)
    * [**D3L2: The Genesis of Delta Rust with QP Hou**](https://www.meetup.com/data-ai-online/events/289672886/)
* 2022-12-10 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2022-12-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcqbrb/)
* 2022-12-13 | Virtual (Rostock, DE) | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/289352426/)
* 2022-12-13 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 25u16**](https://www.meetup.com/rust-saar/events/289663288/)
* 2022-12-14 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcqbsb/)
* 2022-12-14 | Virtual (México City, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Rust y Arduino**](https://www.meetup.com/rust-mx/events/289973784/)
* 2022-12-15 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcqbtb/)
* 2022-12-20 | Virtual (Berlin, DE) | [Berlin.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://berline.rs/2022/12/20/rust-hack-and-learn.html)
* 2022-12-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/vdhxgsydcqbbc/)
* 2022-12-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcqbcc/)
* 2022-12-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcqbkc/)

### Europe

* 2022-11-30 | Amsterdam, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Rust in Critical Infrastructure**](https://www.meetup.com/rust-nederland/events/289204146/)
* 2022-11-30 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet)
    * [**Rust Lille #1**](https://www.meetup.com/meetup-group-zgphbyet/events/289620614/)
* 2022-11-30 | Milan, IT | [Rust Language Milano](https://www.meetup.com/rust-language-milano/)
    * [**Welcome GAT!!**](https://www.meetup.com/rust-language-milano/events/289767176/)
* 2022-11-30 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2022 / 3 - hybrid**](https://www.meetup.com/rust-munich/events/289065390/)
* 2022-11-30 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #54**](https://www.meetup.com/rust-paris/events/289833645/)
* 2022-12-01 | Edinburgh, UK | [Rust Edinburgh](https://www.meetup.com/rust-edi/)
    * [**December Talks + Rust Book Raffle**](https://www.meetup.com/rust-edi/events/289582990/)
* 2022-12-01 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #30**](https://www.meetup.com/rust-wroclaw/events/289884642/)
* 2022-12-06 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Go X Rust: A Very Scalable Christmas Party**](https://www.meetup.com/rust-london-user-group/events/289909563/)
* 2022-12-07 | Zurich, CH | [Rust Zurich](https://www.meetup.com/Rust-Zurich/)
    * [**Next generation i18n with rust (icu4x) and zero-copy deserialization**](https://www.meetup.com/rust-zurich/events/289518586/)
* 2022-12-12 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Rust Meetup - Subject TBA**](https://www.meetup.com/dutch-rust-meetup/events/289021643/)
* 2022-12-15 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsydcqbvb/)

### North America

* 2022-12-01 | Lehi, UT, US + Virtual | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Beginner Projects and Shop Talk with Food!**](https://www.meetup.com/utah-rust/events/289899804/)
* 2022-12-08 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/events/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcqblb/)
* 2022-12-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcqbbc/)

### Oceania

* 2022-12-09 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne)
    * [**December 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/290037796/)

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

> To date, there have been zero memory safety vulnerabilities discovered in Android’s Rust code.

– [Jeffrey Vander Stoep on the google security team blog](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1335) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
