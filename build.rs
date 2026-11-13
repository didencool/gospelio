// build.rs
// В майбутньому додати змінні для того щоб мова обиралась в залежності від локалі
fn main() {
    slint_build::compile("src/ui/appwindow.slint").expect("Помилка компіляції Slint UI");
}
