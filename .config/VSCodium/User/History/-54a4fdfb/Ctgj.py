from TTS.api import TTS

def speak(text, file_path="output.wav"):
    tts = TTS(model_name="tts_models/en/ljspeech/tacotron2-DDC", progress_bar=False)
    tts.tts_to_file(text=text, file_path=file_path)