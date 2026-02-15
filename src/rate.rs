use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, defmt::Format)]
#[repr(u32)]
pub enum WifiPhyRate {
    Rate1mL = 0,
    Rate2m = 1,
    Rate5mL = 2,
    Rate11mL = 3,
    Rate2mS = 4,
    Rate5mS = 5,
    Rate11mS = 6,
    Rate48m = 7,
    Rate24m = 8,
    Rate12m = 9,
    Rate6m = 10,
    Rate54m = 11,
    Rate36m = 12,
    Rate18m = 13,
    Rate9m = 14,
    RateMcs0Lgi = 15,
    RateMcs1Lgi = 16,
    RateMcs2Lgi = 17,
    RateMcs3Lgi = 18,
    RateMcs4Lgi = 19,
    RateMcs5Lgi = 20,
    RateMcs6Lgi = 21,
    RateMcs7Lgi = 22,
    RateMcs0Sgi = 23,
    RateMcs1Sgi = 24,
    RateMcs2Sgi = 25,
    RateMcs3Sgi = 26,
    RateMcs4Sgi = 27,
    RateMcs5Sgi = 28,
    RateMcs6Sgi = 29,
    RateMcs7Sgi = 30,
    RateLora250k = 31,
    RateLora500k = 32,
    RateMax = 33,
}

impl fmt::Display for WifiPhyRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            WifiPhyRate::Rate1mL => "1mL",
            WifiPhyRate::Rate2m => "2m",
            WifiPhyRate::Rate5mL => "5mL",
            WifiPhyRate::Rate11mL => "11mL",
            WifiPhyRate::Rate2mS => "2mS",
            WifiPhyRate::Rate5mS => "5mS",
            WifiPhyRate::Rate11mS => "11mS",
            WifiPhyRate::Rate48m => "48m",
            WifiPhyRate::Rate24m => "24m",
            WifiPhyRate::Rate12m => "12m",
            WifiPhyRate::Rate6m => "6m",
            WifiPhyRate::Rate54m => "54m",
            WifiPhyRate::Rate36m => "36m",
            WifiPhyRate::Rate18m => "18m",
            WifiPhyRate::Rate9m => "9m",
            WifiPhyRate::RateMcs0Lgi => "Mcs0Lgi",
            WifiPhyRate::RateMcs1Lgi => "Mcs1Lgi",
            WifiPhyRate::RateMcs2Lgi => "Mcs2Lgi",
            WifiPhyRate::RateMcs3Lgi => "Mcs3Lgi",
            WifiPhyRate::RateMcs4Lgi => "Mcs4Lgi",
            WifiPhyRate::RateMcs5Lgi => "Mcs5Lgi",
            WifiPhyRate::RateMcs6Lgi => "Mcs6Lgi",
            WifiPhyRate::RateMcs7Lgi => "Mcs7Lgi",
            WifiPhyRate::RateMcs0Sgi => "Mcs0Sgi",
            WifiPhyRate::RateMcs1Sgi => "Mcs1Sgi",
            WifiPhyRate::RateMcs2Sgi => "Mcs2Sgi",
            WifiPhyRate::RateMcs3Sgi => "Mcs3Sgi",
            WifiPhyRate::RateMcs4Sgi => "Mcs4Sgi",
            WifiPhyRate::RateMcs5Sgi => "Mcs5Sgi",
            WifiPhyRate::RateMcs6Sgi => "Mcs6Sgi",
            WifiPhyRate::RateMcs7Sgi => "Mcs7Sgi",
            WifiPhyRate::RateLora250k => "Lora250k",
            WifiPhyRate::RateLora500k => "Lora500k",
            WifiPhyRate::RateMax => "Max",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseWifiPhyRateError;

impl fmt::Display for ParseWifiPhyRateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse WifiPhyRate")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseWifiPhyRateError {}

#[cfg(not(feature = "std"))]
impl core::error::Error for ParseWifiPhyRateError {}

impl TryFrom<&str> for WifiPhyRate {
    type Error = ParseWifiPhyRateError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "1mL" => Ok(WifiPhyRate::Rate1mL),
            "2m" => Ok(WifiPhyRate::Rate2m),
            "5mL" => Ok(WifiPhyRate::Rate5mL),
            "11mL" => Ok(WifiPhyRate::Rate11mL),
            "2mS" => Ok(WifiPhyRate::Rate2mS),
            "5mS" => Ok(WifiPhyRate::Rate5mS),
            "11mS" => Ok(WifiPhyRate::Rate11mS),
            "48m" => Ok(WifiPhyRate::Rate48m),
            "24m" => Ok(WifiPhyRate::Rate24m),
            "12m" => Ok(WifiPhyRate::Rate12m),
            "6m" => Ok(WifiPhyRate::Rate6m),
            "54m" => Ok(WifiPhyRate::Rate54m),
            "36m" => Ok(WifiPhyRate::Rate36m),
            "18m" => Ok(WifiPhyRate::Rate18m),
            "9m" => Ok(WifiPhyRate::Rate9m),
            "Mcs0Lgi" => Ok(WifiPhyRate::RateMcs0Lgi),
            "Mcs1Lgi" => Ok(WifiPhyRate::RateMcs1Lgi),
            "Mcs2Lgi" => Ok(WifiPhyRate::RateMcs2Lgi),
            "Mcs3Lgi" => Ok(WifiPhyRate::RateMcs3Lgi),
            "Mcs4Lgi" => Ok(WifiPhyRate::RateMcs4Lgi),
            "Mcs5Lgi" => Ok(WifiPhyRate::RateMcs5Lgi),
            "Mcs6Lgi" => Ok(WifiPhyRate::RateMcs6Lgi),
            "Mcs7Lgi" => Ok(WifiPhyRate::RateMcs7Lgi),
            "Mcs0Sgi" => Ok(WifiPhyRate::RateMcs0Sgi),
            "Mcs1Sgi" => Ok(WifiPhyRate::RateMcs1Sgi),
            "Mcs2Sgi" => Ok(WifiPhyRate::RateMcs2Sgi),
            "Mcs3Sgi" => Ok(WifiPhyRate::RateMcs3Sgi),
            "Mcs4Sgi" => Ok(WifiPhyRate::RateMcs4Sgi),
            "Mcs5Sgi" => Ok(WifiPhyRate::RateMcs5Sgi),
            "Mcs6Sgi" => Ok(WifiPhyRate::RateMcs6Sgi),
            "Mcs7Sgi" => Ok(WifiPhyRate::RateMcs7Sgi),
            "Lora250k" => Ok(WifiPhyRate::RateLora250k),
            "Lora500k" => Ok(WifiPhyRate::RateLora500k),
            "Max" => Ok(WifiPhyRate::RateMax),
            _ => Err(ParseWifiPhyRateError),
        }
    }
}

impl FromStr for WifiPhyRate {
    type Err = ParseWifiPhyRateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WifiPhyRate::try_from(s)
    }
}
