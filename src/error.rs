use csv::Error as CsvError;
use komodo_rpc::Error as RpcApiError;
use std::error::Error;

use derive_more::Display;

#[derive(Debug, Display)]
#[display(fmt = "{}", kind)]
pub struct SnapshotError {
    pub kind: ErrorKind,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

#[derive(Debug, Display)]
pub enum ErrorKind {
    #[display(fmt = "The snapshot returned no addresses.")]
    EmptySnapshot,
    #[display(fmt = "Something went wrong during the Komodod RPC.")]
    ApiError(RpcApiError),
    #[display(fmt = "Something went wrong writing to CSV.")]
    CsvError(CsvError),
}

impl Error for SnapshotError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|boxed| boxed.as_ref() as &(dyn Error + 'static))
    }
}

impl From<ErrorKind> for SnapshotError {
    fn from(kind: ErrorKind) -> Self {
        SnapshotError { kind, source: None }
    }
}

impl From<RpcApiError> for SnapshotError {
    fn from(e: RpcApiError) -> Self {
        ErrorKind::ApiError(e).into()
    }
}

impl From<CsvError> for SnapshotError {
    fn from(e: CsvError) -> Self {
        ErrorKind::CsvError(e).into()
    }
}
