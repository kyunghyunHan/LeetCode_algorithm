-- Write your PostgreSQL query statement below
SELECT m.symbol AS metal, n.symbol AS nonmetal
FROM Elements m
CROSS JOIN Elements n
WHERE m.type = 'Metal' AND n.type = 'Nonmetal';