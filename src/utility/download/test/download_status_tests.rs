use super::*;

#[test]
fn test_is_pending() {
    let status: DownloadStatus = DownloadStatus::Pending;
    assert_eq!(status.is_pending(), true);

    let status: DownloadStatus = DownloadStatus::Successful;
    assert_eq!(status.is_pending(), false);

    let err = io::Error::new(io::ErrorKind::InvalidInput, "This is a test error.");
    let status: DownloadStatus = DownloadStatus::from(err);
    assert_eq!(status.is_pending(), false);
}

#[test]
fn test_is_successful() {
    let status: DownloadStatus = DownloadStatus::Pending;
    assert_eq!(status.is_successful(), false);

    let status: DownloadStatus = DownloadStatus::Successful;
    assert_eq!(status.is_successful(), true);

    let err = io::Error::new(io::ErrorKind::InvalidInput, "This is a test error.");
    let status: DownloadStatus = DownloadStatus::from(err);
    assert_eq!(status.is_successful(), false);
}

#[test]
fn test_is_failed() {
    let status: DownloadStatus = DownloadStatus::Pending;
    assert_eq!(status.is_failed(), false);

    let status: DownloadStatus = DownloadStatus::Successful;
    assert_eq!(status.is_failed(), false);

    let err = io::Error::new(io::ErrorKind::InvalidInput, "This is a test error.");
    let status: DownloadStatus = DownloadStatus::from(err);
    assert_eq!(status.is_failed(), true);
}

#[test]
fn test_download_status_get_error() {
    let error_description = "This is a test error.";
    let err = Arc::new(DownloadError::from(io::Error::new(
        io::ErrorKind::InvalidInput,
        error_description,
    )));
    let status = DownloadStatus::Failed(Arc::clone(&err));
    match *status.get_error().expect("There must be an error.") {
        DownloadError::IoError(ref err)
            if err.kind() == io::ErrorKind::InvalidInput
                && err.to_string() == error_description => {}
        DownloadError::IoError(ref err) => panic!("{:?} is not the correct error.", err),
        DownloadError::ReqwestError(ref err) => panic!("{:?} is not the correct error.", err),
    }
}
