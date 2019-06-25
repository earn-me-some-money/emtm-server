echo "Get Cow Info Test Case 1: (True)"
curl http://localhost:6789/info/cow?userid=wechat12306\
 -H "Content-Type:application/json"\
 -X GET
echo ""

echo "Get Cow Info Test Case 2: (False)"
curl http://localhost:6789/info/cow?userid=wechat12307\
 -H "Content-Type:application/json"\
 -X GET
echo ""

