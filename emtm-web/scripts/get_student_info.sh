echo "Get Student Info Test Case 1: (True)"
curl http://localhost:6789/info/stu\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12307"}'
echo ""

echo "Get Student Info Test Case 2: (True)"
curl http://localhost:6789/info/stu\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12308"}'
echo ""

echo "Get Student Info Test Case 3: (False)"
curl http://localhost:6789/info/stu\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12306"}'
echo ""

