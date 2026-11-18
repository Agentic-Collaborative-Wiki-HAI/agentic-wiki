# Scenario F: Join / leave

> Research notes and candidate applications. Paper findings are reported research; “For us,” “Application,” and proposed pipelines describe ideas to evaluate, not completed lab-wiki capabilities.

## Scenario workflow

*Join: reading path by role / Leave: list pages this person owns → list items with no new owner → humans assign new owners (propose only)*

The technical core of F is two things: **(a) an ownership/knowledge model** (who knows which page) computed from our git history, and **(b) a planner that turns the wiki into an ordered reading path**. The original proposal assumed Markdown + Git storage. The transfer ideas below depend on that assumption; they do not establish the current implementation architecture.

### Deep dive: the most relevant paper (join flow)

**4. A Multi-agent Onboarding Assistant based on Large Language Models, Retrieval Augmented Generation, and Chain-of-Thought ("Onboarding Buddy")**

*Ionescu, Titov & Izadi (TU Delft, JetBrains Research), FSE 2025 Demo Track ·* [arXiv 2503.23421](https://arxiv.org/abs/2503.23421)

**Architecture:**

- **Four agents in a chain.** A Contextualization Agent uses past interactions. An Onboarding Agent answers questions and produces a step-by-step plan. A Step Processor runs several Onboarding Agents in parallel, recursively breaks each step into sub-steps, and merges the results. A Message Enhancer formats the output as Markdown and checks it against the codebase to fix inaccuracies.

- **Memory.** A database of user/LLM interactions, plus a blob store of project files and docs.

- **RAG.** 2,000-character chunks with 200-character overlap, OpenAI text-embedding-3-large, and FAISS. Each chunk keeps its file path, URL and file ID.

- **Two retrieval tools.** One returns cheap snippets (top 5, similarity > 0.1, with feedback when nothing is found or a query repeats). The other is an expensive "fetch the whole file" tool that the agent uses sparingly.

- **Planning scratchpad.** It logs every reasoning step and tool call so later steps reuse results, and so the process is auditable.

**Evaluation:** 8 participants, 3 tasks on one unfamiliar closed-source project (setup, adding a feature, writing an endpoint with tests). 7 of 8 completed all tasks. Helpfulness was 3.26/4 and ease of onboarding 3.0/4. Limitations: a pilot only, no A/B test against Copilot or ChatGPT, no quantitative hallucination evaluation, and the paper describes encoded project files without evaluating continuous synchronization after repository changes.

**How we could use it:**

- The **Onboarding Agent → Step Processor** pattern is almost exactly "reading path by role." Generate a high-level path for the role (e.g., new PhD on project X), then expand each step in parallel into concrete pages and to-dos.

- The **Message Enhancer's verify-against-source step** matches our Verifier. Every item in the reading path should link to a real page that exists, which lines up with N3.

- The **cheap-snippet vs. full-file tool split** is a useful cost pattern for our search_wiki / read_page tools.

- **Their gaps are our opening:** source-update handling (we propose rebuilding the path from current A–D output each term), no quantitative trust or hallucination evaluation (which a future study could add), and no human approval step (ours is "propose").

- They are agent-based and RAG-heavy, while the original proposal deferred vector RAG. Worth checking whether their snippet tool can be replaced by our search_wiki for a wiki of our size.

### Step: "List pages this person owns" / "List items with no new owner"

**5. A Novel Approach for Estimating Truck Factors**

*Avelino, Passos, Hora & Valente, ICPC 2016 ·* [arXiv 1604.06766](https://arxiv.org/abs/1604.06766)

**What it does:** Computes each developer's "degree of authorship" (DOA) for each file from commit history (first authorship, number of their own changes, and others' changes), assigns file authors, then greedily removes top authors until over half the files have no author left. The authors surveyed developers from 67 of the 133 projects. The abstract reports 84% agreement or partial agreement about the identified main authors, but only 53% about the estimated truck factor; these are distinct judgments.

**How we could use it:** Under the original assumption that wiki changes are captured in Git commits, we could test DOA on wiki page histories. That gives us "pages this person owns" for the leave flow, and simulating a departure gives "items with no new owner" before anyone actually leaves. It also gives a lab-level "truck factor" we could show on the overview.

**6. Quantifying and Mitigating Turnover-Induced Knowledge Loss: Case Studies of Chrome and a Project at Avaya**

*Rigby, Zhu, Donadelli & Mockus, ICSE 2016 ·* [original paper](https://users.encs.concordia.ca/~pcr/paper/Rigby2016ICSE.pdf)

**What it does:** Measures "abandoned" files after departures and simulates the risk of losing knowledge under different turnover scenarios, then compares ways to reduce it, such as planning successors.

**How we could use it:** The simulation idea, run with DOA from #5, lets F flag risk *before* graduation: "if X leaves, 14 pages have no one who knows them." That turns F from a one-time handover into an ongoing check, like D.

### Step: "Reading path by role" (ordering what to read)

**7. Graphusion: Leveraging Large Language Models for Scientific Knowledge Graph Fusion and Construction in NLP Education**

*Yang et al., 2024 preprint ·* [arXiv 2407.10794](https://arxiv.org/abs/2407.10794)

**What it does:** Builds a concept graph from text in three steps: seed concepts (BERTopic), LLM triplet extraction with chain-of-thought, and a global fusion step that merges duplicate entities, resolves conflicting relations, and infers new ones. It works zero-shot and handles prerequisite relations.

**How we could use it:** A reading path is really a topological order over "you need A before B." We could extract prerequisite links between wiki pages (or concepts) with this pipeline, then order the path by role. The fusion step also helps with duplicate pages.

## Proposed technical shape

- **F join:** Onboarding Buddy's plan → expand → verify pattern (#4), ordered by prerequisite links (#7), rebuilt from current pages each term.

- **F leave:** DOA ownership from git history (#5), plus a departure simulation (#6) that runs continuously, not only at graduation.

**Proposed evaluation hook:** newcomer task completion, borrowing Onboarding Buddy’s task design.

**Reading scope:** The original E/F research notes were based mainly on abstracts; Onboarding Buddy was the exception with a full-paper reading. The reference check accompanying this conversion verifies selected claims against primary sources; it is not a full-paper review of every cited work.

## Reference-check clarifications

DOA estimates authorship from software change history; it does not by itself establish formal page ownership or who actually knows a lab procedure. The “14 pages” example is illustrative, not an observed result. The original truck-factor implementation filters for source code and excludes documentation, so applying its model to wiki pages requires adaptation and validation. See the [original algorithm and scope](https://arxiv.org/html/1604.06766v1).
