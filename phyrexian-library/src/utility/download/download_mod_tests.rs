use super::*;

fn new_download(status: DownloadStatus) -> Arc<Mutex<Download>> {
        Arc::new(Mutex::new(Download{status, downloaded_size: 0, total_size: None, speed: 0f64}))
}

#[test]
fn test_fail_download() {
    let download = new_download(DownloadStatus::Pending);
    assert!(!download.lock().unwrap().status.is_failed());
    let err = DownloadError::from(io::Error::new(io::ErrorKind::InvalidInput, "This is a test error."));
    fail_download(err, Arc::clone(&download));
    assert!(download.lock().unwrap().status.is_failed());
}