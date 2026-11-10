# Redis

This Kustomize project deploys a single in-memory Redis instance in the
`projectx` namespace.

Deploy it with:

```sh
kubectl apply -k k8s/redis
kubectl -n projectx rollout status deployment/projectx-redis
```

The web service can connect using:

```text
redis://projectx-redis:6379
```

Redis is exposed only inside the cluster through a `ClusterIP` Service. The
deployment does not persist data; Redis data is lost when the pod is replaced.
Use a persistent Redis deployment or managed Redis service if the data must
survive restarts.
