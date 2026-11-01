# Product brief

*This document is the input the Specs stage builds on: who Smoozy is for and what it does. Everything in Specs traces back to here.*

## What Smoozy is

Smoozy is a desktop app that turns a rented GPU server into a personal Blender render farm. It handles the setup, file uploads, rendering, and downloads over SSH, so the user never has to touch a terminal.

## Who it's for

A Blender user who wants to render faster than their own hardware allows. Commercial render farms solve that, but they cost far more than renting a plain GPU server. Smoozy lets the user rent a cheap bare server and get the same result from it. That price gap is what Smoozy is banking on.

## What Smoozy does

The scope is clearest as a walk through it, start to finish. Each step is in scope; the detail of each belongs to the Specs that follow.

1. **Getting the app.** The user downloads a binary from Smoozy's website. No email, no account, no registration — click download, get the program.
2. **Getting in.** On first launch, Smoozy asks for either a licence key or a 7-day free trial. The trial holds back no features; it is only a time limit. Either way, the user is in.
3. **Making a key.** The user generates an SSH key pair inside Smoozy and can copy the public half to the clipboard. Smoozy keeps keys where the operating system normally keeps them, so it also sees keys the user already has, and it can be pointed at keys kept elsewhere.
4. **Renting a server — on their own.** The user leaves Smoozy, rents a machine wherever they choose (or uses one they own), and gives that machine their public key. This part is outside Smoozy: it does not pick the server or know where it came from.
5. **Adding the server.** The user pastes the server's details into Smoozy's Add Server dialog — host, port, username, and which key to use — plus two convenience options: install or update GPU drivers where possible, and install the latest stable Blender. Both can also be done afterwards, with more choice.
6. **Browsing it.** Smoozy shows the server's files in a built-in file browser.
7. **Making it render-ready.** The user installs what Blender needs on the server: Blender itself — possibly several versions, including unstable daily builds or self-compiled ones — and, where the machine allows it, GPU drivers. Some servers are locked containers with drivers already installed; on those, Smoozy leaves the drivers alone.
8. **Uploading the work.** The user uploads a `.blend` file, and/or folders and assets. They may upload only the file itself, without its textures and other external pieces.
9. **Render settings.** Clicking a file opens a Render Settings panel. On top of the usual render controls, the user can set: which scene (one, or all), the frame range, samples, resolution percentage, view layers, the render engine (Cycles or EEVEE), and — if the server has more than one — which Blender version to render with. Everything else (output format, denoising, where frames are written, and so on) comes from the file's own settings.
10. **Check Dependencies.** If the user uploaded only the `.blend`, Smoozy can inspect that file on the server, find what it is missing, and offer to supply it — pulling the assets from the user's local machine. Heavy assets (textures, volumes, caches) go up once, so later changes re-render fast without re-sending everything.
11. **Auto-download (opt-in).** Before rendering, the user can ask for finished outputs to come back to their machine automatically.
12. **Render, queue, history.** Clicking Render sends the job to a single queue shared across all the user's servers. Each server renders one job at a time — never two at once — while different servers can each work their own job. Finished jobs move into history.

One thing holds throughout: Smoozy's plumbing knows only servers and SSH keys — never providers. It may still speak to the user in provider terms where that helps (for example, suggesting one key per provider), but no provider exists anywhere in the app itself.

## What Smoozy does not do

- It never rents, starts, stops, or bills a server. The provider market shifts constantly and there is no keeping up with it; that burden stays with the user.
- It never splits one render across servers. One job runs on one server.
- It is not a multi-user or hosted service. One user, their own machines.

## Telegram bot

Smoozy includes a Telegram bot so the user can keep in touch with renders from their phone — for example, previews of finished frames, or starting and stopping renders remotely. This is the least-defined part of Smoozy, and its exact scope is settled during the Specs stage.

Settled during the Specs stage, as this said it would be: the [Telegram spec](Telegram.md), written 2026-07-29, owns the bot and states its scope in full. This brief is a frozen snapshot of what the project started from, and does not restate it.

*Added 2026-10-26: from the Creator’s decision of 2026-10-04, Smoozy is free and open source. This brief remains the original snapshot; `Preparation/Specs/Whole app.md` owns the change. Step 2’s trial and licence gate no longer apply. The price gap between a rented machine and a commercial farm still matters.*
