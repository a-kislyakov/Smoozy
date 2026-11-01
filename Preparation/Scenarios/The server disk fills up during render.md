# The server disk fills up during render

*A scenario: the machine has enough space to start the job, and runs out partway through writing the frames.*

## The situation

- One user, one project, one rented machine.
- One server is saved in Smoozy, named "RunPod 4090", and Smoozy is connected to it.
- Its key has no passphrase. Blender is already installed on the machine.
- The machine has 4GB of free disk space.
- A 100-frame animation job is rendering on it. Its `.blend` and everything it needs went up in an earlier sitting. Each frame writes about 100MB.
- Nothing else is queued.
- Auto-download is off.
- Smoozy is open on the user's computer.

## The steps

1. The user watches the progress in Smoozy as frames complete.

2. Frame 40 finishes, and the machine's disk is now full. Frame 41 begins rendering.

3. The user looks at the queue entry.

4. The user looks at what is on the machine, and at what came of frames 40 and 41.

5. The user clears 100GB of space on the machine.

6. The user looks for a way to get the remaining frames rendered.
