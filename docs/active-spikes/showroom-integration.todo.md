---
attention: live
---
# Integrated Showroom walkthrough tasks
Opened 2026-11-07. Slug: `showroom-integration`. [Purpose and frame map](showroom-integration.md).

## Scope
Bring the interview-needs map into Showroom, compare it with Aslan's seven-frame walkthrough, and build a combined walkthrough in the same style. Preserve Aslan's frames, Workshop pages, Erwin's pages and shared library definitions. Repository records contain no participant quotations or private source material.

## To Do
- [ ] Link the two new quotations in I07 and I08 to their source records, or replace them with linked material.
- [ ] Decide whether the Cover should link to the integrated walkthrough.
- [ ] Decide whether the needs-map and reconciliation sections stay on Showroom after the team has reviewed them.

## Ready for Human QA
- [ ] Aslan reviews the integrated walkthrough I01–I09 against his original tour.
- [ ] The team fills the reconciliation board's decision column (keep, merge or drop).
- [ ] Play I01–I09 in the native Figma player: Next, Previous and Restart tour.
- [ ] Tell Erwin that I07 shows a copy of his F2-1 search screen, and confirm he is comfortable with it being presented.
- [ ] Check the needs map's participant codes and counts against the source records.

## Done
- [x] Put needs-map boards 01–03 on Showroom as editable frames in their original styling (section `1829:135`). The first write failed because the account had a view-only seat; it succeeded after the seat changed. Visual inspection of the section found one overflowing flow card in board 01; it was fixed and rechecked.
- [x] Build the reconciliation board (`1832:136`): 16 topics, four covered by both, two partly overlapping, eight only in the needs map and two only in the tour. Visual inspection passed. An unverified claim about when Workshop boards were renamed was corrected.
- [x] Build the integrated walkthrough I01–I09 (section `1838:135`) from copies of Aslan's frames. Copies of his main components were replaced with instances, so no duplicate components exist.
- [x] Add participant counts and needs-map links to the research panels; write the I07 and I08 content; set slide numbers to `/ 09`.
- [x] Rewire Previous, Next and Restart tour for I01–I09.

## Verification · 2026-11-07
- **Structural inspection:** every navigation button resolves to the intended neighbouring frame, and Restart tour on I09 resolves to I01. No integrated frame contains a component. Every research panel's content fits its 740 px column (largest 688 px). None of I08's gap cards overflows.
- **Visual inspection:** I01, I07, I08 and I09 were inspected at presentation scale. Two defects were fixed: I01's slide number had moved to the top-left, and I07's screen crop clipped a header icon. A row-level render was checked afterwards. I02–I06 were inspected only at row scale.
- **Prototype playback:** not run.
- **Human acceptance:** none yet.
