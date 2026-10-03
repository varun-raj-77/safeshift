**Final search: evidence, definitions, and scorecards — 2 October 2026**

Companion to the [50-problem atlas](<C:/Users/rekha/OneDrive/Desktop/PROJECT AVEN/FINAL_SEARCH_PROBLEM_ATLAS_2026-10-02.md>) and [final decision with adversarial review](<C:/Users/rekha/OneDrive/Desktop/PROJECT AVEN/FINAL_PROJECT_SEARCH_2026-10-02.md>). This analysis excludes personal skills, career alignment, available hours, and portfolio deadlines.

**What earns 10/10 engineering — defined before scoring**

A 10 means exceptional successful execution against a consequential, difficult problem. It requires substantial original technical judgment: explicit contracts and failure boundaries; nontrivial algorithms or systems mechanisms; justified tradeoffs among correctness, security, performance, and usability; robust operation under hostile or abnormal conditions; maintainability; and independent evaluation on representative systems. Experts should be able to inspect the design, reproduce counterexamples, and challenge its assumptions. The contribution must exceed assembling standard components and must materially improve an outcome.

It need not contain every fashionable technology or invent a new branch of mathematics. A small, rigorous mechanism can matter more than a huge architecture. Conversely, years of work or formal notation do not earn a 10. **A successful-end-state score of 10 is a conditional assessment of what exceptional execution could demonstrate, not a claim about an implementation that exists today.** None of these proposed projects has earned that grade through work performed in this review.

**How to read the scores**

Scores are comparative judgments, not measurements; differences of one point are weak evidence. Importance, frequency, and severity are separate. Frequency describes recurring exposure within the affected population, not a measured incident rate. A disaster is less frequent than an accessibility failure but can be catastrophic. Numerical and compiler errors are not assumed common simply because programs execute frequently.

Higher is better except technical difficulty and commoditization risk, where higher means harder or more exposed. “Matters in 5 years” is an ordinal probability judgment: 10 means very high confidence in continued relevance, not a calibrated 100% probability. No numerical prevalence or market size is inferred from individual issues, papers, or vendor statements.

Originality opportunity measures plausible additional contribution, not novelty of the broad idea. For SafeShift, the broad idea alone scores approximately 3/10; the 7 below requires a measured advance in a real unresolved subset. The same standard applies to every candidate. Solution weakness scores are deliberately modest: substantial prior art exists.

There is no summed leaderboard. Several dimensions are correlated, and summing them would count the same advantage repeatedly. Selection prioritizes consequential outcomes, a concrete remaining technical problem, credible independent evidence, and usefulness even without a company. Execution burden is recorded but is not subtracted from successful-end-state quality.

**Round two: all 15, all 23 requested dimensions**

Dimensions 1–7:

| Problem | Importance | Frequency | Severity | Clear value | Simple explanation | Solution weakness | Originality opportunity |
|---|---:|---:|---:|---:|---:|---:|---:|
| P08 Memory-safe migration | 10 | 8 | 10 | 9 | 9 | 7 | 7 |
| P06 Numerical correctness | 9 | 8 | 9 | 8 | 8 | 7 | 7 |
| P01 Consistent recovery | 10 | 5 | 10 | 10 | 10 | 5 | 6 |
| P18 Agent Trust Runtime | 9 | 8 | 9 | 9 | 9 | 6 | 6 |
| P27 Clinical semantics | 10 | 8 | 10 | 9 | 9 | 6 | 6 |
| P31 Assistive task interoperability | 9 | 9 | 9 | 10 | 10 | 6 | 6 |
| P02 Runtime semantic correctness | 9 | 7 | 9 | 8 | 8 | 6 | 6 |
| P07 Compiler correctness | 9 | 7 | 9 | 8 | 8 | 6 | 6 |
| P03 Silent hardware corruption | 9 | 6 | 9 | 8 | 8 | 6 | 6 |
| P11 External money reconciliation | 9 | 9 | 9 | 10 | 10 | 4 | 5 |
| P12 Database wrong answers | 9 | 7 | 9 | 9 | 9 | 5 | 6 |
| P16 Deletion and resurrection | 9 | 8 | 8 | 9 | 9 | 6 | 6 |
| P19 Offline encrypted revocation | 8 | 7 | 8 | 8 | 8 | 6 | 6 |
| P21 Transient network correctness | 9 | 7 | 9 | 9 | 9 | 5 | 6 |
| P41 FULL AVEN | 8 | 10 | 7 | 9 | 9 | 5 | 5 |

Dimensions 8–15:

| Problem | Engineering | Systems | Algorithm/research | Difficulty | Measurability | Objective oracle | Incremental evidence | Demo |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| P08 Memory-safe migration | 10 | 9 | 10 | 10 | 9 | 8 | 8 | 8 |
| P06 Numerical correctness | 10 | 9 | 10 | 10 | 8 | 7 | 8 | 9 |
| P01 Consistent recovery | 9 | 10 | 8 | 9 | 9 | 7 | 9 | 10 |
| P18 Agent Trust Runtime | 10 | 10 | 9 | 10 | 8 | 7 | 8 | 9 |
| P27 Clinical semantics | 9 | 8 | 8 | 9 | 7 | 5 | 7 | 8 |
| P31 Assistive task interoperability | 9 | 8 | 8 | 9 | 8 | 6 | 9 | 9 |
| P02 Runtime semantic correctness | 9 | 9 | 9 | 9 | 8 | 6 | 8 | 8 |
| P07 Compiler correctness | 10 | 9 | 10 | 10 | 9 | 8 | 8 | 8 |
| P03 Silent hardware corruption | 10 | 10 | 9 | 10 | 7 | 6 | 6 | 8 |
| P11 External money reconciliation | 9 | 9 | 7 | 8 | 9 | 7 | 9 | 9 |
| P12 Database wrong answers | 9 | 8 | 9 | 9 | 9 | 8 | 9 | 9 |
| P16 Deletion and resurrection | 9 | 9 | 8 | 9 | 7 | 5 | 8 | 9 |
| P19 Offline encrypted revocation | 9 | 9 | 10 | 10 | 8 | 7 | 8 | 8 |
| P21 Transient network correctness | 10 | 10 | 9 | 10 | 8 | 7 | 8 | 9 |
| P41 FULL AVEN | 9 | 9 | 9 | 10 | 6 | 5 | 7 | 9 |

Dimensions 16–23:

| Problem | Open source | Research contribution | Product | Startup | Defensibility | Long-term | Commoditization risk | Matters in 5 years |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| P08 Memory-safe migration | 10 | 9 | 9 | 8 | 8 | 10 | 5 | 10 |
| P06 Numerical correctness | 9 | 10 | 8 | 7 | 7 | 10 | 5 | 10 |
| P01 Consistent recovery | 9 | 8 | 9 | 8 | 7 | 10 | 7 | 10 |
| P18 Agent Trust Runtime | 9 | 9 | 9 | 8 | 7 | 9 | 8 | 9 |
| P27 Clinical semantics | 9 | 8 | 10 | 8 | 8 | 10 | 5 | 10 |
| P31 Assistive task interoperability | 10 | 8 | 9 | 7 | 7 | 10 | 6 | 10 |
| P02 Runtime semantic correctness | 9 | 9 | 8 | 7 | 7 | 10 | 6 | 10 |
| P07 Compiler correctness | 10 | 10 | 7 | 6 | 8 | 10 | 5 | 10 |
| P03 Silent hardware corruption | 8 | 9 | 8 | 7 | 8 | 10 | 6 | 10 |
| P11 External money reconciliation | 8 | 6 | 9 | 8 | 7 | 10 | 8 | 10 |
| P12 Database wrong answers | 10 | 9 | 7 | 6 | 7 | 10 | 6 | 10 |
| P16 Deletion and resurrection | 9 | 8 | 9 | 8 | 7 | 10 | 7 | 10 |
| P19 Offline encrypted revocation | 10 | 9 | 7 | 6 | 8 | 10 | 5 | 10 |
| P21 Transient network correctness | 9 | 9 | 8 | 7 | 8 | 10 | 7 | 10 |
| P41 FULL AVEN | 8 | 9 | 9 | 7 | 6 | 9 | 9 | 9 |

**Deeper evidence and why each remains unsolved**

**P08 — Memory-safe migration: advance.** The problem is repeated whenever maintainers change or replace exposed legacy components; exploit frequency varies greatly by project. Failures can compromise processes or systems, while a bad rewrite can break compatibility. Existing ports show that replacement is feasible, so the question is cost, coverage, and trustworthy maintenance. C2Rust produces a starting point rather than automatically eliminating unsafe behavior. SAW already supports serious C/Rust verification, including compositional work; “use a solver” is not new. Ownership relationships, valid calling contexts, observable effects, and source undefined behavior complicate scalable preservation arguments. Recent interface-inference research still documents unsupported language/program categories. **Why not solved:** usable guarantees must cross language, module, and build boundaries without moving all complexity into manually maintained assumptions. The opportunity is an empirical and technical advance on those boundaries, not a claim that nobody is pursuing them. [C2Rust](https://github.com/immunant/c2rust), [SAW Rust verification](https://galoisinc.github.io/saw-script/master/rust-verification-with-saw/index.html), [&inator](https://arxiv.org/html/2604.17261v2).

**P06 — Numerical correctness: advance.** Kernel developers and scientific users repeatedly change inputs, data types, hardware, and optimizations. Rare wrong answers can invalidate large downstream computations; population prevalence is unknown. Established benchmarks, numerical-analysis tools, and current research already go beyond naive random comparisons. Recent contract-oriented GPU verification and checker-evaluation papers are particularly close prior art. **Why not solved:** the correct answer is often a permitted error envelope, not a single bit pattern; that envelope depends on conditioning, algorithms, and downstream use. Hardware details and compiler transformations complicate independent oracles. A useful advance must catch materially wrong results while accepting legitimate floating-point variation. [Contract-grade verifier](https://arxiv.org/abs/2608.12700), [Measuring the Checker](https://arxiv.org/abs/2609.22220), [KernelBench evaluation](https://github.com/ScalingIntelligence/KernelBench/blob/main/EVAL.md).

**P01 — Consistent recovery: advance.** Recovery readiness is an ongoing operational responsibility even though actual disasters are episodic. Revenue, continuity, and sometimes safety depend on it. AWS already supports restore testing and custom validation; mature recovery vendors support application validation and dependencies. **Why not solved:** independent resource restore points need not form a valid application history, and an external effect may already have happened. A checker needs declared business invariants, real dependencies, credentials, and a safe execution environment. General recovery cannot be guaranteed from a finite rehearsal. The worthwhile residual is economical, cross-service consistency evidence and actionable repair—not another green backup dashboard. [AWS restore testing](https://docs.aws.amazon.com/aws-backup/latest/devguide/restore-testing.html), [Rubrik simulation](https://www.rubrik.com/products/cyber-recovery-simulation), [consistent recovery research](https://design.inf.usi.ch/bac).

**P18 — Agent Trust Runtime: advance.** Delegation and retries occur routinely once agents can act, while severe unauthorized effects are much less frequent and not well quantified. Authentication gateways, vaults, policy engines, and sandboxes solve real portions already. Meta's published agent-security design is substantial overlapping work, not merely a prompt defense. Research also addresses revocation and residual authority. **Why not solved:** complete mediation across tools, subprocesses, browsers, networks, and remote services is difficult; revoking permission cannot undo an already committed external effect. A genuinely portable contract must define when authority ends, what was actually authorized, and which effects remain possible. [Agentgateway authorization](https://agentgateway.dev/docs/kubernetes/latest/documentation/security/authorization/), [Meta agent security](https://research.meta.ai/blog/security-and-safety-for-ai-agents-our-approach-with-muse), [revocation research](https://arxiv.org/abs/2609.21284).

**P27 — Clinical semantic interoperability: advance.** Clinicians, patients, and analysts encounter heterogeneous records continually; clinically harmful interpretation errors are not quantified here. Syntactically valid data can still mean the wrong thing. FHIR validation, terminology products, and OHDSI already provide extensive semantic and quality tools. **Why not solved:** meaning depends on local workflows, units, timing, coding practice, and missing context; a record may not contain enough information to determine the right interpretation. Better mapping alone cannot reconstruct absent facts. The unresolved contribution is adjudicated, context-aware checking with honest uncertainty and measurable reduction of consequential mapping mistakes. [FHIR validation](https://hl7.org/fhir/R4/validation.html), [Rhapsody terminology](https://rhapsody.health/solutions/terminology-management/), [OHDSI checks](https://ohdsi.github.io/DataQualityDashboard/articles/checkIndex.html).

**P31 — Assistive task interoperability: advance.** People relying on assistive technology can encounter blocking failures in ordinary browsing and work. The burden is recurrent, though this search establishes no universal failure rate. ARIA-AT already defines tests, automation interfaces, and support reporting; Deque also offers guided testing. **Why not solved:** correct markup and individual controls do not ensure a person can complete an actual task across browser, screen reader, settings, and interaction history. Some success criteria require human judgment. The remaining opportunity is reproducible task-level evidence co-designed with disabled users, with a demonstrated coverage advance over existing tests. [ARIA-AT](https://w3c.github.io/aria-at/), [automation driver](https://github.com/w3c/aria-at-automation-driver), [Deque guided testing](https://docs.deque.com/devtools-for-web/4/en/devtools-igt-keyboard/).

**P02 — Runtime semantic correctness: advance.** Operators need to detect wrong behavior while systems appear healthy. Exposure recurs with releases and rare state combinations; actual silent-error incidence is unknown. T2C already turns tests into runtime semantic checkers, and Antithesis combines executable properties with controlled exploration. **Why not solved:** tests are incomplete descriptions of intended behavior, distributed observations can be partial, and checks compete with production latency and throughput. A useful checker must avoid confidently generalizing the wrong rule. The potential contribution is a new supported contract family or trustworthy observation mechanism with measured overhead and bug yield. [T2C](https://www.usenix.org/conference/osdi25/presentation/lou), [Antithesis](https://antithesis.com/docs/introduction/how_antithesis_works/).

**P07 — Compiler correctness: eliminate after round two.** Compiler users can suffer silent miscompilation even when their own code satisfies its language contract. Alive2 checks LLVM transformations; CompCert demonstrates a verified compiler architecture. Neither makes every language, target, optimization, and surrounding toolchain verified. **Why not solved:** semantics, undefined behavior, lowering, and scale interact. This remains an excellent research field. It loses here because the proposed general checker lacks a more specific new mechanism, while P08 connects related verification work to a concrete safety transition. That is a selection judgment, not a finding that migration is intrinsically more intellectually difficult. [Alive2](https://github.com/AliveToolkit/alive2/blob/master/README.md), [CompCert](https://compcert.org/), [historical Triton defect](https://github.com/triton-lang/triton/issues/10821).

**P03 — Silent hardware corruption: eliminate after round two.** Datacenter and scientific operators face incorrect computations that may evade ordinary error reporting. ECC and diagnostics are valuable, and fault-injection studies explore additional coverage. **Why not solved:** the real fault population is difficult to observe, application symptoms can be distant from causes, and redundancy costs resources. Simulated faults are not automatically representative of physical defects. An advance requires a credible fault model, deployment evidence, or detection mechanism. None sufficiently specific emerged to beat the finalists; hardware access alone would not fix that conceptual gap. [NVIDIA diagnostics](https://docs.nvidia.com/datacenter/dcgm/latest/learn/modules/dcgm-diagnostics.html), [GPU error study](https://arxiv.org/abs/2605.04213).

**P11 — External money reconciliation: eliminate after round two.** Teams reconcile financial events routinely; unresolved differences can block reporting or hide losses. Formance and Modern Treasury already connect records and expose discrepancies. **Why not solved:** late arrivals, missing identifiers, reversals, fees, and incomplete counterparty information create genuinely ambiguous cases. An algorithm cannot identify a fact that no available record determines. Strong product demand is plausible, but the proposed general contribution largely resembles better integration and exception handling unless a specific new model is demonstrated. It does not beat the finalists on additional technical contribution. [Formance reconciliation](https://www.formance.com/platform/reconciliation), [Modern Treasury](https://docs.moderntreasury.com/ledgers/docs/account-reconciliation).

**P12 — Database wrong answers: eliminate after round two.** Database developers repeatedly need to find queries that return incorrect results without crashing. SQLancer already implements multiple sophisticated test-oracle strategies. **Why not solved:** optimizers and SQL features create enormous state spaces, while differential testing can compare two wrong implementations or semantics that legitimately differ. New oracle mechanisms and coverage remain valuable. The broad new-project pitch does not yet identify such a mechanism; upstream work could be outstanding, but a generic query fuzzer is insufficiently differentiated. [SQLancer](https://github.com/sqlancer/sqlancer), [SQLancer research](https://www.sqlancer.com/).

**P16 — Deletion without resurrection: eliminate after round two.** Deletion requests are recurring operational events; reappearance can defeat the user's intent and organizational policies. Lethe and lineage systems address execution and tracking. **Why not solved:** undeclared copies, partial derivations, restored backups, and information embedded in models defeat universal claims. Useful guarantees must be limited to controlled systems and known transformations. A scoped conformance suite could be valuable, but this proposal risks selling stronger erasure assurance than its observation boundary supports. It loses on oracle clarity, not because privacy matters less. [Lethe](https://www.ethyca.com/lethe), [OpenLineage](https://openlineage.io/).

**P19 — Offline encrypted collaboration and revocation: eliminate after round two.** Collaborators need local operation, confidentiality, membership changes, and later synchronization. Keyhive and related local-first cryptographic work directly address that combination, although the repository explicitly describes an early, unaudited preview. **Why not solved:** offline replicas cannot instantly learn revocation, and a recipient cannot be made to forget plaintext already received. The guarantee must distinguish future access from past disclosure and define concurrent authority changes. Protocol research remains worthwhile, but “encrypted CRDT with revocation” is established territory; no new protocol result was identified in this review. [Keyhive](https://github.com/inkandswitch/keyhive), [current API](https://automerge.org/docs/keyhive/ark-api-guide/).

**P21 — Transient network correctness: eliminate after round two.** Operators must preserve connectivity and isolation during change, not only before and after it. Batfish and network-verification research already provide substantial modeling and checking. **Why not solved:** forwarding transitions, device behavior, control-plane convergence, and inaccurate models complicate guarantees. A realistic advance needs new transient semantics or demonstrably better model fidelity. The generic “verify configurations” proposition is occupied; a particular protocol/device contribution might deserve reopening, but was not established here. [Batfish](https://batfish.org/), [Batfish documentation](https://batfish.readthedocs.io/en/latest/).

**P41 — FULL AVEN: eliminate after round two, retain for explicit final comparison.** Its successful endpoint is a highly useful persistent owner-facing AI: episodic continuity, preference/procedure/correction learning, learning when not to apply something, changing owner state, model-independent stored continuity, tools, bounded autonomy, external authority, skill creation, justified specialists, controlled improvement, and later self-model or functional-affect research when experimentally warranted. Nothing in this assessment reduces it to AVEN-B or a memory CRUD app. **Why not solved:** storing experience is easier than deciding what to learn, when to generalize, when to forget, and when to decline action. Correct adaptation lacks a single objective oracle. Letta and Hermes already cover important state, memory, and skill territory. A successful AVEN could contribute new longitudinal evidence and learning mechanisms, but no specific mechanism is yet distinguished from this active field. Its successful usefulness scores highly; its uncertain novelty and partly subjective evaluation prevent it winning. Affect or self-model labels receive no extra credit without demonstrated benefit. [Letta stateful agents](https://docs.letta.com/concepts/stateful-agents), [Letta memory evaluation](https://www.letta.com/blog/evaluating-memory-in-production-agents/), [Hermes memory](https://hermes-agent.nousresearch.com/docs/user-guide/features/memory/).

**Round three selection**

The seven are P08, P06, P01, P18, P27, P31, and P02. Their full adversarial attacks and contribution tests are in the decision report. Only P08, P06, and P01 advance from those attacks. The other four remain important problems but do not justify their present project proposals as strongly.

**Successful-end-state scoring, separate from execution**

All seven finalists are scored on all 18 requested endpoint dimensions. FULL AVEN and TenantScope are included as additional comparators even though eliminated earlier. These scores assume an exceptionally competent, actually useful completed system within an honest scope. They do not assume impossible universal guarantees.

Endpoint dimensions 1–6:

| Candidate | Importance | User value | Originality | Engineering | Systems | Sophistication |
|---|---:|---:|---:|---:|---:|---:|
| SafeShift / P08 | 10 | 9 | 7 | 10 | 9 | 10 |
| Numerical Assurance / P06 | 9 | 9 | 7 | 10 | 9 | 10 |
| RecoveryProof / P01 | 10 | 10 | 6 | 9 | 10 | 9 |
| Agent Trust Runtime / P18 | 9 | 9 | 6 | 10 | 10 | 10 |
| Clinical Semantics / P27 | 10 | 10 | 6 | 9 | 8 | 9 |
| Assistive Task Interoperability / P31 | 9 | 10 | 6 | 9 | 8 | 8 |
| Runtime Semantic Checks / P02 | 9 | 9 | 6 | 9 | 9 | 9 |
| FULL AVEN / P41 | 8 | 9 | 5 | 9 | 9 | 9 |
| TenantScope / P42 | 8 | 8 | 5 | 8 | 8 | 8 |

Endpoint dimensions 7–12:

| Candidate | Research | Measurability | Demonstrability | Open source | Product | Startup |
|---|---:|---:|---:|---:|---:|---:|
| SafeShift / P08 | 9 | 9 | 8 | 10 | 9 | 8 |
| Numerical Assurance / P06 | 10 | 8 | 9 | 9 | 8 | 7 |
| RecoveryProof / P01 | 8 | 9 | 10 | 9 | 9 | 8 |
| Agent Trust Runtime / P18 | 9 | 8 | 9 | 9 | 9 | 8 |
| Clinical Semantics / P27 | 8 | 7 | 8 | 9 | 10 | 8 |
| Assistive Task Interoperability / P31 | 8 | 8 | 9 | 10 | 9 | 7 |
| Runtime Semantic Checks / P02 | 9 | 8 | 8 | 9 | 8 | 7 |
| FULL AVEN / P41 | 9 | 6 | 9 | 8 | 9 | 7 |
| TenantScope / P42 | 6 | 9 | 9 | 9 | 8 | 6 |

Endpoint dimensions 13–18:

| Candidate | Defensibility | Long-term | Intellectual depth | Builder learning | Field contribution | Simple explanation |
|---|---:|---:|---:|---:|---:|---:|
| SafeShift / P08 | 8 | 10 | 10 | 10 | 9 | 9 |
| Numerical Assurance / P06 | 7 | 10 | 10 | 10 | 9 | 8 |
| RecoveryProof / P01 | 7 | 10 | 9 | 10 | 8 | 10 |
| Agent Trust Runtime / P18 | 7 | 9 | 10 | 10 | 9 | 9 |
| Clinical Semantics / P27 | 8 | 10 | 9 | 10 | 9 | 9 |
| Assistive Task Interoperability / P31 | 7 | 10 | 9 | 10 | 9 | 10 |
| Runtime Semantic Checks / P02 | 7 | 10 | 10 | 10 | 8 | 8 |
| FULL AVEN / P41 | 6 | 9 | 10 | 10 | 8 | 9 |
| TenantScope / P42 | 5 | 9 | 8 | 9 | 7 | 9 |

**Execution dimensions 19–23 — not deductions from endpoint quality**

For these five columns higher means more burden. Capital: 1 means ordinary computing suffices for a credible contribution; 10 means major institutional infrastructure. Team: 1 means a strong individual can carry the useful system; 10 means a large multidisciplinary organization. Time: 1 means days, 5 means several months, 10 means multiple years to meaningful evidence. Estimates assume already-qualified contributors and access to relevant partners; they are planning judgments, not promises or personal schedules.

| Candidate | Difficulty | Risk | Capital | Large-team need | Time score | Plausible first meaningful evidence |
|---|---:|---:|---:|---:|---:|---|
| SafeShift | 10 | 8 | 4 | 7 | 5 | 2–4 months for a narrowly scoped comparative study |
| Numerical Assurance | 10 | 8 | 6 | 7 | 5 | 2–4 months with hardware and numerical expertise |
| RecoveryProof | 9 | 7 | 5 | 7 | 5 | 2–4 months with a realistic multi-service partner |
| Agent Trust Runtime | 10 | 9 | 5 | 8 | 5 | 2–4 months for a bounded authority/effect experiment |
| Clinical Semantics | 9 | 9 | 7 | 8 | 7 | 6–12 months including expert adjudication and data access |
| Assistive Task Interoperability | 9 | 7 | 4 | 6 | 5 | 2–4 months with disabled participants and a chosen task matrix |
| Runtime Semantic Checks | 9 | 8 | 4 | 6 | 5 | 2–4 months for one new contract family |
| FULL AVEN | 10 | 9 | 5 | 7 | 7 | 6–12 months for convincing longitudinal learning evidence |
| TenantScope | 8 | 6 | 2 | 4 | 4 | 1–2 months for meaningful transition failures and integration evidence |

These are times to evidence, not completion. A favorable first experiment would not validate the entire endpoint. SafeShift's mature vision could require several years and substantial specialist collaboration. AVEN's longer evidence window reflects longitudinal claims; it does not lower its finish-state engineering or user-value scores.

**What the numbers do and do not decide**

SafeShift does not win every column. Clinical semantics can have higher direct human stakes; RecoveryProof is easier to explain and demonstrate; numerical assurance has at least as much mathematical depth; AVEN could be more personally transformative. SafeShift wins this review's combined judgment because it addresses a durable source of software harm, has an identifiable technical barrier, permits comparatively strong independent evidence, and leaves useful ports, tools, counterexamples, and knowledge even if no standalone business results.

The largest uncertainty is additionality: an active, well-funded field may already cover the particular contribution one intends to make. That must be tested against current implementations, not settled by this scorecard.
