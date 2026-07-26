use std::fs::File;

use polars::{
    df,
    error::PolarsError,
    frame::DataFrame,
    io::SerReader,
    prelude::{CsvReader, IntoLazy, col},
};

#[allow(dead_code)]
pub fn imprimir_dataframe() {
    let df: DataFrame = df!(
        "nome" => ["Dragon Ball", "Naruto", "One Piece", "CDZ"],
        "score" => [9.0, 8.5, 8.0, 7.5]
    )
    .unwrap();

    println!("{:?}", df);
}

pub fn load_and_filter_notes() -> Result<DataFrame, PolarsError> {
    let file_path = "dados/animes_processado.csv";
    let file = File::open(file_path)?;

    let df = CsvReader::new(file).finish()?;
    let filtered_df = df.lazy().filter(col("Score").gt(5)).collect()?;

    Ok(filtered_df)
}

pub fn filter_first_five_result(df: &DataFrame) -> Result<(), PolarsError> {
    let selected = df.select(["Name", "Score"])?;
    println!("Primeiros 5 animes com suas notas");
    println!("{}", selected.head(Some(5))); // Faz retornar os primeiros 5 animes com suas notas

    let score_columns = df.column("Score")?;
    let media = score_columns
        .as_series()
        .map(|s| s.mean())
        .flatten()
        .unwrap_or(0.0);
    println!("Média das notas de animes: {:.2}", media); // exibe até o segundo valor depois da vírgula

    Ok(())
}
