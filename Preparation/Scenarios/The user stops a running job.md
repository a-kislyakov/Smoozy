# The user stops a running job

*A scenario: the user spots a mistake in the settings a few frames into a render, and ends the job before it goes any further.*

## The situation

- One user, one project, one rented machine.
- One server is saved in Smoozy, named "RunPod 4090", and Smoozy is connected to it.
- Its key has no passphrase. Blender is already installed on the machine.
- A 100-frame animation job is rendering on the machine. Its `.blend` and everything it needs went up in an earlier sitting.
- The first 5 frames have finished. Frame 6 is about twenty minutes into rendering.
- Nothing else is queued.
- Auto-download is off.
- Smoozy is open on the user's computer.

## The steps

1. The user watches the first few frames complete and sees that the resolution is wrong.

2. The user stops the job.

3. The user looks at the queue and at the history.

4. The user looks for the 5 finished frames and brings them down.

5. The user deletes the stopped job from the history.

6. The user looks at the history again.
