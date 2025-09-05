SELECT 
    c.country_name,
       CASE
        WHEN (SUM(w.weather_state)::float / COUNT(*)) <= 15 THEN 'Cold'
        WHEN (SUM(w.weather_state)::float / COUNT(*)) >= 25 THEN 'Hot'
        ELSE 'Warm'
    END AS weather_type
FROM Countries c
LEFT JOIN Weather w
    ON c.country_id = w.country_id
WHERE w.day >= '2019-11-01'::date
  AND w.day <  '2019-12-01'::date
GROUP BY c.country_id, c.country_name
ORDER BY c.country_id;
