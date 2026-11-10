# Persistent PostgreSQL database

This standalone Kustomize project deploys one PostgreSQL 16 instance in the
`projectx` namespace. The database uses a StatefulSet and a 10 GiB
`ReadWriteOnce` PersistentVolumeClaim. The PVC is retained when the pod is
recreated.

## Deploy

Change `POSTGRES_PASSWORD` in `secret.yaml` before applying it. For production,
use an externally managed Secret instead of committing credentials.

```sh
kubectl get pvc -n projectx
kubectl -n projectx rollout status statefulset/postgres
kubectl -n projectx get pod,pvc,service
```

The database is available inside the cluster at:

```text
projectx-postgres.projectx.svc.cluster.local:5432
```

For the web service in this repository, use:

```text
postgres://postgres:<password>@projectx-postgres:5432/projectx
```

## Test locally

```sh
kubectl -n projectx port-forward service/postgres 5432:5432
psql 'postgres://postgres:<password>@127.0.0.1:5432/projectx'
```

The PVC name will be:

```text
postgres-pvc
```

Do not delete this PVC unless the database data should be permanently
removed.
