create table feed (
  id serial not null primary key,
  url text not null unique,
  title text not null,
  description text
);

create table entry (
  id serial not null primary key,
  feed_id int not null references feed(id),
  title text not null,
  link text not null,
  comments text,
  visited bool default false,
  unique(feed_id, title),
  unique(feed_id, link)
);

create table entry_filter (
  entry_id int references entry(id) not null primary key
);
