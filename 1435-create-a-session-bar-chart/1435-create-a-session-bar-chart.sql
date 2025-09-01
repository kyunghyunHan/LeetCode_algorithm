WITH bins(bin, ord) AS (
  VALUES ('[0-5>', 1),
         ('[5-10>', 2),
         ('[10-15>', 3),
         ('15 or more', 4)
),
agg AS (
  SELECT CASE
           WHEN duration < 300 THEN '[0-5>'
           WHEN duration < 600 THEN '[5-10>'
           WHEN duration < 900 THEN '[10-15>'
           ELSE '15 or more'
         END AS bin,
         COUNT(*) AS total
  FROM Sessions
  GROUP BY 1
)
SELECT b.bin, COALESCE(a.total, 0) AS total
FROM bins b
LEFT JOIN agg a USING (bin)
ORDER BY b.ord;
