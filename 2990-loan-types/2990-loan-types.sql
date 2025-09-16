SELECT user_id
FROM Loans
GROUP BY user_id
HAVING 
    COUNT(DISTINCT loan_type) >= 2
    AND SUM(CASE WHEN loan_type = 'Mortgage' THEN 1 ELSE 0 END) > 0
    AND SUM(CASE WHEN loan_type = 'Refinance' THEN 1 ELSE 0 END) > 0;
