echo "Test-Case 1: Cow-User Logup: "
curl http://localhost:6789/logup/cow\
 -H "Content-Type:application/json"\
 -d '{"username":"SDCS-Yard", "userid":"wechat12306", "wechat_ok":true, "logup_mode":false, "email":"1377278218@qq.com", "phone":"13432769342", "infos":"A New Comer"}'
echo " "


echo "Test-Case 2: Student-User Logup: "
curl http://localhost:6789/logup/stu\
 -H "Content-Type:application/json"\
 -d '{"username":"XiaoMIng", "userid":"wechat12309", "wechat_ok":true, "logup_mode":true, "email":"1377278218@qq.com", "phone":"13432769351", "infos":"A student", "major":"CS", "year":3}'
echo ""


# 注意事项
# 1. Content-Type必须为application/json
# 2. 用户的昵称允许重复，但是一个微信只能注册一个帐号，微信ID，邮箱，手机号，学号不允许重复
