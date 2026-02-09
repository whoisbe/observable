# Source: https://docs.neurosity.co/docs/reference/classes/neurosity/

[Skip to main content](https://docs.neurosity.co/docs/reference/classes/neurosity/#__docusaurus_skipToContent_fallback)

On this page

[**@neurosity/sdk**](https://docs.neurosity.co/docs/reference/)

* * *

[@neurosity/sdk](https://docs.neurosity.co/docs/reference/) / Neurosity

# Class: Neurosity

Defined in: [Neurosity.ts:70](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L70)

Example

```typescript
import { Neurosity } from "@neurosity/sdk";

const neurosity = new Neurosity();
```

## Constructors [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#constructors "Direct link to Constructors")

### Constructor [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#constructor "Direct link to Constructor")

> **new Neurosity**(`options`): `Neurosity`

Defined in: [Neurosity.ts:113](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L113)

Creates new instance of the Neurosity SDK

```typescript
const neurosity = new Neurosity();
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters "Direct link to Parameters")

##### options [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#options "Direct link to options")

[`SDKOptions`](https://docs.neurosity.co/docs/reference/interfaces/SDKOptions) = `{}`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns "Direct link to Returns")

`Neurosity`

## Accessors [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#accessors "Direct link to Accessors")

### training [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#training "Direct link to training")

#### Get Signature [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#get-signature "Direct link to Get Signature")

> **get** **training**(): `Training`

Defined in: [Neurosity.ts:1245](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1245)

Streaming modes:Wi-Fi

```typescript
neurosity.training.record({
  metric: "kinesis",
  label: "push"
});

neurosity.training.stop({
  metric: "kinesis",
  label: "push"
});
```

##### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-1 "Direct link to Returns")

`Training`

Training methods

## Methods [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#methods "Direct link to Methods")

### \_\_getApp() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#__getapp "Direct link to __getApp()")

> **\_\_getApp**(): `FirebaseApp`

Defined in: [Neurosity.ts:389](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L389)

**`Internal`**

Not user facing.

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-2 "Direct link to Returns")

`FirebaseApp`

* * *

### accelerometer() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#accelerometer "Direct link to accelerometer()")

> **accelerometer**(): `Observable`< [`Accelerometer`](https://docs.neurosity.co/docs/reference/interfaces/Accelerometer) >

Defined in: [Neurosity.ts:823](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L823)

Streaming modes:Wi-FiBluetooth

Observes accelerometer data
Supported by the Crown and Notion 2 devices.

```typescript
neurosity.accelerometer().subscribe(accelerometer => {
  console.log(accelerometer);
});

// { acceleration: ..., inclination: ..., orientation: ..., pitch: ..., roll: ..., x: ..., y: ..., z: ... }
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-3 "Direct link to Returns")

`Observable`< [`Accelerometer`](https://docs.neurosity.co/docs/reference/interfaces/Accelerometer) >

Observable of accelerometer metric events

* * *

### addDevice() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#adddevice "Direct link to addDevice()")

> **addDevice**(`deviceId`): `Promise`<`void`>

Defined in: [Neurosity.ts:415](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L415)

Add a device to the user's account

```typescript
await neurosity.addDevice("[deviceId]");
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-1 "Direct link to Parameters")

##### deviceId [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#deviceid "Direct link to deviceId")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-4 "Direct link to Returns")

`Promise`<`void`>

* * *

### addMarker() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#addmarker "Direct link to addMarker()")

> **addMarker**(`label`): `Promise`<`Action`>

Defined in: [Neurosity.ts:694](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L694)

Streaming modes:Wi-FiBluetooth

Injects an EEG marker to data stream

```typescript
neurosity.addMarker("eyes-closed");

// later...

neurosity.addMarker("eyes-open");
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-2 "Direct link to Parameters")

##### label [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#label "Direct link to label")

`string`

Name the label to inject

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-5 "Direct link to Returns")

`Promise`<`Action`>

* * *

### brainwaves() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#brainwaves "Direct link to brainwaves()")

> **brainwaves**(`label`): `Observable`< [`Epoch`](https://docs.neurosity.co/docs/reference/interfaces/Epoch) \| [`PSD`](https://docs.neurosity.co/docs/reference/interfaces/PSD) \| [`PowerByBand`](https://docs.neurosity.co/docs/reference/interfaces/PowerByBand) >

Defined in: [Neurosity.ts:903](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L903)

Streaming modes:Wi-FiBluetooth

The `raw` brainwaves parameter emits epochs of 16 samples for Crown and 25 for Notion 1 and 2.

Example

```typescript
neurosity.brainwaves("raw").subscribe(brainwaves => {
  console.log(brainwaves);
});
```

Raw Unfiltered - The `rawUnfiltered` brainwaves parameter emits epochs of 16 samples for Crown and 25 for Notion 1 and 2.

Example

```typescript
neurosity.brainwaves("rawUnfiltered").subscribe(brainwaves => {
  console.log(brainwaves);
});
```

Power By Band - The `powerByBand` brainwaves parameter emits epochs 4 times a second. Every frequency label (e.g. beta) contains an average power value per channel.

Example

```typescript
neurosity.brainwaves("powerByBand").subscribe(brainwaves => {
  console.log(brainwaves);
});
```

Power Spectral Density (PSD) - The `psd` brainwaves parameter emits epochs 4 times a second. Every frequency label (e.g. alpha) contains the computed FFT (Fast Fourier transform) value per channel (see the `psd` property), as well as the frequency ranges (see the `freqs` property).

Example

```typescript
neurosity.brainwaves("psd").subscribe(brainwaves => {
  console.log(brainwaves);
});
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-3 "Direct link to Parameters")

##### label [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#label-1 "Direct link to label")

[`BrainwavesLabel`](https://docs.neurosity.co/docs/reference/type-aliases/BrainwavesLabel)

Name of metric properties to filter by

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-6 "Direct link to Returns")

`Observable`< [`Epoch`](https://docs.neurosity.co/docs/reference/interfaces/Epoch) \| [`PSD`](https://docs.neurosity.co/docs/reference/interfaces/PSD) \| [`PowerByBand`](https://docs.neurosity.co/docs/reference/interfaces/PowerByBand) >

Observable of brainwaves metric events

* * *

### calm() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#calm "Direct link to calm()")

> **calm**(): `Observable`< [`Calm`](https://docs.neurosity.co/docs/reference/interfaces/Calm) >

Defined in: [Neurosity.ts:947](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L947)

Streaming modes:Wi-FiBluetooth

Example

```typescript
neurosity.calm().subscribe(calm => {
  console.log(calm.probability);
});

// 0.45
// 0.47
// 0.53
// 0.51
// ...
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-7 "Direct link to Returns")

`Observable`< [`Calm`](https://docs.neurosity.co/docs/reference/interfaces/Calm) >

Observable of calm events - awareness/calm alias

* * *

### changeSettings() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#changesettings "Direct link to changeSettings()")

> **changeSettings**(`settings`): `Promise`<`void`>

Defined in: [Neurosity.ts:1210](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1210)

Streaming modes:Wi-Fi

Changes device settings programmatically. These settings can be
also changed from the developer console under device settings.

Available settings \[\[Settings\]\]

Example

```typescript
neurosity.changeSettings({
  lsl: true
});
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-4 "Direct link to Parameters")

##### settings [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#settings "Direct link to settings")

[`Settings`](https://docs.neurosity.co/docs/reference/interfaces/Settings)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-8 "Direct link to Returns")

`Promise`<`void`>

* * *

### createAccount() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#createaccount "Direct link to createAccount()")

> **createAccount**(`credentials`): `Promise`<`any`>

Defined in: [Neurosity.ts:1338](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1338)

Creates user account and automatically signs in with same credentials

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-5 "Direct link to Parameters")

##### credentials [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#credentials "Direct link to credentials")

[`EmailAndPassword`](https://docs.neurosity.co/docs/reference/type-aliases/EmailAndPassword)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-9 "Direct link to Returns")

`Promise`<`any`>

user credential

* * *

### createApiKey() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#createapikey "Direct link to createApiKey()")

> **createApiKey**(`data`): `Promise`<`ApiKeyRecord`>

Defined in: [Neurosity.ts:1389](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1389)

Creates API key to use to login with `{ apiKey }`.

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-6 "Direct link to Parameters")

##### data [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#data "Direct link to data")

`CreateApiKeyRequest`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-10 "Direct link to Returns")

`Promise`<`ApiKeyRecord`>

API key record

* * *

### createBluetoothToken() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#createbluetoothtoken "Direct link to createBluetoothToken()")

> **createBluetoothToken**(): `Promise`<`string`>

Defined in: [Neurosity.ts:1358](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1358)

**`Internal`**

Not user facing

Creates token (JWT) designed to authenticate and authorize Bluetooth clients/centrals.

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-11 "Direct link to Returns")

`Promise`<`string`>

token

* * *

### createCustomToken() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#createcustomtoken "Direct link to createCustomToken()")

> **createCustomToken**(): `Promise`< [`CustomToken`](https://docs.neurosity.co/docs/reference/type-aliases/CustomToken) >

Defined in: [Neurosity.ts:1370](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1370)

**`Internal`**

Not user facing yet

Creates custom token (JWT) to use to login with `{ customToken }`.

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-12 "Direct link to Returns")

`Promise`< [`CustomToken`](https://docs.neurosity.co/docs/reference/type-aliases/CustomToken) >

custom token

* * *

### createOAuthURL() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#createoauthurl "Direct link to createOAuthURL()")

> **createOAuthURL**(`config`): `Promise`<`string`>

Defined in: [Neurosity.ts:1482](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1482)

Create OAuth URL
💡 OAuth requires developers to register their apps with Neurosity
[Read full OAuth guide](https://docs.neurosity.co/docs/api/oauth)

Creates client-specific OAuth URL. This is the first step of the OAuth workflow. Use this function to create a URL you can use to redirect users to the Neurosity sign-in page.
💡 This function is designed to only run on the server side for security reasons, as it requires your client secret.

```typescript
const { Neurosity } = require("@neurosity/sdk");

const neurosity = new Neurosity({
  autoSelectDevice: false
});

exports.handler = async function (event) {
  return neurosity
    .createOAuthURL({
      clientId: process.env.NEUROSITY_OAUTH_CLIENT_ID,
      clientSecret: process.env.NEUROSITY_OAUTH_CLIENT_SECRET,
      redirectUri: process.env.NEUROSITY_OAUTH_CLIENT_REDIRECT_URI,
      responseType: "token",
      state: Math.random().toString().split(".")[1],
      scope: [\
        "read:devices-info",\
        "read:devices-status",\
        "read:signal-quality",\
        "read:brainwaves"\
      ]
    })
    .then((url) => ({
      statusCode: 200,
      body: JSON.stringify({ url })
    }))
    .catch((error) => ({
      statusCode: 400,
      body: JSON.stringify(error.response.data)
    }));
};
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-7 "Direct link to Parameters")

##### config [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#config "Direct link to config")

[`OAuthConfig`](https://docs.neurosity.co/docs/reference/type-aliases/OAuthConfig)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-13 "Direct link to Returns")

`Promise`<`string`>

custom token

* * *

### deleteAccount() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#deleteaccount "Direct link to deleteAccount()")

> **deleteAccount**(): `Promise`<`void`>

Defined in: [Neurosity.ts:1346](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1346)

Removes all devices from an account and then deletes the account

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-14 "Direct link to Returns")

`Promise`<`void`>

* * *

### deleteUserExperiment() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#deleteuserexperiment "Direct link to deleteUserExperiment()")

> **deleteUserExperiment**(`experimentId`): `Promise`<`void`>

Defined in: [Neurosity.ts:1604](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1604)

Streaming modes:Wi-Fi

Deletes a specific experiment provided an experiment ID

```typescript
await neurosity.deleteUserExperiment(experiment.id);
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-8 "Direct link to Parameters")

##### experimentId [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#experimentid "Direct link to experimentId")

`string`

The ID of the Experiment

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-15 "Direct link to Returns")

`Promise`<`void`>

void

* * *

### disconnect() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#disconnect "Direct link to disconnect()")

> **disconnect**(): `Promise`<`void`>

Defined in: [Neurosity.ts:646](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L646)

Streaming modes:Wi-FiBluetooth

Ends database connection

```typescript
await neurosity.disconnect();
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-16 "Direct link to Returns")

`Promise`<`void`>

* * *

### focus() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#focus "Direct link to focus()")

> **focus**(): `Observable`< [`Focus`](https://docs.neurosity.co/docs/reference/interfaces/Focus) >

Defined in: [Neurosity.ts:1087](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1087)

Streaming modes:Wi-FiBluetooth

Example

```typescript
neurosity.focus().subscribe(focus => {
  console.log(focus.probability);
});

// 0.56
// 0.46
// 0.31
// 0.39
// ...
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-17 "Direct link to Returns")

`Observable`< [`Focus`](https://docs.neurosity.co/docs/reference/interfaces/Focus) >

Observable of focus events - awareness/focus alias

* * *

### getDevices() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#getdevices "Direct link to getDevices()")

> **getDevices**(): `Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo)\[\]>

Defined in: [Neurosity.ts:520](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L520)

Get user devices

Returns a list of devices claimed by the user authenticated.

```typescript
const devices = await neurosity.getDevices();
console.log(devices);
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-18 "Direct link to Returns")

`Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo)\[\]>

* * *

### getHapticEffects() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#gethapticeffects "Direct link to getHapticEffects()")

> **getHapticEffects**(): [`HapticEffects`](https://docs.neurosity.co/docs/reference/type-aliases/HapticEffects)

Defined in: [Neurosity.ts:803](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L803)

```typescript
const effects = neurosity.getHapticEffects();
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-19 "Direct link to Returns")

[`HapticEffects`](https://docs.neurosity.co/docs/reference/type-aliases/HapticEffects)

* * *

### getInfo() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#getinfo "Direct link to getInfo()")

> **getInfo**(): `Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

Defined in: [Neurosity.ts:593](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L593)

```typescript
const info = await neurosity.getInfo();
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-20 "Direct link to Returns")

`Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

* * *

### getOAuthToken() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#getoauthtoken "Direct link to getOAuthToken()")

> **getOAuthToken**(`query`): `Promise`< [`OAuthQueryResult`](https://docs.neurosity.co/docs/reference/type-aliases/OAuthQueryResult) >

Defined in: [Neurosity.ts:1533](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1533)

Get OAuth Token
💡 OAuth requires developers to register their apps with Neurosity
[Read full OAuth guide](https://docs.neurosity.co/docs/api/oauth)

Gets client-specific OAuth token for a given userId.

💡 This function is designed to only run on the server side for security reasons, as it requires your client secret.
Here's an example of a cloud function that receives a `userId` via query params and loads the client id and client secret securely via environment variables.

```typescript
const { Neurosity } = require("@neurosity/sdk");

const neurosity = new Neurosity({
  autoSelectDevice: false
});

exports.handler = async function (event) {
  const userId = event.queryStringParameters?.userId;

  return neurosity
    .getOAuthToken({
      clientId: process.env.NEUROSITY_OAUTH_CLIENT_ID,
      clientSecret: process.env.NEUROSITY_OAUTH_CLIENT_SECRET,
      userId
    })
    .then((token) => ({
      statusCode: 200,
      body: JSON.stringify(token)
    }))
    .catch((error) => ({
      statusCode: 200,
      body: JSON.stringify(error.response.data)
    }));
};
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-9 "Direct link to Parameters")

##### query [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#query "Direct link to query")

[`OAuthQuery`](https://docs.neurosity.co/docs/reference/type-aliases/OAuthQuery)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-21 "Direct link to Returns")

`Promise`< [`OAuthQueryResult`](https://docs.neurosity.co/docs/reference/type-aliases/OAuthQueryResult) >

custom token

* * *

### getSelectedDevice() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#getselecteddevice "Direct link to getSelectedDevice()")

> **getSelectedDevice**(): `Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

Defined in: [Neurosity.ts:574](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L574)

Get selected device

```typescript
const selectedDevice = await neurosity.getSelectedDevice();
console.log(selectedDevice);
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-22 "Direct link to Returns")

`Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

* * *

### getTimesyncOffset() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#gettimesyncoffset "Direct link to getTimesyncOffset()")

> **getTimesyncOffset**(): `number`

Defined in: [Neurosity.ts:1432](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1432)

**`Internal`**

Not user facing yet

Gets the offset between the device's clock and the client's clock
Requires option.timesync to be true

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-23 "Direct link to Returns")

`number`

timesyncOffset

* * *

### goOffline() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#gooffline "Direct link to goOffline()")

> **goOffline**(): `void`

Defined in: [Neurosity.ts:1319](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1319)

**`Internal`**

Proof of Concept for disconnecting db

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-24 "Direct link to Returns")

`void`

* * *

### goOnline() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#goonline "Direct link to goOnline()")

> **goOnline**(): `void`

Defined in: [Neurosity.ts:1327](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1327)

**`Internal`**

Proof of Concept for resuming db connection

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-25 "Direct link to Returns")

`void`

* * *

### haptics() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#haptics "Direct link to haptics()")

> **haptics**(`effects`): `Promise`<`any`>

Defined in: [Neurosity.ts:754](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L754)

Streaming modes:Wi-FiBluetooth

Queue haptic motor commands

To queue haptic P7 only,

```typescript
await neurosity.haptics({
  P7: ["tripleClick100"]
});
```

To queue both motors at the same time

```typescript
await neurosity.haptics({
  P7: [neurosity.getHapticEffects().strongClick100],
  P8: [neurosity.getHapticEffects().strongClick100]
});
```

You can queue different commands to the motors too

```typescript
const effects = neurosity.getHapticEffects();
await neurosity.haptics({
  P7: [effects.transitionRampUpLongSmooth1_0_to_100,\
        effects.transitionRampDownLongSmooth1_100_to_0],
  P8: [effects.strongClick100]
});
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-10 "Direct link to Parameters")

##### effects [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#effects "Direct link to effects")

`any`

Effects to queue. The key of the object passed should be the location of the motor
to queue. Each key can be an array of up to 7 commands. There is no haptic support for model
version 1, Notion DK1. The Haptic motor's location is positioned in reference to the 10-10 EEG
system used to label the channels of the Crown's EEG sensors. Notion 2 and Crown have haptics
at P7 and P8. A list of haptic commands can be found on ./utils/hapticCodes.ts - there
are about 127 of them!

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-26 "Direct link to Returns")

`Promise`<`any`>

* * *

### kinesis() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#kinesis "Direct link to kinesis()")

> **kinesis**(`label`): `Observable`< [`Kinesis`](https://docs.neurosity.co/docs/reference/interfaces/Kinesis) >

Defined in: [Neurosity.ts:1115](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1115)

Streaming modes:Wi-Fi

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-11 "Direct link to Parameters")

##### label [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#label-2 "Direct link to label")

`string`

Name of metric properties to filter by

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-27 "Direct link to Returns")

`Observable`< [`Kinesis`](https://docs.neurosity.co/docs/reference/interfaces/Kinesis) >

Observable of kinesis metric events

* * *

### login() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#login "Direct link to login()")

> **login**(`credentials`): `Promise`<`void`>

Defined in: [Neurosity.ts:368](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L368)

Starts user session

```typescript
await neurosity.login({
  email: "...",
  password: "..."
});
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-12 "Direct link to Parameters")

##### credentials [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#credentials-1 "Direct link to credentials")

[`Credentials`](https://docs.neurosity.co/docs/reference/type-aliases/Credentials)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-28 "Direct link to Returns")

`Promise`<`void`>

* * *

### logout() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#logout "Direct link to logout()")

> **logout**(): `Promise`<`void`>

Defined in: [Neurosity.ts:381](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L381)

Ends user session

```typescript
await neurosity.logout();
// session has ended
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-29 "Direct link to Returns")

`Promise`<`void`>

* * *

### onAuthStateChanged() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#onauthstatechanged "Direct link to onAuthStateChanged()")

> **onAuthStateChanged**(): `Observable`<`any`>

Defined in: [Neurosity.ts:404](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L404)

Subscribe to auth state changes

Streams the state of the auth session. If user has logged in, the user object will be set. When logged out, the user object will be null.

```typescript
neurosity.onAuthStateChanged().subscribe((user) => {
  console.log(user);
});
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-30 "Direct link to Returns")

`Observable`<`any`>

* * *

### onDeviceChange() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#ondevicechange "Direct link to onDeviceChange()")

> **onDeviceChange**(): `Observable`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

Defined in: [Neurosity.ts:623](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L623)

Observes selected device

```typescript
neurosity.onDeviceChange().subscribe(device => {
 console.log(device);
});
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-31 "Direct link to Returns")

`Observable`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

* * *

### onUserClaimsChange() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#onuserclaimschange "Direct link to onUserClaimsChange()")

> **onUserClaimsChange**(): `Observable`<`UserClaims`>

Defined in: [Neurosity.ts:506](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L506)

Subscribe to user claims changes

```typescript
neurosity.onUserClaimsChange().subscribe((userClaims) => {
  console.log(userClaims);
});
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-32 "Direct link to Returns")

`Observable`<`UserClaims`>

* * *

### onUserDevicesChange() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#onuserdeviceschange "Direct link to onUserDevicesChange()")

> **onUserDevicesChange**(): `Observable`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo)\[\]>

Defined in: [Neurosity.ts:483](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L483)

Subscribe to user devices changes

```typescript
neurosity.onUserDevicesChange().subscribe((devices) => {
  console.log(devices);
});
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-33 "Direct link to Returns")

`Observable`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo)\[\]>

* * *

### onUserExperiments() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#onuserexperiments "Direct link to onUserExperiments()")

> **onUserExperiments**(): `Observable`< [`Experiment`](https://docs.neurosity.co/docs/reference/type-aliases/Experiment)\[\]>

Defined in: [Neurosity.ts:1588](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1588)

Streaming modes:Wi-Fi

Observes and returns a list of all Kinesis `experiments` and all subsequent experiment changes.
Here's an example of how to get a list of all Kinesis labels that have been trained:

```typescript

const getUniqueLabels = (experiments) => {
  const labels = experiments.flatMap((experiment) => experiment.labels);
  // only return unique labels
  return [...new Set(labels)];
}

neurosity.onUserExperiments().subscribe((experiments) => {
  console.log(experiments);
  console.log("labels", getUniqueLabels(experiments));
});

// [{ id: '...', deviceId: '...', labels: [ 'drop' ], name: 'Lightgray cheetah', timestamp: 1577908381552, totalTrials: 16, userId: '...' }]
// ["drop", "lift", "push"]
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-34 "Direct link to Returns")

`Observable`< [`Experiment`](https://docs.neurosity.co/docs/reference/type-aliases/Experiment)\[\]>

Observable of `experiments` events

* * *

### osVersion() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#osversion "Direct link to osVersion()")

> **osVersion**(): `Observable`<`string`>

Defined in: [Neurosity.ts:1055](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1055)

Streaming modes:Wi-Fi

Observes the current OS version and all subsequent version changes in real-time.

```typescript
neurosity.osVersion().subscribe((osVersion) => {
  console.log(osVersion);
});

// "16.0.0"
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-35 "Direct link to Returns")

`Observable`<`string`>

Observable of `osVersion` events. e.g 16.0.0

* * *

### predictions() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#predictions "Direct link to predictions()")

> **predictions**(`label`): `Observable`<`any`>

Defined in: [Neurosity.ts:1141](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1141)

Streaming modes:Wi-Fi

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-13 "Direct link to Parameters")

##### label [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#label-3 "Direct link to label")

`string`

Name of metric properties to filter by

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-36 "Direct link to Returns")

`Observable`<`any`>

Observable of predictions metric events

* * *

### removeApiKey() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#removeapikey "Direct link to removeApiKey()")

> **removeApiKey**(`apiKeyId`): `Promise`<`RemoveApiKeyResponse`>

Defined in: [Neurosity.ts:1409](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1409)

Removes API key

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-14 "Direct link to Parameters")

##### apiKeyId [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#apikeyid "Direct link to apiKeyId")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-37 "Direct link to Returns")

`Promise`<`RemoveApiKeyResponse`>

void

* * *

### removeDevice() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#removedevice "Direct link to removeDevice()")

> **removeDevice**(`deviceId`): `Promise`<`void`>

Defined in: [Neurosity.ts:436](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L436)

Remove a device from the user's account

```typescript
await neurosity.removeDevice("[deviceId]");
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-15 "Direct link to Parameters")

##### deviceId [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#deviceid-1 "Direct link to deviceId")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-38 "Direct link to Returns")

`Promise`<`void`>

* * *

### removeOAuthAccess() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#removeoauthaccess "Direct link to removeOAuthAccess()")

> **removeOAuthAccess**(): `Promise`< [`OAuthRemoveResponse`](https://docs.neurosity.co/docs/reference/type-aliases/OAuthRemoveResponse) >

Defined in: [Neurosity.ts:1559](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1559)

Remove OAuth Access
💡 OAuth requires developers to register their apps with Neurosity
[Read full OAuth guide](https://docs.neurosity.co/docs/api/oauth)

Removes client-specific OAuth token for a given userId. Requires SDK to be signed in with OAuth custom token.

```typescript
await neurosity.removeOAuthAccess().catch((error) => {
  // handle error here...
});
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-39 "Direct link to Returns")

`Promise`< [`OAuthRemoveResponse`](https://docs.neurosity.co/docs/reference/type-aliases/OAuthRemoveResponse) >

custom token

* * *

### selectDevice() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#selectdevice "Direct link to selectDevice()")

> **selectDevice**(`deviceSelector`): `Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

Defined in: [Neurosity.ts:549](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L549)

Select Device

Rarely necessary, but useful when the user owns multiple devices.

A common use case for manually selecting a device is when you wish to build a device dropdown a user can select from, instead of collecting the Device Id from the user ahead of time.

The 3 steps to manually selecting a device are:

- Set `autoSelectDevice` to false when instantiating the `Neurosity` class.
- Authenticate with your Neurosity account to access your devices by calling the `neurosity.login(...)` function.
- Call the `neurosity.selectDevice(...)` function with a device selector function.

```typescript
const devices = await neurosity.selectDevice((devices) =>
  devices.find((device) => device.deviceNickname === "Crown-A1B")
);

console.log(devices);
```

> If you own multiple devices, and don't pass `autoSelectDevice`, then the first device on the list will be automatically selected.

For more info, check out the "Device Selection" guide.

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-16 "Direct link to Parameters")

##### deviceSelector [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#deviceselector "Direct link to deviceSelector")

(`devices`) =\> [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-40 "Direct link to Returns")

`Promise`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

* * *

### settings() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#settings-1 "Direct link to settings()")

> **settings**(): `Observable`< [`Settings`](https://docs.neurosity.co/docs/reference/interfaces/Settings) >

Defined in: [Neurosity.ts:1026](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1026)

Streaming modes:Wi-Fi

Observes last state of `settings` and all subsequent `settings` changes

```typescript
neurosity.settings().subscribe(settings => {
  console.log(settings.lsl);
});

// true
// ...
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-41 "Direct link to Returns")

`Observable`< [`Settings`](https://docs.neurosity.co/docs/reference/interfaces/Settings) >

Observable of `settings` metric events

* * *

### signalQuality() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#signalquality "Direct link to signalQuality()")

> **signalQuality**(): `Observable`< [`SignalQuality`](https://docs.neurosity.co/docs/reference/interfaces/SignalQuality) >

Defined in: [Neurosity.ts:986](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L986)

Streaming modes:Wi-FiBluetooth

Observes signal quality data where each property is the name
of the channel and the value includes the standard deviation and
a status set by the device

```typescript
neurosity.signalQuality().subscribe(signalQuality => {
  console.log(signalQuality);
});

// { FC6: { standardDeviation: 3.5, status: "good" }, C3: {...}, ... }
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-42 "Direct link to Returns")

`Observable`< [`SignalQuality`](https://docs.neurosity.co/docs/reference/interfaces/SignalQuality) >

Observable of signalQuality metric events

* * *

### status() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#status "Direct link to status()")

> **status**(): `Observable`< [`DeviceStatus`](https://docs.neurosity.co/docs/reference/interfaces/DeviceStatus) >

Defined in: [Neurosity.ts:1177](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L1177)

Streaming modes:Wi-FiBluetooth

Observes last state of `status` and all subsequent `status` changes

```typescript
neurosity.status().subscribe(status => {
  console.log(status.state);
});

// "online"
// ...
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-43 "Direct link to Returns")

`Observable`< [`DeviceStatus`](https://docs.neurosity.co/docs/reference/interfaces/DeviceStatus) >

Observable of `status` metric events

* * *

### streamingState() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#streamingstate "Direct link to streamingState()")

> **streamingState**(): `Observable`<{ `activeMode`: `STREAMING_TYPE`; `connected`: `boolean`; `streamingMode`: [`STREAMING_MODE`](https://docs.neurosity.co/docs/reference/enumerations/STREAMING_MODE); }>

Defined in: [Neurosity.ts:197](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L197)

Subscribe to the device's streaming state changes and the current strategy

Streams the current mode of streaming (wifi or bluetooth).

```typescript
neurosity.streamingState().subscribe((streamingState) => {
  console.log(streamingState);
  // { streamingMode: "wifi-only", activeMode: "wifi", connected: true }
});
```

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-44 "Direct link to Returns")

`Observable`<{ `activeMode`: `STREAMING_TYPE`; `connected`: `boolean`; `streamingMode`: [`STREAMING_MODE`](https://docs.neurosity.co/docs/reference/enumerations/STREAMING_MODE); }>

* * *

### transferDevice() [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#transferdevice "Direct link to transferDevice()")

> **transferDevice**(`options`): `Promise`<`void`>

Defined in: [Neurosity.ts:460](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/Neurosity.ts#L460)

Transfer a device to the user's account

```typescript
await neurosity.transferDevice({
  deviceId: "[deviceId]",
  newOwnerEmail: "[newOwnerEmail]"
});
```

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#parameters-17 "Direct link to Parameters")

##### options [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#options-1 "Direct link to options")

`TransferDeviceOptions`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/neurosity/\#returns-45 "Direct link to Returns")

`Promise`<`void`>

- [Constructors](https://docs.neurosity.co/docs/reference/classes/neurosity/#constructors)
  - [Constructor](https://docs.neurosity.co/docs/reference/classes/neurosity/#constructor)
- [Accessors](https://docs.neurosity.co/docs/reference/classes/neurosity/#accessors)
  - [training](https://docs.neurosity.co/docs/reference/classes/neurosity/#training)
- [Methods](https://docs.neurosity.co/docs/reference/classes/neurosity/#methods)
  - [\_\_getApp()](https://docs.neurosity.co/docs/reference/classes/neurosity/#__getapp)
  - [accelerometer()](https://docs.neurosity.co/docs/reference/classes/neurosity/#accelerometer)
  - [addDevice()](https://docs.neurosity.co/docs/reference/classes/neurosity/#adddevice)
  - [addMarker()](https://docs.neurosity.co/docs/reference/classes/neurosity/#addmarker)
  - [brainwaves()](https://docs.neurosity.co/docs/reference/classes/neurosity/#brainwaves)
  - [calm()](https://docs.neurosity.co/docs/reference/classes/neurosity/#calm)
  - [changeSettings()](https://docs.neurosity.co/docs/reference/classes/neurosity/#changesettings)
  - [createAccount()](https://docs.neurosity.co/docs/reference/classes/neurosity/#createaccount)
  - [createApiKey()](https://docs.neurosity.co/docs/reference/classes/neurosity/#createapikey)
  - [createBluetoothToken()](https://docs.neurosity.co/docs/reference/classes/neurosity/#createbluetoothtoken)
  - [createCustomToken()](https://docs.neurosity.co/docs/reference/classes/neurosity/#createcustomtoken)
  - [createOAuthURL()](https://docs.neurosity.co/docs/reference/classes/neurosity/#createoauthurl)
  - [deleteAccount()](https://docs.neurosity.co/docs/reference/classes/neurosity/#deleteaccount)
  - [deleteUserExperiment()](https://docs.neurosity.co/docs/reference/classes/neurosity/#deleteuserexperiment)
  - [disconnect()](https://docs.neurosity.co/docs/reference/classes/neurosity/#disconnect)
  - [focus()](https://docs.neurosity.co/docs/reference/classes/neurosity/#focus)
  - [getDevices()](https://docs.neurosity.co/docs/reference/classes/neurosity/#getdevices)
  - [getHapticEffects()](https://docs.neurosity.co/docs/reference/classes/neurosity/#gethapticeffects)
  - [getInfo()](https://docs.neurosity.co/docs/reference/classes/neurosity/#getinfo)
  - [getOAuthToken()](https://docs.neurosity.co/docs/reference/classes/neurosity/#getoauthtoken)
  - [getSelectedDevice()](https://docs.neurosity.co/docs/reference/classes/neurosity/#getselecteddevice)
  - [getTimesyncOffset()](https://docs.neurosity.co/docs/reference/classes/neurosity/#gettimesyncoffset)
  - [goOffline()](https://docs.neurosity.co/docs/reference/classes/neurosity/#gooffline)
  - [goOnline()](https://docs.neurosity.co/docs/reference/classes/neurosity/#goonline)
  - [haptics()](https://docs.neurosity.co/docs/reference/classes/neurosity/#haptics)
  - [kinesis()](https://docs.neurosity.co/docs/reference/classes/neurosity/#kinesis)
  - [login()](https://docs.neurosity.co/docs/reference/classes/neurosity/#login)
  - [logout()](https://docs.neurosity.co/docs/reference/classes/neurosity/#logout)
  - [onAuthStateChanged()](https://docs.neurosity.co/docs/reference/classes/neurosity/#onauthstatechanged)
  - [onDeviceChange()](https://docs.neurosity.co/docs/reference/classes/neurosity/#ondevicechange)
  - [onUserClaimsChange()](https://docs.neurosity.co/docs/reference/classes/neurosity/#onuserclaimschange)
  - [onUserDevicesChange()](https://docs.neurosity.co/docs/reference/classes/neurosity/#onuserdeviceschange)
  - [onUserExperiments()](https://docs.neurosity.co/docs/reference/classes/neurosity/#onuserexperiments)
  - [osVersion()](https://docs.neurosity.co/docs/reference/classes/neurosity/#osversion)
  - [predictions()](https://docs.neurosity.co/docs/reference/classes/neurosity/#predictions)
  - [removeApiKey()](https://docs.neurosity.co/docs/reference/classes/neurosity/#removeapikey)
  - [removeDevice()](https://docs.neurosity.co/docs/reference/classes/neurosity/#removedevice)
  - [removeOAuthAccess()](https://docs.neurosity.co/docs/reference/classes/neurosity/#removeoauthaccess)
  - [selectDevice()](https://docs.neurosity.co/docs/reference/classes/neurosity/#selectdevice)
  - [settings()](https://docs.neurosity.co/docs/reference/classes/neurosity/#settings-1)
  - [signalQuality()](https://docs.neurosity.co/docs/reference/classes/neurosity/#signalquality)
  - [status()](https://docs.neurosity.co/docs/reference/classes/neurosity/#status)
  - [streamingState()](https://docs.neurosity.co/docs/reference/classes/neurosity/#streamingstate)
  - [transferDevice()](https://docs.neurosity.co/docs/reference/classes/neurosity/#transferdevice)