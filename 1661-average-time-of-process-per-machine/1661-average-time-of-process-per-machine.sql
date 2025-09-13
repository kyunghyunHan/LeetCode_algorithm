-- Write your PostgreSQL query statement below
SELECT machine_id , 
 ROUND(
          (SELECT avg(a1.timestamp) FROM Activity a1 WHERE a1.activity_type = 'end' AND a1.machine_id = a.machine_id) - 
         (SELECT avg(a1.timestamp) FROM Activity a1 WHERE a1.activity_type = 'start' AND a1.machine_id = a.machine_id)

,3 ) 
FROM Activity a
GROUP BY machine_id , processing_time

