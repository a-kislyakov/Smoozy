// **The page's side of the one ordered queue** — Desktop app and delivery P6.
//
// The queue itself, and every rule about it, is `src/events.rs`. This file is
// the page's half: **one listener on one event name, and one place a piece
// says what it wants to be told about.**
//
// **P6, on this side.** There is one listener and there will only ever be one.
// A piece does not call `listen` of its own — it calls `whenTold` here, and
// everything arrives through the one handler below in the one order the queue
// posted it in. A second listener would be a second stream, and two streams
// reaching one page are two orders however carefully each is kept.
//
// **P9 and P10: nothing here polls and nothing here has a timer.** The route is
// woken by the application telling it something and at no other moment. There
// is no interval, no poll and no retry.
//
// **P8, on this side.** Events are volatile and run-held. Nothing here is
// written down, nothing is kept between runs, and a page that loads again is
// sent what has been posted since rather than the run over again — so a piece
// that needs a fact after a reload asks the application for it rather than
// remembering the event.
//
// **The order is checked rather than trusted.** Every delivery carries its
// place in the queue's one order. **Gaps are correct** — P8 discards an event
// whose subject is gone, and a discard is a gap — so what is checked is that
// the numbers go forward and never back. A delivery that arrived out of order
// would mean P6 is not being kept, and this route says so rather than sorting
// it out: sorting it out would hide the fault and produce the very thing P6
// exists to prevent, which is a user watching two panels see one action's
// outcomes in an order nothing promised.

const shell = window.__TAURI__;

// The one name every delivery travels on. It is `THE_ONE_EVENT_NAME` in
// `src/events.rs`, and the two have to agree.
const THE_ONE_EVENT_NAME = "smoozy:event";

// kind → the handlers that asked to be told about it.
const told = new Map();

// The last place in the order this page saw. Null before the first delivery,
// because a page that loads again picks the order up wherever it now is.
let lastPlace = null;

/// **Say what you want to be told about.**
///
/// `kind` is the string the posting service declares as a constant beside
/// itself — `spine:the-run-began` is the first of them. `handler` is given the
/// delivery: its `kind`, its `subject`, and `said`, which is an object of the
/// things the event says.
///
/// Every string in `said` has already been through one of the two routes on the
/// application side — the typography point for prose, the verbatim route for
/// everything else (A6, X9) — so **a handler draws what it is given and
/// substitutes nothing**. X3: neither typography rule is held by anybody
/// remembering it, and this side is where that stops being a hope.
export function whenTold(kind, handler) {
  const already = told.get(kind);
  if (already) {
    already.push(handler);
    return;
  }
  told.set(kind, [handler]);
}

function receive(delivery) {
  if (!delivery || typeof delivery.sequence !== "number") return;

  if (lastPlace !== null && delivery.sequence <= lastPlace) {
    // P6 is not being kept. Say so and draw nothing from it; nothing here
    // repairs an order, because a repaired order is one nobody can see is
    // broken.
    document.documentElement.dataset.eventOrder = "broken";
    return;
  }
  lastPlace = delivery.sequence;

  for (const handler of told.get(delivery.kind) || []) {
    try {
      handler(delivery);
    } catch (trouble) {
      // One handler's trouble is not another's, and it is not the route's.
      // The route goes on delivering, which is what keeps the one order.
      console.error(trouble);
    }
  }
}

/// **Open the route: listen first, then tell the application.**
///
/// The order of those two is the whole of this function. The spine runs at
/// application start, before the window exists, so the first events of a run
/// are already waiting on the queue when this page loads. **Telling the
/// application before the listener is attached would lose exactly those**, and
/// they are the ones that say what happened at start.
export async function theRouteIsOpen() {
  await shell.event.listen(THE_ONE_EVENT_NAME, (arrived) => receive(arrived.payload));
  await shell.core.invoke("the_page_is_ready");
}
