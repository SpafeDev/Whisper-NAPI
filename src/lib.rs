use napi::bindgen_prelude::{Error, Result, spawn_blocking};
use napi_derive::napi;
use mutter::Model;
use std::sync::Arc;
use tokio::fs;

#[napi]
pub struct Transcription {
  pub text: String,
  pub srt:  String,
}

#[napi]
pub struct MutterModel {
  inner: Arc<Model>,
}

#[napi]
impl MutterModel {
  #[napi(constructor)]
  pub fn new(model_path: String) -> Result<Self> {
    let model = Model::new(&model_path)
      .map_err(|e| Error::from_reason(format!("Failed to load model: {:?}", e)))?;
    Ok(MutterModel {
      inner: Arc::new(model),
    })
  }

 #[napi]
pub async fn transcribe(&self, wav_path: String) -> Result<Transcription> {
  let data = fs::read(&wav_path)
    .await
    .map_err(|e| Error::from_reason(format!("Failed to read WAV: {}", e)))?;

  let model = Arc::clone(&self.inner);

  let result = spawn_blocking(move || {
    model
      .transcribe_audio(data, false, false, None)
      .map_err(|e| format!("Transcription failed: {:?}", e))
  })
  .await
  .map_err(|e| Error::from_reason(format!("Join error: {}", e)))?
  .map_err(|s| Error::from_reason(s))?; 

  Ok(Transcription {
    text: result.as_text(),
    srt: result.as_srt(),
  })
}
}