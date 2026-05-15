use std::io::{self, Write};

struct Sensor {
    history: Vec<f32>,
}

impl Sensor {
    fn process(&mut self, raw: f32) -> (f32, f32) {
        let cal = (raw * 1.01) + 0.5; // Kalibrasi langsung
        if cal >= 0.0 && cal <= 100.0 {
            if self.history.len() >= 5 { self.history.remove(0); }
            self.history.push(cal);
        }
        let avg = if self.history.is_empty() { 0.0 } else { self.history.iter().sum::<f32>() / self.history.len() as f32 };
        (cal, avg)
    }
}

struct MonitoringSystem {
    sensor: Sensor,
    count: u32,
}

impl MonitoringSystem {
    fn evaluate(&self, val: f32) -> (&str, &str, &str) {
        if val <= 20.0 { ("[ DANGER KERING ]", "AKTIF", "AKTIF (menambah uap air)") }
        else if val <= 30.0 { ("[ WARNING KERING]", "AKTIF", "AKTIF (menambah uap air)") }
        else if val >= 90.0 { ("[ DANGER LEMBAP ]", "AKTIF", "OFF") } // Sesuai logika: Dehumidifier aktif
        else if val >= 80.0 { ("[ WARNING LEMBAP]", "AKTIF", "OFF") }
        else { ("[ NORMAL        ]", "OFF", "OFF") }
    }

    fn run_cycle(&mut self, raw: f32) {
        self.count += 1;
        let (cal, avg) = self.sensor.process(raw);
        
        println!("\n+------------------------------------------+");
        println!("  Pembacaan ke-{}\n+------------------------------------------+", self.count);
        println!("  Input Raw    : {:.2} %RH\n  Terkalibrasi : {:.2} %RH", raw, cal);

        if cal < 0.0 || cal > 100.0 {
            println!("  Status       : [ SENSOR ERROR ] Nilai di luar rentang!");
            return;
        }

        let (stat, alm, hum) = self.evaluate(cal);
        let deh = if cal >= 80.0 { "AKTIF (menyerap uap air)" } else { "OFF" };

        println!("  Moving Avg   : {:.2} %RH\n  Deviasi      : {:.2} %RH", avg, (cal - avg).abs());
        println!("  Status       : {}\n  Alarm        : {}\n  Humidifier   : {}\n  Dehumidifier : {}", stat, alm, hum, deh);
    }
}

fn input(msg: &str) -> f32 {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap_or(0.0)
}

fn main() {
    println!("+==========================================+\n   SISTEM MONITORING KELEMBAPAN UDARA\n   Teknik Instrumentasi - ITS\n+==========================================+");
    let mut sys = MonitoringSystem { sensor: Sensor { history: vec![] }, count: 0 };
    
    let n = input("Masukkan jumlah siklus pembacaan: ") as u32;
    for i in 1..=n {
        println!("\n--- Siklus {} dari {} ---", i, n);
        let val = input("  Input kelembapan udara raw (%RH): ");
        sys.run_cycle(val);
    }

    println!("\n+==========================================+\n        RINGKASAN AKHIR MONITORING\n+==========================================+");
    println!("  Total Pembacaan : {}\n  Moving Average  : {:.2} %RH", sys.count, sys.sensor.history.iter().sum::<f32>() / sys.sensor.history.len() as f32);
}