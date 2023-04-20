use std::cmp::min;
use std::io::{self, Write};

pub struct ProgressBar {
    bar_length: usize,
    total: Option<usize>,
    multiplier: f64,
}

impl ProgressBar {
    pub fn new(bar_length: usize) -> Self {
        ProgressBar {
            bar_length,
            total: None,
            multiplier: 0.0,
        }
    }

    pub fn set_total(&mut self, total: usize) {
        if self.total.is_some() {
            panic!("total already set");
        }

        self.total = Some(total);
        self.multiplier = self.bar_length as f64 / total as f64;
    }

    pub fn update(&self, current: usize, msg: &str) {
        if self.total.is_none() {
            panic!("total not set");
        }

        let length = min(
            (self.multiplier * current as f64).ceil() as usize,
            self.bar_length,
        );
        let progress_str = format!(
            "\r\t[{}{}]{}",
            "#".repeat(length),
            " ".repeat(self.bar_length - length),
            msg
        );
        let percent = current * 100 / self.total.unwrap();
        let _ = io::stdout().write(format!("\r{}% {}", percent, progress_str).as_bytes());
        let _ = io::stdout().flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut progress_bar = ProgressBar::new(30);
        progress_bar.set_total(100);

        for i in 0..=100 {
            progress_bar.update(i, " Wait");
            std::thread::sleep(std::time::Duration::from_millis(15));
        }

        println!("\nDone!");
    }
}
