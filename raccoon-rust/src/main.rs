mod lunarphase;

fn main() {
    println!("Hello, Raccoon Test Leader!");
    let current_time = chrono::Local::now();

    println!(
        "Current date and time: {}",
        current_time.format("%Y-%m-%d %H:%M:%S %Z")
    );

    let lunarphase = lunarphase::Lunarphase::new();
    let current_phase = lunarphase.calculate_phase(chrono::Utc::now());
    println!(
        "Current lunar phase: {} ({}% illumination, day {} of cycle)",
        current_phase.phase_name, current_phase.illumination, current_phase.days_in_cycle
    );
    println!(
        "Night mission assessment: {}",
        lunarphase.assess_night_mission()
    );

    if let Some(full_moon) = lunarphase.find_next_full_moon() {
        println!("Next full moon: {}", full_moon.date.format("%Y-%m-%d"));
    }
}
