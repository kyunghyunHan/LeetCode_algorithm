# Write your MySQL query statement below
SELECT project_id
FROM Project  
GROUP BY project_id
HAVING COUNT(employee_id) = (
    SELECT count(employee_id)
    FROM Project
    GROUP BY project_id
    ORDER BY count(employee_id) desc
    LIMIT 1
)