# Scenario E: Review prep

> Research notes and candidate applications. Paper findings are reported research; “For us,” “Application,” and proposed pipelines describe ideas to evaluate, not completed lab-wiki capabilities.

## Scenario workflow

*Review queue opened → pull pending proposals → group by page and risk → flag weak evidence → sort by risk → organized queue (auto; organizes, never decides)*

The technical core of E is a **risk score per proposal** built from two main signals: is each claim supported by its source, and does it conflict with existing pages? Papers 1 and 2 give us those signals, and paper 3 shows how to put them into one pipeline.

### Step: "Flag weak evidence" (claim-level support checking)

**1. FActScore: Fine-grained Atomic Evaluation of Factual Precision in Long Form Text Generation**

*Min et al., EMNLP 2023 ·* [arXiv 2305.14251](https://arxiv.org/abs/2305.14251)

**What it does:** Splits a generated text into atomic facts (one claim each), checks each fact against a knowledge source, and reports the share of facts that are supported. It also offers an automated estimator that uses retrieval plus an LM.

**How we could use it:** This is basically the algorithm for "flag weak evidence." Split each proposal into atomic claims, check each claim against the cited source passage, and flag the claims that aren't supported. The proposal's support ratio becomes one input to the risk score, and the reviewer sees exactly which sentence is weak instead of a vague label. An NLI model could be tested as a first pass before calling a bigger LLM; this is a proposed implementation option, not the FActScore estimator or a measured cost result.

### Step: "Group by page and risk" (conflict with existing pages)

**2. WikiContradict: A Benchmark for Evaluating LLMs on Real-World Knowledge Conflicts from Wikipedia**

*Hou et al. (IBM Research), 2024 ·* [arXiv 2406.13805](https://arxiv.org/abs/2406.13805)

**What it does:** A benchmark of 253 real contradictions flagged by Wikipedia editors. Even GPT-4 often didn't notice conflicting passages, especially when spotting the conflict took reasoning. Explicitly telling the model to look for conflicts raised Llama-3-70B's correct rate from 10.4% to 43.8%.

**How we could use it:** A direct design lesson: if we want E to catch "this proposal conflicts with page X," we need a **dedicated conflict-check step** that retrieves related pages and explicitly asks whether they disagree. The model won't notice on its own. Their few-shot LLM judge is a candidate template for our checker. The reported ~0.8 F1 is the F1 for identifying correct responses in a 240-response evaluation; Table 3 gives Llama-3-70B 80.4 for that class and 74.4 macro-F1. It is not an overall conflict-detection score.

### Step: "Sort by risk" (the overall pipeline)

**3. Automating Low-Risk Code Review at Meta: RADAR, Risk Calibration, and Review Efficiency**

*Adams et al. (Meta), 2026 preprint ·* [arXiv 2605.30208](https://arxiv.org/abs/2605.30208)

**What it does:** A layered pipeline: eligibility gates → ML risk score → LLM review → deterministic validation, deployed on 535K+ diffs. Changing the risk threshold shifts the trade-off between approval rate and safety.

**How we could use it:** The pipeline shape maps almost one-to-one onto E: cheap deterministic gates first (does it have a citation? does the page exist?), then a risk score, then the LLM checks above. The difference is that RADAR auto-approves low-risk diffs and we only sort. Their threshold analysis is still useful for picking where our "high risk" line goes.

## Proposed technical shape

- **E pipeline:** deterministic gates (citation present? target page exists?) → atomic-claim support check (#1) → explicit conflict check against related pages (#2) → risk score → sort. Same shape as RADAR (#3), minus the auto-approve.

**Proposed evaluation hooks:** support ratio, conflict recall, and reviewer accept/reject vs. risk level.

**Reading scope:** The original E/F research notes were based mainly on abstracts; Onboarding Buddy was the exception with a full-paper reading. The reference check accompanying this conversion verifies selected claims against primary sources; it is not a full-paper review of every cited work.
