**Final project search — decision and adversarial review — 2 October 2026**

**My choice is SafeShift: make it practical to replace dangerous legacy C components with memory-safe implementations while establishing what behavior has been preserved.**

The name is provisional. The problem is the recommendation; a separately branded platform is not a prerequisite. The most useful work might be new verification machinery, a difficult upstream contribution, or a set of maintained real-world migrations.

This is a fresh decision without career fit, available hours, job-search pressure, or a portfolio deadline. It is not a claim that this is the world's objectively greatest problem. It is the strongest project direction I found under your criteria: important, explainable, technically deep, independently evaluable, durable, and useful even if it never becomes a company.

The decisive caveat is prior art. **Verified migration is already an active field, and Galois/Immunant's CRISP & CLEAR is directly overlapping work.** A generic C-to-Rust translator with a verifier would not be an original contribution. SafeShift earns its recommendation only as work on a demonstrably unresolved migration or maintenance boundary, with current tools as the baseline. [CRISP & CLEAR](https://www.galois.com/project/crisp-clear).

The audit trail is split into three readable documents:

- [50 distinct problem areas and the first 35 eliminations](<C:/Users/rekha/OneDrive/Desktop/PROJECT AVEN/FINAL_SEARCH_PROBLEM_ATLAS_2026-10-02.md>).
- [Top-15 deeper research, the 10/10 standard, all scoring dimensions, and separate execution estimates](<C:/Users/rekha/OneDrive/Desktop/PROJECT AVEN/FINAL_SEARCH_SCORECARDS_2026-10-02.md>).
- This document: seven adversarial reviews, three successful end states and first proofs, then all 30 requested final-decision items.

**What this review establishes**

AVEN was assessed against the supplied [Blackbook](<C:/Users/rekha/Downloads/AVEN_BLACKBOOK_OWNER_COPY_2026-10-01 (1).pdf>), [research catalogue](<C:/Users/rekha/Downloads/AVEN_RESEARCH_CATALOGUE_OWNER_COPY_2026-10-01 (1).pdf>), and [implementation doctrine](<C:/Users/rekha/Downloads/AVEN — IMPLEMENTATION INSTRUCTIONS (1).txt>). Their architecture rules describe the project being evaluated; they do not override your request to reopen the decision. The full successful vision is credited in the comparison below.

Research covered primary papers and proceedings, official product and standards documentation, open-source repositories and issues, engineering incidents, historical projects, and a limited patent search. Product capabilities are credited where documented. Research and vendor claims are not treated as independently reproduced results. The supporting raw research is saved locally in the research-final-search directory.

No comparative implementation benchmark, customer interview, migration, or market-size study was conducted. A documented limitation of one tool is not proof that every competitor shares it. Absence of a found product is not proof of an empty market. The remaining opportunities below are hypotheses to test, not newly discovered facts about the universe.

The field is full of strong existing solutions. I did not find a credible 10/10 across every dimension. SafeShift's successful endpoint is approximately **10 importance / 9 value / 7 originality / 10 engineering / 8 startup potential**, conditional on delivering an actual advance.

**The seven finalists: attempts to destroy them**

Each review separates the human outcome from the machinery and identifies why the proposal survives or is eliminated. “Strongest” means the most threatening comparator identified for this proposed scope, not a comprehensive market ranking.

**1. SafeShift — P08 — survives; winner**

**Normal person:** Replace vulnerable old software without breaking the things people rely on.
**Engineer:** Migrate C modules into memory-safe Rust while checking behavior, interfaces, and subsequent changes against explicit contracts.
**Expert:** Establish scoped observational refinement across C/Rust boundaries under stated aliasing, effect, ABI, and source-definedness assumptions, then maintain that evidence as dependencies change.

**Who fucking cares?** People running software that handles hostile files, packets, and data care because a memory bug can become someone else's control over their system; maintainers care because a rewrite can exchange a known security problem for unknown regressions.

**“This exists.”** Yes, substantially. C2Rust, SAW, VERT, &inator, manual safe ports, and funded translation programs invalidate the broad novelty pitch. Code Metal also markets verified code transformation. Corrode's older design already discussed preservation and verification. The question is whether a specific useful module class or maintenance workflow remains materially underserved. [C2Rust](https://github.com/immunant/c2rust), [SAW](https://tools.galois.com/saw), [VERT](https://arxiv.org/abs/2404.18852), [Code Metal](https://www.codemetal.ai/research/ai-generated-code-that-works-and-proves-it), [Corrode](https://github.com/jameysharp/corrode).

**“I don't care.”** A maintainer with a stable, isolated component, effective mitigations, and little capacity to review a new language may rationally prefer patching. Safer code must be acceptable to operate and maintain; proof output alone has no value if nobody adopts the replacement.

**“A 500-line script solves it.”** A script can invoke a translator, compile, fuzz, and compare outputs for a simple module. That is a useful baseline. It does not establish sound ownership at a stateful foreign-function boundary or preserve behavior over untested executions. If the selected use case needs only that script, do not build a platform.

**“An incumbent adds it.”** Better compiler or cloud tooling can erase a product feature. It cannot erase the usefulness of accepted safer components, reproducible counterexamples, or a new verification result. A weak orchestration layer would be commoditized; a useful contribution can be upstreamed.

**“It works, but nobody adopts.”** Entirely possible: review burden, ABI friction, panic behavior, performance, toolchain policy, and maintenance ownership can block deployment. **“Competitors have more money.”** Also true; avoid competing on generic translation breadth. Useful coverage and honest comparative evidence can still matter.

**Fatal technical assumption:** that a manageable contract captures the source behavior and calling context that actually matter. **Fatal product assumption:** that maintainers will accept the generated implementation and maintain its evidence. **Hidden dependency:** trustworthy language/toolchain models plus maintainers who know undocumented compatibility requirements. **Trap:** placing difficult behavior behind unchecked assumptions or unsafe wrappers, then declaring the migration verified. **Expertise:** C and Rust semantics, verification, compilers, ABI/FFI, fuzzing, performance, and maintainer workflows.

**Cheapest substitute:** patch and isolate the C component; for a needed port, manual Rust plus strong differential tests and existing verification tools. **Strongest direct funded effort:** CRISP & CLEAR. **Strongest open stack:** C2Rust plus SAW/Crux and carefully chosen existing ports. **Research precedents:** VERT and &inator. The latter's documented limits include libraries and multithreading, with other restrictions; that identifies study boundaries, not a universal market gap. [Documented limitations](https://arxiv.org/html/2604.17261v2).

**Discovery versus engineering:** discover whether useful contracts and preserved guarantees can be obtained and maintained at tolerable human cost; engineer reproducible builds, integrations, reports, and counterexample workflows around that result.

**If it works perfectly, what was contributed?** At minimum better implementation, important integration, maintained safer code, and a benchmark. A new mechanism or stronger usable guarantee requires an actual result beyond the cited work. Publishing that the proposed automation does not beat expert workflows would be a useful negative result. **Survives because the underlying safety transition remains consequential even if the proposed standalone product is killed.**

**2. Numerical Assurance — P06 — survives; runner-up**

**Normal person:** Check that faster calculations still produce answers you can trust.
**Engineer:** Validate optimized numerical kernels against explicit accuracy and behavior contracts across inputs, data types, and hardware.
**Expert:** Combine conditioning-aware error specifications, independent reference oracles, adversarial input generation, and scoped analysis to distinguish invalid transformations from permissible floating-point variation.

**Who fucking cares?** Scientists and engineers care when a faster computation quietly changes their conclusion or decision.

**“This exists.”** Yes: KernelBench and FlashInfer provide substantial evaluation infrastructure; Herbie and FLiT address numerical behavior; GPUVerify addresses GPU verification; very recent papers address contract-grade kernel verification and the quality of the checker itself. A new collection of adversarial tests is insufficient novelty. [FlashInfer Bench](https://flashinfer.ai/2025/10/21/flashinfer-bench.html), [FLiT](https://github.com/PRUNERS/FLiT), [contract-grade work](https://arxiv.org/abs/2608.12700), [checker evaluation](https://arxiv.org/abs/2609.22220).

**“I don't care.”** A team whose workload tolerates the observed variation may prefer vendor libraries and ordinary regression tests. Better error bounds do not matter if they never alter a consequential decision.

**“A 500-line script solves it.”** Often it solves a fixed operator on sampled inputs: compare with a trusted implementation and generate difficult cases. It does not solve oracle validity, ill-conditioning, permitted rounding variation, and coverage across interacting operations. A project must demonstrate that these harder cases actually matter.

**“An incumbent adds it.”** Hardware vendors can provide excellent diagnostics and libraries. Cross-vendor, open evaluation and a new numerical result could remain useful; a vendor-specific test dashboard would be vulnerable. **“Technical success without adoption?”** Accurate but expensive checking can lose to cheaper tests, and overly strict checks can reject legitimate optimizations. **“Richer competitors?”** Independent benchmarks and counterexamples remain valuable if clearly better than current ones.

**Fatal technical assumption:** that a tractable contract distinguishes harmful error from acceptable approximation for the intended workload. **Fatal product assumption:** that teams will specify such contracts and accept the runtime cost. **Hidden dependency:** authoritative numerical references, representative workloads, and hardware access. **Trap:** making tolerances stricter and calling the extra failures discoveries. **Expertise:** numerical analysis, floating-point semantics, GPU programming, compilers, statistics, and verification.

**Cheapest substitute:** a high-quality reference implementation plus targeted stress tests. **Strongest incumbent alternative:** vendor libraries and vendor-supported validation for supported operators. **Strongest open alternative:** KernelBench/FlashInfer plus existing numerical tools. **Strongest close research:** the 2026 contract-grade and checker-evaluation papers; bitwise tensor-core modeling is additional prior art. [Tensor-core analysis](https://arxiv.org/abs/2609.11356).

**Discovery:** useful contracts and independent oracles that materially improve the false-accept/false-reject tradeoff. **Engineering:** integration, reproducibility, shrinking failures, hardware matrices, and performance. **Contribution:** potentially new knowledge, a mechanism, a benchmark, or a scoped guarantee. Mere packaging is better UX. **Survives**, but ranks second because “correct enough” often requires more application judgment than preservation of specified program behavior.

**3. RecoveryProof — P01 — survives; third**

**Normal person:** Find out whether your backups can actually bring your business back.
**Engineer:** Rehearse application recovery across databases, queues, objects, identity, and external dependencies, checking declared business invariants.
**Expert:** Evaluate recoverable cross-service state cuts and reconciliation plans under explicit failure scenarios, with auditable limits on consistency and recovery claims.

**Who fucking cares?** An organization that discovers during an outage that its restored systems cannot work together.

**“This exists.”** AWS restore testing, Veeam SureBackup, Rubrik recovery simulation, and Commvault cleanrooms already cover significant restore automation and application testing. They are not merely checking whether a VM boots. Consistent recovery is also longstanding research. [Veeam](https://helpcenter.veeam.com/docs/vbr/userguide/surebackup_hiw.html?ver=13), [Commvault](https://documentation.commvault.com/11.42/software/get_started_with_cloud_based_cleanroom_recovery.html), [recovery consistency research](https://www.iccs-meeting.org/archive/iccs2020/papers/121380468.pdf).

**“I don't care.”** A small stateless service or well-supported homogeneous stack may already have adequate recovery evidence. More dashboards will not improve it.

**“A 500-line script solves it.”** For one application, an isolated restore plus excellent business checks may do exactly what is needed. The residual hypothesis is that consistency diagnostics and reusable contracts reduce expertise and maintenance costs across heterogeneous systems. That must beat a competent script, not a deliberately weak baseline.

**“An incumbent adds it.”** Very plausible; the broad product category is heavily occupied. Open conformance cases and cross-vendor consistency mechanisms could remain useful. **“Technical success without adoption?”** Credentials, data handling, cloud cost, ownership disputes, and fear of affecting production can block installation. **“Richer competitors?”** Partner with their restore engines; do not rebuild backup storage.

**Fatal technical assumption:** that declared invariants and tested failures meaningfully represent future recovery. **Fatal product assumption:** that teams will maintain application-level contracts and pay to rehearse them. **Hidden dependency:** usable backups, keys, identity, and external-system semantics. **Trap:** reporting a successful exercise as a universal recovery guarantee. **Expertise:** distributed systems, storage, cloud security, data recovery, fault injection, and operational incident response.

**Cheapest substitute:** vendor restore testing with well-written application validation. **Strongest incumbent set:** Veeam/Rubrik/Commvault and AWS for their supported estates; no universal best is established. **Strongest open substrate:** Velero plus application-specific validation, with documented consistency limitations. **Research precedent:** consistent microservice recovery and causal-consistency work. [Velero limitations](https://velero.io/docs/v1.18/file-system-backup/), [Antipode](https://www.microsoft.com/en-us/research/?p=968400).

**Discovery:** whether consistency contracts and recovery choices can be made reusable without concealing irreconcilable external effects. **Engineering:** safe restore environments, adapters, scheduling, cleanup, evidence capture. **Contribution:** mainly infrastructure, integration, and a benchmark; a new recovery-selection or checking mechanism could add research value. **Survives**, but ranks below SafeShift because capable existing products plus custom validation are a particularly strong substitute.

**4. Agent Trust Runtime — P18 — eliminated after adversarial review**

**Normal person:** Give an AI permission to help without giving it permission to do anything it wants.
**Engineer:** Enforce scoped, revocable authority across agent delegation, tools, subprocesses, and external effects independently of the model.
**Expert:** Provide complete mediation and explicit revocation/effect semantics across heterogeneous execution boundaries, with independently tested confinement and delegated-capability guarantees.

**Who fucking cares?** Anyone allowing an agent to spend money, change systems, or handle private information.

**“This exists.”** Important parts do. Identity and token-vault products, Agentgateway, policy engines, and major-lab security architectures overlap materially. NIST is also studying agent identity and authorization, while recent papers examine revocation. A model-independent gateway by itself is not a new security primitive. [Auth0 Token Vault](https://auth0.com/features/token-vault), [NIST project](https://www.nccoe.nist.gov/projects/software-and-ai-agent-identity-and-authorization), [revocation research](https://arxiv.org/abs/2609.08062).

**“I don't care.”** Users running read-only agents or agents in disposable environments may need little beyond established sandboxing and narrowly scoped credentials.

**“A 500-line script solves it.”** An allowlist proxy can adequately secure a tiny, completely mediated tool surface. It cannot automatically constrain arbitrary shell/network/browser behavior or define the semantics of in-flight external actions. Broader claims need broader enforcement, not a larger policy file.

**“An incumbent adds it.”** This is a central platform responsibility. The residual open contribution could be interoperability, conformance tests, or a genuinely stronger mechanism, but providers can absorb basic controls. **“Technical success without adoption?”** Developers may reject integration friction or users may bypass controls that continually block desired work. **“Richer competitors?”** Open specifications and adversarial tests remain useful; a duplicate general platform is poorly positioned.

**Fatal technical assumption:** all relevant actions can be mediated and remote effects accurately identified. **Fatal product assumption:** useful tasks remain convenient within those controls. **Hidden dependency:** operating-system isolation and cooperating remote APIs. **Trap:** treating a signed approval or authenticated request as proof that the eventual action matched intent. **Expertise:** capability security, operating systems, distributed authorization, protocol design, threat modeling, and usability.

**Cheapest substitute:** existing sandbox plus scoped credentials, policy gateway, and human approval for consequential actions. **Strongest overlapping major-lab design identified:** Meta's published Muse security architecture. **Strongest open comparator for the gateway slice:** Agentgateway. **Research precedent:** root-scoped quiescence and residual-authority work, alongside established capability security. [Meta design](https://research.meta.ai/blog/security-and-safety-for-ai-agents-our-approach-with-muse), [Agentgateway](https://agentgateway.dev/docs/kubernetes/latest/documentation/security/authorization/), [root-scoped work](https://arxiv.org/abs/2609.21284).

**Discovery:** a portable and useful authority/effect model that can actually be enforced. **Engineering:** adapters, policy distribution, logs, isolation integration, and test harnesses. **Contribution:** potentially a protocol, mechanism, scoped guarantee, and benchmark; without those, important integration. **Eliminated because the current proposal is broader than its demonstrated new mechanism and overlaps core provider work.** Its ideal endpoint remains excellent; ambition and execution difficulty are not the reason for elimination.

**5. Clinical Semantics — P27 — eliminated after adversarial review**

**Normal person:** Stop medical records from changing meaning when they move between systems.
**Engineer:** Detect clinically consequential mapping and interpretation errors across health-data interfaces using context, terminology, and expert-adjudicated cases.
**Expert:** Validate context-dependent semantic preservation across clinical representations, separating identifiable mapping errors from ambiguity and missing information.

**Who fucking cares?** A patient whose treatment or follow-up depends on someone understanding a result correctly.

**“This exists.”** FHIR validation, terminology services, InterSystems/Rhapsody products, and OHDSI's mapping and quality ecosystem already do substantial work. It is inaccurate to portray healthcare interoperability as only transport plumbing. [InterSystems terminology](https://www.intersystems.com/HTC), [OHDSI quality framework](https://ohdsi.github.io/TheBookOfOhdsi/DataQuality.html).

**“I don't care.”** An organization may already have acceptable mappings for its chosen workflows, and clinicians cannot absorb more low-value alerts. A technically real mismatch may have no clinical consequence.

**“A 500-line script solves it.”** A local unit-conversion or code-mapping check can. Recovering workflow-dependent meaning and handling conflicting records cannot be generalized from such a script.

**“An incumbent adds it.”** EHR and terminology vendors control important integration points; a generic checker can be bundled. Shared, independently adjudicated cases and precise new semantic checks could survive. **“Technical success without adoption?”** Absolutely: workflow disruption, trust, procurement, governance, and unclear responsibility can dominate. **“Richer competitors?”** Open evidence and standards contributions remain possible, but access to trustworthy cases is essential.

**Fatal technical assumption:** the available record determines the intended meaning. **Fatal product assumption:** a correct warning leads to a beneficial action. **Hidden dependency:** expert adjudication, representative records, terminology access, and workflow partnerships. **Trap:** converting an LLM's plausible interpretation into clinical ground truth. **Expertise:** clinical informatics, terminology, interoperability standards, data engineering, human factors, and evaluation.

**Cheapest substitute:** specialist mapping review plus existing terminology and quality tools. **Strongest incumbent set:** established EHR/interoperability and terminology vendors, particularly InterSystems and Rhapsody for this scope. **Strongest open alternative:** OHDSI Usagi/Data Quality Dashboard plus FHIR tooling. **Research precedent:** OHDSI's conformance/completeness/plausibility framework. [Usagi](https://www.ohdsi.org/web/wiki/doku.php?id=documentation%3Asoftware%3Ausagi).

**Discovery:** which errors are identifiable, consequential, and correctable from available evidence. **Engineering:** mappings, connectors, provenance, review interfaces, and workflow integration. **Contribution:** important infrastructure and an adjudicated benchmark; potentially new semantic methods or a useful impossibility result. **Eliminated because a broad checker cannot manufacture missing meaning, and a narrower consequential error class has not yet been established.** A concrete clinical partnership and error corpus could change this ranking.

**6. Assistive Task Interoperability — P31 — eliminated after adversarial review**

**Normal person:** Make sure people using assistive technology can actually finish the task.
**Engineer:** Reproduce complete user journeys across browser and assistive-technology combinations, checking task outcomes as well as individual controls.
**Expert:** Evaluate interaction-level accessibility contracts using real assistive-technology behavior, explicit task oracles, and participant-validated success criteria.

**Who fucking cares?** Someone unable to apply for a job, access a service, or complete their work because a supposedly accessible interface fails in practice.

**“This exists.”** ARIA-AT, browser/assistive-technology testing, and commercial guided testing already address real interaction behavior. The archived automation-driver repository is evidence of prior implementation, not a blank slate. Current work must be checked before proposing another driver. [ARIA-AT support tables](https://www.w3.org/WAI/ARIA/apg/about/at-support-tables/), [driver history](https://github.com/w3c/aria-at-automation-driver), [Deque's current approach](https://www.deque.com/blog/intelligent-automation-and-the-new-era-of-guided-testing/).

**“I don't care.”** A buyer interested only in a compliance checkbox may not value deeper task testing. A disabled user cares about a working product, not a sophisticated test report.

**“A 500-line script solves it.”** It may verify one stable journey with a known configuration. It does not create a general human-meaningful success oracle or cover the configuration matrix.

**“An incumbent adds it.”** Browser vendors, standards groups, and accessibility companies are natural owners of this capability. Open task corpora and interoperability fixes still matter. **“Technical success without adoption?”** An expensive, flaky test matrix can be dropped from CI even if it finds real failures. **“Richer competitors?”** A community-maintained corpus and fixes can contribute, but a duplicate dashboard is weak.

**Fatal technical assumption:** machine-observed output reliably predicts whether the intended person can complete the task. **Fatal product assumption:** teams will remediate the failures rather than merely collect reports. **Hidden dependency:** disabled participants, genuine assistive technology, configuration control, and vendor cooperation. **Trap:** comparing exact spoken strings when semantic equivalence or navigability is what matters. **Expertise:** accessibility, browser internals, assistive technology, test automation, HCI, and participatory evaluation.

**Cheapest substitute:** skilled manual task testing plus existing automated checks. **Strongest commercial comparator identified:** Deque's guided testing. **Strongest open and research/standards precedent:** ARIA-AT and its automation ecosystem. **Discovery:** valid task oracles that improve coverage and reduce human testing burden without excluding user variation. **Engineering:** reproducible environments, drivers, artifacts, and failure minimization.

**Contribution:** a benchmark, important integration, better implementation, and potentially new interaction-testing methods. **Eliminated as a standalone broad proposal because the precise advance over active standards and product work is unspecified.** This is among the strongest opportunities for useful upstream contribution, and its elimination is not a low judgment of social value.

**7. Runtime Semantic Checks — P02 — eliminated after adversarial review**

**Normal person:** Catch software doing the wrong thing even when it looks healthy.
**Engineer:** Check application behavior against executable semantic properties during testing and selected production executions.
**Expert:** Derive or maintain sound, low-overhead semantic monitors under partial observation, preserving distinctions between specification, inferred hypothesis, and observed behavior.

**Who fucking cares?** An operator whose service returns success while losing, duplicating, or corrupting the work it promised to do.

**“This exists.”** Runtime verification, contracts, property-based testing, Jepsen, Antithesis, and T2C cover much of the proposed territory. “Turn tests into checks” is explicitly prior art. [T2C](https://www.usenix.org/conference/osdi25/presentation/lou), [Antithesis assertions](https://antithesis.com/docs/product/writing_tests/assertions/), [Jepsen analysis](https://jepsen.io/services/analysis).

**“I don't care.”** Ordinary invariants may already catch the important failures; another alerting layer can increase operational noise without improving decisions.

**“A 500-line script solves it.”** For a small set of known invariants, often yes. The difficult residual is sound generalization and observation across hidden or distributed state.

**“An incumbent adds it.”** Observability/testing vendors can incorporate monitors. A new checker-generation mechanism or reproducible result could survive; another runtime SDK may not. **“Technical success without adoption?”** Overhead, false alerts, and instrumentation maintenance can outweigh benefit. **“Richer competitors?”** A focused mechanism can matter, but broad platform competition has no demonstrated advantage.

**Fatal technical assumption:** intended semantics can be recovered reliably from existing tests or partial traces. **Fatal product assumption:** useful checks are affordable and actionable. **Hidden dependency:** a credible specification and enough observability. **Trap:** promoting a frequently observed pattern into a universal invariant. **Expertise:** program analysis, runtime verification, distributed systems, instrumentation, and testing.

**Cheapest substitute:** explicit domain assertions plus current test infrastructure. **Strongest commercial comparator:** Antithesis for controlled exploration and executable properties. **Strongest open alternative:** Jepsen and established property/contract tooling for their supported scopes. **Strongest close research:** T2C. **Discovery:** a new useful family of sound or explicitly approximate monitors. **Engineering:** instrumentation, deployment, replay, and reporting.

**Contribution:** potentially a mechanism, benchmark, and new knowledge; otherwise better integration. **Eliminated because the broad proposal restates an established field without identifying a specific advance.** P08 and P06 apply related verification ideas to clearer consequential outcomes.

**Historical attempts and patent search: what can actually be concluded**

Retirement is not proof that a problem is solved, and an old repository is not proof that a business failed. I searched for these counterexamples and retained only supportable conclusions.

| Finalist | Historical or discontinued evidence | Implication |
|---|---|---|
| SafeShift | Corrode is an earlier translator explicitly discussing behavioral preservation and SAW; its README is not evidence of a failed business. | Even the broad “translate then verify” combination is old. |
| Numerical Assurance | GPUVerify documents scope limits; I did not establish a comparable abandoned numerical-assurance company from reliable primary evidence. | Do not invent a failure story or infer abandonment from age. |
| RecoveryProof | Netflix retired Simian Army and moved some functionality to other projects. | Useful infrastructure can be absorbed; retirement need not mean failed demand. |
| Agent Trust Runtime | The search did not establish a directly comparable abandoned portable authority runtime from primary evidence. | Incumbent absorption is a reasoned risk, not a documented failure rate. |
| Clinical Semantics | The original Google Health service was discontinued after limited adoption; it was a personal-health-record product, not the same semantic checker. | Adjacent evidence that information access alone does not secure adoption; no inference that semantic checking cannot work. |
| Assistive Task Interoperability | The ARIA-AT automation-driver repository was archived in March 2025. | Recheck current successors and maintenance ownership; do not conclude ARIA-AT itself ended. |
| Runtime Semantic Checks | Microsoft's CodeContracts repository was archived in 2023. | Contract tooling needs a sustainable integration and maintenance path; its archive does not invalidate contracts. |

Sources: [Corrode](https://github.com/jameysharp/corrode), [GPUVerify limitations](https://github.com/mc-imperial/gpuverify/blob/master/Documentation/limitations.rst), [Simian Army status](https://github.com/Netflix/simianarmy), [original Google Health retirement](https://googleblog.blogspot.com/2011/06/update-on-google-health-and-google.html), [ARIA-AT driver archive](https://github.com/w3c/aria-at-automation-driver/pulls), [CodeContracts](https://github.com/microsoft/codecontracts).

Patent searches found examples covering C-to-Rust translation, floating-point error verification, and automated recovery-test verification. These further undermine broad novelty claims. This review did not establish patentability, legal scope, validity, or freedom to operate; it makes no such conclusion. The records are prior-art leads, not proof that a particular technique is unavailable. [C-to-Rust record](https://patents.google.com/patent/CN121209888A/en), [numerical verification record](https://patents.google.com/patent/US8397187B2/en), [recovery verification record](https://patents.google.com/patent/CA3186570A1/en).

**The three successful end states, separately from their first proofs**

**SafeShift — exceptional end state.** Maintainers can retire meaningful unsafe components incrementally. The system records the build and behavioral contract, supports candidate ports from humans or tools, checks safe interfaces and observable behavior, explains counterexamples, and exposes every unresolved assumption. It integrates with real mixed-language builds and keeps accepted evidence valid—or explicitly invalidates it—as source, callers, dependencies, or toolchains change. An ecosystem of maintained ports and reusable verified components grows around it. The maximum plausible vision is broad, sustained migration infrastructure across important systems-software domains, not a magical translator for arbitrary C/C++ with no human judgment.

**SafeShift — first proof.** Establish whether one specific additional mechanism or workflow beats the strongest available baseline on three real stateful modules and later changes. Reuse existing translation and proof tools. Measure human effort, accepted coverage, counterexamples, unsafe boundaries, and maintenance—not just compilation rate. Freeze contracts before comparing candidates. The detailed experiment and kill criteria appear in items 24–25 below.

**Numerical Assurance — exceptional end state.** Numerical contracts travel with important kernels and pipelines. Developers receive reproducible counterexamples, acceptable error envelopes, explicitly unsupported regions, and cost-aware checks across precision modes, compilers, and hardware. Independent references and, where possible, formal or interval-based arguments support the claims. Downstream users can tell what accuracy was established for their operating regime. The system accepts legitimate numerical variation and catches consequential drift; it does not promise one universal tolerance or universal exact proof.

**Numerical Assurance — first proof.** Choose a small but representative operator set with known-good implementations and independently adjudicated wrong variants. Include ill-conditioned inputs, layout/shape boundaries, reductions, and legitimate alternative rounding behavior. Compare current KernelBench/FlashInfer practices and the available contract-oriented research against the proposed addition. Hold out bug variants and workloads. Measure harmful false accepts, legitimate false rejects, coverage, cost, and reduced counterexamples. Kill a new framework if stronger baselines match it, its “discoveries” are merely stricter tolerances, or no independent oracle exists for its claimed scope.

**RecoveryProof — exceptional end state.** An operator can rehearse losing a region, identity dependency, database, object store, or selected credentials and see whether the application returns to a declared usable state. Evidence includes which data is lost, which transactions conflict, which external effects need reconciliation, and what was never tested. The system works with established backup products, diagnoses incompatible restore choices, and makes recovery contracts maintainable across changes. Its claims remain scenario- and invariant-specific. No finite rehearsal proves recovery from every future disaster.

**RecoveryProof — first proof.** Use a realistic application with a database, queue, object store, and a simulated external payment or notification service. Seed incompatible restore points, missing keys, partial writes, duplicate delivery, and already-committed external effects. Compare a competent vendor restore workflow plus custom application checks with the proposed consistency mechanism. Use held-out combinations and independently defined invariants. Measure detection, diagnosis, operator effort, restore time, and incorrect green results. Kill the new abstraction if good existing checks find the same failures at comparable cost or if each new application requires essentially a fresh bespoke model.

**Why these are the three**

Each permits a concrete comparison between promised and observed behavior, produces useful failure cases, and can leave reusable infrastructure after commercial failure. They still require substantial judgment: source contracts can be wrong, numerical specifications can be unsuitable, and recovery invariants can be incomplete. SafeShift wins because the safety transition itself is unusually durable and concrete, while formal preservation evidence can be stronger than statistical checking when the chosen scope supports it.

**Final decision: all 30 requested items**

**1. Project name / working name**

**SafeShift — verified incremental migration of legacy software.** The name is a placeholder, not a cleared brand. Start with C-to-Rust components because the safety objective, ecosystem, and existing verification work make a concrete research program possible.

**2. The problem in one sentence**

Important software still relies on memory-unsafe components that are difficult to replace without introducing new behavioral, compatibility, or performance failures.

**3. Why anyone cares**

A safer implementation is useful only if people can trust it enough to deploy it. Reducing memory hazards while preserving the behavior users depend on removes a recurring source of risk without demanding a blind rewrite. Memory-safety roadmaps and deployed Rust work establish that this is a real engineering priority, not a problem created for this proposal. They do not establish that every C component should be rewritten. [CISA roadmap guidance](https://www.cisa.gov/resources-tools/resources/case-memory-safe-roadmaps), [Android experience](https://blog.google/security/rust-in-android-move-fast-fix-things/).

**4. Who experiences it**

Maintainers of parsers, codecs, networking libraries, security-sensitive services, embedded components, and systems software; organizations depending on those components; and the people affected by failures. Initial partners should be maintainers who already want to replace a particular exposed module and can judge compatibility. “Everyone with C code” is not a useful first customer definition.

**5. Why it is not solved**

The source language can permit undefined behavior, and decades of observable behavior may be undocumented. Ownership and aliasing rarely align neatly with Rust interfaces. Mixed-language callers impose obligations that a safe-looking callee cannot prove by itself. Effects, resource use, callbacks, concurrency, platform assumptions, and maintenance changes complicate preservation.

The barrier is not generating plausible Rust syntax. It is establishing a useful, honest preservation argument at an acceptable total engineering cost.

**6. Current best solutions**

| Existing approach | What it already contributes |
|---|---|
| Manual Rust ports and replacements | Human redesign, domain judgment, and deployable safer components; some preserve established C-facing interfaces. |
| C2Rust | Translation infrastructure and migration assistance. |
| SAW/Crux and related verification tooling | Symbolic reasoning and verification infrastructure that can support C/Rust properties and equivalence arguments. |
| VERT | A research precedent combining generated Rust with equivalence checking. |
| &inator | Interface inference aimed at semantics-preserving translation into safe Rust. |
| CRISP & CLEAR under DARPA TRACTOR | Directly overlapping work on incremental safe translation and behavioral verification, including concurrent-code ambitions. |
| Code Metal | A commercial verified-transformation proposition; its claimed coverage was not independently benchmarked in this review. |

Sources: [real compatibility work](https://www.memorysafety.org/blog/compatibility-with-c/), [C2Rust](https://github.com/immunant/c2rust), [SAW 1.5](https://www.galois.com/articles/galois-releases-saw-1-5-and-cryptol-3-5-0), [VERT](https://arxiv.org/html/2404.18852v2), [&inator](https://arxiv.org/abs/2604.17261), [CRISP & CLEAR](https://www.galois.com/project/crisp-clear), [TRACTOR](https://www.darpa.mil/research/programs/translating-all-c-to-rust), [Code Metal](https://www.codemetal.ai/).

**7. Their limitations**

There is no honest single limitation shared by every solution. Manual ports can work very well but require specialist time. Mechanical translations may retain unsafe behavior. Verification tools require a supported model and useful specifications. Research artifacts cover particular languages, interfaces, and evaluation sets; a program announcement is not evidence of completed universal coverage.

VERT's 2024 evaluation is not a fair proxy for the best 2026 system. &inator's reported unsupported categories do not establish competitors' limits. Existing safe ports are counterexamples to “migration cannot be done.” The unresolved hypothesis is specifically that more real migrations—and their ongoing maintenance—can be made economical without weakening guarantees.

**8. The proposed system**

Use existing compilers, migration tools, and verification engines. Add only machinery justified by the first experiment:

1. Capture the source build, supported platforms, interface obligations, and required observable behavior.
2. Obtain candidate replacements from humans, established transformations, or an optional model.
3. Check memory-safety boundaries and behavioral obligations using the strongest applicable method.
4. Produce minimal counterexamples or explicit unsupported outcomes.
5. Track assumptions and dependencies so later changes invalidate affected evidence.
6. Integrate accepted replacements into real projects and maintain them.

A result must identify whether it is **proved under stated assumptions, checked within stated bounds, tested on stated cases, refuted, or unsupported**. These categories cannot be collapsed into a single pass badge.

A model may propose code or contracts; its proposal is not evidence that either is correct.

**9. What its successful finish state looks like**

Maintainers can progressively remove meaningful unsafe components without losing control over compatibility. Accepted ports have reviewable contracts, sound or explicitly assumed boundaries, reproducible evidence, practical performance, and an established maintenance path. Contributors can extend supported patterns and reuse published cases.

The mature system handles important stateful and mixed-language workloads, not only toy pure functions. Its maximum plausible vision includes increasingly broad effects and concurrency support, but each expansion must bring its own semantics and evidence. It never silently generalizes a proof from one platform, build, or input domain to all deployments.

**10. What is actually original**

**No new mechanism has been demonstrated yet.** The candidate contribution is making useful migration evidence compositional and maintainable at a specific real-world boundary more effectively than existing workflows.

Possible originality could lie in validated interface-contract inference for an unsupported pattern, an improved refinement/checking method, or a dependency-aware method that materially reduces proof repair across real changes. These topics already have prior art. A new result must identify exactly what existing methods could not do, or how much practical burden was removed at equal assurance.

A carefully designed benchmark of real migrations and subsequent maintenance could itself contribute. Calling a report an “evidence bundle” would not.

**11. What is not original**

C-to-Rust translation; AI-generated code; symbolic execution; translation validation; formal equivalence; counterexample-guided refinement; ownership inference; compositional verification; safe wrappers; differential fuzzing; proof caching; CI integration; and incremental migration are all established ideas. Combining their names is not a research contribution.

**12. Why it could be exceptional engineering**

A successful system must make language semantics, ownership, compilation, security boundaries, performance, developer experience, and long-term maintenance agree on the same real artifact. It must be useful when inputs are difficult and assumptions are wrong, not merely when a demo compiles.

Its quality is externally contestable: reviewers can inspect the contract, replay a counterexample, vary build assumptions, compare a baseline, and examine deployed behavior. That is why exceptional execution can meet the 10/10 standard defined before the scorecards. The proposal itself has not earned that grade.

**13. The hardest engineering problems**

- Expressing intended behavior without accidentally excluding the difficult cases.
- Connecting ownership and aliasing across C/Rust interfaces, callbacks, and state.
- Distinguishing source undefined behavior from defined behavior that must be preserved.
- Scaling verification without hiding failures behind unchecked assumptions.
- Preserving ABI, effects, resource behavior, and acceptable performance.
- Maintaining evidence across changes to callers, headers, build flags, dependencies, and toolchains.
- Producing explanations that enable a maintainer to make a correct decision.

Rust memory safety is not functional correctness, freedom from panics, absence of resource exhaustion, or overall security. Unsafe code and foreign interfaces require their own soundness argument. A source-level result also does not automatically prove the shipped binary independently of the compiler.

**14. What must be discovered versus implemented**

**Discovery:** whether the chosen module class admits sufficiently useful contracts; whether those contracts can be checked against callers; whether a proposed mechanism expands supported coverage or lowers human effort; and whether the advantage survives subsequent changes.

**Implementation:** build capture, adapters, job execution, storage, evidence formatting, CI integration, and counterexample replay once the underlying method is justified.

A vital negative finding would be that the hidden assumptions cost as much to maintain as the original code. The project must be able to conclude that its own automation is not worthwhile.

**15. Research contribution if successful**

A new supported verification case, a stronger practical guarantee, an empirical account of migration/maintenance costs, a reproducible benchmark, or a principled account of where preservation cannot be claimed. A paper based only on more generated lines of compiling Rust would not satisfy the recommendation.

The strongest outcome would let another research group reproduce both successes and failures on held-out modules and changes.

**16. Open-source contribution if successful**

Maintained safer components, improvements to existing translators/verifiers, reusable contracts and harnesses, minimized regression cases, reproducible evaluations, and clear unsupported cases. These remain useful even if nobody adopts SafeShift as a separate product. Upstream acceptance is stronger evidence than another repository containing a prototype framework.

**17. Product value**

Reduce the total effort and uncertainty of an already-needed migration. The buyer receives an acceptable replacement and maintainable evidence, not merely translated source. The relevant metrics are reviewer hours, accepted behavior coverage, regression risk indicators, maintenance effort, and operational performance.

Demand has not been validated by interviews here. Organizations that do not need or want a port may not be customers.

**18. Company potential**

Plausible, but less certain than the technical value. A business could begin with difficult migrations, supported verification infrastructure, and ongoing maintenance contracts. Productization would require repeatable module classes and repeatable buyer needs; otherwise it becomes specialist consulting.

Established vendors and funded teams are serious competition. A useful open-source research program can succeed even if the venture-scale company thesis fails.

**19. Defensibility**

Earned expertise in real compatibility problems; trustworthy verification infrastructure; accepted upstream relationships; a substantial maintained corpus; integrations that encode difficult domain knowledge; and a reputation for honest guarantees. Open methods need not prevent a service or support business.

Prompts, a dashboard, a private collection of model outputs, or access to a generally available model are weak defenses. A better compiler can absorb features, so the lasting asset must be demonstrated technical capability and adoption.

**20. Biggest reason it may fail**

The specification and maintenance burden may overwhelm the savings. A system could translate impressive amounts of code while moving the hardest work into fragile assumptions and unreviewable proofs. If maintainers cannot safely own the result, the project fails its central purpose.

The second major risk is additionality: a capable existing toolchain may already serve the chosen subset well enough.

**21. Cheapest substitute**

For many components, patching, sandboxing, and stronger tests are cheaper than a port. When migration is justified, an expert manual Rust implementation plus existing fuzzing and verification tools is the serious substitute. The project must compare against this competent workflow, not against a straw-man untested rewrite.

**22. Strongest competitor**

**Galois/Immunant and partners' CRISP & CLEAR effort under DARPA TRACTOR** is the closest identified technical overlap. Code Metal is a relevant commercial comparator. The first experiment must also use the best publicly available open stack, since a program description is not an executable baseline. [CRISP & CLEAR](https://www.galois.com/project/crisp-clear), [Code Metal](https://www.codemetal.ai/).

**23. Five-minute demo**

This is a proposed demonstration, not something built in this review.

| Time | What the viewer sees | What it establishes |
|---|---|---|
| 0:00–1:00 | A real C component's interface, supported behavior, and a known malformed-input failure | A concrete reason to migrate |
| 1:00–2:00 | A Rust candidate passes ordinary tests, then a checker produces a small valid-input behavioral counterexample | Additional evidence beyond compilation and sampled success |
| 2:00–3:00 | A repaired candidate satisfies the supported contract; assumptions and unsupported behavior remain visible | A scoped preservation result |
| 3:00–4:00 | The malformed-input case is handled according to an explicit approved repair policy | Safety improvement without pretending undefined C behavior was preserved |
| 4:00–5:00 | A caller or build change invalidates affected evidence; rechecking exposes what must be repaired | Maintenance of the guarantee |

Show measured performance and the actual unsafe/FFI boundary. If the backend only establishes a bound, display that bound. No “100% verified” animation.

**24. First decisive experiment**

**Question:** Can one specific addition to existing tools reduce the human cost of an acceptable, behavior-preserving safe migration and its maintenance without lowering assurance?

Select three real, maintainer-relevant stateful modules from existing projects: for example a streaming parser, a binary decoder, and a stateful container. Those are selection categories, not claims that partners have already agreed. Favor a coherent initial semantic scope, such as single-threaded modules with reviewable byte-oriented interfaces, while retaining state transitions and meaningful callers. Do not choose only isolated arithmetic functions because they are easy to prove.

Use two matched workflows: an expert baseline using current translation, fuzzing, and applicable verification tools; and the same workflow with the proposed addition. Include existing hand-written ports where available. Reproduce relevant public research results before claiming to exceed them. If a stronger available system is discovered, add it to the comparison.

Before candidate generation, independently review and freeze the preservation contract, supported build, caller assumptions, and permitted repairs. Separate valid source behavior from malformed-input cases requiring a new policy. Hide selected later changes and seeded regressions from the workflow developer. Include contract mistakes, interface mistakes, and stale-proof cases as well as ordinary code bugs.

Measure:

- Initial and subsequent human engineering/review hours, including contract creation and proof repair.
- Useful API/state/effect coverage, with assumed, bounded, tested, and proved portions separated.
- Rejection of held-out incorrect candidates and acceptance of valid alternatives.
- Remaining unsafe obligations, unsupported cases, and performance against maintainer-agreed budgets.
- Whether an independent maintainer would accept and maintain the result.

A reasonable preregistered screening target is at least **30% lower total human effort on two of three modules**, at equal or better agreed coverage, with no incorrectly accepted seeded regression inside the claimed guarantee. The threshold is a proposed decision rule, not an evidence-based industry standard. Three modules cannot establish a universal defect rate; they can expose whether this mechanism deserves a broader study. Any apparent soundness failure requires investigation and suspension of the affected claim.

The experiment may finish by recommending improvements to existing tools rather than a new framework. That is a valid outcome.

**25. Kill criteria**

Kill or substantially redirect the proposed standalone approach if:

- The strongest existing workflow already matches its cost and assurance on the chosen cases.
- The advantage disappears after counting specifications, review, unsupported cases, and later maintenance.
- Success depends on weakening contracts, excluding realistic callers, retaining broad unsafe regions, or calling bounded testing proof.
- Useful coverage collapses outside toy functions.
- Performance or maintainability prevents acceptance by the intended maintainers.
- Source/build changes repeatedly require near-total proof reconstruction with no credible improvement path.
- An independent evaluator cannot reproduce the claims.

A failed experiment kills a mechanism or product thesis. It does not imply that making legacy systems safer is no longer worth working on. Redirecting to a successful existing project may be the best result.

**26. Full development arc**

| Stage | Required outcome before advancing |
|---|---|
| Prototype | Reproduce a current baseline on one real module; document the contract and unsupported behavior; demonstrate a genuine counterexample. |
| Research/proof | Complete the comparative three-module study; identify a specific method that improves cost, coverage, or assurance; publish negative cases. |
| v0.1 | Support a narrow coherent module class reproducibly, with honest result types, build capture, counterexample replay, and change invalidation. |
| v1 | Several independent maintainers accept useful ports; later releases retain valid evidence; installation and operation are documented and supportable. |
| Mature system | Broader language/effect/platform coverage, reusable contracts, integrations, an independently maintained benchmark, and credible long-term stewardship. |
| Maximum plausible vision | A durable ecosystem for incrementally retiring major classes of unsafe legacy components while preserving specified behavior across their lifetimes. |

Planning judgment for an already-skilled effort: a serious first study might take 2–4 months; a useful v1 could take 12–24 months; mature breadth may take 3–5 years or longer with specialists. These are rough estimates, not promises, personal-budget assumptions, or reasons to discount the end state.

Do not expand to arbitrary concurrency, operating-system kernels, or all C++ merely to enlarge the vision. Each new scope requires demonstrated semantics and demand.

**27. Final scorecard**

These are successful-end-state judgments, conditional on a real contribution beyond current tools. The full definitions and comparative matrices are in the [scorecard document](<C:/Users/rekha/OneDrive/Desktop/PROJECT AVEN/FINAL_SEARCH_SCORECARDS_2026-10-02.md>).

| Successful endpoint dimension | SafeShift / 10 |
|---|---:|
| Problem importance | 10 |
| User value | 9 |
| Originality | 7 |
| Engineering depth | 10 |
| Systems depth | 9 |
| Technical sophistication | 10 |
| Research potential | 9 |
| Measurability | 9 |
| Demonstrability | 8 |
| Open-source potential | 10 |
| Product potential | 9 |
| Startup potential | 8 |
| Defensibility | 8 |
| Long-term relevance | 10 |
| Intellectual depth | 10 |
| Learning value for its builder | 10 |
| Potential contribution to its field | 9 |
| Simplicity of core explanation | 9 |

| Separate execution burden | Score / 10; higher means more burden |
|---|---|
| Execution difficulty | 10 |
| Execution risk | 8 |
| Capital requirements | 4 |
| Need for a large team | 7 |
| Time until meaningful evidence | 5; approximately 2–4 months for the initial study under stated assumptions |

Originality is the most conditional favorable score. The generic proposition “translate C to Rust and verify it” is approximately 3/10 originality. If no specific advance emerges, the project's originality grade must fall; the score is not protected by the recommendation.

**28. Direct comparison: SafeShift vs FULL AVEN vs TenantScope vs Agent Trust Runtime**

| Dimension | SafeShift | FULL AVEN | TenantScope | Agent Trust Runtime |
|---|---|---|---|---|
| Successful human outcome | Safer legacy software with preserved specified behavior | A persistent AI that learns appropriately and remains under its owner's authority | Users' state remains isolated across account transitions | Delegated software stays within current authority |
| Importance | 10 | 8 | 8 | 9 |
| User value | 9 | 9 | 8 | 9 |
| Originality opportunity at endpoint | 7 | 5 | 5 | 6 |
| Engineering | 10 | 9 | 8 | 10 |
| Measurability | 9 | 6 | 9 | 8 |
| Research | 9 | 9 | 6 | 9 |
| Open source | 10 | 8 | 9 | 9 |
| Product | 9 | 9 | 8 | 9 |
| Startup | 8 | 7 | 6 | 8 |
| Defensibility | 8 | 6 | 5 | 7 |
| Main unresolved contribution | Practical preservation and maintenance at a verified unresolved boundary | Reliable context-sensitive learning over time, distinguished from existing agent systems | Better temporal isolation coverage than existing testing | Portable enforceable authority/effect semantics beyond existing controls |
| Cheapest credible alternative | Expert port plus existing tools, or patch/isolate C | Existing stateful agents plus deliberate workflow and memory management | Custom transition tests plus existing scanning/testing tools | Sandbox, scoped credentials, policy gateway, approvals |
| Biggest competitive threat | Current verified-migration teams | Platforms bundling state, learning, skills, and tools | Security/test vendors adding transition scenarios | Model/cloud/identity platforms owning execution boundaries |
| Selection judgment | **Winner** | High-value endpoint, less sharply identified new contribution and oracle | Useful narrower engineering contribution; prior career advantage removed | Exceptional possible security system, but broad proposal overlaps major efforts |

For AVEN, successful implementation means all the requested outcomes: a single persistent owner-facing identity; deep episodic continuity; changing preferences and owner state; procedure, correction, and non-application learning; continuity through model changes; tools and bounded autonomy; authority outside the model and Root enforcement; skill creation; justified specialist creation; controlled self-improvement; and long-term learning. Self-model and functional-affect research count only when they improve a scientifically defined outcome. **Stored continuity across models does not mean identical behavior across models.**

AVEN's potential contribution is a new learning mechanism or convincing longitudinal finding, plus useful infrastructure and integration. A controlled study must show correct generalization and correct non-application, not just successful recall or a persuasive conversation. Its ideal user value is high. Its lower originality and measurability scores are not penalties for ambition. [Letta evaluation work](https://www.letta.com/blog/evaluating-memory-in-production-agents/), [Hermes memory](https://hermes-agent.nousresearch.com/docs/user-guide/features/memory/).

AVEN in one sentence for a normal person: **An AI that remembers your life and learns how to help you without taking control away from you.** For an engineer: **A persistent owner-facing agent with evaluated contextual learning and external enforcement of authority.** For an expert: **A longitudinal adaptation system that must demonstrate calibrated generalization, non-application, and controlled policy change under evolving owner state.** Who cares? Someone tired of repeatedly explaining themselves and correcting the same mistakes. That value is real even though the research contribution remains to be established.

TenantScope is judged on its actual outcome: preventing stale requests, caches, or UI state from crossing identity transitions. Escape already documents multi-user tenant-isolation testing, and scheduling/property-testing tools offer relevant mechanisms. The residual could be excellent temporal coverage and developer experience, but a new broad research result is less evident. [Escape](https://escape.tech/blog/escape-dast-multi-user-testing-tenant-isolation/), [fast-check scheduling](https://fast-check.dev/docs/advanced/race-conditions/).

TenantScope in one sentence for a normal person: **Make sure switching accounts never shows you someone else's information.** For an engineer: **Exercise identity transitions under delayed requests and retained state to expose cross-account leaks.** For an expert: **Check temporal noninterference properties under controlled asynchronous schedules and independently specified ownership oracles.** Who cares? Users whose private information leaks and teams responsible for preventing it. The likely contribution is a better implementation, benchmark, integration, and UX unless a new checking mechanism is established.

The full Agent Trust Runtime is credited with complete enforcement over its declared boundaries, delegation, revocation, and externally meaningful effects. It can match SafeShift's engineering depth. It loses this selection because the current proposal has not identified its new enforceable mechanism clearly enough beyond the strong prior art—not because model-independent security is unnecessary.

**29. Why the winner actually wins**

SafeShift connects a simple human outcome to deep technical work and unusually inspectable evidence. It addresses risk in software that already matters, can progress without an LLM, and produces useful artifacts even if commercialization fails. A partial success can be an accepted safer library, a verification improvement, or a benchmark that makes future claims harder to fake.

Numerical Assurance is exceptionally strong but depends more heavily on deciding what approximation is acceptable. RecoveryProof is immediately understandable but faces particularly capable existing products and custom-check substitutes. AVEN's endpoint is attractive, but reliable personal adaptation is harder to assess independently and its additional mechanism remains less specific. Agent Trust Runtime faces serious enforcement problems, yet its broad abstraction is already a central platform pursuit. TenantScope remains useful but does not have the strongest long-horizon contribution under the revised criteria.

The winner is a problem worthy of sustained work, not a promise that an untouched market awaits. If the best route is contributing to an established verification or migration project, that strengthens the outcome rather than invalidating the choice.

**30. Final statement**

If I could personally choose only one of these problems to spend the next several years understanding deeply, I would choose **making it practical to replace memory-unsafe legacy software without losing the behavior people depend on** because **it removes a durable source of harm, demands serious technical judgment, allows independent evidence of progress, and remains useful even if AI hype disappears and no company is ever built.**
