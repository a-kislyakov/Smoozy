# The user adds a texture and renders again

*A scenario: the user watches back a finished render, adds a texture to the file, and sends the same job round again.*

## The situation

- One user, one project, one rented machine.
- One server is saved in Smoozy, named "RunPod 4090", and Smoozy is connected to it.
- Its key has no passphrase. Blender is already installed on the machine.
- Yesterday the user rendered a 200-frame animation from `shot_040.blend` on this machine. It finished, and it is in that machine's history marked as finished.
- The `.blend` and every asset it needs went up for that job and are still on the machine.
- The user has already downloaded yesterday's frames and watched them back.
- Nothing is queued or running.
- Auto-download is off.
- Smoozy is open on the user's computer.

## The steps

1. Watching yesterday's frames, the user decides the floor needs a texture on it.

2. The user opens `shot_040.blend` in Blender on their own computer, points the floor material at a 200MB displacement map sitting in their project's texture folder, and saves the file. That map has never been on the machine.

3. The user goes to Smoozy, opens the file browser on "RunPod 4090", and sends `shot_040.blend` up to the folder it was in before.

4. Smoozy asks about the file already being there. The user chooses to replace it.

5. Once the file is up, the user opens the render settings for it and sets the same 200-frame range as yesterday.

6. The user submits the job, and looks at what Smoozy shows them as they do.

7. The user looks at the queue.

8. The job finishes. The user looks at the history and downloads the frames.

9. The user opens the frames.
