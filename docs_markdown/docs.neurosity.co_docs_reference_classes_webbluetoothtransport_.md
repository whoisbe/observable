# Source: https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/

[Skip to main content](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#__docusaurus_skipToContent_fallback)

On this page

[**@neurosity/sdk**](https://docs.neurosity.co/docs/reference/)

* * *

[@neurosity/sdk](https://docs.neurosity.co/docs/reference/) / WebBluetoothTransport

# Class: WebBluetoothTransport

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:30](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L30)

## Implements [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implements "Direct link to Implements")

- `BluetoothTransport`

## Constructors [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#constructors "Direct link to Constructors")

### Constructor [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#constructor "Direct link to Constructor")

> **new WebBluetoothTransport**(`options`): `WebBluetoothTransport`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:57](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L57)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters "Direct link to Parameters")

##### options [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#options "Direct link to options")

`Options` = `{}`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns "Direct link to Returns")

`WebBluetoothTransport`

## Properties [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#properties "Direct link to Properties")

### \_isAutoConnectEnabled$ [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#_isautoconnectenabled "Direct link to _isAutoConnectEnabled$")

> **\_isAutoConnectEnabled$**: `ReplaySubject`<`boolean`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:55](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L55)

* * *

### characteristicsByName [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#characteristicsbyname "Direct link to characteristicsByName")

> **characteristicsByName**: `object` = `{}`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:37](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L37)

#### Index Signature [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#index-signature "Direct link to Index Signature")

\[`name`: `string`\]: `BluetoothRemoteGATTCharacteristic`

* * *

### connection$ [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#connection "Direct link to connection$")

> **connection$**: `BehaviorSubject`<`BLUETOOTH_CONNECTION`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:41](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L41)

* * *

### connectionStream$ [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#connectionstream "Direct link to connectionStream$")

> **connectionStream$**: `Observable`<`BLUETOOTH_CONNECTION`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:47](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L47)

* * *

### device [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#device "Direct link to device")

> **device**: `BluetoothDevice`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:34](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L34)

* * *

### logs$ [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#logs "Direct link to logs$")

> **logs$**: `ReplaySubject`<`string`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:45](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L45)

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of "Direct link to Implementation of")

`BluetoothTransport.logs$`

* * *

### onDisconnected$ [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#ondisconnected "Direct link to onDisconnected$")

> **onDisconnected$**: `Observable`<`void`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:46](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L46)

* * *

### options [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#options-1 "Direct link to options")

> **options**: `Options`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:33](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L33)

* * *

### pendingActions$ [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#pendingactions "Direct link to pendingActions$")

> **pendingActions$**: `BehaviorSubject`<`any`\[\]>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:44](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L44)

* * *

### server [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#server "Direct link to server")

> **server**: `BluetoothRemoteGATTServer`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:35](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L35)

* * *

### service [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#service "Direct link to service")

> **service**: `BluetoothRemoteGATTService`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:36](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L36)

* * *

### textCodec [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#textcodec "Direct link to textCodec")

> **textCodec**: `TextCodec`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:32](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L32)

* * *

### type [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#type "Direct link to type")

> **type**: `TRANSPORT_TYPE` = `TRANSPORT_TYPE.WEB`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:31](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L31)

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-1 "Direct link to Implementation of")

`BluetoothTransport.type`

## Methods [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#methods "Direct link to Methods")

### \_addPendingAction() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#_addpendingaction "Direct link to _addPendingAction()")

> **\_addPendingAction**(`actionId`): `void`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:410](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L410)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-1 "Direct link to Parameters")

##### actionId [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#actionid "Direct link to actionId")

`number`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-1 "Direct link to Returns")

`void`

* * *

### \_autoConnect() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#_autoconnect "Direct link to _autoConnect()")

> **\_autoConnect**(`selectedDevice$`): `Observable`<`void`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:85](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L85)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-2 "Direct link to Parameters")

##### selectedDevice$ [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#selecteddevice "Direct link to selectedDevice$")

`Observable`< [`DeviceInfo`](https://docs.neurosity.co/docs/reference/interfaces/DeviceInfo) >

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-2 "Direct link to Returns")

`Observable`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-2 "Direct link to Implementation of")

`BluetoothTransport._autoConnect`

* * *

### \_autoToggleActionNotifications() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#_autotoggleactionnotifications "Direct link to _autoToggleActionNotifications()")

> **\_autoToggleActionNotifications**(): `Observable`<`any`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:422](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L422)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-3 "Direct link to Returns")

`Observable`<`any`>

* * *

### \_getPairedDevices() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#_getpaireddevices "Direct link to _getPairedDevices()")

> **\_getPairedDevices**(): `Promise`<`BluetoothDevice`\[\]>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:81](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L81)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-4 "Direct link to Returns")

`Promise`<`BluetoothDevice`\[\]>

* * *

### \_onDisconnected() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#_ondisconnected "Direct link to _onDisconnected()")

> **\_onDisconnected**(): `Observable`<`any`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:254](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L254)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-5 "Direct link to Returns")

`Observable`<`any`>

* * *

### \_removePendingAction() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#_removependingaction "Direct link to _removePendingAction()")

> **\_removePendingAction**(`actionId`): `void`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:415](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L415)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-3 "Direct link to Parameters")

##### actionId [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#actionid-1 "Direct link to actionId")

`number`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-6 "Direct link to Returns")

`void`

* * *

### addLog() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#addlog "Direct link to addLog()")

> **addLog**(`log`): `void`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:156](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L156)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-4 "Direct link to Parameters")

##### log [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#log "Direct link to log")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-7 "Direct link to Returns")

`void`

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-3 "Direct link to Implementation of")

`BluetoothTransport.addLog`

* * *

### connect() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#connect "Direct link to connect()")

> **connect**(`deviceNickname?`): `Promise`<`void`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:169](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L169)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-5 "Direct link to Parameters")

##### deviceNickname? [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#devicenickname "Direct link to deviceNickname?")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-8 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-4 "Direct link to Implementation of")

`BluetoothTransport.connect`

* * *

### connection() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#connection-1 "Direct link to connection()")

> **connection**(): `Observable`<`BLUETOOTH_CONNECTION`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:165](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L165)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-9 "Direct link to Returns")

`Observable`<`BLUETOOTH_CONNECTION`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-5 "Direct link to Implementation of")

`BluetoothTransport.connection`

* * *

### disconnect() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#disconnect "Direct link to disconnect()")

> **disconnect**(): `Promise`<`void`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:266](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L266)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-10 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-6 "Direct link to Implementation of")

`BluetoothTransport.disconnect`

* * *

### dispatchAction() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#dispatchaction "Direct link to dispatchAction()")

> **dispatchAction**(`__namedParameters`): `Promise`<`any`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:471](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L471)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-6 "Direct link to Parameters")

##### \_\_namedParameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#__namedparameters "Direct link to __namedParameters")

`ActionOptions`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-11 "Direct link to Returns")

`Promise`<`any`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-7 "Direct link to Implementation of")

`BluetoothTransport.dispatchAction`

* * *

### enableAutoConnect() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#enableautoconnect "Direct link to enableAutoConnect()")

> **enableAutoConnect**(`autoConnect`): `void`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:152](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L152)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-7 "Direct link to Parameters")

##### autoConnect [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#autoconnect "Direct link to autoConnect")

`boolean`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-12 "Direct link to Returns")

`void`

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-8 "Direct link to Implementation of")

`BluetoothTransport.enableAutoConnect`

* * *

### getCharacteristicByName() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#getcharacteristicbyname "Direct link to getCharacteristicByName()")

> **getCharacteristicByName**(`characteristicName`): `Promise`<`BluetoothRemoteGATTCharacteristic`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:280](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L280)

Bluetooth GATT attributes, services, characteristics, etc. are invalidated
when a device disconnects. This means your code should always retrieve
(through getPrimaryService(s), getCharacteristic(s), etc.) these attributes
after reconnecting.

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-8 "Direct link to Parameters")

##### characteristicName [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#characteristicname "Direct link to characteristicName")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-13 "Direct link to Returns")

`Promise`<`BluetoothRemoteGATTCharacteristic`>

* * *

### getServerServiceAndCharacteristics() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#getserverserviceandcharacteristics "Direct link to getServerServiceAndCharacteristics()")

> **getServerServiceAndCharacteristics**(`device`): `Promise`<`never`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:217](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L217)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-9 "Direct link to Parameters")

##### device [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#device-1 "Direct link to device")

`BluetoothDevice`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-14 "Direct link to Returns")

`Promise`<`never`>

* * *

### isConnected() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#isconnected "Direct link to isConnected()")

> **isConnected**(): `boolean`

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:160](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L160)

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-15 "Direct link to Returns")

`boolean`

* * *

### readCharacteristic() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#readcharacteristic "Direct link to readCharacteristic()")

> **readCharacteristic**(`characteristicName`, `parse`): `Promise`<`any`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:355](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L355)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-10 "Direct link to Parameters")

##### characteristicName [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#characteristicname-1 "Direct link to characteristicName")

`string`

##### parse [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parse "Direct link to parse")

`boolean` = `false`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-16 "Direct link to Returns")

`Promise`<`any`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-9 "Direct link to Implementation of")

`BluetoothTransport.readCharacteristic`

* * *

### requestDevice() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#requestdevice "Direct link to requestDevice()")

> **requestDevice**(`deviceNickname?`): `Promise`<`BluetoothDevice`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:180](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L180)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-11 "Direct link to Parameters")

##### deviceNickname? [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#devicenickname-1 "Direct link to deviceNickname?")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-17 "Direct link to Returns")

`Promise`<`BluetoothDevice`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-10 "Direct link to Implementation of")

`BluetoothTransport.requestDevice`

* * *

### subscribeToCharacteristic() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#subscribetocharacteristic "Direct link to subscribeToCharacteristic()")

> **subscribeToCharacteristic**(`__namedParameters`): `Observable`<`any`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:286](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L286)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-12 "Direct link to Parameters")

##### \_\_namedParameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#__namedparameters-1 "Direct link to __namedParameters")

`SubscribeOptions`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-18 "Direct link to Returns")

`Observable`<`any`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-11 "Direct link to Implementation of")

`BluetoothTransport.subscribeToCharacteristic`

* * *

### writeCharacteristic() [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#writecharacteristic "Direct link to writeCharacteristic()")

> **writeCharacteristic**(`characteristicName`, `data`): `Promise`<`void`>

Defined in: [api/bluetooth/web/WebBluetoothTransport.ts:388](https://github.com/neurosity/neurosity-sdk-js/blob/ca7e14486daa10044c200522f9909b547069a19a/src/api/bluetooth/web/WebBluetoothTransport.ts#L388)

#### Parameters [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#parameters-13 "Direct link to Parameters")

##### characteristicName [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#characteristicname-2 "Direct link to characteristicName")

`string`

##### data [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#data "Direct link to data")

`string`

#### Returns [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#returns-19 "Direct link to Returns")

`Promise`<`void`>

#### Implementation of [​](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/\#implementation-of-12 "Direct link to Implementation of")

`BluetoothTransport.writeCharacteristic`

- [Implements](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#implements)
- [Constructors](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#constructors)
  - [Constructor](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#constructor)
- [Properties](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#properties)
  - [\_isAutoConnectEnabled$](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#_isautoconnectenabled)
  - [characteristicsByName](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#characteristicsbyname)
  - [connection$](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#connection)
  - [connectionStream$](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#connectionstream)
  - [device](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#device)
  - [logs$](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#logs)
  - [onDisconnected$](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#ondisconnected)
  - [options](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#options-1)
  - [pendingActions$](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#pendingactions)
  - [server](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#server)
  - [service](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#service)
  - [textCodec](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#textcodec)
  - [type](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#type)
- [Methods](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#methods)
  - [\_addPendingAction()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#_addpendingaction)
  - [\_autoConnect()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#_autoconnect)
  - [\_autoToggleActionNotifications()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#_autotoggleactionnotifications)
  - [\_getPairedDevices()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#_getpaireddevices)
  - [\_onDisconnected()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#_ondisconnected)
  - [\_removePendingAction()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#_removependingaction)
  - [addLog()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#addlog)
  - [connect()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#connect)
  - [connection()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#connection-1)
  - [disconnect()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#disconnect)
  - [dispatchAction()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#dispatchaction)
  - [enableAutoConnect()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#enableautoconnect)
  - [getCharacteristicByName()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#getcharacteristicbyname)
  - [getServerServiceAndCharacteristics()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#getserverserviceandcharacteristics)
  - [isConnected()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#isconnected)
  - [readCharacteristic()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#readcharacteristic)
  - [requestDevice()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#requestdevice)
  - [subscribeToCharacteristic()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#subscribetocharacteristic)
  - [writeCharacteristic()](https://docs.neurosity.co/docs/reference/classes/webbluetoothtransport/#writecharacteristic)