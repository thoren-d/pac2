use pac2;

use pac2::PacfileBuilder;

fn main() {
    let mut builder = PacfileBuilder::new();
    builder.add("src/main.rs".to_owned(), "src/main.rs".to_owned());
    builder.add(
        "src/proto/index.rs".to_owned(),
        "src/proto/index.rs".to_owned(),
    );
    builder.sort();
    builder.build("data.p2i").unwrap();
}
