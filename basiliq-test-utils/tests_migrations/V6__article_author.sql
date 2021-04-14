CREATE TABLE "people-article"(
	id				UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	people_id		UUID NOT NULL REFERENCES peoples ON DELETE CASCADE,
	article_id		UUID NOT NULL REFERENCES articles ON DELETE CASCADE
);
