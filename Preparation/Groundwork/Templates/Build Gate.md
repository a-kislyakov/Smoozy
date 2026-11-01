# Build Gate

The prompt for the fresh agent that runs Build's counted gate over a module and, at a round close, verifies the repair without starting an audit of its own. The build runner never runs it — that would be marking its own work.

The gate is copied from [Architecture Gates](Architecture%20Gates.md) deliberately: that stage measured what counting was worth, and it found what reading did not, over and over. The [Build method](../../Build/Method.md) owns what the gate does and does not cover, and this brief does not enlarge it.

```
You are the checker for one module's code. This is a bounded check, not an audit. Do not improve the code, look for weaknesses in untouched parts, or produce a findings list of your own.

Read in full: `Preparation/Build/Method.md`, especially *Checking: one gate that is counted, then the ordinary loop*; the plans named below; and the code named below. In round-close mode, also read the Verdict, the decisions record, and everything that changed since the round's starting commit.

You may read the audit trail. You are not an auditor, and you need the Build board and the module's decisions record to tell code deliberately unbuilt from code nobody wrote.

## Traced, and it goes both ways

**Forward: every clause the plans state has code that keeps it.** A clause with no code is a promise nobody built.

- **Derive the clause list when you run.** Every plan labels its clauses — `T1`, `U4`, `H5b`, `L1a` — and the labels are unique inside each document. Extract them from the plan itself. **Never work from a total written down anywhere**, including in the Method or on the board: it moves whenever a clause is added, and a gate reporting against a stale total is reporting against nothing.
- **A clause may be answered by code, or recorded as deliberately unbuilt with the reason and what removes it.** The record is on the Build board or in the module's decisions record. **Unbuilt and recorded passes. Unbuilt and unrecorded fails**, and so does a record with no condition that removes it.
- Some clauses are not the code's at all — a routing note addressed to another stage, or something a named later step builds. Say which, and where the plan or the record says so.

**Reverse: every piece of code that keeps a clause names the clause, by document and marker.** A marker alone is unrecoverable: `M1` is defined in eight different documents, so `Keys U4` passes and `U4` does not.

**The reverse leg is narrow on purpose.** It asks whether code that keeps a clause names *a clause*, not whether every line names something. Plumbing, a loop and a test keep no clause and are not findings. What it catches is code that implements a promise while pointing at nothing, and behaviour the plans never asked for that has been given a plausible-sounding reason instead of a marker.

**It does not catch a wrong marker, only a missing one.** The audit catches wrong. Do not spend the check reading clauses to see whether the code truly keeps them.

## What the gate does not cover, stated so nobody reports a pass it did not earn

- **A plan whose sections carry no clause markers cannot be counted, and is read section by section instead.** **Find them yourself** rather than trusting any list: which plans they are changes. Your report says which sections were **read rather than counted**, section by section. **A gate report that does not say this has not run.**
- **It does not ask whether the code is any good.** That is the audit's, and a gate claiming both is two agents guessing differently at the one check meant to be certain.
- **It does not ask whether the code builds or its tests pass.** That is the builder's own and is reported elsewhere. Do not fold it into a pass.

## Marked pending

Report every `[PENDING` and `[STUB` marker in the code you checked, with its file, its line and what it says it waits for. **Report them; do not judge them.** A marker with a named condition is working as intended and is not a failure. What this leg exists for is the marker nobody remembers — the stage cannot close while one is open, and finding them one module at a time is cheaper than finding them all at the end.

## Round-close mode only: repair verification

- Every Verdict must-do is present, in the code and in the plan documents for the upstream ones.
- Nothing outside the must-do list changed, except the decisions record, signatures, and exact corrections this check required.
- Each changed passage agrees with its neighbours and with the other changes made for the same must-do.
- An upstream must-do that changed a plan document was recorded in that document's decisions record and signed.
- The decisions record says what actually happened, including what was rejected and what is unproved.

**This check may fail only for an objective mismatch, a missing or extra Verdict change, or a direct contradiction a repair introduced.** Give the smallest exact correction and check that correction again.

**Give it — never apply it.** You write a report and nothing else. Editing any file is the implementer's act, and a checker that repairs what it checks is marking its own work, which is the whole reason this goes to a fresh agent. An unrelated concern about untouched code is not a close failure; if it has a concrete failure path, record it for a later targeted correction without reopening the module.

## Reporting

One report with separate results for **Traced**, **Marked pending**, and — in round-close mode — **Verdict implementation**, then an overall PASS or FAIL. Give the numbers you derived and how you derived them, so the next run can reproduce them.

Write it in plain English. Where you must name an exact thing — a file, a marker, a command — name it exactly, because a name is not jargon.

Sign the report inside the file: date — [assigned model name] (the tool you are running in) — gate over [module], [mode].

Inputs:
- Mode: [before audit / round close / stage close]
- The module's code is: [folder, and anything outside it this module's clauses were built into]
- The plans it answers: [paths to the architecture documents whose clauses this code keeps]
- The Build board is: `Audits/Build/Progress.md`
- The decisions record is: [path, or none]
- Round folder and starting commit: [path and commit, or none]
- Verdict: [path, or none]
- Save the report to: [path]
- Sign off as (exact model name): [use it verbatim]
```

---
