use anyhow::Context;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Target {
    target: String,
    cpu: Option<String>,
    features: BTreeSet<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let mut collection: BTreeMap<String, BTreeSet<Target>> = BTreeMap::new();

    let filename = std::env::args_os().skip(1).next().unwrap();
    let contents = std::fs::read_to_string(filename)?;
    for line in contents.lines() {
        let (hash, remainder) = line.split_once(' ').unwrap();
        let hash = hash.trim();
        let (target, flags) = remainder
            .trim()
            .split_once('|')
            .with_context(|| format!("Failed to parse {:?}", line))?;
        let target = target.trim();
        let mut cpu = None;
        let mut features = BTreeSet::new();
        for part in flags.split(' ') {
            if let Some(maybe_cpu) = part.strip_prefix("-Ctarget-cpu=") {
                cpu = Some(maybe_cpu);
            }
            if let Some(feature_list) = part.strip_prefix("-Ctarget-feature=") {
                for f in feature_list.split(",") {
                    features.insert(f.to_owned());
                }
            }
            if part == "-Csoft-float=y" {
                features.insert("+soft-float".to_owned());
            }
        }
        let target = Target {
            target: target.to_owned(),
            cpu: cpu.map(|s| s.to_owned()),
            features,
        };
        eprintln!("Found {:?}", target);

        collection
            .entry(hash.to_owned())
            .or_default()
            .insert(target);
    }

    let mut values: Vec<BTreeSet<Target>> = collection.values().cloned().collect();
    values.sort();

    print!("{:#?}", values);

    Ok(())
}
