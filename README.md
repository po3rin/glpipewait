# GitLab Pipe Wait

Wait until GitLab pipeline finishes

```sh
$ cargo install glpipewait
$ glpipewait -u https://<GITLAB_HOST>/<NAMESPACE/PROJECT>/-/pipelines/<PIPELINE_ID> -t <GITLAB_ACCESS_TOKEN> && echo done!!!
```

