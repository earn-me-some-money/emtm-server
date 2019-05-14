cd emtm-db
echo "DATABASE_URL=mysql://travis:@127.0.0.1/EMTM_TEST" >.env
echo "TEST_DATABASE_URL=mysql://travis:@127.0.0.1/EMTM_TEST" >>.env
diesel setup
cd ..

