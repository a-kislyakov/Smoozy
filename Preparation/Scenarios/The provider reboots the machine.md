# The provider reboots the machine

*A scenario: the rented machine is forcibly rebooted by the provider while a render is running.*

## The situation

- One user, one project, one rented machine.
- One server is saved in Smoozy, named "RunPod 4090", and Smoozy is connected to it.
- Its key has no passphrase. Blender is already installed on the machine.
- A 100-frame animation job is submitted. The upload is finished, and the job is actively rendering on the server.
- The first 20 frames have finished.
- Auto-download is off.
- Smoozy is open on the user's computer.

## The steps

1. The user watches the progress in Smoozy as frames complete.

2. The server provider performs unannounced hardware maintenance and forcibly reboots the rented machine. The machine powers down abruptly, killing the Blender process, and powers back on.

3. The user looks at Smoozy to see what it reports about the connection dropping.

4. Three minutes pass. The machine finishes booting and comes back online.

5. The user looks at Smoozy to see if it automatically reconnects.

6. The user looks at the queue and the history, and at what is on the machine.

7. The user looks for a way to get the rest of the frames rendered.
