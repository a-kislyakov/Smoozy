// **Where a click on a notification goes** — `Desktop app and delivery` T29.
// The application half is `src/notifications/activation.rs`; read its head
// first.
//
// **The application decided where, from the record at the moment of the
// click, and this file only goes there**: the job's History entry, its Queue
// entry, or the server. **It composes nothing and holds nothing.** A subject
// the record no longer holds posts nothing, so the window comes forward and
// stays where it was.
//
// **It is a file of its own** because it reaches three screens, for the reason
// `a-new-job.js` gives.

import { whenTold } from "./event-route.js";
import { theCurrentPlaceIs, A_SERVER } from "./the-rail.js";
import { theHistoryEntryIsOpened } from "./the-history.js";
import { theQueueEntryIsOpened } from "./the-queue.js";

// Declared beside the service that posts it — `THE_USER_CLICKED_A_NOTIFICATION`
// in `src/notifications/activation.rs`. The two sides have to agree.
const THE_USER_CLICKED_A_NOTIFICATION = "notifications:a-going";

whenTold(THE_USER_CLICKED_A_NOTIFICATION, (delivery) => {
  const said = delivery.said || {};
  if (said.goesTo === "the-history" && said.job) {
    theHistoryEntryIsOpened(said.job);
  } else if (said.goesTo === "the-queue" && said.job) {
    theQueueEntryIsOpened(said.job);
  } else if (said.goesTo === "the-server" && said.server) {
    theCurrentPlaceIs(A_SERVER, said.server);
  }
});
