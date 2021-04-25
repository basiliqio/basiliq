use super::*;

#[basiliq_test(run_migrations)]
async fn nested_sorting(pool: sqlx::PgPool) {
    let state = prepare_basiliq_test(pool).await;
    let request = prepare_basiliq_request(
        Method::GET,
        "/public__peoples?sort=public__articles.public__comments.body",
        Body::empty(),
    );
    let resp = crate::serve::main_service(state.clone(), request)
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    handle_errors(resp, BasiliqErrorId::Ciboulette2PostgresSortingByMultiRel).await;
}
