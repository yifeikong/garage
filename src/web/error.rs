use err_derive::Error;
use hyper::header::HeaderValue;
use hyper::{HeaderMap, StatusCode};

use garage_util::error::Error as GarageError;

/// Errors of this crate
#[derive(Debug, Error)]
pub enum Error {
	/// An error received from the API crate
	#[error(display = "API error: {}", _0)]
	ApiError(#[error(source)] garage_api::Error),

	// Category: internal error
	/// Error internal to garage
	#[error(display = "Internal error: {}", _0)]
	InternalError(#[error(source)] GarageError),

	/// The file does not exist
	#[error(display = "Not found")]
	NotFound,

	/// The request contained an invalid UTF-8 sequence in its path or in other parameters
	#[error(display = "Invalid UTF-8: {}", _0)]
	InvalidUtf8(#[error(source)] std::str::Utf8Error),

	/// The client send a header with invalid value
	#[error(display = "Invalid header value: {}", _0)]
	InvalidHeader(#[error(source)] hyper::header::ToStrError),

	/// The client sent a request without host, or with unsupported method
	#[error(display = "Bad request: {}", _0)]
	BadRequest(String),
}

impl Error {
	/// Transform errors into http status code
	pub fn http_status_code(&self) -> StatusCode {
		match self {
			Error::NotFound => StatusCode::NOT_FOUND,
			Error::ApiError(e) => e.http_status_code(),
			Error::InternalError(
				GarageError::Timeout
				| GarageError::RemoteError(_)
				| GarageError::Quorum(_, _, _, _),
			) => StatusCode::SERVICE_UNAVAILABLE,
			Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
			_ => StatusCode::BAD_REQUEST,
		}
	}

	pub fn add_headers(&self, header_map: &mut HeaderMap<HeaderValue>) {
		#[allow(clippy::single_match)]
		match self {
			Error::ApiError(e) => e.add_headers(header_map),
			_ => (),
		}
	}
}
