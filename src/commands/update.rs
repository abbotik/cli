use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command as ProcessCommand,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::*;

const RELEASE_REPO: &str = "abbotik/cli";
const BREW_FORMULA: &str = "abbotik/tap/abbot";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InstallMethod {
    Brew,
    Curl,
}

impl InstallMethod {
    fn as_str(self) -> &'static str {
        match self {
            InstallMethod::Brew => "brew",
            InstallMethod::Curl => "curl",
        }
    }
}

#[derive(Debug, Deserialize)]
struct LatestRelease {
    tag_name: String,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    #[serde(default)]
    draft: bool,
    #[serde(default)]
    prerelease: bool,
}

pub(super) async fn run(command: UpdateCommand) -> anyhow::Result<()> {
    let executable_path = env::current_exe()
        .map_err(|error| anyhow::anyhow!("failed to resolve current executable: {error}"))?;
    let canonical_path = executable_path
        .canonicalize()
        .unwrap_or_else(|_| executable_path.clone());
    let install_method = detect_install_method(&executable_path, &canonical_path);

    let report = if command.version_list {
        version_list_report(&executable_path, &canonical_path, install_method).await?
    } else {
        match install_method {
            InstallMethod::Brew => run_brew_update(
                &executable_path,
                &canonical_path,
                command.version.as_deref(),
            )?,
            InstallMethod::Curl => {
                run_curl_update(
                    &executable_path,
                    &canonical_path,
                    command.version.as_deref(),
                )
                .await?
            }
        }
    };

    if stdio::stdout().is_terminal() {
        print_text(&render_human_report(&report))?;
    } else {
        print_json(&report)?;
    }

    Ok(())
}

fn run_brew_update(
    executable_path: &Path,
    canonical_path: &Path,
    requested_version: Option<&str>,
) -> anyhow::Result<Value> {
    if let Some(version) = requested_version {
        anyhow::bail!(
            "Homebrew-managed abbot does not support installing an exact release version with `abbot update --version {version}`. Use `brew upgrade {BREW_FORMULA}` for the current formula, or reinstall with the curl/GitHub release path if you need a pinned binary."
        );
    }

    let mut command = ProcessCommand::new("brew");
    command.args(["upgrade", BREW_FORMULA]);

    if stdio::stdout().is_terminal() && stdio::stderr().is_terminal() {
        let status = command.status().map_err(|error| {
            anyhow::anyhow!("failed to run `brew upgrade {BREW_FORMULA}`: {error}")
        })?;
        if !status.success() {
            anyhow::bail!("`brew upgrade {BREW_FORMULA}` exited with status {status}");
        }

        Ok(json!({
            "ok": true,
            "install_method": InstallMethod::Brew.as_str(),
            "current_executable": executable_path.display().to_string(),
            "resolved_executable": canonical_path.display().to_string(),
            "command": format!("brew upgrade {BREW_FORMULA}"),
        }))
    } else {
        let output = command.output().map_err(|error| {
            anyhow::anyhow!("failed to run `brew upgrade {BREW_FORMULA}`: {error}")
        })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if stderr.is_empty() {
                anyhow::bail!(
                    "`brew upgrade {BREW_FORMULA}` exited with status {}",
                    output.status
                );
            }
            anyhow::bail!("`brew upgrade {BREW_FORMULA}` failed: {stderr}");
        }

        Ok(json!({
            "ok": true,
            "install_method": InstallMethod::Brew.as_str(),
            "current_executable": executable_path.display().to_string(),
            "resolved_executable": canonical_path.display().to_string(),
            "command": format!("brew upgrade {BREW_FORMULA}"),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
        }))
    }
}

async fn run_curl_update(
    executable_path: &Path,
    canonical_path: &Path,
    requested_version: Option<&str>,
) -> anyhow::Result<Value> {
    let version = match requested_version {
        Some(version) => normalize_version(version),
        None => fetch_latest_version().await?,
    };
    let target = current_target()?;
    let asset = release_asset_name(&version, target);
    let base_url = release_asset_base_url(&version, &asset);
    let archive_bytes = download_bytes(&base_url).await?;
    let checksum_text = download_text(&format!("{base_url}.sha256")).await?;
    let expected_checksum = checksum_text
        .split_whitespace()
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("invalid checksum file for {asset}"))?;
    let actual_checksum = hex_digest(&archive_bytes);
    if actual_checksum != expected_checksum {
        anyhow::bail!(
            "checksum mismatch for {asset}: expected {expected_checksum}, got {actual_checksum}"
        );
    }

    let temp_dir = make_temp_dir()?;
    let archive_path = temp_dir.join(&asset);
    fs::write(&archive_path, &archive_bytes)
        .map_err(|error| anyhow::anyhow!("failed to write {}: {error}", archive_path.display()))?;
    extract_archive(&archive_path, &temp_dir)?;

    let extracted_binary = temp_dir.join("abbot");
    if !extracted_binary.is_file() {
        anyhow::bail!(
            "release archive did not contain `abbot` at {}",
            extracted_binary.display()
        );
    }

    let install_dir = executable_path.parent().ok_or_else(|| {
        anyhow::anyhow!(
            "failed to determine install directory for {}",
            executable_path.display()
        )
    })?;
    let staged_binary = install_dir.join(format!(
        ".abbot-update-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    ));

    fs::copy(&extracted_binary, &staged_binary).map_err(|error| {
        anyhow::anyhow!(
            "failed to stage updated binary at {}: {error}",
            staged_binary.display()
        )
    })?;
    copy_permissions(&extracted_binary, &staged_binary)?;
    fs::rename(&staged_binary, executable_path).map_err(|error| {
        let _ = fs::remove_file(&staged_binary);
        anyhow::anyhow!("failed to replace {}: {error}", executable_path.display())
    })?;

    let _ = fs::remove_dir_all(&temp_dir);

    Ok(json!({
        "ok": true,
        "install_method": InstallMethod::Curl.as_str(),
        "current_executable": executable_path.display().to_string(),
        "resolved_executable": canonical_path.display().to_string(),
        "install_directory": install_dir.display().to_string(),
        "version": version,
        "asset": asset,
    }))
}

async fn fetch_latest_version() -> anyhow::Result<String> {
    let response = reqwest::Client::new()
        .get(format!(
            "https://api.github.com/repos/{RELEASE_REPO}/releases/latest"
        ))
        .header(reqwest::header::USER_AGENT, "abbot-update")
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .map_err(|error| anyhow::anyhow!("failed to query the latest release: {error}"))?;
    if !response.status().is_success() {
        anyhow::bail!(
            "failed to query the latest release: GitHub returned {}",
            response.status()
        );
    }

    let release: LatestRelease = response
        .json()
        .await
        .map_err(|error| anyhow::anyhow!("failed to parse the latest release response: {error}"))?;
    if release.tag_name.trim().is_empty() {
        anyhow::bail!("latest release response did not include a tag name");
    }
    Ok(release.tag_name)
}

async fn fetch_release_versions() -> anyhow::Result<Vec<String>> {
    let mut page = 1usize;
    let mut versions = Vec::new();
    let client = reqwest::Client::new();

    loop {
        let response = client
            .get(format!(
                "https://api.github.com/repos/{RELEASE_REPO}/releases?per_page=100&page={page}"
            ))
            .header(reqwest::header::USER_AGENT, "abbot-update")
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .send()
            .await
            .map_err(|error| anyhow::anyhow!("failed to query release versions: {error}"))?;
        if !response.status().is_success() {
            anyhow::bail!(
                "failed to query release versions: GitHub returned {}",
                response.status()
            );
        }

        let releases: Vec<Release> = response
            .json()
            .await
            .map_err(|error| anyhow::anyhow!("failed to parse release versions: {error}"))?;
        if releases.is_empty() {
            break;
        }

        let mut page_versions = releases
            .into_iter()
            .filter(|release| !release.draft && !release.prerelease)
            .map(|release| release.tag_name)
            .filter(|tag| !tag.trim().is_empty())
            .collect::<Vec<_>>();
        versions.append(&mut page_versions);
        page += 1;
    }

    if versions.is_empty() {
        anyhow::bail!("no published release versions found for {RELEASE_REPO}");
    }

    Ok(versions)
}

async fn version_list_report(
    executable_path: &Path,
    canonical_path: &Path,
    install_method: InstallMethod,
) -> anyhow::Result<Value> {
    let versions = fetch_release_versions().await?;
    Ok(json!({
        "ok": true,
        "action": "version_list",
        "install_method": install_method.as_str(),
        "current_executable": executable_path.display().to_string(),
        "resolved_executable": canonical_path.display().to_string(),
        "versions": versions,
    }))
}

async fn download_bytes(url: &str) -> anyhow::Result<Vec<u8>> {
    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "abbot-update")
        .send()
        .await
        .map_err(|error| anyhow::anyhow!("failed to download {url}: {error}"))?;
    if !response.status().is_success() {
        anyhow::bail!("failed to download {url}: HTTP {}", response.status());
    }
    response
        .bytes()
        .await
        .map(|bytes| bytes.to_vec())
        .map_err(|error| anyhow::anyhow!("failed to read {url}: {error}"))
}

async fn download_text(url: &str) -> anyhow::Result<String> {
    let response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "abbot-update")
        .send()
        .await
        .map_err(|error| anyhow::anyhow!("failed to download {url}: {error}"))?;
    if !response.status().is_success() {
        anyhow::bail!("failed to download {url}: HTTP {}", response.status());
    }
    response
        .text()
        .await
        .map_err(|error| anyhow::anyhow!("failed to read {url}: {error}"))
}

fn current_target() -> anyhow::Result<&'static str> {
    match env::consts::OS {
        "macos" => match env::consts::ARCH {
            "aarch64" => Ok("aarch64-apple-darwin"),
            "x86_64" => Ok("x86_64-apple-darwin"),
            other => anyhow::bail!("unsupported macOS architecture: {other}"),
        },
        "linux" => match env::consts::ARCH {
            "aarch64" => Ok("aarch64-unknown-linux-gnu"),
            "x86_64" => Ok("x86_64-unknown-linux-gnu"),
            other => anyhow::bail!("unsupported Linux architecture: {other}"),
        },
        other => anyhow::bail!("unsupported operating system: {other}"),
    }
}

fn normalize_version(version: &str) -> String {
    let trimmed = version.trim();
    if trimmed.starts_with('v') {
        trimmed.to_string()
    } else {
        format!("v{trimmed}")
    }
}

fn release_asset_name(version: &str, target: &str) -> String {
    format!("abbotik-cli-{version}-{target}.tar.gz")
}

fn release_asset_base_url(version: &str, asset: &str) -> String {
    format!("https://github.com/{RELEASE_REPO}/releases/download/{version}/{asset}")
}

fn make_temp_dir() -> anyhow::Result<PathBuf> {
    let temp_dir = env::temp_dir().join(format!(
        "abbot-update-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default()
    ));
    fs::create_dir_all(&temp_dir)
        .map_err(|error| anyhow::anyhow!("failed to create {}: {error}", temp_dir.display()))?;
    Ok(temp_dir)
}

fn extract_archive(archive_path: &Path, destination: &Path) -> anyhow::Result<()> {
    let status = ProcessCommand::new("tar")
        .arg("-xzf")
        .arg(archive_path)
        .arg("-C")
        .arg(destination)
        .status()
        .map_err(|error| anyhow::anyhow!("failed to run tar: {error}"))?;
    if !status.success() {
        anyhow::bail!("failed to extract {} with tar", archive_path.display());
    }
    Ok(())
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn copy_permissions(from: &Path, to: &Path) -> anyhow::Result<()> {
    let permissions = fs::metadata(from)
        .map_err(|error| anyhow::anyhow!("failed to read {} metadata: {error}", from.display()))?
        .permissions();
    fs::set_permissions(to, permissions)
        .map_err(|error| anyhow::anyhow!("failed to set {} permissions: {error}", to.display()))?;
    Ok(())
}

fn detect_install_method(executable_path: &Path, canonical_path: &Path) -> InstallMethod {
    if is_homebrew_install_path(executable_path) || is_homebrew_install_path(canonical_path) {
        InstallMethod::Brew
    } else {
        InstallMethod::Curl
    }
}

fn is_homebrew_install_path(path: &Path) -> bool {
    let parts = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    parts
        .windows(2)
        .any(|window| window[0] == "Cellar" && window[1] == "abbot")
}

fn render_human_report(report: &Value) -> String {
    if report.get("action").and_then(|value| value.as_str()) == Some("version_list") {
        let versions = report
            .get("versions")
            .and_then(|value| value.as_array())
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if versions.is_empty() {
            return "No published release versions found.".to_string();
        }

        return format!(
            "Published abbot release versions:\n\n{}",
            versions.join("\n")
        );
    }

    let install_method = report
        .get("install_method")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown");
    let current_executable = report
        .get("current_executable")
        .and_then(|value| value.as_str())
        .unwrap_or("<unknown>");

    match install_method {
        "brew" => {
            let command = report
                .get("command")
                .and_then(|value| value.as_str())
                .unwrap_or("brew upgrade");
            format!(
                "Updated Homebrew-managed abbot.\n\nExecutable: {current_executable}\nCommand: {command}"
            )
        }
        "curl" => {
            let version = report
                .get("version")
                .and_then(|value| value.as_str())
                .unwrap_or("<unknown>");
            format!(
                "Updated curl-installed abbot.\n\nExecutable: {current_executable}\nVersion: {version}"
            )
        }
        _ => "Updated abbot.".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_requested_version_without_prefix() {
        assert_eq!(normalize_version("1.7.1"), "v1.7.1");
    }

    #[test]
    fn preserves_requested_version_with_prefix() {
        assert_eq!(normalize_version("v1.7.1"), "v1.7.1");
    }

    #[test]
    fn detects_homebrew_cellar_path() {
        assert!(is_homebrew_install_path(Path::new(
            "/opt/homebrew/Cellar/abbot/1.7.1/bin/abbot"
        )));
        assert!(is_homebrew_install_path(Path::new(
            "/home/linuxbrew/.linuxbrew/Cellar/abbot/1.7.1/bin/abbot"
        )));
    }

    #[test]
    fn ignores_non_homebrew_paths() {
        assert!(!is_homebrew_install_path(Path::new(
            "/Users/ian/.local/bin/abbot"
        )));
        assert!(!is_homebrew_install_path(Path::new("/usr/local/bin/abbot")));
    }

    #[test]
    fn canonical_homebrew_path_wins_detection() {
        let install_method = detect_install_method(
            Path::new("/opt/homebrew/bin/abbot"),
            Path::new("/opt/homebrew/Cellar/abbot/1.7.1/bin/abbot"),
        );

        assert_eq!(install_method, InstallMethod::Brew);
    }

    #[test]
    fn renders_version_list_report() {
        let report = json!({
            "action": "version_list",
            "versions": ["v1.7.1", "v1.7.0"],
        });

        assert_eq!(
            render_human_report(&report),
            "Published abbot release versions:\n\nv1.7.1\nv1.7.0"
        );
    }
}
