use super::*;

#[test]
fn test_remove_failed() {
    let mut manager = DownloadManager::new().unwrap();
    let success = new_download(DownloadStatus::Successful);
    let pending = new_download(DownloadStatus::Pending);
    let running = new_download(DownloadStatus::Running);
    let download_map = &mut manager.downloads;
    download_map.insert(new_path("/success"), success);
    download_map.insert(new_path("/pending"), pending);
    download_map.insert(new_path("/running"), running);
    let mut failed_list = Vec::new();
    for i in 0..24 {
        let err = io::Error::new(io::ErrorKind::InvalidInput, format!("{}", i));
        let failed_download = new_download(DownloadStatus::from(err));
        download_map.insert(new_path(format!("/{}", i)), Arc::clone(&failed_download));
        failed_list.push(DownloadProxy {
            download: failed_download,
        });
    }
    let obtained_failed = manager.remove_failed();
    assert_eq!(manager.size(), 3);
    assert_eq!(obtained_failed.len(), failed_list.len());
    for fail in obtained_failed {
        assert!(fail.is_failed());
    }
}

#[test]
fn test_size() {
    let mut manager = DownloadManager::new().unwrap();
    let success = new_download(DownloadStatus::Successful);
    let pending = new_download(DownloadStatus::Pending);
    let running = new_download(DownloadStatus::Running);
    let download_map = &mut manager.downloads;
    download_map.insert(new_path("/success"), success);
    download_map.insert(new_path("/pending"), pending);
    download_map.insert(new_path("/running"), running);
    for i in 0..95 {
        let err = io::Error::new(io::ErrorKind::InvalidInput, format!("{}", i));
        let failed_download = new_download(DownloadStatus::from(err));
        download_map.insert(new_path(format!("/{}", i)), Arc::clone(&failed_download));
    }
    assert_eq!(manager.size(), 98);
}
