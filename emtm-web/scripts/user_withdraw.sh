echo "Test Case 1: Cow user withdraw"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "withdraw_amount":1}'
echo ""

echo "Test Case 2: Cow user withdraw (False)"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"[]]", "withdraw_amount":1}'
echo ""

echo "Test Case 3: Cow user withdraw (False)"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "withdraw_amount":"[]"}'
echo ""

echo "Test Case 4: Student user withdraw"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "withdraw_amount":2}'
echo ""

echo "Test Case 5: Student user withdraw (False)"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"[]]", "withdraw_amount":2}'
echo ""

echo "Test Case 6: Student user withdraw (False)"
curl http://localhost:6789/balance/withdraw\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "withdraw_amount":"[]"}'
echo ""
