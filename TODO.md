# TODO
Spike index. Detailed state lives in each spike's `.todo.md`; this file stays scannable.

**GitHub Issues and the project board are the live work tracker for humans.** This file indexes the repo's spikes — bounded chunks of work that carry their own docs. See [AGENTS.md](AGENTS.md) for how the two relate.

## Active Spikes
- Workshop UI System (slug `workshop-ui-system`, opened 2026-10-22): shared foundations and components integrated across 37 current screens; ready for human QA. Source assets live on Prototype Custom Design System page; [reasoning](docs/active-spikes/workshop-ui-system.md) and [tasks](docs/active-spikes/workshop-ui-system.todo.md).
- Wireframe Workshop (slug `wireframe-workshop`, opened 2026-10-21): consolidation implemented; current F1/F4/F8 routes and grouped historical explorations are ready for human QA. Open decisions and review routes are in [reasoning](docs/active-spikes/wireframe-workshop.md) and [tasks](docs/active-spikes/wireframe-workshop.todo.md).
- Policies Migration (slug `policies-migration`, opened 2026-10-14): migrated current guidance and local routing to `docs/policies/`; verification and checkpoint state in [reasoning](docs/active-spikes/policies-migration.md) and [tasks](docs/active-spikes/policies-migration.todo.md). Closure pending.
The proposed product investigations below remain unclaimed.

## Open Questions — Not Yet Spikes
These need answering. Most will become issues; the ones with real work behind them will get a spike doc pair in `docs/active-spikes/`.

**Agent / model.** Can a *cheap* model reliably search a small shared corpus, read documents, call tools, and propose a controlled edit without touching unrelated content? Test several cost tiers and providers. Track task success, failure mode, latency, tokens, and dollars. **The question is what is cheapest-good-enough, not what is best.**

**Existing wiki.** Can Wiki.js expose enough programmatic and UI extension surface to support agent-proposed edits without consuming the semester in framework-specific work? Specifically: read and search pages externally, write revisions programmatically, surface a proposal diff in the UI, extend the interface without fighting the framework.

**Greenfield / editor.** If the wiki route is worse, can Vue plus a Typora-like Markdown editor plus Supabase provide the basic collaborative UI cheaply? Milkdown/Crepe is a candidate.

**Interaction design.** What does an agent proposal actually look like to a reviewer? Diff, rationale, affected pages, confidence and uncertainty, accept/reject/modify, attribution, rollback. **This one needs no code and is the closest to the project's actual research question.**

**Deployment.** Smallest path to a shared prototype with server-side model credentials. Who owns the keys and billing.

## Waiting On A Human
- **Team membership** is not final until the Sept 4 course deadline.
- **Compute budget** — roughly $100 was indicated as possibly available. Not confirmed.

## Current Policies
See `docs/policies/`. In short: the model stays swappable, and **the agent proposes rather than silently mutating shared knowledge.**

## Course Milestones
- **Fri Sept 4, 1:00pm** — team formation and initial idea submission
- **Fri Sept 18, 1:00pm** — formal project proposal. **Requires a timeline with at least three milestones and named responsibilities**, so spike work between now and then should produce evidence for that plan.
