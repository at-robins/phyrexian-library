//! The `download` module contains a rudimentary download manager for 
//! asynchronous download of files via HTTP or HTTPS.

extern crate reqwest;
extern crate serde_json;
extern crate rayon;

use core::fmt::Display;
use std::collections::HashMap;
use std::sync::{Mutex, Arc, PoisonError, MutexGuard};
use std::io;
use std::io::{Write, Read};
use std::convert::TryInto;
use std::fs::{self, File, OpenOptions};
use std::str::FromStr;
use std::path::{Path};
use reqwest::header::{CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use rayon::{ThreadPoolBuilder, ThreadPoolBuildError, ThreadPool};

/// The number of threads per DownloadManager instance.
/// This corresponds to the maximum number of simultanious downloads a manager can perform.
const DOWNLOAD_MANAGER_NUMBER_OF_THREADS: usize = 4;

/// A manager for asynchronous download of files via HTTP and HTTPS.
pub struct DownloadManager<'a> {
    pool: ThreadPool,
    downloads: HashMap<&'a Path, Arc<Mutex<Download>>>,
}

impl<'a> DownloadManager<'a> {
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
    pub fn new() -> Result<DownloadManager<'a>, ThreadPoolBuildError> {
        Ok(DownloadManager{
            pool: ThreadPoolBuilder::new().num_threads(DOWNLOAD_MANAGER_NUMBER_OF_THREADS).build()?,
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
    pub fn get_download(&self, path_to_output_file: &Path) -> Option<DownloadProxy>{
        self.downloads.get(path_to_output_file).map(|val| DownloadProxy{download: Arc::clone(&val)})
    }
    
    /// Downloads a file via HTTP or HTTPS. The progress of the download can be tracked via the `DownloadManager`.
    /// 
    /// # Arguments
    /// 
    /// * `link` - A URL to a file, which should be downloaded.
    /// * `output` - A path specifying the file to which the downloaded data is written.
    pub fn download<U>(&mut self, link: U, output: &'static Path)
        where U: reqwest::IntoUrl + Send + 'static {
        let download: Arc<Mutex<Download>> = Arc::new(Mutex::new(Download::pending()));
        self.downloads.insert(output, Arc::clone(&download));
        self.pool.spawn(move || {download_to_file(link, output, download);});
    }
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


/// An `enum` indicating the current status of a [`Download`].
/// 
/// [`Download`]: ./struct.Download.html
#[derive(Debug)]
enum DownloadStatus {
    /// The download was completed without errors.
    Successful,
    /// The download failed due to the specified error.
    Failed(Arc<DownloadError>),
    /// The download is currently running or waiting to be started.
    Pending
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
            DownloadStatus::Failed(ref err) => write!(f, "Failed({:?})", err),
            DownloadStatus::Pending => write!(f, "Pending"),
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
}

impl Download {
    /// Creates a new pending download instance.
    fn pending() -> Self {
        Download{status: DownloadStatus::Pending, downloaded_size: 0, total_size: None}
    }
    
    fn get_downloaded_size(&self) -> u64 {
        self.downloaded_size
    }
}

impl Display for Download {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.status.fmt(f)
    }
}

// End user interaction without Arc or Mutex.
#[derive(Debug)]
pub struct DownloadProxy {
    download: Arc<Mutex<Download>>,
}

impl DownloadProxy {
    /// Returns `true` if the download is waiting to be started or currently performed.
    /// 
    /// # Errors
    /// 
    /// An error will be propagated if the thread handling the underlying [`Download`] is poisoned.
    /// 
    /// [`Download`]: ./struct.Download.html
    pub fn is_pending(&self) -> Result<bool, PoisonError<MutexGuard<Download>>> {
        self.download.lock().map(|val| val.status.is_pending())
    }
    
    /// Returns `true` if the download was completed without errors.
    /// 
    /// # Errors
    /// 
    /// An error will be propagated if the thread handling the underlying [`Download`] is poisoned.
    /// 
    /// [`Download`]: ./struct.Download.html
    pub fn is_successful(&self) -> Result<bool, PoisonError<MutexGuard<Download>>> {
        self.download.lock().map(|val| val.status.is_successful())
    }
    
    /// Returns `true` if the download did fail due to an error.
    /// 
    /// # Errors
    /// 
    /// An error will be propagated if the thread handling the underlying [`Download`] is poisoned.
    /// 
    /// [`Download`]: ./struct.Download.html
    pub fn is_failed(&self) -> Result<bool, PoisonError<MutexGuard<Download>>> {
        self.download.lock().map(|val| val.status.is_failed())
    }
    
    pub fn get_error(&self) -> Result<Option<Arc<DownloadError>>, PoisonError<MutexGuard<Download>>> {
         self.download.lock().map(|val| val.status.get_error())
    }
    
    pub fn get_downloaded_size(&self) -> Result<u64, PoisonError<MutexGuard<Download>>> {
         self.download.lock().map(|val| val.get_downloaded_size())
    }
}

impl Display for DownloadProxy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.download.lock() {
            Ok(val) => val.fmt(f),
            Err(err) => write!(f, "{}", err),
        }
    }
}

fn download_to_file<U>(link: U, output: &Path, download: Arc<Mutex<Download>>)
    where U: reqwest::IntoUrl {
    println!("{:?}: Started download.", output);
    let url = match link.into_url() {
        Ok(url) => url,
        Err(err) => {
            fail_download(DownloadError::from(err), download);
            return
        },
    };
        
    let mut response =  match reqwest::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            fail_download(DownloadError::from(err), download);
            return
        },
    };
    
    if let Some(Ok(Ok(length))) = response
        .headers()
        .get(CONTENT_LENGTH)
        .map(|l| {
            l.to_str().map(|ll| u64::from_str(ll))
         }) {
            if let Ok(mut dl) = download.lock() {
                dl.total_size = Some(length);
            } else {
                /*
                 * TODO: Some propper error handling in case lock() fails.
                 */
                return
            }
            
    }
    if output.is_dir() {
        fail_download(DownloadError::from(io::Error::new(io::ErrorKind::InvalidInput, 
            format!("{:?} is a folder, not a file.", output))), download);
        return
    }
    
    let parent_path = output.parent().expect("This cannot fail as the download path must point to a file.");
    if let Err(err) = fs::create_dir_all(parent_path) {
        fail_download(DownloadError::from(err), download);
        return
    }
    
    let mut dl_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(false)
        .open(output) {
        Ok(file) => file,
        Err(err) => {
            fail_download(DownloadError::from(err), download);
            return
        },
    };
    let mut buf = [0; 128 * 1024];
    let mut written = 0usize;
    let mut written_update = 0;
    let mut t_start = std::time::SystemTime::now();
    loop {
        if t_start.elapsed().unwrap() >= std::time::Duration::from_secs(3) {
            //println!("{:?}: {} MB [{} MB/sec]", output, (written / 1024 / 1024), (f64::from(written - written_update) /1024.0/1024.0/ (t_start.elapsed().unwrap().as_secs() as f64)));
            t_start = std::time::SystemTime::now();
            written_update = written;
        }
        let len = match response.read(&mut buf) {
            Ok(0) => break,  // EOF.
            Ok(len) => len,
            Err(ref err) if err.kind() == io::ErrorKind::Interrupted => continue,
            Err(err) => {
                fail_download(DownloadError::from(err), download);
                return
            },
        };
        if let Err(err) = dl_file.write_all(&buf[..len]) {
            fail_download(DownloadError::from(err), download);
            return
        };
        written += len;
        if let Ok(mut lock) = download.lock() {
            lock.downloaded_size = written  as u64;
        }
    }
    if let Ok(mut lock) = download.lock() {
        lock.status = DownloadStatus::Successful;
    }
    println!("{:?}: Finished download.", output);
}

fn fail_download(failure: DownloadError, download: Arc<Mutex<Download>>) {
    if let Ok(mut lock) = download.lock() {
        lock.status = DownloadStatus::Failed(Arc::new(failure));
    }
    /*
     * TODO: Some error handling might be advised in case lock() fails.
     */
}

pub fn download_chunks(url: reqwest::Url) -> Result<(), Box<dyn std::error::Error>> {
    const CHUNK_SIZE: u32 = 10240;

    let client = reqwest::Client::new();
    let response = client.head(url.clone()).send()?;
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("response doesn't include the content length")?;
    let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;
    println!("File length: {}", length);
    let mut output_file = File::create("download.bin")?;

    println!("starting download...");
    for range in PartialRangeIter::new(0, ((length - 1) as u32).try_into().unwrap(), CHUNK_SIZE) {
        println!("range {:?}", range);
        let mut response = client.get(url.clone()).header(RANGE, range).send()?;

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
            println!("Unexpected server response: {}", status)
        }

        io::copy(&mut response, &mut output_file)?;
    }

    println!("Finished with success!");
    Ok(())
}

struct PartialRangeIter {
    
    chunk_size: u32,
    max_length: u32,
    val: u32,
    
}

impl PartialRangeIter {
    
    fn new(starting_value: u32, max_value: u32, chunk_size: u32) -> PartialRangeIter {
        PartialRangeIter{
            chunk_size,
            max_length: max_value,
            val: starting_value
        }
    }
    
}

impl Iterator for PartialRangeIter {
    type Item = String;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.val > self.max_length {
            None
        } else {
            let current_val = self.val;
            self.val += self.chunk_size;
            
            if self.val > self.max_length {
                Some(format!("{}-{}", current_val, self.max_length))
            } else {
                Some(format!("{}-{}", current_val, self.val - 1))
            }
        }
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_partial_range_iter() {
        let start = 1;
        let end  = 1045;
        let step = 34;
        let range_iter = PartialRangeIter::new(start,end,step);
        let mut range: Vec<String> = Vec::new();
        let mut i = start;
        while i <= end  {
            let lower = i;
            i += step;
            if i > end {
                range.push(format!("{}-{}", lower, end));
            } else {
                range.push(format!("{}-{}", lower, i-1));

            }
        }
        assert_eq!(range_iter.collect::<Vec<String>>(), range);
    }
    
    #[test]
    fn test_download_status_is_pending() {
        let status: DownloadStatus = DownloadStatus::Pending;
        assert_eq!(status.is_pending(), true);
        
        let status: DownloadStatus = DownloadStatus::Successful;
        assert_eq!(status.is_pending(), false);
        
        let err = io::Error::new(io::ErrorKind::InvalidInput, "This is a test error.");
        let status: DownloadStatus = DownloadStatus::from(err);
        assert_eq!(status.is_pending(), false);
    }
    
    #[test]
    fn test_download_status_is_successful() {
        let status: DownloadStatus = DownloadStatus::Pending;
        assert_eq!(status.is_successful(), false);
        
        let status: DownloadStatus = DownloadStatus::Successful;
        assert_eq!(status.is_successful(), true);
        
        let err = io::Error::new(io::ErrorKind::InvalidInput, "This is a test error.");
        let status: DownloadStatus = DownloadStatus::from(err);
        assert_eq!(status.is_successful(), false);
    }
    
    #[test]
    fn test_download_status_is_failed() {
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
        use std::error::Error;
        let error_description = "This is a test error.";
        let err = Arc::new(DownloadError::from(io::Error::new(io::ErrorKind::InvalidInput, error_description)));
        let status = DownloadStatus::Failed(Arc::clone(&err));
        match *status.get_error().expect("There must be an error.") {
            DownloadError::IoError(ref err) if err.kind() == io::ErrorKind::InvalidInput && err.description() == error_description => {},
            DownloadError::IoError(ref err) => panic!("{:?} is not the correct error.", err),
            DownloadError::ReqwestError(ref err) => panic!("{:?} is not the correct error.", err),
        }
    }

}