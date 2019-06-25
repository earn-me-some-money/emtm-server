echo "Test Case 1: Cow user recharge"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "recharge_amount":0}'
echo ""

echo "Test Case 2: Cow user recharge (False)"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"[]]", "recharge_amount":0}'
echo ""

echo "Test Case 3: Cow user recharge (False)"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "recharge_amount":"[]"}'
echo ""

echo "Test Case 4: Student user recharge"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308", "recharge_amount":6}'
echo ""

echo "Test Case 5: Student user recharge (False)"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"[]]", "recharge_amount":6}'
echo ""

echo "Test Case 6: Student user recharge (False)"
curl http://localhost:6789/balance/recharge\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12308", "recharge_amount":"[]"}'
echo ""
