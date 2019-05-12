# emtm-db

## Setting up database with Docker

```
docker run --name emtm-mysql -e MYSQL_ROOT_PASSWORD=ROOT_PASSWORD -e MYSQL_USER=emtm -e MYSQL_PASSWORD=EMTM_PASSWORD -p 9877:3306  -d mysql:5.7
```

Use phpMyAdmin to manage database:
```
docker run --name emtm-myadmin --rm -d --link emtm-mysql:db -p 8080:80 phpmyadmin/phpmyadmin
```

## Examples

Some examples are included in `src/bin`. Use the following command to test them:
```
cargo run --bin show
cargo run --bin add_cows
```

## Contributing Guideline

When you are making a commit, please make sure:

1. Use cargo fmt to format your code first.
2. Make sure your code are commented.
3. Use pull request and make sure your code is reviewed.



