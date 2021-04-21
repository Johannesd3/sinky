mod pipe;
pub use pipe::open as open_pipe;

#[cfg(feature = "rodio-backend")]
mod rodio;
#[cfg(feature = "rodio-backend")]
pub use self::rodio::open as open_rodio;
