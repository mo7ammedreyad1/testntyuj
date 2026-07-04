use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use rand::Rng;
use sha2::{Sha256, Digest};

// محاكاة لحجم البيانات لكسر الكاش
const SCRATCHPAD_SIZE: usize = 100 * 1024 * 1024; // 100 ميجابايت

fn production_stealth_worker(duration_secs: u64, worker_id: usize, tx: mpsc::Sender<u64>) {
    let start_time = Instant::now();
    let mut actual_hashes: u64 = 0;
    
    let mut rng = rand::thread_rng();
    let mut current_limit = rng.gen_range(40..=85);
    let mut duty_cycle = current_limit as f64 / 100.0;
    let mut last_change = Instant::now();

    // 1. حجز مساحة في الذاكرة لتعطيل الـ L2/L3 Cache
    let mut huge_dataset = vec![0u8; SCRATCHPAD_SIZE];
    rng.fill(&mut huge_dataset[..]);

    let mut seed_data = [0u8; 32];
    rng.fill(&mut seed_data);

    while start_time.elapsed().as_secs() < duration_secs {
        let loop_start = Instant::now();

        // 2. محاكاة مراحل RandomX: تشفير حقيقي مع قفزات عشوائية في الذاكرة
        let mut hasher = Sha256::new();
        hasher.update(&seed_data);
        
        for i in 0..50 {
            let memory_address = (seed_data[i % 32] as usize * 4000) % huge_dataset.len();
            hasher.update(&huge_dataset[memory_address..memory_address + 1]);
        }
        
        let result = hasher.finalize();
        seed_data.copy_from_slice(&result);
        
        actual_hashes += 1;

        // 3. ميكانيكية التحكم في استهلاك المعالج (Throttling)
        let elapsed = loop_start.elapsed().as_micros() as f64;
        if duty_cycle < 1.0 {
            let sleep_duration = elapsed * (1.0 - duty_cycle) / duty_cycle;
            if sleep_duration > 0.0 {
                thread::sleep(Duration::from_micros(sleep_duration as u64));
            }
        }

        // 4. تدوير نسبة الاستهلاك ديناميكياً للتمويه
        if last_change.elapsed().as_secs() > 5 {
            current_limit = rng.gen_range(40..=85);
            duty_cycle = current_limit as f64 / 100.0;
            last_change = Instant::now();
            if worker_id == 0 {
                println!("🔄 [نواة 0] تذبذب طاقة المعالج الفعلي: {}%", current_limit);
            }
        }
    }

    tx.send(actual_hashes).unwrap();
}

fn main() {
    let duration_secs = 60;
    let cores = 2; // عتاد الجيتهاب أكشنز
    
    println!("============================================================");
    println!("⚙️ محاكي بيئة الإنتاج الصارم (Heavy RandomX Simulation)");
    println!("🖥️ عتاد السحابة: {} الأنوية المتاحة | استهلاك متغير: 40% - 85%", cores);
    println!("============================================================");

    let (tx, rx) = mpsc::channel();
    let start_bench = Instant::now();

    for i in 0..cores {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            production_stealth_worker(duration_secs, i, tx_clone);
        });
    }

    drop(tx);

    let mut total_hashes = 0;
    for received in rx {
        total_hashes += received;
    }

    let total_time = start_bench.elapsed().as_secs_f64();
    let hashrate = total_hashes as f64 / total_time;

    println!("\n============================================================");
    println!("🏁 النتيجة النهائية الصافية لبيئة الإنتاج:");
    println!("🔹 إجمالي الهاشات الثقيلة المحسوبة: {}", total_hashes);
    println!("🔹 القوة الواقعية المتوقعة للحساب الواحد: {:.2} H/s", hashrate);
    println!("============================================================");
}
