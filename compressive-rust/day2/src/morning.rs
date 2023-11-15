struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Notice how the `self` parameter
// changes type to indicate the method's required level of ownership
// over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library { books: vec![] }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.len() == 0
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn print_books(&self) {
        for book in self.books.iter() {
            println!("{}, published in {}", book.title, book.year)
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;

        for book in self.books.iter() {
            if oldest.is_none() || book.year < oldest.unwrap().year {
                oldest = Some(book);
            }
        }

        oldest
    }
}

pub fn subject_1() {
    let mut library = Library::new();

    println!(
        "The library is empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!(
        "The library is no longer empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}

pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

#[allow(unused_variables, dead_code)]
pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

#[allow(unused_variables, dead_code)]
impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        User {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_pressure: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn doctor_visits(&self) -> u32 {
        self.visit_count as u32
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    pub fn set_height(&mut self, new_height: f32) {
        self.height = new_height
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        self.visit_count += 1;
        let bp = measurements.blood_pressure;
        let report = HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            height_change: self.height - measurements.height,
            blood_pressure_change: match self.last_blood_pressure {
                Some(lbp) => Some((bp.0 as i32 - lbp.0 as i32, bp.1 as i32 - lbp.1 as i32)),
                None => None,
            },
        };
        self.height = measurements.height;
        self.last_blood_pressure = Some(bp);

        report
    }
}

#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });

    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}
