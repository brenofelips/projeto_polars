use std::fs::File;

use polars::{
    df,
    error::PolarsError,
    frame::DataFrame,
    io::SerReader,
    prelude::{CsvReader, IntoLazy, JoinArgs, SortMultipleOptions, col, lit, when},
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
    let filtered_df = df.lazy().filter(col("Score").gt(5)).collect()?; // faz um gt (greater than)

    Ok(filtered_df)
}

pub fn merge_dataframes(df: &DataFrame) -> Result<DataFrame, PolarsError> {
    let animes = df.clone().lazy();
    let file_path = "dados/animelist2.csv";
    let file = File::open(file_path)?;

    let ratings = CsvReader::new(file).finish()?;
    let joined = animes
        .lazy()
        .join(
            ratings.lazy(),
            [col("ID")],
            [col("anime_id")],
            JoinArgs::default(), // define os argumentos padrão do Join
        )
        .collect()?;

    Ok(joined)
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

pub fn operation_columns(df: &DataFrame) -> Result<DataFrame, PolarsError> {
    let df = df
        .clone()
        .lazy()
        .with_column(
            when(col("Score").gt(lit(8.0)))
                .then(lit("Excelente"))
                .when(col("Score").gt(lit(6.5)))
                .then(lit("Bom"))
                .otherwise(lit("Regular"))
                .alias("Categorias"), // Criar uma nova coluna "Categorias"
        )
        .collect()?; // ?  para extrair o resultado

    let df = df
        .lazy()
        .rename(["MAL_ID", "Name"], ["ID", "Título"], false)
        .collect()?;
    Ok(df)
}

pub fn advanced_filter(df: &DataFrame) -> Result<DataFrame, PolarsError> {
    let df = df.clone().lazy();

    let df_filtered = df
        .filter(col("Genres").str().contains(lit("Action"), false))
        .select([col("Título"), col("Genres"), col("Score")])
        .collect()?;

    let best_ten = df_filtered
        .lazy()
        .sort_by_exprs(
            vec![col("Score")],
            SortMultipleOptions {
                descending: vec![true],
                ..Default::default() // define as demais opções com os valores padrão
            },
        )
        .limit(10) // define o limite de 10 resultados
        .collect()?;

    Ok(best_ten)
}

pub fn aggregation(df: &DataFrame) -> Result<DataFrame, PolarsError> {
    let df = df
        .clone()
        .lazy()
        .group_by([col("Categorias")])
        .agg([
            col("ID").count().alias("Quantidade"),
            col("Score").mean().alias("Média"),
        ])
        .collect()?;
    Ok(df)
}
