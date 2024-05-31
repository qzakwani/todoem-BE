FROM postgres:16.3-bullseye


RUN apt-get update && apt-get install -y --no-install-recommends \
    postgresql-16-cron \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*



RUN mkdir -p /.config
COPY ./postgresql.conf /.config/postgresql.conf
CMD ["postgres", "-c", "config_file=/.config/postgresql.conf"]