mod morning;
mod morning_test;
mod afternoon;
mod afternoon_test;

fn main() {
    morning::subject_1();

    #[allow(unused)]
    let ret = afternoon::subject_1();
}
