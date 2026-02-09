# Source: https://docs.neurosity.co/docs/api/disconnect/

[Skip to main content](https://docs.neurosity.co/docs/api/disconnect/#__docusaurus_skipToContent_fallback)

You should always call disconnect the Neurosity instance when you're ending a session. This will clean up the session.

```js
import { Neurosity } from "@neurosity/sdk";

const neurosity = new Neurosity();

neurosity.disconnect();
```