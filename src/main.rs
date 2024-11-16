use clap::Parser;
use log::{error, info, warn};
use reqwest;
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use url::Url;
use walkdir::WalkDir;
use zip::ZipArchive;
use std::error;

/// This script can search for files locally or in a GitHub repository.
/// It can filter by file extensions, ignore specified directories, and optionally print file contents.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub URL to download and search
    #[arg(short, long)]
    github_url: Option<String>,

    /// List of file extensions to search for
    #[arg(short, long)]
    extensions: Vec<String>,

    /// List of directories to ignore
    #[arg(short, long, default_value = "")]
    ignored_dirs: Vec<String>,

    /// Flag to print file contents
    #[arg(short, long)]
    print_contents: bool,

    /// Increase output verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug)]
struct GithubInfo {
    repo_url: String,
    branch_name: Option<String>,
    folder_path: Option<String>,
}

fn setup_logging(verbosity: u8) {
    let level = match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        _ => log::LevelFilter::Debug,
    };

    env_logger::Builder::new()
        .format_timestamp_secs()
        .filter_level(level)
        .init();
}

fn parse_github_url(url: &str) -> Result<GithubInfo, Box<dyn error::Error>> {
    let parsed_url = Url::parse(url)?;

    if parsed_url.host_str() != Some("github.com") {
        return Err("Not a valid GitHub URL".into());
    }

    let path_segments: Vec<&str> = parsed_url
        .path_segments()
        .ok_or("No path segments")?
        .collect();

    if path_segments.len() < 2 {
        return Err("URL doesn't contain a valid repository path".into());
    }

    let repo_url = format!(
        "https://github.com/{}/{}",
        path_segments[0], path_segments[1]
    );

    let (branch_name, folder_path) = if path_segments.len() >= 4 && path_segments[2] == "tree" {
        let branch = Some(path_segments[3].to_string());
        let folder = if path_segments.len() > 4 {
            Some(path_segments[4..].join("/"))
        } else {
            None
        };
        (branch, folder)
    } else {
        (None, None)
    };

    Ok(GithubInfo {
        repo_url,
        branch_name,
        folder_path,
    })
}

fn build_zip_url(repo_url: &str, branch: &str) -> String {
    format!("{}/archive/{}.zip", repo_url, branch)
}

fn download_and_extract_repo(
    zip_url: &str,
    target_folder: &Path,
) -> Result<PathBuf, Box<dyn error::Error>> {
    let temp_dir = TempDir::new()?;
    let zip_path = temp_dir.path().join("repo.zip");

    // Download zip file
    let mut response = reqwest::blocking::get(zip_url)?;
    let mut file = File::create(&zip_path)?;
    io::copy(&mut response, &mut file)?;

    // Extract zip file
    let zip_file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(zip_file)?;

    // Create target directory if it doesn't exist
    fs::create_dir_all(target_folder)?;

    // Extract all files
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target_folder.join(file.mangled_name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(target_folder.to_path_buf())
}

fn find_files(
    directory: &Path,
    extensions: &[String],
    ignored_dirs: &[String],
    print_contents: bool,
) -> Result<(), Box<dyn error::Error>> {
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_entry(|e| !ignored_dirs.contains(&e.file_name().to_string_lossy().to_string()))
    {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if let Some(extension) = file_path.extension() {
                if extensions.iter().any(|ext| {
                    ext.trim_start_matches('.') == extension.to_string_lossy().to_string()
                }) {
                    info!("Found file: {}", file_path.display());

                    if print_contents {
                        match fs::read_to_string(file_path) {
                            Ok(contents) => {
                                println!("# File: {}", file_path.display());
                                println!("{}", contents);
                                println!("# {}", "-".repeat(50));
                            }
                            Err(e) => error!("Error reading file {}: {}", file_path.display(), e),
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    setup_logging(args.verbose);

    let search_path = if let Some(github_url) = args.github_url {
        let github_info = parse_github_url(&github_url)?;
        let branch_name = github_info
            .branch_name
            .unwrap_or_else(|| "main".to_string());

        let zip_url = build_zip_url(&github_info.repo_url, &branch_name);
        info!("Downloading repository from: {}", zip_url);

        let target_folder = Path::new("downloaded_repo");
        let extracted_path = download_and_extract_repo(&zip_url, target_folder)?;
        info!(
            "Repository downloaded and extracted to: {}",
            extracted_path.display()
        );

        if let Some(folder_path) = github_info.folder_path {
            let search_path = extracted_path.join(folder_path);
            if !search_path.exists() {
                warn!(
                    "Specified folder '{}' not found in the repository.",
                    search_path.display()
                );
                return Ok(());
            }
            search_path
        } else {
            extracted_path
        }
    } else {
        PathBuf::from(".")
    };

    find_files(
        &search_path,
        &args.extensions,
        &args.ignored_dirs,
        args.print_contents,
    )?;
    Ok(())
}
