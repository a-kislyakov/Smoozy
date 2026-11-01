# The user queues multiple jobs overnight

*A scenario: the user queues three separate render jobs before going to sleep, with auto-download enabled.*

## The situation

- One user, three projects, one rented machine.
- One server is saved in Smoozy, named "RunPod 4090", and Smoozy is connected to it.
- Its key has no passphrase. Blender is already installed on the machine.
- The user has prepared three jobs: Job A, Job B, and Job C. 
- Auto-download is enabled, pointing to a single local folder.
- Smoozy is open on the user's computer.

## The steps

1. The user submits Job A, Job B, and Job C in succession to the queue.

2. Job A immediately begins uploading.

3. The user goes to sleep, leaving Smoozy open and their computer connected to the internet.

4. Job A successfully finishes. Its outputs download.

5. Job B begins uploading and then starts rendering. During the render, it encounters a Blender out-of-memory error and fails.

6. Job C begins uploading and then starts rendering. It successfully finishes. Its outputs download.

7. The user wakes up and looks at Smoozy's queue and history to understand what succeeded, what failed, and where their downloaded files are.
