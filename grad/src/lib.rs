pub mod time {
    #[doc(inline)]
    pub use grad_time::utc as utc;
    #[doc(inline)]
    pub use grad_time::local as local;
}

pub mod random {
    #[doc(inline)]
    pub use grad_rand::byte::{random_byte, random_hex, random_base64};
    #[doc(inline)]
    pub use grad_rand::number::random_integer;
    #[doc(inline)]
    pub use grad_rand::digit::random_digit;
    #[doc(inline)]
    pub use grad_rand::password::random_password;
    #[doc(inline)]
    pub use grad_rand::passphrase::{WordList, random_passphrase};
}

pub mod cli {
    #[doc(inline)]
    pub use grad_cli::command as command;
    #[doc(inline)]
    pub use grad_cli::verbosity as verbosity;
    #[doc(inline)]
    pub use grad_cli::parse;
}
