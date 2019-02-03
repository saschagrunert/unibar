use crate::segment::Segment;
use amethyst::ui::UiText;
use failure::{format_err, Fallible};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};
use uuid::Uuid;

#[derive(Default)]
pub struct Cpu {
    id: String,
    idle: u64,
    non_idle: u64,
}

impl Segment for Cpu {
    type Item = UiText;

    fn new() -> Self {
        Cpu {
            id: Uuid::new_v4().to_string(),
            idle: 0,
            non_idle: 0,
        }
    }

    fn update(&mut self, t: &mut UiText) {
        if let Ok(text) = self.load() {
            t.text = text;
        }
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl Cpu {
    fn load(&mut self) -> Fallible<String> {
        // Calculate the frequency
        let mut frequency = 0.;
        let mut cores = 0;
        for line in BufReader::new(File::open("/proc/cpuinfo")?)
            .lines()
            .scan((), |_, x| x.ok())
        {
            if line.starts_with("cpu MHz") {
                cores += 1;
                let words = line.split(' ');
                let last = words
                    .last()
                    .ok_or_else(|| format_err!("Unable to parse last word"))?;
                frequency += last.parse::<f32>()?;
            }
        }
        frequency = frequency / (cores as f32) / 1000.;

        // Calculate the utilization
        let mut utilization = 0;
        for line in BufReader::new(File::open("/proc/stat")?)
            .lines()
            .scan((), |_, x| x.ok())
        {
            if line.starts_with("cpu ") {
                let data: Vec<u64> = (&line)
                    .split(' ')
                    .collect::<Vec<_>>()
                    .iter()
                    .skip(2)
                    .filter_map(|x| x.parse().ok())
                    .collect();

                if data.len() > 7 {
                    let idle = data[3] + data[4];
                    let non_idle = data[0]
                        + data[1]
                        + data[2]
                        + data[5]
                        + data[6]
                        + data[7];

                    let prev_total = self.idle + self.non_idle;
                    let total = idle + non_idle;

                    let (total_delta, idle_delta) =
                        if prev_total < total && self.idle <= idle {
                            (total - prev_total, idle - self.idle)
                        } else {
                            (1, 1)
                        };

                    utilization = (((total_delta - idle_delta) as f64
                        / total_delta as f64)
                        * 100.) as u64;

                    self.idle = idle;
                    self.non_idle = non_idle;
                }
            }
        }

        Ok(format!(" {:>3}% {:.*}GHz", utilization, 1, frequency))
    }
}
