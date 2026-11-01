// **Where *Submit again*, *Resume*, a mark's way and a re-submission's pick
// lead** — the 38's piece 2. The application half is
// `src/a_new_job_from_an_old_one.rs`; read its head first.
//
// **The application decided where, and this file only goes there**: the Render
// Settings page over the entry's server on its file, that server's Files with
// the bar across the top, the server list for the user to open one (hole 3 of
// the map: Smoozy picks no machine), History on the earlier entry, or — for
// *Files on the server* — that server's Files at the job's folder. **It
// composes nothing and holds nothing.**
//
// **It is a file of its own rather than a part of History's** because it
// reaches four screens, and each of those screens' page harnesses would
// otherwise have to stand in for the other three.

import { whenTold } from "./event-route.js";
import { theCurrentPlaceIs, A_SERVER, ALL_SERVERS } from "./the-rail.js";
import { theServerWillOpenOn, thePanelOverItIsCollapsed, FILES } from "./a-server.js";
import { theSettingsAreOpenedOn } from "./render-settings.js";
import { theBrowserIsAskedAgain, theBrowserIsOpenedAt, theSavedWithVersionReadFor } from "./files.js";
import { theHistoryEntryIsOpened } from "./the-history.js";

// Declared beside the service that posts it — `A_NEW_JOB_OPENS` in
// `src/a_new_job_from_an_old_one.rs`. The two sides have to agree and nothing
// holds them together, so copy carefully.
const A_NEW_JOB_OPENS = "history:a-new-job-opens";

whenTold(A_NEW_JOB_OPENS, (delivery) => {
  const said = delivery.said || {};
  if (said.goesTo === "the-page" && said.server) {
    theCurrentPlaceIs(A_SERVER, said.server);
    // **The file is handed as the bytes it is keyed by** — the record keeps it
    // as text, and every file goes back to the application as its bytes.
    const bytes = Array.from(new TextEncoder().encode(said.file || ""));
    // **The saved-with version is the one Files read for that file, where
    // Files holds one** — step 10c's owed piece 2, the same gap as Render
    // Settings' strip: until then none was handed here either. Blank where
    // Files holds no read, and the application's own fallback stands, as for
    // the strip.
    theSettingsAreOpenedOn(said.server, bytes, theSavedWithVersionReadFor(said.server, bytes)).catch(() => {});
  } else if (said.goesTo === "the-browser" && said.server) {
    theServerWillOpenOn(said.server, FILES);
    theCurrentPlaceIs(A_SERVER, said.server);
    theBrowserIsAskedAgain();
  } else if (said.goesTo === "the-folder" && said.server) {
    // **Files on the server, and the Writing to link** — `Queue & History`
    // N18: Files on that server at the job's folder, handed over the way Set
    // up's folder link hands one (`theBrowserIsOpenedAt`), the folder's text
    // as its bytes. Step 10c's *Files on the server* piece.
    theBrowserIsOpenedAt(said.server, Array.from(new TextEncoder().encode(said.folder || "")));
    // **A panel open over that server collapses to its strip**, so the folder
    // is what shows — `Design/Whole app § Panels over a server`, added
    // 2026-10-07. The Creator's fourth notes, piece O.
    thePanelOverItIsCollapsed(said.server);
    theServerWillOpenOn(said.server, FILES);
    theCurrentPlaceIs(A_SERVER, said.server);
  } else if (said.goesTo === "the-servers") {
    theCurrentPlaceIs(ALL_SERVERS);
  } else if (said.goesTo === "the-history" && said.job) {
    theHistoryEntryIsOpened(said.job);
  }
});
