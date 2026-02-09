# Source: https://docs.neurosity.co/docs/api/signal-quality/

[Skip to main content](https://docs.neurosity.co/docs/api/signal-quality/#__docusaurus_skipToContent_fallback)

Standard deviation based signal quality metrics. Great signal happens when the standard deviation is between 1.5 and 10. See [`SignalQuality`](https://docs.neurosity.co/docs/reference/interfaces/signalquality) for using in code.

```js
import { Neurosity } from "@neurosity/sdk";

const neurosity = new Neurosity();

neurosity.signalQuality().subscribe((signalQuality) => {
  console.log(signalQuality);
});
```