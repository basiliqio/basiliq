use basiliq_test_utils::*;
use ciboulette2postgres_test_proc_macro::ciboulette2postgres_test;

#[ciboulette2postgres_test]
async fn toto(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    assert_eq!(true, true);
}
