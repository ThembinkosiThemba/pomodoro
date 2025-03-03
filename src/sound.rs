use rodio::{OutputStream, Sink, Source};

pub fn play_notification() {
    // Simple notification sound using a basic sine wave
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = rodio::source::SineWave::new(440.0) // A4 note
            .take_duration(std::time::Duration::from_secs_f32(0.25))
            .amplify(0.20);

        sink.append(source);
        sink.sleep_until_end();
    }
}

pub fn play_alarm() {
    // More prominent alarm sound
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Two-tone alarm
        let source1 = rodio::source::SineWave::new(880.0) // Higher pitch
            .take_duration(std::time::Duration::from_secs_f32(0.3))
            .amplify(0.25);

        let source2 = rodio::source::SineWave::new(660.0) // Lower pitch
            .take_duration(std::time::Duration::from_secs_f32(0.3))
            .amplify(0.25);

        sink.append(source1.clone());
        sink.append(source2.clone());
        sink.append(source1);
        sink.append(source2);

        sink.sleep_until_end();
    }
}
