echo "Test-Case 1: Cow-User Logup: "
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"wechat12306", "wechat_ok":true, "logup_mode":false, "email":"1377278218@qq.com", "phone":"13432769342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 2: Cow-User Logup: (False)"
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"[]]", "userid":"wechat12306", "wechat_ok":true, "logup_mode":false, "email":"1377278218@qq.com", "phone":"13432769342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 3: Cow-User Logup: (False)"
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"[]]", "wechat_ok":true, "logup_mode":false, "email":"1377278218@qq.com", "phone":"13432769342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 4: Cow-User Logup: (False)"
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"wechat12306", "wechat_ok":"[]]", "logup_mode":false, "email":"1377278218@qq.com", "phone":"13432769342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 5: Cow-User Logup: (False)"
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"wechat12306", "wechat_ok":true, "logup_mode":"[]]", "email":"1377278218@qq.com", "phone":"13432769342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 6: Cow-User Logup: (False)"
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"wechat12306", "wechat_ok":true, "logup_mode":false, "email":"@qq.com", "phone":"13432769342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 7: Cow-User Logup: (False)"
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"wechat12306", "wechat_ok":true, "logup_mode":false, "email":"1377278218@qq.com", "phone":"13439342", "infos":"A New Comer", "organization":"SYSU-SDCS"}'
echo " "

echo "Test-Case 8: Student-User Logup: "
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoMIng", "userid":"wechat12307", "wechat_ok":true, "logup_mode":true, "email":"1377278216@qq.com", "phone":"13432769341", "infos":"A student", "school_name":"中山大学", "student_id":"16340001", "major":"CS", "year":3}'
echo ""

echo "Test-Case 9: Student-User Logup: "
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoLiang", "userid":"wechat12308", "wechat_ok":true, "logup_mode":true, "email":"1377278219@qq.com", "phone":"13432769346", "infos":"A student", "school_name":"中山大学", "student_id":"16340002", "major":"AS", "year":1}'
echo ""

echo "Test-Case 10: Student-User Logup: (False)"
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"[]]", "userid":"wechat12308", "wechat_ok":true, "logup_mode":true, "email":"1377278219@qq.com", "phone":"13432769346", "infos":"A student", "school_name":"中山大学", "student_id":"16340002", "major":"AS", "year":1}'
echo ""

echo "Test-Case 11: Student-User Logup: (False)"
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoLiang", "userid":"[]]", "wechat_ok":true, "logup_mode":true, "email":"1377278219@qq.com", "phone":"13432769346", "infos":"A student", "school_name":"中山大学", "student_id":"16340002", "major":"AS", "year":1}'
echo ""

echo "Test-Case 12: Student-User Logup: (False)"
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoLiang", "userid":"wechat12308", "wechat_ok":"[]]", "logup_mode":true, "email":"1377278219@qq.com", "phone":"13432769346", "infos":"A student", "school_name":"中山大学", "student_id":"16340002", "major":"AS", "year":1}'
echo ""

echo "Test-Case 13: Student-User Logup: (False)"
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoLiang", "userid":"wechat12308", "wechat_ok":true, "logup_mode":"[]]", "email":"1377278219@qq.com", "phone":"13432769346", "infos":"A student", "school_name":"中山大学", "student_id":"16340002", "major":"AS", "year":1}'
echo ""

echo "Test-Case 14: Student-User Logup: (False)"
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoLiang", "userid":"wechat12308", "wechat_ok":true, "logup_mode":true, "email":"@qq.com", "phone":"13432769346", "infos":"A student", "school_name":"中山大学", "student_id":"16340002", "major":"AS", "year":1}'
echo ""

echo "Test-Case 15: Student-User Logup: (False)"
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoLiang", "userid":"wechat12308", "wechat_ok":true, "logup_mode":true, "email":"1377278219@qq.com", "phone":"13439346", "infos":"A student", "school_name":"中山大学", "student_id":"16340002", "major":"AS", "year":1}'
echo ""

# 注意事项
# 1. Content-Type必须为application/json
# 2. 用户的昵称允许重复，但是一个微信只能注册一个帐号，微信ID，邮箱，手机号，学号不允许重复
