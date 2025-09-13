use std::time::Duration;

use anyhow::bail;
use cargo_metadata::{CargoOpt, MetadataCommand};
use clap::{Parser, ValueEnum};

use chrono::{DateTime, Utc};
use crates_io_api::SyncClient;
use prettytable::{format, row, Table};

#[derive(Parser)]
#[clap(name = "cargo-libyear")]
#[clap(about = "A simple measure of software dependency freshness")]
#[clap(version)]
struct Args {
    /// Sort dependencies by the specified criteria
    #[clap(long, value_enum, default_value_t = SortBy::Alphabetical)]
    sort: SortBy,
    
    /// Show only the top N dependencies (when sorted by libyear, shows the most outdated)
    #[clap(long)]
    top: Option<usize>,
    
    /// Path to Cargo.toml file
    #[clap(long, default_value = "./Cargo.toml")]
    manifest_path: String,
}

#[derive(ValueEnum, Clone)]
enum SortBy {
    /// Sort alphabetically by crate name (default)
    Alphabetical,
    /// Sort by libyears behind (most outdated first)
    Libyear,
}

fn main() {
    let args = Args::parse();
    let metadata = MetadataCommand::new()
        .manifest_path(&args.manifest_path)
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();

    let client = SyncClient::new("cargo-libyear", Duration::from_millis(1)).unwrap();

    // Collect all dependency information first
    let mut dependencies = Vec::new();
    let mut total_libyears = 0.;

    for d in tqdm::tqdm(metadata.packages) {
        let Ok(k) = get_crate_info(&client, &d) else {
            continue;
        };
        let libyears = (k.latest_version_date - k.current_version_date)
            .to_std()
            .unwrap()
            .as_secs_f64()
            / 31_556_952.;
        
        dependencies.push(DependencyInfo {
            name: k.name,
            current_version: k.current_version_number,
            latest_version: k.latest_version_number,
            libyears,
        });
        
        total_libyears += libyears;
    }

    // Sort dependencies based on the specified criteria
    match args.sort {
        SortBy::Alphabetical => {
            dependencies.sort_by(|a, b| a.name.cmp(&b.name));
        }
        SortBy::Libyear => {
            dependencies.sort_by(|a, b| b.libyears.partial_cmp(&a.libyears).unwrap_or(std::cmp::Ordering::Equal));
        }
    }

    // Apply top filter if specified
    if let Some(top_count) = args.top {
        dependencies.truncate(top_count);
    }

    // Create and populate the table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row![
        "Crate",
        "Current Version",
        "Latest Version",
        "Libyears behind"
    ]);

    for dep in &dependencies {
        table.add_row(row![
            dep.name,
            dep.current_version,
            dep.latest_version,
            format!("{:.02}", dep.libyears),
        ]);
    }

    table.printstd();

    println!("Your system is {total_libyears:.02} libyears behind")
}

struct DependencyInfo {
    name: String,
    current_version: String,
    latest_version: String,
    libyears: f64,
}

struct CrateInfo {
    name: String,
    current_version_number: String,
    current_version_date: DateTime<Utc>,
    latest_version_number: String,
    latest_version_date: DateTime<Utc>,
}

fn get_crate_info(client: &SyncClient, d: &cargo_metadata::Package) -> anyhow::Result<CrateInfo> {
    let k = client.get_crate(&d.name)?;
    let Some(current_version) = k.versions.iter().find(|x| x.num == d.version.to_string()) else {
        bail!("Current version not found");
    };
    let Some(latest_version) = k.versions.iter().max_by_key(|x| &x.created_at) else {
        bail!("Latest version not found");
    };
    Ok(CrateInfo {
        name: d.name.clone(),
        current_version_number: current_version.num.clone(),
        current_version_date: current_version.created_at,
        latest_version_number: latest_version.num.clone(),
        latest_version_date: latest_version.created_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn create_test_dependency(name: &str, current: &str, latest: &str, libyears: f64) -> DependencyInfo {
        DependencyInfo {
            name: name.to_string(),
            current_version: current.to_string(),
            latest_version: latest.to_string(),
            libyears,
        }
    }

    fn process_dependencies(args: Args, mut dependencies: Vec<DependencyInfo>) -> Vec<DependencyInfo> {
        // Apply the same sorting and filtering logic as main()
        match args.sort {
            SortBy::Alphabetical => {
                dependencies.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortBy::Libyear => {
                dependencies.sort_by(|a, b| b.libyears.partial_cmp(&a.libyears).unwrap_or(std::cmp::Ordering::Equal));
            }
        }

        if let Some(top_count) = args.top {
            dependencies.truncate(top_count);
        }

        dependencies
    }

    #[test]
    fn test_default_args_alphabetical_sorting() {
        // Test default behavior: should sort alphabetically
        let args = Args::try_parse_from(&["cargo-libyear"]).unwrap();
        
        let deps = vec![
            create_test_dependency("zebra", "1.0.0", "1.1.0", 2.5),
            create_test_dependency("alpha", "1.0.0", "1.2.0", 1.0),
            create_test_dependency("beta", "1.0.0", "1.1.0", 3.0),
        ];

        let result = process_dependencies(args, deps);
        
        assert_eq!(result[0].name, "alpha");
        assert_eq!(result[1].name, "beta");
        assert_eq!(result[2].name, "zebra");
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_libyear_sorting_with_clap() {
        // Test --sort libyear option
        let args = Args::try_parse_from(&["cargo-libyear", "--sort", "libyear"]).unwrap();
        
        let deps = vec![
            create_test_dependency("low", "1.0.0", "1.1.0", 0.3),
            create_test_dependency("high", "1.0.0", "1.2.0", 2.1),
            create_test_dependency("medium", "1.0.0", "1.1.0", 1.0),
        ];

        let result = process_dependencies(args, deps);
        
        assert_eq!(result[0].name, "high");
        assert_eq!(result[0].libyears, 2.1);
        assert_eq!(result[1].name, "medium");
        assert_eq!(result[1].libyears, 1.0);
        assert_eq!(result[2].name, "low");
        assert_eq!(result[2].libyears, 0.3);
    }

    #[test]
    fn test_alphabetical_sorting_with_clap() {
        // Test explicit --sort alphabetical option
        let args = Args::try_parse_from(&["cargo-libyear", "--sort", "alphabetical"]).unwrap();
        
        let deps = vec![
            create_test_dependency("zebra", "1.0.0", "1.1.0", 0.5),
            create_test_dependency("alpha", "1.0.0", "1.2.0", 1.0),
            create_test_dependency("beta", "1.0.0", "1.1.0", 0.3),
        ];

        let result = process_dependencies(args, deps);
        
        assert_eq!(result[0].name, "alpha");
        assert_eq!(result[1].name, "beta");
        assert_eq!(result[2].name, "zebra");
    }

    #[test]
    fn test_top_filter_with_clap() {
        // Test --top N option
        let args = Args::try_parse_from(&["cargo-libyear", "--top", "2"]).unwrap();
        
        let deps = vec![
            create_test_dependency("alpha", "1.0.0", "1.1.0", 0.1),
            create_test_dependency("beta", "1.0.0", "1.1.0", 0.2),
            create_test_dependency("gamma", "1.0.0", "1.1.0", 0.3),
        ];

        let result = process_dependencies(args, deps);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "alpha");
        assert_eq!(result[1].name, "beta");
    }

    #[test]
    fn test_combined_sort_libyear_and_top() {
        // Test --sort libyear --top N combination
        let args = Args::try_parse_from(&["cargo-libyear", "--sort", "libyear", "--top", "2"]).unwrap();
        
        let deps = vec![
            create_test_dependency("lowest", "1.0.0", "1.1.0", 0.1),
            create_test_dependency("highest", "1.0.0", "1.2.0", 3.5),
            create_test_dependency("medium", "1.0.0", "1.1.0", 1.2),
            create_test_dependency("high", "1.0.0", "1.1.0", 2.1),
        ];

        let result = process_dependencies(args, deps);
        
        // Should get top 2 sorted by libyear (highest first)
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "highest");
        assert_eq!(result[0].libyears, 3.5);
        assert_eq!(result[1].name, "high");
        assert_eq!(result[1].libyears, 2.1);
    }

    #[test]
    fn test_manifest_path_argument() {
        // Test --manifest-path option
        let args = Args::try_parse_from(&["cargo-libyear", "--manifest-path", "/custom/Cargo.toml"]).unwrap();
        
        assert_eq!(args.manifest_path, "/custom/Cargo.toml");
        assert!(matches!(args.sort, SortBy::Alphabetical)); // default
        assert_eq!(args.top, None);
    }

    #[test]
    fn test_libyear_calculation_formula() {
        // Test the actual libyear calculation used in the application
        use chrono::{TimeZone, Utc};

        let current_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let latest_date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        
        let duration = latest_date - current_date;
        let libyears = duration.to_std().unwrap().as_secs_f64() / 31_556_952.;
        
        // Should be approximately 1 year
        assert!((libyears - 1.0).abs() < 0.01);
    }
}
