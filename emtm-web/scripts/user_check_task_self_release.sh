echo "Test-Case 1: Cow Check Task Self Release: "
curl http://localhost:6789/task/self-release\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12307"}'
echo ""

echo "Test-Case 2: Cow Check Task Self Release: (False)"
curl http://localhost:6789/task/self-release\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"[][]]"}'
echo ""

echo "Test-Case 3: Student Check Task Self Release: "
curl http://localhost:6789/task/self-release\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"wechat12306"}'
echo ""

echo "Test-Case 4: Student Check Task Self Release: (False)"
curl http://localhost:6789/task/self-release\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"userid":"[][]]"}'
echo ""
