use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use rand::Rng;

// محاكاة لحجم البيانات المقروءة عشوائياً (الـ Scratchpad)
// RandomX تستخدم حوالي 256KB للـ Dataset لكل Thread كحد أدنى و2GB للـ Dataset بالكامل
const SCRATCHPAD_SIZE: usize = 256 * 1024; // 256 كيلوبايت من البيانات العشوائية

fn cpu_memory_worker(duration_secs: u64, worker_id: usize, tx: mpsc::Sender<u64>) {
    let start_time = Instant::now();
    let mut simulated_hashes: u64 = 0;
    
    let mut rng = rand::thread_rng();
    let mut current_limit = rng.gen_range(40..=85);
    let mut duty_cycle = current_limit as f64 / 100.0;
    let mut last_change = Instant::now();

    // تخصيص مساحة ذاكرة عشوائية مجهدة للمعالج (تفريغ الـ Cache بانتظام)
    let mut memory_buffer = vec![0u8; SCRATCHPAD_SIZE];
    rng.fill(&mut memory_buffer[..]);

    while start_time.elapsed().as_secs() < duration_secs {
        let loop_start = Instant::now();

        // محاكاة سلوك RandomX: قراءة وكتابة عشوائية في الذاكرة (Memory Churning)
        for _ in 0..100 {
            let read_idx = rng.gen_range(0..SCRATCHPAD_SIZE);
            let write_idx = rng.gen_range(0..SCRATCHPAD_SIZE);
            
            // عملية معالجة وحوسبة تعتمد على قيمة قادمة من الذاكرة لتعطيل معالجات الـ Pipeline
            let data_byte = memory_buffer[read_idx];
            let calculated_value = ((data_byte as f64).sqrt() * (read_idx as f64).sin()) as u8;
            
            memory_buffer[write_idx] = memory_buffer[write_idx].wrapping_add(calculated_value);
        }
        simulated_hashes += 1;

        // ميكانيكية التحكم في استهلاك المعالج (Throttling)
        let elapsed = loop_start.elapsed().as_micros() as f64;
        if duty_cycle < 1.0 {
            let sleep_duration = elapsed * (1.0 - duty_cycle) / duty_cycle;
            if sleep_duration > 0.0 {
                thread::sleep(Duration::from_micros(sleep_duration as u64));
            }
        }

        // تغيير طاقة المعالج ديناميكياً وعشوائياً للتمويه
        if last_change.elapsed().as_secs() > 5 {
            current_limit = rng.gen_range(40..=85);
            duty_cycle = current_limit as f64 / 100.0;
            last_change = Instant::now();
            if worker_id == 0 {
                println!("🔄 [نواة 0] تم تحديث الاستهلاك التكتيكي إلى: {}%", current_limit);
            }
        }
    }

    tx.send(simulated_hashes).unwrap();
}

fn main() {
    let duration_secs = 60; // تشغيل الاختبار لمدة دقيقة
    let cores = 2;
    
    println!("============================================================");
    println!("🚀 محاكي إنتاج Ofoq Solutions (النمط المجهد للذاكرة والـ Cache)");
    println!("🖥️ العتاد المتاح: {} Cores | النطاق الديناميكي: 40% - 85%", cores);
    println!("============================================================");

    let (tx, rx) = mpsc::channel();
    let start_bench = Instant::now();

    for i in 0..cores {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            cpu_memory_worker(duration_secs, i, tx_clone);
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
    println!("🏁 نتائج اختبار محاكاة الـ Production الواقعية:");
    println!("🔹 إجمالي دورات الحوسبة المكتملة: {}", total_hashes);
    println!("🔹 متوسط القوة الواقعية الموزعة: {:.2} H/s", hashrate);
    println!("============================================================");
}
