# Source: https://docs.neurosity.co/docs/api/status/

[Skip to main content](https://docs.neurosity.co/docs/api/status/#__docusaurus_skipToContent_fallback)

On this page

## Metrics: [​](https://docs.neurosity.co/docs/api/status/\#metrics "Direct link to Metrics:")

- state: "online" \| "offline" \| "shuttingOff" \| "updating" \| "booting"
- sleepMode: boolean
- sleepModeReason: "updating" \| "charging" \| null
- charging: boolean
- battery: number
- lastHeartbeat: number
- ssid: string
- claimedBy: string

```js
import { Neurosity } from "@neurosity/sdk";

const neurosity = new Neurosity();

neurosity.status().subscribe((status) => {
  console.log(status);
  // status example: { state: "online", charging: true, battery: 93, ... }
});
```

- [Metrics:](https://docs.neurosity.co/docs/api/status/#metrics)