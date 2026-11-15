# Dependency Upload & Remap — sources

Apple’s documented folder-access behaviour and the limits used by § A slow local read on macOS. Retrieved 2026-11-11. These are outside facts; the two-second delay and the words Smoozy shows are decisions in the Design document.

## Documents, Desktop and Downloads — documented

- [Documents](https://developer.apple.com/documentation/bundleresources/information-property-list/nsdocumentsfolderusagedescription): “the system prompts the user for permission to access the folder’s contents”; “the system remembers the user’s choice”.
- [Desktop](https://developer.apple.com/documentation/bundleresources/information-property-list/nsdesktopfolderusagedescription): “the system prompts the user for permission to access the folder’s contents”; “the system remembers the user’s choice”.
- [Downloads](https://developer.apple.com/documentation/bundleresources/information-property-list/nsdownloadsfolderusagedescription): “the system prompts the user for permission to access the folder’s contents”; “the system remembers the user’s choice”.

Each page says the first access without implied consent asks. Selecting a file through the ordinary Open or Save panel, dragging it to the app or opening it in Finder grants access to that file; an app’s own new file also needs no question. The pages describe a saved answer and a development reset command. Smoozy must respect either answer and does not run that command. A quit and reopen is not a permission reset.

## Removable volumes — documented

[NSRemovableVolumesUsageDescription](https://developer.apple.com/documentation/bundleresources/information-property-list/nsremovablevolumesusagedescription): “the system prompts the user for permission to access removable volumes”; “the system remembers the user’s choice”. First access without implied consent asks; selecting or creating a file gives that file access. A `/Volumes` path alone does not establish whether its disk is removable: the app names the folder it actually reads and says *may*.

## Files and Folders and a blocked read — documented

[Advances in macOS Security, WWDC19 session 701](https://developer.apple.com/videos/play/wwdc2019/701/), transcript, privacy-sensitive files section: “the calling thread is actually blocked, waiting for the user to approve or deny”; “handle any errors”. Apple describes Documents, Desktop, Downloads, cloud storage, removable and network volumes; reading existing contents can ask. Consent covers a category, while inferred intent covers selected files. A denied read must follow the existing unreadable-file or unreadable-folder outcome, not become a successful read.

## Requesting attention — documented, with a limit

[NSApplication.activate()](https://developer.apple.com/documentation/appkit/nsapplication/activate()): “calling this method doesn’t guarantee app activation”. Available from macOS 14. It requests activation of the app, not a named privacy question.

[NSApplication.activate(ignoringOtherApps:)](https://developer.apple.com/documentation/appkit/nsapplication/activate(ignoringotherapps:)): “Makes the receiver the active app”; “you shouldn’t assume the app will be active immediately”. The older call supports Smoozy’s macOS 11 minimum; on macOS 14 and later Smoozy uses the newer call. Neither page promises that a Files and Folders question will be raised.

[NSObjectProtocol.responds(to:)](https://developer.apple.com/documentation/objectivec/nsobjectprotocol/responds%28to%3A%29), retrieved 2026-11-11: “whether the receiver implements or inherits a method”. Smoozy checks that its app supports the newer activation call before using it.

## What the checked pages do not establish — unresolved

The folder/privacy pages above and the activation pages give no public waiting-question status, notification, or call to raise that particular question. No claim is made that a slow read proves one exists. The fallback observes only the beginning and end of a local read; its delay can also mean a slow disk. Folder names come from the requested path, without resolving it first (which would itself read the disk). Aliases or links can hide a protected destination: naming that destination is not promised. There is no permission-database inspection, accessibility automation, synthetic answer, or reset.
