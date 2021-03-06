echo "Test Case 1: (True):"
curl http://localhost:6789/task/errand\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":2, "userid":"wechat12306", "poster_id":2}'
echo ""

echo "Test Case 2: (False, target mission is not errand):"
curl http://localhost:6789/task/errand\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "poster_id":1}'
echo ""

echo "Test Case 3: (False):"
curl http://localhost:6789/task/errand\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":"op", "userid":"wechat12306", "poster_id":1}'
echo ""

echo "Test Case 4: (False):"
curl http://localhost:6789/task/errand\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"[][]]", "poster_id":1}'
echo ""

echo "Test Case 5: (False):"
curl http://localhost:6789/task/errand\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"task_mid":1, "userid":"wechat12306", "poster_id":"cd"}'
echo ""
