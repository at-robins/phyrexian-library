//! The `download` module contains a rudimentary download manager for
//! asynchronous download of files via HTTP or HTTPS.

extern crate parking_lot;
extern crate rayon;
extern crate reqwest;
extern crate serde_json;

use parking_lot::Mutex;
use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};
use reqwest::header::CONTENT_LENGTH;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::{fs, fs::OpenOptions};
use std::{io, io::Read, io::Write};

/// The number of threads per DownloadManager instance.
/// This corresponds to the maximum number of simultanious downloads a manager can perform.
const DOWNLOAD_MANAGER_NUMBER_OF_THREADS: usize = 4;

/// The time interval over which the download speed is averaged.
const DOWNLOAD_SPEED_INTERVAL: std::time::Duration = std::time::Duration::from_millis(200);

/// A manager for asynchronous download of files via HTTP and HTTPS.
#[derive(Debug)]
pub struct DownloadManager {
    pool: ThreadPool,
    downloads: HashMap<Arc<PathBuf>, Arc<Mutex<Download>>>,
}

impl DownloadManager {
    /// Creates a new `DownloadManager`.
    ///
    /// # Examples
    /// ```
    /// use phyrexian_library::utility::download::DownloadManager;
    ///
    /// if let Ok(download_manager) = DownloadManager::new() {
    ///     // Download stuff.
    /// } else {
    ///     // Some error handling.
    /// };
    /// ```
    ///
    /// # Errors
    /// Returns an error if creation of the underlying thread pool failed.
    pub fn new() -> Result<DownloadManager, ThreadPoolBuildError> {
        Ok(DownloadManager {
            pool: ThreadPoolBuilder::new()
                .num_threads(DOWNLOAD_MANAGER_NUMBER_OF_THREADS)
                .build()?,
            downloads: HashMap::new(),
        })
    }

    /// Returns a [`DownloadProxy`] of the download for the specified file if any.
    /// The object allows interaction with the underlying [`Download`].
    ///
    /// # Argumetns
    ///
    /// * `path_to_output_file` - The path to the output file of a download.
    ///
    /// [`Download`]: ./struct.Download.html
    /// [`DownloadProxy`]: ./struct.DownloadProxy.html
    pub fn get_download<P>(&self, path_to_output_file: P) -> Option<DownloadProxy>
    where
        P: AsRef<Path>,
    {
        self.downloads
            .get(&Arc::new(path_to_output_file.as_ref().to_path_buf()))
            .map(|val| DownloadProxy {
                download: Arc::clone(&val),
            })
    }

    /// Downloads a file via HTTP or HTTPS. The progress of the download can be tracked via the `DownloadManager`.
    ///
    /// # Arguments
    ///
    /// * `link` - A URL to a file, which should be downloaded.
    /// * `output` - A path specifying the file to which the downloaded data is written.
    pub fn download<U, P>(&mut self, link: U, output: P)
    where
        U: reqwest::IntoUrl + Send + 'static,
        P: AsRef<Path>,
    {
        let download: Arc<Mutex<Download>> = Arc::new(Mutex::new(Download::pending()));
        let output_path: Arc<PathBuf> = Arc::new(output.as_ref().to_path_buf());
        self.downloads
            .insert(Arc::clone(&output_path), Arc::clone(&download));
        self.pool.spawn(move || {
            download_to_file(link, output_path, download);
        });
    }

    /// Returns `true` if pending or running downloads are present. Returns `false` if
    /// downloads were either completed successfully or did fail.
    pub fn has_active(&self) -> bool {
        for val in self.downloads.values() {
            match val.lock().status {
                DownloadStatus::Pending | DownloadStatus::Running => return true,
                _ => {}
            }
        }
        false
    }

    /// Removes all failed downloads from the manager and returns a list of them.
    pub fn remove_failed(&mut self) -> Vec<DownloadProxy> {
        let mut failed: Vec<DownloadProxy> = Vec::new();
        for val in self.downloads.values() {
            if let DownloadStatus::Failed(_) = val.lock().status {
                failed.push(DownloadProxy {
                    download: Arc::clone(&val),
                });
            }
        }
        self.downloads
            .retain(|_, value| !value.lock().status.is_failed());
        failed
    }

    /// Returns the number of downloads in this manager.
    pub fn size(&self) -> usize {
        self.downloads.len()
    }

    /*pub fn print_all(&self) {
        let mut success = 0;
        let mut pending = 0;
        let mut failures = 0;
        let mut running = 0;
        for val in self.downloads.values() {
            if let Ok(d) = val.lock() {
                match &d.status {
                    DownloadStatus::Successful => success += 1,
                    DownloadStatus::Pending => pending += 1,
                    DownloadStatus::Running => running += 1,
                    DownloadStatus::Failed(_) => failures += 1,
                }
            }
        }
        println!("Success: {}\nRunning: {}\nPending: {}\nFailed: {}", success, running, pending, failures);
    }*/
}

/// An enum containing all the potential errors that may occur during a download.
#[derive(Debug)]
pub enum DownloadError {
    /// An IO error, creating and modifiying a local file.
    IoError(io::Error),
    /// A reqwest error, related to URL parsing and web interaction.
    ReqwestError(reqwest::Error),
}

impl From<io::Error> for DownloadError {
    fn from(error: io::Error) -> Self {
        DownloadError::IoError(error)
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(error: reqwest::Error) -> Self {
        DownloadError::ReqwestError(error)
    }
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DownloadError::IoError(err) => err.fmt(f),
            DownloadError::ReqwestError(ref err) => err.fmt(f),
        }
    }
}

/// An `enum` indicating the current status of a [`Download`].
///
/// [`Download`]: ./struct.Download.html
#[derive(Debug)]
enum DownloadStatus {
    /// The download was completed without errors.
    Successful,
    /// The download failed due to the specified error.
    Failed(Arc<DownloadError>),
    /// The download is currently waiting to be started.
    Pending,
    /// The download is currently running.
    Running,
}

impl DownloadStatus {
    /// Returns `true` if the status is a [`Pending`] value.
    ///
    /// [`Pending`]: #variant.Pending
    fn is_pending(&self) -> bool {
        match self {
            DownloadStatus::Pending => true,
            _ => false,
        }
    }

    /// Returns `true` if the status is a [`Running`] value.
    ///
    /// [`Running`]: #variant.Running
    fn is_running(&self) -> bool {
        match self {
            DownloadStatus::Running => true,
            _ => false,
        }
    }

    /// Returns `true` if the status is a [`Successful`] value.
    ///
    /// [`Successful`]: #variant.Successful
    fn is_successful(&self) -> bool {
        match self {
            DownloadStatus::Successful => true,
            _ => false,
        }
    }

    /// Returns `true` if the status is a [`Failed`] value.
    ///
    /// [`Failed`]: #variant.Failed
    fn is_failed(&self) -> bool {
        match self {
            DownloadStatus::Failed(_) => true,
            _ => false,
        }
    }

    /// Returns the error cause of a failed [`Download`] if applicable.
    /// Returns `None` if the [`Download`] did not fail.
    ///
    /// [`Download`]: ./struct.Download.html
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    fn get_error(&self) -> Option<Arc<DownloadError>> {
        match self {
            DownloadStatus::Failed(ref err) => Some(Arc::clone(err)),
            _ => None,
        }
    }
}

impl Display for DownloadStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DownloadStatus::Successful => write!(f, "Successful"),
            DownloadStatus::Failed(ref err) => write!(f, "Failed({})", err),
            DownloadStatus::Pending => write!(f, "Pending"),
            DownloadStatus::Running => write!(f, "Running"),
        }
    }
}

impl From<io::Error> for DownloadStatus {
    fn from(error: io::Error) -> Self {
        DownloadStatus::Failed(Arc::new(DownloadError::from(error)))
    }
}

impl From<reqwest::Error> for DownloadStatus {
    fn from(error: reqwest::Error) -> Self {
        DownloadStatus::Failed(Arc::new(DownloadError::from(error)))
    }
}

#[derive(Debug)]
pub struct Download {
    status: DownloadStatus,
    downloaded_size: u64,
    total_size: Option<u64>,
    speed: f64,
}

impl Download {
    /// Creates a new pending download instance.
    fn pending() -> Self {
        Download {
            status: DownloadStatus::Pending,
            downloaded_size: 0,
            total_size: None,
            speed: 0f64,
        }
    }

    /// Returns the current size of the downloaded file.
    fn get_downloaded_size(&self) -> u64 {
        self.downloaded_size
    }

    /// Returns the current download speed, if the download is running.
    fn get_download_speed(&self) -> Option<f64> {
        match &self.status {
            DownloadStatus::Running => Some(self.speed),
            _ => None,
        }
    }
}

impl Display for Download {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.status {
            DownloadStatus::Running => write!(
                f,
                "{} ({} byte/sec): {}/{:?} byte",
                &self.status, &self.speed, &self.downloaded_size, &self.total_size
            ),
            _ => write!(
                f,
                "{}: {}/{:?} byte",
                &self.status, &self.downloaded_size, &self.total_size
            ),
        }
    }
}

// End user interaction without Arc or Mutex.
#[derive(Debug)]
pub struct DownloadProxy {
    download: Arc<Mutex<Download>>,
}

impl DownloadProxy {
    /// Returns `true` if the [`Download`] is waiting to be started.
    ///
    /// [`Download`]: ./struct.Download.html
    pub fn is_pending(&self) -> bool {
        self.download.lock().status.is_pending()
    }

    /// Returns `true` if the [`Download`] is currently performed.
    ///
    /// [`Download`]: ./struct.Download.html
    pub fn is_running(&self) -> bool {
        self.download.lock().status.is_running()
    }

    /// Returns `true` if the [`Download`] was completed without errors.
    ///
    /// [`Download`]: ./struct.Download.html
    pub fn is_successful(&self) -> bool {
        self.download.lock().status.is_successful()
    }

    /// Returns `true` if the [`Download`] did fail due to an error.
    ///
    /// [`Download`]: ./struct.Download.html
    pub fn is_failed(&self) -> bool {
        self.download.lock().status.is_failed()
    }

    /// Returns the error this [`Download`] emitted if any.
    ///
    /// [`Download`]: ./struct.Download.html
    pub fn get_error(&self) -> Option<Arc<DownloadError>> {
        self.download.lock().status.get_error()
    }

    /// Returns the current size of the downloaded file.
    pub fn get_downloaded_size(&self) -> u64 {
        self.download.lock().get_downloaded_size()
    }

    /// Returns the current download speed in byte/sec if the [`Download`] is running.
    ///
    /// [`Download`]: ./struct.Download.html
    pub fn get_download_speed(&self) -> Option<f64> {
        self.download.lock().get_download_speed()
    }
}

impl Display for DownloadProxy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.download.lock().fmt(f)
    }
}

fn download_to_file<U>(link: U, output: Arc<PathBuf>, download: Arc<Mutex<Download>>)
where
    U: reqwest::IntoUrl,
{
    download.lock().status = DownloadStatus::Running;

    let url = match link.into_url() {
        Ok(url) => url,
        Err(err) => {
            fail_download(DownloadError::from(err), download);
            return;
        }
    };

    let mut response = match reqwest::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            fail_download(DownloadError::from(err), download);
            return;
        }
    };

    if !response.status().is_success() {
        // TODO: Custom error
        fail_download(
            DownloadError::from(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{:?} status code.", response.status()),
            )),
            download,
        );
        return;
    }

    if let Some(Ok(Ok(length))) = response.headers().get(CONTENT_LENGTH).map(|con_len| {
        con_len
            .to_str()
            .map(|con_len_str| u64::from_str(con_len_str))
    }) {
        download.lock().total_size = Some(length);
    }
    if output.is_dir() {
        fail_download(
            DownloadError::from(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{:?} is a folder, not a file.", output),
            )),
            download,
        );
        return;
    }

    let parent_path = output
        .parent()
        .expect("This cannot fail as the download path must point to a file.");
    if let Err(err) = fs::create_dir_all(parent_path) {
        fail_download(DownloadError::from(err), download);
        return;
    }

    let mut dl_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(output.as_path())
    {
        Ok(file) => file,
        Err(err) => {
            fail_download(DownloadError::from(err), download);
            return;
        }
    };
    let mut buf = [0; 128 * 1024];
    let mut written = 0u64;
    let mut written_update = 0;
    let mut t_start = std::time::SystemTime::now();
    loop {
        if let Ok(time) = t_start.elapsed() {
            if time >= DOWNLOAD_SPEED_INTERVAL {
                download.lock().speed = ((written - written_update) * 1_000_000_000_u64) as f64
                    / time.as_nanos() as f64;
                t_start = std::time::SystemTime::now();
                written_update = written;
            }
        }
        let length = match response.read(&mut buf) {
            Ok(0) => break, // EOF.
            Ok(length) => length,
            Err(ref err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => {
                fail_download(DownloadError::from(err), download);
                return;
            }
        };
        if let Err(err) = dl_file.write_all(&buf[..length]) {
            fail_download(DownloadError::from(err), download);
            return;
        };
        written += length as u64;
        download.lock().downloaded_size = written;
    }
    download.lock().status = DownloadStatus::Successful;
}

fn fail_download(failure: DownloadError, download: Arc<Mutex<Download>>) {
    download.lock().status = DownloadStatus::Failed(Arc::new(failure));
}

#[cfg(test)]
mod test;
