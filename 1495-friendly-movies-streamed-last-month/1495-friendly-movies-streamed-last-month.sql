SELECT DISTINCT c.title
FROM Content c
JOIN TVProgram p
  ON c.content_id = p.content_id
WHERE c.kids_content = 'Y'
  AND c.content_type = 'Movies'
  AND EXTRACT(MONTH FROM p.program_date) = 6
  AND EXTRACT(YEAR  FROM p.program_date) = 2020;
