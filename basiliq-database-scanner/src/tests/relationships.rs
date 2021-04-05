use super::*;

#[ciboulette2postgres_test]
async fn one_to_many(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE director(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE movies(
			id			UUID PRIMARY KEY,
			title		TEXT NOT NULL,
			director	UUID NOT NULL REFERENCES director(id) ON DELETE CASCADE
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 2);
    let director_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "director"))
        .unwrap();
    let movies_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "movies"))
        .unwrap();
    let director_movie_rel = director_table.relationships().get("movies").unwrap();
    let movie_director_rel = movies_table.relationships().get("director").unwrap();

    assert_eq!(director_movie_rel.ftable_name().table_name(), "movies");
    assert_eq!(director_movie_rel.ftable_name().schema_name(), "public");
    assert_eq!(director_movie_rel.lfield_name(), "id");
    assert_eq!(director_movie_rel.ffield_name(), "director");
    assert_eq!(
        matches!(
            director_movie_rel.type_(),
            BasiliqStoreRelationshipType::OneToMany(_)
        ),
        true
    );

    assert_eq!(movie_director_rel.ftable_name().table_name(), "director");
    assert_eq!(movie_director_rel.ftable_name().schema_name(), "public");
    assert_eq!(movie_director_rel.ffield_name(), "id");
    assert_eq!(movie_director_rel.lfield_name(), "director");
    assert_eq!(
        matches!(
            movie_director_rel.type_(),
            BasiliqStoreRelationshipType::ManyToOne(_)
        ),
        true
    );
}

#[ciboulette2postgres_test]
async fn multi_one_to_many(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE peoples(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE movies(
			id			UUID PRIMARY KEY,
			title		TEXT NOT NULL,
			director	UUID NOT NULL REFERENCES peoples(id) ON DELETE CASCADE,
			publisher	UUID NOT NULL REFERENCES peoples(id) ON DELETE CASCADE
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 2);
    let director_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "peoples"))
        .unwrap();
    let movies_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "movies"))
        .unwrap();
    let director_movie_rel = director_table.relationships().get("movies").unwrap();
    let movie_director_rel = movies_table.relationships().get("peoples").unwrap();
    let movie_publisher_rel = movies_table.relationships().get("peoples_0").unwrap();

    assert_eq!(director_movie_rel.ftable_name().table_name(), "movies");
    assert_eq!(director_movie_rel.ftable_name().schema_name(), "public");
    assert_eq!(director_movie_rel.lfield_name(), "id");
    assert_eq!(director_movie_rel.ffield_name(), "director");
    assert_eq!(
        matches!(
            director_movie_rel.type_(),
            BasiliqStoreRelationshipType::OneToMany(_)
        ),
        true
    );

    assert_eq!(movie_director_rel.ftable_name().table_name(), "peoples");
    assert_eq!(movie_director_rel.ftable_name().schema_name(), "public");
    assert_eq!(movie_director_rel.ffield_name(), "id");
    assert_eq!(movie_director_rel.lfield_name(), "director");
    assert_eq!(
        matches!(
            movie_director_rel.type_(),
            BasiliqStoreRelationshipType::ManyToOne(_)
        ),
        true
    );

    assert_eq!(movie_publisher_rel.ftable_name().table_name(), "peoples");
    assert_eq!(movie_publisher_rel.ftable_name().schema_name(), "public");
    assert_eq!(movie_publisher_rel.ffield_name(), "id");
    assert_eq!(movie_publisher_rel.lfield_name(), "publisher");
    assert_eq!(
        matches!(
            movie_publisher_rel.type_(),
            BasiliqStoreRelationshipType::ManyToOne(_)
        ),
        true
    );
}

#[ciboulette2postgres_test]
async fn many_to_many(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE peoples(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut transaction)
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
    .execute(&mut transaction)
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
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 3);
    let staff_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "peoples"))
        .unwrap();
    let movies_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "movies"))
        .unwrap();
    let staff_movie_rel = staff_table.relationships().get("movies").unwrap();
    let staff_movie_staf_rel = staff_table.relationships().get("movies_staff").unwrap();
    let movie_staff_rel = movies_table.relationships().get("peoples").unwrap();
    let movie_movie_staff_rel = movies_table.relationships().get("movies_staff").unwrap();
    assert_eq!(staff_movie_rel.ftable_name().schema_name(), "public");
    assert_eq!(staff_movie_rel.ftable_name().table_name(), "movies");
    assert_eq!(staff_movie_rel.lfield_name(), "id");
    assert_eq!(staff_movie_rel.ffield_name(), "id");
    assert_eq!(
        matches!(
            staff_movie_rel.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = staff_movie_rel.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "person");
        assert_eq!(data.ffield_name(), "movies");
    }

    assert_eq!(movie_staff_rel.ftable_name().schema_name(), "public");
    assert_eq!(movie_staff_rel.ftable_name().table_name(), "peoples");
    assert_eq!(movie_staff_rel.ffield_name(), "id");
    assert_eq!(movie_staff_rel.lfield_name(), "id");
    assert_eq!(
        matches!(
            movie_staff_rel.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = movie_staff_rel.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "movies");
        assert_eq!(data.ffield_name(), "person");
    }

    assert_eq!(staff_movie_staf_rel.ftable_name().schema_name(), "public");
    assert_eq!(
        staff_movie_staf_rel.ftable_name().table_name(),
        "movies_staff"
    );
    assert_eq!(staff_movie_staf_rel.lfield_name(), "id");
    assert_eq!(staff_movie_staf_rel.ffield_name(), "person");
    assert_eq!(
        matches!(
            staff_movie_staf_rel.type_(),
            BasiliqStoreRelationshipType::OneToMany(_)
        ),
        true
    );

    assert_eq!(movie_movie_staff_rel.ftable_name().schema_name(), "public");
    assert_eq!(
        movie_movie_staff_rel.ftable_name().table_name(),
        "movies_staff"
    );
    assert_eq!(movie_movie_staff_rel.lfield_name(), "id");
    assert_eq!(movie_movie_staff_rel.ffield_name(), "movies");
    assert_eq!(
        matches!(
            movie_movie_staff_rel.type_(),
            BasiliqStoreRelationshipType::OneToMany(_)
        ),
        true
    );
}

#[ciboulette2postgres_test]
async fn many_many_to_many_many(mut transaction: sqlx::Transaction<'_, sqlx::Postgres>) {
    sqlx::query!(
        r#"
		CREATE TABLE peoples(
			id			UUID PRIMARY KEY,
			name		TEXT NOT NULL
		);
	"#
    )
    .execute(&mut transaction)
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
    .execute(&mut transaction)
    .await
    .unwrap();
    sqlx::query!(
        r#"
		CREATE TABLE movies_staff(
			id				UUID PRIMARY KEY,
			role			TEXT NOT NULL,
			person			UUID NOT NULL REFERENCES peoples(id) ON DELETE CASCADE,
			backup_person	UUID NOT NULL REFERENCES peoples(id) ON DELETE CASCADE,
			movies			UUID NOT NULL REFERENCES movies(id) ON DELETE CASCADE,
			making_of		UUID NOT NULL REFERENCES movies(id) ON DELETE CASCADE
		);
	"#
    )
    .execute(&mut transaction)
    .await
    .unwrap();

    let raw_table = BasiliqDbScannedTable::scan_db(&mut transaction)
        .await
        .unwrap();
    let builder = BasiliqStoreBuilder::new(raw_table);
    assert_eq!(builder.tables().len(), 3);
    let staff_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "peoples"))
        .unwrap();
    let movies_table = builder
        .get_table(&BasiliqStoreTableIdentifier::new("public", "movies"))
        .unwrap();
    let staff_movie_rel = staff_table.relationships().get("movies").unwrap();
    let staff_movie_rel1 = staff_table.relationships().get("movies_0").unwrap();
    let staff_movie_rel2 = staff_table.relationships().get("movies_1").unwrap();
    let staff_movie_rel3 = staff_table.relationships().get("movies_2").unwrap();
    let movie_staff_rel = movies_table.relationships().get("peoples").unwrap();
    let movie_staff_rel1 = movies_table.relationships().get("peoples_0").unwrap();
    let movie_staff_rel2 = movies_table.relationships().get("peoples_1").unwrap();
    let movie_staff_rel3 = movies_table.relationships().get("peoples_2").unwrap();

    assert_eq!(staff_movie_rel.ftable_name().schema_name(), "public");
    assert_eq!(staff_movie_rel.ftable_name().table_name(), "movies");
    assert_eq!(staff_movie_rel.lfield_name(), "id");
    assert_eq!(staff_movie_rel.ffield_name(), "id");
    assert_eq!(
        matches!(
            staff_movie_rel.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = staff_movie_rel.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "backup_person");
        assert_eq!(data.ffield_name(), "making_of");
    }

    assert_eq!(staff_movie_rel1.ftable_name().schema_name(), "public");
    assert_eq!(staff_movie_rel1.ftable_name().table_name(), "movies");
    assert_eq!(staff_movie_rel1.lfield_name(), "id");
    assert_eq!(staff_movie_rel1.ffield_name(), "id");
    assert_eq!(
        matches!(
            staff_movie_rel1.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = staff_movie_rel1.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "backup_person");
        assert_eq!(data.ffield_name(), "movies");
    }

    assert_eq!(staff_movie_rel2.ftable_name().schema_name(), "public");
    assert_eq!(staff_movie_rel2.ftable_name().table_name(), "movies");
    assert_eq!(staff_movie_rel2.lfield_name(), "id");
    assert_eq!(staff_movie_rel2.ffield_name(), "id");
    assert_eq!(
        matches!(
            staff_movie_rel2.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = staff_movie_rel2.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "person");
        assert_eq!(data.ffield_name(), "making_of");
    }

    assert_eq!(staff_movie_rel3.ftable_name().schema_name(), "public");
    assert_eq!(staff_movie_rel3.ftable_name().table_name(), "movies");
    assert_eq!(staff_movie_rel3.lfield_name(), "id");
    assert_eq!(staff_movie_rel3.ffield_name(), "id");
    assert_eq!(
        matches!(
            staff_movie_rel3.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = staff_movie_rel3.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "person");
        assert_eq!(data.ffield_name(), "movies");
    }

    assert_eq!(movie_staff_rel.ftable_name().schema_name(), "public");
    assert_eq!(movie_staff_rel.ftable_name().table_name(), "peoples");
    assert_eq!(movie_staff_rel.lfield_name(), "id");
    assert_eq!(movie_staff_rel.ffield_name(), "id");
    assert_eq!(
        matches!(
            movie_staff_rel.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = movie_staff_rel.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "making_of");
        assert_eq!(data.ffield_name(), "backup_person");
    }

    assert_eq!(movie_staff_rel1.ftable_name().schema_name(), "public");
    assert_eq!(movie_staff_rel1.ftable_name().table_name(), "peoples");
    assert_eq!(movie_staff_rel1.lfield_name(), "id");
    assert_eq!(movie_staff_rel1.ffield_name(), "id");
    assert_eq!(
        matches!(
            movie_staff_rel1.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = movie_staff_rel1.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "making_of");
        assert_eq!(data.ffield_name(), "person");
    }

    assert_eq!(movie_staff_rel2.ftable_name().schema_name(), "public");
    assert_eq!(movie_staff_rel2.ftable_name().table_name(), "peoples");
    assert_eq!(movie_staff_rel2.lfield_name(), "id");
    assert_eq!(movie_staff_rel2.ffield_name(), "id");
    assert_eq!(
        matches!(
            movie_staff_rel2.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = movie_staff_rel2.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "movies");
        assert_eq!(data.ffield_name(), "backup_person");
    }

    assert_eq!(movie_staff_rel3.ftable_name().schema_name(), "public");
    assert_eq!(movie_staff_rel3.ftable_name().table_name(), "peoples");
    assert_eq!(movie_staff_rel3.lfield_name(), "id");
    assert_eq!(movie_staff_rel3.ffield_name(), "id");
    assert_eq!(
        matches!(
            movie_staff_rel3.type_(),
            BasiliqStoreRelationshipType::ManyToMany(_)
        ),
        true
    );
    if let BasiliqStoreRelationshipType::ManyToMany(data) = movie_staff_rel3.type_() {
        assert_eq!(data.bucket().schema_name(), "public");
        assert_eq!(data.bucket().table_name(), "movies_staff");
        assert_eq!(data.lfield_name(), "movies");
        assert_eq!(data.ffield_name(), "person");
    }
}
