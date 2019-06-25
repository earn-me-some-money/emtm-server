echo "Test-Case 1: Student Receive Task: "
curl http://localhost:6789/task/receive\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308", "poster_id":2, "task_mid":4}'
echo ""

echo "Test-Case 2: Student Receive Task: (False)"
curl http://localhost:6789/task/receive\
 -H "Content-Type:application/json"\
 -d '{"userid":"[]]", "poster_id":2, "task_mid":4}'
echo ""

echo "Test-Case 3: Student Receive Task: (False)"
curl http://localhost:6789/task/receive\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308", "poster_id":"[]", "task_mid":4}'
echo ""

echo "Test-Case 4: Student Receive Task: (False)"
curl http://localhost:6789/task/receive\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308", "poster_id":2, "task_mid":"[]"}'
echo ""
