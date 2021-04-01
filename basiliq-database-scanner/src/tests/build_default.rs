use super::*;

#[ciboulette2postgres_test]
async fn empty_db(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 0);
}
