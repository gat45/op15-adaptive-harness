use op15_adaptive_harness::voice_session::{VoiceSession, VoiceState};
#[test]
fn barge_in_interrupts_speaking_only() {
    let mut s = VoiceSession::default();
    s.speech_started().unwrap();
    s.transcription_ready().unwrap();
    s.speech_output_started().unwrap();
    assert!(s.interrupt());
    assert_eq!(s.state, VoiceState::Interrupted);
    s.finish();
    assert_eq!(s.state, VoiceState::Idle);
}
