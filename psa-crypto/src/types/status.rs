// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! # PSA Status Codes
//!
//! This module defines success and error codes returned by any PSA function.

use log::error;

/// Result type returned by any PSA operation
pub type Result<T> = core::result::Result<T, Error>;

/// Definition of a PSA status code
#[derive(Clone, Copy, Debug)]
pub enum Status {
    /// Status code for success
    Success,
    /// Status codes for errors
    Error(Error),
}

impl Status {
    /// Convert the Status into a Result returning the empty tuple
    pub fn to_result(self) -> Result<()> {
        match self {
            Status::Success => Ok(()),
            Status::Error(error) => Err(error),
        }
    }
}

/// Definition of a PSA status code
#[derive(Clone, Copy, Debug)]
pub enum Error {
    /// An error occurred that does not correspond to any defined failure cause
    GenericError,
    /// The requested operation or a parameter is not supported by this implementation
    NotSupported,
    /// The requested action is denied by a policy
    NotPermitted,
    /// An output buffer is too small
    BufferTooSmall,
    /// Asking for an item that already exists
    AlreadyExists,
    /// Asking for an item that doesn't exist
    DoesNotExist,
    /// The requested action cannot be performed in the current state
    BadState,
    /// The parameters passed to the function are invalid
    InvalidArgument,
    /// There is not enough runtime memory
    InsufficientMemory,
    /// There is not enough persistent storage
    InsufficientStorage,
    /// There was a communication failure inside the implementation
    CommunicationFailure,
    /// There was a storage failure that may have led to data loss
    StorageFailure,
    /// Stored data has been corrupted
    DataCorrupt,
    /// Data read from storage is not valid for the implementation
    DataInvalid,
    /// A hardware failure was detected
    HardwareFailure,
    /// A tampering attempt was detected
    CorruptionDetected,
    /// There is not enough entropy to generate random data needed for the requested action
    InsufficientEntropy,
    /// The signature, MAC or hash is incorrect
    InvalidSignature,
    /// The decrypted padding is incorrect
    InvalidPadding,
    /// Insufficient data when attempting to read from a resource
    InsufficientData,
    /// The key handle is not valid
    InvalidHandle,
}

impl From<Error> for Status {
    fn from(error: Error) -> Self {
        Status::Error(error)
    }
}

impl From<psa_crypto_sys::psa_status_t> for Status {
    fn from(status: psa_crypto_sys::psa_status_t) -> Self {
        match status {
            psa_crypto_sys::PSA_SUCCESS => Status::Success,
            psa_crypto_sys::PSA_ERROR_GENERIC_ERROR => Error::GenericError.into(),
            psa_crypto_sys::PSA_ERROR_NOT_SUPPORTED => Error::NotSupported.into(),
            psa_crypto_sys::PSA_ERROR_NOT_PERMITTED => Error::NotPermitted.into(),
            psa_crypto_sys::PSA_ERROR_BUFFER_TOO_SMALL => Error::BufferTooSmall.into(),
            psa_crypto_sys::PSA_ERROR_ALREADY_EXISTS => Error::AlreadyExists.into(),
            psa_crypto_sys::PSA_ERROR_DOES_NOT_EXIST => Error::DoesNotExist.into(),
            psa_crypto_sys::PSA_ERROR_BAD_STATE => Error::BadState.into(),
            psa_crypto_sys::PSA_ERROR_INVALID_ARGUMENT => Error::InvalidArgument.into(),
            psa_crypto_sys::PSA_ERROR_INSUFFICIENT_MEMORY => Error::InsufficientMemory.into(),
            psa_crypto_sys::PSA_ERROR_INSUFFICIENT_STORAGE => Error::InsufficientStorage.into(),
            psa_crypto_sys::PSA_ERROR_COMMUNICATION_FAILURE => Error::CommunicationFailure.into(),
            psa_crypto_sys::PSA_ERROR_STORAGE_FAILURE => Error::StorageFailure.into(),
            psa_crypto_sys::PSA_ERROR_HARDWARE_FAILURE => Error::HardwareFailure.into(),
            psa_crypto_sys::PSA_ERROR_INSUFFICIENT_ENTROPY => Error::InsufficientEntropy.into(),
            psa_crypto_sys::PSA_ERROR_INVALID_SIGNATURE => Error::InvalidSignature.into(),
            psa_crypto_sys::PSA_ERROR_INVALID_PADDING => Error::InvalidPadding.into(),
            psa_crypto_sys::PSA_ERROR_INSUFFICIENT_DATA => Error::InsufficientData.into(),
            psa_crypto_sys::PSA_ERROR_INVALID_HANDLE => Error::InvalidHandle.into(),
            s => {
                error!("{} not recognised as a valid PSA status.", s);
                Status::Error(Error::GenericError)
            }
        }
    }
}

impl From<Status> for psa_crypto_sys::psa_status_t {
    fn from(status: Status) -> psa_crypto_sys::psa_status_t {
        match status {
            Status::Success => psa_crypto_sys::PSA_SUCCESS,
            Status::Error(error) => match error {
                Error::GenericError => psa_crypto_sys::PSA_ERROR_GENERIC_ERROR,
                Error::NotSupported => psa_crypto_sys::PSA_ERROR_NOT_SUPPORTED,
                Error::NotPermitted => psa_crypto_sys::PSA_ERROR_NOT_PERMITTED,
                Error::BufferTooSmall => psa_crypto_sys::PSA_ERROR_BUFFER_TOO_SMALL,
                Error::AlreadyExists => psa_crypto_sys::PSA_ERROR_ALREADY_EXISTS,
                Error::DoesNotExist => psa_crypto_sys::PSA_ERROR_DOES_NOT_EXIST,
                Error::BadState => psa_crypto_sys::PSA_ERROR_BAD_STATE,
                Error::InvalidArgument => psa_crypto_sys::PSA_ERROR_INVALID_ARGUMENT,
                Error::InsufficientMemory => psa_crypto_sys::PSA_ERROR_INSUFFICIENT_MEMORY,
                Error::InsufficientStorage => psa_crypto_sys::PSA_ERROR_INSUFFICIENT_STORAGE,
                Error::CommunicationFailure => psa_crypto_sys::PSA_ERROR_COMMUNICATION_FAILURE,
                Error::StorageFailure => psa_crypto_sys::PSA_ERROR_STORAGE_FAILURE,
                //Error::DataCorrupt => psa_crypto_sys::PSA_ERROR_DATA_CORRUPT,
                //Error::DataInvalid => psa_crypto_sys::PSA_ERROR_DATA_INVALID,
                Error::HardwareFailure => psa_crypto_sys::PSA_ERROR_HARDWARE_FAILURE,
                //Error::CorruptionDetected => psa_crypto_sys::PSA_ERROR_CORRUPTION_DETECTED,
                Error::InsufficientEntropy => psa_crypto_sys::PSA_ERROR_INSUFFICIENT_ENTROPY,
                Error::InvalidSignature => psa_crypto_sys::PSA_ERROR_INVALID_SIGNATURE,
                Error::InvalidPadding => psa_crypto_sys::PSA_ERROR_INVALID_PADDING,
                Error::InsufficientData => psa_crypto_sys::PSA_ERROR_INSUFFICIENT_DATA,
                Error::InvalidHandle => psa_crypto_sys::PSA_ERROR_INVALID_HANDLE,
                e => {
                    error!("No equivalent of {:?} to a psa_status_t.", e);
                    psa_crypto_sys::PSA_ERROR_GENERIC_ERROR
                }
            },
        }
    }
}

impl From<Status> for Result<()> {
    fn from(status: Status) -> Self {
        status.to_result()
    }
}
