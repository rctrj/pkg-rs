use anyhow::Result;
use proc_macros_rs::DisplayUsingDebug;

#[derive(Clone, Debug, DisplayUsingDebug)]
pub struct UserAgent {
    pub name: String,
    pub version: String,
    pub os: String,
    pub os_version: String,
    pub device: String,
    pub value: String,
}

pub fn parse(value: &str) -> Result<UserAgent> {
    let ua: fast_uaparser::UserAgent = value.parse()?;
    let os: fast_uaparser::OperatingSystem = value.parse()?;
    let device: fast_uaparser::Device = value.parse()?;

    Ok(UserAgent {
        name: ua.family,
        version: extract_version(&ua.version),
        os: os.family,
        os_version: extract_version(&os.version),
        device: device.family,
        value: value.to_string(),
    })
}

fn extract_version(version: &fast_uaparser::Version) -> String {
    let mut out: String = "".to_string();

    macro_rules! append_or_return {
        ($part: expr, $prefix: literal) => {
            if let Some(v) = $part.clone() {
                out.push_str($prefix);
                out.push_str(v.as_str());
            } else {
                return out;
            }
        };
    }

    append_or_return!(version.major, "");
    append_or_return!(version.minor, ".");
    append_or_return!(version.minor, ".");

    out
}
