# The user renders the same project on a new machine

*A scenario: a week after finishing a project and throwing the machine away, the user needs one shot re-rendered, and rents a new machine for it.*

## The situation

- One user, one project.
- A week ago the user rendered a 200-frame animation from `shot_040.blend` on a machine called "RunPod 4090". It finished, they downloaded the frames, they destroyed the machine at the provider, and they removed the server from Smoozy.
- That job is still in Smoozy's history, marked as finished, with the setup it ran with.
- The `.blend` and all its assets are still on the user's own computer, exactly where they were a week ago.
- This morning the client asked for the shot again at a higher resolution.
- The user has rented a new machine, given the provider the same key as before, and it is now saved in Smoozy as "RunPod 4090 (2)" and connected. Blender is installed on it.
- Nothing has been uploaded to the new machine. Nothing is queued or running.
- Auto-download is off.
- Smoozy is open on the user's computer.

## The steps

1. The user opens the history and finds last week's finished job.

2. The user asks to submit that job again.

3. The user looks at what Smoozy shows them and what it asks for.

4. The user opens the file browser on "RunPod 4090 (2)" and sends `shot_040.blend` up to it from their own computer.

5. The user runs a check on that file for the pieces it is missing.

6. The user looks at what the check found.

7. The user sends up what has been found.

8. The user changes the resolution, and submits the job.

9. The user looks at the queue and at the history.
