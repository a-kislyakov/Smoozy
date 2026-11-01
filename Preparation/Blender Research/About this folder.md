# Blender Research

This folder holds the **Blender Research** documents — the true facts about Blender this app needs. Where those facts may come from is set by Rule 2 in the [Rules](../Groundwork/Rules.md).

What the Blender Research stage is, and where it sits in the order, is set out in **`../Groundwork/Start Here.md`**. How the research is done — and how a fact gets from a question to something a spec can lean on — lives in **`Method.md`**.

## This folder is far smaller than it looks

**The part anybody reads is twenty-six documents and about two megabytes** — the map, the ten joined answers beside it, and the fifteen comparisons. That is the whole of the Blender research, for every reader at every stage.

The other thirty megabytes are `Findings/`, the paper trail. **Nothing downstream opens it, nothing downstream links to it, and no comparison points into it.** It is there so that a claim can be checked against the words it came from, so that a new question can be answered from every version already researched, and so that a topic can be reopened when a machine arrives that can run what nobody could. It earns its place by being opened rarely, not often.

So a reader who lands here and sees a folder many times the size of the architecture is looking at a filing cabinet, not at a reading list.

## If you came here for a Blender fact, start in one place

**Open [the map](Across%20topics/The%20map.md) and go where it sends you.**

A comparison answers one topic. A builder's question is rarely one topic's — what a render's exit status is worth is answered in several at once — and a reader who opens one topic and stops gets a true sentence and a false rule. That has happened here, and the wrong claim reached a spec. So the joining has been done already, and checked.

The map lists the questions a builder actually arrives with. Against each it names the joined answer to read first, where one covers the problem, and the comparison questions behind it. It also holds the full index of every question, by topic, which is what makes a search work. **Open a comparison question only where the map, a joined answer, or your own document's sources file names it** — that question, and no more of the file than that.

**Nobody downstream assembles an answer out of several topics themselves.** That is the rule the whole layer exists for, and the [Method](Method.md) sets it out in full.

## What is in `Across topics/`

Eleven documents. The first is the door; the last is the ceiling over everything else.

- [The map](Across%20topics/The%20map.md) — which document answers which problem, and every question by topic. It holds no Blender fact.
- [What a file can be asked before it renders](Across%20topics/What%20a%20file%20can%20be%20asked%20before%20it%20renders.md)
- [What has to travel with a file](Across%20topics/What%20has%20to%20travel%20with%20a%20file.md)
- [What can be set from outside](Across%20topics/What%20can%20be%20set%20from%20outside.md)
- [Where a render's files go](Across%20topics/Where%20a%20render's%20files%20go.md)
- [While a render runs](Across%20topics/While%20a%20render%20runs.md)
- [When a render ends](Across%20topics/When%20a%20render%20ends.md)
- [What a render really did](Across%20topics/What%20a%20render%20really%20did.md)
- [Running Blender with nobody there](Across%20topics/Running%20Blender%20with%20nobody%20there.md)
- [Two renders in one folder](Across%20topics/Two%20renders%20in%20one%20folder.md)
- [What the research cannot say](Across%20topics/What%20the%20research%20cannot%20say.md) — how far the evidence was ever taken, and where a *never* rests on runs that could not have come out any other way. Read it before leaning on any Blender finding here.

## The fifteen topics, which own the facts

A comparison says what every supported version of Blender does, question by question, what is not established, and which computer each result was actually run on. Each has been checked against the findings under it, and **where a comparison and anything in `Across topics/` disagree, the comparison is right.**

Each label below says roughly what the topic covers, so you can tell which one you want. The comparison's own first line says it exactly.

- [Add-ons](Comparison/Add-ons.md) — where add-ons come from, and what a render does with them
- [Builds](Comparison/Builds.md) — what Blender publishes, and what installing one does to a computer
- [Dependencies](Comparison/Dependencies.md) — what a `.blend` points at outside itself, and how those places are found again
- [File output nodes](Comparison/File%20output%20nodes.md) — what the compositor's File Output node writes, and where
- [Frames](Comparison/Frames.md) — what a render does with the frames it is given, and what is left when it stops early
- [Images](Comparison/Images.md) — what pictures come out, in what formats, and what a written file carries about its own making
- [Node setups](Comparison/Node%20setups.md) — what files a geometry node setup makes a `.blend` point at
- [Reading a file](Comparison/Reading%20a%20file.md) — what a `.blend` gives up to something reading it, and what can be written back
- [Reports](Comparison/Reports.md) — what Blender says while it works, and what it says when a render fails
- [Settings](Comparison/Settings.md) — which settings a render uses, and which can be set from outside the file
- [The compositor](Comparison/The%20compositor.md) — what makes a file's compositor run, and what a render becomes once it reaches it
- [The machine](Comparison/The%20machine.md) — what a computer needs before Blender will run and render on its cards
- [What a render demands](Comparison/What%20a%20render%20demands.md) — what a render asks of the computer, and what changes the picture
- [What actually ran](Comparison/What%20actually%20ran.md) — what a finished render can be made to say about how it was produced
- [Writing](Comparison/Writing.md) — where a render's files end up, and what happens when a write fails

## What is here for checking, and not for reading

- **`Findings/`** — one document per topic per version, holding the quoted words, the addresses and the machine output behind every answer. The paper trail. Opened by somebody checking a claim. It runs to many times the length of the comparisons and says nothing they have not already said.
- **`Questions/`** — the blind question sheets the researchers were given.
- **`Question register.md`** — every question this project owes, where it came from, and what breaks if the answer is wrong. **Researchers must never see it**, and neither must anyone about to write a question, until they have written it.
- **`Coverage.md`** — a live tracker of which topics have been answered for which versions.
- **`Stress test questions.md`** — a bank of hard situations, used to test whether the research covers them. It asks and never answers, so nothing in it is a Blender fact and none of it is an open question owed an answer.

The audit trail sits outside this folder, under `Audits/Blender Research/`: the audits, the hand-test lists, the citation checks, the comparison checks, and each topic's **`Comparison trail.md`**, which holds how its comparison was built — what was read, which pass answered what, and what each check corrected. It is kept out of the comparison so that the comparison holds answers and nothing else. A researcher may read none of it.

## Whose words the quotations are

Quotations from Blender's manual (`https://docs.blender.org/manual/…`) and release notes (`https://developer.blender.org/docs/release_notes/…`) are © the Blender authors, used under CC BY-SA 4.0. Each quotation names its page beside it.
