use polars::error::PolarsError;

mod anime_data;

fn main() -> Result<(), PolarsError> {
    println!("Carregando e filtrando notas...");
    let df = anime_data::load_and_filter_notes().unwrap();
    anime_data::filter_first_five_result(&df)?;

    let df_result = anime_data::operation_columns(&df)?;
    println!("Resultado: {}", df_result);

    let df_best_ten = anime_data::advanced_filter(&df_result)?;
    println!("Melhores 10: {}", df_best_ten);

    Ok(())
}
