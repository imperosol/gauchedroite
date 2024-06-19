begin;

create table if not exists input
(
    id      integer primary key autoincrement,
    content text not null unique
);

create unique index if not exists idx_input on input (content);


create table if not exists query
(
    id          integer primary key autoincrement,
    input       integer not null,
    timestamp   text default CURRENT_TIMESTAMP,
    orientation integer not null,  -- 1=gauche  2=droite 3=les deux
    constraint check_orientation
        check (query.orientation in (1, 2, 3)),
    foreign key (input) references input (id)
);

create table if not exists feedback
(
    id                   integer primary key autoincrement,
    input                integer not null,
    original_orientation integer not null,
    agreed               integer not null, -- 1 : agreed ; 0 : disagreed
    constraint check_orientation
        check (feedback.original_orientation in (1, 2, 3)),
    foreign key (input) references input (id)
);

commit;

select i.content, query.orientation, query.timestamp
from query
         inner join input as i on i.id = query.input
order by query.id desc
limit 1;
