# Experience

- **Native feel or nothing.** Platform conventions are law: back gesture and hardware back on Android, swipe-back on iOS, native navigation transitions, platform-correct haptics, share sheets, and keyboards. A web page in a shell is a rejected deliverable.
- **60fps is the floor:** lists virtualized with stable keys, images cached and sized, heavy work off the UI thread, animations on the native driver. A dropped frame during scroll is a defect, not a nuance.
- **Offline-first:** reads serve from cache instantly and refresh in the background; writes queue when offline and reconcile on reconnect; the user is told what is pending, never blocked by a blank screen. Airplane mode is a test case, not an excuse.
- **App lifecycle is hostile:** the OS kills, backgrounds, and restores at will — state survives it all; deep links and push notifications cold-start into the exact right screen with the stack behind it correct.
- **Touch is the medium:** targets at least 44pt, gestures discoverable and cancellable, pull-to-refresh where lists live, safe areas and notches respected on every screen.
- **Permissions in context:** ask for camera/location/notifications at the moment of use with the reason visible, degrade gracefully on denial, never ambush at launch.
- **Startup budget:** cold start to interactive content in under two seconds on a mid-range device; splash is a brand beat, not a loading strategy.
- **Battery and data are user property:** no polling where push serves, batched sync, downloads over Wi-Fi by preference, background work within OS budgets.
- Every state designed for the pocket: skeletons, empty states with actions, error states with retry — thumb-reachable, glanceable, interruptible.
