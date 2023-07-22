struct Row {
    language: String,
    message: String,
}

fn get_rows() -> Vec<Row> {
    vec![
        Row {
            language: "English".to_string(),
            message: "Hello".to_string(),
        },
        Row {
            language: "French".to_string(),
            message: "Bonjour".to_string(),
        },
        Row {
            language: "Spanish".to_string(),
            message: "Hola".to_string(),
        },
        Row {
            language: "Russian".to_string(),
            message: "Zdravstvuyte".to_string(),
        },
        Row {
            language: "Chinese".to_string(),
            message: "Nǐn hǎo".to_string(),
        },
        Row {
            language: "Italian".to_string(),
            message: "Salve".to_string(),
        },
        Row {
            language: "Japanese".to_string(),
            message: "Konnichiwa".to_string(),
        },
        Row {
            language: "German".to_string(),
            message: "Guten Tag".to_string(),
        },
        Row {
            language: "Portuguese".to_string(),
            message: "Olá".to_string(),
        },
        Row {
            language: "Korean".to_string(),
            message: "Anyoung haseyo".to_string(),
        },
        Row {
            language: "Arabic".to_string(),
            message: "Asalaam alaikum".to_string(),
        },
        Row {
            language: "Danish".to_string(),
            message: "Goddag".to_string(),
        },
        Row {
            language: "Swahili".to_string(),
            message: "Shikamoo".to_string(),
        },
        Row {
            language: "Dutch".to_string(),
            message: "Goedendag".to_string(),
        },
        Row {
            language: "Greek".to_string(),
            message: "Yassas".to_string(),
        },
        Row {
            language: "Polish".to_string(),
            message: "Dzień dobry".to_string(),
        },
        Row {
            language: "Indonesian".to_string(),
            message: "Selamat siang".to_string(),
        },
        Row {
            language: "Hindi".to_string(),
            message: "Namaste, Namaskar".to_string(),
        },
        Row {
            language: "Norwegian".to_string(),
            message: "God dag".to_string(),
        },
        Row {
            language: "Turkish".to_string(),
            message: "Merhaba".to_string(),
        },
        Row {
            language: "Hebrew".to_string(),
            message: "Shalom".to_string(),
        },
        Row {
            language: "Swedish".to_string(),
            message: "God dag".to_string(),
        },
    ]
}

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    let now = std::time::Instant::now();
    let rows = get_rows();
    for row in rows.iter() {
        if row.language == "French" {
            println!("{}", row.message);
            break;
        }
    }
    println!("Elapsed: {} nanos", now.elapsed().as_nanos());

    let now = std::time::Instant::now();
    rows.iter()
        .filter(|r| r.language == "French")
        .for_each(|r| println!("{}", r.message));
    println!("Elapsed: {} nanos", now.elapsed().as_nanos());

    // Working with primes
    let now = std::time::Instant::now();
    const MAX:u32 = 200000;
    let mut count = 0;
    for n in 2 .. MAX {
        if is_prime(n) {
            count+=1;
        }
    }
    println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());

    // Iterator for primes
    let now = std::time::Instant::now();
    let count = (2..MAX)
        .filter(|n| is_prime(*n))
        .count();
    println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());

    // Parallel Iterator for primes
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};
    let now = std::time::Instant::now();
    let count = (2..MAX)
        .into_par_iter()
        .filter(|n| is_prime(*n))
        .count();
    println!("Found {count} primes in {:.2} seconds", now.elapsed().as_secs_f32());
}
