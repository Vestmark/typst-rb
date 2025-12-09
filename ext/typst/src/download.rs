use std::fmt::Display;

use typst_kit::download::{DownloadState, Downloader, Progress};

pub struct SlientDownload<T>(pub T);

impl<T: Display> Progress for SlientDownload<T> {
    fn print_start(&mut self) {}

    fn print_progress(&mut self, _state: &DownloadState) {}

    fn print_finish(&mut self, _state: &DownloadState) {}
}

pub struct FakeDownloader;

impl Downloader for FakeDownloader {
    fn download(&self, url: &str) -> Result<Vec<u8>, ureq::Error> {
        Err(ureq::Error::Status(404, "Not Found"))
    }

    fn new() -> Self {
        Self
    }
}

/// Returns a new downloader.
pub fn downloader() -> Downloader {
    FakeDownloader::new()
}
