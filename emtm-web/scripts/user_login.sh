echo "Test-Case 1: Cow-User Login: "
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "wechat_ok":true, "login_mode":false}'
echo " "

echo "Test-Case 2: Cow-User Login: (False)"
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"[]]", "wechat_ok":true, "login_mode":false}'
echo " "

echo "Test-Case 3: Cow-User Login: (False)"
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "wechat_ok":"[]]", "login_mode":false}'
echo " "

echo "Test-Case 4: Cow-User Login: (False)"
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "wechat_ok":true, "login_mode":"[]]"}'
echo " "

echo "Test-Case 5: Student-User Login: "
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "wechat_ok":true, "login_mode":true}'
echo " "

echo "Test-Case 6: Student-User Login: (False)"
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"[]]", "wechat_ok":true, "login_mode":true}'
echo " "

echo "Test-Case 7: Student-User Login: (False)"
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "wechat_ok":"[]]", "login_mode":true}'
echo " "

echo "Test-Case 8: Student-User Login: (False)"
curl http://localhost:6789/login\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "wechat_ok":true, "login_mode":"{}}"}'
echo " "
