echo "Get Top Tasks Test Case 1:"
curl http://localhost:6789/task/top\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"number":5}'
echo ""

echo "Get Top Tasks Test Case 2: "
curl http://localhost:6789/task/top\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"number":1}'
echo ""

echo "Get Top Tasks Test Case 3: "
curl http://localhost:6789/task/top\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"number":2}'
echo ""

echo "Get Top Tasks Test Case 4: "
curl http://localhost:6789/task/top\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"number":"[]]"}'
echo ""
