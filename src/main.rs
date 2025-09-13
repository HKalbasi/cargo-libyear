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

    fn create_test_dependency(name: &str, current: &str, latest: &str, libyears: f64) -> DependencyInfo {
        DependencyInfo {
            name: name.to_string(),
            current_version: current.to_string(),
            latest_version: latest.to_string(),
            libyears,
        }
    }

    #[test]
    fn test_alphabetical_sorting() {
        let mut deps = vec![
            create_test_dependency("zebra", "1.0.0", "1.1.0", 0.5),
            create_test_dependency("alpha", "1.0.0", "1.2.0", 1.0),
            create_test_dependency("beta", "1.0.0", "1.1.0", 0.3),
        ];

        deps.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(deps[0].name, "alpha");
        assert_eq!(deps[1].name, "beta");
        assert_eq!(deps[2].name, "zebra");
    }

    #[test]
    fn test_libyear_sorting() {
        let mut deps = vec![
            create_test_dependency("low", "1.0.0", "1.1.0", 0.3),
            create_test_dependency("high", "1.0.0", "1.2.0", 2.1),
            create_test_dependency("medium", "1.0.0", "1.1.0", 1.0),
        ];

        deps.sort_by(|a, b| b.libyears.partial_cmp(&a.libyears).unwrap_or(std::cmp::Ordering::Equal));

        assert_eq!(deps[0].name, "high");
        assert_eq!(deps[1].name, "medium");
        assert_eq!(deps[2].name, "low");
    }

    #[test]
    fn test_top_filter() {
        let mut deps = vec![
            create_test_dependency("one", "1.0.0", "1.1.0", 0.1),
            create_test_dependency("two", "1.0.0", "1.1.0", 0.2),
            create_test_dependency("three", "1.0.0", "1.1.0", 0.3),
        ];

        deps.truncate(2);

        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].name, "one");
        assert_eq!(deps[1].name, "two");
    }

    #[test]
    fn test_libyear_calculation() {
        // Test the libyear calculation formula
        // 1 year = 31_556_952 seconds (365.2425 days)
        let seconds_per_year = 31_556_952.0;
        let one_year_in_seconds = seconds_per_year;
        let half_year_in_seconds = seconds_per_year / 2.0;

        assert_eq!(one_year_in_seconds / seconds_per_year, 1.0);
        assert_eq!(half_year_in_seconds / seconds_per_year, 0.5);
    }
}
