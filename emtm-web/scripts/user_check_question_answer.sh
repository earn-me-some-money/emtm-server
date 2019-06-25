echo "Test Case 1: (True)"
curl http://localhost:6789/task/question-naire-answer\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "student_id":2}'
echo ""

echo "Test Case 2: (False)"
curl http://localhost:6789/task/question-naire-answer\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":"dd", "userid":"wechat12306", "student_id":2}'
echo ""

echo "Test Case 3: (False)"
curl http://localhost:6789/task/question-naire-answer\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"dd", "student_id":2}'
echo ""

echo "Test Case 3: (False)"
curl http://localhost:6789/task/question-naire-answer\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "student_id":"dd"}'
echo ""




