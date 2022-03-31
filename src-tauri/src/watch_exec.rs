use std::sync::Arc;

use crate::fs_watcher::FsWatcher;
use crate::idx_store::IdxStore;
use crate::utils;
#[cfg(windows)]
use crate::utils::get_win32_ready_drives;

pub fn run() {
  #[cfg(windows)]
  win_run();

  #[cfg(target_os = "linux")]
  linux_run();

  #[cfg(target_os = "macos")]
  macos_run();
}

#[cfg(target_os = "macos")]
fn macos_run() {
  let mut watcher = FsWatcher::new("/".to_string());
  std::thread::spawn(move || {
    watcher.start();
  });
}

#[cfg(target_os = "linux")]
fn linux_run(idx_store: Arc<IdxStore>) {
  let sub_root = utils::subs("/");
  for sub in sub_root {
    let mut watcher = FsWatcher::new(idx_store.clone(), sub);
    std::thread::spawn(move || {
      watcher.start();
    });
  }
}

#[cfg(windows)]
fn win_run() {
  let drives = unsafe { get_win32_ready_drives() };
  for driv in drives {
    let mut watcher = FsWatcher::new(idx_store.clone(), driv);
    watcher.start();
  }
}
