# The machine is destroyed

*A scenario: the user finishes a project, destroys the rented machine at the provider while Smoozy is still connected to it, and then tidies up.*

## The situation

- One user, one project, one rented machine. They have been using Smoozy for a few days.
- **One server** is saved in Smoozy, named "RunPod 4090", and Smoozy is **connected** to it. It is the only server in the list.
- Its key has **no passphrase**. Blender is already installed on the machine. Nothing is marked as needing a new key.
- **One render job** — an animation — was submitted a few hours ago and has **finished**. It is in that machine's history, marked as finished.
- The user has already **downloaded the outputs** to their own machine and checked them. They are happy with the render.
- Nothing else is queued. Nothing else is running. Auto-download was never switched on.
- Smoozy has been **open the whole time**, so whatever it read from the machine while connected, it still holds.

The user is done with this project and does not want to pay for the machine any longer. They intend to destroy it at the provider and then tidy Smoozy up.

## The steps

1. The user looks at Smoozy — the server list, the queue, and the history.

2. Leaving Smoozy open, the user switches to their browser and goes to the provider's website.

3. The user destroys the rented machine at the provider. It stops existing. They do not touch Smoozy while doing this.

4. The user switches back to Smoozy and looks at the server list.

5. The user keeps watching it.

6. The user opens the history to look at the finished job again.

7. The user removes "RunPod 4090" from Smoozy.

8. The removal goes through, and the user looks at the server list, the queue, and the history.
