# Source: https://docs.neurosity.co/docs/api/v7/

[Skip to main content](https://docs.neurosity.co/docs/api/v7/#__docusaurus_skipToContent_fallback)

On this page

The new version 7 of the SDK is mostly backwards compatible with v6.

There are some small breaking changes.

### 1\. TypeScript Types import path change [​](https://docs.neurosity.co/docs/api/v7/\#1-typescript-types-import-path-change "Direct link to 1. TypeScript Types import path change")

All TypeScript types are now imported directly from the `@neurosity/sdk` path.

There's not longer a need to reference them from the `@neurosity/neurosity/dist/esm/{typeName}` path.

Before:

```ts
import { DeviceStatus } from "@neurosity/sdk/dist/esm/types/status";
```

After:

```ts
import { DeviceStatus } from "@neurosity/sdk";
```

### Consolidated bundles [​](https://docs.neurosity.co/docs/api/v7/\#consolidated-bundles "Direct link to Consolidated bundles")

In v6 and prior, the SDK exported a custom bundle for Electron. In v7, the default import "@neurosity/sdk" will now use the same bundle as the browser and Node.js imports.

Before:

```ts
import { Neurosity } from "@neurosity/sdk/dist/electron";
```

After:

```ts
import { Neurosity } from "@neurosity/sdk";
```

- [1\. TypeScript Types import path change](https://docs.neurosity.co/docs/api/v7/#1-typescript-types-import-path-change)
- [Consolidated bundles](https://docs.neurosity.co/docs/api/v7/#consolidated-bundles)