use std::time::Duration;

pub enum ScheduleInterval {
    Seconds(u32),
    Minutes(u32),
    Hours(u32),
    Days(u32),
    Months(u32),
    Years(u32),
}

pub struct TaskSchedule {
    schedule: ScheduleInterval,
}

impl TaskSchedule {
    pub fn to_seconds(&self) -> Duration {
        match self.schedule {
            ScheduleInterval::Seconds(t) => Duration::from_secs(t.into()),
            ScheduleInterval::Minutes(t) => Duration::from_secs((t * 60).into()),
            ScheduleInterval::Hours(t) => Duration::from_secs((t * 60 * 60).into()),
            ScheduleInterval::Days(t) => Duration::from_secs((t * 24 * 60 * 60).into()),
            ScheduleInterval::Months(t) => Duration::from_secs((t * 30 * 24 * 60 * 60).into()),
            ScheduleInterval::Years(t) => Duration::from_secs((t * 365 * 24 * 60 * 60).into()),
        }
    }
}

#[derive(Debug)]
pub struct InvalidFormatError(String);

impl TryFrom<&str> for TaskSchedule {
    type Error = InvalidFormatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let idx = value
            .find(|c: char| !c.is_ascii_digit())
            .ok_or(InvalidFormatError("invalid schedule format".into()))?;
        let interval = value[..idx]
            .parse::<u32>()
            .map_err(|_| InvalidFormatError("unable to parse interval digit".into()))?;
        let unit: ScheduleInterval = match &value[idx..] {
            "s" => ScheduleInterval::Seconds(interval),
            "m" => ScheduleInterval::Minutes(interval),
            "h" => ScheduleInterval::Hours(interval),
            "d" => ScheduleInterval::Days(interval),
            "mt" => ScheduleInterval::Months(interval),
            "y" => ScheduleInterval::Years(interval),
            _ => ScheduleInterval::Minutes(15),
        };
        Ok(Self { schedule: unit })
    }
}
