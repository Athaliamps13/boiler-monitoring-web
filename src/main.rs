use std::{
    thread,
    time::Duration,
};

// ======================
// STRUCT SENSOR
// ======================
struct Sensor {
    temperature: f32,
}

impl Sensor {

    // Membaca suhu boiler
    fn read_temperature(&self) -> f32 {
        self.temperature
    }
}

// ======================
// STRUCT CONTROLLER
// ======================
struct Controller {
    alarm: bool,
}

impl Controller {

    // Mengecek kondisi suhu boiler
    fn check_temperature(&mut self, temp: f32) {

        println!("\n--------------------------------------");
        println!(" SUHU BOILER : {:.1} °C", temp);

        // Suhu terlalu rendah
        if temp < 150.0 {

            self.alarm = true;

            println!(" STATUS      : SUHU TERLALU RENDAH");
            println!(" KETERANGAN  : Boiler kurang panas");
            println!(" SARAN       : Aktifkan sistem pemanas");

            println!("\n ALARM SUHU RENDAH AKTIF !!!");
            println!(" BEEP! BEEP!");

            // Pemanas otomatis
            if temp <= 120.0 {

                println!("\n SISTEM PEMANAS DIAKTIFKAN...");
                println!(" SUHU BERHASIL DINAIKKAN");
            }
        }

        // Suhu normal
        else if temp >= 150.0 && temp <= 180.0 {

            self.alarm = false;

            println!(" STATUS      : NORMAL");
            println!(" KETERANGAN  : Boiler bekerja stabil");
        }

        // Warning
        else if temp > 180.0 && temp <= 200.0 {

            self.alarm = false;

            println!(" STATUS      : WARNING");
            println!(" KETERANGAN  : Suhu mendekati batas aman");
            println!(" SARAN       : Kurangi intensitas pembakaran");
        }

        // Overheat
        else {

            self.alarm = true;

            println!(" STATUS      : OVERHEAT !!!");
            println!(" KETERANGAN  : Suhu boiler melebihi batas aman");
            println!(" SARAN       : Aktifkan sistem pendingin");

            println!("\n ALARM OVERHEAT AKTIF !!!");
            println!(" BEEP! BEEP! BEEP!");

            // Pendingin otomatis
            if temp >= 220.0 {

                println!("\n SISTEM PENDINGIN DIAKTIFKAN...");
                println!(" SUHU BERHASIL DISTABILKAN");
            }
        }
    }
}

// ======================
// STRUCT MONITORING SYSTEM
// ======================
struct MonitoringSystem {
    sensor: Sensor,
    controller: Controller,
}

impl MonitoringSystem {

    // Menjalankan sistem monitoring
    fn run(&mut self) {

        let mut naik = true;

        loop {

            // Simulasi suhu naik dan turun
            if naik {
                self.sensor.temperature += 10.0;
            }

            else {
                self.sensor.temperature -= 15.0;
            }

            // Jika terlalu panas
            if self.sensor.temperature >= 220.0 {
                naik = false;
            }

            // Jika terlalu dingin
            if self.sensor.temperature <= 120.0 {
                naik = true;
            }

            // Membaca suhu sensor
            let temp = self.sensor.read_temperature();

            // Mengecek kondisi boiler
            self.controller.check_temperature(temp);

            // Delay monitoring
            thread::sleep(Duration::from_secs(2));
        }
    }
}

// ======================
// MAIN PROGRAM
// ======================
fn main() {

    println!("==========================================");
    println!(" SISTEM MONITORING SUHU BOILER INDUSTRI ");
    println!("==========================================");

    // Object sensor
    let sensor = Sensor {
        temperature: 160.0,
    };

    // Object controller
    let controller = Controller {
        alarm: false,
    };

    // Object monitoring system
    let mut system = MonitoringSystem {
        sensor,
        controller,
    };

    // Menjalankan sistem
    system.run();
}