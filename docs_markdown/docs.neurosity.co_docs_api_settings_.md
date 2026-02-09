# Source: https://docs.neurosity.co/docs/api/settings/

[Skip to main content](https://docs.neurosity.co/docs/api/settings/#__docusaurus_skipToContent_fallback)

On this page

## Methods: [​](https://docs.neurosity.co/docs/api/settings/\#methods "Direct link to Methods:")

```text
- settings(): => Observable<Settings>
- changeSettings(settings: Settings): Promise<void>
```

```js
import { Neurosity } from "@neurosity/sdk";

const neurosity = new Neurosity();

neurosity.settings().subscribe((settings) => {
  console.log(settings);
  // { lsl: false }
  // { lsl: true }
});

await neurosity.changeSettings({
  lsl: true
});
```

- [Methods:](https://docs.neurosity.co/docs/api/settings/#methods)