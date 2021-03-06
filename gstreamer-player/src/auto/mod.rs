// This file was generated by gir (0fe730d) from gir-files (???)
// DO NOT EDIT

mod player;
pub use self::player::Player;

mod player_audio_info;
pub use self::player_audio_info::PlayerAudioInfo;

mod player_g_main_context_signal_dispatcher;
pub use self::player_g_main_context_signal_dispatcher::PlayerGMainContextSignalDispatcher;

mod player_media_info;
pub use self::player_media_info::PlayerMediaInfo;

mod player_signal_dispatcher;
pub use self::player_signal_dispatcher::PlayerSignalDispatcher;
pub use self::player_signal_dispatcher::PlayerSignalDispatcherExt;

mod player_stream_info;
pub use self::player_stream_info::PlayerStreamInfo;
pub use self::player_stream_info::PlayerStreamInfoExt;

mod player_subtitle_info;
pub use self::player_subtitle_info::PlayerSubtitleInfo;

mod player_video_info;
pub use self::player_video_info::PlayerVideoInfo;

mod player_video_overlay_video_renderer;
pub use self::player_video_overlay_video_renderer::PlayerVideoOverlayVideoRenderer;

mod player_video_renderer;
pub use self::player_video_renderer::PlayerVideoRenderer;
pub use self::player_video_renderer::PlayerVideoRendererExt;

mod player_visualization;
pub use self::player_visualization::PlayerVisualization;

mod enums;
pub use self::enums::PlayerColorBalanceType;
pub use self::enums::PlayerError;
pub use self::enums::PlayerSnapshotFormat;
pub use self::enums::PlayerState;

#[doc(hidden)]
pub mod traits {
    pub use super::PlayerSignalDispatcherExt;
    pub use super::PlayerStreamInfoExt;
    pub use super::PlayerVideoRendererExt;
}
