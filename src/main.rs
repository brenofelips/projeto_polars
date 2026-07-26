use polars::{df, frame::DataFrame};

fn main() {
    let df: DataFrame = df!(
        "nome" => ["Dragon Ball", "Naruto", "One Piece", "CDZ"],
        "score" => [9.0, 8.5, 8.0, 7.5]
    )
    .unwrap();

    println!("{:?}", df);
}
