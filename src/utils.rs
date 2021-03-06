use anyhow::*;
use app_dirs::{app_root, AppDataType, AppInfo};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(target_family = "windows")]
const APP_INFO: AppInfo = AppInfo {
	name: "Polaris",
	author: "Permafrost",
};

#[cfg(not(target_family = "windows"))]
const APP_INFO: AppInfo = AppInfo {
	name: "polaris",
	author: "permafrost",
};

pub fn get_data_root() -> Result<PathBuf> {
	if let Ok(root) = app_root(AppDataType::UserData, &APP_INFO) {
		fs::create_dir_all(&root)?;
		return Ok(root);
	}
	bail!("Could not retrieve data directory root");
}

#[derive(Debug, PartialEq)]
pub enum AudioFormat {
	FLAC,
	MP3,
	MP4,
	MPC,
	OGG,
}

#[cfg_attr(feature = "profile-index", flame)]
pub fn get_audio_format(path: &Path) -> Option<AudioFormat> {
	let extension = match path.extension() {
		Some(e) => e,
		_ => return None,
	};
	let extension = match extension.to_str() {
		Some(e) => e,
		_ => return None,
	};
	match extension.to_lowercase().as_str() {
		"flac" => Some(AudioFormat::FLAC),
		"mp3" => Some(AudioFormat::MP3),
		"m4a" => Some(AudioFormat::MP4),
		"mpc" => Some(AudioFormat::MPC),
		"ogg" => Some(AudioFormat::OGG),
		_ => None,
	}
}

#[test]
fn test_get_audio_format() {
	assert_eq!(get_audio_format(Path::new("animals/🐷/my🐖file.jpg")), None);
	assert_eq!(
		get_audio_format(Path::new("animals/🐷/my🐖file.flac")),
		Some(AudioFormat::FLAC)
	);
}
