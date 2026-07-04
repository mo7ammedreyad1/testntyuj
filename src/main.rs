use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
use rand::Rng; // ستحتاج لإضافة حزمة rand في الـ Cargo.toml

fn cpu_worker(duration_secs: u64, worker_id: usize, tx: mpsc::Sender<u64>) {
    let start_time = Instant::now();
    let mut hash_count: u64 = 0;
    
    // النطاق المستهدف: عشوائي بين 40% و 85%
    let mut rng = rand::thread_rng();
    let mut current_limit = rng.gen_range(40..=85);
    let mut duty_cycle = current_limit as f64 / 100.0;
    let mut last_change = Instant::now();

    while start_time.elapsed().as_secs() < duration_secs {
        let loop_start = Instant::now();

        // محاكاة عملية معالجة ثقيلة (توليد مصفوفة تشفير عشوائية متكررة)
        let mut context = SHA256::new(); // محاكاة برمجية مبسطة للهاش المكثف
        for i in 0..1000 {
            let _ = (i as f64).sqrt() * (i as f64).sin();
        }
        hash_count += 1;

        // ميكانيكية التحكم في استهلاك المعالج (Throttling)
        let elapsed = loop_start.elapsed().as_micros() as f64;
        if duty_cycle < 1.0 {
            let sleep_duration = elapsed * (1.0 - duty_cycle) / duty_cycle;
            if sleep_duration > 0.0 {
                thread::sleep(Duration::from_micros(sleep_duration as u64));
            }
        }

        // تغيير طاقة المعالج ديناميكياً وعشوائياً كل 5 ثوانٍ للتمويه
        if last_change.elapsed().as_secs() > 5 {
            current_limit = rng.gen_range(40..=85);
            duty_cycle = current_limit as f64 / 100.0;
            last_change = Instant::now();
            if worker_id == 0 {
                println!("🔄 [نواة 0] تم تحديث طاقة المعالج برمجياً إلى: {}%", current_limit);
            }
        }
    }

    tx.send(hash_count).unwrap();
}

fn main() {
    let duration_secs = 60; // سنشغل الاختبار لمدة دقيقة واحدة (60 ثانية) لقياس المتوسط
    let cores = 2; // عتاد الـ GitHub Actions الافتراضي
    
    println!("============================================================");
    println!("🚀 مشغل أداة Ofoq Solutions لاختبار Rust على الأكشفنز");
    println!("🖥️ الأنوية المستهدفة: {} Cores | النمط المتغير: 40% - 85%", cores);
    println!("============================================================");

    let (tx, rx) = mpsc::channel();
    let start_bench = Instant::now();

    // تشغيل العمليات المتوازية بناءً على عدد الأنوية
    for i in 0..cores {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            cpu_worker(duration_secs, i, tx_clone);
        });
    }

    drop(tx); // إغلاق القناة الرئيسية

    let mut total_hashes = 0;
    for received in rx {
        total_hashes += received;
    }

    let total_time = start_bench.elapsed().as_secs_f64();
    let hashrate = total_hashes as f64 / total_time;

    println!("\n============================================================");
    println!("🏁 نتائج اختبار الأداء الفعلي:");
    println!("🔹 إجمالي العمليات المستخرجة: {}", total_hashes);
    println!("🔹 متوسط قوة الهاش الفعّالة للشبكة: {:.2} H/s", hashrate);
    println!("============================================================");
}

// هيكل وهمي سريع لمحاكاة الـ SHA داخل الكود بدون مكتبات خارجية معقدة في أول دقيقة
struct SHA256;
impl SHA256 {
    fn new() -> Self { SHA256 }
}
