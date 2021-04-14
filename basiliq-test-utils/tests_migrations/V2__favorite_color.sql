CREATE TABLE favorite_color(
	id			UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	color		TEXT NOT NULL
);
