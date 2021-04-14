CREATE TABLE peoples(
	id				UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
	"first-name"	TEXT NOT NULL,
	"last-name"		TEXT NOT NULL,
	favorite_color	UUID REFERENCES favorite_color,
	age				INTEGER,
	gender			TEXT,
	twitter			TEXT
);
