SELECT
    u.name AS name,
    SUM(t.amount) AS balance
FROM Users u
LEFT JOIN Transactions t
    ON u.account = t.account
GROUP BY u.name
HAVING SUM(t.amount) > 10000;