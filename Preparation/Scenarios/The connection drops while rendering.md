# The connection drops while rendering

*A scenario: the user submits a long animation render, but their local internet connection drops halfway through the render process.*

## The situation

- One user, one project, one rented machine.
- One server is saved in Smoozy, named "RunPod 4090", and Smoozy is connected to it.
- Its key has no passphrase. Blender is already installed on the machine.
- The user has submitted a 100-frame animation job. The upload is finished, and the job is currently rendering on the server.
- The first 10 frames have finished. 
- Auto-download is off.
- Smoozy is open on the user's computer.

## The steps

1. The user watches the progress in Smoozy as frames complete.

2. The user's local internet connection drops completely. Their computer has no internet access.

3. The user looks at Smoozy to see what it reports about the connection and the job.

4. An hour passes. The server finishes rendering all 100 frames during this hour, while the user is still offline.

5. The user's internet connection comes back online.

6. The user checks Smoozy to see if it reconnected automatically and what the history now says about the job.

7. The user downloads the completed frames.
