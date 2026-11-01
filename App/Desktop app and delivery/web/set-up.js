// **Set up — a server's second tab.** The application half is
// `src/surface/set_up.rs`, whose head carries the piece.
//
// **Read the tab and panel contract at the head of `web/a-server.js` first.**
// This module is one of its two tabs: its host is made once for the whole run,
// it is hidden when the user goes away, and it is never taken down. Everything
// this file draws lives inside that host.
//
// **What this file does and does not decide.** It draws what the application
// hands over and decides nothing: not a word, not a mark, not which card the
// server is on, not what a step's mark means, not how many choices the
// admin-access box offers. **Every string here came from the application
// already through the typography point or the verbatim route** (A6, X9), and
// this file joins none of them (X7).
//
// **The four things that bind whatever a piece writes** are at the head of
// `web/smoozy.js`: results arrive as events; values are content and never
// markup (H32); no inline script and no inline style (H33); nothing repeats on
// a timer (P9, P10). This file keeps all four.
//
// **The classes are the drawing's own**, lifted whole into `the-drawing.css` —
// `setup`, `suscroll`, `sus`, `sucard`, `sustat`, `sdot`, `stx`, `sunote`,
// `sudid`, `sumiss`, `stm`, `sufix`, `stgrow`, `butable`, `burow`, `buhead`,
// `buv`, `bup`, `bua`, `btag`, `bwarn`, `modal`, `mbox`, `msub`, `mwarn`,
// `mrow`, `fmts`, `fmt`, `rb`, `cb`, `fmn`, `fmw`, `fmc`, `burule`, `buexp`,
// `bulive`, `steps`, `done`, `now`, `bad`, `skip`, `stnote`, `jl`, `jn`, `jw`,
// `frm`, `fld`, `choice3`, and the step marks' four colours — `stm-ok`,
// `stm-idle`, `stm-busy`, `stm-bad` — which the application names and this file
// never chooses. **`web/set-up.css` holds only what this shape needs that the
// drawing did not carry.**

// **A run is drawn on the card, in the card's place, and never in a box** —
// since 2026-10-02, the Creator's notes piece 2 (note 4 on 0.1.7: a box over
// the server's screen covered both tabs while a setup ran). The run while it
// goes, a setup that stopped part-way, and what the last act came to are all
// on the card; nothing on the server's screen is covered, and *Files* is one
// press away. **The other boxes stay boxes** — install, removal, the two
// warnings, admin access and the below-4.5 question are all questions.
// **Three of the drawings below are exported and nothing calls them yet, and
// that is stated rather than left to be wondered at.** The unpack tool's
// warning, the build going up and the below-the-floor warning are opened by a
// setup act that has reached the machine. *(The ready action's own warning is
// called since 2026-09-27 — `theWorkWarningIsAskedFor`.)* **The admin-access
// box is called** — from `Server Setup` E2's offer, since 2026-09-21 — **and is
// exported for one more reason**: `File Browser` reaches the same box from a
// refusal of its own, which is `Design/Server Setup § Admin access`'s *"this
// screen is one of the two places it is reached"*.

import { aTabIsRegistered, theCurrentTabIs, FILES, SET_UP } from "./a-server.js";
import { whenTold } from "./event-route.js";
import { theBrowserIsOpenedAt } from "./files.js";
import { theCurrentPlaceIs, TRANSFERS } from "./the-rail.js";
import { theFailureShape, EXPANDED } from "./the-failure-shape.js";
import { theWordsPanel, theNewestWordsStayInView } from "./the-machines-own-words.js";
import { theMarkOnAControl, theMarkOnARow, theMarkOverAPane } from "./the-underway-shape.js";
import { theValueIsDrawnInto } from "./the-value-drawn.js";
// **The connects that did not happen are kept on the server list's own
// page**, newest for each server, and this tab reads them from there rather
// than keeping a second copy (the Creator's note 14 on the test copy).
import { theConnectionFailureOn, theConnectionFailureIsTakenAway } from "./servers.js";

const shell = window.__TAURI__;

// The one kind this tab is told about — `THE_ANSWER` in
// `src/surface/set_up.rs`. The two sides have to agree and nothing holds them
// together, so copy carefully.
const THE_ANSWER = "set-up:the-answer";

// **The acts the application composes and this file hands straight back.** They
// are written here only so a control can be given one; **nothing here decides
// what an act means.**
const THE_READY_ACTION = "the-ready-action";
// `Server Setup` W3's *Set up anyway*: the ready action, with its warning
// already shown. Added 2026-09-27.
const THE_READY_ACTION_AFTER_THE_WARNING = "the-ready-action-after-the-warning";
const ABANDON = "abandon";
const STOP = "stop";
const INSTALL_A_BLENDER = "install-a-blender";
const REMOVE_A_BLENDER = "remove-a-blender";
// `Server Setup` G2's *Install it*, which the application answers.
const INSTALL_TELEGRAM = "install-telegram";
// `Servers` B6's connect that carries the remembered setup — the action of the
// card where the connection an add asked for was not made. Added 2026-10-03.
const CONNECT_AND_SET_UP = "connect-and-set-up";
// **`Server Setup` B7's pick and `Server Setup` B7a's two answers** — the 38's piece 14b.
const CHOOSE_A_BUILD = "choose-a-build";
const INSTALL_ANYWAY = "install-anyway";
const DONT_INSTALL_IT = "dont-install-it";
// **The answer to a connect, and the three it can end in when it never became
// live** — declared beside the service that posts them (`THE_ANSWER` and
// `THE_CONNECTION_WAS_NOT_MADE_ANSWER` with its two neighbours in
// `src/surface/servers.rs`). The two sides have to agree and nothing holds
// them together, so copy carefully.
const A_CONNECTS_ANSWER = "servers:the-answer";
const A_CONNECT_NOT_MADE = [
  "the-connection-was-not-made",
  "it-is-not-a-linux-machine",
  "the-passphrase-could-not-be-used",
];

// ── What this module holds ──────────────────────────────────────────────────

let host = null;
let scroll = null;
let card = null;
let table = null;
let boxes = null;
// Which server this tab is showing. **Not a second copy of anything durable** —
// it is which place the user is standing in, and the tab is one host that
// changes server (the contract's section 2).
let current = null;
// The last answer the application gave, so a box opened from the tab can be
// drawn from the same words the tab was.
let said = null;
// **The run this copy is driving on the server showing, as the application
// last answered it** — drawn on the card in the card's place, and never in a
// box (`Design/Server Setup § A long step`; the Creator's note 4 on 0.1.7).
// `{ server, panel }`, or nothing where no run is going. Which drawing is on
// screen, and nothing durable.
let theRun = null;
// **The last act's own ending, said as one line on the card** — server →
// answer — **while it is true: until the next setup act on that server
// replaces it, and never on a timer** (`Design/Server Setup § This server`,
// the runner's ruling (a) of 2026-10-03). Showing the tab again, or the
// server's screen being read again, does not take it away: the Vast pass
// before 0.1.9 watched *Install stopped.* leave the card 9 seconds after the
// stop with no press, when the run's ending moved the work column and the
// screen re-showed this tab. *(It was a box with Close until 2026-10-02.)*
// Which drawing is on screen, held for the run alone.
const theLastWords = new Map();
// **The steps a run's card already showed, kept from its ending until the
// result is read** — `Design/Server Setup § A long step`, the runner's ruling
// (b) of 2026-10-03: the card never goes back to the reading taken before the
// run (*No setup read from this server*) while the result is being read.
// `{ server, panel, from }`, where `from` is the first of this tab's readings
// that carries the result. Which drawing is on screen, and nothing durable.
let theEndedRun = null;
// **How many readings of the tab were asked for, and the newest that
// landed**, so the steps above stay until a reading asked after the run ended
// replaces them.
let theTabAsks = 0;
let theTabLanded = 0;
// **The Blender table's own over-the-pane mark** while it is read again after
// a run ended — the Creator's note 8 on 0.1.7. Only the table, never the card.
let theTableCover = null;
// **How many answers this tab has been told**, so a panel asked for on one
// answer is not drawn over a later one: a run's last answer can arrive while
// the panel of its previous step is still being asked for. Which answer is on
// screen, and nothing durable. Added 2026-09-27.
let theAnswersSoFar = 0;
// **The over-the-pane mark while the tab's first reading is taken** —
// `Design/Whole app § Anything that isn't instant shows that it's underway`:
// nothing on the pane can be used until the reading comes back, and the
// seventh pass on Hyperstack watched this tab stand blank for about 45 seconds
// while `Server Setup` B10's search ran. Added 2026-09-27, fix piece 9.
let theCover = null;
// **What the card and the table were last drawn from**, so an answer saying
// what the one before it said leaves the drawing where it is. Which drawing is
// on screen, and nothing durable. Cleared when the tab changes server.
let theDrawnCard = null;
let theDrawnTable = null;
// **Which server's ready action the application said is under way**, from the
// press until the next answer about that server — `Design/Whole app § Anything
// that isn't instant shows that it's underway`, *on the control*. Which
// drawing is on screen, and nothing durable. Added 2026-09-28, step 10c's
// ninth-pass fix piece 4.
let theActionIsUnderwayOn = null;
// **The window's lines as drawn, where a window is on the card**, so lines
// that moved are drawn into it in place and the card is not built again under
// the user's hand while the server prints — `Design/Server Setup § A long
// step`. Which drawing is on screen, and nothing durable. Added 2026-10-18.
let theWindow = null;
// ***Server output*, opened**: `{ server, panel, records, tabs }`, where
// `records` holds each stream's printed words and `tabs` its explanatory
// state, so both live arrivals and an empty stream's ending can be drawn.
// Which box is open, and nothing durable.
let theServersOutputOpen = null;
// **One reading of the server's words at a time**, and one more after it
// where they moved while it was asked: a step that prints fast moves them
// faster than an answer comes back.
let theWordsAreBeingAskedFor = false;
let theWordsMovedAgain = false;

// ── The tab ─────────────────────────────────────────────────────────────────

aTabIsRegistered({
  tab: SET_UP,

  itIsDrawn(into) {
    host = into;

    scroll = document.createElement("div");
    scroll.className = "suscroll";
    host.appendChild(scroll);

    // **The two sections, and there is no third.** `Design/Server Setup § Two
    // sections, and no third`: there is no add-ons section, and its absence is
    // a decision.
    card = document.createElement("section");
    card.className = "sus";
    scroll.appendChild(card);

    table = document.createElement("section");
    table.className = "sus";
    scroll.appendChild(table);

    // Where every box of this tab's is put. **Inside this tab's own host**, so
    // a box drawn here goes away with the tab.
    boxes = document.createElement("div");
    host.appendChild(boxes);

    // **Escape closes the box on top, and this tab handles its own.**
    // `Design/Whole app § Keyboard shortcuts`: *"Escape closes the thing on
    // top, one layer at a time. A box, then a panel, then a menu."* Every box
    // this tab opens carries `role="dialog"`, which is what `web/a-server.js`
    // steps aside for — so a box over a panel closes rather than collapsing the
    // panel under it.
    document.addEventListener("keydown", (press) => {
      if (press.key !== "Escape" || press.defaultPrevented) return;
      if (!boxes || !boxes.firstChild) return;
      if (!host || host.hidden) return;
      press.preventDefault();
      theBoxIsClosed();
      // **The tab is asked again, exactly as the box's own *Close* asks it.**
      // Added 2026-09-22, step 10c's fourth pass on Vast, fault 5: only the
      // box's control re-read the machine, so a box shut with Escape left the
      // card saying what it said before the act ran. **Nothing where no server
      // is showing**, there being nothing to ask about.
      if (current) theTabIsAskedFor(current);
    });
  },

  // **The contract's section 2: this is where the application is asked
  // again.** It runs on the first showing and again every time the user goes to
  // a different server while this tab is showing. Events are volatile and
  // nothing is replayed (P8).
  itIsShowing(server) {
    // **An open *Server output* survives the same server shown again**: its
    // words are still the record's, so only another server shuts it. The
    // Creator's twenty-third notes, note 3.
    const theOutputStays =
      current === server && theServersOutputOpen && theServersOutputOpen.server === server && theServersOutputOpen.panel.parentNode
        ? theServersOutputOpen.panel
        : null;
    // **A different server is a different drawing**, so nothing of the last
    // one is held against this one's answer — **and nothing of it stays on
    // screen**: the tab is one host for every server, and the mark over the
    // pane is see-through, so another server's card and Blender table stood
    // under it while this one's reading was taken. After 0.1.8, the checks'
    // leftovers, item 3: a new server's tab read *on the clock of* the other
    // server for 11 seconds.
    if (current !== server) {
      if (card) card.textContent = "";
      if (table) table.textContent = "";
      // **The run and its ended steps are the last server's drawing.** The
      // same server shown again keeps both: its run is asked for again below,
      // and an ended run's steps stay until its result is read (ruling (b)).
      theRun = null;
      theEndedRun = null;
    }
    current = server;
    if (said && said.server !== server) said = null;
    theDrawnCard = null;
    theDrawnTable = null;
    theActionIsUnderwayOn = null;
    // **The last act's line is not taken away here** (ruling (a)): it is
    // held per server and stays until the next setup act there replaces it.
    theTableCoverIsTakenAway();
    theBoxIsClosed();
    if (theOutputStays && boxes) {
      boxes.appendChild(theOutputStays);
      theServersOutputIsAskedAgain(server);
    }
    theTabIsAskedFor(server);
    // **Arriving while a run is going lands on its card** — `Design/Server
    // Setup § A second setup while one is running`: *"They land on the card
    // showing the run in flight and what it has reached."*
    theRunUnderWayIsAskedFor(server);
  },

  itIsPutAway() {
    // **A box of this tab's is shut when the tab stops showing.** It belongs to
    // the tab and not to the window, and a box left standing behind a hidden
    // tab is a control nobody can see and Tab can still reach.
    theBoxIsClosed();
  },
});

// **The one kind this tab is told about.** It carries one of two things: what
// an act came to, in one sentence with the machine's own words behind it where
// it printed any; or `Server Setup` E2's offer, naming the step that needs
// admin access (2026-09-21).
whenTold(THE_ANSWER, (delivery) => {
  const answer = delivery.said || {};
  // **An answer about another server is not this tab's** — the answer names
  // the server it is about (must-do 30).
  if (answer.server !== current || !host || host.hidden) {
    // **An offer nobody on this tab can see is declined**, so the run that
    // asked is not left waiting on a box that is not there.
    if (answer.the_step_that_was_refused) theOfferIsAnswered(answer.server, null);
    return;
  }
  // **The press is being answered**, and the control carries the mark until
  // the answer comes — `Design/Whole app § Anything that isn't instant shows
  // that it's underway`, *on the control*. The event carries no words. Added
  // 2026-09-28, step 10c's ninth-pass fix piece 4: the ninth pass pressed *Set
  // up* and saw nothing change for 11 seconds while `Server Setup` W3's reading
  // ran.
  if (answer.the_action_is_underway) {
    theActionIsMarkedUnderway(answer.server);
    return;
  }
  // **What the server printed moved** — `Server Setup` Y7a. It answers no
  // press, so it takes no mark off and counts as no answer; the window's
  // lines and an open *Server output* are asked for again. Added 2026-10-18.
  if (answer.the_servers_words_moved) {
    theServersWordsMoved(answer.server);
    return;
  }
  // **Any other answer about this server is the press answered**, so the mark
  // comes off — the warning owed, a run begun, or the line saying why nothing
  // began. No clock ends it (P9, P10).
  theMarkComesOffTheAction(answer.server);
  theAnswersSoFar += 1;
  if (answer.the_step_that_was_refused) {
    theAdminAccessBoxIsOpened(answer.the_step_that_was_refused, false, answer.server);
    return;
  }
  // **`Server Setup` W3's warning stands, and is asked for and drawn** —
  // `Design/Server Setup § A long step`: *"A step that could disturb work
  // warns first"*. The event carries no box; the application answers with the
  // one it composed. Its *Set up anyway* asks for the act again, as the act
  // that has had its warning. Added 2026-09-27.
  if (answer.the_work_warning_is_owed) {
    theWorkWarningIsAskedFor(answer.server);
    return;
  }
  // **A run moved on**, and its panel is asked for — `Design/Server Setup § A
  // long step`. The event carries no reading; the application answers from the
  // one its driving took. Added 2026-09-27, fix piece 9.
  if (answer.the_run_is_under_way) {
    theRunUnderWayIsAskedFor(answer.server);
    return;
  }
  if (!answer.the_act_is_not_built_yet) return;
  // **What the act came to is one line on the card, and never a box** — the
  // Creator's note 4 on 0.1.7: a box over the server's screen covered both
  // tabs. **The application says where the card already says it** — a setup
  // that reached its end, a Blender installed — and there is no second line.
  if (answer.the_card_says_it) theLastWords.delete(answer.server);
  else theLastWords.set(answer.server, answer);
  // **The Blender table carries the mark while it is read again**, and only
  // the table — the Creator's note 8 on 0.1.7: a new build appeared about 3
  // seconds late, with nothing saying the table was being read.
  if (answer.the_removed_blender && said && said.server === answer.server) {
    const rows = said.the_table.rows.filter((build) => build.path !== answer.the_removed_blender);
    said = { ...said, the_table: { ...said.the_table, rows,
      instead: rows.length === 0 ? answer.no_blender_reading : said.the_table.instead,
    } };
    theTableIsDrawn(said);
  }
  if (answer.the_blenders_are_read_again) theTableIsCoveredWhileItIsReadAgain();
  // **The run is asked for again**, and an ended run answers nothing, so the
  // card goes back to the machine's own reading.
  theRunUnderWayIsAskedFor(answer.server);
  // **An open *Server output* reads as the run left it.**
  theServersOutputIsAskedAgain(answer.server);
  theCardIsDrawnWhereItChanged(said);
  // **The card is read again the moment an act ends.** Added 2026-09-22, step
  // 10c's fourth pass on Vast, fault 5. **This is not a timer** (P9, P10): it
  // fires on the one event that says the machine changed, and the card is
  // `Server Setup` S1 and S2's fresh reading over the connection.
  theTabIsAskedFor(answer.server);
});

// **The server's connection moved**, so the tab and the run are asked for
// again — `Server Setup` S1: the card is read fresh on the connection the user
// opens. *Add and set up* lands here while the connection is being made, and
// the card reads *Connecting…* until it is (the Creator's note 3 on 0.1.7);
// a connection that failed takes the card back to what is true. Declared
// beside the service that posts them — `WHAT_IS_OBSERVED` and
// `THE_CONNECTION_CONDITION` in `src/servers/published.rs`. **Only for the
// server showing**, and nothing here ticks.
for (const kind of ["servers:what-is-observed", "servers:the-connection-condition"]) {
  whenTold(kind, (delivery) => {
    if (!host || host.hidden || !current || delivery.subject !== current) return;
    theTabIsAskedFor(current);
    theRunUnderWayIsAskedFor(current);
  });
}

// **A connect on the server showing did not happen**, so the card is asked
// for again and says so where the user is — `Design/Server Setup § This
// server`, the Creator's note 14 on the test copy: *Add and set up* lands
// here, and the sentence used to be written only above the All servers list,
// which was hidden. **`web/servers.js` keeps it**, and its own listener runs
// first: this module imports it, so it registered before this one did.
whenTold(A_CONNECTS_ANSWER, (delivery) => {
  const said = delivery.said || {};
  if (!A_CONNECT_NOT_MADE.includes(said.what) || !said.server) return;
  if (!host || host.hidden || said.server !== current) return;
  theTabIsAskedFor(current);
});

// ── Asking, and drawing what comes back ─────────────────────────────────────

function theTabIsAskedFor(server) {
  // **The first reading of a server is covered, and a later one is not**: a
  // card already drawn is still the last reading and stays usable while the
  // next is taken, which is Design's *the narrowest shape that is true*.
  if (theDrawnCard === null && !theEndedRun) theCoverIsShown();
  theTabAsks += 1;
  const asking = theTabAsks;
  // **What the last connect on this server that did not happen ended in**,
  // where one is kept — the application turns it into the card's sentence,
  // and only for a server it has not read.
  const failed = theConnectionFailureOn(server);
  shell.core
    .invoke("the_set_up_tab", { server, failed: failed ? failed.what : null })
    .then((answer) => {
      if (current !== server || asking < theTabAsks) return;
      theTabLanded = Math.max(theTabLanded, asking);
      // **A reading asked after the run ended is its result**, and it
      // replaces the steps kept since the ending (ruling (b)).
      if (theEndedRun && theEndedRun.server === server && asking >= theEndedRun.from) {
        theEndedRun = null;
      }
      theCoverIsTakenAway();
      said = answer;
      theTabIsDrawn(answer);
    })
    .catch(() => {
      theCoverIsTakenAway();
      // **Nothing is drawn here**: the sentence would be the application's
      // (A6, X1) and the application is what did not answer, so there is none
      // to draw and this file may not write one.
      if (host) host.dataset.setUp = "failed";
    });
}

/// **Design's *over the pane*, while the tab's first reading is taken.** The
/// shape is the shared one and is not drawn again here; it covers the tab's own
/// scrolling pane and nothing else on the screen.
function theCoverIsShown() {
  if (!scroll || theCover) return;
  theCover = theMarkOverAPane();
  scroll.appendChild(theCover);
}

function theCoverIsTakenAway() {
  if (theCover) theCover.remove();
  theCover = null;
}

/// **The run this copy is driving, asked for and drawn on the card** —
/// `Design/Server Setup § A long step`. **An answer of nothing takes the run
/// card down**, and the card goes back to the machine's own reading. An answer
/// that arrives after a later one was told is not drawn. Added 2026-09-27, fix
/// piece 9; on the card since 2026-10-02.
function theRunUnderWayIsAskedFor(server) {
  const asked = theAnswersSoFar;
  shell.core.invoke("the_setup_under_way", { server }).then((panel) => {
    if (current !== server || !host) return;
    if (asked !== theAnswersSoFar) return;
    if (!panel || panel.the_run_has_ended) {
      if (!theRun) return;
      // **Ended before the answer that says so** — the run's last telling
      // asked for after the run let go: its own last steps stand on the card
      // until that answer comes and its result is read, rather than the card
      // going back to the reading taken before the run (2026-10-26, fault 4).
      if (panel && theTabLanded >= theTabAsks) {
        theRun = { server, panel: theStepsAsTheRunLeftThem(panel) };
        theCardIsDrawnWhereItChanged(said);
        return;
      }
      // **The run ended: its steps stay until its result is read** — the
      // runner's ruling (b) of 2026-10-03. The Vast pass before 0.1.9 watched
      // the card read *No setup read from this server* for 10 seconds here:
      // the reading taken before the run, drawn again while the next one was
      // taken. Where that next reading already landed there is nothing to wait
      // for. **The steps are the run's own last reading where the application
      // answers it** (2026-10-26, the worldwide pass's setup faults, fault 4):
      // the last panel drawn here could be from before the last step was done.
      if (theTabLanded < theTabAsks) {
        theEndedRun = { server, panel: theStepsAsTheRunLeftThem(panel || theRun.panel), from: theTabAsks };
      }
      theRun = null;
      theCardIsDrawnWhereItChanged(said);
      return;
    }
    // **A run under way on this server is the next setup act there**, so the
    // last act's line goes (ruling (a)), and steps kept from an earlier run
    // are not this one's.
    theLastWords.delete(server);
    theEndedRun = null;
    theRun = { server, panel };
    // **`Server Setup` B7a's question, while the run waits on it**, drawn once
    // per question as `su-low` draws it.
    if (panel.the_question && theQuestionIsUp !== server) {
      theQuestionIsUp = server;
      theBelowTheFloorWarningIsDrawn(panel.the_question, server);
    } else if (!panel.the_question && theQuestionIsUp === server) {
      theQuestionIsUp = null;
    }
    // **The run is under way, so the card is not covered** — the first
    // reading's mark over the pane would sit over *Cancel setup*.
    theCoverIsTakenAway();
    theCardIsDrawnWhereItChanged(said);
  });
}

function theTabIsDrawn(answer) {
  if (host) delete host.dataset.setUp;
  // **A reading that says what the last one said rebuilds nothing.** Added
  // 2026-09-22, step 10c's fourth pass on Vast, fault 5. The tab is now asked
  // at every act's ending as well as on a tab switch, and a card taken down
  // and built again puts its controls back under whatever the hand was already
  // doing — one press of that pass, at 16:54:58, reached nothing because of a
  // redraw. **Nothing here decides what is drawn**: it decides only whether
  // this answer differs from the one on screen, and an answer that differs is
  // drawn in full.
  theCardIsDrawnWhereItChanged(answer);
  const table_now = JSON.stringify([
    answer.blender,
    answer.read_from_the_server,
    answer.the_table,
  ]);
  if (theDrawnTable !== table_now) {
    theDrawnTable = table_now;
    theTableIsDrawn(answer);
  }
  theTableCoverIsTakenAway();
}

/// **The card, drawn only where what it would show changed** — which includes
/// whether the press of its action is being answered.
///
/// **Which card is the application's, and this only picks the one it was
/// handed**: a run this copy is driving outranks everything, then a setup that
/// stopped part-way (`the_stopped_setup`), then the card. Each is drawn in the
/// card's place, never in a box.
function theCardIsDrawnWhereItChanged(answer) {
  const going = theRun && theRun.server === current ? theRun.panel : null;
  // **A run that ended keeps its steps until its result is read** (ruling (b)).
  const run = going || (theEndedRun && theEndedRun.server === current ? theEndedRun.panel : null);
  const stopped = !run && answer && answer.server === current ? answer.the_stopped_setup : null;
  const word = theLastWords.get(current) || null;
  if (!run && !(answer && answer.server === current)) return;
  // **The window's lines are left out of what is compared**, and only whether
  // there is a window counts: lines that moved are drawn into the window in
  // place, below, and the card is not built again for them.
  const lines = run ? run.the_servers_newest_lines || [] : [];
  const errors = run ? run.which_lines_are_from_the_error_stream || [] : [];
  const card_now = JSON.stringify([
    run ? { ...run, the_servers_newest_lines: lines.length > 0, which_lines_are_from_the_error_stream: null } : null,
    stopped,
    run || stopped
      ? null
      : [
          answer.this_server,
          answer.the_card,
          answer.the_card_was_last_seen,
          answer.the_driver,
          answer.the_driver_warning,
          answer.the_installs_never_begun,
          answer.server_output,
        ],
    word,
    !run && !stopped && theActionIsUnderwayOn === answer.server,
  ]);
  if (theDrawnCard === card_now) {
    if (theWindow) theWindowsLinesAreDrawnInto(theWindow, lines, errors);
    return;
  }
  theWindow = null;
  theDrawnCard = card_now;
  if (run || stopped) theRunCardIsDrawn(run || stopped, current);
  else theCardIsDrawn(answer);
}

/// **The application said the ready action is under way on this server**: the
/// card's action carries the mark, and so does any *Set up again* standing in
/// a box. **The card is drawn again from the last reading**, so the mark is the
/// shared shape on the control that reading drew and nothing is read again.
function theActionIsMarkedUnderway(server) {
  theActionIsUnderwayOn = server;
  if (said && said.server === server) theCardIsDrawnWhereItChanged(said);
  if (!boxes) return;
  for (const control of boxes.querySelectorAll(`[data-act="${THE_READY_ACTION}"]`)) {
    theMarkOnAControl(control, control.textContent);
  }
}

/// **The press is answered, and the card's action comes back.** A box's own
/// control goes with the box the answer replaces it with.
function theMarkComesOffTheAction(server) {
  if (theActionIsUnderwayOn !== server) return;
  theActionIsUnderwayOn = null;
  if (said && said.server === server) theCardIsDrawnWhereItChanged(said);
}

/// **A control that asks for the ready action**, marked as such so the press
/// can put the mark on it.
function theReadyActionsOwn(control) {
  control.dataset.act = THE_READY_ACTION;
  return control;
}

// ── The *This server* card ──────────────────────────────────────────────────

/// **One state line, when it was done, one action, and a list of what was
/// done. Nothing else.** `Design/Server Setup § This server`.
function theCardIsDrawn(answer) {
  card.textContent = "";

  const heading = document.createElement("h2");
  heading.textContent = answer.this_server;
  card.appendChild(heading);

  const drawn = answer.the_card;
  const one = document.createElement("div");
  one.className = "sucard";
  // **What was last seen reads as such: greyed, and saying when it was read**
  // — `Server Setup` Y6a, `Design/Whole app`'s rule for anything shown from
  // memory. The stamp is the application's words and this computer's own
  // moment, written here as a moment. The 38's piece 6.
  const lastSeen = answer.the_card_was_last_seen;
  if (lastSeen) one.classList.add("stale");

  const state = document.createElement("div");
  state.className = "sustat";
  const dot = document.createElement("span");
  // **The drawing's own three dots, marked by the value the application sent**
  // and never derived here.
  dot.className = "sdot";
  dot.dataset.dot = drawn.dot;
  state.appendChild(dot);
  const words = document.createElement("span");
  words.className = "stx";
  const bold = document.createElement("b");
  bold.textContent = drawn.state;
  words.appendChild(bold);
  state.appendChild(words);
  const grow = document.createElement("span");
  grow.className = "grow";
  state.appendChild(grow);
  if (drawn.when) state.appendChild(theMoment(drawn.when));
  if (lastSeen) {
    const stamp = document.createElement("span");
    stamp.className = "sunote seen";
    for (const part of [lastSeen.lead, ...lastSeen.values.map(theMomentsText)]) {
      if (!part) continue;
      const one = document.createElement("span");
      one.textContent = part;
      stamp.appendChild(one);
      stamp.appendChild(document.createTextNode(" "));
    }
    state.appendChild(stamp);
  }
  one.appendChild(state);
  theLastWordIsDrawnInto(one);
  // **The installs an earlier run of Smoozy closed before it began**, one line
  // under the state line as the last act's own is — `Server Setup` Y9a,
  // `Design/Server Setup § Installing`. The words are the application's, and
  // whether there is a line is the application's too. Added 2026-10-16, step
  // 10c's owed piece 4.
  if (answer.the_installs_never_begun) {
    const line = document.createElement("p");
    line.className = "sunote";
    line.textContent = answer.the_installs_never_begun;
    one.appendChild(line);
  }

  if (drawn.instead) {
    const told = document.createElement("p");
    told.className = "sunote";
    saidInto(told, drawn.instead);
    one.appendChild(told);
  }

  let list = null;
  if (drawn.what_was_done.length) {
    list = document.createElement("ul");
    list.className = "sudid";
    for (const line of drawn.what_was_done) {
      const item = document.createElement("li");
      if (line.missing) item.className = "sumiss";
      saidInto(item, line.said);
      // **The drawing's order: the words, the control, then the mark.**
      if (line.offer && line.offer.act === INSTALL_TELEGRAM) {
        item.appendChild(document.createTextNode(" "));
        const offer = document.createElement("button");
        offer.type = "button";
        offer.className = "mini";
        offer.textContent = line.offer.name;
        offer.addEventListener("click", () => theActIsAskedFor(INSTALL_TELEGRAM, current));
        item.appendChild(offer);
      }
      if (line.mark) {
        item.appendChild(document.createTextNode(" "));
        const mark = document.createElement("span");
        mark.className = "stm";
        // **The mark's colour is the value the application sent** — never
        // derived from the words here.
        if (line.colour) mark.classList.add(line.colour);
        mark.textContent = line.mark;
        item.appendChild(mark);
      }
      list.appendChild(item);
    }
  }
  // ***Server output* stays beside the list once the run has ended**, opening
  // what it printed, until the next run or until Smoozy closes —
  // `Design/Server Setup § A long step`. There is no window here.
  for (const done of theServersWordsBeside(list, answer, current)) one.appendChild(done);

  // **The driver line, and `Servers` H11b's warning under it** — under the
  // steps, beside Telegram reporting's line (`Design/Server Setup § This
  // server`). Both are the server's own reading and are the application's
  // words; this only sets them side by side. Added 2026-10-11.
  if (answer.the_driver) one.appendChild(theDriverLine(answer.the_driver));
  if (answer.the_driver_warning) {
    const warning = answer.the_driver_warning;
    const line = document.createElement("p");
    line.className = "sunote amber";
    if (warning.last_seen) line.classList.add("stale");
    const said = document.createElement("span");
    said.textContent = warning.said;
    line.appendChild(said);
    line.appendChild(document.createTextNode(" "));
    const and = document.createElement("span");
    and.textContent = warning.and;
    line.appendChild(and);
    one.appendChild(line);
  }

  if (drawn.the_render_device) {
    const line = document.createElement("p");
    line.className = "sunote";
    saidInto(line, drawn.the_render_device);
    one.appendChild(line);
  }

  if (drawn.the_note) {
    const note = document.createElement("p");
    note.className = "sunote";
    saidInto(note, drawn.the_note);
    one.appendChild(note);
  }

  // **The one state where the action earns being loud** — the amber sentence
  // and the control beside it.
  if (drawn.the_setup_fault) {
    const fix = document.createElement("div");
    fix.className = "sufix";
    const line = document.createElement("p");
    line.className = "sunote amber";
    const first = document.createElement("b");
    first.textContent = drawn.the_setup_fault.said;
    line.appendChild(first);
    line.appendChild(document.createTextNode(" "));
    const second = document.createElement("span");
    second.textContent = drawn.the_setup_fault.what_to_do;
    line.appendChild(second);
    fix.appendChild(line);
    fix.appendChild(theActionControl(answer, true));
    one.appendChild(fix);
    card.appendChild(one);
    return;
  }

  const row = document.createElement("div");
  row.className = "stgrow";
  row.appendChild(theActionControl(answer, false));
  one.appendChild(row);
  card.appendChild(one);
}

/// **The driver line** — the driver's words and its version, then the CUDA
/// version where the driver gave one, each value drawn as a value. **Greyed,
/// with *last seen* and the moment, where the reading is not this
/// connection's own** (`Servers` H11b); the stamp is written as the card's own
/// is.
function theDriverLine(driver) {
  const line = document.createElement("p");
  line.className = "sunote";
  if (driver.last_seen) line.classList.add("stale");
  line.appendChild(saidInto(document.createElement("span"), driver.the_driver));
  if (driver.the_cuda) {
    line.appendChild(document.createTextNode(" "));
    line.appendChild(saidInto(document.createElement("span"), driver.the_cuda));
  }
  if (driver.last_seen) {
    for (const part of [driver.last_seen.lead, ...driver.last_seen.values.map(theMomentsText)]) {
      if (!part) continue;
      line.appendChild(document.createTextNode(" "));
      const one = document.createElement("span");
      one.textContent = part;
      line.appendChild(one);
    }
  }
  return line;
}

/// **The card's one action.** It is never greyed out — the user may want it for
/// a reason Smoozy cannot see.
function theActionControl(answer, inTheFix) {
  const drawn = answer.the_card.the_action;
  const control = document.createElement("button");
  control.type = "button";
  control.className = inTheFix || drawn.loud ? "btn" : "mini";
  control.textContent = drawn.name;
  // **Under way on itself, or because its press is being answered** — the
  // second added 2026-09-28, step 10c's ninth-pass fix piece 4.
  if (drawn.underway || theActionIsUnderwayOn === answer.server) {
    // **The action goes underway on itself** — `Design/Server Setup § A second
    // setup while one is running`: a control that is pressed and then refused
    // is a control that should not have been pressable. **The shape is the one
    // that exists** and is not rebuilt here.
    theMarkOnAControl(control, drawn.name);
    return control;
  }
  control.addEventListener("click", () =>
    theActIsAskedFor(drawn.act, answer.server)
  );
  return control;
}

// ── The Blender table ───────────────────────────────────────────────────────

/// **A table of what is installed, read from the server every time.** The
/// heading says so, because anything remembered would be a guess dressed as a
/// fact.
function theTableIsDrawn(answer) {
  table.textContent = "";

  const heading = document.createElement("h2");
  heading.textContent = answer.blender;
  const note = document.createElement("span");
  note.className = "h2n";
  note.textContent = answer.read_from_the_server;
  heading.appendChild(document.createTextNode(" "));
  heading.appendChild(note);
  table.appendChild(heading);

  const drawn = answer.the_table;
  if (drawn.instead) {
    const told = document.createElement("p");
    told.className = "sunote";
    saidInto(told, drawn.instead);
    table.appendChild(told);
  } else {
    const grid = document.createElement("div");
    grid.className = "butable";

    const head = document.createElement("div");
    head.className = "burow buhead";
    for (const column of [drawn.version, drawn.location, ""]) {
      const cell = document.createElement("span");
      cell.textContent = column;
      head.appendChild(cell);
    }
    grid.appendChild(head);

    for (const build of drawn.rows) grid.appendChild(aBuildRow(answer, build));
    table.appendChild(grid);
  }

  const install = document.createElement("button");
  install.type = "button";
  install.className = "btn";
  install.textContent = drawn.install;
  install.addEventListener("click", () => theInstallBoxIsOpened(answer.server));
  table.appendChild(install);
}

/// **Three columns: version, where it sits, and Remove.**
function aBuildRow(answer, build) {
  const row = document.createElement("div");
  row.className = "burow";

  const version = document.createElement("span");
  version.className = "buv mono";
  version.textContent = build.version;
  // **Two things the version cannot say get a small tag instead.**
  if (build.tag) {
    version.appendChild(document.createTextNode(" "));
    const tag = document.createElement("span");
    tag.className = "btag";
    tag.textContent = build.tag;
    version.appendChild(tag);
  }
  if (build.below_the_floor) {
    version.appendChild(document.createTextNode(" "));
    const warn = document.createElement("span");
    warn.className = "bwarn";
    warn.textContent = build.below_the_floor;
    version.appendChild(warn);
  }
  // **`Server Setup` B12b's tag, drawn like *below 4.5*** — the application
  // says which rows carry it, from this connection's own driver reading.
  // Added 2026-10-11.
  if (build.driver_too_old) {
    version.appendChild(document.createTextNode(" "));
    const warn = document.createElement("span");
    warn.className = "bwarn";
    warn.textContent = build.driver_too_old;
    version.appendChild(warn);
  }
  row.appendChild(version);

  // **The link is on every row, including the ones it cannot help with** —
  // withholding it would be Smoozy deciding which folders the user may look at.
  const folder = document.createElement("button");
  folder.type = "button";
  folder.className = "bup mono golink";
  // **A Blender's folder reads in the order its bytes run** — X9,
  // `web/the-value-drawn.js`, and so does every name below.
  theValueIsDrawnInto(folder, build.folder);
  folder.addEventListener("click", () => theFolderIsOpened(build));
  row.appendChild(folder);

  const acts = document.createElement("span");
  acts.className = "bua";
  const remove = document.createElement("button");
  remove.type = "button";
  remove.className = "mini";
  remove.textContent = build.remove;
  remove.addEventListener("click", () =>
    theRemovalBoxIsOpened(answer.server, build.path)
  );
  acts.appendChild(remove);
  row.appendChild(acts);
  return row;
}

/// **The folder column links into `File Browser` at that folder** — `Server
/// Setup` B12a. Files is handed the folder's bytes, exactly as the application
/// gave them, and then shown (must-do 30 of the step 10a round 1 audit of these
/// pieces; piece 7 had recorded the link as owed until piece 6 existed).
function theFolderIsOpened(build) {
  theBrowserIsOpenedAt(current, build.folder_bytes);
  theCurrentTabIs(FILES);
}

// ── The boxes ───────────────────────────────────────────────────────────────

/// **Every box this tab opens says it is a box.**
///
/// `role="dialog"` is what `web/a-server.js` steps aside for, so a box opened
/// over a collapsed panel's server closes on Escape rather than collapsing the
/// panel under it. Piece 5's own account asks for exactly this.
function aBox() {
  anOfferIsDeclined();
  boxes.textContent = "";
  const modal = document.createElement("div");
  modal.className = "modal top";
  const box = document.createElement("div");
  box.className = "mbox";
  box.setAttribute("role", "dialog");
  box.setAttribute("aria-modal", "true");
  modal.appendChild(box);
  boxes.appendChild(modal);
  return box;
}

// **The server whose `Server Setup` B7a question box is up**, so it is drawn once.
let theQuestionIsUp = null;

function theBoxIsClosed() {
  anOfferIsDeclined();
  theQuestionIsUp = null;
  if (boxes) boxes.textContent = "";
}

// ── `Server Setup` E2's offer ───────────────────────────────────────────────

// **The server whose run is waiting on the admin-access box, while it stands.**
// Not a copy of anything durable: it is which box is open. **Any way the box
// goes away other than *Try again* declines the offer** — *Cancel*, Escape, the
// tab being put away, another box replacing it — so the run that asked is never
// left waiting on a box nobody can see.
let theOfferStandsFor = null;

function anOfferIsDeclined() {
  if (theOfferStandsFor === null) return;
  const server = theOfferStandsFor;
  theOfferStandsFor = null;
  theOfferIsAnswered(server, null);
}

/// **The offer, answered**: the password the user typed, or nothing. It goes
/// to the run that asked and nowhere else; nothing here keeps it.
function theOfferIsAnswered(server, password) {
  shell.core.invoke("the_admin_access_is_answered", { server, password });
}

/// A row of controls along the bottom of a box.
function aRow(box) {
  const row = document.createElement("div");
  row.className = "mrow";
  const grow = document.createElement("span");
  grow.className = "grow";
  row.appendChild(grow);
  box.appendChild(row);
  return row;
}

function aControl(label, pressed, quiet) {
  const control = document.createElement("button");
  control.type = "button";
  control.className = quiet ? "btn quiet" : "btn";
  control.textContent = label || "";
  control.addEventListener("click", pressed);
  return control;
}

// ── Installing — `Design/Server Setup § Installing` ─────────────────────────

/// **The list is what Blender publishes, in two parts with a rule between
/// them.** The question a user has here is which of these is safe to render a
/// night's work with, and a rule answers it before any entry is read.
function theInstallBoxIsOpened(server) {
  const box = aBox();
  // **Design's *over the pane* while Blender's list is read** — the box has
  // nothing the user can use until the shop-front answers, and it stood empty
  // for the whole fetch. The shared shape, not one drawn here; the drawing
  // below empties the box, and the mark with it. Added 2026-09-27, fix piece
  // 9.
  box.classList.add("reading");
  box.appendChild(theMarkOverAPane());
  shell.core.invoke("the_install_list", { server }).then((list) => {
    box.classList.remove("reading");
    theInstallBoxIsDrawn(box, list, server);
  });
}

/// The box, with the list handed in — **so the drawing and the fetch are two
/// things**, which is what lets the same drawing be watched against a seeded
/// list.
function theInstallBoxIsDrawn(box, list, server) {
  {
    box.textContent = "";
    const title = document.createElement("h3");
    title.textContent = list.title;
    box.appendChild(title);

    const source = document.createElement("p");
    source.className = "msub";
    source.textContent = list.source;
    box.appendChild(source);

    if (list.instead) {
      const told = document.createElement("p");
      told.className = "mwarn";
      saidInto(told, list.instead);
      box.appendChild(told);
    }

    // **Each entry has a tick box, and several can be ticked at once** —
    // `Design/Server Setup § Installing`, the Creator's third notes, note 2
    // (2026-10-06): *"Blender installations should be checkboxes, not
    // radios."* They install one after another in the order listed, each its
    // own run, and the application begins each when the last one's run ends.
    // *(Radios until 2026-10-08.)*
    const ticked = new Set();
    const marks = [];
    const entries = [];
    // **The ticked entries in the order the list shows them**, whatever order
    // they were ticked in.
    const theTicked = () => entries.filter((entry) => ticked.has(entry));
    const aPart = (theseEntries) => {
      const part = document.createElement("div");
      part.className = "fmts";
      for (const entry of theseEntries) {
        const mark = document.createElement("label");
        mark.className = "fmt";
        mark.setAttribute("role", "checkbox");
        mark.setAttribute("aria-checked", "false");
        mark.tabIndex = 0;
        const bullet = document.createElement("span");
        bullet.className = "cb";
        mark.appendChild(bullet);
        const version = document.createElement("span");
        version.className = "fmn mono";
        version.textContent = entry.version;
        mark.appendChild(version);
        const line = document.createElement("span");
        line.className = "fmw";
        line.textContent = entry.line;
        mark.appendChild(line);
        const on = document.createElement("span");
        on.className = "fmc";
        if (entry.mark) saidInto(on, entry.mark);
        mark.appendChild(on);
        const pick = () => {
          const it = !ticked.has(entry);
          if (it) ticked.add(entry);
          else ticked.delete(entry);
          mark.classList.toggle("on", it);
          mark.setAttribute("aria-checked", String(it));
          bullet.classList.toggle("on", it);
          bullet.textContent = it ? "✓" : "";
          theButtonCounts();
        };
        mark.addEventListener("click", pick);
        mark.addEventListener("keydown", (press) => {
          if (press.key === " " || press.key === "Enter") {
            press.preventDefault();
            pick();
          }
        });
        marks.push(mark);
        entries.push(entry);
        part.appendChild(mark);
      }
      return part;
    };

    box.appendChild(aPart(list.finished));

    if (list.unfinished.length) {
      const rule = document.createElement("div");
      rule.className = "burule";
      box.appendChild(rule);
      // **The heading, with nothing explaining it** — `Design/Server Setup §
      // Installing`, Creator-stated 2026-09-27; the drawing's `h4.buexp`.
      const heading = document.createElement("h4");
      heading.className = "buexp";
      heading.textContent = list.the_heading;
      box.appendChild(heading);
      box.appendChild(aPart(list.unfinished));
    }

    // **Pointing at a build on your own computer.** The pick is the system's
    // own file chooser and Smoozy draws none of it.
    const or = document.createElement("p");
    or.className = "msub";
    const before = document.createElement("span");
    before.textContent = list.or;
    or.appendChild(before);
    or.appendChild(document.createTextNode(" "));
    const point = document.createElement("button");
    point.type = "button";
    point.className = "lnkb";
    point.textContent = list.choose_a_build;
    // **The list box closes and the system's picker opens** (K11 to K13).
    point.addEventListener("click", () => {
      theBoxIsClosed();
      theActIsAskedFor(CHOOSE_A_BUILD, server);
    });
    or.appendChild(point);
    or.appendChild(document.createTextNode(" "));
    const after = document.createElement("span");
    after.textContent = list.what_smoozy_checks;
    or.appendChild(after);
    box.appendChild(or);

    if (list.short) {
      const short = document.createElement("p");
      short.className = "msub";
      saidInto(short, list.short);
      box.appendChild(short);
    }

    const row = aRow(box);
    row.appendChild(aControl(list.cancel, () => theBoxIsClosed(), true));
    const install = aControl(list.install, () => {
      const picked = theTicked();
      if (!picked.length) return;
      // **Each picked entry's version goes with the act, as the application
      // composed it to be carried**, and nothing else of it: `Server Setup` B4
      // derives every address on the machine — from the version, or from the
      // build service's constant base and the bare file name carried beside it.
      // **The first is installed now and the rest wait behind it**, in the
      // order listed. **The box closes at the press**, so the card under it —
      // the run, or the line it joined — is what answers (`Design/Server
      // Setup § Asking for more while a setup runs`, 2026-10-27).
      theBoxIsClosed();
      theActIsAskedFor(
        INSTALL_A_BLENDER,
        server,
        picked[0].asked,
        picked.slice(1).map((entry) => entry.asked)
      );
    });
    row.appendChild(install);

    // **The button counts what is ticked** — *Install 5.2.2* for one, *Install
    // 3 versions* for several, and the plain word for none. The words are the
    // application's; the figure is how many boxes are ticked here.
    function theButtonCounts() {
      const picked = theTicked();
      install.textContent = list.install;
      if (!picked.length) return;
      install.appendChild(document.createTextNode(" "));
      const named = document.createElement("span");
      named.textContent = picked.length === 1 ? picked[0].version : String(picked.length);
      install.appendChild(named);
      if (picked.length === 1) return;
      install.appendChild(document.createTextNode(" "));
      const unit = document.createElement("span");
      unit.textContent = list.versions;
      install.appendChild(unit);
    }
  }
}

// ── Removing a Blender ──────────────────────────────────────────────────────

/// **Where a live job needs it, both jobs are named and each is told the truth
/// about itself.** The two outcomes are not alike.
function theRemovalBoxIsOpened(server, path) {
  shell.core
    .invoke("what_removing_a_blender_costs", { server, path })
    .then((warning) => theRemovalBoxIsDrawn(warning, server));
}

/// The box, with the warning handed in.
function theRemovalBoxIsDrawn(warning, server) {
  {
    {
      const box = aBox();
      const title = document.createElement("h3");
      saidInto(title, warning.title);
      box.appendChild(title);

      if (warning.how_many) {
        const many = document.createElement("p");
        many.className = "mwarn hard";
        saidInto(many, warning.how_many);
        box.appendChild(many);
      }

      if (warning.the_live_jobs.length) {
        const list = document.createElement("div");
        list.className = "jl";
        for (const job of warning.the_live_jobs) {
          const named = document.createElement("span");
          named.className = "jn";
          theValueIsDrawnInto(named, job.job);
          list.appendChild(named);
          const said = document.createElement("span");
          said.className = "jw";
          said.textContent = job.said;
          list.appendChild(said);
        }
        box.appendChild(list);
      }

      // **What removal leaves behind**, which changes what somebody about to
      // press *Remove anyway* would do.
      const left = document.createElement("p");
      left.className = "msub";
      left.textContent = warning.what_is_left;
      box.appendChild(left);

      // **Where this module has no carriage for the removal**, the box says so
      // and offers no control — a control that is pressed and then refused is a
      // control that should not have been pressable.
      if (warning.smoozy_cannot_perform_it) {
        const cannot = document.createElement("p");
        cannot.className = "mwarn";
        cannot.textContent = warning.smoozy_cannot_perform_it;
        box.appendChild(cannot);
      }

      const row = aRow(box);
      row.appendChild(aControl(warning.cancel, () => theBoxIsClosed(), true));
      if (warning.remove) {
        // **The build goes back by its executable's path** (`Server Setup`
        // B13), which the application handed in with the warning — the 38's
        // piece 14c. The application decides everything else.
        // **The box closes at the press**, so the card's *Waiting* list — or
        // the removal's own run — is what answers it (2026-10-27).
        const remove = aControl(warning.remove, () => {
          theBoxIsClosed();
          theActIsAskedFor(REMOVE_A_BLENDER, server, null, null, warning.path);
        });
        remove.className = "btn danger";
        row.appendChild(remove);
      }
    }
  }
}

// ── Admin access ────────────────────────────────────────────────────────────

/// **Asked only when a step is actually refused, and never before.**
export function theAdminAccessBoxIsOpened(step, thePasswordCouldNotBeRead, server) {
  shell.core
    .invoke("the_admin_access_box", {
      theStepThatWasRefused: step,
      theSystemWouldNotHandThePasswordBack: Boolean(thePasswordCouldNotBeRead),
    })
    .then((asked) => theAdminAccessBoxIsDrawn(asked, server));
}

/// The box, with the answer handed in.
export function theAdminAccessBoxIsDrawn(asked, server) {
  {
    {
      const box = aBox();
      // **The run on this server is waiting on this box from here on.**
      if (server) theOfferStandsFor = server;
      let field = null;
      const title = document.createElement("h3");
      title.textContent = asked.title;
      box.appendChild(title);

      // **The line that names the step that was refused is the only thing that
      // changes between callers**, so each caller brings its own.
      const which = document.createElement("p");
      which.className = "msub";
      saidInto(which, asked.which_step);
      box.appendChild(which);

      // **Where the store's standing could not be read, the box says so** and
      // offers what a store never proven is offered (must-do 32).
      if (asked.the_standing_could_not_be_read) {
        const unread = document.createElement("p");
        unread.className = "mwarn";
        unread.textContent = asked.the_standing_could_not_be_read;
        box.appendChild(unread);
      }

      // **`Security and licence` S34's one sentence, said once for the life of
      // the installation**, above the field — `Design/Keys § On a computer with
      // nowhere to keep a passphrase`, mockup state `pw-nostore`.
      if (asked.nowhere_to_keep_them) {
        const nowhere = document.createElement("p");
        nowhere.className = "msub";
        nowhere.textContent = asked.nowhere_to_keep_them;
        box.appendChild(nowhere);
      }

      if (asked.the_password_could_not_be_read) {
        // **This shape asks for nothing: it reports, keeps the password
        // remembered, and offers the step again.**
        const warn = document.createElement("p");
        warn.className = "mwarn";
        warn.textContent = asked.the_password_could_not_be_read;
        box.appendChild(warn);
      }

      if (asked.password) {
        const form = document.createElement("div");
        form.className = "frm";
        const label = document.createElement("label");
        label.textContent = asked.password;
        form.appendChild(label);
        field = document.createElement("input");
        field.type = "password";
        field.className = "fld wide mono";
        form.appendChild(field);
        box.appendChild(form);
      }

      // **The choices are radios, not boxes** — one decision with one answer.
      // **How many there are is the application's** and is composed from what
      // this computer can do.
      if (asked.choices.length) {
        const part = document.createElement("div");
        part.className = "fmts";
        const marks = [];
        for (const choice of asked.choices) {
          const mark = document.createElement("label");
          mark.className = choice.chosen ? "fmt choice3 on" : "fmt choice3";
          mark.setAttribute("role", "radio");
          mark.setAttribute("aria-checked", String(Boolean(choice.chosen)));
          mark.tabIndex = 0;
          mark.dataset.choice = choice.which;
          const bullet = document.createElement("span");
          bullet.className = choice.chosen ? "rb on" : "rb";
          mark.appendChild(bullet);
          const name = document.createElement("span");
          name.className = "fmn";
          name.textContent = choice.name;
          mark.appendChild(name);
          const said = document.createElement("span");
          said.className = "fmw";
          said.textContent = choice.said;
          mark.appendChild(said);
          const end = document.createElement("span");
          end.className = "fmc";
          mark.appendChild(end);
          const pick = () => {
            for (const other of marks) {
              const it = other === mark;
              other.classList.toggle("on", it);
              other.setAttribute("aria-checked", String(it));
              other.firstChild.classList.toggle("on", it);
            }
          };
          mark.addEventListener("click", pick);
          mark.addEventListener("keydown", (press) => {
            if (press.key === " " || press.key === "Enter") {
              press.preventDefault();
              pick();
            }
          });
          marks.push(mark);
          part.appendChild(mark);
        }
        box.appendChild(part);
      }

      const row = aRow(box);
      row.appendChild(aControl(asked.cancel, () => theBoxIsClosed(), true));
      // **`Try again` hands the password to the run that is waiting for it**,
      // which takes the refused step again with it (2026-09-21). Until then it
      // started the whole setup over and carried nothing.
      row.appendChild(
        aControl(asked.try_again, () => {
          const waiting = theOfferStandsFor;
          theOfferStandsFor = null;
          const typed = field ? field.value : "";
          if (field) field.value = "";
          if (waiting !== null) theOfferIsAnswered(waiting, typed || null);
          theBoxIsClosed();
        })
      );
    }
  }
}

// ── The run panel ───────────────────────────────────────────────────────────

/// **The steps a run's card showed, as they stand once the run has ended** —
/// the runner's ruling (b) of 2026-10-03. The steps and their marks are kept
/// exactly as drawn; **what only a running setup can offer goes** — *Cancel
/// setup* and what it leaves, the render sentence, the stopped card's two ways
/// out — and the state line carries the mark, because the result is being
/// read. Nothing is composed here: every word is the application's own.
function theStepsAsTheRunLeftThem(panel) {
  return {
    ...panel,
    cancel: null,
    the_cancel_was_asked: false,
    abandon: null,
    set_up_again: null,
    about_the_render: null,
    what_cancel_leaves: null,
    the_closing_paragraph: null,
    the_state_is_underway: true,
    // **The window goes when the run ends; *Server output* stays.**
    the_servers_newest_lines: [],
    which_lines_are_from_the_error_stream: [],
    // **Nothing waits behind a run that ended, and its card offers nothing
    // more** (`Server Setup` Y9a, 2026-10-27).
    the_waiting_heading: null,
    the_waiting: [],
    install_telegram_while_it_runs: null,
  };
}

/// **A run, drawn on the card in the card's place and never in a box** —
/// `Design/Server Setup § A long step`, `§ Cut off part-way`: the drawing's
/// `sugoing` and `burun` while it goes, `bufail` and `bustep` once it stopped.
/// Nothing on the server's screen is covered while it runs, and *Files* is one
/// press away (the Creator's note 4 on 0.1.7). **Every word and mark is the
/// application's**; this decides nothing.
///
/// **While it runs, *Cancel setup* is its first control**, offered throughout,
/// and drawn under way once pressed until the run ends; *Set up again* and,
/// where owed, *Install Telegram reporting* follow it quietly from 2026-10-27,
/// each joining the line of acts the card lists under the steps. **Once it stopped, the
/// two ways out** — *Abandon setup*, quiet, before *Set up again*.
function theRunCardIsDrawn(panel, server) {
  card.textContent = "";
  const heading = document.createElement("h2");
  heading.textContent = panel.this_server;
  card.appendChild(heading);

  const one = document.createElement("div");
  one.className = "sucard";

  const state = document.createElement("div");
  state.className = "sustat";
  const dot = document.createElement("span");
  dot.className = "sdot";
  dot.dataset.dot = panel.dot;
  state.appendChild(dot);
  const words = document.createElement("span");
  words.className = "stx";
  const bold = document.createElement("b");
  bold.textContent = panel.title;
  words.appendChild(bold);
  // **The state line carries the mark where no step does** — a run taken and
  // not yet read, which on a machine never prepared is the whole placement.
  if (panel.the_state_is_underway) {
    words.classList.add("underway-reading");
    words.appendChild(document.createTextNode(" "));
    const turning = document.createElement("i");
    turning.className = "spin";
    words.appendChild(turning);
  }
  state.appendChild(words);
  const grow = document.createElement("span");
  grow.className = "grow";
  state.appendChild(grow);
  const moment = panel.when_it_started || panel.how_it_ended;
  if (moment) state.appendChild(theMoment(moment));
  one.appendChild(state);
  // **A failed install's stopped setup says the last act on its own failed
  // step**, so no line above the steps repeats it — `Design/Server Setup §
  // Installing`. The application says where; this only leaves the line out.
  // Added 2026-10-25.
  if (!panel.the_last_act_is_said_on_its_steps) theLastWordIsDrawnInto(one);

  let steps = null;
  if (panel.steps.length) {
    steps = document.createElement("ol");
    steps.className = "steps";
    for (const step of panel.steps) {
      const line = document.createElement("li");
      // **The mark is the value the application sent** and is never derived here.
      if (step.mark) line.className = step.mark;
      const name = document.createElement("span");
      name.textContent = step.name;
      line.appendChild(name);
      if (step.said) {
        line.appendChild(document.createTextNode(" "));
        const mark = document.createElement("span");
        mark.className = "stm";
        if (step.colour) mark.classList.add(step.colour);
        saidInto(mark, step.said);
        line.appendChild(mark);
      }
      // **How far the download or the unpack has got, in the step's small
      // note** — `Design/Server Setup § A long step`, `Server Setup` Y7b and
      // `Server Setup` Y7c. The figures are values and *of* is the application's word; this
      // sets them side by side and joins nothing. Added 2026-10-18, piece U.
      if (step.the_figure) {
        line.appendChild(document.createTextNode(" "));
        line.appendChild(theStepsFigure(step.the_figure));
      }
      if (step.note) {
        const note = document.createElement("span");
        note.className = "stnote";
        note.textContent = step.note;
        line.appendChild(note);
      }
      // **The step under way carries the shared underway shape, on the row** —
      // a setup step is the *on the row* shape. Added 2026-09-27, fix piece 9.
      if (step.mark === "now") line.appendChild(theMarkOnARow(null));
      // **The one failure shape, on the step itself.**
      if (step.the_failure) line.appendChild(theFailedStep(step.the_failure));
      steps.appendChild(line);
    }
  }
  // **The server's newest lines to the right of the steps, and *Server
  // output* under them** — `Design/Server Setup § A long step`. Under the
  // steps where the card is narrow, which the drawing's own rule does.
  for (const beside of theServersWordsBeside(steps, panel, server)) one.appendChild(beside);

  // **The acts waiting their turn behind this run** — `Design/Server Setup §
  // Asking for more while a setup runs`, the drawing's `su-queue`. Every word
  // is the application's, in the order asked; nothing here decides whether
  // there is a list. Added 2026-10-27, the nineteenth notes, note 2.
  if (panel.the_waiting && panel.the_waiting.length) {
    const waiting = document.createElement("div");
    waiting.className = "suwait";
    const heading = document.createElement("p");
    heading.className = "suwh";
    heading.textContent = panel.the_waiting_heading;
    waiting.appendChild(heading);
    const list = document.createElement("ul");
    for (const act of panel.the_waiting) {
      const item = document.createElement("li");
      item.textContent = act;
      list.appendChild(item);
    }
    waiting.appendChild(list);
    one.appendChild(waiting);
  }

  // **One note under the steps while it runs**, as the drawing writes it: the
  // render sentence where it is true, then what *Cancel setup* leaves.
  const about = [panel.about_the_render, panel.what_cancel_leaves].filter(Boolean);
  if (about.length) {
    const note = document.createElement("p");
    note.className = "sunote";
    about.forEach((part, at) => {
      if (at) note.appendChild(document.createTextNode(" "));
      const piece = document.createElement("span");
      piece.textContent = part;
      note.appendChild(piece);
    });
    one.appendChild(note);
  }

  if (panel.the_closing_paragraph) {
    const closing = document.createElement("p");
    closing.className = "sunote";
    const before = document.createElement("span");
    before.textContent = panel.the_closing_paragraph.before;
    closing.appendChild(before);
    closing.appendChild(document.createTextNode(" "));
    // **The control's name is written in the paragraph exactly as it reads on
    // the control**, which is Design's own requirement.
    const named = document.createElement("b");
    named.textContent = panel.the_closing_paragraph.the_controls_name;
    closing.appendChild(named);
    closing.appendChild(document.createTextNode(" "));
    const after = document.createElement("span");
    after.textContent = panel.the_closing_paragraph.after;
    closing.appendChild(after);
    one.appendChild(closing);
  }

  const row = document.createElement("div");
  row.className = "stgrow";
  if (panel.cancel) {
    const cancel = aControl(panel.cancel, () => theActIsAskedFor(STOP, server), true);
    // **Pressed, and the run has not reached its boundary yet**: the control
    // carries the mark and takes no second press.
    if (panel.the_cancel_was_asked) theMarkOnAControl(cancel, panel.cancel);
    row.appendChild(cancel);
  }
  // **The card's own act, quiet, while it runs** — pressed, it joins the
  // line and appears in the list above (`Server Setup` Y9a item 6). Added
  // 2026-10-27. **No *Set up again* while it runs** — gone, not greyed, and
  // back when the run ends (the twenty-fourth notes, note 1, 2026-10-30).
  if (panel.install_telegram_while_it_runs) {
    row.appendChild(
      aControl(panel.install_telegram_while_it_runs, () => theActIsAskedFor(INSTALL_TELEGRAM, server), true)
    );
  }
  // **S6's second way out, and it is quiet** — most people here want the setup
  // finished rather than dropped.
  if (panel.abandon) {
    row.appendChild(aControl(panel.abandon, () => theActIsAskedFor(ABANDON, server), true));
  }
  if (panel.set_up_again) {
    row.appendChild(
      theReadyActionsOwn(
        aControl(panel.set_up_again, () => theActIsAskedFor(THE_READY_ACTION, server))
      )
    );
  }
  if (row.firstChild) one.appendChild(row);
  card.appendChild(one);
}

// ── What the server printed — `Design/Server Setup § A long step` ───────────
//
// `Server Setup` Y7a and `Desktop app and delivery` M13b's second narrowing.
// Added 2026-10-18, the Creator's thirteenth notes, note 7. **The lines and
// the words arrive composed**: which lines, from which stream, and whether a
// window is drawn at all are the application's, and this file draws them. The
// shapes are the queue's own — `.log`, `.qlines`, `.log.nolines` — inside the
// drawing's `.suwork`.

/// **The list, with the server's words beside it** — the window of the
/// newest lines where the application handed any, and ***Server output***
/// under it, or alone where there are none. **Just the list where no control
/// was handed**, and nothing where there is neither. What is handed back is
/// the card's children, in order.
///
/// **Once the run has ended there is no window, and *Server output* sits on
/// its own line under the steps, at the step names' edge** — the drawing's
/// `.suout`, on a stopped card and a finished one alike, never alone at the
/// top right where the window was (`Design/Server Setup § A long step`, the
/// eighteenth notes, note 4; drawn as `su-stepten`). A run still going is the
/// one that carries *Cancel setup*. Added 2026-10-25, piece 2c.
function theServersWordsBeside(list, drawn, server) {
  if (!drawn || !drawn.server_output) return list ? [list] : [];
  const open = document.createElement("button");
  open.type = "button";
  open.textContent = drawn.server_output;
  open.addEventListener("click", () => theServersOutputIsOpened(server));
  if (!drawn.cancel) {
    open.className = "mini";
    const under = document.createElement("div");
    under.className = "suout";
    under.appendChild(open);
    return list ? [list, under] : [under];
  }
  const work = document.createElement("div");
  work.className = "suwork";
  if (list) work.appendChild(list);
  const log = document.createElement("div");
  log.className = "log";
  const lines = drawn.the_servers_newest_lines || [];
  if (lines.length) {
    const held = document.createElement("span");
    held.classList.add("qlines", "selectable");
    held.classList.add("mono");
    theWindowsLinesAreDrawnInto(held, lines, drawn.which_lines_are_from_the_error_stream || []);
    log.appendChild(held);
    theWindow = held;
  } else {
    // **Before anything is printed there is no window**, and the control sits
    // alone, never inside an empty grey box.
    log.classList.add("nolines");
  }
  log.appendChild(open);
  work.appendChild(log);
  return [work];
}

/// **Each line one isolated left-to-right run inside the window's cut**, as
/// the queue draws its entry's lines (`web/the-value-drawn.js`): the cut falls
/// at a line's start, and a closing `...` stays at its end. **A line from the
/// error stream carries the drawing's `err`**, the brighter ink, so the two
/// streams are never read as one (`Desktop app and delivery` M13b; the
/// eighteenth notes, piece 2c). Which line is which is the application's.
function theWindowsLinesAreDrawnInto(held, lines, errors) {
  held.textContent = "";
  lines.forEach((value, at) => {
    const line = document.createElement("span");
    if (errors && errors[at]) line.className = "err";
    theValueIsDrawnInto(line, value);
    held.appendChild(line);
  });
}

/// **What the server printed moved**: the run's panel is asked for again and
/// its lines drawn, and an open *Server output* takes the new words. One
/// reading at a time, and one more after it where they moved meanwhile.
function theServersWordsMoved(server) {
  if (theWordsAreBeingAskedFor) {
    theWordsMovedAgain = true;
    return;
  }
  theWordsAreBeingAskedFor = true;
  const asked = theAnswersSoFar;
  const done = () => {
    theWordsAreBeingAskedFor = false;
    if (theWordsMovedAgain) {
      theWordsMovedAgain = false;
      theServersWordsMoved(server);
    }
  };
  shell.core
    .invoke("the_setup_under_way", { server })
    .then((panel) => {
      // **Only onto a run already drawn**, and never over a later answer: a
      // run that ended is the answer's to take down.
      if (panel && !panel.the_run_has_ended && current === server && host && asked === theAnswersSoFar && theRun && theRun.server === server) {
        theRun = { server, panel };
        theCardIsDrawnWhereItChanged(said);
      }
      return theServersOutputIsAskedAgain(server);
    })
    .then(done, done);
}

/// ***Server output*, opened** — the whole of what the run printed, its two
/// streams apart, with Copy only for the selected stream's printed words
/// (`Design/Server Setup § A long step`).
/// The panel is the one every screen opens a machine's words in.
function theServersOutputIsOpened(server) {
  shell.core.invoke("the_servers_output", { server }).then((output) => {
    if (!output || current !== server || !boxes) return;
    theBoxIsClosed();
    theServersOutputIsDrawn(server, output);
  });
}

function theServersOutputIsDrawn(server, output, selected = 0) {
  const panel = theWordsPanel({
    title: output.title,
    subtitle: output.subtitle,
    tabs: output.tabs.map((tab) => ({
      name: tab.name,
      says: tab.says,
      said: tab.said,
      saidNext: tab.said_next,
      record: tab.record,
      notShown: tab.not_shown,
      // **What Copy takes where it differs from what is shown** — every byte
      // as printed (M9), the shown record having each rewritten line in its
      // newest form (M13b). The eighteenth notes, piece 2c.
      asPrinted: tab.as_printed,
    })),
    note: output.note,
    close: output.close,
    copy: output.copy,
  });
  const foot = panel.querySelector(".lfoot");
  const close = foot && foot.querySelector("button");
  if (close) {
    close.addEventListener("click", () => {
      if (theServersOutputOpen && theServersOutputOpen.panel === panel) theServersOutputOpen = null;
      panel.remove();
    });
  }
  panel.hidden = false;
  boxes.appendChild(panel);
  // The shared panel decides Copy from the visible printed record. Restore
  // the reader's stream when its first words replace an explanatory sentence.
  const tabs = panel.querySelectorAll(".ltab");
  if (selected > 0 && tabs[selected]) tabs[selected].click();
  theServersOutputOpen = { server, panel, records: output.tabs.map((tab) => tab.record || null), tabs: output.tabs };
}

/// **An open *Server output* takes what was printed since**: each tab's new
/// words are added after the ones drawn, as the queue's live panel adds a
/// render's, and **nothing drawn is edited** (M9). Where a tab's words are not
/// the ones drawn with more after them — the first words of a stream that
/// said nothing yet — the panel is drawn again whole, keeping the chosen tab.
/// A stream's ending also replaces its earlier explanatory sentence.
function theServersOutputIsAskedAgain(server) {
  const open = theServersOutputOpen;
  if (!open || open.server !== server) return Promise.resolve();
  return shell.core.invoke("the_servers_output", { server }).then((output) => {
    if (!output || theServersOutputOpen !== open) return;
    if (!open.panel.parentNode) {
      theServersOutputOpen = null;
      return;
    }
    const bodies = Array.from(open.panel.querySelectorAll(".words-body"));
    const fresh = output.tabs.map((tab) => tab.record || null);
    const added = fresh.every((now, at) => {
      const was = open.records[at];
      const before = open.tabs[at];
      const after = output.tabs[at];
      if (!before || before.says !== after.says || before.said !== after.said || before.said_next !== after.said_next || before.not_shown !== after.not_shown) return false;
      return now === was || (was !== null && now !== null && now.startsWith(was) && bodies[at]);
    });
    // **A line rewritten in place changes what is shown without adding to
    // it** (M13b, the eighteenth notes, piece 2c): where every tab still says
    // what it said and holds words, the words are put in place whole rather
    // than the panel drawn again, so the chosen tab and the place read stay.
    const rewritten = !added && fresh.every((now, at) => {
      const before = open.tabs[at];
      const after = output.tabs[at];
      if (!before || before.says !== after.says || before.said !== after.said || before.said_next !== after.said_next || before.not_shown !== after.not_shown) return false;
      return now === open.records[at] || (now !== null && open.records[at] !== null && bodies[at] && bodies[at].querySelector("pre.lbody"));
    });
    if (!added && !rewritten) {
      const selected = bodies.findIndex((body) => !body.hidden);
      open.panel.remove();
      theServersOutputIsDrawn(server, output, selected);
      return;
    }
    fresh.forEach((now, at) => {
      const was = open.records[at];
      const printed = bodies[at] && bodies[at].querySelector("pre.lbody");
      if (!printed) return;
      const asPrinted = output.tabs[at].as_printed;
      if (typeof asPrinted === "string") printed.dataset.asPrinted = asPrinted;
      else delete printed.dataset.asPrinted;
      if (now === was) return;
      if (now.startsWith(was)) printed.appendChild(document.createTextNode(now.slice(was.length)));
      else printed.textContent = now;
      theNewestWordsStayInView(bodies[at]);
    });
    open.records = fresh;
    open.tabs = output.tabs;
  });
}

/// **The last act's own ending, as one line on the card under its state line**
/// — in the whole-app failure shape where the machine printed words, with them
/// behind *What the server said*; a plain line otherwise. Nothing where there
/// is none.
function theLastWordIsDrawnInto(one) {
  const word = theLastWords.get(current) || null;
  if (!word) return;
  if (!word.the_machines_own_words) {
    const line = document.createElement("p");
    line.className = "sunote";
    line.textContent = word.the_act_is_not_built_yet;
    one.appendChild(line);
    return;
  }
  one.appendChild(
    theFailureShape({
      sentence: word.the_act_is_not_built_yet,
      kind: "ending",
      room: EXPANDED,
      controls: [{ is: "raw-text", label: word.what_the_server_said, act: "the-words" }],
      onAct: () => {
        const panel = theWordsPanel({
          title: word.what_the_server_said,
          tabs: [{ name: word.what_the_server_said, record: word.the_machines_own_words }],
          // **The panel's own *Close*, as the answer carries it** — fault 6 of
          // the second pass: nothing handed it a word, and it was drawn as an
          // empty pill.
          close: word.close,
        });
        panel.hidden = false;
        boxes.appendChild(panel);
      },
    })
  );
}

/// **The Blender table, covered while it is read again after a run ended** —
/// `Design/Whole app § Anything that isn't instant`, *over the pane*, on that
/// section alone; the card stays usable. It comes off when the next reading
/// lands.
function theTableIsCoveredWhileItIsReadAgain() {
  if (!table || theTableCover || !table.firstChild) return;
  theTableCover = theMarkOverAPane();
  table.appendChild(theTableCover);
}

function theTableCoverIsTakenAway() {
  if (theTableCover) theTableCover.remove();
  theTableCover = null;
}

/// **The whole-app failure shape, on the step, with the machine's own words
/// behind the control that names them** — drawn as the shape's own line under
/// a step (`onAStep`): the dot, the sentence in bold and *What the server said*
/// as a link at its end, on the sunk ground (`Design/Server Setup § The step
/// that failed`, the drawing's `su-step` and `su-stepten`). *(Until 2026-10-25
/// it was the general block with the control as a pill on a line of its own —
/// the eighteenth notes, note 4.)*
///
/// **No cross, and not because the matter is over**: a step that stopped on a
/// run that changes the machine leaves a condition standing, and only a run can
/// end it. A cross clears a mark by hand and this is not one.
function theFailedStep(failed) {
  // **The control only where there are the server's words to open behind
  // it** — a step that stopped on this computer has none, and its sentence is
  // Smoozy's own (the Creator's twelfth notes, note 2).
  const theWordsAreThere = Boolean(failed.the_output || failed.the_complaint);
  return theFailureShape({
    sentence: failed.said.lead,
    kind: "ending",
    room: EXPANDED,
    onAStep: true,
    controls: theWordsAreThere
      ? [{ is: "raw-text", label: failed.what_the_server_said, act: "the-words" }]
      : [],
    onAct: () => theMachinesWordsAreShown(failed),
  });
}

/// **The machine's own words, whole and unedited, the two streams apart**, each
/// tab named for its own stream (must-do 33).
function theMachinesWordsAreShown(failed) {
  const tabs = [];
  if (failed.the_output) {
    tabs.push({ name: failed.the_output_tab, record: failed.the_output });
  }
  if (failed.the_complaint) {
    tabs.push({
      name: failed.the_complaint_tab,
      record: failed.the_complaint,
    });
  }
  if (!tabs.length) return;
  const panel = theWordsPanel({
    title: failed.what_the_server_said,
    tabs,
    close: failed.close,
  });
  panel.hidden = false;
  boxes.appendChild(panel);
}

// ── The two warnings before a step that could disturb work ──────────────────

/// **The ready action's warning, asked for and drawn** — and nothing drawn
/// where none stands, where the user has gone to another server, or where a
/// later answer has arrived since it was asked. Added 2026-09-27.
function theWorkWarningIsAskedFor(server) {
  const asked = theAnswersSoFar;
  shell.core.invoke("the_work_warning", { server }).then((warning) => {
    if (!warning || current !== server || !host || host.hidden) return;
    if (asked !== theAnswersSoFar) return;
    theWorkWarningIsDrawn(warning, server, THE_READY_ACTION_AFTER_THE_WARNING);
  });
}

/// **W3's table is closed and the silence is as binding as the warning.** This
/// draws a warning only where the application composed one.
export function theWorkWarningIsDrawn(warning, server, act) {
  const box = aBox();
  const title = document.createElement("h3");
  title.textContent = warning.title;
  box.appendChild(title);

  const said = document.createElement("p");
  said.className = "msub";
  said.textContent = warning.said;
  box.appendChild(said);

  const list = document.createElement("ul");
  list.className = "bulive";
  for (const job of warning.the_live_jobs) {
    const one = document.createElement("li");
    const named = document.createElement("b");
    theValueIsDrawnInto(named, job.job);
    one.appendChild(named);
    one.appendChild(document.createTextNode(" "));
    const mark = document.createElement("span");
    mark.className = "stm";
    mark.textContent = job.said;
    one.appendChild(mark);
    list.appendChild(one);
  }
  box.appendChild(list);

  const row = aRow(box);
  row.appendChild(aControl(warning.not_now, () => theBoxIsClosed(), true));
  row.appendChild(aControl(warning.go_ahead, () => theActIsAskedFor(act, server)));
  return box;
}

// ── The two boxes on the pointed-at route ───────────────────────────────────

/// **The send is an ordinary upload** and shows in Transfers under the Upload
/// tag like anything else going up.
export function theBuildGoingUpIsDrawn(going, server) {
  const box = aBox();
  const title = document.createElement("h3");
  title.textContent = going.title;
  box.appendChild(title);

  const named = document.createElement("p");
  named.className = "msub mono";
  theValueIsDrawnInto(named, going.file);
  named.querySelector("bdi")?.classList.add("selectable");
  named.appendChild(document.createTextNode(" · "));
  const size = document.createElement("span");
  size.textContent = going.size;
  named.appendChild(size);
  box.appendChild(named);

  // **A native progress element rather than the drawing's own two divs**, and
  // in the shared half's own class — `web/the-underway-shape.css` already
  // dresses one, and a second dressing of the same bar is a shape rebuilt.
  const bar = document.createElement("progress");
  bar.className = "underway-bar";
  bar.max = 100;
  bar.value = Number(going.how_far_along);
  box.appendChild(bar);

  const sent = document.createElement("p");
  sent.className = "msub";
  saidInto(sent, going.sent);
  box.appendChild(sent);

  const where = document.createElement("p");
  where.className = "msub";
  const before = document.createElement("span");
  before.textContent = going.it_shows_in;
  where.appendChild(before);
  where.appendChild(document.createTextNode(" "));
  const link = document.createElement("button");
  link.type = "button";
  link.className = "lnkb";
  link.textContent = going.transfers;
  // **The link goes to Transfers**, where the upload is watched (must-do 30;
  // piece 7 had recorded it as owed until piece 12 existed). `server` is the
  // server the build is going up to, and Transfers lists every server's.
  link.addEventListener("click", () => theCurrentPlaceIs(TRANSFERS));
  where.appendChild(link);
  where.appendChild(document.createTextNode(" "));
  const after = document.createElement("span");
  after.textContent = going.with_your_other_uploads;
  where.appendChild(after);
  box.appendChild(where);

  const row = aRow(box);
  row.appendChild(aControl(going.stop, () => theActIsAskedFor(STOP, server), true));
  return box;
}

/// **The below-4.5 warning, and offering the way out is not refusing** — the
/// build is still in staging.
export function theBelowTheFloorWarningIsDrawn(warning, server) {
  const box = aBox();
  const title = document.createElement("h3");
  saidInto(title, warning.title);
  box.appendChild(title);

  const supported = document.createElement("p");
  supported.className = "mwarn";
  const first = document.createElement("b");
  saidInto(first, warning.supported);
  supported.appendChild(first);
  supported.appendChild(document.createTextNode(" "));
  const second = document.createElement("span");
  saidInto(second, warning.nothing_is_written_for);
  supported.appendChild(second);
  box.appendChild(supported);

  const appears = document.createElement("p");
  appears.className = "msub";
  appears.textContent = warning.it_will_appear;
  box.appendChild(appears);

  // **Both answers go back to the run waiting on them** (`Server Setup` B7a);
  // closing the box any other way leaves the question standing, and the card
  // asks it again.
  const answered = (act) => () => {
    theQuestionIsUp = null;
    theBoxIsClosed();
    theActIsAskedFor(act, server);
  };
  const row = aRow(box);
  row.appendChild(aControl(warning.dont_install_it, answered(DONT_INSTALL_IT), true));
  row.appendChild(aControl(warning.install_anyway, answered(INSTALL_ANYWAY)));
  return box;
}

// ── The acts ────────────────────────────────────────────────────────────────

/// **Every act on this tab is answered, and three of them do something.**
///
/// **The ready action and the stop are built** — step 10c piece 9 — and
/// **installing a Blender picked from the list** was built on 2026-09-21, with
/// the picked version carried; the rest answer the one sentence this
/// application says for an act nobody has built. **The answer arrives as an event either way**, which is P6's one
/// route, so this side does not know which is which and must not try to.
function theActIsAskedFor(act, server, version, waiting, path) {
  // **A connect is the next act on that server**, so what the last one that
  // did not happen said goes — here and above the server list.
  if (act === CONNECT_AND_SET_UP) theConnectionFailureIsTakenAway(server);
  // **A new act takes the last one's line off the card** — the one way it
  // leaves besides another act's ending replacing it (ruling (a)).
  if (theLastWords.has(server)) {
    theLastWords.delete(server);
    theCardIsDrawnWhereItChanged(said);
  }
  shell.core.invoke("a_set_up_act_that_needs_a_connection", {
    act,
    server,
    version: version || null,
    // **The Blenders picked after the first**, which the application begins
    // one after another (the Creator's third notes, note 2).
    waiting: waiting && waiting.length ? waiting : null,
    // **The build a removal names**, by its executable's path (the 38's
    // piece 14c). Nothing on every other act.
    path: path || null,
  });
}

// ── The one place a value becomes a node ────────────────────────────────────

// **What stands between a figure and the words after it, where the application
// marks them bound** — U+00A0, so *24 GB* or *2 jobs* never parts across two
// lines. `Design/Whole app § Typography`; the character's one home is
// `src/typography.rs` (`BETWEEN_A_FIGURE_AND_ITS_UNIT`), and this page writes
// it only where the mark says, never by reading which word is a unit.
const BETWEEN_A_FIGURE_AND_ITS_UNIT = "\u00A0";

/// **Draw a piece of prose with the values that belong beside it.**
///
/// The application hands over `{ lead, values, rest }` — the words before the
/// first value, the values, and the words after the last. **This sets them side
/// by side and joins nothing**, which is the page half of X7.
function saidInto(holder, part) {
  if (!part) return holder;
  let first = true;
  const next = (node) => {
    if (!first) holder.appendChild(document.createTextNode(" "));
    first = false;
    holder.appendChild(node);
  };
  if (part.lead) {
    const lead = document.createElement("span");
    lead.textContent = part.lead;
    next(lead);
  }
  for (const value of part.values || []) {
    const one = document.createElement("b");
    theValueIsDrawnInto(one, value);
    next(one);
  }
  if (part.rest) {
    const rest = document.createElement("span");
    rest.textContent = part.rest;
    if (part.the_rest_joins) {
      first = true;
      holder.appendChild(rest);
    } else if (part.the_rest_binds) {
      holder.appendChild(document.createTextNode(BETWEEN_A_FIGURE_AND_ITS_UNIT));
      holder.appendChild(rest);
    } else {
      next(rest);
    }
  }
  return holder;
}

/// **A step's figure** — *214 of 352 MB*, *214 MB*, *41%* — in the step's
/// small note, plain: the values are drawn by the value route and not set in
/// bold, because the drawing writes the figure in the note's own weight.
function theStepsFigure(figure) {
  const note = document.createElement("span");
  note.className = "stm";
  const parts = [figure.so_far, figure.of, figure.whole];
  let first = true;
  parts.forEach((part, at) => {
    if (part == null || part === "") return;
    if (!first) note.appendChild(document.createTextNode(" "));
    first = false;
    const piece = document.createElement("span");
    if (at === 1) piece.textContent = part;
    else theValueIsDrawnInto(piece, part);
    note.appendChild(piece);
  });
  return note;
}

/// **A moment, on the clock it was taken on** — §16.1 M3a.
///
/// **The moment goes with the name of the clock it is on**, so a moment read on
/// a rented machine never reads as one read here. **Nothing is subtracted**: an
/// elapsed time would cross two clocks, which `State, identity and recovery` D5
/// forbids.
function theMoment(moment, className) {
  const said = document.createElement("span");
  said.className = className || "sunote";
  for (const part of [
    moment.lead,
    theMomentsText(moment.reading.taken_at),
    moment.on_the_clock,
    moment.reading.on_the_clock,
  ]) {
    if (!part) continue;
    const one = document.createElement("span");
    one.textContent = part;
    said.appendChild(one);
    said.appendChild(document.createTextNode(" "));
  }
  return said;
}

/// **The moment, with its date and the zone it is drawn in.**
///
/// **A bare time of day is a plain falsehood** and it is the one format M3a
/// exists to stop — the same reasoning `web/the-machines-own-words.js` carries,
/// and **the same six fields, written out one at a time.** The two shorthand
/// styles cannot be asked for beside a zone name at all: the engine refuses the
/// pair outright, and a moment written that way throws where it is drawn rather
/// than coming out wrong.
function theMomentsText(seconds) {
  if (!seconds) return "";
  const when = new Date(Number(seconds) * 1000);
  return when.toLocaleString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    timeZoneName: "short",
  });
}

