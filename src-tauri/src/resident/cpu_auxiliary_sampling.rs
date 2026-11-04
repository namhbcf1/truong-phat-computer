//! CPU details and auxiliary sensors have one bounded owner, independent of utilization.
use super::sampling_schedule::Demand;
use truongphatcomputer_platform::system_resources::cpu::details::{CpuDetails, CpuDetailsReader};

pub struct Observation {
    pub(super) battery_present: Option<bool>,
    pub(super) fan_speed: Option<
        truongphatcomputer_platform::PlatformResult<
            truongphatcomputer_platform::system_resources::fans::FanSample,
        >,
    >,
    pub(super) system_power: Option<
        truongphatcomputer_platform::PlatformResult<
            truongphatcomputer_platform::system_resources::power::SystemPower,
        >,
    >,
    pub(super) battery: Option<
        truongphatcomputer_platform::PlatformResult<
            truongphatcomputer_platform::system_resources::battery::BatterySample,
        >,
    >,
    pub(super) details: CpuDetails,
}

#[derive(Default)]
pub struct Reader {
    battery_presence: truongphatcomputer_platform::system_resources::battery::BatteryPresenceReader,
    power: truongphatcomputer_platform::system_resources::power::SystemPowerReader,
    fans: Option<Box<truongphatcomputer_platform::system_resources::fans::FanReader>>,
    battery: truongphatcomputer_platform::system_resources::battery::BatteryReader,
    details: Box<CpuDetailsReader>,
}

impl Reader {
    pub fn read(&mut self, demand: &Demand) -> Observation {
        let Self {
            battery_presence,
            power,
            fans,
            battery,
            details,
        } = self;
        let details = details.read(demand.detailed, demand.temperature);
        // Read battery last so a slow CPU detail query cannot age its timestamp.
        let battery_present = battery_presence.read();
        let battery_sample = if demand.battery && battery_present != Some(false) {
            Some(battery.read())
        } else {
            *battery = Default::default();
            None
        };
        Observation {
            battery_present,
            fan_speed: if demand.fan_speed {
                Some(fans.get_or_insert_with(Box::default).read())
            } else {
                *fans = None;
                None
            },
            system_power: if demand.system_power {
                Some(power.read())
            } else {
                *power = Default::default();
                None
            },
            battery: battery_sample,
            details,
        }
    }
}
