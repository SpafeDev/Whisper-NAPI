# whisper-napi

## Usage Example

```js
const { MutterModel } = require("./whisper-napi.win32-x64-msvc.node");

const model = new MutterModel("./ggml-tiny.bin");
const audioPath = "./example.wav";

(async () => {
  try {
    // Transcribe audio from WAV file asynchronously
    const result = await model.transcribe(audioPath);

    console.log("Text:");
    console.log(result.text);

    console.log("\nSRT:");
    console.log(result.srt);
  } catch (err) {
    console.error("Error:", err);
  }
})();
```

## API

### `new MutterModel(modelPath: string)`

Constructs a new MutterModel instance by loading a GGML model from the specified path.

- **modelPath**: Path to the GGML model file (e.g. `"./ggml-tiny.bin"`).

Throws an error if the model fails to load.

---

### `transcribe(wavPath: string): Promise<Transcription>`

Asynchronously transcribes the audio from a WAV file.

- **wavPath**: Path to the WAV audio file to transcribe.

Returns a Promise resolving to a `Transcription` object containing:

- `text` — the transcription as plain text.
- `srt` — the transcription formatted as subtitles (SRT).

Throws an error if reading the WAV file or transcription fails.

---

## Transcription Object

```ts
interface Transcription {
  text: string; // plain transcription text
  srt: string; // transcription in SRT subtitle format
}
```
