# Agent Workflow Checklist (Strict, Generic)

Applies to: any coding agent (Copilot, Antigravity, etc.) on an already working app.

---

## Phase 1 — Intake

### Checklist
- [ ] Classify request: `feature` | `bugfix` | `refactor` | `other`
- [ ] Identify scope and constraints from user input
- [ ] Start implementation directly unless critical ambiguity blocks progress

### Required output phrase
`STATUS: INTAKE_COMPLETE`

---

## Phase 2 — Implement/Review Iteration Loop

### Checklist (repeat until user approval)
- [ ] Implement requested code changes
- [ ] Stop and hand off for human compile/review
- [ ] Wait for user feedback
- [ ] Apply feedback in next iteration

### Required output phrase after each iteration
`STATUS: READY_FOR_REVIEW`

### Human gate (mandatory)
Agent must not proceed to Phase 3 until user explicitly approves with wording like:
- "continue to next step"
- "approved, proceed"
- equivalent explicit approval

### Required output phrase when gate is passed
`STATUS: APPROVED_FOR_DOCUMENTATION`

---

## Phase 3 — Documentation & Project Records

### Checklist
- [ ] Update `features.md` **only if** change introduced a new user-visible feature
- [ ] Update `architecture.md` **only if** architecture/design changed
- [ ] Add a changelog entry (**always required**)

### Required skip phrases
- If `features.md` not updated:
  `SKIP: FEATURES_MD_NOT_APPLICABLE`
- If `architecture.md` not updated:
  `SKIP: ARCHITECTURE_MD_NOT_APPLICABLE`

### Required completion phrase
`STATUS: DOCUMENTATION_COMPLETE`

---

## Phase 4 — Finalization

### Checklist
- [ ] Provide final summary of code changes
- [ ] Provide final summary of docs/changelog updates
- [ ] Mark workflow complete

### Required output phrase
`STATUS: DONE`

---

## Enforcement Rules

- Never skip Phase 2 review loop.
- Never enter Phase 3 without explicit human approval.
- Never omit changelog entry.
- Use required status/skip phrases exactly as written.
