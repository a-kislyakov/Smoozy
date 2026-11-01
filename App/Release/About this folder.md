# Release

**This is not a module.** It is the machinery one release runs: `smoozy-release`, a program that composes and signs a release over artifacts that already exist, and checks a published release from the outside.

Nothing in Smoozy depends on it, and nothing ever should. It is a workspace member only so that it is built and its tests are run beside everything else.

What it does, what it does not do, how a version is named, and the exact commands a release runs are at `Audits/Build/Step 8a/Decisions record.md`.

The rules it keeps are `Preparation/Architecture/Server protocol and transfers.md` §5 — B8, B8a, B9 and B10 — and the code that keeps them is `App/Server protocol and transfers/src/release/`, which this program calls and never re-implements.

## Signing and notarizing the Mac download

`smoozy-release sign-the-mac-bundle --bundle <Smoozy.app> --into <Smoozy-macos.zip>` signs the Mac bundle with the Creator's Developer ID, has Apple notarize it, staples Apple's ticket to it, checks it, and only then writes the Mac download. That download is the one `compose --also` takes. Where something is missing, it stops before it touches the bundle and says what is missing. Its code is `src/the_mac_bundle.rs`, and what Apple requires is cited there.

It needs two things that only the Creator can put on this Mac. Neither is ever typed by an agent, and neither is ever in a file in this project.

### What the Creator does, once

**The certificate.** You must be the Account Holder on the Apple Developer account.

1. On this Mac, open **Keychain Access** (in Applications, then Utilities).
2. From the **Keychain Access** menu, choose **Certificate Assistant**, then **Request a Certificate From a Certificate Authority**.
3. Type your email address. In **Common Name**, type a name for the key, for example *Smoozy Developer ID*. Leave **CA Email Address** empty. Choose **Saved to disk**, click **Continue**, and save the file.
4. Sign in to your Apple Developer account in a browser and open **Certificates, Identifiers & Profiles**. Click **Certificates**, then the **+** button.
5. Under **Software**, choose **Developer ID**, click **Continue**, and choose **Developer ID Application**. Where it asks for a **Profile Type**, choose **G2 Sub-CA (Xcode 11.4.1 or later)**, not the default *Previous Sub-CA*. *(Added 2026-10-24 by the build runner, from the Creator meeting the choice.)*
6. Click **Choose File**, pick the file you saved in step 3, click **Continue**, then **Download**.
7. Double-click the downloaded file (it ends in `.cer`). It appears in Keychain Access under **My Certificates**.
8. To check, type this in Terminal. It should show one line with *Developer ID Application* in it:

   `security find-identity -v -p codesigning`

**The notarization password.**

9. Find your **Team ID**: in your Apple Developer account, click **Membership details**. It is ten letters and numbers.
10. Make an app-specific password: sign in at `account.apple.com`, open **Sign-In and Security**, choose **App-Specific Passwords**, then **Generate an app-specific password**. Name it *Smoozy notarization*. Keep the page open; you need the password in the next step.
11. In Terminal, type this, with your own Apple Account email and Team ID, and press Return:

    `xcrun notarytool store-credentials "Smoozy notarization" --apple-id "you@example.com" --team-id ABCDE12345`

    It asks for the password without showing what you type. Paste the app-specific password and press Return. It checks the password with Apple and then saves it in this Mac's keychain under the name *Smoozy notarization*, which is the name the release step looks for. **Do not put the password on the command line.**

That is all. Tell the release runner it is done.

**If you change your Apple Account password later**, Apple turns off every app-specific password, and the step will stop and say the notarization password did not answer. Do steps 10 and 11 again.

**If the keychain ever holds two Developer ID Application certificates** — after a renewal, say — the step stops rather than choose. Remove the one that should not be used in Keychain Access.

## The desktop update file

`compose` also writes **`desktop-update`**, the file every copy of Smoozy reads to learn of a new version (`Desktop app and delivery` E10a, E15, E19). It holds one entry per row of the release matrix — the row's download, its size and its fingerprint — signed with the **everyday key**, and every trust statement the two **master keys** have signed. Before anything is written, the file is read back through the desktop's own reading code for every row, and each row's download is checked against its entry; a file the desktop would refuse stops the whole release. Its code is `src/the_desktop_update.rs`, and the reading it goes through is `App/Desktop app and delivery/src/updating/`, never a copy of it.

Each row is given with `--update-row <row>=<name>`, naming one of the `--also` files. All five rows are needed: `macos-aarch64`, `windows-x86_64`, `ubuntu-24.04-amd64`, `ubuntu-26.04-amd64`, `debian-13-amd64`. For example:

```
--also Smoozy-macos.zip=<from>/Smoozy-macos.zip  --update-row macos-aarch64=Smoozy-macos.zip
```

`check` also fetches `desktop-update` from the address every copy reads it from (the releases repository's latest release), reads it the desktop's way for every row, and fetches each row's download from beside it to check its size and fingerprint.

**Until the build carries the two master keys, no update file is written or checked**, and both commands say so in one sentence. No copy of Smoozy could believe one, and a release should not stop for a key nothing can use yet. From the day the master keys are written into the build, every release writes the file, and a missing everyday key stops the release.

### What the Creator does, once, before the first release that carries it

This is after the update keys are made (the steps are in `Audits/Build/Step 11/D3, the trust set and the manifest — what landed.md`), when the everyday key's text is already in your password manager as *Smoozy update everyday key*.

1. Open **Terminal** and type this line, then press Return:

   `security add-generic-password -s "Smoozy update everyday key" -a smoozy-release -T "" -w`

2. It asks for the password without showing what you type. Paste the everyday key's text — sixty-four letters and digits, copied from the password manager item — and press Return. **Do not put the key on the command line.**

That makes one entry in your Mac's login keychain: its name is **Smoozy update everyday key**, its account is **smoozy-release**, and it holds the everyday key's text and nothing else. The `-T ""` keeps other programs from reading it silently, the same care the release signing key has. A release may ask for your login password when it reads it.

Never put a master key in the keychain. They are only ever needed to replace a lost or stolen key.
