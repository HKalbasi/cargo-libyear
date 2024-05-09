use std::time::Duration;

use anyhow::bail;
use cargo_metadata::{CargoOpt, MetadataCommand};

use chrono::{DateTime, Utc};
use crates_io_api::SyncClient;
use prettytable::{format, row, Table};

fn main() {
    let metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();

    let client = SyncClient::new("cargo-libyear", Duration::from_millis(1)).unwrap();

    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row![
        "Crate",
        "Current Version",
        "Latest Version",
        "Libyears behind"
    ]);

    let mut result = 0.;

    for d in tqdm::tqdm(metadata.packages) {
        let Ok(k) = get_crate_info(&client, &d) else {
            continue;
        };
        let libyears = (k.latest_version_date - k.current_version_date)
            .to_std()
            .unwrap()
            .as_secs_f64()
            / 31_556_952.;
        table.add_row(row![
            k.name,
            k.current_version_number,
            k.latest_version_number,
            format!("{libyears:.02}"),
        ]);

        result += libyears;
    }

    table.printstd();

    println!("Your system is {result:.02} libyears behind")
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
