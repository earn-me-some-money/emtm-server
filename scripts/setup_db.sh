cargo install diesel_cli --no-default-features --features mysql
cd emtm-db
echo "DATABASE_URL=mysql://travis:@127.0.0.1/EMTM" >.env
diesel setup
cd ..

