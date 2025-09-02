WITH  t1 AS  (
    SELECT 
        max(salary) as n1
    FROM salaries 
    WHERE department = 'Marketing'
), 

t2 AS (
    SELECT 
        max(salary) AS n2
    FROM salaries 
    WHERE department = 'Engineering'
)

SELECT 
    abs(n1 - n2) AS salary_difference 
FROM t1
CROSS JOIN t2
