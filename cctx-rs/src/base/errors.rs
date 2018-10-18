//!
//! CCXT errors binding, we want to keep same e
//! 

use failure::Fail;
use hyper::Error as HyperError;
use serde_json::Error as SerdeError;

impl Into<CCXTError> for i32 {
    fn into(self) -> CCXTError {
        match self {
            422 =>  CCXTError::ExchangeError,
            418 =>  CCXTError::DDoSProtection,
            429 =>  CCXTError::DDoSProtection,
            404 =>  CCXTError::ExchangeNotAvailable,
            409 =>  CCXTError::ExchangeNotAvailable,
            500 =>  CCXTError::ExchangeNotAvailable,
            501 =>  CCXTError::ExchangeNotAvailable,
            502 =>  CCXTError::ExchangeNotAvailable,
            520 =>  CCXTError::ExchangeNotAvailable,
            521 =>  CCXTError::ExchangeNotAvailable,
            522 =>  CCXTError::ExchangeNotAvailable,
            525 =>  CCXTError::ExchangeNotAvailable,
            400 =>  CCXTError::ExchangeNotAvailable,
            403 =>  CCXTError::ExchangeNotAvailable,
            405 =>  CCXTError::ExchangeNotAvailable,
            503 =>  CCXTError::ExchangeNotAvailable,
            530 =>  CCXTError::ExchangeNotAvailable,
            408 =>  CCXTError::RequestTimeout,
            504 =>  CCXTError::RequestTimeout,
            401 =>  CCXTError::AuthenticationError,
            511 =>  CCXTError::AuthenticationError,
            _ => CCXTError::BadResponse,
        }
    }
}

#[derive(Debug, Fail)]
pub enum CCXTLoadingError {
    #[fail(display = "Id is undefined ! {}", field)]
    UndefinedField {
        field: String,
    }
}

#[derive(Debug, Fail)]
pub enum CCXTError {    
    #[fail(display = "Undefined error")]
    Undefined,

    #[fail(display = "Raised when te requested uri is malformated")]
    ApiUrlMalformated,

    #[fail(display = "Raised when the requested api can't be found in api_urls")]
    ApiUrlNotFound,

    #[fail(display = "Raised when the requested api method is undefined")]
    ApiMethodNotFound,

    #[fail(display = "Raised when connector is not loaded")]
    ConnectorNotLoaded,
    
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

impl From<HyperError> for CCXTError {
    fn from(err: HyperError) -> CCXTError {
        CCXTError::BadResponse//Maybe wrong
    }
}

impl From<SerdeError> for CCXTError {
    fn from(err: SerdeError) -> CCXTError {
        CCXTError::BadResponse
    }
}