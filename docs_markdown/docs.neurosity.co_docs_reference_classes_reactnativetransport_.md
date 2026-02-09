# Source: https://docs.neurosity.co/docs/reference/classes/reactnativetransport/

[Skip to main content](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#__docusaurus_skipToContent_fallback)

On this page

[**@neurosity/sdk**](https://docs.neurosity.co/docs/reference/)

* * *

[@neurosity/sdk](https://docs.neurosity.co/docs/reference/) / ReactNativeTransport

# Class: ReactNativeTransport

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:56](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L56)

## Implements [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implements "Direct link to Implements")

- `BluetoothTransport`

## Constructors [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#constructors "Direct link to Constructors")

### Constructor [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#constructor "Direct link to Constructor")

> **new ReactNativeTransport**(`options`): `ReactNativeTransport`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:84](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L84)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters "Direct link to Parameters")

##### options [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#options "Direct link to options")

`Options`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns "Direct link to Returns")

`ReactNativeTransport`

## Properties [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#properties "Direct link to Properties")

### \_isAutoConnectEnabled$ [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#_isautoconnectenabled "Direct link to _isAutoConnectEnabled$")

> **\_isAutoConnectEnabled$**: `ReplaySubject`<`boolean`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:82](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L82)

* * *

### bleEvents [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#bleevents "Direct link to bleEvents")

> **bleEvents**: `BleManagerEvents`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:63](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L63)

* * *

### BleManager [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#blemanager "Direct link to BleManager")

> **BleManager**: `BleManager`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:60](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L60)

* * *

### bleManagerEmitter [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#blemanageremitter "Direct link to bleManagerEmitter")

> **bleManagerEmitter**: `NativeEventEmitter`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:61](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L61)

* * *

### characteristicsByName [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#characteristicsbyname "Direct link to characteristicsByName")

> **characteristicsByName**: `CharacteristicsByName` = `{}`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:66](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L66)

* * *

### connection$ [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#connection "Direct link to connection$")

> **connection$**: `BehaviorSubject`<`BLUETOOTH_CONNECTION`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:68](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L68)

* * *

### connectionStream$ [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#connectionstream "Direct link to connectionStream$")

> **connectionStream$**: `Observable`<`BLUETOOTH_CONNECTION`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:74](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L74)

* * *

### device [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#device "Direct link to device")

> **device**: `Peripheral`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:65](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L65)

* * *

### logs$ [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#logs "Direct link to logs$")

> **logs$**: `ReplaySubject`<`string`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:72](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L72)

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of "Direct link to Implementation of")

`BluetoothTransport.logs$`

* * *

### onDisconnected$ [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#ondisconnected "Direct link to onDisconnected$")

> **onDisconnected$**: `Observable`<`void`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:73](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L73)

* * *

### options [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#options-1 "Direct link to options")

> **options**: `Options`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:59](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L59)

* * *

### pendingActions$ [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#pendingactions "Direct link to pendingActions$")

> **pendingActions$**: `BehaviorSubject`<`any`\[\]>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:71](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L71)

* * *

### platform [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#platform "Direct link to platform")

> **platform**: `PlatformOSType`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:62](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L62)

* * *

### textCodec [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#textcodec "Direct link to textCodec")

> **textCodec**: `TextCodec`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:58](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L58)

* * *

### type [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#type "Direct link to type")

> **type**: `TRANSPORT_TYPE` = `TRANSPORT_TYPE.REACT_NATIVE`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:57](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L57)

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-1 "Direct link to Implementation of")

`BluetoothTransport.type`

## Methods [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#methods "Direct link to Methods")

### \_addPendingAction() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#_addpendingaction "Direct link to _addPendingAction()")

> **\_addPendingAction**(`actionId`): `void`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:558](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L558)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-1 "Direct link to Parameters")

##### actionId [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#actionid "Direct link to actionId")

`number`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-1 "Direct link to Returns")

`void`

* * *

### \_autoConnect() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#_autoconnect "Direct link to _autoConnect()")

> **\_autoConnect**(`selectedDevice$`): `Observable`<`void`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:167](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L167)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-2 "Direct link to Parameters")

##### selectedDevice$ [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#selecteddevice "Direct link to selectedDevice$")

`Observable`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-2 "Direct link to Returns")

`Observable`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-2 "Direct link to Implementation of")

`BluetoothTransport._autoConnect`

* * *

### \_autoToggleActionNotifications() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#_autotoggleactionnotifications "Direct link to _autoToggleActionNotifications()")

> **\_autoToggleActionNotifications**(): `Observable`<`any`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:570](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L570)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-3 "Direct link to Returns")

`Observable`<`any`>

* * *

### \_fromEvent() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#_fromevent "Direct link to _fromEvent()")

> **\_fromEvent**(`eventName`): `Observable`<`any`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:205](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L205)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-3 "Direct link to Parameters")

##### eventName [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#eventname "Direct link to eventName")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-4 "Direct link to Returns")

`Observable`<`any`>

* * *

### \_removePendingAction() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#_removependingaction "Direct link to _removePendingAction()")

> **\_removePendingAction**(`actionId`): `void`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:563](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L563)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-4 "Direct link to Parameters")

##### actionId [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#actionid-1 "Direct link to actionId")

`number`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-5 "Direct link to Returns")

`void`

* * *

### addLog() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#addlog "Direct link to addLog()")

> **addLog**(`log`): `void`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:158](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L158)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-5 "Direct link to Parameters")

##### log [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#log "Direct link to log")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-6 "Direct link to Returns")

`void`

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-3 "Direct link to Implementation of")

`BluetoothTransport.addLog`

* * *

### connect() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#connect "Direct link to connect()")

> **connect**(`peripheral`): `Promise`<`void`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:319](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L319)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-6 "Direct link to Parameters")

##### peripheral [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#peripheral "Direct link to peripheral")

`Peripheral`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-7 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-4 "Direct link to Implementation of")

`BluetoothTransport.connect`

* * *

### connection() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#connection-1 "Direct link to connection()")

> **connection**(): `Observable`<`BLUETOOTH_CONNECTION`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:201](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L201)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-8 "Direct link to Returns")

`Observable`<`BLUETOOTH_CONNECTION`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-5 "Direct link to Implementation of")

`BluetoothTransport.connection`

* * *

### disconnect() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#disconnect "Direct link to disconnect()")

> **disconnect**(): `Promise`<`void`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:390](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L390)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-9 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-6 "Direct link to Implementation of")

`BluetoothTransport.disconnect`

* * *

### dispatchAction() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#dispatchaction "Direct link to dispatchAction()")

> **dispatchAction**(`__namedParameters`): `Promise`<`any`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:624](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L624)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-7 "Direct link to Parameters")

##### \_\_namedParameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#__namedparameters "Direct link to __namedParameters")

`ActionOptions`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-10 "Direct link to Returns")

`Promise`<`any`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-7 "Direct link to Implementation of")

`BluetoothTransport.dispatchAction`

* * *

### enableAutoConnect() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#enableautoconnect "Direct link to enableAutoConnect()")

> **enableAutoConnect**(`autoConnect`): `void`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:197](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L197)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-8 "Direct link to Parameters")

##### autoConnect [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#autoconnect "Direct link to autoConnect")

`boolean`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-11 "Direct link to Returns")

`void`

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-8 "Direct link to Implementation of")

`BluetoothTransport.enableAutoConnect`

* * *

### getCharacteristicByName() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#getcharacteristicbyname "Direct link to getCharacteristicByName()")

> **getCharacteristicByName**(`characteristicName`): `Characteristic`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:400](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L400)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-9 "Direct link to Parameters")

##### characteristicName [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#characteristicname "Direct link to characteristicName")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-12 "Direct link to Returns")

`Characteristic`

* * *

### isConnected() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#isconnected "Direct link to isConnected()")

> **isConnected**(): `boolean`

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:162](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L162)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-13 "Direct link to Returns")

`boolean`

* * *

### readCharacteristic() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#readcharacteristic "Direct link to readCharacteristic()")

> **readCharacteristic**(`characteristicName`, `parse`): `Promise`<`any`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:491](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L491)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-10 "Direct link to Parameters")

##### characteristicName [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#characteristicname-1 "Direct link to characteristicName")

`string`

##### parse [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parse "Direct link to parse")

`boolean` = `false`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-14 "Direct link to Returns")

`Promise`<`any`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-9 "Direct link to Implementation of")

`BluetoothTransport.readCharacteristic`

* * *

### scan() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#scan "Direct link to scan()")

> **scan**(`options?`): `Observable`<`Peripheral`\[\]>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:220](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L220)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-11 "Direct link to Parameters")

##### options? [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#options-2 "Direct link to options?")

###### once? [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#once "Direct link to once?")

`boolean`

###### seconds? [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#seconds "Direct link to seconds?")

`number`

###### skipConnectionUpdate? [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#skipconnectionupdate "Direct link to skipConnectionUpdate?")

`boolean`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-15 "Direct link to Returns")

`Observable`<`Peripheral`\[\]>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-10 "Direct link to Implementation of")

`BluetoothTransport.scan`

* * *

### subscribeToCharacteristic() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#subscribetocharacteristic "Direct link to subscribeToCharacteristic()")

> **subscribeToCharacteristic**(`__namedParameters`): `Observable`<`any`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:410](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L410)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-12 "Direct link to Parameters")

##### \_\_namedParameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#__namedparameters-1 "Direct link to __namedParameters")

`SubscribeOptions`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-16 "Direct link to Returns")

`Observable`<`any`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-11 "Direct link to Implementation of")

`BluetoothTransport.subscribeToCharacteristic`

* * *

### writeCharacteristic() [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#writecharacteristic "Direct link to writeCharacteristic()")

> **writeCharacteristic**(`characteristicName`, `data`): `Promise`<`void`>

Defined in: [api/bluetooth/react-native/ReactNativeTransport.ts:532](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/react-native/ReactNativeTransport.ts#L532)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#parameters-13 "Direct link to Parameters")

##### characteristicName [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#characteristicname-2 "Direct link to characteristicName")

`string`

##### data [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#data "Direct link to data")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#returns-17 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/\#implementation-of-12 "Direct link to Implementation of")

`BluetoothTransport.writeCharacteristic`

- [Implements](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#implements)
- [Constructors](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#constructors)
  - [Constructor](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#constructor)
- [Properties](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#properties)
  - [\_isAutoConnectEnabled$](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#_isautoconnectenabled)
  - [bleEvents](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#bleevents)
  - [BleManager](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#blemanager)
  - [bleManagerEmitter](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#blemanageremitter)
  - [characteristicsByName](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#characteristicsbyname)
  - [connection$](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#connection)
  - [connectionStream$](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#connectionstream)
  - [device](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#device)
  - [logs$](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#logs)
  - [onDisconnected$](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#ondisconnected)
  - [options](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#options-1)
  - [pendingActions$](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#pendingactions)
  - [platform](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#platform)
  - [textCodec](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#textcodec)
  - [type](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#type)
- [Methods](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#methods)
  - [\_addPendingAction()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#_addpendingaction)
  - [\_autoConnect()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#_autoconnect)
  - [\_autoToggleActionNotifications()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#_autotoggleactionnotifications)
  - [\_fromEvent()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#_fromevent)
  - [\_removePendingAction()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#_removependingaction)
  - [addLog()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#addlog)
  - [connect()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#connect)
  - [connection()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#connection-1)
  - [disconnect()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#disconnect)
  - [dispatchAction()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#dispatchaction)
  - [enableAutoConnect()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#enableautoconnect)
  - [getCharacteristicByName()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#getcharacteristicbyname)
  - [isConnected()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#isconnected)
  - [readCharacteristic()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#readcharacteristic)
  - [scan()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#scan)
  - [subscribeToCharacteristic()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#subscribetocharacteristic)
  - [writeCharacteristic()](https://docs.neurosity.co/docs/reference/classes/reactnativetransport/#writecharacteristic)