select json_agg(feeds) as json
from (select feed.title as title,
             feed.url as url,
             feed.description as description,
             (select json_agg(entries)
              from (select id,
                           title,
                           link,
                           comments,
                           visited
                    from entry
                    left join entry_filter on entry_filter.entry_id = entry.id
                    where entry.feed_id = feed.id and
                          entry_filter.entry_id is null
                    order by entry.id desc) as entries) as entries
      from feed
      order by feed.id) as feeds;
