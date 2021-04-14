CREATE TABLE articles(
	id			UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	title		TEXT NOT NULL,
	body		TEXT
);
