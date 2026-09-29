pub async fn connect(db_url: &str) -> Result<toasty::Db, toasty::Error> {
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(db_url)
        .await?;
    Ok(db)
}