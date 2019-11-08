#[derive(Debug, Clone)]
pub struct Error {}

// TODO: Better error handling
pub enum LSMError {
    IO(std::io::Error),
    Other,
}
