# Source: https://docs.neurosity.co/docs/api/v6/

[Skip to main content](https://docs.neurosity.co/docs/api/v6/#__docusaurus_skipToContent_fallback)

On this page

The new version 6 of the SDK is mostly backwards compatible with v5.

There are 3 changes.

### 1\. Import name change [​](https://docs.neurosity.co/docs/api/v6/\#1-import-name-change "Direct link to 1. Import name change")

The most important change is the new renamed npm package and the name of the main SDK class.

Before:

```ts
import { Notion } from "@neurosity/notion";

const notion = new Notion();
```

After:

```ts
import { Neurosity } from "@neurosity/sdk";

const neurosity = new Neurosity();
```

### 2\. Single metric/label pair [​](https://docs.neurosity.co/docs/api/v6/\#2-single-metriclabel-pair "Direct link to 2. Single metric/label pair")

Previously, the following function could accept multiple labels:

Before:

```ts
neurosity.brainwaves("raw", "psd", "powerByBand");
neurosity.kinesis("push", "pull");
neurosity.predictions("push", "pull");
```

In v6, these functions only accept 1 label.

After:

```ts
neurosity.brainwaves("raw");
neurosity.kinesis("pull");
neurosity.predictions("push");
```

To get the same behavior as before, you can merge the streams:

```ts
import { merge } from "rxjs";

merge(
  neurosity.brainwaves("raw"),
  neurosity.brainwaves("psd"),
  neurosity.brainwaves("powerByBand")
).subscribe((brainwaves) => {
  //
});
```

### 3\. Removed Local Mode [​](https://docs.neurosity.co/docs/api/v6/\#3-removed-local-mode "Direct link to 3. Removed Local Mode")

We've removed the following local mode functions in favor of the new Bluetooth support:

```ts
neurosity.enableLocalMode();
neurosity.isLocalMode();
```

If you are interested in supporting Bluetooth for your app, you can check out the [Bluetooth tutorial](https://docs.neurosity.co/docs/api/streaming).

- [1\. Import name change](https://docs.neurosity.co/docs/api/v6/#1-import-name-change)
- [2\. Single metric/label pair](https://docs.neurosity.co/docs/api/v6/#2-single-metriclabel-pair)
- [3\. Removed Local Mode](https://docs.neurosity.co/docs/api/v6/#3-removed-local-mode)