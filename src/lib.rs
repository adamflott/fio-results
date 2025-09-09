use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use bytesize::ByteSize;
use chrono::serde::ts_milliseconds;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FioResult {
    #[serde(rename = "fio version")]
    pub fio_version: String,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "timestamp_ms", with = "ts_milliseconds")]
    pub timestamp_ms: DateTime<Utc>,
    pub time: String,
    #[serde(rename = "global options")]
    pub global_options: GlobalOptions,
    pub jobs: Vec<Job>,
    #[serde(rename = "disk_util")]
    pub disk_util: Vec<DiskUtil>,
}

#[serde_as]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalOptions {
    pub filename: PathBuf,
    pub size: ByteSize,
    #[serde(rename = "cpus_allowed")]
    pub cpus_allowed: String,
    #[serde(rename = "cpus_allowed_policy")]
    pub cpus_allowed_policy: String,
    pub ioscheduler: String,
    pub direct: String,
    pub rw: TestType,
    #[serde_as(as = "DisplayFromStr")]
    pub bs: BlockSize,
    pub ioengine: String,
    pub iodepth: String,
    pub runtime: String,
    pub numjobs: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub jobname: String,
    pub groupid: u64,
    #[serde(rename = "job_start", with = "ts_milliseconds")]
    pub job_start: DateTime<Utc>,
    pub error: u64,
    pub eta: u64,
    pub elapsed: u64,
    #[serde(rename = "job options")]
    pub job_options: JobOptions,
    pub read: Read,
    pub write: Write,
    pub trim: Trim,
    pub sync: Sync,
    #[serde(rename = "job_runtime")]
    pub job_runtime: u64,
    #[serde(rename = "usr_cpu")]
    pub usr_cpu: f64,
    #[serde(rename = "sys_cpu")]
    pub sys_cpu: f64,
    pub ctx: u64,
    pub majf: u64,
    pub minf: u64,
    #[serde(rename = "iodepth_level")]
    pub iodepth_level: IodepthLevel,
    #[serde(rename = "iodepth_submit")]
    pub iodepth_submit: IodepthSubmit,
    #[serde(rename = "iodepth_complete")]
    pub iodepth_complete: IodepthComplete,
    #[serde(rename = "latency_ns")]
    pub latency_ns: LatencyNs,
    #[serde(rename = "latency_us")]
    pub latency_us: LatencyUs,
    #[serde(rename = "latency_ms")]
    pub latency_ms: LatencyMs,
    #[serde(rename = "latency_depth")]
    pub latency_depth: u64,
    #[serde(rename = "latency_target")]
    pub latency_target: u64,
    #[serde(rename = "latency_percentile")]
    pub latency_percentile: f64,
    #[serde(rename = "latency_window")]
    pub latency_window: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobOptions {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Read {
    #[serde(rename = "io_bytes")]
    pub io_bytes: ByteSize,
    #[serde(rename = "io_kbytes")]
    pub io_kbytes: u64, // TODO
    #[serde(rename = "bw_bytes")]
    pub bw_bytes: ByteSize,
    pub bw: u64,
    pub iops: f64,
    pub runtime: u64,
    #[serde(rename = "total_ios")]
    pub total_ios: u64,
    #[serde(rename = "short_ios")]
    pub short_ios: u64,
    #[serde(rename = "drop_ios")]
    pub drop_ios: u64,
    #[serde(rename = "slat_ns")]
    pub slat_ns: SlatNs,
    #[serde(rename = "clat_ns")]
    pub clat_ns: ClatNs,
    #[serde(rename = "lat_ns")]
    pub lat_ns: LatNs,
    #[serde(rename = "bw_min")]
    pub bw_min: u64,
    #[serde(rename = "bw_max")]
    pub bw_max: u64,
    #[serde(rename = "bw_agg")]
    pub bw_agg: f64,
    #[serde(rename = "bw_mean")]
    pub bw_mean: f64,
    #[serde(rename = "bw_dev")]
    pub bw_dev: f64,
    #[serde(rename = "bw_samples")]
    pub bw_samples: u64,
    #[serde(rename = "iops_min")]
    pub iops_min: u64,
    #[serde(rename = "iops_max")]
    pub iops_max: u64,
    #[serde(rename = "iops_mean")]
    pub iops_mean: f64,
    #[serde(rename = "iops_stddev")]
    pub iops_stddev: f64,
    #[serde(rename = "iops_samples")]
    pub iops_samples: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlatNs {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClatNs {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
    pub percentile: Percentile,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Percentile {
    #[serde(rename = "1.000000")]
    pub n1_000000: u64,
    #[serde(rename = "5.000000")]
    pub n5_000000: u64,
    #[serde(rename = "10.000000")]
    pub n10_000000: u64,
    #[serde(rename = "20.000000")]
    pub n20_000000: u64,
    #[serde(rename = "30.000000")]
    pub n30_000000: u64,
    #[serde(rename = "40.000000")]
    pub n40_000000: u64,
    #[serde(rename = "50.000000")]
    pub n50_000000: u64,
    #[serde(rename = "60.000000")]
    pub n60_000000: u64,
    #[serde(rename = "70.000000")]
    pub n70_000000: u64,
    #[serde(rename = "80.000000")]
    pub n80_000000: u64,
    #[serde(rename = "90.000000")]
    pub n90_000000: u64,
    #[serde(rename = "95.000000")]
    pub n95_000000: u64,
    #[serde(rename = "99.000000")]
    pub n99_000000: u64,
    #[serde(rename = "99.500000")]
    pub n99_500000: u64,
    #[serde(rename = "99.900000")]
    pub n99_900000: u64,
    #[serde(rename = "99.950000")]
    pub n99_950000: u64,
    #[serde(rename = "99.990000")]
    pub n99_990000: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatNs {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Write {
    #[serde(rename = "io_bytes")]
    pub io_bytes: ByteSize,
    #[serde(rename = "io_kbytes")]
    pub io_kbytes: u64,
    #[serde(rename = "bw_bytes")]
    pub bw_bytes: ByteSize,
    pub bw: u64,
    pub iops: f64,
    pub runtime: u64,
    #[serde(rename = "total_ios")]
    pub total_ios: u64,
    #[serde(rename = "short_ios")]
    pub short_ios: u64,
    #[serde(rename = "drop_ios")]
    pub drop_ios: u64,
    #[serde(rename = "slat_ns")]
    pub slat_ns: SlatNs2,
    #[serde(rename = "clat_ns")]
    pub clat_ns: ClatNs2,
    #[serde(rename = "lat_ns")]
    pub lat_ns: LatNs2,
    #[serde(rename = "bw_min")]
    pub bw_min: u64,
    #[serde(rename = "bw_max")]
    pub bw_max: u64,
    #[serde(rename = "bw_agg")]
    pub bw_agg: f64,
    #[serde(rename = "bw_mean")]
    pub bw_mean: f64,
    #[serde(rename = "bw_dev")]
    pub bw_dev: f64,
    #[serde(rename = "bw_samples")]
    pub bw_samples: u64,
    #[serde(rename = "iops_min")]
    pub iops_min: u64,
    #[serde(rename = "iops_max")]
    pub iops_max: u64,
    #[serde(rename = "iops_mean")]
    pub iops_mean: f64,
    #[serde(rename = "iops_stddev")]
    pub iops_stddev: f64,
    #[serde(rename = "iops_samples")]
    pub iops_samples: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlatNs2 {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClatNs2 {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatNs2 {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trim {
    #[serde(rename = "io_bytes")]
    pub io_bytes: ByteSize,
    #[serde(rename = "io_kbytes")]
    pub io_kbytes: u64, // TODO
    #[serde(rename = "bw_bytes")]
    pub bw_bytes: ByteSize,
    pub bw: u64,
    pub iops: f64,
    pub runtime: u64,
    #[serde(rename = "total_ios")]
    pub total_ios: u64,
    #[serde(rename = "short_ios")]
    pub short_ios: u64,
    #[serde(rename = "drop_ios")]
    pub drop_ios: u64,
    #[serde(rename = "slat_ns")]
    pub slat_ns: SlatNs3,
    #[serde(rename = "clat_ns")]
    pub clat_ns: ClatNs3,
    #[serde(rename = "lat_ns")]
    pub lat_ns: LatNs3,
    #[serde(rename = "bw_min")]
    pub bw_min: u64,
    #[serde(rename = "bw_max")]
    pub bw_max: u64,
    #[serde(rename = "bw_agg")]
    pub bw_agg: f64,
    #[serde(rename = "bw_mean")]
    pub bw_mean: f64,
    #[serde(rename = "bw_dev")]
    pub bw_dev: f64,
    #[serde(rename = "bw_samples")]
    pub bw_samples: u64,
    #[serde(rename = "iops_min")]
    pub iops_min: u64,
    #[serde(rename = "iops_max")]
    pub iops_max: u64,
    #[serde(rename = "iops_mean")]
    pub iops_mean: f64,
    #[serde(rename = "iops_stddev")]
    pub iops_stddev: f64,
    #[serde(rename = "iops_samples")]
    pub iops_samples: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SlatNs3 {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClatNs3 {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatNs3 {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sync {
    #[serde(rename = "total_ios")]
    pub total_ios: u64,
    #[serde(rename = "lat_ns")]
    pub lat_ns: LatNs4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatNs4 {
    pub min: u64,
    pub max: u64,
    pub mean: f64,
    pub stddev: f64,
    #[serde(rename = "N")]
    pub n: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IodepthLevel {
    #[serde(rename = "1")]
    pub n1: f64,
    #[serde(rename = "2")]
    pub n2: f64,
    #[serde(rename = "4")]
    pub n4: f64,
    #[serde(rename = "8")]
    pub n8: f64,
    #[serde(rename = "16")]
    pub n16: f64,
    #[serde(rename = "32")]
    pub n32: f64,
    #[serde(rename = ">=64")]
    pub n64: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IodepthSubmit {
    #[serde(rename = "0")]
    pub n0: f64,
    #[serde(rename = "4")]
    pub n4: f64,
    #[serde(rename = "8")]
    pub n8: f64,
    #[serde(rename = "16")]
    pub n16: f64,
    #[serde(rename = "32")]
    pub n32: f64,
    #[serde(rename = "64")]
    pub n64: f64,
    #[serde(rename = ">=64")]
    pub n642: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IodepthComplete {
    #[serde(rename = "0")]
    pub n0: f64,
    #[serde(rename = "4")]
    pub n4: f64,
    #[serde(rename = "8")]
    pub n8: f64,
    #[serde(rename = "16")]
    pub n16: f64,
    #[serde(rename = "32")]
    pub n32: f64,
    #[serde(rename = "64")]
    pub n64: f64,
    #[serde(rename = ">=64")]
    pub n642: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatencyNs {
    #[serde(rename = "2")]
    pub n2: f64,
    #[serde(rename = "4")]
    pub n4: f64,
    #[serde(rename = "10")]
    pub n10: f64,
    #[serde(rename = "20")]
    pub n20: f64,
    #[serde(rename = "50")]
    pub n50: f64,
    #[serde(rename = "100")]
    pub n100: f64,
    #[serde(rename = "250")]
    pub n250: f64,
    #[serde(rename = "500")]
    pub n500: f64,
    #[serde(rename = "750")]
    pub n750: f64,
    #[serde(rename = "1000")]
    pub n1000: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatencyUs {
    #[serde(rename = "2")]
    pub n2: f64,
    #[serde(rename = "4")]
    pub n4: f64,
    #[serde(rename = "10")]
    pub n10: f64,
    #[serde(rename = "20")]
    pub n20: f64,
    #[serde(rename = "50")]
    pub n50: f64,
    #[serde(rename = "100")]
    pub n100: f64,
    #[serde(rename = "250")]
    pub n250: f64,
    #[serde(rename = "500")]
    pub n500: f64,
    #[serde(rename = "750")]
    pub n750: f64,
    #[serde(rename = "1000")]
    pub n1000: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatencyMs {
    #[serde(rename = "2")]
    pub n2: f64,
    #[serde(rename = "4")]
    pub n4: f64,
    #[serde(rename = "10")]
    pub n10: f64,
    #[serde(rename = "20")]
    pub n20: f64,
    #[serde(rename = "50")]
    pub n50: f64,
    #[serde(rename = "100")]
    pub n100: f64,
    #[serde(rename = "250")]
    pub n250: f64,
    #[serde(rename = "500")]
    pub n500: f64,
    #[serde(rename = "750")]
    pub n750: f64,
    #[serde(rename = "1000")]
    pub n1000: f64,
    #[serde(rename = "2000")]
    pub n2000: f64,
    #[serde(rename = ">=2000")]
    pub n20002: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskUtil {
    pub name: String,
    #[serde(rename = "read_ios")]
    pub read_ios: u64,
    #[serde(rename = "write_ios")]
    pub write_ios: u64,
    #[serde(rename = "read_sectors")]
    pub read_sectors: u64,
    #[serde(rename = "write_sectors")]
    pub write_sectors: u64,
    #[serde(rename = "read_merges")]
    pub read_merges: u64,
    #[serde(rename = "write_merges")]
    pub write_merges: u64,
    #[serde(rename = "read_ticks")]
    pub read_ticks: u64,
    #[serde(rename = "write_ticks")]
    pub write_ticks: u64,
    #[serde(rename = "in_queue")]
    pub in_queue: u64,
    pub util: f64,
}

#[derive(Default, Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub enum TestType {
    #[default]
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "trim")]
    Trim,
    #[serde(rename = "randread")]
    RandRead,
    #[serde(rename = "randwrite")]
    RandWrite,
    #[serde(rename = "randtrim")]
    RandTrim,
    #[serde(rename = "rw,readwrite")]
    RwReadWrite,
    #[serde(rename = "randrw")]
    RandRw,
    #[serde(rename = "trimwrite")]
    TrimWrite,
    #[serde(rename = "randtrimwrite")]
    RandTrimWrite,
}

impl Display for TestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestType::Read => write!(f, "read"),
            TestType::Write => write!(f, "write"),
            TestType::Trim => write!(f, "trim"),
            TestType::RandRead => write!(f, "randread"),
            TestType::RandWrite => write!(f, "randwrite"),
            TestType::RandTrim => write!(f, "randtrim"),
            TestType::RwReadWrite => write!(f, "rw_readwrite"),
            TestType::RandRw => write!(f, "randrw"),
            TestType::TrimWrite => write!(f, "trimwrite"),
            TestType::RandTrimWrite => write!(f, "randtrimwrite"),
        }
    }
}
#[derive(Debug, Error)]
pub enum FioResultError {
    #[error("i/o {0}")]
    Io(#[from] std::io::Error),

    #[error("json {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Default, Debug, Clone, Eq, Deserialize, Serialize)]
pub struct BlockSize {
    orig: String,
    bs: ByteSize,
}

#[derive(Debug, Error)]
pub enum ParseBlockSizeError {
    #[error("{0}")]
    Int(#[from] ParseIntError),

    #[error("unknown format")]
    UnknownFormat,
}

impl FromStr for BlockSize {
    type Err = ParseBlockSizeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^([0-9]+)([kKG]?)$").unwrap();
        if let Some((_, [amount, unit])) = re.captures_iter(s).map(|c| c.extract()).next() {
            let a = amount.parse::<u64>()?;
            match unit {
                "K" | "k" => {
                    return Ok(BlockSize {
                        orig: s.to_string(),
                        bs: ByteSize::kb(bytesize::kb(a)),
                    });
                }
                _ => return Err(ParseBlockSizeError::UnknownFormat),
            }
        }

        Err(ParseBlockSizeError::UnknownFormat)
    }
}

impl Display for BlockSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.orig.fmt(f)
    }
}
impl PartialEq for BlockSize {
    fn eq(&self, other: &Self) -> bool {
        self.bs == other.bs
    }
}

impl Ord for BlockSize {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bs.cmp(&other.bs)
    }
}

impl PartialOrd for BlockSize {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_file<P: AsRef<Path>>(filepath: P) -> Result<FioResult, FioResultError> {
    let contents = std::fs::read_to_string(&filepath)?;

    let deserialized: FioResult = serde_json::from_str(&contents)?;

    Ok(deserialized)
}

pub type IoDepth = String;

pub type GroupByTestType = BTreeMap<(TestType, BlockSize), BTreeMap<IoDepth, FioResult>>;
pub type MergeByTestType =
    BTreeMap<(TestType, BlockSize), BTreeMap<IoDepth, (FioResult, FioResult)>>;
