Title: This Week in Rust 677
Number: 677
Date: 2026-11-11
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

* [Google Summer of Code 2026 results](https://blog.rust-lang.org/2026/11/11/gsoc-2026-results)

### Newsletters

* [The Embedded Rustacean Issue #84](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-84)

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

* [Securing Rust's supply chain with an extended standard library](https://kerkour.com/rust-supply-chain-security-stdx)
* [Run Linux applications in hypervisor-isolated micro-VMs with Hyperlight](https://www.cncf.io/blog/2026/11/09/run-linux-applications-in-hypervisor-isolated-micro-vms-with-hyperlight/)
* [series] [Apache Iggy's migration journey to clustering powered by Viewstamped Replication (part 3)](https://iggy.apache.org/blogs/2026/10/27/vsr-clustering-part-3/)
* [I snuck into the Rust Foundation's secret meeting; here's what you need to know.](https://rust-lang.zulipchat.com/#narrow/channel/122651-general/topic/I.20snuck.20into.20the.20Rust.20Foundation.27s.20secret.20meeting)
* [Breaking a Rust actor cluster on purpose, and what it taught us](https://heikoseeberger.de/2026-11-09-chaos-verifier/)
* [Catching Panics in Production](https://xd009642.github.io/2026/11/10/Catching-Panics-in-Production/)
* [Skipping Cloud Vendor SDKs: How ScopeDB Accesses Object Storage with Apache OpenDAL](https://opendal.apache.org/blog/2026/11/11/scopedb-on-opendal/)
* [Bringing Rust into Ubuntu at Scale: Interview with Jon Seager](https://serokell.io/blog/ubuntu-rust-coreutils-jon-seager)
* [Rust Benchmarking: How to Detect Performance Regressions in CI](https://hotpath.rs/blog/rust-performance-benchmarking)
* [video] [Code Matters: Reflections on Rust in the LLM Age — Bryan Cantrill | Rust NYC](https://www.youtube.com/watch?v=5D7QPOIZ7gY)
* [A Critique of GPUI](https://blog.zortax.de/posts/a-critique-of-gpui)
* [Memory Safety Bugs in Rust Standard Library](https://forge-blog.github.io/posts/rust-std-memory-safety/)
* [Rust Declarative Macros: Organization, Scoping, and Common Use-Cases](https://sigseis.dev/articles/2026/11/10)

### Rust Walkthroughs

* [RSPHP Part 1: Bindings](https://scherzer.dev/Blog/20261106-rsphp-bindings)
* [Pointer Provenance in Rust vs C++: Pointers Are Not Just Glorified Ints](https://sofiabelen.github.io/projects/pointer-provenance-in-rust-vs-cpp/)
* [Tokio, channels and poisoned locks](https://www.reymom.xyz/blog/rust/2026-11-06-tokio-channels-and-poisoned-locks)
* [video] [RustCurious lesson 12: Lifetimes](https://www.youtube.com/watch?v=h3okIDxOZIA)

## Crate of the Week

This week's crate is [iddqd](https://docs.rs/iddqd), a library of maps where keys are borrowed from values.

Thanks to [Miro](https://users.rust-lang.org/t/crate-of-the-week/2704/1715) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
* [Klicky - Cover silence and signed sample boundaries in audio-normalization tests](https://github.com/Muneer320/klicky/issues/116)
* [Octomind - `parallel = "true"` in a workflow step fails with "missing field `role`"](https://github.com/Muvon/octomind/issues/296)
* [Octomind - `schedule` accepts `when="13am"` and fires at 13:00](https://github.com/Muvon/octomind/issues/295)
* [Octomind - `octomind configure` saves a pasted API key with spaces around it](https://github.com/Muvon/octomind/issues/299)
* [Umbera - Fuzz target for pdf_strip_metadata](https://github.com/UmberaAPP/UMBERA/issues/3)
* [Umbera - Add NIST ACVP known-answer tests for ML-KEM-1024](https://github.com/UmberaAPP/UMBERA/issues/2)
* [sysknife - publish-mcp.yml runs an unpinned mcp-publisher with id-token: write, and pinned-dependencies.test.sh never reads that file](https://github.com/lacs-project/sysknife/issues/702)
* [sysknife - MCP plan and execute results do not name the host they came from](https://github.com/lacs-project/sysknife/issues/621)
* [sysknife - dev-stories.sh runs the stories against the sysknife on PATH, not the binary it builds](https://github.com/lacs-project/sysknife/issues/659)
* [Elpis - Add DeepSeek, Mistral or Groq as a built-in provider](https://github.com/MasihMoafi/Elpis/issues/112)
* [Elpis - Publish and maintain the elpis-bin package on the AUR](https://github.com/MasihMoafi/Elpis/issues/113)
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

659 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-11-03..2026-11-10

#### Compiler
* [add enclosing item context when pointing at `FnMut` bound](https://github.com/rust-lang/rust/pull/163241)
* [add `let_underscore_drop_guard` lint](https://github.com/rust-lang/rust/pull/164414)
* [don't emit no-op `&T` → `&T` reborrows](https://github.com/rust-lang/rust/pull/164014)
* [lint: do not emit `unreachable_code` on become with diverging callee](https://github.com/rust-lang/rust/pull/165539)
* [make inline attributes apply to the generated `poll` in async fn](https://github.com/rust-lang/rust/pull/149245)
* [next solver unsize coercion diagnostics](https://github.com/rust-lang/rust/pull/165404)
* [reborrow: Allow multiple lifetimes](https://github.com/rust-lang/rust/pull/163284)
* [restore the fast path of `QueryTypeOp`](https://github.com/rust-lang/rust/pull/165672)
* [reuse CanonicalVarKinds if no new variables were added](https://github.com/rust-lang/rust/pull/164304)
* [`rustc_builtin_macros` cleanup, part 9](https://github.com/rust-lang/rust/pull/165161)
* [skip `reachable_set` calculation during metadata encoding when unused](https://github.com/rust-lang/rust/pull/165602)
* [support mutable captures in move expressions](https://github.com/rust-lang/rust/pull/165571)

#### Library
* [`Move` trait feature gate](https://github.com/rust-lang/rust/pull/165569)
* [`String::retain`: convert only the untouched UTF-8 tail](https://github.com/rust-lang/rust/pull/163624)
* [add `BTreeMap::allocator` and `BTreeSet::allocator`](https://github.com/rust-lang/rust/pull/165463)
* [add `core::arch::{x86, x86_64}::x87_f80`](https://github.com/rust-lang/rust/pull/163992)
* [add `Discriminant::ord`](https://github.com/rust-lang/rust/pull/165443)
* [add `LinkedList::allocator`](https://github.com/rust-lang/rust/pull/165697)
* [drop array elements in order after a partial move](https://github.com/rust-lang/rust/pull/165037)
* [expose `str::split_at_[mut_]unchecked` and `OsStr::split_at_unchecked`](https://github.com/rust-lang/rust/pull/165546)
* [implement `read_le/read_be` for f16 and f128](https://github.com/rust-lang/rust/pull/165330)
* [implement `bit` and `set_bit` for integral types](https://github.com/rust-lang/rust/pull/147696)
* [optimize memset/memcpy size](https://github.com/rust-lang/compiler-builtins/pull/1333)

#### Cargo
* [`config`: Expose config merging, reading, and schemas in a crate](https://github.com/rust-lang/cargo/pull/17705)
* [`schema`: Expose SBOM types](https://github.com/rust-lang/cargo/pull/17683)
* [`doc`: do not suggest cargo fix for rustdoc warnings](https://github.com/rust-lang/cargo/pull/17702)
* [`profile`: preserve inherited strip](https://github.com/rust-lang/cargo/pull/17678)
* [`registry`: handle HTTP 410/451 for missing crates](https://github.com/rust-lang/cargo/pull/17665)
* [`schema`: Improve deserialization errors and performance](https://github.com/rust-lang/cargo/pull/17697)
* [`parse`: Trade remote manifest diagnostic quality for performance](https://github.com/rust-lang/cargo/pull/17701)
* [clean `/out` dir before compilation](https://github.com/rust-lang/cargo/pull/17675)
* [dont strip if any transtive dep has debuginfo on](https://github.com/rust-lang/cargo/pull/17689)
* [strip host deps whose debuginfo was weakened already](https://github.com/rust-lang/cargo/pull/17691)

#### Rustdoc
* [Avoid re-HIR-ty-lowering tys & const args where possible](https://github.com/rust-lang/rust/pull/163217)
* [frontend: Properly clear search state when sidebar link is clicked](https://github.com/rust-lang/rust/pull/165726)
* [Allow `.` to be used the same as `:`: in the search](https://github.com/rust-lang/rust/pull/164036)
* [Render `Deref` methods on aliased items](https://github.com/rust-lang/rust/pull/163571)
* [fix invalid clean of paths when they end with `self`](https://github.com/rust-lang/rust/pull/165357)
* [improve performance by skipping unneeded intermediate JSON serializations](https://github.com/rust-lang/rust/pull/165495)
* [make implementors collapseable if there is more than one line of docs](https://github.com/rust-lang/rust/pull/165427)
* [suggest reference definitions for unresolved links](https://github.com/rust-lang/rust/pull/165660)

#### Rustfmt
* [allow reading args from `@path`](https://github.com/rust-lang/rustfmt/pull/7220)
* [fix unclosed trailing block comments with `wrap_comments=true` or `normalize_comments=true`](https://github.com/rust-lang/rustfmt/pull/7217)
* [formatting empty where clauses when there are return types](https://github.com/rust-lang/rustfmt/pull/6167)
* [remove each comment's leading whitespace in `comment::rewrite_comment`](https://github.com/rust-lang/rustfmt/pull/5392)

#### Clippy
* [`while_let_loop`: lint only wildcard `break` patterns and special cases](https://github.com/rust-lang/rust-clippy/pull/17906)
* [add `is_lint_allowed` to `future_not_send`](https://github.com/rust-lang/rust-clippy/pull/17813)
* [extend `assert_is_empty` to support `BinaryHeap`](https://github.com/rust-lang/rust-clippy/pull/17961)
* [extend `unnecessary_to_owned` to cover `vec.contains(&str.to_string())`](https://github.com/rust-lang/rust-clippy/pull/17920)
* [fix `collapsible_match` suggestion producing non-exhaustive match](https://github.com/rust-lang/rust-clippy/pull/17964)
* [fix false positives on `duplicated_attributes`](https://github.com/rust-lang/rust-clippy/pull/17655)
* [improve `future_not_send` for next-solver](https://github.com/rust-lang/rust-clippy/pull/17849)
* [rename `large_stack_frames` and move it to suspicious](https://github.com/rust-lang/rust-clippy/pull/17749)

#### Rust-Analyzer
* [disallow ZSTs in `ThinMap`](https://github.com/rust-lang/rust-analyzer/pull/23640)
* [complete trait `dyn` prefix in type location](https://github.com/rust-lang/rust-analyzer/pull/23478)
* [offer on wildcard pattern for `destructure_struct_binding`](https://github.com/rust-lang/rust-analyzer/pull/23620)
* [add macOS, iOS, Android and musl to cfg completion](https://github.com/rust-lang/rust-analyzer/pull/23637)
* [add support for final associated functions](https://github.com/rust-lang/rust-analyzer/pull/23585)
* [do not complete dyn Trait in sized location](https://github.com/rust-lang/rust-analyzer/pull/23631)
* [do not suggest unreachable path of `struct` lit](https://github.com/rust-lang/rust-analyzer/pull/23579)
* [don't complete transparent after `repr(align(...))` or `repr(packed)`](https://github.com/rust-lang/rust-analyzer/pull/23636)
* [emit E0158 and E0532 for invalid value items in path patterns](https://github.com/rust-lang/rust-analyzer/pull/23583)
* [emit assoc item bound only with generics on builtin derive](https://github.com/rust-lang/rust-analyzer/pull/23606)
* [ensure r-a matches rustc coercion logic](https://github.com/rust-lang/rust-analyzer/pull/23610)
* [fix where clause position for assoc-type](https://github.com/rust-lang/rust-analyzer/pull/23554)
* [i28 should be i128 in completion](https://github.com/rust-lang/rust-analyzer/pull/23615)
* [ide-diagnostics: emit error E0220 for unresolved associated types](https://github.com/rust-lang/rust-analyzer/pull/23619)
* [match rustc-unicode locations in the rustc problem matcher](https://github.com/rust-lang/rust-analyzer/pull/23598)
* [nested cast before `<` add parens for `apply_demorgan`](https://github.com/rust-lang/rust-analyzer/pull/23616)
* [offer gen doc example on re-exported item](https://github.com/rust-lang/rust-analyzer/pull/23592)
* [out-of-bounds crash due to copypasta](https://github.com/rust-lang/rust-analyzer/pull/23608)
* [panic on `for<'a>` on types using `#[derive()]`](https://github.com/rust-lang/rust-analyzer/pull/23543)
* [implement hiding implicit drop hints for types without drop glue](https://github.com/rust-lang/rust-analyzer/pull/23600)
* [perf: micro-optimizations for lang items](https://github.com/rust-lang/rust-analyzer/pull/23591)

### Rust Compiler Performance Triage

A quiet week performance-wise. The only notable change was a major regression in [#165335](https://github.com/rust-lang/rust/pull/165335), which might get reverted.

Triage done by **@Kobzol**.
Revision range: [4c46afa6..54d0b7ed](https://perf.rust-lang.org/?start=4c46afa63688fb473618f9b3553d70b0c15a88eb&end=54d0b7edad511e84181ad50ec8413ef10f93b367&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.0%  | [0.2%, 3.7%]   | 163   |
| Regressions ❌ <br /> (secondary)  | 1.2%  | [0.0%, 4.6%]   | 159   |
| Improvements ✅ <br /> (primary)   | -2.2% | [-3.9%, -0.3%] | 8     |
| Improvements ✅ <br /> (secondary) | -0.7% | [-1.6%, -0.2%] | 10    |
| All ❌✅ (primary)                 | 0.8%  | [-3.9%, 3.7%]  | 171   |

6 Regressions, 5 Improvements, 3 Mixed; 6 of them in rollups
189 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/119d90a1c7fe399e1ef8cd27f257c5f6dca6f29f/triage/2026/2026-11-08.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Associated const underscore](https://github.com/rust-lang/rfcs/pull/3527)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [lexer: Update to Unicode 18](https://github.com/rust-lang/rust/pull/165289)
* [s390x: replace -Ctarget-feature=+backchain with -Cforce-frame-pointers](https://github.com/rust-lang/rust/pull/163315)
* [Stabilize `int_from_ascii`](https://github.com/rust-lang/rust/pull/163696)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC for limited crates.io self-service version deletion](https://github.com/rust-lang/rfcs/pull/4015)
* [disposition: close] [Deprecate `std::env::{vars, args}`](https://github.com/rust-lang/rfcs/pull/4021)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [perf(compile)!: Switch dev profile to debug=lines-and-tables-only](https://github.com/rust-lang/cargo/pull/17518)
* [fix(trim-paths): sanitize absolute source paths in fingerprint](https://github.com/rust-lang/cargo/pull/17556)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [T-mods 2026 budget request](https://github.com/rust-lang/leadership-council/issues/350)

*No Items entered Final Comment Period this week for
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2026-11-11 - 2026-12-09 🦀

### Virtual
* 2026-11-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/315619619/)
* 2026-11-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563020/)
* 2026-11-17 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Miscompilation in Rust 1.97**](https://luma.com/2qfai4a8)
* 2026-11-17 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: The Harness Wore a Trenchcoat - Building a Rust MCP Server**](https://www.meetup.com/women-in-rust/events/315719313/)
* 2026-11-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/317182776/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907781/)
* 2026-11-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254769/)
* 2026-11-26 | Virtual | [betterCode() Rust](https://rust.bettercode.eu/)
    * [**betterCode() Rust 2026**](https://rust.bettercode.eu/)
* 2026-12-01 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Cursed Rust**](https://luma.com/omlooqv8)
* 2026-12-01 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Christmas Coding Challenge - Kick Off!**](https://www.meetup.com/women-in-rust/events/316993134/)
* 2026-12-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcqbdb/)
* 2026-12-03 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907782/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)
* 2026-12-06 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331278/)
* 2026-12-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254767/)
* 2026-12-08 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/316993086/)

### Asia
* 2026-11-14 | Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore/)
    * [**November 2026 Rustacean Meetup**](https://hasgeek.com/rustbangalore/november-2026-rustacean-meetup/)

### Europe
* 2026-11-12 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Debugging in Rust**](https://www.meetup.com/rust-rhein-main/events/317175844/)
* 2026-11-12 | Geneva, CH | [Rust Geneva](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-11-12 | Milano, MI, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano/events/)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-language-milan/events/317094800/)
* 2026-11-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Everlock: Everything Is Git, Protocols Are Just Adapters**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816501/)
* 2026-11-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Talk Night at EIVA**](https://www.meetup.com/rust-aarhus/events/316938696/)
* 2026-11-19 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust: FFI with C++, and Intro to Async**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/317045463/)
* 2026-11-20 - 2026-11-21 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Embedded Rust mit Embassy - Workshop und Projekttag**](https://my.tuxedocomputers.com/event/embedded-rust-mit-embassy-workshop-und-projekttag-28/register)
* 2026-11-24 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #17 @ ERNI**](https://www.meetup.com/rust-basel/events/317258865/)
* 2026-11-25 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group)
    * [**Rust Meetup @ Reclamefolder**](https://www.meetup.com/rust-amsterdam-group/events/317381204/)
* 2026-11-26 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #73 sponsored by Adapt!**](https://www.meetup.com/copenhagen-rust-community/events/317181792/)
* 2026-11-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**Rust London x Zed**](https://www.meetup.com/rust-london-user-group/events/317253958/)
* 2026-11-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/317222338/)
* 2026-11-26 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @Lovable**](https://www.meetup.com/stockholm-rust/events/317161063/)
* 2026-12-01 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #90**](https://www.meetup.com/rust-paris/events/316865676/)
* 2026-12-02 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**From Curiosity to Capability: Build Your Own Device Swarm**](https://www.meetup.com/rust-rhein-main/events/317186779/)
* 2026-12-04 | Hybrid (Moscow, RU) | [RustCon RU](https://rustcon.ru/)
    * [**RustCon 2026**](https://rustcon.ru/)

### North America
* 2026-11-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Rust Game Jam**](https://www.meetup.com/utah-rust/events/317178211/)
* 2026-11-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust November Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/316886787/)
* 2026-11-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**MIT Rust Lunch, Nov 14**](https://www.meetup.com/bostonrust/events/316924331/)
* 2026-11-17 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA November w/ Hadrian!**](https://www.meetup.com/rust-los-angeles/events/316859673/)
* 2026-11-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783989/)
* 2026-11-18 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Writing a firmware for Xteink ereader in Rust**](https://www.meetup.com/vancouver-rust/events/315210234/)
* 2026-11-18 | San Francisco, CA, US) | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Social Night**](https://luma.com/v0mxbgs8)
* 2026-11-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Winter Hill Rust Lunch, Nov 21**](https://www.meetup.com/bostonrust/events/316924366/)
* 2026-11-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**South Station Rust Lunch, Nov 28**](https://www.meetup.com/bostonrust/events/316924412/)
* 2026-12-04 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC: The UnConf 2026.**](https://www.meetup.com/rust-nyc/events/317259308/)
* 2026-12-04 | New York, NY, US | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋Women in Rust x Rust NYC UnConf 2026**](https://www.meetup.com/women-in-rust/events/317383331/)

### Oceania
* 2026-11-12 | Brisbane City, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane • November 2026**](https://www.meetup.com/rust-brisbane/events/317296037/)
* 2026-11-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/316907340/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1wzctie/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> No, actually it is harder. Much harder. Your brain normally runs an error-correction pass on every word you read before that word reaches your conscious attention. This is why it's so easy to miss typos, but the correction doesn't just hide typos from you, it can replace entire phrases with what "ought" to be there. It takes continuous concentration to suppress this effect.
>
> \[…\]
>
> (Curly brace placement is of course also subject to the error filter, but curly braces go with large changes in indentation, and that's usually a big enough visual flag to get past the filter. In fact that's why Rust requires all blocks to be braced -- the experience with C was that people would think indented statements were braced when they weren't.)

– [Zack Weinberg on rust-internals](https://internals.rust-lang.org/t/yet-another-performance-footgun-in-rust-multithreading-lock-edition/24726/68)

Thanks to [Michael Howell](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1841) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1xuwet4/this_week_in_rust_677/)</small>
