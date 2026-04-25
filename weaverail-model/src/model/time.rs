use std::ops::{Add, AddAssign};

use serde::{Deserialize, Serialize};

/// 時刻
#[derive(
    ts_rs::TS, Copy, Clone, PartialEq, Default, Serialize, Deserialize, Eq, PartialOrd, Ord,
)]
pub struct Time(u32);

impl Time {
    pub const fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self(hour * 60 * 60 + minute * 60 + second)
    }

    pub const fn new_from_total_second(second: u32) -> Self {
        Self(second)
    }

    /// 0時0分からの累計秒
    pub fn total_second(&self) -> u32 {
        self.0
    }

    /// 時
    pub fn get_hour(&self) -> u32 {
        self.0 / 60 / 60 % 24
    }

    /// 分
    pub fn get_minute(&self) -> u32 {
        self.0 / 60 % 60
    }

    /// 秒
    pub fn get_second(&self) -> u32 {
        self.0 % 60
    }
}
impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}",
            self.get_hour(),
            self.get_minute(),
            self.get_second()
        )?;
        Ok(())
    }
}
impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}",
            self.get_hour(),
            self.get_minute(),
            self.get_second()
        )?;
        Ok(())
    }
}
impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.total_second() + rhs.total_second()
    }
}
impl Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_from_total_second(self.total_second() + rhs.total_second())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::time::Time;

    #[test]
    fn time() {
        assert_eq!(3600, Time::new(1, 0, 0).total_second());
    }
    #[test]
    fn destruct_time() {
        let time = Time::new(1, 2, 3);
        assert_eq!(1, time.get_hour());
        assert_eq!(2, time.get_minute());
        assert_eq!(3, time.get_second());
    }
    #[test]
    fn to_string() {
        let time = Time::new(1, 2, 3);
        assert_eq!("01:02:03", time.to_string());
    }
}
