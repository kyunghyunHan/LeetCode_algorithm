SELECT MIN(ABS(x - prev_x)) AS shortest
FROM (
    SELECT 
        x,
        LAG(x) OVER (ORDER BY x) AS prev_x
    FROM Point
) t
WHERE prev_x IS NOT NULL;