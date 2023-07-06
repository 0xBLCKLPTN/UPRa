CREATE TABLE phones (
	id              bigserial     NOT NULL auto_increment,
	phone_number    text          NOT NULL
);

CREATE TABLE comments (
	id		bigserial     NOT NULL auto_increment,
	phone_id        bigserial     NOT NULL,
	commentary      text          NOT NULL
);

