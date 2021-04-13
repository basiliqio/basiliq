use super::*;

async fn apply_migrations(transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE peoples(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE movies(
			id			UUID PRIMARY KEY,
			title		TEXT NOT NULL
		);
		"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE movies_staff(
			id			UUID PRIMARY KEY,
			role		TEXT NOT NULL,
			person		UUID NOT NULL REFERENCES peoples(id) ON DELETE CASCADE,
			movies		UUID NOT NULL REFERENCES movies(id) ON DELETE CASCADE
		);
	"#
    )
    .execute(&mut *transaction)
    .await
    .unwrap();
}

#[ciboulette2postgres_test]
async fn config_default(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    apply_migrations(&mut transaction).await;
    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    let config = builder.gen_config();
    insta::assert_json_snapshot!(config);
}

#[ciboulette2postgres_test]
async fn merging_some_informations_into_builder(
    mut transaction: sqlx::Transaction<'_, sqlx::Postgres>,
) {
    apply_migrations(&mut transaction).await;
    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let mut builder = BasiliqStoreBuilder::new(raw_table);
    let mut config = builder.gen_config();
    config
        .resources_mut()
        .remove_entry("public__movies")
        .and_then(|(_k, v)| {
            config
                .resources_mut()
                .insert("court_metrage".to_string(), v)
        }); // Rename "public__movies" to court_metrage
    config
        .resources_mut()
        .remove_entry("public__peoples")
        .and_then(|(_k, v)| config.resources_mut().insert("employee".to_string(), v)); // Rename "public__peoples" to "employee"
    config
        .resources_mut()
        .remove_entry("public__movies_staff")
        .and_then(|(_k, v)| config.resources_mut().insert("movies_staff".to_string(), v)); // Rename "public__movies_staff" to "movies_staff"
    config
        .resources_mut()
        .get_mut("court_metrage")
        .and_then(|v| {
            v.relationships_mut()
                .remove_entry("peoples")
                .and_then(|(_k, v2)| {
                    v.relationships_mut()
                        .insert(ArcStr::from("participant"), v2)
                })
        }); // Rename the "peoples" relationships in "court_metrage" to "participant"
    if let Some(v) = config.resources_mut().get_mut("movies_staff") {
        *v.enabled_mut() = false; // Disable direct access to the relationships "movies_staff"
    }
    config.resources_mut().get_mut("employee").and_then(|v| {
        v.relationships_mut().get_mut("movies_staff").map(|v2| {
            *v2.enabled_mut() = false;
        })
    }); // Disable accessing to "movies_staff" table through "employee"
    builder.basiliq_config_merge(&config).unwrap();
    insta::assert_json_snapshot!(builder.config());
}
