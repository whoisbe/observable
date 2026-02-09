# Source: https://docs.neurosity.co/docs/api/bluetooth-web/

[Skip to main content](https://docs.neurosity.co/docs/api/bluetooth-web/#__docusaurus_skipToContent_fallback)

On this page

> ⚠️ **Requires**: Neurosity OS v16+, to be released in January 2023

Not all browsers support Web Bluetooth. You can refer to browser-specific support [here](https://caniuse.com/web-bluetooth).

Additionally, there are some Browser Bluetooth flags that need to be enabled:

![Browser Feature Flags](https://docs.neurosity.co/img/api/web-bluetooth-browser-flags.png)

[chrome://flags/#enable-experimental-web-platform-features](chrome://flags/#enable-experimental-web-platform-features)

Before

```jsx
import { Neurosity } from "@neurosity/sdk";

export const neurosity = new Neurosity({
  autoSelectDevice: true
});
```

After

```ts
import { Neurosity, WebBluetoothTransport } from "@neurosity/sdk";

export const neurosity = new Neurosity({
  autoSelectDevice: true,
  bluetoothTransport: new WebBluetoothTransport(),
  streamingMode: "bluetooth-with-wifi-fallback"
});
```

When using Bluetooth, there are 2 streaming modes you can choose from:

- `wifi-with-bluetooth-fallback`
- `bluetooth-with-wifi-fallback`

## Bluetooth Connection State [​](https://docs.neurosity.co/docs/api/bluetooth-web/\#bluetooth-connection-state "Direct link to Bluetooth Connection State")

```ts
const { bluetooth } = neurosity;

bluetooth.connection().subscribe((connection) => {
  console.log(`Bluetooth connected is ${connection}`);
});
```

The following connection states are possible:

```ts
enum BLUETOOTH_CONNECTION {
  SCANNING = "scanning",
  CONNECTED = "connected",
  CONNECTING = "connecting",
  DISCONNECTING = "disconnecting",
  DISCONNECTED = "disconnected"
}
```

## Auto Connect [​](https://docs.neurosity.co/docs/api/bluetooth-web/\#auto-connect "Direct link to Auto Connect")

By default, the Web Bluetooth transport will attempt to auto connect to the [selected device](https://docs.neurosity.co/docs/api/device-selection). To disable this behavior, set the `autoConnect` transport option to `false`:

```ts
import { Neurosity, WebBluetoothTransport } from "@neurosity/sdk";

export const neurosity = new Neurosity({
  autoSelectDevice: true,
  bluetoothTransport: new WebBluetoothTransport({
    autoConnect: false
  }),
  streamingMode: "bluetooth-with-wifi-fallback"
});
```

It is also possible to enable or disable this behavior at runtime:

```ts
const { bluetooth } = neurosity;

bluetooth.enableAutoConnect(true);

// or

bluetooth.enableAutoConnect(false);
```

- [Bluetooth Connection State](https://docs.neurosity.co/docs/api/bluetooth-web/#bluetooth-connection-state)
- [Auto Connect](https://docs.neurosity.co/docs/api/bluetooth-web/#auto-connect)