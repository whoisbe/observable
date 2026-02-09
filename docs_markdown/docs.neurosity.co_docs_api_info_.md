# Source: https://docs.neurosity.co/docs/api/info/

[Skip to main content](https://docs.neurosity.co/docs/api/info/#__docusaurus_skipToContent_fallback)

On this page

Non-mutable device information.

## Metrics: [​](https://docs.neurosity.co/docs/api/info/\#metrics "Direct link to Metrics:")

```js
interface IInfo {
  deviceId: string;
  channels: number;
  channelNames: Array<string>;
  samplingRate: number;
  manufacturer: string;
  model: string;
  osVersion: string;
  apiVersion: string;
}
```

## Example [​](https://docs.neurosity.co/docs/api/info/\#example "Direct link to Example")

```js
import { Neurosity } from "@neurosity/sdk";

const neurosity = new Neurosity();

const info = await neurosity.getInfo();
console.log(info); // { channels: 8, samplingRate: 250, ... }
```

- [Metrics:](https://docs.neurosity.co/docs/api/info/#metrics)
- [Example](https://docs.neurosity.co/docs/api/info/#example)