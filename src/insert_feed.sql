with new_id as
     (insert into feed
             (url, title, description)
      values ($1,  $2,    $3)
      on conflict(url) do nothing
      returning id)
select id from new_id
  union
select id from feed where url = $1;
