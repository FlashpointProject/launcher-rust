CREATE TABLE IF NOT EXISTS "tag_category" (
	"id"	integer NOT NULL,
	"name"	varchar NOT NULL COLLATE NOCASE,
	"color"	varchar NOT NULL,
	"description"	varchar,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "tag_alias" (
	"id"	integer NOT NULL,
	"tagId"	integer,
	"name"	varchar NOT NULL COLLATE NOCASE,
	CONSTRAINT "FK_c838531770328702eb9e630bf05" FOREIGN KEY("tagId") REFERENCES "tag"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	PRIMARY KEY("id" AUTOINCREMENT),
	CONSTRAINT "UQ_34d6ff6807129b3b193aea26789" UNIQUE("name")
);
CREATE TABLE IF NOT EXISTS "tag" (
	"id"	integer NOT NULL,
	"dateModified"	datetime NOT NULL DEFAULT (datetime('now')),
	"primaryAliasId"	integer,
	"categoryId"	integer,
	"description"	varchar,
	CONSTRAINT "FK_3c002904ab97fb1b4e61e8493cb" FOREIGN KEY("primaryAliasId") REFERENCES "tag_alias"("id") ON DELETE CASCADE ON UPDATE NO ACTION,
	CONSTRAINT "FK_60fbdce32f9ca3b5afce15a9c32" FOREIGN KEY("categoryId") REFERENCES "tag_category"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	PRIMARY KEY("id" AUTOINCREMENT),
	CONSTRAINT "REL_3c002904ab97fb1b4e61e8493c" UNIQUE("primaryAliasId")
);
CREATE TABLE IF NOT EXISTS "additional_app" (
	"id"	varchar NOT NULL,
	"applicationPath"	varchar NOT NULL,
	"autoRunBefore"	boolean NOT NULL,
	"launchCommand"	varchar NOT NULL,
	"name"	varchar NOT NULL COLLATE NOCASE,
	"waitForExit"	boolean NOT NULL,
	"parentGameId"	varchar,
	CONSTRAINT "FK_c174651de0daf9eae7878d06430" FOREIGN KEY("parentGameId") REFERENCES "game"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "playlist_game" (
	"id"	integer NOT NULL,
	"playlistId"	varchar NOT NULL,
	"order"	integer NOT NULL,
	"notes"	varchar NOT NULL,
	"gameId"	varchar,
	CONSTRAINT "FK_38567e9966c4d5776fb82d6fce5" FOREIGN KEY("playlistId") REFERENCES "playlist"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	CONSTRAINT "FK_178854ad80431146589fa44418a" FOREIGN KEY("gameId") REFERENCES "game"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "game_tags_tag" (
	"gameId"	varchar NOT NULL,
	"tagId"	integer NOT NULL,
	CONSTRAINT "FK_d12253f0cbce01f030a9ced11d6" FOREIGN KEY("tagId") REFERENCES "tag"("id") ON DELETE CASCADE ON UPDATE NO ACTION,
	CONSTRAINT "FK_6366e7093c3571f85f1b5ffd4f1" FOREIGN KEY("gameId") REFERENCES "game"("id") ON DELETE CASCADE ON UPDATE NO ACTION,
	PRIMARY KEY("gameId","tagId")
);
CREATE TABLE IF NOT EXISTS "playlist" (
	"id"	varchar NOT NULL,
	"title"	varchar NOT NULL,
	"description"	varchar NOT NULL,
	"author"	varchar NOT NULL,
	"icon"	varchar,
	"library"	varchar NOT NULL,
	"extreme"	boolean NOT NULL DEFAULT (0),
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "source_data" (
	"id"	integer NOT NULL,
	"sourceId"	integer,
	"sha256"	varchar NOT NULL,
	"urlPath"	varchar NOT NULL,
	CONSTRAINT "FK_acb50fae94d956d35c329dae2d7" FOREIGN KEY("sourceId") REFERENCES "source"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "source" (
	"id"	integer NOT NULL,
	"name"	varchar NOT NULL,
	"dateAdded"	datetime NOT NULL,
	"lastUpdated"	datetime NOT NULL,
	"sourceFileUrl"	varchar NOT NULL,
	"baseUrl"	varchar NOT NULL,
	"count"	integer NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "game" (
	"id"	varchar NOT NULL,
	"parentGameId"	varchar,
	"title"	varchar NOT NULL,
	"alternateTitles"	varchar NOT NULL,
	"series"	varchar NOT NULL,
	"developer"	varchar NOT NULL,
	"publisher"	varchar NOT NULL,
	"dateAdded"	datetime NOT NULL,
	"dateModified"	datetime NOT NULL DEFAULT (datetime('now')),
	"platform"	varchar NOT NULL,
	"broken"	boolean NOT NULL,
	"extreme"	boolean NOT NULL,
	"playMode"	varchar NOT NULL,
	"status"	varchar NOT NULL,
	"notes"	varchar NOT NULL,
	"source"	varchar NOT NULL,
	"applicationPath"	varchar NOT NULL,
	"launchCommand"	varchar NOT NULL,
	"releaseDate"	varchar NOT NULL,
	"version"	varchar NOT NULL,
	"originalDescription"	varchar NOT NULL,
	"language"	varchar NOT NULL,
	"library"	varchar NOT NULL,
	"orderTitle"	varchar NOT NULL,
	"activeDataId"	integer,
	"activeDataOnDisk"	boolean NOT NULL DEFAULT (0),
	"tagsStr"	varchar NOT NULL DEFAULT ('') COLLATE NOCASE,
	CONSTRAINT "FK_45a9180069d42ac8231ff11acd0" FOREIGN KEY("parentGameId") REFERENCES "game"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "game_data" (
	"id"	integer NOT NULL,
	"gameId"	varchar,
	"title"	varchar NOT NULL,
	"dateAdded"	datetime NOT NULL,
	"sha256"	varchar NOT NULL,
	"crc32"	integer NOT NULL,
	"presentOnDisk"	boolean NOT NULL DEFAULT (0),
	"path"	varchar,
	"size"	integer NOT NULL,
	"parameters"	varchar,
	CONSTRAINT "FK_8854ee113e5b5d9c43ff9ee1c8b" FOREIGN KEY("gameId") REFERENCES "game"("id") ON DELETE NO ACTION ON UPDATE NO ACTION,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE INDEX IF NOT EXISTS "IDX_34d6ff6807129b3b193aea2678" ON "tag_alias" (
	"name"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_playlist_gameId" ON "playlist_game" (
	"gameId"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_playlist_playlistId" ON "playlist_game" (
	"playlistId"
);
CREATE INDEX IF NOT EXISTS "IDX_6366e7093c3571f85f1b5ffd4f" ON "game_tags_tag" (
	"gameId"
);
CREATE INDEX IF NOT EXISTS "IDX_d12253f0cbce01f030a9ced11d" ON "game_tags_tag" (
	"tagId"
);
CREATE INDEX IF NOT EXISTS "IDX_gameTitle" ON "game" (
	"title"
);
CREATE INDEX IF NOT EXISTS "IDX_total" ON "game" (
	"library",
	"broken",
	"extreme"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_platform" ON "game" (
	"library",
	"platform"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_series" ON "game" (
	"library",
	"series"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_publisher" ON "game" (
	"library",
	"publisher"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_developer" ON "game" (
	"library",
	"developer"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_dateModified" ON "game" (
	"library",
	"dateModified"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_dateAdded" ON "game" (
	"library",
	"dateAdded"
);
CREATE INDEX IF NOT EXISTS "IDX_lookup_title" ON "game" (
	"library",
	"title"
);
CREATE INDEX IF NOT EXISTS "IDX_sourcedata_hash" ON "source_data" (
	"sha256"
);
CREATE INDEX IF NOT EXISTS "IDX_sourcedata_sourceid" ON "source_data" (
	"sourceId"
);