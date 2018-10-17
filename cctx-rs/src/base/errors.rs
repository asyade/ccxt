//!
//! CCXT errors binding, we want to keep same e
//! 

use failure::Fail;

impl Into<CCXTErrors> for i32 {
    fn into(self) -> CCXTErrors {
        match self {
            422 =>  CCXTErrors::ExchangeError,
            418 =>  CCXTErrors::DDoSProtection,
            429 =>  CCXTErrors::DDoSProtection,
            404 =>  CCXTErrors::ExchangeNotAvailable,
            409 =>  CCXTErrors::ExchangeNotAvailable,
            500 =>  CCXTErrors::ExchangeNotAvailable,
            501 =>  CCXTErrors::ExchangeNotAvailable,
            502 =>  CCXTErrors::ExchangeNotAvailable,
            520 =>  CCXTErrors::ExchangeNotAvailable,
            521 =>  CCXTErrors::ExchangeNotAvailable,
            522 =>  CCXTErrors::ExchangeNotAvailable,
            525 =>  CCXTErrors::ExchangeNotAvailable,
            400 =>  CCXTErrors::ExchangeNotAvailable,
            403 =>  CCXTErrors::ExchangeNotAvailable,
            405 =>  CCXTErrors::ExchangeNotAvailable,
            503 =>  CCXTErrors::ExchangeNotAvailable,
            530 =>  CCXTErrors::ExchangeNotAvailable,
            408 =>  CCXTErrors::RequestTimeout,
            504 =>  CCXTErrors::RequestTimeout,
            401 =>  CCXTErrors::AuthenticationError,
            511 =>  CCXTErrors::AuthenticationError,
            _ => CCXTErrors::Undefined,
        }
    }
}

#[derive(Debug, Fail)]
pub enum CCXTErrors {
    #[fail(display = "Undefined error")]
    Undefined,
    
    #[fail(display = "Raised when an exchange server replies with an error in JSON")]
    ExchangeError,

    #[fail(display = "Raised if the endpoint is not offered/not yet supported by the exchange API")]
    NotSupported,

    #[fail(display = "Raised when API credentials are required but missing or wrong")]
    AuthenticationError,

    #[fail(display = "Raised when API credentials are required but missing or wrong")]
    PermissionDenied,

    #[fail(display = "Raised when user account has been suspended or deactivated by the exchange")]
    AccountSuspended,

    #[fail(display = "Raised when you don't have enough currency on your account balance to place an order")]
    InsufficientFunds,

    #[fail(display = "Base class for all exceptions related to the unified order API")]
    InvalidOrder,

    #[fail(display = "Raised when you are trying to fetch or cancel a non-existent order")]
    OrderNotFound,

    #[fail(display = "Raised when the order is not found in local cache (where applicable)")]
    OrderNotCached,

    #[fail(display = "Raised when an order that is already pending cancel is being canceled again")]
    CancelPending,

    #[fail(display = "Base class for all errors related to networking")]
    NetworkError,

    #[fail(display = "Raised whenever DDoS protection restrictions are enforced per user or region/location")]
    DDoSProtection,

    #[fail(display = "Raised when the exchange fails to reply in .timeout time")]
    RequestTimeout,

    #[fail(display = "Raised if a reply from an exchange contains keywords related to maintenance or downtime")]
    ExchangeNotAvailable,

    #[fail(display = "Raised in case of a wrong or conflicting nonce number in private requests")]
    InvalidNonce,

    #[fail(display = "Raised on invalid funding address")]
    InvalidAddress,

    #[fail(display = "Raised when the address requested is pending (not ready yet, retry again later)")]
    AddressPending,

    #[fail(display = "A generic exception raised by unified methods when required arguments are missing.")]
    ArgumentsRequired,

    #[fail(display = "A generic exception raised by the exchange if all or some of required parameters are invalid or missing in URL query or in request body")]
    BadRequest,

    #[fail(display = "Raised if the endpoint returns a bad response from the exchange API")]
    BadResponse,

    #[fail(display = "Raised if the endpoint returns a null response from the exchange API")]
    NullResponse,

    #[fail(display = "Raised when an order placed as a market order or a taker order is not fillable upon request")]
    OrderNotFillable,

    #[fail(display = "Raised when an order placed as maker order is fillable immediately as a taker upon request")]
    OrderImmediatelyFillable   
}