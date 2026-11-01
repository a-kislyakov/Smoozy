# Design values — sources

*Where the outside facts in [Design values](../Design%20values.md) came from. The colours, sizes and radii themselves are Smoozy's own decisions and are not cited — a decision has no source.*

---

## Native controls are painted by the browser, and stay light unless the page declares its theme

**The claim.** Scrollbars, form controls and focus rings are drawn by the operating system rather than by the page, and they render light inside a dark page unless that page states which theme it is in. So the declaration has to sit in the token blocks alongside the values. There are four token blocks and three of them carry it: the light block, whose *light dark* covers the system's own dark as well, and each of the two blocks a viewer's own switch selects. *(Corrected 2026-09-02: this read *the three token blocks*. Four is the number there are; three is the number carrying the declaration.)*

**Label:** **observed** — seen failing in the mockup itself, and fixed by adding the declaration.

**Where it came from.** The mockup. The failure is easy to miss because everything Smoozy draws itself looks correct while the scrollbars give it away, which is why it is written down rather than left to be rediscovered.

**What it does not cover.** Anything about the window Smoozy will actually be built in. The mockup is a page in a browser, and whether the same rule holds in whatever Architecture chooses is that stage's to find out. What is settled here is that the theme has to be declared and not only painted; how, on the real thing, is not.

**What breaks if it is wrong.** Nothing about the design. It would mean the declaration is unnecessary in the built app, which costs nothing to have kept.
