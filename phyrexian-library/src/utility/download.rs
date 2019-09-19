extern crate reqwest;
extern crate serde_json;
extern crate rayon;

use core::fmt::Display;
use std::collections::HashMap;
use std::sync::{Mutex, Arc, PoisonError, MutexGuard};
use std::io::{Write, Read};
use std::convert::TryInto;
use std::fs::{self, File, OpenOptions};
use std::str::FromStr;
use std::path::{Path};
use reqwest::header::{CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use rayon::{ThreadPoolBuilder, ThreadPoolBuildError, ThreadPool};

const DOWNLOAD_MANAGER_NUMBER_OF_THREADS: usize = 4;

///
/// A manager for asynchronous download of files via HTTP and HTTPS.
/// 
pub struct DownloadManager<'a> {
    pool: ThreadPool,
    downloads: HashMap<&'a Path, Arc<Mutex<Download>>>,
}

impl<'a> DownloadManager<'a> {
    /// 
    /// Creates a new DownloadManager instance.
    /// 
    /// # Examples
    /// ```
    /// let download_mamager = phyrexian_library::DownloadManager::new().unwrap();
    /// ```
    /// 
    /// # Errors
    /// Returns an error if creation of the underlying thread pool failed.
    /// 
    pub fn new() -> Result<DownloadManager<'a>, ThreadPoolBuildError> {
        Ok(DownloadManager{
            pool: ThreadPoolBuilder::new().num_threads(DOWNLOAD_MANAGER_NUMBER_OF_THREADS).build()?,
            downloads: HashMap::new(),
        })
    }
    
    pub fn get_download(&self, path_to_output_file: &Path) -> Option<DownloadProxy>{
        self.downloads.get(path_to_output_file).map(|val| DownloadProxy{download: Arc::clone(&val)})
    }
    
    ///
    /// Downloads a file via HTTP or HTTPS. Returns a Download, which reflects the state of the download process. 
    /// 
    /// # Arguments
    /// * `link` - A URL to a file for downloading.
    /// * `output` - A path specifying the file to which the downloaded data is written.
    /// 
    pub fn download<U>(&mut self, link: U, output: &'static Path)
        where U: reqwest::IntoUrl + Send + 'static {
        let download: Arc<Mutex<Download>> = Arc::new(Mutex::new(Download::pending()));
        self.downloads.insert(output, Arc::clone(&download));
        self.pool.spawn(move || {download_to_file(link, output, download);});
    }
}

#[derive(Debug)]
pub enum DownloadStatus {
    Successful,
    Failed(Arc<dyn std::error::Error + Send + Sync>),
    Pending
}

impl DownloadStatus {
    fn is_pending(&self) -> bool {
        match self {
            DownloadStatus::Pending => true,
            _ => false,
        }
    }
    
    fn get_error(&self) -> Option<Arc<dyn std::error::Error + Send + Sync>> {
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

#[derive(Debug)]
pub struct Download {
    status: DownloadStatus,
    downloaded_size: u32,
}

impl Download {
    fn pending() -> Self {
        Download{status: DownloadStatus::Pending, downloaded_size: 0}
    }
    
    fn get_downloaded_size(&self) -> u32 {
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
    pub fn is_pending(&self) -> Result<bool, PoisonError<MutexGuard<Download>>> {
        self.download.lock().map(|val| val.status.is_pending())
    }
    
    pub fn get_error(&self) -> Result<Option<Arc<dyn std::error::Error + Send + Sync>>, PoisonError<MutexGuard<Download>>> {
         self.download.lock().map(|val| val.status.get_error())
    }
    
    pub fn get_downloaded_size(&self) -> Result<u32, PoisonError<MutexGuard<Download>>> {
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
            fail_download(err, download);
            return
        },
    };
        
    let mut response =  match reqwest::get(url) {
        Ok(resp) => resp,
        Err(err) => {
            fail_download(err, download);
            return
        },
    };
    
    if output.is_dir() {
        fail_download(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
            format!("{:?} is a folder, not a file.", output)), download);
        return
    }
    
    let parent_path = output.parent().expect("This cannot fail as the download path must point to a file.");
    if let Err(err) = fs::create_dir_all(parent_path) {
        fail_download(err, download);
        return
    }
    
    let mut dl_file = match OpenOptions::new().read(true).write(true).create(true).append(false).open(output) {
        Ok(file) => file,
        Err(err) => {
            fail_download(err, download);
            return
        },
    };
    let mut buf = [0; 128 * 1024];
    let mut written = 0u32;
    let mut written_update = 0;
    let mut t_start = std::time::SystemTime::now();
    loop {
        if t_start.elapsed().unwrap() >= std::time::Duration::from_secs(3) {
            println!("{:?}: {} MB [{} MB/sec]", output, (written / 1024 / 1024), (f64::from(written - written_update) /1024.0/1024.0/ (t_start.elapsed().unwrap().as_secs() as f64)));
            t_start = std::time::SystemTime::now();
            written_update = written;
        }
        let len = match response.read(&mut buf) {
            Ok(0) => break,  // EOF.
            Ok(len) => len,
            Err(ref err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(err) => {
                fail_download(err, download);
                return
            },
        };
        if let Err(err) = dl_file.write_all(&buf[..len]) {
            fail_download(err, download);
            return
        };
        written += len as u32;
        if let Ok(mut lock) = download.lock() {
            lock.downloaded_size = written;
        }
    }
    if let Ok(mut lock) = download.lock() {
        lock.status = DownloadStatus::Successful;
    }
    println!("{:?}: Finished download.", output);
}

fn fail_download<E>(failure: E, download: Arc<Mutex<Download>>) where E: std::error::Error + Send + Sync + 'static{
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

        std::io::copy(&mut response, &mut output_file)?;
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
    
}