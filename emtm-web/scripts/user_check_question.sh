echo "Test Case 1: (True)"
curl http://localhost:6789/task/question-naire\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "poster_id":1}'
echo ""

echo "Test Case 1: (False)"
curl http://localhost:6789/task/question-naire\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":"dd", "userid":"wechat12306", "poster_id":1}'
echo ""

echo "Test Case 1: (False)"
curl http://localhost:6789/task/question-naire\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"dd", "poster_id":1}'
echo ""

echo "Test Case 1: (False)"
curl http://localhost:6789/task/question-naire\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "poster_id":"dd"}'
echo ""


