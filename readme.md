run the app:

    docker-compose up -d


execute the migration: (1 time)

    sqlx mig run (it will use the .env file for database url)

start psql server:
    sudo systemctl start postgresql
    -see status : systemctl status postgresql.service

    connect: sudo su - postgres
             psql

florian/yoann = azertyuiop