use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use rand::Rng;
use sha2::{Sha256, Digest};

// حجم مصفوفة الذاكرة للمحاكاة المحلية (لتوليد إجهاد حقيقي للكاش والرام دون تدمير حاوية الفحص)
const LOCAL_SCRATCHPAD_SIZE: usize = 64 * 1024 * 1024; // 64 ميجابايت لكل نواة

fn local_production_worker(duration_secs: u64, worker_id: usize, tx: mpsc::Sender<u64>) {
    let start_time = Instant::now();
    let mut simulated_hashes: u64 = 0;
    
    let mut rng = rand::thread_rng();
    let mut current_limit = rng.gen_range(40..=85);
    let mut duty_cycle = current_limit as f64 / 100.0;
    let mut last_change = Instant::now();

    // 1. حجز مصفوفة ذاكرة محلية مجهدة لتعطيل التنبؤ التلقائي للمعالج
    let mut local_dataset = vec![0u8; LOCAL_SCRATCHPAD_SIZE];
    rng.fill(&mut local_dataset[..]);

    let mut seed_data = [0u8; 32];
    rng.fill(&mut seed_data);

    // حلقة التشغيل المستمر حتى انتهاء الوقت المحدد
    while start_time.elapsed().as_secs() < duration_secs {
        let loop_start = Instant::now();

        // 2. محاكاة تعقيد RandomX (خلط البيانات العشوائية المتسلسلة)
        let mut hasher = Sha256::new();
        hasher.update(&seed_data);
        
        // قفزات عشوائية مكثفة داخل الذاكرة لمنع المعالج من استخدام خاصية الـ Prefetching الذكية
        for i in 0..120 { // زيادة عدد القفزات لزيادة الثقل البرمجي
            let memory_address = (seed_data[i % 32] as usize * 7321) % local_dataset.len();
            hasher.update(&local_dataset[memory_address..memory_address + 1]);
        }
        
        let result = hasher.finalize();
        seed_data.copy_from_slice(&result); // ربط النواتج ببعضها بالتوالي
        
        simulated_hashes += 1;

        // 3. ميكانيكية التحكم الذكي في استهلاك المعالج (Throttling)
        let elapsed = loop_start.elapsed().as_micros() as f64;
        if duty_cycle < 1.0 {
            let sleep_duration = elapsed * (1.0 - duty_cycle) / duty_cycle;
            if sleep_duration > 0.0 {
                thread::sleep(Duration::from_micros(sleep_duration as u64));
            }
        }

        // 4. تدوير نسبة الاستهلاك عشوائياً كل 5 ثوانٍ للتمويه التام
        if last_change.elapsed().as_secs() > 5 {
            current_limit = rng.gen_range(40..=85);
            duty_cycle = current_limit as f64 / 100.0;
            last_change = Instant::now();
            if worker_id == 0 {
                println!("🔄 [نواة 0] تعديل طاقة المعالج محلياً إلى: {}%", current_limit);
            }
        }
    }

    tx.send(simulated_hashes).unwrap();
}

fn main() {
    let duration_secs = 600; // 600 ثانية تعادل 10 دقائق كاملة للاختبار المستقر
    let cores = 2; // عدد الأنوية المستخدمة في المحاكاة
    
    println!("============================================================");
    println!("⚙️ محاكي Ofoq Solutions المغلق - بيئة الإنتاج الصافية (Offline)");
    println!("🖥️ عتاد التشغيل: {} Cores | النطاق المتغير: 40% - 85%", cores);
    println!("⏳ مدة الفحص المقررة: 10 دقائق متواصلة وبدون أي اتصالات شبكية");
    println!("============================================================");

    let (tx, rx) = mpsc::channel();
    let start_bench = Instant::now();

    for i in 0..cores {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            local_production_worker(duration_secs, i, tx_clone);
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
    println!("🏁 التقرير النهائي النهائي لـ 10 دقائق من الضغط المحلي:");
    println!("🔹 إجمالي دورات الحوسبة المنجزة: {}", total_hashes);
    println!("🔹 متوسط القوة التقديرية للإنتاج: {:.2} H/s", hashrate);
    println!("============================================================");
}
