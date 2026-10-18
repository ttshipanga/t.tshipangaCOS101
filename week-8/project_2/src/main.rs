fn main() {
    let candidates = vec![("Tships", 6), ("Somto", 3), ("Mary", 9), ("Sekemi", 12), ("Fikemi", 4)];

    let mut top_candidate = candidates[0];

    for candidate in &candidates {
        if candidate.1 > top_candidate.1 {
           let top_candidates = *candidate; 
        }
    }
    println!("Welcome to Ernst & Young Global Limited Recruitment Program!");
    println!("The developer with the highest programming experience is {} with {} years of experience", top_candidate.0, top_candidate.1);
}
