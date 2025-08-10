WITH TotalSales AS (
    SELECT seller_id, SUM(price) AS total_price
    FROM Sales
    GROUP BY seller_id
),
MaxSales AS (
    SELECT MAX(total_price) AS max_price
    FROM TotalSales
)
SELECT seller_id
FROM TotalSales
WHERE total_price = (SELECT max_price FROM MaxSales);
