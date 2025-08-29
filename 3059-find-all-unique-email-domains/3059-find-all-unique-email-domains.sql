SELECT 
    split_part(email, '@', 2) AS email_domain,
    COUNT(*) AS count
FROM Emails
WHERE split_part(email, '@', 2) LIKE '%.com'
GROUP BY split_part(email, '@', 2)
ORDER BY email_domain;
