SELECT s.user_id as user_id , SUM(s.quantity * p.price) as spending
FROM Sales s
LEFT JOIN Product p
ON p.product_id = s.product_id
GROUP BY s.user_id
ORDER BY s.user_id ASC;
