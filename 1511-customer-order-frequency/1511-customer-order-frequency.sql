SELECT 
    c.customer_id,
    c.name AS name
FROM Customers c
JOIN Orders o
    ON c.customer_id = o.customer_id
JOIN Product p
    ON o.product_id = p.product_id
WHERE o.order_date >= '2020-06-01'
  AND o.order_date < '2020-08-01'
GROUP BY c.customer_id, c.name
HAVING 
    SUM(CASE WHEN EXTRACT(MONTH FROM o.order_date) = 6 THEN o.quantity * p.price ELSE 0 END) >= 100
    AND
    SUM(CASE WHEN EXTRACT(MONTH FROM o.order_date) = 7 THEN o.quantity * p.price ELSE 0 END) >= 100;
