# Mastery

React Native at the 60fps bar.

- **List performance is a discipline, not a fix:** the modern list component (FlashList-shaped) with fixed size estimates, memoized rows with stable props, images requested at cell size through a caching image component, pagination by keyset cursor prefetching ahead of scroll — the feed is profiled on a mid-range Android device, which is the real floor.
- **Animation stays off the JS thread:** Reanimated worklets for gestures-driving-motion (swipes, sheets, headers), layout animations for list changes, springs tuned to platform feel — any animation that stutters when JS is busy is running on the wrong thread.
- **Startup engineering:** measure cold start; defer everything after first interactive frame (lazy screens, deferred SDK inits, on-demand fonts); hydrate the query cache from disk so the first screen paints from cache while the network refreshes.
- **The offline queue in practice:** mutations persisted with ids + timestamps, replayed in order on reconnect, idempotent against the server (same keys the backend demands), conflicts resolved server-side with the client reconciling — pending state visible in the UI, never silent.
- **Media pipeline:** pick → crop/compress on-device → resumable background upload with progress → render CDN-sized variants; originals never re-upload, galleries never decode full-size images for thumbnails.
- **Push + deep links as one system:** notification payloads carry the same typed route params as links; cold start restores the full stack behind the target screen; badge and channel/category discipline per platform.
- **Native modules through one seam:** each native capability (biometrics, payments sheet, maps) behind a small TS facade — the port-and-adapter law; a library swap or a New-Architecture migration touches the facade, not forty call sites.
- **Release craft:** OTA for JS-level changes with staged rollout + instant rollback, store builds for native diffs; crash-free rate watched the first hours of every rollout; feature flags decouple launch from release.
- Debug with the real tools: native logs for native crashes, flame charts for JS stalls, network inspector for chatty sync — guessing layers wastes days; the trace names the thread.
