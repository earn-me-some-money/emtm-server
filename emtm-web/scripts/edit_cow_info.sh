echo "Edit Cow Info Test Case 1: (True)"
curl http://localhost:6789/info/cow/edit\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306", "new_email":"1377278212@qq.com", "new_phone":"13432769345", "new_infos":"中文~"}'
echo ""

echo "Edit Cow Info Test Case 2: (False)"
curl http://localhost:6789/info/cow/edit\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "new_email":"1377278216@qq.com", "new_phone":"13432769341", "new_infos":"中文~"}'
echo ""

echo "Edit Cow Info Test Case 3: (False)"
curl http://localhost:6789/info/cow/edit\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "new_email":"@qq.com", "new_phone":"13432769341", "new_infos":"中文~"}'
echo ""

echo "Edit Cow Info Test Case 4: (False)"
curl http://localhost:6789/info/cow/edit\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "new_email":"1377278216@qq.com", "new_phone":"1343271", "new_infos":"中文~"}'
echo ""

echo "Edit Cow Info Test Case 5: (False)"
curl http://localhost:6789/info/cow/edit\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307", "new_email":"1377278216@qq.com", "new_phone":"13432769341", "new_infos":""}'
echo ""

