# Example usage

```js
const { MutterModel } = require("./whisper-napi.win32-x64-msvc.node");

const model = new MutterModel("./ggml-tiny.bin");

const audioPath = "./example.wav";

(async () => {
  try {
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
