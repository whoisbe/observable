# Source: https://docs.neurosity.co/docs/api/osc/

[Skip to main content](https://docs.neurosity.co/docs/api/osc/#__docusaurus_skipToContent_fallback)

On this page

The Neurosity SDK supports Open Sound Control (OSC) protocol for real-time data streaming. This is particularly useful for integrating with audio/visual software and creative coding frameworks that support OSC.

## Overview [​](https://docs.neurosity.co/docs/api/osc/\#overview "Direct link to Overview")

OSC data is streamed over port _9000_ via UDP in two modes:

- **Perform Mode**: Streams processed metrics like focus, calm, and power by band data
- **Raw Mode**: Streams raw EEG data samples

## Configuration [​](https://docs.neurosity.co/docs/api/osc/\#configuration "Direct link to Configuration")

The OSC server uses the following default configuration:

- Local Host: Configured via `oscLocalHost`
- Local Port: Configured via `oscLocalPort`
- Remote Port: Configured via `oscRemotePort`
- Broadcast: Enabled
- Protocol: UDP

### Enabling OSC Modes [​](https://docs.neurosity.co/docs/api/osc/\#enabling-osc-modes "Direct link to Enabling OSC Modes")

Before using either mode, you'll need to enable them in your device settings:

1. Go to [console.neurosity.co](https://console.neurosity.co/)
2. Select your device
3. Go to Settings
4. Enable either or both:
   - "OSC Raw Mode" - for raw EEG data
   - "OSC Perform Mode" - for processed metrics

Note: Each mode can be enabled independently, allowing you to use either or both modes simultaneously.

## Streaming Modes [​](https://docs.neurosity.co/docs/api/osc/\#streaming-modes "Direct link to Streaming Modes")

### Raw Mode [​](https://docs.neurosity.co/docs/api/osc/\#raw-mode "Direct link to Raw Mode")

When raw mode is enabled via `startRawMode()`, you'll receive raw EEG data samples at the device's sampling rate (256Hz for Crown, 250Hz for Notion 1/2). Each packet contains:

- 16 samples per channel for Crown (62.5ms of data)
- 25 samples per channel for Notion 1/2 (100ms of data)

The data is sent to the OSC address `/neurosity/notion/{deviceId}/raw` with the following arguments:

- channelData (array of floats) - Raw amplitude values for each channel
- timestamp (string) - Timestamp of the first sample in the packet
- count (integer) - Sample count
- marker (string) - Any marker associated with this data

### Perform Mode [​](https://docs.neurosity.co/docs/api/osc/\#perform-mode "Direct link to Perform Mode")

When perform mode is enabled via `startPerformMode()`, you'll receive processed metrics at a lower rate (4Hz). This includes:

- Power by band data (delta, theta, alpha, beta, gamma) - Updated every 250ms
- Focus/Calm probability scores - Updated every 250ms
- Signal quality metrics - Updated every 250ms
- Battery status - Updated when changed

Each metric is sent to its own OSC address (see Address Patterns section below).

The power band data represents averaged frequency power over a 16 second sliding window, providing a more stable signal for creative applications.

## OSC Raw Address Patterns [​](https://docs.neurosity.co/docs/api/osc/\#osc-raw-address-patterns "Direct link to OSC Raw Address Patterns")

### Device Info [​](https://docs.neurosity.co/docs/api/osc/\#device-info "Direct link to Device Info")

`/neurosity/notion/{deviceId}/info`

Arguments:

- deviceId (string)
- deviceNickname (string)
- model (string)
- modelName (string)
- modelVersion (string)
- manufacturer (string)
- samplingRate (integer)
- channels (integer)
- channelNames (string - comma separated)

### Raw EEG Data [​](https://docs.neurosity.co/docs/api/osc/\#raw-eeg-data "Direct link to Raw EEG Data")

`/neurosity/notion/{deviceId}/raw`

Arguments:

- channelData (array of floats) - Amplitude values for each channel
- timestamp (string)
- count (integer)
- marker (string)

### Markers [​](https://docs.neurosity.co/docs/api/osc/\#markers "Direct link to Markers")

`/neurosity/notion/{deviceId}/markers`

Argument:

- label (string)

## OSC Perform Address Patterns [​](https://docs.neurosity.co/docs/api/osc/\#osc-perform-address-patterns "Direct link to OSC Perform Address Patterns")

### Metrics [​](https://docs.neurosity.co/docs/api/osc/\#metrics "Direct link to Metrics")

The following metrics are streamed with the pattern `/crown{deviceShortId}/{metricName}`:

#### Power Bands [​](https://docs.neurosity.co/docs/api/osc/\#power-bands "Direct link to Power Bands")

`/crown{deviceId}/{bandName}`

Where bandName is one of:

- delta
- theta
- alpha
- beta
- gamma

Arguments:

- values (array of floats) - Power values for each channel over a 16 second window

#### Power Band Slopes [​](https://docs.neurosity.co/docs/api/osc/\#power-band-slopes "Direct link to Power Band Slopes")

`/crown{deviceId}/slope/{bandName}`

Arguments:

- values (array of floats) - Slope values for each channel over a 16 second window

#### Battery [​](https://docs.neurosity.co/docs/api/osc/\#battery "Direct link to Battery")

`/crown{deviceId}/battery`

Argument:

- chargePercentage (float)

#### Signal Quality [​](https://docs.neurosity.co/docs/api/osc/\#signal-quality "Direct link to Signal Quality")

`/crown{deviceId}/signalQuality`

Argument:

- quality (float)

## Usage Example [​](https://docs.neurosity.co/docs/api/osc/\#usage-example "Direct link to Usage Example")

```javascript
const osc = require("osc");

// Set up an OSC UDP port listening on port 9000
const udpPort = new osc.UDPPort({
  localAddress: "0.0.0.0",
  localPort: 9000,
});

udpPort.on("message", (oscMessage) => {
  console.log("Received OSC message:", oscMessage);
});

udpPort.open();
```

- [Overview](https://docs.neurosity.co/docs/api/osc/#overview)
- [Configuration](https://docs.neurosity.co/docs/api/osc/#configuration)
  - [Enabling OSC Modes](https://docs.neurosity.co/docs/api/osc/#enabling-osc-modes)
- [Streaming Modes](https://docs.neurosity.co/docs/api/osc/#streaming-modes)
  - [Raw Mode](https://docs.neurosity.co/docs/api/osc/#raw-mode)
  - [Perform Mode](https://docs.neurosity.co/docs/api/osc/#perform-mode)
- [OSC Raw Address Patterns](https://docs.neurosity.co/docs/api/osc/#osc-raw-address-patterns)
  - [Device Info](https://docs.neurosity.co/docs/api/osc/#device-info)
  - [Raw EEG Data](https://docs.neurosity.co/docs/api/osc/#raw-eeg-data)
  - [Markers](https://docs.neurosity.co/docs/api/osc/#markers)
- [OSC Perform Address Patterns](https://docs.neurosity.co/docs/api/osc/#osc-perform-address-patterns)
  - [Metrics](https://docs.neurosity.co/docs/api/osc/#metrics)
- [Usage Example](https://docs.neurosity.co/docs/api/osc/#usage-example)