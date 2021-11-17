# Backups (restic, duplicity...)

Backups are essential for disaster recovery but they are not trivial to manage.
Using Garage as your backup target will enable you to scale your storage as needed while ensuring high availability.

## Borg Backup

Borg Backup is very popular among the backup tools but it is not yet compatible with the S3 API.
We recommend using any other tool listed in this guide because they are all compatible with the S3 API.
If you still want to use Borg, you can use it with `rclone mount`.



## Restic

Create your key and bucket:

```bash
garage key new my-key
garage bucket create backup
garage bucket allow backup --read --write --key my-key
```

Then register your Key ID and Secret key in your environment:

```bash
export AWS_ACCESS_KEY_ID=GKxxx
export AWS_SECRET_ACCESS_KEY=xxxx
```

Configure restic from environment too:

```bash
export RESTIC_REPOSITORY="s3:http://localhost:3900/backups"

echo "Generated password (save it safely): $(openssl rand -base64 32)"
export RESTIC_PASSWORD=xxx # copy paste your generated password here
```

Do not forget to save your password safely (in your password manager or print it). It will be needed to decrypt your backups.

Now you can use restic:

```bash
# Initialize the bucket, must be run once
restic init

# Backup your PostgreSQL database
# (We suppose your PostgreSQL daemon is stopped for all commands)
restic backup /var/lib/postgresql

# Show backup history
restic snapshots

# Backup again your PostgreSQL database, it will be faster as only changes will be uploaded
restic backup /var/lib/postgresql

# Show backup history (again)
restic snapshots

# Restore a backup
# (79766175 is the ID of the snapshot you want to restore)
mv /var/lib/postgresql /var/lib/postgresql.broken
restic restore 79766175 --target /var/lib/postgresql
```

Restic has way more features than the ones presented here.
You can discover all of them by accessing its documentation from the link below.

*External links:* [Restic Documentation > Amazon S3](https://restic.readthedocs.io/en/stable/030_preparing_a_new_repo.html#amazon-s3)

## Duplicity

*External links:* [Duplicity > man](https://duplicity.gitlab.io/duplicity-web/vers8/duplicity.1.html) (scroll to "URL Format" and "A note on Amazon S3")

## Duplicati

*External links:* [Duplicati Documentation > Storage Providers](https://github.com/kees-z/DuplicatiDocs/blob/master/docs/05-storage-providers.md#s3-compatible)

## knoxite

*External links:* [Knoxite Documentation > Storage Backends](https://knoxite.com/docs/storage-backends/#amazon-s3)

## kopia

*External links:* [Kopia Documentation > Repositories](https://kopia.io/docs/repositories/#amazon-s3)

