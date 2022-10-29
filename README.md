# GitLab Pipe Wait

[![ci](https://github.com/po3rin/glpipewait/actions/workflows/ci.yaml/badge.svg)](https://github.com/po3rin/glpipewait/actions/workflows/ci.yaml) [![Crate](https://img.shields.io/crates/v/glpipewait.svg)](https://crates.io/crates/glpipewait)

Wait until GitLab pipeline finishes

```sh
$ cargo install glpipewait
$ glpipewait -u https://<GITLAB_HOST>/<NAMESPACE/PROJECT>/-/pipelines/<PIPELINE_ID> -t <GITLAB_ACCESS_TOKEN> && echo done!!!
```

