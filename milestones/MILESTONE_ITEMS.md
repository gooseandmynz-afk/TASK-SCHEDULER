# Project Milestones — Week-by-week checklist

Use this file to populate the GitHub milestone description or attach to the milestone as a reference.

Week 1 — Proposal & Planning

- [ ] 1–2 page Project Proposal (scope, risks, target OS)
  - See: `milestones/code_snapshots/week1_PROPOSAL.md`
- [ ] UI wireframes (rough sketches)
  - See: `milestones/code_snapshots/week1_PROPOSAL.md`
- [ ] Work plan with week-by-week tasks and hour estimates
  - See: `milestones/code_snapshots/week1_PROPOSAL.md`

Week 2 — Architecture & Spike

- [ ] `ARCHITECTURE.md` (state model, message flow, data sources, error handling strategy)
  - See: `milestones/code_snapshots/week2_ARCHITECTURE.md`
- [ ] Iced shell application that builds successfully
- [ ] One screen with mock data; finalize crate selection; prove background task integration

Week 3 — Data Layer & Scanning

- [ ] Real data retrieval behind async boundaries (example: process list or file scan)
- [ ] Unit tests for data layer
- [ ] Implement initial structured logging
  - See: `milestones/code_snapshots/week3_DATA.md`

Week 4 — Core UI & Actions

- [ ] Table view with sort/filter capabilities
- [ ] Detail pane implementation
- [ ] One state-changing action with confirmation dialogs and error handling
  - See: `milestones/code_snapshots/week4_UI.md`

Week 5 — Persistence & Polish

- [ ] Configuration persistence; settings screen
- [ ] Progress indicators; ensure non-blocking user experience
  - See: `milestones/code_snapshots/week5_PERSISTENCE.md`

Week 6 — Testing & Hardening

- [ ] Integration tests; fault injection (simulate I/O errors)
- [ ] Improved error messages; basic accessibility features
  - See: `milestones/code_snapshots/week6_TESTING.md`

Week 7 — Beta & Documentation

- [ ] Beta release tag (`v0.1.0-beta`)
- [ ] `USER_GUIDE.md` (installation/usage/troubleshooting)
- [ ] `TEST_PLAN.md` with test results and performance analysis
  - See: `milestones/code_snapshots/week7_RELEASE.md`

How to use

- Copy the checklist contents into the GitHub milestone description field, or attach this file to the milestone.
- Optionally attach the `milestones/code_snapshots` files as additional references.
