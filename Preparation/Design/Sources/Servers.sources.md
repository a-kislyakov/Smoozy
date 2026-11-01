# Servers — sources

*Where the outside facts in [Servers](../Servers.md) came from. Smoozy's own decisions are not here — the port field refusing in place, and the wording of what it says, are Design's and are recorded in that document.*

---

## What range a port number may take

**The claim.** A port number is a 16-bit value, so the whole space is **0 to 65535**; **0 is reserved** and not available for regular assignment. So the lowest number a server can actually be listening on is **1**, and the Add Server form takes **1 to 65535** and refuses anything outside it.

**Label:** **documented** for the space and for 0 being reserved. **inferred** for the step from those two to 1 as the floor of what the form accepts — the source says 0 is reserved for special purposes, not that a machine cannot be reached on it, and no source was found that says so outright.

**Where it came from.** RFC 6335, *Internet Assigned Numbers Authority (IANA) Procedures for the Management of the Service Name and Transport Protocol Port Number Registry*, section 6, *Port Number Ranges* — read at `https://www.rfc-editor.org/rfc/rfc6335.txt` on 2026-08-25. It states that TCP and UDP "use 16-bit namespaces for their port number registries", divides them into System Ports 0–1023, User Ports 1024–49151 and Dynamic Ports 49152–65535, and says of reserved numbers that they "include values at the edges of each range, e.g., 0, 1023, 1024, etc."

**What it does not cover.** Whether any particular provider, operating system or SSH server will accept a given number — a machine may well refuse a port for reasons of its own, and Smoozy learns that only by trying to connect. It says nothing about which ports need administrator rights to listen on, and nothing about what a rented server is likely to be using. **Smoozy's field is checking a number is a port number at all, not predicting that the connection will work.**

**What breaks if it is wrong.** One line beside one field. The form would refuse a number it should take, or take one it should refuse, and the user would find out at the first connection either way.
