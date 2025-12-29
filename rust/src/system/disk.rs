//! Disk utilities
//!
//! Disk space, temp folder cleanup, etc.

#![allow(dead_code)]

use sysinfo::Disks;
use std::fs;
use std::path::PathBuf;

/// Disk/drive info
#[derive(Debug, Clone, Default)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_gb: f64,
    pub available_gb: f64,
    pub used_gb: f64,
    pub usage_percent: f64,
}

/// Get all disk drives
pub fn get_disks() -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    
    disks
        .iter()
        .map(|disk| {
            let total = disk.total_space() as f64;
            let available = disk.available_space() as f64;
            let used = total - available;
            
            DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                total_gb: total / 1024.0 / 1024.0 / 1024.0,
                available_gb: available / 1024.0 / 1024.0 / 1024.0,
                used_gb: used / 1024.0 / 1024.0 / 1024.0,
                usage_percent: if total > 0.0 { (used / total) * 100.0 } else { 0.0 },
            }
        })
        .collect()
}

/// Folder size info
#[derive(Debug, Clone, Default)]
pub struct FolderSize {
    pub path: String,
    pub size_mb: f64,
    pub file_count: usize,
}

/// Get size of temp folders
pub fn get_temp_folder_sizes() -> Vec<FolderSize> {
    let mut folders = Vec::new();
    
    // Windows temp folder
    if let Ok(temp) = std::env::var("TEMP") {
        if let Some(size) = get_folder_size(&PathBuf::from(&temp)) {
            folders.push(FolderSize {
                path: temp,
                size_mb: size.0,
                file_count: size.1,
            });
        }
    }
    
    // User temp folder (sometimes different)
    if let Ok(tmp) = std::env::var("TMP") {
        if let Some(size) = get_folder_size(&PathBuf::from(&tmp)) {
            // Avoid duplicates
            if !folders.iter().any(|f| f.path == tmp) {
                folders.push(FolderSize {
                    path: tmp,
                    size_mb: size.0,
                    file_count: size.1,
                });
            }
        }
    }
    
    // Windows Temp
    let win_temp = PathBuf::from(r"C:\Windows\Temp");
    if win_temp.exists() {
        if let Some(size) = get_folder_size(&win_temp) {
            folders.push(FolderSize {
                path: win_temp.to_string_lossy().to_string(),
                size_mb: size.0,
                file_count: size.1,
            });
        }
    }
    
    // Prefetch
    let prefetch = PathBuf::from(r"C:\Windows\Prefetch");
    if prefetch.exists() {
        if let Some(size) = get_folder_size(&prefetch) {
            folders.push(FolderSize {
                path: prefetch.to_string_lossy().to_string(),
                size_mb: size.0,
                file_count: size.1,
            });
        }
    }
    
    folders
}

/// Calculate folder size (returns size in MB and file count)
fn get_folder_size(path: &PathBuf) -> Option<(f64, usize)> {
    let mut total_size: u64 = 0;
    let mut file_count: usize = 0;
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total_size += metadata.len();
                    file_count += 1;
                } else if metadata.is_dir() {
                    // Recursively add subdirectory size (with depth limit)
                    if let Some((sub_size, sub_count)) = get_folder_size_recursive(&entry.path(), 3) {
                        total_size += sub_size;
                        file_count += sub_count;
                    }
                }
            }
        }
    }
    
    Some((total_size as f64 / 1024.0 / 1024.0, file_count))
}

/// Recursive folder size with depth limit
fn get_folder_size_recursive(path: &PathBuf, depth: usize) -> Option<(u64, usize)> {
    if depth == 0 {
        return Some((0, 0));
    }
    
    let mut total_size: u64 = 0;
    let mut file_count: usize = 0;
    
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total_size += metadata.len();
                    file_count += 1;
                } else if metadata.is_dir() {
                    if let Some((sub_size, sub_count)) = get_folder_size_recursive(&entry.path(), depth - 1) {
                        total_size += sub_size;
                        file_count += sub_count;
                    }
                }
            }
        }
    }
    
    Some((total_size, file_count))
}

/// Clean a temp folder (requires admin for Windows\Temp)
pub fn clean_temp_folder(path: &str) -> Result<(usize, f64), String> {
    let path = PathBuf::from(path);
    if !path.exists() {
        return Err("Folder does not exist".to_string());
    }
    
    let mut deleted_count = 0;
    let mut deleted_size: u64 = 0;
    
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            
            // Try to delete file or folder
            let result = if entry_path.is_dir() {
                fs::remove_dir_all(&entry_path)
            } else {
                fs::remove_file(&entry_path)
            };
            
            if result.is_ok() {
                if let Ok(metadata) = entry.metadata() {
                    deleted_size += metadata.len();
                }
                deleted_count += 1;
            }
            // Silently skip files that can't be deleted (in use, etc.)
        }
    }
    
    Ok((deleted_count, deleted_size as f64 / 1024.0 / 1024.0))
}

/// Get recycle bin size (Windows only, approximation)
pub fn get_recycle_bin_info() -> Option<FolderSize> {
    // The recycle bin is a bit tricky to access directly
    // This is a simple approximation using the $Recycle.Bin folder
    let recycle_paths = vec![
        PathBuf::from(r"C:\$Recycle.Bin"),
    ];
    
    for path in recycle_paths {
        if path.exists() {
            if let Some((size, count)) = get_folder_size(&path) {
                return Some(FolderSize {
                    path: "Recycle Bin".to_string(),
                    size_mb: size,
                    file_count: count,
                });
            }
        }
    }
    
    None
}
