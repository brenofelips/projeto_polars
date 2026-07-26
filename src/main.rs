use polars::error::PolarsError;

mod anime_data;

fn main() -> Result<(), PolarsError> {
    println!("Carregando e filtrando notas...");
    let df = anime_data::load_and_filter_notes().unwrap();
    anime_data::filter_first_five_result(&df)?;

    Ok(())
}
